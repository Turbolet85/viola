# P5 controls — 2026-10-14-resume-and-compact-ledger-rows (validation check 4 (9))

Fired 2026-10-16 by the resumed P5 (this run dir), each AS THE ENTRY: `gate.py run --plan
viola-0.1.0/chunks/2026-10-14-resume-and-compact-ledger-rows/plan.md --entry {n} --project-root {clone}`, no run dir.
The clone: `git clone --shared` of the project at HEAD `880b056377f4`, in the session scratchpad
(`$TMPDIR/…/scratchpad/clone-p5`). Each input was made in the clone, read, then undone; the clone read 0 status rows after.
Lines copied as printed.

## Entry 13 — the fixture guard
- unmodified (must read red, as on the real tree):
  ` 13 probe       red · exit 0 ✗ (exit 1) · exit 1 · 0.01s · 0 B → 13.log · test -z "$(git diff --name-only --diff-filter=MD 880b05637… (308 chars)`
- must-pass — `fixtures/claude/2.1.287/SessionStart.resume-1.json` added, untracked:
  ` 13 probe       green · exit 0 · 0.01s · 0 B → 13.2.log · test -z "$(git diff --name-only --diff-filter=MD 880b05637… (308 chars)`
- must-fail — the resume variant added and the committed `SessionStart.default.json` modified:
  ` 13 probe       red · exit 0 ✗ (exit 1) · exit 1 · 0.01s · 0 B → 13.3.log · test -z "$(git diff --name-only --diff-filter=MD 880b05637… (308 chars)`
- must-fail — the resume variant and a second addition (`SessionStart.compact-1.json`):
  ` 13 probe       red · exit 0 ✗ (exit 1) · exit 1 · 0.01s · 0 B → 13.log · test -z "$(git diff --name-only --diff-filter=MD 880b05637… (308 chars)`

## Entry 15 — the preservation guard
- unmodified (must read green, as on the real tree):
  ` 15 probe       green · exit 0 · 0.01s · 0 B → 15.log · git diff --quiet 880b056377f4 -- . ':!crates/viola-agent-c… (695 chars)`
- must-pass — `crates/viola-agent-claude/src/ledger.rs` edited, the resume variant added, a chunk `evidence/live-round.md`
  written, `.andromeda/friction-log.ndjson` appended:
  ` 15 probe       green · exit 0 · 0.01s · 0 B → 15.2.log · git diff --quiet 880b056377f4 -- . ':!crates/viola-agent-c… (695 chars)`
- must-fail — `src/run/send/slot.rs` edited, beside the must-pass input:
  ` 15 probe       red · exit 0 ✗ (exit 1) · exit 1 · 0.01s · 0 B → 15.3.log · git diff --quiet 880b056377f4 -- . ':!crates/viola-agent-c… (695 chars)`
- must-fail — an untracked `src/p5_control.rs`, beside the must-pass input:
  ` 15 probe       red · exit 0 ✗ (exit 1) · exit 1 · 0.01s · 0 B → 15.log · git diff --quiet 880b056377f4 -- . ':!crates/viola-agent-c… (695 chars)`

## Entry 6 — the literal
The red baseline reads `error: no tests to run` (the five `check_resume_session_` cases are step 2's). Its atom
`"suite":"nextest-unit","passed":5,"failed":0` is spelt from the run's own output: the previous wrap's firing of
`bash scripts/agent-run.sh run --unit` (run dir `2026-10-14T06-37-54-wrap`, entry 5) printed
`{"v":1,"cmd":"run","ok":true,"suites":[{"suite":"nextest-unit","passed":2075,"failed":0,…`.
