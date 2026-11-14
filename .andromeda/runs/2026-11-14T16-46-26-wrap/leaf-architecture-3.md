@leaf_group group=architecture-3
@leaf_read leaf=.claude/docs/services/viola.md lines=58
last: - `doc:architecture` · `.claude/rules/api.md` · `.claude/rul
@leaf_read leaf=.claude/docs/stack.md lines=66
last: 5. Commit as `chore: bump {tool} to {version}`
@leaf_read leaf=.claude/docs/workflow.md lines=67
last: This project uses a **reference-based** CLAUDE.md architectu
@leaf_edit n=1 leaf=.claude/docs/services/viola.md line=6 class=retired
leaf-text: the PTY pump, the wheel, the budget governor, the readiness-gate wiring
source: dec-utofgtfh «The root bin holds the subcommand dispatch, the run pump and the wheel, all of which consume only normalised events.»
replace-with: the PTY pump, the wheel, the readiness-gate wiring
@leaf_edit n=2 leaf=.claude/docs/services/viola.md line=6 class=stale
leaf-text: all twenty-four of `LedgerRow::ALL`
source: dec-ucvfrmxd «adding one needs a 24th row, its viola verify probe and a re-stamp of every stamped version.»
replace-with: all twenty-three of `LedgerRow::ALL`
@leaf_edit n=3 leaf=.claude/docs/services/viola.md line=20 class=retired
leaf-text: → the budget gate at start (span `run.budget_eval`, `trigger: start`: reads `budget.json`, restores no override, writes nothing) → first snapshot (with `endpoint`; its `budget_paused` that verdict) + heartbeat
source: dec-wx7tk6ur «gates the version, binds the endpoint, writes the first snapshot, heartbeat and start events, and only then spawns the child.»
replace-with: → first snapshot (with `endpoint`) + heartbeat
@leaf_edit n=4 leaf=.claude/docs/services/viola.md line=20 class=retired
leaf-text: → `wheel` + `budget-gate` events → child spawn
source: dec-yyuuzpxg «Event kinds the wrapper appends (`wheel`, `send-issued`, `send-confirmed`, `send-refused`, `link`, `unlink`) are log-only and never wake `wait`.»
replace-with: → `wheel` event → child spawn
@leaf_edit n=5 leaf=.claude/docs/services/viola.md line=23 class=retired
leaf-text: no instance check, no `budget.json` write, no command run, nothing printed)
source: ctr-tvjufgqj «With `--capture <DIR>` the arm takes `viola verify`'s probe capture: no instance check, no command run, nothing printed; without it the pass-through is unchanged.»
replace-with: no instance check, no command run, nothing printed)
@leaf_edit n=6 leaf=.claude/docs/services/viola.md line=23 class=retired
leaf-text: reads stdin through `take(MAX_FRAME + 1)`, writes `budget.json` only with a `rate_limits` object, reads the snapshot
source: abs-ywmx3ndp «A usage-budget gate: no subscription rate-limit reading, no budget state, no send paused or refused on usage»
replace-with: reads stdin through `take(MAX_FRAME + 1)`, reads the snapshot
@leaf_edit n=7 leaf=.claude/docs/services/viola.md line=24 class=retired
leaf-text: - The budget gate (`src/run/budget.rs`, chunk 2026-10-10-budget-governor): one `BudgetGate` per wrapper, no thread and no timer. `read_thresholds` reads `config.json`'s `five_hour` / `seven_day` once at start (defaults 90 / 85; a bad value takes its default with one `parse-rejected` line per key). The pure `decide(reading, thresholds, override_until, now)` crosses a window only when its percentage is known and at or over its threshold and its `resets_at` is a known, parseable, future instant; with both crossed the detail is `five-hour`. Three evaluation points: `start` (the first snapshot's `budget_paused` and record two), a `send`'s arrival (after `human-typing`, before `turn-running`; refusal `budget-paused`, exit 11) and each `turn-ended`. `budget-gate` and the snapshot's `budget_paused` / `budget_override_until` are written only when the triple (paused, window, override end) changes. `release --budget` (`lift`) records `{paused:false, override_until}` only when a pause stands; an override does not outlive its wrapper. Fail-open: no reading, an unknown figure, a passed reset, an unreadable `budget.json` and a bad threshold never pause. `answer`, `wait`, `last`, `pause` and `hook.dialog` do not read the gate. Human lines: `<name>  budget gate lifted until <end>  wheel <holder>` and `<name>  budget gate open  wheel <holder>` (the date form of `<end>` and the open line are the founder's wording).
source: abs-ywmx3ndp «A usage-budget gate: no subscription rate-limit reading, no budget state, no send paused or refused on usage»
replace-with: - No usage-budget gate: no subscription rate-limit reading, no budget state, no send paused or refused on usage.
@leaf_edit n=8 leaf=.claude/docs/services/viola.md line=27 class=retired
leaf-text: - Wheel / budget / dialog rules as in arch:
source: abs-ywmx3ndp «A usage-budget gate: no subscription rate-limit reading, no budget state, no send paused or refused on usage»
replace-with: - Wheel / dialog rules as in arch:
@leaf_edit n=9 leaf=.claude/docs/services/viola.md line=27 class=retired
leaf-text: only CLI `release` returns the wheel; `budget-paused` gates `send` only; a pending dialog
source: ctr-xguevze4 «control-character, empty-text, human-typing, turn-running, the readiness gate (input-not-ready), human-typing and turn-running read again after the gate, then no-prompt-submitted.»
replace-with: only CLI `release` returns the wheel; a pending dialog
@leaf_edit n=10 leaf=.claude/docs/services/viola.md line=36 class=retired
leaf-text: prints nothing and reads no `budget.json` or snapshot.
source: ctr-tvjufgqj «With `--capture <DIR>` the arm takes `viola verify`'s probe capture: no instance check, no command run, nothing printed; without it the pass-through is unchanged.»
replace-with: prints nothing and reads no snapshot.
@leaf_edit n=11 leaf=.claude/docs/services/viola.md line=41 class=retired
leaf-text: (pump, wheel, governor, gate wiring;
source: dec-utofgtfh «The root bin holds the subcommand dispatch, the run pump and the wheel, all of which consume only normalised events.»
replace-with: (pump, wheel, gate wiring;
@leaf_edit n=12 leaf=.claude/docs/services/viola.md line=41 class=retired
leaf-text: `params.rs` `parse` / `parse_from`; `budget.rs` `Ctx::budget_paused`; and beside
source: ctr-xguevze4 «control-character, empty-text, human-typing, turn-running, the readiness gate (input-not-ready), human-typing and turn-running read again after the gate, then no-prompt-submitted.»
replace-with: `params.rs` `parse` / `parse_from`; and beside
@leaf_edit n=13 leaf=.claude/docs/services/viola.md line=41 class=retired
leaf-text: through `run::dialog::read_hold` right after the thresholds)
source: abs-ywmx3ndp «A usage-budget gate: no subscription rate-limit reading, no budget state, no send paused or refused on usage»
replace-with: through `run::dialog::read_hold`)
@leaf_edit n=14 leaf=.claude/docs/services/viola.md line=45 class=retired
leaf-text: , and `budget.json` through `read_budget_stamped`, the thresholds through `run::budget::read_thresholds` at each run; the `--json` `{items, budget, skipped}` document
source: ctr-t5xneaha «The `list` `ok` payload, CLI `--json` and MCP `list` alike, is `{items, skipped}`; `list` is not a channel method and needs no live wrapper.»
replace-with: ; the `--json` `{items, skipped}` document
@leaf_edit n=15 leaf=.claude/docs/services/viola.md line=49 class=retired
leaf-text: (exit-code mapping, wheel state machine, thresholds, confirmation matcher
source: abs-ywmx3ndp «A usage-budget gate: no subscription rate-limit reading, no budget state, no send paused or refused on usage»
replace-with: (exit-code mapping, wheel state machine, confirmation matcher
@leaf_edit n=16 leaf=.claude/docs/stack.md line=15 class=retired
leaf-text: (wheel mirror, links, budget readings)
source: abs-ywmx3ndp «A usage-budget gate: no subscription rate-limit reading, no budget state, no send paused or refused on usage»
replace-with: (wheel mirror, links)
@leaf_edit n=17 leaf=.claude/docs/stack.md line=21 class=retired
leaf-text: `ViolaName` (charset + length cap), `Percent` (0–100), `DialogHold` (whole seconds 60–3600)
source: dec-vvbdbaq4 «Bounded domain values are nutype newtypes: ViolaName (charset and length cap), DialogHold (whole seconds 60 to 3600).»
replace-with: `ViolaName` (charset + length cap), `DialogHold` (whole seconds 60–3600)
@leaf_edit n=18 leaf=.claude/docs/workflow.md line=49 class=stale
leaf-text: all twenty-four on every build
source: dec-ucvfrmxd «adding one needs a 24th row, its viola verify probe and a re-stamp of every stamped version.»
replace-with: all twenty-three on every build
