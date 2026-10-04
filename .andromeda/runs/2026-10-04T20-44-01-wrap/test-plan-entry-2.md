
## 2026-10-04-the-wheel — the one-Rust-test example selects through --integration
**Section:** §3 → 5-command implementation (Test selection)
**Change:** the one-Rust-test example: was `scripts/agent-run.sh run --e2e --filter 'test(/path2_send_confirms/)'`; now `run --integration --filter 'test(/path2_send_confirms/)'`, a whole root test file through `binary(<stem>)`. `--e2e` selects nothing yet: it is a usage error (exit 2) until the first E2E binary lands with its filterset.
**Why:** measured this chunk — the plan's own gate written with `--e2e` exited 2 and was corrected to `--integration` before implement; `path2_send_confirms` is an integration-tier test.
**Ref:** .andromeda/runs/2026-10-04T20-44-01-wrap/
