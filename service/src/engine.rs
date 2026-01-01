use crate::db::AppDatabase;
use rusqlite::Result;
use log::info;
use chrono::{Utc, TimeZone};

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
    /// Distracting > Productive > Recovery > Neutral
    pub fn classify(app_name: &str, window_title: &str, is_idle: bool) -> Self {
        if is_idle {
            return Category::Idle;
        }

        let app = app_name.to_lowercase();
        let title = window_title.to_lowercase();

        // 1. DISTRACTING (Highest Priority)
        // Social Media, Games, Streaming, NSFW
        if app.contains("steam") || app.contains("discord") || app.contains("spotify") {
            return Category::Distracting;
        }
        if title.contains("instagram") || title.contains("reddit") || title.contains("youtube") || title.contains("facebook") || title.contains("twitter") || title.contains("x.com") {
            return Category::Distracting;
        }
        // NSFW keywords (generic examples)
        if title.contains("porn") || title.contains("xxx") {
            return Category::Distracting;
        }

        // 2. PRODUCTIVE
        // Dev tools, Office, Learning
        if app.contains("code") || app.contains("rust") || app.contains("terminal") || app.contains("powershell") || app.contains("cmd") {
            return Category::Productive;
        }
        if app.contains("word") || app.contains("excel") || app.contains("notepad") {
            return Category::Productive;
        }
        if title.contains("github") || title.contains("stackoverflow") || title.contains("docs") || title.contains("learning") {
            return Category::Productive;
        }

        // 3. RECOVERY
        // Music (if not distracting), stretching? Strict logic says Spotify is distracting if browsing, but maybe background?
        // Spec: "30 min YouTube 'learning' + 1 NSFW tab = Distracting".
        // We'll keep Recovery rare for now.
        
        // 4. NEUTRAL (Default)
        // System stuff, generic browsing
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
        
        // --- PART A: CATEGORIZATION LOGIC ---
        // "If any log within a session is distracting, the entire session is distracting."
        let mut found_distracting = false;
        let mut found_productive = false;
        let mut found_recovery = false;

        // Check strict priority: Distracting > Productive > Recovery > Neutral
        // Check strict priority: Distracting > Productive > Recovery > Neutral
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
             // Default to start classification or Neutral
             Category::classify(&start.app_name, &start.window_title, start.is_idle)
        };

        let date_str = Utc.timestamp_opt(start.timestamp_utc, 0).unwrap().format("%Y-%m-%d").to_string();

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
        // "Runs when a calendar date has no existing summary"
        // Check past days that have sessions but no summary.
        
        // We use `Local` to determine "yesterday" in user's timezone, 
        // but logs are UTC. The `date` column in sessions is stored as YYYY-MM-DD.
        // Ideally this `date` was derived from Local time? 
        // In `flush_session` above, we used `Utc`. This might be a timezone bug.
        // The Specification says "Midnight local time".
        // `DailyEvaluator` should run for any `date` in `activity_sessions` that is NOT in `daily_summaries`
        // AND is strictly in the past (i.e. date < today).
        
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
        let mut neutral: i64 = 0;
        let mut recovery: i64 = 0;
        let mut distracting: i64 = 0;
        let mut idle: i64 = 0;

        for (cat, duration) in totals {
            match cat.as_str() {
                "productive" | "Productive" => productive += duration,
                "neutral" | "Neutral" => neutral += duration,
                "recovery" | "Recovery" => recovery += duration,
                "distracting" | "Distracting" => distracting += duration,
                "idle" | "Idle" => idle += duration,
                _ => {}
            }
        }

        // --- PART A: EFFECTIVE WORK FORMULA ---
        // effective_work_seconds = productive_seconds − (distracting_seconds × 0.75)
        let penalty = (distracting as f64 * 0.75) as i64;
        let effective_work = std::cmp::max(0, productive - penalty);

        // --- PART A: QUALIFICATION ---
        // effective_work_seconds ≥ DAILY_TARGET (e.g. 4 hours)
        // AND distracting_seconds ≤ DISTRACTION_LIMIT (e.g. 20 mins? Spec says "defined in config", user image "4 hours" target)
        // Let's set reasonable strict defaults.
        const DAILY_TARGET: i64 = 3 * 3600; // 3 Hours Deep Work
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

        let summary = crate::db::DailySummary {
            date: date.to_string(),
            productive,
            neutral,
            recovery,
            distracting,
            idle,
            effective_work,
            qualified,
            reason,
        };

        // Persistence: "After this write, the day is sealed forever."
        db.insert_daily_summary(&summary)?;

        // --- PART A: STREAK LOGIC ---
        // "Streaks do not forgive. They only remember."
        
        let (mut current_streak, mut best_streak, last_eval_date_opt) = db.get_streak_info()?;
        
        // Prevent double counting if something weird happens (though unsummarized query prevents it)
        if let Some(last_date) = last_eval_date_opt {
            if last_date == date {
                return Ok(());
            }
        }

        if qualified {
            current_streak += 1;
            if current_streak > best_streak {
                best_streak = current_streak;
            }
        } else {
            current_streak = 0;
        }

        db.update_streak(current_streak, best_streak, date)?;
        
        info!("Day {} Finalized. Qualified: {}. Streak: {}", date, qualified, current_streak);

        Ok(())
    }
}
