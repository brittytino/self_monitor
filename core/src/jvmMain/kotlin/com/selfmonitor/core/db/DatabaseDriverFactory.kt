package com.selfmonitor.core.db

import app.cash.sqldelight.db.SqlDriver
import app.cash.sqldelight.driver.jdbc.sqlite.JdbcSqliteDriver
import com.selfmonitor.db.AppDatabase
import java.io.File

actual class DatabaseDriverFactory {
    actual fun createDriver(): SqlDriver {
        // Point to the same file Rust writes to: c:\dev\self_monitor\data.db
        // In dev, we assume CWD or explicit path.
        val dbFile = File("data.db")
        val url = "jdbc:sqlite:${dbFile.absolutePath}"
        val driver = JdbcSqliteDriver(url)
        
        // Create tables if not exist (Rust might have done it, but safe to check)
        // Note: JdbcSqliteDriver doesn't auto-create schema like Android does usually unless handled.
        // We'll trust Rust or run migration if needed.
        if (!dbFile.exists()) {
             AppDatabase.Schema.create(driver)
        }
        // Enable WAL
        driver.execute(null, "PRAGMA journal_mode = WAL;", 0)
        driver.execute(null, "PRAGMA synchronous = NORMAL;", 0)
        
        return driver
    }
}
