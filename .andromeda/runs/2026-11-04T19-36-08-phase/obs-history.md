@history_item n=1 marker=2026-09-24-three-os-ci-headless-harness-skeleton
**2026-09-24-three-os-ci-headless-harness-skeleton** — The fake agent (`src/bin/viola-fake-agent.rs`, a root-package bin) is exempted from the print ban by a crate-level `#![allow(clippy::print_stdout, clippy::print_stderr)]`, and only `viola-e2e` carries its own `[lints.clippy]`. Matters because the lint job must read green on ubuntu-latest and macos-latest while the full fake-agent suite runs there, with no new `#[allow]`.

@history_item n=2 marker=2026-09-24-diagnostics-plane
**2026-09-24-diagnostics-plane** — Schema default-deny is enforced by a top-level `unevaluatedProperties: false` (not per-event `additionalProperties`), and `diag-detail.v1.json` is self-contained. `obs_event!` attaches `event`, `process` and `instance`, while `corr` is caller-supplied. This is the mechanism by which an OS-only field fails G4 on a Unix leg, which is why a field one OS cannot fill must be absent.

@history_item n=3 marker=2026-09-24-log-redaction-and-never-log-floor
**2026-09-24-log-redaction-and-never-log-floor** — anyhow is scoped to the root-bin dispatch edge and its catch-site reporter `viola::obs::report_internal_error`. The chain goes only to `detail-<process>.ndjson`. The Unix `run` wrapper must keep this error-capture boundary unchanged.

@history_item n=4 marker=2026-09-24-observability-gates
**2026-09-24-observability-gates** — The raw-tracing ban is enforced by the `clippy.toml` `disallowed-macros` path ban plus a fail-closed raw-`event!` grep in `scripts/lint-probes.sh`. That grep runs on the Linux lint leg, and its failure is a build failure (D-33). It bears on the lint job reading green on ubuntu-latest/macos-latest over any new Unix-only code.

@history_item n=5 marker=2026-09-25-pty-wrapper-on-windows
**2026-09-25-pty-wrapper-on-windows** — `process-start{subject:"claude-child"}` gains `env_kept`. D-34 supersedes D-14: every stripped and kept `CLAUDE*` name is logged, and no value is ever read. `batch-script-child` is decided by program resolution before `pty.spawn`. `PtyError`'s `Display` is hand-written, and the seam spans (`pty.spawn` etc.) are required. The openpty spawn must log the same names-only strip fields, and seam spans must exist on Unix as on ConPTY.

@history_item n=6 marker=2026-09-26-ci-chunk-base-and-union-verdict
**2026-09-26-ci-chunk-base-and-union-verdict** — The secret scan covers `target/agent-run/*` except exactly `target/agent-run/chunk.diff`, which `harness-<os>` also never uploads. This is the scan scope that must read clean on both Unix OSes before the per-OS uploads.

@history_item n=7 marker=2026-09-27-wrapper-channel
**2026-09-27-wrapper-channel** — `diag-line.v1.json` makes `corr` required per event (`channel-*`, `dialog-*`, `release-from-driver`, `send-*` wrapper side, some `hook-decision`) and null-by-absence elsewhere (§3 Null encoding). A malformed peer `conn` is logged as `srv_conn`. This is the null encoding P3 must reconcile with the scope's "never a missing key", and these are the channel fields the Unix-socket lines must carry identically.

@history_item n=8 marker=2026-09-27-hooks-to-normalised-events
**2026-09-27-hooks-to-normalised-events** — D-28: the concurrent-append check landed for the hook files: 8 processes, 3 OSes, no line over 4 KiB, with the >4 KiB half deferred to the hook perf gate. It is the precedent for "append atomicity with concurrent hook writers: POSIX `O_APPEND`" yielding identical lines on Unix.

