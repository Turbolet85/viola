# design extract

## Relevance
partial — no web-spa rendering, tokens, motion or layout in this chunk; the design surface it touches is the cli: `viola run`'s silent passthrough, the `viola verify` step-counter lines that grow with the new ledger rows, and the phraseology words (`not-delivered · input-not-ready`, `unverified-cli`) the gate's verdict feeds.

## Constraints
- design-system §Surface: cli → Colour decision order, rule 2, and the §Brand Identity per-surface expression table (cli `viola run` passthrough row, expression 0.0) require that `viola run` print nothing at all while the child runs: the vt100 feed and the gate add no byte, colour, glyph, cursor control or status line to the human terminal. Whether the run pump already keeps that silence is research's question.
- design-system §Surface: cli → Component Patterns 5 (`verify`) requires every ledger row to print as one static, appended stdout step-counter line, `[NN/MM] <row-id> <description>  pass|fail`. MM is the row count, which grows from six as rows land, and the last line is the summary `stamped <ver>  <n> pass  <m> fail`, with `fail` uncoloured. The new input-box, modal-signature, quiet-period / maximum-wait and confirmation-window rows, and the typed-input probe step, each follow that shape.
- design-system §Anti-Patterns → Per-Surface Bans (cli) bans a spinner, progress bar or live redraw. The typed-input probe's wait for quiet or `turn-ended`, however long, shows only as its step line once the step is done: no elapsed counter and no redrawn line.
- design-system §Color Palette → Semantic Colors (domain status rows `RefusalReason::not-delivered`, `RefusalDetail::input-not-ready`) and §Brand Identity ("Standard phraseology and 'unable' plus a reason") fix the vocabulary: the gate's failing verdict surfaces only as the fixed words `not-delivered` / `input-not-ready`. No synonym or new word is coined (e.g. `screen-busy`, `modal-open`).
- design-system §Color Palette → Semantic Colors (Warning row; `CliVerified::false`) treats an unverified CLI build as a degraded state printed as the word `unverified-cli` (transport only), not as a refusal hue or a new state. The gate's unverified-build fallback stays inside those words.
- design-system §Surface: cli → Platform-Specific Notes (stack traces) and §Anti-Patterns → Per-Surface Bans (cli) ("NEVER print upstream text, paths, pids …", "NEVER print stack traces") require a vt100 parser panic to reach no human stream: no backtrace and no screen excerpt. Its detail goes only to `diagnostics/`.

## Patterns to follow
- The `verify` step-counter line (design-system §Surface: cli → Component Patterns 5) is the project's one progress pattern. A new ledger row extends the count and adds one line, and the summary line keeps its shape.
- The refusal and hint table (design-system §Surface: cli → Component Patterns 2) already fixes the `input-not-ready` hint, `<name> was not ready for input; viola wait <name>, then send again`, and exit 13 (§Surface: cli → Exit-code phraseology). Printing it belongs to the `send` entry (:74). This chunk's gate verdict only has to map onto that existing detail code unchanged.
- The `verify` refusals of exit 1 (design-system §Surface: cli → Exit-code phraseology, row 1) use a fixed `unable: …` message plus a hint, with no path and no pid. If the typed-input probe gains a new start failure (e.g. the PTY could not spawn the CLI), it uses the same form.

## Anti-patterns to avoid
- An optimistic "ready" or "sent" signal before confirmation (design-system §Anti-Patterns → Rejected Defaults, "Optimistic 'Sending…' spinner"). A `ready` gate verdict is not a delivery, so nothing human-facing reads as success from it.
- Colour, glyphs or cursor control under `viola run`, `--json` or non-TTY output (design-system §Anti-Patterns → Per-Surface Bans, cli). This covers the verify probe's own output when it is piped.
- Green `✓` / red `✗` prefixes or emoji on the new verify step lines (design-system §Anti-Patterns → Per-Surface Bans, cli).

## Contract bindings
- design ↔ architecture/core: the `input-not-ready` word in design-system §Color Palette → Semantic Colors (`RefusalDetail::input-not-ready`) and exit 13 (§Surface: cli → Exit-code phraseology) bind to `viola-core`'s `RefusalReason` / detail enum and architecture's refusal contract. Whether that variant already exists in `viola-core` is research's question.
- design ↔ obs: the gate's closed `ready | input-not-ready` outcome (obs-plan `run.readiness_gate` span) uses the same `input-not-ready` word as the human refusal detail (design-system §Color Palette → Semantic Colors), and screen content stays out of both.
- design ↔ tests: the `verify` step-line and summary shapes (design-system §Surface: cli → Component Patterns 5) are what a verify output test asserts once the row count changes.

## Acceptance criteria contributions
- (design) With the gate and the vt100 feed active, `viola run` writes no viola-originated byte to the human terminal: the passthrough is byte-identical to the child's output (per design-system §Surface: cli → Colour decision order, rule 2).
- (design) `viola verify` prints exactly one static `[NN/MM] <row-id> …  pass|fail` stdout line per ledger row, new rows and the typed-input probe included, with MM equal to the compiled row count. The final line reads `stamped <ver>  <n> pass  <m> fail`, with no colour, spinner or redraw (per design-system §Surface: cli → Component Patterns 5; §Anti-Patterns → Per-Surface Bans, cli).
- (design) The gate's failing verdict maps to the existing detail word `input-not-ready` under reason `not-delivered`, and no new refusal word is introduced (per design-system §Color Palette → Semantic Colors, `RefusalDetail::input-not-ready`).
- (design) A forced vt100 parser panic produces no stderr or stdout text on any human stream: no backtrace, screen bytes or path (per design-system §Surface: cli → Platform-Specific Notes, stack traces; §Anti-Patterns → Per-Surface Bans, cli).
