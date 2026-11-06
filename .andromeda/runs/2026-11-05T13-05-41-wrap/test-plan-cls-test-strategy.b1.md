@answer n=1 entity=test-strategy#1.1 v=yes mode=contradict
why: the session makes Viola Linux only, so the runs-on-all-3-OSes claim and the macOS-gated server-check carve-out no longer hold
@answer n=2 entity=test-strategy#1.2 v=no
why: the crate-level suites with a real interprocess endpoint, tempfile home and assert_cmd still hold on Linux, and no crate-level suite is retired
@answer n=3 entity=test-strategy#1.3 v=no
why: the seven critical paths, the four scenarios, the outer PTY and the rmcp client are untouched by the session
@answer n=4 entity=test-strategy#1.4 v=no
why: the six proptest surfaces and the ubuntu nightly fuzz targets are unchanged and no parser is retired
@answer n=5 entity=test-strategy#1.5 v=yes mode=stale
why: the session keeps perf on one ubuntu-latest matrix leg, so the per-OS perf CI job now runs a single OS
@answer n=6 entity=test-strategy#1.6 v=yes mode=stale
why: the session removes the ConPTY sideload, PtyError::Load and the backend choice from viola-pty, which the viola-pty mockall seam named here reaches
@answer n=7 entity=test-strategy#1.7 v=no
why: the mutation verdict and the known survivor sit in client and mcp seams the change does not reach, and the no-CI-mutation-gate claim still holds
@answer n=8 entity=test-strategy#1.8 v=no
why: the fixture sets, the schemas and the contract_fake_agent_drift byte comparisons are unchanged, and the Unix gate on the statusline replay is not among the retired items
@answer n=9 entity=test-strategy#2.1 v=yes mode=contradict
why: the session removes the icacls Windows arms of home.rs, the Windows debugger freeze in frozen.rs, the TMPDIR socket root on macOS and scripts/agent-run.ps1
@answer n=10 entity=test-strategy#2.2 v=no
why: contract_os_gated_tests.rs stays a root contract_ binary, auto-discovered and selected by the E2E regex, and its new fault class leaves that placement unchanged
@answer n=11 entity=test-strategy#3.1 v=yes mode=contradict
why: the session deletes security_negatives_channel.rs with the Windows listener and DACL controls it names, and makes the off-macOS server-identity and hook impostor rows unconditional
@answer n=12 entity=test-strategy#3.2 v=no
why: the paste validation and its proptest and integration layers are untouched
@answer n=13 entity=test-strategy#3.3 v=no
why: the hook stdin parser and its assert_cmd boundary are unchanged
@answer n=14 entity=test-strategy#3.4 v=no
why: the MCP rmcp E2E scenarios and the security sweep name no retired Windows or macOS control
@answer n=15 entity=test-strategy#3.5 v=no
why: the CLI and env layers and their unit and integration homes still hold, and the registry env reader is not named here
@answer n=16 entity=test-strategy#3.6 v=yes mode=contradict
why: the session removes the Windows owner/DACL strict-modes arm, so the icacls read-back of a Windows --home protected DACL no longer exists
@answer n=17 entity=test-strategy#3.7 v=no
why: the child-spawn row names only the §5 and §6 Scenario E2 locations, which still exist, and the .cmd refusal is not named
@answer n=18 entity=test-strategy#3.8 v=no
why: the lint, supply-chain and release stages keep their membership, and the narrowed deny targets and orphans-check script do not change which stages run
@answer n=19 entity=test-strategy#3.9 v=no
why: the property-test layer and its §4 and §9 fuzz placements are unchanged
@answer n=20 entity=test-strategy#3.10 v=no
why: the chaos suite placement is unchanged
@answer n=21 entity=test-strategy#3.11 v=no
why: the performance-budget row names only §10 and the session does not move that placement
@answer n=22 entity=test-strategy#3.12 v=no
why: the cross-surface scenarios and parity placements are untouched
@answer n=23 entity=test-strategy#3.13 v=yes mode=contradict
why: the session makes the CI matrix ubuntu-latest only and makes a gate naming another OS a census fault, so the multi-os-compat coverage no longer holds
@answer n=24 entity=test-strategy#3.14 v=no
why: the viola-agent-claude section, the §7 fixtures and the claude --version replay with its per-version fixtures still hold
@answer n=25 entity=test-strategy#3.15 v=no
why: the contract-sandbox row cites only §6 and §7, and §7 still holds once the seed_conpty exception is gone
