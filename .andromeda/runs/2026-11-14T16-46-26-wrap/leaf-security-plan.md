@leaf_group group=security-plan
@leaf_read leaf=.claude/docs/security-summary.md lines=77
last: **Full plan:** `.andromeda/security-plan.md`.
@leaf_read leaf=.claude/rules/security.md lines=44
last: _This section is owned by `/wrap-session`. setup-project pre
@leaf_edit n=1 leaf=.claude/docs/security-summary.md line=29 class=retired
leaf-text: pids, `agent_session_id`, budget percentages, OS username in paths
source: dec-qa23cxuy «decision/budget-gate-fails-open · Budget gate reads an untrusted file, failing open — The wrapper refuses a driver's send as budget-paused on budget.json»
replace-with: pids, `agent_session_id`, OS username in paths
@leaf_edit n=2 leaf=.claude/docs/security-summary.md line=64 class=retired
leaf-text: It reads `budget.json` through the one checked read `read_budget_stamped` and decides nothing on it.
source: dec-sq4u7teq «decision/failed-budget-read-no-reading · Failed budget read is no reading — Every failed read of the budget file counts as no reading.»
replace-with: It reads no budget file.
@leaf_edit n=3 leaf=.claude/docs/security-summary.md line=66 class=retired
leaf-text: The budget gate (chunk 2026-10-10-budget-governor): the wrapper reads `budget.json` through `viola_state::budget::read_budget` (`check_path` on the home and the file, a shared lock, `take(MAX_FRAME + 1)`, `v` = 1; any failure is no reading) and refuses a driver's `send` as `budget-paused` on it. `budget.json` is written by a hook arm and is not on the trusted-file list: this boundary widening is ratified by the founder's own answer, 2026-10-13, relayed verbatim by the Viola overseer (it had landed on the operator's word, 2026-10-10). The refusal also rests on the `rate_limits` shape, since chunk 2026-10-13-owed-ledger-rows the ledger row `statusline-rate-limits` (relied on every build, stamped on 2.1.287). Thresholds are `Percent` with a per-key fallback; no config value switches the gate off; the gate is fail-open.
source: dec-qa23cxuy «decision/budget-gate-fails-open · Budget gate reads an untrusted file, failing open — The wrapper refuses a driver's send as budget-paused on budget.json, read behind the strict-modes check on the home and the file»
replace-with: The budget gate and `budget.json` are removed (chunk 2026-11-14-budget-gate-removed): no send is refused for usage, and nothing reads or writes a budget file.
@leaf_edit n=4 leaf=.claude/docs/security-summary.md line=67 class=retired
leaf-text: `resets_at` is read as epoch seconds or RFC 3339.
source: ctr-rb7f2dby «contract/budget-json/resets-at · budget.json · resets_at — Read as epoch seconds or RFC 3339 and written as RFC 3339 UTC with milliseconds; a malformed value becomes "unknown".»
replace-with: The statusline hook reads no `rate_limits` and writes no file.
@leaf_edit n=5 leaf=.claude/rules/security.md line=27 class=stale
leaf-text: `budget.json` is written only by this arm, at 0600 under its lock, only when the payload holds `rate_limits`.
source: dec-pbuahe7x «decision/statusline-payload-fails-silent · Statusline payload fails silent — The statusline hook payload takes the hook stdin cap and one-object rule, each failure ending in exit 0, empty stderr and nothing printed; the hook reads no rate_limits and writes no file.»
replace-with: A malformed or oversize payload ends in exit 0, empty stderr and nothing printed; the hook reads no `rate_limits` and writes no file.
@leaf_edit n=6 leaf=.claude/rules/security.md line=28 class=retired
leaf-text: Budget gate: `config.json`'s `five_hour` / `seven_day` are whole numbers into `Percent` (0–100, defaults 90 / 85), read once at the wrapper's start through `take(MAX_FRAME)` (and by `viola list` at each run, through the same `read_thresholds`); a bad value takes that key's default with one `parse-rejected{parser:"config-json"}` line per key, and no value switches the gate off. The dialog hold `dialog_hold_s` is read once at the wrapper's start right after the thresholds (through `take(MAX_FRAME)`)
source: ctr-qvs5uxd2 «contract/config-json/five-hour · config.json · five_hour — A JSON whole number 0 to 100, default 90; any other value takes the default with one parse-rejected config-json malformed line; no value switches the budget gate off.»
replace-with: Dialog hold (the budget gate and `config.json`'s `five_hour` / `seven_day` are removed, chunk 2026-11-14-budget-gate-removed): `dialog_hold_s` is read once at the wrapper's start (through `take(MAX_FRAME)`)
@leaf_edit n=7 leaf=.claude/rules/security.md line=28 class=retired
leaf-text: The gate reads `budget.json` only through `viola_state::budget::read_budget` (`check_path` on the home and the file, a shared lock, `take(MAX_FRAME + 1)`, tolerant parse, `v` = 1); every failure is no reading with no line, and the gate is fail-open. `budget.json` is not on the trusted-file list: the wrapper refusing a driver's `send` on it is a boundary widening the founder ratified (his own answer, 2026-10-13, relayed verbatim by the Viola overseer; it had landed on the operator's word, 2026-10-10, which is no ratification).
source: dec-qa23cxuy «decision/budget-gate-fails-open · Budget gate reads an untrusted file, failing open — The wrapper refuses a driver's send as budget-paused on budget.json, read behind the strict-modes check on the home and the file»
replace-with: No `budget.json` is read or written, and no `send` is refused for usage.
@leaf_edit n=8 leaf=.claude/rules/security.md line=28 class=stale
leaf-text: A `release` with `budget: true` and a string `from` is refused first and lifts nothing.
source: ctr-rd2qbhhd «contract/channel-frames/release-params · channel frames · release params — from is absent or null, a string from is -32602 with data reason release-from-driver, any other type -32602 with data null; after those checks any budget key»
replace-with: A `release` with a string `from` is refused first (`release-from-driver`); after the `from` checks any `budget` key, of any value, is `-32602` invalid-params with `data: null`, no wheel moved and no record written.
