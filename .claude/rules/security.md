# Security Rules

Universal security requirements for viola. Apply to all files. This rule file has no `paths:` frontmatter — it loads unconditionally, so it stays lean; the full contract is `.andromeda/security-plan.md` (tier Minimal with local-boundary elevations).

## Secrets and the NEVER-log floor
- Secrets in v1: the GUI per-launch token (32 `getrandom` bytes), the launch URL, the `viola_<port>` cookie, inherited `CLAUDE_CODE_MESSAGING_TOKEN` / `_SOCKET` and every R8-stripped `CLAUDE*` value. None may reach a log, diagnostic, event, snapshot, fixture or error body. The one exception: the launch line `viola ui` prints once to stderr and writes to the 0600 `ui/<port>.url`.
- HTTP logging records `uri.path()` only (never the `?t=` query) and never the `Cookie` header.
- Never commit `.env*`, local `--home` test directories, tokens, cookies or signing material; never hardcode a credential in source or in the `include_str!` plugin files.
- Compare the token/cookie only with `constant_time_eq`, never `==`; issue the cookie `HttpOnly; SameSite=Strict; Path=/`, 303-redirect away from `?t=` at once, send `Referrer-Policy: no-referrer`.

## Local trust boundary (IPC + home)
- Windows pipe listener: explicit protected SDDL `D:P(A;;GA;;;<user-SID>)(A;;GA;;;SY)`, `accept_remote(false)`, first-instance, non-inheritable. Client opens with `SECURITY_SQOS_PRESENT | SECURITY_IDENTIFICATION` — never interprocess's default connect.
- Unix socket: a verified 0700 per-user dir (`$XDG_RUNTIME_DIR/viola/`, `$TMPDIR/viola/`, `/tmp/viola-<uid>/`), never `/tmp` root, never abstract namespace, the socket chmod 0600 after the bind as a secondary control only (never `ListenerOptionsExt::mode`: `Unsupported` on macOS fails the bind), `peer_creds` euid on both sides, `try_overwrite` off.
- Before the FIRST frame of any method (including `hook.event`), verify the server: pid (+ sysinfo start time) against a `snapshot.json` that passed strict-modes in the same process (macOS: euid + dir only, recorded gap).
- Home: 0700 dirs, 0600 files via `OpenOptionsExt::mode` (pinned `bin/<version>-<hash>/viola` is 0700); every replaced file goes through the one helper `viola_state::fs::replace_private` (tempfile `persist`), which sets the temp file's mode before any byte is written. Strict-modes check (Unix mode/owner; Windows owner + DACL incl. inherit-only and generic bits, NULL DACL, non-persistent-ACL volume) at every entry point that reads `snapshot.json`, `ledger/stamps.json` or `statusline_command`.
- Every exec-form `command` in `hooks.json`, `.mcp.json`, `viola plugin install`'s `.mcp.json` and the `settings.json` statusline is the absolute pinned path — never a PATH lookup. Rewrite `plugin/` and `settings.json` atomically on every start; re-hash (truncated SHA-256) a pinned exe before reuse; never FNV/`DefaultHasher` for it.
- Never spawn a `.cmd`/`.bat` PTY child (portable-pty bypasses BatBadBut escaping) — resolve to the real `.exe`.
- A non-`null` dialog decision requires a `viola verify` stamp for the child's CLI version; `release` carrying `from` → `-32602 release-from-driver`; `from` is self-reported, never identity.

## Input validation
- Shared validators live in `viola-core`; the wrapper re-runs them (client checks and schemars schemas are advisory).
- `validate_paste_text` over decoded `char`s: allow LF/CR/TAB, refuse every other C0, DEL and C1 with `not-delivered` / `control-character` — reject, never strip.
- `Read::take(MAX_FRAME)` (16 MiB) before `read_line` / `read_to_end` on every external reader; serde_json default depth (never `unbounded_depth`).
- `Last-Event-ID`: parse every pair with `ViolaName::try_new` + `u64`; any bad pair drops the whole header.

