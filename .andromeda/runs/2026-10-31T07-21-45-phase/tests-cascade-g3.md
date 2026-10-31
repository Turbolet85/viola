# tests · cascade read · group 3 of 7

## Carried
- (.andromeda/test-plan-detail/unit-test-strategy.md:10) The channel witness and its unit cases must hold these codes, `data` shapes and the exact `-32603` body, and the `-32602` carry must tell its causes apart without changing any `error` body on the wire beyond a code or null.
  > JSON-RPC error mapping: -32700 / -32600 / -32601 / -32602 with `data:{supported,wrapper}` / -32603 with the exact `"internal error"` and `data:null`.
  (.andromeda/test-plan.md:380)
  > the JSON-RPC error mapping
- (.andromeda/test-plan-detail/unit-test-strategy.md:29) The `thiserror` clause requires every product `Display` that can reach a surface to be fixed, so `StampsError` and `PinError` must stay under a fixed-message case and the enumeration must account for them.
  > Stamps and pin: `update_stamps_refuses_a_widened_ledger_and_leaves_it` (`StampsError::StrictModes`, checked before `create_private_dir(ledger/)` and again under the lock, nothing read or written), `stamps_error_messages_are_fixed`, `pin_key_is_the_version_and_the_content_key` and `pin_companions_refuses_a_path_another_user_may_write` (`PinError::StrictModes`, nothing written).
  (.andromeda/test-plan-detail/unit-test-strategy.md:30)
  > the lost placement (`pin::place_absent`): `place_absent_over_a_copy_another_start_placed` (the same bytes reused, other bytes `PinError::HashMismatch` with the file byte-identical) and `place_absent_on_a_free_path_writes_the_copy`.
- (.andromeda/test-plan-detail/unit-test-strategy.md:26) A strict-modes refusal that names its refused entry is a path-bearing message the `thiserror` clause rules over: the entry named here must never reach a `--json`, MCP or channel body, whatever P3 reads of the `Display` that holds it.
  > `strict_instance_refuses_a_plain_home_made_before_viola_and_names_the_entry` refuses a home made with `create_dir_all` before viola, its refused entry an inherited write grant to a SID strict-modes does not trust, whichever SID holds it (its oracle below).
- (.andromeda/test-plan-detail/unit-test-strategy.md:28) The candidate claim's leg of content and drift reports reaching only owner-readable detail files reads these checks and the opens beneath them, so P3's reading and any claim of `v1-47` must name them as its witnesses or leave the leg unclaimed.
  > The home-tree and diagnostics checks: `check_home_tree_of_a_tree_viola_wrote_passes`, `check_home_tree_refuses_each_widened_path`, `check_diagnostics_of_what_viola_wrote_or_of_nothing_passes`, `check_diagnostics_refuses_a_widened_mode` and `check_diagnostics_refuses_a_symlinked_role_file_and_folder` (a symlink is refused, a folder is held to the read rule, a file to the read rule on Unix and the write rule on Windows, and an absent path passes), and `strict_instance_holds_the_content_folders_to_the_read_rule`.
  (.andromeda/test-plan-detail/unit-test-strategy.md:81)
  > The diagnostics opens (`src/obs.rs`): a `diagnostics/` folder or a role or detail file that fails `strict::check_diagnostics` is a failed open, checked before `ensure_private_dir` narrows anything, so the writer ends at `std::io::sink` and a detail line is dropped (`open_role_file_refuses_a_widened_diagnostics_path_and_leaves_it`, `open_role_file_refuses_a_symlinked_role_file`, `write_detail_refuses_a_widened_diagnostics_folder_and_leaves_it`).
- (.andromeda/test-plan-detail/unit-test-strategy.md:19) The status-line row's cap-driving witness must run over a present snapshot from a real wrapper, because a replayed one holds no command and would run nothing, so its cap would never be reached.
  > `src/cmd/hook/statusline.rs` `serve` (a replayed snapshot holds no `statusline_command`)
  (.andromeda/test-plan-detail/unit-test-strategy.md:20)
  > `statusline_over_a_replayed_snapshot_runs_nothing` (`src/cmd/hook/statusline.rs`)
- (.andromeda/test-plan-detail/unit-test-strategy.md:23) The MCP witness re-asserts `instance-unreachable` as a code-only body, so a snapshot read failure must stay this `None` on that path, its read error's text never reaching the result.
  > `live_endpoint` returns `LiveEndpoint { endpoint, pid, started_at }`, all from its one `read_snapshot_or_replay` call: only a present snapshot proceeds, and a replayed or absent snapshot or a read error is `None`, since a replayed snapshot carries no `endpoint`, `pid` or `started_at`.
