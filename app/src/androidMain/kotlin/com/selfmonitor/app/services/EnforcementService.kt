package com.selfmonitor.app.services

import android.accessibilityservice.AccessibilityService
import android.view.accessibility.AccessibilityEvent
import android.content.Intent
import android.util.Log
import com.selfmonitor.app.ui.LauncherActivity

class EnforcementService : AccessibilityService() {

    override fun onServiceConnected() {
        super.onServiceConnected()
        Log.d("EnforcementService", "Service Connected")
    }

    override fun onAccessibilityEvent(event: AccessibilityEvent?) {
        if (event == null) return

        if (event.eventType == AccessibilityEvent.TYPE_WINDOW_STATE_CHANGED) {
            val packageName = event.packageName?.toString() ?: return
            val className = event.className?.toString()
            
            Log.d("EnforcementService", "App opened: $packageName")
            
            // TODO: Consult Rules Engine (Shared Core)
            // For now, hardcode a block check for demo/verification
            if (shouldBlock(packageName)) {
                performGlobalAction(GLOBAL_ACTION_HOME)
                // Optionally show a blocking overlay or toast
                // startActivity(Intent(this, LauncherActivity::class.java).addFlags(Intent.FLAG_ACTIVITY_NEW_TASK))
            }
        }
    }

    override fun onInterrupt() {
        Log.d("EnforcementService", "Service Interrupted")
    }
    
    private fun shouldBlock(packageName: String): Boolean {
        // Simple blacklist for now. Real implementation will query SQLite via Core.
        val blacklist = listOf("com.instagram.android", "com.facebook.katana", "com.zhiliaoapp.musically")
        return blacklist.contains(packageName)
    }
}
