package com.selfmonitor.core.domain

import kotlinx.datetime.Instant
import java.sql.DriverManager
import java.util.Properties
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.withContext

class PostgresDataSource : RemoteDataSource {
    override suspend fun pushEvents(events: List<RawEvent>, dbUrl: String) {
        withContext(Dispatchers.IO) {
            getConnection(dbUrl).use { conn ->
                conn.autoCommit = false
                val stmt = conn.prepareStatement(
                    "INSERT INTO raw_event (id, timestamp, device_id, app_pkg_name, window_title, is_idle) VALUES (?, ?, ?, ?, ?, ?) ON CONFLICT (id) DO NOTHING"
                )
                for (event in events) {
                    stmt.setString(1, event.id)
                    stmt.setLong(2, event.timestamp.toEpochMilliseconds())
                    stmt.setString(3, event.deviceId)
                    stmt.setString(4, event.appPkgName)
                    stmt.setString(5, event.windowTitle)
                    stmt.setBoolean(6, event.isIdle)
                    stmt.addBatch()
                }
                stmt.executeBatch()
                conn.commit()
            }
        }
    }

    override suspend fun pullRules(lastSync: Instant, dbUrl: String): List<AppRule> {
        return withContext(Dispatchers.IO) {
            val rules = mutableListOf<AppRule>()
            getConnection(dbUrl).use { conn ->
                // Ensure table exists on remote (First run check)
                conn.createStatement().execute("CREATE TABLE IF NOT EXISTS app_rule (pkg_name_pattern TEXT PRIMARY KEY, category TEXT)")
                conn.createStatement().execute("CREATE TABLE IF NOT EXISTS raw_event (id TEXT PRIMARY KEY, timestamp BIGINT, device_id TEXT, app_pkg_name TEXT, window_title TEXT, is_idle BOOLEAN)")
                
                val stmt = conn.prepareStatement("SELECT * FROM app_rule")
                val rs = stmt.executeQuery()
                while (rs.next()) {
                    val catStr = rs.getString("category")
                    val cat = try { AppCategory.valueOf(catStr) } catch(e: Exception) { AppCategory.NEUTRAL }
                    rules.add(AppRule(rs.getString("pkg_name_pattern"), cat))
                }
            }
            rules
        }
    }
    
    private fun getConnection(url: String): java.sql.Connection {
        // Parse simple URL or pass directly if driver supports it
        // Expected format: postgresql://user:pass@host:port/dbname?sslmode=...
        // JDBC expects: jdbc:postgresql://host:port/dbname?user=...&password=...
        // Just prepending jdbc: might work if URL structure matches
        val jdbcUrl = if (url.startsWith("jdbc:")) url else "jdbc:$url"
        return DriverManager.getConnection(jdbcUrl)
    }
}
