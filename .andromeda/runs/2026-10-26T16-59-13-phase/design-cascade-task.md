# Task — design · 2026-10-26-home-and-code-bearing-file-integrity

## The chunk
Makes the home owner-only at every entry point and ends the dated strict-modes exceptions: the strict-modes check (Unix mode/owner, Windows owner + DACL) now runs before any read or open of the home by `send`, `wait`, `last`, `answer`, `pause`, `release`, `viola hook`'s nine events (failing open), `viola mcp`'s frame tools, `verify`'s stamps read and `run`'s start, and before `client::start`'s role-file open.
Adds the Windows DACL check before any diagnostics or role file is opened, with `check_instance`'s first Windows proof and the seeded-test-home question (`seed_conpty`) answered.
Hardens the code-bearing files: `bin/<version>-<hash>/viola` re-hashed before reuse and its first copy made race-free (`pin_exe`), the `conpty/` companions joined to the Windows strict-modes set, `plugin/` rewritten atomically on every start with absolute pinned `command`s, and the `marketplace/` tree's integrity.
Re-reads that only the wrapper writes `snapshot.json` and only `viola verify` writes `ledger/stamps.json`, and measures whether two clean release builds of one tree differ by file and by section.
Whether the snapshot readers move onto the log replay is the founder's P4 fork; no boundary is widened without his live word.

## Reached files
- crates/viola-state/src/strict.rs · edits `check_instance` (its Windows owner + DACL proof)
- crates/viola-state/src/fs.rs · re-reads `create_private_dir` and `replace_private` (0700 / 0600, the Windows DACL at creation)
- crates/viola-state/src/liveness.rs · edits `live_endpoint` (the strict-modes check before its snapshot read)
- crates/viola-state/src/snapshot.rs · re-reads `read_snapshot` (an unreadable or newer snapshot reads as absent)
- crates/viola-state/src/pin.rs · edits `pin_exe` (the home copy of the binary and its re-hash before reuse)
- crates/viola-state/src/stamps.rs · edits the stamps read `update_stamps` takes
- src/cmd/verify.rs · edits `update_stamps` (the stamps read)
- src/cmd/client.rs · edits `start` (the check before the role-file open)
- src/obs.rs · edits `open_role_file` (the check before the open)
- src/cmd/hook.rs · edits the hook client's snapshot read through `VIOLA_DIR`
- src/cmd/hook/statusline.rs
- src/cmd/run.rs · edits `start` (the home, `instances/<name>/`, `bin/`, `plugin/`, `ui/` and trusted-file checks; the collision check; the per-start plugin rewrite)
- src/conpty.rs · edits the `conpty/` folder and both files joining the Windows strict-modes set
- src/cmd/send.rs
- src/cmd/wait.rs
- src/cmd/last.rs
- src/cmd/answer.rs
- src/cmd/pause.rs
- src/cmd/release.rs
- src/cmd/mcp.rs
- crates/viola-mcp/src/tools.rs
- src/cmd/plugin.rs · edits the `marketplace/` tree's integrity (an unknown entry, the installed `.mcp.json` `command`)
- src/cmd/revive.rs · re-reads its `check_instance` before the log opens
- tests/cli_controls_not_disableable.rs · edits its 0770 `--home` negative (exit 21) and the table's completeness case
- tests/support/home.rs · edits `seed_conpty` (viola creates the home first)
- .claude/rules/security.md
- .andromeda/security-plan.md

