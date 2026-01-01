// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

use tauri::command;
use rusqlite::Connection;
use std::path::{Path, PathBuf};
use serde::Serialize;
use std::process::Command;

#[derive(Serialize)]
struct DashboardStats {
    streak: i32,
    best_streak: i32,
    today_productivity_min: i64,
    today_distraction_min: i64,
}

#[derive(Debug, Serialize)]
struct ActivityLog {
    app: String,
    title: String, 
    category: String,
    duration: i64,
}

fn get_db_path() -> PathBuf {
    // In dev: ../target/release/self_monitor.db (relative to gui/src-tauri/target/debug/...)
    // Actually, simpler: Look for it in the service build output location relative to CWD of the app
    // For production (installed), it should be in the same folder.
    
    // Simplified: Always look next to executable first (Production/Same-Folder), then check dev paths.
    // Since we will move the UI to the same folder as the service in build.ps1, this is robust.
    let exe = std::env::current_exe().unwrap_or_default();
    let dir = exe.parent().unwrap_or(Path::new("."));
    
    let sibling_db = dir.join("self_monitor.db");
    if sibling_db.exists() {
        return sibling_db;
    }

    // fallback for dev (cargo tauri dev)
    // CWD is usually src-tauri
    // DB is in target/release/self_monitor.db (from root) -> ../../target/release/self_monitor.db
    if Path::new("../../target/release/self_monitor.db").exists() {
        return PathBuf::from("../../target/release/self_monitor.db");
    }

    // fallback default
    sibling_db
}

#[command]
fn get_dashboard_stats() -> Result<DashboardStats, String> {
    let db_path = get_db_path();
    let conn = Connection::open(&db_path).map_err(|e| format!("DB Error at {:?}: {}", db_path, e))?;

    // 1. Streaks
    let mut streak = 0;
    let mut best_streak = 0;
    
    // Check if table exists first to avoid panic logic
    let mut check_stmt = conn.prepare("SELECT name FROM sqlite_master WHERE type='table' AND name='streaks'").map_err(|e| e.to_string())?;
    let mut check_rows = check_stmt.query([]).map_err(|e| e.to_string())?;
    
    if check_rows.next().unwrap_or(None).is_some() {
        let mut stmt = conn.prepare("SELECT current_streak, best_streak FROM streaks WHERE id = 1").map_err(|e| e.to_string())?;
        let mut rows = stmt.query([]).map_err(|e| e.to_string())?;
        if let Ok(Some(row)) = rows.next() {
            streak = row.get(0).unwrap_or(0);
            best_streak = row.get(1).unwrap_or(0);
        }
    }

    // 2. Today Stats (Read Only from intraday_stats)
    let mut effective_minutes = 0; // minutes
    let mut distracting_minutes = 0; // minutes

    // Check if table exists (race condition safety)
    let mut check_live = conn.prepare("SELECT name FROM sqlite_master WHERE type='table' AND name='intraday_stats'").map_err(|e| e.to_string())?;
    if check_live.query([]).map_err(|e| e.to_string())?.next().unwrap_or(None).is_some() {
        let mut stmt = conn.prepare("SELECT effective_work_seconds, distracting_seconds FROM intraday_stats WHERE id = 1").map_err(|e| e.to_string())?;
        let mut rows = stmt.query([]).map_err(|e| e.to_string())?;
        if let Ok(Some(row)) = rows.next() {
            let eff_sec: i64 = row.get(0).unwrap_or(0);
            let dist_sec: i64 = row.get(1).unwrap_or(0);
            
            effective_minutes = eff_sec / 60;
            distracting_minutes = dist_sec / 60;
        }
    }

    Ok(DashboardStats {
        streak,
        best_streak,
        today_productivity_min: effective_minutes,
        today_distraction_min: distracting_minutes,
    })
}

#[command]
fn get_recent_activity() -> Result<Vec<ActivityLog>, String> {
    let db_path = get_db_path();
    let conn = Connection::open(&db_path).map_err(|e| format!("DB Error: {}", e))?;
    let mut activities = Vec::new();

    let mut check_stmt = conn.prepare("SELECT name FROM sqlite_master WHERE type='table' AND name='activity_sessions'").map_err(|e| e.to_string())?;
    if check_stmt.query([]).map_err(|e| e.to_string())?.next().unwrap_or(None).is_some() {
        let mut stmt = conn.prepare("SELECT app_name, window_title, category, duration_seconds FROM activity_sessions ORDER BY start_time_utc DESC LIMIT 20").map_err(|e| e.to_string())?;
        let rows = stmt.query_map([], |row| {
            Ok(ActivityLog {
                app: row.get(0)?,
                title: row.get(1)?,
                category: row.get(2)?,
                duration: row.get(3)?,
            })
        }).map_err(|e| e.to_string())?;

        for log in rows {
            if let Ok(l) = log {
                activities.push(l);
            }
        }
    }
    
    Ok(activities)
}

#[command]
fn get_service_status() -> String {
    // Passive Check: Is the Service writing to the DB or Log?
    // This avoids spawning "tasklist.exe" which causes black screens/flashing.
    
    let db_path = get_db_path();
    // Also check service.log as a backup or primary indicator of life
    let exe = std::env::current_exe().unwrap_or_default();
    let dir = exe.parent().unwrap_or(Path::new("."));
    let log_path = dir.join("service.log");

    let check_file = |path: &Path| -> bool {
        if let Ok(metadata) = std::fs::metadata(path) {
            if let Ok(modified) = metadata.modified() {
                if let Ok(elapsed) = modified.elapsed() {
                    // Service writes every 10s. Allow 20s buffer. 
                    // Say 30s to be safe.
                    return elapsed.as_secs() < 30;
                }
            }
        }
        false
    };

    if check_file(&db_path) || check_file(&log_path) {
        "Running".to_string()
    } else {
        // If neither file is touched in 30s, likely stopped
        "Stopped (No Heartbeat)".to_string()
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            greet, 
            get_dashboard_stats, 
            get_recent_activity,
            get_service_status
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
