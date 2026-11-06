@answer n=1 entity=ci-integration#1.1 v=no
why: the diag upload keeps its matrix.os name and secret-scan gate, which the task says stands in ci.yml
@answer n=2 entity=ci-integration#1.2 v=no
why: the harness upload keeps its matrix.os name, which the task says stands
@answer n=3 entity=ci-integration#1.3 v=yes mode=contradict
why: the session retires the per-OS junit artifacts, so the junit-${{ matrix.os }} upload no longer stands
@answer n=4 entity=ci-integration#1.4 v=yes mode=contradict
why: the session retires the three junit-<os> artifacts and the junit-os-sets.py reader that reads them
@answer n=5 entity=ci-integration#1.5 v=no
why: the perf uploads keep their matrix.os names and the perf budgets and hyperfine gates are untouched
@answer n=6 entity=ci-integration#1.6 v=no
why: OTel trace export is N/A and the session adds no exporter or sink
@answer n=7 entity=ci-integration#2.1 v=yes mode=stale
why: the session removes the OS conditions on the Lint probes steps, so the "Linux leg" qualifier no longer names a leg of that step
@answer n=8 entity=ci-integration#2.2 v=no
why: the unit tests still parse lines against the schema file by path and make no enum or count claim the session moves
@answer n=9 entity=ci-integration#2.3 v=no
why: the integration-test homes and the diag-<os> upload keep their names, and the hyperfine perf checks are untouched
@answer n=10 entity=ci-integration#2.4 v=no
why: the E2E gates G2 and G4 and the agent-run artifacts are unchanged by the session
@answer n=11 entity=ci-integration#2.5 v=yes mode=stale
why: contract_os_gated_tests.rs is reached and its GATES rows and eight-case self-check may have moved with the removed Windows and macOS gates
@answer n=12 entity=ci-integration#2.6 v=yes mode=contradict
why: the session deletes windows-mutants.yml, the Windows form this entity describes
@answer n=13 entity=ci-integration#3.1 v=no
why: the rg install step and the G1 and G3 commands name no OS leg and the session does not change them
@answer n=14 entity=ci-integration#3.2 v=no
why: the G2 probe plants a synthetic panic record, not a line of hook.rs, and the seam exemption file is not reached
@answer n=15 entity=ci-integration#3.3 v=no
why: G4 checks against the schema files by path, and its scope and step placement are not changed by the session
@answer n=16 entity=ci-integration#4.1 v=no
why: the perf job keeps its matrix.os form and the session does not move the test, gate or KEEP_HOMES ordering
@answer n=17 entity=ci-integration#5.1 v=no
why: the secret scan stays if: always() after each job's run and AGENT_RUN_KEEP_HOMES stays 1 on the job
