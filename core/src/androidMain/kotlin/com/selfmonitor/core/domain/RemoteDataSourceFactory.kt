package com.selfmonitor.core.domain

import com.selfmonitor.core.utils.SecretManager

actual class RemoteDataSourceFactory {
    actual fun create(): RemoteDataSource? {
        val secretManager = SecretManager()
        val url = secretManager.getDatabaseUrl()
        if (url.isNullOrBlank()) return null
        
        return PostgresDataSource()
    }
}
