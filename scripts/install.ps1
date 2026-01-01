# Install SelfMonitor Service
$binPath = "C:\dev\self_monitor\target\release\time_authority_service.exe"
$serviceName = "SelfMonitorService"

if (!(Test-Path $binPath)) {
    Write-Error "Binary not found at $binPath. Please build release first."
    exit 1
}

# Check for Administrator privileges
$currentPrincipal = [Security.Principal.WindowsPrincipal][Security.Principal.WindowsIdentity]::GetCurrent()
$isAdmin = $currentPrincipal.IsInRole([Security.Principal.WindowsBuiltInRole]::Administrator)

if (-not $isAdmin) {
    Write-Warning "Service installation skipped: Administrator privileges are required to create services."
    Write-Host "The 'service.exe' binary has been built successfully."
    Write-Host "To install the service, please run PowerShell as Administrator and execute: .\scripts\install.ps1"
    exit 0
}

# Stop existing service if running (ignore error if not exists)
Stop-Service $serviceName -ErrorAction SilentlyContinue

# Ensure service is removed if it exists (to update binary path or clean state)
if (Get-Service $serviceName -ErrorAction SilentlyContinue) {
    # Remove-Service is not available in older PowerShell versions, use sc.exe
    $serviceFn = $serviceName
    sc.exe stop $serviceFn
    Start-Sleep -Seconds 2
    sc.exe delete $serviceFn
    # Give it a moment to release
    Start-Sleep -Seconds 2
}

New-Service -Name $serviceName -BinaryPathName $binPath -DisplayName "Time Authority Service" -StartupType Automatic -Description "Observations and Accountability."

Start-Service $serviceName
Write-Host "Service Installed and Started."
