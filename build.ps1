# build.ps1 - A robust build script for the Rust project (with compatibility fixes)

# Ensures that the script will stop on any error, making it safer.
$ErrorActionPreference = 'Stop'
Set-StrictMode -Version Latest

# --- Helper function to check if a command is available in the PATH ---
function Test-Command {
    param($command)
    # Get-Command returns a command object if found, or throws an error.
    # We silently ignore the error and return $true/$false.
    return [bool](Get-Command $command -ErrorAction SilentlyContinue)
}

# --- Main Script Logic ---
try {
    # Step 1: Check for the rustup command
    Write-Host "1. Checking for rustup..." -ForegroundColor Yellow
    if (-not (Test-Command "rustup")) {
        Write-Host "   ERROR: 'rustup' command not found." -ForegroundColor Red
        Write-Host "   -> Please install Rust via rustup to manage toolchains." -ForegroundColor Cyan
        Write-Host "   -> Since you use Scoop, you can run: scoop install rustup" -ForegroundColor Cyan
        exit 1 # Exit the script with an error code
    }
    Write-Host "   OK: rustup is installed." -ForegroundColor Green

    # Step 2: Check if the 'nightly' toolchain is installed
    Write-Host "2. Checking for the 'nightly' toolchain..." -ForegroundColor Yellow
    $installed_toolchains = rustup toolchain list
    if ($installed_toolchains -notmatch 'nightly') {
        Write-Host "   INFO: Nightly toolchain not found. Installing now..." -ForegroundColor Cyan
        rustup toolchain install nightly
        Write-Host "   OK: Nightly toolchain installed successfully." -ForegroundColor Green
    } else {
        Write-Host "   OK: Nightly toolchain is already installed." -ForegroundColor Green
    }

    # Step 3: Check for the 'rust-src' component required by '-Z build-std'
    Write-Host "3. Checking for 'rust-src' component on nightly..." -ForegroundColor Yellow
    $installed_components = rustup component list --toolchain nightly
    if ($installed_components -notmatch 'rust-src \(installed\)') {
        Write-Host "   INFO: 'rust-src' component is missing. Installing now..." -ForegroundColor Cyan
        rustup component add rust-src --toolchain nightly
        Write-Host "   OK: 'rust-src' component installed successfully." -ForegroundColor Green
    } else {
        Write-Host "   OK: 'rust-src' component is already installed." -ForegroundColor Green
    }

    # Step 4: Run the final optimized build
    Write-Host "4. Starting release build with the nightly toolchain..." -ForegroundColor Yellow
    Write-Host "   (This uses the settings from your .cargo/config.toml file)"
    
    cargo +nightly build --release

    # Step 5: Display success message
    Write-Host "`nBuild completed successfully!" -ForegroundColor Green

    # Try to find the project name from Cargo.toml for a user-friendly output message
    $projectName = ""
    if (Test-Path ./Cargo.toml) {
        # This regex robustly finds the package name
        # --- THIS IS THE LINE THAT WAS FIXED ---
        $match = Get-Content ./Cargo.toml | Select-String -Pattern '^name\s*=\s*"([^"]+)"' | Select-Object -First 1
        if ($match) {
            # The structure of the match object is slightly different now
            $projectName = $match.Matches[0].Groups[1].Value
        }
    }
    $exeName = if (-not [string]::IsNullOrEmpty($projectName)) { "$($projectName).exe" } else { "(your_executable_name).exe" }
    
    # Assumes the target from your .cargo/config.toml
    $targetDir = "target/x86_64-pc-windows-msvc/release"
    Write-Host "-> Executable available at: $($targetDir)\$($exeName)" -ForegroundColor Cyan

} catch {
    # This block runs if any command in the 'try' block fails
    Write-Host "`nAN ERROR OCCURRED DURING THE BUILD PROCESS." -ForegroundColor Red
    Write-Host "ERROR DETAILS: $($_.Exception.Message)" -ForegroundColor Red
    exit 1
}