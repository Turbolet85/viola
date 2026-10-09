#!/usr/bin/env bash
# A6 coverage: the project's coverage command in its JSON-summary form (the Epoch 2b record's commands.coverage),
# run from the repository root. Writes the tool's exit code to cov.exit; never piped.
R=.andromeda/runs/2026-10-08T10-08-51-code-audit
date -u +%FT%TZ > "$R/cov.start"
cargo llvm-cov nextest --workspace --features viola/fake-agent --profile ci --json --summary-only \
  --output-path "$R/cov-raw.json" \
  --ignore-filename-regex '(viola-fake-agent|crates[/\\]viola-e2e|tests[/\\]support|fuzz[/\\])' \
  > "$R/cov.log" 2>&1
echo $? > "$R/cov.exit"
date -u +%FT%TZ > "$R/cov.end"
