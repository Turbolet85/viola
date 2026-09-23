## Stack Vulnerabilities

### Rust toolchain (stable; workspace `rust-version = "1.89"`, edition 2024; host 1.95)

- **Version:** >=1.96.0
- **Last release:** 2026-09-01
- **Status:** actively maintained
- **Fits because:** Every viola surface is one native Rust binary. `std::process::Command` spawns `claude agents --json` and the `--version` probe, `std::fs::File::lock` guards state, and std sockets and files carry the local trust boundary (Threat Assessment §2: child process spawning, filesystem state).
- **Key detail:**
  - Current stable is 1.98.1 (2026-09-01).
  - The only 2026 toolchain advisory is CVE-2026-5222: Cargo mis-normalised sparse-registry URLs in Rust 1.68–1.95, fixed in 1.96 (2026-05-28). It affects third-party sparse registries only, and "crates.io users are not affected". viola uses only crates.io, so the host 1.95 is not exposed, but CI's `@stable` already builds on 1.98.x. Moving the host to ≥1.96 aligns local and CI builds.
  - Enforce the crates.io-only property with `deny.toml` `[sources] unknown-registry = "deny"`, `unknown-git = "deny"`.
  - The Windows batch-file argument escaping fixes (the BatBadBut class) apply only to `std::process::Command`. portable-pty builds its own `CreateProcessW` command line (see the portable-pty block), so they do not protect `viola run -- <program>` when `<program>` resolves to a `.cmd`/`.bat` shim. The arch's npm-shim → `claude.exe` resolution is therefore a security control, not only a convenience.
- **Source:** https://blog.rust-lang.org/2026/05/25/cve-2026-5222/ ; https://static.rust-lang.org/dist/channel-rust-stable.toml

### Tokio (only in `viola-mcp` and `viola-ui`)

- **Version:** 1.53.1
- **Last release:** 2026-07-20
- **Status:** actively maintained
- **Fits because:** It is the runtime under the loopback GUI (axum and SSE) and the stdio MCP server, the two async surfaces in Threat Assessment §2.
- **Key detail:**
  - OSV lists no advisory for 1.53.1.
  - The only 2025–2026 advisory on the `tokio` crate itself is RUSTSEC-2025-0023 (2025-04-07): the broadcast channel calls `clone` in parallel without requiring `Sync`. It is fixed in 1.44.2 and later, so 1.53.1 is unaffected.
  - The 2026 "unmaintained" advisories cover only the Tokio 0.1 crates: `tokio-reactor` (RUSTSEC-2026-0057), `tokio-timer` (RUSTSEC-2026-0060) and `tokio-process` (RUSTSEC-2026-0055). `cargo deny check advisories` would catch any of them arriving transitively.
  - The arch's `tokio` ban on the sync crates also keeps the `hook` path small and synchronous.
- **Source:** https://rustsec.org/packages/tokio.html ; https://rustsec.org/advisories/RUSTSEC-2026-0057

### axum (feature `sse`)

- **Version:** 0.8.9
- **Last release:** 2026-04-14
- **Status:** actively maintained
- **Fits because:** It serves the loopback GUI. That GUI streams prompts, assistant output and raw permission `input` to any local process (Threat Assessment §2 Loopback HTTP; §6 "GUI cross-user and cross-origin readability").
- **Key detail:**
  - Neither OSV nor RustSec has an advisory against any axum version.
  - The closest analogue is in the same stack: RUSTSEC-2026-0189 / GHSA-89vp-x53w-74fx (2026-04-29), a DNS-rebinding hole in rmcp's axum-based Streamable HTTP server. The fix was a loopback-only Host allowlist that returns 403. That is exactly viola's planned `host-not-allowed` check, which confirms the Host allowlist is the right control and must run before routing on every route, including `/assets/*` and SSE.
  - axum 0.8 moved the `Host` extractor to axum-extra, and that extractor can lose the port on HTTP/2. Read `http::header::HOST` directly in a middleware, compare it exactly against `127.0.0.1:<port>` and `localhost:<port>`, and keep axum's `http2` feature off (it is not in axum's default features).
  - axum emits no CORS headers by default, so browsers block cross-origin reads of `/api/*` and `EventSource`. Adding a `CorsLayer` would silently open the feed cross-origin.
- **Source:** https://rustsec.org/advisories/RUSTSEC-2026-0189.html ; https://github.com/tokio-rs/axum/issues/2955 ; https://docs.rs/axum-extra/latest/axum_extra/extract/struct.Host.html

### tower-http (`CompressionLayer`, excluding `text/event-stream`)

