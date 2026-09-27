# design extract

## Relevance
partial. The chunk adds no UI or CLI output, and harness tooling (pre-push, `run --mutants`, `scripts/wsl-exec.sh`, the cache report) is not a design surface. Design applies only as a regression guard: the moved or deduplicated code sits behind the `viola run` passthrough (`src/cmd/run.rs` tracing capture, the `viola-pty` HostTerminal split) and the CLI stream contract.

## Constraints
- design-system §Brand Identity (expression table, the "cli, `--json` / non-TTY / `NO_COLOR` / `TERM=dumb` / `viola run` passthrough" row, 0.0) requires that `viola run` print nothing at all while the wrapped `claude` TUI runs. The shared tracing-capture helper that replaces the `src/cmd/run.rs` / `viola-channel` `test_capture` clone must not add any stdout/stderr write on the run path. Whether today's `run.rs` capture already writes only to the log sink and never to the terminal is research's question.
- design-system §Surface: cli, Colour decision order item 2 requires `viola run` to have no colour, glyphs, SGR or cursor control while the child runs. The `viola-pty` `lib.rs` split, including the Windows `HostTerminal::enter` / `host_size` code moved into submodules, must keep the host terminal belonging to the child's screen.
- design-system §Surface: cli, component 5 (`run`) requires the exit-1 start refusals to keep their fixed-message line plus that cause's `hint:` line, with no path or pid. Any `viola-channel` `server.rs` or `run.rs` code moved in the splits must leave these strings byte-identical, per the scope's "no product behaviour change".
- design-system §Surface: cli, Streams requires results on stdout and context, refusals, hints and errors on stderr. The endpoint-fixture dedupe between `server.rs` and `tests/channel_endpoint.rs` must not change which stream production code writes to.
- design-system §Surface: cli, Platform-Specific Notes requires that stack traces never print and that full detail go only to `instances/<name>/diagnostics/`. The shared tracing-capture helper must not route diagnostic detail to the terminal.

## Patterns to follow
- The phraseology vocabulary (`unable` + typed reason, fixed-message start refusals) is the design. Treat existing refusal and hint strings as fixed text when moving code, per design-system §Brand Identity (Standard phraseology) and §Surface: cli component 2 / component 5.
- The CLI styling is a small hand-written SGR module in the `viola` bin with no dependency, per design-system §Surface: cli, Toolkit. If a split or dedupe touches styling call sites, keep that single module rather than adding a crate.

## Anti-patterns to avoid
- design-system §Anti-Patterns, Per-Surface Bans cli: "NEVER emit colour, glyphs or cursor control under `--json`, non-TTY, `NO_COLOR`, `TERM=dumb` or `viola run`." Any new killing test or forced-window repro hold in `viola-pty` must not create a production path that writes to the host terminal.
- design-system §Anti-Patterns, Per-Surface Bans cli: "NEVER print upstream text, paths, pids or anyhow chains with serde sources in errors or hints" and "NEVER print stack traces." Shared test/prod helpers must not surface these on stderr.

## Contract bindings
- design ↔ security/obs: the §Surface: cli "nothing printed under `viola run`" and "detail only to `diagnostics/`" rules are the terminal side of the security plan's NEVER-log floor and of obs's log sink. The tracing-capture dedupe (item 2) lands on exactly this seam.
- design ↔ tests: the §Surface: cli exit-code phraseology table (typed exit + first stderr word) matches the "typed exit per command" contract the scope says holds. Split or dedupe work must keep both unchanged.

## Acceptance criteria contributions
- (design) After the `run.rs` tracing-capture dedupe and the `viola-pty` split, `viola run` still writes no bytes to stdout or stderr between child start and child exit (per design-system §Surface: cli, Colour decision order item 2 and §Brand Identity expression table 0.0 row).
- (design) `viola run` start-refusal and `viola send` refusal/hint strings, and their streams (stderr) and exit codes, are byte-identical before and after the four splits (per design-system §Surface: cli, component 5 and Exit-code phraseology).
- (design) No new code path from the clone-pair helpers or killing tests adds SGR, glyphs, cursor control or stack-trace output to production CLI output (per design-system §Anti-Patterns, Per-Surface Bans cli).

## Relevant amendment history
(none)
