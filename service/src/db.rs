use rusqlite::{params, Connection, Result};
use uuid::Uuid;

pub fn init_tables(conn: &Connection) -> Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS rawEvent (
            id TEXT NOT NULL PRIMARY KEY,
            timestamp INTEGER NOT NULL,
            device_id TEXT NOT NULL,
            app_pkg_name TEXT NOT NULL,
            window_title TEXT,
            is_idle INTEGER NOT NULL DEFAULT 0
        )",
        [],
    )?;
    Ok(())
}

pub fn log_event(
    conn: &Connection, 
    timestamp: i64, 
    device_id: &str, 
    app: &str, 
    title: &str, 
    is_idle: bool
) -> Result<()> {
    let id = Uuid::new_v4().to_string();
    conn.execute(
        "INSERT INTO rawEvent (id, timestamp, device_id, app_pkg_name, window_title, is_idle)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
        params![id, timestamp, device_id, app, title, if is_idle { 1 } else { 0 }],
    )?;
    Ok(())
}
