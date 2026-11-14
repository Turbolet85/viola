@leaf_group group=architecture+security-plan
@leaf_read leaf=.claude/rules/api.md lines=41
last: _This section is owned by `/wrap-session`. setup-project pre
@leaf_edit n=1 leaf=.claude/rules/api.md line=20 class=stale
leaf-text: its `ok` payload is `{items, budget, skipped}`
source: ctr-t5xneaha «The `list` `ok` payload, CLI `--json` and MCP `list` alike, is `{items, skipped}`; `list` is not a channel method and needs no live wrapper.»
replace-with: its `ok` payload is `{items, skipped}`
@leaf_edit n=2 leaf=.claude/rules/api.md line=20 class=stale
leaf-text: As landed CLI `list` reads `<home>/instances/` and `budget.json`, then
source: dec-z2m7225c «`list` is not a channel method: the calling process reads the instances itself plus one `claude agents --json` read joined into the board, so `list` needs no live wrapper.»
replace-with: As landed CLI `list` reads `<home>/instances/`, then
@leaf_edit n=3 leaf=.claude/rules/api.md line=22 class=retired
leaf-text: `human-typing` · `budget-paused` · `unverified-cli`
source: ctr-tjgjkewn «Serialised kebab-case; its detail is five-hour or seven-day.»
replace-with: `human-typing` · `unverified-cli`
@leaf_edit n=4 leaf=.claude/rules/api.md line=22 class=stale
leaf-text: `send`: human-typing → budget-paused → turn-running → input-not-ready
source: ctr-xguevze4 «control-character, empty-text, human-typing, turn-running, the readiness gate (input-not-ready), human-typing and turn-running read again after the gate, then no-prompt-submitted.»
replace-with: `send`: human-typing → turn-running → input-not-ready
@leaf_edit n=5 leaf=.claude/rules/api.md line=24 class=stale
leaf-text: or a `budget` that is not a bool (`null` included), → `-32602` with `data: null`.
source: ctr-yjjehinc «A `release` whose `from` is of a non-string type, or that carries a `budget` key of any value (`null` included), is `-32602` `invalid-params` with `data: null`, no wheel moved and no record; only t»
replace-with: or one that carries a `budget` key of any value (`null` included), → `-32602` with `data: null`, no wheel moved and no record; only the string-`from` refusal comes before it.
@leaf_edit n=6 leaf=.claude/rules/api.md line=25 class=stale
leaf-text: `release` `{budget?, from?}` → `{wheel, budget_paused, override_until?}`: `budget:true` leaves the wheel and lifts the instance's budget gate (`override_until` = the crossed window's `resets_at` when a pause was lifted; nothing recorded when none stood), answered only after its `budget-gate` line and the snapshot land (`-32603` otherwise); a plain `release` answers the gate's recorded `budget_paused`.
source: ctr-tla4wlaw «A request: `release` `{from?}` answers `{wheel:<holder>}`.»
replace-with: `release` `{from?}` → `{wheel}`; a `release` on a driver-held wheel changes nothing.
@leaf_edit n=7 leaf=.claude/rules/api.md line=25 class=retired
leaf-text: `send` reads the gate once at arrival, after `human-typing` and before `turn-running`: a closed gate is `budget-paused` with detail `five-hour` | `seven-day` (exit 11), no cursor, nothing typed.
source: ctr-yoqejd2f «A closed budget gate refuses `send` `budget-paused` before `send-issued`, with nothing typed.»
replace-with: No budget gate exists: `send` is never refused for usage.
@leaf_edit n=8 leaf=.claude/rules/api.md line=29 class=stale
leaf-text: · 11 budget-paused ·
source: ctr-tf6njmtt «Unassigned: the retired `budget-paused` refusal's code, never reused or renumbered.»
replace-with: · 11 unassigned (the retired `budget-paused` code, never reused or renumbered) ·
