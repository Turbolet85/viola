@history_item n=1 marker=2026-10-10-budget-governor
**2026-10-10-budget-governor** (run.budget_eval in the start chain, the gate's log surface) — put `run.budget_eval` (trigger `start|send|turn-ended`, `paused`, `window`) between `channel.bind` and `state.snapshot_write` in the start span chain; declared `read_budget` and `BudgetGate::lift` spanless; `budget-gate` appended only on a (paused, window, override end) change; `release` with `budget: true` appends one `budget-gate` when a pause stood; `run` reads `config.json` a second time for the thresholds, one `parse-rejected{config-json, malformed}` per bad threshold key, `CONFIG_KEYS` five names. This is the obs surface this chunk deletes: the start chain row (contract/run-start-spans/span-chain), contract/budget-span-attributes/*, contract/events-log/budget-gate, contract/release-budget/budget-gate and the config-json five_hour / seven_day rows all trace to it, and `CONFIG_KEYS` (`src/obs.rs`) must shrink with the keys.

@history_item n=2 marker=2026-10-10-budget-governor
**2026-10-10-budget-governor** (a refused instance check writes no line, on both arms) — made the statusline arm's refused check lineless and named `read_budget`'s refused `budget.json` file among the strict-modes failures that leave no line (Vector 4 / Vector 7 triggers, §10 exemption 7). The removal retires the `budget.json` read, so every no-line-refusal list naming it (§1, §6 Filesystem refusals, §10) loses that item.

@history_item n=3 marker=2026-10-10-statusline-pass-through
**2026-10-10-statusline-pass-through** — set the Budget governor scenario's log fields: `parse-rejected{hook-stdin}`, the `statusline-shell` spawn pair, `hook-decision` fail-open details, `budget_written` true only when the payload held `rate_limits` and the write succeeded; `statusline.shell_out` closes at `STATUSLINE_DEADLINE`. This chunk removes `budget_written` and the `budget.json` write (contract/statusline-log-fields/hook-decision, contract/budget-spans/span-chain's `state.budget_write`), while the pass-through, shell-out and their lines stay for the next entry («Status-line interception removed»).

@history_item n=4 marker=2026-09-27-instance-state-and-start-order
**2026-09-27-instance-state-and-start-order** — fixed the product start order with `run` writing `wheel` + `budget-gate` as its first records. Removing the budget step changes contract/run-start-events/order and decision/product-order-from-events-log (wheel{cause:"start"} then budget-gate then session-start).

@history_item n=5 marker=2026-09-27-hooks-to-normalised-events
**2026-09-27-hooks-to-normalised-events** — made `session-start{source:"hook"}` record three after `wheel` and `budget-gate`. With `budget-gate` gone, session-start becomes record two; every master site and test counting it third moves.

@history_item n=6 marker=2026-09-29-verify-stamped-test-homes-and-harness
**2026-09-29-verify-stamped-test-homes-and-harness** — the harness `boot` readiness stage `start_records` checks `events.ndjson` lines 1-3 (missing code `<name>:events`). The e2e harness (`crates/viola-e2e/src/harness/boot.rs`) must read two start records once `budget-gate` leaves, or boot readiness breaks.

@history_item n=7 marker=2026-10-04-the-wheel
**2026-10-04-the-wheel** — defined `release-from-driver` as a `release` carrying a string `from` (one INFO line, `from` only when valid, beside `-32602`), and «another `from` type or a non-bool `budget` is a plain `-32602` with no line». The driver guard stays; the non-bool-`budget` clause (contract/release-refusal/invalid-params) loses its subject when the `budget` param leaves.

@history_item n=8 marker=2026-09-27-wrapper-channel
**2026-09-27-wrapper-channel** — made `corr` required in `diag-line.v1.json` on `release-from-driver` and `send-*` wrapper lines, among others. The kept release guard's row contract/corr-by-event/release-from-driver rests on it; schema edits removing budget fields must keep these required-corr rules intact.

