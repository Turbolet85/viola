group: security-plan
read: .claude/docs/security-summary.md · 77 · «**Full plan:** `.andromeda/security-plan.md`.»
read: .claude/rules/security.md · 44 · «_This section is owned by `/wrap-session`. setup-project pre»

- leaf: .claude/rules/security.md:12   class: stale
  leaf-text: «(macOS: euid + dir only, recorded gap)»
  source: .andromeda/security-plan.md:439 «every client's euid check and macOS's pid and start-time check (no peer pid) are owed to "Unix endpoint and home hardening"»
  replace-with: «(the client euid check, and macOS, where the peer reports no pid, owed to the route entry "Unix endpoint and home hardening")»

- leaf: .claude/rules/security.md:12   class: stale
  leaf-text: «`viola hook`'s `hook.event` goes without verification and strict-modes until the Epoch 6 entries land»
  source: .andromeda/security-plan-detail/authentication-authorization.md:25 «on Linux and Windows `viola hook` verifies the serving process before its `hook.event` frame»
  replace-with: «`viola hook`'s `hook.event` reads its snapshot without strict-modes (on Linux and Windows it verifies the serving process before the frame) until the Epoch 6 entry "Home and code-bearing file integrity"»

- leaf: .claude/rules/security.md:12   class: stale
  leaf-text: «— no server identity, no strict-modes — until the Epoch 6 route entries "Server verification before any frame" and "Home and code-bearing file integrity"»
  source: .andromeda/security-plan-detail/authentication-authorization.md:25 «and then, on Linux and Windows, checks the serving process's identity before the frame (`client::connect`: the peer pid and its start time against the snapshot's `pid` and `started_at`»
  replace-with: «and then, on Linux and Windows, the server identity check (`client::connect`: the peer pid and its start time against the snapshot's `pid` and `started_at`), but no strict-modes — until the Epoch 6 route entry "Home and code-bearing file integrity"»

- leaf: .claude/rules/security.md:12   class: stale
  leaf-text: «write their `wait` / `last` frame after the same liveness-only pre-check, until the same Epoch 6 entries»
  source: .andromeda/security-plan-detail/authentication-authorization.md:25 «then, on Linux and Windows, check the serving process's identity before the frame through the same `client::connect`, but not the snapshot's strict-modes»
  replace-with: «write their `wait` / `last` frame after the same liveness pre-check and, on Linux and Windows, the same server check, until the same Epoch 6 entry»

- leaf: .claude/rules/security.md:12   class: stale
  leaf-text: «CLI `viola answer` writes its `answer` frame after the same liveness-only pre-check, and `viola hook pre-tool-use|permission-request` writes its `hook.dialog` frame after only its payload shape check, until the same Epoch 6 entries»
  source: .andromeda/security-plan-detail/authentication-authorization.md:26 «on Linux and Windows both check the serving process's identity before the frame (`answer` through `client::connect`, `hook.dialog` through `src/cmd/hook.rs`'s shared `connect`)»
  replace-with: «CLI `viola answer` writes its `answer` frame after the same liveness pre-check, and `viola hook pre-tool-use|permission-request` writes its `hook.dialog` frame after its payload shape check, each, on Linux and Windows, after the server check (`answer` through `client::connect`, `hook.dialog` through `src/cmd/hook.rs`'s shared `connect`), until the same Epoch 6 entry»

- leaf: .claude/rules/security.md:12   class: stale
  leaf-text: «write their `pause` / `release` frame after the same liveness-only pre-check, until the same Epoch 6 entries»
  source: .andromeda/security-plan-detail/authentication-authorization.md:26 «on Linux and Windows both then check the serving process's identity before the frame through `client::connect`, and neither checks the snapshot's strict-modes»
  replace-with: «write their `pause` / `release` frame after the same liveness pre-check and, on Linux and Windows, the same server check, until the same Epoch 6 entry»

- leaf: .claude/rules/security.md:12   class: stale
  leaf-text: «(`viola_state::liveness::live_endpoint`; else `instance-unreachable`), until the same Epoch 6 entries»
  source: .andromeda/security-plan-detail/authentication-authorization.md:27 «on Linux and Windows each tool then checks the serving process's identity after `TokioClient::connect` and before its frame»
  replace-with: «(`viola_state::liveness::live_endpoint`; else `instance-unreachable`) and, on Linux and Windows, the server check after `TokioClient::connect`, until the same Epoch 6 entry»

- leaf: .claude/rules/security.md:12   class: omission
  leaf-text: «they borrow none of the seven.»
  source: .andromeda/security-plan-detail/authentication-authorization.md:27 «**On mismatch** (or a check that cannot be made): no frame is written; the CLI frame verbs exit 21 `instance-unreachable`»
  replace-with: «they borrow none of the seven. On a mismatch, or a check that cannot be made, no frame is written: the CLI frame verbs exit 21 `instance-unreachable`, MCP returns `instance-unreachable` with `detail: null` and writes no line, and `hook` fails open (exit 0, no body, `hook-decision` carrying `server-verify-failed`); no line or output holds a pid, start time, path or endpoint.»

- leaf: .claude/docs/security-summary.md:15   class: stale
  leaf-text: «On macOS it is euid + dir only (a recorded known gap). Interim, ratified by the founder (Decisions Log `2026-09-28`): `viola hook` sends `hook.event` without server verification or strict-modes until the Epoch 6 entries land.»
  source: .andromeda/security-plan-detail/threat-model-summary.md:55 «compares the connected peer's pid and that process's start time with the snapshot's `pid` and `started_at` before its first frame»
  replace-with: «As built, on Linux and Windows every frame-writing client (the CLI frame verbs, `viola hook`'s event and dialog paths, `viola mcp`'s frame tools) compares the connected peer's pid and start time with the snapshot's `pid` and `started_at` before its first frame and sends no frame on a mismatch or a check it cannot make. Interim, ratified by the founder (Decisions Log `2026-09-28` and the 2026-10-04 / 2026-10-17 rulings): the snapshot is read without strict-modes, owed to "Home and code-bearing file integrity"; the client euid check and macOS (no peer pid) are owed to "Unix endpoint and home hardening".»

- leaf: .claude/docs/security-summary.md:17   class: stale
  leaf-text: «`viola send`'s snapshot read and `send` frame after a liveness-only pre-check, `viola wait` / `viola last`'s snapshot read and `wait` / `last` frame after the same pre-check, and `viola answer`'s `answer` frame after the same pre-check with `viola hook`'s `hook.dialog` frame after its shape check (the sixth gap), and `viola pause` / `viola release`'s frames after the same liveness-only pre-check (the seventh gap, F-W1)»
  source: .andromeda/security-plan-detail/security-anti-patterns.md:8 «The dated exceptions hold only the strict-modes half — the reference values taken from a `snapshot.json` read without the strict-modes check — owed to the Epoch 6 entry»
  replace-with: «the snapshot read without strict-modes behind `viola send`'s `send` frame, `viola wait` / `viola last`'s `wait` / `last` frames, `viola answer`'s `answer` frame with `viola hook`'s `hook.dialog` frame (the sixth gap) and `viola pause` / `viola release`'s frames (the seventh gap, F-W1), each frame written after the liveness pre-check (`hook.dialog` after its shape check) and, on Linux and Windows, the server check — the gaps' strict-modes half only»

- leaf: .claude/docs/security-summary.md:17   class: stale
  leaf-text: «`viola mcp`'s `send` / `wait` / `last` / `answer` tool frames after the same liveness-only pre-check (the eighth gap»
  source: .andromeda/security-plan-detail/authentication-authorization.md:27 «on Linux and Windows each tool then checks the serving process's identity after `TokioClient::connect` and before its frame»
  replace-with: «`viola mcp`'s `send` / `wait` / `last` / `answer` tool frames, their snapshot read without strict-modes, after the liveness pre-check and, on Linux and Windows, the server check (the eighth gap»
