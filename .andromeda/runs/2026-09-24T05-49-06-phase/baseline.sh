#!/usr/bin/env bash
# P5 check 4 (9): baseline-run each new, non-leg gate entry on the untouched tree.
cd /d/dev/projects/viola || exit 9
out=.andromeda/runs/2026-09-24T05-49-06-phase/baseline
mkdir -p "$out"
run() {
  local n="$1"; shift
  bash -o pipefail -c "$*" >"$out/$n.log" 2>&1
  local rc=$?
  printf '%s exit=%s | %s\n' "$n" "$rc" "$(tail -c 200 "$out/$n.log" | tr '\n' ' ' | cut -c1-160)"
}
run 01 'cargo build --workspace --features fake-agent'
run 02 'cargo fmt --all --check'
run 03 'cargo clippy --workspace --all-targets --features fake-agent -- -D warnings'
run 04 'bash scripts/agent-run.sh run --unit'
run 05 'bash scripts/agent-run.sh run --integration'
run 06 'bash scripts/agent-run.sh cleanup --session gate-smoke'
run 07 'bash scripts/agent-run.sh boot --session gate-smoke --instance builder'
run 08 'bash scripts/agent-run.sh status --session gate-smoke'
run 12 'AGENT_RUN_CHUNK_BASE=0000000000000000000000000000000000000000 bash scripts/agent-run.sh run --mutants'
run 14 "grep -E '^\s*-?\s*uses:' .github/workflows/ci.yml | grep -Evc '@[0-9a-f]{40} # '"
run 15 "rg -n --hidden -g 'Cargo.toml' -g 'config.toml' -g '*.yml' -g '*.yaml' \"(panic|_PANIC)\s*[:=]\s*[\\\"']?abort\" ."
# known-positive controls for the two inline guards (inputs they must fail on)
ctl=.andromeda/runs/2026-09-24T05-49-06-phase/control
mkdir -p "$ctl/.github/workflows"
printf 'jobs:\n  a:\n    steps:\n      - uses: actions/checkout@v7\n      - uses: actions/upload-artifact@043fb46d1a93c77aae656e7c1c64a875d1fc6a0a # v7.0.1\n' >"$ctl/.github/workflows/ci.yml"
printf '[profile.release]\npanic = "abort"\n' >"$ctl/Cargo.toml"
run 14c "cd $ctl && grep -E '^\s*-?\s*uses:' .github/workflows/ci.yml | grep -Evc '@[0-9a-f]{40} # '"
run 15c "cd $ctl && rg -n --hidden -g 'Cargo.toml' -g 'config.toml' -g '*.yml' -g '*.yaml' \"(panic|_PANIC)\s*[:=]\s*[\\\"']?abort\" ."
ls /d/dev/projects/Cargo.toml /d/dev/Cargo.toml /d/Cargo.toml 2>&1
git check-ignore -v target/agent-run/chunk.diff mutants.out/outcomes.json target/e2e-home/x
