# design extract

## Relevance
Partial. This chunk has no rendered UI surface, so tokens, typography, spacing, depth, radius, motion and iconography do not apply. The cli surface's rules on stream discipline, stack traces and "detail goes to diagnostics/" do bind the sinks, the panic hook and the `viola run` migration.

## Constraints
- `viola run` must print nothing at all while the wrapped `claude` TUI runs, and it sits at expression 0.0: no colour, no glyphs, no cursor control. So the new JSON sink layer and the panic hook must not write diagnostics to the terminal during passthrough. Whether today's `src/run/mod.rs` role lines or the panic hook reach stdout or stderr is research's question. (per design-system.md §Brand Identity, per-surface expression table, row "cli, `--json` / non-TTY / … / `viola run` passthrough"; §Surface: cli, Colour decision order item 2; §Surface: cli, Component Patterns 5 `run`)
- Stack traces never print. Errors are fixed messages, and full detail goes only to `instances/<name>/diagnostics/`. The CARRY panic-hook routing (payload and backtrace go to `detail-run.ndjson`, the home-level line stays payload-free) is the mechanism this rule requires. The human-facing output of a panic must stay a fixed message, never the default Rust `panicked at …` payload. (per design-system.md §Surface: cli, Platform-Specific Notes "Stack traces"; §Per-Surface Bans cli "NEVER print stack traces")
- Faults have no hint line, and their detail goes only to `diagnostics/`. `error: internal error` (exit 1) is a fixed message with no chain containing serde sources, no paths and no hint. The same applies to `error: wrapper fault` (exit 20). (per design-system.md §Surface: cli, Component Patterns 2, last hint bullet; §Exit-code phraseology rows 1 and 20)
- Errors and hints must never print upstream text, paths, pids or anyhow chains that include serde sources. That content belongs in the owner-only detail files, not on stderr. (per design-system.md §Per-Surface Bans cli "NEVER print upstream text, paths, pids or anyhow chains…")
- `with_ansi(false)` on the JSON layer matches the plan's rule that machine and non-TTY output carries no SGR. Diagnostics files are machine output and must never carry colour codes. (per design-system.md §Brand Identity expression 0.0 row; §Per-Surface Bans cli "NEVER emit colour, glyphs or cursor control under `--json`, non-TTY…")
- Stream discipline: results go to stdout, and messages and errors go to stderr. The diagnostics plane must not add a third category of terminal output. `viola hook` writes nothing to stderr and `viola mcp` writes only MCP frames on stdout, so the future `hook-<name>` / `mcp` sink roles this chunk names must be file-only. (per design-system.md §Surface: cli, Streams; Platform-Specific Notes "`hook` … `mcp`"; §Per-Surface Bans cli "NEVER mix data and messages")

## Patterns to follow
- The fixed phraseology words used by human cli output (`unable: <name> is already live`, `error: internal error`, `error: wrapper fault  <code>`) are the only terminal text for exit-1 and exit-20 outcomes. The obs line is where the matching detail goes. (per design-system.md §Surface: cli, Component Patterns 2 and 5; §Exit-code phraseology)
- The per-cause log detail codes that design points at for exit-1 and exit-21 causes are obs-plan D-20's `instance-dead`, `strict-modes-failed`, `server-verify-failed`, `already-live`, `squatted-name`, `pinned-hash-mismatch` and `batch-script-child`. Design leaves their naming to obs and does not restate them. (per design-system.md §Surface: cli, Component Patterns 2, the `--json` hint bullet; §Design Decisions Log, overseer fix pass T4)
- Torn and unknown lines are counted and kept visible where they occurred, never dropped silently. This is the design analogue of the harness merge emitting `{"torn":true,"offset":n}` in place. (per design-system.md §Brand Identity, "tower voice recorder" anchor; §Color Palette, domain status rows `Skipped::nonzero`)

## Anti-patterns to avoid
- Printing a stack trace, a panic payload or a backtrace to the terminal, including from the panic hook when no instance resolves. (per design-system.md §Per-Surface Bans cli "NEVER print stack traces")
- Any subscriber or fmt layer that writes to stdout or stderr, or that emits ANSI, while `viola run` owns the terminal. (per design-system.md §Per-Surface Bans cli "NEVER emit colour, glyphs or cursor control under … `viola run`")
- Putting paths, pids, upstream text or an anyhow/serde chain into a human-facing error line instead of the detail file. (per design-system.md §Per-Surface Bans cli "NEVER print upstream text, paths, pids or anyhow chains…")

## Contract bindings
- **design ↔ obs:** the design plan's "full detail goes only to `instances/<name>/diagnostics/`" is the target that this chunk's `detail-<process>.ndjson` writer and the panic routing must satisfy. The D-20 detail codes the cli hints point at live in obs's closed vocabulary. Whether they belong in `ObsEvent` or in a field is obs's decision. (per design-system.md §Surface: cli, Platform-Specific Notes; Component Patterns 2)
- **design ↔ security:** the NEVER-log floor for human output (no paths, pids or upstream text) mirrors the 0600/0700 owner-only detail-file contract. Content that is banned on the terminal goes to the owner-only files. (per design-system.md §Per-Surface Bans cli)
- **design ↔ tests:** `viola run` must be terminal-silent during passthrough, and no stack trace may appear on stderr. Tests can assert both on the run path this chunk migrates. (per design-system.md §Surface: cli, Component Patterns 5)

## Acceptance criteria contributions
- (design) While the wrapped child runs, `viola run` writes no diagnostics bytes to stdout or stderr. All role lines go only to `<home>/diagnostics/run-<name>.ndjson`. (per design-system.md §Surface: cli, Colour decision order item 2 / Component Patterns 5)
- (design) A forced panic prints no payload, backtrace or `panicked at` text to the terminal. The payload and backtrace appear only in `instances/<name>/diagnostics/detail-run.ndjson`. (per design-system.md §Surface: cli, Platform-Specific Notes "Stack traces")
- (design) Diagnostics sink lines contain no ANSI/SGR escape sequences (`\x1b[`). (per design-system.md §Per-Surface Bans cli, colour ban under non-TTY)

## Relevant amendment history
(none). `design-system-amendments.md` does not exist yet. The in-plan Design Decisions Log has no entry that touches diagnostics, logging or the panic path. The nearest is the overseer fix pass T4 (2026-09-24), which pointed the cli exit-1 and exit-21 hints at obs-plan D-20 detail codes and left `--json` detail codes pending an arch amendment.
