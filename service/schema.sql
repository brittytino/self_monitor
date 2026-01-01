-- Self Monitor Database Schema
-- Enforces: Append-only logs, Merged sessions, Frozen daily summaries, Single streak row.

-- 1. ACTIVITY LOGS (Append-only, Raw Observation)
CREATE TABLE IF NOT EXISTS activity_logs (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    timestamp_utc INTEGER NOT NULL,
    app_name TEXT NOT NULL,
    window_title TEXT NOT NULL,
    is_idle BOOLEAN NOT NULL CHECK (is_idle IN (0, 1))
);

-- Index for efficient querying by time
CREATE INDEX IF NOT EXISTS idx_logs_time ON activity_logs(timestamp_utc);


-- 2. ACTIVITY SESSIONS (Derived once, Merged)
CREATE TABLE IF NOT EXISTS activity_sessions (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    date TEXT NOT NULL, -- YYYY-MM-DD
    app_name TEXT NOT NULL,
    category TEXT NOT NULL CHECK (category IN ('productive', 'neutral', 'recovery', 'distracting')),
    start_time_utc INTEGER NOT NULL,
    end_time_utc INTEGER NOT NULL,
    duration_seconds INTEGER NOT NULL,
    UNIQUE(start_time_utc, end_time_utc) -- Prevent duplicates
);

CREATE INDEX IF NOT EXISTS idx_sessions_date ON activity_sessions(date);


-- 3. DAILY SUMMARIES (Frozen Truth)
CREATE TABLE IF NOT EXISTS daily_summaries (
    date TEXT PRIMARY KEY, -- YYYY-MM-DD
    productive_seconds INTEGER NOT NULL DEFAULT 0,
    neutral_seconds INTEGER NOT NULL DEFAULT 0,
    recovery_seconds INTEGER NOT NULL DEFAULT 0,
    distracting_seconds INTEGER NOT NULL DEFAULT 0,
    idle_seconds INTEGER NOT NULL DEFAULT 0,
    effective_work_seconds INTEGER NOT NULL DEFAULT 0,
    qualified BOOLEAN NOT NULL CHECK (qualified IN (0, 1)),
    qualification_reason TEXT,
    finalized_at_utc INTEGER NOT NULL
);


-- 4. STREAKS (Single Authority Row)
CREATE TABLE IF NOT EXISTS streaks (
    id INTEGER PRIMARY KEY CHECK (id = 1), -- Singleton
    current_streak INTEGER NOT NULL DEFAULT 0,
    best_streak INTEGER NOT NULL DEFAULT 0,
    last_evaluated_date TEXT -- YYYY-MM-DD
);

-- Initialize streak row if not exists
INSERT OR IGNORE INTO streaks (id, current_streak, best_streak, last_evaluated_date)
VALUES (1, 0, 0, NULL);
