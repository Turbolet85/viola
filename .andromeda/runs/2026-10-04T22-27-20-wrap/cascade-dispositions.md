# Cascade dispositions — 2026-10-04-running-turn-refusal

## The search
`cascade.py sweep --patterns-file cascade-patterns.toml` (baseline `eb37914c`, the pre-CI parent), over the seven
masters, `.andromeda/registries/**`, the three curation homes, the two judgment bases and the leaf bodies. Patterns,
each from a claim this pass retired or restated — its wording and its mechanism:
- `no-turn-state` `no running-turn state` · `wheel-alone` `returns the wheel alone` · `is-to-clear` `is to clear the
  running-turn state` · `two-field-lock` `holder and cause under one lock` — the retired arch claim and the two-field
  model, by wording;
- `release-clears` (regex, case-folded) `release … clears nothing | clear(s) the running` — the release↔turn mechanism
  however worded;
- `tr-cause` (regex) `turn-running … in flight | second send` — the detail stated with only its in-flight cause;
- `exits-on-ctrl-c` (regex) ``xits on `\x03` `` — the fake agent's exit, the test-plan §7 fact;
- `four-negatives` `four control negatives` — the controls table's count.
- Dropped: `harness-send-0` (a harness turn's `send` exiting 0, the hypothesis the chunk closed) — its control never
  fired over the pre-pass masters: no master stated the harness-turn send's outcome (research.md §Files inspected read
  test-plan §6 Path 5's harness-turn signal as naming none). A statement about that pattern, not an absence proof.

Every control fired (exit 0). Five patterns read 0 rows after the pass (`no-turn-state`, `wheel-alone`, `is-to-clear`,
`two-field-lock`, `four-negatives` — the retired wording gone from every swept file).

## Rows (4)
1. `.andromeda/architecture.md:70` · `release-clears` · new @c3710 — the amended text itself ("the `viola release`
   that returns the wheel clears the running-turn state"); true as measured. **amended** (no further change).
2. `.andromeda/architecture.md:136` · `tr-cause` · standing — §Conventions error-handling `send` order: "`turn-running`
   (a turn running, or another `send` in flight)" already names both causes. A true claim sharing the token.
   **no change**.
3. `.andromeda/test-plan.md:1077` · `exits-on-ctrl-c` · standing, edited — the amended line ("It exits on `\x03` or at
   stdin EOF …"); the swept text stands inside the new truth. **amended** (no further change).
4. `.claude/rules/verification-harness.md:42` · `exits-on-ctrl-c` · leaf — "Exits on `\x03`." (the rule body's Fake
   agent modes line, not its Session Additions). **re-derived**: exits on `\x03` or at stdin EOF, first waiting for a
   running hook and starting no other.

## Leaves (step 3, by the table and provenance)
- architecture → CLAUDE.md `GENERATED:setup:*`: recomputed against [Human Takeover / Wheel] — overview / modules
  ("wheel and budget governor", "the `run` pump … wheel") and warnings name no running-turn mechanism; **no change**.
  docs: `stack.md` · `conventions.md` · `commands.md` · `gotchas.md` carry no wheel/turn claim (grep `running-turn|
  running turn|turn-running|holder and cause|wheel alone` over `.claude/docs` + `.claude/rules`: 4 files hit, all
  dispositioned here); `services/viola.md:23` — "`send` during a running turn → `turn-running`" was true but carried
  no mechanism; **re-derived** with where the turn lives, what marks / ends / clears it and the human-typing reading.
- test-plan → `tests-summary.md`: `:26` Path 2 ("a second send in flight → exit 13 `turn-running`") **re-derived**
  (either cause; the first turn ended by a scripted Stop; the driver-own-turn case); `:29` Path 5 **re-derived** (the
  harness-turn refusal until `turn-ended`; `pause` then `release` recovery); `:21` fake agent names no exit
  behaviour, **no change**. Rules it scopes: `testing.md` (no hit) **no change**; `verification-harness.md:42`
  **re-derived** (row 4). CLAUDE.md `GENERATED:setup:warnings`: no test-plan claim touched, **no change**.
- obs-plan → `obs-summary.md`, `observability.md`, `events.md`: no `turn-running` cause stated (grep 0 hits); **no
  change**. `.claude/rules/api.md:23` lists the `send` refusal order (unchanged by this chunk); **no change**.
- Binds: test-plan §3 ↔ obs-plan §3 untouched (no verb, status shape, log format or CI step changed); a11y ↔ obs
  schema untouched.
- Curation homes and judgment bases: 0 rows across every pattern; nothing routed to P3 or the propose channel.
