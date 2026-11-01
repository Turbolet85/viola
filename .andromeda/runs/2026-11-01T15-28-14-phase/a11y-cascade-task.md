# Task — a11y · 2026-11-01-unix-endpoint-and-home-hardening

## The chunk
Verifies the per-user Unix socket directory (owner, mode, not a symlink) before a wrapper binds and before a client connects, refusing a failing directory; the socket stays 0600 by chmod after the bind.
Checks the peer euid on both sides of the local socket: the wrapper against a connecting client, every Unix client against the server before its first frame.
Gives each new refusal (socket directory at a wrapper's start and at a client's connect, peer euid mismatch on either side) a closed code and a hint in the exit-cause catalogue's form, with a row in the catalogue's test; which exit each one is stays P3's read.
Makes the board's home check in `viola list` one rule, applies Unix strict modes on the paths the entry reaches, and records the macOS peer pid gap as a named carve-out with its 0755 exit-cause case.
Re-derives at HEAD the kill-path measurement (`holder_gone`) and whether a leftover socket's takeover holds beside the new checks, and measures the harness `cleanup` hypothesis on a force-killed wrapper.

## Reached files
- tests/cross_exit_causes.rs · edits a row per new refusal: the socket directory refusal, the peer euid mismatch
- src/cmd/run.rs · edits `bind_endpoint`
- crates/viola-state/src/board.rs · edits `board::scan`'s home check
- crates/viola-state/src/strict.rs
- crates/viola-channel/src/client.rs
- tests/support/home.rs · re-reads `holder_gone`, the root kill path's connect probe
- crates/viola-e2e/src/harness/cleanup.rs · re-reads its Unix rule, `endpoint_gone` after a force-kill

## Needs
- The CLI refusal surface at a wrapper's start: a refused socket directory reads as `unable <reason> <detail>` on stderr, the hint as the last stderr line, with a typed exit code.
- The CLI refusal surface at a client's connect: a refused socket directory and a peer euid mismatch take that same form on every Unix client verb, and the client-side check fires before the first frame.
- The move from fault to refusal: `bind_endpoint`'s `create_private_dir` failure is an anyhow error at the catch site today; as a refusal it carries a hint, its text is no stack trace, and the hint-free exception of `unknown` (exit 14) does not fit it.
- Plain-language rule over the new codes: each kebab-case reason or detail code sits beside the word `unable`, and each hint is a recovery instruction the reader can act on.
- Exit codes on the refusal surface: the typed set stays 0, 1, 2, 10–14, 20, 21, and no refusal has a bare status number as its only text.
- `--json` under the new refusals: one document with no hint, and the carve-out of `viola answer` and `viola send` exit-2 paths (a fixed stderr line, no document), for whichever exit P3 reads.
- `viola revive` through the start it shares with `run`: a refused socket directory reaches it there, where a refusal reads as one `unable: <text>` line and, under `--json`, one exit-1 document.
- `viola list` under the one-rule home check: the homes it refuses change, and the refusal, home or `ScanRefused::Instance`, reads in the board's linear static output in human, plain and `--json` modes.
- Control characters in a refusal's detail: a detail carrying a socket directory, home path or instance name still escapes C0/C1 except `\n` and `\t`.
- SGR on the new refusals: `unable` is never coloured, and no SGR of viola's own appears under non-TTY, `NO_COLOR`, `TERM=dumb`, `--json`, `viola run` or a passed `viola revive`.
- The TUI boundary inside a live wrapper: a peer euid refusal of a connecting client writes no byte to the outer terminal, and its diagnostic goes to `diagnostics/`.
- The human keystroke path: the directory and peer checks on the wrapper's connection path never block, refuse or delay a key past the current atomic paste, and a security refusal never blocks the human.
- `hook` and the plugin's stdio MCP server as clients of the checked socket: a not-assertable surface with no output-discipline assertion, `hook` keeping exit 0, no body, no stderr, so a refusal reaches it only as a fail-open.
- The macOS carve-out: macOS reports no peer pid, so the server check there is a match with no comparison and the server-verify refusal's verification holds on windows-2025 and ubuntu-latest only; no weaker comparison and no test reads it as one.
- The macOS exit-cause case of test-plan §6: the socket directory loosened to 0755 gives exit 21, a refusal case on the macOS leg beside the plan's per-leg verification.
- The Windows leg under Unix-only refusals: the new refusals and their cases do not exist on windows-2025, so the plan's all-three-legs verification reads as Unix legs for them, and every Windows reading stays as it stands.
- The existing connect refusals beside the new ones: the server-verify and `endpoint-lost` hints stay as their literals pin them, and each new refusal stays a separate closed code on that same connect path, including a client reaching a killed wrapper's leftover socket file.

## Standing conventions
- S1 · A new CLI or boundary case runs inside `nextest-integration` / `nextest-e2e`, in CI inside the per-OS `test` job's `coverage` suite, with no new `suite` value and no second driver · body §9 CI Integration and §11 CI
- S2 · A new case is held by `gate --require coverage,doctest` on each OS leg and never skips · body §9 CI Integration and §10
- S3 · Zero flakiness for a new case: `retries: 0`, no retry-once, `#[ignore]` banned · body §10 and §11 SLO
- S4 · A new fixed CLI string is pinned by trycmd or by literal, so a copy change fails CI until the pin moves on purpose · .andromeda/a11y-plan-detail/cognitive-accessibility.md
- S5 · A new output case uses `assert_cmd`, `trycmd` or the portable-pty outer PTY, and never parses the `claude` screen or takes a screenshot · body §1 harness specification and §11 Universal
- S6 · Results of these cases are output-discipline stand-ins, and no WCAG conformance claim rests on them · body §11 Strategy

## Not needed
- The frontend's page, its browser a11y tooling and its later a11y inputs: the frontend stays deferred to 0.2.0 and the chunk renders no page.
- The triggers that did not fire (wcag-mapping, regulated-compliance, cognitive-accessibility, motion-sensitive, multi-language, target-size): they ask nothing of a socket check or its refusal.
- The manual NVDA, VoiceOver and Orca passes on P4 and P6: a founder-owned supplement this scope neither adds nor re-runs.
- What moves the wheel (focus, mouse and resize sequences, `pause`, `release`, `human-typing`): the chunk changes no wheel logic.
- Windows platform readings (the injected mouse report of F-W3, the byte reading owed to "Windows-only live measurements"): no Windows behaviour moves.
- The stale-row dim outer-PTY witness: it is owed to "Linux and macOS parity", another entry.
- The time-bound inventory of viola's waits, holds and windows: the scope adds and moves no time bound.
- P2's `[RB]` readback mirror and P3's `unverified-cli` withholding: the chunk changes no readback or answer logic, only the connect those verbs stand on.
