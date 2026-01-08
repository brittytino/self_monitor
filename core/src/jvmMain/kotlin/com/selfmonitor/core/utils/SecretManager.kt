package com.selfmonitor.core.utils

import java.io.File
import java.util.Properties

actual class SecretManager {
    actual fun getDatabaseUrl(): String? {
        val envFile = File(".env")
        if (envFile.exists()) {
            val props = Properties()
            props.load(envFile.inputStream())
            return props.getProperty("DATABASE_URL")
        }
        return System.getenv("DATABASE_URL")
    }

    actual fun setDatabaseUrl(url: String) {
        val envFile = File(".env")
        val props = Properties()
        if (envFile.exists()) {
            props.load(envFile.inputStream())
        }
        props.setProperty("DATABASE_URL", url)
        props.store(envFile.outputStream(), "Updated by App")
    }
}
