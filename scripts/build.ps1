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
    if ($LASTEXITCODE -ne 0) { throw "Backend Build Failed" }
    
    Copy-Item "categories.json" "target\release\categories.json" -Force
}
if ($LASTEXITCODE -ne 0) { Write-Error "Backend Build Failed."; exit 1 }

# 3. Build UI
Write-Host "Building UI (Tauri)..." -ForegroundColor Cyan
if (Test-Path "$PSScriptRoot\..\gui") {
    Push-Location "$PSScriptRoot\..\gui"
    # Ensure dependencies are installed if needed, or just build
    # npm install # (Skipped for speed over repeated runs, assume installed)
    
    npm run tauri build
    if ($LASTEXITCODE -ne 0) { 
        Pop-Location
        throw "Tauri Build Failed." 
    }
    Pop-Location

    # 4. Consolidate Artifacts (Production Layout)
    Write-Host "Consolidating Production Binaries..." -ForegroundColor Cyan
    
    $releaseDir = "$PSScriptRoot\..\target\release"
    $uiSource = "$PSScriptRoot\..\gui\src-tauri\target\release\time_authority_ui.exe"
    
    if (Test-Path $uiSource) {
        Copy-Item $uiSource "$releaseDir\time_authority_ui.exe" -Force
        
        # Also copy webview2 loader if present (needed for some systems)
        $loader = "$PSScriptRoot\..\gui\src-tauri\target\release\WebView2Loader.dll"
        if (Test-Path $loader) {
            Copy-Item $loader "$releaseDir\WebView2Loader.dll" -Force
        }
    } else {
        Write-Error "UI Binary not found after build!"
    }
} else {
    Write-Error "GUI directory not found."
    exit 1
}

Write-Host "Build Complete!" -ForegroundColor Green
Write-Host "Artifacts are in: target\release\"
Write-Host "  [1] time_authority_service.exe (Backend)"
Write-Host "  [2] time_authority_ui.exe (Frontend)"
Write-Host "  [3] self_monitor.db (Database)"

Write-Host "Service: target\release\time_authority_service.exe"
Write-Host "UI: gui\src-tauri\target\release\time_authority_ui.exe"
