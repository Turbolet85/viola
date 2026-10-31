# Task — security-plan · 2026-10-31-exit-cause-code-catalogue

## The session
Exit 21 now names its cause: the document keeps `v` 1 and `error` `instance-unreachable`, and its `detail`, null before, is one of five closed cause words, on CLI `--json` and on MCP `isError` (the same less `v`); MCP's text stays `refused: instance-unreachable`; no exit, `RefusalReason` or exit code was added.
Exit 1's causes split too: a stale holder reads `stale-heartbeat` in place of `already-live`, `viola revive --json` prints one document for every refusal of its start, the `cli` catch site under `--json` prints the `internal-error` document on stdout, and `verify` writes six refusal codes on its `process-exit`, three of them new words.
Added: `viola_core::UnreachableCause`, `viola_state::liveness::without_endpoint`, `unreachable_document`, `internal_error_document`, `json_asked`, `refused_with`, `Ended::Refused`, the `unwrapped-name` and `endpoint-lost` hints, the `tests/cross_exit_causes.rs` table (39 tests on Linux), and six members of the `process-exit` `detail` enum (13 to 19).
Retired: the exit-21 `detail: null`; `already-live` as a stale holder's refusal; `instance-dead` with `during` `call`; the `revive --json` post-preflight gap; the not-built lines of the exit-cause matrix and the unwrapped-name hint; the `NOT_ERROR_BODIES` row for post-preflight start refusals.
Measured: CI green on `8a4c1a2c` (15 of 15 checks), pre-push 3403 passed; a socket-directory failure at bind has no refusal code and is an anyhow error at the catch site; `tool_inputs_that_pass_reach_the_liveness_check` never reached its check before this chunk, its strict-modes refusal hidden by the shared null body.
The report's Changes name 41 files and the list below holds 40 by its cap: the one left off is `tests/chaos_wait_vanish.rs`, whose `kill` helper moved to `tests/support/start.rs`.

## Reached files
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
- tests/cross_exit_causes.rs
- tests/cli_send.rs
- tests/cli_list.rs
- tests/cli_machine_contract.rs
- tests/cli_wait_last.rs
- tests/cli_controls_not_disableable.rs
- tests/cli_plugin_install.rs
- tests/cli_links.rs
- tests/cli_error_bodies.rs
- tests/cli_instance_state.rs
- tests/cli_revive.rs
- tests/cli_verify.rs
- tests/security_negatives_server_identity.rs
- tests/security_negatives_strict_modes.rs

