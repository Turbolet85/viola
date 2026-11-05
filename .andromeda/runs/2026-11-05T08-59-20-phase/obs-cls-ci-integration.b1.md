@answer n=1 entity=ci-integration#1.1 v=yes
why: the diag-<os> upload gated on secret-scan success is named by task item 2 on the remaining Linux test job
@answer n=2 entity=ci-integration#1.2 v=yes
why: the harness-<os> capture on failure() before the scan is named by task item 2 for the remaining Linux jobs
@answer n=3 entity=ci-integration#1.3 v=yes
why: the junit-<os> upload gated on secret-scan success is named by task item 2 on the remaining Linux test jobs
@answer n=4 entity=ci-integration#1.4 v=no
why: an operator-only script over the three-OS junit set; the chunk edits no file it reads and no chunk test honours it
@answer n=5 entity=ci-integration#1.5 v=yes
why: the per-OS perf job keeps only its Linux leg and task item 5 reads the five hyperfine perf-<hook> max values
@answer n=6 entity=ci-integration#1.6 v=no
why: OTel trace export is N/A in v1 and the chunk adds no exporter
@answer n=7 entity=ci-integration#2.1 v=yes
why: task item 7 names clippy disallowed-macros, the print and dbg bans, lint-probes.sh and cargo deny (ubuntu) as gates the removal must still pass
@answer n=8 entity=ci-integration#2.2 v=no
why: the in-memory writer unit tests and their schema check are unchanged, and the chunk keeps every Linux wire and log value (task item 11)
@answer n=9 entity=ci-integration#2.3 v=yes
why: task item 15 names the e2e-home rule, and the free-space fix must keep both harness homes under target/e2e-home/
@answer n=10 entity=ci-integration#2.4 v=yes
why: the E2E agent-run logs and status capture and the G2 and G4 exit codes are named by task items 2 and 6
@answer n=11 entity=ci-integration#2.5 v=yes
why: tests/contract_os_gated_tests.rs is reached file n=11, its census re-pointed or retired with the gates it counted
@answer n=12 entity=ci-integration#2.6 v=yes
why: task item 4 keeps the mutation gate (run --mutants, survived obs-code mutants) on Linux once windows-mutants.yml is deleted
@answer n=13 entity=ci-integration#3.1 v=yes
why: G1 with its PCRE2 ripgrep install is a gate task item 1 keeps as its own run step on each remaining Linux job
@answer n=14 entity=ci-integration#3.2 v=yes
why: the G2 zero-panic gate with the fake-agent seam exemption is named by task items 6 and 18 on the remaining test and perf jobs
@answer n=15 entity=ci-integration#3.3 v=yes
why: the G4 schema-conformance step under if: always() is named by task items 1 and 2 on the remaining Linux test job
@answer n=16 entity=ci-integration#4.1 v=yes
why: the step order (tests, then G2 and G4 under always, homes kept) is named by task item 2 for the remaining Linux test and perf jobs
@answer n=17 entity=ci-integration#5.1 v=yes
why: the scan-gated secret-scan step and AGENT_RUN_KEEP_HOMES on the remaining Linux job are named by task items 2 and 15
