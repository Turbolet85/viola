@history_item n=1 marker=2026-11-22-lineage-registry
**2026-11-22-lineage-registry** (1. Test Scope Summary / 3. Test Harness Contract) — the direct precedent for a per-instance record: `~ contract/viola-list-json/items` gained `parent?, role?`; `~ decision/snapshot-replay-fields` recovers lineage too, the last lineage line winning; `~ contract/run-start-events/order` and `~ contract/boot-readiness/events` moved to three start records (wheel, lineage, session-start); `+ decision/supervisor-spawn-strips-bridge-env`. A model/effort record beside lineage moves the same four rows again (replay field set, start-record order, boot readiness line count, list item keys), and the start-order change was PROVISIONAL (F3).

@history_item n=2 marker=2026-11-14-budget-gate-removed
**2026-11-14-budget-gate-removed** (1. Test Scope Summary, 3. Test Harness Contract, 6. E2E Test Strategy) — removed `budget_paused` / `budget_override_until` from `decision/snapshot-replay-fields`, `decision/snapshot-replay-from-log`, `contract/viola-list-json/items` and dropped the budget-gate start record from `contract/run-start-events/order`, `contract/boot-readiness/events` and `contract/event-log/start-wheel-record`; the guard of `decision/snapshot-replay-from-log` was later re-pointed to `a_start_over_an_unreadable_snapshot_replays_the_log` and the `e5_the_replay_reads_` case of `cli_links`. History of every row this chunk extends when it adds a recorded field and start record.

@history_item n=3 marker=2026-11-14-status-line-interception-removed
**2026-11-14-status-line-interception-removed** (6. E2E Test Strategy, 7. Test Data & Fixtures, 1. Test Scope Summary) — `+ contract/viola-run-start/settings`: a start hands its child no `--settings` and writes no `settings.json`, the user's settings byte- and mtime-equal; `~ decision/plugin-rewritten-on-start` lost the settings rewrite; `+ contract/fake-agent-cli/settings` (accepted, otherwise ignored). A trap if the model/effort channel lands as a settings key: it would re-open a retired surface and break that contract (and v2-31's byte-identical promise).

@history_item n=4 marker=2026-11-21-cross-session-prefix-ledger-row
**2026-11-21-cross-session-prefix-ledger-row** (7. Test Data & Fixtures, 3. Test Harness Contract) — `~ contract/fake-agent-cli/settings`: the fake agent reads its cross-session inbox only under the accept settings; `~ contract/fake-agent-receipt/start` carries `cli_version`, pid and the print prompt; `+ decision/stamping-verify-test-owned-runtime-dir`. The fake agent's `--settings` and start receipt are where a launch model/effort would be observed or collide; a new ledger row's stamping verify needs the same test-owned dirs.

@history_item n=5 marker=2026-11-22T17-29-41Z-wrap
**2026-11-22T17-29-41Z-wrap** (7. Test Data & Fixtures; 1. Test Scope Summary) — `~ contract/fake-agent-cli/argv-only-options` and `~ decision/fake-agent-argv-only`: options come from argv and a fake-args file beside the exe, argv winning, never the environment; `+ decision/run-start-verify-and-trust-after-bind` fixes where verify and trust sit in the start order. A new fake-agent option (e.g. receipting `--model` / effort) follows this argv-only rule, and the pair's handing-over must sit inside the bound start order.

@history_item n=6 marker=2026-10-10-viola-revive
**2026-10-10-viola-revive** (resume payload and options) — under `--resume <id>` the fake agent sets `source` / `session_id` on SessionStart; ten argv options, each counted by its rule; revive is the replay's first product reader and a replayed snapshot carries no `cwd`. Precedent for revive's resume tests this chunk extends to replay the recorded pair.

@history_item n=7 marker=2026-10-14-resume-and-compact-ledger-rows
**2026-10-14-resume-and-compact-ledger-rows** — Run E and the `resume-session` row: relied count, `STEPS_PASS` / `ROW_IDS` / `LEDGER_ROWS` literals, `cli_verify` summaries, the resume variant fixture and the `verify_window_` timing all moved together. Template for what a new ledger row (an undocumented `claude` model/effort channel) cascades through, and for resume-side recorded payloads.

@history_item n=8 marker=2026-10-13-paste-newline-ledger-row
**2026-10-13-paste-newline-ledger-row** — a ledger row added end to end: `LedgerRow::ALL`, boot stamps all rows, `[NN/MM]` lines, `contract_ledger_probes` ids, Run B's recorded variant, the hygiene walk; the tmpfs live home reads `row-missing` until re-stamped. Precedent for `decision/ledger-row-verify-probe` / `decision/ledger-probe-contract-suite` when this chunk adds a row.

