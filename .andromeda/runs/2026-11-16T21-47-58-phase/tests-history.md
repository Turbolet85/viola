@history_item n=1 marker=amd-tvc747xe
**amd-tvc747xe** — (2026-10-19-overseer-switch-to-viola) named the `background_tasks` cases this chunk mirrors: units `turn_ended_background_tasks_counts_the_array` / `turn_ended_background_tasks_is_null_when_absent_or_not_an_array` (hook.rs), on-disk `state_events_background_tasks_and_option_details_are_known_keys` with TurnEnded's data keys, Path 3's `hook_a_recorded_stop_carries_its_background_task_count` (recorded 2.1.287 empty array reads 0; non-zero and null at unit tier) and the live 2.1.296 wait/last reading as a local measurement, no suite. The direct precedent for the count's unit, integration and Path 3 placement and the null-vs-0 rule (fct-r6v5qsh2).

@history_item n=2 marker=amd-dyfqp2u2
**amd-dyfqp2u2** — (2026-11-15-api-error-turn-end, §5) `~ contract/event-data/turn-ended`: data keys moved 2 → 3 with `cause`, present on an API-error end only. The row this chunk extends again with the new count key; the precedent for growing the turn-ended key list.

@history_item n=3 marker=amd-ktkoz6oa
**amd-ktkoz6oa** — (2026-11-16-granular-api-error-cause, §5) `~ contract/event-data/turn-ended`: cause became one of nine closed words. The current wording of the row this chunk amends; the new count must be added beside it without disturbing the nine-word set (the last chunk on this row, HEAD 75d636b5).

@history_item n=4 marker=amd-warr2i6n
**amd-warr2i6n** — (2026-11-15-api-error-turn-end, §7) StopFailure registered with no recorded payload, ledger row or verify probe, tests on a synthetic StopFailure payload; `+ decision/send-cases-copy-recorded-fixtures` (a synthetic-only fixtures root draws no input box, so a send after the first turn reads input-not-ready). Bears on whether an API-error turn end carries the count and on how any synthetic Stop payload with non-empty `session_crons` must be rooted.

@history_item n=5 marker=amd-lewfnqh4
**amd-lewfnqh4** — (2026-11-16T06-05-33-wrap, §7) `decision/hook-payloads-recorded-by-verify` except made standing: StopFailure no-ledger-row ratified by the founder. Precedent for P3's ledger-row question (fct-6zblfae3): an undocumented field read with no ledger row and no verify probe was ratified once.

@history_item n=6 marker=amd-5b4mdtyq
**amd-5b4mdtyq** — (2026-11-15-api-error-turn-end, §6) `~ contract/viola-send/turn-running`: after turn-ended, an API-error turn end included, a send is accepted with no pause or release. A picked row on turn-end handling; the count must change nothing in send's turn-running clearance.

@history_item n=7 marker=amd-6grdlnwe
**amd-6grdlnwe** — (2026-10-19-dialog-hold-for-the-driver) the wake set is six by rule `EventKind::WAIT_WAKE` (dialog-expired added), `state_events.rs` round-trips kinds by rule `EventKind::ALL`, viola-core counts cases `ALL` / `WAIT_WAKE`. The wake rule this chunk must leave unchanged (fct-pfrz7a3d); a witness that wait wakes exactly as before reads against these sets.

@history_item n=8 marker=amd-yhvvmkub
**amd-yhvvmkub** — (2026-11-16-wait-wake-condition, §6) `~ contract/viola-wait/non-waking-records`: activity, wheel and harness-origin prompt-submitted never wake a parked wait. Current statement of the non-waking set, a picked row the chunk's "wait wakes exactly as before" acceptance rests on.

