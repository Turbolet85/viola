# Curation — 2026-11-17-booted-wrapper-does-not-outlive-its-test

CLAUDE.md ecosystem: no new learnings this session.
  Filters: 1 dup-home · 0 task-specific · 0 conflict · 0 deferred · 1 below threshold
  CLAUDE.md size: 134/200 · T1 3.9 KB, 0 over 600 B

## Dispositions, one line per Decisions & corrections entry
- implement · the operator's directive at invocation (no live start, the operator pass with the ci.py read, the exact cache hit, no wake): not a learning — a per-chunk directive, kept and recorded in the report.
- implement · sweep hazard «`Command::new\((support::|home::)?VIOLA\)` misses `Command::new(support::home::VIOLA)`; a census over a constant's spawns matches any path ending in the constant» (also inputs#I4's candidate «a site sweep is run over every spelling of the constant, not one»): rejected — Filter 4, confidence 0.6, not above 0.6 (verified by measurement +0.4: the 45th site at tests/cross_exit_causes.rs:461, the census green on its planted positive; specific technical detail +0.2; no repeat across events; the no-other-home +0.2 does not apply: this wrap landed it as test-plan `decision/no-untied-viola-spawn`, its why «A census over one spelling of a constant missed a spawn site; a census over a constant's spawns matches any path ending in the constant», tx 2304, and the leaf `.claude/rules/testing.md:18` names the census). Nearest curated entry `.claude/docs/session-learnings.md:221` (a sweep finds only the wordings its patterns name) — same class, a code-path facet; not extended under the threshold.
- implement · the parent-death signal follows the spawning thread: not a curation entry — landed as test-plan `decision/tied-child-spawn-thread` (tx 2304), re-derived into `.claude/rules/testing.md:18` and `.claude/rules/verification-harness.md:42`.
- implement · guard refusal of a `cd` into the skill's references dir: not a learning — the guard put the call back on the documented path (guard-refused, telemetry).
- after implement · the pass's directive (exact cache hit, no cache deleted): not a learning — a per-chunk directive, kept.
- after implement · the pass's ruling on the supply-chain red (no re-run, no ignore): rejected — Filter 1 dup: CLAUDE.md «A red stays open until its cause is known: a green re-run never closes it…», security.md's ignore rule and the playbook's «External decay» rule hold it.
- after implement · inputs#I4 (the red's remedy asked at route-resolve; chrono four ways in the graph): not a learning — a route question, P5's.
- this window · the Bash guard refused a `cd` into the run dir and a pipe of `rec.py get` into `head`: not a learning — guard-refused, each re-issued once in the passing form.