## Touches
- `.andromeda/security-plan.md:413` (MCP `isError` codes): `instance-unreachable` with `detail: null` is no longer true; `detail` is now the cause word.
- `.andromeda/security-plan-detail/error-handling.md:12`, `:13`, `:22`: the exit-21 body, its null `detail`, and the claim that the unreachable cause adds no error word; the line at `:12` is the one the report names for that claim, and `error` itself stays `instance-unreachable`.
- `.andromeda/security-plan-detail/authentication-authorization.md:27` (null document at offset 561; null detail at 1115, 1596, 1688) and `:36` (null document at 89, 1480; null detail at 2116): each moves to its cause, a strict-modes site to `strict-modes-failed`, a not-running site to its own cause.
- `.andromeda/security-plan-detail/authentication-authorization.md:73`: a search hit of the report's security-plan amendment for the same document, detail, gap or owed-entry wording.
- `.andromeda/security-plan-detail/input-validation.md:37`, `:38`: the `revive --json` gap and its owed-entry wording; the sentence that a refusal of `run`'s start after a passed preflight prints no document is no longer true.
- `.andromeda/security-plan-detail/threat-model-summary.md:99`: a boundary-mcp site the report's amendment names for the same document, gap or owed-entry wording.
- Any text saying work is owed to the exit-cause catalogue entry: the entry is now built, so each such clause is stale.
- The strict-modes refusal as clients report it (IPC access control, client-side verification, the `~/.viola/` rows): a refused check now reports `strict-modes-failed` in `detail`; its order and conditions did not move and it still writes no frame and no line.
- Client-side server verification: a failed check now reports `server-verify-failed` with `during` `connect`; macOS's server check stays owed to Unix endpoint and home hardening.
- `v1-18`'s carve-outs: anything listing what Unix endpoint and home hardening owes gains the socket-directory refusal's code, left out of this chunk by the delegate's PROVISIONAL word.
- `v1-39`: the `revive --json` cap is now met, its ref extended with the table's four start rows and the stale document; any text holding the gap open against it moves.
- `of_reply` and `of_board` in MCP's sanitising rule: `Connect` now reads `instance-dead`, `Closed` and `Io` read `endpoint-lost`, and the board's refusal reads `strict-modes-failed`; JSON-RPC errors and the fixed `-32603` body did not move.
- CLI `Unreachable`: now `Dead`, `Unwrapped`, `Lost`, `ServerUnverified`, with `Dead(&'static str)` gone; any text naming the old variant or the single refusal form moves.
- The catch site: any row stating the output of a failed verb under `--json` takes the `internal-error` document on stdout with stderr empty; the human line stands byte for byte.
- The external-response and NEVER-log rules for CLI `--json` errors and MCP `isError`: reached indirectly through the new `detail` words, which are closed and carry no path, pid or upstream text, so the rules hold but their examples may move.
- The `viola_core::is_code_word` rule and the error-body inventory: every new word passes it and is a row of `tests/contract_error_bodies.rs`; the `NOT_ERROR_BODIES` row for post-preflight refusals is gone.
- Stale holder wording: any text with a stale holder refused `already-live` moves to `stale-heartbeat`; `already-live` stays for a live holder and a fresh replayed beat, and the `run.collision_check` span's `outcome` did not move.
- `instance-dead` with `during` `call`: retired; a wrapper lost with the call open is `endpoint-lost` with `during` `call`, and `instance-dead`, `unwrapped-name` and `server-verify-failed` are `during` `connect`.
- The `process-exit` `detail` enum in `schemas/diag-line.v1.json:78`: 13 to 19 members, adding `stale-heartbeat`, `unwrapped-name`, `endpoint-lost`, `program-not-found`, `version-unreadable`, `unclean-fixture`; any count or list of it moves.
- Child executable and pinned-binary rows: `batch-script-child`, `pinned-hash-mismatch` and `squatted-name` now print a document under `revive --json`, and a program not found prints `internal-error`; all return before `spawn_child` and the human pairs did not change.
- Repository fixtures and verify's recording refusal: `verify` now records `unclean-fixture` on its `process-exit`, with the fixture's `<file>` in no code.
- Session id and recorded-directory rows: `no-session`, `session-live` and `cwd-missing` stand in `revive --json`'s closed code list; the route carry's claim that `session-live` was not built was false.
- Refusal hints: the `unwrapped-name` hint is built and the `endpoint-lost` hint is new; `not-running` stays the hint cause's key, not renamed to the log's `instance-dead`.
- Test inventory: `tests/security_negatives_server_identity.rs` and `tests/security_negatives_strict_modes.rs` now assert the cause-bearing body; `tool_inputs_that_pass_reach_the_liveness_check` now uses a viola-made home and reads `unwrapped-name`; the `Frozen` guard in `tests/cli_instance_state.rs` now exists on Unix too.
- Line citations into any file under Reached files: additions shifted lines, above all in `src/cmd/client.rs`, `src/cmd/run.rs`, `src/cmd/revive.rs`, `src/main.rs`, `crates/viola-mcp/src/result.rs` and `crates/viola-mcp/src/tools.rs`.
- Test-only feature and CI profile statements: `Cargo.toml` gains a `[[test]]` with `required-features` `fake-agent`, and the `ci` kill line in `.config/nextest.toml:24` goes from 24 to 25 `binary(` operands; no master states that count, so a matching 24 is another true claim.
- `human::refuse(` stays 18 sites and `refused_document` keeps its signature and exit-1 callers; a count or rule naming them holds.

