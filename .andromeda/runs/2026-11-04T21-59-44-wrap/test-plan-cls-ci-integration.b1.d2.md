@answer n=1 entity=ci-integration#1.1 v=no
why: the lint commands, the orphans and rmcp checks and the Windows and macOS clippy recipes are unchanged, and no CI step, Cargo.toml or Cargo.lock moved
@answer n=2 entity=ci-integration#1.2 v=no
why: the supply-chain checks read Cargo.lock, deny.toml and the workflow files, none of which the session moved or whose bans it changed
@answer n=3 entity=ci-integration#1.3 v=no
why: agent-run run --unit and its nextest process-per-test mode still hold on the changed .config/nextest.toml, since only the ci profile's 20 s kill line moved
@answer n=4 entity=ci-integration#1.4 v=no
why: agent-run run --integration and its parallel mode still hold on the changed .config/nextest.toml, since only the ci profile's 20 s kill line moved, and the stage names no binary
@answer n=5 entity=ci-integration#1.5 v=no
why: run --e2e still runs on all three OSes, and the Unix-gated dim witness and the census binary sit inside that run without changing what the stage says
@answer n=6 entity=ci-integration#1.6 v=no
why: the coverage run, the per-OS test job and the conpty verification step are unchanged, and nextest still runs under the instrumented run
@answer n=7 entity=ci-integration#1.7 v=no
why: the mutants jobs, their matrix count and their target files are unchanged, since the session moved no src/ file and no windows-mutants.yml line
@answer n=8 entity=ci-integration#1.8 v=no
why: the perf run, its hyperfine pins and its gate are unchanged, and the session touched no perf row or workflow file
@answer n=9 entity=ci-integration#1.9 v=no
why: the MSRV unit run and cargo check are unchanged, with no toolchain pin or workspace rust-version moved
@answer n=10 entity=ci-integration#1.10 v=no
why: the fuzz replay and the nightly fuzz job are unchanged, and no fuzz target or corpus moved
@answer n=11 entity=ci-integration#1.11 v=no
why: release-check still judges the release build's own artifacts, and the session added no src/ change or test-only feature to that build
@answer n=12 entity=ci-integration#1.12 v=yes mode=add-to
why: F8, scripts/junit-os-sets.py is an operator-pass reader of the per-OS junit and never a CI step, so it joins this stage's list of junit readers
