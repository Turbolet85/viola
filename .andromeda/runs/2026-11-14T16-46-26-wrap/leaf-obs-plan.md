@leaf_group group=obs-plan
@leaf_read leaf=.claude/docs/obs-summary.md lines=76
last: **Full plan:** `.andromeda/obs-plan.md`. Path-scoped rules:
@leaf_edit n=1 leaf=.claude/docs/obs-summary.md line=13 class=stale
leaf-text: obs may add fields, never rename or remove them.
source: dec-5as2d3rg «decision/obs-aligns-to-tests-log-format · Observability aligns to the tests log format — The log format is bound by the test plan's harness contract; observability may add fields but must not rename them or remove one the product still emits.»
replace-with: obs may add fields, never rename them or remove one the product still emits.
@leaf_edit n=2 leaf=.claude/docs/obs-summary.md line=69 class=stale
leaf-text: `hook-decision{budget_written, duration_ms}`
source: ctr-5ebkr3a5 «contract/statusline-log-fields/hook-decision · statusline hook log fields · hook-decision — hook-decision{hook_event:"statusline", duration_ms}, deadline_hit only when true.»
replace-with: `hook-decision{duration_ms}`
@leaf_edit n=3 leaf=.claude/docs/obs-summary.md line=69 class=retired
leaf-text: runs no instance check, no shell-out and no `budget.json` write:
source: ctr-5hdp5oqe «contract/budget-span-attributes/state-budget-write · budget span attributes · state.budget_write — state.budget_write carries five_hour_pct and seven_day_pct.»
replace-with: runs no instance check and no shell-out:
@leaf_edit n=4 leaf=.claude/docs/obs-summary.md line=71 class=retired
leaf-text: the per-key `config-json` threshold lines and the `agents-probe` spawn pair
source: ctr-4jy5xleo «contract/config-json/five-hour · config.json · five_hour — Key five_hour holds a whole percentage from 0 to 100; any other value is logged as one parse-rejected{parser:"config-json", detail:"malformed"} WARN line.»
replace-with: the `agents-probe` spawn pair
@leaf_edit n=5 leaf=.claude/docs/obs-summary.md line=71 class=retired
leaf-text: `parse-rejected{parser:"state-entry"}`, per-key `config-json` and `agents-probe` lines
source: ctr-4jy5xleo «contract/config-json/five-hour · config.json · five_hour — Key five_hour holds a whole percentage from 0 to 100; any other value is logged as one parse-rejected{parser:"config-json", detail:"malformed"} WARN line.»
replace-with: `parse-rejected{parser:"state-entry"}` and `agents-probe` lines
@leaf_edit n=6 leaf=.claude/docs/obs-summary.md line=72 class=retired
leaf-text: The budget gate (as landed, chunk 2026-10-10-budget-governor): `run.budget_eval` (INTERNAL; `trigger` `start|send|turn-ended`, `paused`, `window` only when paused), declared once, opened once per evaluation; the start's span chain holds it between `channel.bind` and `state.snapshot_write`. A `budget-paused` refusal is one `send-refused` line with `window`, `used_percentage`, `resets_at` and `paused:true`, client `exit_code:11`. `budget-gate` is appended only when (paused, window, override end) changes. No span and no line of their own: `read_budget` (a refused or unreadable `budget.json` is no reading) and `release --budget`'s lift.
source: ctr-5c7jj5fz «contract/send-refused/budget-paused · wrapper send-refused · budget-paused — send-refused{corr:<cursor>, refusal:"budget-paused", detail:"five-hour"|"seven-day", window, used_percentage, resets_at, paused:true}, with client exit_code 11.»
replace-with: The budget gate (landed in chunk 2026-10-10-budget-governor) is retired: no `run.budget_eval` span (the start's chain runs `channel.bind` › `state.snapshot_write`), no `budget-paused` refusal (exit 11 retired, never reused), no `budget-gate` record, no `budget.json` read or written; a `release` carrying any budget key (true, false, a string or null) is a plain `-32602` after the `release-from-driver` check, no `release-from-driver` line, no wheel moved and no record, its `channel-response` carrying `detail:"invalid-params"` on both sides at WARN.
@leaf_edit n=7 leaf=.claude/docs/obs-summary.md line=72 class=stale
leaf-text: Each bad threshold key is one `parse-rejected{parser:"config-json", detail:"malformed"}` at start, and a `dialog_hold_s` that is no whole number of seconds from 60 to 3600 one more (`run::dialog::read_hold`, the hold then 60); `CONFIG_KEYS` holds six names.
source: ctr-4hxbzvo4 «contract/config-json/known-keys · config.json · known keys — config.json has four known keys: v, diagnostics_level, claude_env_keep, dialog_hold_s; any other key, a retired five_hour or seven_day included, is counted as skipped»
replace-with: `config.json` has four known keys (`v`, `diagnostics_level`, `claude_env_keep`, `dialog_hold_s`); any other key, a retired `five_hour` or `seven_day` included, is counted in one `parse-rejected{parser:"config-json", detail:"unknown-keys"}` line at its process's start and nothing refuses; a `dialog_hold_s` that is no whole number of seconds from 60 to 3600 is one `parse-rejected{parser:"config-json", detail:"malformed"}` (`run::dialog::read_hold`, the hold then 60).
