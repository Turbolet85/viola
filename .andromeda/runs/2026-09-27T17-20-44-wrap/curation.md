# Curation — 2026-09-27-epoch-2-cleanup wrap

CLAUDE.md ecosystem curated:
  Tier 1 (CLAUDE.md USER:session-learnings):  + "A red stays open until its cause is known: a green re-run never closes it, and a red met during a chunk folds into that chunk even outside its diff." (confidence 1.0)
  Proof: the overseer's direction at this chunk ("The red stays OPEN … A green re-run does not close it"); the operator pre-push red of 16:02:58Z (evidence/operator-pass.md) re-ran green (op-35b) and the watch stays OPEN (evidence/pty-watch-recorder.md). It also absorbs the handoff's deferred 0.7 learning of the same class.
  Tier 2 (.claude/rules/*):                   + host-win32.md: "In the interactive Git Bash `rg` is a shell function … probe a tool with `type`, and put a pinned tool on PATH inside the command that needs it." (confidence 0.8)
  Proof: gate entry 22 exit 127 in the gate tool's non-login bash (`/usr/bin/bash: line 1: rg: command not found`) while `command -v rg` answered `rg` interactively (`type rg` → "rg is a function"); fixed by the inline pinned PATH (plan entry 22, green unaided). No-other-home fired (+0.2 on an exact 0.6): the hazard is in no master, route annotation or ledger note.
  Correction (cap-exempt):                    host-win32.md 2026-09-24 entry, Extended 2026-09-26 clause "stop it by exact ExecutablePath immediately before each run --mutants" → [corrected 2026-09-27]
  Proof: rust-analyzer ran (2 processes) through the implement gate's pre-push (green, 1470 s), both operator pre-pushes (the red one failed on the viola-pty baseline, not on mutants.out), and two scoped runs; `test ! -e mutants.out` green after them (evidence/operator-pass.md, evidence/pre-push-sizing.md).
  Filters: 0 dup · 1 below threshold (C4 "nest a throwaway test repo one level down so its sibling scratch is its own", 0.2) · 0 conflict · 1 deferred (→ handoff)
  No-other-home: "`rg` is a function in the interactive Git Bash; the gate shell has none"
  Extended: T2/testing.md: "2026-09-24: A test wait that a mutant can reach …" + "a deadline EQUAL to the kill line is a race the test can lose — keep it strictly below, and have a spawned child stream its report to a known file outside the test's tempdir" (confidence 1.0)
  Proof: evidence/pty-watch-recorder.md — at 10 s = kill the dump got out in the control and was lost in the 16:02Z red; at 12 s always lost; at 7 s always kept; the known-path report file kept the report in every row. This is a recurrence-despite-learning of the same entry's 2026-09-27 extension (the watched test's bound equalled the kill line) → logged in the handoff.

Deferred (→ handoff): C6 "cargo-mutants 27.1.0 grades mutants unviable without the fake-agent feature" — CARRY 2's third fact. The other two (mutated-package tests; const initializers) are already in testing.md's Session Additions; this one's evidence was not located beyond the evolve P3 proposal's parenthetical, so it was not curated from a paraphrase.
