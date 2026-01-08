package com.selfmonitor.core.domain

import kotlinx.datetime.Instant

interface Sessionizer {
    fun processEvents(events: List<RawEvent>): List<Session>
}

interface RulesEngine {
    fun classifyApp(pkgName: String, rules: List<AppRule>): AppCategory
    fun evaluateDay(sessions: List<Session>, manualInputs: ManualInputs): DailyVerdict
}

data class ManualInputs(
    val leetcodeSolved: Boolean,
    val sugarAvoided: Boolean,
    val dietFollowed: Boolean
)

// Simple implementation placeholders
class DefaultRulesEngine : RulesEngine {
    override fun classifyApp(pkgName: String, rules: List<AppRule>): AppCategory {
        // Simple exact match or fallback to NEUTRAL
        // In reality, this should support regex or glob patterns as hinted by "pattern"
        val rule = rules.find { it.pkgNamePattern == pkgName } // Naive implementation
        return rule?.category ?: AppCategory.NEUTRAL
    }

    override fun evaluateDay(sessions: List<Session>, manualInputs: ManualInputs): DailyVerdict {
        val totalWork = sessions.filter { it.category == AppCategory.WORK }.sumOf { it.durationSec }
        val totalDistraction = sessions.filter { it.category == AppCategory.DISTRACTION }.sumOf { it.durationSec }
        
        // Limits (in seconds)
        val workGoal = 4 * 3600 // 4 hours
        val distractionLimitGreen = 30 * 60 // 30 mins
        val distractionLimitRed = 2 * 3600 // 2 hours
        
        // RED Logic
        if (totalDistraction > distractionLimitRed) return DailyVerdict.RED
        
        // Prompt says: "If Time > 9PM AND LeetCode == 0 -> RED"
        // We assume this evaluation happens AT end of day.
        if (!manualInputs.leetcodeSolved) return DailyVerdict.RED
        
        // GREEN Logic
        if (totalWork >= workGoal && totalDistraction <= distractionLimitGreen) return DailyVerdict.GREEN
        
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
