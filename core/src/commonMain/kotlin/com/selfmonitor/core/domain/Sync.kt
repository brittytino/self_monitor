import com.selfmonitor.core.utils.SecretManager
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.StateFlow

enum class SyncStatus { IDLE, PUSHING, PULLING, ERROR }

interface SyncManager {
    val status: StateFlow<SyncStatus>
    suspend fun syncNow()
}

interface RemoteDataSource {
    suspend fun pushEvents(events: List<RawEvent>, dbUrl: String)
    suspend fun pullRules(lastSync: Instant, dbUrl: String): List<AppRule>
}

expect class RemoteDataSourceFactory {
    fun create(): RemoteDataSource?
}

class DefaultSyncManager(
    private val database: com.selfmonitor.db.AppDatabase,
    private val remoteDataSource: RemoteDataSource?
) : SyncManager {
    
    private val _status = MutableStateFlow(SyncStatus.IDLE)
    override val status: StateFlow<SyncStatus> = _status
    
    // SecretManager used internally by Factory, but SyncManager needs to know if sync IS possible.
    // Actually, if remoteDataSource is null, sync is disabled.
    
    override suspend fun syncNow() {
        if (remoteDataSource == null) {
             logState("Sync Disabled: No Credentials")
             return
        }
        
        if (_status.value != SyncStatus.IDLE) return
        
        // Credentials check is implicitly done by factory returning non-null, 
        // OR we might need url here if pushEvents takes it.
        // Factory could configure the source with the URL.
        // Let's assume RemoteDataSource implementation holds the URL or we pass it?
        // The previous `RemoteDataSource` interface had `dbUrl` in methods.
        // If we want to hide it, `RemoteDataSource` impl should have it.
        // But `SecretManager` is used to get it.
        // Let's stick to the prompt: Factory creates it. Factory reads secret.
        // So `RemoteDataSource` implementation might already have the URL or we fetch it again?
        // Simpler: `RemoteDataSource` methods DON'T need dbUrl if the impl has it.
        // BUT `PostgresDataSource` in previous turn took `dbUrl` in methods.
        // I will refactor `PostgresDataSource` to take `dbUrl` in constructor soon.
        // For now, let's keep `dbUrl` in methods if that's what `SyncManager` uses?
        // No, `DefaultSyncManager` was looking up secret.
        // Let's clean this up:
        // SyncManager uses separate SecretManager to get URL to pass to methods.
        
        val dbUrl = com.selfmonitor.core.utils.SecretManager().getDatabaseUrl()
        if (dbUrl.isNullOrBlank()) {
             logState("Error: Missing DATABASE_URL") // Should have been null factory? 
             // Double check helpful for runtime clearing of secrets.
             return
        }

        try {
            if (!isNetworkAvailable()) return
            // ...
            // remoteDataSource.pushEvents(..., dbUrl)

            
            // 1. PUSH
            _status.value = SyncStatus.PUSHING
            
            // Push Raw Events
            val lastPushEvent = getSyncTimestamp("raw_event")
            val pendingEvents = database.rawEventQueries.selectAllRawEvents().executeAsList()
                .filter { it.timestamp > lastPushEvent }
            
            if (pendingEvents.isNotEmpty()) {
                remoteDataSource.pushEvents(pendingEvents, dbUrl)
                updateSyncState("raw_event", pushed = kotlinx.datetime.Clock.System.now().toEpochMilliseconds())
            }
            
            // 2. PULL
            _status.value = SyncStatus.PULLING
             val lastPullRules = getSyncTimestamp("app_rule", pulled = true)
             val lastSyncInstant = Instant.fromEpochMilliseconds(lastPullRules)
             
             val remoteRules = remoteDataSource.pullRules(lastSyncInstant, dbUrl)
             remoteRules.forEach { 
                database.appRuleQueries.upsertAppRule(it.pkgNamePattern, it.category)
             }
             
             updateSyncState("app_rule", pulled = kotlinx.datetime.Clock.System.now().toEpochMilliseconds())

            _status.value = SyncStatus.IDLE
            logState("Success: ${kotlinx.datetime.Clock.System.now()}")
            
        } catch (e: Exception) {
            _status.value = SyncStatus.ERROR
            logState("Failed: ${e.message}")
            _status.value = SyncStatus.IDLE
        }
    }
    
    private fun getSyncTimestamp(entity: String, pulled: Boolean = false): Long {
        val state = database.syncStateQueries.getSyncState(entity).executeAsOneOrNull()
        return if (pulled) state?.last_pulled_at ?: 0L else state?.last_pushed_at ?: 0L
    }
    
    private fun updateSyncState(entity: String, pushed: Long? = null, pulled: Long? = null) {
        val current = database.syncStateQueries.getSyncState(entity).executeAsOneOrNull()
        database.syncStateQueries.updateSyncState(
            entity_name = entity,
            last_pushed_at = pushed ?: current?.last_pushed_at,
            last_pulled_at = pulled ?: current?.last_pulled_at,
            pending_push_count = 0 // In real impl, we'd verify
        )
    }

    private fun logState(msg: String) {
        database.systemConfigQueries.setConfig("sync_status", msg)
    }
    
    private fun isNetworkAvailable(): Boolean = true 
}
