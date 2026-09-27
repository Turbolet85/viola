#!/usr/bin/env bash
# The baseline record's coverage firing form, run dir substituted; run by path (the regex carries backslashes).
cd /d/dev/projects/viola || exit 9
R=.andromeda/runs/2026-09-27T13-39-34-code-audit
cargo llvm-cov nextest --workspace --features viola/fake-agent --profile ci --json --summary-only --output-path $R/cov-raw.json --ignore-filename-regex '(viola-fake-agent|crates[/\\]viola-e2e|tests[/\\]support|fuzz[/\\])' > $R/cov.log 2>&1
echo "exit=$?" >> $R/cov.log
