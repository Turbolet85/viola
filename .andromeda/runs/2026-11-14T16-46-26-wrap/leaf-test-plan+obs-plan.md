@leaf_group group=test-plan+obs-plan
@leaf_read leaf=.claude/rules/observability.md lines=46
last: - 2026-09-24: `obs_event!` expands to `::tracing::event!`, s
@leaf_edit n=1 leaf=.claude/rules/observability.md line=38 class=retired
leaf-text: (the latter runs no instance check, no shell-out and no `budget.json` write, opens no role file and prints nothing)
source: ctr-kuzlzznh «contract/viola-hook-statusline/budget-json · viola hook statusline · budget.json — rate_limits is written to budget.json only when the payload holds a rate_limits object.»
replace-with: (the latter runs no instance check and no shell-out, opens no role file and prints nothing)
@leaf_edit n=2 leaf=.claude/rules/observability.md line=39 class=stale
leaf-text: the `config-json` threshold rejections and one `agents-probe` spawn pair
source: ctr-4hxbzvo4 «contract/config-json/known-keys · config.json · known keys — config.json has four known keys: v, diagnostics_level, claude_env_keep, dialog_hold_s; any other key, a retired five_hour or seven_day included, is counted»
replace-with: the `config-json` `parse-rejected` lines (a retired `five_hour` or `seven_day` key counted as skipped under `unknown-keys`, nothing refused) and one `agents-probe` spawn pair
@leaf_edit n=3 leaf=.claude/rules/observability.md line=39 class=stale
leaf-text: and `config-json` threshold lines into `mcp.ndjson`
source: ctr-4hxbzvo4 «contract/config-json/known-keys · config.json · known keys — config.json has four known keys: v, diagnostics_level, claude_env_keep, dialog_hold_s; any other key, a retired five_hour or seven_day included, is counted»
replace-with: and `config-json` `parse-rejected` lines into `mcp.ndjson`
@leaf_edit n=4 leaf=.claude/rules/observability.md line=41 class=retired
leaf-text: `run.budget_eval` (INTERNAL; `trigger` `start|send|turn-ended`, `paused`, `window` only when paused) is declared once and opened once per evaluation; at start it sits between `channel.bind` and `state.snapshot_write`.
source: ctr-6f6hckro «contract/run-start-spans/span-chain · viola run start spans · span chain — run.start (INTERNAL, root) › state.snapshot_recover › run.collision_check › run.pin_copy › run.version_gate › channel.bind › state.snapshot_write»
replace-with: Run start's span chain is `run.start` (INTERNAL, root) › `state.snapshot_recover` › `run.collision_check` › `run.pin_copy` › `run.version_gate` › `channel.bind` › `state.snapshot_write` › `state.heartbeat_start` › `pty.spawn` (CLIENT), with no budget span.
@leaf_edit n=5 leaf=.claude/rules/observability.md line=41 class=retired
leaf-text: `read_budget` and `release --budget`'s lift carry no span and no line of their own; `budget-gate` is appended only when (paused, window, override end) changes.
source: ctr-5ipr6zzx «contract/release-refusal/invalid-params · release refusal lines · invalid-params — A from of another type, or any budget key (true, false, a string or null), is a plain -32602 (data null) after the release-from-driver check»
replace-with: A `release` whose `from` is not a string or that carries any budget key (true, false, a string or null) is a plain `-32602` (data null) after the release-from-driver check, with no `release-from-driver` line, no wheel moved and no record; its `channel-response` carries `detail` `invalid-params` on both sides at WARN.
@leaf_edit n=6 leaf=.claude/rules/observability.md line=41 class=retired
leaf-text: A `budget-paused` refusal's `send-refused` line carries `window`, `used_percentage`, `resets_at` and `paused:true`.
source: ctr-ahjw2v7q «contract/diag-line-fields/send-events · diagnostic line additive fields · send-issued / send-confirmed / send-refused — from, from_trust, text_bytes (normalised typed length), confirmed, refusal, detail, side (client or wrapper), wheel, duration_ms»
replace-with: A `send-refused` line carries `refusal`, `detail`, `side` and `wheel`, never `window`, `used_percentage`, `resets_at` or `paused`.
