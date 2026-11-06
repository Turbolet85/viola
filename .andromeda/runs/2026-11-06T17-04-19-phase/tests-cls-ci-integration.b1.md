@answer n=1 entity=ci-integration#1.1 v=yes
why: the lint gates (fmt, clippy -D warnings, per-crate tokio-free check, orphans gate and probe, rmcp features) are gates the chunk re-reads green after the portable-pty move

@answer n=2 entity=ci-integration#1.2 v=yes
why: the supply-chain job, deny-probes 14/14, the sole-root tokio ban and zizmor are where RUSTSEC-2026-0441 must clear with no ignore-list widening

@answer n=3 entity=ci-integration#1.3 v=yes
why: the unit run is the test gate the chunk re-reads green over the viola-pty consumers and the edited Windows-remnant code

@answer n=4 entity=ci-integration#1.4 v=yes
why: the integration run carries the real openpty spawn, exit-on-handle and resize tests that a viola-pty dependency move must keep green

@answer n=5 entity=ci-integration#1.5 v=yes
why: the e2e run covers the viola-e2e harness whose EXE_SUFFIX joins are edited and whose commands must still print one JSON document

@answer n=6 entity=ci-integration#1.6 v=yes
why: the test job's coverage run and its junit feed the coverage gate at lines 85, functions 95 and regions 80 that the chunk re-reads

@answer n=7 entity=ci-integration#1.7 v=no
why: the chunk runs no mutation gate (task not_needed item 14) and CI has no mutation job for it to re-read

@answer n=8 entity=ci-integration#1.8 v=yes
why: the perf job is one of CI's seven jobs that the chunk re-reads green at the wrap

@answer n=9 entity=ci-integration#1.9 v=yes
why: the MSRV 1.96 job is named by the task among the seven CI jobs that must read green

@answer n=10 entity=ci-integration#1.10 v=yes
why: the fuzz corpus replay is named by the task among the seven CI jobs that must read green

@answer n=11 entity=ci-integration#1.11 v=yes
why: the release-check line, probes 5/5 refused with a clean control, is a named line the chunk re-reads green

@answer n=12 entity=ci-integration#1.12 v=yes
why: viola-harness gate reads the junit and llvm-cov summary that enforce the coverage floors the test job must hold after the COVERAGE_FLOORS edit

@answer n=13 entity=ci-integration#1.13 v=yes
why: the 20 s nextest kill bounds the ci-profile test gate the chunk re-reads green, and the pty tests it touches run under that bound

@answer n=14 entity=ci-integration#1.14 v=yes
why: it records the RUSTSEC-2026-0441 red on portable-pty =0.8.1 that this chunk owns clearing without widening the ignore list
