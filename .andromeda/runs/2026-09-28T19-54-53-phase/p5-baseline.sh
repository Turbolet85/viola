#!/usr/bin/env bash
# P5 check 4 (9): baseline runs of the plan's new=true, non-leg entries on the untouched tree, plus the
# known-positive controls of the green inline guards. Run from the repo root.
set -u
b() { echo "=== $1"; bash -o pipefail -c "$1" > /tmp/p5b.out 2>&1; e=$?; echo "exit $e"; echo "lines $(wc -l < /tmp/p5b.out)"; echo "last: $(tail -n 1 /tmp/p5b.out)"; }
b "grep -cE '^  (mutants|mutants-verdict):' .github/workflows/ci.yml"
b "grep -c download-artifact .github/workflows/ci.yml"
b 'grep -c "tool: cargo-nextest@0.9.146,cargo-mutants@27.1.0,cargo-llvm-cov@0.9.1" .github/workflows/ci.yml'
b 'git diff --quiet HEAD -- scripts/wsl-provision.sh'
b "grep -rlE 'mutants-legs|leg_verdict|cfg_legs|compiled_legs|mutants-verdict|invalid-leg|scoped-leg|linux-leg|windows-leg' crates src scripts .github Cargo.toml"
b 'git diff --quiet HEAD -- .config/nextest.toml'
b 'git diff --quiet HEAD -- src tests crates/viola-core crates/viola-pty crates/viola-channel crates/viola-state crates/viola-agent-claude'
b 'git diff HEAD -- .github/workflows/ci.yml'
echo "##### controls"
c=".andromeda/runs/2026-09-28T19-54-53-phase/p5-control"
mkdir -p "$c"
sed 's/cargo-mutants@27.1.0,cargo-llvm-cov/cargo-mutants@27.2.0,cargo-llvm-cov/' .github/workflows/ci.yml > "$c/ci-tool-line-changed.yml"
b "grep -c \"tool: cargo-nextest@0.9.146,cargo-mutants@27.1.0,cargo-llvm-cov@0.9.1\" $c/ci-tool-line-changed.yml"
b 'git diff --quiet HEAD -- .claude/session-handoff.md'
rm -rf "$c"
