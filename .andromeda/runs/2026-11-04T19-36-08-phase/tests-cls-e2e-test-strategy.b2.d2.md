@answer n=1 entity=e2e-test-strategy#6.4 v=yes
why: the outer-PTY cleanup (Ctrl-C, then child.wait()) is the exit rule the chunk extends to the Unix outer-PTY driver (item 12)
@answer n=2 entity=e2e-test-strategy#7.1 v=yes
why: steps 1 and 6 state the Unix statusline pass-through (settings.json override, 0770 home), which the chunk makes the Unix reading of (item 13)
@answer n=3 entity=e2e-test-strategy#7.2 v=yes
why: the byte-equal marker and the Windows no-op arm are the statusline pass-through split the chunk must hold on Unix (item 13)
@answer n=4 entity=e2e-test-strategy#7.3 v=yes
why: the cfg(unix) hook_statusline case and the cfg(not(unix)) Windows arm are the one-OS statusline gates the chunk states (item 13)
@answer n=5 entity=e2e-test-strategy#7.4 v=no
why: a bare harness cleanup with no OS behaviour, and the chunk adds no harness command (not_needed item 8)
@answer n=6 entity=e2e-test-strategy#8.1 v=no
why: the unstamped and report-version boots state the version gate with no OS branch, already green on all three runners, so a re-check every change owes
@answer n=7 entity=e2e-test-strategy#8.2 v=yes
why: the hook returning within the spine deadline is the hook-row bound the per-OS perf job re-measures against SPINE_DEADLINE (item 4)
@answer n=8 entity=e2e-test-strategy#8.3 v=no
why: the MCP two-first-starts leg states an OS-agnostic result the chunk does not change, its three-runner green record being already held
@answer n=9 entity=e2e-test-strategy#8.4 v=no standing=S3
why: the TempDir home drop for H3 states a home convention that S3 replaces with a home under target/e2e-home
@answer n=10 entity=e2e-test-strategy#9.1 v=yes
why: the Command::env setup and the Unix live fd-table read of the child are the claude_env_keep survival the chunk moves to Unix (item 13)
@answer n=11 entity=e2e-test-strategy#9.2 v=yes
why: the Unix claude_env_keep survival and the child's live fd table are named by the scope (item 13)
@answer n=12 entity=e2e-test-strategy#9.3 v=no standing=S3
why: the TempDir home drop states a home convention that S3 replaces with a home under target/e2e-home
@answer n=13 entity=e2e-test-strategy#10.1 v=no
why: the link and unlink records and their cli cases state no OS branch, and the chunk changes no link record or verb
@answer n=14 entity=e2e-test-strategy#11.1 v=no
why: the replay readers over a replayed snapshot state no OS branch, and the chunk does not change the replay
@answer n=15 entity=e2e-test-strategy#12.1 v=yes
why: the Linux and macOS hook fail-open rows, the Unix socket-dir refusal and the macOS server-verify carve-out are the per-OS readings the chunk names (items 8, 13, 15)
@answer n=16 entity=e2e-test-strategy#13.1 v=yes
why: the canary secret scan over target/e2e-home and the per-OS uploads behind it are named by the scope (item 5), and the diagnostics files are 0600 on Unix (item 13)
@answer n=17 entity=e2e-test-strategy#14.1 v=yes
why: the exit 21 and exit 1 causes name the Unix strict-modes and socket-dir refusals and the macOS server-verify carve-out (items 8, 13, 14)
@answer n=18 entity=e2e-test-strategy#15.1 v=yes
why: the Unix socket-dir refusal, the umask-independent Unix modes, the macOS server-identity carve-out and the Windows-only controls are the per-OS gating the chunk states (items 3, 8, 13)
@answer n=19 entity=e2e-test-strategy#16.1 v=yes
why: the list TTY row asserts no ESC Unix-only, on the list surface the chunk drives under the Unix outer PTY (items 10, 12)
@answer n=20 entity=e2e-test-strategy#17.1 v=yes
why: the no-EOF holder by process group, the Unix SIGSTOP stale case, the Unix vt100 panic byte search and the 5 s stale threshold are Unix readings the chunk states (items 9, 11, 12)
@answer n=21 entity=e2e-test-strategy#18.1 v=no
why: the proptest parsers and feed properties carry no OS branch and no fake agent, and the chunk does not change them
@answer n=22 entity=e2e-test-strategy#19.1 v=yes
why: the fake-agent drift against the recorded fixtures and the sha256 cases on all three OSes are what the Unix readings rest on (item 18)
