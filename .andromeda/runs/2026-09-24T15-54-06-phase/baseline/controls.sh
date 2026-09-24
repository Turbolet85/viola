#!/usr/bin/env bash
# P5 baseline + known-positive controls for the inline guard entries (one-shot; reverts what it plants).
set -u
B=.andromeda/runs/2026-09-24T15-54-06-phase/baseline
C=$B/controls
mkdir -p "$C/wf"

echo "== sha-pin probe on the tree"
python -X utf8 -c "import re,glob;print(sum(1 for f in glob.glob('.github/workflows/*.yml') for l in open(f,encoding='utf-8') if re.search(r'uses:\s',l) and not re.search(r'@[0-9a-f]{40} # v',l)))"
echo "exit=$?"

echo "== sha-pin probe control (planted mutable ref)"
printf 'jobs:\n  x:\n    steps:\n      - uses: actions/checkout@v7\n' > "$C/wf/x.yml"
python -X utf8 -c "import re,glob;print(sum(1 for f in glob.glob('$C/wf/*.yml') for l in open(f,encoding='utf-8') if re.search(r'uses:\s',l) and not re.search(r'@[0-9a-f]{40} # v',l)))"
echo "exit=$?"

echo "== viola-ui build-free guard on the tree"
git ls-files --cached --others --exclude-standard -- tsconfig.json 'crates/viola-ui/**/tsconfig*.json' 'crates/viola-ui/**/package.json' 'crates/viola-ui/**/*.config.*'
echo "exit=$?"

echo "== viola-ui build-free guard control (planted tsconfig, then removed)"
mkdir -p crates/viola-ui
printf '{}\n' > crates/viola-ui/tsconfig.json
git ls-files --cached --others --exclude-standard -- tsconfig.json 'crates/viola-ui/**/tsconfig*.json' 'crates/viola-ui/**/package.json' 'crates/viola-ui/**/*.config.*'
echo "exit=$?"
rm crates/viola-ui/tsconfig.json
rmdir crates/viola-ui
test ! -e crates/viola-ui && echo "control reverted"

echo "== inventory gate on the tree (file absent)"
python -X utf8 -c "a=open('.andromeda/architecture.md',encoding='utf-8').read();r=[l.split(chr(9)) for l in open('viola-0.1.0/chunks/2026-09-24-workspace-tree-and-code-graph-planes/artifact-inventory.tsv',encoding='utf-8').read().splitlines()[1:] if l.strip()];print(len(r));print(sum(1 for x in r if x[0] not in a))"
echo "exit=$?"
