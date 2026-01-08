package com.selfmonitor.core.domain

import com.selfmonitor.core.utils.SecretManager

actual class RemoteDataSourceFactory {
    actual fun create(): RemoteDataSource? {
        val secretManager = SecretManager()
        val url = secretManager.getDatabaseUrl()
        if (url.isNullOrBlank()) return null
        
        // Validate minimally? (e.g. check prefix)
        if (!url.startsWith("postgres")) return null // Basic sanity
        
        return PostgresDataSource()
    }
}
