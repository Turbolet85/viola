# Task — design-system · 2026-10-26-server-verification-before-any-frame

## The session
Every frame-writing client now checks the wrapper's pid and start time against the one `snapshot.json` read before its first frame: the CLI verbs `send`, `wait`, `last`, `answer`, `pause` and `release`, the `viola hook` event and dialog paths, and the `viola mcp` frame tools `send`, `wait`, `last` and `answer`.
On a mismatch, or when the check cannot be made, a CLI verb sends no frame and exits 21. Under `--json` it prints the unchanged `{"v":1,"error":"instance-unreachable","detail":null}` document with stderr empty. In human mode it prints the `unable  <name>  instance-unreachable` line, then a new hint cause `server-verify` last on stderr.
The new hint (`src/human/hints.rs`) prints exactly `the process on <name>'s endpoint is not its recorded wrapper; it was not contacted`. That is design-system's cli-send string, byte for byte. Tests assert it is plain ASCII with no SGR and holds no pid, start time, path or endpoint.
The hook on a mismatch still exits 0 with empty streams and fails open, writing `hook-decision` `server-verify-failed`. The MCP tools return `isError` `refused: instance-unreachable`. The CLI writes `process-exit{exit_code:21, detail:"server-verify-failed", during:"connect"}`, the first build to write that detail.
Retired: the server-identity half of the dated gaps on Linux and Windows. macOS stays on the recorded gap: no peer pid there, so the check passes with no comparison and the hint cannot appear on macOS.
Unchanged: exit codes, `--json` codes, `RefusalReason`, MCP error words, config keys, verbs, IPC methods and dependencies.

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

## Touches
- surface cli-send hint row (the row naming server verification as a cause): re-read unmoved. The built string equals it byte for byte, and it is now built and printed. An as-built qualifier is a candidate.
- surface cli-send closed-enum list naming `server-verify-failed`: still true. This build is the first to write it (CLI `process-exit`, hook `hook-decision`). An as-landed qualifier is a candidate.
- surface cli-exit-codes row for exit 21 and its server-verification cause: now built for the six verbs `send`, `wait`, `last`, `answer`, `pause` and `release`. The exit code and the document are unchanged.
- Body Exit-code phraseology, 21 `unable  instance-unreachable` then the cause's hint: the cause set now holds `server-verify` beside the not-running cause. The line itself is unmoved.
- Body Component Pattern 2 (`viola send`), one `hint:` line per reason, detail or cause: gains the `server-verify` cause for `send` (exit 21, the hint last on stderr).
- Body Component Pattern 2, a per-cause `detail` code under `--json` pending (owner: the arch amendment naming them): still pending. This session adds no `--json` code, and the document stays `detail:null` for both causes.
- Body Component Pattern 3 (`viola wait` / `viola last`): both can now refuse exit 21 with the server-verify hint before any result line, and print no `waiting:` result on a mismatch.
- Body Component Pattern 4 (wheel verbs and `answer`): `pause`, `release` and `answer` can now refuse exit 21 with the server-verify hint before their phraseology line.
- Body Streams, refusal then its `hint:` line on stderr and no hint under `--json`: the new cause follows it exactly (asserted for both modes).
- cli bans, NEVER print upstream text, paths, pids or anyhow chains in errors or hints: rule confirmed by the new refusal and hint. A test scans every output for digit runs.
- cli bans, NEVER colour anything except `DIALOG` and `stale`: confirmed for the new hint (plain ASCII, no SGR asserted).
- Platform-Specific Notes, `hook` writes nothing to stderr and always exits 0: re-read unmoved. The three impostor rows assert empty streams and exit 0 in under 1.0 s.
- Platform-Specific Notes, `mcp` writes only MCP frames: re-read unmoved. The mismatch returns the existing `instance-unreachable` `isError` result with no new log line.
- Colour decision order, items 1 and 3 (`--json`, `hook` and `mcp` print no styled bytes): the new paths obey them unmoved.
- Any surface text saying server verification is owed, pending or not yet built for these verbs (Linux and Windows) is now stale. The macOS gap stands.

## New facts
- F1 · surface · As built, `send`, `wait`, `last`, `answer`, `pause` and `release` refuse a server that is not the recorded wrapper (pid or start time differ, or the check cannot be made) with exit 21. They send no frame and print `unable  <name>  instance-unreachable` then the server-verify hint last on stderr, or under `--json` the unchanged `instance-unreachable` document with stderr empty.
- F2 · surface · The server-verify hint is shared by all six verbs: the hint cause `server-verify` beside `not-running`. It is plain ASCII with no SGR, and holds no pid, start time, path or endpoint.
- F3 · surface · On macOS no peer pid is available, so the server check passes with no comparison and the server-verify hint cannot print there (a recorded gap, owned by "Unix endpoint and home hardening").
- F4 · surface · `viola hook` against an impostor server fails open silently (exit 0, empty streams, no frame), and `viola mcp`'s frame tools return `refused: instance-unreachable`. Neither gains a human-visible surface.

## Untouched
- Color Palette (core, surface scale, text hierarchy, semantic and domain status colours, border progression): no colour, token or status variant changed.
- Typography, Spacing, Depth Strategy, Border Radius, Motion and Iconography: 0.2.0 frontend inputs no CLI or code change reaches.
- Brand Identity and the readback box's `[RB]` / `[  ]` / `[/ ]` states: the readback logic, `prompt-submitted` read-back and post-conditions are unchanged. The new refusal precedes any frame.
- `viola list` (cli-list): it writes no frame, and its strict-modes refusal and board columns are unchanged.
- `viola verify` / `viola run` / `viola revive` (cli-verify-run): no line changed. `run`'s collision check does not use the new verification, and its start refusals and hints are unchanged.
- `viola plugin install` lines and its exit-21 strict-modes refusal: unchanged.
- Navigation Pattern and `viola --help` rows (cli-help): no verb added or removed, and `mcp` stays hidden.
- CLI tokens and colour depth selection (cli-tokens): no SGR or depth rule changed.
- Self-Validation Protocol, including the Token Test: no frontend surface.
- Exit codes 0, 1, 2, 10 to 14 and 20: no code or phraseology changed. Only exit 21 gains a cause.
