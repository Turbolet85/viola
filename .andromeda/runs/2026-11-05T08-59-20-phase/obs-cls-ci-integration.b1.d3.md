@answer n=1 entity=ci-integration#1.1 v=yes
why: the scope (item 2) keeps the diag-<os> upload on the remaining Linux jobs, gated on secret-scan success
@answer n=2 entity=ci-integration#1.2 v=yes
why: the scope (item 2) keeps the harness-<os> capture and upload on failure() gated on the secret scan
@answer n=3 entity=ci-integration#1.3 v=yes
why: the scope (item 2) keeps the junit-<os> uploads on the remaining test jobs, gated on secret-scan success
@answer n=4 entity=ci-integration#1.4 v=no
why: junit-os-sets.py is an operator tool outside the reached files, and its three-OS inputs are the wrap's per-OS amendment, not a rule the Linux code or tests must honour
@answer n=5 entity=ci-integration#1.5 v=yes
why: the per-OS perf job keeps a Linux leg, and its perf-<os> and diag-perf uploads and hook-row exports are named by scope items 2 and 5
@answer n=6 entity=ci-integration#1.6 v=no
why: no OTel exporter exists in v1 and the chunk adds none
@answer n=7 entity=ci-integration#2.1 v=yes
why: scope item 7 names the lint gates the removal must still pass (clippy disallowed-macros, print_stdout, print_stderr, dbg_macro, lint-probes.sh) and cargo deny on ubuntu
@answer n=8 entity=ci-integration#2.2 v=no
why: the in-memory writer unit-test oracle is a re-check every change owes; the chunk adds no emitted line and edits no schema or writer test file
@answer n=9 entity=ci-integration#2.3 v=yes
why: scope item 15 names the e2e-home rule that every integration-test viola home lives under target/e2e-home/, so G2, G4, the secret scan and the diag upload cover it
@answer n=10 entity=ci-integration#2.4 v=yes
why: the E2E agent-run logs and status and the G2 and G4 gate exit codes stay on the remaining Linux jobs (scope items 1 and 6)
@answer n=11 entity=ci-integration#2.5 v=yes
why: the census tests/contract_os_gated_tests.rs is an edited file (task n=11), re-pointed or retired with the gates it counted
@answer n=12 entity=ci-integration#2.6 v=yes
why: the mutation gate run --mutants stays on Linux (scope item 4) and its red rule binds that gate, while the windows-mutants.yml counts leave with the deleted file
@answer n=13 entity=ci-integration#3.1 v=yes
why: G1 is a gate step kept verbatim on the Linux job (scope item 1), with the SHA-pinned rg install step before G1 and G3
@answer n=14 entity=ci-integration#3.2 v=yes
why: G2 runs in the remaining test and perf jobs with the fake-agent seam exemption (scope items 6 and 18)
@answer n=15 entity=ci-integration#3.3 v=yes
why: G4 is kept on the remaining Linux test job as if: always() after the last test step (scope items 2 and 6)
@answer n=16 entity=ci-integration#4.1 v=yes
why: the step order, the always() conditions and the home retention on the remaining Linux test and perf jobs are named by scope items 2, 5 and 15
@answer n=17 entity=ci-integration#5.1 v=yes
why: AGENT_RUN_KEEP_HOMES on the ci.yml job and the always() secret-scan condition stay on the remaining Linux jobs (scope items 2 and 15)
