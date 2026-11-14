@leaf_group group=test-plan
@leaf_read leaf=.claude/docs/tests-summary.md lines=65
last: **Full plan:** `.andromeda/test-plan.md`. Path-scoped rules:
@leaf_read leaf=.claude/rules/testing.md lines=115
last: - 2026-11-14: A remove-the-guard control whose entry runs a 
@leaf_read leaf=.claude/rules/verification-harness.md lines=73
last: - 2026-11-14: A test-only recorder that replaces every off-l
@leaf_edit n=1 leaf=.claude/docs/tests-summary.md line=25 class=stale
leaf-text: event order `wheel{start}` → `budget-gate` → `session-start` (the third with "Hooks to normalised events")
source: ctr-hp7s2s2r «Every start, a revived one included, appends wheel{holder:"driver", cause:"start"} as its first record, and the hook's session-start is its second and last start record.»
replace-with: event order `wheel{start}` → `session-start` (the second and last start record, with "Hooks to normalised events")
@leaf_edit n=2 leaf=.claude/docs/tests-summary.md line=30 class=retired
leaf-text: **Path 6 — budget governor** — 90/85 thresholds, exit 11, per-instance `release --budget` override, statusline pass-through; as landed, the pass-through half only, as root tests on a planted `<home>/statusline-source.json` (`tests/hook_statusline.rs`, `tests/cli_instance_state.rs`; `budget.json` only with a `rate_limits` object)
source: dec-dj4tb537 «Budget governor thresholds — Automated send is refused with budget-paused once five_hour reaches 90 % or seven_day reaches 85 % (89 and 84 stay open), and release --budget overrides the pause until its override»
replace-with: **Path 6 — statusline pass-through** — the statusline output passes through byte for byte, as root tests on a planted `<home>/statusline-source.json` (`tests/hook_statusline.rs`, `tests/cli_instance_state.rs`); the budget governor (its thresholds, `release --budget`, `budget.json`) is removed whole
@leaf_edit n=3 leaf=.claude/docs/tests-summary.md line=30 class=retired
leaf-text: ; and since chunk 2026-10-10-budget-governor the cli and ipc signals of the gate as `path6_` cases of `tests/cli_budget.rs` (thresholds and boundaries, exit 11 with the window word, an unknown or passed reset does not pause, `release --budget`, the hint never naming `release`, a turn end closing the gate, a restart pausing again, and since chunk 2026-10-10-session-links `answer` / `wait` / `last` / `pause` left alone by a closed gate), with E5's budget leg beside them; override expiry is a unit case of the pure `decide`, fed its instant as an argument. `list --json`'s `budget` and the board's cell landed at chunk 2026-10-11-the-board-viola-list (`path6_list_shows_the_reading_and_the_pause`, green in CI). `boot --statusline-echo`, the harness-session form and step 7's two-instance leg landed at chunk 2026-10-17-end-to-end-path-binary-and-harness-legs as `path6_two_instance_release_over_a_harness_session` (`crates/viola-e2e/tests/path6_budget_harness.rs`). The MCP form of the refusal landed at chunk 2026-10-17-mcp-server-for-drivers as `mcp_tools_path6_budget_paused_send` (`crates/viola-e2e/tests/mcp_tools.rs`: `isError:true`, `{"refusal":"budget-paused","detail":"five-hour"}`, equal to the CLI's document less `v`).
source: ctr-jcrenivt «viola send · 11 — Retired (was budget-paused) and never renumbered or reused: no refusal maps to 11, and an older wrapper's budget-paused word reads unknown, exit 14.»
replace-with: ; exit 11 is retired (was `budget-paused`) and never renumbered or reused: no refusal maps to 11, and an older wrapper's `budget-paused` word reads `unknown`, exit 14; the `path6_` budget cases, `list --json`'s `budget` and `budget_paused`, the two-instance `release --budget` leg and the MCP `budget-paused` refusal are removed with the gate.
@leaf_edit n=4 leaf=.claude/docs/tests-summary.md line=52 class=stale
leaf-text: NEVER hand-write stamps, snapshots or `budget.json` in a booted-wrapper test
source: dec-izcdbt6a «State files are never hand-written — Every booted-wrapper test takes `ledger/stamps.json` from `viola verify` and `snapshot.json` from the wrapper, never writing them by hand.»
replace-with: NEVER hand-write stamps or snapshots in a booted-wrapper test
@leaf_edit n=5 leaf=.claude/rules/testing.md line=19 class=retired
leaf-text: (rmcp client; a numbered Path binary, e.g. `path6_budget_harness.rs`)
source: dec-hyr4w4za «Budget override is per instance — release --budget lifts the pause for the named instance alone; other instances under the same reading stay paused and are refused. · guard: path6_two_instance_release_over_a_harness_session»
replace-with: (rmcp client; a numbered Path binary is named `path<N>_<topic>.rs`)
@leaf_edit n=6 leaf=.claude/rules/testing.md line=24 class=omission
leaf-text: fake scripts `fixtures/fake-scripts/<scenario>.json` (schema `schemas/fake-script.v1.json`).
source: dec-n5cm3kti «Retired words kept as committed fixtures — The 0.1.0 own-format shapes and the retired words are committed fixtures in a viola-0.1.0 fixture directory outside the identifier-probed paths, read by the cases»
replace-with: fake scripts `fixtures/fake-scripts/<scenario>.json` (schema `schemas/fake-script.v1.json`); the 0.1.0 own-format shapes and the retired words are committed fixtures in a `viola-0.1.0` fixture directory outside the identifier-probed paths, read by the cases that must spell an old word and walked by the fixture hygiene check, since a removal's identifier probe and such a case cannot share a tree.
@leaf_edit n=7 leaf=.claude/rules/testing.md line=28 class=stale
leaf-text: or an instant handed to a pure decision as an argument (the budget gate's `decide`)
source: dec-ke3rmgnd «pure time decisions take their instant — A pure time decision takes its instant as an argument instead of reading a clock.»
replace-with: or an instant handed to a pure time decision as an argument
@leaf_edit n=8 leaf=.claude/rules/testing.md line=34 class=stale
leaf-text: unserved row) or `budget.json` (only `viola hook statusline`).
source: dec-izcdbt6a «State files are never hand-written — Every booted-wrapper test takes `ledger/stamps.json` from `viola verify` and `snapshot.json` from the wrapper, never writing them by hand.»
replace-with: unserved row).
@leaf_edit n=9 leaf=.claude/rules/verification-harness.md line=28 class=stale
leaf-text: (no env var, no `budget.json`)
source: ctr-etd5ao5s «harness boot · --statusline-echo — --statusline-echo conflicts with --unstamped and plants an owner-only statusline settings source in the session home that viola only reads; it sets no env var.»
replace-with: (no env var)
@leaf_edit n=10 leaf=.claude/rules/verification-harness.md line=29 class=stale
leaf-text: then `events.ndjson` lines 1–3 = `wheel{cause:start}` → `budget-gate` → `session-start{source:hook}`
source: ctr-ephtbyyb «boot readiness · events — Event lines 1 and 2 are a wheel line with cause start, then session-start with source hook; missing reads events.»
replace-with: then `events.ndjson` lines 1 and 2 = `wheel{cause:start}` → `session-start{source:hook}`
