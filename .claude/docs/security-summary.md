# Security Summary — viola

_Distilled from `.andromeda/security-plan.md` by `/andromeda-setup-project`. wrap-session does not modify._

## Posture

viola is a local-only, single-user tool: no accounts, no public listener, no database, no stored credentials and no regulated data. Its risk is the local privilege boundary. The IPC endpoint is effectively a code-execution interface (`send` types prompts, `answer` can allow PermissionRequest tool calls). The loopback GUI streams persistent prompt and tool content. Several state files decide what code runs (`statusline_command`, the plugin exec paths, the pinned binaries, the ledger stamps). The plan is a Minimal baseline with targeted elevations for each of these.

**Tier:** Minimal (0), with targeted elevations for the local privilege boundary
**Auth approach:** OS-user ambient trust enforced per OS (IPC endpoints + `~/.viola/`), plus a per-launch 256-bit GUI token exchanged once for a `SameSite=Strict` HttpOnly cookie (v1 reads and v1.x brake writes)

## Threat model highlights

- **A foreign OS user connects to the IPC endpoint** → Windows protected SDDL (user SID + SYSTEM), `accept_remote(false)`; Unix 0700 per-user socket dir + socket chmod 0600 after the bind + `peer_creds` euid check.
- **A squatter impersonates the wrapper** → the client verifies the server's pid (+ start time) against a strict-modes-checked `snapshot.json` before the first frame; Windows clients open with SQOS Identification. On macOS it is euid + dir only (a recorded known gap). Interim, ratified by the founder (Decisions Log `2026-09-28`): `viola hook` sends `hook.event` without server verification or strict-modes until the Epoch 6 entries land. A squatted name blocks `run` (DoS accepted, no takeover).
- **Bracketed-paste breakout** (`ESC[201~` inside `send.text`) → `validate_paste_text` rejects every C0 except LF/CR/TAB, DEL and C1, client- and wrapper-side.
- **Another local user reads the feed over loopback TCP** → cookie gate on `/api/*` + SSE; Host allowlist first (DNS rebinding); no CORS; CSP + Trusted Types; text-only rendering.
- **Tampered code-bearing state** → strict-modes check (Unix mode/owner; Windows owner + DACL) at every entry point that reads a trusted file (lands at Epoch 6; until then ratified interim gaps: `hook`'s snapshot read, `verify`'s stamps read (`run`'s version gate reads stamps strictly since the dialog-answers chunk), the Windows x64 ConPTY companions loaded under the held-handle re-hash alone, `viola send`'s snapshot read and `send` frame after a liveness-only pre-check, `viola wait` / `viola last`'s snapshot read and `wait` / `last` frame after the same pre-check, and `viola answer`'s `answer` frame after the same pre-check with `viola hook`'s `hook.dialog` frame after its shape check (the sixth gap), and `viola pause` / `viola release`'s frames after the same liveness-only pre-check (the seventh gap, F-W1), the founder's 2026-10-04 rulings); a home created outside `%USERPROFILE%` gets the protected user + SYSTEM DACL at creation; pinned exe re-hashed (truncated SHA-256) before reuse; the ConPTY companions (`bin/<key>/conpty/`) fully re-hashed against compiled pins through `FILE_SHARE_READ`-only handles held until the spawn, a mismatch falling back to the inbox ConPTY; plugin files and `settings.json` rewritten each start; single writers for snapshot and stamps.
- **Planted `conpty.dll` in the CWD or on PATH** (a bare-name DLL load) → every `viola` process restricts its DLL search to System32 as the second statement of `main`; the sideloaded `conpty.dll` is pre-loaded only by absolute path.
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
- NEVER answer `hook.dialog` non-`null` without a `viola verify` stamp read through `run`'s strict stamps read; only `viola verify` writes the stamps. The stamp holds seventeen rows (six spine, four screen and timing, four dialog-tier, three framing: `long-paste-wrapper` · `tag-escaping` · `local-command-clear`), so a decision flows only when every row passes (`13 pass  4 fail`, or a stamp of the fourteen older ids all `pass`, → `cli_verified:false`, `answer` exit 12); the founder's dated gap R2 is closed.
- NEVER let a process other than the instance's wrapper write its `snapshot.json`.
- NEVER let `config.json`, a `VIOLA_*` env var or a flag switch off a control. The two env carve-outs are the test seams `FAKE_AGENT_PUMP_DELAY_MS` (capped at 5 s) and `FAKE_AGENT_HOOK_PANIC` (exact `1`, forced fail-open hook panic; G2 exempts only its exact file path), both only under `fake-agent`, absent from release builds, switching off nothing; another seam needs its own Decisions Log entry.

## Arch amendments this plan requires (routed through wrap reconciles)

1. v1 GUI cookie on `/api/*` + SSE. 2. Unix endpoint in a per-user 0700 dir. 3. `not-delivered`/`control-character` + URNs `unauthorized` / `cross-origin-forbidden`. 4. `release` with `from` → `-32602 release-from-driver`. 5. `<home>/ui/<port>.url` (0600). 6. `MAX_FRAME` = 16 MiB in `viola-core`. 7. Ledger row "largest hook payload seen". 8. `<hash>` = truncated SHA-256 (16 hex). Amendments 1, 5 and amendment 3's two URNs were folded into architecture at the 2026-10-01 0-pending wrap; amendment 3's `control-character` detail lands with the route's "Confirmed send" entry.

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
- `validate_paste_text` allows LF/CR/TAB (multi-line relays are normal), rejects the rest — reject, never strip a refused character. Only allowed characters are rewritten or removed: after validation `send` types the text with every CR LF pair and every other CR as one LF and without its trailing CR and LF characters, and the exact match, the local-command list, `text_bytes` and the paste read that typed text, which holds no CR (the founder's rulings, 2026-10-07T15:21Z for a trailing LF, 2026-10-09 for a trailing CR and CRLF, 2026-10-09T16:51Z for a CR or a CR LF inside). An empty typed text is refused `not-delivered` / `empty-text` (exit 13), by the client before any frame and by the wrapper before the first wheel read.
- `MAX_FRAME` stays 16 MiB, checked against the ledger's measured largest hook payload.
- A driver's `release` is refused as an affordance guard, not enforcement (same-user processes are trusted in v1).
- GUI cookie is per-port (`viola_<port>`) with no `Max-Age`; cross-port cookie exposure accepted as residual.
- `viola revive` spawns its child in the snapshot's recorded `cwd` (read after the instance strict-modes check; founder-ratified as a boundary widening, live, 2026-10-10) and passes the logged session id as one closed-shape argv element after `--resume`; the founder's word on that second crossing is owed and no ratification is recorded.
- The local `pre-push` gate runs natively on the Linux dev host: every child runs under `/usr/bin/env -i HOME=<passwd home> PATH=<home>/.cargo/bin:<home>/.local/viola-node/bin:/usr/local/bin:/usr/bin:/bin`, so no host value — `CLAUDE*` included — crosses (canary pair: 0 through the launcher, 1 through the control without `-i`). HOME is the passwd entry's field 6, read by two PATH-only probes; the harness's `$HOME` is never read for it (that would be a host value crossing). It checks the host against ci.yml's own pins and installs nothing. No other launcher and no uid-0 launch exists. An assignment taking a host value is a boundary widening (its own Decisions Log entry).
- Supply chain beyond cargo: the test-side `e2e-web/package-lock.json` (`@playwright/test` =1.63.0) has its own audit, `scripts/npm-audit.sh` (advisories + registry.npmjs.org-only sources) in `supply-chain` and weekly in `nightly.yml`; Node is a sha256-pinned official download (`scripts/install-node.sh`, ci.yml `NODE_PIN_*`), no toolchain action. The vendored Microsoft ConPTY binaries (`vendor/conpty/1.24.260710001/x64/`, MIT, embedded; founder live ratification, Decisions Log `2026-09-29`) have their own audit, `scripts/conpty-vendor.sh --verify` / `--probe` (nupkg + per-file SHA-256, Authenticode signer), CI `windows-2025` step `ConPTY vendor verification`; four pins, one textual home `src/conpty.rs`.
- The release build carries no test-only feature: `scripts/release-check.sh` refuses an artifact built with `test-support` (viola-channel's shared test helpers, enabled only by the root `[dev-dependencies]`) or `fake-agent` (probe `5/5 refused, control clean`). On a Windows host `run --mutants` wipes the host mutation scratch outside the repository only behind a guard: final component exactly `viola-mutants-scratch`, never the repository or an ancestor (`scratch-refused`, no fallback).
- Harness origin (Decisions Log `2026-09-29`, a boundary widening the founder ratified live): the compiled harness prefixes are four — `<agent-message from=`, `<task-notification>`, `<\cross-session-message`, `<cross-session-message` — matched on the prompt's raw start; the escaped cross-session form is the one escaped tag that classifies, so a human typing it at a prompt's start is filed `harness` (accepted side effect). As measured on 2.1.287 on the Linux dev host (chunk 2026-10-07-live-rows-and-paste-shapes-on-the-dev-host) a real cross-session message arrived unescaped and was filed `harness` by the plain prefix, and a `<task-notification>` typed at a prompt's start arrives as typed, so it is filed `harness` too; the escaped injection form stays a relayed measurement and a compiled prefix. That reading and a names-only list of a 2.1.287 hook's `CLAUDE*` environment (12 names, four outside the 11-name floor, the floor unchanged) were two one-off scratch measurements on the founder's live answers of 2026-10-07T09:43Z, relayed by the overseer; neither answer covers a second message, a verify child or another probe.
- verify's typed-input PTY probe (the founder's live rulings of 2026-10-05, relayed by the overseer; a boundary widening): four interactive `claude` children after the print probe — Run A untrusted in a fresh 0700 OS-temp `viola-verify-*` dir, empty input, killed; Run B trusted in `<cwd>/.viola-verify-<pid>/` (0700, removed on every exit path), one `PROBE_PROMPT` paste, Ctrl-C ×2 then kill; Run C (`-dialogs/`, an `ask` rule, three dialog prompts; its one `allow` runs `touch viola-probe-permission` in its own dir) and Run D (`-plan/`, `--permission-mode plan`, `plansDirectory` a 0700 `plans/` inside its dir), their dialogs answered only by the hidden `hook --capture --answers` arm (`take(64)` closed `ProbeAnswer` → one `decision_body` on stdout), settled then killed — never a byte typed into any dialog, nothing written by viola under `~/.claude` (the M7 = A and STOP 7 rulings, 2026-10-05). Accepted residual: Runs B, C and D each leave the CLI's synthetic-prompt transcript under `~/.claude/projects/` (+2 per verify for C and D) and run the user's global hooks and status line. Dev-host prerequisite: run it from a trusted folder whose external CLAUDE.md import is answered (the CLI keys that answer on the git root). Screen fixtures are signature-rows-only; the `--record` refusal names file + closed code (`home-path` · `absolute-path` · `username` · `email`), never content (founder-ratified live at the chunk's wrap).
- Test homes on the Linux dev host (the founder's live answer, 2026-10-07T07:25Z, relayed by the overseer; a boundary widening): `target/e2e-home` is a link to the per-user tmpfs directory `/tmp/viola-e2e-home-<uid>`, so the fixture chain's sweep and drop and harness `cleanup` create and delete outside the working directory, each only a dir the test side itself made. The two keepers follow a linked base only when its target is absolute and one `lstat` reads a real directory with no group or other bit; they create it 0700 and remove nothing. `viola` never names the backing; no CI runner has the link.

---

**Full plan:** `.andromeda/security-plan.md`.
