group: architecture-2
read: .claude/docs/services/viola-pty.md · 47 · «- `.andromeda/architecture.md` [PTY] · `.claude/docs/gotchas»
read: .claude/docs/services/viola-state.md · 49 · «- `.andromeda/architecture.md` §Standard Contracts (snapshot»
read: .claude/docs/services/viola-ui.md · 9 · «None: the `viola_ui` log target in `src/obs.rs`, the reserve»
read: .claude/docs/services/viola.md · 58 · «- `.andromeda/architecture.md` · `.claude/rules/api.md` · `.»
read: .claude/docs/stack.md · 66 · «5. Commit as `chore: bump {tool} to {version}`»

- leaf: .claude/docs/stack.md:11   class: stale
  leaf-text: «(derive; `default-features = false`, its default set minus `color`: plain help/usage, no colour/terminal crate, no terminal env reads)»
  source: .andromeda/architecture.md:15 «clap 4.6.7 (derive; `default-features = false` with `std`, `derive`, `help`, `usage`, `error-context`, `suggestions` — its default set minus `color`, so help and usage output is plain and no colour/terminal crate is in the graph)»
  replace-with: «(derive; `default-features = false` with `std`, `derive`, `help`, `usage`, `error-context`, `suggestions` — its default set minus `color`, so help and usage output is plain and no colour/terminal crate is in the graph)»

- leaf: .claude/docs/services/viola.md:6   class: omission
  leaf-text: «with its phases under `src/human/` — mirror, escape, board, wait, verbs, hints —»
  source: .andromeda/registries/contracts/architecture/project-directory-structure.md:23 «style.rs: the colour layer (`Depth`, `Tokens`, `Facts`, `decide`, `read`, the Windows VT enable and its restoring guard);»
  replace-with: «with its phases under `src/human/` — mirror, escape, board, style (the colour layer: `Depth`, `Tokens`, `Facts`, `decide`, `read`, the Windows VT enable and its restoring guard), wait, verbs, hints —»

- leaf: .claude/docs/services/viola.md:11   class: stale
  leaf-text: «windows-sys (Windows only: VT enable, console width, the registry value-name reader»
  source: .andromeda/registries/contracts/architecture/crate-dependency-direction.md:10 «`GetConsoleMode` / `SetConsoleMode` for `src/human/style.rs`'s VT enable on stdout: `ENABLE_VIRTUAL_TERMINAL_PROCESSING` (`0x0004`) alone, never viola-pty's `0x000c`»
  replace-with: «windows-sys (Windows only: `GetConsoleMode` / `SetConsoleMode` for `src/human/style.rs`'s VT enable on stdout — `ENABLE_VIRTUAL_TERMINAL_PROCESSING` (`0x0004`) alone, never viola-pty's `0x000c`, made only once every no-colour rung has passed in `viola list`'s human mode, the saved mode restored on drop, a failed get or set writing no colour and changing no mode, no dependency or feature added —, the registry value-name reader»

- leaf: .claude/docs/services/viola.md:29   class: omission
  leaf-text: «Output: colour decision order, stdout/stderr split, `[RB]` mirror, cause-specific `hint:` lines, C0/C1 escaping (see `rules/frontend.md`).»
  source: .andromeda/architecture.md:404 «`NO_COLOR`, `TERM` and `COLORTERM`, read by viola's own colour layer (`src/human/style.rs`, `viola list`'s human mode only) to decide whether and how deep its SGR bytes are written»
  replace-with: «Output: colour decision order, stdout/stderr split, `[RB]` mirror, cause-specific `hint:` lines, C0/C1 escaping (see `rules/frontend.md`). The colour layer (`src/human/style.rs` `read`, `viola list`'s human mode only, every build) reads, never sets, `NO_COLOR` (present, empty included: colour off), `TERM` (exactly `dumb`: off; containing `256color`: 256 colours) and `COLORTERM` (`truecolor` or `24bit`: truecolor) through `std::env::var_os`, only compared, never echoed, deciding only whether and how deep SGR bytes are written (the founder's own word, chunk 2026-10-17-cli-output-discipline); `COLUMNS`, `CLICOLOR`, `CLICOLOR_FORCE` and `CI` are not read (the board's width is `viola_pty::host_size()`).»

- leaf: .claude/docs/services/viola.md:45   class: omission
  leaf-text: «or the human board as one stdout write»
  source: .andromeda/registries/contracts/architecture/project-directory-structure.md:23 «`write_board`: with a known terminal width the unwrapped NAME cut, at a colour depth the spans); style.rs: the colour layer»
  replace-with: «or the human board as one stdout write (`write_board`: with a known terminal width the unwrapped NAME cut, at a colour depth from `src/human/style.rs` the spans)»
