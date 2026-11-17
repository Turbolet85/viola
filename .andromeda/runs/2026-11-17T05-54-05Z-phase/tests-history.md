@history_item n=1 marker=2026-09-24-three-os-ci-headless-harness-skeleton
**2026-09-24-three-os-ci-headless-harness-skeleton** (interim supervisor, readiness, status and cleanup) — made the harness supervisor an ordinary child (no detach), added cleanup's `processes_gone` and the "no field false" assertion; the precedent for decision/supervisor-ordinary-child and contract/cleanup-document/processes-gone, which the rig's own end-what-it-starts proof (process census at exit) extends.

@history_item n=2 marker=2026-09-24-supply-chain-and-workflow-gates
**2026-09-24-supply-chain-and-workflow-gates** — `viola-e2e` is never a sole root of the tokio ban (`deny-sync.toml`, `scripts/deny-probes.sh`); a rig placed as a test-only `viola-e2e` binary inherits this, and decision/test-only-e2e-crate / decision/tokio-only-in-e2e-crate rest on it.

@history_item n=3 marker=2026-09-24-workspace-tree-and-code-graph-planes
**2026-09-24-workspace-tree-and-code-graph-planes** — the per-target orphans gate (`scripts/orphans-check.sh`) and the `release` job (`scripts/release-check.sh --probe`, `--bin viola`); any new rig binary or module must pass decision/cargo-modules-orphans-gate and stay out of the release build (decision/release-build-viola-only).

@history_item n=4 marker=2026-09-25-pty-wrapper-on-windows
**2026-09-25-pty-wrapper-on-windows** — the 11-name identity floor replaced the 14-name literal (rows decision/r8-claude-env-strip, decision/identity-floor-tested-against-literal-copy); harness `supervise` owns an outer PTY per instance and cleanup re-presses Ctrl-C every 500 ms (decision/cleanup-ctrl-c-repeated-until-exit); fake-agent receipts gained `size`, `cwd`, `hold`. The rig's own pty and stop path follow these.

@history_item n=5 marker=2026-09-27-epoch-2-cleanup
**2026-09-27-epoch-2-cleanup** — the release-check probe reads `5/5 refused, control clean` and also fails an artifact built with a test-only feature; a rig binary must be a test-only artifact this check keeps out (decision/release-build-viola-only, decision/no-test-binary-in-release).

@history_item n=6 marker=2026-09-27-instance-state-and-start-order
**2026-09-27-instance-state-and-start-order** — mutation runs keep no home (`AGENT_RUN_KEEP_HOMES=0`), a fixture owner record and a gone-owner sweep by pid + start time; the earlier leak-cleanup mechanism the rig's process-hygiene census stands beside (decision/pid-start-time-verified-kill, decision/session-home-retention).

@history_item n=7 marker=2026-09-27-wrapper-channel
**2026-09-27-wrapper-channel** — E2 fd premise fixed to the live `/proc/<pid>/fd` table (portable-pty `close_random_fds()`); `tests/support/ndjson.rs` named the one complete-lines reader (decision/complete-lines-reader). A rig reading `events.ndjson` live must use that reader, not its own.

@history_item n=8 marker=2026-09-28-mutation-testing-to-the-epoch-boundary
**2026-09-28-mutation-testing-to-the-epoch-boundary** — no per-chunk, pre-push or CI mutation gate; mutation only at the epoch boundary and under `run --mutants` (decision/mutation-at-epoch-boundary, decision/mutation-gate-cadence). Backs the scope's "no mutation run" directive; note picked row decision/mutation-testing (per chunk) contradicts it.

@history_item n=9 marker=2026-09-29-h2-conpty-resize-probe
**2026-09-29-h2-conpty-resize-probe** — a test writing a key after a resize waits for the child's size report (decision/key-after-resize-waits-for-size), never a timer or retry; whether the real `claude` loses a key typed right after a resize was left unmeasured — the "Key after a resize" CARRY (DEL fidelity) the rig is to carry.

