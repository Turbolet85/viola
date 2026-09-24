# scripts/agent-run.ps1 - agent-driven verification harness driver (PowerShell shim)
# Generated once by /andromeda-setup-project Phase 4 from test-plan section 3 + obs-plan section 3.
# Project-evolvable: setup never overwrites this file on a re-run (drift is backed up and surfaced instead).
#
# Contract (test-plan section 3 "Harness implementation"): agent-run.sh and agent-run.ps1 are identical
# THIN shims. Each runs `cargo run -q -p viola-e2e --bin viola-harness -- <command> [flags]` and forwards
# its exit code and stdout unchanged. All logic lives once, in Rust (crates/viola-e2e).
#
# 5-command discipline (mirrors agent-run.sh): boot, run, status, cleanup, logs.
# Internal subcommands forwarded unchanged (not agent-facing): supervise, ui-restart, gate, and the
# CI gate bodies schema-check (G4) and secret-scan.
# Every command prints exactly one JSON document on stdout; exit 0 ok, 1 failure, 2 usage error.
#
# Run with: .\scripts\agent-run.ps1 boot

$ErrorActionPreference = 'Stop'
Set-StrictMode -Version Latest

# UTF-8 on both halves of the boundary: Python-mediated legs AND PowerShell's own decoding of
# external output (legacy-codepage defaults mangle em-dash/arrow-dense project text otherwise)
$env:PYTHONUTF8 = '1'; $env:PYTHONIOENCODING = 'utf-8'
[Console]::OutputEncoding = [System.Text.Encoding]::UTF8
$OutputEncoding = [System.Text.Encoding]::UTF8

$Root = Split-Path -Parent $PSScriptRoot
$HarnessArgs = @($args)
$Cmd = if ($HarnessArgs.Count -gt 0) { [string]$HarnessArgs[0] } else { '' }

# Embedded-asset freshness: test-plan section 3 puts the refresh INSIDE `viola-harness boot` - step 1
# always runs `cargo build --workspace --features fake-agent` before any process starts, and after UI
# readiness boot byte-compares every embedded /assets/* path with the repo file ("stale-embedded-assets").
# The shim adds no step of its own. Both shell variants must expose identical semantics.
function Invoke-EnsureFreshArtifacts {
}

function Invoke-Harness {
    Push-Location $Root
    try {
        & cargo run -q -p viola-e2e --bin viola-harness -- @HarnessArgs
        $code = $LASTEXITCODE
    } finally {
        Pop-Location
    }
    exit $code
}

switch ($Cmd) {
    'boot' {
        Invoke-EnsureFreshArtifacts
        Invoke-Harness
    }
    'run' {
        Invoke-Harness
    }
    'status' {
        Invoke-Harness
    }
    'cleanup' {
        Invoke-Harness
    }
    'logs' {
        Invoke-Harness
    }
    'supervise' {
        Invoke-Harness
    }
    'ui-restart' {
        Invoke-Harness
    }
    'gate' {
        Invoke-Harness
    }
    'schema-check' {
        Invoke-Harness
    }
    'secret-scan' {
        Invoke-Harness
    }
    default {
        Write-Output '{"v":1,"cmd":null,"ok":false,"reason":"usage"}'
        [Console]::Error.WriteLine('Usage: .\agent-run.ps1 {boot|run|status|cleanup|logs} [flags]')
        exit 2
    }
}
