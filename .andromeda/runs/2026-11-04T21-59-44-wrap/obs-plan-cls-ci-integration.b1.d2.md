@answer n=1 entity=ci-integration#1.1 v=no
why: the session moved no .github step and the diag-<os> upload row it describes stands unchanged
@answer n=2 entity=ci-integration#1.2 v=no
why: the harness artifact upload is in .github and the session touched no file of it
@answer n=3 entity=ci-integration#1.3 v=yes mode=add-to
why: the junit-<os> upload row stands and now carries F1, scripts/junit-os-sets.py reading junit-windows-2025, junit-macos-latest and junit-ubuntu-latest through gh run download
@answer n=4 entity=ci-integration#1.4 v=no
why: the perf artifacts and hyperfine export are untouched by a test-only change
@answer n=5 entity=ci-integration#1.5 v=no
why: OTel export is N/A and the session added no exporter
@answer n=6 entity=ci-integration#2.1 v=no
why: lint, deny and lint-probes steps are unchanged and no lint-stage file moved
@answer n=7 entity=ci-integration#2.2 v=no
why: the unit tests still emit the same lines and nextest JUnit remains their consumer, the new script only reads the artifacts
@answer n=8 entity=ci-integration#2.3 v=no
why: the only change in cli_fake_agent.rs is the sigignore reader, and the integration homes under target/e2e-home stay as stated
@answer n=9 entity=ci-integration#2.4 v=yes mode=add-to
why: the E2E layer gains F3, the census binary tests/contract_os_gated_tests.rs run through run --e2e in suite nextest-e2e
@answer n=10 entity=ci-integration#2.5 v=no
why: the mutation job and its gate are untouched and no obs-code file changed
@answer n=11 entity=ci-integration#3.1 v=no
why: the G1 rg command and its PCRE2 install step are unchanged and no .github or Cargo file moved
@answer n=12 entity=ci-integration#3.2 v=no
why: G2 and its script scripts/g2-zero-panics.sh are unchanged and the session did not touch the panic seam
@answer n=13 entity=ci-integration#3.3 v=no
why: G4 and the schema-conformance step are unchanged and no schema file moved
@answer n=14 entity=ci-integration#4.1 v=no
why: the test-step order and G2 and G4 gates stay as stated, .github was not touched, and the census binary is recorded on entity 2.4
@answer n=15 entity=ci-integration#5.1 v=no
why: the secret-scan reading after each job's run and AGENT_RUN_KEEP_HOMES stay as stated, no CI file moved
