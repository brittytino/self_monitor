# Start Dev Environment (User Mode)

# 1. Start Background Service (Console Mode)
Write-Host "Starting Service (Console Mode)..." -ForegroundColor Cyan
$serviceProc = Start-Process -FilePath "target\release\time_authority_service.exe" -PassThru -NoNewWindow

Write-Host "Service PID: $($serviceProc.Id)"
Write-Host "Logs: target\release\service.log"

Start-Sleep -Seconds 2

# 2. Start UI
Write-Host "Starting UI..." -ForegroundColor Cyan

$uiPath = "gui\src-tauri\target\release\time_authority_ui.exe"
if (-not (Test-Path $uiPath)) {
    $uiPath = "gui\src-tauri\target\release\gui.exe"
}

if (Test-Path $uiPath) {
    Start-Process -FilePath $uiPath
} else {
    Write-Error "UI Binary not found at $uiPath or time_authority_ui.exe"
}

# 3. Cleanup on Exit
Stop-Process -Id $serviceProc.Id -Force -ErrorAction SilentlyContinue
Write-Host "Stopped."
