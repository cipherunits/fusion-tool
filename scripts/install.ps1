#!/usr/bin/env pwsh

# Runs both as a file and as `irm <url> | iex`, so the whole body lives in a
# function: `exit` would close the caller's PowerShell session.

function Install-FusionTool {
    # Function-scoped, so non-terminating errors become catchable here without
    # changing the caller's session.
    $ErrorActionPreference = "Stop"

    # Windows PowerShell downloads are drastically slower with the progress bar.
    $ProgressPreference = "SilentlyContinue"

    $repo = "cipherunits/fusion-tool"

    $installDir = if ($env:FUSION_INSTALL_DIR) {
        $env:FUSION_INSTALL_DIR
    } else {
        Join-Path $env:LOCALAPPDATA "Programs\fusion"
    }

    Write-Host "Installing fusion-tool..."

    # Older PowerShell defaults to TLS 1.0, which GitHub rejects.
    try {
        [Net.ServicePointManager]::SecurityProtocol = [Net.SecurityProtocolType]::Tls12
    } catch {}

    $arch = if ($env:PROCESSOR_ARCHITEW6432) {
        $env:PROCESSOR_ARCHITEW6432
    } else {
        $env:PROCESSOR_ARCHITECTURE
    }

    switch ($arch) {
        "AMD64" {}
        "ARM64" { Write-Host "Note: no native ARM64 build yet, installing the x64 build." }
        default { throw "Unsupported architecture: $arch" }
    }

    $target = "x86_64-pc-windows-msvc"

    $version = $env:FUSION_VERSION

    if (-not $version) {
        try {
            $version = (Invoke-RestMethod `
                -Uri "https://api.github.com/repos/$repo/releases/latest" `
                -Headers @{ "User-Agent" = "fusion-tool-installer" } `
                -UseBasicParsing).tag_name
        } catch {
            throw "Could not determine the latest version. Set `$env:FUSION_VERSION and retry."
        }
    }

    Write-Host "Version: $version"

    $archive = "fusion-$version-$target.zip"
    $url = "https://github.com/$repo/releases/download/$version/$archive"

    $tmpDir = Join-Path ([IO.Path]::GetTempPath()) ("fusion-" + [Guid]::NewGuid())
    New-Item -ItemType Directory -Force -Path $tmpDir | Out-Null

    $exePath = Join-Path $installDir "fusion.exe"

    try {
        Write-Host "Downloading $archive..."

        try {
            Invoke-WebRequest -Uri $url -OutFile (Join-Path $tmpDir $archive) -UseBasicParsing
        } catch {
            throw "Failed to download $archive.`n" +
                  "Make sure the release exists: https://github.com/$repo/releases/tag/$version"
        }

        Write-Host "Extracting..."

        Expand-Archive -Path (Join-Path $tmpDir $archive) -DestinationPath $tmpDir -Force

        New-Item -ItemType Directory -Force -Path $installDir | Out-Null

        try {
            Copy-Item (Join-Path $tmpDir "fusion.exe") $exePath -Force -ErrorAction Stop
        } catch {
            throw "Could not write to $installDir. Close any running fusion.exe and retry."
        }
    } finally {
        Remove-Item $tmpDir -Recurse -Force -ErrorAction SilentlyContinue
    }

    # Persist the install directory in the user PATH, so plain `fusion` works in
    # cmd and PowerShell instead of only the full path to the exe.
    $userPath = [Environment]::GetEnvironmentVariable("Path", "User")

    $entries = if ($userPath) {
        $userPath -split ";" | ForEach-Object { $_.Trim().TrimEnd("\") }
    } else {
        @()
    }

    $pathWasUpdated = $false

    if ($entries -notcontains $installDir.TrimEnd("\")) {
        $newPath = if ([string]::IsNullOrEmpty($userPath)) {
            $installDir
        } else {
            "$($userPath.TrimEnd(';'));$installDir"
        }

        [Environment]::SetEnvironmentVariable("Path", $newPath, "User")

        $pathWasUpdated = $true
    }

    # Usable in the current session as well, without reopening a terminal.
    if (($env:Path -split ";") -notcontains $installDir) {
        $env:Path = "$($env:Path.TrimEnd(';'));$installDir"
    }

    $ranSuccessfully = $false

    try {
        & $exePath --version | Out-Null
        $ranSuccessfully = ($LASTEXITCODE -eq 0)
    } catch {
        $ranSuccessfully = $false
    }

    if (-not $ranSuccessfully) {
        throw "fusion.exe was installed to $installDir but failed to run.`n" +
              "Install the Microsoft Visual C++ Redistributable and try again."
    }

    Write-Host ""
    Write-Host "fusion-tool installed successfully!" -ForegroundColor Green
    Write-Host ""
    Write-Host "  Location: $exePath"
    Write-Host ""

    if ($pathWasUpdated) {
        Write-Host "Added to your PATH. Open a new terminal for it to take effect."
        Write-Host ""
    }

    Write-Host "Run:"
    Write-Host ""
    Write-Host "  fusion --help"
}

try {
    Install-FusionTool
} catch {
    Write-Host ""
    Write-Host "Error: $_" -ForegroundColor Red

    # Only when run as a file: exiting a piped-to-iex script would close the
    # user's PowerShell session.
    if ($MyInvocation.MyCommand.Path) {
        exit 1
    }
}