@history_item n=9 marker=2026-09-28-hook-perf-gate
**2026-09-28-hook-perf-gate** (the `perf` job's uploads) — Perf moved from the `test` job into its own per-OS `perf` job with its own G2, G4 and scan. `perf-<os>`/`diag-perf-<os>` are uploaded only on a successful scan, and `secret-scan-perf-<os>` on a failed one. These are the per-OS artifact names and step order the chunk must see green on ubuntu-latest and macos-latest.

@history_item n=10 marker=2026-09-28-hook-perf-gate
**2026-09-28-hook-perf-gate** (G2 seam exemption, D-28 over-4 KiB half) — G2 runs as `scripts/g2-zero-panics.sh` (`--probe` first, `test` and `perf` jobs). It exempts only a `panic_location` of exactly `src/cmd/hook/seam.rs:<digits>`, the founder's live ratification of 2026-09-28. The over-4 KiB concurrent append landed on 3 OSes. The exemption must not widen to make a Unix case pass, and the Unix seam panic must render that exact workspace-relative form.

@history_item n=11 marker=2026-09-28-cli-output-tokens
**2026-09-28-cli-output-tokens** — Every `run` pre-spawn start refusal writes two fixed stderr lines through `src/human.rs` `refuse` → `write_refusal` (one `write_all`, no print macro, no `#[allow]`). This is the precedent for any Unix pre-child refusal pair the parity suite reads.

@history_item n=12 marker=2026-09-28-capability-ledger-and-viola-verify
**2026-09-28-capability-ledger-and-viola-verify** (raw backtrace frames; cli internal-error line) — The detail-line `backtrace` is raw frames from `src/panic_frames.rs`: `libc::backtrace` + `dladdr` on Unix and `RtlCaptureStackBackTrace` on Windows, up to 62 strings, never symbolised. Symbolising cost 351 ms and broke the 1.0 s spine bound. The Unix detail line must validate against `diag-detail.v1.json` on both Unix runners, and must not reintroduce symbolisation.

@history_item n=13 marker=2026-09-28-capability-ledger-and-viola-verify
**2026-09-28-capability-ledger-and-viola-verify** (verify's outcome, the uninstrumented capture arm) — `hook --capture` is uninstrumented by design (§10 bounded exemption 5), and verify's outcome is on stdout with a fixed stderr pair on refusal. Both hold under the Unix fake-agent suite and G2's exemptions.

@history_item n=14 marker=2026-09-29-verify-stamped-test-homes-and-harness
**2026-09-29-verify-stamped-test-homes-and-harness** — `verify-probe` joins the closed `subject` enum (D-35). In CI verify runs only against the fake agent, and the harness `boot` readiness checks the start records. It sets the closed subject set and the fake-agent-only verify that the Unix suite runs.

@history_item n=15 marker=2026-09-29-sideloaded-conpty
**2026-09-29-sideloaded-conpty** — D-36 sets `pty_backend` to `conpty-sideload|conpty|openpty` from `viola_pty::pty_backend()`. `run.conpty_sideload` is Windows-only, and `sideload_fallback` is present only on a Windows degrade and closed in `diag-line.v1.json`. The openpty wrapper must report `openpty`, and `sideload_fallback` must be absent on Unix, never an OS-only word.

@history_item n=16 marker=2026-09-29-t15-07-57-wrap
**2026-09-29-t15-07-57-wrap** — The Decisions Log moved to the archive, and its in-force items were lifted into the body. They include the OFF third-party targets (`portable_pty`) with failures captured at the seam (spawn `Err` → `internal-error`, handle-wait `exit_source`, D-11), `pty_backend` as an open string and `sideload_fallback` as closed (D-36), and the R8 strip's 11-name `IDENTITY_FLOOR` (D-34). These are the seam-capture rules the openpty backend must keep.

@history_item n=17 marker=2026-10-02-epoch-2b-cleanup
**2026-10-02-epoch-2b-cleanup** — CI's `AGENT_RUN_KEEP_HOMES=1` keeps every home, so neither harness `cleanup` nor a test home's drop (`remove_owned`) deletes it before G2, G4, the scan and uploads. Without it a green Unix run leaves `target/e2e-home/` empty and G2 fails its non-empty check.

@history_item n=18 marker=2026-10-03-mutation-scoring-completion
**2026-10-03-mutation-scoring-completion** (§1 kept current) — §1 stays a labelled verbatim copy, kept current. Until a wrap brings a difference current, §3, §6 and §12 win where they differ. This is the section-currency rule the cascade bindings rely on: the re-asserted schema is `diag-line.v1.json` with the §6 catalog, and §10's "no transition logged" beats §1's "on change only".

@history_item n=19 marker=2026-10-04-windows-boundary-mutation-workflow
**2026-10-04-windows-boundary-mutation-workflow** — The dispatch-only `windows-mutants.yml` is the Windows mutation leg, scoring the `cfg(windows)` obs code in `src/panic_frames.rs`. It bears on the extract's `#[cfg(unix)]`-twin pattern and on which leg judges which OS's code.

@history_item n=20 marker=2026-10-04-readiness-gate-and-timing-constants
**2026-10-04-readiness-gate-and-timing-constants** — The vt100 feed thread is a contained worker: `catch_unwind` per feed and resize, one `parse-rejected{vt100-feed}`, `run` continues. The "no worker panic leaves `main` without `process-exit`" ban covers the PTY pump and handle-wait threads. These are the catch sites that must hold under openpty for G2 to read zero on Unix.

@history_item n=21 marker=2026-10-04-confirmed-send-with-cl-1-records
**2026-10-04-confirmed-send-with-cl-1-records** (the chaos home outside the scans, F4) — `tests/chaos_feed_panic.rs` witnesses one `parse-rejected{vt100-feed, panicked}` and one `event:"panic"`. Its `TestHome::outside_scan()` home lives outside `target/e2e-home/`, G2, G4 and the scan (founder ruling, live, 2026-10-04), not a second G2 exemption. This is the one home carve-out that must stay unchanged when the suite runs on both Unix OSes.

@history_item n=22 marker=2026-10-04-dialog-answers-by-dialog-id
**2026-10-04-dialog-answers-by-dialog-id** — Perf moved to five hyperfine rows (`pre-tool-use` added on an unstamped session), each required by name by `gate --require perf`. `parse-rejected{ledger-stamps}` gains `strict-modes-failed`, and CI verify runs at 2.1.287. These are the five rows the per-OS perf job must hold on ubuntu-latest and macos-latest.

@history_item n=23 marker=2026-10-07-test-homes-off-the-contended-volume
**2026-10-07-test-homes-off-the-contended-volume** — G2's `find` start points read `target/e2e-home/`, with a trailing slash so a linked scope is descended. On the Linux dev host `target/e2e-home` is a link to an owner-only tmpfs directory. The trap: without the slash, `find` reads an empty scope. No CI runner has the link, so a local Linux G2 and the CI legs walk differently shaped roots.

@history_item n=24 marker=2026-10-07-live-rows-and-paste-shapes-on-the-dev-host
**2026-10-07-live-rows-and-paste-shapes-on-the-dev-host** — The R8 strip always removes the 11 `IDENTITY_FLOOR` names measured on Windows. A differing Linux dev-host reading on 2.1.287 stands beside it and moves nothing (home: architecture §Occupied Resources). This is a known per-OS env difference the Unix child spawn's names-only `env_stripped_*` fields must tolerate.

@history_item n=25 marker=2026-10-08-first-live-test-and-self-drive
**2026-10-08-first-live-test-and-self-drive** — G2's `--probe` other-file case names its planted literal by its home in `scripts/g2-zero-panics.sh`, never as `path:N`. G2 `--probe` must run first on both Unix legs, and a citation sweep must not re-point the planted literal.

@history_item n=26 marker=2026-10-09-inner-cr-and-crlf-in-a-sent-text
**2026-10-09-inner-cr-and-crlf-in-a-sent-text** — `text_bytes` is the typed text's length: every CR LF and other CR becomes one LF, and trailing CR/LF are dropped. The openpty paste must log the same count as ConPTY.

@history_item n=27 marker=2026-10-10-windows-mutation-grade
**2026-10-10-windows-mutation-grade** — On the Windows leg the harness leaves `#[cfg(unix)]` twins in `src/panic_frames.rs` out as host-excluded (23 tested, 14 caught, 9 left out). A missed mutant the host never compiles is not counted. This is the precedent for code only one OS builds: Unix-only obs code must be judged on a Unix leg.

@history_item n=28 marker=2026-10-10-self-healing-state
**2026-10-10-self-healing-state** — `state-recovered` is written by `read_snapshot_or_replay` (`snapshot-replayed|snapshot-unsupported-v`) and by the healing appender (`torn-line-healed`, `file`, `offset`), one line per call or heal, at WARN with no `corr`. These detail words must read the same on every OS.

@history_item n=29 marker=2026-10-10-viola-revive
**2026-10-10-viola-revive** — revive's start arm logs as role `run` into `run-<name>.ndjson`, `human::refuse` gains revive as a caller, and the `process-exit` exit-1 details grow. The kill-and-revive case the Unix suite runs (via `tests/support/home.rs`) drives this arm.

@history_item n=30 marker=2026-10-10-statusline-pass-through
**2026-10-10-statusline-pass-through** — The `statusline-shell` pair is written only when a command is recorded and a shell is built, never on Windows. Two start steps carry no span by design, among them the Unix-only settings override write. These are the Unix-only behaviours a test must name its OS reason for, and Windows writes no OS-only word for them.

@history_item n=31 marker=2026-10-10-budget-governor
**2026-10-10-budget-governor** (a refused instance check writes no line) — revive's instance check moved before log init, and a refusal is the stderr pair with no line. The hook statusline arm's refused check opens no role file, and §10 gains exemption 7. This is the start of the "refused check writes no line" rule, which the Unix suite must read by stderr pair and not as a missing G2/G4 line.

@history_item n=32 marker=2026-10-10-budget-governor
**2026-10-10-budget-governor** (run.budget_eval in the start chain) — `run.budget_eval` was inserted between `channel.bind` and `state.snapshot_write` in the must-trace start chain. This is the chain the Unix wrapper's start must reproduce up to `pty.spawn`.

@history_item n=33 marker=2026-10-13-interrupted-verify-cleanup
**2026-10-13-interrupted-verify-cleanup** — `process-exit` exit 1 gains `interrupted` for a verify stopped by Unix `SIGINT`/`SIGTERM` (Windows Ctrl-C/Ctrl-Break). The handler logs nothing, and the probe dir is left by SIGKILL or a crash. This is a per-OS signal path whose closed word must match across OSes.

@history_item n=34 marker=2026-10-13-owed-ledger-rows
**2026-10-13-owed-ledger-rows** — A second capture arm, `hook statusline --capture` (Unix only, Run B's probe), is uninstrumented by design (exemption 5). verify's `MM` relied count differs per OS (then 21 Unix / 18 Windows). This is the precedent for a per-OS relied-row count and a Unix-only uninstrumented arm.

@history_item n=35 marker=2026-10-14-resume-and-compact-ledger-rows
**2026-10-14-resume-and-compact-ledger-rows** — A PTY child killed at a Ctrl-C/TERM stop carries `child_exit_status`, because portable-pty reports a code for a signalled child, measured at that chunk. The print-mode/`--version` probe case was not measured. This bears on openpty kill/wait semantics on Unix.

@history_item n=36 marker=2026-10-16-remove-the-browser-leg
**2026-10-16-remove-the-browser-leg** (the `ui` process and four events) — `process` is closed to `run|hook|mcp|cli`, the role files are `run-<name>`, `hook-<name>`, `mcp`, `cli-<name>`, and `portable_pty` is among the OFF targets. No process logs a liveness transition: the heartbeat flips at >5 s, read via `list --json`. These are the four closed roles and the heartbeat rule the v1-40 Unix witness relies on.

@history_item n=37 marker=2026-10-16-remove-the-browser-leg
**2026-10-16-remove-the-browser-leg** (§5–§11) — §10's liveness signal says no process logs a transition, and the staleness flip is read as `list --json` `liveness`. The cross-input parallel is nextest `retries = 0`. This is the no-retry rule and liveness reading the Unix legs must keep.

@history_item n=38 marker=2026-10-17-end-to-end-path-binary-and-harness-legs
**2026-10-17-end-to-end-path-binary-and-harness-legs** — `agent-run status` reads `state: degraded` when a booted instance is `stale` or absent, derived from `viola list`. This is the harness reading the obs↔tests binding requires to give the same result on every OS.

@history_item n=39 marker=2026-10-17-cli-machine-contract
**2026-10-17-cli-machine-contract** — `viola revive --list` logs as `cli` after its strict-modes check, exemption 6 is retired, and `state-recovered` gains `scanned` (a `ViolaName` last component, never a path). These are closed shapes the one-schema re-assertion reads on every OS.

@history_item n=40 marker=2026-10-17-mcp-server-for-drivers
**2026-10-17-mcp-server-for-drivers** (the mcp sink) — `viola mcp` always writes `<home>/diagnostics/mcp.ndjson` (D-09), checked with `diagnostics/` before the sink opens. stderr JSON is used only when the file cannot be opened, never stdout. This is the identical-sink requirement for `mcp` on both Unix OSes.

@history_item n=41 marker=2026-10-18-compact-post-condition-ledger-row
**2026-10-18-compact-post-condition-ledger-row** — verify's `MM` is now 24 on Unix and 21 on Windows, the latest per-OS relied-row count the cascade cites. A Unix verify reading must expect 24.

@history_item n=42 marker=2026-10-26-server-verification-before-any-frame
**2026-10-26-server-verification-before-any-frame** (the server check's per-OS peer-pid read) — Peer pid is read via Linux `SO_PEERCRED` and Windows `GetNamedPipeServerProcessId`, and not at all on macOS (`SERVER_PID_REPORTED` false, verdict `Matches` with no comparison). This is the origin of the macOS carve-out the scope keeps, and of the multi-OS trigger's OS-difference list.

@history_item n=43 marker=2026-10-26-server-verification-before-any-frame
**2026-10-26-server-verification-before-any-frame** (server-identity negatives) — `tests/security_negatives_server_identity.rs` is `#![cfg(not(target_os = "macos"))]` and scans outputs with `digit_runs` on Linux and Windows. This is an existing per-OS gate whose named reason (no peer pid on macOS) must stand under the scope's cfg-reason rule.

@history_item n=44 marker=2026-10-26-server-verification-before-any-frame
**2026-10-26-server-verification-before-any-frame** (the pre-frame check's place in the span tree) — The captured-span cases (`request_refuses_an_unverified_server_inside_its_dispatch_span`, `hook_refuses_an_unverified_server_inside_hook_handle`) go through the test-only `cmd::client::refused_inside`. That helper is macOS-gated, and its OS gate is a cfg reason the parity suite must keep named.

@history_item n=45 marker=2026-10-26-server-verification-before-any-frame
**2026-10-26-server-verification-before-any-frame** (the server check inside the timed hook path) — The server check sits in the timed hook path under the same `max <` spine-deadline gate (2.2–2.4 ms then). This is the perf precedent for the Unix perf job.

@history_item n=46 marker=2026-10-26-explicit-session-links
**2026-10-26-explicit-session-links** (`server-verify-failed`: eight CLI frame verbs) — `server-verify-failed` is written by eight frame verbs (`link`/`unlink` into `cli-<driven>.ndjson`). The per-verb captured-span cases run on Linux and Windows and are compiled out on macOS. These are the eight writers and the macOS compile-out the Unix suite meets.

@history_item n=47 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (the `diagnostics/` opens behind `check_diagnostics`) — `open_role_file`/`write_detail` run `strict::check_diagnostics`. On Unix that means no group or other bit on the folder and the read rule on the file. A refusal is a failed open (`std::io::sink`, detail line dropped). A Unix test home whose `diagnostics/` fails the read rule silently drops lines, and G2, G4 and the scan would read vacuous green.

@history_item n=48 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (home-tree check of `run`/revive's start) — `checked_exe` + `check_home_tree` run before the wrapper log. A refusal exits 1 with the home or instance pair, no log and no `detail`. These are Unix-reached start refusals the suite must read by stderr pair.

@history_item n=49 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (verify's home-tree check, catch-site reach) — The per-OS split is measured. On Unix a home under a regular file is a strict-modes refusal with no line: verify exit 1 with the home pair, frame verbs exit 21. On Windows such a path stats `NotFound` and the catch-site `process-exit{detail:"internal-error"}` stands. This is a known OS divergence a parity test must carry as a named one-OS reason.

@history_item n=50 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (pre-init window, exemption 8, init step 3) — Every home check runs before its first open, because the open narrows an existing home to 0700. A refusal writes no line (exemption 8). This is the init-order trap: a check after the open passes a widened home, which matters for Unix mode-based tests.

@history_item n=51 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (the hook's instance check before obs init) — Every hook arm but the two `--capture` arms checks the instance before obs init, and a refused or absent instance is a silent exit 0. `FAKE_AGENT_HOOK_PANIC` runs after obs init. This is the hook contract and seam placement the Unix fake-agent suite reads.

@history_item n=52 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (six frame verbs through `refuse_untrusted`) — A refused check exits 21 with no line. `tests/security_negatives_strict_modes.rs` reads every output with `digit_runs` on Linux, macOS and Windows. This is an existing all-OS negative the parity re-read keeps.

@history_item n=53 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (the hook's instance check inside the timed path) — The instance check is in the timed hook path, with five rows at max 2.5/2.4/2.4/2.4/3.3 ms. The gate passes vacuously if `VIOLA_DIR` names no instance or the instance fails the check. On Unix the timed instance must pass the strict-modes read rule, or the perf job reads green vacuously.

@history_item n=54 marker=2026-10-27-pinned-copy-race-and-reproducible-builds
**2026-10-27-pinned-copy-race-and-reproducible-builds** — The absent pinned copy is placed no-clobber (`place_private_new`) because a concurrent first start hit `ETXTBSY` (4 of 24 two-instance boots before, 0 after). A lost placement re-hashes the copy that stands, with no new line or code. This is a Unix-specific exec race already fixed, and a precedent the parity suite's concurrent boots rest on.

@history_item n=55 marker=2026-10-27-bounded-inputs-at-every-boundary
**2026-10-27-bounded-inputs-at-every-boundary** (the feed-panic test's passthrough half) — `chaos_feed_panic_refuses_send_and_keeps_passthrough` reads the child's bytes `e4 b8 ad` on Unix. The Windows byte reading is owed to "Windows-only live measurements" (ConPTY re-renders wide chars). This is an existing one-OS split with a named reason, and Windows must not move to match Unix.

@history_item n=56 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** (v1-47's secret-scan reading) — The scan reads after each job's end-to-end run (`AGENT_RUN_KEEP_HOMES=1`, `if: always()`, the `viola-chaos-*` home out), not after every test. This is the reading the Unix test and perf jobs' scan must satisfy.

@history_item n=57 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** (the mcp role writes no self process-exit) — `internal_error_exit_line` writes only for `Run` and `Cli`, so `mcp` has no self `process-exit` line on any OS. A Unix MCP case must not expect one.

@history_item n=58 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** (exit 21's document names its cause; enum count) — Exit 21's `--json` document carries one closed `UnreachableCause` word. `diag-line.v1.json` closed the `process-exit` `detail` at 19 values, counted from the enum's members. These are the closed words and the count rule the one-schema re-assertion extends.

@history_item n=59 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** (Founder Direction 4's codes and named carve-outs) — Distinct exit-21/exit-1 codes, with named carve-outs from the founder's own word (`inputs#I2` Q4). This is the origin of the carve-out frame that the 2026-11-01 entries narrowed to macOS alone.

@history_item n=60 marker=2026-11-01-t02-46-44-wrap
**2026-11-01-t02-46-44-wrap** — §3's Log format copy is verbatim with test-plan §3 again (`obs-plan D-07`, `corr` rule placement). The `v1-47` scan reading stands. This is the bound obs↔tests Log format the one-schema re-assertion reads.

@history_item n=61 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** (exit 21's closed causes, five to six) — `UnreachableCause::SocketDir` adds `socket-dir-refused` as the sixth word. `refused_with` prints the exit-1 document for it before any snapshot, child or `Server::bind`. This is a Unix-reached closed word the parity suite must read identically on both Unix OSes.

@history_item n=62 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** (MCP's frame tools: socket-directory and euid refusals) — `frame` runs `check_endpoint_dir` after `live_endpoint`, then `peer_admitted(server_euid)` and `server_verified`. Each refusal returns `instance-unreachable` (`socket-dir-refused`/`server-verify-failed`) with no frame and no line, on macOS an euid mismatch alone.

@history_item n=63 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** (`socket-dir-refused` in the log catalogue) — The `process-exit` `detail` enum went from 19 to 20 values and the `hook-decision` fail-open codes from 7 to 8, both counted from the schema's enum members. These are the latest closed counts the one-schema re-assertion rests on, and a new OS word would move them.

@history_item n=64 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** (`server-verify-failed` reaches macOS) — A server of another euid (Unix) reads `server-verify-failed` on Linux, Windows and macOS, on macOS an euid mismatch alone. The multi-OS trigger adds the peer euid (Unix `peer_creds`, Windows none), and the macOS pid gap becomes the named carve-out owned by no route entry. This is the carve-out the scope keeps: no test reads macOS as a pid comparison.

@history_item n=65 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** (a refused socket directory is no strict-modes failure) — `check_socket_dir` takes the strict-modes read verdict but writes under `socket-dir-refused`. That is one line at the `run`/revive start (exit 1), at the eight frame verbs (exit 21, `during:"connect"`) and at `viola hook`, with none on MCP. This is the line shape the Unix suite must read, never as a no-line strict-modes refusal.

@history_item n=66 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** (the frame verbs' socket-directory refusal) — `cmd::client::connect` checks the recorded endpoint's parent and changes no mode. A refusal is one exit-21 `process-exit` at WARN inside the dispatch span, with no `channel-request`, counted by the Counter row's `detail` grouping.

@history_item n=67 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** (`run`'s start refusals and the `human::refuse` census) — The census of `human::refuse(` call sites moved from 18 to 19: `run` 8, `verify` 6, `list` 2, `revive` 1, `plugin install` 1, `client` 1. The start refusals gain a `socket_dir_ready` refusal (Unix, exit 1). This is the census a new Unix refusal site would extend, re-derived by `grep -rcE 'human::refuse\(' src`.

@history_item n=68 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** (the OS call sites the new checks add) — The OS-APIs entity's call sites are the peer-euid read (Unix), the peer-pid and start-time reads, and `check_socket_dir` (one `symlink_metadata`, with no read off Unix). The `accept_loop` euid read writes no line. These are the boundary-only call sites the openpty/Unix paths instrument.

@history_item n=69 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** (Founder Direction 4: socket-directory carve-out retired, macOS's named and unowned) — A refused socket directory has its own code, whether from a failed create, a lost race or a refused check. macOS comparing no peer pid is the one remaining named carve-out, owned by no route entry. The scope's macOS boundary rests on this.

@history_item n=70 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** (the hook's timed path holds both new checks) — The socket-directory check and the peer-euid decision sit in the timed hook path. `run --perf` read means of 2.0–2.3 ms and a highest max of 2.8 ms, under `SPINE_DEADLINE`, and the recorded band stays. This is the perf baseline for the Unix perf job.

@history_item n=71 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** (the wrapper's drop of a refused peer writes no line) — `accept_loop` drops a refused-euid peer on the accept thread before any frame, with no span and no line, and the client reads EOF. It was tested only over an injected decision (a P5 lean, PROVISIONAL). The obs criterion requires this no-line drop on Unix, and no seam may force it (scope `fct-kte5hhfb`).

@history_item n=72 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** (the start's socket-directory step) — `socket_dir_ready` is a fourth span-less start step. It checks, creates if absent, checks again, and writes one exit-1 `socket-dir-refused` line before any snapshot, child or bind. A `Server::bind` failure keeps its own outcome. This bears on the Unix wrapper start chain to `pty.spawn`.

@history_item n=73 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** (a refused socket directory at `viola hook`) — The hook `connect` chain is `read_snapshot_or_replay`, `check_endpoint_dir`, `connect_by`, `peer_admitted`, `verify_server`. A refused directory gives one `hook-decision{socket-dir-refused}` at WARN with exit 0 and empty streams. This is the hook contract under the Unix suite.

@history_item n=74 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** (PII reading and clean-body witnesses) — `tests/security_negatives_socket_dir.rs` is `cfg(unix)` with no Windows case and ran green on ubuntu-latest and macos-latest. The server-identity negatives stay compiled out on macOS. These are existing per-OS test gates whose named reasons the scope's cfg rule must keep.

@history_item n=75 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** (the checks' order before any frame) — The CLI order before any frame is `check_endpoint_dir`, connect, the euid decision, then `verify_server`. Pid and start-time comparison runs on Linux and Windows only. Each scenario carries the macOS euid-only parenthesis that the Unix suite must read.

@history_item n=76 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** (the client euid check at three seams) — `peer_admitted` refuses another euid or an unreadable one where `PEER_EUID_REPORTED` (`cfg!(unix)`). Windows reads no euid. Each seam's wiring was read only through forced refusals, as one account can make no foreign peer. This is the trap that a same-account Unix CI run cannot produce a foreign-euid peer without a seam, which needs a Decisions Log entry.

@history_item n=77 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** (the socket-directory refusal's captured-span cases) — The recorder `Seen` moved into `refused_seen` in `src/cmd/client.rs`, out of the macOS-gated `refused_inside`, so that the two new cases run on macOS. This is the precedent for un-gating a test helper so a case runs on every Unix OS.

@history_item n=78 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** (OTel SDK init: the `cli` chain after the log opens) — After a passed instance check the `cli` chain runs the liveness pre-check, the socket-directory check (exit 21 `socket-dir-refused`), the euid decision, then the server check, for all 8 callers. This is the init order the Unix frame verbs run.
