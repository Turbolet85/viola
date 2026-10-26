proposals:
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/error-handling.md
    near: 10
    section: '## Error Handling'
    change: '- The channel methods `link` / `unlink`, served by the driven instance wrapper (chunk 2026-10-26-explicit-session-links), answer a missing, non-string or invalid `driver` / `driven` name, a `driven` that is not the serving wrapper own name, and a `driver` equal to `driven` with `-32602` and `data: null`, writing no line; a failed `record` / `remove` is `-32603` (`internal error`, `data: null`); neither method reads `from`.'
    sidecar: 'Error Handling: the served `link` / `unlink` channel methods error answers recorded (`-32602` with `data: null` and no line for a bad name or pair, `-32603` for a failed record or remove), chunk 2026-10-26-explicit-session-links.'
    rationale: 'Report Changes, Symbols / APIs, first bullet: the wrapper now serves `link` / `unlink` (before `-32601`), each bad name or pair `ProtocolError::InvalidParams` (`-32602`, `data: null`, no line written), a `record` / `remove` error `ProtocolError::Internal` (`-32603`); a new external error surface this category holds no record of.'
    basis: viola-0.1.0/chunks/2026-10-26-explicit-session-links/report.md:18
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/error-handling.md
    near: 10
    section: '## Error Handling'
    change: '- `viola link` / `viola unlink` `<driver> <driven> [--json]` (the driven instance is the verb instance) refuse before any frame: a driven instance failing `check_instance`, run before any role file, log or snapshot read, exits 21 with the fixed instance-state `unable` / `hint:` pair, or under `--json` exactly `{"v":1,"error":"instance-unreachable","detail":null}` on stdout with stderr empty, no line in any log and the directory mode as found; a server that is not the recorded wrapper (Linux and Windows; macOS the recorded gap) gets no frame and exits 21 `instance-unreachable` with the server-verify hint, its `process-exit` detail `server-verify-failed`; a wrapper error (`-32602` included) is exit 20, and a reply of another shape is an internal error, exit 1.'
    sidecar: 'Error Handling: the `viola link` / `viola unlink` refusal outputs recorded (strict-modes refusal exit 21 with the fixed pair or the code-only `--json` document, server mismatch exit 21 `instance-unreachable`, wrapper error exit 20), chunk 2026-10-26-explicit-session-links, the founder word inputs#I3.'
    rationale: 'Report Changes, Symbols / APIs, the new verbs bullet, steps 1, 5 and 6: the verbs external error outputs are codes and fixed text only (the `--json` document exactly `{"v":1,"error":"instance-unreachable","detail":null}`, stderr empty), new external-response detail beside the file `viola hook` and `viola mcp` server-check records.'
    basis: viola-0.1.0/chunks/2026-10-26-explicit-session-links/report.md:38
note: 'D-security-input, D-security-auth and D-security-deps read clean for this file (params through ViolaName::try_new, the server check through cmd::client::connect, Dependencies none); no Retired item stands in it. The body line .andromeda/security-plan.md:418 (Error format, Channel) names `-32602` only for release-with-from; the link / unlink invalid-params cause is the body owner to weigh.'
