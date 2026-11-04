@answer n=1 entity=ci-integration#1.1 v=yes
why: the lint job over matrix.os is re-read green on ubuntu-latest and macos-latest, with its fmt, clippy, orphans and rmcp steps
@answer n=2 entity=ci-integration#1.2 v=no
why: supply-chain is one ubuntu job outside the test, perf and lint jobs the chunk re-reads, and the chunk adds no crate or ban
@answer n=3 entity=ci-integration#1.3 v=no
why: the unit row is the local agent-run equivalent that CI replaces with the coverage run (1.6), and its process-per-test isolation is one every new unit test owes equally
@answer n=4 entity=ci-integration#1.4 v=no
why: the integration row is the local equivalent that CI replaces with the coverage run (1.6), and its suite and isolation are owed equally by every integration test
@answer n=5 entity=ci-integration#1.5 v=yes
why: the E2E row runs agent-run run --e2e on all three OSes, which is the chunk's fake-agent suite on windows-2025, ubuntu-latest and macos-latest
@answer n=6 entity=ci-integration#1.6 v=yes
why: the per-OS test job's one instrumented coverage run and its gate --require coverage,doctest are where the Unix tests and the per-OS coverage gate are counted
@answer n=7 entity=ci-integration#1.7 v=no
why: mutation has no push or pull-request job, windows-mutants is Windows-only and report-only, and the chunk's mutants kill sits in the mutants profile rather than this row
@answer n=8 entity=ci-integration#1.8 v=yes
why: the per-OS perf job with its G2 probe and check, the secret-scan gated uploads and gate --require perf are re-read on each OS, timing the hook rows against SPINE_DEADLINE
@answer n=9 entity=ci-integration#1.9 v=no
why: the 1.96 MSRV floor is a workspace-wide rule every change owes equally, and the chunk changes no rust-version or toolchain
@answer n=10 entity=ci-integration#1.10 v=no
why: fuzz-replay is a single ubuntu job, and the chunk adds no parser and no fuzz target
@answer n=11 entity=ci-integration#1.11 v=yes
why: the per-OS release job refuses a build carrying test-only features, so the chunk's Unix test seams (injected euid, fake-agent openpty spawn) must stay out of the viola-only release binary
@answer n=12 entity=ci-integration#1.12 v=yes
why: viola-harness gate exits non-zero on the §10 breaches, so the per-OS coverage gate (85, 95, 80) and the perf gate the chunk must pass in each OS job are enforced here
