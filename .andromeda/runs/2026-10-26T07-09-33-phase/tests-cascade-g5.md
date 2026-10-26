# tests · cascade read · group 5 of 5

## Carried
- (.andromeda/test-plan-detail/test-data-fixtures.md:10) Every case of the chunk that needs a `snapshot.json` (the pid / start-time mismatch per client family, and the snapshot-hold case of an unreadable or newer-`v` snapshot read as absent) must take it from a real wrapper's own write, altered only by changing the bytes the wrapper wrote, never hand-written; and should verification take the strict-modes half with it, a Windows home made by `seed_conpty` is refused like `hook statusline`'s instance check refuses it, so every verified-client case that must pass on Windows starts from `stamped_home`.
  > Produced only by running real `viola run` / `hook` / CLI verbs against `viola-fake-agent` in a fresh home,
  > never hand-written. Two exceptions: the chaos tests, which corrupt files the product already wrote; and on
  > Windows x64 the root tests' `seed_conpty`, which hard-links (or copies) the ConPTY companions into
  > `<home>/bin/<key>/conpty/`, byte-identical to what viola would write (§5 carve-out; `conpty_sideload` never
  > seeds, so it covers the product's write). A home that seed made is not one viola created, and a first boot and
  > stop does not change that: `hook statusline`'s instance check refuses it (as measured at chunk
  > 2026-10-10-statusline-pass-through, CI run 38080061631, three cases; the refused home's DACL was not read, and
  > that reading is owed to the working-route entry "Home and code-bearing file integrity"). A case that needs a home
  > viola created on every OS starts from `stamped_home`, as the every-OS `hook_statusline` cases that run the arm's
  > whole course do.
  (.andromeda/test-plan.md:631)
  > - **On-disk product state:** produced only by real `viola` runs against the fake agent in a fresh home, never
  > hand-written — except the chaos tests' corruption of files the product already wrote and the Windows x64
  > `seed_conpty` companions; a case that needs a home viola created starts from `stamped_home`.
  (.andromeda/test-plan.md:905)
  > NEVER hand-write `ledger/stamps.json`, `snapshot.json` or `budget.json`. Stamps come from `viola verify`,
  > snapshots from the wrapper and budget from `viola hook statusline`. The ban binds every booted-wrapper test. A
  > reader's own unit cases may plant a malformed state file only by altering the writer's own bytes: write through
  > the writer, then pad, replace or re-version (the operator's answer at the chunk 2026-10-10-budget-governor wrap,
  > where `budget_read_*` landed in that form).