@history_item n=9 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** (channel-response's detail closed to three words) — closed `channel-response.detail` to `unsupported-version|invalid-params|release-from-driver`; release's string-`from` reply carries `release-from-driver`, a plain `-32602` `invalid-params`, both sides at WARN. The kept guard's lines (contract/channel-response/detail, contract/release-refusal/*) stand; the invalid-params cause list drops the non-bool `budget`.

@history_item n=10 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** (a peer's reply screened by sanitised_fault and is_code_word) — `sanitised_fault` reads `data.reason` `release-from-driver`; `is_code_word` keeps an unknown refusal word visible with exit 14. Once `RefusalReason::BudgetPaused` is gone, a `budget-paused` word from an older wrapper reads as an unknown code word, exit 14 (contract/refusal-reason/unknown-fallback, decision/peer-replies-screened-as-code-words) — exit 11 is retired, not reused.

@history_item n=11 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** (v_rejected counts version rejections alone) — split the `-32602` causes by `detail`, `release` carrying `from` reading `release-from-driver`. Context for the kept guard's metric reading; no budget cause remains among them.

@history_item n=12 marker=2026-10-04-confirmed-send-with-cl-1-records
**2026-10-04-confirmed-send-with-cl-1-records** — set `send.client`'s exits (0/10/11/12/13/14/20/21, 1 internal-error), 11 being `budget-paused`. This chunk retires exit 11 from that set and from contract/send-refused/budget-paused, never renumbering the others.

@history_item n=13 marker=2026-10-11-the-board-viola-list
**2026-10-11-the-board-viola-list** — named `read_budget_stamped` (taken by the gate and by `list`) as a no-line strict-modes refusal, and had `list` read `config.json` thresholds through `read_thresholds` under `cli.list`, one per-key `config-json` line under a name. Both leave with the budget; `list`'s board loses its budget reading.

@history_item n=14 marker=2026-10-17-mcp-server-for-drivers
**2026-10-17-mcp-server-for-drivers** (the mcp list tool's lines) — the MCP `list` tool writes the board read's per-key `config-json` lines into `mcp.ndjson`. With the threshold keys gone, the threshold-rejection lines from `list` (CLI and MCP) disappear; the `list` payload's `budget` field leaves contract/viola-mcp-tools/list's tool.

@history_item n=15 marker=2026-10-13-t06-38-35-wrap
**2026-10-13-t06-38-35-wrap** — recorded the checked `budget.json` read's no-line refusal as the operator's word (not the founder's) among the no-line refusals. One of the provenance marks on the budget read this chunk deletes.

@history_item n=16 marker=2026-10-20-t06-58-31-wrap
**2026-10-20-t06-58-31-wrap** — the founder accepted the four no-line strict-modes refusals, the checked read of `budget.json` among them, as built. The removal takes that item out of the list; the other three stand.

@history_item n=17 marker=2026-10-26-explicit-session-links
**2026-10-26-explicit-session-links** (the link verbs' instance check before their log) — listed five no-line strict-modes refusals, the `budget.json` read among them (§1 Vector 7). A register this chunk shrinks.

@history_item n=18 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (the refusal-telemetry record) — the Vector 7 trigger and §6 Filesystem refusals list no-line refusals ending with «the `budget.json` read and `list`'s scan». That item leaves with `read_budget`.

@history_item n=19 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (the snapshot readers over a replayed snapshot) — a replayed snapshot carries no endpoint, pid, started_at or statusline_command and is unreachable (decision/replayed-snapshot-unreachable, contract/run-span-attributes/run-collision-check). Replay also recovers `budget_paused` / `budget_override_until`, which leave; old disk state carrying them must still read (skip and count).

@history_item n=20 marker=2026-10-10-self-healing-state
**2026-10-10-self-healing-state** — set where `state-recovered` is written (snapshot-replayed / snapshot-unsupported-v, one line per call, none when replay fails). Replay's recovery of the budget fields is removed here; the state-recovered rows picked (contract/state-recovered/*) must stay true for logs and snapshots still holding `budget-gate` / budget fields.

@history_item n=21 marker=2026-10-17-cli-machine-contract
**2026-10-17-cli-machine-contract** — `state-recovered` gained `scanned` on the snapshot details (contract/state-recovered/scanned). The board read that writes it loses its budget reading but keeps this line.

@history_item n=22 marker=2026-10-19-dialog-hold-for-the-driver
**2026-10-19-dialog-hold-for-the-driver** — `run`'s start reads `config.json` a third time for `dialog_hold_s`; `CONFIG_KEYS` six names. After removal `config.json` keeps `v`, `diagnostics_level`, `claude_env_keep`, `dialog_hold_s` (contract/config-json/known-keys goes six to four); a file still carrying `five_hour` / `seven_day` then counts them as unknown skipped keys (contract/parse-rejected-detail/unknown-keys) unless P3 measures otherwise.

@history_item n=23 marker=2026-10-13-owed-ledger-rows
**2026-10-13-owed-ledger-rows** — the statusline capture arm is uninstrumented and touches no `budget.json`; `verify`'s step lines are one per relied row, `MM` the relied count. The `statusline-rate-limits` row's removal moves `MM` (contract/verify-stdout/step-line, summary-line).

@history_item n=24 marker=2026-10-18-compact-post-condition-ledger-row
**2026-10-18-compact-post-condition-ledger-row** — set `verify`'s `MM` to 24 (Unix). This chunk takes it 24 → 23 if the `statusline-rate-limits` row leaves here (a P4 fork if it moves to the next entry).

@history_item n=25 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (the Windows host arms … retired) — the relied count reads all 24 rows of `ALL`, and the statusline-shell pair needs only a recorded `statusline_command`. The current relied-count statement this chunk changes to 23.

@history_item n=26 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (the closed words that left `diag-line.v1.json` with their emitter) — set decision/unemittable-schema-words-removed: a closed schema word no build can emit leaves the schema with its emitter. The precedent and rule for dropping `budget-paused`, `budget_written`, `window`, `used_percentage`, `resets_at`, `paused` and the budget event fields from `schemas/diag-line.v1.json` (and the detail schema) with the code; trap: a row edit the store refuses stays owed by a CARRY.

@history_item n=27 marker=2026-10-16-remove-the-browser-leg
**2026-10-16-remove-the-browser-leg** (the reserved web vocabulary leaves) — precedent removal chunk: vocabulary with no emitter left the enums, schema and the bound §3 Log-format copy together, with the copy moving in step with test-plan §3's. The same applies to the budget fields in the log format copy and decision/diag-schema-additive-only / log-fields-additive-only (removal goes by a recorded decision, not a silent rename).

@history_item n=28 marker=2026-10-03-mutation-scoring-completion
**2026-10-03-mutation-scoring-completion** (§1 kept current) — §1 is a verbatim copy kept current; a hit inside it is judged like any body amendment. §1's budget passages (budget-metadata-loggable, statusline-command-confined on the budget governor path, the budget path events) retire with the census at this chunk's wrap rather than standing stale.

@history_item n=29 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** (`socket-dir-refused` in the log catalogue) — `hook-decision` fail-open codes at eight; `viola hook statusline` writes three of them (`oversize-stdin`, `malformed-json`, `deadline`), and the «before chunk 2026-10-10-budget-governor» clause left. The statusline arm's codes (contract/statusline-decision-detail/*) stay after the budget write leaves.

@history_item n=30 marker=2026-11-08T20-32-52-wrap
**2026-11-08T20-32-52-wrap** — census C-301 (the instrumentability rule for the status line's user-written shell command) was held back for «Status-line interception removed». Sets the boundary: decision/statusline-command-duration-exit-only and the shell-out rows stay in this chunk.

@history_item n=31 marker=2026-11-14-hyprland-seam
**2026-11-14-hyprland-seam** (6. Log Coverage: 5 rows) — `~ contract/diag-line-fields/parse-rejected` (parser list gains `hyprctl-json`), `~ contract/diag-line-fields/process-start`. The latest edit to the picked parse-rejected row; the budget removal keeps the `config-json` parser and must not drop the new hyprctl words.

@history_item n=32 marker=2026-11-14-hyprland-seam
**2026-11-14-hyprland-seam** (1. Obs Scope Summary: 3 rows; 2 rows) — `~ decision/parse-failures-surface-as-unknown` and `~ decision/parse-failures-unknown-counter` gained the hyprctl exception. Both are picked rows; their current text includes that except clause, untouched by the budget removal.
