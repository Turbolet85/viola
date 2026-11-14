@history_item n=1 marker=2026-10-10-budget-governor
**2026-10-10-budget-governor** (the budget gate as landed) — built `src/run/budget.rs` (one `BudgetGate` per wrapper, no thread or timer), the crossing rule, the `send` rung `control-character → empty-text → human-typing → budget-paused → turn-running → gate`, `answer`/`wait`/`last`/`pause`/`hook.dialog` not reading it, the override covering both windows until its end, `budget-gate` + snapshot `budget_paused`/`budget_override_until` written only on change, the start-order step between the endpoint bind and the first snapshot (record two carrying its verdict), and `release`'s `{wheel, budget_paused, override_until?}` with the `budget:true` durability and `-32603`. This is the whole surface the chunk removes; the start order, record two, `release`'s reply shape and the refusal order each lose a step or field here and their rows (run-start-order, contract/send-refusal/order, contract/refusal-order/send, contract/channel-methods/release*) must be rewritten in step.

@history_item n=2 marker=2026-10-10-budget-governor
**2026-10-10-budget-governor** (the threshold keys, the reader of budget.json, the owed ledger row) — registered `config.json`'s `five_hour`/`seven_day` (0–100, defaults 90/85, read once at start, one `parse-rejected{config-json, malformed}` per bad key, no value switches the gate off), `viola_state::budget::read_budget` (check_path, shared lock, `take(MAX_FRAME + 1)`, tolerant parse, `v`=1, every failure no reading with no line), the boundary widening of a driver refused on a hook-written file, and the root-wait count 28/20 that `tests/cli_budget.rs` contributed to. Names exactly the config keys, reader and counts that leave; the WITHIN-wait count moves when the budget tests go.

@history_item n=3 marker=2026-10-10-budget-governor
**2026-10-10-budget-governor** (the instance check before any log, on both arms) — moved `hook statusline`'s `check_instance` ahead of anything under the home, a refused instance exiting 0 with no `budget.json` written and no line. The statusline arm keeps this order after the `budget.json` write leaves; decision/refused-statusline-check-no-trace and contract/hook-statusline/refused-check name `budget.json` untouched and need re-wording, not removal of the check.

@history_item n=4 marker=2026-10-10-statusline-pass-through
**2026-10-10-statusline-pass-through** (the pass-through as landed) — fixed the arm's order (canonicalise, check, log, bounded stdin, `budget.json` write only with a `rate_limits` object, snapshot read, command) and the `budget.json` shape `{"v":1,"five_hour","seven_day","read_at"}` replaced at 0600 under its lock; [Budget Governor] tied to the statusline wrap. The chunk cuts the `budget.json` step out of this order while the pass-through itself stays for «Status-line interception removed».

@history_item n=5 marker=2026-10-10-statusline-pass-through
**2026-10-10-statusline-pass-through** (counts, crate edges, budget types) — `used_percentage` is a `Reading<f64>` in `viola_core::BudgetWindow`, `Percent` stays viola's own newtype; the tree names `viola-state`'s `budget.rs`, `viola-core`'s budget reading types and the agent crate's `statusline` module (chrono for it). Precedent for the scope's `[inferred]` question whether `Reading<T>` survives with no budget user, and for `Percent` leaving nutype-domain-newtypes.

@history_item n=6 marker=2026-10-10-statusline-pass-through
**2026-10-10-statusline-pass-through** (the capability ledger: relied dependences with no row) — named the payload's `rate_limits` object as a relied dependence with no row, its row owed to the budget entry. Origin of the `statusline-rate-limits` row the chunk retires; the next entry keeps the override and `/bin/sh` shapes.

@history_item n=7 marker=2026-10-10-session-links
**2026-10-10-session-links** (the link set in the replay, the snapshot and the start) — pinned `path6_a_closed_gate_leaves_answer_wait_last_and_pause_alone` (`tests/cli_budget.rs`) as the guard of decision/only-send-reads-budget-gate, and the replay's recovery set beside `links`. That guard row and its test leave together; the replay's recovered-field list (snapshot-replay-log-fields-only, contract/snapshot-envelope/unreadable-snapshot) drops `budget_paused`/`budget_override_until`.

