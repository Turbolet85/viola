@answer n=1 entity=test-strategy#1.1 v=yes mode=contradict
why: the unit tier says it runs on all 3 OSes with a macOS carve-out, both retired with the Windows and macOS code (task items 1 and 4)
@answer n=2 entity=test-strategy#1.2 v=no
why: the integration tier names the interprocess endpoint, tempfile home and assert_cmd, none of which the session changed
@answer n=3 entity=test-strategy#1.3 v=no
why: the E2E tier's portable-pty outer PTY and its seven paths and four scenarios still hold, and no OS count is stated
@answer n=4 entity=test-strategy#1.4 v=no
why: the property tier's proptest suite and six fuzz parser targets on an ubuntu nightly job still hold, with no OS count or retired workflow named
@answer n=5 entity=test-strategy#1.5 v=no
why: the perf tier's hyperfine rows and the perf job keep the matrix.os form with one leg (task item 15), so per-OS names that form and still holds
@answer n=6 entity=test-strategy#1.6 v=no
why: the chaos faults from sysinfo, std file and process operations and the mockall viola-pty seam are unchanged by the session
@answer n=7 entity=test-strategy#1.7 v=no
why: the mutation rules, the WITHIN witness and the known survivor name no retired OS gate or count, and the survivor's client read is not one the session changed
@answer n=8 entity=test-strategy#1.8 v=no
why: the contract tier's fixture sets, schemas and Unix-gated statusline replay still hold on Linux and name no Windows or macOS arm
@answer n=9 entity=test-strategy#2.1 v=yes mode=contradict
why: the directory naming still gives frozen.rs a Windows debugger arm, the home.rs icacls widen helpers, SOCKET_ROOT_VAR as TMPDIR on macOS and scripts/agent-run.ps1, all retired (task items 13, 14 and 19)
@answer n=10 entity=test-strategy#2.2 v=no
why: tests/contract_os_gated_tests.rs stays a root contract_ binary auto-discovered with no [[test]] table, and its new fault class does not change that
@answer n=11 entity=test-strategy#3.1 v=yes mode=contradict
why: the V1 row cites the security_negatives_channel.rs remote-client and DACL rows, which the session deleted, and the off-macOS impostor rows, which now run unconditionally (task items 3 and 26)
@answer n=12 entity=test-strategy#3.2 v=no
why: the paste row points to §4 viola-core and §6 Scenario 2, which the session did not touch
@answer n=13 entity=test-strategy#3.3 v=no
why: the hook stdin row points to the §5 hook boundary and the §6 sweep, both unchanged by the session
@answer n=14 entity=test-strategy#3.4 v=no
why: the MCP row points to §6 Scenarios 2, 5 and 7 and the sweep, and the MCP surface rules were not changed
@answer n=15 entity=test-strategy#3.5 v=no
why: the CLI and env row points to §4 and §5 with no OS or registry claim, so the retired registry env reader is not named
@answer n=16 entity=test-strategy#3.6 v=yes mode=contradict
why: the V7 row names a Windows --home DACL case read back through icacls, which is retired, and an off-Unix umask case, which now runs unconditionally (task items 5, 14 and 26)
@answer n=17 entity=test-strategy#3.7 v=no
why: the child-spawn row points to the §5 process boundary and §6 Scenario E2, and Scenario E2 is unchanged, the retired .cmd case sitting in other rows
@answer n=18 entity=test-strategy#3.8 v=no
why: the supply-chain row names the lint, deny, release and nightly stages and their scripts, all still present, with no OS count or target stated
@answer n=19 entity=test-strategy#3.9 v=no
why: the property-test row points to the §4 proptest suite and the §9 fuzz job, both unchanged
@answer n=20 entity=test-strategy#3.10 v=no
why: the chaos-test row points to the §6 Chaos suite, which still exists, and the retired .cmd case does not move the pointer
@answer n=21 entity=test-strategy#3.11 v=no
why: the performance-budget row points to §10 Performance budgets, and the budgets and perf gate are unchanged
@answer n=22 entity=test-strategy#3.12 v=no
why: the cross-surface row points to §6 Scenarios 2 and E3 and cross-surface parity, which are unchanged
@answer n=23 entity=test-strategy#3.13 v=yes mode=contradict
why: the multi-os-compat row relies on a §9 CI matrix that is now ubuntu-latest only, and the OS-gate census now faults any other-OS gate (task items 15 and 26)
@answer n=24 entity=test-strategy#3.14 v=no
why: the multi-version row points to §4 viola-agent-claude and §7 fixtures, which keep their version coverage
@answer n=25 entity=test-strategy#3.15 v=no
why: the contract-sandbox row points to §6 Contract suite and §7, which still exist, so the seed_conpty exception leaving §7 does not falsify the pointer
