@answer n=1 entity=ci-integration#1.1 v=yes
why: the scope names the diag-<os> upload gated on secret-scan success, which the remaining Linux jobs keep
@answer n=2 entity=ci-integration#1.2 v=yes
why: the scope names the harness-<os> capture on failure() and its upload gated on secret-scan success
@answer n=3 entity=ci-integration#1.3 v=yes
why: the scope names the junit-<os> uploads gated on secret-scan success for the remaining test jobs
@answer n=4 entity=ci-integration#1.4 v=no
why: scripts/junit-os-sets.py is operator tooling the chunk neither edits nor names, and its three-OS input is the wrap's per-OS amendment
@answer n=5 entity=ci-integration#1.5 v=yes
why: the scope names the per-OS perf job's run --perf timing of the five hook rows and their scan-gated exports
@answer n=6 entity=ci-integration#1.6 v=no
why: the row says OTel trace export is not applicable, and the chunk adds no exporter and no rule it must honour
@answer n=7 entity=ci-integration#2.1 v=yes
why: the scope names clippy disallowed-macros, the print and dbg bans, scripts/lint-probes.sh, G1 and cargo deny with its banned obs crates
@answer n=8 entity=ci-integration#2.2 v=no
why: in-memory writer unit tests are not reached, since the chunk removes only Windows-gated tests and the Linux schemas keep their values
@answer n=9 entity=ci-integration#2.3 v=yes
why: the scope names the e2e-home rule that integration-test viola homes stay under target/e2e-home/ so G2, G4 and the scan cover them
@answer n=10 entity=ci-integration#2.4 v=yes
why: the scope keeps the E2E agent-run logs and status capture and the G2 and G4 gates on the Linux jobs
@answer n=11 entity=ci-integration#2.5 v=yes
why: the census of OS-gated tests is itself an edited unit, which the scope re-points or retires
@answer n=12 entity=ci-integration#2.6 v=yes
why: the scope deletes windows-mutants.yml and keeps the run --mutants gate on Linux, so the Linux mutation gate rules
@answer n=13 entity=ci-integration#3.1 v=yes
why: the scope keeps G1 and G3 verbatim in ci.yml, so the PCRE2 ripgrep install step before them must stay
@answer n=14 entity=ci-integration#3.2 v=yes
why: the scope keeps G2 in the remaining test and perf jobs and names the fake-agent seam's forced panic exemption
@answer n=15 entity=ci-integration#3.3 v=yes
why: the scope keeps G4 after the last test step under if: always() on the Linux jobs
@answer n=16 entity=ci-integration#4.1 v=yes
why: the scope names the Linux test step order with G2 and G4 under if: always() and the homes kept
@answer n=17 entity=ci-integration#5.1 v=yes
why: the scope names the secret-scan step under if: always() with AGENT_RUN_KEEP_HOMES set and the chaos home kept out
