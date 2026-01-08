package com.selfmonitor.app.services

import android.content.BroadcastReceiver
import android.content.Context
import android.content.Intent
import android.util.Log

class PackageReceiver : BroadcastReceiver() {
    override fun onReceive(context: Context, intent: Intent) {
        val packageName = intent.data?.schemeSpecificPart
        when (intent.action) {
            Intent.ACTION_PACKAGE_ADDED -> {
                Log.d("PackageReceiver", "Installed: $packageName")
                // TODO: Check if this new app is allowed. If not, trigger block/uninstall flow.
            }
            Intent.ACTION_PACKAGE_REMOVED -> {
                Log.d("PackageReceiver", "Removed: $packageName")
            }
            Intent.ACTION_PACKAGE_REPLACED -> {
                Log.d("PackageReceiver", "Updated: $packageName")
            }
        }
    }
}
