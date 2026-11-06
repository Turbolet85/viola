@answer n=1 entity=ci-integration#1.1 v=no
why: the diag-${{ matrix.os }} upload is not in the removed set, and the session keeps every artifact name in matrix.os form.
@answer n=2 entity=ci-integration#1.2 v=no
why: the harness-${{ matrix.os }} upload and its chunk.diff exclusion are not removed, and the session keeps artifact names in matrix.os form.
@answer n=3 entity=ci-integration#1.3 v=yes mode=contradict
why: the session retires the three per-OS junit-os artifacts, so the junit-${{ matrix.os }} upload this row describes is removed from ci.yml.
@answer n=4 entity=ci-integration#1.4 v=yes mode=contradict
why: scripts/junit-os-sets.py and the three junit-os artifacts it reads are deleted.
@answer n=5 entity=ci-integration#1.5 v=no
why: the perf-${{ matrix.os }} and diag-perf-${{ matrix.os }} uploads stay in matrix.os form on the ubuntu-latest matrix.
@answer n=6 entity=ci-integration#1.6 v=no
why: OTel trace export stays absent, and the session adds no exporter or sink.
@answer n=7 entity=ci-integration#2.1 v=no
why: the Lint probes stay on the Linux leg and the cargo deny check keeps the named bans, which the deny.toml target change does not touch.
@answer n=8 entity=ci-integration#2.2 v=no
why: the unit tests still parse emitted lines against schemas/diag-line.v1.json, which is the claim the row makes.
@answer n=9 entity=ci-integration#2.3 v=no
why: the integration tests and the diag-<os> upload stand, and the carve-out tests/chaos_feed_panic.rs is not reached.
@answer n=10 entity=ci-integration#2.4 v=no
why: agent-run logs and status and the G2 and G4 gates are unchanged.
@answer n=11 entity=ci-integration#2.5 v=yes mode=stale
why: tests/contract_os_gated_tests.rs is reached whole, and the removed Windows and macOS gated tests change the GATES census it checks, so its gated set and self-check count can have moved.
@answer n=12 entity=ci-integration#2.6 v=yes mode=contradict
why: the Windows form of the mutation audit, windows-mutants.yml, is deleted, so the panic_frames Windows job this row cites is removed with it.
@answer n=13 entity=ci-integration#3.1 v=no
why: the rg PCRE2 install step and the G1 command are unchanged, since the Gates G1 to G4 definitions stay as they are.
@answer n=14 entity=ci-integration#3.2 v=no
why: G2 runs unchanged in the test and perf jobs, and its probe plants its own lines, so the reached src/cmd/hook.rs does not change its cases.
@answer n=15 entity=ci-integration#3.3 v=no
why: G4 still checks every line against schemas/diag-line.v1.json through the same schema-conformance step and tests-owned validator.
@answer n=16 entity=ci-integration#4.1 v=yes mode=stale
why: ci.yml is reached whole and drops its pwsh shim and ConPTY vendor verification steps, which may sit in the per-OS perf job whose step list this row gives.
@answer n=17 entity=ci-integration#5.1 v=no
why: the secret scan stays green in the test and perf jobs with its rules unchanged, and AGENT_RUN_KEEP_HOMES on the job and the chaos home carve-out stand.
