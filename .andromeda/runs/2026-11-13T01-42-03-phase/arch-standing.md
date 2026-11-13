# arch · standing conventions

- S1 · each new test runs against the fake agent, never a live claude · ci-cd-approach
- S2 · each new test runs in its own isolated viola home · established-decisions
  decision/test-home-owner-sweep · Test homes removed by owner record only — A test home sweep removes only dirs whose recorded owner process is gone, never by age or name and never a dir without a record, and every removal deletes the owner record last. · why: a removal that stops part-way keeps the record for a later sweep
  decision/test-homes-cleaned-by-owner · Test homes cleaned by owner process — Harness and test homes are kept in CI for the obs gates, and a new test home removes only the dirs whose recorded owner process is gone, never by age or name.
  decision/test-isolated-viola-home · Each test in its own viola home — Each test runs in its own viola home (`--home`), so parallel tests never share an endpoint, log or `budget.json`.
  decision/test-side-removal-outside-workdir · Test-side removals outside the working directory — Through the e2e-home link, the owner sweep, a test home's drop and harness cleanup create and delete outside the working directory, each taking only a dir the test side itself made. · why: a boundary widening, ratified
  decision/viola-home-resolution · Viola home resolution order — The viola home resolves from --home, then the grandparent of VIOLA_DIR, then .viola under the user home; viola run passes the home to its child only through VIOLA_DIR, so every per-home file follows it. · why: The home is the isolation point for tests: each test runs in its own home.
- S3 · each new or re-recorded screen fixture is scrubbed, hygiene-walked and keeps only signature rows · occupied-resources
  contract/unclean-fixture-refusal/absolute-path · unclean fixture refusal · absolute-path — The code given when a recording holds an absolute path.
  contract/unclean-fixture-refusal/email · unclean fixture refusal · email — The code given when a recording holds an email-shaped token.
  contract/unclean-fixture-refusal/home-path · unclean fixture refusal · home-path — The code given when a recording holds a home path.
  contract/unclean-fixture-refusal/message · unclean fixture refusal · message — An unclean recording is refused whole as `unable: a recorded fixture is not clean: <file> <code>`, or `<file> row <n>[ seam] <code>` for a screen; the line never names the content.
  contract/unclean-fixture-refusal/username · unclean fixture refusal · username — The code given when a recording holds the username, including one split across a screen row seam.
  decision/fixture-keeps-own-repository-remote · Fixture keeps this repository's own remote — A workspace.repo naming this repository's own remote is kept in a fixture rather than scrubbed. · why: ratified
- S4 · each new test respects the coverage floors and the gate steps that fail closed · ci-cd-approach
  decision/coverage-floors · Coverage floors — The test job runs the instrumented suite with coverage floors 85/95/80, then doctest, and its gate requires coverage and doctest. · guard: gate --require coverage,doctest
  decision/coverage-floors-gate · Coverage floors enforced by the harness gate — Coverage is measured by cargo-llvm-cov and the harness gate enforces the coverage floors.
  decision/gate-steps-fail-closed · Gate steps fail closed — Every CI gate step runs fail-closed in bash.
- S5 · each new end-to-end test leaves zero panic lines, and insta snapshots run in check mode only · project-directory-structure
  decision/chaos-home-outside-gates · Chaos panic home outside the obs gates — The forced vt100 feed-panic test uses a home outside the e2e home tree, so its deliberate panic line falls outside the zero-panic gate, schema conformance and the secret scan; the test asserts the panic and parse-rejected lines itself. · why: the one named carve-out to the test-data rule · guard: chaos_feed_panic
  decision/snapshots-check-mode-only · Insta snapshots in check mode only — The agent crate's decision-body snapshots are only checked by tests, never written by them; the root integration tests take no snapshots.
  decision/zero-panics-gate · Zero panic lines in end-to-end runs — No panic role line may appear under the end-to-end home, checked fail-closed. · guard: obs G2 zero-panics gate · except: A panic whose location is the hook seam module.
- S6 · each new log line goes through the one event emitter, with no print macros and no tracing level macros · build-system
  decision/clippy-print-and-macro-bans · clippy deny warnings with print and dbg bans — clippy runs with warnings denied, the workspace bans on print_stdout, print_stderr and dbg_macro, and disallowed tracing level macros.
  decision/no-print-macros · No print macros — The workspace lints deny `print_stdout`, `print_stderr` and `dbg_macro`, and every product member inherits them. · guard: tests/contract_lints.rs · except: viola-e2e denies only `dbg_macro`, because the harness prints its JSON document; the fake agent carries a crate-level print allow.
  decision/no-tracing-level-macros · Tracing level macros banned — The tracing info, warn, error, debug and trace macros are disallowed in the code. · guard: clippy disallowed-macros, checked by the lint probes
  decision/obs-event-only-emitter · obs_event! is the only event emitter — The tracing level macros are banned, so `obs_event!` is the only sanctioned emitter; raw `event!` is caught by a fail-closed grep. · why: Clippy cannot exempt a macro's inner expansion, so raw event! needs the grep. · guard: clippy.toml disallowed-macros; scripts/lint-probes.sh
  decision/tracing-process-logs · tracing for one-line JSON process logs — Process logs are one-line JSON written by tracing into the viola home's diagnostics, with the root bin the only product dependent of tracing-subscriber; the line format and levels are owned by the obs plan.
