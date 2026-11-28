# Answer card — wrap 2026-11-28T12-37-34Z-wrap (chunk 2026-11-28-fleet-console-view)

Written 2026-11-28T13:11:02Z. ONE card (inputs#I7 item 2; `rul-bznresq5`), confirmed by the overseer, the operator away.

## His answers, by their record
inputs#I3 (`relay-console-view-forks.md`, his own answers by three question dialogs read 2026-11-28T10:35:17Z,
11:04:30Z, 11:05:42Z, relayed verbatim by viola-overseer):
- (a) «Переформулировать (Recommended)» — the two «no verb's output» rows reworded to «The terminal fleet console
  (`viola console`), its own surface, is governed by no verb-output rule».
- (b) «Не трогать (Recommended)» — the eight verb-output rows untouched (their four offers rejected, not on this card).
- F1 «Расширить два (Recommended)» — the two named exceptions gain «…or the fleet it shows changes».
- F2 «Ячейка LAST в консоли (Recommended)» — a seventh cell `LAST`, the console alone.
- F4 «Резать строку (Recommended)» — the row cut at the terminal's width ending `...`.
- F5 «Порядок доски + «ещё N» (Recommended)» — the board's order, then one `... <n> more` line.
- F6 «Раз в секунду (Recommended)» — the board re-read once a second, a redraw only on a change, the agents read at
  most once every 5 s off the drawing loop (he was told the read spawns the real CLI every 5 s).
- F7 «Только при изменении (Recommended)» — a named console exception: the skip-count line only when the count changes.

## The operations lying wholly inside them — each fit-judged first
Old → new, member by member; a new row's old reads «—».

1. `ops-main-design-system.md` · decision/fleet-console-not-verb-output · surface-cli · fit: fits type 1 · (a)
   rule: «The terminal fleet console is no verb's output, so verb output rules do not govern its redraw.» → «The
   terminal fleet console (`viola console`), its own surface, is governed by no verb-output rule.»
2. `ops-main-design-system.2.md` · decision/fleet-console-not-verb-output · surface-cli · fit: fits type 1 · (a)
   name: «Fleet console is no verb's output» → «Fleet console governed by no verb-output rule»
3. `ops-main-layout-templates.md` · decision/no-prompt-or-pager · surface-cli · fit: fits type 2 · (a)
   rule: «No verb has an interactive prompt or a pager, and the terminal fleet console is no verb's output.» → «No
   verb has an interactive prompt or a pager, and the terminal fleet console (`viola console`), its own surface, is
   governed by no verb-output rule.»
4. `ops-a11y-plan.md` · decision/static-linear-output · 1-a11y-scope-summary · fit: fits type 2 · F1
   except: «… which redraws its frame when the terminal's size changes» → «… which redraws its frame when the
   terminal's size changes or the fleet it shows changes»
5. `ops-a11y-plan.md` · decision/cli-no-interactive-input · 5-keyboard-navigation · fit: fits type 2 · F1
   except: «… reads keys from stdin in raw mode and redraws on a resize» → «… reads keys from stdin in raw mode and
   redraws on a resize or a change in the fleet it shows»
6. `ops-obs-plan.md` · decision/symlink-skip-logged-as-count · log-coverage · fit: fits type 2 · F7
   except: «viola up, which never takes the strict board read, writes no parse-rejected state-entry line.» → the same,
   then «; viola console writes the line only when the scan's counts change.» (viola up's member kept whole)
7. `ops-layout-templates.md` · decision/shared-captions-fixed · surface-cli · fit: fits type 1 · F2
   rule: «The six captions, words and slot order are fixed, and the terminal fleet console reuses them.» → «… and the
   terminal fleet console reuses them and adds `LAST` after them.»
8. `ops-design-system.md` · contract/fleet-console-view/captions · surface-console · fit: fits type 3 · F2
   — → «The console's caption row is the board's six captions, then a seventh, `LAST`.»
9. `ops-design-system.md` · contract/fleet-console-view/last · surface-console · fit: fits type 3 · F2
   — → «`LAST` is the age of the instance's newest log line in one unit (`12s`, `4m`, `2h`, `3d`); `unknown` on a
   wrapped row with no log line, `n/a` on an unwrapped row.»
