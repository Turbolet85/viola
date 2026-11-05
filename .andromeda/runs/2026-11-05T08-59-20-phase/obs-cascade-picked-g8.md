@account n=1 entity=ci-integration#1.1 v=carried
why: the diag upload is named by `matrix.os`, and the chunk shrinks that matrix to ubuntu-latest
@account n=2 entity=ci-integration#1.2 v=carried
why: the harness capture upload is named by the `matrix.os` the chunk shrinks
@account n=3 entity=ci-integration#1.3 v=carried
why: the JUnit upload is named by the `matrix.os` the chunk shrinks
@account n=4 entity=ci-integration#1.5 v=carried
why: the per-OS perf job loses its Windows and macOS legs, and its Linux leg must keep the five hyperfine rows and their scan-gated uploads
@account n=5 entity=ci-integration#2.1 v=carried
why: the chunk edits deny.toml and the lint legs, and cargo deny with its obs bans and the clippy print bans must still pass on Linux
@account n=6 entity=ci-integration#2.3 v=carried
why: the chunk's harness fix edits the e2e-home backing cases, so homes must stay under target/e2e-home/ for the gates to cover them
@account n=7 entity=ci-integration#2.4 v=carried
why: every gate green on Linux is the chunk's acceptance, and G2 and G4 are its E2E gates
@account n=8 entity=ci-integration#2.5 v=carried
why: the chunk re-points or retires tests/contract_os_gated_tests.rs, the census this row states
@account n=9 entity=ci-integration#2.6 v=carried
why: the chunk deletes windows-mutants.yml, the Windows leg this mutation row names
@account n=10 entity=ci-integration#3.1 v=carried
why: G3 scans the manifests and workflows the chunk edits, and needs the rg install step kept in the remaining Linux leg
@account n=11 entity=ci-integration#3.2 v=carried
why: G2 must stay in the Linux test and perf jobs that remain and go green as the chunk's acceptance
@account n=12 entity=ci-integration#3.3 v=carried
why: G3 runs over the edited Cargo.toml and workflows, and G4 is a gate the acceptance requires green on Linux
@account n=13 entity=ci-integration#4.1 v=carried
why: the step order names the per-OS perf job whose Windows and macOS legs the chunk removes
@account n=14 entity=ci-integration#5.1 v=not_in_scope
why: v1-47's job-level AGENT_RUN_KEEP_HOMES reading on the Linux job only keeps holding, a re-check every change owes
@account n=15 entity=slo-invariants-telemetry-budgets#1.1 v=carried
why: the removal leaves the panic path Linux-only, with the cfg(unix) panic-frame twins made unconditional, and the invariant must still hold under G2
@account n=16 entity=slo-invariants-telemetry-budgets#2.1 v=carried
why: the session-end budget is judged by gate --require perf on the perf job that keeps only its Linux leg
@account n=17 entity=slo-invariants-telemetry-budgets#2.2 v=carried
why: the spine budget's timed path names the macOS carve-out the removal retires, and its Linux leg stays gated
@account n=18 entity=slo-invariants-telemetry-budgets#2.4 v=carried
why: the obs-overhead gate holds only with homes under target/e2e-home/, which the harness fix must keep
@account n=19 entity=obs-anti-patterns#1.1 v=carried
why: the print-macro ban counts the human::refuse call sites, which change if the Windows .cmd/.bat refusal is removed
