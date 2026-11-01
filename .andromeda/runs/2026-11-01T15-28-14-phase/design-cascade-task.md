# Task — design · 2026-11-01-unix-endpoint-and-home-hardening

## The chunk
Verifies the per-user Unix socket directory (owner, mode, not a symlink) before a wrapper binds and before a client connects, refusing a directory that fails and never narrowing it in place; the socket stays 0600 by chmod after the bind.
Checks the peer euid on both sides of the local socket: the wrapper against a connecting client, every Unix client against the server before its first frame.
Applies Unix strict modes on the paths it reaches, makes the home check of `viola list`'s `board::scan` one rule, and records the macOS pid gap as a named carve-out where the server check is a match with no comparison.
Gives each new refusal (socket directory at a wrapper's start and at a client's connect, peer euid mismatch on either side) a closed code and a hint in the exit-cause catalogue's form, with a row in the catalogue's test; the exit each one takes is read at P3.
Re-derives the `holder_gone` connect-probe measurement and whether a leftover socket file's takeover holds beside the new checks, and measures the hypothesis that the harness `cleanup` reads `endpoint_gone:false` after a Unix force-kill.

## Reached files
- tests/cross_exit_causes.rs · edits a row for each new refusal
- board.rs · edits `board::scan`'s home check
- tests/support/home.rs · re-reads `holder_gone`, the root kill path's connect probe
- crates/viola-e2e/src/harness/cleanup.rs · re-reads its Unix `endpoint_gone` rule, the socket path gone

## Needs
- Exit-code phraseology for exit 21: the `unable  instance-unreachable` human form, the cause's hint, and the `--json` document with its `detail`, for a socket-directory refusal or a peer euid mismatch read at P3 as exit 21.
- The closed set of exit-21 causes (`strict-modes-failed`, `instance-dead`, `unwrapped-name`, `endpoint-lost`, `server-verify-failed`) and what each one's word and hint say, since a new refusal either reads as one of them or adds a closed word beside them.
- The `server-verify-failed` cause and its hint: the existing refusal beside the client-side peer euid check, and the wording that has to stay true where macOS's verdict is a match with no comparison.
- The macOS exit-cause case: the per-user socket directory loosened to 0755 reaching exit 21, and how the CLI reads it.
- The fixed home-state and instance-state `unable:` / `hint:` pairs of a refused strict-modes check, which of the two names an instance, and the verbs that print each.
- `viola list`'s refusal of the whole board on a failed strict-modes check, in its human and `--json` forms: making the home check one rule decides which pair a 0750 home reads, with or without an instance held.
- `run`'s start refusals: the fixed-message `unable:` line and one hint per cause at exit 1, and the per-cause lines listed under the send hints, for a socket directory refused at a wrapper's start if P3 reads it as a `run` refusal.
- `revive`'s start shared with `run`: its fixed-order hints and the closed `--json` code list of that start, since the same bind and the same directory check stand behind it.
- `run` prints nothing once the child starts, and `revive` the same: a wrapper refusing a connecting client on euid does so while the child holds the terminal.
- `hook` writes no stderr and always exits 0, so a refusal of this chunk reaches it only as a fail-open.
- `mcp` writes only MCP frames on stdout and stays silent at the catch site, and it is a Unix client under the euid and directory checks.
- The catch site's `error: internal error` form and its `--json` `internal-error` document at exit 1: where the `create_private_dir` failure lands today, the form the new closed code replaces.
- The strict-modes refusal before any frame on `link`, `unlink`, `send`, `wait`, `last`, `answer`, `pause` and `release`: no frame and no line written, the shape the client's check before its first frame sits beside.
- `verify`'s and `plugin install`'s strict-modes refusals (`verify` refused on its home tree prints its fixed pair and exits 1; a `stamps.json` update and `plugin install` print the home-state pair), for wherever Unix strict modes reach those paths.
- The exit-21 causes a client reads against a killed wrapper's leftover socket file (`instance-dead`, `endpoint-lost`), beside the leftover file's takeover the chunk re-derives.
- The typed exit-code set as the phraseology lists it (0, 1, 2, 10 to 14, 20, 21): no new code joins it.

## Standing conventions
- S1 · each new refusal line goes to stderr with its hint line directly after it, results staying on stdout · Surface: cli › Streams (.andromeda/design-system-detail/surface.md)
- S2 · under `--json` each new refusal is one stdout document with the typed exit code, no hint line and stderr empty · Surface: cli › Streams (.andromeda/design-system-detail/surface.md)
- S3 · each new line is plain ASCII with no colour, glyph or cursor control · Anti-Patterns › Per-Surface Bans › cli
- S4 · each new refusal or hint prints no path, pid, upstream text or anyhow chain · Anti-Patterns › Per-Surface Bans › cli
- S5 · a new fault prints no stack trace, its full detail going only to the instance's diagnostics · Surface: cli › Platform-Specific Notes
- S6 · each new word is short fixed phraseology, `unable` plus a typed reason, with no filler and no guessing · Brand Identity › Domain anchors
- S7 · each new hint is one line per reason, detail or cause, names no `viola release`, quotes no upstream text, and a fault or `unknown` takes none · Surface: cli › Component Patterns, pattern 2 (.andromeda/design-system-detail/surface.md)

## Not needed
- Core colours, surface scale, text hierarchy and border progression in color-palette.md: 0.2.0 frontend inputs; 0.1.0 renders no page.
- The domain status rows and semantic colours in color-palette.md: visual treatments for a frontend that is deferred to 0.2.0.
- Typography's faces and per-OS fallback stacks: their Linux and macOS rows concern a future page's fonts, not the Linux and macOS parity of the end-to-end suite.
- Spacing, depth, border radius and web glyphs: geometry and glyphs of a page that 0.1.0 does not render.
- Motion and the expression level's duration scale: the chunk's refusals are static lines and the CLI has no motion.
- The Self-Validation Token Test: it traces frontend tokens, and 0.1.0 has none.
- The Surface: web-spa section: deferred to 0.2.0, its technology not chosen.
- The CLI colour tokens at truecolor, 256-colour and 16-colour depth, and how a depth is chosen: nothing this chunk prints is coloured.
- `viola list`'s healthy output (the six columns, the BAY line's figures, row order, unwrapped truncation): the chunk changes the board's home check, not its table.
- `viola send`'s readback states and result lines (`[RB]`, `[  ]`, `[/ ]`, `unconfirmable`) and their `/clear` and `/compact` post-conditions: the chunk changes no send result.
- `viola wait` and `viola last` result lines and the message-mode escaper: the chunk changes no message output.
- The success lines of the wheel, link and gate verbs, and `plugin install`'s four stdout lines: the chunk touches only the refusals before their frames.
- `verify`'s step lines and its `stamped` summary: no ledger row or stamp changes here.
- The `viola --help` verb groups: the chunk adds no verb.
- The Windows notes (Git Bash's rewritten-path warning, the conhost and VT fallback): no Windows behaviour moves in this chunk.
