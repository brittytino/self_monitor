# Self Monitor: Discipline Enforcement System

**Self Monitor** is a production-grade, single-user, offline-first digital wellbeing system designed for strict discipline enforcement on Windows 11 and Android 14. 

This system is not a habit tracker or a motivational tool. It is an enforcement utility that assumes "System Authority" to mandate productivity windows, block distractions, and maintain an immutable record of daily activity.

## Core Philosophy

1.  **Offline-First & Truthful**: SQLite is the single source of truth. Data is recorded locally and is immutable.
2.  **Enforcement, Not Nudges**: On Android, the system acts as the default launcher to physically prevent access to blocked apps. On Windows, it runs as a system service.
3.  **Deterministic Rules**: Daily verdicts (Green/Yellow/Red) are calculated mathematically based on configurable thresholds. There are no manual overrides.
4.  **Optional Sync**: Data is strictly local by default. Encrypted cloud sync (Neon Postgres) is available for multi-device consistency but is not required for operation.

## Architecture

The system follows a strict **4-Layer Architecture**:

1.  **Watcher Layer**: 
    - **Windows**: Rust-based background service (low-footprint) polling `GetForegroundWindow` and `GetLastInputInfo`.
    - **Android**: Accessibility Service & UsageStats Manager.
2.  **Ingestion Layer**: 
    - Raw events are written to a local SQLite database (`raw_event` table).
    - **Single-Writer Constraint**: Only the local device writes to its own event log.
3.  **Logic Layer (Core)**: 
    - Shared Kotlin Multiplatform (KMP) logic for Sessionization, Rule Evaluation, and Sync.
    - Configuration is loaded from the database (`system_config`).
4.  **Presentation Layer**: 
    - **UI**: Jetpack Compose (Android) and Compose for Desktop (Windows).
    - **State**: Read-only, reactive UI driven exclusively by the database.

## Technology Stack

-   **Shared Core**: Kotlin Multiplatform (KMP)
-   **Database**: SQLite (via SQLDelight), enabled with WAL mode.
-   **Desktop**: Rust (WinAPI Service), Kotlin (UI).
-   **Mobile**: Android (Jetpack Compose, AccessibilityService).
-   **Sync**: Neon Postgres (Write-only mirror for backup/consistency).

## Setup & configuration

### Prerequisites
-   **JDK 17+**
-   **Rust Toolchain** (stable)
-   **Android SDK 34**
-   **Neon Postgres Account** (Optional, for sync)

### 1. Environment Configuration
The system requires a `DATABASE_URL` for sync features.
-   **Desktop**: Create a `.env` file in the root directory:
    ```bash
    DATABASE_URL=postgresql://user:pass@host/dbname?sslmode=require
    ```
-   **Android**: You will be prompted to securely enter the connection string on the first launch. Credentials are stored in `EncryptedSharedPreferences`.

### 2. Running the System

#### Windows (Desktop)
1.  **Start the Tracker Service**:
    ```powershell
    cd service
    cargo run --release
    ```
    *This starts the background process that records window activity.*

2.  **Start the Interface**:
    ```powershell
    ./gradlew :app:run
    ```

#### Android (Mobile)
1.  **Install the APK**:
    ```bash
    ./gradlew :app:installDebug
    ```
2.  **Permissions**:
    -   Launch "Self Monitor".
    -   Grant **Accessibility Permissions** (Required for blocking).
    -   Set as **Default Launcher** (Required for enforcement).
    -   Bypass Battery Optimization.

## Sync Engine

The sync engine uses a deterministic state machine:
-   **Push**: Local raw events are pushed to the cloud.
-   **Pull**: Global configuration rules are pulled from the cloud.
-   **Conflict Resolution**: Last-write-wins based on UTC timestamps.

Sync status is visible on the dashboard:
-   `Idle`: System is operational, waiting for trigger.
-   `Pushing`/`Pulling`: Active network operation.
-   `Success: <Timestamp>`: Last successful sync.
-   `Failed: <Reason>`: Network or Auth error.

## Development

-   **Core Logic**: Modified in `core/src/commonMain`.
-   **Database Schema**: `core/src/commonMain/sqldelight`.
-   **UI**: `app/src/androidMain` and `app/src/desktopMain`.

To verify the installation:
```bash
./gradlew check
```