- **Version:** 0.7.1
- **Last release:** 2026-08-31
- **Status:** actively maintained
- **Fits because:** It is the GUI middleware layer, and so the place for the headers a page that renders untrusted upstream text needs (Threat Assessment §2 Loopback HTTP, §6 "GUI output encoding").
- **Key detail:**
  - No advisory against 0.7.x. The only tower-http advisories (the ServeDir Windows path traversal) predate 2025. viola embeds its assets and does not use `ServeDir`, which avoids that class.
  - Breaking change in 0.7: `CompressionLayer` returns 406 for `Accept-Encoding: identity;q=0` and for the `*` handling per RFC 9110. Re-test the SSE exclusion under 0.7 semantics.
  - Enable the `set-header` feature and use `SetResponseHeaderLayer::overriding(...)` to set:
    - `Content-Security-Policy: default-src 'none'; script-src 'self'; style-src 'self'; connect-src 'self'; img-src 'self'; base-uri 'none'; form-action 'none'; frame-ancestors 'none'; require-trusted-types-for 'script'`
    - `X-Content-Type-Options: nosniff`
    - `Referrer-Policy: no-referrer`
    - `Cross-Origin-Resource-Policy: same-origin`
    - `Cache-Control: no-store` on `/api/*`
  - HSTS does not apply (plain HTTP on loopback).
  - Constraints on the design-owned frontend so the CSP can be enforced (GUI output encoding):
    - Render every event field (prompts, `last_assistant_message`, plan text, tool `input`) with `textContent` or framework text interpolation, never `innerHTML`, `v-html` or `dangerouslySetInnerHTML`.
    - Never render assistant Markdown to HTML without a sanitizer.
    - Keep all JS in `/assets/*`, with no inline scripts.

    `connect-src 'self'` stops a successful injection from sending the feed to another origin.
- **Source:** https://github.com/tower-rs/tower-http/releases ; https://docs.rs/tower-http/0.7.1/tower_http/set_header/response/struct.SetResponseHeaderLayer.html

### clap (derive)

- **Version:** 4.6.7
- **Last release:** 2026-09-14
- **Status:** actively maintained
- **Fits because:** It is the CLI argument surface: `--home`, `--file`, `viola run <name> -- <program> <args>` (Threat Assessment §2 CLI arguments).
- **Key detail:**
  - No advisory (OSV).
  - clap validates syntax only. Route `ViolaName` arguments through `value_parser` calling `ViolaName::try_new`, so a name never reaches path joins or endpoint hashing unvalidated.
  - `--home` and `VIOLA_DIR` choose which `snapshot.json` (and so which `statusline_command`) runs. Canonicalise them and apply the home-permission check in the atomic-write-file block.
- **Source:** https://crates.io/crates/clap

### portable-pty (arch pin `=0.8.1`)

- **Version:** 0.8.1
- **Last release:** 2025-02-11
- **Status:** actively maintained
- **Fits because:** It hosts the `claude` child that receives `send` text as keystrokes (Threat Assessment §2 CLI input, §6 "Bracketed-paste breakout").
- **Key detail:**
  - **Stale-pin risk:** the arch pins `=0.8.1`, released 2023-03-13, outside the 2025–2026 window. The crate's latest upstream release is 0.9.0 (2025-02-11), which the arch excludes for wezterm#6783. The wezterm repo is active (pushed 2026-09-21, not archived). The named swap candidate is portable-pty-psmux 0.9.7 (2026-08-18).
  - OSV lists no advisory for portable-pty.
  - 0.8.1 depends unconditionally on `serial = "0.4"`, which carries RUSTSEC-2017-0008 (unmaintained, still active, re-issued 2024-12-04). cargo-deny's `unmaintained = "all"` default will fail `cargo deny check advisories` until you add `ignore = [{ id = "RUSTSEC-2017-0008", reason = "pulled by portable-pty =0.8.1; serial ports never opened" }]`. Re-check that ignore whenever the pin moves.
  - Source review of 0.8.1: `CreateProcessW` is called with `bInheritHandles = 0`, so viola's pipe and listener handles do not leak into the `claude` child. Keep this property on any swap, and keep interprocess listeners `inheritable(false)` (the default).
  - 0.8.1 builds the Windows command line itself and does PATHEXT search (`cmdbuilder.rs`). A `.cmd` target runs through `cmd.exe` without std's BatBadBut escaping, so resolve to a real `.exe` before spawning.
  - **Bracketed-paste breakout:** neither portable-pty nor vt100 sanitises written input. A bridge send is `ESC[200~…ESC[201~` + CR, so an embedded `ESC[201~` ends the paste early, and the remainder reaches the `claude` TUI as typed keys, including keys that could confirm a dialog. There is no 2025–2026 CVE for this class, but the terminator is plain in-band data.
    - viola-core should reject `send.text` containing `ESC` (0x1B), the C1 CSI (0x9B) or other C0 controls except `\n`, `\t` and `\r`. Reject before pasting (a typed refusal detail) rather than strip, so the delivery-confirmation text match stays exact.
    - Apply the same check to `answer` `message` and free-text answers.
