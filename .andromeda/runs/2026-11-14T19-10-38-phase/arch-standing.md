# arch · standing conventions

- S1 · each rewritten or new test runs in its own isolated viola home · established-decisions
  decision/test-home-owner-sweep · Test homes removed by owner record only — A test home sweep removes only dirs whose recorded owner process is gone, never by age or name and never a dir without a record, and every removal deletes the owner record last. · why: a removal that stops part-way keeps the record for a later sweep
  decision/test-homes-cleaned-by-owner · Test homes cleaned by owner process — Harness and test homes are kept in CI for the obs gates, and a new test home removes only the dirs whose recorded owner process is gone, never by age or name.
  decision/test-isolated-viola-home · Each test in its own viola home — Each test runs in its own viola home (`--home`), so parallel tests never share an endpoint or log.
  decision/viola-home-resolution · Viola home resolution order — The viola home resolves from --home, then the grandparent of VIOLA_DIR, then .viola under the user home; viola run passes the home to its child only through VIOLA_DIR, so every per-home file follows it. · why: The home is the isolation point for tests: each test runs in its own home.
- S2 · each test runs against the fake agent in CI, never a live claude · ci-cd-approach
  decision/ci-against-fake-agent · CI on ubuntu-latest against the fake agent — GitHub Actions on `ubuntu-latest` runs the tests against the fake agent; the real CLI runs only in local live tests. · why: No tokens spent, no flakes in CI, and no cross-compilers (cross 0.2.5 is stale).
  decision/ci-linux-fake-agent · CI on Linux against the fake agent — CI runs on GitHub Actions ubuntu-latest against the fake agent.
  decision/ci-ubuntu-fake-agent · CI on ubuntu against the fake agent — CI builds, lints and tests on ubuntu-latest native runners against the fake agent, with SHA-pinned actions and toolchains installed by rustup.
  decision/no-live-claude-in-ci · No live claude in CI — When CI is present, the test harness's run --local-live refuses before any build, spawn or suite, and viola itself never reads the harness variables. · why: so the real claude never runs in CI
  decision/tests-against-fake-agent · Tests run against the fake agent — The workspace test suite runs against the fake agent, replaying recorded Claude fixtures.
- S3 · an OS-gated test names its reason and no test gate names another OS · cross-cutting-patterns
  decision/linux-only · Linux only — viola targets Linux only: the core on any current Linux distribution, built and CI-tested on `ubuntu-latest`, the window features on Hyprland; no Windows or macOS code, test, fixture or CI job stands in the tree. · why: The Windows and macOS support is cut, recoverable at the tag `multi-os-final`.
  decision/no-other-os-test-gates · No test gate naming another OS — A test gate whose predicate names windows or macOS is a fault even when the gates table names it. · guard: census_reads_a_gate_naming_another_os_as_a_fault_even_when_named
  decision/os-gated-tests-name-reason · OS-gated tests name their reason — Every OS-gated test names its reason in the census, and a gate whose predicate names windows or macos reads as a fault even when a row names it. · guard: contract_os_gated_tests
  decision/os-gated-tests-named · OS-gated tests named with their reason — Every test held off a runner by an OS gate is named in the gates table with a non-empty reason, and the table names no gate that no longer stands. · guard: contract_os_gated_tests: os_gated_tests_are_each_named_with_their_reason
