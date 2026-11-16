# arch · standing conventions

- S1 · each new test runs in its own isolated viola home · established-decisions
  decision/test-home-owner-sweep · Test homes removed by owner record only — A test home sweep removes only dirs whose recorded owner process is gone, never by age or name and never a dir without a record, and every removal deletes the owner record last. · why: a removal that stops part-way keeps the record for a later sweep
  decision/test-homes-cleaned-by-owner · Test homes cleaned by owner process — Harness and test homes are kept in CI for the obs gates, and a new test home removes only the dirs whose recorded owner process is gone, never by age or name.
  decision/test-isolated-viola-home · Each test in its own viola home — Each test runs in its own viola home (`--home`), so parallel tests never share an endpoint or log.
  decision/test-side-removal-outside-workdir · Test-side removals outside the working directory — Through the e2e-home link, the owner sweep, a test home's drop and harness cleanup create and delete outside the working directory, each taking only a dir the test side itself made. · why: a boundary widening, ratified
  decision/viola-home-resolution · Viola home resolution order — The viola home resolves from --home, then the grandparent of VIOLA_DIR, then .viola under the user home; viola run passes the home to its child only through VIOLA_DIR, so every per-home file follows it. · why: The home is the isolation point for tests: each test runs in its own home.
- S2 · each new test runs against the fake agent, never a live claude · ci-cd-approach
  decision/ci-linux-fake-agent · CI on Linux against the fake agent — CI runs on GitHub Actions ubuntu-latest against the fake agent.
- S3 · each recorded payload fixture added is scrubbed and hygiene-walked before it is committed · occupied-resources
  contract/unclean-fixture-refusal/absolute-path · unclean fixture refusal · absolute-path — The code given when a recording holds an absolute path.
  contract/unclean-fixture-refusal/email · unclean fixture refusal · email — The code given when a recording holds an email-shaped token.
  contract/unclean-fixture-refusal/home-path · unclean fixture refusal · home-path — The code given when a recording holds a home path.
  contract/unclean-fixture-refusal/message · unclean fixture refusal · message — An unclean recording is refused whole as `unable: a recorded fixture is not clean: <file> <code>`, or `<file> row <n>[ seam] <code>` for a screen; the line never names the content.
  contract/unclean-fixture-refusal/username · unclean fixture refusal · username — The code given when a recording holds the username, including one split across a screen row seam.
  decision/recorded-fixtures-scrubbed · Recorded fixtures scrubbed and hygiene-walked — Fixtures recorded by viola verify --record are written only when no row fails, each is scrubbed and hygiene-walked, and an unclean recording is refused whole, naming the file and the check code, never the content. · guard: contract_fixture_hygiene
- S4 · each new test's wait shares the one root test wait bound and its evidence survives a killed test · occupied-resources
  decision/end-to-end-homes-kept-after-each-job · Homes kept after each job's end-to-end run — Each CI job keeps its harness and test homes after its end-to-end run, whatever the outcome (AGENT_RUN_KEEP_HOMES "1" on the job, if: always()), the viola-chaos-* home out. · why: The homes are kept in CI for the obs gates.
  decision/test-evidence-survives-kill · Test evidence survives a killed test — Real-process test rigs stream step evidence to report files outside the viola home, kept when the test panics or is killed and removed when it passes; viola never reads or writes them. · why: so a test the runner kills keeps its evidence
  decision/wait-report-kept-on-failure · Wait report kept on failure — A test wait writes a report created empty at its start, with a line only when the polled state changes, codes, counts and booleans only; the report is kept when the test panics or is killed and removed when it passes. · why: A test the runner kills keeps its evidence.
- S5 · each new snapshot assertion runs in check mode only, and new end-to-end runs keep zero panic lines · project-directory-structure
  decision/snapshots-check-mode-only · Insta snapshots in check mode only — The agent crate's decision-body snapshots are only checked by tests, never written by them; the root integration tests take no snapshots.
  decision/zero-panics-gate · Zero panic lines in end-to-end runs — No panic role line may appear under the end-to-end home, checked fail-closed. · guard: obs G2 zero-panics gate · except: A panic whose location is the hook seam module.
- S6 · each new log line is codes-only, emitted through the one event emitter, with no upstream text interpolated · cross-cutting-patterns · build-system