## Dependencies and CI
- **Audit tool:** `cargo deny check` (>=0.20.2: advisories, bans, licences — `allow` exactly MIT, Apache-2.0, Zlib, Unicode-3.0, `0BSD` only as the per-crate `[[licenses.exceptions]]` for interprocess's `doctest-file` / `recvmsg`, a new exception needs a Decisions Log entry — sources — crates.io only; the only ignore is RUSTSEC-2017-0008 via portable-pty `=0.8.1`); the tokio ban is `deny-sync.toml`, run per sync crate as sole root; `scripts/deny-probes.sh` proves every ban fires; weekly `cargo deny check advisories` in `nightly.yml` over the root and fuzz lockfiles; the test-side `e2e-web/package-lock.json` has its own audit, `scripts/npm-audit.sh` (npm advisories at every level + registry.npmjs.org-only sources; CI `supply-chain`, weekly `npm-advisories`); `zizmor` on workflows. Never widen an ignore, add a `skip`/`allow` or silence zizmor to reach green.
- `Cargo.lock` committed; `[workspace.dependencies]` pins versions (rmcp `>=3.4.1, <3.5`); rmcp features exactly `server` + `transport-io`; axum without `http2`.
- `fuzz/` is its own cargo workspace, and its `fuzz/Cargo.lock` (libfuzzer-sys builds C++) sits outside the root `cargo deny` graph by a ratified test-only exemption: never link it into `viola`, never add `fuzz` to the root `[workspace]`. Its lockfile gets its own audit, `cargo deny --manifest-path fuzz/Cargo.toml check advisories sources`, run from the repo root (CI `supply-chain`; advisories weekly).
- The release build carries `viola` only: `scripts/release-check.sh` judges the build's own artifact records (never a `target/release/` listing) and refuses any test-only binary and any artifact built with a test-only feature (`test-support`, `fake-agent`).
- Actions pinned by full commit SHA; workflow `permissions: {}`, job `contents: read`; no `rust-cache` in a release workflow.
- Node is a sha256-pinned official download (`scripts/install-node.sh`; the `NODE_PIN_*` lines in ci.yml's workflow `env:`, parsed from the file text, are its only home), never a toolchain action or runner-image Node.
- The WSL2 `pre-push` distro installs only CI's own pins (`scripts/wsl-provision.sh`: sha256-pinned rustup-init, `cargo install --locked` of ci.yml's `test`-job line, the pinned Node, the locked Playwright's Chromium; the `env -i` PATH gains only the distro-derived constant `<home>/.local/viola-node/bin`). Chromium's system libraries are the one root install, `wsl.exe -d Ubuntu -u root … wsl-provision.sh --install-deps <user home>`: OPERATOR-ONLY, never the gate tool, a harness command or a pre-push stage, because as shipped root runs user-writable code (overseer live ratification 2026-09-27); before any re-provision it must run only `apt-get install` over an allowlisted dry-run list. Every WSL call runs under `env -i` — no host value crosses (`scripts/wsl-exec.sh`, the operator-only second launcher, passes only operator-typed argv and `--cd`; no gate, harness or plan entry runs a command through it, its `--probe` excepted — overseer ratification 2026-09-27) (the Linux mutation leg's `TMPDIR=<distro home>/viola-pre-push-scratch` is a named, distro-derived constant; an assignment taking a host value is a boundary widening).
- The only env var outside `VIOLA_*` any `viola` build reads is the test seam `FAKE_AGENT_PUMP_DELAY_MS`: `cfg(feature = "fake-agent")` only, capped at 5 s, never in a release build. Another seam needs a Decisions Log entry.

## Error handling
- thiserror `Display` impls use fixed messages (no paths, no payloads); channel `-32603` is exactly `"internal error"` / `data: null`; Problem Details `detail` strings are fixed and never name a path.
- Full detail goes only to `instances/<name>/diagnostics/detail-*.ndjson` (0600) when an instance resolves.

## Session Additions
_This section is owned by `/wrap-session`. setup-project preserves content added here on re-run. See `section-markers.md` for the convention._