- **Source:** https://crates.io/crates/portable-pty ; https://rustsec.org/advisories/RUSTSEC-2017-0008.html ; https://lib.rs/crates/portable-pty-psmux ; https://en.wikipedia.org/wiki/Bracketed-paste ; https://github.com/psmux/psmux/issues/684

### windows-sys (TerminateProcess fallback; recommended also for pipe DACL and SQOS)

- **Version:** 0.61.2
- **Last release:** 2025-10-06
- **Status:** actively maintained
- **Fits because:** It is already in the stack. It is the dependency-free way to build the per-user named-pipe DACL and the client-side impersonation limit (Threat Assessment §6 "IPC endpoint access control and server impersonation").
- **Key detail:**
  - No advisory.
  - Use `GetTokenInformation(TokenUser)` and `ConvertSidToStringSidW` to get the current user's SID string for the pipe SDDL.
  - Use `CreateFileW(..., SECURITY_SQOS_PRESENT | SECURITY_IDENTIFICATION | FILE_FLAG_OVERLAPPED, ...)` for the client open (see the interprocess block).
  - These are pure bindings with no C build, consistent with the cargo-deny `cc` ban.
  - `windows-permissions` was rejected: last release 0.2.4 on 2021-06-29. Use windows-sys plus interprocess's `SecurityDescriptor` instead.
- **Source:** https://crates.io/crates/windows-sys ; https://learn.microsoft.com/en-us/windows/win32/ipc/named-pipe-security-and-access-rights ; https://crates.io/crates/windows-permissions

### vt100 (screen model, readiness gate)

- **Version:** 0.16.2
- **Last release:** 2025-07-12
- **Status:** actively maintained
- **Fits because:** It parses every byte the `claude` child writes, which is untrusted upstream output (Threat Assessment §2, Cross-cutting "Untrusted upstream text").
- **Key detail:**
  - No advisory (OSV). Activity is low: doy/vt100-rust is not archived, and its last push and release were 2025-07-12.
  - A parser panic would kill `run`'s pump thread, which is also the human's terminal path. Wrap parser feeding in `std::panic::catch_unwind`. On a panic, degrade the gate to "not ready" (`input-not-ready`) and keep passing bytes through. It must never block the human.
  - vt100 is used for signatures only, never to read content, so it has no content-injection role.
- **Source:** https://github.com/doy/vt100-rust ; https://crates.io/crates/vt100

### interprocess (`local_socket`, sync API; Tokio flavour in `viola-mcp`)

- **Version:** 2.4.4
- **Last release:** 2026-09-03
- **Status:** actively maintained
- **Fits because:** It implements the highest-impact surface. The per-instance endpoint is effectively a code-execution interface (`send`, `answer` allow), with no decided per-OS enforcement and predictable names (Threat Assessment §2, §3, §6).
- **Key detail:** No advisory (OSV); previous releases were 2026-08-01 and 2026-04-19; single main maintainer (per arch). Findings from the 2.4.4 source review:
  - **Windows server defaults:**
    - `accept_remote` defaults to `false`, which sets `PIPE_REJECT_REMOTE_CLIENTS`.
    - The first instance is created with `FILE_FLAG_FIRST_PIPE_INSTANCE`, so a taken name fails the bind, which is the arch's exit-1 refusal.
    - With no `security_descriptor`, the pipe gets the Windows default DACL: "full control to the LocalSystem account, administrators, and the creator owner … read access to members of the Everyone group and the anonymous account." Fix this with `interprocess::os::windows::local_socket::ListenerOptionsExt::security_descriptor(SecurityDescriptor::deserialize(sddl))` and an SDDL such as `D:P(A;;GA;;;<current-user-SID>)(A;;GA;;;SY)`. Optionally add the logon SID to shut out the same user's other sessions.
  - **Windows client, no impersonation limit:** `connect_without_waiting` calls `CreateFileW` with `FILE_FLAG_OVERLAPPED` only, without `SECURITY_SQOS_PRESENT | SECURITY_IDENTIFICATION`. A squatting server that holds SeImpersonatePrivilege (a service account) can impersonate any connecting viola client (`hook`, `send`, `mcp`).
    - Mitigation: open the pipe via windows-sys with SQOS Identification, then adopt the handle with `interprocess::os::windows::named_pipe::local_socket::Stream::try_from(OwnedHandle)`. `TryFrom<OwnedHandle>` exists in 2.4.4; the overlapped flag requirement must be verified in a spike.
    - After connecting, check `server_process_id()` against the snapshot's wrapper `pid` (plus the sysinfo start time) before sending any `hook.dialog` or `send` payload.
  - **Unix, socket file mode:** `ListenerOptionsExt::mode(0o600)` does `fchmod()` before `bind()`, so there is no umask race (the racy fallback was removed in 2.3.0). The docs list only Linux, FreeBSD 14.3+ and OpenBSD as supported, so macOS returns `Unsupported`. `unix(7)` also warns that on some systems socket permissions are ignored and "portable programs should not rely on this feature for security."
    - Primary control: put the socket in a per-user directory with mode 0700, verified with `lstat` (owner == euid, mode & 0o077 == 0, not a symlink). Use `$XDG_RUNTIME_DIR/viola/` on Linux, `$TMPDIR/viola/` on macOS (per-user `/var/folders/...`), and `/tmp/viola-<uid>/` only as a fallback.
    - Never use `/tmp/viola-<h12>.sock` directly. The Kea DHCP 2025 CVEs (CVE-2025-32801/32802/32803) are the precedent: any local user could pre-create sockets or lock files in `/tmp`.
    - Keep `try_overwrite` off. Its docs describe a TOCTOU in which interprocess can delete an attacker-swapped file.
  - **Unix peer check:** `Stream::peer_creds()` gives `euid()` on Linux and macOS and `pid()` on Linux and FreeBSD. The server rejects `euid != geteuid()`. The client verifies the server's euid, plus the pid against the snapshot where available, before sending.
