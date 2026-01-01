use crate::db::AppDatabase;
use rusqlite::Result;
use log::info;
use chrono::TimeZone;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Category {
    Productive,
    Neutral,
    #[allow(dead_code)]
    Recovery,
    Distracting,
    Idle,
}

impl Category {
    pub fn as_str(&self) -> &'static str {
        match self {
            Category::Productive => "productive",
            Category::Neutral => "neutral",
            Category::Recovery => "recovery",
            Category::Distracting => "distracting",
            Category::Idle => "idle",
        }
    }

    /// Strict Pessimistic Categorization
    /// Config-driven via categories.json
    pub fn classify(app_name: &str, window_title: &str, is_idle: bool) -> Self {
        if is_idle {
            return Category::Idle;
        }

        let rules = crate::config::load_rules();
        let app = app_name.to_lowercase();
        let title = window_title.to_lowercase();

        // 1. DISTRACTING (Highest Priority)
        for keyword in &rules.distracting {
            if app.contains(keyword) || title.contains(keyword) {
                return Category::Distracting;
            }
        }

        // 2. PRODUCTIVE
        for keyword in &rules.productive {
            if app.contains(keyword) || title.contains(keyword) {
                return Category::Productive;
            }
        }

        // 3. RECOVERY
        for keyword in &rules.recovery {
            if app.contains(keyword) || title.contains(keyword) {
                return Category::Recovery;
            }
        }

        // 4. NEUTRAL (Default)
        Category::Neutral
    }
}

pub struct SessionEngine;

impl SessionEngine {
    pub fn process_sessions(db: &AppDatabase) -> Result<()> {
        // "Sessions represent final structural truth"
        // "Derived once... already processed logs are skipped"
        
        let last_end = db.get_last_session_end_time()?.unwrap_or(0);
        let logs = db.get_logs_since(last_end)?;

        if logs.is_empty() {
            return Ok(());
        }

        // Logic: Group raw logs into sessions
        // A session breaks if:
        // 1. App Name changes
        // 2. Gap > 1.5 * interval (90s)
        
        // We only write a session when it is BROKEN (completed).
        // The last pending session remains in memory (implicitly, by not writing it)
        // and will be re-processed next tick from the same `last_end` anchor.
        
        let mut current_buffer: Vec<&crate::db::ActivityLog> = Vec::new();

        for log in &logs {
            let should_break = if let Some(last) = current_buffer.last() {
                let gap = log.timestamp_utc - last.timestamp_utc;
                let app_changed = log.app_name != last.app_name;
                
                app_changed || gap > 90
            } else {
                false
            };

            if should_break {
                if !current_buffer.is_empty() {
                    Self::flush_session(db, &current_buffer)?;
                    current_buffer.clear();
                }
            }

            current_buffer.push(log);
        }

        // Do NOT flush the final buffer. 
        // We leave it. Next time we run `get_logs_since(last_end)`, we will fetch these logs again
        // plus any new ones, allowing the session to extend.
        // Once a break occurs (or app closes/restarts later), it will be flushed.
        
        Ok(())
    }

    fn flush_session(db: &AppDatabase, logs: &[&crate::db::ActivityLog]) -> Result<()> {
        if logs.is_empty() { return Ok(()); }

        let start = logs.first().unwrap();
        let end = logs.last().unwrap();
        
        // Duration: Time diff + 60s (inclusive of the last minute tick)
        let duration = (end.timestamp_utc - start.timestamp_utc) + 60;
        
        let mut found_distracting = false;
        let mut found_productive = false;
        let mut found_recovery = false;

        for log in logs {
            let cat = Category::classify(&log.app_name, &log.window_title, log.is_idle);
            match cat {
                Category::Distracting => found_distracting = true,
                Category::Productive => found_productive = true,
                Category::Recovery => found_recovery = true,
                _ => {}
            }
        }

        let final_category = if found_distracting {
            Category::Distracting
        } else if found_productive {
             Category::Productive
        } else if found_recovery {
             Category::Recovery
        } else {
             Category::classify(&start.app_name, &start.window_title, start.is_idle)
        };

        // FIXED: Use Local time for Date boundaries to match User Experience
        let start_dt = chrono::Local.timestamp_opt(start.timestamp_utc, 0).unwrap();
        let date_str = start_dt.format("%Y-%m-%d").to_string();

        db.insert_session(
            &date_str,
            &start.app_name,
            final_category.as_str(),
            start.timestamp_utc,
            end.timestamp_utc,
            duration,
        )?;

        info!("Session Sealed: {} [{}s] ({})", start.app_name, duration, final_category.as_str());

        Ok(())
    }

    pub fn evaluate_history(db: &AppDatabase) -> Result<()> {
        let today_local = chrono::Local::now().format("%Y-%m-%d").to_string();
        let pending_days = db.get_unsummarized_days(&today_local)?;

        for date in pending_days {
            Self::evaluate_day(db, &date)?;
        }
        Ok(())
    }

