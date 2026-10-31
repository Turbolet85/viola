# tests · cascade read · group 6 of 7

## Carried
- (.andromeda/test-plan-detail/e2e-test-strategy.md:259) The three surfaces' witnesses and the chunk's scan legs must satisfy this check as written, with a hit reported by place and class and never by the matched bytes.
  > - **Error sanitization and secret scan:** after every E2E test, the `events` support helper checks:
  >   - CLI `--json`, MCP `isError` and channel `error.data` contain no absolute path (drive letter, `/home/`, `/Users/`, the temp home prefix), no serde path and no `Caused by`
  >   - every -32603 is exactly `"internal error"` / `null`
  >   - every `diagnostics/*` file and log contains no `Cookie` header and no stripped-var values
  >   - `diagnostics/` files are 0600 on Unix
  >   - **Canary** (obs-plan §8, High-class verification): every integration and E2E input is synthetic, and every prompt, send text, tool `input`, dialog answer and statusline payload the tests feed embeds one fixed canary string, a constant in `tests/support`. The scan asserts that the canary never appears in any home-level `<home>/diagnostics/*.ndjson` line. It may appear in `events.ndjson` (contract payloads) and in `instances/<name>/diagnostics/detail-*.ndjson` only. The same scan runs in CI over `target/e2e-home/**`, the harness capture in `target/agent-run/` except `run --mutants`' `target/agent-run/chunk.diff` (repository source text, excluded from the scan and from the `harness-<os>` upload), and `target/nextest/ci/junit.xml` before any upload (obs-plan §9 step order). A hit is reported as file, line, byte offset and pattern class, never the matched bytes.
  (.andromeda/test-plan.md:605)
  > after every E2E test, no absolute path, serde path or `Caused by` in any error payload, every `-32603` exactly `"internal error"` / `null`, no `Cookie` header or stripped-variable value in any log, `diagnostics/` files 0600 on Unix, and the fixed canary in no home-level diagnostics line; a hit is reported by place and class, never the matched bytes.
  (.andromeda/test-plan.md:475)
  > - **ipc-internal (wrapper channel):** the sync client, a raw interprocess stream and jsonschema; the endpoint is read only from `snapshot.json`.
  (.andromeda/test-plan.md:477)
  > - **ipc-internal (MCP stdio):** an rmcp client over a `viola mcp` child the test spawns and owns, closing its stdin and awaiting its exit, never rmcp's `TokioChildProcess`; beside it, a replay of the recorded MCP exchange.
- (.andromeda/test-plan-detail/e2e-test-strategy.md:213) The proof's planted stripped value must follow this form, a registered canary under a name the strip removes, with `CLAUDE_VIOLA_TEST_NEW_X` the name no reading keeps.
  > 1. Run `viola run builder` with `Command::env` set for the 11 identity-floor names plus an unknown `CLAUDE_VIOLA_TEST_NEW_X`. Values are secret-scan canaries (`canary-<kind>-value-<hex4>`, one per name, registered in the `claude-stripped` table), including `CLAUDE_CODE_MESSAGING_TOKEN` and `_SOCKET`.
  (.andromeda/test-plan-detail/e2e-test-strategy.md:216)
  > No stripped name appears in the receipt. On Unix a `claude_env_keep` name survives. `process-start{subject:"claude-child"}` carries `env_stripped_count` (≥ 12), `env_stripped_known` and `env_kept`, names only, and the secret scan finds 0 canaries.
  (.andromeda/test-plan-detail/e2e-test-strategy.md:219)
  > The secret-scan test finds none of the random values in any log or `diagnostics/` file.
  (.andromeda/test-plan.md:561)
  > no stripped name reaches the child, `process-start{subject:"claude-child"}` carries the strip counts and names only, on Unix a `claude_env_keep` name survives and the child holds no viola socket or home path, and the secret scan finds none of the canary values.
- (.andromeda/test-plan-detail/e2e-test-strategy.md:150) The CLI rule of a closed code and a code-shaped or null `detail` must be read against this object-shaped `detail`, and the plan must say whether it stands or changes, because the `-32602` carry also tells this cause apart.
  > `release` with `from` gives exit 20, `{"error":"wrapper-fault","detail":{"code":-32602,…}}` carrying `release-from-driver`.
  (.andromeda/test-plan-detail/e2e-test-strategy.md:141)
  > Run `viola release builder --json` with env `VIOLA_NAME=overseer`.
