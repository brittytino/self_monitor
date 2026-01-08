use std::thread;
use std::time::Duration;
// Windows imports moved to tracker.rs
use rusqlite::Connection;
use chrono::Utc;

mod db;
mod tracker;

fn main() -> anyhow::Result<()> {
    println!("Starting Self Monitor Service...");
    
    // Initialize DB
    let db_path = "../data.db"; // Store in root for now
    let conn = Connection::open(db_path)?;
    db::init_tables(&conn)?;
    
    // Main Loop
    loop {
        // 1. Get Active Window
        let (window_title, app_pkg_name) = tracker::get_active_window_info();
        
        // 2. Check Idle
        let is_idle = tracker::is_user_idle();
        
        // 3. Log to DB
        let timestamp = Utc::now().timestamp_millis();
        let device_id = "WINDOWS_PC"; // TODO: Get actual hostname or UUID
        
        println!("Tick: {} | {} | Idle: {}", app_pkg_name, window_title, is_idle);
        
        db::log_event(&conn, timestamp, device_id, &app_pkg_name, &window_title, is_idle)?;
        
        // Sleep 1s
        thread::sleep(Duration::from_secs(1));
    }
}
