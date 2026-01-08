package com.selfmonitor.app.ui

import com.selfmonitor.core.domain.DailyLog
import com.selfmonitor.core.domain.DailyVerdict
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.launch

data class DashboardState(
    val startupError: String? = null, // Null = Healthy
    val todayVerdict: DailyVerdict = DailyVerdict.YELLOW,
    val workDuration: String = "0h 0m",
    val distractionDuration: String = "0h 0m",
    val syncStatus: String = "Idle",
    val recentHistory: List<DailyLog> = emptyList()
)

class DashboardViewModel(
    private val scope: CoroutineScope,
    private val database: com.selfmonitor.db.AppDatabase,
    private val syncManager: com.selfmonitor.core.domain.SyncManager
) {
    private val _state = MutableStateFlow(DashboardState())
    val state: StateFlow<DashboardState> = _state
    
    private val secretManager = com.selfmonitor.core.utils.SecretManager()
    private val sessionizer = com.selfmonitor.core.domain.DefaultSessionizer()
    private val rulesEngine = com.selfmonitor.core.domain.DefaultRulesEngine()


    init {
        checkHealth()
    }
    
    private fun checkHealth() {
        try {
             // 1. DB Access Check
             database.systemConfigQueries.getConfig("health_check").executeAsOneOrNull()
             
             // 2. Secret Check
             val url = secretManager.getDatabaseUrl()
             if (url.isNullOrBlank()) {
                 _state.value = _state.value.copy(startupError = "MISSING_SECRET")
                 return
             }
             
             loadData()
        } catch (e: Exception) {
            _state.value = _state.value.copy(startupError = "System Integrity Failed: ${e.message}")
        }
    }
    
    fun setSecretAndRetry(url: String) {
        try {
            secretManager.setDatabaseUrl(url)
            _state.value = _state.value.copy(startupError = null)
            checkHealth()
        } catch (e: Exception) {
            _state.value = _state.value.copy(startupError = "Failed to save secret: ${e.message}")
        }
    }
    
    fun refresh() {
        loadData()
    }
    
    private fun loadData() {
        scope.launch {
            try {
                // ... Existing Logic ...
                
                // Fetch Sync Status
                val syncStat = database.systemConfigQueries.getConfig("sync_status").executeAsOneOrNull()?.value ?: "Idle"
                
                _state.value = DashboardState(
                    startupError = null,
                    todayVerdict = verdict,
                    workDuration = formatDuration(totalWork),
                    distractionDuration = formatDuration(totalDistraction),
                    syncStatus = syncStat,
                    recentHistory = emptyList() 
                )
            } catch (e: Exception) {
                _state.value = _state.value.copy(startupError = "Data Load Failed: ${e.message}")
            }
        }
    }
    
    private fun formatDuration(seconds: Long): String {
        val hours = seconds / 3600
        val mins = (seconds % 3600) / 60
        return "${hours}h ${mins}m"
    }
}
