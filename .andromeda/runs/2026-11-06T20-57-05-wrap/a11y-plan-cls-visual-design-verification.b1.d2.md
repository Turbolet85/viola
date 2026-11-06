@answer n=1 entity=visual-design-verification#1.1 v=no
why: the list.rs and cross_list_rows.rs edits are remnant-only and the full suite (3367 passed 0 failed) still runs the named witnesses, with no SGR, caption or column moved
@answer n=2 entity=visual-design-verification#2.1 v=no
why: the stale-row SGR sequence is unchanged and the outer-PTY e2e binaries that assert it are green on the 0.9.0 pin
