@answer n=1 entity=ci-integration#1.1 v=yes
why: the lint job runs over matrix.os and is one of the test, perf and lint jobs the chunk re-reads green on ubuntu-latest and macos-latest
@answer n=2 entity=ci-integration#1.2 v=no
why: the supply-chain job is a single ubuntu job outside the matrix, and its deny, tokio-ban and zizmor checks are re-checks every change owes, not the test, perf and lint re-read
@answer n=3 entity=ci-integration#1.3 v=yes
why: the unit tier's per-OS invocation (agent-run.sh on Unix, pwsh on Windows) is where the chunk's Unix unit cases and its openpty seam unit cases run
@answer n=4 entity=ci-integration#1.4 v=yes
why: the integration tier is where the v1-40 integration-tier witness (stale viola list row dim under the outer PTY) lives as the first Unix fake-agent consumer
@answer n=5 entity=ci-integration#1.5 v=yes
why: the E2E stage runs the fake-agent E2E on all three OSes, which scope item 2 names as the full fake-agent suite on windows-2025, ubuntu-latest and macos-latest
@answer n=6 entity=ci-integration#1.6 v=yes
why: the test job's one instrumented --coverage run and its gate --require coverage,doctest are the per-OS coverage gate (lines 85, functions 95, regions 80) the scope names in each OS job
@answer n=7 entity=ci-integration#1.7 v=yes
why: the scope's mutants profile 10 s kill, which keeps the v1-40 witness inside the stale-threshold bound, is carried by the on-demand agent-run run --mutants call this entity states
@answer n=8 entity=ci-integration#1.8 v=yes
why: the per-OS perf job (matrix.os, G2, schema-check, gate --require perf) is one the chunk re-reads green on ubuntu-latest and macos-latest
@answer n=9 entity=ci-integration#1.9 v=no
why: the MSRV job is an ubuntu-only toolchain floor check that every Rust change owes and that the scope does not name
@answer n=10 entity=ci-integration#1.10 v=no
why: the fuzz-replay stage covers parsers and fuzz targets, and the task marks the nightly fuzz runs not needed because the chunk adds no parser and no fuzz target
@answer n=11 entity=ci-integration#1.11 v=no
why: the release job checks the production artifact's feature set, which the chunk's test-only seams do not reach, and the scope does not name a release job
@answer n=12 entity=ci-integration#1.12 v=yes
why: the viola-harness gate reads the junit, llvm-cov and hyperfine outputs and enforces the per-OS coverage gate and the G1 to G4 obs gates the scope names
