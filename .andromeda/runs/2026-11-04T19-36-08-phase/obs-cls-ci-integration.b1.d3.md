@answer n=1 entity=ci-integration#1.1 v=yes
why: the per-OS diag upload's scan-gated condition and diag-${{ matrix.os }} name are the upload gating need 14 names for the test job on every OS
@answer n=2 entity=ci-integration#1.2 v=yes
why: the failure-only harness capture and its scan-gated upload per matrix.os are what need 14 names for the test job
@answer n=3 entity=ci-integration#1.3 v=yes
why: the scan-gated junit upload per matrix.os is named by need 14 for the test job the chunk re-reads
@answer n=4 entity=ci-integration#1.4 v=yes
why: the per-OS perf job's hyperfine exports and scan-gated uploads are the perf job need 16 names and the scope re-reads green on ubuntu and macos
@answer n=5 entity=ci-integration#1.5 v=no
why: the row records that v1 has no OTel trace exporter and the chunk adds none (not-needed item 2); it names no code the chunk must act on
@answer n=6 entity=ci-integration#2.1 v=yes
why: the lint job's clippy disallowed-macros, print and dbg bans, lint-probes.sh and cargo deny are need 17, which must be green on every OS
@answer n=7 entity=ci-integration#2.2 v=yes
why: in-memory writer tests that check each line against diag-line.v1.json are the unit-tier witness of the schema re-assertion the scope names
@answer n=8 entity=ci-integration#2.3 v=yes
why: the perf row's hyperfine max assertions are need 16, and the fake-agent diagnostics the Unix suite writes must be covered by G2, G4 and the scan the scope re-reads
@answer n=9 entity=ci-integration#2.4 v=yes
why: the E2E agent-run logs and the G2 and G4 exit codes cover the fake-agent suite the scope runs on ubuntu and macos
@answer n=10 entity=ci-integration#2.5 v=no
why: mutation runs only at the epoch-boundary audit or on demand, not in this chunk's CI, and the scope neither runs nor changes the mutants workflow (not-needed item 7)
@answer n=11 entity=ci-integration#3.1 v=yes
why: the G1 bare-instrument step and its SHA-pinned ripgrep PCRE2 install are gate G1 of need 12, which the scope re-reads green on every OS
@answer n=12 entity=ci-integration#3.2 v=yes
why: G2 (g2-zero-panics.sh, fail-closed, --probe, the seam.rs exemption) is named by needs 12 and 22 and must stay green on both Unix OSes
@answer n=13 entity=ci-integration#3.3 v=yes
why: G4 schema conformance against diag-line.v1.json is need 13, and the scope re-reads it on both Unix OSes
@answer n=14 entity=ci-integration#4.1 v=yes
why: the CI step order, the if-always G2 and G4 after the tests, and AGENT_RUN_KEEP_HOMES=1 are need 14, the per-OS test job the chunk edits
@answer n=15 entity=ci-integration#5.1 v=yes
why: the secret scan's after-run reading and AGENT_RUN_KEEP_HOMES=1 on the job are need 14 and need 15, which order the scan before artifact upload
