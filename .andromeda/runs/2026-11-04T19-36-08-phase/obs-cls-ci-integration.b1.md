@answer n=1 entity=ci-integration#1.1 v=no standing=S4
why: the diag upload's scan gate, SHA pin and 7-day retention are the S4 convention, carried for every CI change
@answer n=2 entity=ci-integration#1.2 v=yes
why: the harness capture step runs on failure before the secret scan so the scan covers it, a test-job step order the scope names
@answer n=3 entity=ci-integration#1.3 v=no standing=S4
why: the JUnit upload's scan gate and retention are the S4 convention
@answer n=4 entity=ci-integration#1.4 v=yes
why: the per-OS perf job exports one hyperfine JSON per hook row that the max gate reads, and the scope names that perf job on ubuntu and macos
@answer n=5 entity=ci-integration#1.5 v=no
why: it states that no OTel exporter exists in v1, the chunk adds none, and the task excludes OTel
@answer n=6 entity=ci-integration#2.1 v=yes
why: the lint job on ubuntu and macos runs clippy disallowed-macros, the print and dbg bans, lint-probes.sh and cargo deny, all named by the scope
@answer n=7 entity=ci-integration#2.2 v=yes
why: unit tests validate emitted lines against schemas/diag-line.v1.json, the witness for the one-schema re-assert on every OS
@answer n=8 entity=ci-integration#2.3 v=yes
why: the perf stage's hyperfine max assertions bind the perf job the scope names on ubuntu and macos, while its home-placement rule is the S1 convention
@answer n=9 entity=ci-integration#2.4 v=yes
why: the E2E gate exit codes for G2 and G4 are the zero-panic and schema measures the scope re-reads
@answer n=10 entity=ci-integration#2.5 v=no
why: mutation runs at the epoch-boundary audit and on dispatch only, the chunk neither runs nor changes windows-mutants.yml, and it adds no mutation job
@answer n=11 entity=ci-integration#3.1 v=yes
why: G1 is the bare-instrument gate the scope re-reads, with its PCRE2 ripgrep install and exit-code rules on all three OSes
@answer n=12 entity=ci-integration#3.2 v=yes
why: G2 is the zero-panics gate the scope re-reads, with its probe and the single seam.rs exemption the chunk must not widen
@answer n=13 entity=ci-integration#3.3 v=yes
why: G4 is the schema-conformance step against both schemas that the scope re-reads
@answer n=14 entity=ci-integration#4.1 v=yes
why: the test-job and perf-job step order, the if-always gates and AGENT_RUN_KEEP_HOMES=1 are what the chunk's ci.yml must honour on each OS
@answer n=15 entity=ci-integration#5.1 v=yes
why: AGENT_RUN_KEEP_HOMES=1 and the scan running after each job's end-to-end run are what ci.yml must keep on each OS, with the chaos home left out
