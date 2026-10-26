# Task — a11y · 2026-10-26-server-verification-before-any-frame

## The chunk
Builds server verification before the first frame in every client that writes to a wrapper's endpoint: the connected server's pid and start time must match the `pid` and `started_at` of the instance's `snapshot.json` (Unix `peer_creds`, Windows the pipe's server process id, start time from `sysinfo`; macOS euid + dir only, a recorded gap).
The clients: CLI `viola send`, `wait`, `last`, `answer`, `pause`, `release`; `viola hook`'s `hook.event` (every event) and `hook.dialog` (`pre-tool-use` · `permission-request`); `viola mcp`'s `send` / `wait` / `last` / `answer` tools through the Tokio client.
On a mismatch or a check that cannot be made: CLI verbs report the instance unreachable (exit 21, today's liveness pre-check code), MCP tools return `instance-unreachable`, `viola hook` fails open (exit 0, nothing on stdout or stderr, no frame).
Closes the dated exceptions one, four, five, six, seven and eight (removed from security.md at the wrap); an unreadable or newer `snapshot.json` keeps today's outcome (read as absent: exit 21 / fail open) behind a held founder fork; the five snapshot readers and the Windows exiting-wrapper pipe window are re-read at P3.

## Reached files
- src/cmd/client.rs · edits the snapshot read and the liveness-only pre-check before the frame
- src/cmd/send.rs
- src/cmd/wait.rs
- src/cmd/last.rs
- src/cmd/answer.rs
- src/cmd/pause.rs
- src/cmd/release.rs
- src/cmd/hook.rs · edits its two snapshot reads before the `hook.event` and `hook.dialog` frames
- src/cmd/hook/statusline.rs · edits its snapshot read
- src/cmd/run.rs · edits its collision check's snapshot read
- src/cmd/mcp.rs
- crates/viola-mcp/src/tools.rs · edits the `send` / `wait` / `last` / `answer` tools' pre-check
- crates/viola-channel/src/client.rs · edits the sync client's connect before the first frame
- crates/viola-channel/src/tokio_client.rs · edits the Tokio client's connect before the first frame
- crates/viola-channel/src/endpoint.rs
- crates/viola-state/src/liveness.rs · edits `live_endpoint`
- crates/viola-state/src/snapshot.rs · edits `read_snapshot` and the classified read's callers
- crates/viola-state/src/strict.rs
- viola-0.1.0/verification-matrix.json · edits row v1-41
- .claude/rules/security.md · edits the dated-exception list (at the wrap)

## Needs
- cli surface: the refusal a CLI verb prints on a failed server verification — exit 21, instance unreachable — in human mode and `--json` mode
- cli refusal layout: the human-mode unreachable refusal is `unable  <reason>  <detail>` on stderr with `hint:` as the last stderr line and a typed exit code, the same as today's liveness refusal
- cli `--json` mode: a failed verification gives one document on stdout with no `hint:` line and no SGR
- kebab-case reason/detail codes (the unreachable code) always sit beside the plain word `unable`; no stack trace or bare status number is the only error text
- `unable` is never coloured and no SGR, glyph or cursor control is written under non-TTY, `NO_COLOR`, `TERM=dumb` or `--json` on the new refusal path
- C0/C1 control characters escaped in any CLI refusal text; no path, pid, upstream text or start time in the refusal or the `--json` document
- the fixed CLI copy pinned by trycmd: an unchanged unreachable refusal string stays pinned; a new or changed refusal or hint string updates its pin on purpose
- P2 confirmed send or refusal: `viola send` refused on a mismatched server reads as a refusal in text with a typed exit, never as a delivered `[RB]` readback
- P3 dialog then `answer`: `viola answer` refused on a mismatched server is a text refusal with a typed exit, distinct from the `unverified-cli` refusal
- P4 human takes the wheel, cli half: `pause` / `release` refused on a mismatched server keep the `unable … hint:` layout and typed exit; a refusal here goes to automation only and never blocks the human
- P6 wait/last output: `viola wait` / `viola last` refused on a mismatched server keep linear static output and the refusal reason in text
- the human always wins through the hook client: `viola hook` on a mismatch or an unmakeable check exits 0, writes nothing to stdout or stderr and nothing to the terminal, so no human keystroke in `viola run` or a passed `viola revive` is blocked or delayed by it
- viola writes zero terminal bytes of its own inside the TUI passthrough: the hook's verification failure and its diagnostics go to `diagnostics/`, never the terminal
- every time bound the verification holds (connect, peer lookup, `sysinfo` read) falls on the driver or the hook, never on the human; no key is blocked, refused or delayed by it
- MCP `isError` results carry codes only (`instance-unreachable`), no hint and no text beyond `refused: <code>`: the plugin/MCP surface is not-assertable but the code is the driver-facing error identification
- the outer-PTY tui boundary assertion that a `viola hook` failing open emits no viola-originated byte, if the chunk adds a hook fail-open case run under `viola run`
- an unreadable or newer `snapshot.json` keeps today's outcome (exit 21 refusal with its hint / hook fail open): the refusal text and layout must not change under the held fork
- no WCAG conformance claim rests on the new refusal assertions: they are output-discipline stand-ins for SC 3.3.1 / 3.3.3

## Standing conventions
- S1 · every new CLI output-discipline or refusal test runs inside `nextest-integration` / `nextest-e2e` (CI: the per-OS `coverage` suite) under `scripts/agent-run.* run`, on all three OS legs, adding no `suite` value and no separate a11y driver · §1 A11y Scope Summary (CI integration) / §9 CI Integration
- S2 · a new refusal path has its `unable <reason> <detail>` + final `hint:` layout pinned by trycmd and its typed exit code and hint-free `--json` asserted by assert_cmd · .andromeda/a11y-plan-detail/cognitive-accessibility.md
- S3 · no new fixed CLI string goes unpinned; a copy change fails CI until its pin is updated · .andromeda/a11y-plan-detail/cognitive-accessibility.md
- S4 · no SGR on `unable`, and none under non-TTY / `NO_COLOR` / `TERM=dumb` / `--json` / `viola run` / a passed `viola revive` in any new output · .andromeda/a11y-plan-detail/visual-design-verification.md
- S5 · a new test is zero-flake: `retries: 0`, no `#[ignore]`, a flake keeps the chunk red; the per-OS `gate --require coverage,doctest` admits no skip · §10 SLO Invariants & A11y Budgets
- S6 · no assertion is written up as WCAG conformance evidence for the cli or tui · §11 A11y Anti-Patterns › Strategy

## Not needed
- `viola list` board captions, the `CAPTION` literal and its order, the `-- UNWRAPPED - READ-ONLY --` separator, bold NAME / amber `DIALOG` / dim stale-row SGR and NAME truncation: the chunk changes no board output (the board's read model is not among the clients)
- the `--help` group labels and their owed `handoff:` row: owed to "Explicit session links", held and not served here
- `viola verify` step lines `[NN/NN] … pass|fail` and its interrupted `unable:` pair: verify writes no frame to a wrapper's endpoint
- `viola revive --json`'s owed document for a refusal after a passed preflight: owed to "Exit-cause code catalogue"
- `viola plugin install` output and its pinned literals: no frame client
- P4's tui outer-PTY wheel cases (keystrokes delivered past the atomic paste; focus, mouse and resize sequences not moving the wheel; the Windows win32-input-mode mouse fact F-W3): the wrapper's pump and wheel are unchanged
- the `waiting: <name>` stderr line of `viola wait`: printed after a passed pre-check, unchanged
- the dim-on-stale-rows outer-PTY witness: owed to "Linux and macOS parity"
- manual NVDA / VoiceOver / Orca and keyboard passes: founder-owned supplements, no new step
- the fixed empty-board copy `no wrapped sessions — start one with  viola run <name> -- claude`, `unknown` and `n/a`: list copy, untouched
- motion and spinners, multi-language, target size, cognitive AAA criteria: not triggered, no surface here
- the deferred 0.2.0 frontend's landmarks, keyboard order and a11y tooling: no page in 0.1.0