- (.andromeda/test-plan-detail/test-data-fixtures.md:44) A hook fired by the fake agent at a wrapper whose identity fails verification must read back, through its `hook` receipt, exit 0 with an empty stdout and stderr: the receipt's fields are the cross-process oracle of the fail-open outcome the chunk adds.
  > `hook` {`event`, `command_absolute`, `ran`, plus `exit_code`, `stderr_len`, `stdout_hex` and `stdin_hex` (the
  > exact bytes offered on the hook's stdin) when it ran}
- (.andromeda/test-plan-detail/mocking-stubbing-discipline.md:8) The edited `viola-state` liveness decision must keep its heartbeat age clock-driven through the injected `viola_core::Clock` (or an age handed in), so the pid or start-time mismatch reads `gone` at any heartbeat age with no real clock, and no test-side clock is used to steer the spawned wrapper's start time or the MCP tools' new pre-frame check.
  > Time: sync crates use mock_instant 0.6.1 (global mode) or the injected `viola_core::Clock` for heartbeat 1 s / 5 s
  > and the send confirmation window. `budget_override_until` is no clock-seam case: the budget gate's decision takes
  > `chrono::DateTime<Utc>` as an argument, and its expiry is a pure-function unit case (§4, chunk
  > 2026-10-10-budget-governor). Async crates (`viola-mcp`) use `#[tokio::test(start_paused = true)]` from tokio
  > 1.53.3 `test-util` for any timer the MCP adapter itself owns. The `wait` `timeout_ms` deadline runs in the wrapper
  > process, and only the injected `viola_core::Clock` in the §4 root-bin case drives it. A paused tokio clock in a
  > test never reaches a spawned wrapper (§11 Universal).
  (.andromeda/test-plan-detail/mocking-stubbing-discipline.md:17)
  > Inject the `Clock` dependency through constructors and call `MockClock::advance(Duration::from_secs(6))`; or hand
  > a pure decision its instant as an argument, as the budget gate's `decide` takes `chrono::DateTime<Utc>`
  (.andromeda/test-plan.md:705)
  > - Time: sync crates use the injected `viola_core::Clock` (or mock_instant) for the heartbeat and the send
  > confirmation window, `viola-mcp` uses tokio's paused clock for a timer it owns, and the budget gate's override
  > expiry is a pure-function case; `chrono::Utc::now()` is read only through the clock seam, except at the budget
  > gate's three evaluation points, and a paused clock in a test never reaches a spawned wrapper.
  (.andromeda/test-plan.md:842)
  > | Heartbeat staleness flip (`viola-state` `classify`, injected age) | n/a | same process: 5.0 s → live, 5.1 s →
  > stale; pid or start-time mismatch → gone at any age (exact boundary test) |
  (.andromeda/test-plan.md:944)
  > NEVER use mock_instant `MockClock` (or any in-process clock) to control another process's time. Global mode
  > reaches only threads of the test process, never a spawned `viola run`.
- (.andromeda/test-plan-detail/mocking-stubbing-discipline.md:19) The impostor or squatter server each mismatch case connects a client to must be a real listener bound through the interprocess server API (on Windows beside the real `viola_channel::Server::bind` listener), at the endpoint the wrapper's own `snapshot.json` records, never an HTTP or in-process fake of the channel.
  > | Fake servers | interprocess 2.4.4 server API (squatter listener); `viola-fake-agent` as the fake CLI; windows-sys
  > 0.61.2 `CreateNamedPipeW` (Windows only: test-owned control pipes) | The squatter binds the computed endpoint
  > before `run`; in the listener-admission negatives the test-owned control pipes (accept-remote, reject-remote, the
  > default descriptor) stand beside the real `viola_channel::Server::bind` listener, and `ImpersonateAnonymousToken`
  > stands in for a foreign principal; no HTTP fake servers |
  (.andromeda/test-plan.md:724)
  > - **Fake servers:** the interprocess server API binds the squatter before `run`, and `viola-fake-agent` is the
  > fake CLI; no HTTP fake servers.
  (.andromeda/test-plan.md:889)
  > NEVER use interprocess `try_overwrite`, or reuse an endpoint name across tests. Take the endpoint only from
  > `snapshot.json`.
- (.andromeda/test-plan-detail/ci-integration.md:9) The server identity read added to `viola-state`'s liveness and to `viola-channel`'s sync client must keep the sync crates tokio-free, `viola-channel`'s Tokio client staying behind its `tokio` feature with its normal graph held by the tree form; and the chunk's `cfg(windows)` pipe server-pid code, which the Linux host's own clippy never builds, takes the check-only Windows-target clippy as a chunk gate entry.
  > `cargo check` with one `-p` per crate listed in `scripts/sync-crates.txt` (no `--features`, so no tokio; target
  > set `viola-core`, `viola-pty`, `viola-state`, `viola-agent-claude`, four since `viola-channel` left the list at
  > chunk 2026-10-17-mcp-server-for-drivers — its Tokio client sits behind its `tokio` feature and its normal graph is
  > held by the tree form `scripts/channel-tokio-free.sh`, Supply-chain row — each joining the list with its crate; an
  > empty list fails; job `lint`)
  (.andromeda/test-plan-detail/ci-integration.md:10)
  > Beside CI, a chunk plan may list a check-only Windows-target clippy as a chunk gate entry on the Linux dev host,
  > which does not build the `cfg(windows)` code CI lints (first listed by chunk 2026-10-13-epoch-4-cleanup;
  > `CARGO_TARGET_DIR=target/wincheck cargo clippy --workspace --all-targets --features fake-agent --target
  > x86_64-pc-windows-msvc -- -D warnings`, nothing linked, the target std installed as host preparation): neither a
  > CI step nor a `pre-push` stage.
  (.andromeda/test-plan.md:741)
  > - **Lint:** fmt, clippy `-D warnings`, the tokio-free `cargo check` of each listed sync crate, the cargo-modules
  > orphans gate with its probe, and the rmcp release-graph assertion, per OS; the `--acyclic` review and a dev-host
  > Windows-target clippy are not gates.
- (.andromeda/test-plan-detail/ci-integration.md:26) Each OS branch of the server verification must be proved on its own runner: Unix peer credentials on `ubuntu-latest`, the Windows pipe server process id on `windows-2025`, the euid + dir-only reading on `macos-latest`; a Linux dev-host run proves no other OS.
  > **Runs:** `agent-run run --e2e` on all 3 OSes
  (.andromeda/test-plan-detail/ci-integration.md:30)
  > Its one instrumented nextest run replaces the Unit, Integration and `run --e2e` rows, which are the local
  > `agent-run` equivalents, and the doctest run follows it. Each job's last step is `viola-harness gate --require
  > <suites>`: the per-OS test job uses `coverage,doctest`
  (.andromeda/test-plan.md:767)
  > - OS: `windows-2025` (ConPTY, named pipe, DACL / SDDL read-back, `.cmd` refusal), `macos-latest` (macOS 26 arm64:
  > openpty, Unix socket, euid + dir-only server verification; pin `macos-26` if the label moves), `ubuntu-latest`
  > (openpty, Unix socket, modes; also the deny, zizmor, fuzz and MSRV jobs). `fail-fast: false`.
  (.andromeda/test-plan.md:813)
  > Per-OS gating is deliberate: `#[cfg(windows)]` / `#[cfg(unix)]` code compiles only on its own runner.
  (.andromeda/test-plan.md:873)
  > NEVER treat a Linux-only local run as proof for Windows or macOS. OS-branch code is verified only on its own
  > runner.
- (.andromeda/test-plan-detail/ci-integration.md:38) The verification added before the hook's `hook.event` and `hook.dialog` frames must stay inside every timed perf row on every OS, the verdict read by `viola-harness gate` on the hyperfine export.
  > **Runs:** `agent-run run --perf` (hyperfine 1.20.0 `--export-json` to
  > `target/agent-run/artifacts/perf-<hook>.json`). The verdict comes from `viola-harness gate` per §10, never from a
  > `jq` binary;
  (.andromeda/test-plan.md:751)
  > - **Perf:** `agent-run run --perf` in its own per-OS job, its verdict from `viola-harness gate`, never from a `jq`
  > binary.
- (.andromeda/test-plan-detail/quality-gates-coverage-targets.md:25) Every perf row times a hook through the real channel path to the live `builder` wrapper, so the server verification now runs inside each timed sample and must pass there (a verification failing open would time the no-frame path, not the channel path), with the session's role files still holding no `panic` line and the hook's connect deadline still below `SPINE_DEADLINE`.
  > - **Perf session:** `--perf` then boots ONE session through its `PerfSession` seam (`boot::boot` /
  > `cleanup::cleanup`; the harness's own tests stand in for both): `perf-<harness pid>`, a `builder` instance,
  > `BootOptions { bin_dir: target/perf/release, build: false, stamp: false }`, its home under `target/e2e-home/`
  > (boot step 2), unstamped, so the `pre-tool-use` row times the unverified-CLI path (`hook.dialog` → `null` at
  > once). Each row's synthetic payload (string fields only, the tests-owned content canary in content fields; no
  > recorded fixture exists yet) is written to `target/agent-run/<session>/payload-<hook>.json` and fed through
  > `--input`; hyperfine runs with the repo root as cwd and `VIOLA_NAME=builder` /
  > `VIOLA_DIR=<perf home>/instances/builder` set, so every timed hook takes the real logging and channel path, never
  > the no-instance no-op (obs-plan §10 obs-overhead row) — as measured at chunk 2026-09-28-hook-perf-gate: a
  > kept-home run held 132 `hook-decision` lines with no fallback `detail` and 33 wrapper events per row. Each row's
  > stale export is removed first. After the last row and **before** `cleanup`, `--perf` asserts the home's role files
  > (`diagnostics/*.ndjson` without `detail-*`) hold no `event:"panic"` line, with no seam exemption, because a hook
  > panic exits 0 and hyperfine cannot see it; a panic line fails `panic`. Then `cleanup` (`keep_homes` =
  > `AGENT_RUN_KEEP_HOMES` exactly `1`) must report `ok:true`. A failed build (`build`), boot (`boot: <reason>`), row
  > (the hook's name), panic check or cleanup turns suite `perf` red with the failure named, never a missing or
  > skipped row.
  (.andromeda/test-plan.md:834)
  > - **Spine deadline:** the gate reads the product constant `viola_core::SPINE_DEADLINE` (1.0 s), imported by
  > `viola-harness` (`gate.rs` `perf()`), never an edited literal in `ci.yml` or this table. The hook's own connect
  > deadline is the crate-private, provisional `CONNECT_DEADLINE` = 750 ms in the root bin's `src/cmd/hook.rs`, below
  > the gate and asserted below it by `connect_deadline_is_below_the_spine_deadline`.
  (.andromeda/test-plan.md:835)
  > `run --perf` times the `session-start`, `user-prompt-submit`, `stop`, `session-end` and `pre-tool-use` rows
  > (`pre-tool-use` since chunk 2026-10-04-dialog-answers-by-dialog-id); CI runs it in its own per-OS `perf` job
  > (`test` and the pre-push carry no perf step), ending in `gate --require perf`. No async-tier row is timed. The
  > fail-open matrix's `Instant` bound (`hook_fails_open_silently_within_the_spine_bound`, each case `< 1.0 s`) still
  > asserts the spine bound at unit speed as well.
  (.andromeda/test-plan.md:841)
  > | `viola hook pre-tool-use` on an unverified CLI (`hook.dialog` → `null` at once), timed by `run --perf` and
  > required by `perf::ROWS` | mean, median, stddev | `max <` `viola_core::SPINE_DEADLINE` (1.0 s) |

## Bindings
- **test-data-fixtures ↔ mocking-stubbing-discipline**: a mismatch is manufactured either by an impostor bound through the interprocess server API at the endpoint the wrapper's own snapshot records, or by altering the bytes the wrapper wrote, never by a hand-written `snapshot.json` naming a foreign pid (.andromeda/test-plan-detail/test-data-fixtures.md:10) (.andromeda/test-plan-detail/mocking-stubbing-discipline.md:19).
- **test-data-fixtures ↔ ci-integration**: the Windows leg of the verification runs on `windows-2025` in root tests whose homes `seed_conpty` made, so the P4 fork over the strict-modes half decides whether those cases must move to `stamped_home` (.andromeda/test-plan-detail/test-data-fixtures.md:10) (.andromeda/test-plan-detail/ci-integration.md:30).
- **mocking-stubbing-discipline ↔ quality-gates-coverage-targets**: the liveness mismatch row (`gone` at any age) and the 5.0 s / 5.1 s boundary row are the same-process, injected-age cases of the perf table, never a cross-process timing (.andromeda/test-plan-detail/mocking-stubbing-discipline.md:8) (.andromeda/test-plan.md:842).
- **ci-integration ↔ quality-gates-coverage-targets**: the perf job runs the perf session whose timed hooks now verify the live wrapper before each frame, the `max` read against `SPINE_DEADLINE` by `viola-harness gate` (.andromeda/test-plan-detail/ci-integration.md:38) (.andromeda/test-plan-detail/quality-gates-coverage-targets.md:25).
- **test-data-fixtures ↔ quality-gates-coverage-targets**: the hook's fail-open outcome is read cross-process from the `hook` receipt (exit 0, empty stdout and stderr) and at unit speed by the fail-open matrix's `Instant` bound (.andromeda/test-plan-detail/test-data-fixtures.md:44) (.andromeda/test-plan.md:835).
- **ci-integration ↔ the scope**: the server identity read in `viola-channel`'s sync and Tokio clients and in `viola-state`'s liveness must leave the sync-crate tokio-free check and the channel's tree form green (.andromeda/test-plan-detail/ci-integration.md:9).
