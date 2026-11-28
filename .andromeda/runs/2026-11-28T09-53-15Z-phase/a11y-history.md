@history_item n=1 marker=2026-11-28-console-scaffold
**2026-11-28-console-scaffold** (1. A11y Scope Summary: 2 rows) — `~ decision/static-linear-output` gained the named console exception «redraws its frame when the terminal's size changes» and `~ decision/cli-not-keyboard-interactive` «reads keys from stdin in raw mode», on the operator's «Именное исключение». This chunk's F1 widens exactly these console exceptions to «…or the fleet it shows changes»: the view's once-a-second board re-read with a redraw on a changed row is a redraw the standing exception does not cover until the wrap lands his edit.

@history_item n=2 marker=2026-11-28-console-scaffold
**2026-11-28-console-scaffold** (5. Keyboard Navigation: 1 row) — `~ decision/cli-no-interactive-input` gained the console exception «reads keys from stdin in raw mode and redraws on a resize». F1 names this row too, its redraw clause to gain the fleet-change trigger; the keys stay the scaffold's (`q`, Ctrl-C, «unbound»), so the raw-mode clause is unchanged.

@history_item n=3 marker=2026-11-28-console-scaffold
**2026-11-28-console-scaffold** (1. A11y Scope Summary: 1 row, PROVISIONAL) — `~ contract/log-identity/process` added `process:"console"` with no instance for `viola console`, on a delegate word. The board read's and agents read's lines (`parse-rejected`, `state-recovered`, the `agents-probe` pair) and the skip-count WARN this chunk writes into the console's role file log under that tag; the mark is still the delegate's.

@history_item n=4 marker=2026-11-28-console-scaffold
**2026-11-28-console-scaffold** (8. Cognitive Accessibility: 2 rows) — `~ contract/cli-fixed-message/form` named «viola console's two refusals» as fixed `unable:`/`hint:` pairs and `~ decision/strict-modes-refusal-before-any-write` added console to its exit-1 list. The view re-reads the board every second inside a console that already passed its home check at start; a mid-run strict-modes failure of the board read is not covered by these rows and must not invent a third refusal form silently.

@history_item n=5 marker=2026-11-22-lineage-registry
**2026-11-22-lineage-registry** (1. A11y Scope Summary and 4. ARIA Patterns & Roles: 3 rows) — `~ contract/viola-list-board/order` now draws wrapped rows as a tree then the `-- LINKS --` block; `~ decision/terminal-structural-equivalents` and `~ decision/cli-structural-landmark-equivalents` gained the plain-space NAME indent of tree depth and the `-- LINKS --` separator (his F4 «Отступ в NAME везде», F5 «Блок LINKS под доской»). The console's lineage trees must reuse that forest and its plain-space indent as the depth carrier, the structural equivalent the plan names, not a second tree builder or a glyph.

