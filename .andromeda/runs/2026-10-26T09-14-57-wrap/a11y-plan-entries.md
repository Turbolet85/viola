## 2026-10-26-server-verification-before-any-frame — the six CLI frame verbs' server-verify refusal
**Section:** §1 A11y Scope Summary (a11y-scope-summary.md: the CLI verbs entity); §8 Cognitive Accessibility › Error recovery (cognitive-accessibility.md: the cognitive-errors pattern, its CLI item)
**Change:**
- The CLI verbs entity now records that `send`, `wait`, `last`, `answer`, `pause` and `release` each refuse, before any frame, an endpoint whose serving process is not the recorded wrapper: exit 21, stdout empty and the server-verify `hint:` line last on stderr (plain ASCII, no SGR), or under `--json` `{"v":1,"error":"instance-unreachable","detail":null}` with stderr empty; on macOS no peer pid is read and the refusal cannot occur (the recorded gap); their `--json` codes and exit codes are unchanged.
- The cognitive-errors CLI item now carries the refusal's text: `unable  <name>  instance-unreachable` on stderr, then `hint: the process on <name>'s endpoint is not its recorded wrapper; it was not contacted` as the last stderr line; under `--json` the same document with stderr empty and no hint.
**Why:** the chunk built the check: each of the six verbs goes through one shared connect that compares the serving process's pid and start time against the snapshot before its first frame, and `send_hint`'s new cause `server-verify` prints that hint, the design-system cli-send string byte for byte; a mismatch or an unmakeable check is exit 21 with the unchanged document. macOS stays on the recorded gap, its euid and socket-directory checks owned by "Unix endpoint and home hardening".
**Ref:** .andromeda/runs/2026-10-26T09-14-57-wrap/

## 2026-10-26-server-verification-before-any-frame — the plugin against an impostor wrapper writes no human-facing byte
**Section:** §1 A11y Scope Summary (a11y-scope-summary.md: a new scope-entities group after the TUI passthrough group)
**Change:**
- New entity detail, the first for the not-assertable plugin entity: the Claude Code plugin against an endpoint whose serving process is not the recorded wrapper — the `viola hook` event and dialog calls and the stdio MCP server's frame tools (`send`, `wait`, `last`, `answer`), off macOS (no peer pid is read there and the check reads as passed: the recorded gap).
- Source: `src/cmd/hook.rs` (`connect`, `NotSent`) and `crates/viola-mcp/src/tools.rs` (`server_verified`); `tests/hook_fail_open.rs` (the impostor rows) and `crates/viola-mcp/tests/handlers.rs` (`handlers_frame_tools_write_no_frame_to_a_server_that_is_not_the_recorded_wrapper`).
- Assertability: not-assertable. A hook event or dialog call that meets such a server sends no frame and fails open: exit 0, empty stdout and stderr, under 1.0 s; an MCP frame tool returns `isError` with the code `instance-unreachable` only (text `refused: instance-unreachable`) and sends no frame. Neither writes a human-facing byte, so no human keystroke is touched.
**Why:** the chunk gave the hook one shared connect, used by the event and the dialog paths, whose failed server check is the `NotSent` detail `server-verify-failed` with no frame, and the MCP `frame` a `server_verified` step that returns the unreachable result with no frame; the server-identity half of the hook and MCP dated gaps is retired on Linux and Windows, measured by the impostor rows of `hook_fail_open` and `binary(handlers)`.
**Ref:** .andromeda/runs/2026-10-26T09-14-57-wrap/

## 2026-10-26-server-verification-before-any-frame — the server-verify refusal's literal pin and its two-leg reach
**Section:** §8 Cognitive Accessibility › Error recovery (cognitive-accessibility.md: the cognitive-errors verification, its cli item and a new cli, macOS item; a11y-plan.md: the Error recovery Verification line)
**Change:**
- The cli verification item now names the six frame verbs' exit-21 server-verify refusal as pinned by the `HINT` literal in `tests/security_negatives_server_identity.rs`, whose `security_negative_cli_verb_writes_no_frame_to_a_server_that_is_not_the_recorded_wrapper` reads, over the six verbs in both modes, the hint as the last stderr line with no ESC byte and stdout empty, or under `--json` the exact document with stderr empty, and no output holding the recorded pid, the home's path or the endpoint, beside its control `security_negative_cli_verb_reaches_the_recorded_wrapper`.
- Its OS reach: was «These run on all three OS legs»; now all three legs except `tests/security_negatives_server_identity.rs`, gated `not(target_os = "macos")`, which runs on windows-2025 and ubuntu-latest only. The body's Verification line now adds the same exception (macOS reports no peer pid).
- New cli, macOS item: the refusal cannot occur there (no peer pid; the check reads as passed, the recorded gap), so its cases compile out; its macOS witness is owed to "Unix endpoint and home hardening".
**Why:** the refusal landed in a new security-negatives binary rather than a trycmd case, gated off macOS, green on CI ubuntu-latest and windows-2025. The macOS server-verification cases joined "Unix endpoint and home hardening" as one CARRY on P5 review card 1's word — delegate the Viola overseer, founder-delegated.
**Ref:** .andromeda/runs/2026-10-26T09-14-57-wrap/

## 2026-10-26-server-verification-before-any-frame — the server-verify hint among the literal-pinned strings
**Section:** §8 Cognitive Accessibility › Plain language (cognitive-accessibility.md: the cognitive-plain-language verification; a11y-plan.md: the Plain language Verification line)
**Change:**
- The literal-pinned exceptions to trycmd now add the server-verify hint (`send_hint`'s cause `server-verify`, `src/human/hints.rs`), pinned by the `HINT` literal in `tests/security_negatives_server_identity.rs`.
- The body: was «by literal» for `viola list`, `viola --help` and `viola plugin install`; now for those three and the server-verify hint.
**Why:** the chunk added a fixed hint string and pinned it by literal, adding no trycmd case, so the claim that trycmd pins every fixed CLI string outside the named exceptions needed the new exception named.
**Ref:** .andromeda/runs/2026-10-26T09-14-57-wrap/
