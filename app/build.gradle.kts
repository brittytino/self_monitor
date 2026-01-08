plugins {
    kotlin("multiplatform")
    id("com.android.application")
    id("org.jetbrains.compose")
}

kotlin {
    androidTarget {
        compilations.all {
            kotlinOptions {
                jvmTarget = "1.8"
            }
        }
    }
    
    // Desktop target for Windows UI
    jvm("desktop")
    
    sourceSets {
        val androidMain by getting {
            dependencies {
                implementation(project(":core"))
                implementation("androidx.activity:activity-compose:1.8.0")
                implementation("androidx.compose.ui:ui:1.5.4")
                // Android specific system libs
                implementation("androidx.core:core-ktx:1.12.0")
                implementation("androidx.work:work-runtime-ktx:2.9.0") // For background jobs
            }
        }
        val desktopMain by getting {
            dependencies {
                implementation(compose.desktop.currentOs)
                implementation(project(":core"))
            }
        }
        val commonMain by getting {
            dependencies {
                implementation(compose.runtime)
                implementation(compose.foundation)
                implementation(compose.material3)
                implementation(project(":core"))
            }
        }
    }
}

android {
    namespace = "com.selfmonitor.app"
    compileSdk = 34
    defaultConfig {
        applicationId = "com.selfmonitor.app"
        minSdk = 26
        targetSdk = 34
        versionCode = 1
        versionName = "1.0"
    }
}
