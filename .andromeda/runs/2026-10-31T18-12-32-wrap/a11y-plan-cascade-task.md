# Task — a11y-plan · 2026-10-31-exit-cause-code-catalogue

## The session
Exit 21's `--json` document moved from one form with a null `detail` to one form per cause: `detail` is one of five closed words, `error` stays `instance-unreachable`, `v` stays 1, no hint is added, and no exit moved and no exit code was added; MCP's `isError` body reads the same less `v`.
Exit 1 gained `stale-heartbeat` for a stale holder, `viola revive --json` now prints one document for every refusal of the start it shares with `run`, and the `cli` catch site prints the `internal-error` document under `--json`.
The `unwrapped-name` hint was built and an `endpoint-lost` hint was added; `viola verify`'s refusals gained `process-exit` log codes, and the `process-exit` `detail` enum moved from 13 to 19 members.
Added `tests/cross_exit_causes.rs`, the exit-cause table, 39 tests on Linux, and the `ci` kill line moved from 24 to 25 `binary(` operands; CI on the final HEAD read green, 15 of 15 checks.
Retired: the exit-21 document's `detail` null, `already-live` as a stale holder's refusal, `instance-dead` with `during` `call`, the `revive --json` post-preflight gap, the not-built lines of the exit-cause matrix and the unwrapped-name hint, and the post-preflight row of `NOT_ERROR_BODIES`.
No dependency, crate, harness verb, CI step or config key moved; the report's Changes count 41 files and the list below holds 40 by its cap, leaving off `tests/cli_controls_not_disableable.rs`, one added line.

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
- The Owed line under the CLI verbs entity (`a11y-plan.md:31`, its detail in `a11y-scope-summary.md:8`): a refusal of `run`'s start under `viola revive --json` printing no document is no longer true, so the line and its owed-to title Exit-cause code catalogue are spent.
- `a11y-scope-summary.md:8` also states the exit-21 document in two places (report offsets c1856 and c2264): each takes the per-cause form.
- `a11y-scope-summary.md:25` and `:80`: the report's search sites for this chunk (the null exit-21 document, the revive gap, the owed-to title, the catch site); each is re-read against the facts below.
- `cognitive-accessibility.md:19`, `:25`, `:31`, `:49` and `:92`: the same search's sites under Error recovery and the literal-pinned list; each is re-read against the facts below.
- Any statement that exit 21 has one document, a null `detail`, or adds no error word of its own, in the Error recovery groups.
- The Error recovery pattern's sentence that `--json` gives one document with no hint: it stands, and it now also covers each of the five exit-21 causes, every `revive --json` start refusal and the catch site's `internal-error`.
- The rule that `viola revive`'s exit-1 refusals print one `unable:` line: it stands for the human form, while under `--json` every one of them is a document.
- The rule that faults are not refusals and carry no hint: it stands, while a fault under `--json` on a CLI verb now prints a document on stdout with stderr empty, in place of the `error: internal error` line.
- Any code list for `viola revive --json`: it is now `strict-modes-failed`, `already-live`, `no-session`, `session-live`, `cwd-missing`, `stale-heartbeat`, `batch-script-child`, `pinned-hash-mismatch`, `squatted-name`, `internal-error`.
- Any text naming a stale holder's refusal as `already-live`: that code now means a live holder or a fresh replayed beat, and a stale holder reads `stale-heartbeat`.
- Any text naming a refusal hint as pending or not built, or saying a cause has no code of its own: the unwrapped-name hint and the five causes' words are now built.
- The literal-pinned list in Plain language verification (the server-verify hint among its members): the two hints the session added or built reach it.
- The Error recovery verification claim of which tools pin each refusal's layout and exit code on all three OS legs: the exit-cause table now also witnesses them.
- The frame verbs' server-verify exception (windows-2025 and ubuntu-latest only, macOS reporting no peer pid): the exception stands, and the refusal now has the word `server-verify-failed`, with macOS's server check a named carve-out of `v1-18`.
- The kebab-case rule that a code sits beside the plain word `unable`, and the plain-language examples of codes: the session adds new code words, each passing `is_code_word` and each a row of `tests/contract_error_bodies.rs`.
- Critical paths P2, P3 and P4 and P6's `wait` and `last`: the no-endpoint arms of `send`, `answer`, `pause`, `wait` and `last` read their cause from the instance directory, and `release` reaches the same arm through `wheel_call`, so an exit-21 refusal on those paths now names its cause.
- Critical path P6's board (`viola list`): its refusal document now reads `strict-modes-failed`.
- `viola link`, `viola unlink` and `viola plugin install` as CLI entities: their exit-21 forms take the per-cause document, `unlink` through `link_call`, `plugin install` as `strict-modes-failed`.
- The `viola verify` entity, through its new log codes: the codes reach `process-exit` lines only, `verify` takes no `--json`, and the report's coverage row marks them a11y n/a.
- The not-assertable MCP half of the plugin entity, through the moved `isError` body: the report marks it a11y n/a, so its no-UI-surface reason should still hold.
- The boundary sentence that a passed `viola revive` writes nothing while its child runs, through `src/cmd/revive.rs`: the report re-asserts it as met, and every new start document returns before `spawn_child`.
- The statement that CLI output discipline runs inside `nextest-integration` and `nextest-e2e`, through the new root `cross_` binary: the E2E regex takes it, and the `ci` kill line names it.
- Any count or inventory of a11y-relevant test binaries or cases, through the new `tests/cross_exit_causes.rs` and the added cases in `tests/cli_wait_last.rs`, `tests/cli_instance_state.rs` and `tests/cli_verify.rs`.