@history_item n=9 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (the relied set; the R8 strip) — `LedgerRow::relied()` is every row on every build with ungated `[&str; N]` literals; the R8 strip removes CLAUDE* names by exact match, `claude_env_keep` the only exception. A new row moves one count; an env-named channel starting `CLAUDE` would be stripped by R8 unless the design says otherwise.

@history_item n=10 marker=2026-11-20-harness-prefix-and-identity-floor-ledger-rows
**2026-11-20-harness-prefix-and-identity-floor-ledger-rows** — `~ decision/r8-claude-env-strip`, `~ decision/claude-env-strip`, `~ decision/literal-test-oracles` (the floor grew 11 → 13, counts dropped from rows); `+ contract/claude-fixtures/env-names`; `+ contract/fake-agent-hooks/session-start-env`; `+ decision/strip-control-oracle-keeps-guarded-names`. Bears directly if the pair is handed over by environment: the strip, its literal oracle and its control rule bind any new child env name.

@history_item n=11 marker=2026-11-18-browser-tools-prompt-modal
**2026-11-18-browser-tools-prompt-modal** — `~ contract/verify-output/step-line`, `~ contract/verify-output/summary-line`, `~ decision/version-gate-needs-all-relied-rows`, `~ decision/ledger-row-verify-probe`, `~ contract/local-live-suite/pass`: a row may end `absent` on one named row only. A new row's verdict words and its effect on `unverified-cli` gating sit on these rows.

@history_item n=12 marker=2026-11-14-budget-gate-removed
**2026-11-14-budget-gate-removed** (4. Unit Test Strategy, version gate) — `~ decision/version-gate-needs-all-relied-rows`: a retired row a stamp still holds changes nothing; a stamp of only older ids reads unverified. Adding a relied row makes every existing stamp unverified until re-stamped: test homes, the harness boot and `contract_ledger_probes` must stamp the new set.

@history_item n=13 marker=2026-10-19-dialog-hold-for-the-driver
**2026-10-19-dialog-hold-for-the-driver** — the precedent for a new `config.json` key: a `SETTINGS` row (`dialog_hold_s`) in `tests/cli_controls_not_disableable.rs` with a booted case where each bad value gives one `parse-rejected{config-json, malformed}`. A configured default model/effort pair key owes the same row under `decision/controls-not-disableable` and `decision/bad-config-value-falls-back`.

@history_item n=14 marker=2026-10-17-cli-machine-contract
**2026-10-17-cli-machine-contract** — the completeness case `every_flag_variable_and_config_key_is_a_row_of_the_table`: `viola --help` options, the `VIOLA_*` literals and `schemas/config.v1.json` properties must equal `CONFIG_KEYS` and each be a row. A new `run` / `revive` flag or config key fails this guard unless added.

