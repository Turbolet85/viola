# Session Handoff

**Last Updated:** 2026-11-08T21:07:38Z
**Branch:** build/viola-0.2.0 · upstream set, 0 ahead as read at this wrap's Setup (this wrap's push lands its commit)
**Status:** 0.2.0 census complete — every row applied, noted, held with a reason, or skipped
**Last Commit:** chore(route): operator-requested adaptation — 0-pending wrap (the 0.2.0 census, part 2)

## Position
- Done: route 0.2.0 (25 markerless entries, ledger v2-01…v2-28) and the version census, both parts
  (`.andromeda/runs/2026-11-08T20-32-52-wrap/`, `.andromeda/runs/2026-11-08T21-01-50-wrap/` — each `adaptation-record.md`).
- Next: the first 0.2.0 chunk, «Epoch 8 cleanup» (`/andromeda-new-session`).
- Held, each with its reason in part 2's record: C-240 (premise false: the a11y plan reads the tier), C-301 (waits for
  «Status-line interception removed»), C-565 (premise false: `tests/support/piped.rs` still answers DA1), `doc:input`'s
  23 rows (doc:input is never mutated — the overseer's answer; he files its census inclusion as a pipeline finding).
- Observed, unowned: dead `#[cfg(not(unix))]` constants in `crates/viola-e2e/src/harness/run.rs` (:349, :1026, :1097).

## Work done
- 14 dated `matrix.py note`s on 0.1.0 caps (C-660…C-704); `project.yaml` description Linux-only (C-488); ten harness
  files landed TX 789–798: preserve-verbatim entries tagged in place, four doc leaves replaced.

## Drift resolved
- The Windows/macOS and web entries in the rule files' Session Additions, session-learnings and gotchas now carry
  their census retirement tag; the four stale doc-leaf clauses (test prefixes, «on every OS», SSE names) are gone.

## Notes
- **Owned by 0.2.0** (carried): the 3600 s hold ceiling, PROVISIONAL; the live round's two reds; matrix#v1-27 deferred.
- **Carried, unchanged:** `chunks/2026-10-26-explicit-session-links`'s card confirm and P2 halt;
  `chunks/2026-10-27-bounded-inputs-at-every-boundary`'s P2 halt Q1 and Path 1's runner-variance reading;
  `obs-scope-summary#20.5` and `slo-invariants-telemetry-budgets#2.2` label `max` readings the report labels `mean`.
- **Deferred learnings (carried):** as at the 2026-11-06 wrap (eight items, unchanged).
- **Host:** python `yaml` is absent; read `df -h /tmp` before a stamped-home gate. `rec.py amend` refuses
  `marker=no-marker` in a payload ("a block no entry"): a 0-pending wrap's entries carry the run-dir name as marker.
  A design-system category file cannot be emptied by `retire` (each kept one N/A row).
- **Last failed command:** none.
