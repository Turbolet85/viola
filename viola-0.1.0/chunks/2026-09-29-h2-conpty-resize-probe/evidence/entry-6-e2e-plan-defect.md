# Entry 6 `run --e2e` — a plan defect, not a red to fold

**Operator ruling, 2026-09-29 (operator pass invocation), verbatim:** "Entry 6 --e2e is a plan defect, not a red to
fold: record it, and have the wrap pin it on the route entry that lands the first E2E test binary, with its owner
named."

## What ran
- Plan `## Test Commands` entry 6: `bash scripts/agent-run.sh run --e2e` (role `e2e`, expect `exit 0` +
  `contains "ok":true`).
- /implement gate run `implement-2026-09-29T05-24-12`, entry 6: **red · exit 2** in 0.28 s. Output:
  `error: unexpected argument '--e2e' found` then `{"v":1,"cmd":"run","ok":false,"reason":"usage","detail":"arguments"}`.
- **Control (two-sided):** the same command on a clean detached worktree at `90aba7c` (HEAD, no chunk edits),
  separate `CARGO_TARGET_DIR`: exit 2, the identical `unexpected argument '--e2e'` and the identical JSON document. The
  red is not this chunk's; the chunk's edits sit wholly inside `crates/viola-pty/src/lib.rs`'s `#[cfg(test)] mod tests`
  and touch no harness source.

## Why it is a plan defect
- The harness's `run` accepts `--unit --integration --mutants --coverage --fuzz-replay --browser --perf --all`
  (`run --help`); `--e2e` was never built. test-plan §3's `run` command body lists `--e2e` as specified, and test-plan
  §12 `2026-09-24` records "Unbuilt selectors are usage errors" and "The integration filterset is `kind(test)` until
  an E2E binary exists"; `crates/viola-e2e/src/harness/run/nextest.rs:10-11` says the E2E exclusion "arrives with the
  first E2E binary and `--e2e`".
- The plan copied `--e2e` from test-plan §3 into a gate ("`--e2e` runs because the fix branch may touch the pump"),
  and P5's dry-run cannot see a usage error. No earlier chunk plan lists `run --e2e`.
- What the entry meant to cover ran green elsewhere in the same gate run: entry 5 (`run`: 718 unit + 216
  integration, the pump contract tests and the `tui_` resize cases `pty_resize_reaches_the_child`,
  `tui_host_resize_reaches_the_child`, `tui_host_resize_in_the_pump_start_window_reaches_the_child` all PASS), and
  on this probe's recorder path the pump is untouched.

## Pin for the wrap
- **Pin on:** `viola-0.1.0/working-route.md:85`, the markerless entry **"The board: viola list"** (Epoch 4 — Session
  state & governance). Its CARRY (from chunk `2026-09-27-instance-state-and-start-order`) states that this entry
  "lands the `viola-e2e` `path_` E2E binary for Path 1" — the first E2E test binary the route names.
- **Owner:** the chunk that takes up "The board: viola list" builds the harness `run --e2e` selector together with
  that first `path_` binary (the selector and the integration filterset's `!binary(…)` exclusion arrive together,
  per test-plan §3 and `nextest.rs:10-11`). Until then no plan lists `run --e2e` as a gate.
- A fact for that pin, not ruled here: root `tests/` already holds `tui_*` and `contract_*` binaries, both prefixes
  inside test-plan §3's E2E exclusion regex `^(path|tui|mcp|http|sse|cross|chaos|contract)_`; the recorded
  decision ties the switch to "an E2E binary", and the route ties it to the `viola-e2e` `path_` binary.
