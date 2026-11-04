@answer n=1 entity=test-strategy#1.1 v=yes
why: the per-OS gating of unit cases (cargo-mutants harness tests, server check captured-span cases) and the server_verdict_rows macOS carve-out are what the chunk's gating governs.
@answer n=2 entity=test-strategy#1.2 v=yes
why: the Unix socket and openpty integration tests the chunk adds must run on a real endpoint in a real tempfile home against the viola binary.
@answer n=3 entity=test-strategy#1.3 v=yes
why: the fake-agent E2E on all three runners and the Unix outer-PTY driver are the critical-path E2E this rule places under harness-booted viola run.
@answer n=4 entity=test-strategy#1.4 v=no
why: the chunk adds no parser or fuzz target, and the proptest and corpus replay rules sit over parser surfaces it does not change.
@answer n=5 entity=test-strategy#1.5 v=yes
why: the per-OS perf job timing the hook rows against SPINE_DEADLINE on max is the performance-budget gate this rule defines.
@answer n=6 entity=test-strategy#1.6 v=yes
why: the mockall faults on the viola-pty seam, the no-EOF child and the SIGSTOP heartbeat freeze must hold on the openpty backend as well.
@answer n=7 entity=test-strategy#1.7 v=yes
why: the mutants 10 s kill profile and the WITHIN bound on Unix witnesses that spawn product processes are what the chunk's tests must stay inside.
@answer n=8 entity=test-strategy#1.8 v=yes
why: the Unix statusline pass-through and the fake-agent-only Unix readings replay the recorded fixture sets this contract tier names.
@answer n=9 entity=test-strategy#2.1 v=yes
why: the scope names the Wrapper::kill helper in tests/support/home.rs and the Unix outer-PTY driver, which this layout places in tests/support.
@answer n=10 entity=test-strategy#3.1 v=yes
why: the macOS carve-out of the server-identity negatives and the Unix socket-directory refusals in root security_negatives files are named by the scope.
@answer n=11 entity=test-strategy#3.2 v=no
why: paste validation (V2) is not touched by the chunk's seam, driver or diagnostics work.
@answer n=12 entity=test-strategy#3.3 v=yes
why: the Unix hook exit-0 and fail-open matrix runs are hook stdin tests at the integration layer, assert_cmd hook, that this row names.
@answer n=13 entity=test-strategy#3.4 v=yes
why: the fake-agent E2E the chunk runs on all three runners includes the MCP critical-path scenarios this row places on the rmcp client.
@answer n=14 entity=test-strategy#3.5 v=yes
why: the --json document parity on every OS and the per-child env handling are CLI and env cases at the unit and integration layers this row names.
@answer n=15 entity=test-strategy#3.6 v=yes
why: the Unix modes (socket directory 0700, socket 0600, umask owner-only homes, widen helpers) and their Windows DACL pairing are filesystem rules the scope names.
@answer n=16 entity=test-strategy#3.7 v=yes
why: the openpty spawn of the fake agent and the Unix child-ends-with-wrapper reading are the process boundary this row covers.
@answer n=17 entity=test-strategy#3.8 v=yes
why: the per-OS lint job the scope names carries the sync-crate cargo check and orphans check that this lint stage lists.
@answer n=18 entity=test-strategy#3.9 v=no
why: the property-test row governs parser surfaces and the nightly fuzz job, and the chunk adds no parser and no fuzz target.
@answer n=19 entity=test-strategy#3.10 v=yes
why: the Unix killed-wrapper and kill-and-revive readings are fault injections against a wrapper, which the chaos suite row governs.
@answer n=20 entity=test-strategy#3.11 v=yes
why: the per-OS perf job timing the hook rows against SPINE_DEADLINE on max is the performance-budget layer this row names.
@answer n=21 entity=test-strategy#3.12 v=yes
why: one diagnostics schema across events.ndjson, snapshots and every --json document, and the v1-40 list-beside-wrapper witness, are cross-surface coordination checks.
@answer n=22 entity=test-strategy#3.13 v=yes
why: the per-OS matrix builds on windows-2025, ubuntu-latest and macos-latest are the multi-os-compat trigger this row maps to the CI matrix.
@answer n=23 entity=test-strategy#3.14 v=no
why: the chunk changes no viola-agent-claude parse and records no fixture, so the per-version multi-version-compat trigger does not move.
@answer n=24 entity=test-strategy#3.15 v=yes
why: the Unix readings run the fake agent as the sandbox against recorded fixtures, which the contract-test-against-sandbox row governs.
@answer n=25 entity=test-strategy#3.16 v=yes
why: the diagnostics documented nulls and closed words, the Unix socket-directory refusal documents and the per-OS secret-scan uploads fall under this sanitization rule.
