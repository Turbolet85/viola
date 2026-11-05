@answer n=1 entity=ci-integration#1.1 v=yes
why: the lint row's per-OS matrix wording and the dev-host Windows- and macOS-target check-only clippy recipes, which task item 16 says lose their purpose
@answer n=2 entity=ci-integration#1.2 v=yes
why: the supply-chain row's deny.toml, per-crate tokio ban and deny-probes 14/14 are re-run after the Windows dependencies and their entries leave (task item 17)
@answer n=3 entity=ci-integration#1.3 v=yes
why: the Unit row's pwsh agent-run.ps1 run on windows leaves with the Windows shim steps (task item 6)
@answer n=4 entity=ci-integration#1.4 v=no
why: the Integration row names no OS and no removed unit; agent-run run --integration is unchanged on Linux
@answer n=5 entity=ci-integration#1.5 v=yes
why: the E2E row runs agent-run run --e2e on all 3 OSes as a per-OS matrix, which the chunk shrinks to ubuntu
@answer n=6 entity=ci-integration#1.6 v=yes
why: the coverage job's windows-2025 ConPTY vendor verification and probe steps, and the conpty_sideload coverage, leave with vendor/conpty and the Windows job (task item 2)
@answer n=7 entity=ci-integration#1.7 v=yes
why: windows-mutants.yml, its ten windows-2025 mutants jobs and their agent-run.ps1 run leave whole (task item 3)
@answer n=8 entity=ci-integration#1.8 v=yes
why: the Perf row's per-OS perf (matrix.os) job is one of the Windows and macOS jobs the chunk removes from ci.yml (task prose)
@answer n=9 entity=ci-integration#1.9 v=yes
why: the ubuntu msrv job (Rust 1.96, unit suite) must be green after the e2e-home floor fix (task item 18)
@answer n=10 entity=ci-integration#1.10 v=yes
why: the nightly fuzz job's time-boxed cargo fuzz runs stay on ubuntu, a behaviour the scope names (task item 4) while nightly.yml's Windows and macOS jobs leave
@answer n=11 entity=ci-integration#1.11 v=yes
why: the ci.yml release job runs per OS, and its Windows and macOS legs leave with the release job the scope removes (task prose)
@answer n=12 entity=ci-integration#1.12 v=yes
why: the local pre-push gate's check of host tools against ci.yml's pins is a behaviour the scope names (task item 24), and the workflow list names windows-mutants.yml, which leaves whole; the viola-harness gate line alone does not bind
@answer n=13 entity=ci-integration#1.13 v=yes
why: the 27-binary count on the stamped-home filter line may name the Windows-only test binaries the chunk removes (conpty_sideload, contract_windows_mutation_scope), so the count must be re-checked
@answer n=14 entity=ci-integration#1.14 v=no
why: the per-OS junit reader is an operator-pass tool, not a CI or harness step, and no unit it names is reached by the chunk
@answer n=15 entity=ci-integration#1.15 v=no
why: it records past per-OS junit counts at ci#41918694243; the chunk neither re-measures them nor edits the units behind them
