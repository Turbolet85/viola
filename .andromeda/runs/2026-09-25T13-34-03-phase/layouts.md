# layouts extract

## Relevance
Partial. The only layout content that applies is the cli surface's `viola run` output rules: passthrough, the start-refusal line form, the stderr/stdout split and the exit terminator. The web-spa surface is out of scope because the chunk boundaries exclude the UI.

## Constraints
- While the child runs, `viola run` prints nothing, and the terminal belongs to the wrapped TUI until it exits (per layout-templates §Surface: cli / Output structure — `viola run`; §Surface: cli / Component — Header / banner "No banner anywhere… `viola run` prints nothing"). This is the layout side of the chunk's "zero own bytes" requirement.
- `viola run` passthrough is expression level 0.0: no colour, no non-ASCII glyph and no cursor control from viola (per layout-templates §Surface: cli / Expression level).
- A start refusal uses the fixed-message exit-1 form `unable: <name> is already live` "and its sibling causes, listed in design-system cli pattern 2" (per layout-templates §Surface: cli / Component — Primary content block 2: refusal lines). The plan does not list a `.cmd`/`.bat` cause or a shim-resolution cause. Whether design-system cli pattern 2 already lists these causes and their wording is a question for research.
- Every exit-1 cause gets its own `hint:` line, and that hint is the last line on stderr. A hint never quotes upstream text and never names a path or a pid (per layout-templates §Surface: cli / Component — Primary content block 2 "Hint line"; §Output structure — `viola run` "no paths, no pids"). So a `.cmd` refusal must not print the resolved shim or `.exe` path.
- Refusals, hints and errors go to stderr, and results go to stdout. The two streams are never mixed (per layout-templates §Surface: cli / Component — Primary content block 2 "Streams").
- The exit code is the terminator. There is no `done.`, `success` or summary banner (per layout-templates §Surface: cli / Component — Footer / terminator). The chunk's process-handle exit must therefore surface only as the wrapper's exit code.
- The invocation form is `viola run <name> -- claude`, and `<name>` is always a `ViolaName` (per layout-templates §Surface: cli / Component — Primary navigation (verb structure)).

## Patterns to follow
- A refusal is followed directly by one hint line: `unable…` on stderr, then `hint: <one plain instruction>` keyed by cause. The existing run example is `unable: builder is already live` / `hint: viola list` (per layout-templates §Surface: cli / Output structure — `viola run`).
- Hints act as breadcrumbs by naming the next verb (`viola list`, `viola verify`), never a path (per layout-templates §Surface: cli / Component — Primary navigation "Discoverability").
- Across surfaces, the page's only printed command is the empty-rack `viola run <name> -- claude`, so the verb's spelling and argument order stay stable (per layout-templates §Surface: web-spa / IA notes "Multi-surface coordination").

## Anti-patterns to avoid
- The wrapper must not write its own bytes to stdout during passthrough: no banner, no `Starting…`, no spinner, no trailing status line and no SGR or cursor sequences (per layout-templates §Surface: cli / Output structure — `viola run`; §Expression level 0.0).
- Refusal or hint text must not name the resolved `claude.cmd` / `claude.exe` path or a pid, and must not echo upstream or environment text (per layout-templates §Surface: cli / Component — Primary content block 2 "Hint line").
- The wrapper must not print a terminator word or summary after the child exits (per layout-templates §Surface: cli / Component — Footer / terminator).

## Contract bindings
- cli refusal lines ↔ design-system cli pattern 2. That pattern is the authority for the list of exit-1 causes and the hint for each cause (per layout-templates §Surface: cli / Component — Primary content block 2).
- cli hints ↔ tests' exit-cause matrix, which requires a cause-specific hint (per layout-templates §Decisions Log, overseer fix pass T4).
- cli refusal/hint content ↔ security (no paths, pids or upstream text). This lines up with the chunk's rule that the stripped `CLAUDE*` values never reach an error body (per layout-templates §Surface: cli / Component — Primary content block 2 "Hint line").

## Acceptance criteria contributions
- (layouts) While `viola run <name> -- <child>` passes through, the wrapper's stdout is byte-identical to the child's PTY output, with no leading or trailing bytes and no SGR or cursor sequences (per layout-templates §Surface: cli / Output structure — `viola run`; §Surface: cli / Expression level).
- (layouts) A `.cmd` / `.bat` child refusal exits 1 with a fixed-message `unable: …` line on stderr, followed by a cause-specific `hint: …` as the last stderr line. Stdout stays empty, and neither line contains a filesystem path or a pid. The exact wording follows design-system cli pattern 2 (per layout-templates §Surface: cli / Component — Primary content block 2).
- (layouts) After the child exits, the wrapper prints nothing further and its exit code carries the outcome (per layout-templates §Surface: cli / Component — Footer / terminator).

## Relevant amendment history
(none). The sidecar `D:/dev/projects/viola/.andromeda/layout-templates-amendments.md` does not exist yet. The plan's own Decisions Log T4 entry (one hint per exit-1 cause) is the nearest earlier change and is cited above.