- **Source:** https://docs.rs/interprocess/latest/interprocess/local_socket/struct.ListenerOptions.html ; https://docs.rs/interprocess/latest/x86_64-pc-windows-msvc/interprocess/os/windows/named_pipe/struct.PipeStream.html ; https://learn.microsoft.com/en-us/windows/win32/ipc/named-pipe-security-and-access-rights ; https://man7.org/linux/man-pages/man7/unix.7.html ; https://security.opensuse.org/2025/05/28/kea-dhcp-security-issues.html ; https://crates.io/crates/interprocess

### atomic-write-file (Database: none — ndjson logs + atomic snapshots + std `File::lock`)

- **Version:** 0.3.1
- **Last release:** 2026-08-11
- **Status:** actively maintained
- **Fits because:** `~/.viola/` holds persistent plaintext prompts and tool `input`, plus code-bearing state: `statusline_command`, `hooks.json` exec paths, pinned binaries and ledger stamps (Threat Assessment §1, §2 Filesystem, §6 "Integrity of `~/.viola/`"). "Database security hardening" therefore becomes file-permission hardening.
- **Key detail:**
  - No advisory for atomic-write-file or its drop-in alternative tempfile 3.27.0 (2026-03-11).
  - No DB engine means no SQLi, connection-string or DB-auth concerns. The controls are OS permissions.
  - Unix: create `~/.viola` (and any `--home`) with mode 0700 and files with `OpenOptionsExt::mode(0o600)`. Check that the temp file atomic-write-file creates ends up 0600 before rename. At startup, refuse (ssh StrictModes style) when the home or `instances/<name>/` is group- or world-writable or not owned by euid. `hook statusline` executes a string from `snapshot.json`, so write access to it equals code execution.
  - Windows: `%USERPROFILE%` inherits a user + SYSTEM + Administrators ACL. A `--home` outside the profile inherits its parent's ACL, so check or set an explicit DACL there.
  - Cap ndjson line length on replay and tail so a corrupted log cannot exhaust memory.
- **Source:** https://crates.io/crates/atomic-write-file ; https://crates.io/crates/tempfile

### serde

- **Version:** 1.0.229
- **Last release:** 2026-07-18
- **Status:** actively maintained
- **Fits because:** It sits behind every external payload: hook stdin, `claude agents --json`, statusline, channel frames and `answer` `response` JSON (Threat Assessment §2 hook stdin, IPC, CLI).
- **Key detail:** No advisory (OSV). Tolerant parsing (no `deny_unknown_fields`) is fine for drift, but security-relevant fields (`behavior`, `dialog_id`, wheel holder) must stay closed enums or integers, never free strings.
- **Source:** https://crates.io/crates/serde

### serde_json

