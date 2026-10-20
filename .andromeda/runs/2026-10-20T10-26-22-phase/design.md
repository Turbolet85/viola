# design extract

## Relevance
partial — only item (5)'s split of `src/human/board.rs` (the `viola list` human board) touches a design surface, and that split is a pure move. Item (4) (mutation survivors), the `ledger.rs` test-module move, the `viola-mcp` `serde` reading and item (7) (the `tools/list` recording) have no human design surface. Per design-system §Surface: cli §Platform-Specific Notes, `mcp` has none. Whether `src/cmd/verify/typed.rs` prints any of verify's step-counter lines is research's question.

## Constraints
- The board split must leave every styled byte of `viola list`'s human output where it was: bold NAME, the attention token on the `DIALOG` word only (its kind word plain), SGR dim over a whole wrapped `stale` row, NAME dim and bold, every inner span reopening dim after its reset, and a reset after every styled span (per design-system §Surface: cli §Tokens (platform-specific)).
- The board split must keep the colour decision order (`--json`, `viola run`, `hook`/`mcp`, `NO_COLOR`, `TERM=dumb`, a non-terminal stdout, a failed VT enable) and the depth choice (truecolor, then 256-colour, then the 16-colour fallback), each made once per process. Every one of those cases must still write no SGR (per design-system §Surface: cli §Tokens (platform-specific), "Colour decision order").
- The board split must keep the As-built board shape. That covers: the BAY line, caption, empty-board line and separator carrying no SGR; the spans laid over the escaped, padded cells, so the columns are the plain board's; the board as one write; `escape_row` escaping every control character (DEL and C1 included) as `\xHH`; WHEEL padded to 23; and the unwrapped rows after `-- UNWRAPPED - READ-ONLY --`, sorted by name (per design-system §Surface: cli §Component Patterns, pattern 1 "As built").
- The board split must keep the width rule. The width is read only through `viola_pty::host_size()` and `COLUMNS` is never read. Only an unwrapped NAME is cut, ending in `...`, and the cut never splits an escape. A wrapped NAME is never cut. A piped board or `--json` keeps every name whole, and lines never wrap (per design-system §Surface: cli §Width).
- The board split must keep the exit-21 strict-modes refusal pair word for word, the home pair and the instance pair, with nothing from state printed (per design-system §Surface: cli §Component Patterns, pattern 1 "Refusal").
- If any moved `typed.rs` code prints verify's output, the move must keep the static appended `[NN/MM] <row> <text>  pass|fail` lines, the uncoloured `fail`, the `stamped <ver>  <n> pass  <m> fail` summary, and the stopped-run `unable:` / `hint:` pair (per design-system §Surface: cli §Component Patterns, pattern 5).

## Patterns to follow
- Styling stays in the bin's one hand-written SGR module, `src/human/style.rs`, and adds no dependency. A board submodule calls that module and never builds its own escape sequences (per design-system §Surface: cli, "Toolkit / Framework").
- A pure move keeps the k9s strip table exactly as the pattern-1 sample draws it: fixed columns, two spaces between columns, ` - ` inside a field, and ASCII only (per design-system §Surface: cli §Component Patterns, pattern 1; design-system §Iconography, "Separator").

## Anti-patterns to avoid
- NEVER colour anything except `DIALOG` (amber) and `stale` rows (dim), and never either without its word. A split that loses or duplicates a span breaks this (per design-system §Anti-Patterns §Per-Surface Bans, cli).
- NEVER write colour, glyphs or cursor control under `--json`, a non-TTY stdout, `NO_COLOR`, `TERM=dumb` or `viola run` (per design-system §Anti-Patterns §Per-Surface Bans, cli).
- NEVER hardcode widths or wrap at arbitrary points (per design-system §Anti-Patterns §Per-Surface Bans, cli).

## Contract bindings
- The board's not-colour-alone rule (the amber `DIALOG` always beside its word, dim `stale` always beside the word `stale`) binds to the a11y plan's use-of-colour landmark for the cli surface, which a11y owns.
- The board's `escape_row` and the no-path, no-pid refusal wording bind to the security plan's NEVER-log floor and input-escaping rules, which security owns.
- The board's colour-fact reads (`NO_COLOR` / `TERM` / `COLORTERM`, human mode only) bind to the security plan's list of env vars outside `VIOLA_*`, which security owns.

## Acceptance criteria contributions
- (design) After the `src/human/board.rs` split, `viola list` prints byte-identical output before and after the move, both on a colour terminal (bold NAME, amber `DIALOG`, a dim stale row, the dialog-on-stale case) and with no SGR (piped, `NO_COLOR`, `TERM=dumb`, `--json`). The same board tests pass unchanged (per design-system §Surface: cli §Tokens (platform-specific)).
- (design) After the split, the board's width cut and escaping behave as before: only an unwrapped NAME is cut with `...`, no escape is split, and every control character is escaped as `\xHH` (per design-system §Surface: cli §Width; design-system §Surface: cli §Component Patterns, pattern 1 "As built").
- (design) Neither split adds a dependency or a second SGR writer outside `src/human/style.rs` (per design-system §Surface: cli, "Toolkit / Framework").
