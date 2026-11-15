@history_item n=1 marker=2026-09-27-wrapper-channel
**2026-09-27-wrapper-channel** (viola-channel landed: Unix endpoint path and arbiter) — moved the Unix endpoint into the per-user 0700 socket dir and registered the test-only `viola-test-chan-<pid>-<label>` name outside the `viola-<h12>` namespace; item 1 renames exactly this test endpoint, so the new name must stay outside the product namespace (row test-endpoints-outside-namespace) and unique per test.

@history_item n=2 marker=2026-09-25-security-prerequisites
**2026-09-25-security-prerequisites** (SQOS client open, test-only pipe) — first registered a test-only endpoint (`viola-test-sqos-<pid>-<label>`) outside the `viola-<h12>` namespace; the precedent the `viola-test-chan-*` name follows, and a reminder that any new test endpoint form is a registry fact.

@history_item n=3 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (the wrapper endpoint is a Unix socket only) — retired the pipes and stated that `viola-test-chan-<pid>-<label>` is a socket of that base name; with item 1 the socket base name's length is now bounded by `sun_path` alone, the constraint the chunk must prove.

@history_item n=4 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** (the socket-directory check's read and its home in the code) — built `check_socket_dir` / `check_endpoint_dir` (row socket-dir-strict-check): symlink, non-directory, foreign owner or group/other bit refused, never chmodded; a test socket placed under the mutation scratch's tempdir must sit in a directory that passes this check if it goes through product connect paths.

@history_item n=5 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** (the test-owned socket roots) — registered test-only socket roots under `/tmp` via `tests/support/home.rs` `SocketRoot` / `SOCKET_ROOT_VAR`; a nearby precedent for giving a test a short, test-owned socket location, and an occupied-resources row to extend if item 1 moves the socket.

@history_item n=6 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (test support and harness: the Windows and macOS arms retired) — `SOCKET_ROOT_VAR` reads `XDG_RUNTIME_DIR`, `Frozen` ungated, keepers' linked-base contract with no OS split; the current state of the test support item 1 and item 5 edit.

@history_item n=7 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (the OS-gate census faults a gate naming another OS) — `tests/contract_os_gated_tests.rs` caught a multi-line `cfg_attr` pair in `tests/hook_fail_open.rs`; any gate the chunk adds or moves in that file needs its `GATES` row, and a gate naming another OS reads as a fault.

@history_item n=8 marker=2026-11-04-linux-and-macos-parity
**2026-11-04-linux-and-macos-parity** (the OS-gated tests census) — every OS-gated test names its reason in `GATES` or the census goes red both ways; a constraint on any new or removed gate in the test files this chunk touches.

@history_item n=9 marker=2026-10-03-mutation-scoring-completion
**2026-10-03-mutation-scoring-completion** (the mutation scratch dir on the Linux host) — origin of row mutation-scratch-nocow: on the Linux dev host `<repo parent>/viola-mutants-scratch/` is the operator's NOCOW `TMPDIR` for every mutation run (tmpfs too small, reflink drops the fake agent's exec bit); item 1's socket path is computed from this scratch path, and the scope pins that this row is not amended.

@history_item n=10 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (mutation runs: no scratch arm, no Windows leg) — `run --mutants` has no scratch arm, `mutants.out/` stays at the repo root, the scratch is an environment fact only; so the long `TMPDIR` reaching `tempfile::tempdir()` in tests is the operator's NOCOW dir, the premise item 1 recomputes.

@history_item n=11 marker=2026-09-28-mutation-testing-to-the-epoch-boundary
**2026-09-28-mutation-testing-to-the-epoch-boundary** — removed mutation from CI and the pre-push; mutation runs only through `agent-run run --mutants` for the epoch-boundary audit (row mutation-at-epoch-boundary); why this chunk runs no mutation itself and the next boundary's audit reads the green baseline.

@history_item n=12 marker=2026-09-27-hooks-to-normalised-events
**2026-09-27-hooks-to-normalised-events** (Repository rows) — registered `target/mutants/`, the mutation run's own target dir (row mutants-own-target-dir); the relative target dir under which the `mutants` nextest profile's cases run.

@history_item n=13 marker=2026-09-27-instance-state-and-start-order
**2026-09-27-instance-state-and-start-order** — `target/e2e-home` got its owner record and gone-owner sweep, not kept under `run --mutants`; the test-home lifecycle the record-home helper (item 5) sits in.

@history_item n=14 marker=2026-09-24-fake-agent-and-test-data-fixtures
**2026-09-24-fake-agent-and-test-data-fixtures** — named the test homes `viola-session-*/home` (harness) and `viola-test-*/home` (root rstest): the standing test-home shape ends in a component named `home`, exactly what item 5's rule forbids for a `viola verify --record` home; the helper must name the record home differently.