## New facts
- F1 · error-handling · Exit 21's `detail` is a closed set of five causes, `strict-modes-failed`, `instance-dead`, `unwrapped-name`, `endpoint-lost` and `server-verify-failed`, the same on CLI `--json` and MCP `isError`, by the founder's own word (`inputs#I2` Q1).
- F2 · error-handling · `viola_core::UnreachableCause` has no serde path, every cause word is written by the two clients, `unreachable_document` is the CLI's one writer of the exit-21 document, and `crates/viola-mcp/src/result.rs` `unreachable` is MCP's.
- F3 · error-handling · Where the `cli` catch site prints its internal-error line, `json_asked` (reading `--json` from argv before a `--`) prints the `internal-error` document with `detail` null on stdout instead, stderr empty; `run`, `revive`'s start, `mcp` and `hook` stay silent there.
- F4 · error-handling · Exit 1 tells a stale holder from a live one: `collision_check` prints and logs `stale-heartbeat` for a stale holder, while a frame verb against a stale instance reads `instance-dead`, stale heartbeat being exit 1's cause alone.
- F5 · error-handling · The `unwrapped-name` and `endpoint-lost` hints come last on stderr in plain ASCII, the only variable being the parsed `ViolaName`; the `endpoint-lost` wording is the delegate's word, PROVISIONAL (`inputs#I2` Q3).
- F6 · error-handling · `tests/cross_exit_causes.rs` is a root `cross_` binary requiring `fake-agent` that holds the exit-21 and exit-1 cause table, with `every_code_is_a_code_word`, `no_two_codes_share_a_hint`, its control `shared_hints_reads_a_planted_duplicate`, and `no_hint_names_release`.
- F7 · authentication-authorization · `without_endpoint` tells a dead instance from an unwrapped name by `symlink_metadata` alone, a directory reading `instance-dead` and anything else, a symlink included, reading `unwrapped-name`; it opens nothing, writes no line and always runs after the strict-modes check, the delegate's word, PROVISIONAL (`inputs#I2` Q2).
- F8 · authentication-authorization · MCP's `frame` maps a refused `check_instance` to `strict-modes-failed`, no `live_endpoint` to the `without_endpoint` reading and a failed `server_verified` to `server-verify-failed`, with no frame and no line on any of them and `isError` the only record (`inputs#I2` Q4).
- F9 · authentication-authorization · A socket-directory failure at bind (`bind_endpoint`'s `create_private_dir`) has no refusal code and is an anyhow error at the catch site; its code is owed to Unix endpoint and home hardening, by the delegate's PROVISIONAL word (`inputs#I3`).
- F10 · input-validation · `viola revive --json`'s closed exit-1 codes are `strict-modes-failed`, `already-live`, `no-session`, `session-live`, `cwd-missing`, `stale-heartbeat`, `batch-script-child`, `pinned-hash-mismatch`, `squatted-name` and `internal-error`, with one document for every refusal of the start it shares with `run`.
- F11 · input-validation · Under `revive --json`, an anyhow error from `trusted`, `checked_exe`, `open_wrapper_log`, `preflight` or `start` prints `internal-error` and returns the same error so the catch site still logs it, while a `pump_child` error prints no document because the child has held the terminal.
- F12 · data-protection · `verify` writes `batch-script-child`, `program-not-found`, `pinned-hash-mismatch`, `version-unreadable`, `strict-modes-failed` or `unclean-fixture` on its `process-exit` under a parsed `VIOLA_NAME`; a refused home tree stays lineless, a failing row is no refusal and no code, the fixture's `<file>` enters no code, and `verify` takes no `--json`.

## Untouched
- Dependency audit: the `deny.toml` bans, advisories and licences, `scripts/rmcp-features.sh`, and the toolchain pins; no dependency was added or bumped.
- CI workflows: `.github/` was not reached, so action pinning, workflow permissions, zizmor, nightly and `windows-mutants.yml` rows stand, and no harness verb, status shape or CI step moved.
- Vendored ConPTY: the pins in `src/conpty.rs` and the gate `scripts/conpty-vendor.sh` were not reached.
- Secret management: no stored secret, the no-repo-scanner position and the obs-owned artifact canary scan and its classes did not move.
- Listener hardening in `viola-channel`: the Windows DACL, the Unix socket chmod, the server-side peer check, the SQOS client open and the channel's JSON-RPC error format and `sanitised_fault`; nothing in `viola-channel` moved.
- Hook fail-open and the statusline shell-out: `src/cmd/hook.rs` is not in the diff, so hook's exit 0, empty stderr and no-body rules stand.
- Plugin files and exec-form paths: the `plugin`, `fixtures` and `scripts` directories were not reached, so what the plugin install writes stands.
- Threat tier, data classification, infrastructure and compliance groups: no asset, principal, surface or regulated-data fact changed.
- Bootstrap phase ordering and the dep-audit, secret-scanning and supply-chain phases: no phase, tool or gate was added or reordered.