@history_item n=15 marker=2026-10-10-budget-governor
**2026-10-10-budget-governor** (strict-modes cases, the gate's negatives) — decided that an out-of-range config value falls back to the key's default with exactly one `parse-rejected{parser:"config-json", detail:"malformed"}` line and never refuses the start. This is the origin of `decision/bad-config-value-falls-back`, which a default-pair key inherits.

@history_item n=16 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** (bounded readers' cap-driving witnesses) — every `config.json` reader (thresholds, dialog hold, `claude_env_keep`) names a cap-driving case in `BOUNDARIES`. A new config reader for the default pair joins `decision/bounded-readers` with its own witness.

@history_item n=17 marker=2026-10-27-bounded-inputs-at-every-boundary
**2026-10-27-bounded-inputs-at-every-boundary** — `tests/contract_bounded_readers.rs`: `product_readers_are_each_bounded_or_named` holds every product reader of external input bounded or named, by census. A new reader (config key, a `claude` model listing) is caught by this census.

@history_item n=18 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** (error-body enumeration) — `tests/contract_error_bodies.rs`: `product_error_writers_are_each_named` and `thiserror_messages_interpolate_nothing`. A new refusal (say, an unknown model or effort word) must be named with a witness and must not interpolate the value.

@history_item n=19 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** (the exit-cause table; revive's codes) — `tests/cross_exit_causes.rs` holds each cause's exit, `--json` document and own hint (`no_two_codes_share_a_hint`, `every_code_is_a_code_word`); revive `--json` codes are closed. A new start refusal for a bad pair owes a table row, its own hint and a code word (`decision/hint-names-its-cause`, `contract/revive-exit-codes/1`).

@history_item n=20 marker=2026-10-10-self-healing-state
**2026-10-10-self-healing-state** (snapshot read and replay) — the classified snapshot read and read-or-replay with one `state-recovered` line; a field no line gave is absent; replay writes no file. Origin of `decision/snapshot-read-through-replay` and the replay's fixed field set that the pair joins.

@history_item n=21 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (E5 built; replay readers; start home-tree check) — six product readers go through `read_snapshot_or_replay`; E5's chaos cases (`tests/chaos_snapshot_replay.rs`) hold what a replay recovers; the start checks the home tree before its log. A replayed pair is read by these same readers, and a revive over a replayed snapshot is where the pair's recovery is witnessed.

@history_item n=22 marker=2026-11-15-live-round-on-the-trimmed-product
**2026-11-15-live-round-on-the-trimmed-product** — `~ contract/event-log/start-wheel-record` (a `wheel{human-input}` may precede session-start under trust keys) and `+ decision/start-lines-read-from-start-offset`. A new start record must be read from the start's own offset, and trust keys can interleave a record into the start's order.

@history_item n=23 marker=2026-11-18-session-folder-trusted-by-viola
**2026-11-18-session-folder-trusted-by-viola** — `+ decision/run-under-test-owned-user-home` and `+ decision/harness-wrappers-under-session-user-home`: a start writes the user's Claude config, so every test `run` / `revive` runs under a test-owned user home. Any test of the launch pair must keep that, and it is the guard that nothing of the real `~/.claude*` is touched.

@history_item n=24 marker=2026-11-17-trust-record-ledger-row
**2026-11-17-trust-record-ledger-row** — `+ decision/verify-under-test-owned-user-home` and `+ decision/no-verify-start-outside-test-home` (census): a ledger row whose verify probe touches user config needs a test-owned user home and is held by the census. Precedent if the model/effort probe reads or writes CLI config.

@history_item n=25 marker=2026-11-20-a-new-cli-version-stamped-automatically
**2026-11-20-a-new-cli-version-stamped-automatically** — `~ decision/single-writer-state-files`: the verify a keyed start runs is still the one stamps writer. A new row changes what that automatic verify stamps at start.

@history_item n=26 marker=2026-11-16-local-command-delivery
**2026-11-16-local-command-delivery** (3. Test Harness Contract; 7. Test Data & Fixtures) — `+ decision/stamping-verify-passes-transcript`: every stamping helper must pass the option the new row's probe needs, or its stamp lacks the row; `~ decision/fake-agent-replays-recorded-bytes` and `~ contract/fake-agent-hooks/stdin-payload` list every payload field the fake agent sets. A probe for the pair repeats both: stamping helpers gain the option, and any set field joins those closed lists.

@history_item n=27 marker=2026-11-19-a-fired-scheduled-wake-keeps-the-wheel
**2026-11-19-a-fired-scheduled-wake-keeps-the-wheel** (7. Test Data & Fixtures) — `+ contract/fake-agent-cli/stop-crons`, `+ contract/fake-agent-cli/wake-after-ms`, `~ decision/fake-agent-replays-recorded-bytes`: each new fake-agent option is its own contract row, and the closed list of set fields grows with it. This is the pattern a `--model` / effort receipt on the fake agent follows.

@history_item n=28 marker=2026-09-29-verify-stamped-test-homes-and-harness
**2026-09-29-verify-stamped-test-homes-and-harness** (fake-agent default version) — the fake agent answers `--version` for the recorded set's version and the harness default equals it (`decision/harness-cli-version-matches-fake`, `contract/fake-agent-cli/cli-version`). A probed model/effort CLI surface is recorded per CLI version under the same default set.

@history_item n=29 marker=2026-10-17-end-to-end-path-binary-and-harness-legs
**2026-10-17-end-to-end-path-binary-and-harness-legs** (status over viola list) — harness `status` runs `viola list --json` and the status document embeds it verbatim (`contract/status-document/list`, `decision/status-shape-ownership`). If the pair is shown on `list`, the agent-run status polled fields move with it.

@history_item n=30 marker=2026-10-17-mcp-server-for-drivers
**2026-10-17-mcp-server-for-drivers** — MCP tool results equal the CLI `--json` document less `v` (`decision/cli-mcp-identical-payloads`, `contract/viola-list-json/mirrors-channel`). If the pair joins `list`, the MCP `list` and CLI documents must change together.

@history_item n=31 marker=2026-11-17-standing-live-rig
**2026-11-17-standing-live-rig** — `+ decision/standing-live-rig` and `~ decision/real-claude-local-only`: the real `claude` is reached only locally, through `viola verify` and the rig's live starts, never in CI. The chunk runs no live start, so the measured CLI channel and its ledger probe are proven against the fake agent only, with the live reading owed later.

@history_item n=32 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (the CI kill line) — the `ci` kill line lists the verify-driven binaries by name, counted by its rule. A new stamped-home test binary for the pair joins `.config/nextest.toml`'s filter, and the key file's count moves.
