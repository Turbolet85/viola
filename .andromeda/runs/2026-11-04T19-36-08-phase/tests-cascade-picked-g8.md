@account n=1 entity=ci-integration#1.1 v=carried
why: the per-OS lint job the chunk re-reads green on ubuntu-latest and macos-latest, and the dev-host macOS-target clippy that is no gate entry
@account n=2 entity=ci-integration#1.3 v=not_in_scope
why: the local unit row, which CI's one instrumented coverage run replaces; a re-check every change owes
@account n=3 entity=ci-integration#1.4 v=not_in_scope
why: the local integration row, which CI's one instrumented coverage run replaces; a re-check every change owes
@account n=4 entity=ci-integration#1.5 v=carried
why: the fake-agent E2E on all three OSes is the chunk's own same-suite obligation
@account n=5 entity=ci-integration#1.6 v=carried
why: the per-OS test job and its gate --require coverage,doctest are what the chunk re-reads green on both Unix runners
@account n=6 entity=ci-integration#1.7 v=not_in_scope
why: the dispatch-only Windows mutation leg; the chunk re-reads no mutation job and adds none
@account n=7 entity=ci-integration#1.8 v=carried
why: the per-OS perf job with its G2 and schema-check steps is re-read green on ubuntu-latest and macos-latest
@account n=8 entity=ci-integration#1.11 v=not_in_scope
why: the release job, which the chunk does not re-read (only test, perf and lint); a re-check every change owes
@account n=9 entity=ci-integration#1.12 v=carried
why: it names which jobs carry G1/G3 and G2 and the gate verdict, the obs gates the chunk re-reads per OS
@account n=10 entity=quality-gates-coverage-targets#1.1 v=carried
why: the measured profraw mechanisms rule over the kill-shaped tests the chunk reads on Unix runners inside the coverage run
@account n=11 entity=quality-gates-coverage-targets#2.0 v=carried
why: the two real-cargo-mutants harness tests compiled out on macOS are a per-OS gate the scope requires to carry its reason
@account n=12 entity=quality-gates-coverage-targets#3.1 v=carried
why: the perf session and its Unix-only timed checks are what the perf job re-measures on ubuntu-latest and macos-latest
