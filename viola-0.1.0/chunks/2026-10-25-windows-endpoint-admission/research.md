# Codebase Research — 2026-10-25-windows-endpoint-admission

## Scope
- **Depth:** moderate · **Reads:** 14 · **Globs/Greps:** 12
- **Harness rules consulted:** none — no live leg in this chunk (its Windows witnesses run on CI's `test (windows-2025)` leg through the existing `coverage` suite; the dev-host gates are cargo commands)
- **Platform issues consulted:** `PIPE_REJECT_REMOTE_CLIENTS \\localhost\pipe loopback SMB` (web search) → no fetched source states whether an open through `\\localhost\pipe\…` (the SMB redirector) counts as a remote client; the hits name the redirector path and tokio's CVE-2023-22466 (`pipe_mode` silently resetting `reject_remote_clients`, a tokio-API regression, not this crate's sync path). Microsoft Learn `CreateNamedPipeA` (fetched): «PIPE_REJECT_REMOTE_CLIENTS — Connections from remote clients are automatically rejected»; «If you attempt to create multiple instances of a pipe with [FILE_FLAG_FIRST_PIPE_INSTANCE], creation of the first instance succeeds, but creation of the next instance fails with ERROR_ACCESS_DENIED»; «The ACLs in the default security descriptor for a named pipe grant full control to the LocalSystem account, administrators, and the creator owner. They also grant read access to members of the Everyone group and the anonymous account.»
- **External inputs:** `inputs#I1` — the operator's phase directive (forced sample, evidence kept, a hard cascade failure stops the phase) · `inputs#I2` — the overseer's disposition of the supply-chain red (owner: a new route entry "tokio RUSTSEC-2026-0386 lock bump", no ignore widened) · `inputs#I3` — the invocation (forced sample; the founder's go that the cascade debug loop runs on viola) · `inputs#I4` — the operator's resume: the expired P4 dialog re-asked live · `inputs#I5` — the founder's live answers to the four P4 forks

## Files inspected
- `crates/viola-channel/src/server.rs` (full) — the Windows `listen` (lines 95–117): `ListenerOptions::new().name(..).security_descriptor(owner_only_descriptor()).create_sync()`; a `PermissionDenied` create maps to `ChannelError::BindTaken` (108–112); the only mention of remote rejection is the doc comment at 95–96. Unit `bind_of_a_live_endpoint_is_taken_until_it_drops` (970–995) runs on every OS, Windows included.
- `crates/viola-channel/src/server/win.rs` (full) — `owner_only_sddl` / `owner_only_descriptor` / `user_sid`; unit cases `owner_only_sddl_is_protected_user_and_system`, `bound_pipe_dacl_reads_back_protected_user_and_system` (121–130, through `test_support::{canonical_sddl, dacl_of, user_sid}`), `pipe_handles_on_both_ends_are_not_inheritable` (133–155).
- `crates/viola-channel/src/test_support.rs` (1–80) — `JsonFields`; the Windows read-back helpers `canonical_sddl`, `dacl_of`, an independent `user_sid` behind `cfg(windows)`; the comment records CI's runner user as the built-in Administrator (rendered `LA`).
- `crates/viola-channel/Cargo.toml` (full) — windows-sys is a `cfg(windows)` dependency taking the workspace pin; features `fake-agent`, `test-support`, `tokio`.
- `Cargo.toml` (247) — the workspace windows-sys `=0.61.2` already carries `Win32_Security` (`ImpersonateAnonymousToken`, `RevertToSelf`), `Win32_Storage_FileSystem` (`CreateFileW`), `Win32_System_Pipes` (`CreateNamedPipeW`) and `Win32_System_Threading`: no new feature is needed for any witness below.
- `~/.cargo/registry/…/interprocess-2.4.4/src/os/windows/named_pipe/local_socket/listener.rs` (28–41) — the local-socket listener builds `PipeListenerOptions::new()`, setting only path, nonblocking and the security descriptor.
- `~/.cargo/registry/…/interprocess-2.4.4/src/os/windows/named_pipe/listener/options.rs` (59–93) — defaults `accept_remote: false`, `inheritable: false`.
- `~/.cargo/registry/…/interprocess-2.4.4/src/os/windows/named_pipe/listener/create_instance.rs` (26–101) — the first instance is created with `first = true` → `FILE_FLAG_FIRST_PIPE_INSTANCE` (86); every instance, the per-accept ones included (`listener.rs:124`, `first = false`), gets `PIPE_REJECT_REMOTE_CLIENTS` while `accept_remote` is false (100–101).
- `tests/security_negatives_channel.rs` (fn list) — `#![cfg(windows)]`; holds `security_negative_viola_client_grants_only_identification` (104), the client SQOS negative — not the DACL read-back CARRY 1 names.
- `tests/channel_sqos_open.rs` (1–80) — the pattern for a Windows negative: a test-owned pipe through `CreateNamedPipeW` on a unique name, windows-sys calls under `// SAFETY:` comments, a control case.
- `tests/path1_run_start.rs` (4, 23, 116–124) / `tests/cli_instance_state.rs` (30, 263) — the squatted-name start refusal asserted: squatter bound first through `viola_channel::Server::bind`, the exact stderr pair, `refused_with(.., "squatted-name")`.
- `src/run/send/budget.rs` (1–140) — the folded CARRY's case: `capture_lines` around one `send`, then a `send-refused` line looked up by `find`.
- `src/run/send/test_support.rs` (405–424) — `capture_lines`: a `Lines` layer on a fresh registry, installed with `tracing::subscriber::with_default` (thread-scoped).
- `src/run/gate.rs` (415–440) — `feed_panic_writes_one_parse_rejected_line_until_the_size_changes` installs `set_global_default` ("the one global-subscriber test of its process") in the same root-bin test binary.
- `.andromeda/test-plan-detail/test-scope-summary.md` (11) — the `viola-channel` entity's untestable zone: «rejecting remote pipe clients, and denying access through the Windows pipe DACL … cannot be driven on hosted GitHub runners, which have no second OS account».

## Graph impact (from the code-graph query; rust plane, regenerated at this run: 6597 nodes / 36629 edges, Linux-indexed)
- **`Server::bind`** (`crates/viola-channel/src/server.rs`) — 22 call sites: product caller `cmd/run/bind_endpoint()` (`src/cmd/run.rs:482`); tests in `server.rs` (962–1004), `client.rs` (344, 509), `tokio_client.rs:144`, `crates/viola-channel/tests/channel_frames.rs:36`, `crates/viola-e2e/src/harness/cleanup.rs:392`, `crates/viola-mcp/tests/handlers.rs:69`, `src/cmd/hook.rs:745`, `tests/channel_endpoint.rs` (107, 141–147, 155), `tests/cli_instance_state.rs:263`, `tests/path1_run_start.rs:116`. This chunk changes no signature, so no caller threads.
- **`refuse_squatted`** — 1 caller, `cmd/run/start()` (`src/cmd/run.rs:253`); unchanged.
- **`listen` / `owner_only_descriptor` / `user_sid`** — the query returned `Server::bind` → `listen` only: the `cfg(windows)` items are not on a Linux-indexed plane (rust-analyzer resolves the host's cfg), so their 0 rows are an index gap by construction, not an absence; the grep of `server/win.rs` above is the witness.

## Patterns detected
- **The Windows listener takes interprocess's defaults whole** (`server.rs:104-107`): the only option viola sets is the security descriptor; remote rejection, first instance and non-inheritance come from `PipeListenerOptions::new()` (interprocess `options.rs:88,93`, `create_instance.rs:86,100-101`). The equality the chunk needs: for every instance of a viola endpoint, `dwPipeMode & PIPE_REJECT_REMOTE_CLIENTS != 0` — it holds at HEAD by interprocess's code, and nothing in viola pins it.
- **First instance is already a listener property, asserted** (`server.rs:970-995`, `tests/channel_endpoint.rs:141-147`): a second `Server::bind` of a live endpoint is `ChannelError::BindTaken`, which on Windows is the `ERROR_ACCESS_DENIED` the first-instance flag yields (`server.rs:108-112`; Microsoft Learn above) — the CLI squat refusal rests on the same error.
- **A Windows witness is a test-owned pipe plus a control** (`tests/channel_sqos_open.rs`): raw windows-sys calls, a unique `viola-test-sqos-<pid>-<label>` name, a control proving the check can fire.
- **Read-back helpers live behind `test-support`** (`crates/viola-channel/src/test_support.rs`), enabled only by the root `[dev-dependencies]`.

## Conventions to follow
- **Test pipe names**: `viola-test-chan-<pid>-<label>` / `viola-test-sqos-<pid>-<label>`, never a `viola-<h12>` (architecture §Occupied Resources → IPC endpoints; `crates/viola-channel/src/lib.rs:203` `test_capture::test_endpoint`).
- **Every `unsafe` block carries a `// SAFETY:` line** (`server/win.rs`, `tests/channel_sqos_open.rs`).
- **Root integration binaries `<surface>_<topic>.rs`, functions `<subject>_<condition>_<expected>`** (test-plan §2; `tests/security_negatives_channel.rs`).

## New files to create
- none

## Files to modify
- `tests/security_negatives_channel.rs` — the Windows admission negatives (remote client rejected, read back; an anonymous open denied), each with its controls
- `Cargo.toml` — a root `[target.'cfg(windows)'.dev-dependencies]` windows-sys entry adding the `Wdk_Storage_FileSystem` feature for the pipe-type read-back
- `src/main.rs` — the shared scoped-capture helper in `test_support` and its witness case in `tests`
- `src/run/send/test_support.rs` — `capture_lines` through the helper
- `src/run/dialog.rs` — its two scoped captures through the helper
- `src/run/budget.rs` — its scoped capture through the helper
- `src/run/gate.rs` — its scoped capture through the helper
- `src/obs.rs` — its scoped capture through the helper
- `src/cmd/run.rs` — its two scoped captures through the helper
- `src/cmd/hook/statusline.rs` — its scoped capture through the helper

## Sweeps
- `REJECT_REMOTE|accept_remote|FIRST_PIPE_INSTANCE` over `crates/ src/ tests/` (grep -rn -i -E): 1 hit (`crates/viola-channel/src/server.rs:95`, a doc comment) · 0 changed · 1 no-change (doc text, already true)
- `squat` over `tests crates src` (grep -rn -i -l): 4 files (`tests/security_negatives_channel.rs`, `tests/path1_run_start.rs`, `tests/cli_instance_state.rs`, `src/cmd/run.rs`) · 0 changed · 4 no-change (the start refusal stands unchanged)

## Measurements
- **The folded CARRY's case under a plain `cargo test`** (`cargo test --bin viola`, three runs, Linux dev host, HEAD `b6740d59`; script `$TMPDIR/{scratchpad}/budget-case-x3.sh`): `send_budget_paused_is_refused_with_the_window_word_and_no_cursor::case_4_both_name_five_hour` **ok 3 of 3** (the other three cases ok 3 of 3). The CARRY's mechanism («its log capture found no `send-refused` line») did not reproduce: unverified, neither confirmed nor falsified.
- **The same three runs are red elsewhere** (`test result: FAILED. 1036 passed; 2 failed` · `1035 passed; 3 failed` · `1036 passed; 2 failed`): `run::gate::tests::feed_panic_writes_one_parse_rejected_line_until_the_size_changes` 3 of 3 (`left: 2 right: 1` at `src/run/gate.rs:438` — its global subscriber also took a vt100 `attempt to subtract with overflow` line from another test's feed thread); `obs::tests::log_config_rejection_writes_one_warn_line` 2 of 3 (`lines[0].get("instance").is_none()` at `src/obs.rs:714`); `cmd::verify::stop::tests::install_turns_the_signal_into_the_stop_with_the_process_alive::case_2_sigterm` 2 of 3 (`!stopped()` at `src/cmd/verify/stop.rs:99`). Each shares process-global state (a global subscriber, the process context, a signal handler) and is written for nextest's process-per-test, the project's gate runner (CI `test` green on all three legs at `1862fcfb`, ci#39493188302).

## P4 additions (read after the founder's answers, `inputs#I5`)
- **The case_4 mechanism, re-derived at the pinned versions** (tracing 0.1.44, tracing-core 0.1.36 per `Cargo.lock`):
  `NoSubscriber::register_callsite` returns `Interest::never()` (`tracing-core-0.1.36/src/subscriber.rs:676`); with no
  global default and no scoped dispatcher on a thread, `get_default` hands that thread `NONE`
  (`dispatcher.rs:383-386, 446-449`); while at most one dispatcher is registered (`has_just_one`, set by
  `register_dispatch` as `len <= 1`, `callsite.rs:551-557`) a callsite's first registration reads only the registering
  thread's own default (`Rebuilder::JustOne` → `get_default`, `callsite.rs:544-547, 562-567`), so a callsite first hit
  on a subscriber-less thread caches `INTEREST_NEVER` (`callsite.rs:308-341`); the event macro then skips it on every
  thread (`tracing-0.1.44/src/macros.rs:36`), the one scoped capture's own thread included, until the next
  `register_dispatch` rebuilds the cache (`callsite.rs:484-488`). The equality the fix needs: with a second
  always-interested dispatcher kept registered for the process, `has_just_one` is false whenever a scoped capture is
  live, so a first registration on any thread takes the `Read` path over every live dispatcher and the capture's own
  `registry().with(..)` (no filter: `Interest::always()`) makes the callsite's interest non-never.
- **The same fact is already recorded twice in the repo**, each time for a line emitted OFF the test thread:
  `crates/viola-channel/src/lib.rs:186-188` (`capture_global`) and `.claude/rules/testing.md:64` (2026-09-27). Case_4's
  line is on the test thread; its loss is the callsite's first registration on another, subscriber-less thread.
- **The root bin's scoped captures** (`grep -rn 'subscriber::with_default' src`, 9 sites in 7 files):
  `src/cmd/hook/statusline.rs:375` · `src/cmd/run.rs:772` · `src/cmd/run.rs:909` · `src/run/gate.rs:465` ·
  `src/run/budget.rs:859` · `src/run/send/test_support.rs:421` · `src/run/dialog.rs:902` · `src/run/dialog.rs:1270` ·
  `src/obs.rs:624`. The one global install of the binary is `src/run/gate.rs:429` («the one global-subscriber test of its
  process»), so the fix cannot be a second global default.
- **The pipe-type read-back** is in windows-sys 0.61.2 under `Wdk_Storage_FileSystem`: `NtQueryInformationFile`
  (`src/Windows/Wdk/Storage/FileSystem/mod.rs:671`, `windows_link` to ntdll), `FILE_PIPE_LOCAL_INFORMATION` (2341, its
  `NamedPipeType`), `FilePipeLocalInformation` = 24 (3210), `FILE_PIPE_REJECT_REMOTE_CLIENTS` = 2 (2372); `windows-link`
  is already in `Cargo.lock` (`grep -c '^name = "windows-link"'`: 1). `ImpersonateAnonymousToken` is in `Win32_Security`
  (`src/Windows/Win32/Security/mod.rs:103`), already in the workspace pin. Whether NPFS reports the reject bit through
  `NamedPipeType` is unmeasured: the controls decide it on CI.
- **`pre-push` runs no `cargo deny`** (`crates/viola-e2e/src/harness/pre_push.rs:3`: `tools → linux-tests`), so the
  advisory red reaches only `cargo deny check`'s advisories family and CI's `supply-chain` job.
- **The harness `run`'s first step runs through nextest** (`crates/viola-e2e/src/harness/run.rs:238-271`:
  `nextest-unit`, `nextest-integration`, `nextest-e2e`), so test-plan's «the workspace unit and integration tests
  (`cargo test`)» (`.andromeda/test-plan-detail/test-scope-summary.md:80`) names libtest executed by nextest, as §4 states.

## Open questions
- none — the four P4 forks were answered by the founder live (`inputs#I5`)
