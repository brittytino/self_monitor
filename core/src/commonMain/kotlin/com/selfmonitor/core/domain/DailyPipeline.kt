package com.selfmonitor.core.domain

import kotlinx.datetime.Clock
import kotlinx.datetime.TimeZone
import kotlinx.datetime.toLocalDateTime

class DailyPipeline(
    private val database: com.selfmonitor.db.AppDatabase,
    private val sessionizer: Sessionizer,
    private val rulesEngine: RulesEngine,
    private val syncManager: SyncManager
) {
    // Should be called ideally at 9PM or on app open if missed
    suspend fun runEndOfDayPipeline() {
        val now = Clock.System.now()
        val today = now.toLocalDateTime(TimeZone.currentSystemDefault()).date
        
        // 1. Idempotency Check
        val existingLog = database.dailyLogQueries.getDailyLog(today.toString()).executeAsOneOrNull()
        if (existingLog != null) {
             // Already ran for today? 
             // Logic might allow re-running to update if new data came in, usually safe.
        }
        
        // 2. Aggregate Sessions
        // Fetch all raw events for today? Or all unprocessed?
        // Detailed implementation would filter by time range.
        
        // 3. Rule Evaluation (Mocked inputs or fetched)
        // val verdict = rulesEngine.evaluateDay(...)
        
        // 4. Persist
        // database.dailyLogQueries.insertDailyLog(...)
        
        // 5. Trigger Sync
        syncManager.syncNow()
    }
}
