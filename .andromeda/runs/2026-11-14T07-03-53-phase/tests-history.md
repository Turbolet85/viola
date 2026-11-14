@history_item n=1 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (the OS scope: Linux only, one CI leg) — set the §1 OS scope behind `decision/linux-only`: core on any distribution, CI on `ubuntu-latest`, the 0.2.0 window features on Hyprland, Windows/macOS out at tag `multi-os-final`. This is the exact boundary the seam rests on: Hyprland-only window features, no other compositor or OS arm, and its tests must go green on `ubuntu-latest`, where no compositor runs.

@history_item n=2 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (the OS-gate census reads another OS as a fault) — `contract_os_gated_tests` `faults` reads three ways: an unnamed gate, a stale row, and a gate naming `windows`/`macos` even when named. The counts moved to 39 functions + 2 files = 41 `GATES` rows. Any `cfg` gate the seam's tests add (for example a host-only live probe) needs a `GATES` row with its reason. It may never name another OS.

@history_item n=3 marker=2026-11-04-linux-and-macos-parity
**2026-11-04-linux-and-macos-parity** (the census of OS-gated tests) — built `tests/contract_os_gated_tests.rs`. It walks only `.rs` files directly under `tests/` and `crates/*/tests/`, counting leading `#![cfg]` and gated test functions. A new crate's `tests/` dir joins the walk automatically, so gated seam tests there are counted.

@history_item n=4 marker=2026-11-13-epoch-8-cleanup
**2026-11-13-epoch-8-cleanup** (2. Test Strategy) — added `decision/os-gated-cases-in-binary-root`. A split integration binary keeps every OS-gated case and its gated helpers in its root file, because the census walks only top-level test files. This is a trap if the seam's test binary is split into a module directory.

@history_item n=5 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (the CI matrix [ubuntu-latest] and two workflows) — CI is one `ubuntu-latest` leg in `test`/`perf`/`lint`/`release`, job and artifact names keep `${{ matrix.os }}`, there are two workflows, and the `ci` kill line reads 26 binaries. This backs `decision/linux-only-ci-matrix`, `decision/ci-single-ubuntu-leg` and `decision/single-os-ci-leg`, which the fake-compositor CI green must satisfy.

@history_item n=6 marker=2026-11-08T20-32-52-wrap
**2026-11-08T20-32-52-wrap** (version census 0.2.0) — the founder-confirmed 0.2.0 census: no web or graphical frontend (E4 reads N/A), Linux the one target, `run`'s browser step reads none. It explains the picked absences (browser suite, Windows/macOS, E4). The window features are reached through the compositor seam, never a viola GUI.

@history_item n=7 marker=2026-10-27-bounded-inputs-at-every-boundary
**2026-10-27-bounded-inputs-at-every-boundary** (the bounded-reader enumeration in the Contract suite) — built `tests/contract_bounded_readers.rs`: the literal `BOUNDARIES`/`EXCEPTIONS` tables plus a census of product reader calls, where a read passes only with `take(` in its own statement. Guard: `product_readers_are_each_bounded_or_named`. The seam's `take(MAX_FRAME + 1)` reply read (socket or `hyprctl` stdout) must be a new `BOUNDARIES` row naming its witness, or the census goes red.

