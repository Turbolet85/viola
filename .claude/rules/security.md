# Security Rules

Universal security requirements for viola. Apply to all files. This rule file has no `paths:` frontmatter — it loads unconditionally, so it stays lean; the full contract is `.andromeda/security-plan.md` (tier Minimal with local-boundary elevations).

## Secrets and the NEVER-log floor
- Secrets in v1: the GUI per-launch token (32 `getrandom` bytes), the launch URL, the `viola_<port>` cookie, inherited `CLAUDE_CODE_MESSAGING_TOKEN` / `_SOCKET` and every R8-stripped `CLAUDE*` value. None may reach a log, diagnostic, event, snapshot, fixture or error body. The one exception: the launch line `viola ui` prints once to stderr and writes to the 0600 `ui/<port>.url`.
- HTTP logging records `uri.path()` only (never the `?t=` query) and never the `Cookie` header.
- Never commit `.env*`, local `--home` test directories, tokens, cookies or signing material; never hardcode a credential in source or in the `include_str!` plugin files.
- Compare the token/cookie only with `constant_time_eq`, never `==`; issue the cookie `HttpOnly; SameSite=Strict; Path=/`, 303-redirect away from `?t=` at once, send `Referrer-Policy: no-referrer`.

## Local trust boundary (IPC + home)
- Windows pipe listener: explicit protected SDDL `D:P(A;;GA;;;<user-SID>)(A;;GA;;;SY)`, `accept_remote(false)`, first-instance, non-inheritable. Client opens with `SECURITY_SQOS_PRESENT | SECURITY_IDENTIFICATION` — never interprocess's default connect.
- Unix socket: a verified 0700 per-user dir (`$XDG_RUNTIME_DIR/viola/`, `$TMPDIR/viola/`, `/tmp/viola-<uid>/`), never `/tmp` root, never abstract namespace, `mode(0o600)` as a secondary control only, `peer_creds` euid on both sides, `try_overwrite` off.
- Before the FIRST frame of any method (including `hook.event`), verify the server: pid (+ sysinfo start time) against a `snapshot.json` that passed strict-modes in the same process (macOS: euid + dir only, recorded gap).
- Home: 0700 dirs, 0600 files via `OpenOptionsExt::mode` (pinned `bin/<version>-<hash>/viola` is 0700); set the atomic-write temp file's mode before `commit()`. Strict-modes check (Unix mode/owner; Windows owner + DACL incl. inherit-only and generic bits, NULL DACL, non-persistent-ACL volume) at every entry point that reads `snapshot.json`, `ledger/stamps.json` or `statusline_command`.
- Every exec-form `command` in `hooks.json`, `.mcp.json`, `viola plugin install`'s `.mcp.json` and the `settings.json` statusline is the absolute pinned path — never a PATH lookup. Rewrite `plugin/` and `settings.json` atomically on every start; re-hash (truncated SHA-256) a pinned exe before reuse; never FNV/`DefaultHasher` for it.
- Never spawn a `.cmd`/`.bat` PTY child (portable-pty bypasses BatBadBut escaping) — resolve to the real `.exe`.
- A non-`null` dialog decision requires a `viola verify` stamp for the child's CLI version; `release` carrying `from` → `-32602 release-from-driver`; `from` is self-reported, never identity.

## Input validation
- Shared validators live in `viola-core`; the wrapper re-runs them (client checks and schemars schemas are advisory).
- `validate_paste_text` over decoded `char`s: allow LF/CR/TAB, refuse every other C0, DEL and C1 with `not-delivered` / `control-character` — reject, never strip.
- `Read::take(MAX_FRAME)` (16 MiB) before `read_line` / `read_to_end` on every external reader; serde_json default depth (never `unbounded_depth`).
- `Last-Event-ID`: parse every pair with `ViolaName::try_new` + `u64`; any bad pair drops the whole header.

## Dependencies and CI
- **Audit tool:** `cargo deny check` (>=0.20.2: advisories, bans, licences, sources — crates.io only; the only ignore is RUSTSEC-2017-0008 via portable-pty `=0.8.1`); the tokio ban is `deny-sync.toml`, run per sync crate as sole root; `scripts/deny-probes.sh` proves every ban fires; weekly `cargo deny check advisories` in `nightly.yml`; `zizmor` on workflows. Never widen an ignore, add a `skip`/`allow` or silence zizmor to reach green.
- `Cargo.lock` committed; `[workspace.dependencies]` pins versions (rmcp `>=3.4.1, <3.5`); rmcp features exactly `server` + `transport-io`; axum without `http2`.
- Actions pinned by full commit SHA; workflow `permissions: {}`, job `contents: read`; no `rust-cache` in a release workflow.

## Error handling
- thiserror `Display` impls use fixed messages (no paths, no payloads); channel `-32603` is exactly `"internal error"` / `data: null`; Problem Details `detail` strings are fixed and never name a path.
- Full detail goes only to `instances/<name>/diagnostics/detail-*.ndjson` (0600) when an instance resolves.

## Session Additions
_This section is owned by `/wrap-session`. setup-project preserves content added here on re-run. See `section-markers.md` for the convention._
