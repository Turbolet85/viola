# design · Surface: {type} — category read

## Carried
- (.andromeda/design-system-detail/surface.md:62) The squatted-name start refusal the chunk keeps must print exactly this fixed-message line and this hint, unchanged.
  > `unable: the endpoint for <name> is held by another process` (squatted name) → `hint: another process holds this name's endpoint; stop it or pick another name`
- (.andromeda/design-system-detail/surface.md:59) Every `viola run` exit-1 start refusal the chunk touches keeps one fixed-message line and one hint, and none shows a path or a pid (the pipe name included).
  > The exit-1 start refusals of `viola run` have one fixed-message line and one hint per cause (T4). None of them shows a path or a pid:
- (.andromeda/design-system-detail/surface.md:21) The refusal goes to stderr in the fixed `unable: <text>` form, its `hint:` line directly after it.
  > only the exit-1 refusals of `viola run`, `viola revive` and `viola verify`, and `viola list`'s exit-21 strict-modes refusal (chunk 2026-10-11-the-board-viola-list, the founder's own answer, 2026-10-13, relayed verbatim by the Viola overseer; the operator's answer superseded 2026-10-13), use the fixed message form, `unable: <text>`, as in `unable: <name> is already live`), each refusal's `hint:` line directly after it, and errors (`error: …`).
- (.andromeda/design-system-detail/surface.md:102) The exit-1 row of the exit-code table lists the squatted-name refusal's stderr text; the chunk's refusal must still match it (exit 1, no code change).
  > or a `viola run` start refusal `unable: <name> is already live` / `… is still running but not answering` / `the endpoint for <name> is held by another process` / `the pinned viola copy failed its integrity check` / `<name>'s command is a .cmd or .bat script` / `the viola home is not private to you`
- (.andromeda/design-system-detail/surface.md:102) A `run` start refusal prints no `--json` document; the chunk adds none (owed to another route entry).
  > every other exit 1 has no `--json` document (a `run` start refusal reached through `revive` included, owed to "Exit-cause code catalogue")
- (.andromeda/design-system-detail/surface.md:73) Under `--json` no hint prints, and the squatted-name cause's log detail code stays `squatted-name`.
  > Hints are human-mode only. Under `--json`, no hint text is printed; the machine view carries the cause as a detail code, never as prose.
- (.andromeda/design-system-detail/surface.md:73) The named log detail code for the squat the chunk asserts.
  > obs-plan D-20 already names the log detail codes: `instance-dead`, `strict-modes-failed`, `server-verify-failed`, `already-live`, `squatted-name`, `pinned-hash-mismatch`, `batch-script-child`,
- (.andromeda/design-system.md:332) `run` prints a start refusal as its cause's fixed line then its hint, never a path or a pid.
  > A start refusal (exit 1) prints its cause's fixed-message line, for example `unable: builder is already live`, then that cause's hint (`hint: viola list`). The per-cause lines are listed under the send hints above. None of them shows a path or a pid.
- (.andromeda/design-system.md:261) The refusal's stream is stderr, in the fixed form for `run`'s exit-1 refusals.
  > - stderr: context lines, refusals (two-space `unable  <name>  <reason>  <detail>`, or the fixed `unable: <text>` form for the exit-1 refusals of `run`, `revive` and `verify` and `list`'s exit-21 refusal), each refusal's `hint:` line directly after it, and errors.
- (.andromeda/design-system.md:377) The refusal line carries no colour, glyph or cursor control.
  > **NEVER emit colour, glyphs or cursor control under `--json`, non-TTY, `NO_COLOR`, `TERM=dumb` or `viola run`.**
- (.andromeda/design-system.md:380) No pipe name, path, pid or error chain reaches the refusal or its hint.
  > **NEVER print upstream text, paths, pids or anyhow chains with serde sources in errors or hints.**
- (.andromeda/design-system.md:354) A listener or admission failure prints a fixed message, never a stack trace; detail goes to diagnostics.
  > **Stack traces** never print. Errors are fixed messages, and full detail goes only to `instances/<name>/diagnostics/`.
- (.andromeda/design-system.md:338) Exit 1 keeps its human form; no `--json` document is added for a `run` refusal.
  > 1 `error: internal error` or a fixed-message `unable:` refusal with its hint (a `--json` document only for `viola revive`'s strict-modes and preflight refusals)
- (.andromeda/design-system.md:270) Human output stays ASCII only.
  > - Human output is ASCII only (`->`, `[RB]`, `[/ ]`, `[  ]`).

## Bindings
- **squatted-name refusal text ↔ security-plan §Authentication & Authorization (Windows pipe first-instance)**: the first-pipe-instance failure the chunk asserts surfaces only as this fixed line and hint, exit 1 (.andromeda/design-system-detail/surface.md:62)
- **squatted-name detail code ↔ obs-plan D-20**: the log line of the squat carries the detail code `squatted-name`, never prose or a path (.andromeda/design-system-detail/surface.md:73)
- **start-refusal stderr strings ↔ test-plan (`tests/path1_run_start.rs`, `tests/cli_instance_state.rs`)**: an assertion on the squat refusal matches the fixed line and hint verbatim, with no path or pid (.andromeda/design-system-detail/surface.md:59)
- **no path in errors ↔ security-plan §Error Handling**: the pipe name is a path-like value and reaches no refusal, hint or error (.andromeda/design-system.md:380)
