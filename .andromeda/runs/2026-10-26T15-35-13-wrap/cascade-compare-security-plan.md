# Cascade compare — security-plan · wrap · 2026-10-26-explicit-session-links
E items 14 · held 8 · lost at classify 1 · lost at strong 0 · beside a placed fact 1 · cascade only 2 · listed - · body 0 · unplaced 0

## Lost — the full read carried these and the cascade did not; the wrap validates each, marked «(sampler)»

### bootstrap-phases#1.1 · lost at classify · .andromeda/security-plan-detail/bootstrap-phases.md:15
  - detector: D-security-auth
    severity: warning
    file: .andromeda/security-plan-detail/bootstrap-phases.md
    near: 15
    section: '## Bootstrap phases (derive for route / setup-project)'
    change: 'In the `viola-channel` client item, the clause "the snapshot read after the strict-modes check is owed to the route entry "Home and code-bearing file integrity"" becomes: "the snapshot read after the strict-modes check is owed to the route entry "Home and code-bearing file integrity" for every client except CLI `viola link` / `viola unlink`, which run `viola_state::strict::check_instance` on the home and the driven instance before any role file or log opens and before any snapshot value is read, then the liveness pre-check and the server check (a refused check exit 21, no line written; the founder''s word, chunk 2026-10-26-explicit-session-links)"; the rest of the item stands.'
    sidecar: 'security-plan Bootstrap phases (auth-scaffolding-baseline, viola-channel client): the owed strict-modes snapshot read now names its first two exceptions, CLI viola link / viola unlink, which check the driven instance before any log or snapshot read (chunk 2026-10-26-explicit-session-links).'
    rationale: 'The report (viola-0.1.0/chunks/2026-10-26-explicit-session-links/report.md:41-46, :265-267) records both new verbs running check_instance before client::start and before any snapshot value is read, the founder''s word inputs#I3 ratifying it with no dated gap taken, and its expected amendments (:176-179) name the CLI frame verbs under the instance check, not a dated gap; the file states the strict-modes snapshot read as owed for every client.'
    basis: .andromeda/security-plan-detail/bootstrap-phases.md:15

### error-handling#1.3 · lost at classify · beside a placed fact F5 (near 10, error-handling#1.3) · .andromeda/security-plan-detail/error-handling.md:10
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/error-handling.md
    near: 10
    section: '## Error Handling'
    change: '- The channel methods `link` / `unlink`, served by the driven instance wrapper (chunk 2026-10-26-explicit-session-links), answer a missing, non-string or invalid `driver` / `driven` name, a `driven` that is not the serving wrapper own name, and a `driver` equal to `driven` with `-32602` and `data: null`, writing no line; a failed `record` / `remove` is `-32603` (`internal error`, `data: null`); neither method reads `from`.'
    sidecar: 'Error Handling: the served `link` / `unlink` channel methods error answers recorded (`-32602` with `data: null` and no line for a bad name or pair, `-32603` for a failed record or remove), chunk 2026-10-26-explicit-session-links.'
    rationale: 'Report Changes, Symbols / APIs, first bullet: the wrapper now serves `link` / `unlink` (before `-32601`), each bad name or pair `ProtocolError::InvalidParams` (`-32602`, `data: null`, no line written), a `record` / `remove` error `ProtocolError::Internal` (`-32603`); a new external error surface this category holds no record of.'
    basis: viola-0.1.0/chunks/2026-10-26-explicit-session-links/report.md:18

### error-handling#1.3 · lost at classify · beside a placed fact F6 (near 10, error-handling#1.3) · .andromeda/security-plan-detail/error-handling.md:10
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/error-handling.md
    near: 10
    section: '## Error Handling'
    change: '- `viola link` / `viola unlink` `<driver> <driven> [--json]` (the driven instance is the verb instance) refuse before any frame: a driven instance failing `check_instance`, run before any role file, log or snapshot read, exits 21 with the fixed instance-state `unable` / `hint:` pair, or under `--json` exactly `{"v":1,"error":"instance-unreachable","detail":null}` on stdout with stderr empty, no line in any log and the directory mode as found; a server that is not the recorded wrapper (Linux and Windows; macOS the recorded gap) gets no frame and exits 21 `instance-unreachable` with the server-verify hint, its `process-exit` detail `server-verify-failed`; a wrapper error (`-32602` included) is exit 20, and a reply of another shape is an internal error, exit 1.'
    sidecar: 'Error Handling: the `viola link` / `viola unlink` refusal outputs recorded (strict-modes refusal exit 21 with the fixed pair or the code-only `--json` document, server mismatch exit 21 `instance-unreachable`, wrapper error exit 20), chunk 2026-10-26-explicit-session-links, the founder word inputs#I3.'
    rationale: 'Report Changes, Symbols / APIs, the new verbs bullet, steps 1, 5 and 6: the verbs external error outputs are codes and fixed text only (the `--json` document exactly `{"v":1,"error":"instance-unreachable","detail":null}`, stderr empty), new external-response detail beside the file `viola hook` and `viola mcp` server-check records.'
    basis: viola-0.1.0/chunks/2026-10-26-explicit-session-links/report.md:38

## Held

- authentication-authorization#1.8
- authentication-authorization#3.3
- authentication-authorization#3.5
- input-validation#1.12
- input-validation#1.3
- input-validation#1.6
- security-anti-patterns#1.1
- threat-model-summary#2.1

## Cascade only

- authentication-authorization#1.2
- authentication-authorization#1.6
