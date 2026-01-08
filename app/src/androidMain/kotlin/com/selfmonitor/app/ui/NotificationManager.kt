package com.selfmonitor.app.ui

// Simple abstraction for notifications
// In a real Android app, this would wrap NotificationManager
class NotificationManager {
    fun sendVerdictNotification(verdict: com.selfmonitor.core.domain.DailyVerdict) {
        // TODO: Implement actual NotificationChannel and Builder
        println("SENDING NOTIFICATION: Today's verdict is $verdict")
    }
    
    fun sendViolationAlert(appName: String) {
        println("SENDING NOTIFICATION: Violation detected for $appName")
    }
}
