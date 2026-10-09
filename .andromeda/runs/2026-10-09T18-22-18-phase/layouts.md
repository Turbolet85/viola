# layouts extract

## Relevance
partial — the chunk creates and modifies no surface element (no region, component, focus stop or breakpoint); it only moves one class of `viola send` (a text with an inner CR or CRLF) from the refusal outcome to the confirmed outcome, so the cli surface's existing `viola send` output contract is what applies. The web-spa surface is not built by this chunk.

## Constraints
- A confirmed send takes the filled-mirror outcome line on stdout with exit 0, and a refused one the struck-mirror line on stderr with its typed exit; the chunk's newly confirmed shape must take the first form and no form of its own (per layout-templates §Surface: cli · Signature placement).
- The outcome line keeps its fixed layout: the four-character box column, the padded state word, the two-space gutter, NAME, then the outcome fields; the chunk adds no field to it — not a typed byte count, not a note that a CR was typed as LF (per layout-templates §Surface: cli · Component — Hero / signature output line (the `viola send` readback mirror)).
- The `empty-text` refusal is decided by the client before any frame and by the wrapper for a frame sent straight to it, with the mirror's `unable` line on stderr, exit 13 and its hint as the last stderr line; scope §2 keeps a text of only CR and LF characters in this refusal, so its line and its hint text must not change (per layout-templates §Surface: cli · Component — Primary content block 2: refusal lines and the `unable` column, bullet "Refusals decided before any request").
- A hint never quotes the sent text or any upstream text; no line this chunk touches may echo the received or the typed text, with or without its CRs (per layout-templates §Surface: cli · Component — Primary content block 2: refusal lines and the `unable` column, bullet "Hint line").
- Results go to stdout; the issue line, refusals and hints go to stderr; the two are never mixed. The confirmed inner-CR send follows the same split (per layout-templates §Surface: cli · Component — Primary content block 2: refusal lines and the `unable` column, bullet "Streams").
- The human columns and words are a stable contract read by LLM drivers; a new field goes into `--json` first and a new human column is a design change, not an implementation choice (per layout-templates §Surface: cli · IA notes, "Output as a contract").
- Under `--json` the mirror is replaced by the one typed document with the typed exit code, no glyph, no hint and no stderr line (per layout-templates §Surface: cli · Component — Hero / signature output line (the `viola send` readback mirror), bullet "Machine view").

## Patterns to follow
- The `viola send` transcript shape: the TTY-only issue line, then the outcome line appended under it and aligned column for column, never redrawn; with stdout piped the outcome line alone carries the mirror (per layout-templates §Surface: cli · Output structure — `viola send`). A test above the unit tier that reads the driver-visible outcome reads this line.
- Exit codes as the typed tail of every outcome: 0 for the confirmed send, 13 for `not-delivered` (per layout-templates §Surface: cli · Component — Primary content block 2: refusal lines and the `unable` column, bullet "Exit codes as the typed tail"). The chunk's change of outcome is visible to a driver as this exit and the line's first column.
- The last printed line says what happened, to whom and where, and the exit code is the terminator; on a refusal the hint is the last stderr line (per layout-templates §Surface: cli · Component — Footer / terminator).
- Whether the existing print path already yields these forms for the newly confirmed shape with no change to it is research's question: the plan fixes the output, and the scope changes the typed text, not a printed line.

## Anti-patterns to avoid
- No new line, word, glyph or colour for the conversion: no notice that a CR became an LF, no terminator word, no spinner, and `turn-ended` is never reported as a send outcome (per layout-templates §Surface: cli · Component — Hero / signature output line (the `viola send` readback mirror), bullets "Colour" and "What never happens").
- No new refusal detail and no new or reworded hint to carry the inner-CR case; the hint set is keyed by reason or by reason and detail, and the scope adds neither (per layout-templates §Surface: cli · Component — Primary content block 2: refusal lines and the `unable` column, bullet "Hint line").
- No human-output field added to explain the smaller `text_bytes` of a text that held CR LF pairs (per layout-templates §Surface: cli · IA notes, "Output as a contract").

## Contract bindings
- layouts ↔ architecture [Delivery Confirmation] and security-plan §Input Validation: the bullet "Refusals decided before any request" (layout-templates §Surface: cli · Component — Primary content block 2: refusal lines and the `unable` column) restates the typed-text rule in the words "once its trailing CR and LF characters are removed". Scope §4 names this sentence among the wrap's expected amendments; phase amends no master. Under the rule as scope §1 reads it, the `empty-text` condition and its hint text are unchanged, so whether that sentence needs any rewording is the wrap's reading, not this chunk's build.
- layouts ↔ tests (test-plan §4): the driver-visible outcome a tier above the unit tier asserts is the outcome line, its stream and its exit as fixed in layout-templates §Surface: cli · Output structure — `viola send`.
- layouts ↔ obs (event catalog, `send-issued` / `prompt-submitted` origin): the web readback box fills on the matching driver-origin `prompt-submitted` and a human-origin prompt takes its own tape line (per layout-templates §Surface: web-spa · Signature placement). The chunk's filing of the inner-CR prompt as `driver` is what that placement reads; no web element is built or changed here.

## Acceptance criteria contributions
- (layouts) A `viola send` of a text holding an inner CR or CRLF that is confirmed prints the filled-mirror outcome line on stdout with exit 0, in the same column form as any confirmed send, and no struck-mirror or hint line on stderr (per layout-templates §Surface: cli · Output structure — `viola send`).
- (layouts) A `viola send` of a text of only CR and LF characters still prints the struck-mirror `not-delivered` / `empty-text` line on stderr with exit 13 and the unchanged `empty-text` hint as the last stderr line (per layout-templates §Surface: cli · Component — Primary content block 2: refusal lines and the `unable` column).
- (layouts) Under `--json` the confirmed inner-CR send prints exactly one `ok` document on stdout and nothing on stderr (per layout-templates §Surface: cli · Component — Hero / signature output line (the `viola send` readback mirror)).
- (layouts) The chunk's diff adds no column, word, line or hint to the human output of `viola send` (per layout-templates §Surface: cli · IA notes).