- (.andromeda/test-plan-detail/e2e-test-strategy.md:145) The MCP `refused: <code>` text and the CLI refusal documents must keep these exact bodies, codes only, and name no `release`.
  > `send` exits 10 with `detail:null`. MCP gives `is_error:true` with text `refused: human-typing`.
  (.andromeda/test-plan-detail/e2e-test-strategy.md:149)
  > The MCP refusal text and `structured_content` do not contain `release` either.
  (.andromeda/test-plan-detail/e2e-test-strategy.md:151)
  > While the injected turn runs, `send` exits 13 with `{"v":1,"refusal":"not-delivered","detail":"turn-running"}`,
  (.andromeda/test-plan-detail/e2e-test-strategy.md:137)
  > Run `viola send builder --json`, then rmcp `send`.
- (.andromeda/test-plan-detail/e2e-test-strategy.md:184) The cross-surface parity the chunk re-asserts must hold this recorded body: MCP `{refusal, detail}` equal to the CLI document less `v`.
  > The MCP form of the refusal landed at chunk 2026-10-17-mcp-server-for-drivers as `mcp_tools_path6_budget_paused_send` (`crates/viola-e2e/tests/mcp_tools.rs`): `isError:true`, `{"refusal":"budget-paused","detail":"five-hour"}` with no cursor, equal to the CLI's document less `v`.
  (.andromeda/test-plan.md:616)
  > for the same refusal, the MCP `structured_content` `{refusal, detail}` equals the CLI `--json` `{refusal, detail}`