@history_item n=6 marker=2026-11-08T20-32-52-wrap
**2026-11-08T20-32-52-wrap** (version census 0.2.0) — retired the frontend stubs: no web or graphical frontend, the terminal fleet console (matrix#v2-14) taking its a11y scope at the route entry «Fleet a11y verification»; §5 names the console's keyboard reach as that entry's; the P6 cli mirror's caption order became «which the terminal fleet console reuses». This is why the console's keyboard/no-trap/SC verification stays out of this chunk (owned by `chk-722s5r7n`) and why the six captions are bound to the console with `LAST` only after them.

@history_item n=7 marker=2026-10-11-the-board-viola-list
**2026-10-11-the-board-viola-list** — pinned the caption order NAME · LIVE · STATUS · WHEEL · DIALOG · CLI by the `CAPTION` literal (`list_human_board_is_the_literal_rows`, `board_row_caption_and_default_row_are_the_literals`), held the row escape (ESC, C1, DEL, `\n`, `\t` → `\xHH`) by unit cases, and made each board state carried by its word alone. The console's six captions must come from that same literal (`decision/list-captions-fixed`, `contract/viola-list-caption/order`); a seventh `LAST` caption must not alter the pinned six or their slots.

@history_item n=8 marker=2026-10-16-unwrapped-sessions-on-the-board
**2026-10-16-unwrapped-sessions-on-the-board** — built the `-- UNWRAPPED - READ-ONLY --` separator and rows (NAME the `sessionId` through `escape_row`, `n/a` in WHEEL, DIALOG and CLI), the one `claude agents --json` read (direct spawn, `take(MAX_FRAME + 1)`, 5 s kill) that fails open, its bound on viola's own child. The console's agents read at most every 5 s off the drawing loop reuses this read and its fail-open (`decision/claude-agents-read-fails-open`) and its bound in the time-bound lists; idle/busy exist only through it.

@history_item n=9 marker=2026-10-17-cli-output-discipline
**2026-10-17-cli-output-discipline** — on a narrow terminal an unwrapped NAME is cut with `...`, never inside an escape, the width from `viola_pty::host_size()`; each state still carried by its word beside any SGR. Precedent and trap for the console's F4 row cut at the terminal width ending `...`, never inside an escape: `decision/name-truncation` / `decision/unwrapped-name-truncation` say a wrapped NAME is never cut, so the console's whole-row cut (CLI and later cells first) must be stated as the console's own surface, not read as those rows.

@history_item n=10 marker=2026-10-20-t06-58-31-wrap
**2026-10-20-t06-58-31-wrap** — the board's cut width from `viola_pty::host_size()` stands as the founder's (provisional marks gone). The console's width for its row cut and frame floor rests on the same source; the scaffold polls size every 100 ms.

@history_item n=11 marker=2026-11-14-budget-gate-removed
**2026-11-14-budget-gate-removed** (6. Visual Design Verification: 1 row) — `~ decision/state-carried-by-printed-word` dropped `budget-paused`; the board's printed state words are stale, DIALOG <kind>, expired, unverified-cli, idle, busy, n/a. The console shows state «in the board's words» with no new vocabulary: «stalled» is `stale` (F3), and no budget word may return.

@history_item n=12 marker=2026-10-20-epoch-5-cleanup-ii
**2026-10-20-epoch-5-cleanup-ii** — the `viola list` literal-pinning units are cited under `src/human/board/` (was `src/human/board.rs`). The console's row builder reuse and any new pins sit in that module; citations must use the current path (the scope already corrects the caption line to `rows.rs:108`).

@history_item n=13 marker=2026-11-06-version-done-check
**2026-11-06-version-done-check** (the cli checks on `std::process::Command` and literal pins; §8 verification lines) — re-worded all trycmd/assert_cmd text to literal pins over `std::process::Command`, every fixed CLI string pinned by literal, the Consistency verification source the `CAPTION` literal. The console's new fixed strings (`LAST`, `... <n> more`, the skip-count line) fall under `decision/fixed-cli-strings-pinned` / `decision/fixed-cli-strings-pinned-by-literal` and `decision/cli-a11y-by-literal-tests`: each needs a literal pin.

@history_item n=14 marker=2026-11-24-viola-up
**2026-11-24-viola-up** (8. Cognitive Accessibility; 1. A11y Scope Summary) — the two time-bound lists took the lead «Eight» and three later bounds got rows of their own (F1 option 1); `+ contract/viola-up-output/instance-line` gave a new verb's human output its own row. Precedent for the console: a new bound or a new human output surface lands as its own row, never by growing the full lists `decision/time-bounds-never-on-human` / `decision/time-bounds-fall-on-the-driver`.

@history_item n=15 marker=2026-11-24-viola-focus
**2026-11-24-viola-focus** (the operator's answers to F-A and F1 ratified) — ratified F1's rule «a later bound a row only where it is new on a new bearer»; viola focus's 5 s compositor read added nothing as `decision/hyprctl-read-deadline` already named it. The console's reuse of the 5 s agents read on a new bearer (the console) and its 1 s re-read cadence are to be judged by this rule: a cadence is no time bound on the human, a reused bound adds a row only if new on a new bearer.

@history_item n=16 marker=2026-11-22-epoch-4-cleanup
**2026-11-22-epoch-4-cleanup** — `+ decision/start-verify-stop-grace-off-the-human`: with the two time-bound lists full at their bound, a new bound lands as a row of its own beside them. The same constraint binds any bound the console view introduces.

@history_item n=17 marker=2026-11-24-viola-adopt
**2026-11-24-viola-adopt** (8. Cognitive Accessibility: 4 rows, 1 of 2) — `~ decision/claude-agents-read-fails-open` widened to «passes revive's and adopt's preflight» because adopt became a new reader. The console becomes a further reader of the agents read whose failure must leave its rows as the board read gave them; the row names only the list board and two preflights, a likely edit this chunk owes or must flag.

@history_item n=18 marker=2026-10-04-the-wheel
**2026-10-04-the-wheel** — the non-editing set became the closed F-W2 list of focus reports, mouse reports and terminal replies, every other byte editing (`decision/non-editing-sequences`). The console reads raw keys from stdin; the view adds no key, but any terminal reply or resize sequence it meets must still be read by that closed list, not as a human key.

@history_item n=19 marker=2026-10-08-first-live-test-and-self-drive
**2026-10-08-first-live-test-and-self-drive** — recorded the takeover clause's live reading on Linux as supplemental and never gating (`decision/live-readings-never-gate`). The view's compositor half (tiled, quarter-screen) is a live reading at the operator's host, his to grant; it gates the v2-14 claim, never this chunk's CI.

@history_item n=20 marker=2026-10-16T14-58-10-wrap
**2026-10-16T14-58-10-wrap** — the web surfaces left; the tier line `Standard (1)` was not re-decided, its justification resting on the terminal surfaces alone. `decision/a11y-tier-standard` binds the console with no WCAG conformance claim; output discipline stands in for SC 1.3.2, 1.4.1, 3.3.1.

@history_item n=21 marker=2026-11-01-t02-46-44-wrap
**2026-11-01-t02-46-44-wrap** — the founder's answer keeps the a11y tier `Standard (1)`. The console view raises no tier question; it inherits Standard (1).

@history_item n=22 marker=2026-09-28-mutation-testing-to-the-epoch-boundary
**2026-09-28-mutation-testing-to-the-epoch-boundary** — the CLI output-discipline invariants in crate tests are covered by the epoch-boundary mutation audit (`run --mutants`), not per chunk (`decision/mutation-audit-covers-crate-invariants`). Consistent with the scope's «no mutation run»: the console's escaping and caption invariants meet mutation only at Epoch 6's boundary audit.

@history_item n=23 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** — Linux only (`decision/linux-only`), CI on `ubuntu-latest`, the 0.2.0 window features on Hyprland; `tests/cross_list_rows.rs` holds the no-ESC assertion in each mode, and the a11y cases run in the `coverage` suite. The console's escaping and no-SGR guards need no other OS arm; its Hyprland tiling is the 0.2.0 window feature the plan names.