- **Version:** 1.0.151
- **Last release:** 2026-07-20
- **Status:** actively maintained
- **Fits because:** It is the concrete parser for channel ndjson frames, hook stdin and snapshots (Threat Assessment §2 IPC, hook stdin, Filesystem).
- **Key detail:**
  - No advisory (OSV).
  - The default recursion limit (128) protects against stack-exhaustion DoS from nested hook or channel JSON. Do not enable `unbounded_depth`.
  - serde_json does not bound input size, and `read_line` on a channel stream and `read_to_end` on hook stdin are unbounded. Wrap readers in `Read::take(MAX_FRAME)` and refuse with `-32600` over the limit. Otherwise any same-user local peer can make a wrapper allocate without limit.
- **Source:** https://crates.io/crates/serde_json

### serde_path_to_error

- **Version:** 0.1.20
- **Last release:** 2025-09-15
- **Status:** actively maintained
- **Fits because:** It produces drift reports for external payloads (Threat Assessment §2 hook stdin).
- **Key detail:** No advisory (OSV). Drift reports quote JSON paths and possibly values from upstream payloads, which can include tool `input`. Write them only to `diagnostics/` (0600) and never echo them into GUI Problem Details or MCP errors.
- **Source:** https://crates.io/crates/serde_path_to_error

### nutype (`ViolaName`, `Percent`)

