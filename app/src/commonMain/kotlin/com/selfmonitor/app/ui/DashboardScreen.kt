package com.selfmonitor.app.ui

import androidx.compose.foundation.background
import androidx.compose.foundation.layout.*
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.runtime.collectAsState
import androidx.compose.runtime.getValue
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.unit.dp

@Composable
fun DashboardScreen(viewModel: DashboardViewModel) {
    val state by viewModel.state.collectAsState()
    
    Column(
        modifier = Modifier.fillMaxSize().padding(16.dp),
        horizontalAlignment = Alignment.CenterHorizontally
    ) {
        StatusCard(state.todayVerdict)
        
        Spacer(modifier = Modifier.height(24.dp))
        
        Text("Work: ${state.workDuration}", style = MaterialTheme.typography.headlineMedium)
        Text("Distraction: ${state.distractionDuration}", style = MaterialTheme.typography.bodyLarge, color = Color.Red)
        
        Spacer(modifier = Modifier.height(24.dp))
        
        // Heatmap placeholder
        Text("History Heatmap (Last 30 Days)")
        
        Spacer(modifier = Modifier.height(24.dp))
        
        ManualInputSection()
    }
}

@Composable
fun ManualInputSection() {
    androidx.compose.material3.Card(modifier = Modifier.fillMaxWidth()) {
        Column(modifier = Modifier.padding(16.dp)) {
            Text("Daily Check-in (One-time)", style = MaterialTheme.typography.titleMedium)
            // Functionality to be wired up to ViewModel
            androidx.compose.material3.Button(onClick = { /* TODO */ }) { Text("LeetCode Solved") }
            androidx.compose.material3.Button(onClick = { /* TODO */ }) { Text("No Sugar") }
            androidx.compose.material3.Button(onClick = { /* TODO */ }) { Text("Diet Followed") }
        }
    }
}

@Composable
fun StatusCard(verdict: com.selfmonitor.core.domain.DailyVerdict) {
    val color = when(verdict) {
        com.selfmonitor.core.domain.DailyVerdict.GREEN -> Color.Green
        com.selfmonitor.core.domain.DailyVerdict.YELLOW -> Color.Yellow
        com.selfmonitor.core.domain.DailyVerdict.RED -> Color.Red
    }
    
    Box(
        modifier = Modifier
            .size(200.dp)
            .background(color)
            .padding(16.dp),
        contentAlignment = Alignment.Center
    ) {
        Text("VERDICT: $verdict", style = MaterialTheme.typography.displayMedium)
    }
}