@history_item n=10 marker=2026-09-29-verify-stamped-test-homes-and-harness
**2026-09-29-verify-stamped-test-homes-and-harness** (run --local-live specified; stamped root seam) — `--local-live` refuses `live-in-ci` (exit 2) before any build, then builds and runs one verify against the real `claude` through the runner seam, claiming no live measurement beyond the rows; `Wrapper::stop` waits for the endpoint unconnectable. Rows decision/live-verify-never-in-ci, decision/local-live-runner-seam, contract/run-reason/live-in-ci, contract/local-live-suite/pass, decision/killed-wrapper-endpoint-wait: the rig must not become a second live-in-CI path and its tests must use a stand-in for the real CLI.

@history_item n=11 marker=2026-09-29-fake-agent-drift-contract
**2026-09-29-fake-agent-drift-contract** (the drift contract byte for byte; change-driven size) — `contract_fake_agent_drift` compares each recorded set byte for byte, never insta, an empty walk failing; the `size` receipt became a 10 ms watcher with no key needed. Rows decision/fake-agent-contract-suite, decision/fixture-drift-byte-compare, contract/fake-agent-receipt/size; any modal/trust fidelity the rig feeds into the fake agent lands under this contract.

@history_item n=12 marker=2026-09-29-fake-agent-drift-contract
**2026-09-29-fake-agent-drift-contract** (harness prefixes: four) — the harness-prompt class (`<agent-message from=`, `<task-notification>`, the cross-session tags); the classification step v2-36 would widen, which the rig's scheduled-wake `source` reading decides.

@history_item n=13 marker=2026-09-24-fake-agent-and-test-data-fixtures
**2026-09-24-fake-agent-and-test-data-fixtures** (fake-agent contract as built, consumer-first; fixture naming) — fake-agent modes land with their consumers, no shapes before a recorded fixture; fixtures named `<Event>.<variant>.json` under `<cli-version>` (contract/claude-fixtures/payload-path). Constrains any trust/modal screen the rig wants the fake agent to draw.

@history_item n=14 marker=2026-09-28-capability-ledger-and-viola-verify
**2026-09-28-capability-ledger-and-viola-verify** — the fixture hygiene walk is a run-time walk (`claude_fixtures`), not rstest `#[files]` (rstest refuses an empty glob); `verify`'s lines pinned by literal. Rows decision/fixture-hygiene-walk, decision/human-output-pinned-by-literals.

@history_item n=15 marker=2026-10-04-dialog-answers-by-dialog-id
**2026-10-04-dialog-answers-by-dialog-id** — print mode raises no dialog hook, so the dialog tier was relayed from viola-lab's live captures, redacted and reviewed before commit; a precedent for live captures becoming fixtures and for the review the rig's kept screens would need.

@history_item n=16 marker=2026-10-04-confirmed-send-with-cl-1-records
**2026-10-04-confirmed-send-with-cl-1-records** (the second carve-out: the chaos home) — `tests/chaos_feed_panic.rs` alone boots outside `target/e2e-home` and the G2/G4/secret scans (decision/chaos-home-outside-scan, decision/fresh-home-per-test except). A rig scratch root outside the repo is a further carve-out needing its own ruling.

@history_item n=17 marker=2026-10-04-the-wheel
**2026-10-04-the-wheel** — Path 5 outer-PTY wheel tests (`tests/tui_wheel.rs`): focus/mouse/resize never take the wheel, human keys do. Rows decision/human-input-takes-the-wheel, decision/focus-mouse-resize-keep-wheel; the rig typing keys into a live session (trust dialog) moves the wheel.

@history_item n=18 marker=2026-10-04-running-turn-refusal
**2026-10-04-running-turn-refusal** (the fake agent quiesces its hooks before it exits) — the fake agent exits on `\x03` or stdin EOF and waits for a running hook first (contract/fake-agent-exit/exit-triggers); it was the truncated-profile cause. Also Path 5/2 span the running turn (contract/viola-send/turn-running).

@history_item n=19 marker=2026-10-05-real-cli-verify-probes
**2026-10-05-real-cli-verify-probes** — verify's interactive runs, the screen fixture class (signature rows only, scrub-as-detector, refused whole), `--trusted-root`/`--screens` for wrapper boots. Rows decision/screen-fixture-scrub-as-detector, decision/wrapper-boots-pass-trusted-screens, contract/verify-recording-refusal/unable-message: the rig's kept screens and trust handling sit against this existing screen pipeline.

