# layouts extract

## Relevance
partial — no surface is created or changed (the scope says no product behaviour change), but item (5) splits `src/human/board.rs`, which renders the `viola list` human board, and moves `ledger.rs` / `typed.rs` code that feeds `viola verify`'s step counter. So the cli surface's output structure is a preservation constraint. Item (6) ties a test summary to the relied count that the verify layout keys on.

## Constraints
- The `src/human/board.rs` split must leave the human board as layout-templates §Output structure — `viola list` (piped / `NO_COLOR` / `TERM=dumb`) **As built** requires it: one stdout write, then the BAY line, the caption row, the wrapped rows in name order (or the empty line), then the `-- UNWRAPPED - READ-ONLY --` rack only when unwrapped rows exist. Whether the existing tests already pin this byte for byte is research's question.
- The split must keep the column rule of layout-templates §Output structure — `viola list` (piped / `NO_COLOR` / `TERM=dumb`): NAME padded to the longer of 4 and the longest name over both racks, LIVE 5, STATUS 7, WHEEL 23, DIALOG 17, CLI last and unpadded, and no line ending in a space. The width cut applies only to an unwrapped NAME, only when `viola_pty::host_size()` reads a width, and never inside a `\xHH` escape.
- Styling has to survive the split exactly as layout-templates §Component — Primary content block 1: `viola list` strip rows requires it: NAME bold, the `DIALOG` word in the attention token with its kind word plain, a `stale` row dim end to end (with only `DIALOG` amber and not dim on a stale row that has a dialog), and nothing else styled.
- Under layout-templates §Component — Primary content block 1: `viola list` strip rows, `escape_row` keeps covering every control character (DEL and C1 included) in the upstream-derived cells (the recorded CLI version and each unwrapped NAME), so a row never splits.
- The BAY line stays the only header on any verb, plain, with no rule line under it, per layout-templates §Component — Header / banner (the BAY context line).
- In layout-templates §Output structure — `viola verify`, the step counter's `MM` is `LedgerRow::relied()`'s length (24 on Unix, 21 on Windows). Moving `ledger.rs`'s in-file test module (item 5) must not change `LedgerRow::ALL`, `relied()` or the ledger order. Item (6)'s `dialogless_home` summary should be derived from the same `relied()` and not from a literal, so it matches the `stamped <ver>  <n> pass  <m> fail` form.
- The human columns and words are an output contract that LLM drivers read, so a refactor adds, drops and reorders no column, per layout-templates §IA notes.

## Patterns to follow
- Styling stays in the small hand-written SGR module in the `viola` bin (`src/human/style.rs`) with fixed-width padding, per layout-templates §Surface: cli (Tooling context). Concerns split out of `board.rs` keep calling that module and do not grow their own SGR.
- Expression level 0.0 (no SGR, no non-ASCII glyph, no cursor control) applies under `--json`, a pipe, `NO_COLOR`, `TERM=dumb` and a failed Windows VT enable. Level 0.2 applies only on a terminal that takes colour, per layout-templates §Surface: cli (Expression level). A split by concern can separate the plain layout from the colour layer along this line.
- Verify step lines are appended once, plain ASCII, in ledger order, with the summary as the last stdout line, per layout-templates §Output structure — `viola verify`. Moving `src/cmd/verify/typed.rs` code keeps this.

## Anti-patterns to avoid
- No colour or table crate brought in by the split, and no box-drawing borders, per layout-templates §Surface: cli (Tooling context) and §Component — Primary content block 1: `viola list` strip rows.
- No redraw, spinner or progress bar in any moved code: nothing redraws in place, per layout-templates §Surface: cli (Motion tokens row) and §Output structure — `viola verify`.
- A wrapped NAME is never cut, and nothing is truncated when the width is unknown, per layout-templates §Component — Primary content block 1: `viola list` strip rows.

## Contract bindings
- layouts ↔ architecture: the `viola list --json` document's item, `budget` and `skipped` shapes are architecture §Standard Contracts' `list` payload (per layout-templates §Output structure — `viola list --json`). The board split must not change them.
- layouts ↔ security: the colour facts `NO_COLOR` / `TERM` / `COLORTERM` decide only the SGR depth (per layout-templates §Output structure — `viola list` (piped / `NO_COLOR` / `TERM=dumb`) As built). Moving them in the split keeps them out of every log line, error and refusal.
- layouts ↔ design-system: the cli token table (per layout-templates §Surface: cli, Token names) resolves the shared token vocabulary that item (10)'s design-system Brand Identity edits touch. Removing cut web motion values and page components there must leave the cli rows unchanged. Whether layout-templates itself still carries a web leftover beyond its one-line `## Surface: web-spa` deferral is research's question.

## Acceptance criteria contributions
- (layouts) After the `src/human/board.rs` split, `viola list` output is byte-identical to the pre-split output, both piped / `NO_COLOR` and in the styled TTY form, over a board with a wrapped, a stale-with-dialog and an unwrapped row (per layout-templates §Output structure — `viola list` (piped / `NO_COLOR` / `TERM=dumb`)).
- (layouts) The split adds no SGR and no styled element beyond bold NAME, the amber `DIALOG` and the dim `stale` row (per layout-templates §Component — Primary content block 1: `viola list` strip rows).
- (layouts) `viola verify`'s `MM` and the `dialogless_home` summary both equal `LedgerRow::relied().len()` on each OS, with no count literal (per layout-templates §Output structure — `viola verify`).
