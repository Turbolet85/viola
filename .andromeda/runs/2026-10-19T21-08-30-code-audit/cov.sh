#!/usr/bin/env bash
# Coverage collector (A6): the project's own coverage command, summary JSON into the run dir.
# Run under a clean environment (no VIOLA_* / CLAUDE* value crosses), as pre-push does.
R=.andromeda/runs/2026-10-19T21-08-30-code-audit
H=$(getent passwd "$(id -u)" | cut -d: -f6)
exec /usr/bin/env -i HOME="$H" PATH="$H/.cargo/bin:/usr/local/bin:/usr/bin:/bin" \
  cargo llvm-cov nextest --workspace --features viola/fake-agent --profile ci --json --summary-only \
  --output-path "$R/cov-raw.json" \
  --ignore-filename-regex '(viola-fake-agent|crates[/\\]viola-e2e|tests[/\\]support|fuzz[/\\])' \
  > "$R/cov-run.log" 2>&1
