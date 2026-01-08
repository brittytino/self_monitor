package com.selfmonitor.app.ui

import androidx.compose.foundation.background
import androidx.compose.foundation.border
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
@Composable
fun DashboardScreen(viewModel: DashboardViewModel) {
    val state by viewModel.state.collectAsState()
    
    MaterialTheme(
        colorScheme = androidx.compose.material3.darkColorScheme(
            background = Color(0xFF121212),
            surface = Color(0xFF1E1E1E),
            onBackground = Color.White,
            onSurface = Color.White
        )
    ) {
        Column(
            modifier = Modifier
                .fillMaxSize()
                .background(MaterialTheme.colorScheme.background)
                .padding(24.dp),
            horizontalAlignment = Alignment.CenterHorizontally
        ) {
            if (state.startupError != null) {
                if (state.startupError == "MISSING_SECRET") {
                    var secretInput by androidx.compose.runtime.remember { androidx.compose.runtime.mutableStateOf("") }
                    Column(
                        modifier = Modifier.fillMaxSize(),
                        horizontalAlignment = Alignment.CenterHorizontally,
                        verticalArrangement = Arrangement.Center
                    ) {
                        Text("SETUP REQUIRED", color = Color.Yellow, style = MaterialTheme.typography.headlineMedium)
                        Spacer(modifier = Modifier.height(16.dp))
                        Text("Enter Neon Connection String:", color = Color.Gray)
                        Spacer(modifier = Modifier.height(8.dp))
                        androidx.compose.material3.OutlinedTextField(
                            value = secretInput,
                            onValueChange = { secretInput = it },
                            modifier = Modifier.fillMaxWidth(),
                            textStyle = androidx.compose.ui.text.TextStyle(color = Color.White)
                        )
                        Spacer(modifier = Modifier.height(16.dp))
                        androidx.compose.material3.Button(onClick = { viewModel.setSecretAndRetry(secretInput) }) {
                            Text("SAVE SECRETS")
                        }
                    }
                } else {
                    Box(modifier = Modifier.fillMaxSize(), contentAlignment = Alignment.Center) {
                        Text("SYSTEM HALTED\n${state.startupError}", color = Color.Red, style = MaterialTheme.typography.headlineMedium)
                    }
                }
                return@Column
            }

            Text(
                "DISCIPLINE MONITOR",
                style = MaterialTheme.typography.labelSmall,
                color = Color.Gray,
            )
            Text(
                state.syncStatus,
                style = MaterialTheme.typography.labelSmall,
                color = if (state.syncStatus.startsWith("Success")) Color.Green else Color.Gray,
                modifier = Modifier.padding(bottom = 32.dp)
            )
            
            StatusCard(state.todayVerdict)
            
            Spacer(modifier = Modifier.height(48.dp))
            
            Row(
                modifier = Modifier.fillMaxWidth(),
                horizontalArrangement = Arrangement.SpaceEvenly
            ) {
                StatItem("DEEP WORK", state.workDuration, Color(0xFF4CAF50))
                StatItem("DISTRACTION", state.distractionDuration, Color(0xFFE53935))
            }
            
            Spacer(modifier = Modifier.height(48.dp))
            
            Text("CONSISTENCY", style = MaterialTheme.typography.labelMedium, modifier = Modifier.align(Alignment.Start))
            Spacer(modifier = Modifier.height(16.dp))
            HeatmapGrid() // Placeholder for visual grid
            
            Spacer(modifier = Modifier.weight(1f))
            
            ManualInputSection()
        }
    }
}

@Composable
fun StatItem(label: String, value: String, color: Color) {
    Column(horizontalAlignment = Alignment.CenterHorizontally) {
        Text(value, style = MaterialTheme.typography.headlineLarge, color = color)
        Text(label, style = MaterialTheme.typography.labelSmall, color = Color.Gray)
    }
}

@Composable
fun HeatmapGrid() {
    Row(horizontalArrangement = Arrangement.spacedBy(4.dp)) {
        repeat(7) { 
            Box(
                modifier = Modifier
                    .size(24.dp)
                    .background(Color(0xFF333333)) // Placeholder gray
            )
        }
    }
}

@Composable
fun ManualInputSection() {
    Column(
        modifier = Modifier
            .fillMaxWidth()
            .background(MaterialTheme.colorScheme.surface, shape = androidx.compose.foundation.shape.RoundedCornerShape(8.dp))
            .padding(16.dp)
    ) {
        Text("EVENING REFLECTION", style = MaterialTheme.typography.labelMedium, color = Color.Gray)
        Spacer(modifier = Modifier.height(16.dp))
        Row(horizontalArrangement = Arrangement.spacedBy(8.dp)) {
            LogButton("LeetCode")
            LogButton("No Sugar") 
            LogButton("Diet")
        }
    }
}

@Composable
fun LogButton(text: String) {
    androidx.compose.material3.OutlinedButton(
        onClick = {},
        shape = androidx.compose.foundation.shape.RoundedCornerShape(4.dp)
    ) {
        Text(text, style = MaterialTheme.typography.bodyMedium)
    }
}

@Composable
fun StatusCard(verdict: com.selfmonitor.core.domain.DailyVerdict) {
    val (color, text) = when(verdict) {
        com.selfmonitor.core.domain.DailyVerdict.GREEN -> Color(0xFF4CAF50) to "DISCIPLINED"
        com.selfmonitor.core.domain.DailyVerdict.YELLOW -> Color(0xFFFFC107) to "AVERAGE"
        com.selfmonitor.core.domain.DailyVerdict.RED -> Color(0xFFE53935) to "FAILED"
    }
    
    Box(
        modifier = Modifier
            .size(200.dp)
            .background(color.copy(alpha = 0.1f), shape = androidx.compose.foundation.shape.CircleShape)
            .border(2.dp, color, androidx.compose.foundation.shape.CircleShape),
        contentAlignment = Alignment.Center
    ) {
        Column(horizontalAlignment = Alignment.CenterHorizontally) {
            Text(text, style = MaterialTheme.typography.headlineMedium, color = color)
            Text("TODAY", style = MaterialTheme.typography.labelSmall, color = color.copy(alpha = 0.8f))
        }
    }
}
