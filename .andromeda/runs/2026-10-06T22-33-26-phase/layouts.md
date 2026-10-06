# layouts extract

## Relevance
partial — W1's human and `--json` rendering of `viola send` is the cli surface's signature line (layout-templates §Surface: cli); W2, W3 and W4 place nothing on a surface, and the web-spa readback is outside this chunk by scope.md's Boundaries.

## Constraints
- Surface and region: the chunk renders only on the cli surface, in `viola send`'s outcome line (the second of the mirror's two components); layout-templates §Surface: cli (Signature placement) requires the `unconfirmable` outcome on stdout with exit 0, the box drawn open and never filled. Whether `src/human.rs` / `src/cmd/send.rs` already hold a writer for it is research's question.
- Column structure: layout-templates §Component — Hero / signature output line (Layout) requires the fixed four-character box first, one space, the state word padded so all four state words share one width, the two-space gutter, NAME, then the outcome fields, with the unconfirmable note in the outcome-field position. §Output structure — `viola send` requires the name to start in the same column on every mirror line. Whether the current padding width already admits the longest word is research's question.
- Append-only, two streams: §Output structure — `viola send` requires the issue line to stay and the outcome to be appended under it, never redrawn; the issue line is stderr and TTY-only, and without it the outcome line alone carries the mirror. §Component — Primary content block 2 (Streams) requires results on stdout and the issue line, refusals and hints on stderr, never mixed.
- Not a refusal: §Component — Primary content block 2 (Hint line) places `hint:` lines under refusals only, and §Component — Footer / terminator requires the last printed line to say what happened with the exit code as the terminator. The plan's unconfirmable example carries no hint and no stderr outcome line, so the unconfirmable outcome line is the last line printed.
- Machine view: §Component — Hero / signature output line (Machine view) requires `--json` to print only the typed document (the `ok` with `confirmed:false` and `unconfirmable` among its three forms), with the typed exit code and no glyph, no issue line and no hint; §Component — Primary content block 2 (Exit codes as the typed tail) requires no stderr line under `--json`.
- Expression floor: §Surface: cli (Expression level) requires 0.0 on `--json`, non-TTY, `NO_COLOR` and `TERM=dumb` (no colour, no non-ASCII glyph, no cursor control), and §Component — Hero / signature output line (Colour) requires the mirror to carry no colour on a TTY either.
- Human output is a contract: §IA notes (Output as a contract; Multi-surface coordination) fixes the readback vocabulary at four words shared with the web and sends new fields to `--json` first. The `/clear` outcomes therefore add no fifth state word and no new human column.

## Patterns to follow
- The three worked `viola send` transcripts in §Output structure — `viola send` are the column-for-column shape for the human assertions in `tests/cli_send.rs`: open issue line, then one of the filled, struck or unconfirmable outcome lines.
- The unconfirmable box is the open drawing reused, told apart from the issue line by its word alone; this mirrors the web's unconfirmable state as §Component — Hero / signature section (anatomy) describes it, so the cli needs no new glyph.
- A listed command whose post-condition does not arrive in the window, if architecture rules it a refusal, takes the existing struck form: `[/ ] unable` padded into the word column on stderr with its typed exit, then its `hint:` line keyed by reason or reason · detail (§Component — Primary content block 2, Line form / Hint line / Exit codes).
- `viola verify`'s output stays as §Output structure — `viola verify` gives it: one step line per ledger row with `MM` at 17 and the `stamped` summary last. W3's fake-agent variant and its red/green control add no human line and move no count.

## Anti-patterns to avoid
- Drawing the unconfirmable outcome as anything but the open box and its word: no fill, no new bracket glyph, no colour, no spinner, check mark or `done`, and never `turn-ended` reported as a send outcome (§Component — Hero / signature output line, Colour / What never happens).
- Routing the unconfirmable outcome as a refusal: stderr, a non-zero exit, or a `hint:` line under it (§Surface: cli, Signature placement; §Component — Primary content block 2).
- Building the web readback's unconfirmable state or its tape and marker triggers here: §Surface: web-spa (Signature placement) and §Component — Hero / signature section describe them, and scope.md's Boundaries owe them to the Epoch 8 entries.

## Contract bindings
- layouts ↔ architecture (`send` wire contract, [Delivery Confirmation]): the machine view prints architecture's typed document unchanged; the human line is a rendering of it. Three points the cli section leaves to that contract, for P2 to read and P3 to close:
  - the confirmed `/clear` line: §Surface: cli (Signature placement) names the filled outcome's trigger as the matching `prompt-submitted` only; the `session-start` cause `clear` trigger is stated only for the web (§Surface: web-spa, Signature placement). Which outcome fields a confirmed `/clear` prints on the cli (the time's source, `cursor`) is not stated in the cli section;
  - the unconfirmable note: the plan shows one note, worded for a command with no measured post-condition. Whether the same note serves `/clear` on an unverified CLI version is not stated; a second note is a human-contract change under §IA notes;
  - `cursor`: the wire shape scope.md cites carries `cursor` on the unconfirmable `ok`, while the plan's human unconfirmable line shows none and its filled line shows one. The plan's example is the requirement for the human line; `cursor` reaches a driver through `--json`.
- layouts ↔ design-system (readback vocabulary, cli patterns): design-system.md is the authority for the words and the box drawing; this plan only places them (§Surface: web-spa, token note). A wording change belongs to design, not to this chunk's writer.
- layouts ↔ tests (test-plan §6 Path 2 `local`): the stream and exit placement above is what `send_window_local_command_is_not_presumed_delivered`'s replacement asserts on the human and `--json` views.
- layouts ↔ a11y: none here. The cli has no focus order, and the web readback's a11y verdicts are Epoch 8's by scope.md.
- W2 (readiness-gate card), W3 (guard control) and W4 (two comments): no layout binding beyond the `viola verify` line count staying at 17.

## Acceptance criteria contributions
- (layouts) A `viola send` of a ledger-listed local command with no measured post-condition prints its outcome line on stdout beginning with the open box and the word `unconfirmable`, exits 0, and puts no `unable` or `hint:` line on stderr (per layout-templates §Surface: cli, Signature placement; §Output structure — `viola send`)
- (layouts) On the unconfirmable outcome line the name starts in the same column as on the read-back and unable outcome lines, the state word padded to one shared width and followed by the two-space gutter (per layout-templates §Component — Hero / signature output line, Layout)
- (layouts) With stdout and stderr not a terminal, the unconfirmable outcome is the only mirror line printed, with no issue line, no SGR sequence and no non-ASCII byte (per layout-templates §Output structure — `viola send`; §Surface: cli, Expression level)
- (layouts) Under `--json` the unconfirmable outcome is exactly one typed document on stdout, with no box glyph, no hint and nothing on stderr, exit 0 (per layout-templates §Component — Hero / signature output line, Machine view; §Component — Primary content block 2, Exit codes as the typed tail)
