# Task — a11y-plan · 2026-10-26-server-verification-before-any-frame

## The session
Every frame-writing client now checks the wrapper's pid and start time against the recorded snapshot before its first frame: the CLI verbs `send`, `wait`, `last`, `answer`, `pause` and `release`, the `viola hook` event and dialog paths, and the four `viola mcp` frame tools.
On a mismatch, or when the check cannot be made, a CLI verb sends no frame and exits 21: under `--json` it prints the unchanged document `{"v":1,"error":"instance-unreachable","detail":null}` with stderr empty; in human mode it prints `unable  <name>  instance-unreachable` with a new hint as the last stderr line.
The new hint cause `server-verify` in `src/human/hints.rs` prints exactly `the process on <name>'s endpoint is not its recorded wrapper; it was not contacted`, which matches design-system surface.md cli-send byte for byte. It is plain ASCII, carries no SGR (asserted), and holds no pid, start time, path or endpoint.
A hook that fails the check fails open: exit 0, empty streams, under 1.0 s, no frame, and a `hook-decision` line with `server-verify-failed`. An MCP tool that fails it returns `isError` with `instance-unreachable` and sends no frame.
macOS keeps the recorded gap: there is no peer pid, so the check reads as passed. The new negative tests compile out on macOS, and the macOS 0755-directory case is re-pinned to "Unix endpoint and home hardening".
No exit code, `--json` code, refusal reason, config key, env var, SGR rule, `suite` value or CI gate was added. The server-identity half of the dated client gaps is retired on Linux and Windows. Their strict-modes half stands.

## Reached files
- tests/security_negatives_server_identity.rs
- crates/viola-channel/src/client.rs
- crates/viola-channel/src/tokio_client.rs
- crates/viola-channel/src/lib.rs
- crates/viola-state/src/liveness.rs
- src/cmd/client.rs
- src/human/hints.rs
- src/cmd/send.rs
- src/cmd/wait.rs
- src/cmd/last.rs
- src/cmd/answer.rs
- src/cmd/pause.rs
- src/cmd/release.rs
- src/cmd/hook.rs
- crates/viola-mcp/src/tools.rs
- crates/viola-mcp/tests/handlers.rs
- tests/hook_fail_open.rs
- tests/contract_fake_agent_drift.rs
- viola-0.1.0/verification-matrix.json
- viola-0.1.0/working-route.md

## Touches
- Error recovery pattern (`unable  <reason>  <detail>` on stderr, then `hint:` as the last stderr line, with a typed exit code; `--json` one document with no hint): six CLI verbs gain a new refusal cause that follows this pattern at exit 21, so any listing of hinted refusals or causes is now short one entry.
- Error recovery verification claim (trycmd pins the layout per refusal; assert_cmd pins the typed exit codes and a hint-free `--json`): the new server-verify refusal is pinned by a literal `HINT` constant and by assert_cmd-style assertions in the integration binary `tests/security_negatives_server_identity.rs`, not by a trycmd case, and it runs on two OS legs (Linux, Windows), not three.
- Plain language, every fixed CLI string pinned so a copy change fails CI: the new hint string is a fixed CLI string, and its pin is that test's literal.
- Plain language, a kebab-case code always beside the word `unable`: the new refusal keeps `instance-unreachable` beside `unable`, and the hint gives the plain-word explanation.
- Anti-pattern, never show an error without a recovery suggestion (every human-mode refusal except `unknown` ends with `hint:`): the new refusal ends with its hint, which holds the rule.
- Path P2, confirmed send and readback or refusal (cli `viola send`): `send` gains the server-verify refusal (exit 21, hint last on stderr, `--json` document unchanged).
- Path P3, dialog pending then `answer` (cli `viola answer`): `answer` gains the same refusal beside the `unverified-cli` refusal.
- Path P4, human takes the wheel (cli `pause`, `release`): both verbs gain the same refusal before any frame. The tui keystroke boundary is unchanged.
- Path P6, CLI board, wait/last and verify (cli `wait`, `last`): both verbs gain the same refusal. The new stderr text is linear, plain ASCII and free of SGR, which supports the SC 1.3.2, 1.4.1 and 3.3.1 stand-ins with no conformance claim.
- Scope entity list, CLI verbs in human TTY, plain and `--json` modes: `send`, `wait`, `last`, `answer`, `pause` and `release` gain a refusal outcome in human and `--json` modes. Their `--json` code and exit code sets are unchanged.
- Visual SGR rule (no SGR under non-TTY, `NO_COLOR`, `TERM=dumb` or `--json`; `unable` never coloured): the new hint and `unable` line write no SGR, and this is asserted.
- Not-assertable entity, the plugin hooks and stdio MCP server: the hook's server check fails open (exit 0, empty streams, no stderr) and MCP returns codes only, so the entity stays not-assertable and the human-wins guarantee holds.
- Timeout extensions, the list of time bounds viola holds: no new time bound. The hook check runs inside the existing SPINE_DEADLINE (hook perf rows max 2.2–2.4 ms against a 1 s bound) and falls on no human key.
- CI integration, cases run inside the per-OS `test` job's `coverage` suite on all three OS legs: the new output-discipline case runs on windows-2025 and ubuntu-latest only, because its binary is gated `not(target_os = "macos")`. No suite value or gate changed.
- Owed items naming route entries: the macOS server-verification cases (the per-user socket directory loosened to 0755, then exit 21) now sit on "Unix endpoint and home hardening". Any a11y text that places a macOS refusal case under "Server verification before any frame" is stale.

