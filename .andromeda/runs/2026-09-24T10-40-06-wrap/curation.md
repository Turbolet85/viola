CLAUDE.md ecosystem curated:
  Tier 1 (CLAUDE.md USER:session-learnings):  corrected "Every viola format carries `v` …" → names the process-log-line exception (correction, cap-exempt)
    Proof: operator directive at phase P5 review; schemas/diag-line.v1.json admits no `v` (top-level unevaluatedProperties:false), tests/contract_diag_schema.rs green; obs-plan §8 + arch §Stack amended this wrap.
  Tier 2 (.claude/rules/*):
    + host-win32.md: "Under Git Bash `${var//\\//}` deletes forward slashes … normalise with `tr '\\' '/'`" (confidence 0.8)
      Proof: GNU bash 5.2.37 probe, `a/b\c/d` → `ab\cd`; the .claude/settings.json write guard built on it blocked 0/6 paths; the `tr` form gives 2,2,2,2,0,0 (scripts/guard-probe.py gate green).
    + testing.md: "A test that sets a process-global `OnceLock` … is isolated only by nextest's process-per-test …" (confidence 0.8)
      Proof: src/main.rs panic_hook tests both set PANIC_SINK; merged into one test; harness run/mutants use --test-tool=nextest (crates/viola-e2e/src/harness/run.rs mutants()).
    + observability.md: "`obs_event!` expands to `::tracing::event!`, so a raw-log census greps `tracing::(info|warn|error|debug|trace)!` …" (confidence 0.8)
      Proof: plan gate grep baseline red 6 hits at HEAD → green 0 after migration while viola_core::obs carries ::tracing::event!.
  Tier 3 (.claude/docs/session-learnings.md): none
  Filters: 2 homed-in-master (exact 0.6: self-contained schemas / no cross-file $ref; unevaluatedProperties vs additionalProperties — both amended into obs §8) · 1 already-homed (gate `env = []` for a run that sets its own shell variable — stated in the pipeline's plan-template §Test Commands; recorded as friction) · 0 conflict · 0 deferred
  No-other-home: "Git Bash ${var//\\//}"; "process-global OnceLock tests need nextest"; "obs_event! census pattern"
  CLAUDE.md size: 121/200 · T1 region ~1.3 KB, 0 over 600 B