- (.andromeda/test-plan-detail/e2e-test-strategy.md:201) The `unverified-cli` bodies must stay true on both surfaces through the chunk's MCP changes, witnessed by the named MCP leg and driven as the step states.
  > `answer` exits 12 `{"refusal":"unverified-cli","detail":null}`. MCP gives `is_error:true` with text `refused: unverified-cli`.
  (.andromeda/test-plan-detail/e2e-test-strategy.md:203)
  > the MCP leg is `mcp_tools_path7_unverified_cli_answer` (`crates/viola-e2e/tests/mcp_tools.rs`), which boots the default `overseer` + `builder` pair unstamped (`SessionOptions { unstamped: true, ..SessionOptions::default() }`), so it also witnesses two first starts on one fresh unstamped home both landing (green on all three CI OSes, as measured at ci#40439188053).
  (.andromeda/test-plan-detail/e2e-test-strategy.md:196)
  > The fake agent fires `PreToolUse.ask-question.json`. Run `viola answer builder <id> --json`, and rmcp `answer`.
- (.andromeda/test-plan-detail/e2e-test-strategy.md:170) The status-line row's cap-driving case must read the arm per OS: the command's stdout only on Unix, no command run on Windows, and exit 0 with empty stderr on every OS.
  > Statusline stdout equals the marker byte-for-byte, with exit 0 and empty stderr. The marker equality is Unix's: there `run` writes the instance's `settings.json` override and the arm runs the recorded command as `/bin/sh -c <command>` under a 5 s bound (PROVISIONAL). On Windows `run` writes no override and the arm runs no command (`override_document` and `shell_argv` return `None` for every input). Exit 0 with empty stderr holds on every OS.
  (.andromeda/test-plan-detail/e2e-test-strategy.md:175)
  > `statusline_command` never runs when strict-modes fails, nor over a replayed snapshot (an unreadable or newer `snapshot.json`, read through `read_snapshot_or_replay`), which carries no `statusline_command`: the arm runs and prints nothing (the unit `statusline_over_a_replayed_snapshot_runs_nothing`).
  (.andromeda/test-plan-detail/e2e-test-strategy.md:161)
  > The user statusline command becomes `viola-fake-agent statusline-echo`, which prints a fixed marker and appends to a marker file. The settings source is named: `<home>/statusline-source.json` (the Claude settings shape; viola only reads it) whenever it exists, else in the default home the user's `.claude/settings.json`, and in any other home none. The pass-through bullets (marker stdout in step 2, the unchanged marker file in step 6) landed at chunk 2026-10-10-statusline-pass-through as root tests on a planted source: `tests/hook_statusline.rs` cases 1 and 5 and `tests/cli_instance_state.rs` `path6_wrapped_statusline_prints_the_user_output_unchanged`, through `tests/support/home.rs` `plant_statusline_source`, `statusline_marker` and `statusline_echo_command`.
  (.andromeda/test-plan-detail/e2e-test-strategy.md:162)
  > into `viola hook statusline` with `VIOLA_NAME=builder VIOLA_DIR=…`.
  (.andromeda/test-plan-detail/e2e-test-strategy.md:184)
  > on Unix the statusline arm prints the marker output and files the marker, and on Windows (`cfg(not(unix))`, run on `windows-2025` in CI) it prints nothing and files no marker.
- (.andromeda/test-plan-detail/e2e-test-strategy.md:172) The thresholds row's oversize-`config.json` case must read its fallback against these boundaries, the default it falls back to being an input of what that case asserts.
  > `send` exits 11 with `detail:"five-hour"`; the 86 case gives `"seven-day"`; exactly 90 pauses and 89 does not.
- (.andromeda/test-plan-detail/e2e-test-strategy.md:246) The MCP schema-violation bodies must stay true and code-only across the chunk's MCP work, in these named cases.
  > The MCP schema-violating calls live in `crates/viola-mcp/tests/handlers.rs` `handlers_schema_violations_write_no_frame` (no frame reaches the test wrapper), the `crates/viola-mcp/src/tools.rs` unit `tool_inputs_are_revalidated_before_any_frame` and the E2E `mcp_tools_list_is_exactly_the_five_tools_and_list_matches_the_cli` (each violation `invalid-params` with its field and no `send-issued`; a control-character or empty text `not-delivered`)
  (.andromeda/test-plan.md:603)
  > nextest runs the MCP schema-violation cases (each `invalid-params` with its field, no frame written)
- (.andromeda/test-plan-detail/e2e-test-strategy.md:246) Any test-only seam the chunk adds to force a `-32603` or MCP handler fault must meet this rule: `fake-agent` only, absent from the release build, recorded in the Decisions Log and a row of the controls table.
  > The seam `src/cmd/hook/seam.rs` fires only on exactly `1` and compiles only with feature `fake-agent`, so `cargo build --release --bin viola` never contains it; it carries its security-plan Decisions Log entry (`2026-09-28`, the founder's live ratification), and its name is a row in the `cli_controls_not_disableable.rs` table.
- (.andromeda/test-plan-detail/e2e-test-strategy.md:247) The owed pre-`initialize` case must stand beside this session-end case and leave it true, its oversize line's canary held to the same scan.
  > The MCP stdin line past `MAX_FRAME`: `mcp_tools_a_line_past_max_frame_ends_the_session_with_one_line` (`crates/viola-e2e/tests/mcp_tools.rs`), from a fresh home of its own, writes an `initialize`, `notifications/initialized` and then one line past `MAX_FRAME` with stdin left open: exit 0, stderr empty, stdout only the `initialize` response, exactly one `parse-rejected{parser:"mcp-stdin", detail:"oversize", count:1}` line in `mcp.ndjson`, valid against `schemas/diag-line.v1.json`, and the line's canary in no file under the home (green on the three CI OSes, as measured at ci#40458923095).
- (.andromeda/test-plan-detail/e2e-test-strategy.md:278) The CLI enumeration must list this one exit-21 document for every cause, with a `null` `detail`.
  > its `--json` document is every exit-21 cause's `{"v":1,"error":"instance-unreachable","detail":null}`.
- (.andromeda/test-plan-detail/e2e-test-strategy.md:335) The killed-wrapper bodies the chunk keeps pinned must stay exit 21 on the CLI and `instance-unreachable` on MCP.
  > endpoint vanishes during `wait`: exit 21
  (.andromeda/test-plan.md:511)
  > a killed wrapper gives CLI exit 21 and MCP `instance-unreachable`.
- (.andromeda/test-plan-detail/e2e-test-strategy.md:288) The CLI enumeration must read `revive`'s landed documents as they stand, a code and a `null` `detail`, and must stop at the start refusals this line owes to the catalogue entry.
  > The `--json` documents landed with chunk 2026-10-17-cli-machine-contract as `tests/cli_revive.rs` cases, one per code: `revive_json_on_a_live_name_is_the_already_live_document`, `revive_json_of_a_name_never_started_is_the_no_session_document`, `revive_json_with_an_id_the_log_does_not_hold_is_the_no_session_document`, `revive_json_of_a_session_open_in_another_claude_is_the_session_live_document`, `revive_json_of_a_snapshot_with_no_cwd_is_the_cwd_missing_document` and `revive_json_refuses_state_another_user_could_write_on_both_arms`, each exit 1 with the `{"v":1,"error":"<code>","detail":null}` document and empty stderr, the human pair unchanged without `--json`.
  (.andromeda/test-plan-detail/e2e-test-strategy.md:289)
  > A refusal `run`'s start makes after a passed preflight (`batch-script-child`, `pinned-hash-mismatch`, `squatted-name`, a `claude` not on `PATH` as `internal-error`) prints no document under `revive --json` and no case asserts one. Joining the rstest table, and that document, are owed to the route entry "Exit-cause code catalogue".
- (.andromeda/test-plan-detail/e2e-test-strategy.md:345) Whichever way the `Refusal` question is answered, the `.cmd` / `.bat` refusal and the shim resolution must keep this exit, line and pair, verified on the Windows runner.
  > `.cmd` child (`#[cfg(windows)]`): `run` exits 1 — landed in `tests/cli_program_resolution.rs` (a `.cmd` and a `.bat` child each exit 1 with no child spawned, `process-exit{subject:"self", exit_code:1, detail:"batch-script-child"}`, exactly the two fixed `unable:`/`hint:` stderr lines; the same binary proves `claude.cmd` → sibling `claude.exe` resolution)
  (.andromeda/test-plan-detail/e2e-test-strategy.md:286)
  > the PTY child is `.cmd` / `.bat` (`#[cfg(windows)]`)
- (.andromeda/test-plan-detail/e2e-test-strategy.md:306) The channel witness and the `-32602` carry must keep this cause's `data: null` while the causes are told apart, and the enumeration must list it.
  > `channel_link_names_are_validated_by_the_wrapper` (`tests/channel_links.rs`, `binary(channel_links)` on the `ci` profile's 20 s kill line) writes its frames through `viola_channel::Client::request`, past every CLI check and the server check. For each of `link` and `unlink`, a `driver` or a `driven` of `../x`, `BUILDER`, `a/b`, an empty string, a 33-character name or a number, a `driven` that is not the serving wrapper's own name, and a `driver` equal to `driven` each get `-32602` with `data: null`, with no `link` / `unlink` record and the snapshot's `links` empty; its control, a valid `link`, answers `already_linked: false` and appends one `link` record.
  (.andromeda/test-plan-detail/e2e-test-strategy.md:305)
  > Frames written straight to the endpoint with `driver` / `driven` set to `../x`, `BUILDER`, `a/b`, an empty string and a 33-character name each get `-32602`, and no `link` / `unlink` record is appended.
- (.andromeda/test-plan-detail/e2e-test-strategy.md:313) The CLI surface's property must keep holding on this cause, and this case's `digit_runs` check is the plan's existing form of a no-path, no-pid oracle.
  > exit 21 with the server-verification cause's `--json` document (stderr empty), hint and `process-exit` line (Exit-cause matrix), no `channel-request` line, no frame reaching the server, and no pid, start time, path or endpoint in any output or line (`digit_runs`).
- (.andromeda/test-plan-detail/e2e-test-strategy.md:371) The five edited rows must each name a witness that drives past the row's cap, and every test of this file must keep holding over them; the error-body enumeration takes this table's literal, checkable form.
  > `tests/contract_bounded_readers.rs` (a `contract_` binary in the E2E layer, `run --e2e`) holds two literal tables, never the product's own list: `BOUNDARIES`, one row per external reader naming its bounded read, its cap, depth and type rule and the existing test that witnesses it, and `EXCEPTIONS`, the reads of viola's own executable (`checked_exe`, `pin_exe`), no external input. `boundaries_each_name_a_bounded_read_that_stands` holds each row's read to its file and its witness to a defined test, and each exception to a read that stands; its other tests are `product_readers_are_each_bounded_or_named`, `census_finds_an_unbounded_read_and_passes_a_bounded_one` and `serde_json_unbounded_depth_is_banned_in_deny_toml` (the `deny.toml` feature ban read).
  (.andromeda/test-plan-detail/e2e-test-strategy.md:372)
  > The census (`product_readers_are_each_bounded_or_named`, its file carrying the `andromeda:walks-tree` token) walks the product source roots, comments, strings and test items left out: a reader call passes when a `take(` stands in its own statement up to the call, or when an `EXCEPTIONS` needle stands on the call's whole source line; its self-check `census_finds_an_unbounded_read_and_passes_a_bounded_one` reads a planted unbounded read as a hit; with the `replace_private_shared` compare bound undone it reads red at exactly `crates/viola-state/src/fs.rs:354` (as measured at `viola-0.1.0/chunks/2026-10-27-bounded-inputs-at-every-boundary/evidence/controls.md`).
  (.andromeda/test-plan.md:623)
  > - Owed: the enumeration's rows for the status-line command's stdout, the child pipe drain shared by `claude --version` and `claude agents --json`, and the `config.json` readers of the thresholds, the dialog hold and `claude_env_keep` name a witness that reads the bounded read without driving past its cap, so their caps are held by the row's needle and the census alone; owner: the route entry "Sanitised error surfaces", as a PREREQ (a cap-driving case for each row).

## Bindings
- **Planted value ↔ the strip's scope in the body**: the plant sets the 11 floor names for stripping with a count of at least 12, (.andromeda/test-plan-detail/e2e-test-strategy.md:213) and (.andromeda/test-plan-detail/e2e-test-strategy.md:216), and the body lists the floor under the strip, (.andromeda/test-plan.md:71), whereas the task's `## Needs` calls the floor kept; the plan must read the strip from these lines and plant under a name stripped on every reading.
- **Sanitisation check ↔ obs-plan**: the canary scan's places and upload order are obs-plan §8 and §9's, so a leg of `v1-47` claimed must read both, (.andromeda/test-plan-detail/e2e-test-strategy.md:264).
- **`-32603` check ↔ the seam rule**: a fault forced to witness `"internal error"` / `null`, (.andromeda/test-plan-detail/e2e-test-strategy.md:261), must come through a seam or fault that meets (.andromeda/test-plan-detail/e2e-test-strategy.md:246).
- **`-32602` causes ↔ obs-plan's metric**: the `release-from-driver` and name-validation causes, (.andromeda/test-plan-detail/e2e-test-strategy.md:150) and (.andromeda/test-plan-detail/e2e-test-strategy.md:306), must stay out of `viola.compat.v_rejected`'s count once the line tells them apart, (.andromeda/obs-plan-detail/metric-coverage.md:28).
- **Session-end case ↔ the §4 PREREQ clause**: the past-cap line after `initialize`, (.andromeda/test-plan-detail/e2e-test-strategy.md:247), and the owed one before it, (.andromeda/test-plan.md:390), are two ends of one stdin cap and must be read together.
- **Bounded-reader table ↔ the witnesses' rules**: the status-line and thresholds rows, (.andromeda/test-plan-detail/e2e-test-strategy.md:371), take their witnesses' oracles from (.andromeda/test-plan-detail/e2e-test-strategy.md:170) and (.andromeda/test-plan-detail/e2e-test-strategy.md:172), and the child pipe drain row from the oversize `unknown` requirement at (.andromeda/test-plan.md:224).
- **Cross-surface parity ↔ the §1 trigger**: the parity bodies, (.andromeda/test-plan-detail/e2e-test-strategy.md:184), are the coverage trigger's identical payloads, (.andromeda/test-plan.md:251).
- **`Refusal` carry ↔ the §1 child-spawn trigger**: the `.cmd` / `.bat` and shim behaviour, (.andromeda/test-plan-detail/e2e-test-strategy.md:345), are this trigger's required cases, (.andromeda/test-plan.md:222) and (.andromeda/test-plan.md:223).
- **CLI enumeration ↔ the route's catalogue entry**: the enumeration reads the landed `revive` documents and leaves the undocumented start refusals to that entry, (.andromeda/test-plan-detail/e2e-test-strategy.md:289).
