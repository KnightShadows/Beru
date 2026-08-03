$ErrorActionPreference = "Stop"

$Repo = "KnightShadows/Beru"
$BinName = "beru.exe"

Write-Host "Installing Beru for Windows..."

$ReleaseUrl = "https://api.github.com/repos/$Repo/releases/tags/v0.3.0"
try {
    $Release = Invoke-RestMethod -Uri $ReleaseUrl
} catch {
    Write-Error "No releases found for $Repo or failed to fetch release data. You will need to build from source."
    exit 1
}

$Asset = $Release.assets | Where-Object { $_.name -like "*x86_64-pc-windows-msvc.zip" } | Select-Object -First 1

if ($null -eq $Asset) {
    Write-Error "Could not find a Windows binary in the latest release. Please build from source."
    exit 1
}

$DownloadUrl = $Asset.browser_download_url
$TempDir = Join-Path $env:TEMP (New-Guid).ToString()
New-Item -ItemType Directory -Path $TempDir -Force | Out-Null

$ZipPath = Join-Path $TempDir "beru.zip"

Write-Host "Downloading $DownloadUrl..."
Invoke-WebRequest -Uri $DownloadUrl -OutFile $ZipPath

Write-Host "Extracting..."
Expand-Archive -Path $ZipPath -DestinationPath $TempDir -Force

$InstallDir = Join-Path $env:USERPROFILE ".cargo\bin"
if (-not (Test-Path $InstallDir)) {
    $InstallDir = Join-Path $env:USERPROFILE ".local\bin"
    New-Item -ItemType Directory -Path $InstallDir -Force | Out-Null
}

Write-Host "Installing to $InstallDir..."
Move-Item -Path (Join-Path $TempDir $BinName) -Destination (Join-Path $InstallDir $BinName) -Force

Remove-Item -Path $TempDir -Recurse -Force

Write-Host "Installation complete! Make sure $InstallDir is in your PATH environment variable."
