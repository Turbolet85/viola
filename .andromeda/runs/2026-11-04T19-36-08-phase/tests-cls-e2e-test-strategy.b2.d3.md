@answer n=1 entity=e2e-test-strategy#6.4 v=yes
why: the outer-PTY driver on Unix ends its child by Ctrl-C and child.wait(), which the scope names for the tui outer-PTY driver (task item 12).

@answer n=2 entity=e2e-test-strategy#7.1 v=yes
why: the Unix statusline-echo arm, its settings source and the chmod 0770 step are the statusline pass-through the scope names on Unix only (task item 13).

@answer n=3 entity=e2e-test-strategy#7.2 v=yes
why: the marker equality on Unix against the no-override, no-command Windows arm states the statusline pass-through per OS (task item 13).

@answer n=4 entity=e2e-test-strategy#7.3 v=yes
why: its cfg(unix) hook_statusline case and cfg(not(unix)) Windows arm are per-OS test gates the chunk must carry with their reasons (task item 3).

@answer n=5 entity=e2e-test-strategy#7.4 v=no
why: harness cleanup is teardown of a harness session with no OS branch and no kill, wait or exit rule stated here; the chunk does not change it.

@answer n=6 entity=e2e-test-strategy#8.1 v=yes
why: the fake agent fires its PreToolUse fixture and answers --version for the unstamped boot, child behaviour the seam must carry on Unix (task items 1, 18).

@answer n=7 entity=e2e-test-strategy#8.2 v=yes
why: an empty hook stdout with exit 0 inside the spine deadline is the hook exit-0 contract on every OS (task item 14) and the per-OS perf job's SPINE_DEADLINE timing (task item 5).

@answer n=8 entity=e2e-test-strategy#8.3 v=yes
why: it boots an unstamped pair through the seam and is one of the three-OS witnesses the shared fake-agent suite must keep green on Unix (task item 2).

@answer n=9 entity=e2e-test-strategy#8.4 v=no
why: harness cleanup and the TempDir drop of H3 are teardown with no OS branch and no kill, wait or exit rule stated here; the chunk does not change them.

@answer n=10 entity=e2e-test-strategy#9.1 v=yes
why: the R8 strip runs the child with Command::env on Unix and reads its live /proc fd table, the claude_env_keep and fd rules the scope names on Unix (task item 13).

@answer n=11 entity=e2e-test-strategy#9.2 v=yes
why: claude_env_keep surviving on Unix and the child holding no viola socket or path on Unix are per-OS rules the scope names (task item 13).

@answer n=12 entity=e2e-test-strategy#9.3 v=no
why: TempDir drop is home teardown with no OS branch and no kill, wait or exit rule stated here; the chunk does not change it.

@answer n=13 entity=e2e-test-strategy#10.1 v=no
why: link and unlink record the wrapper's own state with no OS branch and no child, PTY or exit behaviour that the seam change reaches.

@answer n=14 entity=e2e-test-strategy#11.1 v=no
why: the replay of budget, link and list state reads the wrapper's files with no OS branch and no child or PTY behaviour, and the chunk does not change the replay.

@answer n=15 entity=e2e-test-strategy#12.1 v=yes
why: the Unix socket-dir refusals, the cfg(not(target_os = "macos")) impostor and server-identity rows, and the 0600 diagnostics modes are per-OS rules the chunk must carry (task items 8, 13).

@answer n=16 entity=e2e-test-strategy#13.1 v=yes
why: the secret scan over target/e2e-home and the uploads named by matrix.os, plus the Unix 0600 diagnostics modes, are CI and diagnostics rules the chunk must hold on every OS (task items 5, 7).

@answer n=17 entity=e2e-test-strategy#14.1 v=yes
why: the closed exit-21 and exit-1 causes and their documents must read the same closed words on every OS, with the Unix socket-dir causes and the macOS server-verify carve-out (task items 7, 8, 14).

@answer n=18 entity=e2e-test-strategy#15.1 v=yes
why: the cfg(unix) socket-dir and umask-mode negatives and the cfg(windows) SQOS, pipe and DACL negatives are the per-OS gates the chunk must carry with their reasons (task items 3, 13).

@answer n=19 entity=e2e-test-strategy#16.1 v=yes
why: the terminal case asserts no ESC only on Unix and reads escaped names as text on Windows, a per-OS split the shared suite must carry (task items 2, 3, 7).

@answer n=20 entity=e2e-test-strategy#17.1 v=yes
why: the Unix SIGSTOP case, the Linux-only held-output process group, the macOS exit-on-handle rule and the Unix outer-terminal panic bytes are per-OS readings of the seam and the wrapper's child (task items 6, 9, 12).

@answer n=21 entity=e2e-test-strategy#18.1 v=yes
why: its input-not-ready reading of the poisoned vt100 model is the value the Unix forced-panic chaos case re-asserts (task item 12).

@answer n=22 entity=e2e-test-strategy#19.1 v=yes
why: the closed --json error-body tables over the product roots and the contract_content_hash run on all three OSes name the --json documents the one diagnostics schema must hold closed (task item 7).