## Needs
- The exit-21 refusal of a CLI frame verb (`send`, `wait`, `last`, `answer`, `pause`, `release`): the two-space `unable  instance-unreachable` line on stderr, then that cause's `hint:` line; under `--json` the one `instance-unreachable` document on stdout with stderr empty and no hint — the form a newly added strict-modes refusal of these verbs prints.
- The fixed `unable:` / `hint:` strict-modes pair that `viola list`, `viola link` / `viola unlink` and `viola plugin install` print on a refused home or instance check (exit 21), and its `--json` `instance-unreachable` document — the form a frame verb's or `plugin install`'s new strict-modes refusal matches or departs from.
- The exit-code phraseology table: which typed code and which human and `--json` form a strict-modes refusal takes per verb (21 for the frame verbs, `list`, `link`, `unlink` and `plugin install`; 1 with a fixed-message `unable:` line for `run`, `revive` and `verify`).
- `viola run`'s start refusal: one fixed-message `unable:` line plus that cause's hint (exit 1), showing no path or pid — the form a refused check of the home, `instances/<name>/`, `bin/`, `plugin/`, `ui/`, a trusted file, a `conpty/` companion or a failed pinned-binary re-hash takes; and `run` prints nothing once the child starts.
- `viola revive`'s refusal forms: the human `unable:` / `hint:` pair and the `--json` `strict-modes-failed` document, stderr empty, exit 1 — reached through the `check_instance` this chunk changes on Windows.
- `viola verify`'s output: the step-counter lines and summary, and a refusal as its fixed `unable:` / `hint:` pair with exit 1 — the form a refused strict-modes check of `ledger/stamps.json` in `update_stamps` takes.
- `viola hook` writes nothing to stderr and always exits 0: the strict-modes check added before its snapshot read fails open with no human output.
- `viola mcp` writes only MCP frames on stdout: its frame tools' refused strict-modes check surfaces as the `instance-unreachable` result, never a human line.
- `viola list`'s whole-board refusal (exit 21, the fixed `unable:` / `hint:` form) on a failed strict-modes scan, reached through the `check_instance` the Windows DACL proof changes.
- `viola plugin install`'s refused strict-modes check (exit 21): the home-state `unable:` / `hint:` pair or the `--json` `instance-unreachable` document — the surface the `marketplace/` integrity work reaches.
- No refusal or hint names a path, pid, start time or endpoint: a refusal over the home, an instance dir, a `bin/` copy, a `conpty/` file or a `marketplace/` entry prints none of them, nor an anyhow chain.
- Full detail goes only to `instances/<name>/diagnostics/`, and stack traces never print: a refusal made before the diagnostics file may be opened (the DACL check comes first) prints its fixed message only.
- One `hint:` line per cause, directly after its refusal; no hint names `viola release`; a hint never quotes upstream text.
- `viola wait`'s static `waiting:` line and `viola send`'s `[  ] open  ...  issued` stderr line: where they stand relative to a strict-modes refusal that now precedes the frame.
- The colour decision order for refusal output: none under `--json`, `viola run`, `viola hook`, `viola mcp`, `NO_COLOR`, `TERM=dumb`, a non-terminal stdout or a failed Windows VT enable.

## Standing conventions
- S1 · every new human output line is ASCII only and carries no colour beyond `DIALOG` (amber) and a `stale` row (dim) · surface.md (cli-tokens, cli-output)
- S2 · results on stdout; refusals, their `hint:` lines and errors on stderr; under `--json` one document on stdout and no hint · body §Surface: cli (Streams) and Anti-Patterns › Per-Surface Bans › cli
- S3 · every refusal is `unable` plus a typed reason in the short fixed phraseology vocabulary, never "done" or "success" without context · body §Brand Identity (Standard phraseology) and brand-identity.md

## Not needed
- The colour palette, surface scale, text hierarchy, border progression and contrast table: 0.2.0 frontend inputs; this chunk adds no styled span.
- The domain status colours and their guards (semantic colours): no strip state or event kind is added or redrawn.
- Typography, spacing, depth, border radius, motion and the expression level: 0.1.0 renders no page and the chunk adds no animation.
- Iconography and the CLI glyph rules (`->`, ` - `): no new glyph or link column.
- The web-spa surface and the Token, Swap, Squint, Signature and Sameness tests: the frontend is deferred to 0.2.0.
- The `viola list` table's columns, BAY line, row order, unwrapped-name truncation and control-character escaping: the chunk changes only whether the board is refused, not how it is drawn.
- The send readback mirror's `[RB]` / `unconfirmable` states, the `/clear` and `/compact` post-conditions and the not-delivered reasons (exit 13): the readback path is untouched.
- The `wait` / `last` result line forms, the message-mode escaper and the `link` / `unlink`, `pause` / `release`, `answer` success lines.
- `viola --help`'s verb rows, the flat verb list and the Git Bash rewritten-path warning: no verb is added, renamed or hidden.
- `viola plugin install`'s four-line success output: only its refusal and its tree's integrity are reached.