@history_item n=20 marker=2026-10-05-dialog-rows-and-re-probe
**2026-10-05-dialog-rows-and-re-probe** — dialog tier recorded live by verify Runs C/D, relayed set superseded; the verify-driven kill (20 s) and `verify_window_` override sized as a designed floor. Precedent for the modal readings and kill-line sizing (decision/verify-driven-tests-20s-kill).

@history_item n=21 marker=2026-10-07-test-homes-off-the-contended-volume
**2026-10-07-test-homes-off-the-contended-volume** — the dev host's `target/e2e-home` is a link to tmpfs `/tmp/viola-e2e-home-<uid>`, the keeper runs before `--local-live`'s verify; standing rule: a stalled-start red is a finding about the backing, never re-run for green. Bears on where the rig's scratch root and live homes sit.

@history_item n=22 marker=2026-10-07-live-rows-and-paste-shapes-on-the-dev-host
**2026-10-07-live-rows-and-paste-shapes-on-the-dev-host** — `unwrap_pastes` changed on a live measurement, measured shapes added as literal oracles, failing readings kept in evidence; the floor's differing Linux dev-host reading (12 names) recorded beside the eleven. Precedent for how live readings become literal oracles (decision/literal-test-oracles).

@history_item n=23 marker=2026-10-07-send-waits-out-the-paste-hint
**2026-10-07-send-waits-out-the-paste-hint** — `GATE_MAX_WAIT` 8 500 ms (contract/readiness-gate/gate-max-wait); a kill line moves only for a by-design floor, recorded both ways with a planted hang. Bounds any rig wait on readiness.

@history_item n=24 marker=2026-10-07-a-send-ending-in-a-newline-is-confirmed
**2026-10-07-a-send-ending-in-a-newline-is-confirmed** — the matcher reads the typed text; an instrumented child killed in its exit-time profile write leaves a short profile, so the start test's child became uninstrumented `whoami` (decision/killed-test-child-uninstrumented); trap: the wrapper puts its plugin flag first, so a test binary can never be its child. Applies to a rig that kills what it starts.

@history_item n=25 marker=2026-10-08-first-live-test-and-self-drive
**2026-10-08-first-live-test-and-self-drive** (`--local-live` alone runs no other suite; Path 2 live readings) — the seventeen-row live firing; `verify` stamps the first `claude` on `PATH`; live readings of trailing newlines, `/clear`, a lone trailing CR recorded. Trap: an outcome read live may have no pin at any tier.

@history_item n=26 marker=2026-10-08-first-live-test-and-self-drive
**2026-10-08-first-live-test-and-self-drive** (Path 5 classifier tables; the real CLI has two local routes) — a route entry's live readings of a real session are that chunk's own local measurement, not a test layer, not harness surface, never in CI; the terminal-reply reading is live-measured. Source of decision/live-readings-not-a-test-layer, decision/terminal-reply-reading-live-measured, decision/real-claude-local-only — the core constraint on where a standing rig may live.

@history_item n=27 marker=2026-10-09-epoch-3-cleanup
**2026-10-09-epoch-3-cleanup** (the typed text, the `empty-text` refusal) — the typed text drops every trailing CR and LF; `send` order control-character → empty-text → human-typing. Rows decision/sent-text-typed-form, decision/send-refusal-order, contract/viola-send/trailing-newlines.

@history_item n=28 marker=2026-10-09-inner-cr-and-crlf-in-a-sent-text
**2026-10-09-inner-cr-and-crlf-in-a-sent-text** — every inner CR / CR LF typed as one LF, pinned at two tiers and read live on 2.1.287; a fake-agent green proves it only by the typed bytes, so live readings stand as local measurement. Rows contract/viola-send/inner-cr, contract/wrapper-channel/accepted-text; the precedent for the carried whitespace-trim reading.

@history_item n=29 marker=2026-10-09-epoch-3-cleanup-ii
**2026-10-09-epoch-3-cleanup-ii** — the shared root helpers `tests/support/{home.rs,outer_pty.rs,fake.rs,events.rs,cli.rs,ndjson.rs}`; no root test file defines its own copy. A rig must reuse, not copy, these.

@history_item n=30 marker=2026-10-10-viola-revive
**2026-10-10-viola-revive** (the fake agent's resume payload; the kill path's endpoint wait) — a killed wrapper leaves its socket file, so `Wrapper::kill` waits for the exit, then the recorded child gone by pid and start time, then `holder_gone`; the live resume payload's 10 keys were read but not made a fixture. Bears on the rig's kill path and process census.