10. `ops-design-system.md` · contract/fleet-console-view/line-cut · surface-console · fit: fits type 3 · F4
   — → «A console line wider than the terminal is cut whole at its columns ending in `...`, never inside an escape; an
   unwrapped NAME is not cut separately in the console.»
11. `ops-design-system.md` · contract/fleet-console-view/more-line · surface-console · fit: fits type 3 · F5
   — → «A fleet taller than the console's body rows ends in a line of the form `... <n> more`.»
12. `ops-layout-templates.md` · decision/console-fleet-body-lines · surface-console · fit: fits type 1 · F5
   — → «The terminal fleet console draws the fleet as body lines between its title line and its key line, the board's
   rows in the board's forest order and indent.»
13. `ops-layout-templates.md` · decision/console-redraw-on-change · surface-console · fit: fits type 1 · F6
   — → «The terminal fleet console re-reads the board once a second and redraws only when its view changes, with no
   redraw while the terminal size and the view stand.»
14. `ops-main-layout-templates.md` · decision/console-agents-read-off-loop · surface-console · fit: fits type 1 · F6
   — → «The terminal fleet console reads the claude agents list on its own thread at most once every 5 s, off its
   drawing and key loop, so a key never waits on it.»
15. `ops-obs-plan.md` · contract/diag-subject/agents-probe · log-coverage · fit: fits type 3 · F6
   promise: «… verify's Run B and the list tool of viola mcp into their role files.» → «… verify's Run B, the list tool
   of viola mcp and viola console's agents thread into their role files.» (every writer kept)
16. `ops-security-plan.md` · decision/claude-agents-resolved-from-path · input-validation · fit: fits type 1 · F6
   rule: «list, revive's preflight, adopt's live-elsewhere check and the MCP list tool resolve claude …» → «list,
   revive's preflight, adopt's live-elsewhere check, the MCP list tool and viola console resolve claude …» (the rest
   unchanged)
17. `ops-architecture.md` · decision/agents-join-read-ungated · cross-cutting · fit: fits type 1 · F6
   rule: «viola list and viola revive do not gate their claude agents --json read on a ledger stamp.» → «viola list,
   viola revive and viola console do not gate …»
The card's count: 17 (lines 1–17). Check 6 groups no occurrence with them: the report disproves no claim.

## Asked beside the card, its own question — partly inside
- `ops-a11y-plan.md` · decision/claude-agents-read-fails-open · cognitive-accessibility · fit: fits type 1 — its
  console clause rests on his F6; its «hands an empty list, so STATUS reads unknown» rests on the delegate's technical
  word (inputs#I5, the failed agents read reading `unknown`), PROVISIONAL.
  rule: «A failed, oversize, malformed or timed-out claude agents --json read leaves the list board as the board read
  gave it and passes revive's and adopt's preflight.» → the same, then «; in the fleet console it hands an empty
  list, so STATUS reads unknown.»

## Not asked
- Rejected before the card, fit `not_master` type 4: none of the card's; architecture
  decision/board-last-activity-never-serialised (an add) and security-plan decision/list-scan-checks-before-open (a
  set) — neither escalated.
- The delegate's technical rows (inputs#I4, I5) land as `apply · word: delegate viola-overseer` — PROVISIONAL for the
  epoch batch, no card item.

## The answer
Given by the overseer through this session's question dialog, read 2026-11-28T13:11:35Z, verbatim:
- The card: «Confirm all 17 (Recommended)» — notes: «delegate viola-overseer (2026-11-28T13:11:35Z): confirmed, all 17
  — each lies inside the operator's own answers (his dialogs read 2026-11-28T10:35:17Z, 11:04:30Z, 11:05:42Z). The
  three rows that gain «viola console» as a named member of who runs the agents read follow from his F6, which chose
  that the console runs it at most every 5 s: a named addition, no rule generalised.»
- The empty list: «Land, PROVISIONAL (Recommended)» — notes: «delegate viola-overseer (technical, 2026-11-28T13:11:35Z):
  land it PROVISIONAL — a failed read must not keep showing a stale busy.»
The confirm is the delegate's word: each line it resolves is PROVISIONAL (`delegate viola-overseer`), the ratification
his answers (inputs#I3), superseded by his own word in the epoch batch.
