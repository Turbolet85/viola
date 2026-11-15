@history_item n=1 marker=2026-10-13-owed-ledger-rows
**2026-10-13-owed-ledger-rows** — the `viola verify` worked example went to 21 relied rows, adding `[19/21] statusline-override`, `[20/21] statusline-shell`, `[21/21] statusline-rate-limits`, and `MM` became `LedgerRow::relied()`'s length; the three statusline rows were relied only where the status line is wrapped (the founder's cut). This is the origin of the two interception rows this chunk removes (`statusline-override`, `statusline-shell`); the verify step-line / summary / exit-0 rows (contract/viola-verify-output/*) must drop them and the relied count go 23 → 21.

@history_item n=2 marker=2026-10-14-resume-and-compact-ledger-rows
**2026-10-14-resume-and-compact-ledger-rows** — verify example at 22 relied rows ending `resume-session`; the Windows relied set skipped the three statusline rows. Shows the ledger order the statusline rows sit inside (after `paste-newline`, before `resume-session`), which the wrap's wireframe amendment must re-count when they leave.

@history_item n=3 marker=2026-10-16-unwrapped-sessions-on-the-board
**2026-10-16-unwrapped-sessions-on-the-board** — verify at 23 rows (`[23/23] agents-join`), the failing example `21 pass  2 fail`, `MM` 23/20; also revive's fifth pair and the board's unwrapped rack. Precedent for the verify wireframe/usage counts this chunk's removal re-counts; the revive passthrough (contract/cli-verbs/revive) is where the status-line override was also written at start.

@history_item n=4 marker=2026-10-18-compact-post-condition-ledger-row
**2026-10-18-compact-post-condition-ledger-row** — verify at 24 relied rows ending `local-command-compact`, failing example `22 pass  2 fail`. The ledger-order end-row and the count format the step-line / summary rows still follow; after this chunk the order loses the two statusline ids but still ends at `local-command-compact`.

@history_item n=5 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** — (verify's relied count and stop on one OS) `MM` became all 24 rows on every build, the Windows set that skipped the three statusline rows gone; (the Windows VT enable leaves) the plain-output conditions lost the VT clause; (per-OS help wording) the usage line and `viola <verb> --help` re-worded. Precedent for a removal re-counting `relied()` and re-wording contract/viola-verify-output, decision/plain-output-modes and contract/viola-help/usage-line / contract/viola-verb-help/usage with claims kept.

@history_item n=6 marker=2026-11-14-budget-gate-removed
**2026-11-14-budget-gate-removed** — (1 of 2, 2 of 2) the sibling removal in this Epoch: `statusline-rate-limits` left with it, and it amended decision/board-planning-forms-wording, contract/bay-line/cell-order, contract/viola-list-json/document (the `budget` member gone) and retired the release `--budget` rows. The direct precedent for this chunk's shape (rows `~`/`−` at the wrap, `v` stayed 1 on the founder's ruling for that removal only) and the source of three picked rows' current wording, which this chunk must leave as they stand.

@history_item n=7 marker=2026-09-28-capability-ledger-and-viola-verify
**2026-09-28-capability-ledger-and-viola-verify** — established the `viola verify` output structure: step lines, the `stamped …  n pass  m fail` summary, a failing-row run exiting 1 with the stamp still written, and verify's `unable:`/`hint:` pairs. The base of contract/viola-verify-output/{step-line,summary,failing-row,exit-1} that the removal re-counts but must not otherwise change.

@history_item n=8 marker=2026-10-13-paste-newline-ledger-row
**2026-10-13-paste-newline-ledger-row** — `MM` stopped being a literal and became a rule (`LedgerRow::ALL.len()`, later `relied()`): one step line per row in ledger order. Why removing rows needs no new wording rule, only the count and the order list.

@history_item n=9 marker=2026-10-13-interrupted-verify-cleanup
**2026-10-13-interrupted-verify-cleanup** — the summary closes only a completed run; a stopped run prints the fixed pair and no summary, exit 1. Source of contract/viola-verify/summary and the stop clause of contract/viola-verify-output/exit-1, rows the removal must keep intact.

@history_item n=10 marker=2026-10-10-viola-revive
**2026-10-10-viola-revive** — `viola revive` added as passthrough like `viola run`, its start shared with run, and `setup:    run, revive, verify, plugin install` in help. The start whose status-line override and `--settings` leave (contract/cli-verbs/revive, contract/cli-verbs/run, contract/viola-run/start-output must stay "prints nothing of its own").

@history_item n=11 marker=2026-10-17-cli-output-discipline
**2026-10-17-cli-output-discipline** — `viola --help` built as the grouped table over served verbs only, the `HELP` literal pin, the usage override literal, `viola <verb> --help` clap's; the board's plain/styled conditions. Source of contract/cli-verbs/help, contract/viola-help/{verb-groups,usage-line}, contract/viola-verb-help/usage and decision/plain-output-modes; the hidden `hook statusline` arm leaving must not move the help table.

@history_item n=12 marker=2026-10-17-mcp-server-for-drivers
**2026-10-17-mcp-server-for-drivers** — `viola mcp` is a hidden verb in no help row with the `HELP` literal unchanged. Precedent for hidden verbs (`hook`, incl. the `hook statusline` arm) living outside the help table: removing the arm changes no help row.

@history_item n=13 marker=2026-10-17-optional-plugin-install
**2026-10-17-optional-plugin-install** — `setup:` row reads `run, revive, verify, plugin install`, `help_lists_only_served_verbs` pins it. Source of the picked verb-groups row's `setup:` wording.

@history_item n=14 marker=2026-10-26-explicit-session-links
**2026-10-26-explicit-session-links** — (the `handoff:  link, unlink` help row built) help gains its fifth row, the wireframe and as-built rows matching. Source of contract/viola-help/verb-groups and contract/cli-verbs/help as they stand.

@history_item n=15 marker=2026-10-11-the-board-viola-list
**2026-10-11-the-board-viola-list** — `viola list` as built, its uncoloured form and `--json` document. Origin of contract/cli-verbs/list and contract/viola-list-json/document; the board's `"statusline"` literal (`board.rs:784`) the scope flags must be read against these rows.

@history_item n=16 marker=2026-10-13-t06-38-35-wrap
**2026-10-13-t06-38-35-wrap** — the board's planning forms ratified as the founder's wording. Origin of decision/board-planning-forms-wording (later narrowed by the budget removal); a ratified wording this chunk must not touch.

@history_item n=17 marker=2026-10-16T14-58-10-wrap
**2026-10-16T14-58-10-wrap** — `list --json`'s shapes became the `list` payload contract (no longer `/api/sessions`); the web surface cut to a stub. Source of contract/viola-list-json/document's payload-contract clause and the web absence's lineage.

@history_item n=18 marker=2026-11-08T20-32-52-wrap
**2026-11-08T20-32-52-wrap** — 0.2.0 census: no web surface (absence/web-surface), Linux the one target, flat verbs with no interactive prompt or pager. The census that maps v2-05's layout rows; context for the absences picked.

@history_item n=19 marker=2026-11-12T03-23-49-rows
**2026-11-12T03-23-49-rows** — the master became rows (226), each verified against the code, guards re-pointed and relied rows kept/cut. Why the picked rows exist as handles and why the wrap amends them by `~`/`−` row entries, not prose.

@history_item n=20 marker=2026-10-04-wait-and-last
**2026-10-04-wait-and-last** — the one `cli` catch site printing `error: internal error`, verify among its verbs. Context for contract/viola-verify-output/exit-1's fail paths, which the removal of verify's statusline probe must not alter.
