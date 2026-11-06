@answer n=1 entity=ci-integration#1.1 v=yes
why: the scope names the gated diag-<os> upload in its step order (task item 7)
@answer n=2 entity=ci-integration#1.2 v=yes
why: the scope names the gated harness-<os> upload and its failure() condition (task item 7)
@answer n=3 entity=ci-integration#1.3 v=yes
why: the scope names the gated junit-<os> upload and the ubuntu nextest JUnit read (task items 7 and 9)
@answer n=4 entity=ci-integration#1.4 v=yes
why: the perf job is one of CI's seven jobs that must stay green, and its matrix-os artifact names must hold on the ubuntu-only matrix (task item 13)
@answer n=5 entity=ci-integration#1.5 v=no
why: OTel trace export is N/A in v1 and the chunk adds no exporter
@answer n=6 entity=ci-integration#2.1 v=yes
why: task item 8 names the clippy disallowed-macros and print bans, lint-probes.sh and cargo deny, which the chunk must clear
@answer n=7 entity=ci-integration#2.2 v=no
why: the unit-test oracle for in-memory writers is a test-layer convention every change owes, and the chunk adds no log line
@answer n=8 entity=ci-integration#2.3 v=no standing=S1
why: the viola-home rule under target/e2e-home/ is standing S1; the chunk adds no integration-test home and the chaos carve-out is untouched
@answer n=9 entity=ci-integration#2.4 v=yes
why: the E2E evidence read through agent-run logs and status and the G2 and G4 gate exits is what the scope re-runs green (task item 7)
@answer n=10 entity=ci-integration#2.5 v=yes
why: the OS-gated census is the re-read check of contract_os_gated_tests.rs (task item 16), and its windows and macOS predicate fault binds the remnant removal
@answer n=11 entity=ci-integration#2.6 v=yes
why: task item 15 makes a surviving obs-code mutant at run --mutants a build failure, and the host.rs windows test-host key sits in that harness
@answer n=12 entity=ci-integration#3.1 v=yes
why: G1 bare #[instrument] is a gate task item 7 names to re-run green
@answer n=13 entity=ci-integration#3.2 v=yes
why: G2 zero panics, with its seam exemption and probe, is a gate task item 7 names to re-run green
@answer n=14 entity=ci-integration#3.3 v=yes
why: G4 schema conformance is a gate task item 7 names to re-run green
@answer n=15 entity=ci-integration#4.1 v=yes
why: the test job's G2 and G4 if-always ordering and AGENT_RUN_KEEP_HOMES are the step order task item 7 names
@answer n=16 entity=ci-integration#5.1 v=yes
why: the secret-scan timing after each job's run is what the scope's secret-scan and gates-green check reads (task items 7 and 9)
