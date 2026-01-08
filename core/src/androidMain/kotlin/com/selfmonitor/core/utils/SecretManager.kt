package com.selfmonitor.core.utils

import android.content.Context
import androidx.security.crypto.EncryptedSharedPreferences
import androidx.security.crypto.MasterKey
import com.selfmonitor.app.SelfMonitorApp

actual class SecretManager {
    actual fun getDatabaseUrl(): String? {
        val masterKey = MasterKey.Builder(SelfMonitorApp.context)
            .setKeyScheme(MasterKey.KeyScheme.AES256_GCM)
            .build()

        val sharedPreferences = EncryptedSharedPreferences.create(
            SelfMonitorApp.context,
            "secret_shared_prefs",
            masterKey,
            EncryptedSharedPreferences.PrefKeyEncryptionScheme.AES256_SIV,
            EncryptedSharedPreferences.PrefValueEncryptionScheme.AES256_GCM
        )
        return sharedPreferences.getString("DATABASE_URL", null)
    }

    actual fun setDatabaseUrl(url: String) {
        val masterKey = MasterKey.Builder(SelfMonitorApp.context)
            .setKeyScheme(MasterKey.KeyScheme.AES256_GCM)
            .build()

        val sharedPreferences = EncryptedSharedPreferences.create(
            SelfMonitorApp.context,
            "secret_shared_prefs",
            masterKey,
            EncryptedSharedPreferences.PrefKeyEncryptionScheme.AES256_SIV,
            EncryptedSharedPreferences.PrefValueEncryptionScheme.AES256_GCM
        )
        sharedPreferences.edit().putString("DATABASE_URL", url).apply()
    }
}
