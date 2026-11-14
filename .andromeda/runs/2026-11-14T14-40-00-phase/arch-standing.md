# arch · standing conventions

- S1 · each test runs in its own isolated viola home, cleaned by its owner · established-decisions
  decision/test-home-owner-sweep · Test homes removed by owner record only — A test home sweep removes only dirs whose recorded owner process is gone, never by age or name and never a dir without a record, and every removal deletes the owner record last. · why: a removal that stops part-way keeps the record for a later sweep
  decision/test-homes-cleaned-by-owner · Test homes cleaned by owner process — Harness and test homes are kept in CI for the obs gates, and a new test home removes only the dirs whose recorded owner process is gone, never by age or name.
  decision/test-isolated-viola-home · Each test in its own viola home — Each test runs in its own viola home (`--home`), so parallel tests never share an endpoint, log or `budget.json`.
  decision/test-side-removal-outside-workdir · Test-side removals outside the working directory — Through the e2e-home link, the owner sweep, a test home's drop and harness cleanup create and delete outside the working directory, each taking only a dir the test side itself made. · why: a boundary widening, ratified
- S2 · tests run against the fake agent in CI, never a live claude · ci-cd-approach
  decision/no-live-claude-in-ci · No live claude in CI — When CI is present, the test harness's run --local-live refuses before any build, spawn or suite, and viola itself never reads the harness variables. · why: so the real claude never runs in CI
- S3 · a recorded fixture is scrubbed and hygiene-walked, recorded only from a clean verify · occupied-resources
  contract/unclean-fixture-refusal/absolute-path · unclean fixture refusal · absolute-path — The code given when a recording holds an absolute path.
  contract/unclean-fixture-refusal/email · unclean fixture refusal · email — The code given when a recording holds an email-shaped token.
  contract/unclean-fixture-refusal/home-path · unclean fixture refusal · home-path — The code given when a recording holds a home path.
  contract/unclean-fixture-refusal/message · unclean fixture refusal · message — An unclean recording is refused whole as `unable: a recorded fixture is not clean: <file> <code>`, or `<file> row <n>[ seam] <code>` for a screen; the line never names the content.
  contract/unclean-fixture-refusal/username · unclean fixture refusal · username — The code given when a recording holds the username, including one split across a screen row seam.
  contract/verify-exit-detail/unclean-fixture · viola verify process-exit detail · unclean-fixture — A refusal code verify writes under a parsed VIOLA_NAME; the fixture's file name enters no code.
  contract/viola-verify-refusal-codes/unclean-fixture · viola verify process-exit refusal codes · unclean-fixture — Written on the process-exit verify logs under a parsed VIOLA_NAME when a recording is unclean; the fixture's file enters no code.
  decision/fixture-keeps-own-repository-remote · Fixture keeps this repository's own remote — A workspace.repo naming this repository's own remote is kept in a fixture rather than scrubbed. · why: ratified
  decision/screen-fixtures-signature-rows-only · Screen fixtures keep only signature rows — A recorded screen fixture keeps a row holding a footer or modal literal whole, the input box's rule row as its rule chars alone and its prompt row as the prompt char alone, every other row empty; a box whose live prompt row holds a home path, the username or an email refuses the recording.
- S4 · an OS-gated test names its reason · cross-cutting-patterns
  decision/os-gated-tests-name-reason · OS-gated tests name their reason — Every OS-gated test names its reason in the census, and a gate whose predicate names windows or macos reads as a fault even when a row names it. · guard: contract_os_gated_tests
