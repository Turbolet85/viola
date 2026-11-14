@history_item n=1 marker=2026-10-10-budget-governor
**2026-10-10-budget-governor** — set the `viola release <name> --budget` lines (`budget gate lifted until <end>  wheel <holder>`, the `HH:MM:SSZ` / `YYYY-MM-DD HH:MM:SSZ` override-time form, `budget gate open`) and kept the `budget-paused` hint as the design's string; these are exactly the rows (contract/viola-release-budget/*, contract/cli-verb-output/release-budget, contract/refusal-hints/budget-paused) this removal retires, while the plain `release` line stays.

@history_item n=2 marker=2026-10-10-viola-revive
**2026-10-10-viola-revive** — fixed that no hint names `viola release` (the origin of decision/no-hint-names-viola-release); the rationale (budget-paused and human-typing refusals reach drivers, a driver's `release` is refused `release-from-driver`) loses its budget-paused half here, while the driver `release` guard the scope keeps still carries the rule.

@history_item n=3 marker=2026-10-11-the-board-viola-list
**2026-10-11-the-board-viola-list** — built the board with WHEEL padded to 23 for `unknown - budget-paused` and the five forms (figure rounded down, `read Nm ago` / `read unknown`, bare skip counts, `human - budget-paused`, empty board's BAY line + caption); every budget-born one of these (decision/wheel-column-width-23, window-figure, reading-age, human-holder-budget-paused, BAY-line budget header) leaves or must be re-derived, and WHEEL's width loses its widest-word justification.

@history_item n=4 marker=2026-10-13-t06-38-35-wrap
**2026-10-13-t06-38-35-wrap** — made WHEEL 23, the five board forms and the two `release --budget` lines the founder's own wording («Подтвердить всё», the budget governor's item 4); retiring them now rests on the founder's later ruling «Убрать совсем» (2026-11-08), which supersedes this ratification — the plan must cite that ruling, not an operator word.

@history_item n=5 marker=2026-10-13-owed-ledger-rows
**2026-10-13-owed-ledger-rows** — introduced `statusline-rate-limits` as a relied verify row and the step counter at `LedgerRow::relied()`'s count (contract/viola-verify-output/step-line, summary); removing that row moves `MM` and the `stamped … N pass  0 fail` sample (24 → 23), the boundary the scope's item 6 flags as a possible P4 fork with the next entry.

@history_item n=6 marker=2026-10-18-compact-post-condition-ledger-row
**2026-10-18-compact-post-condition-ledger-row** — the latest verify counter sample, `[NN/24]` and `stamped 2.1.287  24 pass  0 fail`; the sample the master's verify rows carry and that drops to 23 if this chunk retires the rate-limits row.

@history_item n=7 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (verify's relied count on every build) — folded `WRAPPED_HERE` so `relied()` is all 24 rows on every build with the statusline rows relied unconditionally; the precedent for a removal retiring OS-gated rows, and the count this chunk's ledger-row removal changes.

@history_item n=8 marker=2026-10-16-unwrapped-sessions-on-the-board
**2026-10-16-unwrapped-sessions-on-the-board** — built the board's columns NAME · LIVE · STATUS · WHEEL · DIALOG · CLI and the `-- UNWRAPPED - READ-ONLY --` rack (contract/viola-list-table/columns, list-six-fixed-columns); these stay fixed when the budget header and cells leave, so the removal must not move a column.

@history_item n=9 marker=2026-10-17-cli-output-discipline
**2026-10-17-cli-output-discipline** — built the colour layer and width cut: BAY line, caption, empty-board line and separator uncoloured, spans over escaped padded cells in one write, NAME width = max(4, longest wrapped, unwrapped capped at width less 65); removing the BAY line's budget figures and WHEEL's budget words must keep these rows (uncoloured-lines, colour-spans, name-column-width, name-cut, stale) true, and the 65-column constant depends on the other columns' widths including WHEEL 23.

@history_item n=10 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (the stale-row colour witness) — `cross_list_rows_dim_a_stale_row_end_to_end_on_a_terminal` asserts no SGR on the BAY line and the stale row's exact SGR sequence on `ubuntu-latest`; a witness the board change must keep green.

@history_item n=11 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** — closed exit 20's `--json` detail (`-32602` with `data.reason` `release-from-driver` reads that body; other `-32602` reads `invalid params`), made `unknown` (exit 14) a code word or nothing, and kept a code word outside the closed refusal list visible and exiting 14; these back the kept driver `release` guard (wrapper-fault-detail rows) and decide how an old peer's `budget-paused` reply reads once `RefusalReason::BudgetPaused` leaves the closed list (contract/peer-reply-screening/unlisted-code-word).

@history_item n=12 marker=2026-11-13-rows-proposal-rule-work
**2026-11-13-rows-proposal-rule-work** — `~ contract/refusal-hints/refusal-line`: `send` and `answer` print the same hint for the same reason or detail; the picked row this chunk edits when the `budget-paused` hint and refusal leave both verbs.

@history_item n=13 marker=2026-11-08T20-32-52-wrap
**2026-11-08T20-32-52-wrap** (the never-built page's design tokens retired) — the 0.2.0 census retired the domain status colour table with its budget rows, and kept Border Progression's table and premise (C-148, built: yes) naming «Budget gate removed» as its removing chunk; the census rows routed `→ v2-04` (F-08) retire at this chunk's wrap.

@history_item n=14 marker=2026-11-08T20-32-52-wrap
**2026-11-08T20-32-52-wrap** (brand, surface and bans scoped to the CLI) — kept the CLI status guards (refused send folded, nonzero `skipped`, no-spinner `busy`) and the brand direction; decision/skipped-count-stays-visible and the brand's budget decisions (budget-paused-stops-new-sends, readings-carry-their-age, budget-as-ruled-figures, the `budget-paused` word in the fixed vocabulary) are the brand rows this removal retires or narrows.

@history_item n=15 marker=2026-11-12T03-23-49-rows
**2026-11-12T03-23-49-rows** — turned the master into rows (decision · contract · absence · name · overview), each verified against the code; the row handles this chunk's wrap retires (`− {handle}`) or narrows (`~ {handle}`) are these, and the phase itself amends no master.