- **Version:** 0.8.0
- **Last release:** 2026-09-20
- **Status:** actively maintained
- **Fits because:** `ViolaName` (`[a-z0-9-]`, 1–32 characters, starting with a letter) is the only barrier between attacker-influenced names and filesystem paths. The threat assessment flags that the SSE `Last-Event-ID` cursor is not stated to be parsed through the newtype (Threat Assessment §2 Loopback HTTP).
- **Key detail:**
  - No advisory.
  - Parse every `<name>:<offset>` pair of `Last-Event-ID` with `ViolaName::try_new` and `u64::from_str` before any `instances/<name>` join. Drop the whole header on any failure; do not skip the bad pair. Apply the same rule to MCP `target`, CLI arguments, and channel `link`/`unlink` `driver`/`driven` params.
  - The charset excludes `.`, `/`, `\` and `:`, which rules out traversal and Windows ADS/device names by construction.
- **Source:** https://crates.io/crates/nutype

### chrono

- **Version:** 0.4.45
- **Last release:** 2026-06-04
- **Status:** actively maintained
- **Fits because:** It parses the external `resets_at` from the statusline (Threat Assessment §2 hook stdin).
- **Key detail:** No advisory (OSV). The arch's tolerant parse (a malformed value becomes `"unknown"`) is the correct DoS posture. Keep the hook path UTC-only and avoid local-timezone APIs.
- **Source:** https://crates.io/crates/chrono

### thiserror

- **Version:** 2.0.20
- **Last release:** 2026-08-08
- **Status:** actively maintained
- **Fits because:** Minimal-tier baseline item "error sanitization": typed errors cross the channel, CLI `--json`, MCP and GUI Problem Details, all read by other local processes and LLM drivers.
- **Key detail:** No advisory. `Display` impls on crate errors must not interpolate upstream text or full paths, because these strings reach MCP `structuredContent` and Problem Details `detail`.
- **Source:** https://crates.io/crates/thiserror

### anyhow

- **Version:** 1.0.104
- **Last release:** 2026-07-18
- **Status:** actively maintained
- **Fits because:** Error context chains at the bin edge (Threat Assessment §6 baseline "error sanitization").
- **Key detail:** No advisory. Keep anyhow chains out of `application/problem+json` and MCP results. They can embed absolute paths, and `/api/info`'s `viola_home` (which contains the OS username) should be the only intended path disclosure. `hook` never writes to stderr, which is correct.
- **Source:** https://crates.io/crates/anyhow

### notify (log tailing for SSE)

- **Version:** 8.2.0
- **Last release:** 2025-08-03
- **Status:** actively maintained
- **Fits because:** It feeds the SSE feed that carries prompts and permission `input` (Threat Assessment §1, §2).
- **Key detail:** No advisory (OSV). The crate was updated 2026-08-30 with 9.0.0-rc.5, a pre-release; stay on 8.2.0. Tail only paths derived from validated `ViolaName`s under the resolved home, and ignore symlinks and names that fail `ViolaName` parsing. Otherwise a planted `instances/<junk>` directory becomes an SSE source.
- **Source:** https://docs.rs/crate/notify/latest ; https://crates.io/crates/notify

### sysinfo (pid + start-time liveness)

- **Version:** 0.39.6
- **Last release:** 2026-07-09
- **Status:** actively maintained
- **Fits because:** The pid + start-time check is the primitive for verifying that the process answering on an endpoint is the wrapper recorded in `snapshot.json` (Threat Assessment §2 "a process holding that name while the real wrapper is gone").
- **Key detail:** No advisory. Combine it with interprocess `server_process_id()` or `peer_creds().pid()` to close the pid-reuse race interprocess's own docs warn about.
- **Source:** https://crates.io/crates/sysinfo

### rmcp (features `server`, `transport-io`)

- **Version:** 3.4.1
- **Last release:** 2026-09-23
- **Status:** actively maintained
- **Fits because:** It is the MCP surface through which an LLM driver can `answer` PermissionRequests (Threat Assessment §2 MCP).
- **Key detail:** rmcp had five 2026 advisories, all in features viola does not enable:
  - GHSA-89vp-x53w-74fx / RUSTSEC-2026-0189: Streamable HTTP DNS rebinding; fixed in 1.4.0.
  - GHSA-9pj6-vhgr-3mwh: Streamable HTTP session-table leak; fixed in 2.0.0.
  - GHSA-33f5-2c5q-wgwj / CVE-2026-63127: OAuth resource metadata validation, CVSS 8.2; fixed in 2.0.0.
  - GHSA-9g45-5xwm-f3wc / CVE-2026-64684: custom headers leaked on cross-origin redirect; fixed in 2.1.0.
  - OAuth metadata SSRF, per the source's reading of an rmcp 1.5 → 2.0.0 upgrade PR. No separate GHSA or fix version was confirmed.

  3.4.1 is past every fix (OSV: none). Keep the features exactly `server`, `transport-io`, and add a cargo-deny `[bans]` feature check so `transport-streamable-http-server` and `auth` are never enabled. Because the minor is pinned (`~3.4`), a scheduled advisory run matters more than bumping.
- **Source:** https://rustsec.org/advisories/RUSTSEC-2026-0189.html ; https://advisories.gitlab.com/cargo/rmcp/CVE-2026-63127/ ; https://www.strix.ai/cve/CVE-2026-64684 ; https://github.com/modelcontextprotocol/rust-sdk

### schemars

- **Version:** 1.2.2
- **Last release:** 2026-07-27
- **Status:** actively maintained
- **Fits because:** It generates the MCP tool input schemas, the first validation layer for driver-supplied `target`, `text`, `dialog_id` and `response` (Threat Assessment §2 MCP).
- **Key detail:** No advisory (OSV). The schema is advisory to the client and is not enforced server-side by rmcp. viola must still deserialise into the `ViolaName` newtype and closed enums, and apply the control-character check to `text`.
- **Source:** https://crates.io/crates/schemars

### bytes (transitive via axum/hyper)

- **Version:** 1.12.1
- **Last release:** 2026-07-08
- **Status:** actively maintained
- **Fits because:** It sits under the GUI's HTTP and SSE bodies (Threat Assessment §2 Loopback HTTP).
- **Key detail:** RUSTSEC-2026-0007 (2026-02-03): integer overflow in `BytesMut::reserve` in bytes 1.2.1 up to (not including) 1.11.1. `Cargo.lock` must resolve ≥ 1.11.1; cargo-deny advisories enforces this.
- **Source:** https://osv.dev/vulnerability/RUSTSEC-2026-0007

### tracing-subscriber (conditional: only if obs selects it)

- **Version:** 0.3.23
- **Last release:** 2026-03-13
- **Status:** actively maintained
- **Fits because:** Diagnostics, `wait` and `last` print untrusted assistant and prompt text (Threat Assessment §1, Cross-cutting "Untrusted upstream text").
- **Key detail:**
  - RUSTSEC-2025-0055 (2025-09-02): versions below 0.3.20 did not escape ANSI escape sequences in logged user input, so the floor is 0.3.20.
  - The same class applies to viola's own human-mode CLI output of `last`/`wait`: strip or escape C0/C1 controls (keep `\n`, `\t`) before writing to a terminal. `--json` output is already JSON-escaped.
- **Source:** https://rustsec.org/advisories/RUSTSEC-2025-0055.html

### dtolnay/rust-toolchain (GitHub Action, `@stable`)

- **Version:** stable@6bed0761d98439e5a578e2877258200ad565ba87
- **Last release:** 2026-09-03
- **Status:** actively maintained
- **Fits because:** It is a third-party action executing in CI, part of the supply-chain vector (Threat Assessment §2 Supply chain).
- **Key detail:** The action publishes no versioned releases; its only GitHub release is `v1` from 2022. It is consumed by branch ref, and the `stable` ref's last commit is dated 2026-09-03. Replace `@stable` with the full commit SHA plus a `# stable` comment, and set `permissions: {}` at workflow level with `contents: read` per job. GitHub's Actions policy (2025-08-15) can enforce SHA pinning.
- **Source:** https://github.com/dtolnay/rust-toolchain ; https://github.blog/changelog/2025-08-15-github-actions-policy-now-supports-blocking-and-sha-pinning-actions/

