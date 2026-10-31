# Task — architecture · 2026-10-31-exit-cause-code-catalogue

## The session
Every exit-21 cause now has its own code: the document reads `{"v":1,"error":"instance-unreachable","detail":"<cause>"}` on CLI `--json`, and MCP's `isError` result carries the same body less `v`; no exit, `RefusalReason` or exit code was added.
Exit 1 gained a distinct code where causes shared one or had none: `stale-heartbeat` for a stale holder, `viola revive --json` documents for every refusal of its start, an `internal-error` document at the `cli` catch site under `--json`, and `verify`'s log codes on `process-exit`.
Added `viola_core::UnreachableCause`, `viola_state::liveness::without_endpoint`, `client::unreachable_document`, `refused_with`, `json_asked`, `internal_error_document`, `Ended::Refused`, the unwrapped-name and endpoint-lost hints, and the table `tests/cross_exit_causes.rs`.
Moved: the `process-exit` `detail` enum in `schemas/diag-line.v1.json` from 13 to 19 members, and the nextest `ci` kill line from 24 to 25 `binary(` operands.
Retired: exit 21's `detail: null`, `already-live` as a stale holder's refusal, `instance-dead` with `during` `call`, the `revive --json` post-preflight gap, and the `NOT_ERROR_BODIES` row for start refusals with no document.
Measured: CI green on final HEAD `8a4c1a2c`, checks 15/15; `v1-18` met with named carve-outs (the socket-directory cause and macOS's server check), `v1-39` met.

## Reached files
- tests/cross_exit_causes.rs
- .config/nextest.toml
- Cargo.toml
- schemas/diag-line.v1.json
- crates/viola-core/src/lib.rs
- crates/viola-state/src/liveness.rs
- crates/viola-mcp/src/result.rs
- crates/viola-mcp/src/tools.rs
- src/main.rs
- src/human/hints.rs
- src/cmd/mod.rs
- src/cmd/client.rs
- src/cmd/send.rs
- src/cmd/wait.rs
- src/cmd/last.rs
- src/cmd/answer.rs
- src/cmd/pause.rs
- src/cmd/link.rs
- src/cmd/list.rs
- src/cmd/plugin.rs
- src/cmd/run.rs
- src/cmd/revive.rs
- src/cmd/verify.rs
- crates/viola-mcp/tests/handlers.rs
- crates/viola-e2e/tests/mcp_tools.rs
- tests/support/start.rs
- tests/contract_error_bodies.rs
- tests/chaos_wait_vanish.rs
- tests/cli_send.rs
- tests/cli_list.rs
- tests/cli_machine_contract.rs
- tests/cli_wait_last.rs
- tests/cli_plugin_install.rs
- tests/cli_links.rs
- tests/cli_error_bodies.rs
- tests/cli_instance_state.rs
- tests/cli_revive.rs
- tests/cli_verify.rs
- tests/security_negatives_server_identity.rs
- tests/security_negatives_strict_modes.rs

## Touches
- The report's Changes list 41 files and this section holds 40: the 41st is `tests/cli_controls_not_disableable.rs`, one line added, and it counts as reached.
- The exit-21 document form: `.andromeda/architecture.md:226` and `.andromeda/architecture-detail/standard-contracts.md:111` state `detail` null; it is now the cause word, one form per cause.
- The exit-1 document's codes: `.andromeda/architecture.md:228` and `.andromeda/architecture-detail/standard-contracts.md:117` list five; the list is now ten, adding `stale-heartbeat`, `batch-script-child`, `pinned-hash-mismatch`, `squatted-name` and `internal-error`.
- The sentence that a refusal of `run`'s start after a passed preflight prints no document is no longer true, and its pointer owed to the working-route entry Exit-cause code catalogue is spent, in the body and in contract-cli-json-output.
- `viola plugin install`'s refused strict-modes check: `.andromeda/architecture.md:229` names `client::refused_document` as its writer and implies a null `detail`; it now writes through `client::unreachable_document` with `strict-modes-failed`.
- Any text naming `client::refused_document` as a writer of the exit-21 document: that function now holds exit 1 only.
- The CLI exit-code rows: conventions.md hits at `:35`, `:36`, `:46`, `:64`, `:70`, `:76` and `:89` in `.andromeda/architecture-detail/`, and `.andromeda/architecture.md` §Conventions **CLI exit codes**, where a null `detail`, one exit-21 form, a stale holder reading `already-live`, or a `revive --json` gap is stated.
- The catch site: `.andromeda/architecture-detail/conventions.md:38`, where it states the output under `--json` too, takes the `internal-error` document; the human line `error: internal error` stands byte for byte without `--json`.
- The MCP unreachable result: conv-mcp-results and `.andromeda/architecture.md` §Conventions **MCP** say it is shaped like CLI `--json`; that now holds a cause word, with no frame and no line on any cause.
- dec-mcp at `.andromeda/architecture-detail/established-decisions.md:124` and dec-cli-conventions at `:305`, per the plan's expected amendments.
- dec-session-liveness: the rule that a `live` or `stale` name refuses to start stands, but the two now read distinct codes, and a stale instance reads `instance-dead` on a frame verb.
- `.andromeda/architecture-detail/occupied-resources.md:42` and `:168`, hits the report lists for the plan's search over the exit-21 document, `revive --json`, the entry's title, the stale holder and `instance-dead`'s `during`.
- `.andromeda/architecture-detail/occupied-resources.md:148` at offset c1741 and `:149` at offset c1362: the exit-21 document in those long lines moved.
- `.andromeda/architecture-detail/occupied-resources.md:122` at offset c926: `instance-dead` no longer carries `during` `call`.
- contract-snapshot-envelope and contract-session-liveness, indirectly through `without_endpoint`: `live_endpoint`'s none now splits into a cause, and a replayed snapshot's fresh beat still refuses `already-live`.
- The `process-exit` `detail` enum in `schemas/diag-line.v1.json:78`, 13 to 19 members: any standing count or list of its values; the `channel-response` three words and `hook-decision` seven did not move.
- `Unreachable::Dead(&'static str)` is gone from `src/cmd/client.rs`, with `Unreachable` now `Dead`, `Unwrapped`, `Lost` and `ServerUnverified`: any text naming the old shape or its line.
- The nextest `ci` kill line at `.config/nextest.toml:24`, 24 to 25 `binary(` operands: any text stating the count or listing its binaries; the report found the ledger's and the screen's 24 rows are another fact sharing the number.
- The test inventory: one new root `cross_` binary with its `[[test]]` in `Cargo.toml`, `required-features = ["fake-agent"]`, which the E2E regex takes as it took `path1_`; any count of root test binaries moves by one.
- crosscut-sanitised-error-surfaces, through `tests/contract_error_bodies.rs`: new error-body rows named, and the `NOT_ERROR_BODIES` row for post-preflight start refusals retired.
- `human::refuse(` stays 18 sites by the report's re-read; `refused_document` keeps its signature and its exit-1 callers: a count or caller claim re-read as unmoved.
- dec-cli-version-compatibility, through `src/cmd/verify.rs`: where it states `verify`'s `process-exit` codes or its refusals, `record` and `refuse_recording` now return `Ended`.
- dec-module-boundaries and dec-error-handling, indirectly: `viola-state` gains a `viola_core::UnreachableCause` use, `viola-core` stays pure, `RefusalReason` and the crate error enums gain nothing.
- Line citations into any reached file: citations of `src/cmd/client.rs`, `src/cmd/run.rs`, `src/cmd/revive.rs`, `src/cmd/verify.rs`, `src/main.rs`, `crates/viola-mcp/src/result.rs` or `crates/viola-mcp/src/tools.rs` may have shifted.
- Test setups moved into `tests/support/start.rs` (`remove_dir_once_free`, `FIXTURE_SESSION`, `kill_wrapper` among them), and `Frozen` in `tests/cli_instance_state.rs` now exists on Unix too: any text placing them elsewhere or calling `Frozen` Windows-only.
- Any text saying an exit-cause code, the exit-cause table or the unwrapped-name hint is not built, or is owed to the Exit-cause code catalogue entry.

## New facts
- F1 · standard-contracts · The exit-21 `detail` is one of five closed causes, `strict-modes-failed`, `instance-dead`, `unwrapped-name`, `endpoint-lost` and `server-verify-failed`, with `v` still 1 and `error` still `instance-unreachable` (the founder's own word, `inputs#I2` Q1).
- F2 · conventions · `viola_core::UnreachableCause` (`StrictModes`, `InstanceDead`, `UnwrappedName`, `EndpointLost`, `ServerVerify`, with `as_str`) has no serde path; every word is written by the two clients, and it is no `RefusalReason`.
- F3 · established-decisions · `viola_state::liveness::without_endpoint` reads the instance directory by `symlink_metadata`: a directory is `InstanceDead`, anything else (absent, a regular file, a symlink) is `UnwrappedName`; it opens nothing, writes no line, stays behind the strict-modes check at every caller, and its reading is the delegate's word, PROVISIONAL (`inputs#I2` Q2).
- F4 · standard-contracts · `client::unreachable_document` is the one writer of the exit-21 CLI document; `refuse_untrusted` and `src/cmd/plugin.rs` write `StrictModes` through it, and `src/cmd/list.rs`'s `refused_document` builds `strict-modes-failed` as a `Value` for `board_document`, the board's only refusal.
- F5 · conventions · MCP's `isError` body is the CLI document less `v`, its text stays `refused: instance-unreachable`, no cause writes a frame or a line, JSON-RPC errors are unmoved and `-32603` keeps its fixed body.
- F6 · established-decisions · The MCP mapping: `of_reply` reads `Connect` as `InstanceDead` and `Closed | Io` as `EndpointLost`, `of_board`'s refusal is `StrictModes`, and `tools.rs` `frame` reads a refused `check_instance` as `StrictModes`, no `live_endpoint` by `without_endpoint`, and a failed `server_verified` as `ServerVerify`.
- F7 · conventions · The CLI mapping: `answer_of` reads `Connect` as `Dead` and `Closed | Io` as `Lost`, and the six no-endpoint arms (`send`, `wait`, `last`, `answer`, `pause`, `link`, with `release` and `unlink` reaching them) read `Unreachable::without_endpoint`.
- F8 · occupied-resources · On `process-exit`, `endpoint-lost` carries `during:"call"` while `instance-dead`, `unwrapped-name` and `server-verify-failed` carry `during:"connect"`.
- F9 · established-decisions · `collision_check` prints and logs `stale-heartbeat` for a stale holder and `already-live` for a live holder or a fresh replayed beat; the `run.collision_check` span's `outcome` stays `already-live` for both, and stale heartbeat is exit 1's cause alone.
- F10 · standard-contracts · `Launch` gains `json: bool` (`run` passes false, `revive` passes `args.json`), and `refused_with` prints `refused_document` in place of the human pair for `batch-script-child`, `pinned-hash-mismatch`, `squatted-name` and a program not found as `internal-error`, all before `spawn_child`.
- F11 · standard-contracts · `viola revive --json` prints one exit-1 document for every refusal of its start; an anyhow error from `trusted`, `checked_exe`, `open_wrapper_log`, `preflight` or `start` prints `internal-error` and returns the same error, and a `pump_child` error prints no document because the child has held the terminal.
- F12 · conventions · `src/main.rs` `json_asked` reads `--json` from argv before a `--`; where `prints_internal_error` is true it prints `internal_error_document` on stdout in place of the human line, stderr empty, and `run`, `revive`'s start, `mcp` and `hook` stay silent at that site.
- F13 · established-decisions · `verify` writes `Ended::Refused`'s code on the `process-exit` it logs under a parsed `VIOLA_NAME`: `batch-script-child`, `program-not-found`, `pinned-hash-mismatch`, `version-unreadable`, `strict-modes-failed`, `unclean-fixture`, beside `interrupted`; a refused home tree stays lineless, a failing row has no code, the fixture's `<file>` enters no code, and `verify` takes no `--json`.
- F14 · occupied-resources · `schemas/diag-line.v1.json`'s `process-exit` `detail` now admits `stale-heartbeat`, `unwrapped-name`, `endpoint-lost`, `program-not-found`, `version-unreadable` and `unclean-fixture`.
- F15 · conventions · The human hints: `src/human/hints.rs` holds an unwrapped-name hint and a new endpoint-lost hint, last on stderr and plain ASCII; the hint cause keys are `not-running`, `unwrapped-name`, `endpoint-lost` and `server-verify`, `not-running` is kept unrenamed, and the endpoint-lost wording is the delegate's word, PROVISIONAL (`inputs#I2` Q3).
- F16 · occupied-resources · `tests/cross_exit_causes.rs` is the exit-cause table, a root `cross_` binary of 39 tests on Linux, with `no_two_codes_share_a_hint`, `shared_hints_reads_a_planted_duplicate`, `no_hint_names_release` and `every_code_is_a_code_word` as its controls.
- F17 · cross-cutting-patterns · Every new cause and exit-1 word passes `is_code_word` and is a row of `tests/contract_error_bodies.rs`, carrying no path, pid or upstream text.
- F18 · conventions · An absence: the socket-directory refusal has no code, `bind_endpoint`'s `create_private_dir` failure being an anyhow error at the catch site, owned by the route entry Unix endpoint and home hardening, the delegate's word, PROVISIONAL (`inputs#I3`).
- F19 · conventions · An exception: `server-verify-failed` does not arise on macOS, which reports no peer pid; `v1-18`'s carve-out leaves macOS's server check with Unix endpoint and home hardening (`inputs#I2` Q4, the founder's own word).

## Untouched
- dec-pty and dec-screen-model: the session changed nothing in `viola-pty`, vt100 or the readiness gate.
- dec-hook-contract, dec-hook-transport and contract-hook-statusline: `src/cmd/hook.rs` is not in the diff, `hook` stays silent at the catch site, and it still always exits 0.
- dec-message-broker-ipc and the wrapper channel frames: nothing in `viola-channel` moved, and the causes are read on the client side of the channel.
- contract-event-data: no event kind or `data` shape moved; the new codes are process-log `detail` words, never `events.ndjson` content.
- dec-snapshot-writer and dec-database-state-store: no writer, lock or append path changed, and `without_endpoint` writes nothing.
- crosscut-capability-ledger and contract-ledger-stamps: the rows live in `viola-agent-claude` and the stamps in `viola_state::stamps`, neither in the diff; `verify`'s change is log codes on its refusals only.
- crosscut-bounded-readers: no reader of external bytes was added, since `without_endpoint` opens nothing.
- res-repository's fixtures, fake scripts, `vendor/conpty`, `fuzz/`, `scripts` and `target/` rows: the diff holds nothing there beyond the one schema named above.
- stack's toolchain and CI tool pins: `rust-toolchain.toml` and CI's pinned `tool:` line read as they stood.
