@answer n=1 entity=ci-integration#1.1 v=yes
why: the per-OS diag upload gated on the secret scan is named by the scope's Need 14
@answer n=2 entity=ci-integration#1.2 v=yes
why: the harness capture and its scan-gated upload on failure are named by the scope's Need 14
@answer n=3 entity=ci-integration#1.3 v=yes
why: the per-OS JUnit upload gated on the secret scan is named by the scope's Need 14
@answer n=4 entity=ci-integration#1.4 v=yes
why: the per-OS perf job whose hyperfine samples feed gate --require perf is named by the scope's Need 16
@answer n=5 entity=ci-integration#1.5 v=no
why: OTel trace export has no exporter in v1, and parity adds none (not-needed 2)
@answer n=6 entity=ci-integration#2.1 v=yes
why: the lint job on every OS (clippy disallowed-macros, print bans, lint-probes.sh, cargo deny) is named by the scope's Need 17
@answer n=7 entity=ci-integration#2.2 v=yes
why: unit tests parsing emitted lines against schemas/diag-line.v1.json are the witness that re-asserts one schema on every OS
@answer n=8 entity=ci-integration#2.3 v=no standing=S1
why: the rule that integration-test viola homes live under target/e2e-home is the standing convention every new integration test owes
@answer n=9 entity=ci-integration#2.4 v=yes
why: the E2E zero-panic gate G2 and schema gate G4 on each OS are named by the scope's Needs 11, 13 and 14
@answer n=10 entity=ci-integration#2.5 v=no
why: the mutation gate runs at the epoch boundary through the audit, and the scope names no mutation step (not-needed 7)
@answer n=11 entity=ci-integration#3.1 v=yes
why: gate G1 must be its own bash step green on the Unix OSes, and its ripgrep PCRE2 install is required for that (Need 12)
@answer n=12 entity=ci-integration#3.2 v=yes
why: gate G2 with its --probe and the seam.rs exemption is named by the scope's Need 12
@answer n=13 entity=ci-integration#3.3 v=yes
why: gate G4 against both schemas is named by the scope's Needs 12 and 13
@answer n=14 entity=ci-integration#4.1 v=yes
why: the CI step order (tests, then G2 and G4 if always, with AGENT_RUN_KEEP_HOMES=1) is named by the scope's Need 14
@answer n=15 entity=ci-integration#5.1 v=yes
why: the secret scan running after each job's end-to-end run with AGENT_RUN_KEEP_HOMES=1 is named by the scope's Need 14
