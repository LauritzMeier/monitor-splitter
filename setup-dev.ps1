# Monitor Splitter — Developer Setup (Windows)
# Run: powershell -ExecutionPolicy Bypass -File setup-dev.ps1

$ErrorActionPreference = "Stop"
Write-Host "`n=== Monitor Splitter — Dev Setup ===" -ForegroundColor Cyan

# 1. Check/install Rust
if (Get-Command rustup -ErrorAction SilentlyContinue) {
    Write-Host "[OK] Rust already installed" -ForegroundColor Green
    rustup default nightly 2>$null
} else {
    Write-Host "[..] Installing Rust..." -ForegroundColor Yellow
    Invoke-WebRequest https://win.rustup.rs/x86_64 -OutFile "$env:TEMP\rustup-init.exe"
    & "$env:TEMP\rustup-init.exe" -y --default-toolchain nightly
    $env:PATH = "$env:USERPROFILE\.cargo\bin;$env:PATH"
    Write-Host "[OK] Rust installed" -ForegroundColor Green
}

# 2. Check/install VS Build Tools (C++ workload)
$vsWhere = "${env:ProgramFiles(x86)}\Microsoft Visual Studio\Installer\vswhere.exe"
if (Test-Path $vsWhere) {
    Write-Host "[OK] Visual Studio Build Tools found" -ForegroundColor Green
} else {
    Write-Host "[..] Installing VS Build Tools (C++ workload)..." -ForegroundColor Yellow
    Invoke-WebRequest "https://aka.ms/vs/17/release/vs_BuildTools.exe" -OutFile "$env:TEMP\vs_BuildTools.exe"
    Start-Process "$env:TEMP\vs_BuildTools.exe" -ArgumentList "--quiet --wait --norestart --add Microsoft.VisualStudio.Workload.VCTools --includeRecommended" -Wait
    Write-Host "[OK] VS Build Tools installed" -ForegroundColor Green
}

# 3. Install tauri-cli
if (Get-Command cargo-tauri -ErrorAction SilentlyContinue) {
    Write-Host "[OK] tauri-cli already installed" -ForegroundColor Green
} else {
    Write-Host "[..] Installing tauri-cli..." -ForegroundColor Yellow
    cargo install tauri-cli --version "^2.0"
    Write-Host "[OK] tauri-cli installed" -ForegroundColor Green
}

Write-Host "`n=== Done! ===" -ForegroundColor Cyan
Write-Host "To run the app:  cd app && cargo tauri dev"
Write-Host "To build:        cd app && cargo tauri build"
Write-Host ""

