// #[macro_use] removed as it triggered unused warning
extern crate windows_service;

use std::ffi::OsString;
use std::sync::mpsc::channel;
use std::time::Duration;
use std::thread;

use windows_service::service::{
    ServiceControl, ServiceControlAccept, ServiceExitCode, ServiceState, ServiceStatus,
    ServiceType,
};
use windows_service::service_control_handler::{self, ServiceControlHandlerResult};
use windows_service::{define_windows_service, service_dispatcher};

mod db;
mod watcher;
mod engine; 

use db::AppDatabase;
use watcher::SystemWatcher;
use engine::SessionEngine;

use log::{info, error};
use simplelog::{WriteLogger, Config, LevelFilter};
use std::fs::File;

define_windows_service!(ffi_service_main, my_service_main);

fn main() -> windows_service::Result<()> {
    // Attempt to run as a service
    // If this fails (e.g., running from console), we could fallback to console mode for debugging
    // But typically for a pure service binary, we just let it fail or handle specific args.
    // For debugging convenience, if it fails, we run the logic directly.
    
    let result = service_dispatcher::start("SelfMonitorService", ffi_service_main);
    
    if let Err(_e) = result {
        // Fallback for console debugging
        println!("Failed to start as service (expected if running from console). Running in console mode...");
        run_service_logic(None)?;
    }
    Ok(())
}

fn my_service_main(arguments: Vec<OsString>) {
    if let Err(_e) = run_service(arguments) {
        // Handle error in service startup
    }
}

fn run_service(_arguments: Vec<OsString>) -> windows_service::Result<()> {
    let (shutdown_tx, shutdown_rx) = channel();

    let event_handler = move |control_event| -> ServiceControlHandlerResult {
        match control_event {
            ServiceControl::Stop | ServiceControl::Interrogate => {
                let _ = shutdown_tx.send(());
                ServiceControlHandlerResult::NoError
            }
            _ => ServiceControlHandlerResult::NotImplemented,
        }
    };

    let status_handle = service_control_handler::register("SelfMonitorService", event_handler)?;

    let next_status = ServiceStatus {
        service_type: ServiceType::OWN_PROCESS,
        current_state: ServiceState::Running,
        controls_accepted: ServiceControlAccept::STOP,
        exit_code: ServiceExitCode::Win32(0),
        checkpoint: 0,
        wait_hint: Duration::default(),
        process_id: None,
    };

    status_handle.set_service_status(next_status.clone())?;

    // Execute actual logic in a separate thread or just here if non-blocking?
    // Our logic IS blocking (loop). So we need to handle shutdown.
    // We can run the loop and check the channel, or spawn a thread.
    // Spawning a thread for the work and monitoring the channel here is standard.
    
    let (work_tx, work_rx) = channel();
    
    let worker = thread::spawn(move || {
        let _ = run_service_logic(Some(work_rx));
    });

    // Wait for stop signal
    let _ = shutdown_rx.recv();

    // Signal worker to stop
    let _ = work_tx.send(()); 

    let stop_status = ServiceStatus {
        service_type: ServiceType::OWN_PROCESS,
        current_state: ServiceState::Stopped,
        controls_accepted: ServiceControlAccept::empty(),
        exit_code: ServiceExitCode::Win32(0),
        checkpoint: 0,
        wait_hint: Duration::default(),
        process_id: None,
    };

    status_handle.set_service_status(stop_status)?;
    
    // In a real app we'd join the worker, but since the worker loops with sleep, 
    // strictly we'd need an atomic bool or channel check inside the loop.
    let _ = worker.join();
    
    Ok(())
}

fn run_service_logic(shutdown_rx: Option<std::sync::mpsc::Receiver<()>>) -> windows_service::Result<()> {
    // Determine log path: next to exe
    let exe_path = std::env::current_exe().unwrap_or_else(|_| std::path::PathBuf::from("."));
    let exe_dir = exe_path.parent().unwrap_or(std::path::Path::new("."));
    let log_path = exe_dir.join("service.log");

    if let Ok(file) = File::create(&log_path) {
        let _ = WriteLogger::init(LevelFilter::Info, Config::default(), file);
    }
    
    info!("Starting Self Monitor Background Service");

    // db_path resolution
    let db_path = exe_dir.join("self_monitor.db");
    
    // Ensure the path converts to string for Rusqlite or use impl AsRef<Path>
    let db = match AppDatabase::new(&db_path) {
        Ok(d) => d,
        Err(e) => {
            error!("Failed to open database: {}", e);
            return Ok(());
        }
    };

    if let Err(e) = db.init_schema() {
        error!("Failed to initialize schema: {}", e);
        return Ok(());
    }
    info!("Database initialized at {}", db_path.display());

    // Watcher Loop
    info!("Starting monitoring loop...");
    loop {
        // Check for shutdown signal if running as service
        if let Some(ref rx) = shutdown_rx {
             if rx.try_recv().is_ok() {
                 info!("Shutdown signal received. Exiting loop.");
                 break;
             }
        }

        let (app, title) = SystemWatcher::get_active_window_info();
        let idle_sec = SystemWatcher::get_idle_seconds();
        let is_idle = idle_sec > 300; // 5 minutes strict

        info!("Observed: [{}] {} (Idle: {}s)", app, title, idle_sec);

        if let Err(e) = db.insert_activity_log(&app, &title, is_idle) {
            error!("Failed to write log: {}", e);
        }

        // Run Session Engine
        if let Err(e) = SessionEngine::process_sessions(&db) {
            error!("Session processing failed: {}", e);
        }

        // Run Daily Evaluator
        if let Err(e) = SessionEngine::evaluate_history(&db) {
            error!("Daily evaluation failed: {}", e);
        }

        // 60 seconds sampling interval as per spec
        thread::sleep(Duration::from_secs(60));
    }
    
    Ok(())
}
