plugins {
    kotlin("multiplatform")
    id("app.cash.sqldelight")
    id("com.android.library")
}

android {
    namespace = "com.selfmonitor.core"
    compileSdk = 34
    defaultConfig {
        minSdk = 26
    }
}

kotlin {
    // Targets
    jvm() // For testing and potentially desktop/server logic
    androidTarget {
        publishLibraryVariants("release")
        compilations.all {
            kotlinOptions {
                jvmTarget = "1.8"
            }
        }
    }
    
    sourceSets {
        val commonMain by getting {
            dependencies {
                implementation("app.cash.sqldelight:runtime:2.0.1")
                implementation("org.jetbrains.kotlinx:kotlinx-datetime:0.5.0")
                implementation("org.jetbrains.kotlinx:kotlinx-coroutines-core:1.7.3")
                implementation("org.jetbrains.kotlinx:kotlinx-serialization-json:1.6.2")
                implementation("org.postgresql:postgresql:42.7.1")
            }
        }
        val commonTest by getting {
            dependencies {
                implementation(kotlin("test"))
            }
        }
    }
}

sqldelight {
    databases {
        create("AppDatabase") {
            packageName.set("com.selfmonitor.db")
        }
    }
}
