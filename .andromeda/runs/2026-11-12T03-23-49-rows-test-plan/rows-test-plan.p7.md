@row_add n=1 row=decision/send-and-wait-latency-not-timed area=10-quality-gates-coverage-targets rec=2,1
name: Send confirmation and wait wake latency not timed
rule: The `send` to `prompt-submitted` confirmation latency and the `wait` wake latency are deliberately kept out of the timed quality gates and are asserted as behaviour, not timed.
why: Both are bounded by product constants that are still open.
@row_add n=2 row=decision/boot-deadline-not-a-budget area=10-quality-gates-coverage-targets rec=3,1
name: Boot deadline is not a performance budget
rule: The 20 s `boot` deadline is a harness readiness timeout, deliberately not a product performance budget in the quality gates.
@row_add n=3 row=decision/hyperfine-max-perf-gate area=10-quality-gates-coverage-targets rec=4,9,53
name: hyperfine max is the performance gate
rule: Performance budgets, hook deadlines among them, are gated by hyperfine on `max`, never on `mean`, and a budget regression fails the build; criterion 0.8.2 is not a gate.
why: criterion 0.8.2 does not exit non-zero on a regression.
@row_add n=4 row=decision/hook-deadlines-asserted-at-unit-speed area=10-quality-gates-coverage-targets rec=4
name: Hook deadlines asserted at unit speed
rule: The hook deadlines are also asserted at unit speed inside the hook subprocess tests, with a monotonic `std::time::Instant` bound.
@row_add n=5 row=decision/zero-flakiness area=10-quality-gates-coverage-targets rec=5,45,48,49
name: Zero-flakiness budget
rule: Flaky tests are not tolerated: nextest `retries` stays 0, no retry-once policy exists, and a test that flakes once is quarantined at once and fixed at the root cause, the chunk staying red until it is.
why: Agent-driven development cannot tell a flake from a real bug, and retry policies mask actual failures.
@row_add n=6 row=decision/no-ignore-parking area=10-quality-gates-coverage-targets rec=5
name: No #[ignore] parking
rule: `#[ignore]` is not allowed as a parking place for a failing or flaky test.
why: Agent-driven development cannot tell a flake from a real bug.
@row_add n=7 row=decision/test-failure-fails-build area=10-quality-gates-coverage-targets rec=7,6
name: Any test failure fails the build
rule: A test failure at any level is a build failure condition.
@row_add n=8 row=decision/coverage-threshold-fails-build area=10-quality-gates-coverage-targets rec=8,6
name: Coverage below tier threshold fails the build
rule: Coverage below the threshold for its tier, measured in the `test` job on `ubuntu-latest`, is a build failure condition.
@row_add n=9 row=decision/mutants-verdict-counted-by-harness area=10-quality-gates-coverage-targets rec=10,52
name: Mutation verdict counted by the harness
rule: At a `run --mutants` the harness counts missed, timeout, unviable and caught outcomes itself, leaving host-excluded missed mutants out; any other missed mutant, any timed-out mutant, or more unviable than caught mutants is red.
why: The cargo-mutants exit code and summary cannot be trusted alone: exit 3 masks exit 2, and the summary counts a host-excluded mutant as missed.
@row_add n=10 row=decision/lint-gates-fail-build area=10-quality-gates-coverage-targets rec=11,6
name: Lint and audit gates fail the build
rule: A lint, clippy, cargo-deny, zizmor or cargo-modules orphans failure is a build failure condition.
@row_add n=11 row=decision/no-test-binary-in-release area=10-quality-gates-coverage-targets rec=11,6
name: No test-only binary in a release build
rule: A release build carrying a test-only binary is a build failure condition.
@row_none n=1 rec=12 type=S
@row_none n=2 rec=13 type=S
@row_add n=12 row=decision/e2e-asserts-only-cross-process area=11-test-anti-patterns-never-do-these rec=14
name: E2E asserts only cross-process behaviour
rule: A case a lower layer can hold is never asserted at E2E: parser, refusal-order, threshold and clock cases belong to unit tests, contract shapes to contract tests, and E2E asserts only cross-process behaviour.
why: The test pyramid.
@row_add n=13 row=decision/every-surface-asserted area=11-test-anti-patterns-never-do-these rec=15
name: Every surface of a critical path asserted
rule: A multi-surface critical path is never asserted on one surface only; every surface an E2E scenario lists needs its own driver step and transition signal.
@row_add n=14 row=decision/no-over-mocking-integration area=11-test-anti-patterns-never-do-these rec=16
name: No over-mocking at integration level
rule: Integration tests never over-mock.
why: Over-mocking stops testing the integration.
@row_add n=15 row=decision/no-real-cli-in-ci area=11-test-anti-patterns-never-do-these rec=17
name: No real claude CLI in CI
rule: The real `claude` CLI, and `viola verify` against it, never run in CI; suites reach the real CLI only through a local-live run, and live measurement of a real session is local too.
guard: `agent-run run --local-live` refuses when `CI` is set
@row_add n=16 row=decision/fake-agent-not-proof-of-real-cli area=11-test-anti-patterns-never-do-these rec=18
name: Fake-agent run is not proof of real-CLI behaviour
rule: A green fake-agent run never counts as proof of real-CLI behaviour; a behaviour change requires a local `viola verify` fixture refresh plus the contract suite.
why: The fake agent must not drift from the real CLI.
@row_add n=17 row=decision/test-public-behaviour-only area=11-test-anti-patterns-never-do-these rec=19
name: Test public API and observable behaviour
rule: Tests assert the public API and observable behaviour, never implementation details such as private methods or internal state.
@row_add n=18 row=decision/assert-on-contract-not-fixture area=11-test-anti-patterns-never-do-these rec=20
name: Assert on contract, not fixture internals
rule: Assertions target the contract, never brittle fixture-internal values.
@row_add n=19 row=decision/literal-test-oracles area=11-test-anti-patterns-never-do-these rec=21
name: Test oracles written as literals
rule: Tests never import the product's own identity floor, ledger rows or refusal ordering as the oracle; expected values such as the 11 identity-floor names, the refusal order and exit codes are written as literals in the test.
@row_add n=20 row=decision/no-debug-string-comparison area=11-test-anti-patterns-never-do-these rec=22
name: No comparison by Debug string
rule: Refusal reasons and exit codes are never compared by their Debug string; tests assert on the serde kebab-case value and the integer exit code.
@row_add n=21 row=decision/real-on-disk-state area=11-test-anti-patterns-never-do-these rec=23
name: On-disk state is never mocked
rule: Tests never mock the on-disk state and use a real tempfile home instead.
why: The architecture has no database; this is the database-equivalent rule.
@row_add n=22 row=decision/cleanup-between-integration-tests area=11-test-anti-patterns-never-do-these rec=24
name: Cleanup between integration tests
rule: Integration tests never skip cleanup between tests.
why: Order-dependent tests hide bugs.
@row_add n=23 row=decision/endpoint-from-snapshot-only area=11-test-anti-patterns-never-do-these rec=25
name: Endpoint taken only from the snapshot
rule: Tests never use interprocess `try_overwrite` nor reuse an endpoint name across tests; the endpoint is taken only from `snapshot.json`.
@row_add n=24 row=decision/no-process-env-mutation-in-tests area=11-test-anti-patterns-never-do-these rec=26
name: No process-wide env mutation in tests
rule: Tests never set process environment variables globally; environment is set per child command.
why: Setting the process environment is unsafe in edition 2024 and races across threads.
@row_add n=25 row=decision/env-clear-keeps-profile-file area=11-test-anti-patterns-never-do-these rec=27
name: Cleared environment keeps LLVM_PROFILE_FILE
rule: A test that clears a child's environment always re-adds `LLVM_PROFILE_FILE`.
why: Without it the run silently drops out of coverage.
@row_add n=26 row=decision/no-sleep-synchronization area=11-test-anti-patterns-never-do-these rec=28
name: No sleep for synchronization
rule: Tests never synchronize with a sleep; they wait for an explicit signal such as an `events.ndjson` offset, `viola wait --after` or a status field.
@row_add n=27 row=decision/no-screen-parsing area=11-test-anti-patterns-never-do-these rec=29
name: Rendered child screen is never parsed
rule: Tests never parse the rendered child screen (the vt100 grid or PTY master output) for content; verdicts come from hook events, `events.ndjson`, channel payloads and the fake-agent receipt.
@row_add n=28 row=decision/child-exit-by-wait area=11-test-anti-patterns-never-do-these rec=30
name: Child exit detected by waiting on the handle
rule: Child exit is detected by waiting on the child handle, never by reading master EOF.
@row_add n=29 row=decision/wrapper-spawned-with-guard area=11-test-anti-patterns-never-do-these rec=31
name: Long-lived wrapper spawned with a drop guard
rule: Tests start the long-lived `viola run` wrapper by spawning it under a drop guard, never with a blocking output or status call.
why: A blocking call waits until the wrapper exits.
@row_add n=30 row=decision/no-production-data-in-tests area=11-test-anti-patterns-never-do-these rec=32
name: No production data in tests
rule: Tests never use production data; fixtures come only from `viola verify` probe text and are scrubbed of paths and usernames.
except: A `workspace.repo` naming this repository's own remote is kept.
@row_add n=31 row=decision/no-shared-mutable-state area=11-test-anti-patterns-never-do-these rec=33
name: No shared mutable state across parallel tests
rule: Parallel tests never share mutable state.
@row_add n=32 row=decision/seeded-generators area=11-test-anti-patterns-never-do-these rec=34
name: Seed-controlled generators
rule: Non-deterministic generators are always seed-controlled, and `proptest-regressions/` is committed.
@row_add n=33 row=decision/state-files-never-hand-written area=11-test-anti-patterns-never-do-these rec=35
name: State files are never hand-written
rule: Every booted-wrapper test takes `ledger/stamps.json` from `viola verify`, `snapshot.json` from the wrapper and `budget.json` from `viola hook statusline`, never writing them by hand.
except: A reader's own unit cases may plant a malformed state file by writing through the writer and then padding, replacing or re-versioning its bytes.
@row_add n=34 row=decision/no-committed-test-secrets area=11-test-anti-patterns-never-do-these rec=36
name: No test dirs, env files or tokens committed
rule: `--home` test directories, `.env` files and tokens are never committed.
@row_add n=35 row=decision/no-mocking-unowned-code area=11-test-anti-patterns-never-do-these rec=37
name: No mocking of what is not owned
rule: Libraries and the language runtime are never mocked without an abstraction layer.
@row_add n=36 row=decision/no-over-stubbing area=11-test-anti-patterns-never-do-these rec=38
name: No over-stubbing
rule: Tests never over-stub.
why: Over-stubbing stops testing real behaviour.
@row_add n=37 row=decision/no-runtime-monkey-patching area=11-test-anti-patterns-never-do-these rec=39
name: No runtime monkey-patching
rule: Production code is never monkey-patched at runtime; dependency injection is used instead.
@row_add n=38 row=decision/no-tokio-failpoint-crate area=11-test-anti-patterns-never-do-these rec=40
name: No failpoint crate pulling tokio
rule: No failpoint crate that pulls tokio into the sync crates is introduced; fault seams go through mockall doubles of existing traits plus process and file faults.
@row_add n=39 row=decision/real-hook-in-e2e area=11-test-anti-patterns-never-do-these rec=41
name: Real viola hook in E2E
rule: `viola hook` is never stubbed in E2E; the fake agent invokes the real pinned binary through the plugin files `run` wrote.
@row_add n=40 row=decision/tests-run-in-parallel area=11-test-anti-patterns-never-do-these rec=42
name: Tests run in parallel
rule: Tests are never serialised with a single thread; isolation comes from the per-test home.
except: The fuzz stages are single jobs, by design.
@row_add n=41 row=decision/rust-cache-only-in-ci area=11-test-anti-patterns-never-do-these rec=43,47
name: rust-cache is the only cache, CI only
rule: Swatinem/rust-cache is the only cache in `ci.yml`, and no release workflow uses it.
@row_add n=42 row=decision/coverage-ignore-regex-not-widened area=11-test-anti-patterns-never-do-these rec=44
name: Coverage ignore regex is never widened
rule: The coverage ignore-filename regex, covering the fake agent, the E2E crate, test support and fuzz trees, is never widened to pass a gate without a Decisions Log entry first.
why: Widening it is a back door to lowering the coverage thresholds.
except: The path-separator class kept in the regex is not a widening, because the four trees are unchanged.
@row_add n=43 row=decision/sha-pinned-actions area=11-test-anti-patterns-never-do-these rec=46
name: SHA-pinned Actions
rule: Every Action is SHA-pinned, never referenced by a mutable tag, with `permissions: {}`.
@row_add n=44 row=decision/coverage-threshold-never-lowered area=11-test-anti-patterns-never-do-these rec=50
name: Coverage threshold never lowered to pass
rule: The coverage threshold is never lowered to pass a build.
why: Raise quality, not the bar.
@row_add n=45 row=decision/no-skipping-quality-gates area=11-test-anti-patterns-never-do-these rec=51
name: Quality gates never skipped
rule: Quality gates are never skipped, not even once.
@row_add n=46 row=decision/no-human-reviewed-visual-regression area=11-test-anti-patterns-never-do-these rec=54
name: No human-reviewed visual regression
rule: Percy, Chromatic, Applitools and other visual regression with human review are never used.
@row_add n=47 row=decision/no-manual-smoke-step area=11-test-anti-patterns-never-do-these rec=55
name: No manual smoke step
rule: No manual smoke step, such as a developer verifying before deploy, is part of the pipeline.
