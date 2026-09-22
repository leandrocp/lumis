$ErrorActionPreference = "Stop"
Set-StrictMode -Version Latest

$Repository = "https://github.com/leandrocp/lumis"
$ReleasesApi = "https://api.github.com/repos/leandrocp/lumis/releases?per_page=100"

function Get-LumisVersion {
    if ($env:LUMIS_VERSION) {
        return $env:LUMIS_VERSION.TrimStart("v")
    }

    $Headers = @{
        Accept = "application/vnd.github+json"
        "X-GitHub-Api-Version" = "2022-11-28"
    }
    $Release = Invoke-RestMethod -Uri $ReleasesApi -Headers $Headers |
        Where-Object { $_.tag_name.StartsWith("cargo-lumis-cli/v") } |
        Select-Object -First 1
    if (-not $Release) {
        throw "Could not find the latest CLI release"
    }
    return $Release.tag_name.Substring("cargo-lumis-cli/v".Length)
}

function Get-LumisTarget {
    $Architecture = [System.Runtime.InteropServices.RuntimeInformation]::OSArchitecture
    switch ($Architecture) {
        "Arm64" { return "aarch64-pc-windows-msvc" }
        "X64" { return "x86_64-pc-windows-msvc" }
        default { throw "Unsupported architecture: $Architecture" }
    }
}

function Add-LumisToPath([string] $Directory) {
    $UserPath = [Environment]::GetEnvironmentVariable("Path", "User")
    $Entries = if ($UserPath) { $UserPath.Split(";", [StringSplitOptions]::RemoveEmptyEntries) } else { @() }
    if ($Entries -contains $Directory) {
        if (($env:Path.Split(";", [StringSplitOptions]::RemoveEmptyEntries)) -notcontains $Directory) {
            $env:Path = "$Directory;$env:Path"
        }
        return
    }

    $NewPath = (@($Directory) + $Entries) -join ";"
    [Environment]::SetEnvironmentVariable("Path", $NewPath, "User")
    $env:Path = "$Directory;$env:Path"
    Write-Host "Added $Directory to your user PATH."
}

$Version = Get-LumisVersion
if ($Version -notmatch "^\d+\.\d+\.\d+(?:-[0-9A-Za-z.-]+)?$") {
    throw "Invalid CLI version: $Version"
}
$Target = Get-LumisTarget
$Archive = "lumis-$Target.zip"
$Tag = "cargo-lumis-cli/v$Version"
$DownloadBase = "$Repository/releases/download/$Tag"
$InstallDir = if ($env:LUMIS_INSTALL_DIR) { $env:LUMIS_INSTALL_DIR } else { Join-Path $HOME ".local\bin" }
$TempDir = Join-Path ([IO.Path]::GetTempPath()) ("lumis-install-" + [guid]::NewGuid())

New-Item -ItemType Directory -Path $TempDir | Out-Null
try {
    $ArchivePath = Join-Path $TempDir $Archive
    $ChecksumPath = Join-Path $TempDir "lumis-$Target.sha256"

    Write-Host "Downloading lumis $Version for $Target"
    Invoke-WebRequest -Uri "$DownloadBase/$Archive" -OutFile $ArchivePath
    Invoke-WebRequest -Uri "$DownloadBase/lumis-$Target.sha256" -OutFile $ChecksumPath

    $EscapedArchive = [Regex]::Escape($Archive)
    $ChecksumLine = Get-Content $ChecksumPath |
        Where-Object { $_ -match "^([0-9a-fA-F]{64})\s+\*?$EscapedArchive$" } |
        Select-Object -First 1
    if (-not $ChecksumLine) {
        throw "Checksum for $Archive is missing"
    }
    $Expected = ([Regex]::Match($ChecksumLine, "^[0-9a-fA-F]{64}")).Value.ToLowerInvariant()
    $Actual = (Get-FileHash -Algorithm SHA256 $ArchivePath).Hash.ToLowerInvariant()
    if ($Actual -ne $Expected) {
        throw "Checksum verification failed for $Archive"
    }

    Expand-Archive -Path $ArchivePath -DestinationPath $TempDir -Force
    New-Item -ItemType Directory -Force -Path $InstallDir | Out-Null
    Copy-Item -Force (Join-Path $TempDir "lumis.exe") (Join-Path $InstallDir "lumis.exe")

    Write-Host "Installed lumis $Version to $InstallDir\lumis.exe"
    if ($env:LUMIS_NO_MODIFY_PATH -eq "1") {
        Write-Host "Add $InstallDir to PATH to use lumis."
    } else {
        Add-LumisToPath $InstallDir
    }
} finally {
    Remove-Item -Recurse -Force $TempDir
}
