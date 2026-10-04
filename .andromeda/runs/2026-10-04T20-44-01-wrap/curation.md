# Curation — 2026-10-04-the-wheel

CLAUDE.md ecosystem curated:
  Tier 1 (CLAUDE.md USER:session-learnings):  none
  Tier 2 (.claude/rules/*):                   + testing.md: "a test counting `events.ndjson` lines after `Wrapper::stop` counts the stop's own `wheel` record too…"
    Proof: the `hook_events` count cases went red when the stop's Ctrl-C began taking the wheel (report Deviations, `tests/hook_events.rs` count expectations; the bounded exit flush made them deterministic). Score 0.4 measured + 0.2 technical detail + 0.2 no-other-home = 0.8.
  Tier 3 (.claude/docs/session-learnings.md): + "Reading a Windows CI red from the kept home's role log"
    Proof: CARRY §8's first windows-2025 red (kept home `viola-test-wvRdGs`, `run-<name>.ndjson`: the child's exit 0 at +52 ms after the `^Z`, no Ctrl-C, no channel call) located the fake agent's std console read, not viola's (report Spec claims disproved 2). Score 0.4 + 0.2 + 0.2 no-other-home = 0.8.
  Correction (exempt from the cap): testing.md §Running tests "one Rust test: `run --e2e --filter …`" — corrected at its SOURCE, test-plan §3 → 5-command implementation (a P2 amendment raised here, its sidecar entry under this run's Ref); the leaf re-derived by the cascade. Proof: the chunk's own gate written with `--e2e` exited 2 (usage) and was corrected to `--integration` at P5 (report Decisions & corrections).
  Filters: 0 dup · 0 task-specific · 0 conflict · 0 deferred
  Rejected at exactly 0.6 (a master carries the fact this wrap — no conditional signal): the `answer`-to-an-unknown-id wheel probe (a11y-plan §4 case (3) + the keyboard-harness key file); win32-input-mode turning injected bytes into typed keys (architecture [Human Takeover / Wheel], a11y-plan §3); the fake agent's console read through `viola_pty::host_stdin()` (architecture [PTY]).
  Recurrence (→ handoff Deferred learnings): `recurrence-despite-learning: host-win32.md 2026-09-28` — the Bash guard refused a `cat` heredoc to a file again, in this window (fanout-results.md append) and the prior one (a scratchpad script).
  CLAUDE.md size: 124/200 · T1 1.8 KB, 0 over 600 B