@history_item n=8 marker=2026-10-10-self-healing-state
**2026-10-10-self-healing-state** — the reader `events::read_from` counts `unknown_kinds`/`unknown_fields`/`torn_lines` and never errs; replay writes no file and a field no line gave is absent. This is what lets old `budget-gate` lines and snapshot budget fields keep reading after the removal (scope's «Readers of old disk state»): a `budget-gate` line becomes an unknown kind, counted in `skipped`, which P3 must measure on replay and the board.

@history_item n=9 marker=2026-10-11-the-board-viola-list
**2026-10-11-the-board-viola-list** — `list` reads `budget.json` through `read_budget_stamped` (one checked read, two product paths: the gate's `read_budget` and `list`) and `config.json`'s thresholds through `read_thresholds` as a second reader; items carry `budget_paused`, payload `budget`. Every one of these readers and fields leaves; `list`'s strict-check-before-log order and `skipped` stay.

@history_item n=10 marker=2026-10-13-t06-38-35-wrap
**2026-10-13-t06-38-35-wrap** — the founder ratified the driver refusal on `budget.json` (read behind strict-modes, failing open) and ruled the `budget-paused`-on-`rate_limits` reliance. Both ratifications lose their subject with this chunk; the release-from-driver guard and the board's whole-refusal shape ratified in the same batch stay.

@history_item n=11 marker=2026-10-04-the-wheel
**2026-10-04-the-wheel** (pause and release served; release-from-driver landed) — `release` `{budget?:bool, from?}` → `{wheel, budget_paused}`, a string `from` refused `-32602` `release-from-driver` plus one obs line, another `from` type or a non-bool `budget` (`null` included) `-32602` `data: null`, `budget:true` leaving the wheel. The scope keeps the driver guard and the type check but removes the `budget` param: decide what a `budget` key now reads (unknown field skipped vs `-32602`) without loosening the guard's first-check order (contract/channel-methods/release-type-check, release-from-driver).

@history_item n=12 marker=2026-10-04-running-turn-refusal
**2026-10-04-running-turn-refusal** — only a `release` returning a human-held wheel clears the running turn; a driver-held `release`, `release --budget`, a refused `release` and `pause` leave it. With `--budget` gone the clearing rule must still read whole for plain `release`.

@history_item n=13 marker=2026-10-04-confirmed-send-with-cl-1-records
**2026-10-04-confirmed-send-with-cl-1-records** — set the `send` order with `budget-paused` between `human-typing` and `turn-running` and `send-refused {refusal, detail, cursor?}` absent cursor before `send-issued`. The rung leaves; the remaining order and the first-refusal-only rule stay.

@history_item n=14 marker=2026-10-09-epoch-3-cleanup
**2026-10-09-epoch-3-cleanup** (an empty typed text is refused `empty-text`) — placed `empty-text` after `control-character`, before `human-typing` «and the rest as before», no exit code or `v` moving. Precedent for editing a refusal rung without renumbering exits; contract/send-refusals/empty-text and the order rows stay minus `budget-paused`.

@history_item n=15 marker=2026-10-13-epoch-4-cleanup
**2026-10-13-epoch-4-cleanup** (send and human split by phase) — created `src/run/send/budget.rs` (`Ctx::budget_paused`), `human/verbs.rs`' budget lift/open lines and `human/hints.rs`. Names the split modules the removal deletes or trims; the module roots' re-exports must drop them.

@history_item n=16 marker=2026-10-20-epoch-5-cleanup-ii
**2026-10-20-epoch-5-cleanup-ii** (the split modules) — `src/human/board.rs` re-exports `BayBudget` over `board/{pairs,bay,cells,rows}.rs`. `BayBudget` and the budget cells leave.

@history_item n=17 marker=2026-10-20-epoch-5-cleanup-ii
**2026-10-20-epoch-5-cleanup-ii** (the recorded live MCP exchange) — `fixtures/claude/2.1.302/Mcp.tools-list-1.json` is recorded only by `viola-mcp-tap` against a live `claude`, scrubbed, schema-checked and replayed by `tests/mcp_recorded_exchange.rs` (decision/mcp-tools-list-held-to-recorded-exchange: key and type shape, tool names in order). Changing the MCP `list` description touches a live-recorded fixture; the replay holds shape and names only, so the scope's `[inferred]` re-record question rests on this.

@history_item n=18 marker=2026-10-13-owed-ledger-rows
**2026-10-13-owed-ledger-rows** (three statusline ledger rows and the per-OS relied set) — added `statusline-rate-limits` (`whole_rate_limits`), `--record`'s `StatusLine.rate-limits-1.json`, «`budget-paused` now rests on a stamped row», and the rule that a version is verified only when every relied row reads `pass`. Retiring the row moves the relied count and `[NN/MM]` and changes which stamps verify; prior ledger chunks each record how older stamps read after a count change.

@history_item n=19 marker=2026-10-13-owed-ledger-rows
**2026-10-13-owed-ledger-rows** (Run B's probe --settings, the statusline capture arm) — the hidden `hook statusline --capture` arm writes no `budget.json`; the fake agent's `--framing` replays the StatusLine variants after the first Stop. The capture arm and the replay of `StatusLine.default-1.json` stay for the next entry; only the rate-limits variant leaves.

@history_item n=20 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (the relied set: all twenty-four rows on every build) — `LedgerRow::relied()` is every row of `ALL`, `MM` = 24, an all-`pass` stamp of an older id set reads unverified; 2.1.287 stamped `24 pass  0 fail`. The scope's 24 → 23 lands on these rows (contract/viola-verify-output/step-line, summary-line, decision/version-verified-all-relied-rows).

@history_item n=21 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (the statusline wrap and its probe on every build) — [Budget Governor]'s wrap no longer Unix only; `statusline-override`, `statusline-shell`, `statusline-rate-limits` relied on every build. Fixes the boundary: two of the three statusline rows stay for the next entry, only rate-limits leaves here.

@history_item n=22 marker=2026-11-06-version-done-check
**2026-11-06-version-done-check** (the statusline-rate-limits row reads the account's plan state) — measured `stamped 2.1.287  23 pass  1 fail` on an account whose payload carries `five_hour` `unknown`, and opened F-06i: whether the budget gate stays and what it stands on, owner 0.2.0's intent. The direct cause of this removal (the founder's «Убрать совсем» answering F-06i); the OPEN and the cross-cutting «`budget-paused` rests on a stamped row» fact close with this chunk.

@history_item n=23 marker=2026-11-08T20-32-52-wrap
**2026-11-08T20-32-52-wrap** — the 0.2.0 census dropped the never-built page's budget-reading clause and left «the budget's removal with "Budget gate removed"», rows whose subject is built waiting for their removing chunks. Confirms this chunk owns every built budget row (census F-08 `→ v2-04`).

@history_item n=24 marker=2026-10-16T14-58-10-wrap
**2026-10-16T14-58-10-wrap** — moved the item, `budget` and `skipped` shapes out of the GUI envelope into the `list` payload contract (CLI `list --json`, MCP `list`). contract/list-payload/ok, budget, items and list-item/budget-paused are those shapes; `budget` and `budget_paused` leave, `items`/`skipped` stay.

@history_item n=25 marker=2026-10-16-unwrapped-sessions-on-the-board
**2026-10-16-unwrapped-sessions-on-the-board** (Standard Contracts) — an unwrapped item carries no `wheel`/`budget_paused`/`cli_version`/`cli_verified`. contract/list-item/wrapped names `budget_paused` in its absent list and needs re-wording.

@history_item n=26 marker=2026-10-17-mcp-server-for-drivers
**2026-10-17-mcp-server-for-drivers** (MCP list builds the board and reads agents) — MCP `list` returns `board_document`'s `ok`, the same `{items, budget, skipped}` as CLI `list`. MCP `list`'s payload and its tool description (`crates/viola-mcp/src/tools.rs`) move with the CLI board.

@history_item n=27 marker=2026-10-17-cli-machine-contract
**2026-10-17-cli-machine-contract** — registered `schemas/config.v1.json` with the five `CONFIG_KEYS` (`v`, `diagnostics_level`, `claude_env_keep`, `five_hour`, `seven_day`), tolerant, admitting additional properties. decision/config-keys-match-schema (guard `config_keys_are_the_config_schema_properties`) requires the key set and schema to drop the two thresholds together; contract/config-json/unknown-keys answers the scope's `[inferred]` question on a leftover key (skipped and counted).

@history_item n=28 marker=2026-10-19-dialog-hold-for-the-driver
**2026-10-19-dialog-hold-for-the-driver** — added `config.json`'s `dialog_hold_s` and made `DialogHold` a nutype beside `ViolaName` and `Percent`. `dialog_hold_s` and `DialogHold` stay while `Percent` leaves; decision/nutype-domain-newtypes and decision/nutype-newtypes list all three.

@history_item n=29 marker=2026-09-24-diagnostics-plane
**2026-09-24-diagnostics-plane** — registered `config.json`'s `MAX_FRAME`-capped read requiring `v`=1 and its `parse-rejected` report, and the process-log `v` exception. The config reader that stays after the thresholds leave; the `parse-rejected{config-json}` lines for the two keys go.

@history_item n=30 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** (`viola list`'s one home rule) — `board::scan` holds the home to the read rule; `read_budget_stamped`, `mcp`'s start and `plugin install` keep the write rule. decision/list-home-read-rule's except names «the budget read», which leaves with `read_budget_stamped`.

@history_item n=31 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** (`viola mcp`'s exit on a stdin cut before `initialize`) — moved `ServeError::Initialize` from exit 11 to 1, noting `budget-paused`'s exit 11 «is another code and did not move». Exit 11 is retired here and never reused; this entry is the last record of 11's one other near-use.

@history_item n=32 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** (`unknown`'s detail; `sanitised_fault`; `channel-response` detail) — `unknown` is the serde catch-all whose detail is a code word or null; `sanitised_fault` reads `-32602` `data.reason` exactly `release-from-driver`; `channel-response`'s `detail` is the closed enum including `release-from-driver`. An old wrapper's `budget-paused` reaches a new client as `unknown` (contract/refusal-reason/unknown-catch-all); the release-from-driver bodies stay unchanged.

@history_item n=33 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** (the sanitised error surfaces pattern and its contract binary) — `tests/contract_error_bodies.rs` tables (`CLI_DOCUMENTS`, `MCP_RESULTS`, `CHANNEL_FAULTS`) and the writers census; `crates/viola-channel/src/client.rs` tests use refusals as samples. Budget-paused rows/writers leave or switch to another closed word, or the census goes red.

@history_item n=34 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** (the code-word rule and the exit-cause table) — `tests/cross_exit_causes.rs` holds each code to its hint, every code a code word and a row of `contract_error_bodies`. The table must lose exit 11's code without renumbering the others.

@history_item n=35 marker=2026-10-27-bounded-inputs-at-every-boundary
**2026-10-27-bounded-inputs-at-every-boundary** (bounded readers) — `tests/contract_bounded_readers.rs`' `BOUNDARIES` table names each bounded read and its witness, checked by `boundaries_each_name_a_bounded_read_that_stands`. The `budget.json` read (and any statusline `rate_limits` boundary) must leave the table with its reader or the contract fails.

@history_item n=36 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (the root `WITHIN` waits: 31 sites in 23 files) — the count by `grep -rn 'Instant::now() + WITHIN' tests/` lives in occupied-resources and test-plan. Deleting `tests/cli_budget.rs` and budget cases moves it; restate it by its rule.

@history_item n=37 marker=2026-11-14-hyprland-seam
**2026-11-14-hyprland-seam** (Infrastructure Patterns: root-bin-consumes-normalised-events) — re-worded the row to «subcommand dispatch, the run pump, the wheel and the budget governor». The latest edit to a picked row that still names the budget governor; precedent of dropping a false clause rather than lengthening.

@history_item n=38 marker=2026-11-12T03-23-49-rows
**2026-11-12T03-23-49-rows** — the masters became rows (decision · contract · absence · name), each verified against the code, the fates the founder's. The budget rows this chunk retires are amended as `− {handle}` / `~ {handle}` row edits at wrap; the phase itself amends no master (scope boundary).
