@answer n=1 entity=visual-design-verification#1.1 v=no
why: the list rendering rules and their board_row / cross_list_rows checks stand, the reached list.rs and cross_list_rows.rs edits are remnant-only and the pre-push suite stays green at 3367 passed 0 failed
@answer n=2 entity=visual-design-verification#2.1 v=no
why: the outer-PTY stale-row witness still asserts the same spans on ubuntu-latest, since the outer-PTY e2e binaries are green on portable-pty 0.9.0 and the sweep left that witness's expectations unchanged
