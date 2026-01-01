# Nuke Service & Data (Admin Required - One Time)

$currentPrincipal = [Security.Principal.WindowsPrincipal][Security.Principal.WindowsIdentity]::GetCurrent()
$isAdmin = $currentPrincipal.IsInRole([Security.Principal.WindowsBuiltInRole]::Administrator)

if (-not $isAdmin) {
    Write-Warning "Requesting Administrator privileges to remove System Service..."
    Start-Process PowerShell -Verb RunAs "-NoProfile -ExecutionPolicy Bypass -Command `""$PSCommandPath`""";
    exit
}

Write-Host "Stopping SelfMonitorService..." -ForegroundColor Yellow
Stop-Service SelfMonitorService -Force -ErrorAction SilentlyContinue

Write-Host "Deleting Service Registration..." -ForegroundColor Yellow
sc.exe delete SelfMonitorService

Write-Host "Killing Processes..." -ForegroundColor Yellow
taskkill /F /IM time_authority_service.exe 2>$null
taskkill /F /IM time_authority_ui.exe 2>$null

Write-Host "Deleting Stale Database..." -ForegroundColor Red
Remove-Item "target\release\self_monitor.db*" -Force -ErrorAction SilentlyContinue

Write-Host "Cleanup Complete! You can now run builds without Admin." -ForegroundColor Green
Read-Host "Press Enter to exit"
