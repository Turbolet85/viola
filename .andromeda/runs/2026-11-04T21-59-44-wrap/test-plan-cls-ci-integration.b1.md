@answer n=1 entity=ci-integration#1.1 v=no
why: the Lint command set and its CI steps are unchanged, and the new test files sit under the clippy --all-targets run that CI passed green.
@answer n=2 entity=ci-integration#1.2 v=no
why: deny.toml, Cargo.toml and Cargo.lock did not move, and no supply-chain step names a test file the session changed.
@answer n=3 entity=ci-integration#1.3 v=no
why: the unit run command and its nextest process-per-test parallelism still hold, and the entity names no test file the session moved.
@answer n=4 entity=ci-integration#1.4 v=no
why: the integration run command and its parallelism are unchanged, and the entity names no test file or binary the session moved.
@answer n=5 entity=ci-integration#1.5 v=no
why: run --e2e on all 3 OSes still holds, since the new contract binary and the Unix witness run inside the same command and matrix, and the entity names no binary.
@answer n=6 entity=ci-integration#1.6 v=no
why: the coverage run and its per-OS test job still hold, the conpty_sideload [[test]] entry did not change, and the new binaries are covered with the rest.
@answer n=7 entity=ci-integration#1.7 v=no
why: the mutants scope is over src/ files, which the session did not move.
@answer n=8 entity=ci-integration#1.8 v=no
why: the perf run, its hyperfine pins and its gate are untouched, and no src/ or fixture file moved.
@answer n=9 entity=ci-integration#1.9 v=no
why: the MSRV toolchain pin and its unit run are unchanged, and the session moved no workspace rust-version or Cargo file.
@answer n=10 entity=ci-integration#1.10 v=no
why: the fuzz replay runs over the fuzz/ corpus with cargo-fuzz, and the session touched neither.
@answer n=11 entity=ci-integration#1.11 v=no
why: release-check builds only the viola binary and judges its artifacts, and no src/ or Cargo file moved, so the release build is unchanged.
@answer n=12 entity=ci-integration#1.12 v=yes mode=add-to
why: F8, scripts/junit-os-sets.py is an operator-pass reader of the per-OS junit artifacts beside this gate's junit-*.xml reading, never a CI step.
