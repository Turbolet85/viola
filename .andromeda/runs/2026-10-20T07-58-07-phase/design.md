# design extract

## Relevance
partial — item (10) edits the design-system master's own Brand Identity web leftovers (wrap/cascade work), and item (5)'s `src/human/board.rs` split touches the cli board's rendering, which must stay byte-identical; items (1)–(4) and (6)–(9) are test, harness and gate work with no design surface.

## Constraints
- Item (10)'s Brand Identity edit keeps every domain anchor and phraseology word the cli surface still uses (strip fields NAME · LIVE · STATUS · WHEEL · DIALOG · CLI, `unknown`, readback, `expired` age, `unverified-cli`, the wheel words): only the web-surface references (the `<viola-atis>` / `<viola-event-feed>` element names, the `web-spa (viola ui)` expression row, the single-source paragraph's web-page motion sentence) are in reach, per design-system §Brand Identity.
- The cli expression rows stay as they stand after item (10): 0.2 on a human TTY (static rows, colour a second cue only) and 0.0 under `--json` / non-TTY / `NO_COLOR` / `TERM=dumb` / `viola run` passthrough, per design-system §Brand Identity (per-surface expression table).
- The removed web text is not lost: the scope keeps it "in the sidecars for 0.2.0", and design-system's sidecar is the one its Decisions Log names, per design-system §Design Decisions Log (`.andromeda/design-system-amendments.md`).
- The `src/human/board.rs` split by concern changes no byte of the board: the colour decision order, the depth choice and the attention / stale / callsign / reset tokens are applied as they stand, per design-system §Surface: cli → Tokens (platform-specific).
- The split board keeps the As-built rendering rules: spans laid over the escaped, padded cells, the board one write, the BAY line / caption / empty-board line / separator unstyled, the unwrapped-NAME width cut only on a known width, `escape_row` over every control character, per design-system §Surface: cli → Component Patterns (1. `viola list`).
- `src/cmd/verify/typed.rs`'s split moves no human output wording: whether that file prints any of the `viola verify` step-counter or stopped-run lines is research's question; if it does, they stay as written, per design-system §Surface: cli → Component Patterns (5. `viola verify` / `viola run`).

## Patterns to follow
- The deferral marker form the master already uses on every web-only section ("A 0.2.0 frontend design input; 0.1.0 renders no page.") is the existing pattern for text that stays as a 0.2.0 input, per design-system §Color Palette / §Typography / §Iconography / §Self-Validation Protocol (4. Token Test).
- The `## Surface: web-spa` section is already reduced to one deferral line; Brand Identity's web references are the remaining body that has not followed it, per design-system §Surface: web-spa.
- The cli's styling is one hand-written SGR module in the `viola` bin (`src/human/style.rs`), adding no dependency; a board split reuses it rather than re-emitting SGR bytes per concern, per design-system §Surface: cli (Toolkit / Framework).

## Anti-patterns to avoid
- No colour on anything but `DIALOG` (amber) and `stale` rows (dim), never either without its word, and no colour, glyph or cursor control under `--json`, non-TTY, `NO_COLOR`, `TERM=dumb` or `viola run` — a board split must not shift a span onto another cell, per design-system §Anti-Patterns → Per-Surface Bans (cli).
- No hardcoded widths or wrapping: the split keeps truncation only where the terminal width is known, per design-system §Anti-Patterns → Per-Surface Bans (cli).

## Contract bindings
- design ↔ a11y: item (10) also re-grounds a11y-plan's `Standard (1)` tier on the terminal surfaces; the cli's not-colour-alone rule (amber `DIALOG` and dim `stale` always beside their word) is the shared ground that tier rests on, per design-system §Surface: cli → Tokens (platform-specific) and §Anti-Patterns → Per-Surface Bans (cli).
- design ↔ tests: the board's styled and plain byte output is what the existing `viola list` cli suites assert; a split by concern is proven by those suites passing unchanged, per design-system §Surface: cli → Component Patterns (1. `viola list`).
- design ↔ wrap cascade: the master-body edit of design-system is the chunk's wrap (cascade) work, never a phase edit, and its superseded text goes to the design sidecar, per design-system §Design Decisions Log.
- Open question for research: besides the four lines the scope names, design-system §Brand Identity's supporting-attention paragraph (the cocked strip's translate offset and duration) and design-system-direction paragraph ("The page is…"), the §Motion heading's `0.3 on web-spa` calibration, and §Self-Validation Protocol's Signature Test (web places 1 and 2, `<viola-transfer>`, `<viola-readback>`) carry web-surface text without the 0.2.0 deferral marker; whether item (10)'s reach covers them is research's and P4's question, not this extract's ruling.

## Acceptance criteria contributions
- After item (10)'s cascade, design-system §Brand Identity names no `<viola-*>` custom element, no `web-spa (viola ui)` expression row and no web-page motion value outside a 0.2.0 deferral marker, and the removed text reads back in `.andromeda/design-system-amendments.md` (per design-system §Brand Identity and §Design Decisions Log).
- After item (10), design-system §Brand Identity's cli expression rows (0.2 TTY, 0.0 machine / passthrough) read unchanged (per design-system §Brand Identity).
- After item (5)'s `src/human/board.rs` split, `viola list`'s styled board (truecolor, 256 and 16-colour depths) and its plain board (`--json`, pipe, `NO_COLOR`, `TERM=dumb`) are byte-identical to the pre-split output, the existing list suites passing unchanged (per design-system §Surface: cli → Tokens (platform-specific) and → Component Patterns (1. `viola list`)).
