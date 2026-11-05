@answer n=1 entity=e2e-test-strategy#6.3 v=no
why: Path 5 wheel and release refusals are untouched (task not-needed 3) and no clause names a Windows or macOS gate.
@answer n=2 entity=e2e-test-strategy#6.4 v=no
why: Path 5 cleanup (Ctrl-C, child wait, TempDir drop) names no OS gate and the chunk does not change the scenario.
@answer n=3 entity=e2e-test-strategy#7.1 v=yes
why: step 1 names the statusline pass-through root tests and tests/support/home.rs plant_statusline_source, the Unix-only arm the chunk makes unconditional (need 12).
@answer n=4 entity=e2e-test-strategy#7.2 v=yes
why: its Windows clause (override_document and shell_argv return None, the arm runs no command) is the hook statusline Windows arm the chunk removes, and its Unix marker clause is kept (needs 12, 13).
@answer n=5 entity=e2e-test-strategy#7.3 v=yes
why: names the cfg(not(unix)) Windows statusline leg and the cfg(unix) hook_statusline step-6 case, both gates the chunk rewrites (needs 12, 13).
@answer n=6 entity=e2e-test-strategy#7.4 v=no
why: Path 6 cleanup is a harness cleanup call with no OS gate and the chunk does not change it.
@answer n=7 entity=e2e-test-strategy#8.1 v=no
why: Path 7 version gate and verify stamped line are not needed (task not-needed 5) and the steps name no OS gate.
@answer n=8 entity=e2e-test-strategy#8.2 v=no
why: Path 7 verification signals (exit codes, one question event, the verify stamped line) are untouched by the removal.
@answer n=9 entity=e2e-test-strategy#8.3 v=no
why: the Path 7 MCP leg records a three-runner green measurement of an untouched Linux test, with no gate or code the chunk removes.
@answer n=10 entity=e2e-test-strategy#8.4 v=no
why: Path 7 cleanup (harness cleanup, TempDir drop for H3) names no OS gate.
@answer n=11 entity=e2e-test-strategy#9.1 v=yes
why: the R8 strip runs on Unix through claude_env_keep, which the chunk makes unconditional, and names the identity-floor names the Windows host measures (needs 12, 13).
@answer n=12 entity=e2e-test-strategy#9.2 v=yes
why: its Windows handle non-inheritance clause is a removed Windows assertion, and its Unix claude_env_keep and fd-table clauses are the Unix arm the chunk keeps (needs 9, 12).
@answer n=13 entity=e2e-test-strategy#9.3 v=no
why: cleanup only (TempDir drop); the binding rules sit in entities 9.1 and 9.2.
@answer n=14 entity=e2e-test-strategy#10.1 v=no
why: E3 link and unlink is not needed (task not-needed 6) and names no OS gate; its three-runner record is a measurement, not a rule the chunk must honour.
@answer n=15 entity=e2e-test-strategy#11.1 v=no
why: the E5 replay readers are untouched by the removal and the entity names no Windows or macOS gate.
@answer n=16 entity=e2e-test-strategy#12.1 v=yes
why: names the cfg_attr(not(target_os = "macos")) impostor rows, the Windows server-verify and Windows pipe clauses and the Unix socket-directory negative, all gates or units the chunk removes or re-gates (needs 11, 13).
@answer n=17 entity=e2e-test-strategy#13.1 v=yes
why: its diagnostics-0600-on-Unix rule is the Unix arm the chunk makes unconditional, and its CI scan and harness-<os> upload wording sits on the CI jobs the chunk removes (needs 1, 12).
@answer n=18 entity=e2e-test-strategy#14.1 v=yes
why: names the Windows .cmd refusal, the Windows strict-modes and Windows server-verify cases and the macOS carve-out, all removed or re-pointed by the chunk (needs 10, 11).
@answer n=19 entity=e2e-test-strategy#15.1 v=yes
why: names the cfg(windows) client SQOS, listener and --home DACL negatives, the macOS server-identity gate and the Unix socket-directory negative, which the chunk removes or re-gates (needs 8, 9, 11).
@answer n=20 entity=e2e-test-strategy#16.1 v=yes
why: names the Windows terminal case and its Unix-only no-ESC assertion in the list row integrity binary the chunk re-gates (needs 13).
@answer n=21 entity=e2e-test-strategy#16.2 v=yes
why: its cfg(unix) stale-row witness falls under the Unix gate review and names the macos-latest leg the chunk retires (needs 1, 13).
@answer n=22 entity=e2e-test-strategy#17.1 v=yes
why: names the Windows-only literal oracle, the cfg(windows) .cmd child refusal and the macOS held-output exclusion, all removed or re-pointed by the chunk (needs 10, 11).
@answer n=23 entity=e2e-test-strategy#17.2 v=yes
why: its cfg(target_os = "linux") hangup case reads the fake agent's SIGHUP-ignoring exit, the Unix behaviour the chunk keeps unconditional (needs 11, 12).
@answer n=24 entity=e2e-test-strategy#18.1 v=no
why: the property suite's parser targets and proptest settings are not needed (task not-needed 10) and the entity names no OS gate.
@answer n=25 entity=e2e-test-strategy#19.1 v=yes
why: its deny.toml feature-ban read stands on the file the chunk trims, and its contract census and content-hash witnesses are reached by the removal (needs 14, 17).
