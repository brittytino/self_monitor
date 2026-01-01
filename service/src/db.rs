use rusqlite::{params, Connection, Result};
use std::path::Path;
use chrono::Utc;

pub struct AppDatabase {
    pub conn: Connection,
}

impl AppDatabase {
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self> {
        let conn = Connection::open(path)?;
        
        // Performance & Reliability settings
        // Performance & Reliability settings
        conn.pragma_update(None, "journal_mode", "WAL")?;
        conn.pragma_update(None, "synchronous", "NORMAL")?;
        conn.pragma_update(None, "foreign_keys", "ON")?;

        Ok(Self { conn })
    }

    pub fn init_schema(&self) -> Result<()> {
        let schema = include_str!("../schema.sql");
        self.conn.execute_batch(schema)?;
        Ok(())
    }

    // --- Activity Logs ---
    pub fn insert_activity_log(&self, app_name: &str, window_title: &str, is_idle: bool) -> Result<()> {
        let timestamp = Utc::now().timestamp();
        self.conn.execute(
            "INSERT INTO activity_logs (timestamp_utc, app_name, window_title, is_idle) VALUES (?1, ?2, ?3, ?4)",
            params![timestamp, app_name, window_title, is_idle],
        )?;
        Ok(())
    }

    // --- Streaks ---
    // Read specific streak info
    pub fn get_streak_info(&self) -> Result<(i32, i32, Option<String>)> {
        let mut stmt = self.conn.prepare("SELECT current_streak, best_streak, last_evaluated_date FROM streaks WHERE id = 1")?;
        let mut rows = stmt.query([])?;
        
        if let Some(row) = rows.next()? {
            Ok((
                row.get(0)?,
                row.get(1)?,
                row.get(2)?,
            ))
        } else {
            // Should be initialized by schema, but fallback safety
            Ok((0, 0, None))
        }
    }

    pub fn get_last_session_end_time(&self) -> Result<Option<i64>> {
        let mut stmt = self.conn.prepare("SELECT MAX(end_time_utc) FROM activity_sessions")?;
        let mut rows = stmt.query([])?;
        if let Some(row) = rows.next()? {
            Ok(row.get(0)?)
        } else {
            Ok(None)
        }
    }

    pub fn get_logs_since(&self, timestamp: i64) -> Result<Vec<ActivityLog>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, timestamp_utc, app_name, window_title, is_idle FROM activity_logs WHERE timestamp_utc > ?1 ORDER BY timestamp_utc ASC"
        )?;
        let rows = stmt.query_map([timestamp], |row| {
            Ok(ActivityLog {
                id: row.get(0)?,
                timestamp_utc: row.get(1)?,
                app_name: row.get(2)?,
                window_title: row.get(3)?,
                is_idle: row.get(4)?,
            })
        })?;

        let mut logs = Vec::new();
        for log in rows {
            logs.push(log?);
        }
        Ok(logs)
    }

    pub fn insert_session(&self, date: &str, app: &str, category: &str, start: i64, end: i64, duration: i64) -> Result<()> {
        self.conn.execute(
            "INSERT OR IGNORE INTO activity_sessions (date, app_name, category, start_time_utc, end_time_utc, duration_seconds) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            params![date, app, category, start, end, duration],
        )?;
        Ok(())
    }

    pub fn get_unsummarized_days(&self, before_date: &str) -> Result<Vec<String>> {
        let mut stmt = self.conn.prepare(
            "SELECT DISTINCT date FROM activity_sessions WHERE date < ?1 AND date NOT IN (SELECT date FROM daily_summaries) ORDER BY date ASC"
        )?;
        let rows = stmt.query_map([before_date], |row| row.get(0))?;
        let mut dates = Vec::new();
        for r in rows {
            dates.push(r?);
        }
        Ok(dates)
    }

    pub fn get_day_category_totals(&self, date: &str) -> Result<Vec<(String, i64)>> {
        let mut stmt = self.conn.prepare(
            "SELECT category, SUM(duration_seconds) FROM activity_sessions WHERE date = ?1 GROUP BY category"
        )?;
        let rows = stmt.query_map([date], |row| Ok((row.get(0)?, row.get(1)?)))?;
        let mut totals = Vec::new();
        for r in rows {
            totals.push(r?);
        }
        Ok(totals)
    }

    pub fn insert_daily_summary(&self, summary: &DailySummary) -> Result<()> {
        self.conn.execute(
            "INSERT INTO daily_summaries (date, productive_seconds, neutral_seconds, recovery_seconds, distracting_seconds, idle_seconds, effective_work_seconds, qualified, qualification_reason, finalized_at_utc) 
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)",
             params![
                 summary.date,
                 summary.productive,
                 summary.neutral,
                 summary.recovery,
                 summary.distracting,
                 summary.idle, // Handled if tracked, otherwise 0
                 summary.effective_work,
                 summary.qualified,
                 summary.reason,
                 Utc::now().timestamp()
             ]
        )?;
        Ok(())
    }

    pub fn update_streak(&self, new_current: i32, new_best: i32, evaluated_date: &str) -> Result<()> {
        self.conn.execute(
            "UPDATE streaks SET current_streak = ?1, best_streak = ?2, last_evaluated_date = ?3 WHERE id = 1",
            params![new_current, new_best, evaluated_date]
        )?;
        Ok(())
    }

    pub fn update_intraday_stats(&self, date: &str, prod: i64, distract: i64, effective: i64) -> Result<()> {
        let now = Utc::now().timestamp();
        self.conn.execute(
            "UPDATE intraday_stats SET date = ?1, productive_seconds = ?2, distracting_seconds = ?3, effective_work_seconds = ?4, updated_at_utc = ?5 WHERE id = 1",
            params![date, prod, distract, effective, now]
        )?;
        Ok(())
    }
}

pub struct DailySummary {
    pub date: String,
    pub productive: i64,
    pub neutral: i64,
    pub recovery: i64,
    pub distracting: i64,
    pub idle: i64,
    pub effective_work: i64,
    pub qualified: bool,
    pub reason: String,
}
#[derive(Debug)]
#[allow(dead_code)]
pub struct ActivityLog {
    pub id: i64,
    pub timestamp_utc: i64,
    pub app_name: String,
    pub window_title: String,
    pub is_idle: bool,
}
