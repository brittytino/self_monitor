plugins {
    // Little trick to get version catalog or just use hardcoded versions for simplicity as requested (KISS)
    kotlin("multiplatform") version "1.9.21" apply false
    id("app.cash.sqldelight") version "2.0.1" apply false
    id("com.android.application") version "8.2.0" apply false
    id("com.android.library") version "8.2.0" apply false
}

buildscript {
    repositories {
        google()
        mavenCentral()
    }
}

allprojects {
    repositories {
        google()
        mavenCentral()
    }
}
