# layouts extract

## Relevance
partial — the chunk adds no surface, but item (5)'s split of `src/human/board.rs` (the `viola list` human render) and of `src/cmd/verify/typed.rs`, and item (7)'s recorded `tools/list` of the no-screen `viola mcp` touch surfaces this plan fixes; item (4) is outside this domain.

## Constraints
- The `viola list` human form is one stdout write in a fixed order (BAY line, caption row, wrapped rows in name order or the empty line, then the unwrapped rack under `-- UNWRAPPED - READ-ONLY --` only when there is one), with fixed column paddings (NAME to the longer of 4 and the longest name over both racks, LIVE 5, STATUS 7, WHEEL 23, DIALOG 17, CLI last and unpadded) and no line ending in a space; the board.rs split must leave every one of these as it is (per layout-templates §Surface: cli, Output structure — `viola list` (piped / `NO_COLOR` / `TERM=dumb`), "As built").
- The styled form is unchanged by a move: NAME bold, only the `DIALOG` word amber with its kind word plain, a `stale` row dim end to end with only `DIALOG` amber and not dim on a stale row with a dialog, nothing else styled, no SGR when colour is off (per layout-templates §Surface: cli, Component — Primary content block 1: `viola list` strip rows).
- Truncation and escaping stay where the plan puts them: only an unwrapped NAME is cut with `...`, only with a known terminal width (`viola_pty::host_size()`, never `COLUMNS`), never inside a `\xHH` escape; `escape_row` covers every control character, DEL and C1 included (per layout-templates §Surface: cli, Component — Primary content block 1: `viola list` strip rows).
- The expression level is 0.0 (no colour, no non-ASCII glyph, no cursor control) under `--json`, a non-TTY, `NO_COLOR`, `TERM=dumb`; a split module carries no new path that writes SGR (per layout-templates §Surface: cli, Expression level).
- The human columns and words are a contract LLM drivers read; a split adds, renames or reorders no column or word (per layout-templates §Surface: cli, IA notes).
- `viola verify`'s output is one plain-ASCII step line per relied row, appended once, then the `stamped <ver>  <n> pass  <m> fail` summary last; if any of it is produced in `src/cmd/verify/typed.rs`, its split leaves those bytes as they are — whether typed.rs writes any verify output is research's question (per layout-templates §Surface: cli, Output structure — `viola verify`).
- `viola mcp` is no screen: its five tools are `send · wait · last · answer · list`, each result the CLI verb's `--json` document less `v`, a refusal `isError: true` with text `refused: <code>`, and no tool, description or result names `release`, `pause`, `link` or `unlink`; a recorded `tools/list` exchange is read against that set (per layout-templates §Surface: cli, Primary screens (commands)).

## Patterns to follow
- Pin a human surface by a literal in a test, as the `HELP` literal in `tests/cli_output_plain.rs` pins `viola --help` (per layout-templates §Surface: cli, Output structure — `viola --help`); a board.rs move is read against the existing board output tests unchanged, not new expectations.
- One token vocabulary resolved on the cli to SGR tokens only through the small hand-written style module (`src/human/style.rs`) — a split keeps styling calls routed through it (per layout-templates §Surface: cli, Tooling context / Token names on this surface).
- The `send` → `src/run/send/*` module-directory shape is the scope's own precedent for M2; layout-wise, the board's render order (BAY → captions → wrapped → rack) is the natural concern seam (per layout-templates §Surface: cli, Component — Header / banner (the BAY context line)).

## Anti-patterns to avoid
- Adding or reordering a human column, or styling any cell beyond NAME / `DIALOG` / `stale`, during a "pure move" (per layout-templates §Surface: cli, IA notes).
- A recorded `tools/list` fixture or its replay case that admits a tool named `release`, `pause`, `link` or `unlink` (per layout-templates §Surface: cli, Primary screens (commands)).

## Contract bindings
- `viola list --json` ↔ architecture §Standard Contracts (the `list` payload: item, `budget`, `skipped` shapes) — the board.rs split must not move the `--json` document's shape (per layout-templates §Surface: cli, Output structure — `viola list --json`).
- `viola mcp` tool results ↔ each CLI verb's `--json` document less `v` (per layout-templates §Surface: cli, Primary screens (commands)); the `list` tool reaches the board through `list::board_document`.
- The cli landmarks of the a11y plan read the same `viola list` / `viola verify` bytes; a byte-identical move keeps them.

## Acceptance criteria contributions
- (layouts) `viola list` prints byte-identical stdout before and after the `src/human/board.rs` split, in both the plain (piped / `NO_COLOR` / `TERM=dumb`) and the styled form, with and without an unwrapped rack, and with a known narrow width cutting an unwrapped NAME (per layout-templates §Surface: cli, Output structure — `viola list` (piped / `NO_COLOR` / `TERM=dumb`)).
- (layouts) `viola list --json` and the exit-21 strict-modes document are unchanged by the split (per layout-templates §Surface: cli, Output structure — `viola list --json`).
- (layouts) If `src/cmd/verify/typed.rs` writes any `viola verify` line, the step lines and the summary line read the same before and after its split (per layout-templates §Surface: cli, Output structure — `viola verify`).
- (layouts) The recorded `tools/list` exchange the item (7) case replays lists exactly the tools `send`, `wait`, `last`, `answer`, `list`, and none named `release`, `pause`, `link` or `unlink` (per layout-templates §Surface: cli, Primary screens (commands)).
