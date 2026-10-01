#!/usr/bin/env bash
# A6 coverage — the project's own command (commands.md), JSON summary into the run dir.
cd /d/dev/projects/viola || exit 9
R=.andromeda/runs/2026-10-01T09-18-50-code-audit
cargo llvm-cov nextest --workspace --features viola/fake-agent --profile ci --json --summary-only \
  --output-path "$R/cov-raw.json" \
  --ignore-filename-regex '(viola-fake-agent|crates[/\\]viola-e2e|tests[/\\]support|fuzz[/\\])' \
  > "$R/cov-stdout.txt" 2> "$R/cov-stderr.txt"
echo "exit $?" > "$R/cov-exit.txt"
