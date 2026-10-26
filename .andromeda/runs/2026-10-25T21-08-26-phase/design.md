# design extract

## Relevance
partial — the chunk is IPC access control (no token, colour, layout or new output); design binds only through the CLI surface's existing `viola run` squatted-name start refusal (exit 1), which the scope says stands unchanged.

## Constraints
- design-system §Surface: cli › Component Patterns (5. `viola verify` / `viola run`) requires a `run` start refusal (exit 1) to print its cause's fixed-message `unable:` line then that cause's hint, showing no path and no pid; the squatted-name cause's pair is listed in `.andromeda/design-system-detail/surface.md` (cli-send group, lines 43–74) as `unable: the endpoint for <name> is held by another process` → `hint: another process holds this name's endpoint; stop it or pick another name`. Any listener-level change in this chunk must leave that pair byte-identical.
- design-system §Surface: cli › Tokens (platform-specific), Streams, requires refusals and their `hint:` line on stderr, the `unable: <text>` fixed form reserved to the exit-1 refusals of `run`, `revive`, `verify` (and `list`'s exit 21); stdout carries no refusal text.
- design-system §Surface: cli › Component Patterns, Exit-code phraseology, requires exit 1 to read `error: internal error` or a fixed-message `unable:` refusal with its hint, and gives a `run` start refusal no `--json` document (owed to the route entry "Exit-cause code catalogue"); a new listener failure surfaced at start must map to one of these forms, not a new one.
- design-system §Anti-Patterns › Per-Surface Bans (cli) forbids printing paths, pids or anyhow chains with serde sources in errors or hints: the pipe name (`\\.\pipe\…`), the holding process's pid and an OS error text must not reach the squat refusal or any new start-refusal line.
- design-system §Surface: cli › Platform-Specific Notes requires that stack traces never print and errors stay fixed messages, full detail going only to `instances/<name>/diagnostics/`; a pipe-mode or DACL failure detail belongs there, not on stderr.
- design-system §Surface: cli › Tokens (platform-specific), Colour decision order item 2, gives `viola run` no colour, glyph or SGR; the start refusal stays plain ASCII.

## Patterns to follow
- The fixed `unable:` / `hint:` pair per start-refusal cause, one hint per cause, as already catalogued for `already live`, `still running but not answering` and the squatted-name cause (design-system §Surface: cli › Component Patterns; detail `surface.md` cli-send group).
- Exit-1 refusal text as a closed, fixed-message set listed in the exit-code table (detail `surface.md` cli-exit-codes group, lines 99–110): a test asserts the exact stderr pair and an empty stdout.
- Hints human-mode only; the machine view carries a closed code (`squatted-name`, obs-plan D-20 per detail `surface.md` cli-send group), never prose.

## Anti-patterns to avoid
- Rewording, restyling or extending the squatted-name refusal while hardening the listener (design-system §Surface: cli › Component Patterns: fixed messages).
- Leaking the pipe path, a pid or a Win32 error string into a refusal or hint (design-system §Anti-Patterns › Per-Surface Bans, cli).
- Adding a `--json` document or a new human line for a `run` exit-1 start refusal ahead of "Exit-cause code catalogue" (design-system §Surface: cli › Component Patterns, Exit-code phraseology).

## Contract bindings
- design (cli refusal phraseology) ↔ obs: the squatted-name refusal's logged detail code `squatted-name` (obs-plan D-20, named in detail `surface.md` cli-send group).
- design (exit-code phraseology) ↔ architecture §Conventions (exit codes, refusal words): exit 1 and `squatted-name` stand.
- design (no paths/pids in errors) ↔ security-plan §Error Handling / NEVER-log floor.
- No a11y binding: the chunk adds no colour, motion or state hue.

## Acceptance criteria contributions
- (design) A `viola run` start on a held endpoint name exits 1 with stderr exactly `unable: the endpoint for <name> is held by another process` then `hint: another process holds this name's endpoint; stop it or pick another name`, stdout empty — whether `tests/path1_run_start.rs` / `tests/cli_instance_state.rs` already assert the exact hint text is research's question (per design-system §Surface: cli › Component Patterns).
- (design) No start-refusal or listener-failure output names the pipe path, a pid or an OS error text (per design-system §Anti-Patterns › Per-Surface Bans).
- (design) The chunk adds no new human output line, SGR byte or `--json` document for `run`'s exit 1 (per design-system §Surface: cli › Component Patterns, Exit-code phraseology).