- (.andromeda/test-plan-detail/unit-test-strategy.md:35) The planted stripped `CLAUDE*` value must sit under a name this rule removes and outside the persistent set, and the `claude_env_keep` row's oversize case must read what its fallback does to the persistent set against this prefix rule, since a value under a kept name proves nothing.
  > The R8 strip against a literal copy of the 11-name identity floor measured on the Windows host (`CLAUDECODE`, `CLAUDE_CODE_BRIDGE_SESSION_ID`, `CLAUDE_CODE_CHILD_SESSION`, `CLAUDE_CODE_ENTRYPOINT`, `CLAUDE_CODE_EXECPATH`, `CLAUDE_CODE_MESSAGING_SOCKET`, `CLAUDE_CODE_MESSAGING_TOKEN`, `CLAUDE_CODE_SESSION_ATTENDED`, `CLAUDE_CODE_SESSION_ID`, `CLAUDE_EFFORT`, `CLAUDE_PID`) written into the test, never imported from `IDENTITY_FLOOR`, plus the prefix rule: every inherited `CLAUDE*` name is removed (ASCII case-insensitive on Windows, exact on Unix) unless it is in the persistent set, and a floor name is removed whatever the persistent set says. A Linux dev-host reading on `claude` 2.1.287 stands beside the Windows one and differs (12 names in a hook's environment, four outside the eleven, three of the eleven absent; architecture §Occupied Resources → Environment variables); the floor and this literal stay the eleven.
  (.andromeda/test-plan.md:384)
  > the R8 strip against a literal copy of the identity floor (never imported)
- (.andromeda/test-plan-detail/unit-test-strategy.md:36) The `Refusal` carry reaches `resolve_program`, so this resolution must keep its behaviour whether the enum is folded into `AgentError` or ratified as an exception.
  > npm-shim → `claude.exe` resolution over fixture shim files in a tempdir.
  (.andromeda/test-plan.md:384)
  > npm-shim resolution
- (.andromeda/test-plan-detail/unit-test-strategy.md:40) The child pipe drain row owes a case past its cap, and its oversize `claude agents --json` must read `unknown` through the closed `AgentsFailure`, leaving the board as this line states.
  > The `claude agents --json` parser: an over-`MAX_FRAME` or malformed payload gives `unknown`, and a row `name` never becomes a `ViolaName`. As landed (chunk 2026-10-16-unwrapped-sessions-on-the-board): `agents::parse` returns `AgentsList { rows, skipped, tolerated }` or the closed `AgentsFailure` (`Oversize` · `Malformed`), and a failed, oversize or malformed read leaves the board as `board::read` gave it (STATUS `unknown`, no unwrapped row); the row's `sessionId` is its NAME, never a `ViolaName`; cases `agents_parse_*`, `agents_status_maps_each_measured_word_and_nothing_else`, `agents_row_joins_by_child_pid_or_session_id` and `agents_failure_details_are_the_closed_words`.
- (.andromeda/test-plan-detail/unit-test-strategy.md:52) The PREREQ owes the missing `serve` child case, a line past `MAX_FRAME` before `initialize` completes ending in `ServeError::Initialize` under the same 5 s bound as its neighbours, at the cap these cases pin, while the MCP witness keeps the handlers' `isError` results code-only.
  > The `serve` child cases (`crates/viola-mcp/src/lib.rs`), each child killed at its 5 s bound: `serve_ends_the_session_at_a_line_past_max_frame` and `serve_on_a_closed_stdin_is_the_initialize_error` (`ServeError::Initialize`).
  (.andromeda/test-plan-detail/unit-test-strategy.md:51)
  > The stdin line cap (`crates/viola-mcp/src/stdin.rs`, `stdin::tests`): `CappedLines` yields whole lines of at most the cap bytes, the `\n` counted; a line of exactly the cap passes whole, and one byte more releases no byte, sets the cut flag and reads as end of stream, at the literal `MAX_FRAME` (`16 * 1024 * 1024`) too;
  (.andromeda/test-plan.md:390)
  > not driven by any test: a stdin line past `MAX_FRAME` landing before `initialize` completes ends `serve` with `ServeError::Initialize` (exit 11), as a closed stdin before `initialize` does; owner: the route entry "Sanitised error surfaces", as a PREREQ
  (.andromeda/test-plan.md:387)
  > a non-`ViolaName` `target`, a non-`u64` `dialog_id`, an open-string `behavior` and control chars in `text` all give `isError:true` with code-only payloads
- (.andromeda/test-plan-detail/unit-test-strategy.md:59) The CLI `--json` clause requires a failure the verb cannot name to be no free text, so the `unknown` fallback must keep its exit 14 document, its `detail` code-shaped or null, never a panic and never exit 20.
  > The exit-code mapping from `RefusalReason` / fault to 0/1/2/10–14/20/21. It includes the multi-version-compat case: a wrapper `{"result":{"refusal":"<literal unknown string>","detail":…}}` payload deserialises to the `#[serde(other)] unknown` variant and maps to exit 14 with a `--json` refusal document. It never panics and never maps to 20.
  (.andromeda/test-plan.md:392)
  > the exit-code mapping (an unknown refusal maps to exit 14, never a panic and never 20)
  (.andromeda/test-plan.md:372)
  > `RefusalReason` serde round-trip in kebab-case, including the `#[serde(other)] unknown` fallback.
- (.andromeda/test-plan-detail/unit-test-strategy.md:60) `release-from-driver` is one of the `-32602` causes the carry must tell apart on the `channel-response` line, and its refusal must stay a code on the channel surface.
  > The wheel state machine: human editing key → `human`; harness-origin prompt → no flip; `release` only via the CLI path; `release` with `from` → `release-from-driver`.
- (.andromeda/test-plan-detail/unit-test-strategy.md:61) The thresholds row's cap-driving case must sit beside these reader cases and read an oversize `config.json` to the fallback the product gives, within `Percent`'s range.
  > Beside them: `percent_*`, `thresholds_*`, `gate_eval_*` and `gate_release_*` (`src/run/budget.rs`, `src/cmd/run.rs`, `src/run/wheel.rs`)
  (.andromeda/test-plan.md:374)
  > `Percent` accepts 0–100 and rejects 101 and negatives.
- (.andromeda/test-plan-detail/unit-test-strategy.md:73) The dialog hold row's named witness reads the bound without driving past it, so it must gain an oversize `config.json` case past `take(MAX_FRAME)` that reads the fallback to sixty with its one rejection.
  > `read_hold` takes `config.json`'s `dialog_hold_s` once at start through `take(MAX_FRAME)` and `v` = 1 (`read_hold_takes_a_whole_number_of_seconds_in_range`, `read_hold_falls_back_to_sixty_with_one_rejection`)
  (.andromeda/test-plan.md:375)
  > `DialogHold` accepts whole seconds 60–3600 and rejects 0, 59, 3601 and `u16::MAX`, its `Default` the 60 s `DIALOG_HOLD_DEFAULT`
- (.andromeda/test-plan-detail/unit-test-strategy.md:79) The channel surface's `InvalidParams` and `-32603` bodies must be witnessed, and a forced handler fault must come from a file or process fault like this append failure, never from a failpoint crate.
  > `links_set_method_refuses_a_bad_pair_with_invalid_params`, `links_set_method_a_from_changes_nothing` and `links_set_method_an_append_that_fails_is_internal_with_the_set_unchanged` (`src/run/links.rs`)
  (.andromeda/test-plan.md:920)
  > NEVER introduce a failpoint crate that pulls tokio into the sync crates (for example fail-parallel). Fault seams go through mockall doubles of existing traits plus process and file faults.
- (.andromeda/test-plan-detail/unit-test-strategy.md:74) A unit case that captures the changed `channel-response` line, or any other log line of this chunk, must go through the helper in the root bin's unit modules, and a capture in `crates/viola-channel/src/lib.rs` must not read this rule as already holding there.
  > Scoped log captures: every scoped capture in the root bin's unit modules goes through `crate::test_support::with_scoped_default` (`src/main.rs`)
  (.andromeda/test-plan-detail/unit-test-strategy.md:76)
  > The three crate-level scoped captures that share the mechanism, in `crates/viola-state/src/lib.rs`, `crates/viola-channel/src/test_support.rs` and `crates/viola-channel/src/lib.rs`, run on a bare `with_default`; never observed red, they have no owner.

## Bindings
- **The channel error mapping ↔ the §6 error sanitization sweep**: the unit case and the after-every-E2E scan must both read `-32603` as exactly `"internal error"` / null, the one literal on both layers, (.andromeda/test-plan-detail/unit-test-strategy.md:10) and (.andromeda/test-plan.md:605).
- **The cap-driving unit and reader cases ↔ the §6 Contract suite's owed clause**: each of the five `BOUNDARIES` rows must name a witness that drives past its cap, the agents parser, the hold reader, the thresholds readers and the `claude_env_keep` fallback standing as the behaviours those witnesses read, (.andromeda/test-plan-detail/unit-test-strategy.md:40), (.andromeda/test-plan-detail/unit-test-strategy.md:73), (.andromeda/test-plan-detail/unit-test-strategy.md:61), (.andromeda/test-plan-detail/unit-test-strategy.md:35) and (.andromeda/test-plan.md:623).
- **The `unknown` exit-code mapping ↔ the §6 cross-surface parity**: the CLI `--json` `{refusal, detail}` the mapping prints must equal the MCP `structured_content` for the same refusal, (.andromeda/test-plan-detail/unit-test-strategy.md:59) and (.andromeda/test-plan.md:616).
- **The R8 strip's unit rule ↔ scenario E2**: the planted value the witnesses use must be one the strip removes, as E2's canary scan reads it, (.andromeda/test-plan-detail/unit-test-strategy.md:35) and (.andromeda/test-plan.md:561).
- **`release-from-driver` and the mapping's `-32602` ↔ obs-plan's `viola.compat.v_rejected`**: telling the causes apart must leave the metric counting only the unsupported-version cause, (.andromeda/test-plan-detail/unit-test-strategy.md:60), (.andromeda/test-plan-detail/unit-test-strategy.md:10) and (.andromeda/obs-plan-detail/metric-coverage.md:28).
- **The diagnostics checks and opens ↔ the §6 sweep's `diagnostics/` clause and the `v1-47` leg**: the unit refusals and the after-every-E2E reading of `diagnostics/` files at 0600 on Unix must both stand for the leg to be claimed, (.andromeda/test-plan-detail/unit-test-strategy.md:28), (.andromeda/test-plan-detail/unit-test-strategy.md:81) and (.andromeda/test-plan.md:605).
