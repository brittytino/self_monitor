package com.selfmonitor.core.utils

expect class SecretManager {
    fun getDatabaseUrl(): String?
    fun setDatabaseUrl(url: String)
}
