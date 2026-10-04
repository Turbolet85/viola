# Cascade dispositions — 2026-10-04-wait-and-last

**The search:** `cascade.py sweep` over `cascade-patterns.toml` (17 patterns, derived from every amendment of this pass
before the first grep), across the seven masters + `.andromeda/registries/**`, the leaf bodies, the three curation
homes and the two judgment bases; every pattern's known-positive control fired on the pre-pass text (baseline
`bcff692`). Sections read beside it: architecture §Conventions exit `1` · §Standard Contracts channel methods ·
§Established Decisions [Database / State Store] + [Message Broker / IPC] · §Occupied Resources `diagnostics/` · the
two keyed files; security-plan §Authentication & Authorization (IPC row, home row) · §Input Validation (channel
frames, CLI arguments, own state files) · §Security Anti-Patterns → Authentication; obs-plan §4 Scenario `wait` /
`last`; test-plan §6 Path 3; layout-templates §Surface: cli (signature placement, Output structure wait/last,
Primary content block 2); design-system cli Component Patterns 3.

## Retired claims — 0 rows left (each control fired on the pre-pass text)
`send-arm` (the `Send` dispatch arm printing the internal error) · `verify-only-cli` (only `verify` filed `cli`; a
`send` panic printing nothing) · `verify-internal` · `last-empty` (`last` `{}`) · `wait-params` (`{after?,
timeout_ms?}` without `from`) · `producers` (`run`, `hook` and `verify` the only producers) · `borrow-either` (the
two-exception NEVER-write tail) · `wait-last-after` (`after` / `timeout_ms` on `last` too) · `human-callers`
(`human.rs` called by run / verify / send only).

## Rows and dispositions
- `send-gap` · security-plan:207, :230 — standing lines this pass EDITED (the fifth gap appended beside `send`'s);
  the `send` clause stays true. → amended.
- `heal` · architecture:50 — edited (the as-landed qualifier; "Readers heal" stays the locked decision, healing owed
  to `:85`). → amended. test-plan:47 · obs-plan:71 · obs-plan:278 — crate-responsibility / scenario lists naming
  torn-line healing as `viola-state`'s planned duty; still the plan (owed, not retired). → no change. Leaves
  CLAUDE.md:24 · `.claude/rules/events.md:34` · `.claude/docs/services/viola-state.md:6` → re-derived (step 3).
- `wait-detail3` · `.claude/docs/obs-summary.md:68` (leaf) — D-20's exit-21 code catalog is the cross-role union
  (obs-plan §6), unchanged; the narrowing is §4's wait/last scenario only. → no change.
- `line-cap` · security-plan:235 — edited (the reader clause appended); the replay/tail rule stays. → amended.
- `send-served` · security-plan:205 ×2 — edited (`send`'s served-as sentence stands; the wait/last one added). →
  amended.
- `fourth-gap` · security-plan:205 (the row's "Second interim gap", `send`'s, stands; the third, wait/last's, added) ·
  security-plan:230 ("A fourth interim gap", `send`'s, stands; the fifth added). → amended.
- `wake-polls` · architecture:52 ("nothing polls" — true as landed) → no change · architecture:276 → new text
  (this pass) · design-system:846 ("generations" of a design choice) · obs-plan:552 ("snapshot generation") — a true
  claim sharing the token. → no change.
- `envelope-from` · architecture:274-275 · security-plan:224 → new text (this pass) · security-plan:506 (`-32600`
  for an over-cap frame) — a true claim sharing the token. → no change.

## Leaves re-derived (step 3)
`CLAUDE.md` `GENERATED:setup:modules` (`viola-state` line) · `.claude/rules/security.md` (the fifth exception) ·
`.claude/rules/events.md` (reader as landed) · `.claude/rules/api.md` (method params, envelope) ·
`.claude/docs/security-summary.md` (interim gaps) · `.claude/docs/tests-summary.md` (Path 3 as landed) ·
`.claude/docs/services/viola.md` (`cli` role, wrapper `wait.rs`, the wait / last verbs) ·
`.claude/docs/services/viola-state.md` (the events reader) · `.claude/docs/services/viola-channel.md` (dispatch_call,
envelope, wait log fields) · `.claude/docs/services/viola-core.md` (13 kinds, `WAIT_WAKE`). Read and unchanged
(still true): `.claude/rules/frontend.md:38-39` · `.claude/rules/observability.md:37` · `.claude/docs/obs-summary.md`
· `.claude/docs/design-summary.md` · `.claude/docs/conventions.md` (no exit-1 / role text) ·
`.claude/docs/commands.md` (no channel-method text). Curation homes and judgment bases: 0 rows.
