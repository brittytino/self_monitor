package com.selfmonitor.core.domain

import kotlinx.datetime.Instant

interface Sessionizer {
    fun processEvents(events: List<RawEvent>, sessionGapMs: Long): List<Session>
}

class DefaultSessionizer : Sessionizer {
    override fun processEvents(events: List<RawEvent>, sessionGapMs: Long): List<Session> {
        if (events.isEmpty()) return emptyList()
        
        val sorted = events.sortedBy { it.timestamp }
        val sessions = mutableListOf<Session>()
        
        var currentStart = sorted.first()
        var lastEvent = sorted.first()
        
        // Threshold from Config
        val gapThresholdMs = sessionGapMs 
        
        for (i in 1 until sorted.size) {
            val event = sorted[i]
            val timeDiff = event.timestamp.toEpochMilliseconds() - lastEvent.timestamp.toEpochMilliseconds()
            val isSameApp = event.appPkgName == currentStart.appPkgName
            
            if (timeDiff > gapThresholdMs || !isSameApp) {
                // seal previous session
                val duration = (lastEvent.timestamp.toEpochMilliseconds() - currentStart.timestamp.toEpochMilliseconds()) / 1000
                if (duration > 0) {
                    sessions.add(Session(
                        id = currentStart.id, // Use start event ID for simplicity
                        startTime = currentStart.timestamp,
                        endTime = lastEvent.timestamp,
                        category = AppCategory.NEUTRAL, // Category lookup would happen here via RulesEngine
                        durationSec = duration
                    ))
                }
                currentStart = event
            }
            lastEvent = event
        }
        
        // Add final
        val duration = (lastEvent.timestamp.toEpochMilliseconds() - currentStart.timestamp.toEpochMilliseconds()) / 1000
        if (duration > 0) {
            sessions.add(Session(
                id = currentStart.id,
                startTime = currentStart.timestamp,
                endTime = lastEvent.timestamp,
                category = AppCategory.NEUTRAL,
                durationSec = duration
            ))
        }
        
        return sessions
    }
}

interface RulesEngine {
    fun classifyApp(pkgName: String, rules: List<AppRule>): AppCategory
    fun evaluateDay(sessions: List<Session>, manualInputs: ManualInputs, config: DailyConfig): DailyVerdict
}

data class DailyConfig(
    val workGoalSec: Long,
    val distractionLimitGreenSec: Long,
    val distractionLimitRedSec: Long,
    val leetCodeRequired: Boolean,
    val sessionGapSec: Long
)

// Simple implementation placeholders
class DefaultRulesEngine : RulesEngine {
    override fun classifyApp(pkgName: String, rules: List<AppRule>): AppCategory {
        // Simple exact match or fallback to NEUTRAL
        val rule = rules.find { it.pkgNamePattern == pkgName } 
        return rule?.category ?: AppCategory.NEUTRAL
    }

    override fun evaluateDay(sessions: List<Session>, manualInputs: ManualInputs, config: DailyConfig): DailyVerdict {
        val totalWork = sessions.filter { it.category == AppCategory.WORK }.sumOf { it.durationSec }
        val totalDistraction = sessions.filter { it.category == AppCategory.DISTRACTION }.sumOf { it.durationSec }
        
        // RED Logic
        if (totalDistraction > config.distractionLimitRedSec) return DailyVerdict.RED
        
        // Prompt says: "If Time > 9PM AND LeetCode == 0 -> RED"
        if (config.leetCodeRequired && !manualInputs.leetcodeSolved) return DailyVerdict.RED
        
        // GREEN Logic
        if (totalWork >= config.workGoalSec && totalDistraction <= config.distractionLimitGreenSec) return DailyVerdict.GREEN
        
        // YELLOW Logic
        return DailyVerdict.YELLOW
    }

    fun determineConsequences(yesterdayVerdict: DailyVerdict): EnforcementState {
        return when (yesterdayVerdict) {
            DailyVerdict.RED -> EnforcementState(blockNonEssential = true, strictMode = true)
            DailyVerdict.YELLOW -> EnforcementState(blockNonEssential = false, strictMode = true)
            DailyVerdict.GREEN -> EnforcementState(blockNonEssential = false, strictMode = false)
        }
    }
}

data class EnforcementState(
    val blockNonEssential: Boolean,
    val strictMode: Boolean
)
