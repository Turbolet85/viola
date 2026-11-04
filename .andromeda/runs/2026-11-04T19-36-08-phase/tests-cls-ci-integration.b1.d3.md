@answer n=1 entity=ci-integration#1.1 v=yes
why: the per-OS lint job is one the scope requires green on ubuntu-latest and macos-latest beside windows-2025.
@answer n=2 entity=ci-integration#1.2 v=no
why: the supply-chain job is a single ubuntu job the scope does not name, so the chunk does not reach it.
@answer n=3 entity=ci-integration#1.3 v=yes
why: the Unit tier carries the unit cases the scope gates per OS, and its pwsh-on-Windows split is the runner the Unix parity rides.
@answer n=4 entity=ci-integration#1.4 v=yes
why: the v1-40 integration-tier witness the chunk brings to Unix runs in the Integration tier this stage names.
@answer n=5 entity=ci-integration#1.5 v=yes
why: the E2E stage on all three OSes is the full fake-agent suite the chunk makes the same on every runner.
@answer n=6 entity=ci-integration#1.6 v=yes
why: the per-OS test job's one instrumented --coverage run and its gate --require coverage,doctest are the per-OS coverage gate the scope enforces.
@answer n=7 entity=ci-integration#1.7 v=yes
why: the scope gates the cargo-mutants harness tests by cfg, and this stage's Windows-gated mutants jobs run over exactly those Windows-gated files.
@answer n=8 entity=ci-integration#1.8 v=yes
why: the per-OS perf job times the hook rows against SPINE_DEADLINE and runs G2, schema-check and the scan-gated uploads, all named by the scope.
@answer n=9 entity=ci-integration#1.9 v=no
why: the ubuntu MSRV job on toolchain 1.96 is not a job the scope names, and the chunk changes no toolchain pin.
@answer n=10 entity=ci-integration#1.10 v=no
why: the fuzz-replay job is an ubuntu-only corpus replay, and the chunk adds no parser and no fuzz target.
@answer n=11 entity=ci-integration#1.11 v=no
why: the release job's build and release-check are not named by the scope, and the chunk changes no release artifact.
@answer n=12 entity=ci-integration#1.12 v=yes
why: the viola-harness gate --require in each OS job, exiting non-zero on any §10 breach, is the per-OS gate the scope enforces.