### Swatinem/rust-cache (GitHub Action)

- **Version:** v2.9.2 (6323deb102c322ba6fcbdcafc7e3dddab59af2b6)
- **Last release:** 2026-08-06
- **Status:** actively maintained
- **Fits because:** It is a third-party action that restores build state into CI (Threat Assessment §2 Supply chain).
- **Key detail:** Pin to SHA `6323deb102c322ba6fcbdcafc7e3dddab59af2b6` with a `# v2.9.2` comment. v1 has no secrets or release jobs, so cache poisoning has low impact now. The v1.x dist release workflow must not restore this cache.
- **Source:** https://github.com/Swatinem/rust-cache ; https://www.wiz.io/blog/github-actions-security-guide

### dist (cargo-dist) — v1.x release

- **Version:** >=0.33.0
- **Last release:** 2026-09-11
- **Status:** actively maintained
- **Fits because:** The future public distribution is the code-integrity chain for the pinned `bin/<version>-<hash>/viola(.exe)` that Claude Code runs on every hook (Threat Assessment §2 Supply chain, Filesystem code-bearing artefacts).
- **Key detail:** 0.33.0 is published on GitHub; crates.io's `cargo-dist` still shows 0.32.0. It supports GitHub Artifact Attestations (`gh attestation verify`) and Windows codesigning through Azure Artifact Signing (x86_64 only). Enable both when distribution goes public.
- **Source:** https://axodotdev.github.io/cargo-dist/book/supplychain-security/attestations/github.html

### cargo-auditable — v1.x release

- **Version:** >=0.7.6
- **Last release:** 2026-09-13
- **Status:** actively maintained
- **Fits because:** It embeds the dependency list in shipped binaries so they can be scanned after release (Threat Assessment §2 Supply chain).
- **Key detail:** Pairs with `cargo audit bin` (see the cargo-audit block) to re-scan installed viola binaries against new RustSec advisories.
- **Source:** https://crates.io/crates/cargo-auditable

### self_update — v1.x (later)

- **Version:** 1.3.0
- **Last release:** 2026-09-02
- **Status:** actively maintained
- **Fits because:** It will replace the binary that hooks execute (Threat Assessment §2 Supply chain).
- **Key detail:** No advisory (OSV). Build it with the `signatures` feature (zipsign verification of `.zip`/`.tar.gz`), not checksums alone. A checksum fetched from the same release as the binary proves no authenticity.
- **Source:** https://lib.rs/crates/self_update ; https://docs.rs/self_update/latest/self_update/

## Dependency Audit Tools

### cargo-deny (advisories + bans + licences + sources)

- **Version:** >=0.20.2
- **Last release:** 2026-07-09
- **Status:** actively maintained
- **Fits because:** It is already in the stack. Adding `[advisories]` and `[sources]` covers the Minimal-tier dependency-audit baseline and the rmcp, bytes and serial findings above (Threat Assessment §6 baseline, §2 Supply chain).
- **Key detail:**
  - Arch lists only licences and bans. Add:
    - `[advisories] unmaintained = "all"` (the default), `unsound = "all"`, `yanked = "deny"`.
    - `ignore = [{ id = "RUSTSEC-2017-0008", reason = "..." }]`, the only expected ignore, for portable-pty's `serial`.
    - `[sources] unknown-registry = "deny"`, `unknown-git = "deny"`.
    - A `[bans]` feature restriction on rmcp.
  - The advisory DB changes without code changes, so also run `cargo deny check advisories` on a weekly `schedule:` cron. The default `maximum-db-staleness` is P90D.
- **Source:** https://embarkstudios.github.io/cargo-deny/checks/advisories/cfg.html ; https://embarkstudios.github.io/cargo-deny/checks/sources/cfg.html

### cargo-audit (v1.x binary scan)

- **Version:** >=0.22.2
- **Last release:** 2026-06-05
- **Status:** actively maintained
- **Fits because:** It uses the same RustSec DB. `cargo audit bin` scans cargo-auditable release binaries in v1.x (Threat Assessment §2 Supply chain).
- **Key detail:** It is redundant with cargo-deny's advisories check in v1. Do not gate on both against the same `Cargo.lock`; keep it for the release-artifact scan.
- **Source:** https://github.com/rustsec/rustsec ; https://rustsec.org/advisories/

### zizmor (GitHub Actions workflow audit)

