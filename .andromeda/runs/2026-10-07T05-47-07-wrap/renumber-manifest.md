# Route-coordinate renumber — the 2026-10-07 0-pending wrap (split of "First live test and self-drive")

Authority: the overseer's answer at this wrap's dialogue, item 6, relayed by the operator, 2026-10-07 ("close the
4-line lag in the same pass, the manifest in the run dir"); the practice is the founder's ruling R-S1 of 2026-10-05
("route citations only, verified by manifest").

**What moved.** At `af179a9` the live citations sat 4 lines behind their entries: entries were inserted into the
route after the last renumber (the 2026-10-05 wraps) and no pass renumbered. This wrap then moved the route again:
two entries with their `↓` ahead of "First live test and self-drive" (+4 for every later line), "Linux live
confirmation" and its `↓` retired (−2 after it), "Windows-only live measurements" and its `↓` added at the end of
Epoch 7 (+2 after it). Each citation is rewritten to its entry's line after the adaptation:

| cited at `af179a9` | entry | line at `af179a9` | line now |
|---|---|---|---|
| `:93` | Self-healing state | 97 | `:101` |
| `:110` | MCP server for drivers | 114 | `:118` |
| `:117` | Server verification before any frame | 121 | `:125` |
| `:119` | Home and code-bearing file integrity | 123 | `:127` |
| `:137` | viola ui loopback server | 141 | `:145` |
| `:139` | Resumable SSE feed | 143 | `:147` |
| `:147` | Strip-bay live page | 151 | `:155` |
| `:90` (one site, the real Windows terminal's mouse report) | First live test and self-drive, then at 94; the item now sits on Windows-only live measurements | — | `:140` |

**Changed occurrences: 37 in 10 files** — the table is `renumber-table.md` in this run dir (file:line @char · old ·
new · entry), written by the pass itself. Per file: `architecture.md` 1 · `obs-plan.md` 2 · `security-plan.md` 9 ·
`test-plan.md` 11 · `registries/contracts/a11y-plan/keyboard-test-harness.md` 1 · `docs/services/viola-state.md` 1 ·
`docs/tests-summary.md` 8 · `rules/events.md` 1 · `rules/security.md` 2 · `CLAUDE.md` 1.

**Kept as written: the ledger's dated notes.** `verification-matrix.json` carries 11 backticked route citations
(`:80`, `:84`, `:88` ×2, `:90` ×5, `:108`, `:110`) and three `working-route.md:78`, all inside dated notes; none was
rewritten, since a ledger note is a dated record whose one writer is the ledger tool. The card for item 6 counted 38
live citations; two of those (`:108` and `:110` in the ledger) are in this kept set, so 36 of the 38 were rewritten,
plus the `:90` site above. Sidecars, archives and the route's own freight were not scanned for rewriting.

**Guard.** Binary writes; each position asserted to hold the old number between `:` and a backtick before the write;
every untouched line compared byte-for-byte after it; each new coordinate asserted to open the named entry in the
route as it stands after the adaptation. `git diff --numstat` after the pass showed only the ten files above with
equal insertions and deletions. A re-scan reads `:101` ×5 · `:118` ×6 · `:125` ×6 · `:127` ×8 · `:145` ×2 ·
`:147` ×2 · `:155` ×7 · `:140` ×1 outside the ledger.

**Order.** The route edit landed first; the renumber ran on the adapted route; the owner amendments followed.
