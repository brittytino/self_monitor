package com.selfmonitor.app.ui

import android.os.Bundle
import androidx.activity.ComponentActivity
import androidx.activity.compose.setContent
import androidx.compose.material3.Text
import androidx.compose.material3.MaterialTheme
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier

class LauncherActivity : ComponentActivity() {
    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        // Dependency Injection Root
        val driver = com.selfmonitor.core.db.DatabaseDriverFactory().createDriver()
        val database = com.selfmonitor.db.AppDatabase(driver)
        
        // Factory uses SecretManager internally
        val remoteSource = com.selfmonitor.core.domain.RemoteDataSourceFactory().create()
        val syncManager = com.selfmonitor.core.domain.DefaultSyncManager(database, remoteSource)
        
        val viewModel = com.selfmonitor.app.ui.DashboardViewModel(
            scope = androidx.lifecycle.lifecycleScope,
            database = database,
            syncManager = syncManager
        )

        super.onCreate(savedInstanceState)
        setContent {
            com.selfmonitor.app.ui.DashboardScreen(viewModel)
        }
    }
    
    @Override
    override fun onBackPressed() {
        // Do nothing - we are the launcher
    }
}

@Composable
fun LauncherScreen() {
    Text(text = "Self Monitor Launcher")
    // TODO: List allowed apps
}
