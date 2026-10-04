#!/usr/bin/env bash
# P5 check 4 (9): baseline every new entry on the untouched tree, then the known-positive control of each inline guard.
cd /home/turbolet/dev/projects/viola || exit 9
R=.andromeda/runs/2026-10-04T01-32-13-phase
B() { # label, command
  local out; out=$(bash -o pipefail -c "$2" 2>&1); local code=$?
  printf '%s | exit %s | last: %s\n' "$1" "$code" "$(printf '%s' "$out" | tail -1 | cut -c1-160)"
}
echo "== baselines (untouched tree, HEAD $(git rev-parse --short HEAD))"
B zizmor 'zizmor .github/workflows/'
B no-secret-etc "! grep -nE 'secrets\\.|upload-artifact|rust-cache|concurrency:|^  (push|pull_request|schedule):|inputs:|needs:|persist-credentials: true' .github/workflows/windows-mutants.yml"
B uses-pinned "grep -E '^[[:space:]]*- uses:|^[[:space:]]*uses:' .github/workflows/windows-mutants.yml | grep -cvE '@[0-9a-f]{40} # v[0-9]'"
B shape-count "grep -cE '^permissions: \\{\\}\$|^      contents: read\$|^    runs-on: windows-2025\$|^      fail-fast: false\$' .github/workflows/windows-mutants.yml"
B ci-unchanged 'git diff --quiet 7aca5587bf93 -- .github/workflows/ci.yml .github/workflows/nightly.yml'
B terminate-wait "grep -c 'fail-fast = { max-fail = 1, terminate = \"wait\" }' .config/nextest.toml"
B nextest-only-failfast "git diff -U0 7aca5587bf93 -- .config/nextest.toml | grep -E '^[-+][^-+#]' | grep -vcE 'fail-fast = \\{ max-fail = 1, terminate = \"(immediate|wait)\" \\}'"

echo "== known-positive controls (synthetic input minted in the run dir)"
mkdir -p $R/p5-controls/wf
cat > $R/p5-controls/wf/bad.yml <<'YML'
name: bad
on:
  push:
  workflow_dispatch:
permissions: write-all
jobs:
  x:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - run: echo ${{ secrets.TOKEN }} ${{ github.event.inputs.x }}
YML
B zizmor-control "zizmor $R/p5-controls/wf/"
B no-secret-control "! grep -nE 'secrets\\.|upload-artifact|rust-cache|concurrency:|^  (push|pull_request|schedule):|inputs:|needs:|persist-credentials: true' $R/p5-controls/wf/bad.yml"
B uses-pinned-control "grep -E '^[[:space:]]*- uses:|^[[:space:]]*uses:' $R/p5-controls/wf/bad.yml | grep -cvE '@[0-9a-f]{40} # v[0-9]'"
B shape-count-control "grep -cE '^permissions: \\{\\}\$|^      contents: read\$|^    runs-on: windows-2025\$|^      fail-fast: false\$' $R/p5-controls/wf/bad.yml"
B ci-unchanged-control 'git diff --quiet 4a3062d -- .github/workflows/ci.yml .github/workflows/nightly.yml'
B nextest-only-failfast-control "printf '+slow-timeout = { period = \"9s\", terminate-after = 2 }\n+fail-fast = { max-fail = 1, terminate = \"wait\" }\n' | grep -E '^[-+][^-+#]' | grep -vcE 'fail-fast = \\{ max-fail = 1, terminate = \"(immediate|wait)\" \\}'"
B terminate-wait-control "printf 'fail-fast = { max-fail = 1, terminate = \"wait\" }\n' | grep -c 'fail-fast = { max-fail = 1, terminate = \"wait\" }'"
