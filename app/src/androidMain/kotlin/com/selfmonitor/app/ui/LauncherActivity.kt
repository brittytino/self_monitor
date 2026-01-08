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
        setContent {
            MaterialTheme {
                LauncherScreen()
            }
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
