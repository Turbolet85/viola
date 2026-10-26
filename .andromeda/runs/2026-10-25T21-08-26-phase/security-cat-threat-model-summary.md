# security · Threat Model Summary — category read

## Carried
- (.andromeda/security-plan-detail/threat-model-summary.md:41) The Windows pipe listener the chunk hardens must be treated as the highest-impact surface: its admission rules are the whole defence of the wrapper channel.
  > **Vector:** Local IPC, the wrapper channel (JSON-RPC 2.0 over ndjson). This is the highest-impact surface.
- (.andromeda/security-plan-detail/threat-model-summary.md:43) The endpoint whose admission the chunk asserts is the one named pipe per `viola run`; the asserted DACL, remote rejection and first-instance properties must hold for that pipe name form.
  > One endpoint per `viola run`: named pipe `\\.\pipe\viola-<h12>` on Windows, socket file `$TMPDIR/viola-<h12>.sock` (else `/tmp`) on Unix.
- (.andromeda/security-plan-detail/threat-model-summary.md:45) The threat the admission controls must close: any principal admitted to the pipe gains code execution as the user and reads the conversation, so (c)'s denied-open test must show an outside principal gets no handle at all.
  > Whoever can connect can type prompts into the driven Claude session (`send`) and allow PermissionRequest dialogs (`answer` with `behavior:"allow"`). That amounts to running arbitrary tools, and so arbitrary code, as the user. They can also read the whole conversation (`wait`, `last`).
- (.andromeda/security-plan-detail/threat-model-summary.md:48) The admitted principal set must be exactly the same OS user (plus SYSTEM per §Authentication), never wider: the pipe DACL is the Windows enforcement of this boundary.
  > v1 intends to trust every process of the same OS user and no other. How that is enforced per OS (pipe DACL, socket file mode, directory permissions) is not decided in arch and is explicitly handed to security.
- (.andromeda/security-plan-detail/threat-model-summary.md:49) No per-connection authentication sits behind the listener, so the OS admission (DACL, remote rejection) must be the gate the chunk asserts, and no admission test may rest on a frame-level identity.
  > There is no per-connection authentication.
- (.andromeda/security-plan-detail/threat-model-summary.md:50) The pipe name is predictable and not secret, so admission must never rely on name secrecy; first-pipe-instance and the squat refusal are what defend a predictable name.
  > The endpoint name is predictable: FNV-1a over `ViolaName` + home path, which is not secret (Conventions: Endpoint name).
- (.andromeda/security-plan-detail/threat-model-summary.md:53) The shared machine-wide pipe namespace is the threat item (b) answers: a second listener on a held name must fail as a property of the listener itself.
  > On Windows, the named-pipe namespace is shared across the machine.
- (.andromeda/security-plan-detail/threat-model-summary.md:55) The squatted-name start refusal must stand as stated: a squatted name blocks `viola run` at start with exit 1, unchanged by this chunk.
  > `run`'s exclusive bind means a squatted name blocks startup (exit 1).
- (.andromeda/security-plan-detail/threat-model-summary.md:146) The listener's remote-rejection, first-instance and non-inheritable properties come from interprocess defaults of a single-maintainer crate, so they must be asserted by a read-back test on the pipe, not by a doc comment naming the default.
  > Arch notes interprocess has a single main maintainer
- (.andromeda/security-plan-detail/threat-model-summary.md:150) Any `windows-sys` feature or dependency the read-back tests add must pass the supply-chain trust boundary unchanged (no ignore, skip or allow added to reach green; the open tokio advisory red is owned by its own route entry).
  > `cargo deny check` over `deny.toml` (advisories, licences, sources, C-crate / telemetry-crate / feature bans)
- (.andromeda/security-plan-detail/threat-model-summary.md:157) The chunk's assertions must run on the native `windows-2025` leg of `ci.yml`, the leg the scope names for "fully asserted".
  > GitHub Actions `ci.yml`, triggered on push and PR, on a native matrix of `windows-2025`, `macos-latest` and `ubuntu-latest`
- (.andromeda/security-plan-detail/threat-model-summary.md:158) The new Windows admission tests must ride the existing per-OS test job against the fake agent, adding no job.
  > tests against the fake agent with recorded fixtures (per OS under coverage)
