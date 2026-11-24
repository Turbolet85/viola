# tests · standing conventions

- S1 · each new test function follows the test naming, runs in parallel with its own fresh home, and waits only on logged events, never sleeps or retries · 2-test-strategy · test-strategy
  contract/harness-session/session-id · harness session · session id — The session id is the test name, the label (or main) and the pid; each session has its own home and session dir.
  decision/cleanup-between-integration-tests · Cleanup between integration tests — Integration tests never skip cleanup between tests. · why: Order-dependent tests hide bugs.
  decision/e2e-home-base-checks · Checked base of the test homes — The test-home base is a plain directory or, on Unix, a link to an absolute target accepted only when one lstat reads a real owner-only directory; a refusal panics with a fixed message naming the failed check and no path.
  decision/no-shared-mutable-state · No shared mutable state across parallel tests — Parallel tests never share mutable state.
  decision/owner-record-removed-last · Owner record removed last — Removing a test home deletes its owner record last. · why: A removal that stops part-way keeps the record, so a later sweep can still take the dir.
  decision/tempfile-test-dirs · tempfile for test directories — tempfile is what provides temporary test directories.
  decision/test-function-naming · Test function naming — Test functions are named <subject>_<condition>_<expected> in snake_case. · except: critical-path scenarios are named path<N>_<slug>
  decision/test-home-owner-record-sweep · Owner-record sweep of test homes — Every root test home records its owner pid and start time; a new test home, when no keep flag is set, first removes only dirs whose recorded owner process is gone, never by age or name, never a dir without a record. · why: A test the runner kills never drops its home. · except: A build's verify stamp dir carries no owner record; another build's stamp is swept only when its lock is taken without waiting.
  decision/tests-run-in-parallel · Tests run in parallel — Tests are never serialised with a single thread; isolation comes from the per-test home. · except: The fuzz stages are single jobs, by design.
- S2 · each new root integration binary for a CLI surface is prefixed by its surface and selected by nextest filterset · 2-test-strategy
  decision/integration-binary-surface-prefix · Root integration binaries prefixed by surface — Root integration binaries use a <surface>_<topic>.rs prefix, the surface prefix naming the layer; crate-local integration binaries need no surface prefix. · why: so nextest filtersets can select layers
  decision/nextest-is-the-runner · nextest is the runner — nextest is the test runner; a plain cargo test run is not supported.
  decision/test-layer-filtersets · Test layers by nextest filterset — The unit layer is lib and bin targets; the E2E layer is the path, tui, mcp, cross, chaos and contract test binaries; the integration layer is the complement, so a test binary reports in exactly one layer.
- S3 · each new security control the verb adds (name validation, strict modes, address re-parse) gets one negative test · 1-test-scope-summary
  decision/one-negative-per-security-control · One negative test per security control — Every security-plan control has a negative test of its own.
- S4 · each new fixture is synthetic or recorded and scrubbed, walked by the hygiene check, and carries no real host data · 7-test-data-fixtures · test-data-fixtures
  contract/mcp-fixture-scrub-refusal/absolute-path · MCP fixture scrub refusal codes · absolute-path — A closed refusal code of the MCP fixture scrub, naming the file only.
  contract/mcp-fixture-scrub-refusal/email · MCP fixture scrub refusal codes · email — A closed refusal code of the MCP fixture scrub, naming the file only.
  decision/synthetic-fixture-text · synthetic fixture text only — Fixture prompts and checked-in fake agent scenario scripts hold synthetic probe text only, never real prompts.
- S5 · each new double sits only at a product seam trait or is a real stub executable; nothing owned by others is mocked and nothing is mocked in E2E · 8-mocking-stubbing-discipline · mocking-stubbing-discipline
  decision/mocks-only-at-product-seams · Mocks only at product seams — Unit tests use mockall doubles only for traits the product already defines at its seams: Pty, the liveness probe and Clock.
  decision/no-mocking-unowned-code · No mocking of what is not owned — Libraries and the language runtime are never mocked without an abstraction layer.
  decision/no-mocks-in-e2e · nothing mocked in E2E — E2E tests mock none of the wrapper channel, PTY, vt100, serde or viola hook invocations; the fake agent calls the real pinned viola binary, and code paths that must be verified end-to-end go in E2E.
  decision/no-over-mocking-integration · No over-mocking at integration level — Integration tests never over-mock. · why: Over-mocking stops testing the integration.
  decision/no-over-stubbing · No over-stubbing — Tests never over-stub. · why: Over-stubbing stops testing real behaviour.
- S6 · each new test that spawns viola ties the child with a drop guard and drops inherited env while keeping LLVM_PROFILE_FILE · integration-test-strategy
  decision/env-clear-keeps-llvm-profile-file · env_clear tests re-add LLVM_PROFILE_FILE — Any test that uses env_clear() must re-add LLVM_PROFILE_FILE.
  decision/no-untied-viola-spawn · No root test spawns viola untied — No root test spawns viola untied: a census over every test source, recursively, counts any spawn whose path equals or ends in the viola binary constant outside the tied builder, and is proven on a planted positive. · why: A census over one spelling of a constant missed a spawn site; a census over a constant's spawns matches any path ending in the constant. · guard: cross_killed_test_census_finds_no_untied_viola_spawn
  decision/tied-child-spawn-thread · Tied children spawned on a thread that outlives them — A tied viola child is spawned only on a thread that outlives it. · why: The parent-death signal follows the spawning thread.
  decision/wrapper-drop-guard · Test wrappers held in a Drop guard — Every long-lived viola child a root test spawns is tied to its spawning test thread by a parent-death kill, exiting when its recorded parent is already gone, so a killed or panicked test leaves no wrapper or fake agent; a Drop guard that kills, then waits, still covers the normal end.
