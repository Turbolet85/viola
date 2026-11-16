# arch · standing conventions

- S1 · each new test runs against the fake agent in its own isolated viola home, a stamped home coming from the shared stamp's copy · established-decisions
  decision/ci-ubuntu-fake-agent · CI on ubuntu against the fake agent — CI builds, lints and tests on ubuntu-latest native runners against the fake agent, with SHA-pinned actions and toolchains installed by rustup.
  decision/test-homes-cleaned-by-owner · Test homes cleaned by owner process — Harness and test homes are kept in CI for the obs gates, and a new test home removes only the dirs whose recorded owner process is gone, never by age or name.
  decision/test-isolated-viola-home · Each test in its own viola home — Each test runs in its own viola home (`--home`), so parallel tests never share an endpoint or log.
  decision/viola-home-resolution · Viola home resolution order — The viola home resolves from --home, then the grandparent of VIOLA_DIR, then .viola under the user home; viola run passes the home to its child only through VIOLA_DIR, so every per-home file follows it. · why: The home is the isolation point for tests: each test runs in its own home.
- S2 · each test that waits on a channel or a thread takes the shared bound and cannot hang past a failure · occupied-resources
  decision/root-waits-share-one-bound · Root test waits share one bound — Root test waits on a child share the one 12 s WITHIN bound of the watch support.
- S3 · each test channel socket goes under the short /tmp root, outside the product namespace · occupied-resources
  decision/test-endpoints-outside-namespace · Test channel endpoints outside the product namespace — Test-only channel endpoints are named outside the `viola-<h12>` namespace, unique per test and never bound by `viola`.
  decision/test-sockets-under-short-tmp-root · Test channel sockets under a short /tmp root — Test channel sockets bind under a test-owned short root directory made under /tmp, never under TMPDIR; the product endpoint socket is not under that root. · why: So the socket path length bound holds whatever the length of TMPDIR.
- S4 · each recorded fixture is scrubbed and hygiene-walked, keeping only signature rows for screens · occupied-resources
  contract/unclean-fixture-refusal/absolute-path · unclean fixture refusal · absolute-path — The code given when a recording holds an absolute path.
  contract/unclean-fixture-refusal/email · unclean fixture refusal · email — The code given when a recording holds an email-shaped token.
  contract/unclean-fixture-refusal/home-path · unclean fixture refusal · home-path — The code given when a recording holds a home path.
  contract/unclean-fixture-refusal/username · unclean fixture refusal · username — The code given when a recording holds the username, including one split across a screen row seam.
- S5 · each new log line is codes-only, emitted through obs_event!, with content-bearing detail only in the instance · occupied-resources · cross-cutting-patterns
  contract/diag-line/format · diagnostics line · format — Owned by obs and fixed by schemas/diag-line.v1.json; one line per write; folder 0700, files 0600.
  contract/diagnostics-lines/detail-line · Diagnostics lines · instance detail line — Format owned by obs and fixed by the default-deny, self-contained schema diag-detail.v1.
  contract/diagnostics-lines/process-log-line · Diagnostics lines · home-level process-log line — Format owned by obs and fixed by the default-deny, self-contained schema diag-line.v1.
  contract/diagnostics-role-files/run-name · home-level diagnostics role files · run-<name>.ndjson — The wrapper's codes-only process lines, viola revive's start arm included.
  decision/codes-only-process-logs · Codes-only process logs per role — Every role's codes-only process log goes to its home-level role file under diagnostics (run, hook, mcp or cli ndjson).
  decision/hook-never-writes-stderr · Hook logs only to diagnostics — A hook never writes to stderr; its codes-only log goes to the home-level diagnostics and content-bearing detail only to the instance's diagnostics, and a failure to write either is ignored.
  decision/no-tracing-level-macros · Tracing level macros banned — The tracing info, warn, error, debug and trace macros are disallowed in the code. · guard: clippy disallowed-macros, checked by the lint probes
  decision/obs-event-only-emitter · obs_event! is the only event emitter — The tracing level macros are banned, so `obs_event!` is the only sanctioned emitter; raw `event!` is caught by a fail-closed grep. · why: Clippy cannot exempt a macro's inner expansion, so raw event! needs the grep. · guard: clippy.toml disallowed-macros; scripts/lint-probes.sh
  decision/process-logs-hold-codes-only · Home-level process logs hold codes only — Home-level per-role process logs carry codes only, one line per write; content-bearing detail stays in the instance detail files.
  decision/tracing-process-logs · tracing for one-line JSON process logs — Process logs are one-line JSON written by tracing into the viola home's diagnostics, with the root bin the only product dependent of tracing-subscriber; the line format and levels are owned by the obs plan.
- S6 · each new JSON field is snake_case and each new wire enum value kebab-case, absent optional fields omitted · conventions
  decision/absent-optional-fields-omitted · Absent optional fields omitted on write — viola omits an absent optional field on write rather than writing `null`.
  decision/json-field-names-snake-case · JSON field names in snake_case — JSON field names are snake_case in every viola format. · except: `last_assistant_message` stays as Claude emits it.
- S7 · each end-to-end run ends with zero panic lines · project-directory-structure
  decision/zero-panics-gate · Zero panic lines in end-to-end runs — No panic role line may appear under the end-to-end home, checked fail-closed. · guard: obs G2 zero-panics gate · except: A panic whose location is the hook seam module.
