package com.selfmonitor.app.services

import android.accessibilityservice.AccessibilityService
import android.view.accessibility.AccessibilityEvent
import android.content.Intent
import android.util.Log
import com.selfmonitor.app.ui.LauncherActivity

class EnforcementService : AccessibilityService() {

    private val rulesEngine = com.selfmonitor.core.domain.DefaultRulesEngine()
    // In a real app, inject this via DI (Koin/Hilt). For simple KMP, manual is okay.
    private val database by lazy { 
        val driver = com.selfmonitor.core.db.DatabaseDriverFactory().createDriver()
        com.selfmonitor.db.AppDatabase(driver)
    }

    override fun onServiceConnected() {
        super.onServiceConnected()
        Log.d("EnforcementService", "Service Connected")
    }

    override fun onAccessibilityEvent(event: AccessibilityEvent?) {
        if (event == null) return

        if (event.eventType == AccessibilityEvent.TYPE_WINDOW_STATE_CHANGED) {
            val packageName = event.packageName?.toString() ?: return
            
            Log.d("EnforcementService", "App opened: $packageName")
            
            // Query DB for rules (in background thread ideally)
            // For now, we do a quick synchronous check or minimal breakdown
            // val category = rulesEngine.classifyApp(packageName, database.appRuleQueries.getAllRules().executeAsList())
            
            // NOTE: For this demo, we keep the strict blacklist as a fallback 
            // until the DB is populated.
            if (shouldBlock(packageName)) {
                performGlobalAction(GLOBAL_ACTION_HOME)
            }
            
            checkSecurity(packageName)
        }
    }

    override fun onInterrupt() {
        Log.d("EnforcementService", "Service Interrupted")
    }
    
    private fun checkSecurity(currentPackage: String) {
        // HARDENING: Detect if a rival launcher is active
        val knownLaunchers = listOf("com.google.android.apps.nexuslauncher", "com.teslacoilsw.launcher", "com.sec.android.app.launcher")
        if (knownLaunchers.contains(currentPackage)) {
            // User escaped to a different launcher!
            Log.w("EnforcementService", "SECURITY: Non-compliant launcher detected: $currentPackage")
            // Strict Enforcement: Force back to our launcher (if we were default, pushing HOME works. If not, we might be stuck).
            // performGlobalAction(GLOBAL_ACTION_HOME) // This might loop if we aren't default.
            
            // Launch our Launcher Activity explicitly
             val intent = packageManager.getLaunchIntentForPackage(packageName)
             intent?.addFlags(android.content.Intent.FLAG_ACTIVITY_NEW_TASK)
             startActivity(intent)
        }
    }
    
    private fun shouldBlock(packageName: String): Boolean {
        // 1. Query DB for explicit rule
        try {
            val rule = database.appRuleQueries.getRuleForApp(packageName).executeAsOneOrNull()
            if (rule != null) {
                return rule.category == com.selfmonitor.core.domain.AppCategory.DISTRACTION
            }
        } catch (e: Exception) {
            Log.e("EnforcementService", "DB Read Failed: ${e.message}")
        }

        // 2. Fallback to hardcoded blacklist for safety/bootstrapping
        val blacklist = listOf("com.instagram.android", "com.facebook.katana", "com.zhiliaoapp.musically")
        return blacklist.contains(packageName)
    }
}
