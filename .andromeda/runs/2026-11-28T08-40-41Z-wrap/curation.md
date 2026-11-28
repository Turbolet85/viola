CLAUDE.md ecosystem curated:
  Tier 1 (CLAUDE.md USER:session-learnings):  none
  Tier 2 (.claude/rules/*):                   + observability.md: "A new `ObsEvent` moves three lists, not two: both schemas' event enums AND the pinned `LOG_FORMAT_EVENTS` array in `tests/contract_diag_schema.rs` … only `run --e2e` reads it red."
    Proof: the implement run's first firing of `bash scripts/agent-run.sh run --e2e` (2026-11-28T08:03:44Z) red on `contract_diag_schema diag_line_schema_event_enum_equals_the_log_format_literals`, one failed assertion (report, Decisions & corrections; outcome file Gates).
  Tier 3 (.claude/docs/session-learnings.md): none
  Filters: 0 dup · 2 task-specific · 0 conflict · 0 deferred
  CLAUDE.md size: 136/200 · T1 4.8 KB, 0 over 600 B

Dispositions, each Decisions & corrections entry:
- implement — the implement directive (inputs#I6): not a learning — a run's directive, its words held by inputs#I6.
- implement — sweep hazard, the pinned `LOG_FORMAT_EVENTS` list: applied (Tier 2, observability.md).
- implement — the termios read «on the master's fd» unreachable from a root test: not a learning here — it has a
  master home, landed as test-plan `decision/console-restore-read-on-own-terminal` (P2, tx 5299).
- implement — Bash guard refusals, each re-issued once: rejected — Filter 2, task-specific (the guard's own form,
  already the host rule).
- after implement — the wrap directive (inputs#I7): not a learning — a run's directive.
- this window — a `fanout-results.md` edit by script asserted a needle twice (the replacement text itself held it):
  rejected — Filter 2, task-specific to one script.
