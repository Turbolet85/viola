#!/usr/bin/env bash
# A6 coverage, the baseline record's commands.coverage form (the project's own coverage command, summary-only JSON),
# run once from the repository root; the raw report lands in this run dir and is deleted after covsum.py.
RUN="$(cd "$(dirname "$0")" && pwd)"
cd "$RUN/../../.." || exit 9
cargo llvm-cov nextest --workspace --features viola/fake-agent --profile ci --json --summary-only \
  --output-path "$RUN/cov-raw.json" \
  --ignore-filename-regex '(viola-fake-agent|crates[/\\]viola-e2e|tests[/\\]support|fuzz[/\\])' \
  > "$RUN/cov-run.log" 2>&1
echo "cov exit $?"
