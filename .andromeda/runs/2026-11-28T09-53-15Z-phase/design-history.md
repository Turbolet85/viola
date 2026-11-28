@history_item n=1 marker=2026-11-28-console-scaffold
**2026-11-28-console-scaffold** (Surface: console: 3 rows) — added `+ contract/fleet-console-keys/q`, `+ contract/fleet-console-keys/ctrl-c`, `+ contract/fleet-console-keys/unbound` under the new Surface: console area (the area itself the delegate's PROVISIONAL Q2 answer). Why it matters: the view adds no key beyond these three; any key change belongs to «Console actions», and this area is where the view's new rows (the frame, `LAST`, the cut, `... <n> more`, the redraw cadence) would land.

@history_item n=2 marker=2026-11-28-console-scaffold
**2026-11-28-console-scaffold** (Surface: cli: 5 rows) — `~ decision/list-is-the-board` (now `console` acts on no strip), `~ contract/cli-refusal-line/fixed-unable-form` (names `console`), `+ contract/viola-console-refusals/no-terminal`, `+ contract/viola-console-refusals/home-refused`, and the help board row `list, focus, console`. Why it matters: these are the scaffold's standing promises the view builds on; the view must keep the two refusal pairs and the exit 1 untouched, and its board re-read must not move the home-refused path, where no line is written and the mode is left as found.

@history_item n=3 marker=2026-11-08T20-32-52-wrap
**2026-11-08T20-32-52-wrap** (version census 0.2.0 — brand, surface and bans scoped to the CLI) — the design direction became the one the CLI carries and «the terminal fleet console (matrix#v2-14) inherits»; «no interactive prompt, no TUI mode and no pager» and the spinner / progress-bar / live-redrawing ban were scoped to «a verb's output», the console being no verb's output and its redraw rule left to «Console scaffold». The census also cut web-spa, so the console got a surface of its own. Why it matters: this is the source of `decision/fleet-console-not-verb-output` and `decision/no-interactive-prompt-or-pager`, the rows his answer (a) rewords in this chunk. The view's once-a-second re-read and redraw-on-change rest on this scoping, and the inherited strip-bay direction still binds it: stable slots, amber only on `DIALOG`, `stale` dimmed in place.

@history_item n=4 marker=2026-11-08T20-32-52-wrap
**2026-11-08T20-32-52-wrap** (version census 0.2.0 — the never-built page's design tokens retired) — retired the page tokens, contrast, type, spacing, depth, radius and motion; each section now reads N/A, «the cli's type, spacing and ink the terminal's». Why it matters: this explains the picked areas' N/A lines and `absence/rendered-page`. The console draws on the terminal grid with no tokens to introduce, and its spacing and type follow the cli rows (two spaces between columns, bold only on NAME).

@history_item n=5 marker=2026-11-22-lineage-registry
**2026-11-22-lineage-registry** (Iconography: 2 rows) — `~ decision/list-six-fixed-columns` and `~ contract/viola-list/columns`: the six columns, their captions and their slot order stay fixed, a tree's indent stands inside the NAME cell, and links print apart in the `-- LINKS --` block (F4 «Отступ в NAME везде», F5 «Блок LINKS под доской»). Why it matters: the view's lineage trees reuse this indent-in-NAME form (the `forest` builder), and `LAST` goes after the six without moving a caption. This is the precedent the CARRY `cry-geee3cfh` relies on.

@history_item n=6 marker=2026-11-22-lineage-registry
**2026-11-22-lineage-registry** (Surface: cli: 1 row, row order) — `~ contract/viola-list-table/row-order`: was «Rows are ordered by name»; now «Rows keep the board's name order, each wrapped child drawn directly under its parent». Why it matters: the view draws the board's order, then `... <n> more` (F5). It must reuse this order, not re-sort (`decision/no-auto-sorting-strips`, `decision/stable-session-slots`).

@history_item n=7 marker=2026-11-22-lineage-registry
**2026-11-22-lineage-registry** (Surface: cli: 1 row, NAME width) — `~ contract/viola-list-board/name-column-width`: NAME's column is the widest wrapped NAME cell with its tree indent included, the unwrapped NAME capped at the width less 65. Why it matters: the console's width cut (F4, CLI cut first, ending `...`) has to work alongside the board's own NAME-width rule, which counts the indent.

@history_item n=8 marker=2026-11-22-lineage-registry
**2026-11-22-lineage-registry** (Iconography: 1 row, the ASCII arrow) — `~ contract/cli-transfer-arrow/ascii-arrow`: links also print as `<driver> -> <driven>` lines in `viola list`'s `-- LINKS --` block. Why it matters: whether the console draws the LINKS block is undecided. If it does, the block is plain ASCII `->` with no handoff blue (`contract/cli-colour-tokens/handoff-blue`).

@history_item n=9 marker=2026-10-17-cli-output-discipline
**2026-10-17-cli-output-discipline** — built the colour layer: every NAME bold, the `DIALOG` word amber with its kind plain, a `stale` row dim end to end (NAME dim and bold, on a cocked stale row only `DIALOG` amber and not dim), the BAY, caption, empty and separator lines plain, one write. It also built the width cut: NAME's column capped, an unwrapped NAME cut with `...` and never inside an escape. The width came from stdout's own size, `COLUMNS` not read. Why it matters: this is the source of `contract/cli-colour-tokens/stale`, `callsign`, `contract/viola-list-board/uncoloured-lines`, `decision/truncate-only-unwrapped-name` and `contract/human-tables/width-source`. The scaffold's frame is plain ASCII with no SGR (CARRY `cry-wz66zaxx`), so whether the view takes these SGR spans is a planning decision, and its row cut ending `...` must follow the same never-split-an-escape precedent. It is a trap too: the board truncates only unwrapped NAMEs, while F4 cuts whole rows in the console alone.

@history_item n=10 marker=2026-10-20-t06-58-31-wrap
**2026-10-20-t06-58-31-wrap** (the founder's Epoch 5 batch) — the width from `viola_pty::host_size()` stands on the founder's word, no longer a delegate's PROVISIONAL. Why it matters: the console's 100 ms size poll and the F4 row cut must use this width source and never read `COLUMNS` (`decision/columns-never-read`).

@history_item n=11 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (the cli width read by TIOCGWINSZ alone) — `host_size()` is a `TIOCGWINSZ` ioctl only. Why it matters: this is the one width source the view's cut and its floor case (80 columns, 24 rows) read. Tests can drive it under a PTY.

@history_item n=12 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (the stale-row colour witness and where it now stands) — `cross_list_rows_dim_a_stale_row_end_to_end_on_a_terminal` (`tests/cross_list_rows.rs`) runs on `ubuntu-latest` alone. Why it matters: it is the outer-PTY witness pattern (exact DIM, BOLD, RESET, DIM, RESET bytes) for a stale row. The view should mirror it if its frame carries the stale dim, and must not break it if it reuses the row builder.

@history_item n=13 marker=2026-11-04-linux-and-macos-parity
**2026-11-04-linux-and-macos-parity** — recorded the stale row's end-to-end PTY witness: the row dim from first byte to last, NAME dim and bold, no SGR on the BAY line, caption or separator. Why it matters: these are the exact byte expectations behind `contract/cli-colour-tokens/stale` and `contract/viola-list-board/uncoloured-lines`, which a console frame reusing the board's cells is measured against.

@history_item n=14 marker=2026-10-16-unwrapped-sessions-on-the-board
**2026-10-16-unwrapped-sessions-on-the-board** — STATUS reads `idle` or `busy` only for a wrapped row joined to a `claude agents --json` row, else `unknown`. The `-- UNWRAPPED - READ-ONLY --` rack follows with NAME = `sessionId`, LIVE `live` and `n/a` in WHEEL, DIALOG and CLI. A failed, oversize or malformed agents read prints the board as with no `claude`. Why it matters: this is the source of `contract/viola-list-board/status`, `unwrapped-row` and `failed-agents-read`. The view's «board's words» premise-correction (idle/busy only from the agents spawn) and its agents read at most every 5 s off the loop must keep these readings, a failed read degrading to `unknown`.

@history_item n=15 marker=2026-10-11-the-board-viola-list
**2026-10-11-the-board-viola-list** — built the board: `escape_row` escapes every control character, DEL and C1 included, as `\xHH`; WHEEL padded to 23; bare nonzero skip counts in key order; the empty board's lines; and `list`'s exit-21 whole-board refusal. Why it matters: «rows control-escaped» in the scope is this `escape_row` (`src/human/escape.rs`). The view reuses the board's cells and widths (WHEEL 23, skip counts `skipped 2 - 0 - 1`) and the empty-board line.

@history_item n=16 marker=2026-10-13-t06-38-35-wrap
**2026-10-13-t06-38-35-wrap** — the founder confirmed as his own wording WHEEL 23, the board forms (bare skip counts, the empty board's lines) and `viola list`'s exit-21 whole-board refusal pairs. Why it matters: these forms are his word, so the console reuses them as they stand and must not reword them (scope boundary: no master row reworded beyond his answers).

@history_item n=17 marker=2026-11-14-budget-gate-removed
**2026-11-14-budget-gate-removed** (Brand Identity: 3 rows) — `~ decision/fixed-phraseology-vocabulary` dropped `budget-paused` from the fixed words. Why it matters: the view's vocabulary is this closed set plus the board's own words. `LAST` and `... <n> more` are new console words, and planning must check them against this row and record them.

@history_item n=18 marker=2026-11-14-budget-gate-removed
**2026-11-14-budget-gate-removed** (Surface: cli: 10 rows) — `~ decision/wheel-column-width-23` keeps the width 23 with no widest-word reason left; the budget holder forms, reading age and window figures were removed. Why it matters: WHEEL reads only `driver` · `human` · `unknown` now, and the 23 width stands by ratification alone. The row cut and the 82-column narrowest-row measurement rest on it.

@history_item n=19 marker=2026-11-14-budget-gate-removed
**2026-11-14-budget-gate-removed** (Surface: cli: 4 rows) — `~ contract/viola-list-table/bay-line`: the BAY line carries the skip counts only (`BAY  skipped a - b - c`). Why it matters: this is what the console's title or BAY line can show. The skip-count WARN line on a change only (F7, a named console exception to `decision/skipped-count-stays-visible`) relates to it.

@history_item n=20 marker=2026-11-14-budget-gate-removed
**2026-11-14-budget-gate-removed** (Iconography: 1 row) — `~ contract/cli-separators/space-hyphen-space`: the example became `skipped 2 - 0 - 1`. Why it matters: the separator rule for any in-field label the view prints.

@history_item n=21 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** (the board's one rule on the home) — the scan holds the home to the read rule and `instances/` to the write rule, then each instance's set; a home another user may read refuses the whole board with the home-state pair, with or without an instance. Why it matters: this is the source of `contract/viola-list-board/strict-modes-scan`. The console's once-a-second `board::scan` / `read` meets this refusal live, and planning must decide what a mid-run strict-modes failure does to the frame, given the console's own exit-1 `home-refused` pair at start.

@history_item n=22 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (which pair viola list's refusal prints, and its owner) — recorded the scan's two-rule inconsistency, in which the refusal pair depended on whether an instance existed. Superseded by item 21. Why it matters: it is the trap behind the board's refusal path. A console reusing the board read must keep the settled one-rule reading and not reintroduce the instance-dependent pair.

@history_item n=23 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** (exit 21's document names its cause) — `list`'s refused strict-modes check prints the `instance-unreachable` document with `detail` `strict-modes-failed`, the board's only refusal. Why it matters: `decision/list-refuses-board-on-strict-modes-failure` is pinned byte for byte and shared with the MCP list tool through the board reader. Changing the board read model (the unserialised last-activity time) must leave this refusal and `list --json` untouched.

@history_item n=24 marker=2026-10-04-wait-and-last
**2026-10-04-wait-and-last** — message mode: every control character but `\n` and `\t` prints as `\xHH`. Why it matters: this is the `except` of `decision/list-rows-escape-all-controls`. The console's rows must use the row mode (`escape_row`, `\n` and `\t` escaped too), not the message mode, so every drawn row stays one line.

@history_item n=25 marker=2026-11-16-granular-api-error-cause
**2026-11-16-granular-api-error-cause** — `~ decision/api-error-turn-end-shown-as-turn-end`: the trace became one closed cause word of nine in `--json` and MCP `wait`, with STATUS `idle` and no distinct status, colour or hint. Why it matters: the console's STATUS cell must show an API-error turn end as plain `idle`, and its `LAST` cell must give it no mark (guard `list_status_reads_idle_after_an_api_error_turn_end`).

@history_item n=26 marker=2026-11-15-api-error-turn-end
**2026-11-15-api-error-turn-end** — `+ decision/api-error-turn-end-shown-as-turn-end`, ratified 2026-11-17 and 2026-11-27. Why it matters: it is the origin of that rule. The view adds no new state word for it, in line with «the board's words» and no new vocabulary.

@history_item n=27 marker=2026-11-24-viola-up
**2026-11-24-viola-up** (Surface: cli: 6 rows, 1 of 2) — `~ decision/list-is-the-board` gained «`up` acts on the whole fleet or on the instances it names». Why it matters: it is the prior edit of the row the scaffold extended with `console`. The view keeps the console acting on no strip, and actions stay «Console actions»'s.

@history_item n=28 marker=2026-10-20-epoch-5-cleanup
**2026-10-20-epoch-5-cleanup** — Brand Identity: the cocked strip keeps its slot and shows `DIALOG` plus the kind, amber on the CLI; its translate and band motions were moved out to a frontend input; the per-surface table holds the cli rows only. Why it matters: this is the source of `decision/dialog-pending-cocked-strip` as the console inherits it. The cocked strip stays in place, with no motion (`decision/cli-has-no-motion`).

@history_item n=29 marker=2026-09-28-cli-output-tokens
**2026-09-28-cli-output-tokens** — clap built without `color`, so every styled byte of human output is viola's own SGR module's. Why it matters: it backs `decision/hand-written-sgr-module`. Any styling the view adds goes through that module, with no new dependency.

@history_item n=30 marker=2026-09-29-t15-07-57-wrap
**2026-09-29-t15-07-57-wrap** — moved the Decisions Log into the body, among it the Squint Test's «`open` and `unconfirmable` look identical by design». Why it matters: this is the origin of `decision/squint-test`'s except clause. The console frame is held to the squint test before it is presented.