@history_item n=9 marker=amd-eyjldzde
**amd-eyjldzde** — (2026-10-04-wait-and-last) Path 3 as landed: `tests/cli_wait_last.rs` (`path3_wait_parks_until_turn_ended_then_last_reads_it`, `wait_after_a_send_cursor_returns_the_turn`, `last_survives_a_wrapper_restart`) over `fixtures/fake-scripts/path3.json`, plus `tests/chaos_wait_vanish.rs`. The CLI `wait --json` / `last` hop sites the count must reach and be asserted at.

@history_item n=10 marker=amd-cnisrb36
**amd-cnisrb36** — (2026-10-17-mcp-server-for-drivers) MCP tests as landed: `crates/viola-mcp/tests/handlers.rs` over the transport-free `Tools` handle, `mcp_tools_path2_path3_send_wait_last_match_the_cli` in `crates/viola-e2e/tests/mcp_tools.rs` comparing each MCP result with the CLI's `--json` less `v`. The MCP wait hop's witness: the count must equal across CLI and MCP (`contract/mcp-tools/result-body`).

@history_item n=11 marker=amd-7t5rnlqi
**amd-7t5rnlqi** — (2026-10-20-epoch-5-cleanup-ii) `tests/mcp_recorded_exchange.rs` holds the served `tools/list` result to the recorded `Mcp.tools-list-1.json` key and type shape. Trap: if MCP `wait`'s declared output shape names the turn-ended data keys, an added key can diverge from the recorded exchange.

@history_item n=12 marker=amd-hlgfwft6
**amd-hlgfwft6** — (2026-10-10-self-healing-state) the event reader's three counts (`three_counts`): torn lines and unknown kinds counted and not returned, known-kind lines with unknown keys counted and returned. The forward-compat guarantee the scope demands for an older reader meeting the new data key (fct-lxwqxf6f).

@history_item n=13 marker=amd-4cs5rgtp
**amd-4cs5rgtp** — (2026-10-11-the-board-viola-list) the forward-compat `list --json` leg landed (`chaos_forward_compat.rs`), `skipped` counts on the board. Where an unknown field on a known kind is witnessed on a surface; a candidate hop if P3 measures `list` carrying the count.

