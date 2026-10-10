# layouts extract

## Relevance
partial — the chunk creates no surface element, region or focusable item on either surface; layout-templates binds it only through what the cli surface forbids on the paths it touches (`run` / `revive` passthrough, `hook`), through the `viola verify` step-line form if a ledger row lands (scope item 8, behind the founder's word), and through the header cells a later reader fills from the reading this chunk writes.

## Constraints
- `viola hook` has no human surface: layout-templates §Surface: cli · Component — Header / banner requires that the `hook statusline` arm print no viola-authored line of its own, so everything on its stdout is the user's command's output (scope item 4).
- `viola run` is passthrough and prints nothing of its own while the child holds the terminal: layout-templates §Surface: cli · Output structure — `viola run` requires that resolving the statusline source and writing `instances/<name>/settings.json` (scope items 1, 3) add no line on a start that succeeds.
- `viola revive` is passthrough like `run`: layout-templates §Surface: cli · Output structure — `viola revive` requires the same silence on a start that passes its preflight, so the override write on the revive path (scope item 3) adds no line there either.
- The passthrough runs at expression level 0.0: layout-templates §Surface: cli · Expression level requires no colour, no non-ASCII glyph and no cursor control from viola on the `run` / `revive` path, which bounds anything the wrapper side might emit around the wrapped statusline.
- A start refusal has one fixed form: layout-templates §Surface: cli · Component — Primary content block 2: refusal lines and the `unable` column requires an exit-1 start refusal to be an `unable: <text>` line with its own `hint:` line last on stderr, naming no path and no pid. The section lists no statusline refusal pair; whether this chunk's start path refuses at all (an unreadable source, a failed override write) is research's question, and a new pair is a layout amendment, not an implementation choice.
- The `viola verify` step counter follows the ledger: layout-templates §Surface: cli · Output structure — `viola verify` requires one stdout step line per ledger row in ledger order, a counter total equal to the row count (it grows as owning chunks land their rows) and the `stamped` summary as the last stdout line. This applies only if a statusline row lands under scope item 8, which waits for the founder's word.
- The verb surface is flat and non-interactive: layout-templates §Surface: cli · Component — Primary navigation (verb structure) requires no interactive prompt and no new global flag, so the per-home redirect for tests (scope item 2) takes no human-facing flag or verb form on this surface.

## Patterns to follow
- The step-line form `[NN/MM] <row id> <row words>  pass|fail`, plain ASCII, appended once with no redraw, for any probe a statusline ledger row adds (layout-templates §Surface: cli · Output structure — `viola verify`).
- The `unable:` / `hint:` pair with a fixed message, distinct per cause and never naming `viola release`, for any refusal the start path gains (layout-templates §Surface: cli · Component — Primary content block 2: refusal lines and the `unable` column; §Surface: cli · Component — Primary navigation (verb structure)).
- The exit code as the terminator with no closing word, for any verb output this chunk touches (layout-templates §Surface: cli · Component — Footer / terminator).
- A missing reading is printed as the word `unknown` in its cell, never left blank: the vocabulary the reading's `"unknown"` values map onto when a reader lands (layout-templates §Surface: web-spa · Wireframe — Bay, first reading / empty).

## Anti-patterns to avoid
- No banner, version line, status line or summary word on `run`, `revive` or `hook` output (layout-templates §Surface: cli · Component — Header / banner).
- No new human column or field for the statusline command or its source on the board or the strip: the six shared columns are fixed and a seventh is a design change (layout-templates §Surface: cli · IA notes, "Output as a contract"; §Surface: cli · Component — Primary content block 1: `viola list` strip rows).
- No spinner, progress bar or elapsed counter while the wrapper waits on the user's command: the step counter of `viola verify` is the surface's only progress pattern (layout-templates §Surface: cli · Primary screens (commands)).

## Contract bindings
- layouts ↔ architecture (`budget.json`, §Standard Contracts): the reading this chunk writes is the source of the cli BAY line's `5H` / `7D` / `expired` / `read <age> ago` cells (layout-templates §Surface: cli · Component — Header / banner) and of the web header's `5H` / `7D` figures, `resets` times, `expired` box and `read … ago` cell (layout-templates §Surface: web-spa · Component — Header (`<viola-atis>`)). This chunk builds no reader; the per-window `used_percentage` / `resets_at` / `read_at` shape and its `"unknown"` values must be enough for those cells. Whether the written shape carries what each cell prints is research's question.
- layouts ↔ architecture (capability ledger) and test-plan (verify probes): a landed statusline row changes the row-id list and the counter total printed by `viola verify` (layout-templates §Surface: cli · Output structure — `viola verify`), which is a layout-templates amendment at the wrap.
- layouts ↔ security-plan (error sanitization): any start refusal text is a fixed message with no path, pid or upstream text, the statusline command string included (layout-templates §Surface: cli · Component — Primary content block 2: refusal lines and the `unable` column).
- layouts ↔ a11y: (none) — the chunk adds no focusable element, region or announcement.

## Acceptance criteria contributions
- (layouts) `viola run <name> -- <agent>` with a user statusline configured prints no viola-authored line on stdout or stderr on a start that succeeds (per layout-templates §Surface: cli · Output structure — `viola run`)
- (layouts) `viola hook statusline` stdout holds the user's command's output and nothing else: no header, banner or terminator from viola (per layout-templates §Surface: cli · Component — Header / banner)
- (layouts) `viola --help` prints the same five verb groups with no statusline verb added (per layout-templates §Surface: cli · Output structure — `viola --help`)
- (layouts) Only if a statusline ledger row lands under the founder's word: `viola verify` prints one step line for it in ledger order, the counter total equals the row count, and the `stamped` summary stays the last stdout line (per layout-templates §Surface: cli · Output structure — `viola verify`)
