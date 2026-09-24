# Fan-out results — 2026-09-24-log-redaction-and-never-log-floor

| doc | verdict | proposals |
|---|---|---|
| architecture | drift | 2 (D-arch-decisions ×2, the second `dependent-of` the first) — raw twin `.raw-fanout-arch.md` |
| security-plan | no drift | `proposals: []` (D-security-input / -auth / -deps: no external input, no secret handling, no dependency added) |
| design-system | no drift | `proposals: []` (no UI element; no `hardcoded✗`) |
| layout-templates | no drift | `proposals: []` (no new surface; `viola run` clean path still prints nothing) |
| test-plan | no drift | `proposals: []` (every new path tested at unit/integration; runner on spec; harness/log format unchanged) |
| obs-plan | no drift | `proposals: []` (catch-site path instrumented per §7; chain only in the detail file per §8) |
| a11y-plan | no drift | `proposals: []` (no interactive element; no schema change) |

All returns arrived as bare YAML with trailing `#` comment lines; no preamble stripped; `entities=0` (no `&lt;`/`&gt;`/`&amp;` in any return).

## Validate
1. **Playbook:** both arch proposals match "Accurate this-chunk addition" (the named surfaces — `obs::report_internal_error`, `cmd::Failure`, anyhow in `src/obs.rs` — are in the report's Changes; the invariant "anyhow only in the root bin crate" holds; only the decision's wording of WHERE in the bin lagged, and obs-plan §7 layer 3 already required the chain in the detail file) → routine. Not a reversal of the locked decision: anyhow stays at the bin edge.
2. **Cross-contradiction:** none (two proposals, two sections, same direction).
3. **Intent-consistency:** the report's deviations (four subjects deferred for zero sites at HEAD; the `cli` stderr line deferred; step 6 split) are the P3 scope closure and the plan's own Constraints — justified, aligned.
4. **Absence needs evidence:** cascade sweep below, every hit dispositioned; post-sweep grep with a known-positive control.
5. **Expected amendments:** the plan lists none ("none anticipated") — nothing to reconcile.
6. **Disproved claims:** the report lists none.

**Escalations:** 0.

## Apply + cascade
- Applied: arch :27 (Stack, Error types row) and :95 ([Error Handling]) — re-derived from the invariant + report, not pasted.
- Cascade step 2 (sweep; patterns in architecture-amendments): obs :1117 amended (§7 Platform pick restated the dispatch-only scope). obs :54 — amended then REVERTED in the same pass: it sits in obs §1, the verbatim obs-scope copy that keeps its pending wording by rule (obs :485), which an earlier arch entry already applied to §1 hits; no change. security :447, arch :413, CLAUDE.md :28/:37, rules/observability.md :19, docs/services/viola.md :11/:28 — no change (still true).
- Cascade step 3 (leaves): `.claude/docs/stack.md:23`, `.claude/docs/conventions.md:34`, `.claude/docs/services/viola.md:6` re-derived. CLAUDE.md `GENERATED:setup:*` blocks recomputed against §Stack / §Established Decisions: no line states the amended scope (the modules line "the only crate with anyhow" still holds) — no change. obs leaves (`obs-summary.md`, `rules/observability.md`) carry neither amended sentence — no change.
- Sidecars: `architecture-amendments.md` +1 entry, `obs-plan-amendments.md` +1 entry (both read back: each entry is its file's last block).
