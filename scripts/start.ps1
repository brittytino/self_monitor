# 1. Start Background Service (User Session Mode)
# "User Session Mode" is required to see window titles (Chrome, VLC, etc.)
# Windows Services (Session 0) are blind to user apps.

Write-Host "Starting Background Service (Top Secret)..." -ForegroundColor Cyan
$serviceProcess = Start-Process -FilePath "target\release\time_authority_service.exe" -WindowStyle Hidden -PassThru

Write-Host "Service Running (PID: $($serviceProcess.Id))"
Write-Host "  -> To kill: Stop-Process -Id $($serviceProcess.Id) -Force"
Write-Host "  -> Logs: target\release\service.log"

Start-Sleep -Seconds 2

# 2. Start UI
Write-Host "Starting UI..." -ForegroundColor Cyan

# Use the consolidated production binary
$uiPath = "target\release\time_authority_ui.exe"

if (Test-Path $uiPath) {
    Start-Process -FilePath $uiPath
} else {
    Write-Error "UI Binary not found at $uiPath. Did you run build.ps1?"
}

Write-Host "Done. You can close this terminal." -ForegroundColor Green
