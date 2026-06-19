param(
    [Parameter(Position = 0)]
    [ValidateSet('validate-env', 'show-config', 'format-project', 'lint-project', 'test-project', 'compile-project', 'doc-project', 'validate-project')]
    [string]$Command = 'show-config'
)

$ErrorActionPreference = 'Stop'
$root = Split-Path -Parent $PSScriptRoot
$manifest = Join-Path $root 'Cargo.toml'

function Invoke-Checked {
    param([string[]]$CargoArgs)

    Push-Location $root
    try {
        & cargo @CargoArgs
        if ($LASTEXITCODE -ne 0) {
            throw "cargo $($CargoArgs -join ' ') failed with exit code $LASTEXITCODE"
        }
    } finally {
        Pop-Location
    }
}

function Require-Command {
    param([string]$Name)

    if (-not (Get-Command $Name -ErrorAction SilentlyContinue)) {
        throw "Required command not found on PATH: $Name"
    }
}

function Test-IsWorkspace {
    if (-not (Test-Path $manifest)) {
        return $false
    }

    $content = Get-Content -Path $manifest -Raw
    return $content -match '(?m)^\s*\[workspace\]'
}

function Get-ScopeArgs {
    if (Test-IsWorkspace) {
        return @('--workspace')
    }

    return @()
}

switch ($Command) {
    'validate-env' {
        Require-Command 'cargo'
        Require-Command 'rustc'
        if (-not (Test-Path $manifest)) {
            throw "Cargo manifest not found: $manifest"
        }
        Push-Location $root
        try {
            cargo --version
            rustc --version
            cargo metadata --no-deps --format-version 1 | Out-Null
        } finally {
            Pop-Location
        }
        Write-Host 'Rust workspace validation succeeded.'
    }
    'show-config' {
        if (-not (Test-Path $manifest)) {
            throw "Cargo manifest not found: $manifest"
        }
        Write-Host "Root: $root"
        Write-Host "Manifest: $manifest"
        Write-Host ("Workspace: {0}" -f (Test-IsWorkspace))
        Push-Location $root
        try {
            cargo metadata --no-deps --format-version 1
        } finally {
            Pop-Location
        }
    }
    'format-project' {
        Invoke-Checked -CargoArgs @('fmt', '--all', '--', '--check')
    }
    'lint-project' {
        $scope = Get-ScopeArgs
        Invoke-Checked -CargoArgs (@('clippy') + $scope + @('--all-targets', '--all-features', '--', '-D', 'warnings'))
    }
    'test-project' {
        $scope = Get-ScopeArgs
        Invoke-Checked -CargoArgs (@('test') + $scope + @('--all-features'))
    }
    'compile-project' {
        $scope = Get-ScopeArgs
        Invoke-Checked -CargoArgs (@('build') + $scope + @('--all-features'))
    }
    'doc-project' {
        $scope = Get-ScopeArgs
        Invoke-Checked -CargoArgs (@('doc') + $scope + @('--all-features', '--no-deps'))
    }
    'validate-project' {
        Invoke-Checked -CargoArgs @('fmt', '--all', '--', '--check')
        $scope = Get-ScopeArgs
        Invoke-Checked -CargoArgs (@('clippy') + $scope + @('--all-targets', '--all-features', '--', '-D', 'warnings'))
        Invoke-Checked -CargoArgs (@('test') + $scope + @('--all-features'))
        Invoke-Checked -CargoArgs (@('build') + $scope + @('--all-features'))
        Invoke-Checked -CargoArgs (@('doc') + $scope + @('--all-features', '--no-deps'))
    }
}
