@answer n=1 entity=ci-integration#1.1 v=no
why: the diag-os artifact naming is re-read on ci#42288148289 as diag-ubuntu-latest and the section 9 re-read moved no rule
@answer n=2 entity=ci-integration#1.2 v=no
why: the harness-os artifact rests on the matrix.os naming that still resolves on the ubuntu-latest-only matrix, and the section 9 re-read moved no rule
@answer n=3 entity=ci-integration#1.3 v=no
why: the nextest reached change is its timing comments, and the junit-ubuntu-latest artifact is confirmed on ci#42288148289 so the junit.xml profile path still holds
@answer n=4 entity=ci-integration#1.4 v=no
why: the perf-os and diag-perf-os artifacts still resolve on the ubuntu-latest matrix, and run --perf and gate --require perf are green on ci#42288148289 with budgets unchanged
@answer n=5 entity=ci-integration#1.5 v=no
why: the OTel row stays N/A because v1 still has no exporter and the session added no exporter, obs tier or egress surface
@answer n=6 entity=ci-integration#2.1 v=no
why: the cargo deny bans claim holds on the changed deny.toml since the report reads advisories, bans, licenses and sources ok, and clippy and the probes are green on ci#42288148289
@answer n=7 entity=ci-integration#2.2 v=no
why: the in-memory writer unit tests and their diag-line schema check still hold on the changed crates since the seven-job run on ci#42288148289 is green and the session changed no schema
@answer n=8 entity=ci-integration#2.3 v=no
why: the viola homes under target/e2e-home and the chaos_feed_panic outside-scan carve-out still hold on the changed test files since G2 and G4 are green on ci#42288148289
@answer n=9 entity=ci-integration#2.4 v=no
why: the agent-run logs and status reads and the G2 and G4 gates are unchanged and green on ci#42288148289
@answer n=10 entity=ci-integration#2.5 v=no
why: the census rule and its windows and macOS predicate fault are unchanged and the census stays green on ci#42288148289 with the Windows and macOS gated tests removed
@answer n=11 entity=ci-integration#2.6 v=no
why: no push or pull-request mutation job exists, the ci#42288148289 run carries only junit, diag, diag-perf, perf and supply-chain artifacts, and the section 9 re-read moved no rule
@answer n=12 entity=ci-integration#3.1 v=no
why: the G1 bare-instrument gate and its rg PCRE2 step are unchanged and green on ci#42288148289 over the changed workspace
@answer n=13 entity=ci-integration#3.2 v=no
why: the G2 seam exemption and its probe, including the src/cmd/hook.rs planted-panic case, still read as stated on the changed files since G2 is green on ci#42288148289
@answer n=14 entity=ci-integration#3.3 v=no
why: the G4 schema-conformance step and both schemas are unchanged and green on ci#42288148289
@answer n=15 entity=ci-integration#4.1 v=no
why: the test and perf job steps, the homes kept until the gates run and the chaos home excluded as before are unchanged and green on ci#42288148289
@answer n=16 entity=ci-integration#5.1 v=no
why: the secret-scan reading after each job's end-to-end run and the viola-chaos carve-out are unchanged and green on ci#42288148289, and v1-47 is not touched by the session
