# 1. Check Admin (Optional)
$currentPrincipal = [Security.Principal.WindowsPrincipal][Security.Principal.WindowsIdentity]::GetCurrent()
$isAdmin = $currentPrincipal.IsInRole([Security.Principal.WindowsBuiltInRole]::Administrator)
if (-not $isAdmin) {
    Write-Warning "Running in User Mode. Service management will be skipped."
}

# 2. Setup Environment
$vswhere = "${env:ProgramFiles(x86)}\Microsoft Visual Studio\Installer\vswhere.exe"
$vcvars = $null
if (Test-Path $vswhere) {
    $installPath = & $vswhere -latest -products * -requires Microsoft.VisualStudio.Component.VC.Tools.x86.x64 -property installationPath
    if ($installPath) {
        $vcvars = "$installPath\VC\Auxiliary\Build\vcvars64.bat"
    }
}

# 3. Stop Service to release locks
Write-Host "Stopping Services..." -ForegroundColor Cyan
Stop-Service SelfMonitorService -Force -ErrorAction SilentlyContinue
Start-Sleep -Seconds 1
taskkill /F /IM time_authority_service.exe 2>$null
taskkill /F /IM self_monitor_core.exe 2>$null
taskkill /F /IM time_authority_ui.exe 2>$null
taskkill /F /IM gui.exe 2>$null

# 4. Clean Artifacts
Write-Host "Cleaning Artifacts..." -ForegroundColor Cyan
Remove-Item "target\release\time_authority_service.exe" -ErrorAction SilentlyContinue
Remove-Item "gui\src-tauri\target\release\time_authority_ui.exe" -ErrorAction SilentlyContinue
Remove-Item "gui\dist" -Recurse -Force -ErrorAction SilentlyContinue

# 5. Build Backend Service
Write-Host "Building Backend Service..." -ForegroundColor Cyan
if ($vcvars) {
    cmd.exe /c " `"$vcvars`" && rustup default stable-x86_64-pc-windows-msvc && cargo build --release --bin time_authority_service "
}
else {
    cargo build --release --bin time_authority_service
}
if ($LASTEXITCODE -ne 0) { Write-Error "Backend Build Failed."; exit 1 }

# 6. Build Frontend & UI Bundle (Tauri CLI)
Write-Host "Building UI Bundle..." -ForegroundColor Cyan
Push-Location gui
npm install
npm run tauri build
if ($LASTEXITCODE -ne 0) { Write-Error "Tauri Build Failed."; Pop-Location; exit 1 }
Pop-Location

Write-Host "Build Complete!" -ForegroundColor Green
Write-Host "Service: target\release\time_authority_service.exe"
Write-Host "UI: gui\src-tauri\target\release\time_authority_ui.exe"
