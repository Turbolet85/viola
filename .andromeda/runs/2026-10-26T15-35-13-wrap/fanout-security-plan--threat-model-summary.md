proposals:
  - detector: D-security-auth
    severity: warning
    file: .andromeda/security-plan-detail/threat-model-summary.md
    near: 55
    section: '## Threat Model Summary'
    change: 'As built, on Linux and Windows every frame-writing client (CLI `send`, `wait`, `last`, `answer`, `pause`, `release`, `link`, `unlink`; `viola hook`''s event and dialog paths; `viola mcp`''s `send` / `wait` / `last` / `answer` tools) compares the connected peer''s pid and that process''s start time with the snapshot''s `pid` and `started_at` before its first frame, and sends no frame on a mismatch or a check it cannot make. `viola link` / `viola unlink` first run `viola_state::strict::check_instance` on the home and the driven instance, before any role file, log or snapshot read (a refusal is exit 21; the founder''s own word at P4 of chunk 2026-10-26-explicit-session-links), so their snapshot read takes no dated gap; every other client''s snapshot is read without the strict-modes check (owed to "Home and code-bearing file integrity"), and macOS, where the peer reports no pid, stays a recorded gap (owed to "Unix endpoint and home hardening").'
    sidecar: 'threat-model-summary boundary-ipc: the frame-writing clients'' server check names the eight CLI frame verbs (`link`, `unlink` added), the two new verbs running the instance strict-modes check before any snapshot read (chunk 2026-10-26-explicit-session-links).'
    rationale: 'The report''s Counts moves the CLI frame verbs six to eight and names this line stale under `frameverbs` (report.md:94-95); Symbols / APIs step 1 puts `check_instance` before `client::start` and step 5 the server check through `cmd::client::connect` (report.md:41-55), the founder''s instance-check form with no dated gap taken (report.md:265-267).'
    basis: viola-0.1.0/chunks/2026-10-26-explicit-session-links/report.md:94
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/threat-model-summary.md
    near: 57
    section: '## Threat Model Summary'
    change: '- The served `link` / `unlink` methods (chunk 2026-10-26-explicit-session-links, the boundary widening the founder ratified at P4 in the instance-check form) let whoever can connect record or remove a driver''s link pair on the driven instance; the wrapper reads `driver` and `driven` through `ViolaName::try_new` and refuses a missing, non-string or invalid name, a `driven` that is not its own name and a `driver` equal to `driven` (each `-32602`, `data: null`, no line written), and reads no `from`, so a recorded driver is a self-reported name like `from`, never an authenticated identity.'
    sidecar: 'threat-model-summary boundary-ipc: new item, the served `link` / `unlink` methods as an IPC entry point, their wrapper-side name checks and the self-reported driver (chunk 2026-10-26-explicit-session-links).'
    rationale: 'The report''s Symbols / APIs makes `link` / `unlink` served methods (before `-32601`) whose params are checked by `driver_of` through `ViolaName::try_new`, own-name and self-pair, each `-32602` with no line, neither reading `from` (report.md:18-29); Decisions records the widening ratified at P4 (report.md:265-267). Validation matches §Input Validation (Channel frames row), so D-security-input holds; this is new IPC-vector detail.'
    basis: viola-0.1.0/chunks/2026-10-26-explicit-session-links/report.md:18
note: 'Same verb-list change sits outside this file (security-plan.md:439 frame-ban dated-gap list, security-plan.md:114, authentication-authorization.md:34 per the report); line 58''s open driver-`release` question predates this chunk and was not proposed.'