## New facts
- F1 · cognitive-accessibility · CLI `send`, `wait`, `last`, `answer`, `pause` and `release` refuse an endpoint whose serving process is not the recorded wrapper with exit 21: `unable  <name>  instance-unreachable` on stderr, then the hint `the process on <name>'s endpoint is not its recorded wrapper; it was not contacted` as the last stderr line; under `--json` the unchanged `{"v":1,"error":"instance-unreachable","detail":null}` with stderr empty and no hint.
- F2 · cognitive-accessibility · The server-verify hint is pinned by a literal in `tests/security_negatives_server_identity.rs` (12 mismatch rows covering six verbs in two modes, plus a control per verb), which asserts it is the last stderr line, plain ASCII with no SGR, and free of any pid, start time, path or endpoint digits.
- F3 · cognitive-accessibility · On macOS the server-verify refusal cannot occur (no peer pid; the check reads as passed, the recorded gap), and its output-discipline case compiles out there. Its macOS witness is owed to "Unix endpoint and home hardening".
- F4 · a11y-scope-summary · A `viola hook` event or dialog call that meets an impostor wrapper fails open: exit 0, empty stdout and stderr, under 1.0 s, no frame. An MCP frame tool that meets one returns `isError` with code `instance-unreachable` only. Neither writes any human-facing byte.

## Untouched
- The `viola run` tui passthrough boundary (zero viola bytes, keystrokes never blocked or delayed, focus, mouse and resize sequences, F-W3): the session changes no PTY, wheel or keystroke path.
- The `viola list` board: the caption literal and order, the `-- UNWRAPPED - READ-ONLY --` separator, the amber `DIALOG`, dim stale rows, bold NAME and NAME truncation. `list` sends no frame and is not among the changed verbs.
- The `--help` group labels and the owed `handoff:` row: unchanged.
- `viola revive` refusals and the owed `revive --json` post-preflight document, and `viola verify` step lines: not reached.
- `viola plugin install` output and its literal pins: not reached.
- The `unknown` (exit 14) no-hint exception: unchanged. The new refusal is hinted.
- The tier, the compliance cross-check, the not-triggered triggers (wcag-mapping, regulated-compliance, cognitive, motion, multi-language, target-size) and the no-WCAG-conformance stance: unchanged.
- The manual screen-reader passes (NVDA, VoiceOver, Orca on P4 and P6) and their founder ownership: unchanged.
- The SLO invariants, retries-0 rule, `gate --require coverage,doctest` and the no-new-suite rule: no harness, script or workflow file changed.
- The test tool versions (assert_cmd 2.2.2, predicates 3.1.4, trycmd 1.2.1, portable-pty `=0.8.1`): no dependency was added or bumped.
