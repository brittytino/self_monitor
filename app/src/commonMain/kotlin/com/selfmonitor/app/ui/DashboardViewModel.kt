package com.selfmonitor.app.ui

import com.selfmonitor.core.domain.DailyLog
import com.selfmonitor.core.domain.DailyVerdict
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.launch

data class DashboardState(
    val todayVerdict: DailyVerdict = DailyVerdict.YELLOW,
    val workDuration: String = "0h 0m",
    val distractionDuration: String = "0h 0m",
    val recentHistory: List<DailyLog> = emptyList()
)

class DashboardViewModel(
    private val scope: CoroutineScope
    // private val repo: Repository
) {
    private val _state = MutableStateFlow(DashboardState())
    val state: StateFlow<DashboardState> = _state
    
    init {
        loadData()
    }
    
    private fun loadData() {
        scope.launch {
            // Simulate data loading from SQLite
            _state.value = DashboardState(
                todayVerdict = DailyVerdict.YELLOW,
                workDuration = "3h 15m",
                distractionDuration = "10m",
                recentHistory = listOf() // Populate from DB
            )
        }
    }
}
