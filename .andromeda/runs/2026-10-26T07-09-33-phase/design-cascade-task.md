# Task — design · 2026-10-26-server-verification-before-any-frame

## The chunk
Builds server verification before the first frame in every client that writes to a wrapper's endpoint: the connected server's process id and start time must match the `pid` and `started_at` in the instance's `snapshot.json` (macOS: euid + dir only, a recorded gap).
The clients are CLI `viola send`, `wait`, `last`, `answer`, `pause` and `release`, `viola hook`'s `hook.event` (every event) and `hook.dialog` (`pre-tool-use` · `permission-request`), and `viola mcp`'s `send` / `wait` / `last` / `answer` tools (Tokio client); any client in `run`'s start, `revive`, `list` or `verify` is read at P3.
If the identities do not match, or the check cannot be made, the CLI verbs report the instance unreachable (exit 21), the MCP tools return `instance-unreachable`, and `viola hook` fails open (exit 0, nothing on stdout or stderr, no frame written).
Closes the dated liveness-only exceptions (the first and the fourth to the eighth). It keeps today's outcome for an unreadable or newer `snapshot.json` (read as absent: exit 21, or fail open for the hook), and that outcome is held for the founder's word. It re-reads the existing `viola_state::strict` checks without re-building them.

## Reached files
- src/cmd/client.rs · edits the liveness pre-check and its snapshot read
- src/cmd/hook.rs · edits the `hook.event` and `hook.dialog` sends and their two snapshot reads
- src/cmd/hook/statusline.rs · edits its snapshot read
- src/cmd/send.rs
- src/cmd/wait.rs
- src/cmd/last.rs
- src/cmd/answer.rs
- src/cmd/pause.rs
- src/cmd/release.rs
- src/cmd/mcp.rs
- crates/viola-mcp/src/tools.rs · edits the `send` / `wait` / `last` / `answer` tools' pre-check
- crates/viola-state/src/liveness.rs · edits `live_endpoint`
- crates/viola-state/src/snapshot.rs
- crates/viola-state/src/strict.rs · re-reads the existing strict-modes checks
- crates/viola-channel/src/client.rs
- crates/viola-channel/src/tokio_client.rs
- crates/viola-channel/src/endpoint.rs
- viola-0.1.0/verification-matrix.json · edits row v1-41
- .claude/rules/security.md · edits the dated-exception list of §Local trust boundary

## Needs
- The exit-21 refusal form: human stderr `unable  instance-unreachable` followed by the cause's `hint:` line, or under `--json` its one stdout document with stderr empty. This is what a mismatched or unverifiable server now produces for `send`, `wait`, `last`, `answer`, `pause` and `release`.
- The exit-code phraseology table: the typed code 21 and its human and `--json` forms, kept unchanged while a new cause (server identity mismatch, or a check that cannot be made) is added under it.
- A hint for each refusal cause: any hint added for a new unreachable cause is a fixed message, quotes no sent or upstream text, and names no `viola release`.
- The refusal and hint rules shared by every verb: `unable` plus reason and detail, one `hint:` line per cause, no hint under `--json` (the typed fields are the whole answer), and no hint for faults or `unknown`.
- Streams: refusals and their `hint:` lines go to stderr, results to stdout, and under `--json` the whole result is one stdout document with the typed exit code. A pre-frame refusal never mixes into stdout data.
- No path or pid in human output: a mismatch refusal and its hint never print the recorded pid, the server's pid, a start time, the endpoint or socket path, the snapshot path or an anyhow chain.
- Stack traces never print. A mismatch's full detail goes only to `instances/<name>/diagnostics/`.
- `viola hook`'s surface: it never writes stderr and always exits 0, so the fail-open-on-mismatch case prints nothing on either stream, for every hook event and for both `hook.dialog` events.
- `viola mcp`'s surface: stdout carries MCP frames only, so the `instance-unreachable` result is an MCP frame and never a human line.
- The colour decision order: `viola hook` and `viola mcp` are emitted without colour, and a `--json`, piped, `NO_COLOR` or `TERM=dumb` refusal carries no SGR. The `unable` refusal line is never coloured, because only `DIALOG` (amber) and `stale` rows (dim) are coloured.
- ASCII-only human output, and no glyph, emoji, check mark or cross prefix on a new refusal line.
- `viola send`'s readback mirror: a send refused before any frame is written is reported in the exit-21 form and never draws `[RB]` or `[  ] open`. Its exit-13 `[/ ] unable` strike stays the form for `not-delivered`.
- `viola wait` / `viola last`: when the server is refused before the frame, no result line is printed. The static `waiting:` line (stderr, TTY only) prints no spinner or counter in any case.
- `viola pause` / `viola release`: the wheel phraseology lines (`I have control` / `you have control`) print only once a frame has been written. A refused server check prints the exit-21 form instead.
- `viola answer`: `answered  <name>  dialog <n>` prints only after a verified frame. Its existing `not-delivered  unknown-dialog` refusal is unchanged.

## Standing conventions
- S1 · Every new or changed human refusal is one ASCII two-space `unable` line on stderr with no colour, followed by its `hint:` line · body §Surface: cli (Streams, Component Patterns 2) and .andromeda/design-system-detail/surface.md
- S2 · Every `--json` refusal is one stdout document with the typed exit code, stderr empty and no hint · body §Surface: cli (Streams, Exit-code phraseology) and .andromeda/design-system-detail/surface.md
- S3 · No human output, error or hint prints upstream text, a path, a pid or an anyhow chain with a serde source · body §Anti-Patterns (Per-Surface Bans, cli)
- S4 · `viola hook` writes nothing to stderr and exits 0, and `viola mcp` writes only MCP frames on stdout · body §Surface: cli (Platform-Specific Notes)

## Not needed
- The Color Palette, Surface Scale, Text Hierarchy, Semantic and domain status colours, and Border Progression: they are 0.2.0 frontend design inputs, and 0.1.0 renders no page.
- Typography, Spacing, Depth Strategy, Border Radius, Motion and Iconography: they are frontend tokens, and this chunk changes no visual surface.
- The Token Test of the Self-Validation Protocol: it governs a frontend stylesheet, and this chunk has none.
- Brand Identity's expression level and readback-box drawing: no animation or drawn box is added. The CLI mirror's success states are unchanged.
- `viola list`'s strip table (BAY line, columns, unwrapped rows, truncation, escaping, its strict-modes exit-21 board refusal): the board writes no frame to an endpoint and its output is unchanged.
- The CLI colour depth choice (truecolor, 256-colour, 16-colour) and the `DIALOG` / `stale` styling tokens: no styled span is added.
- `viola verify`'s step counter, and `run` / `revive` start refusals and their `--json` documents: they are not frame clients of this check, and their output forms are unchanged.
- `viola plugin install`'s four-line output and its exit-21 strict-modes refusal: plugin install writes no frame.
- The `link` / `unlink` phraseology and the pending `handoff:` row of `viola --help`: "Explicit session links" stays held, and nothing here serves them.
- The `viola --help` verb rows and the flat-verb navigation pattern: no verb is added or renamed.
- The Git Bash rewritten-path warning and the conhost no-VT fallback: neither is touched by a pre-frame check.
