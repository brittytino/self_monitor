package com.selfmonitor.core.domain

import kotlinx.datetime.Instant
import kotlin.test.Test
import kotlin.test.assertEquals
import kotlin.test.assertTrue

class RulesEngineTest {

    private val engine = DefaultRulesEngine()
    private val now = Instant.fromEpochMilliseconds(0) // Mock time not used directly in logic yet
    
    // Limits: Green < 30m distraction, Red > 2h distraction.

    @Test
    fun testGreenVerdict() {
        val workSessions = listOf(
            Session("1", now, now, AppCategory.WORK, 4 * 3600 + 1) // > 4h
        )
        val distractionSessions = listOf(
            Session("2", now, now, AppCategory.DISTRACTION, 10 * 60) // 10m
        )
        
        val verdict = engine.evaluateDay(
            sessions = workSessions + distractionSessions,
            manualInputs = ManualInputs(leetcodeSolved = true, sugarAvoided = true, dietFollowed = true),
            config = DailyConfig()
        )
        
        assertEquals(DailyVerdict.GREEN, verdict)
    }

    @Test
    fun testRedVerdict_TooMuchDistraction() {
        val sessions = listOf(
            Session("1", now, now, AppCategory.DISTRACTION, 2 * 3600 + 1) // > 2h
        )
        
        val verdict = engine.evaluateDay(
            sessions = sessions,
            manualInputs = ManualInputs(leetcodeSolved = true, sugarAvoided = true, dietFollowed = true),
            config = DailyConfig()
        )
        
        assertEquals(DailyVerdict.RED, verdict)
    }
    
    @Test
    fun testRedVerdict_NoLeetCode() {
        // Even if work is good and distraction is low
        val sessions = listOf(
            Session("1", now, now, AppCategory.WORK, 5 * 3600),
            Session("2", now, now, AppCategory.DISTRACTION, 0)
        )
        
        val verdict = engine.evaluateDay(
            sessions = sessions,
            manualInputs = ManualInputs(leetcodeSolved = false, sugarAvoided = true, dietFollowed = true),
            config = DailyConfig()
        )
        
        assertEquals(DailyVerdict.RED, verdict)
    }

    @Test
    fun testYellowVerdict() {
        // Work < 4h, Distraction < 2h, LeetCode = Yes
        val sessions = listOf(
            Session("1", now, now, AppCategory.WORK, 3 * 3600), // 3h
            Session("2", now, now, AppCategory.DISTRACTION, 60 * 60) // 1h
        )
        
        val verdict = engine.evaluateDay(
            sessions = sessions,
            manualInputs = ManualInputs(leetcodeSolved = true, sugarAvoided = true, dietFollowed = true)
        )
        
        assertEquals(DailyVerdict.YELLOW, verdict)
    }
}
