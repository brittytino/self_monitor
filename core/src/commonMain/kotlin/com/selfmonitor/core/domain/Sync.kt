package com.selfmonitor.core.domain

import kotlinx.datetime.Instant

interface SyncManager {
    suspend fun pushEvents(events: List<RawEvent>)
    suspend fun pushDailyLogs(logs: List<DailyLog>)
    suspend fun pullLatestRules(lastSync: Instant): List<AppRule>
    
    // Offline-first handling
    suspend fun syncNow()
}

class DefaultSyncManager(
    private val localDb: Any, // Placeholder for SQLDelight DB
    private val remoteApi: Any // Placeholder for Ktor Client
) : SyncManager {
    
    override suspend fun syncNow() {
        // Logic:
        // 1. Check internet
        // 2. Load un-synced events locally
        // 3. Push to remove
        // 4. Mark synced
        // 5. Pull updates
    }
    
    override suspend fun pushEvents(events: List<RawEvent>) {
        // Implementation
    }
    
    override suspend fun pushDailyLogs(logs: List<DailyLog>) {
        // Implementation
    }
    
    override suspend fun pullLatestRules(lastSync: Instant): List<AppRule> {
        return emptyList()
    }
}