@history_item n=8 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** (the bounded readers' cap-driving witnesses) — each `BOUNDARIES` row must name a cap-driving case, for example `statusline_shell_out_past_the_frame_cap_keeps_exactly_the_cap`, `drain_keeps_cap_bytes_of_an_endless_pipe`. A read the cut ends counts as a failed command. This set `decision/bounded-readers-keep-the-cap` and `decision/statusline-shell-out-frame-cap`. Precedent: the seam's reply read needs its own over-cap witness, not a read-only one. The earlier entry owed such witnesses as a PREREQ.

@history_item n=9 marker=2026-10-27-bounded-inputs-at-every-boundary
**2026-10-27-bounded-inputs-at-every-boundary** (the serde_json `unbounded_depth` ban and deny-probes 14/14) — `deny.toml` bans serde_json `unbounded_depth`, with a firing probe (`deny-probes: 14/14 banned, control clean`). Guard: `serde_json_unbounded_depth_is_banned_in_deny_toml`. The seam's JSON reply parsing stays on the default depth limit, and any new ban changes the probe count.

@history_item n=10 marker=2026-10-27-bounded-inputs-at-every-boundary
**2026-10-27-bounded-inputs-at-every-boundary** (the `viola mcp` stdin line cap's tests) — a line past `MAX_FRAME` is refused whole, never truncated or skipped. It logs one `parse-rejected{…, detail:"oversize"}` line holding no byte of the input, and the child test runs under a kill deadline. This is the closest precedent for the seam's "refused past the cap, untrusted, codes-only" reply read and its witnesses.

@history_item n=11 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** (the error-body enumeration in the contract suite) — built `tests/contract_error_bodies.rs` with guards `product_error_writers_are_each_named` and `thiserror_messages_interpolate_nothing`. A new seam error type or writer must be named in the enumeration, and its `thiserror` messages may interpolate no value: reply text must never reach an error.

@history_item n=12 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** (the after-every-test error-body check, replaced and not built) — the error-body check (`error_body_hit`/`assert_error_body_clean` in `tests/support/events.rs`) reads the classes `absolute-path` · `caused-by` · `serde-path` · `planted` and reports place and class, never bytes. Use it to hold the seam's refusal outputs free of reply text and paths.

@history_item n=13 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** (the error bodies' surface witnesses and the planted value) — the §7 planted-value row: a value is planted per test, a hit reads class `planted`, and the report never prints the bytes so no canary reaches JUnit. This is the source of `decision/error-body-check-reports-place-and-class`. A planted window title/class in a fake compositor reply is the natural canary for "reply text reaches no log, error or refusal".

@history_item n=14 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** (the socket directory's error bodies) — `assert_names_nothing` and `digit_runs` hold log lines free of path, pid and uid. This is the existing instrument for the scope's "no reply text, address, pid, path or socket name in any line".

@history_item n=15 marker=2026-10-26-server-verification-before-any-frame
**2026-10-26-server-verification-before-any-frame** (the server-identity security negative and the `security_negatives` suite prefix) — root sync E2E suites are named `tests/{cli,hook,tui,channel,chaos,contract,security_negatives}_<topic>.rs`, and `digit_runs` checks that no pid or start time appears in output. Use this naming and these helpers for any seam negative binary.

@history_item n=16 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** (the IPC boundary's directory check and MCP's refusal) — `check_socket_dir` (via `check_endpoint_dir`) refuses a foreign bit or owner, a symlink or a non-directory, passes an absent path and never chmods. The client lacks the check deliberately, and that is recorded. If the seam uses Hyprland's request socket under `$XDG_RUNTIME_DIR/hypr/<instance>/`, the scope says it needs these same 0700/owner checks or a recorded reason. This is the precedent for both outcomes.

@history_item n=17 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** (the socket directory's security negatives) — the negative is `#![cfg(unix)]` over a test-owned socket root. A foreign owner is tested by injected uid through `socket_dir_verdict`. `setting_does_not_disable_the_socket_dir_check` runs every `SETTINGS` row. This is the shape of `decision/socket-dir-check`, `decision/foreign-owner-by-injected-uid` and `decision/controls-not-disableable` that a compositor-socket check would follow.

@history_item n=18 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** (the socket directory's unit cases) — `socket_dir_verdict_needs_an_owned_directory_with_no_foreign_bit` and `connect_refuses_a_loosened_socket_directory_inside_its_dispatch_span`, where the case opens its span by hand. These are the guards of `decision/socket-directory-check` and `decision/socket-dir-checked-before-any-frame`, and the pattern for a check placed inside a seam call's span.

@history_item n=19 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** (the socket directory's fixtures and helpers) — `tests/support/home.rs` holds `SOCKET_ROOT_VAR` (`XDG_RUNTIME_DIR` after the Linux-only cut), `SocketRoot` and `Form` (`apply`/`assert_as_found`/`restore`). A loosened dir is made by the test on the real directory, never by viola. Any test of a socket under `$XDG_RUNTIME_DIR/hypr/` should reuse these, not copy them.

@history_item n=20 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** (a start's socket directory refusal) — `socket_dir_ready` runs before any snapshot, child or bind; a refused dir is exit 1 `socket-dir-refused`, left as found. This is the source of `contract/viola-run-start/socket-dir-check` and `contract/start-exit-1/socket-dir-refused`. The seam must not disturb this start ordering.

@history_item n=21 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** (exit 21's sixth cause and `revive`'s eleventh code) — added `socket-dir-refused` to `UnreachableCause`, as a code word with its own hint, logged as `process-exit{…, during:"connect"}`. This explains `contract/unreachable-cause/socket-dir-refused` and `contract/unreachable-log-cause/socket-dir-refused`. The seam's closed outcome codes (ok, unsupported, unreachable, malformed, …) must be code words (`contract/code-word/format`).

@history_item n=22 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** (a test's endpoint directory is made private) — a test that places an endpoint in its own directory makes it with `create_private_dir`, never a bare `tempfile::tempdir()`, because 0755 under the umask is refused by `check_endpoint_dir`. The first run refused nine cases. This is the trap behind `decision/private-dir-for-test-endpoints`/`decision/test-endpoint-dirs-made-private`, and it applies to a fake compositor socket bound in a temp dir.

@history_item n=23 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** (the bare temp-dir home and the liveness-check case) — trap: a bare temp-dir home is refused at 0755, and an oracle moved by a shared body can hide a test that never reached its subject. The seam's fake-compositor tests must prove they reach the code under test.

@history_item n=24 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** (the peer euid decision and the wrapper's drop) — the peer-euid decision is held over injected values (`peer_euid_admitted`), with dependency injection and no env, flag or feature seam. This is the precedent behind `decision/cross-user-tested-at-peer-check` and `decision/mocks-only-at-product-seams` for any compositor-socket peer check.

@history_item n=25 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** (the euid wiring, untestable and a known survivor) — each client seam's euid wiring has no committed witness. An always-admit mutant survives by construction, and adding a seam is not allowed. This states the trade in `decision/cross-user-tested-at-peer-check`. A new socket-reading seam must state its own untestable remainder, not add a test seam.

@history_item n=26 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (remove-the-guard witnesses bounded by `WITHIN`) — a control whose neutralised guard lets a product process start needs a bounded witness spawned through `tests/support/cli.rs` `Running`, because an unbounded one hung for 27 minutes. This is the source of `decision/bounded-remove-the-guard-witness`, which the scope's directive items 3 and 4 (`gate.py control` under a kill deadline) rest on.

@history_item n=27 marker=2026-09-27-browser-verdict-reachability
**2026-09-27-browser-verdict-reachability** (root waits at 7 s) — the root `booted_wrapper` bound and `WITHIN` are 7 s, below the 10 s mutants kill. A remove-the-guard pair read 7 s FAIL with the report against 12 s TIMEOUT with the output lost. This is the source of `decision/root-wait-bound`/`decision/within-7s-wait-bound` for any seam test wait.

@history_item n=28 marker=2026-10-20-epoch-5-cleanup
**2026-10-20-epoch-5-cleanup** (the mutants profile's paste-hint class and the verify_window_ kill at 60 s) — standing rule: a kill line moves only for a by-design floor, measured, with a planted-hang control read both ways, never by raising the default. This applies if a seam or compositor test needs a longer kill.

@history_item n=29 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (the bootstrap phases and test data bootstrap without the Windows arms) — the `[profile.ci]` 20 s kill line lists 26 verify-driven binaries, by the rule of the `binary(…)` names on the filter line. `Instant::now() + WITHIN` is at 31 sites in 23 files. A new seam binary that boots a stamped home joins that line, and the counts move by their rules.

@history_item n=30 marker=2026-09-28-mutation-testing-to-the-epoch-boundary
**2026-09-28-mutation-testing-to-the-epoch-boundary** — removed every per-chunk, pre-push and CI mutation gate. `run --mutants` is named-only and runs at the epoch-boundary code audit. This is the origin of `decision/mutation-at-epoch-boundary` and its siblings, and it agrees with the scope's "no mutation run, local or dispatched". The older `decision/mutation-testing` row reading "per chunk" predates this.

@history_item n=31 marker=2026-10-08-first-live-test-and-self-drive
**2026-10-08-first-live-test-and-self-drive** (Path 5 records the classifier's tables; the real CLI has two local routes) — a route entry's live readings of a real session are that chunk's own local measurement, not a test layer, not harness surface, never in CI. This is the source of `decision/live-readings-not-a-test-layer`/`decision/terminal-reply-reading-live-measured`. The scope's L1–L3 live-desktop steps (`hyprctl` readings on the founder's host) fall under this.

@history_item n=32 marker=2026-09-29-verify-stamped-test-homes-and-harness
**2026-09-29-verify-stamped-test-homes-and-harness** (run --local-live specified; its suite, reason and codes closed) — `--local-live` under `CI` is `reason:"live-in-ci"`, exit 2, before any build or spawn. A new closed value takes a Decisions Log entry. This is the source of `contract/run-reason/live-in-ci` and `decision/no-real-cli-in-ci`. Live Hyprland recording must stay off CI in the same way.

@history_item n=33 marker=2026-09-24-fake-agent-and-test-data-fixtures
**2026-09-24-fake-agent-and-test-data-fixtures** (fake-agent contract as built, consumer-first) — operator direction: consumer-first, with no shapes invented before a recorded fixture, and modes landing with their consumers. This is the governing precedent for the fake compositor: replay only recorded replies of the relied-on requests, with no general client shapes.

@history_item n=34 marker=2026-09-24-fake-agent-and-test-data-fixtures
**2026-09-24-fake-agent-and-test-data-fixtures** (fixture naming and `--fixtures` root) — fixtures are named `<Event>.<variant>.json` under a per-version dir, code wins where the doc invented a shape, and there are no mapping tables. This is the naming and versioning precedent for the per-Hyprland-version fixture directory the scope infers.

@history_item n=35 marker=2026-10-20-epoch-5-cleanup-ii
**2026-10-20-epoch-5-cleanup-ii** (the `Mcp.` exchange: its walk, its seed row and its replay) — a new recorded fixture class from a non-claude source, recorded by a test-only tool (`viola-mcp-tap`). Each line is read through `take(MAX_FRAME)`, the scrub and refusal run on record, it has its own schema (`claude-mcp-fixture.v1.json`), its own hygiene walk with planted clean/dirty/schema cases, and a replay test. This is the closest template for recorded Hyprland replies: schema, walk, recorder rule and `decision/fixtures-recorded-by-live-verify`'s second recorder.

@history_item n=36 marker=2026-10-16-unwrapped-sessions-on-the-board
**2026-10-16-unwrapped-sessions-on-the-board** (test data: the fake agent's agents files, the test-scoped PATH, the Agents fixture class, the agents property and fuzz target) — a fake external tool placed on a test-scoped `PATH` answers from files beside its exe, with no env var (the founder's "no third env seam"). Every test-started child runs on a test `PATH` so it never reads the host's live sessions. The new parser got proptest at 512 with committed regressions and a fuzz target with binary seeds. This sets `decision/tests-never-read-host-sessions`/`decision/fake-agent-on-test-path`, and it is the direct model for a fake `hyprctl` and for keeping tests off the founder's live compositor.

@history_item n=37 marker=2026-09-29-fake-agent-drift-contract
**2026-09-29-fake-agent-drift-contract** (the fake-agent drift contract, byte for byte) — fake output is compared byte for byte with recorded fixtures, never through insta, and an empty walk fails. The recorded files are the pinned artifact. This is the source of `decision/fixture-drift-byte-compare` and siblings, and the precedent for holding the fake compositor to its recordings.

@history_item n=38 marker=2026-09-28-capability-ledger-and-viola-verify
**2026-09-28-capability-ledger-and-viola-verify** (… the hygiene walk over the first recorded set) — fixture hygiene is a run-time directory walk, not rstest `#[files]`, because rstest 0.27 refuses an empty glob at compile time. This is the why of `decision/fixture-hygiene-walk`, and it holds for a new compositor-fixture walk that starts empty.

@history_item n=39 marker=2026-10-13-owed-ledger-rows
**2026-10-13-owed-ledger-rows** (… the kept workspace.repo) — the only hygiene exception: a `workspace.repo` naming this repo's own remote is kept, on the founder's word, and nothing else in the rule widens. The except-clauses of `decision/fixture-hygiene-walk`/`decision/recorder-scrub`/`decision/no-production-data-in-tests` come from this. Window titles or classes in recorded Hyprland replies get no such exception unless ruled.

@history_item n=40 marker=2026-11-13-rows-proposal-rule-work
**2026-11-13-rows-proposal-rule-work** (7. Test Data & Fixtures) — `decision/hygiene-reports-class-only` lost its closed class list (the email class joined). The checker reports class only, never content. A new fixture walk follows the same rule.

@history_item n=41 marker=2026-10-05-real-cli-verify-probes
**2026-10-05-real-cli-verify-probes** (… the screen fixture class) — a new fixture class (`Screen.*.json`) is walked apart against its own schema. It refuses email-shaped tokens and seam-split usernames, and a dirty file is refused whole with a named code. This is the precedent for a new schema per fixture kind (`decision/fixtures-schema-checked`).

@history_item n=42 marker=2026-09-24-quality-gates
**2026-09-24-quality-gates** (… seeded fuzz replay …) — fuzz targets live in the separate `fuzz/` workspace on its own toolchain file, with a seed target and replay `fuzz-linux-only`/`tool-missing`/`corpus-empty`. This is the basis for `decision/fuzz-separate-workspace`, `decision/fuzz-corpus-never-vacuous` and `contract/run-reason/corpus-empty`, if the Hyprland reply parser joins as a parser surface.

@history_item n=43 marker=2026-10-04-readiness-gate-and-timing-constants
**2026-10-04-readiness-gate-and-timing-constants** (… the vt100 property and fuzz target …) — a new parser joins with a proptest at 512 cases and a cargo-fuzz target with synthetic seeds, as `vt100_feed` did. This is the precedent for `decision/property-tests-on-parsers`/`decision/fuzzing-with-cargo-fuzz`, where "each parser joins with its own target".

@history_item n=44 marker=2026-10-04-confirmed-send-with-cl-1-records
**2026-10-04-confirmed-send-with-cl-1-records** (Path 2 as landed; paste_text joins the fuzz targets) — `paste_text`'s seeds are byte-exact under `.gitattributes` binary. This is the source of `decision/fuzz-corpora-byte-exact` for any new target's corpus.

@history_item n=45 marker=2026-10-04-confirmed-send-with-cl-1-records
**2026-10-04-confirmed-send-with-cl-1-records** (the second test-data carve-out: the chaos home (F4)) — `tests/chaos_feed_panic.rs` alone boots outside G2, G4 and the secret scan, by the founder's ruling. This is the except of `decision/fresh-home-per-test`. A seam test needs no carve-out, and any new one would need a ruling.

@history_item n=46 marker=2026-10-25-windows-endpoint-admission
**2026-10-25-windows-endpoint-admission** (scoped log captures through with_scoped_default) — every scoped log capture in root-bin unit tests goes through `crate::test_support::with_scoped_default`, a pinned always-interested dispatch. The three crate-level captures run on a bare scoped default with no owner. This is the source of `decision/scoped-capture-pinned-dispatch`. Codes-only seam logging tests use this helper, and a seam placed in a new crate meets the bare-default exception.

@history_item n=47 marker=2026-10-16-remove-the-browser-leg
**2026-10-16-remove-the-browser-leg** (Log format: the `ui` process, the four web events and the a11y rows leave) — process logs are `run|hook|mcp|cli` role files under a closed event vocabulary, kept equal to obs-plan §3. This is the base of `contract/log-line/event`/`decision/codes-only-role-logs`. A new seam outcome event or code is a closed-enum change.

@history_item n=48 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** (`channel-response`'s closed `detail` and the wire bodies) — the `diag-line.v1` schema closes a line's `detail` to named words, held by the schema check. The seam's closed outcome codes on log lines must be admitted by `schemas/diag-line.v1.json` or G4 (`decision/diagnostics-schema-conformance`) fails.

@history_item n=49 marker=2026-10-03-mutation-scoring-completion
**2026-10-03-mutation-scoring-completion** (pre-push native on the Linux host) — `pre-push` is Linux-host only, with stages `host · tools · linux-tests` and exit 2 `pre-push-linux-only`. Closed-enum changes are recorded in the amendment in place of a Decisions Log entry. This explains the picked `contract/pre-push-*` rows and the operator pass this chunk's pre-CI commit runs.

@history_item n=50 marker=2026-09-29-t15-07-57-wrap
**2026-09-29-t15-07-57-wrap** (registry migration (U35)) — the test-plan Decisions Log left the body for the archive, with in-force items lifted into sections. Read `decision/closed-enums-need-decisions-log` with this: a new closed value now lands as an amendment and row, not a dated §12 entry.

@history_item n=51 marker=2026-10-17-end-to-end-path-binary-and-harness-legs
**2026-10-17-end-to-end-path-binary-and-harness-legs** (tests: run --e2e, the E2E regex widened, the integration complement) — the E2E layer regex is `^(path[0-9]*|tui|mcp|cross|chaos|contract)_`, integration is its complement, and an unmatched `binary()` regex is a parse error. This is the basis of `decision/test-layer-filtersets`/`decision/integration-binary-surface-prefix`, and the naming of a seam test binary picks its layer.

@history_item n=52 marker=2026-10-09-epoch-3-cleanup-ii
**2026-10-09-epoch-3-cleanup-ii** (the shared root helpers named …) — root tests share `tests/support/{home,outer_pty,fake,events,cli,ndjson}.rs`, and no root test file defines its own copy. This is the source of `decision/one-shared-test-helper`. A fake-compositor runner or reply helper used by two files goes in `tests/support/`.

@history_item n=53 marker=2026-11-04-linux-and-macos-parity
**2026-11-04-linux-and-macos-parity** (the Frozen guard's one home) — two local `Frozen` twins were retired into `tests/support/frozen.rs`, so the no-copy rule now names the freeze guard. This shows the rule is enforced at wrap by moving duplicates.

@history_item n=54 marker=2026-10-07-a-send-ending-in-a-newline-is-confirmed
**2026-10-07-a-send-ending-in-a-newline-is-confirmed** — measured: an instrumented child killed during its exit-time profile write leaves a short profile, and the start test's child became `whoami`, uninstrumented. This is the source of `decision/killed-test-child-uninstrumented`. A fake compositor or `hyprctl` stand-in killed right after start must be uninstrumented, or exit by itself.

@history_item n=55 marker=2026-09-29-h2-conpty-resize-probe
**2026-09-29-h2-conpty-resize-probe** (coverage: the harness self-tests' throwaway crate writes no profile into the outer run) — the one deliberate exception to child coverage: the nested cargo over the throwaway crate strips the four llvm-cov variables through `test_support::uninstrumented`. This is the except of `decision/child-process-coverage`/`decision/coverage-includes-child-processes`. `env_clear` tests must re-add `LLVM_PROFILE_FILE`.

@history_item n=56 marker=2026-10-26-server-verification-before-any-frame
**2026-10-26-server-verification-before-any-frame** (fake-agent copies planted one after another (ETXTBSY)) — on the tmpfs `target/e2e-home`, a test planting several fake executables must plant every copy before any thread spawns a child (ETXTBSY otherwise: 6 of 40 red). This is a trap for planting a fake `hyprctl`.

@history_item n=57 marker=2026-10-27-pinned-copy-race-and-reproducible-builds
**2026-10-27-pinned-copy-race-and-reproducible-builds** (the pinned copy's no-clobber placement …) — a placed copy is persisted only once its write handle is closed. Guard: `place_private_new_placed_copy_execs_at_once`. The `placed` hook is a test parameter, not a seam. This is the source of `decision/placed-copy-execs-at-once`, and it is relevant if the seam's tests place an executable stand-in.

@history_item n=58 marker=2026-10-26-server-verification-before-any-frame
**2026-10-26-server-verification-before-any-frame** (the server-identity impostor: its held process, its listener, its test-built snapshot) — the Fake servers row admits a test-owned listener bound through the product API beside real held processes, killed and reaped by `Drop` guards. Process faults are real, never doubles. This is the precedent for a fake compositor as a test-owned socket server (`decision/no-http-fake-servers`, `decision/external-processes-real-stubs`, `decision/real-process-faults`).

@history_item n=59 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (the home-widening helpers and the Filesystem faults row) — `widen`/`grant_read`/`unwiden`/`still_widened` live in `tests/support/home.rs`. Filesystem faults are made on the real file and refused by the product's own check, never stubbed, with the pure verdict alone taking injected values. This is the source of `decision/real-filesystem-faults`.

@history_item n=60 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (program resolution on PATH, the .cmd/.bat refusal retired) — `resolve_program` takes a given path as given and a bare name on `PATH` (`AgentError::ProgramNotFound`). This is the existing resolution a `hyprctl` transport would reuse: a direct spawn, no shell, an absolute path (scope inferred item), tested over a test-scoped `PATH`.

@history_item n=61 marker=2026-10-17-cli-machine-contract
**2026-10-17-cli-machine-contract** (… the completeness case …) — `every_flag_variable_and_config_key_is_a_row_of_the_table` walks the `VIOLA_*` literals of the product sources and the config schema against `SETTINGS`. A test-started `viola` drops `VIOLA_NAME`/`VIOLA_DIR`/`VIOLA_BIN`. A new environment read (`HYPRLAND_INSTANCE_SIGNATURE`) is outside the `VIOLA_*` walk, so `decision/controls-not-disableable` and the env-read log entry must be met by hand.

@history_item n=62 marker=2026-10-10-viola-revive
**2026-10-10-viola-revive** (the fake agent's resume payload and ten options; … the kill path's endpoint wait) — a killed wrapper leaves its socket file on Unix, so a wait for the file to vanish never ends. The kill path waits on `holder_gone` (a connect refused or the path absent). This is the source of `decision/killed-wrapper-endpoint-wait`, and the same trap applies to a killed fake compositor's socket.

@history_item n=63 marker=2026-11-06-version-done-check
**2026-11-06-version-done-check** (the CLI tests over std::process::Command, fixtures, gates, bans and contracts) — no manifest holds `assert_cmd`/`predicates`/`trycmd`/`snapbox`. Children run over `std::process::Command`, and the long-lived `viola run` is spawned under a `Drop` guard, never `.output()`/`.status()`. This is the source of `decision/no-cli-snapshot-crates` and `decision/wrapper-spawned-with-guard`.

@history_item n=64 marker=2026-09-24-supply-chain-and-workflow-gates
**2026-09-24-supply-chain-and-workflow-gates** (Lint row from sync-crates.txt …) — `scripts/sync-crates.txt` drives one `cargo check -p` per crate and the sole-root tokio ban per crate, an empty list fails, and `viola-e2e` is never a ban root. If the seam lands as a new sync workspace crate, it joins this list (`decision/sync-crates-compile-without-features`, `decision/tokio-banned-per-sync-crate`).

@history_item n=65 marker=2026-10-17-mcp-server-for-drivers
**2026-10-17-mcp-server-for-drivers** (the CI gates: the channel tree form, rmcp features, deny-probes 13/13) — the sync-crate list was cut to four after `viola-channel` left it, because no listed crate depended on it, and a crate's list membership follows its tokio standing with a dedicated tokio-free probe. This is the precedent for how a new seam crate's sync/tokio-free standing is checked.

@history_item n=66 marker=2026-09-24-workspace-tree-and-code-graph-planes
**2026-09-24-workspace-tree-and-code-graph-planes** (Lint row disposed, release-check job …) — `scripts/orphans-check.sh` runs per lib and bin target with a planted-orphan probe, and the release check runs `--probe` then default. A new crate or module is covered by `decision/no-orphan-modules`/`decision/cargo-modules-orphans-gate` and `decision/release-build-viola-only`. A fake compositor binary must be test-only and stay out of the release build.

@history_item n=67 marker=2026-11-13-verify-readiness-on-the-input-box
**2026-11-13-verify-readiness-on-the-input-box** (9. CI Integration) — `decision/no-ignored-advisories` gained its only except: rmcp advisories confined to banned or disabled features. Any new dependency the seam brings (a Hyprland client crate) faces `cargo deny` with no other ignore.
