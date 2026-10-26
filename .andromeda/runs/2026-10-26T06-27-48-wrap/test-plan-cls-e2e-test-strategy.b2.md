# Classified — test-plan · e2e-test-strategy.b2

## Answers
e2e-test-strategy#6.4 · no · the Path 5 cleanup names only the outer PTY, `child.wait()` and the TempDir drop, none of which the tokio pin bump reaches
e2e-test-strategy#7.1 · no · the Path 6 steps name statusline, budget and harness surfaces that the tokio pin bump does not reach and name no tokio or manifest line
e2e-test-strategy#7.2 · no · the Path 6 signals name budget, statusline and exit-code behaviour, which the tokio patch bump with the same dependency set leaves unchanged
e2e-test-strategy#7.3 · no · the as-landed test names and the earlier three-runner green reads are historical and stay true because the session moved no test or product surface
e2e-test-strategy#7.4 · no · the Path 6 cleanup is the harness `cleanup` call, which the session did not touch
e2e-test-strategy#8.1 · no · the Path 7 steps name the unstamped boot, `answer` and `verify` surfaces, which the tokio pin bump does not reach
e2e-test-strategy#8.2 · no · the Path 7 signals name the `send`, `answer` and `verify` exit codes, which the session did not change
e2e-test-strategy#8.3 · no · the Path 7 cleanup is the harness `cleanup` and the H3 TempDir drop, neither moved by the session
e2e-test-strategy#9.1 · no · the E2 steps name the `CLAUDE*` canary env names and the claude-stripped table, which the session did not change
e2e-test-strategy#9.2 · no · the E2 signals rest on portable-pty's `close_random_fds()`, whose `=0.8.1` pin stays unmoved because only the tokio lock entry moved
e2e-test-strategy#9.3 · no · the E2 cleanup is the TempDir drop, which the session did not touch
e2e-test-strategy#10.1 · no · the E3 as-landed reading names the `-32601` refusals and the implicit-link root cases, which the session did not change
e2e-test-strategy#11.1 · no · the E5 as-landed reading names the replay readers and their root cases, none of which the session moved
e2e-test-strategy#12.1 · no · the security sweep names MCP handler tests, the hook fail-open matrix and `assert_cmd`, whose version the report shows unmoved
e2e-test-strategy#13.1 · no · the error-sanitization scan names the helper checks and the e2e home, junit and upload paths, none of which the session touched
e2e-test-strategy#14.1 · no · the exit-cause matrix names exit causes and their root cases, which the session did not change
e2e-test-strategy#15.1 · no · the security negatives name Windows pipe and DACL checks through windows-sys, whose version and feature gates the session did not move
e2e-test-strategy#16.1 · no · the list row integrity case names the stub `claude agents --json` output and the nextest kill line, neither reached by the session
e2e-test-strategy#17.1 · no · the chaos suite names sysinfo 0.39.6, which stays unmoved because only the tokio lock entry moved
e2e-test-strategy#18.1 · no · the property suite names proptest 1.11.0, which stays unmoved because only the tokio lock entry moved
e2e-test-strategy#19.1 · no · the contract suite names `sha2` as a root dev-dependency, which stays unmoved because only the tokio pin line moved in Cargo.toml
