CLAUDE.md ecosystem curated:
  Tier 1 (CLAUDE.md USER:session-learnings):  none
  Tier 2 (.claude/rules/*):
    + testing.md: "Every new guard test carries its remove-the-guard run …"
      Proof: this chunk ran four such runs — the MAX_FRAME pin (cargo-mutants on lib.rs: 4 missed → 12 caught), the obs.rs cap witness
      (left (Debug, None) with the take removed), the mutants flags test (FAIL at mutants.rs:583 with the flags removed), and the swamp
      tests (left (8, 0, 0, 143) / left 0 with the rule neutralised). The operator's "every new guard test" direction (2026-09-24);
      evidence/remove-the-guard.md, evidence/leak-guard-and-swamp-rule.md.
    + testing.md: "Keep every `#[cfg(test)]` module inline …"
      Proof: scratch probe with cargo-modules 0.27.0 — lib.rs with `#[cfg(test)] mod t;` + src/t.rs → `Found 1 orphans`, rc 1 (phase P3,
      research.md §Measured facts); it decided the run.rs split shape.
    + host-win32.md: "Stop a process by its exact `ExecutablePath` …"
      Proof: a PowerShell stop keyed on command lines containing `cli-drive-38172` also stopped the calling bash (51832) and ended that
      tool call with exit 255; the leak-stop.ps1 re-run by ExecutablePath stopped exactly the 3 probe processes (evidence/leak-guard-and-swamp-rule.md).
  Tier 3 (.claude/docs/session-learnings.md): none
  Filters: 1 dup (leaked-process Drop guard — the cascade wrote it into verification-harness.md's body) · 2 below threshold at 0.6
    (force-push base-missing; `output()` pipe held by a leaked grandchild — both amended into test-plan this wrap, their home) ·
    0 conflict · 2 deferred (→ handoff)
  No-other-home: "Keep every #[cfg(test)] module inline" · "Stop a process by its exact ExecutablePath"
  CLAUDE.md size: 121/200 · T1 1.3 KB, 0 over 600 B
