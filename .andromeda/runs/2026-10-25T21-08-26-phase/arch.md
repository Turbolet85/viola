# arch extract

## Relevance
relevant — the chunk hardens the Windows half of the wrapper endpoint (`viola-channel`'s named-pipe listener) and asserts the `run` start refusal; the folded budget-test CARRY is test-harness work outside arch's domain.

## Constraints
- Any process that can open an instance's endpoint can `send` and `answer` (PermissionRequest included). v1 trusts every process of the same OS user and no other, so a named pipe must not be connectable by another OS user. arch fixes this boundary, and security owns how each OS enforces it (per architecture §Cross-cutting Patterns → Local endpoint trust boundary).
- The Windows endpoint is the named pipe `\\.\pipe\viola-<h12>`, one per `viola run` (per architecture §Occupied Resources → IPC endpoints). Its name rule is the FNV-1a 64-bit hash over `ViolaName` + `\0` + the absolute home, first 12 hex. It is hand-written in `viola-channel` and recorded in the snapshot, with `DefaultHasher` forbidden. The chunk must not change it (per architecture §Conventions → Data model conventions, Endpoint name).
- The wrapper IPC is interprocess 2.4.4 `local_socket` behind viola's own `channel` seam. The server's pipe DACL is converted from SDDL by windows-sys 0.61.2 (`Win32_Security_Authorization`) (per architecture §Stack and Technologies → Wrapper IPC). windows-sys is a Windows-only dependency of `viola-channel`, and any new Win32 feature the listener needs (for example, reading the pipe's mode back) is a dependency change against the workspace pin (per architecture §Infrastructure Patterns → Crate dependency direction).
- The start refusal stays as it is. `viola run` losing the exclusive endpoint bind to another holder exits 1 with `detail:"squatted-name"` and the fixed pair `unable: the endpoint for <name> is held by another process` / `hint: another process holds this name's endpoint; stop it or pick another name`, with no path, pid or pipe name (per architecture §Conventions → CLI exit codes). As landed, it prints no `--json` document, which is owed to "Exit-cause code catalogue" (per architecture §Standard Contracts → CLI `--json` output).
- The channel-server path uses std threads and blocking I/O. Only `viola-mcp` may build a runtime (per architecture §Cross-cutting Patterns → Tokio containment). `viola-channel` reaches tokio only behind its `tokio` feature, and its normal graph stays tokio-free (per architecture §Established Decisions → [Module Boundaries]).
- Every OS-specific branch compiles and is tested on its own CI runner (per architecture §Cross-cutting Patterns → Cross-platform discipline). So a Windows-only listener assertion runs on the `windows-2025` leg of `ci.yml`'s three-OS matrix (per architecture §Infrastructure Patterns → CI/CD approach).
- `viola-channel` keeps one thiserror enum (`ChannelError`) with fixed messages, and anyhow appears only in the root bin (per architecture §Established Decisions → [Error Handling]).

## Patterns to follow
- Code placement:
  - The listener and its admission settings live in `crates/viola-channel` (the server module and its Windows submodule).
  - The CLI-level squat refusal lives in the root bin's `run` start.
  - The crate edges stay `viola-channel` → `viola-core` + third-party, and the root bin → all members (per architecture §Infrastructure Patterns → Project directory structure; per architecture §Infrastructure Patterns → Crate dependency direction).
- Test pipes use the test-only name `viola-test-chan-<pid>-<label>` (or `viola-test-sqos-<pid>-<label>` for the SQOS open). These names sit outside the `viola-<h12>` namespace, are unique per test and are never bound by `viola`. A listener-property test (a second listener on a held name, or reading back the pipe mode or DACL) follows this rule (per architecture §Occupied Resources → IPC endpoints).
- `viola-channel`'s Windows read-back helpers (`canonical_sddl`, `dacl_of`, an independent `user_sid`) sit behind the `test-support` feature. Only the root package's `[dev-dependencies]` enables it, and `scripts/release-check.sh` refuses a release artifact built with it. A new shared read-back helper belongs behind the same feature (per architecture §Occupied Resources → Workspace crates).
- The Windows client half (the SQOS Identification open adopted by `Stream::try_from`, never interprocess's default connect) is the paired control on the other end of the pipe. This chunk leaves it as it is (per architecture §Established Decisions → [Message Broker / IPC]).

## Anti-patterns to avoid
- Do not widen who can open the endpoint, and add no `config.json` key, env var or flag that relaxes the listener's admission. Environment variables are not a configuration channel (per architecture §Cross-cutting Patterns → Config management; per architecture §Cross-cutting Patterns → Local endpoint trust boundary).
- Do not change the wire contract, exit code or refusal word. The squat refusal must never put the pipe name, a path or a pid into any `unable:`/`hint:` line or log detail (per architecture §Conventions → CLI exit codes).
- Do not use Tokio or an async listener on the channel-server path (per architecture §Cross-cutting Patterns → Tokio containment).

## Contract bindings
- arch ↔ security: the Local endpoint trust boundary defers per-OS enforcement (the protected SDDL, `accept_remote(false)`, first-instance, non-inheritable handles) to security-plan §Authentication & Authorization (IPC access control, Windows). arch requires only that no other OS user can connect (per architecture §Cross-cutting Patterns → Local endpoint trust boundary).
- arch ↔ tests:
  - The `windows-2025` matrix leg (harness `run --coverage`, with per-OS floors) is where these Windows assertions are measured (per architecture §Infrastructure Patterns → CI/CD approach).
  - The test-only pipe namespace keeps test endpoints apart from `viola-<h12>` (per architecture §Occupied Resources → IPC endpoints).
  - Whether a CI runner can stand in for a second principal is tests' and research's question.
- arch ↔ obs: the start refusal's codes-only record goes to the home-level `run-<name>.ndjson` role file, and nothing reaches the terminal outside the child's screen (per architecture §Cross-cutting Patterns → Diagnostic output channels).
- arch ↔ wrap: any new test-only resource (pipe label, scratch path) or new windows-sys feature the chunk lands is recorded in §Occupied Resources or the Crate dependency direction key when the chunk wraps (per architecture §Occupied Resources → IPC endpoints; per architecture §Infrastructure Patterns → Crate dependency direction).

## Acceptance criteria contributions
- (arch) The listener changes and their tests live in `crates/viola-channel` (plus the root bin's `run` start for the squat refusal only). `bash scripts/channel-tokio-free.sh` stays green, so `viola-channel`'s normal graph without its `tokio` feature has no `tokio v` line (per architecture §Established Decisions → [Module Boundaries]).
- (arch) A squatted name still refuses `viola run` with exit 1, `squatted-name` and the fixed `unable:`/`hint:` pair, with no path, pid or pipe name and no new `--json` document (per architecture §Conventions → CLI exit codes).
- (arch) Every new Windows-only listener test binds only `viola-test-chan-<pid>-<label>` / `viola-test-sqos-<pid>-<label>` names, never a `viola-<h12>` name, and runs green on the `windows-2025` CI leg (per architecture §Occupied Resources → IPC endpoints).
- (arch) No new crate edge beyond windows-sys (Windows-only) in `viola-channel`. Any added windows-sys feature or `test-support` helper is named in the Crate dependency direction key or Workspace crates at wrap (per architecture §Infrastructure Patterns → Crate dependency direction).