@history_item n=14 marker=amd-kdzdt5fs
**amd-kdzdt5fs** — (2026-10-10-self-healing-state) the crate-level on-disk suite `crates/viola-state/tests/` is `state_events.rs` and `state_replay.rs`. The integration site (named in the scope's grep) where the new TurnEnded data key must round-trip as a known key.

@history_item n=15 marker=amd-tgf2f6sm
**amd-tgf2f6sm** — (2026-10-10-session-links) `state_events.rs` round-trips kinds by the rule `EventKind::ALL`'s length, an unknown kind counted and not returned. Sets the rule-not-count convention the chunk follows when it adds a key, not a kind.

@history_item n=16 marker=amd-hcwxgizg
**amd-hcwxgizg** — (2026-09-29-fake-agent-drift-contract) `contract_fake_agent_drift` compares each hook's stdin byte for byte with every recorded `fixtures/claude/<ver>/` set walked at run time, never insta. The recorded Stop payloads already carrying `session_crons` are replayed unchanged; the count is read from these bytes, not invented by the fake agent.

@history_item n=17 marker=amd-kae2hbyp
**amd-kae2hbyp** — (2026-10-04-dialog-answers-by-dialog-id) default CLI version became 2.1.287 (`DEFAULT_CLI_VERSION`, `RECORDED_CLI_VERSION`, `stamped_home`, `Wrapper::boot`). Which recorded `Stop.default.json` the fake agent replays by default, so which `session_crons` shape an end-to-end case reads.

@history_item n=18 marker=amd-mbgpqmaf
**amd-mbgpqmaf** — (2026-10-19-newer-claude-cli-rows) the 2.1.302 set joined the suite, stamped and hygiene-walked. One of the four recorded sets the scope names as holding `session_crons` (2.1.283, 2.1.287, 2.1.288, 2.1.302); its shape per version is P3's measurement.

@history_item n=19 marker=amd-t37kppzk
**amd-t37kppzk** — (2026-11-16-local-command-delivery, §7) `~ decision/fake-agent-replays-recorded-bytes`: the closed list of fields the fake agent sets (default UserPromptSubmit prompt, --resume source/session_id, --transcript transcript_path, compact session_id). A non-empty `session_crons` case cannot be produced by the fake agent setting a field without widening this list; a synthetic fixture is the standing route.

@history_item n=20 marker=amd-cqpelreg
**amd-cqpelreg** — (2026-11-16-local-command-delivery, §7) `~ contract/fake-agent-hooks/stdin-payload`: the stdin payload is the recorded bytes save the listed set fields. Same constraint as the previous record on the stdin contract row picked for this chunk.

@history_item n=21 marker=amd-6r6ezlbv
**amd-6r6ezlbv** — (2026-11-15-api-error-turn-end, §7) `~ contract/fake-script/format`: ten registered hook events (StopFailure added). The script format a gated turn with a scripted Stop (variant) uses for Path 3-style count cases.

@history_item n=22 marker=amd-5ufeodyv
**amd-5ufeodyv** — (2026-11-16-wait-wake-condition, §7) `~ contract/fake-agent-cli/script`: script steps cover PreToolUse, PermissionRequest, PostToolUse, Stop, Notification and UserPromptSubmit. The step kinds available to a script that ends a turn with a chosen Stop variant.

@history_item n=23 marker=amd-xdupmfpc
**amd-xdupmfpc** — (2026-09-28-capability-ledger-and-viola-verify) fixture hygiene is a run-time directory walk (`claude_fixtures`), not rstest `#[files]` (rstest 0.27 refuses an empty glob at compile time), schema-checked against `schemas/claude-fixture.v1.json`. Any new or synthetic Stop fixture holding wake elements must pass that walk and carry only synthetic text (no prompt, cron or id of a real wake).

@history_item n=24 marker=amd-w5k5ft3a
**amd-w5k5ft3a** — (2026-09-27-hooks-to-normalised-events) hook stdin property landed at 512 cases and fuzz target `hook_stdin` joined. The parser surface the count's read extends; the property/fuzz rows picked for this chunk cover it.

@history_item n=25 marker=amd-646cf32a
**amd-646cf32a** — (2026-10-10-statusline-pass-through) `hook_stdin` seed count stated by rule `ls fuzz/corpus/hook_stdin | wc -l` (12). If a Stop seed with `session_crons` is added the count moves by that rule, not by a literal.

@history_item n=26 marker=amd-quvutcb3
**amd-quvutcb3** — (2026-10-27-bounded-inputs-at-every-boundary) `tests/contract_bounded_readers.rs` census: every product reader of external input bounded in-statement or named. The count is read from the already-bounded hook stdin parse (elements as `IgnoredAny`), so no new reader row should be needed; a new read path would trip the census.

@history_item n=27 marker=amd-ee2rxbze
**amd-ee2rxbze** — (2026-11-05-linux-only-code-and-ci) `LedgerRow::relied()` all rows on every build; `STEPS_PASS`, `ROW_IDS`, `LEDGER_ROWS` literal arrays; older-stamp oracle. If P3 rules the count needs a ledger row, every literal count in `tests/cli_verify.rs`, `contract_ledger_probes.rs` and `--local-live` moves together, and older stamps read unverified.

@history_item n=28 marker=amd-qdobhuzf
**amd-qdobhuzf** — (2026-11-14-budget-gate-removed, §4) `~ decision/version-gate-needs-all-relied-rows`: a retired row a stamp also holds changes nothing. The picked row's current rule, which a new relied row would test against (a stamp lacking the new row reads unverified).

@history_item n=29 marker=amd-5m6sscbm
**amd-5m6sscbm** — (2026-11-16-local-command-delivery, §3) `+ decision/stamping-verify-passes-transcript`: stamping helpers pass `--transcript` for the stamp to hold the 22nd row. Trap: a new relied row's probe input must be carried by every stamping helper, or the shared stamp misses the row.

@history_item n=30 marker=amd-wglepaoj
**amd-wglepaoj** — (2026-11-14-budget-gate-removed, §1) `~ contract/viola-list-json/items` dropped `budget_paused`; the list ok payload lost its budget member. The current item shape of a picked row; if the count reaches `list`, this row's field list is the one to amend.

@history_item n=31 marker=amd-anewhkwa
**amd-anewhkwa** — (2026-11-16-wait-wake-condition, §3) `+ decision/one-verify-stamp-per-build` and the stamp dir's sweep except. The shared-stamp mechanism the directive requires a stamped home to copy from (fct-3vhp4th7).

@history_item n=32 marker=amd-smxrzozm
**amd-smxrzozm** — (2026-11-16-wait-wake-condition, §3) `~ decision/verify-driven-tests-20s-kill` why: one stamp per build keeps each stamped-home test under the 20 s kill. A new stamped-home binary joins that kill line and stays under it only by copying the shared stamp.

@history_item n=33 marker=amd-jiwpp76u
**amd-jiwpp76u** — (2026-11-16-wait-wake-condition, §5) `~ decision/fresh-home-per-test`: a stamped home is a copy of its build's verify stamp, never a link. Rule every new end-to-end count case follows.

@history_item n=34 marker=amd-tveox2hq
**amd-tveox2hq** — (2026-11-15-mutation-and-coverage-baselines-green, §3) `~ decision/root-wait-bound`: root waits 12 s, below the mutants 15 s kill and the 20 s cargo-mutants floor. The bound any new wait on a channel or thread takes (fct-3vhp4th7).

@history_item n=35 marker=amd-kmpo2mdb
**amd-kmpo2mdb** — (2026-11-15-mutation-and-coverage-baselines-green, §3) `~ decision/within-7s-wait-bound`: `WITHIN` = 12 s. The shared bound for every test wait not bounded by a per-test kill.

@history_item n=36 marker=amd-gjhi22be
**amd-gjhi22be** — (2026-11-15-mutation-and-coverage-baselines-green, §3) `~ decision/mutants-kill-below-cargo-mutants-timeout`: default mutants kill 15 s, a departure from fixing slow cases by removing slow work. Constraint on any new waits a root-package mutant can reach.

@history_item n=37 marker=amd-3fwhzts6
**amd-3fwhzts6** — (2026-10-26-home-and-code-bearing-file-integrity) remove-the-guard witnesses that let a product process start are bounded by `WITHIN` through `tests/support/cli.rs` `Running` (an unbounded one hung 27 minutes). The directive's controls (`--timeout S`, filtered cases, FAILED with no TIMEOUT) inherit this trap.

@history_item n=38 marker=amd-5gt53dby
**amd-5gt53dby** — (2026-09-28-mutation-testing-to-the-epoch-boundary) no per-chunk, pre-push or CI mutation gate; `run --mutants` named-only, mutation at the epoch-boundary audit. Matches the directive's "no mutation run": the chunk's strength rests on hand-applied remove-the-guard controls.

@history_item n=39 marker=amd-e54mtnrn
**amd-e54mtnrn** — (2026-10-08-first-live-test-and-self-drive) a route entry's live readings of a real session are that chunk's own local measurement, not a test layer, never in CI (`decision/live-readings-not-a-test-layer`). Any live `session_crons` reading P3 shapes (no live claude start is given here) stays evidence, never a suite.

@history_item n=40 marker=amd-zrj7cxkl
**amd-zrj7cxkl** — (2026-11-05-linux-only-code-and-ci) OS scope Linux only, CI on `ubuntu-latest`; the absence row `absence/windows-and-macos-support`. No per-OS arm or gate for the new count's tests.