- **Version:** >=1.30.1
- **Last release:** 2026-09-09
- **Status:** actively maintained
- **Fits because:** CI is the only automated build path, and third-party actions are in `ci.yml` (Threat Assessment §2 Supply chain, §4 CI/CD).
- **Key detail:** Run `zizmor .github/workflows/` on the ubuntu leg. It flags unpinned actions, `excessive-permissions`, template injection and cache-poisoning (relevant once the dist release workflow lands). Online audits with `GH_TOKEN` add known-vulnerable-actions and impostor-commit checks.
- **Source:** https://crates.io/crates/zizmor ; https://www.wiz.io/blog/github-actions-security-guide

## Framework Security Features

### axum 0.8.9: what it gives vs. what viola must add

- **Version:** 0.8.9
- **Last release:** 2026-04-14
- **Status:** actively maintained
- **Fits because:** The view-only GUI serves persistent conversation content on a loopback port that other OS users can reach (Threat Assessment §2 Loopback HTTP, §3 "GUI currently has no principal check").
- **Key detail:**
  - **Given for free:**
    - `MethodRouter` answers 405 for unrouted methods.
    - No CORS headers by default, so cross-origin reads are blocked by the browser.
    - No template engine, so no server-side HTML injection.
    - `Sse` with `KeepAlive`.
  - **Must add:**
    1. A Host allowlist middleware on all routes.
    2. The security headers, CSP and frontend encoding constraints in the tower-http block.
    3. A per-launch secret for reads as well as for the v1.x brake, because loopback TCP does not enforce the same-OS-user boundary. The pattern:
       - Generate 32 bytes with `getrandom` 0.4.3 (2026-06-17) at `viola ui` start.
       - Write `http://127.0.0.1:<port>/?t=<token>` to a 0600 file under the viola home, and print it.
       - Exchange the token once for a `Set-Cookie: viola_<port>=…; HttpOnly; SameSite=Strict; Path=/`, then redirect so the token leaves the URL.
       - Check the cookie on `/api/*` and SSE (`EventSource` sends cookies same-origin).
       - Compare with `constant_time_eq` 0.6.0 (2026-08-30).
       - `subtle` was rejected: last release 2.6.1 on 2024-06-24.
    4. For the v1.x POST brake: require the same cookie and additionally reject unless `Sec-Fetch-Site` is `same-origin` or `none`, or `Origin` matches the allowlist. This is the check Go 1.25's `CrossOriginProtection` standardised.
  - **Rejected tools:**
    - `tower-csrf` 0.1.0, the Rust port of that check: GitHub repo archived; implement the ~20-line check in viola-ui instead.
    - `axum_csrf`: token/cookie CSRF, not needed with SameSite=Strict + Sec-Fetch-Site.
  - Chrome 142's Local Network Access prompt adds a browser-side barrier for public-site → loopback requests. It is defence in depth only.
- **Source:** https://www.calhoun.io/csrf-protection-via-headers-in-go-125/ ; https://github.com/yawn/tower-csrf ; https://developer.chrome.com/blog/local-network-access ; https://cheatsheetseries.owasp.org/cheatsheets/Cross-Site_Request_Forgery_Prevention_Cheat_Sheet.html ; https://crates.io/crates/getrandom ; https://crates.io/crates/constant_time_eq ; https://crates.io/crates/subtle

### interprocess 2.4.4: OS-level access control primitives

- **Version:** 2.4.4
- **Last release:** 2026-09-03
- **Status:** actively maintained
- **Fits because:** It supplies the "same OS user and no other" enforcement that the arch handed to security (Threat Assessment §3).
- **Key detail:**
  - Built-in primitives:
    - Windows: `PipeListenerOptions`/`ListenerOptionsExt::security_descriptor`, `SecurityDescriptor::deserialize` (SDDL), `accept_remote(false)` (the default), the first-instance flag (always set), `instance_limit`, `inheritable`, and `PipeStream::{client,server}_process_id`, `impersonate_client`.
    - Unix: `ListenerOptionsExt::mode` (Linux/FreeBSD/OpenBSD only, not macOS) and `Stream::peer_creds()` with `euid()` and `pid()`.
  - Missing, so viola must add them:
    - a client-side SQOS impersonation limit (Windows);
    - a macOS socket mode (use a 0700 directory instead);
    - a server-identity check on the client (compare the server pid or euid with the snapshot).
  - Same-user trust stays ambient per v1. The optional per-connection proof for a driver-originated `release` can reuse the per-launch-secret pattern: a 0600 file in the instance directory, read by clients and sent in `params`.
- **Source:** https://docs.rs/interprocess/latest/interprocess/local_socket/struct.ListenerOptions.html ; https://docs.rs/interprocess/latest/x86_64-pc-windows-msvc/interprocess/os/windows/named_pipe/struct.PipeListenerOptions.html ; https://learn.microsoft.com/en-us/windows/win32/ipc/named-pipe-security-and-access-rights
