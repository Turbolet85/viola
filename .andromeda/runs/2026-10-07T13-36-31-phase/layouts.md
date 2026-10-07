# layouts extract

## Relevance
partial — the chunk creates and modifies no surface element; it changes WHICH of `viola send`'s existing cli outcome lines a text ending in a newline ends on (scope items 2-3), so only the cli send-output mandates bind; the folded CI red (scope item 14) has no layout surface.

## Constraints
- layout-templates §Surface: cli (Signature placement) requires a confirmed send to end on the `[RB] read back` outcome line on stdout with exit 0, confirmed by the matching `prompt-submitted`; the chunk's newline case must land on that line through that same confirmation, not through a new outcome form. Whether the code already prints it once the claim succeeds is research's question.
- layout-templates §Output structure — `viola send` requires the outcome line to be appended under the TTY-only issue line, never redrawn, with the state word padded to the shared word column and the name in the same column; the newline case gets no line shape of its own.
- layout-templates §Component — Hero / signature output line (the `viola send` readback mirror) requires the `--json` machine view to print only the typed document (`ok` with `submitted_at` + `cursor`, or `refusal` + `detail`) with the typed exit code, no glyph and no hint; the chunk's `ok` result for the newline case takes that shape unchanged.
- layout-templates §Component — Primary content block 2: refusal lines and the `unable` column requires every refusal to carry a `hint:` line directly under it, keyed by reason (or reason · detail), and requires that a hint never quote the sent text or any upstream text. It binds only if the P4 card picks the refuse-with-nothing-typed shape (scope item 3): a new detail then needs its own hint.
- layout-templates §Component — Primary content block 2: refusal lines and the `unable` column fixes the exit codes as the typed tail (13 `not-delivered`, 10 `human-typing`); the chunk's outcome must stop producing exit 13 for a delivered newline send and must not surface exit 10 on the following send, with no new code minted.
- layout-templates §IA notes (under §Surface: cli, "Output as a contract") requires the human columns and words to stay stable because LLM drivers read them, with new fields going into `--json` first; no remedy may add a human column, a state word or an outcome field to the mirror.
- layout-templates §Component — Footer / terminator requires the last printed line to say what happened, to whom and where, with the exit code as the only terminator; a remedy that alters the sent text (scope item 3, second shape) adds no notice line, banner or `done` word after the outcome line.

## Patterns to follow
- The four-word readback vocabulary (`open`, `read back`, `unable`, `unconfirmable`) is closed and shared by both surfaces, per layout-templates §IA notes (under §Surface: cli, "Multi-surface coordination"); the newline case maps onto `read back`, or onto `unable` if the refusal shape is chosen.
- Stream split: results on stdout; the issue line, refusals and hints on stderr, never mixed, per layout-templates §Component — Primary content block 2: refusal lines and the `unable` column (Streams).
- On the web tape a matching driver-origin `prompt-submitted` is folded into its send line and gets no tape line of its own, per layout-templates §Component — Hero / signature section (Trigger events); the chunk's relabel to `origin:"driver"` is what that fold reads. Whether the page already implements the fold is research's question; the chunk builds nothing on the page.
- `human` in the WHEEL cell is the strong-weight reading on the web strip and the WHEEL column word in `viola list`, per layout-templates §Component — Primary content block 1: racks of `<viola-session-row>` strips and §Output structure — `viola list` (human TTY); the chunk's "wheel still `driver`" outcome is the reading both surfaces must keep showing.

## Anti-patterns to avoid
- No spinner, `✓`, `done`, or colour on the mirror, and `turn-ended` is never reported as a send outcome, per layout-templates §Component — Hero / signature output line (the `viola send` readback mirror); the remedy must not confirm the newline send off the turn that ran.
- No hint that names `viola release`, per layout-templates §Component — Primary navigation (verb structure) (Discoverability); a refusal-shape remedy's hint points to another verb or to resending, never to `release`.
- No truncated, redrawn or wrapped outcome line and no extra human line describing the dropped newline, per layout-templates §Output structure — `viola send`.

## Contract bindings
- cli send outcome ↔ architecture [Delivery Confirmation] and the `prompt-submitted` contract: layout-templates §Surface: cli (Signature placement) names the matching `prompt-submitted` as the confirmation; what "matching" means for a text whose last byte is a newline is architecture's to state (the wrap amends it, scope item 12), not this plan's.
- Refusal detail codes ↔ architecture: layout-templates §Component — Primary content block 2: refusal lines and the `unable` column says the machine view carries the cause as a detail code once arch names it; a new refusal detail from the refuse shape is arch's code first, then this plan's hint row and design-system cli pattern 2.
- Hint wording ↔ security: the never-quote-the-sent-text rule of layout-templates §Component — Primary content block 2: refusal lines and the `unable` column ties to the security plan's error-sanitization floor.
- Web refused-box announcement ↔ a11y: layout-templates §Component — Hero / signature section (Trigger events) has `send-refused` read by the polite announcement; it binds only under the refuse shape.
- Outcome lines ↔ tests harness: the chunk's confirmed-outcome case (scope item 5) reads the `ok` document and exit code that layout-templates §Component — Hero / signature output line (the `viola send` readback mirror) fixes.

## Acceptance criteria contributions
- (layouts) A confirmed send of a text ending in a newline ends with `[RB] read back  <name>  <time>  cursor <n>` as the last stdout line, exit 0, and prints no `[/ ] unable` line and no `hint:` line on stderr (per layout-templates §Output structure — `viola send`).
- (layouts) The same send under `--json` prints exactly one `ok` document carrying `submitted_at` and `cursor` on stdout, nothing on stderr, no glyph (per layout-templates §Component — Hero / signature output line (the `viola send` readback mirror)).
- (layouts) If the P4 card picks the refusal shape: the outcome is `[/ ] unable  <name>  <reason>  <detail>` on stderr with its typed exit code, the `hint:` line is the last stderr line, is keyed to that reason · detail, and holds none of the sent text (per layout-templates §Component — Primary content block 2: refusal lines and the `unable` column).
- (layouts) The diff adds no human column, state word or outcome field to `viola send` output; any new datum appears in `--json` only (per layout-templates §IA notes, under §Surface: cli).
