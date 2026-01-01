$vswhere = "${env:ProgramFiles(x86)}\Microsoft Visual Studio\Installer\vswhere.exe"
if (-not (Test-Path $vswhere)) {
    Write-Error "vswhere.exe not found. Is Visual Studio Installer installed?"
    exit 1
}

$installPath = & $vswhere -latest -products * -property installationPath
if (-not $installPath) {
    Write-Error "Visual Studio Build Tools installation not found. Please install Visual Studio Build Tools 2022."
    exit 1
}

$installer = "${env:ProgramFiles(x86)}\Microsoft Visual Studio\Installer\vs_installer.exe"
Write-Host "Opening Visual Studio Installer for $installPath..."
Write-Host "Please click 'Modify' at the bottom right if checks are selected."
Write-Host "Ensure 'Desktop development with C++' or 'MSVC v143 - VS 2022 C++ x64/x86 build tools' is checked."

# Launch the installer UI pre-configured to add the workload
Start-Process -FilePath $installer -ArgumentList "modify --installPath `"$installPath`" --add Microsoft.VisualStudio.Workload.VCTools --includeRecommended" -Wait
