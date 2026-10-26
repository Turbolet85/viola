proposals:
  - detector: D-security-auth
    severity: warning
    file: .andromeda/security-plan-detail/authentication-authorization.md
    near: 37
    section: '## Authentication & Authorization'
    change: 'In the strict-modes entry-point list opened at line 34, after the `viola plugin install` clause (ending "its integrity owed to the route entry "Home and code-bearing file integrity"") and before "), refuse when", add: "; `viola link` and `viola unlink` (chunk 2026-10-26-explicit-session-links, the founder''s own word `inputs#I3`) run `viola_state::strict::check_instance` on the absolute home and `instances/<driven>` before `client::start`, so before any role file or log opens and before any snapshot value is read; a refusal exits 21 (the fixed instance-state `unable:` / `hint:` pair on stderr, or under `--json` exactly `{"v":1,"error":"instance-unreachable","detail":null}` on stdout with stderr empty), writes no line in any log and leaves the directory''s mode as found; it is none of the dated interim gaps; proven by `cfg(unix)` cases only"'
    sidecar: 'authentication-authorization.md strict-modes entry points: `viola link` / `viola unlink` added, `check_instance` before any role file, log or snapshot read, exit 21 on a refusal (chunk 2026-10-26-explicit-session-links, inputs#I3)'
    rationale: 'Report Changes › Symbols / APIs, link_call step 1 (src/cmd/link.rs:98-106: check_instance BEFORE client::start, refused → exit 21, instance-state pair, --json instance-unreachable, no line, mode as found) and Counts › checkinst, which names this list (authentication-authorization.md:34) stale: it names run, revive, list, mcp, hook statusline and plugin install, not the two verbs; Decisions & corrections: the founder''s word inputs#I3, no dated gap taken; Outcome: link_verbs_refuse_a_widened_instance_before_any_log (ubuntu, macOS)'
    basis: viola-0.1.0/chunks/2026-10-26-explicit-session-links/report.md:102
  - detector: D-security-auth
    severity: warning
    file: .andromeda/security-plan-detail/authentication-authorization.md
    near: 66
    section: '## Authentication & Authorization'
    change: 'The CLI refusal reads "A CLI frame verb (`send`, `wait`, `last`, `answer`, `pause`, `release`, `link`, `unlink`) meeting a mismatch, …" (rest unchanged), and after "The client''s own exit-13 refusals come first." adds: "`link` and `unlink` run `check_instance` before their liveness pre-check and this check: a refusal there is exit 21 with the fixed instance-state `unable:` / `hint:` pair (under `--json` the same `instance-unreachable` document), never the server-verify hint."'
    sidecar: 'authentication-authorization.md CLI refusal: the frame verbs are eight (`link`, `unlink` added), and the two verbs'' earlier strict-modes refusal named (chunk 2026-10-26-explicit-session-links)'
    rationale: 'Report Counts › CLI frame verbs six → eight; Symbols / APIs step 5 (a server that is not the recorded wrapper gets no frame, exit 21 instance-unreachable with the server-verify hint, process-exit detail server-verify-failed) and step 1 (the instance-state pair); "The CLI frame verbs are now eight … Only link / unlink also run check_instance first"'
    basis: viola-0.1.0/chunks/2026-10-26-explicit-session-links/report.md:77
  - detector: D-security-auth
    severity: warning
    file: .andromeda/security-plan-detail/authentication-authorization.md
    near: 67
    section: '## Authentication & Authorization'
    change: 'The CLI refusal''s stack reference reads "`src/cmd/{send,wait,last,answer,pause,release,link,unlink}.rs` (`link` and `unlink` through the shared `link_call` in `src/cmd/link.rs`)" in place of the six-verb set.'
    sidecar: 'authentication-authorization.md CLI refusal stack reference: `link.rs`, `unlink.rs` added (chunk 2026-10-26-explicit-session-links)'
    rationale: 'Report Changes › Files (new src/cmd/link.rs, src/cmd/unlink.rs) and Symbols / APIs (link_call src/cmd/link.rs:85-136 shared by both verbs; cmd::client::connect callers 6 → 8)'
    basis: viola-0.1.0/chunks/2026-10-26-explicit-session-links/report.md:73
    dependent-of: D-security-auth
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/authentication-authorization.md
    near: 73
    section: '## Authentication & Authorization'
    change: 'After line 73, inside the same detail block, add the item "- **Requirement:** IPC client-side server verification, `viola link` / `viola unlink`" / "  - **Implementation:** The two verbs (chunk 2026-10-26-explicit-session-links; the boundary widening ratified by the founder''s own word, `inputs#I3`) write their `link` / `unlink` frame to the driven instance''s wrapper only after `check_instance` (before any role file, log or snapshot read), the liveness-only pre-check (`live_endpoint`) and, on Linux and Windows, the server identity check through `cmd::client::connect`; their snapshot passed the strict-modes check in the same process, so they take none of the eight dated gaps (the client euid check and macOS stay owed to "Unix endpoint and home hardening", as for every client). The wrapper serves both methods under the channel row''s controls: `driver` and `driven` read through `ViolaName::try_new`, a `driven` that is not the wrapper''s own name or a `driver` equal to `driven` refused `-32602` with `data: null` and no line, `from` never read." / "  - **Stack reference:** root bin `src/cmd/link.rs` (`link_call`), `src/cmd/unlink.rs`, `src/run/links.rs` (`LinkSet::link` / `unlink`, `driver_of`); `tests/cli_links.rs`, `tests/channel_links.rs`, `tests/security_negatives_server_identity.rs`"'
    sidecar: 'authentication-authorization.md: new item, the served `link` / `unlink` methods and their verbs'' server verification under the instance check, no dated gap (chunk 2026-10-26-explicit-session-links, inputs#I3)'
    rationale: 'Report Changes › Symbols / APIs (channel methods link and unlink served, driver_of src/run/links.rs:85-99: ViolaName::try_new, own-name, self-pair → -32602 data null, no line, from not read; link_call steps 1-5) and Decisions & corrections (the founder''s word inputs#I3: the boundary widening ratified, no dated gap taken); every other frame-writing client in this file carries its dated gap, the two verbs none'
    basis: viola-0.1.0/chunks/2026-10-26-explicit-session-links/report.md:18
note: 'The frameverbs sweep did not list authentication-authorization.md:66 (its six verbs sit in a parenthesised enumeration) — found by read; the ban at .andromeda/security-plan.md:439 and the Universal ban at .andromeda/security-plan.md:506 (a Decisions Log entry mapping the served link / unlink methods to the IPC controls) lie outside this category file, owned by the anti-patterns / Decisions Log detectors; D-security-input (link / unlink params through ViolaName::try_new) and D-security-deps (Dependencies: none) read clean.'
