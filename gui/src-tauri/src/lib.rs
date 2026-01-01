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
    
    let exe = std::env::current_exe().unwrap_or_default();
    let dir = exe.parent().unwrap_or(Path::new("."));
    
    // Check if we are in dev mode (cargo run) -> db might be in ../../../target/release/self_monitor.db
    let dev_path = dir.join("../../../target/release/self_monitor.db");
    if dev_path.exists() {
        return dev_path;
    }

    // Default to sibling (prod)
    dir.join("self_monitor.db")
}

#[command]
fn get_dashboard_stats() -> Result<DashboardStats, String> {
    let db_path = get_db_path();
    let conn = Connection::open(&db_path).map_err(|e| format!("DB Error: {}", e))?;

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

    // 2. Today Stats (Live Calc)
    let today = chrono::Local::now().format("%Y-%m-%d").to_string();
    let mut prod = 0;
    let mut dist = 0;

    let mut check_sessions = conn.prepare("SELECT name FROM sqlite_master WHERE type='table' AND name='activity_sessions'").map_err(|e| e.to_string())?;
    if check_sessions.query([]).map_err(|e| e.to_string())?.next().unwrap_or(None).is_some() {
        let mut stmt = conn.prepare("SELECT category, SUM(duration_seconds) FROM activity_sessions WHERE date = ?1 GROUP BY category").map_err(|e| e.to_string())?;
        let mut rows = stmt.query([&today]).map_err(|e| e.to_string())?;
        while let Ok(Some(row)) = rows.next() {
            let cat: String = row.get(0).unwrap_or_default();
            let dur: i64 = row.get(1).unwrap_or(0);
            let c = cat.to_lowercase();
            if c == "productive" { prod += dur; }
            else if c == "distracting" { dist += dur; }
        }
    }

    // Strict Formula: effective = prod - (dist * 0.75)
    let penalty = (dist as f64 * 0.75) as i64;
    let effective = std::cmp::max(0, prod - penalty);

    Ok(DashboardStats {
        streak,
        best_streak,
        today_productivity_min: effective / 60, // Return Effective !
        today_distraction_min: dist / 60,
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
    let output = Command::new("sc")
        .arg("query")
        .arg("SelfMonitorService")
        .output();

    match output {
        Ok(o) => {
             let stdout = String::from_utf8_lossy(&o.stdout);
             if stdout.contains("RUNNING") {
                 "Running".to_string()
             } else if stdout.contains("STOPPED") {
                 "Stopped".to_string()
             } else {
                 "Unknown".to_string()
             }
        },
        Err(_) => "Unknown".to_string(),
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