@history_item n=15 marker=2026-10-02-epoch-2b-cleanup
**2026-10-02-epoch-2b-cleanup** (test homes removed with their owner record last) — `remove_owned` in `tests/support/home.rs` deletes `owner.json` last (row test-home-owner-sweep); any new record-home helper there must keep this removal order.

@history_item n=16 marker=2026-10-07-test-homes-off-the-contended-volume
**2026-10-07-test-homes-off-the-contended-volume** — `prepare_home_base` in `tests/support/home.rs` (from `TestHome::new`) and the dev host's `target/e2e-home` link to `/tmp/viola-e2e-home-<uid>`; removals outside the working directory ratified (row test-side-removal-outside-workdir); a stalled-start red is a backing finding, never re-run for green — relevant to the slow-case timings item 2 measures.

@history_item n=17 marker=2026-10-20-epoch-5-cleanup
**2026-10-20-epoch-5-cleanup** (the keepers' headroom floor) — the keepers refuse a linked base below `HEADROOM_FLOOR_MIB` (9 GiB) free; a constraint on the home helper item 5 extends and on cold-baseline runs.

@history_item n=18 marker=2026-10-04-confirmed-send-with-cl-1-records
**2026-10-04-confirmed-send-with-cl-1-records** (fuzz targets, the chaos test home) — `TestHome::outside_scan()` gives the one carve-out home `<temp dir>/viola-chaos-*/home` outside `target/e2e-home`; another `home`-named test home variant the record-home rule must not be confused with.

@history_item n=19 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** (`v1-47`'s reading over the homes CI keeps) — `AGENT_RUN_KEEP_HOMES=1` on the CI job with `if: always()` (row test-homes-cleaned-by-owner); homes are kept in CI for the obs gates, so a helper change must not break the kept-home scan.

@history_item n=20 marker=2026-09-28-capability-ledger-and-viola-verify
**2026-09-28-capability-ledger-and-viola-verify** (the six landed ledger rows, `viola verify`'s probe) — `--record` writes scrubbed fixtures only at 0 fail and the real CLI runs only locally, CI verify against the fake agent (rows verify-record-only-clean, real-cli-probe-local-only); the frame item 5's record-home rule protects.

@history_item n=21 marker=2026-10-05-real-cli-verify-probes
**2026-10-05-real-cli-verify-probes** (registry rows for verify's interactive runs and screen fixtures) — the `--record` refusal names the file and a closed code, never content (rows recorded-fixtures-scrubbed, unclean-fixture-refusal/home-path); a record home with a `home` path component trips the home-path scrub, the failure item 5 prevents.

@history_item n=22 marker=2026-11-13-verify-readiness-on-the-input-box
**2026-11-13-verify-readiness-on-the-input-box** (Occupied Resources) — `decision/screen-fixtures-signature-rows-only` now refuses a recording whose live prompt row holds a home path, the username or an email; another home-path refusal the record-home naming must not trigger.

@history_item n=23 marker=2026-11-13-verify-readiness-on-the-input-box
**2026-11-13-verify-readiness-on-the-input-box** (Established Decisions) — `decision/verify-box-wait-for-literal` reworded: verify's waits end on the input box (footer literal or the box's rows) or a modal; the waits that bound verify cases like the Run E case item 3 names.

@history_item n=24 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** (`viola verify`'s refusal codes on its `process-exit`) — verify's `process-exit` carries `unclean-fixture` and `interrupted`, the fixture's file entering no code (contracts verify-process-exit/unclean-fixture, viola-verify-refusal-codes/unclean-fixture, verify-exit-detail/unclean-fixture); what a red record-home test reads.

@history_item n=25 marker=2026-10-13-interrupted-verify-cleanup
**2026-10-13-interrupted-verify-cleanup** — verify stopped by SIGINT/SIGTERM removes its dirs and exits 1 with `detail:"interrupted"` (contract verify-process-exit/interrupted); the behaviour `cli_verify::verify_stopped_during_the_resume_run_removes_its_dir_and_exits_1`, the case red under item 3's `WITHIN`, asserts.

@history_item n=26 marker=2026-10-14-resume-and-compact-ledger-rows
**2026-10-14-resume-and-compact-ledger-rows** (the resume row, Run E) — Run E runs last and resumes Run B's first SessionStart id (row verify-run-e-resumes-run-b-first-session); the slow `timed out waiting for Run E's start` wait in item 3 sits behind Runs A–D, which is why the shared 7 s bound is tight there.

@history_item n=27 marker=2026-10-14-resume-and-compact-ledger-rows
**2026-10-14-resume-and-compact-ledger-rows** (Run E's resources and a killed PTY child's exit status) — a child killed at a stop closes with `duration_ms` and `child_exit_status`, measured by the resume-run stop case; the same case item 3 makes green.

@history_item n=28 marker=2026-09-29-verify-stamped-test-homes-and-harness
**2026-09-29-verify-stamped-test-homes-and-harness** (verify's spawn pairs, verify-stamped homes and the ninth root wait) — root homes stamped by `stamped_home`; the Watch report's bound used by root waits on a child, `wait_endpoint_gone` added; origin of the shared-bound row root-waits-share-one-bound.

@history_item n=29 marker=2026-10-09-epoch-3-cleanup-ii
**2026-10-09-epoch-3-cleanup-ii** (the Watch bound's waits counted by pattern) — `WITHIN` = 7 s shared by root waits, counted by `Instant::now() + WITHIN`; trap: a wait through `EXIT_WITHIN` or a qualified path is outside the pattern. Item 3 moves this bound or splits a caller off it; the count and its rule move with it.

@history_item n=30 marker=2026-10-10-self-healing-state
**2026-10-10-self-healing-state** (the root waits read 23 sites in 17 files) — a WITHIN-count re-measure precedent; the count travels with test-plan's 5-command-implementation in the same pass.

@history_item n=31 marker=2026-10-10-viola-revive
**2026-10-10-viola-revive** (root wait count 26 sites in 19 files, the kill path's endpoint wait) — `Wrapper::kill` waits on `holder_gone`; another `WITHIN` user and count update the chunk may hit.

@history_item n=32 marker=2026-10-10-budget-governor
**2026-10-10-budget-governor** (the threshold keys, the reader of budget.json) — root waits 28 sites in 20 files; the count's rule stays the pattern, not a census of every wait on the bound.

@history_item n=33 marker=2026-11-04-linux-and-macos-parity
**2026-11-04-linux-and-macos-parity** (the root waits on the WITHIN bound) — 32 sites in 24 files with the grep rule; `Frozen`'s `attach` polls under `super::watch::WITHIN`, so changing `WITHIN` also changes the freeze guard's bound.

@history_item n=34 marker=2026-11-04-linux-and-macos-parity
**2026-11-04-linux-and-macos-parity** (the shared Frozen guard) — `tests/support/frozen.rs` is the one freeze home (row root-tests-freeze-wrappers-with-one-guard), used by `cli_instance_state` — the file holding two of item 2's ~5 s cases.

@history_item n=35 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (the root `WITHIN` waits: 31 sites in 23 files) — the current registered count and rule; item 3 must re-measure and amend it if it changes the bound's sites.

@history_item n=36 marker=2026-10-16-remove-the-browser-leg
**2026-10-16-remove-the-browser-leg** (Infrastructure Patterns) — `.config/nextest.toml` holds the `ci` and `mutants` profiles (the `fixed-port` group gone), and the `test` job gates `coverage,doctest`; item 2 edits the `mutants` profile of this file.

@history_item n=37 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** (the sanitised error surfaces pattern and its contract binary) — two root binaries sit in the `ci` profile's kill-line filter in `.config/nextest.toml`; precedent for per-test overrides in nextest config, relevant to the inheritance premise item 2 measures.

@history_item n=38 marker=2026-09-24-quality-gates
**2026-09-24-quality-gates** — introduced cargo-llvm-cov coverage and the gate (row coverage-floors-gate); the coverage suite whose green the next audit reads.

@history_item n=39 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (CI on the one `ubuntu-latest` leg) — `test` runs `run --coverage` at floors 85/95/80 on `ubuntu-latest` (row coverage-floors); the suite M1/M2's coverage-red case belongs to.

@history_item n=40 marker=2026-09-28-hook-perf-gate
**2026-09-28-hook-perf-gate** (perf arm registry, the `perf` CI job) — perf runs only in its own job, the `test` job and pre-push carry no perf step (row perf-gate); the chunk's wall-time measurements are evidence, not a perf gate.

@history_item n=41 marker=2026-09-24-workspace-tree-and-code-graph-planes
**2026-09-24-workspace-tree-and-code-graph-planes** — orphans gate and release-check landed (rows no-orphan-modules, release-carries-viola-only); any new test module or bin change must keep both green.

@history_item n=42 marker=2026-09-27-epoch-2-cleanup
**2026-09-27-epoch-2-cleanup** — `release-check.sh` also refuses a test-only feature, viola-channel's `test-support` feature (row release-build-viola-only); test-only additions must stay out of release artifacts.

@history_item n=43 marker=2026-09-24-three-os-ci-headless-harness-skeleton
**2026-09-24-three-os-ci-headless-harness-skeleton** (test-only crate, bins, env vars, paths) — registered `viola-e2e` with its no-op `fake-agent` feature and `.config/nextest.toml` (rows e2e-noop-fake-agent-feature, e2e-test-only); viola-hypr-tap's mutants run package-scoped under this feature.

@history_item n=44 marker=2026-09-25-pty-wrapper-on-windows
**2026-09-25-pty-wrapper-on-windows** — `viola-pty` and `viola-agent-claude` got code-free `fake-agent` features (row fake-agent-feature-on-named-members), so package-scoped `--features fake-agent` builds (cargo-mutants) resolve.

@history_item n=45 marker=2026-10-17-mcp-server-for-drivers
**2026-10-17-mcp-server-for-drivers** (the Tokio client and viola-channel's tree-form ban) — viola-channel's `fake-agent = ["tokio"]`, the except of row fake-agent-feature-on-named-members; a package-scoped mutation build compiles its Tokio client.

@history_item n=46 marker=2026-09-26-local-linux-pre-push-gate
**2026-09-26-local-linux-pre-push-gate** — `FAKE_AGENT_PUMP_DELAY_MS` test seam, ratified exception under `fake-agent` (rows fake-agent-test-seams, viola-name-prefix); no new env seam may be added for timing without its own ratification.

@history_item n=47 marker=2026-09-28-hook-perf-gate
**2026-09-28-hook-perf-gate** (the second fake-agent test seam `FAKE_AGENT_HOOK_PANIC`) — the second ratified seam, used by `tests/hook_fail_open.rs`'s panic case; the file item 1 edits.

@history_item n=48 marker=2026-10-20-epoch-5-cleanup-ii
**2026-10-20-epoch-5-cleanup-ii** (the recorded live MCP exchange: the tap) — `viola-mcp-tap` as a test-only `[[bin]]` of `viola-e2e`: direct spawn, `take(MAX_FRAME)` capture, scrub and closed refusal codes; the template `viola-hypr-tap` follows, relevant to item 4's survivors and whether its `main` sits in the tier.

@history_item n=49 marker=2026-11-14-hyprland-seam
**2026-11-14-hyprland-seam** (Occupied Resources) — added `name/binary/viola-hypr-tap` and `name/crate/viola-hyprland`; item 4's ten survivors are in this test-only binary (row test-binaries-never-released).

@history_item n=50 marker=2026-11-14-hyprland-seam
**2026-11-14-hyprland-seam** (Infrastructure Patterns: viola-hyprland is the one compositor seam) — `decision/hyprland-seam-crate`: closed request set, sync, tokio-free, depends on viola-core alone; tests killing the hypr-tap survivors must go through that seam.

@history_item n=51 marker=2026-11-14-hyprland-seam
**2026-11-14-hyprland-seam** (Standard Contracts) — `decision/hyprctl-direct-spawn` and the five `hyprland-seam-errors` contracts; survivors like `version_of`'s `Ok(Reply::Version(..))` arm and `texts`' `Value::Array` arm are read against these closed replies and errors.

@history_item n=52 marker=2026-11-14-hyprland-seam
**2026-11-14-hyprland-seam** (Conventions) — `decision/unparseable-external-values-unknown` gained the except: an unparseable hyprctl reply is the seam's `malformed` error; a test killing the recorder's reply-parsing survivors asserts this, not `"unknown"`.

@history_item n=53 marker=2026-11-14-hyprland-seam
**2026-11-14-hyprland-seam** (Infrastructure Patterns: root-bin row) — viola-hyprland is a root dev-dependency only while no verb calls the seam; a constraint on where hypr-tap tests may live.

@history_item n=54 marker=2026-10-13-epoch-4-survivors
**2026-10-13-epoch-4-survivors** — precedent for an audit-survivor chunk: kill const-gated mutants by passing the decision in from the caller, give per-process report files (`<test name>.<pid>.report`) so parallel tests never share one, register a new test-only tempdir; the shape item 4's survivor work and item 1's per-test name follow.

@history_item n=55 marker=2026-09-24-observability-gates
**2026-09-24-observability-gates** — workspace `print_stdout` / `print_stderr` / `dbg_macro` bans with `tests/contract_lints.rs` (row no-print-macros; viola-e2e denies only `dbg_macro`); applies to tests added to `viola-hypr-tap.rs`.

@history_item n=56 marker=2026-10-18-plugin-precedence-live-readings
**2026-10-18-plugin-precedence-live-readings** (the VIOLA_DIR home step ratified) — home resolution `--home`, `VIOLA_DIR`'s grandparent, default (row viola-home-resolution); tests stay isolated by `--home`, the point item 5's helper names.

@history_item n=57 marker=2026-11-14-budget-gate-removed
**2026-11-14-budget-gate-removed** (Established Decisions 1 of 2) — `decision/test-isolated-viola-home` reworded: each test in its own `--home`, never sharing an endpoint or log; the isolation row items 1 and 5 rest on.