@history_item n=31 marker=2026-10-10-budget-governor
**2026-10-10-budget-governor** (the state-file ban scoped) — no booted-wrapper test hand-writes `stamps.json`/`snapshot.json`; a reader's unit cases may alter the writer's bytes only. Row decision/state-files-never-hand-written.

@history_item n=32 marker=2026-10-16-unwrapped-sessions-on-the-board
**2026-10-16-unwrapped-sessions-on-the-board** (test data) — the fake agent as `claude` in a test-scoped bin dir answers from files beside its exe, no env var; every test-started `list`/`revive` runs under a test-scoped PATH because a test otherwise read the host's live sessions (11 on the dev host). Rows decision/fake-agent-on-test-path, decision/tests-never-read-host-sessions: the isolation precedent for the rig.

@history_item n=33 marker=2026-10-17-end-to-end-path-binary-and-harness-legs
**2026-10-17-end-to-end-path-binary-and-harness-legs** (run --e2e, the E2E regex widened) — the E2E layer is `^(path[0-9]*|tui|mcp|cross|chaos|contract)_` and integration its complement. Rows decision/e2e-layer-by-binary-prefix, decision/test-layer-filtersets: a rig binary's name decides which layer (if any) runs it.

@history_item n=34 marker=2026-10-17-end-to-end-path-binary-and-harness-legs
**2026-10-17-end-to-end-path-binary-and-harness-legs** (boot's two flags, harness_session, status) — `harness_session` is a plain function + drop guard over a spawned `supervise` so cleanup's pid + start-time proof stays exact; homes kept only under `AGENT_RUN_KEEP_*`. Pattern for a rig that ends what it starts (decision/e2e-boot-model-by-cleanup, decision/supervisor-spawned-only-by-boot).

@history_item n=35 marker=2026-10-17-cli-machine-contract
**2026-10-17-cli-machine-contract** — a test-started `viola` drops `VIOLA_NAME`/`VIOLA_DIR`/`VIOLA_BIN`, and a path oracle compares the resolved home. Rows decision/test-started-viola-drops-env, decision/test-viola-drops-session-vars, decision/home-resolution-order; a rig run from inside a viola session must drop them.

@history_item n=36 marker=2026-10-17-mcp-server-for-drivers
**2026-10-17-mcp-server-for-drivers** (the MCP tests as landed; harness keys) — `viola-e2e` takes rmcp `client` only as a normal dependency, tokio `rt` + `process`, no child-process transport (it killed the server and truncated a profile). Rows decision/viola-e2e-rmcp-client-only; constrains a rig placed in `viola-e2e`.

@history_item n=37 marker=2026-10-18-plugin-precedence-live-readings
**2026-10-18-plugin-precedence-live-readings** — the `VIOLA_DIR` home step and canonicalisation ratified by the founder (decision/home-resolution-order).

@history_item n=38 marker=2026-10-19-overseer-switch-to-viola
**2026-10-19-overseer-switch-to-viola** — `background_tasks` and `option_details` cases plus live readings on 2.1.296 stated as local measurement, no suite; the sibling `background_tasks` shape the scheduled-wake reading mirrors.

@history_item n=39 marker=2026-10-19-newer-claude-cli-rows
**2026-10-19-newer-claude-cli-rows** (long-turn screen, second literal; readiness stall report) — two input-box literals (`for agents`, `(shift+tab to cycle)`), `--long-turn-screen`; `wait_ready` appends the stalled wrapper's last event, `/proc/<pid>/wchan` and state before the panic. Rows decision/screen-signatures-only, decision/readiness-stall-report; the rig's screen model reads these literals.

@history_item n=40 marker=2026-10-20-epoch-5-cleanup
**2026-10-20-epoch-5-cleanup** (the headroom floor, the hook hangup named) — `HEADROOM_FLOOR_MIB` refusal on the tmpfs base; every command the fake agent spawns runs in its own process group, so a PTY hangup no longer cuts a hook's profile. Rows decision/fake-agent-children-own-process-group, decision/fake-agent-own-process-group.

@history_item n=41 marker=2026-10-20-epoch-5-cleanup-ii
**2026-10-20-epoch-5-cleanup-ii** — `viola-mcp-tap`, a test-only recorder between a live `claude -p` and `viola mcp`, its capture 0600 and never committed, its fixtures through the `--record` scrub. The nearest precedent for a test-only live binary (rows decision/fixtures-recorded-by-live-verify, decision/recorder-scrub).

@history_item n=42 marker=2026-10-26-server-verification-before-any-frame
**2026-10-26-server-verification-before-any-frame** (fake-agent copies planted one after another) — on tmpfs each copy is written, and a child forked by one thread inherits another's write handle (ETXTBSY). Row decision/fake-agent-copies-planted-first; applies to a rig planting binaries concurrently.

@history_item n=43 marker=2026-10-26-server-verification-before-any-frame
**2026-10-26-server-verification-before-any-frame** (the fake agent's exit ignores SIGHUP) — the agent, its PTY's session leader, ignores SIGHUP from its exit's first step. Row decision/fake-agent-exit-ignores-sighup.

@history_item n=44 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (remove-the-guard witnesses bounded by `WITHIN`) — an unbounded witness hung 27 minutes; a control whose neutralised guard lets a product process start needs a bounded witness. Row decision/bounded-remove-the-guard-witness; matches the directive that every channel/thread wait takes a bound.

@history_item n=45 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (security control negatives; a home a test makes before viola) — a home another user may read refuses `viola run`; a test making a home before viola uses `create_private_dir`, never `create_dir_all`. Rows decision/home-another-user-may-read-refused, decision/private-filesystem-permissions; governs the rig's scratch/config dirs.

@history_item n=46 marker=2026-10-27-pinned-copy-race-and-reproducible-builds
**2026-10-27-pinned-copy-race-and-reproducible-builds** — `fs::place_private_new` places mode-first with no temp left and no clobber; the `hook` receipt carries `spawn_os_error`. Row decision/private-placement-mode-first; a credentials copy the rig places should follow it.

@history_item n=47 marker=2026-10-27-bounded-inputs-at-every-boundary
**2026-10-27-bounded-inputs-at-every-boundary** (feed-panic passthrough witnessed on the outer terminal) — the forced vt100 panic case byte-searches the outer terminal's drained stream, never parses the child screen. Rows decision/no-child-screen-assertions, decision/never-parse-rendered-child-screen, decision/tui-passthrough-never-parses-screen: a rig screen model is evidence, never a test oracle.

@history_item n=48 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** (the after-every-test error-body check) — no per-test scan helper; the secret scan runs after each job's E2E run; scan hits report place and class, never bytes. Rows decision/scan-hits-never-echo-bytes, decision/error-body-check-reports-place-and-class, decision/no-secrets-in-logs; kept rig screens/records must name no host path.

@history_item n=49 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** (a stale holder; a wrapper lost under an open call) — `Frozen` a real `SIGSTOP` guard, a lost wrapper a real kill (`kill_wrapper`), no double. Row decision/real-process-faults.

@history_item n=50 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** (a test's endpoint directory is made private; socket fixtures) — a bare `tempfile::tempdir()` reads 0755 and the socket-dir check refuses it; a test-owned socket root under `/tmp` (`SOCKET_ROOT_VAR`, `XDG_RUNTIME_DIR`). Rows decision/private-dir-for-test-endpoints, decision/test-endpoint-dirs-made-private, contract/channel-endpoint/socket-path: the directive's "test socket under the short `/tmp` root".

@history_item n=51 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** (the socket directory's security negatives; `viola list` on the read rule) — socket dir at 0755 or a symlink refuses start, frame verbs and hook, left as found; a clean start under umask 000 keeps the socket private; `viola list` refuses a readable home. Rows decision/socket-directory-refusal, decision/socket-dir-refusal, decision/umask-independent-modes.

@history_item n=52 marker=2026-11-04-linux-and-macos-parity
**2026-11-04-linux-and-macos-parity** (the census of OS-gated tests) — `tests/contract_os_gated_tests.rs` with its `GATES` table; an unnamed or stale gate fails. Rows decision/os-gated-tests-registered, decision/os-gated-tests-each-named.

@history_item n=53 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (the OS scope: Linux only, one CI leg) — Linux only, CI on `ubuntu-latest`, Windows/macOS cut at tag `multi-os-final`. Rows decision/linux-only, decision/single-os-ci-leg, decision/tests-run-on-linux-ubuntu-latest.

@history_item n=54 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (the PTY seam on openpty alone) — PTY tests on real openpty, exit on the handle; the tui row's mode is openpty. Rows decision/pty-tested-on-real-openpty, decision/child-exit-read-on-the-process-handle, decision/pty-exit-from-wait-not-eof.

@history_item n=55 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (the R8 strip by exact name) — the persistent exception is `config.json` `claude_env_keep`, strip by exact name, floor stays eleven. Rows decision/claude-env-strip, decision/child-env-strip; the rig's isolated config/env must reckon with what viola strips.

@history_item n=56 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (the OS-gate census reads another OS as a fault) — a gate naming windows or macos is a fault even when named (decision/linux-only guard `contract_os_gated_tests`).

@history_item n=57 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (the relied set, all twenty-four rows) — `LEDGER_ROWS` `[&str; 24]`; the H2 key-after-resize question moved to residuals targeted `another-os`.

@history_item n=58 marker=2026-11-06-version-done-check
**2026-11-06-version-done-check** (the portable-pty pin at =0.9.0; the child's fd inheritance on 0.9.0) — portable-pty `=0.9.0`; the live child's fd table read (31 fds) as a local live measurement, no suite. The rig's pty crate pin and a precedent live reading (decision/fd-test-reads-live-child-table).

@history_item n=59 marker=2026-11-06-version-done-check
**2026-11-06-version-done-check** (CLI tests over std::process::Command) — no `assert_cmd`/`trycmd`; long-lived `viola run` started with `.spawn()` + a Drop guard (`tests/support/cli.rs` `Running`), never `.output()`. Rows decision/cli-tests-as-subprocess, decision/wrapper-spawned-with-guard, decision/cli-e2e-via-process.

@history_item n=60 marker=2026-11-06-version-done-check
**2026-11-06-version-done-check** (the statusline-rate-limits arm's live red) — a live verify read red because the row read the account's plan state, not the CLI; a trap for a rig whose readings ride the founder's real account/config.

@history_item n=61 marker=2026-11-08T20-32-52-wrap
**2026-11-08T20-32-52-wrap** — `--local-live`'s H2 key-after-resize question now belongs to the route entry «Key after a resize», the CARRY whose DEL fidelity rides this rig.

@history_item n=62 marker=2026-11-13-verify-readiness-on-the-input-box
**2026-11-13-verify-readiness-on-the-input-box** (4. Unit Test Strategy; 7. Test Data & Fixtures) — `~ contract/readiness-gate/gate-max-wait` (the input box reads up on its rule and prompt rows with no footer literal) and `+ decision/screen-fixture-box-rows-as-literals`; the rig's screen model reading the box and its kept screens must agree with these.

@history_item n=63 marker=2026-11-13-epoch-8-cleanup
**2026-11-13-epoch-8-cleanup** — `~ decision/logs-carry-no-secrets`, `~ decision/no-secrets-in-logs`, `~ decision/secrets-never-in-logs` (Cookie clause retired) and `+ decision/os-gated-cases-in-binary-root`.

@history_item n=64 marker=2026-11-13-rows-proposal-rule-work
**2026-11-13-rows-proposal-rule-work** — `~ decision/pty-tested-on-real-openpty` (a resize read from the child's size report alone, no key after it) and `~ decision/hygiene-reports-class-only`.

@history_item n=65 marker=2026-11-14-hyprland-seam
**2026-11-14-hyprland-seam** — a second test-only recorder `viola-hypr-tap`; `~ decision/fixtures-recorded-by-live-verify`, `~ decision/recorder-scrub`, `~ decision/dirty-payload-refuses-recording`, `~ decision/fake-agent-on-test-path`, `~ decision/fake-agent-argv-only` (except), `~ decision/ci-never-real-claude`. Precedent for adding a test-only live tool and the rows it must amend.

@history_item n=66 marker=2026-11-14-hyprland-exact-frame-bound-pinned
**2026-11-14-hyprland-exact-frame-bound-pinned** — `~ decision/fakes-for-external-services` except: a crate's own unit tests may plant a shell stub because cargo-mutants runs only the mutated package's tests.

@history_item n=67 marker=2026-11-14-status-line-interception-removed
**2026-11-14-status-line-interception-removed** — `+ contract/viola-run-start/settings`: a start hands its child no `--settings`, writes no `settings.json`, leaves the user's settings byte- and mtime-equal, makes no default home under `--home`; `~ decision/no-production-data-in-tests` (workspace.repo except gone). The rig's no-write-to-`~/.claude.json` check rests on this.

@history_item n=68 marker=2026-11-15-live-round-on-the-trimmed-product
**2026-11-15-live-round-on-the-trimmed-product** — `~ contract/event-log/start-wheel-record` (folder-trust keys passing through the wrapper log `wheel{human, human-input}` before session-start) and `+ decision/start-lines-read-from-start-offset`: the trust-dialog reading and the reader trap a rig on a reused instance name meets.

@history_item n=69 marker=2026-11-15-mutation-and-coverage-baselines-green
**2026-11-15-mutation-and-coverage-baselines-green** — `~ decision/root-wait-bound` and `~ decision/within-7s-wait-bound` (WITHIN 12 s, mutants kill 15 s), `~ decision/mutants-kill-below-cargo-mutants-timeout`, and `+ decision/record-home-names-no-home` — the tests' half of the record-home rule whose live-rig half this chunk carries.

@history_item n=70 marker=2026-11-15-api-error-turn-end
**2026-11-15-api-error-turn-end** — `~ contract/event-data/turn-ended` (cause), `~ contract/viola-send/turn-running`, `~ decision/hook-payloads-recorded-by-verify` (StopFailure with no recorded payload), `+ decision/send-cases-copy-recorded-fixtures` (synthetic payloads alone draw no input box, so a send reads input-not-ready).

@history_item n=71 marker=2026-11-16-wait-wake-condition
**2026-11-16-wait-wake-condition** — `+ decision/one-verify-stamp-per-build`, `~ decision/fresh-home-per-test` (stamped home a copy, never a link), `~ decision/verify-driven-tests-20s-kill`, `~ contract/viola-wait/non-waking-records` (a harness-origin prompt-submitted never wakes wait). The directive's stamped-home-from-the-shared-stamp rule and the harness-prompt class v2-36 touches.

@history_item n=72 marker=2026-11-16-send-confirmation
**2026-11-16-send-confirmation** — `− decision/send-confirmation-exact-match → decision/send-confirmation-trailing-trim-match` (trailing ASCII space, TAB, CR, LF; the trim set an assumption, not a live reading), `~ contract/viola-send/records`, `~ contract/fake-agent-input/bracketed-paste` (`--trim-prompt-end`). The tolerance the rig's whitespace-trim reading is to confirm or widen.

@history_item n=73 marker=2026-11-16T06-05-33-wrap
**2026-11-16T06-05-33-wrap** (send trailing-only tolerance ratified — 4. Unit Test Strategy and 6. E2E Test Strategy) — `decision/send-confirmation-trailing-trim-match` made standing; a leading trim («either end») waits for the live reading carried on «Standing live rig with a screen model».

@history_item n=74 marker=2026-11-16T06-05-33-wrap
**2026-11-16T06-05-33-wrap** (StopFailure no-ledger-row ratified) — `~ decision/hook-payloads-recorded-by-verify` except made standing; StopFailure's live reading owned by «The 2.1.325 recorded fixture set».

@history_item n=75 marker=2026-11-16-local-command-delivery
**2026-11-16-local-command-delivery** — `+ decision/stamping-verify-passes-transcript`, `+ contract/fake-agent-cli/transcript`, `~ contract/fake-agent-hooks/stdin-payload`; every stamping helper passes `--transcript`, which a rig-side stamped home must keep.

@history_item n=76 marker=2026-11-16-scheduled-wake-count
**2026-11-16-scheduled-wake-count** — `~ contract/event-data/turn-ended` (scheduled_wakes), `+ contract/dialog-data/scheduled-wakes`, `+ contract/turn-ended/scheduled-wakes`; only the measured `0` and synthetic counts claimed, no live probe — the rig carries «a real wake reads 1» and the `source` field reading.

@history_item n=77 marker=2026-11-17T05-21-35Z-wrap
**2026-11-17T05-21-35Z-wrap** (the delegate's confirm ratified — scheduled-wake-count's contract/dialog-data/scheduled-wakes, contract/event-data/turn-ended, contract/turn-ended/scheduled-wakes) — the scheduled-wake rows stand on the founder's word; the rows the carried live reading extends.
