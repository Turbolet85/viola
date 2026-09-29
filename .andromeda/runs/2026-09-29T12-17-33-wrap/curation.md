# Curation — wrap of 2026-09-29-sideloaded-conpty

Source: this session's conversation (the wrap from P2 on; the P1 window's conversation is gone) + the report's
*Decisions & corrections*. The implement session's own corrections are not in this window, so only what the report
carries was curated.

```
CLAUDE.md ecosystem curated:
  Tier 1 (CLAUDE.md USER:session-learnings):  none
  Tier 2 (.claude/rules/*):                   + testing.md: "Read a red root test's source before blaming the binary under test: `viola never exited` is the stamped-home fixture's `viola verify` step …, and a nextest `LEAK` means a child still held the test's stdio"
  Tier 3 (.claude/docs/session-learnings.md): + "Run-dir hygiene: the operator's disposition for phase-run CI copies"
  Filters: 1 dup · 2 task-specific · 0 conflict · 1 deferred (→ handoff)
  No-other-home: "the sweep hazards `viola never exited` / nextest `LEAK`" · "the Bash guard refuses a heredoc redirected to a file"
  Extended: T2/host-win32.md: "2026-09-28: The Bash guard refuses a heredoc whose payload carries a doubled backslash…" + "it also refuses a heredoc redirected to a file (`cat <<'EOF' > f`)"
  CLAUDE.md size: 124/200 · T1 1.8 KB, 0 over 600 B
```

## Applied
- **T2 testing.md — sweep hazard** (0.8: measured +0.4, specific detail +0.2, no-other-home +0.2)
  Proof: report *Decisions & corrections* — "`viola never exited` comes from `tests/support/verify.rs:81` (a `viola
  verify` in the stamped-home fixture), not from a wrapper"; the classification of `conpty_sideload`'s 5 pre-push
  failures under entry 6's basis rested on it (the control fails 22 and 25 tests with the identical message);
  "nextest `LEAK` means a child held the test's stdio after it ended".
- **T2 host-win32.md — extension** (0.8: measured +0.4, specific +0.2, no-other-home +0.2; the file has no
  `paths:`, so judged at Tier 1's bar — the whole entry is 545 B after the extension)
  Proof: report *Decisions & corrections*, Host mechanics — "the Bash guard refuses a `cat` heredoc with a file target
  (scripts go through the Write tool)".
- **T3 session-learnings.md — hygiene disposition** (0.8: the operator's direction +0.4, verified by a real gate
  refusal +0.4)
  Proof: report *Deviations* — "Hygiene control renamed `…-phase/baseline/control/net.rs` → `net.rs.txt` (bytes kept;
  the operator's convention)"; `gate.py hygiene` `refused 1` → renamed → `clean` (`evidence/operator-pass.md`); the
  prior wrap's handoff carried the same convention at 0.7, held by the max-3 cap.

## Filtered
- dup: "`rm -rf` in `target/e2e-home` was denied at the prompt" — host-win32 §Compound commands (`rm -rf` compounds get
  denied).
- task-specific: "leaked processes stopped by exact executable path, never the prototype's" (dup of host-win32
  2026-09-25 in substance, and the prototype clause is this host's); "do not chase the host (D: is a ReFS Dev Drive,
  C: NTFS)" (an overseer ruling for this chunk, cause unestablished).
- deferred (max-3 cap): "a `git worktree` cannot be moved across drives on this host" (0.8) → handoff.
- recurrence-despite-learning: host-win32.md Session Additions 2026-09-28 (the doubled-backslash Bash guard) — the
  report's host mechanics met it again ("any command carrying a doubled backslash") → handoff.