## New facts
- F1 · cognitive-accessibility · Under `--json` exit 21 prints one document per cause: `error` is `instance-unreachable` and `detail` is one of the closed words `strict-modes-failed`, `instance-dead`, `unwrapped-name`, `endpoint-lost`, `server-verify-failed`, with stderr empty and no ESC.
- F2 · cognitive-accessibility · The human forms of `unwrapped-name` and `endpoint-lost` each end with their own `hint:` as the last stderr line, in plain ASCII, the `endpoint-lost` hint being new text and the name in a hint the parsed `ViolaName`.
- F3 · cognitive-accessibility · A name's no-endpoint refusal reads the instance directory: a directory is `instance-dead`, anything else (absent, a regular file, a symlink) is `unwrapped-name`, and the strict-modes check stays first.
- F4 · cognitive-accessibility · A wrapper lost with the call open is `endpoint-lost`, and a frame verb against a stale instance reads `instance-dead`, stale heartbeat being exit 1's cause alone.
- F5 · cognitive-accessibility · Exit 1 tells a stale holder from a live one: `stale-heartbeat` for a `stale` holder, `already-live` for a live holder and a fresh replayed beat.
- F6 · cognitive-accessibility · Where a CLI verb prints `error: internal error`, under `--json` it prints the `internal-error` document with a null `detail` on stdout in its place and stderr is empty; without `--json` the line is unchanged, and `run`, `revive`'s start, `mcp` and `hook` stay silent there.
- F7 · a11y-scope-summary · Under `viola revive --json` every refusal of the start it shares with `run` prints one exit-1 document before the child spawns, including `batch-script-child`, `pinned-hash-mismatch` and `squatted-name`; the human pairs are unchanged byte for byte.
- F8 · a11y-scope-summary · A `claude` not found at a start reads as the `internal-error` document under `viola revive --json`, and it has no human pair, none being added.
- F9 · a11y-scope-summary · A `pump_child` error under `viola revive --json` prints no document, because the child has held the terminal and viola writes nothing past it.
- F10 · cognitive-accessibility · No two codes share a hint and no hint names `release`, witnessed by `no_two_codes_share_a_hint`, its control `shared_hints_reads_a_planted_duplicate`, and `no_hint_names_release` in `tests/cross_exit_causes.rs`.
- F11 · cognitive-accessibility · `tests/cross_exit_causes.rs` is the exit-cause table: its rows check each cause's exit, document and hint in human and `--json` modes as a `fake-agent` E2E binary, and its top comment names the causes held outside the table.
- F12 · cognitive-accessibility · The `unwrapped-name` and `endpoint-lost` hints join the literal-pinned strings, through `send_hint_is_the_design_string` and the table.

## Untouched
- The frontend deferral to 0.2.0 and its unchosen technology: the session has no frontend surface.
- The Standard tier and the no-WCAG-conformance claim: the session adds output-discipline witnesses, which stay stand-ins and support no conformance claim.
- The triggers marked not triggered (wcag-mapping, regulated-compliance, cognitive-accessibility, motion-sensitive, multi-language, target-size): they rest on the brief and the security excerpt, which the session did not change.
- The manual screen-reader and keyboard passes (NVDA, VoiceOver, Orca), founder-owned and supplemental: the report records no manual pass and moves none.
- CI Integration's platform, OS matrix, `gate --require coverage,doctest`, suite enum and no-a11y-artifact rule: no harness verb, status shape or CI step moved, and nothing changed in `.github` or `scripts`.
- The not-assertable hook half of the plugin entity and the wrapper IPC channel: `src/cmd/hook.rs` is not in the diff and nothing changed in `viola-channel`.
- The no-readability-scorer decision (D-A11Y-16) and the no-spinner ban: neither rests on an error code or an exit document.
