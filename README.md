# Self Monitor (Time Authority)

**A System-Grade, Offline-Only Productivity Authority for Windows 11.**

> "If the data is not real, the system is worthless."

## Philosophy
This is not a productivity "helper". It is an **Authority**.
- **Enforcer Mode**: If you haven't done your work (120 mins), distracting apps are **killed instantly**.
- **0% AI, 0% Cloud, 0% Telemetry**: Your data never leaves your laptop.
- **Strict & Unforgiving**: Idle time is idle. Distractions are penalized.
- **Service-Driven**: The logic runs in the background (hidden process) even if you close the UI.

## Architecture (The 4 Layers)
The system enforces a strict one-way data flow.

1.  **System Watcher** (Raw Truth):
    *   Polls active window/process every 10s.
    *   Writes raw, append-only logs.
    *   *Passive Health Check: No flashing windows.*

2.  **Session Engine** (Merge):
    *   Merges logs into continuous "Sessions".
    *   **App Blocking**: If `Effective Work < 2 Hours`, `Distracting` apps are terminated.
    *   **Container Neutrality**: Browsers (`chrome`, `brave`) are **Neutral** by default. Only specific sites (e.g., "GitHub") count as Productive.

3.  **Daily Evaluator** (Freeze):
    *   Summarizes sessions into a single "Daily Report".
    *   Target: **120 Minutes** (2 Hours) Deep Work.
    *   Calculates `Effective Work = Productive - (Distracting * 0.75)`.

4.  **Streak Engine** (Judge):
    *   Qualified? Streak +1.
    *   Failed? Streak = 0.

## How to Run (Production)

The system runs as a **User-Session Background Process** to ensure it can see your active windows (avoiding Session 0 blindness).

### 1. Build & Deploy
If you changed config or updated code:
*   **Command**: `.\scripts\build.ps1`
*   *Action*: Compiles Rust backend + Tauri UI and consolidates them into `target/release`.

### 2. Launch (Daily)
To start the hidden background monitor + Dashboard:
*   **Command**: `.\scripts\start.ps1`
*   *Action*: Launches `time_authority_service.exe` (Hidden) and opens the Dashboard.

### 3. Reset (Maintenance)
If the database is locked or you want a fresh start:
*   **Command**: `.\scripts\nuke_service.ps1` (Run as Admin)
*   *Action*: Kills all processes, deletes the database.

## Categorization Rules
Categories are defined in `categories.json`.

*   **Distracting** (Blocked if < 120m):
    *   Social: Instagram, Twitter, Reddit, Discord.
    *   Media: VLC, Netflix, Prime, Spotify (Recovery).
    *   Games: Steam, Riot, Valorant.
*   **Productive**:
    *   Apps: VS Code, Terminal, Obsidian, Office.
    *   Sites: GitHub, Docs, localhost, StackOverflow.
*   **Neutral** (The Void):
    *   Browsers (Chrome/Edge/Brave) without specific productive titles.
    *   System apps (Explorer, Task Manager).
    *   *Neutral time earns 0 credit.*
