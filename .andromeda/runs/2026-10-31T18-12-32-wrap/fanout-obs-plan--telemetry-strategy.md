proposals:
  - detector: D-retired
    severity: warning
    file: .andromeda/obs-plan-detail/telemetry-strategy.md
    near: 43
    section: '## 2. Telemetry Strategy'
    change: In the creator-explicit (Founder Direction 4) row, the second cell reads §6 detail code catalog, §12 D-01; a strict-modes refusal made before any process log opens (the six frame verbs and `link` / `unlink`, exit 21; `run`, `revive`'s start and `verify`, exit 1) writes no line and so no process-log `detail` code; its machine-readable form is the exit code, and under `--json` the `instance-unreachable` document carrying `detail:"strict-modes-failed"` (the frame verbs) or the exit-1 `strict-modes-failed` document (`revive`)
    sidecar: Telemetry Strategy trigger map, Founder Direction 4 row - a strict-modes refusal still writes no line and no process-log `detail` code, and its exit-21 `--json` document now names its cause in `detail` (`strict-modes-failed`) (chunk 2026-10-31-exit-cause-code-catalogue).
    rationale: The report's Retired bullet retires the exit-21 document's null `detail`, and Symbols / APIs says a strict-modes site now prints `detail` as `strict-modes-failed` (Counts, `doc-null`), so "carries no `detail` code" followed by the bare `instance-unreachable` document no longer holds for the --json form; the no-line half stands (Outcome, no refused check gains a line).
    basis: .andromeda/obs-plan-detail/telemetry-strategy.md:43
note: this file took two Write calls, the second only quoting the section value, which YAML otherwise reads as a comment.
