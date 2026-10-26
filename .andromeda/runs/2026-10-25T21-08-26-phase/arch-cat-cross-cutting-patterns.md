# arch · Cross-cutting Patterns — category read

## Carried
- (.andromeda/architecture.md:420) The chunk's whole purpose: the Windows pipe must admit the owning user's processes and no other OS user's, and the per-OS enforcement (the SDDL, remote rejection, first-instance) is the security plan's to specify, so the chunk asserts that boundary on the Windows leg without widening it.
  > **Local endpoint trust boundary.** Any process that can open an instance's endpoint can `send` prompts and `answer` dialogs, including PermissionRequest. v1 trusts every process of the same OS user and no other. Each endpoint, whether a named pipe or a Unix socket file, must not be connectable by another OS user. The security specialist owns how this is enforced per OS and any per-connection proof beyond it.
- (.andromeda/architecture.md:413) The admission assertions (remote-client rejection read back, second listener on a held name failing, DACL effect) are Windows-only branches: each must compile and be tested on the `windows-2025` CI runner, and the endpoint name stays the short hash.
  > **Cross-platform discipline.**
  >   - Every path written into plugin files uses forward slashes.
  >   - Nothing shells out through `sh`, `bash` or `cmd`, except `hook statusline` running the user's own statusline command (see Hook contract).
  >   - Child processes are spawned directly.
  >   - Endpoint names are hashed and short.
  >   - Every OS-specific branch compiles and is tested on its CI runner.
- (.andromeda/architecture.md:410) Any change to the listener in `viola-channel`'s server (`server.rs`, `server/win.rs`, `endpoint.rs`) stays on std threads and blocking I/O; no runtime is built on the channel-server path.
  > **Tokio containment.** New code on the `run`, `hook`, `send` or channel-server paths uses std threads and blocking I/O. Only `viola-mcp` may build a runtime.
- (.andromeda/architecture-detail/cross-cutting-patterns.md:9) The squatted-name refusal sits on `run`'s start order: the home is canonicalised before the collision check and the endpoint bind, and the endpoint hash follows the home, so a held-name test and the CLI squat refusal must keep that order and that derivation.
  > `run`'s start canonicalises the home after its wrapper log has created it, before the collision check, the pin and the endpoint bind. `viola run --home <dir>` passes the home to the child only through `VIOLA_DIR`, so the pinned copy, plugin folder, endpoint hash, logs, `budget.json` and ledger stamps all follow it.
- (.andromeda/architecture-detail/cross-cutting-patterns.md:9) Every new admission test, and the fix of the folded `send_budget_paused_is_refused_with_the_window_word_and_no_cursor` CARRY, keeps each test in its own home so parallel libtest threads share no endpoint, log or `budget.json`.
  > This is the isolation point for tests: each test runs in its own home, and that home's `ledger/stamps.json` can stamp the fake agent's reported version.
- (.andromeda/architecture-detail/cross-cutting-patterns.md:12) No test seam or environment read may be added to force a remote client, a squat or a capture fix: the two `fake-agent` seams are the only test-seam reads.
  > There are no secrets in v1, and environment variables are not a configuration channel. The `VIOLA_*` variables are wrapper-to-child plumbing set by `viola run` only. The only other variables any `viola` build reads are the two test seams `FAKE_AGENT_PUMP_DELAY_MS` (capped at 5 s) and `FAKE_AGENT_HOOK_PANIC` (exact value `1`, a forced fail-open `hook` panic), both compiled only under `cfg(feature = "fake-agent")` and absent from release builds,
- (.andromeda/architecture-detail/cross-cutting-patterns.md:12) A change to interprocess's features (or any dependency) on the listener that brings an environment read breaks the rule like a direct read.
  > a dependency or feature that reads an environment variable breaks this rule like a direct read does.
- (.andromeda/architecture.md:412) If `src/cmd/run.rs`'s start refusal is touched, its codes-only line goes to the role's home-level process log and no content-bearing detail leaves the instance's `diagnostics/`.
  > Every role's codes-only process log goes to its home-level role file under `diagnostics/`: `run-<name>`, `hook-<name>`, `mcp` or `cli-<name>` `.ndjson`. Content-bearing detail belongs only in the instance's `diagnostics/detail-<role>.ndjson`.

## Bindings
- **Local endpoint trust boundary ↔ security-plan §Authentication & Authorization (IPC access control, Windows)**: arch states the boundary (same OS user only, any endpoint) and hands its per-OS enforcement to security, whose SDDL `D:P(A;;GA;;;<user-SID>)(A;;GA;;;SY)`, `accept_remote(false)` and first-instance clause the chunk asserts; SYSTEM's ACE is security's carve-out under arch's "no other OS user" line (.andromeda/architecture.md:420)
- **Local endpoint trust boundary ↔ body §Established Decisions [Message Broker / IPC] and §Occupied Resources → IPC endpoints**: the guarded endpoint is interprocess 2.4.4's named pipe `\\.\pipe\viola-<h12>`, one per `viola run`; the test-only `viola-test-chan-<pid>-<label>` names stay outside that namespace for any new listener test (.andromeda/architecture.md:39) (.andromeda/architecture.md:260)
- **Cross-platform discipline ↔ test-plan (Windows CI leg)**: "every OS-specific branch ... tested on its CI runner" is the arch side of the chunk's acceptance being measured on `windows-2025`; the DACL-effect assertion with no second interactive user falls to "Windows-only live measurements" (.andromeda/architecture.md:413)
- **Tokio containment ↔ body [Concurrency / Backend Framework] and the sync-crate ban**: `viola-channel`'s normal graph without its `tokio` feature must stay tokio-free (`scripts/channel-tokio-free.sh`) after the listener change (.andromeda/architecture.md:410) (.andromeda/architecture.md:23)
- **Config management (test homes) ↔ test-plan and the folded budget-test CARRY**: per-test homes are the isolation point the CARRY's log-capture fix relies on (.andromeda/architecture-detail/cross-cutting-patterns.md:9)
- **Diagnostic output channels ↔ obs-plan (logger and format owned by obs)**: a touched start refusal logs codes only through obs's format (.andromeda/architecture.md:412)