- (.andromeda/security-plan.md:23) The tier's elevation names the code-execution IPC endpoint and its predictable Windows pipe names as the reason for the local-boundary items this chunk closes on Windows.
  > First, the local IPC endpoint is effectively a code-execution interface (`send` types prompts, `answer` can allow PermissionRequest tool calls). It currently has no decided per-OS enforcement of its same-OS-user boundary, and its names are predictable in shared namespaces (Windows pipes, `/tmp`).
- (.andromeda/security-plan.md:23) Of the elevated local-boundary items, the chunk owns only IPC endpoint access control on Windows; server impersonation stays with "Server verification before any frame".
  > On top of it sit specific local-boundary items: IPC endpoint access control and server impersonation
- (.andromeda/security-plan.md:35) The body's broad line for the IPC vector: the same-OS-user-and-no-other boundary the Windows DACL must enforce.
  > **Vector:** Local IPC, the wrapper channel, the highest-impact surface: whoever can connect can type prompts into the driven session, allow PermissionRequest dialogs and read the whole conversation. v1 intends to trust every process of the same OS user and no other; there is no per-connection authentication, the self-reported `from` authenticates nothing, and the endpoint name is predictable.
- (.andromeda/security-plan.md:60) Access control rests entirely on OS-level isolation of who can connect, so the DACL's effect (not only its SDDL text) is the requirement item (c) measures.
  > Access control therefore depends entirely on OS-level isolation: who can connect to each IPC endpoint and who can read or write `~/.viola/`. Cross-cutting Patterns (Local endpoint trust boundary) sets the target principal as "every process of the same OS user and no other".
- (.andromeda/security-plan.md:69) Networking is local only: a remote (network) pipe client must be rejected by the listener, the requirement item (a) asserts.
  > **Networking:** local only.
- (.andromeda/security-plan.md:70) The Windows endpoint stays a per-instance named pipe; the chunk changes no endpoint kind or name.
  > Per-instance named pipes (Windows) or Unix domain sockets (`$TMPDIR` or `/tmp`); no Linux abstract namespace.

## Bindings
- **boundary-ipc trust boundary ↔ body §Authentication & Authorization (IPC access control (Windows))**: the same-OS-user boundary (.andromeda/security-plan-detail/threat-model-summary.md:48) is enforced on Windows by the protected user + SYSTEM DACL, remote rejection, first-pipe-instance and non-inheritable handle the chunk asserts (.andromeda/security-plan.md:108); the logon SID stays out.
- **boundary-ipc predictable name ↔ body §Security Anti-Patterns § Authentication**: a listener without `security_descriptor` grants Everyone and anonymous read (.andromeda/security-plan.md:434); the shared namespace and predictable name (.andromeda/security-plan-detail/threat-model-summary.md:53) make that ban binding on any listener the chunk's tests create.
- **boundary-ipc ↔ body §Data Protection (Handle inheritance) and § Data Protection bans**: non-inheritable handles on both ends (.andromeda/security-plan.md:192, .andromeda/security-plan.md:463) are part of the admission set the chunk re-verifies.
- **boundary-ipc squatter ↔ next route entry "Server verification before any frame"**: the client-side impersonation threat (.andromeda/security-plan-detail/threat-model-summary.md:54) and its dated gaps (.andromeda/security-plan.md:439, .andromeda/security-plan.md:441) are out of this chunk's scope; only the start-time squat refusal (.andromeda/security-plan-detail/threat-model-summary.md:55) binds here.
- **boundary-supply-chain ↔ body §Dependency Security**: no `deny.toml` ignore widening (.andromeda/security-plan.md:250) while `supply-chain` reads red on RUSTSEC-2026-0386 (.andromeda/security-plan-detail/threat-model-summary.md:150).
- **threat-infra ↔ test-plan (CI Integration)**: the Windows assertions run in the `windows-2025` per-OS test job (.andromeda/security-plan-detail/threat-model-summary.md:157), with item (c) falling to "Windows-only live measurements" where a CI runner cannot measure it without a second interactive user.