    fn evaluate_day(db: &AppDatabase, date: &str) -> Result<()> {
        info!("Evaluating day: {}", date);
        let totals = db.get_day_category_totals(date)?;

        let mut productive: i64 = 0;
        let mut distracting: i64 = 0;

        for (cat, duration) in totals {
            match cat.as_str() {
                "productive" | "Productive" => productive += duration,
                "distracting" | "Distracting" => distracting += duration,
                _ => {}
            }
        }

        let penalty = (distracting as f64 * 0.75) as i64;
        let effective_work = std::cmp::max(0, productive - penalty);

        // effective_work_seconds ≥ DAILY_TARGET (e.g. 4 hours)
        // AND distracting_seconds ≤ DISTRACTION_LIMIT (e.g. 20 mins? Spec says "defined in config", user image "4 hours" target)
        // User Requested: 2 Hours (120 mins)
        const DAILY_TARGET: i64 = 2 * 3600; // 2 Hours Deep Work
        const DISTRACTION_LIMIT: i64 = 45 * 60; // 45 Mins max

        let qualified = effective_work >= DAILY_TARGET && distracting <= DISTRACTION_LIMIT;

        let reason = if qualified {
            "Targets Met".to_string()
        } else {
            if effective_work < DAILY_TARGET {
                format!("Effective Work low ({}/{}s)", effective_work, DAILY_TARGET)
            } else {
                format!("Distraction too high ({}/{}s)", distracting, DISTRACTION_LIMIT)
            }
        };

        // For summary, we can default unused stats to 0 or calc them if needed. 
        // We focus on the strict ones.
        let summary = crate::db::DailySummary {
            date: date.to_string(),
            productive,
            neutral: 0,
            recovery: 0,
            distracting,
            idle: 0,
            effective_work,
            qualified,
            reason,
        };

        db.insert_daily_summary(&summary)?;

        let (mut current_streak, mut best_streak, last_eval_date_opt) = db.get_streak_info()?;
        
        if let Some(last_date) = last_eval_date_opt {
            if last_date == date { return Ok(()); }
        }

        if qualified {
            current_streak += 1;
            if current_streak > best_streak { best_streak = current_streak; }
        } else {
            current_streak = 0;
        }

        db.update_streak(current_streak, best_streak, date)?;
        info!("Day {} Finalized. Qualified: {}. Streak: {}", date, qualified, current_streak);

        Ok(())
    }

    pub fn update_live_stats(db: &AppDatabase) -> Result<()> {
        let today = chrono::Local::now().format("%Y-%m-%d").to_string();
        
        // 1. Committed Sessions (Completed)
        let totals = db.get_day_category_totals(&today)?;
        
        let mut productive: i64 = 0;
        let mut distracting: i64 = 0;
        
        for (cat, duration) in totals {
            match cat.to_lowercase().as_str() {
                "productive" => productive += duration,
                "distracting" => distracting += duration,
                _ => {}
            }
        }
        
        // 2. Pending Logs (Active Session)
        let last_session_end = db.get_last_session_end_time()?.unwrap_or(0);
        let pending_logs = db.get_logs_since(last_session_end)?;
        
        for log in pending_logs {
             let log_dt = chrono::Local.timestamp_opt(log.timestamp_utc, 0).unwrap();
             if log_dt.format("%Y-%m-%d").to_string() == today {
                 let cat = Category::classify(&log.app_name, &log.window_title, log.is_idle);
                 match cat {
                     Category::Productive => productive += 60,
                     Category::Distracting => distracting += 60,
                     _ => {}
                 }
             }
        }

        // Strict Formula
        let penalty = (distracting as f64 * 0.75) as i64;
        let effective = std::cmp::max(0, productive - penalty);
        
        db.update_intraday_stats(&today, productive, distracting, effective)?;
        
        Ok(())
    }

    /// Strict Enforcement: "Block every other app until 120 mins"
    pub fn enforce_policy(db: &AppDatabase, current_app: &str, current_title: &str) -> Result<()> {
        let today = chrono::Local::now().format("%Y-%m-%d").to_string();
        
        // Check current status
        // Efficient way: read intraday_stats directly (it was just updated)
        // But for strictness let's assume we called update_live_stats first.
        
        // Using raw query for speed
        let mut stmt = db.conn.prepare("SELECT effective_work_seconds FROM intraday_stats WHERE id = 1 AND date = ?1")?;
        let mut rows = stmt.query([&today])?;
        
        let effective_work: i64 = if let Some(row) = rows.next()? {
            row.get(0)?
        } else {
            0
        };

        const TARGET: i64 = 2 * 3600; // 120 Minutes

        if effective_work < TARGET {
            // "Until that (120m), every other app (Distracting) will be blocked"
            let cat = Category::classify(current_app, current_title, false);
            
            if cat == Category::Distracting {
                info!("ENFORCEMENT: Blocking {} (Target not met: {}/{})", current_app, effective_work, TARGET);
                
                // Kill Process
                use std::process::Command;
                use std::os::windows::process::CommandExt;
                const CREATE_NO_WINDOW: u32 = 0x08000000;
                
                let _ = Command::new("taskkill")
                    .args(&["/F", "/IM", current_app])
                    .creation_flags(CREATE_NO_WINDOW)
                    .output();
            }
        }
        
        Ok(())
    }
}
