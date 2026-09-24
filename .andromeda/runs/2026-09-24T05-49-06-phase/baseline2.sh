#!/usr/bin/env bash
# P5 check 4 (9): re-baseline the reworked probes + their known-positive controls.
cd /d/dev/projects/viola || exit 9
out=.andromeda/runs/2026-09-24T05-49-06-phase/baseline
ctl=/d/dev/projects/viola/.andromeda/runs/2026-09-24T05-49-06-phase/control
run() {
  local n="$1"; shift
  bash -o pipefail -c "$*" >"$out/$n.log" 2>&1
  local rc=$?
  printf '%s exit=%s | %s\n' "$n" "$rc" "$(tail -c 220 "$out/$n.log" | tr '\n' ' ' | cut -c1-180)"
}
PIN='python -X utf8 -c "import re;t=open('"'"'.github/workflows/ci.yml'"'"').read();u=re.findall(r'"'"'uses:\s*(.+)'"'"',t);assert u;print(sum(1 for x in u if not re.match(r'"'"'[^@\s]+@[0-9a-f]{40} # '"'"',x)))"'
G3="grep -rEn --include=Cargo.toml --include=config.toml --include='*.yml' --include='*.yaml' --exclude-dir=target --exclude-dir=.git --exclude-dir=.andromeda \"(panic|_PANIC)\s*[:=]\s*[\\\"']?abort\" ."
run 13 'test ! -e target/agent-run/chunk.diff'
run 14 "$PIN"
run 15 "$G3"
mkdir -p "$ctl/target/agent-run"
: >"$ctl/target/agent-run/chunk.diff"
run 13c "cd $ctl && test ! -e target/agent-run/chunk.diff"
run 14c "cd $ctl && $PIN"
run 15c "cd $ctl && $G3"
printf 'jobs:\n  a:\n    steps:\n      - uses: actions/checkout@3d3c42e5aac5ba805825da76410c181273ba90b1 # v7.0.1\n' >"$ctl/pinned.yml"
mkdir -p "$ctl/p/.github/workflows"; cp "$ctl/pinned.yml" "$ctl/p/.github/workflows/ci.yml"
run 14p "cd $ctl/p && $PIN"
