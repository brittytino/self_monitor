package com.selfmonitor.app

import android.app.Application
import android.content.Context

class SelfMonitorApp : Application() {
    companion object {
        lateinit var context: Context
    }

    override fun onCreate() {
        super.onCreate()
        context = this
    }
}
