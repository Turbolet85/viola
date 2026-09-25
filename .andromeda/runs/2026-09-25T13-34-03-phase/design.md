# design extract

## Relevance
Partial. The chunk has no rendered UI: no web page, no tokens, no motion. It does touch the design system's cli surface in two places: `viola run` passthrough output, which is expression 0.0, and the exit-1 `.cmd`/`.bat` start refusal wording. Nothing in design-system §Color Palette, §Typography, §Spacing, §Depth Strategy, §Border Radius, §Motion, §Iconography or §Surface: web-spa applies.

## Constraints
- design-system §Brand Identity (per-surface expression table, row "cli, `--json` / non-TTY / `NO_COLOR` / `TERM=dumb` / `viola run` passthrough") sets `viola run` at expression 0.0. That means no colour, no glyphs, no cursor control and no spinner, and `viola run` prints nothing at all while the wrapped `claude` TUI runs. This is the design-side form of the scope's "zero own bytes".
- design-system §Surface: cli → Colour decision order, item 2, requires that `viola run` emit no colour, no glyph and no SGR while the child runs. The rule is first-match and sits above the `NO_COLOR` / TTY checks, so a TTY stdout does not bring colour back for `run`.
- design-system §Surface: cli → Component Patterns 5 (`run`) requires that nothing is printed once the child starts. A start refusal (exit 1) prints exactly one fixed-message line and then that cause's `hint:` line. Neither shows a path or a pid.
- design-system §Surface: cli → Component Patterns 2 (exit-1 start refusals) and the Exit-code phraseology row for exit 1 fix the `.cmd`/`.bat` refusal text:
  - line: `unable: <name>'s command is a .cmd or .bat script`
  - hint: `hint: pass the real executable, not a .cmd or .bat shim`
  - Both go to stderr. Exit 1 has no `--json` document until arch amends it, and hints are human-mode only.
- design-system §Surface: cli → Streams requires refusals, their `hint:` line and `error: …` lines on stderr. Results go to stdout. Because of the passthrough rule, the wrapper's stdout during `run` carries only the child's bytes.
- design-system §Surface: cli → Platform-Specific Notes ("Stack traces never print") and the Exit-code row for exit 1 (`error: internal error`, fixed message, no chain, no paths, no hint) govern any spawn, resolution or PTY failure that is not the `.cmd` refusal. Full detail goes only to `instances/<name>/diagnostics/`. Whether that directory exists yet is out of this chunk's boundary; that is research's question.
- design-system §Surface: cli → Toolkit notes that Windows VT enabling (`SetConsoleMode(ENABLE_VIRTUAL_TERMINAL_PROCESSING)`) exists for viola's own styled output. Under the 0.0 row, the wrapper's console belongs to the child's screen. Whether the ConPTY pump needs to change console modes for byte-exact passthrough is a question for research and architecture, not design. Design only requires that viola adds no bytes of its own.

## Patterns to follow
- Refusal vocabulary: `unable` plus a fixed reason, in the phraseology anchor (design-system §Brand Identity → "Standard phraseology and 'unable' plus a reason"). The exit-1 start refusals use the fixed-message form `unable: <cause>`, not the two-space field form (design-system §Surface: cli → Streams).
- One hint per cause, directly after its refusal line (design-system §Surface: cli → Component Patterns 2, T4). The `.cmd`/`.bat` cause has its own line and its own hint.
- Human output is ASCII only (design-system §Surface: cli → Width). The refusal and hint text contain no `·` and no non-ASCII glyphs.

## Anti-patterns to avoid
- Emitting colour, glyphs or cursor control under `viola run`. That includes a banner, a "starting…" line, a title-set OSC or any wrapper-injected escape before, during or after the child's screen (design-system §Anti-Patterns → Per-Surface Bans → cli, "NEVER emit colour, glyphs or cursor control under … `viola run`").
- Printing paths, pids, upstream text or anyhow/serde chains in errors or hints. For example, the resolved `claude.exe` path, the shim path, the `.cmd` path, the child pid, or any stripped `CLAUDE*` value (design-system §Anti-Patterns → Per-Surface Bans → cli, "NEVER print upstream text, paths, pids or anyhow chains…"; also "NEVER print stack traces").
- Mixing data and messages: refusal and error text must never land on stdout, where it would merge with the child's passthrough stream (design-system §Anti-Patterns → Per-Surface Bans → cli, "NEVER mix data and messages").

## Contract bindings
- **design ↔ obs:** the `.cmd`/`.bat` refusal's machine view is the obs-plan D-20 log detail code `batch-script-child`. Design sets the human line; obs sets the code (design-system §Surface: cli → Component Patterns 2, the "Pending an arch amendment" note).
- **design ↔ security:** the `.cmd`/`.bat` refusal enforces the security rule "never spawn a `.cmd`/`.bat` PTY child". The no-paths / no-pids / no-upstream-text floor is the security plan's NEVER-log floor. That floor also covers the stripped `CLAUDE*` secrets (design-system §Anti-Patterns → cli).
- **design ↔ arch:** exit 1 has no `--json` document "until arch amends it". Design does not invent one (design-system §Surface: cli → Exit-code phraseology, exit 1 row).
- **design ↔ tests:** the tests' exit-cause matrix lists the exit-1 causes, `.cmd`/`.bat` child among them (design-system §Design Decisions Log, 2026-09-24 fix pass, T4). The `run_cli` / harness tests switched to the PTY form here are the natural place for the checks below.

## Acceptance criteria contributions
- (design) While the child runs, `viola run` adds zero bytes to the child's output stream on stdout: no SGR, no OSC, no cursor control, no banner, even when stdout is a TTY and `NO_COLOR` is unset (per design-system §Brand Identity per-surface table, 0.0 row, and §Surface: cli → Colour decision order, item 2).
- (design) A child that resolves to `.cmd`/`.bat`:
  - exits 1 and is never spawned
  - prints to stderr exactly `unable: <name>'s command is a .cmd or .bat script`, then `hint: pass the real executable, not a .cmd or .bat shim`
  - prints nothing to stdout
  - prints no path and no pid
  (per design-system §Surface: cli → Component Patterns 2 and the Exit-code phraseology exit 1 row)
- (design) Any other start or spawn failure prints the fixed `error: internal error` line with no hint, no error chain, no path, no pid and no stack trace (per design-system §Surface: cli → Exit-code phraseology exit 1 row and Platform-Specific Notes).
- (design) No stderr or stdout text from `viola run`, including refusals and errors, contains any stripped `CLAUDE*` variable name/value pair or the resolved executable path (per design-system §Anti-Patterns → Per-Surface Bans → cli, "NEVER print upstream text, paths, pids…").

## Relevant amendment history
(none). The sidecar `D:/dev/projects/viola/.andromeda/design-system-amendments.md` does not exist, which is normal before a first amendment. For nearby context, the plan's own §Design Decisions Log has two relevant 2026-09-24 entries:
- "Exit-1 start refusal hint": every `run` start refusal carries a hint, with no paths or pids.
- The overseer fix pass T4: one fixed-message line and one hint per exit-1 cause, `.cmd`/`.bat` child included. No `--json` detail code until arch amends it. The goal was cause-specific recovery without leaking paths or pids.
