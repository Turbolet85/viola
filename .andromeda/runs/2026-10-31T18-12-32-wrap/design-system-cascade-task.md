# Task — design-system · 2026-10-31-exit-cause-code-catalogue

## The session
Every exit-21 cause now has its own code: the `--json` document's `detail` and MCP's `isError` body carry one of five closed causes in place of null; no exit, no `RefusalReason` and no exit code moved, and `error` stays `instance-unreachable`.
Exit 1's causes were told apart too: a stale holder has its own code `stale-heartbeat`, `revive --json` prints one document for every refusal of the start it shares with `run`, and the `cli` catch site prints an `internal-error` document under `--json`.
The `unwrapped-name` hint is built and an `endpoint-lost` hint is new; `verify` writes its refusal's code on its `process-exit`; `schemas/diag-line.v1.json`'s `process-exit` `detail` enum went from 13 to 19 members; a new table, `tests/cross_exit_causes.rs`, witnesses the causes across the surfaces.
Retired: the exit-21 document's `detail: null`, `already-live` as a stale holder's refusal, `instance-dead` with `during` `call`, the `revive --json` post-preflight gap, the not-built lines of the exit-cause matrix and of the unwrapped-name hint, and the matching `NOT_ERROR_BODIES` row.
Measured: CI green on final HEAD `8a4c1a2c`, checks 15/15; an earlier read on `115cdbb9` was red on a macOS compile error in `crates/viola-mcp/tests/handlers.rs`, fixed by that commit.
The change holds 41 files and `## Reached files` caps at 40, so it leaves out one, `tests/chaos_wait_vanish.rs`, the test file furthest from this doc.

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
- tests/cross_exit_causes.rs
- tests/support/start.rs
- tests/contract_error_bodies.rs
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
- The hint rows at `surface.md:64` and `:65` (cli-send): their wording is unmoved, but the unwrapped-name hint is now built and the pending status at `:65` moves.
- `surface.md:83` (cli-send): its causes and its stale holder's code said no code or not built; both now exist.
- `surface.md:82` (cli-send): its `revive` clause read no-change in the report's sweep; `surface.md:84` (cli-send) is listed for the same amendment.
- `surface.md:135` (cli-verify-run), and `:144`, `:152` and `:168` (cli-exit-codes), are the report's remaining sites of this amendment; check each against the `--json` documents below.
- `surface.md:144` holds two readings: the catch site's human line stands byte for byte, and its owner clause naming the entry Exit-cause code catalogue changes.
- The send paragraph in the body's pattern 2, where a per-cause `detail` code under `--json` is pending with the arch amendment as owner: that code now exists.
- The `revive` paragraph at `design-system.md:334`: its sentence that a refusal `run`'s start makes after a passed preflight prints no `--json` document is no longer true, and its owner, the working-route entry Exit-cause code catalogue, is spent.
- Exit-code row 1 at `design-system.md:338`: it limits exit 1's `--json` document to `revive`'s strict-modes and preflight refusals, and that limit no longer holds, through the new start documents and the catch site's document.
- Exit-code row 21 at `design-system.md:338`: the document is now one per cause, and the human form's cause hint gains the built unwrapped-name hint and the new endpoint-lost hint.
- The `viola plugin install` strict-modes paragraph: its `instance-unreachable` document now carries `strict-modes-failed`; the home-state pair is unmoved.
- The `link`, `unlink`, `send`, `wait`, `last`, `answer`, `pause` and `release` strict-modes paragraph: the same document change; the instance-state pair, no frame and no line are unmoved.
- The As-built paragraph of `viola list`: its exit-21 strict-modes refusal is the board's only refusal and now reads `strict-modes-failed` under `--json`; the human pair is unmoved.
- The Streams line on `--json`, through two moves: more refusals now reach a document, and some paths stay without one (a `pump_child` error, and the catch site's silence on `run`, `revive`'s start, `mcp` and `hook`).
- The start-refusal bullet of `run` and its sample `unable: builder is already live`: the human pairs are byte for byte as they stood, but `already-live` now means a live holder or a fresh replayed beat, never a stale holder.
- The statement that `revive` prints nothing once the child starts, and the cli ban calling a passed `revive` passthrough: both re-read true, now also under `--json`, through the `pump_child` rule.
- The rule of one `hint:` line per reason, detail or cause, and the rule that no hint names `viola release`: unmoved, now witnessed by `no_two_codes_share_a_hint` and `no_hint_names_release` in `tests/cross_exit_causes.rs`.
- Any hint row keyed by cause: the keys are `not-running`, `unwrapped-name`, `endpoint-lost` and `server-verify`; `not-running` keeps its key and was not renamed to the log's `instance-dead`.
- The rule that no hint prints under `--json`, and that faults and `unknown` get no hint: unmoved; every new document holds no hint, and the `internal-error` document is a fault.
- The cli ban on printing upstream text, paths or pids in errors or hints, and the Streams rule of stdout results: unmoved, and the new words are closed codes holding no path, pid or upstream text.
- The `viola verify` paragraph: its human output is unmoved, a refused home tree stays lineless and `verify` takes no `--json`; only its log gains refusal codes, through `Ended::Refused`.
- The `mcp` platform note and the Callers line: the note holds, while the MCP `isError` body now names the cause; MCP's text stays `refused: instance-unreachable`, with no frame and no line on those refusals.
- Any line claiming every cause now has a code: two carve-outs stay unmet, the socket-directory cause and macOS's server check, both with the route entry Unix endpoint and home hardening.
- Any other pending or owner line naming the entry Exit-cause code catalogue: that entry is what this session delivered.

