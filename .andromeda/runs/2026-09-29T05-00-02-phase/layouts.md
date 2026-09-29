# layouts extract

## Relevance
partial — the chunk builds no surface and changes no layout. Its only tie to layouts is the cli surface's `viola run` passthrough, and that tie applies only if step 1 or step 3 puts the loss in the `run` pump's resize/write path. The web-spa surface is out of scope.

## Constraints
- While the child runs, `viola run` is pure passthrough and prints nothing, because the terminal belongs to the wrapped claude TUI until it exits (per layout-templates §Surface: cli / Primary screens (commands) and §Output structure — `viola run`). A pump-side fix or a resize observation must add no line, banner or notice to the wrapper's terminal. Whether the pump already writes anything on a resize is research's question.
- The `viola run` passthrough sits at expression level 0.0, which means no colour, no non-ASCII glyph and no cursor control from viola itself (per layout-templates §Surface: cli, expression level). A resize-path change must not inject any SGR or cursor sequence of viola's own into the child's screen stream.
- The rule "No banner anywhere" includes `viola run` (per layout-templates §Component — Header / banner (the BAY context line)). A documented H2 platform limit therefore lands in docs (gotcha, crate note or architecture amendment, which is P4's fork). It never lands as a runtime warning line.
- Streams stay separated: results go to stdout, and refusals, hints and errors go to stderr, never mixed (per layout-templates §Component — Primary content block 2: refusal lines and the `unable` column). The child recorder's report lines belong to the test child inside the PTY. They are not viola CLI output and must not be routed through or modelled on any viola verb's output structure.
- Human CLI output is a stable contract that LLM drivers read, and new fields go to `--json` first (per layout-templates §Surface: cli / IA notes, "Output as a contract"). No H2 measurement or resize state may become a new human column or word in `viola list`, `viola send` or any other verb within this chunk.

## Patterns to follow
- Silent passthrough: while `viola run` holds the terminal, the wrapper speaks only through the exit-1 start refusals, before the child exists (per layout-templates §Output structure — `viola run`). Keep any diagnostics from the probe or fix off the terminal. They go to the recorder, the test report or process logs, which are other domains.
- Terminal width is used only where it is known, and no line ever wraps (per layout-templates §Component — Primary content block 1: `viola list` strip rows, Truncation). This is the only place the layout plan consumes terminal size. A seam change to how size is read or propagated must not change `viola list`'s width-known behaviour. Whether `viola list` reads its width through viola-pty at all is research's question.

## Anti-patterns to avoid
- Printing a "resize lost" or "key may have been dropped" notice, spinner or status line into the `viola run` terminal (per layout-templates §Output structure — `viola run` and §Component — Footer / terminator: no summary banner, no persistent footer).
- Surfacing H2 state as a new human column or word on the CLI or web strips. That would break the six-column parity and is a design change, not an implementation choice (per layout-templates §Surface: cli / IA notes).

## Contract bindings
- cli expression level 0.0 for `viola run` passthrough ↔ design §cli expression level / cli patterns (no colour, glyph or cursor control from viola) ↔ architecture's reserved-stdout rule ("the child's screen" is one of stdout's reserved uses). A pump-side fix is bound by all three.
- Stream separation (per layout-templates §Component — Primary content block 2) ↔ obs §3 pipeline: anything the probe wants to record goes through the obs/diagnostics channel, never the terminal.

## Acceptance criteria contributions
- (layouts) If the fix touches the `run` pump's resize/write path, a wrapped session that is resized while running shows no viola-originated byte on the terminal (no line, SGR or cursor sequence). The child's screen is the only content (per layout-templates §Output structure — `viola run`).
- (layouts) No verb's human output gains a column, word or line for H2 or resize state. `viola list` keeps exactly NAME · LIVE · STATUS · WHEEL · DIALOG · CLI (per layout-templates §Component — Primary content block 1: `viola list` strip rows).
