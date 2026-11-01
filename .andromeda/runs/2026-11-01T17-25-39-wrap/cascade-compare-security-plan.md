# Cascade compare — security-plan · wrap · 2026-11-01-unix-endpoint-and-home-hardening
E items 54 · held 19 · lost at classify 2 · lost at strong 2 · beside a placed fact 1 · cascade only 4 · listed - · body 0 · unplaced 0

## Lost — the full read carried these and the cascade did not; the wrap validates each, marked «(sampler)»

### error-handling#1.13 · lost at classify · .andromeda/security-plan-detail/error-handling.md:20
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/error-handling.md
    near: 20
    section: "## Error Handling"
    change: "New item after this line: - A stream `accept_loop` refuses by `admitted(peer_euid_of(stream))` is dropped on the accept thread, before its worker thread and before any frame: the peer reads end-of-file, and the wrapper writes no reply, no line and nothing on the terminal, there being no frame to key a `channel-response` on."
    sidecar: "error-handling: the wrapper's drop of a refused peer has no error body and no line."
    rationale: "The server-side refusal's error path, stated at report.md:13 and report.md:75; `channel-response`'s `detail` stays closed, so this entry records why no fault word appears."
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:13

### error-handling#1.15 · lost at strong · beside a placed fact F15 (near 24, error-handling#1.17) · .andromeda/security-plan-detail/error-handling.md:22
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/error-handling.md
    near: 22
    section: "## Error Handling"
    change: "New item after this line: - The CLI's `connect`, behind `send` · `wait` · `last` · `answer` · `pause` · `release` · `link` · `unlink`, runs `check_endpoint_dir` on the recorded endpoint before `Client::connect`: a refusal (`CallError::SocketDirRefused`, `Unreachable::SocketDir`) exits 21 with `detail` `socket-dir-refused`, `during` `connect`, a `process-exit` at WARN, no frame and no `channel-request`, the directory left as found. After the connect, a refused `peer_admitted(client.server_euid())` is `CallError::ServerUnverified` and reads `server-verify-failed`, ahead of `verify_server`."
    sidecar: "error-handling: the CLI frame verbs' refusal on a refused socket directory and on a peer of another euid."
    rationale: "New refusal at the CLI's `connect` and its 8 callers (report.md:15), its record at report.md:71; a refused directory stays as found (report.md:11)."
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:15

### error-handling#1.23 · lost at strong · .andromeda/security-plan-detail/error-handling.md:30
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/error-handling.md
    near: 30
    section: "## Error Handling"
    change: "New item after this line: - `Refused::NotADirectory` carries the fixed `Display` message `a trusted state path is not a directory`; `check_socket_dir` returns it for a path that is no directory, `Refused::Symlink` for a symlink and `Refused::Unreadable` for an unstattable path."
    sidecar: "error-handling: `Refused` gains the fixed-message variant `NotADirectory`; the `thiserror` derives stay nine."
    rationale: "New fixed-message variant and the verdict's order (report.md:11); a variant, not a derive, so line 31's nine stands (report.md:36)."
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:11

### security-anti-patterns#5.1 · lost at classify · .andromeda/security-plan-detail/security-anti-patterns.md:39
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/security-anti-patterns.md
    near: 39
    section: "## Security Anti-Patterns (NEVER do these) › ### Universal"
    change: "Add after this item: «- NEVER add an env, flag or feature seam to force a refusing peer-euid decision: one account can make no foreign peer, so the decisions are proven over injected values (`peer_euid_admitted(observed, own, reported)`, the wrapper's private `serve_admitting(dispatch, admitted)` over an `Admission` function), and each client seam's `peer_admitted` wiring (`cmd::client::connect`, `cmd::hook::connect`, `viola-mcp`'s `frame`) stays the named weakest proof, read only through a run's forced refusal, no committed test repeating it.»"
    sidecar: "security-anti-patterns · Universal: the peer-euid decisions are proven by injected values with no seam; the client seams' wiring recorded as the named weakest proof."
    rationale: "report.md:13 states the injection and that it is no env, flag or feature seam; report.md:54 binds the wiring's survivor to this section's no-seam rule."
    basis: crates/viola-channel/src/client.rs:151-156

### threat-model-summary#8.1 · lost at strong · .andromeda/security-plan-detail/threat-model-summary.md:123
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/threat-model-summary.md
    near: 123
    section: "## Threat Model Summary"
    change: "Add after line 123, at its level: `viola list`'s board scan holds the home to the read rule on every OS (`check_read`), `instances/` staying `check_path`, so a home another user may read is refused whole, with or without an instance; its order and no-open are unchanged, and `read_budget_stamped`, `mcp`'s start and `plugin install` keep the write rule on the home. PROVISIONAL: the overseer's answer as founder's delegate, the founder's own word owed (`inputs#I2` Q3)."
    sidecar: "Threat Model Summary, boundary-cli trust boundary: `viola list` holds the home to the read rule on every OS, PROVISIONAL on the delegate's answer."
    rationale: "report.md:18 and report.md:112 change the board's rule on a state tree `--home` or `VIOLA_DIR` chooses; report.md:50 and report.md:94 carry its PROVISIONAL standing."
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:18

## Held

- authentication-authorization#1.4
- authentication-authorization#1.5
- authentication-authorization#1.6
- authentication-authorization#1.8
- authentication-authorization#3.1
- authentication-authorization#3.5
- authentication-authorization#3.6
- authentication-authorization#3.8
- bootstrap-phases#1.1
- error-handling#1.1
- error-handling#1.16
- error-handling#1.18
- error-handling#1.2
- error-handling#1.20
- error-handling#1.5
- error-handling#1.6
- input-validation#1.7
- security-anti-patterns#1.1
- threat-model-summary#2.1

## Cascade only

- authentication-authorization#3.4
- authentication-authorization#3.9
- error-handling#1.17
- threat-model-summary#6.1
