# design extract

## Relevance
partial — no token, web-spa or motion work; the design surface touched is the cli surface only, through §3's pure-move split of `src/human.rs` (human output) and §1's `src/cmd/send.rs` `Out<'_>::open` survivor (the send readback mirror's lines); `src/run/send.rs`, the mutation survivors elsewhere, the Windows baseline and the clippy gate carry no design mandate.

## Constraints
- The split of `src/human.rs` must leave every human-output line byte-identical: the strip table and BAY line, its as-built shape (wrapped rows only, uncoloured, WHEEL padded to 23) and its exit-21 `unable:` / `hint:` refusal pair, per design-system §Surface: cli → Component Patterns (pattern 1); the readback mirror lines (`[  ] open`, `[RB] read back`, `[/ ] unable`, `[  ] unconfirmable`) with their padding and the per-reason `hint:` texts, per design-system §Surface: cli → Component Patterns (pattern 2); the `wait` / `last` result lines and the message-mode `\xHH` escaper, per design-system §Surface: cli → Component Patterns (pattern 3); the wheel / gate verb lines, per design-system §Surface: cli → Component Patterns (pattern 4); the `verify` step counter and `revive --list` lines, per design-system §Surface: cli → Component Patterns (pattern 5).
- Stream assignment must survive the move unchanged: results on stdout; `waiting:`, the send issue line, refusals with their `hint:` line directly after, `error:` lines and the `viola ui` launch line on stderr; no `hint:` under `--json`, per design-system §Surface: cli → Tokens (Streams).
- TTY-only context lines (the `[  ] open` issue line of `send`, `waiting:`, the `last` header) print only when the stream is a terminal, per design-system §Surface: cli → Component Patterns (patterns 2 and 3); whether `Out<'_>::open`'s negation at `src/cmd/send.rs:119:12` is that TTY gate is research's question, and a test disposing that survivor asserts the documented TTY / non-TTY behaviour.
- Human output stays ASCII only and is never wrapped; nothing truncates where the width is unknown, per design-system §Surface: cli → Tokens (Width).
- The split adds no colour: as built the board is uncoloured and the colour layer (amber `DIALOG`, dim `stale`, bold NAME, the colour decision order) is owed to the later entry "CLI output discipline", which lands in `src/human.rs`, per design-system §Surface: cli → Component Patterns (pattern 1, As built) and design-system §Surface: cli → Tokens (Colour decision order); the phase boundaries chosen should leave that entry one place to add the hand-written SGR module, per design-system §Surface: cli (Toolkit / Framework).
- Refusal and hint wording, the fixed `unable:` form's limited users and the no-path / no-pid / no-upstream-text rule are unchanged by the move, per design-system §Surface: cli → Tokens (Streams) and design-system §Surface: cli → Component Patterns (pattern 2).

## Patterns to follow
- One writer per verb's documented line shapes, keyed by the verb, matches the cli surface's per-verb component patterns (strip table · readback mirror · wait/last · wheel/gate verbs · verify/ui/run/revive), per design-system §Surface: cli → Component Patterns — a natural seam for splitting `src/human.rs` by phase.
- The shared escapers are two modes, not one per verb: the row mode (every control character as `\xHH`, `\n` / `\t` included) for `list` rows and the message mode (`\n` / `\t` kept) shared by `wait` and `last`, per design-system §Surface: cli → Component Patterns (patterns 1 and 3); a split keeps each mode in one home.
- The exit-code phraseology table is the reference for which first word each typed exit prints, per design-system §Surface: cli → Component Patterns (Exit-code phraseology).

## Anti-patterns to avoid
- No green `✓` / red `✗`, emoji, spinner, progress bar or redrawing output introduced while moving code, per design-system §Anti-Patterns → Per-Surface Bans (cli).
- No colour, glyph or cursor control under `--json`, non-TTY, `NO_COLOR`, `TERM=dumb` or `viola run`, and no mixing of data and messages across stdout / stderr, per design-system §Anti-Patterns → Per-Surface Bans (cli).
- No upstream text, path, pid or serde-sourced chain in an error or hint line, per design-system §Anti-Patterns → Per-Surface Bans (cli).

## Contract bindings
- design ↔ tests: the human-output line shapes are pinned by the tests that read `src/human.rs` / `src/cmd/send.rs` output; the pure-move proof is those tests staying green with unchanged expected strings (design-system §Surface: cli → Component Patterns ↔ test-plan's CLI output checks).
- design ↔ security: the no-path / no-upstream-text rule of hints and errors and the `\xHH` escaping of `list` / `last` output bind to the security plan's NEVER-log floor and output escaping (design-system §Anti-Patterns → Per-Surface Bans (cli) ↔ security-plan §Error Handling).
- design ↔ layouts: the per-verb line layouts are also `layout-templates.md`'s CLI layouts; the layouts distiller owns their column geometry.

## Acceptance criteria contributions
- (design) After the `src/human.rs` / `src/run/send.rs` split, every existing human-output test passes with its expected strings unchanged, and no line moves between stdout and stderr (per design-system §Surface: cli → Tokens (Streams)).
- (design) The split introduces no SGR byte and no non-ASCII byte into any human output; the board stays uncoloured as built (per design-system §Surface: cli → Component Patterns (pattern 1, As built)).
- (design) A test disposing the `Out<'_>::open` survivor fails under the deleted `!` and asserts the `[  ] open` issue line's documented stream and TTY-only condition (per design-system §Surface: cli → Component Patterns (pattern 2)).
