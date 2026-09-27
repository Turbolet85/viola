# Codebase Research — 2026-09-27-wrapper-channel

## Scope
- **Depth:** moderate (a new crate on a mature workspace; the start order it threads into is small) · **Reads:** 12 · **Globs/Greps:** 24
- **Harness rules consulted:** `.claude/rules/verification-harness.md` read in full (6 Session Additions — the `test(=tests::name)` filter form, never pipe `boot`, `--leg windows-2025` for a diff with `#[cfg(unix)]` bodies, verdict-not-counts); `.claude/rules/testing.md` (auto-loaded, 12 Session Additions — the remove-the-guard run, test-only hold for timing windows, one test per `OnceLock`, `try_wait`-bounded waits, byte-identical pinned copy skipped by scans)
- **Platform issues consulted:** GitHub REST "List check runs for a Git reference" (docs.github.com/en/rest/checks/runs, fetched at P4 for the plan's CI-read entry) — `per_page` default 30, max 100; `filter` `latest` "returns the most recent check runs"; "If there are more than 1000 check suites on a single git reference, this endpoint will limit check runs to the 1000 most recent check suites." The read passes `per_page=100` (ci.yml yields 15 checks per push, run 36302088443). No CI verdict was folded at take-up (run 36302088443 green), and the carried viola-pty red is a recurrence watch with no instrumentation planned

## Files inspected
- `src/cmd/run.rs` (full, 254 lines) — the start order: `run()` :54 → `collision_check` :108 → `pin_and_plugin` :122 → `start_state` :140 (first snapshot with `endpoint: None` :148) → `spawn_and_pump` :175. The doc comment :51-53 says "The version gate and the endpoint bind have no step yet". Refusal writer `refuse(unable, hint)` :222; `refuse_live` :236 (`already-live`), `refuse_stale` :241.
- `src/run/mod.rs` (full) — `log_self_start` :17 (`process-start{subject:"self"}` — no `endpoint_kind` yet), `log_child_start` :30 (`pty_backend`, `env_stripped_count` already fields), `log_self_exit(code, detail)` :55.
- `crates/viola-state/src/snapshot.rs` (full) — `InstanceSnapshot.endpoint: Option<String>` :31 exists, skipped when `None`; `write_snapshot` :54 goes through `replace_private` under the `.lock` sibling; `read_snapshot` :68 already `take(MAX_FRAME)`.
- `crates/viola-state/src/fs.rs` :96-101 — `replace_private_shared`: accepts a failed replace when the target already holds the bytes.
- `tests/channel_sqos_open.rs` (full, 166 lines) — `SQOS_OPEN` const :32, `open_adopted` :119 (`CreateFileW` → `OwnedHandle` → `Stream::try_from`), identification (level 1) and no-SQOS control (level 2) tests :149/:157. Test-owned pipe `\\.\pipe\viola-test-sqos-<pid>-<label>` :138.
- `crates/viola-core/src/lib.rs` — `MAX_FRAME: u64 = 16 MiB` :9, `VERSION` :6; `crates/viola-core/src/obs.rs` — `ObsEvent::ChannelRequest` / `ChannelResponse` / `ParseRejected` already in the enum (:13/:14/:31).
- `schemas/diag-line.v1.json` — top-level `required` = `timestamp, level, target, message, event, process`; `allOf[19]` channel-request and `allOf[20]` channel-response already ALLOW `corr` (`$defs/corr`), `conn`, `srv_conn`, `method`, `error_code` enum; `corr` is required nowhere (python json read of `allOf`/`required`).
- `tests/contract_diag_schema.rs` — the schema contract suite (:98 a channel line with `conn`; :103-:138 null-corr and corr-confinement cases) — the negative test's home.
- `Cargo.toml` :112-141 — `interprocess = "=2.4.4"` and `windows-sys = "=0.61.2"` already in `[workspace.dependencies]` (features include `Win32_Security`, `Win32_System_Pipes`; NOT `Win32_Security_Authorization`); `tracing = "=0.1.44"`; no `veil` entry. `deny.toml` :45-47 already bans veil's `toggle`.
- `deny-sync.toml` (sole-root tokio ban); `.github/workflows/ci.yml` :290-291 and :391-394 read `scripts/sync-crates.txt` (currently viola-core, viola-pty, viola-agent-claude, viola-state).
- `fuzz/Cargo.toml` — own workspace, one `[[bin]] viola_name`, deps `arbitrary =1.4.2`, `libfuzzer-sys =0.4.13`, `viola-core` by path; `fuzz/corpus/viola_name/` only.
- `crates/viola-e2e/src/harness/pre_push.rs` :216-276 — `stages()`: tools → sync → cache → `linux-tests` → `linux-leg` → `windows-leg` (`run_with` mutants only, `HOST_LEG` :34) → `union`. No Windows coverage stage.
- `crates/viola-e2e/src/harness/cleanup.rs` :67 — `"endpoint_gone": null`; `crates/viola-e2e/tests/harness_lifecycle.rs` :44 boots `["builder", "overseer"]`, :81 asserts `endpoint_gone` is null.
- `src/bin/viola-fake-agent.rs` :342-349 — the `fds` receipt (Unix, `/dev/fd` listing); `tests/cli_fake_agent.rs` :211 asserts it CONTAINS 0/1/2, never an exact set.
- interprocess 2.4.4 source (`D:/dev/rust/cargo/registry/src/index.crates.io-*/interprocess-2.4.4/src`): `os/windows/named_pipe/local_socket/listener.rs` :30-41, `os/windows/named_pipe/listener/options.rs` :81-93, `create_instance.rs` :82-105, `os/unix/c_wrappers.rs` :177, `os/unix/uds_local_socket/listener.rs` :32-49 — see the measured facts below.

## Graph impact
- **write_snapshot / start_state / collision_check / spawn_and_pump / log_child_start / log_self_start** — callers: `src/cmd/run.rs` only (run() :56/:85/:93/:103, start_state :158, spawn_and_pump :194/:195) plus `viola-state` snapshot unit tests :106/:125/:141 (trace `tree-query-2026-09-27-wrapper-channel.json`, record 1; `line` +1 applied). The start-order change is confined to the root bin.
- **crate_edges** — `viola → {viola-core, viola-pty, viola-state, viola-agent-claude}`, `viola-state → viola-core`, `viola-e2e → {viola-core, viola-pty}` (record 2). `viola-channel` is new: no inbound edge today; it gains `viola → viola-channel` and `viola-channel → viola-core`.
- **InstanceSnapshot construction sites** (name grep `InstanceSnapshot {` over the whole tree: 4 hits · 1 changed · 3 no-change): `src/cmd/run.rs:147` (changed — `endpoint: Some(..)`); `viola-state/src/snapshot.rs:86` and `liveness.rs:79` test helpers (no change — `None` stays valid); the struct def :29.
- **`endpoint` name sweep** (`grep -rn endpoint --include=*.rs src crates tests`: 5 hits outside snapshot.rs · 3 changed · 2 no-change): `src/cmd/run.rs:53` doc comment (changed), `:148` (changed), `harness/cleanup.rs:67` `endpoint_gone: null` (changed — test-plan §3 cleanup step 4), `harness_lifecycle.rs:81` (changed with it), `harness/status.rs:1` doc prose (no change), `liveness.rs:80` (no change).

## Measured facts (research-originated mechanisms, at HEAD's pinned versions)
- **Windows listener defaults (interprocess 2.4.4).** The `local_socket` listener builds `PipeListenerOptions::new()` (listener.rs :35) whose defaults are `accept_remote: false` → `PIPE_REJECT_REMOTE_CLIENTS` (options.rs :88, create_instance.rs :101-103), `inheritable: false` (:93), `FILE_FLAG_FIRST_PIPE_INSTANCE` on the first instance (create_instance.rs :86), and `security_descriptor` taken ONLY from `ListenerOptionsExt::security_descriptor` (listener.rs :38) — `None` → a null `SECURITY_ATTRIBUTES` → the Windows default named-pipe DACL (per security-plan §Security Anti-Patterns → Authentication: read to Everyone and anonymous). So of security-plan's four listener controls, three are interprocess defaults and only the SDDL needs code: `SecurityDescriptor::deserialize` (owned.rs :55, over `ConvertStringSecurityDescriptorToSecurityDescriptorW`) exists in 2.4.4; the user SID string needs `GetTokenInformation(TokenUser)` + `ConvertSidToStringSidW`, which is windows-sys feature `Win32_Security_Authorization` — absent from the workspace pin's feature list today.
- **The arbiter on Windows** is `FILE_FLAG_FIRST_PIPE_INSTANCE`: a second `create` of the same pipe name fails while the first instance lives, and the pipe dies with its process (no stale name).
- **The arbiter on Unix** is NOT free: interprocess binds through `listen_and_maybe_overwrite` with `try_overwrite` off → a leftover socket file from a crashed wrapper makes every later bind fail, and an unlink-then-bind takeover races (A unlinks, A binds, B unlinks A's file, B binds: both win). A race-free Unix arbiter needs a held exclusive lock (std `File::try_lock` on a `.lock` sibling in the 0700 socket dir, released by the OS at process death) taken before the stale unlink + bind. This is the EQUALITY the criterion needs: two concurrent same-name starts → exactly one `try_lock` succeeds → exactly one binds.
- **Unix handle inheritance.** interprocess creates the listening socket with `SOCK_CLOEXEC` (c_wrappers.rs :177) and accepts through std `UnixListener::accept` (listener.rs :53), which sets CLOEXEC — so the E2 fds-only receipt is expected to hold; the test must assert the EXACT set, since `cli_fake_agent.rs:211` only asserts containment.
- **Endpoint path.** architecture §Occupied Resources :335 still registers `$TMPDIR/viola-<h12>.sock`; security-plan Decisions Log amendment 2 (:15 of the log) moves it to `<per-user 0700 dir>/viola-<h12>.sock`, and §Bootstrap phases :371 folds amendment 2 "before the `viola-channel` chunk". The plan follows amendment 2; arch §Occupied Resources is an Expected amendment at wrap.

## Patterns detected
- **Fixed two-line start refusal** (`src/cmd/run.rs:222`): `unable: …` then `hint: …` on stderr, then `log_self_exit(1, Some(<detail>))`. The bind-loser reuses it.
- **Codes-only role lines** (`src/run/mod.rs:17-74`): every line through `obs_event!` with typed fields; `detail` a `&'static str`.
- **Snapshot writes** only through `write_snapshot` (`viola-state/src/snapshot.rs:54`) → `replace_private` under the `.lock` sibling.
- **SQOS open** (`tests/channel_sqos_open.rs:119-134`): `CreateFileW` with `SQOS_OPEN`, `OwnedHandle`, `Stream::try_from`.
- **Stage-by-stage pre-push document** (`pre_push.rs:216-276`): each stage sets `doc.stage`, runs through the `Runner` seam, and stops at the first red; host stages call `run_with(ws, Selection{..}, …, runner)`.

## Conventions to follow
- **New crate manifest**: `[lints] workspace = true`, `license.workspace = true`, `publish = false` — `tests/contract_lints.rs` asserts every product member inherits the workspace lints.
- **Test names** `<subject>_<condition>_<expected>`; rstest `#[case::label]` tables with literal oracles (testing.md).
- **No `tokio` feature yet**: a feature with no consumer is unobservable (testing.md 2026-09-24) — the Tokio client lands with `viola-mcp`.

## New files to create
- `crates/viola-channel/Cargo.toml` · `src/lib.rs` (`ChannelError`, re-exports) · `src/frame.rs` (bounded ndjson read/write, envelope types, error mapping) · `src/endpoint.rs` (FNV-1a `viola-<h12>`, per-OS path, per-user dir) · `src/server.rs` (bind + arbiter + accept thread + dispatch with `-32601`) · `src/client.rs` (sync client; Windows SQOS open; `conn`) · `crates/viola-channel/tests/<topic>.rs`.
- `tests/channel_<topic>.rs` (root: real endpoint, oversize, newer-`v`, squat/arbiter), `tests/security_negatives_channel.rs` (Windows SQOS via the viola client).
- `fuzz/fuzz_targets/channel_frame.rs` + `fuzz/corpus/channel_frame/` seeds; `crates/viola-channel/proptest-regressions/` when the property first shrinks.

## Files to modify
- `Cargo.toml` — `veil = "=0.3.0"` (no `toggle`) in `[workspace.dependencies]`; windows-sys feature `Win32_Security_Authorization`; root `[dependencies]` gains `viola-channel`.
- `src/cmd/run.rs` — the bind between `pin_and_plugin` and `start_state`; `endpoint` in the first snapshot; the bind-loser refusal; the Scenario 1 spans; doc comment :51-53.
- `src/run/mod.rs` — `endpoint_kind` on `process-start{subject:"self"}`.
- `crates/viola-pty/Cargo.toml` + `src/lib.rs` — `tracing` and the `pty.spawn` / seam spans (lean in plan).
- `schemas/diag-line.v1.json` + `tests/contract_diag_schema.rs` — `corr` required where obs-plan defines it (see closure).
- `scripts/sync-crates.txt` — `viola-channel`.
- `fuzz/Cargo.toml` — a `[[bin]] channel_frame` and a `viola-channel` path dep.
- `crates/viola-e2e/src/harness/pre_push.rs` — a `windows-tests` coverage stage before `windows-leg`; `harness/cleanup.rs:67` + `boot` readiness (endpoint); `crates/viola-e2e/tests/harness_lifecycle.rs:81`.
- `tests/cli_fake_agent.rs` or a new `tests/tui_*.rs` Unix case — the exact fds set under `viola run`.
- `Cargo.lock` (new crate, veil).

## Open questions
- Where `pty.spawn` and the seam spans live → blocks: plan-decision. Lean available: obs-plan §3 lists `tracing` for `viola-pty`, and the 2026-09-25 wrap REJECTED the tracing-free proposal (obs-plan-amendments :101) — spans on the seam, `tracing` dep added, arch dependency direction amended at wrap.
- Unix arbiter: lock sibling vs bind alone → blocks: plan-decision. Lean: the lock (measured race above).
- Whether `endpoint_gone` flips to a real check in this chunk (test-plan §3 cleanup) → blocks: implementation-scope (the harness surface exists once the bind lands; the CLI exit-21 half needs a CLI verb that connects, which is Epoch 3's).
