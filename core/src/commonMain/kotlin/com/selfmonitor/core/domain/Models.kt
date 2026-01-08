package com.selfmonitor.core.domain

import kotlinx.datetime.Instant
import kotlinx.serialization.Serializable

enum class AppCategory {
    WORK, DISTRACTION, NEUTRAL
}

enum class DailyVerdict {
    GREEN, YELLOW, RED
}

@Serializable
data class RawEvent(
    val id: String,
    val timestamp: Instant,
    val deviceId: String,
    val appPkgName: String,
    val windowTitle: String?,
    val isIdle: Boolean
)

@Serializable
data class Session(
    val id: String,
    val startTime: Instant,
    val endTime: Instant,
    val category: AppCategory,
    val durationSec: Long
)

@Serializable
data class DailyLog(
    val date: String, // YYYY-MM-DD
    val totalWorkSec: Long,
    val totalDistractionSec: Long,
    val verdict: DailyVerdict,
    val manualLeetCode: Boolean,
    val manualSugar: Boolean,
    val manualDiet: Boolean
)

@Serializable
data class AppRule(
    val pkgNamePattern: String,
    val category: AppCategory
)
