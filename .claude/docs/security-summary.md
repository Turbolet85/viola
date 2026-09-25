# Security Summary — viola

_Distilled from `.andromeda/security-plan.md` by `/andromeda-setup-project`. wrap-session does not modify._

## Posture

viola is a local-only, single-user tool: no accounts, no public listener, no database, no stored credentials and no regulated data. Its risk is the local privilege boundary. The IPC endpoint is effectively a code-execution interface (`send` types prompts, `answer` can allow PermissionRequest tool calls). The loopback GUI streams persistent prompt and tool content. Several state files decide what code runs (`statusline_command`, the plugin exec paths, the pinned binaries, the ledger stamps). The plan is a Minimal baseline with targeted elevations for each of these.

**Tier:** Minimal (0), with targeted elevations for the local privilege boundary
**Auth approach:** OS-user ambient trust enforced per OS (IPC endpoints + `~/.viola/`), plus a per-launch 256-bit GUI token exchanged once for a `SameSite=Strict` HttpOnly cookie (v1 reads and v1.x brake writes)

## Threat model highlights

- **A foreign OS user connects to the IPC endpoint** → Windows protected SDDL (user SID + SYSTEM), `accept_remote(false)`; Unix 0700 per-user socket dir + `mode(0o600)` + `peer_creds` euid check.
- **A squatter impersonates the wrapper** → the client verifies the server's pid (+ start time) against a strict-modes-checked `snapshot.json` before the first frame; Windows clients open with SQOS Identification. On macOS it is euid + dir only (a recorded known gap). A squatted name blocks `run` (DoS accepted, no takeover).
- **Bracketed-paste breakout** (`ESC[201~` inside `send.text`) → `validate_paste_text` rejects every C0 except LF/CR/TAB, DEL and C1, client- and wrapper-side.
- **Another local user reads the feed over loopback TCP** → cookie gate on `/api/*` + SSE; Host allowlist first (DNS rebinding); no CORS; CSP + Trusted Types; text-only rendering.
- **Tampered code-bearing state** → strict-modes check (Unix mode/owner; Windows owner + DACL) at every entry point that reads a trusted file; pinned exe re-hashed (truncated SHA-256) before reuse; plugin files and `settings.json` rewritten each start; single writers for snapshot and stamps.
- **Planted `claude` on PATH** (for `claude agents --json`) → accepted: it can inject display rows only; those rows are never a `ViolaName`/`target` and render as text.
- **Driver LLM prompt-injected into `answer allow`** → accepted: policy is outside viola (R1/R4); viola offers only a closed `behavior` enum and the `unverified-cli` gate.

## Data classifications

| Class | Examples | Handling |
|---|---|---|
| Critical (secret) | GUI token, launch URL, `viola_<port>` cookie, `CLAUDE_CODE_MESSAGING_TOKEN`/`_SOCKET`, other R8-stripped `CLAUDE*` | Never logged anywhere; token only in process memory + the 0600 `ui/<port>.url` + one stderr launch line; rotated every launch |
| High (user content) | prompts, `last_assistant_message`, tool `input`, plans, dialog answers, drift reports | Only in contract payloads (`events.ndjson`, SSE, `wait`/`last`, `hook.dialog`) and `instances/<name>/diagnostics/detail-*`; never in stderr, `--json` errors, MCP errors, Problem Details |
| Integrity-sensitive config | `snapshot.json` (`endpoint`, `pinned_bin`, `statusline_command`), `ledger/stamps.json`, `plugin/*/hooks.json`, pinned `bin/` | 0600/0700 + strict-modes check; single writers; absolute exec paths |
| Low (operational) | pids, `agent_session_id`, budget percentages, `bind`, OS username in paths | Typed fields in diagnostics; paths never in external errors (only the cookie-gated `/api/info` `viola_home`) |

## Universal anti-patterns

- NEVER add a listener, transport or channel method without a Security Decisions Log entry.
- NEVER let a security refusal block the human — refusals go only to automation, hooks fail open.
- NEVER assume loopback TCP is same-user: the cookie check is mandatory on `/api/*` and SSE.
- NEVER let an exec-form command viola writes resolve `viola` through PATH — absolute pinned path only.
- NEVER answer `hook.dialog` non-`null` without a `viola verify` stamp; only `viola verify` writes the stamps.
- NEVER let a process other than the instance's wrapper write its `snapshot.json`.
- NEVER let `config.json`, a `VIOLA_*` env var or a flag switch off a control.

## Arch amendments this plan requires (routed through wrap reconciles)

1. v1 GUI cookie on `/api/*` + SSE. 2. Unix endpoint in a per-user 0700 dir. 3. `not-delivered`/`control-character` + URNs `unauthorized` / `cross-origin-forbidden`. 4. `release` with `from` → `-32602 release-from-driver`. 5. `<home>/ui/<port>.url` (0600). 6. `MAX_FRAME` = 16 MiB in `viola-core`. 7. Ledger row "largest hook payload seen". 8. `<hash>` = truncated SHA-256 (16 hex).

## Resolved prerequisites (Decisions Log `2026-09-25`)

- **SQOS spike: passed.** `CreateFileW(SECURITY_SQOS_PRESENT | SECURITY_IDENTIFICATION | FILE_FLAG_OVERLAPPED)` adopted by interprocess 2.4.4 `Stream::try_from` leaves the server at `SecurityIdentification` (measured; `tests/channel_sqos_open.rs` pins it). `FILE_FLAG_OVERLAPPED` is required (non-overlapped adoption hangs); never fall back to the default connect (it reads `SecurityImpersonation`).
- **SHA-256 crate: `sha2 =0.11.0`** (`default-features = false`, pure Rust, no `cc`); `<hash>` = the first 16 hex; `tests/contract_content_hash.rs` pins it.
- **Licences:** `0BSD` enters only per crate (`doctest-file`, `recvmsg` via interprocess) through `[[licenses.exceptions]]`, never through `allow`.

## Open questions (block specific chunks)

- `events.ndjson` retention (arch open item) — any scheme keeps 0600 and valid offsets.

## Path-scoped enforcement

For the enforcement bullets (secrets, trust boundary, validation, dependencies, error handling), see `.claude/rules/security.md` (always loaded) and `.claude/rules/api.md` (channel/MCP/CLI/GUI). This summary is on-demand reference.

## Critical decisions

- Windows pipe SDDL = user SID + SYSTEM only; the logon SID is rejected — a same-user driver in another logon session (SSH, scheduled task) must still connect.
- `validate_paste_text` allows LF/CR/TAB (multi-line relays are normal), rejects the rest — reject, never strip (stripping would break exact-match confirmation).
- `MAX_FRAME` stays 16 MiB, checked against the ledger's measured largest hook payload.
- A driver's `release` is refused as an affordance guard, not enforcement (same-user processes are trusted in v1).
- GUI cookie is per-port (`viola_<port>`) with no `Max-Age`; cross-port cookie exposure accepted as residual.

---

**Full plan:** `.andromeda/security-plan.md`.
