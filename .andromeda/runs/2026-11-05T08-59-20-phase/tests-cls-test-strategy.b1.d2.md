@answer n=1 entity=test-strategy#1.1 v=yes
why: the unit tests compiled out on macOS (the two real-cargo-mutants harness tests, the server check's captured-span cases, refused_inside, the macOS server_pid case) are units the chunk ungates or removes (scope items 5, 11)
@answer n=2 entity=test-strategy#1.2 v=no standing=S4
why: each integration test makes a real tempfile home, the convention S4 lists
@answer n=3 entity=test-strategy#1.3 v=no
why: the E2E path and scenario suites name no OS gate and no unit the chunk edits; the free-space cases are not in this entity
@answer n=4 entity=test-strategy#1.4 v=yes
why: states the cargo-fuzz targets run as a time-boxed ubuntu nightly job, which the nightly.yml edit must keep on ubuntu (scope item 4)
@answer n=5 entity=test-strategy#1.5 v=yes
why: names the per-OS perf CI job the chunk removes on Windows and macOS (scope item 1) while the hook-deadline hyperfine gates stay on Linux
@answer n=6 entity=test-strategy#1.6 v=no
why: the chaos fault set (sysinfo, std, File::set_len, SIGSTOP freeze, mockall on viola-pty) names no removed arm and no unit the chunk edits
@answer n=7 entity=test-strategy#1.7 v=yes
why: the known survivor at the three client seams' peer_admitted reads is the euid wiring that scope item 20 keeps held by no test, and the remove-the-guard witness rule covers the cfg(unix) controls the chunk ungates
@answer n=8 entity=test-strategy#1.8 v=yes
why: the statusline replay test is Unix-gated and the fake agent's statusline pass-through and per-command process group are cfg(unix) arms the chunk makes unconditional (scope items 12, 19)
@answer n=9 entity=test-strategy#2.1 v=yes
why: names the support units the chunk edits: home.rs icacls widen and read-back, frozen.rs Windows debugger attach, the SOCKET_ROOT_VAR macOS arm, and the scripts/agent-run.{sh,ps1} shim pair (scope items 6, 8)
@answer n=10 entity=test-strategy#2.2 v=yes
why: tests/contract_os_gated_tests.rs is the OS-gate census the chunk re-points or retires with its gates (scope item 11)
@answer n=11 entity=test-strategy#3.1 v=yes
why: names the Windows named-pipe listener's remote-client rejection and DACL denial, and the macOS-off server-identity and Unix socket-dir root tests the chunk removes or re-gates (scope items 9, 11)
@answer n=12 entity=test-strategy#3.2 v=no
why: the paste-injection layers name no OS gate and no unit the chunk edits
@answer n=13 entity=test-strategy#3.3 v=no
why: the hook stdin layer names no OS gate, and the chunk does not touch the hook assert_cmd tests
@answer n=14 entity=test-strategy#3.4 v=no
why: the MCP E2E layer names no OS gate, and the rmcp suites are untouched by the chunk
@answer n=15 entity=test-strategy#3.5 v=no
why: the CLI and env row names only the test tiers and locations, with no OS arm or env seam the chunk changes
@answer n=16 entity=test-strategy#3.6 v=yes
why: names the Windows --home protected-DACL read-back and the Unix umask and mode cases in security_negatives_strict_modes, whose Windows strict-modes over real grants the chunk removes (scope item 8)
@answer n=17 entity=test-strategy#3.7 v=no
why: the child-spawn layers name no OS gate; the .cmd refusal question is scope item 10, decided by P4, not this row
@answer n=18 entity=test-strategy#3.8 v=yes
why: supply chain over the removed Windows dependencies (cargo deny, ban probes, sole-root tokio ban, release-check) and the weekly nightly advisories the chunk keeps on ubuntu (scope items 4, 17)
@answer n=19 entity=test-strategy#3.9 v=no
why: the property-test row names proptest and the fuzz job as locations only; the fuzz job's ubuntu rule is carried by test-strategy#1.4
@answer n=20 entity=test-strategy#3.10 v=no
why: the chaos-test row names the suite location only, with no OS gate and no removed unit
@answer n=21 entity=test-strategy#3.11 v=no
why: the performance-budget row names section 10 as the location, with no per-OS rule
@answer n=22 entity=test-strategy#3.12 v=no
why: the cross-surface row names Scenarios 2 and E3 with no OS gate, and E3 is not needed by the chunk
@answer n=23 entity=test-strategy#3.13 v=yes
why: multi-os-compat is the 3-OS CI matrix and the OS-gate census in contract_os_gated_tests.rs, both of which the chunk edits
@answer n=24 entity=test-strategy#3.14 v=no
why: the multi-version-compat row names agent-claude tests and fixtures with no OS gate, and the chunk does not touch them
@answer n=25 entity=test-strategy#3.15 v=no
why: the contract-sandbox row names suite and fixture locations only, with no OS rule
