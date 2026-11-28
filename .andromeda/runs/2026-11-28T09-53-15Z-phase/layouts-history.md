@history_item n=1 marker=2026-10-11-the-board-viola-list
**2026-10-11-the-board-viola-list** — set the board's built widths (NAME the longer of 4 and the longest name, LIVE 5, STATUS 7, WHEEL 23, DIALOG 17, CLI unpadded, no trailing space), the `unknown` WHEEL / CLI and `DIALOG unknown` cells, and `escape_row` covering DEL and C1 on every upstream cell. These are the board's own words and widths, and the console is to reuse them and its row cut rests on them. Every cell the console draws must still pass `escape_row` (the rows-control-escaped clause).

@history_item n=2 marker=2026-10-13-t06-38-35-wrap
**2026-10-13-t06-38-35-wrap** — WHEEL 23 and the board's planning forms became the founder's own word (they had stood on the operator's). The console's seventh `LAST` cell goes after these fixed widths. A WHEEL holder word that the console draws must not change WHEEL's founder-ruled 23.

@history_item n=3 marker=2026-10-16T14-58-10-wrap
**2026-10-16T14-58-10-wrap** — added the «Shared vocabulary» parity line: the six captions, their words and slot order are fixed, and a future surface reuses them. This is where `decision/shared-captions-fixed` and CARRY `cry-geee3cfh` come from. `LAST` may follow the six only with their words and slots unchanged.

@history_item n=4 marker=2026-10-16-unwrapped-sessions-on-the-board
**2026-10-16-unwrapped-sessions-on-the-board** — built the board's one `claude agents --json` read after the board read: a joined wrapped row reads STATUS `idle` or `busy`, else `unknown`, and a failed, oversize or malformed read prints the board as if there were no claude. It also built the unwrapped rack (`-- UNWRAPPED - READ-ONLY --`, NAME the escaped `sessionId`) and measured NAME's width over both racks. These are the sources behind `contract/viola-list-board/status-join` and `agents-read-failure`. The console's agents read (at most once every 5 s, off the loop) must keep the same join and the same failure fallback.

@history_item n=5 marker=2026-10-17-cli-output-discipline
**2026-10-17-cli-output-discipline** — built `viola list` styled on a TTY (bold NAME, amber DIALOG, dim stale row) and uncoloured under `--json`, a pipe, `NO_COLOR` or `TERM=dumb`. It also built the unwrapped NAME cut: the width comes from `viola_pty::host_size()`, never `COLUMNS`; the cut is the column less 3 plus `...`, never inside an escape; with no known width nothing is cut. This is the board's only truncation precedent. The console's F4 row cut (at the width, ending `...`, never inside an escape, cells after CLI and then CLI cut first) is a new rule beside it and must not change the board's cut. The console stays plain ASCII with no SGR.

@history_item n=6 marker=2026-10-20-t06-58-31-wrap
**2026-10-20-t06-58-31-wrap** — the founder's Epoch 5 batch ratified the board's width from `host_size()` and the board's other delegate marks. The width source and the board forms the console reuses are therefore the founder's, not provisional.

@history_item n=7 marker=2026-10-17-mcp-server-for-drivers
**2026-10-17-mcp-server-for-drivers** — made `viola mcp`'s tools the driver's machine view of the verbs: each success is the CLI verb's `--json` document less `v`. This underlies `contract/mcp-tools/list` and `contract/mcp-tool-results/success`. The console's unserialised last-activity time must stay out of the `list --json` item and therefore out of the MCP list item (F2).

@history_item n=8 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (the Windows VT enable leaves the cli colour surface) — the cli Tooling context became clap, std `IsTerminal` and the hand-written SGR module with its fixed-width padding, nothing else. Its uncoloured conditions dropped the VT clause. This backs `decision/hand-written-sgr-styling` and `decision/std-isterminal-tty-detection`: the console's padding and cut must not pull in a table, colour or terminal crate.

@history_item n=9 marker=2026-11-08T20-32-52-wrap
**2026-11-08T20-32-52-wrap** — the 0.2.0 census removed the web surface and gave the keyboard-driven terminal fleet console (matrix#v2-14) a surface of its own. It reworded the flat-verbs rule to «no interactive prompt and no pager in a verb, the fleet console being no verb's output» (the source of `decision/no-prompt-or-pager`) and «Shared vocabulary» to «the terminal fleet console reuses the six captions». The founder's answer (a) rewords that same no-prompt-or-pager sentence this chunk.

@history_item n=10 marker=2026-11-12T03-23-49-rows
**2026-11-12T03-23-49-rows** — turned every area of the master into rows verified against the code. Every picked row comes from this conversion. Any reword of a row this chunk lands (the founder's four row edits) goes through row amendments, not prose.

@history_item n=11 marker=2026-11-14-budget-gate-removed
**2026-11-14-budget-gate-removed** (1 of 2) — narrowed `contract/viola-list-json/document` to «its item and skipped shapes» (the `budget` member gone) and the BAY line to `BAY  skipped a - b - c`. The list payload is exactly `{items, skipped}`, and the console's last-activity field must not join it (F2). The skip count that the console's WARN line reports on a change is this `skipped` count.

@history_item n=12 marker=2026-11-22-lineage-registry
**2026-11-22-lineage-registry** (Surface: cli: 4 row(s), 1 of 2) — reworded `decision/board-ordered-by-name` to the tree order (each child directly under its parent, indented, a child that sorts before its parent drawn after it, no second sort). It reworded `contract/viola-list-board/column-widths` so NAME is the widest NAME cell with its tree indent. The console's «lineage trees» reuse exactly this `forest` order and indent, with no second tree builder. The console's NAME width must also count the indent.

@history_item n=13 marker=2026-11-22-lineage-registry
**2026-11-22-lineage-registry** (Surface: cli: 2 row(s), 2 of 2) — added `contract/viola-list-board/tree` (two spaces per depth, a row whose parent has no wrapped row is a root, cycle members drawn once, from the first in order) and the `-- LINKS --` block. These are the tree rules the console's lineage view inherits, and the cycle case is a trap for the console's frame model.

@history_item n=14 marker=2026-11-22-lineage-registry
**2026-11-22-lineage-registry** (Surface: cli: 1 row(s)) — reworded `decision/no-link-column`: the six columns, captions and slot order stay unchanged, with no link column, and links show only in the verbs and the board's LINKS block. This is the precedent for the board gaining information without a new column. The console's `LAST` breaks it as a seventh cell, a console-only exception (F2) that must not reach the board.

@history_item n=15 marker=2026-11-24-viola-up
**2026-11-24-viola-up** (Surface: cli: 3 row(s), 2 of 2) — reworded `decision/board-vs-strip` to name `viola up` as a fleet verb, which is the row the console scaffold later extended. It was provisional on a delegate's word until the ratification at 2026-11-27T19-07-11Z-wrap.

@history_item n=16 marker=2026-11-27T19-07-11Z-wrap
**2026-11-27T19-07-11Z-wrap** (ratified — 2026-11-22-lineage-registry 1 of 2, and 2026-11-24-viola-up 2 of 2) — the operator's own word ratified the provisional marks on the tree-order, column-width and board-vs-strip rows. The rows the console's lineage view rests on are no longer provisional.

@history_item n=17 marker=2026-11-28-console-scaffold
**2026-11-28-console-scaffold** (Surface: cli: 4 row(s)) — added `contract/cli-verbs/console` (no argument, the global `--home` only, no `--json`, exit 0 on quit, interrupt or end of input, exit 1 on a refusal or an internal error). It put `console` in the help's `board:` row and made `decision/board-vs-strip` name `console` as strip-less. The view must keep this verb contract: no new argument, no `--json`, the same exits.

@history_item n=18 marker=2026-11-28-console-scaffold
**2026-11-28-console-scaffold** (Surface: console: 2 row(s)) — added `decision/console-own-surface` (alternate screen, cursor hidden and restored on every end, plain ASCII, no colour) and `decision/console-frame-floor` (80x24: the whole frame from a cleared top-left, `viola console` on row 1 and `q quit` on the last row; below the floor one size line; an unread size counts as 0x0). Both are PROVISIONAL on the delegate viola-overseer's word (inputs#I8), so no founder's word stands on them yet. The view's 22 rows between title and key line, the `... <n> more` line and the ASCII-only cells all have to fit this floor.
