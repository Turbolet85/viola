@answer n=1 entity=e2e-test-strategy#6.4 v=yes
why: the Unix outer-PTY driver must exit only via child.wait() and clean up by Ctrl-C, a rule the scope names in its outer-PTY item
@answer n=2 entity=e2e-test-strategy#7.1 v=yes
why: Path 6 step 6 chmods the home to 0770 on Unix and the statusline-echo steps run the Unix statusline pass-through the scope names
@answer n=3 entity=e2e-test-strategy#7.2 v=yes
why: the Unix marker equality and the Windows no-override arm are the statusline pass-through split the scope names
@answer n=4 entity=e2e-test-strategy#7.3 v=yes
why: the cfg(unix) hook_statusline case and the Unix-arm marker signal are per-OS gates with named reasons the scope names
@answer n=5 entity=e2e-test-strategy#7.4 v=no
why: a generic harness cleanup with no OS branch, a guarantee the harness keeps by its own code as a component the chunk only uses
@answer n=6 entity=e2e-test-strategy#8.1 v=no
why: Path 7 setup steps have no OS branch and the chunk changes no version-gate behaviour that these steps drive
@answer n=7 entity=e2e-test-strategy#8.2 v=yes
why: the Path 7 hook signal, empty stdout with exit 0 inside the spine deadline, is the hook behaviour the scope names for every OS
@answer n=8 entity=e2e-test-strategy#8.3 v=no
why: the landed MCP leg is OS-agnostic and already green on all three CI runners, with no per-OS rule for the chunk to honour
@answer n=9 entity=e2e-test-strategy#8.4 v=no
why: a harness cleanup and a TempDir drop with no OS branch, which the chunk does not change
@answer n=10 entity=e2e-test-strategy#9.1 v=yes
why: the R8 strip read through the running child's live fd table on Unix is the claude_env_keep survival the scope names for the Unix strip
@answer n=11 entity=e2e-test-strategy#9.2 v=yes
why: claude_env_keep survival on Unix and the Unix child holding no viola fd, fds 0 to 2 as the PTY, are R8 strip obligations the scope names over openpty
@answer n=12 entity=e2e-test-strategy#9.3 v=no
why: a TempDir cleanup with no OS branch and no convention the chunk's Unix runs change
@answer n=13 entity=e2e-test-strategy#10.1 v=no
why: the landed E3 link and unlink cases carry no OS branch and stay green on all three runners unchanged by the chunk
@answer n=14 entity=e2e-test-strategy#11.1 v=no
why: the E5 replay readers and their absent snapshot keys hold alike on every OS, so no OS-divergent field is at stake for the one-schema rule
@answer n=15 entity=e2e-test-strategy#12.1 v=yes
why: the Unix socket-dir refusal, the macOS-excluded server-identity impostor rows and the Unix hook fail-open rows are the per-OS gates and exit-zero rules the scope names
@answer n=16 entity=e2e-test-strategy#13.1 v=yes
why: the Unix diagnostics 0600 modes and the per-OS secret scan over target/e2e-home behind the per-OS uploads are the diagnostics and CI obligations the scope names
@answer n=17 entity=e2e-test-strategy#14.1 v=yes
why: the Unix socket-dir and server-verify exit-21 rows, the macOS carve-out and the Unix-widened strict-modes case are the exit-code contract readings the scope names per OS
@answer n=18 entity=e2e-test-strategy#15.1 v=yes
why: the cfg unix socket-dir, umask and server-identity negatives and their cfg windows controls are the per-OS gates with named reasons the scope names
@answer n=19 entity=e2e-test-strategy#16.1 v=yes
why: the TTY list case asserts no ESC only on Unix and reads escaped names on Windows, the same outer-PTY list reading the v1-40 witness rests on
@answer n=20 entity=e2e-test-strategy#17.1 v=yes
why: the no-EOF held-output split, the Unix SIGSTOP stale case and the forced vt100 panic bytes on Unix are the Unix chaos readings the scope names
@answer n=21 entity=e2e-test-strategy#18.1 v=no
why: the property suite rules over OS-agnostic parsers and the vt100 model the chunk does not change, its chaos use being carried by entity 17.1
@answer n=22 entity=e2e-test-strategy#19.1 v=yes
why: the contract census walks the product roots, so the chunk's new Unix PTY reader must be named there, and contract_content_hash must stay green on all three OSes