## New facts
- F1 · surface · The exit-21 `--json` document carries its cause in `detail`, one of five closed words: `strict-modes-failed`, `instance-dead`, `unwrapped-name`, `endpoint-lost`, `server-verify-failed`; `v` stays 1 and `error` stays `instance-unreachable`.
- F2 · surface · A refused strict-modes check at exit 21 reads `strict-modes-failed` in that document on the eight frame verbs, `list` and `plugin install`.
- F3 · surface · With no live endpoint a frame verb reads its instance directory after the strict-modes check: a directory is `instance-dead`; absent, a regular file or a symlink is `unwrapped-name`.
- F4 · surface · A failed connect is `instance-dead`, a wrapper lost with the call open is `endpoint-lost`, and a failed server verification is `server-verify-failed`.
- F5 · surface · `endpoint-lost` has its own new hint, last on stderr and plain ASCII; its wording is the delegate's word and stands provisional.
- F6 · surface · A start against a stale holder prints and logs `stale-heartbeat`; a live holder and a fresh replayed beat stay `already-live`.
- F7 · surface · A frame verb against a stale instance reads `instance-dead`: stale heartbeat is exit 1's cause alone.
- F8 · surface · `revive --json` prints one exit-1 document for every refusal of the start it shares with `run`, its closed codes `strict-modes-failed`, `already-live`, `no-session`, `session-live`, `cwd-missing`, `stale-heartbeat`, `batch-script-child`, `pinned-hash-mismatch`, `squatted-name`, `internal-error`; `run` itself prints none of these documents.
- F9 · surface · Under `revive --json` a program not found prints `internal-error`, a cause with no human pair and none added; an anyhow error of its start prints the same document and still reaches the catch site's log.
- F10 · surface · A `pump_child` error under `revive --json` prints no document: the child has held the terminal and viola writes nothing past it.
- F11 · surface · Under `--json` the `cli` catch site prints the `internal-error` document with a null `detail` on stdout in place of `error: internal error`, stderr then empty; without `--json` the line stands byte for byte.
- F12 · surface · The catch site counts `--json` only among the arguments before a `--`, read from argv.
- F13 · surface · `run`, `revive`'s start, `mcp` and `hook` stay silent at the catch site, under `--json` too.

## Untouched
- Brand Identity: the session moved no anchor, the signature element or the expression level.
- Color Palette: its values and the domain status treatments are a 0.2.0 frontend design input; the session added no colour, and its new cause words get no visual in 0.1.0.
- Typography, Spacing, Border Radius, Depth Strategy, Motion and Iconography: frontend inputs or platform-controlled values, none moved.
- Surface: web-spa stays deferred to 0.2.0.
- The cli colour tokens, the colour decision order and the depth choice: the styling module is not in the change, and `--json` stays first in the order.
- The `hook` platform note: the change holds no `src/cmd/hook.rs`, and `hook` still writes nothing to stderr and exits 0.
- The Universal Bans and Rejected Defaults: their rules govern hue, spinners, icons and ranking, none of which the session reached.
- The Swap, Token and Sameness tests of the Self-Validation Protocol: their rules read frontend craft, not exit causes.
