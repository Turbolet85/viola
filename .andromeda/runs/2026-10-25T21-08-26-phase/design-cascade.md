# design · cascade read

## Carried
- (.andromeda/design-system-detail/brand-identity.md:21) `viola run`'s output stays at expression 0.0: the squatted-name start refusal that `refuse_squatted` re-asserts prints no colour, no glyph and no cursor control, and nothing prints once the child starts.
  > | cli, `--json` / non-TTY / `NO_COLOR` / `TERM=dumb` / `viola run` passthrough | 0.0 | No colour, no glyphs, no cursor control, no spinner. `viola run` prints nothing at all while the wrapped `claude` TUI runs, and neither does a `viola revive` that passed its preflight. |
  (.andromeda/design-system.md:246)
  > 2. `viola run` (while the child runs, nothing is printed at all)
- (.andromeda/design-system-detail/surface.md:21) The squatted-name refusal goes to stderr in the fixed `unable: <text>` form, with its `hint:` line directly after it, as every `viola run` exit-1 refusal does.
  > - stderr: context lines (`waiting: <name>`, the issue line of a send), refusals (`unable  <name>  <reason>  <detail>`, two-space separated; `viola send` prefixes the `[/ ]` mirror and pads `unable` to the readback-word column, as in its sample; the exit-code table omits the `<name>` field; only the exit-1 refusals of `viola run`, `viola revive` and `viola verify`, and `viola list`'s exit-21 strict-modes refusal (chunk 2026-10-11-the-board-viola-list, the founder's own answer, 2026-10-13, relayed verbatim by the Viola overseer; the operator's answer superseded 2026-10-13), use the fixed message form, `unable: <text>`, as in `unable: <name> is already live`), each refusal's `hint:` line directly after it, and errors (`error: …`).
  (.andromeda/design-system.md:261)
  > - stderr: context lines, refusals (two-space `unable  <name>  <reason>  <detail>`, or the fixed `unable: <text>` form for the exit-1 refusals of `run`, `revive` and `verify` and `list`'s exit-21 refusal), each refusal's `hint:` line directly after it, and errors.
  (.andromeda/design-system.md:247)
  > 3. `viola hook` (never writes stderr) and `viola mcp` (stdout carries MCP frames only)
  (.andromeda/design-system.md:353)
  > - **`hook`** writes nothing to stderr and always exits 0. **`mcp`** writes only MCP frames on stdout. Neither has a human design surface.
  (.andromeda/design-system.md:354)
  > - **Stack traces** never print. Errors are fixed messages, and full detail goes only to `instances/<name>/diagnostics/`.
- (.andromeda/design-system-detail/surface.md:59) The squatted-name start refusal keeps exactly this fixed-message line and hint, showing no path, pipe name or pid, and its log detail code stays `squatted-name`.
  > - The exit-1 start refusals of `viola run` have one fixed-message line and one hint per cause (T4). None of them shows a path or a pid:
  >     - `unable: <name> is already live` → `hint: viola list`
  >     - `unable: <name> is still running but not answering` (stale heartbeat, live pid) → `hint: viola list shows it as stale; stop that process before starting <name> again`
  >     - `unable: the endpoint for <name> is held by another process` (squatted name) → `hint: another process holds this name's endpoint; stop it or pick another name`
  (.andromeda/design-system-detail/surface.md:73)
  > obs-plan D-20 already names the log detail codes: `instance-dead`, `strict-modes-failed`, `server-verify-failed`, `already-live`, `squatted-name`, `pinned-hash-mismatch`, `batch-script-child`,
  (.andromeda/design-system.md:332)
  > - **`run`:** prints nothing once the child starts. A start refusal (exit 1) prints its cause's fixed-message line, for example `unable: builder is already live`, then that cause's hint (`hint: viola list`). The per-cause lines are listed under the send hints above. None of them shows a path or a pid.
- (.andromeda/design-system-detail/surface.md:93) The chunk adds no `--json` document for the squatted-name refusal, including one reached through `viola revive`'s start: that document stays owed to "Exit-cause code catalogue".
  > a refusal `run`'s start makes after a passed preflight prints no document as landed (owed to the working-route entry "Exit-cause code catalogue").
  (.andromeda/design-system.md:334)
  > A refusal `run`'s start makes after a passed preflight prints no `--json` document: pending, owner: the working-route entry "Exit-cause code catalogue".
- (.andromeda/design-system-detail/surface.md:102) The squatted-name refusal stays an exit-1 fixed-message refusal with its hint and no `--json` document, never moved to another typed code.
  > | 1 | `error: internal error` (fixed message; no chain with serde sources, no paths, no hint), or a `viola run` start refusal `unable: <name> is already live` / `… is still running but not answering` / `the endpoint for <name> is held by another process` / `the pinned viola copy failed its integrity check` / `<name>'s command is a .cmd or .bat script` / `the viola home is not private to you`,
  (.andromeda/design-system.md:338)
  > 1 `error: internal error` or a fixed-message `unable:` refusal with its hint (a `--json` document only for `viola revive`'s strict-modes and preflight refusals)
- (.andromeda/design-system-detail/iconography.md:10) The squatted start refusal and the budget refusal print ASCII only, with the refusal's reason and detail kept as separate two-space fields.
  > - **Separator.** Middle dot `·` on the web. The CLI is ASCII only:
  >   - Two spaces separate columns, and a refusal's reason from its detail (they are separate fields, as in the exit-code table).
  (.andromeda/design-system.md:270)
  > - Human output is ASCII only (`->`, `[RB]`, `[/ ]`, `[  ]`).
- (.andromeda/design-system-detail/color-palette.md:99) The re-greened budget case's refusal carries the reason word `budget-paused` unchanged; only its log capture changes.
  > | RefusalReason::budget-paused | #1E2124 | #E6D8AE | #E6D8AE | `unable · budget-paused` |
  (.andromeda/design-system.md:21)
  > Short fixed words (`open`, `RB`, `unable`, `human-typing`, `budget-paused`, `not-delivered`) are the vocabulary of every label, CLI column and refusal.
- (.andromeda/design-system-detail/color-palette.md:89) The five-hour window's refusal detail word is `five-hour`, which the re-greened case asserts as the refusal's window word.
  > | BudgetWindow::five-hour | #1E2124 | #4A5057 | #E6D8AE | Label `5H`. Detail word `five-hour` in refusals. |
  (.andromeda/design-system-detail/color-palette.md:105)
  > | RefusalDetail::five-hour | #1E2124 | #E6D8AE | #E6D8AE | `budget-paused · five-hour` |
- (.andromeda/design-system-detail/color-palette.md:90) The seven-day window's refusal detail word is `seven-day`, which the re-greened case asserts as the refusal's window word.
  > | BudgetWindow::seven-day | #1E2124 | #4A5057 | #E6D8AE | Label `7D`. Detail word `seven-day` in refusals. |
  (.andromeda/design-system-detail/color-palette.md:106)
  > | RefusalDetail::seven-day | #1E2124 | #E6D8AE | #E6D8AE | `budget-paused · seven-day` |
- (.andromeda/design-system-detail/color-palette.md:96) The `send-refused` line the re-greened case's log capture must find carries the refusal's reason and detail, the pair a refused readback prints after `unable`.
  > | Readback::refused (from CL-1 `send-refused`) | #1E2124 + 1px #E6D8AE `/` strike | #E6D8AE | #E6D8AE | Strike, then `unable`, then `reason · detail` |
- (.andromeda/design-system-detail/surface.md:105) The budget-paused send refusal stays exit 11 with its reason and window detail and a refusal object under `--json`; the re-greened case's assertion of the window word reads this refusal.
  > | 11 | `unable  budget-paused  five-hour` / `seven-day` | refusal object |
- (.andromeda/design-system.md:377) No colour, glyph or cursor control reaches `viola run`'s terminal, a start refusal included.
  > - **NEVER emit colour, glyphs or cursor control under `--json`, non-TTY, `NO_COLOR`, `TERM=dumb` or `viola run`.** LLM drivers parse this output, and `viola run`'s terminal belongs to the child's screen; a passed `viola revive` is the same passthrough.
- (.andromeda/design-system.md:379) The squatted refusal and its hint go to stderr only, never mixed into stdout.
  > - **NEVER mix data and messages.** Results go to stdout. `waiting:`, refusals and their `hint:` lines, and errors go to stderr.
- (.andromeda/design-system.md:380) `refuse_squatted` prints neither the pipe path nor the holder's pid, and no anyhow chain from a pipe-creation failure reaches the human.
  > - **NEVER print upstream text, paths, pids or anyhow chains with serde sources in errors or hints.** This is the security plan's NEVER-log floor, and human output of `last` / `list` escapes C0/C1 controls.
- (.andromeda/design-system.md:383) A listener or pipe-creation failure at start prints no stack trace; its detail goes to `diagnostics/` only.
  > - **NEVER print stack traces.** Full detail goes to `diagnostics/` only.

## Bindings
- **surface (start refusal lines, exit 1) ↔ brand-identity (`viola run` passthrough at 0.0)**: the squatted start refusal is a fixed-message stderr pair, and `run` prints it with no colour, glyph or SGR, nothing once the child starts, (.andromeda/design-system-detail/surface.md:59) with (.andromeda/design-system-detail/brand-identity.md:21) and (.andromeda/design-system.md:377).
- **surface (refusal form) ↔ iconography (ASCII, two-space fields)**: both the `unable:` start refusal and the `unable  budget-paused  <window>` send refusal are ASCII lines whose fields are two-space separated, (.andromeda/design-system-detail/surface.md:21) with (.andromeda/design-system-detail/iconography.md:10).
- **color-palette (budget reason and detail words) ↔ surface (exit 11)**: the reason `budget-paused` and the detail words `five-hour` / `seven-day` the re-greened case asserts are the exit-11 refusal's fields, (.andromeda/design-system-detail/color-palette.md:99) with (.andromeda/design-system-detail/surface.md:105).
- **surface (no path or pid in a start refusal) ↔ the body's cli bans**: the squatted refusal shows no pipe name, path, pid, chain or stack trace, (.andromeda/design-system-detail/surface.md:59) with (.andromeda/design-system.md:380) and (.andromeda/design-system.md:383).
- **surface (no `--json` document for a `run` start refusal) ↔ surface (exit-1 row)**: the squatted refusal stays exit 1 with no `--json` document until "Exit-cause code catalogue", (.andromeda/design-system-detail/surface.md:93) with (.andromeda/design-system-detail/surface.md:102).
