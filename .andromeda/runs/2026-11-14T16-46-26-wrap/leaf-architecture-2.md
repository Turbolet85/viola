@leaf_group group=architecture-2
@leaf_read leaf=.claude/docs/services/viola-channel.md lines=45
last: - `doc:architecture` §Standard Contracts (Wrapper channel fr
@leaf_read leaf=.claude/docs/services/viola-core.md lines=43
last: - `.claude/docs/stack.md` · `.claude/docs/conventions.md` · 
@leaf_read leaf=.claude/docs/services/viola-mcp.md lines=39
last: - `doc:architecture` [MCP] · `.claude/rules/api.md`
@leaf_read leaf=.claude/docs/services/viola-pty.md lines=47
last: - `doc:architecture` [PTY] · `.claude/docs/gotchas.md` (ConP
@leaf_read leaf=.claude/docs/services/viola-state.md lines=49
last: - `doc:architecture` §Standard Contracts (snapshot envelope,
@leaf_edit n=1 leaf=.claude/docs/services/viola-core.md line=6 class=stale
leaf-text: the nutype newtypes `ViolaName`, `Percent` and `DialogHold` (whole seconds 60–3600)
source: dec-ug62hew5 «decision/nutype-newtypes · nutype newtypes for validation — Validated values are nutype newtypes (`ViolaName`, `DialogHold`); no garde or validator.»
replace-with: the nutype newtypes `ViolaName` and `DialogHold` (whole seconds 60–3600)
@leaf_edit n=2 leaf=.claude/docs/services/viola-core.md line=11 class=stale
leaf-text: nutype 0.8.0 (`ViolaName`, `Percent`, `DialogHold`)
source: dec-ug62hew5 «decision/nutype-newtypes · nutype newtypes for validation — Validated values are nutype newtypes (`ViolaName`, `DialogHold`); no garde or validator.»
replace-with: nutype 0.8.0 (`ViolaName`, `DialogHold`)
@leaf_edit n=3 leaf=.claude/docs/services/viola-core.md line=20 class=retired
leaf-text: `EventKind` holds 16 kinds (the length of `EventKind::ALL`)
source: ctr-zpc6ox2b «contract/normalised-event-kinds/budget-gate · normalised event kinds · budget-gate — `budget-gate` is a normalised event kind.»
replace-with: `EventKind` holds 15 kinds (the length of `EventKind::ALL`; `budget-gate` is no longer one)
@leaf_edit n=4 leaf=.claude/docs/services/viola-core.md line=22 class=retired
leaf-text: - The budget reading: `BudgetReading {five_hour, seven_day}`, each a `BudgetWindow {used_percentage: Reading<f64>, resets_at: Reading<String>}`; a `Reading` is a value or the word `"unknown"`. It carries no timestamp and no Claude name. `BudgetWindowKind { FiveHour, SevenDay }` is the closed window word (serde kebab-case, `five-hour` · `seven-day`; a third word does not deserialize), the `budget-paused` refusal's detail. `Percent` is `nutype` over `u8` with `less_or_equal = 100`, built only through `Percent::try_new`; the two thresholds are `Percent`, the reading's `used_percentage` stays an `f64`.
source: abs-ywmx3ndp «absence/usage-budget-gate — A usage-budget gate: no subscription rate-limit reading, no budget state, no send paused or refused on usage»
replace-with: - No usage-budget gate: no subscription rate-limit reading and no budget state, so this crate holds no budget reading, window word, threshold or `budget-paused` refusal.
@leaf_edit n=5 leaf=.claude/docs/services/viola-state.md line=6 class=retired
leaf-text: `budget.json`, `ledger/stamps.json` reads (`config.json` is read by the root bin, `run::budget::read_thresholds` and `run::dialog::read_hold`, not here)
source: dec-wdxulb5x «decision/budget-json-single-writer · budget.json written only by the statusline hook — budget.json holds the newest rate_limits reading across wrapped sessions, last writer wins, written whole only by hook statusline»
replace-with: `ledger/stamps.json` reads (`config.json` is read by the root bin, `run::dialog::read_hold`, not here)
@leaf_edit n=6 leaf=.claude/docs/services/viola-state.md line=30 class=retired
leaf-text: only `hook statusline` writes `budget.json` (under its lock, last-writer-wins): `budget::write_budget` replaces it whole through `replace_private` at 0600 as `{"v":1,"five_hour","seven_day","read_at"}`, and only when the payload held a `rate_limits` object. Its one checked read is `budget::read_budget_stamped(home) -> Option<StampedReading>` (`{reading, read_at}`, `read_at` `"unknown"` when absent or no string; chunk 2026-10-11-the-board-viola-list), with two product paths: `budget::read_budget(home) -> Option<BudgetReading>` (= `read_budget_stamped(home).map(|s| s.reading)`), called by the wrapper's budget gate, and `viola list`.
source: dec-wdxulb5x «decision/budget-json-single-writer · budget.json written only by the statusline hook — budget.json holds the newest rate_limits reading across wrapped sessions, last writer wins, written whole only by hook statusline»
replace-with: no process writes or reads a `budget.json` (no budget reading is kept).
@leaf_edit n=7 leaf=.claude/docs/services/viola-state.md line=30 class=retired
leaf-text: The read runs: `strict::check_path` on the home and on `budget.json`, a shared lock on `budget.json.lock`, `take(MAX_FRAME + 1)`, a tolerant parse, `v` = 1; every failure is `None` with no line, it never writes, and an absent file creates nothing. The snapshot gains an optional `budget_override_until` (the wrapper's write; `v` stays 1).
source: abs-ywmx3ndp «absence/usage-budget-gate — A usage-budget gate: no subscription rate-limit reading, no budget state, no send paused or refused on usage»
replace-with: The snapshot holds no budget field (`budget_paused` and `budget_override_until` are gone; `v` stays 1).
@leaf_edit n=8 leaf=.claude/docs/services/viola-state.md line=45 class=retired
leaf-text: (the 16 kinds round-trip
source: ctr-zpc6ox2b «contract/normalised-event-kinds/budget-gate · normalised event kinds · budget-gate — `budget-gate` is a normalised event kind.»
replace-with: (the 15 kinds round-trip
