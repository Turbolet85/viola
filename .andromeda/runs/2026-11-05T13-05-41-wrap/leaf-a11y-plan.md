@leaf_group group=a11y-plan
@leaf_read leaf=.claude/docs/a11y-summary.md lines=58
last: **Full plan:** `.andromeda/a11y-plan.md`. Path-scoped rules:
@leaf_read leaf=.claude/rules/a11y.md lines=30
last: _This section is owned by `/wrap-session`. setup-project pre
@leaf_edit n=1 leaf=.claude/docs/a11y-summary.md line=18 class=stale
leaf-text: on all three OSes (the frame verbs' server-verify peer pid witness on windows-2025 and ubuntu-latest only: macOS reports no peer pid, the word there an euid mismatch alone; the socket-directory refusal and the stale-row dim witness `cross_list_rows_dim_a_stale_row_end_to_end_on_a_terminal` on ubuntu-latest and macos-latest)
source: fct-2jdilj7v «on ubuntu-latest (the frame verbs' server-verify peer pid witness and the socket-directory refusal among them).»
replace-with: on the `ubuntu-latest` CI leg, Linux only (the frame verbs' server-verify peer pid witness, the socket-directory refusal and the stale-row dim witness `cross_list_rows_dim_a_stale_row_end_to_end_on_a_terminal` among them)
@leaf_edit n=2 leaf=.claude/docs/a11y-summary.md line=18 class=stale
leaf-text: manual NVDA / VoiceOver / Orca passes supplemental only
source: fct-orpg5jsb «**Manual pass (supplemental):** Orca (Linux) on Section 4 P4 and P6. It is founder-owned per the creator brief excerpt, Appendix A.»
replace-with: manual Orca passes (Linux) supplemental only
@leaf_edit n=3 leaf=.claude/docs/a11y-summary.md line=37 class=stale
leaf-text: the manual NVDA / VoiceOver / Orca pass stays founder-owned
source: fct-t3p7hz2f «**screen-reader-test-spec-setup:** none in 0.1.0; the manual Orca pass stays founder-owned (Section 1 → Screen reader test pattern).»
replace-with: the manual Orca pass stays founder-owned
@leaf_edit n=4 leaf=.claude/docs/a11y-summary.md line=38 class=stale
leaf-text: the cli / tui cases run inside the per-OS `test` job's `coverage` suite
source: fct-yftct3hq «the cli / tui cases run inside the `test` job's `coverage` suite (matrix `[ubuntu-latest]`) under `gate --require coverage,doctest`, and no a11y lint, aggregation or upload step exists.»
replace-with: the cli / tui cases run inside the `test` job's `coverage` suite (matrix `[ubuntu-latest]`)
@leaf_edit n=5 leaf=.claude/docs/a11y-summary.md line=47 class=retired
leaf-text: or terminal replies as editing — a mouse report the Windows ConPTY already turned into win32 key-down records is typing (F-W3).
source: fct-csjr2vg7 «NEVER block, refuse or delay a human keystroke in `viola run`, or in a start a passed `viola revive` made, past the current atomic paste, and never count focus / mouse / resize sequences or terminal replies as editing (P4).»
replace-with: or terminal replies as editing (P4).
@leaf_edit n=6 leaf=.claude/docs/a11y-summary.md line=53 class=stale
leaf-text: is its reading of the zero-viola-bytes clause, green on the three CI runners.
source: fct-avr5o2ub «adds none of viola's own literals to the outer-PTY stream, green in CI (chunk 2026-10-10-statusline-pass-through, run 38080855246).»
replace-with: is its reading of the zero-viola-bytes clause, green in CI (run 38080855246).
@leaf_edit n=7 leaf=.claude/docs/a11y-summary.md line=54 class=stale
leaf-text: the outer PTY holds none of viola's literals, green on the three CI runners.
source: fct-avr5o2ub «the outer-PTY stream holds none of `unable`, `hint:` and `budget-paused`, green in CI (chunk 2026-10-10-budget-governor, run 38091081413).»
replace-with: the outer PTY holds none of viola's literals, green in CI (run 38091081413).
@leaf_edit n=8 leaf=.claude/rules/a11y.md line=19 class=retired
leaf-text: terminal replies are never counted as editing — a mouse report the Windows ConPTY already turned into win32 key-down records is typing (F-W3).
source: fct-itb2wxsm «focus, mouse and resize sequences and terminal replies (F-W2's closed list) pass through and do not count as human editing; every other byte does.»
replace-with: terminal replies (F-W2's closed list) are never counted as editing; every other byte is.
@leaf_edit n=9 leaf=.claude/rules/a11y.md line=25 class=stale
leaf-text: inside the per-OS `test` job's `coverage` suite under `gate --require coverage,doctest` (skips never allowed) on every leg;
source: fct-yftct3hq «the cli / tui cases run inside the `test` job's `coverage` suite (matrix `[ubuntu-latest]`) under `gate --require coverage,doctest`, and no a11y lint, aggregation or upload step exists.»
replace-with: inside the `test` job's `coverage` suite (matrix `[ubuntu-latest]`) under `gate --require coverage,doctest` (skips never allowed);
