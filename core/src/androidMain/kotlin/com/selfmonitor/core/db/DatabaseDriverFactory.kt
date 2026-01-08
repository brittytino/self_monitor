package com.selfmonitor.core.db

import android.content.Context
import app.cash.sqldelight.db.SqlDriver
import app.cash.sqldelight.driver.android.AndroidSqliteDriver
import com.selfmonitor.db.AppDatabase

import com.selfmonitor.app.SelfMonitorApp

import androidx.sqlite.db.SupportSQLiteDatabase

actual class DatabaseDriverFactory {
    actual fun createDriver(): SqlDriver {
        return AndroidSqliteDriver(
            schema = AppDatabase.Schema, 
            context = SelfMonitorApp.context, 
            name = "AppDatabase.db",
            callback = object : AndroidSqliteDriver.Callback(AppDatabase.Schema) {
                override fun onOpen(db: SupportSQLiteDatabase) {
                    db.execSQL("PRAGMA journal_mode=WAL;")
                    db.execSQL("PRAGMA synchronous=NORMAL;")
                    super.onOpen(db)
                }
            }
        )
    }
}
