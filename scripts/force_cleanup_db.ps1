# Force Cleanup Database (Run as Admin)

$currentPrincipal = [Security.Principal.WindowsPrincipal][Security.Principal.WindowsIdentity]::GetCurrent()
$isAdmin = $currentPrincipal.IsInRole([Security.Principal.WindowsBuiltInRole]::Administrator)

if (-not $isAdmin) {
    Write-Warning "Must be run as Administrator to delete System-owned files."
    Start-Process PowerShell -Verb RunAs "-NoProfile -ExecutionPolicy Bypass -Command `""$PSCommandPath`""";
    exit
}

Write-Host "Stopping Services..." -ForegroundColor Yellow
Stop-Service SelfMonitorService -Force -ErrorAction SilentlyContinue
taskkill /F /IM time_authority_service.exe 2>$null
taskkill /F /IM time_authority_ui.exe 2>$null

Write-Host "Forcing Database Deletion..." -ForegroundColor Red
Remove-Item "target\release\self_monitor.db" -Force -ErrorAction SilentlyContinue
Remove-Item "target\release\self_monitor.db-shm" -Force -ErrorAction SilentlyContinue
Remove-Item "target\release\self_monitor.db-wal" -Force -ErrorAction SilentlyContinue

Write-Host "Done. You can now run '.\scripts\start.ps1' as User." -ForegroundColor Green
Read-Host "Press Enter to exit"
