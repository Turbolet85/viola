# security · Error Handling — category read

## Carried
- (.andromeda/security-plan-detail/error-handling.md:8) A pipe open that the hardened listener refuses (a remote-client rejection, a DACL denial, a squatted name) must still leave `viola hook` failing open with exit 0, no body and empty stderr, its detail the closed `channel-unreachable`; the chunk adds no hook-side refusal and moves no hook detail word.
  > `hook` never writes to stderr and never exits non-zero (Hook Contract). A security check failing in `hook` (server verification mismatch, oversized stdin, strict-modes failure) still ends in exit 0 with no body, so the dialog renders for the human. As landed, `viola hook <event>` runs the stdin cap, the `VIOLA_NAME` / `VIOLA_DIR` shape checks and the one-object parse, and fails open with `hook-decision` detail `oversize-stdin` · `malformed-json` · `channel-unreachable`; server verification and strict-modes join at Epoch 6 (Decisions Log `2026-09-28`).
- (.andromeda/security-plan.md:406) Any error the listener's admission or first-instance failure surfaces to a channel client, CLI `--json` or an MCP driver must be sanitised: no anyhow chain, no absolute path, no internal type name (the pipe name and an interprocess / windows-sys error text included).
  > **External responses** (to channel clients, CLI `--json` and MCP drivers): sanitised.
  > - No anyhow context chains, no absolute paths, no upstream text (prompts, tool `input`, assistant output), no serde_path_to_error paths or values, and no internal type names.
- (.andromeda/security-plan.md:408) Any `ChannelError` variant the chunk adds or reuses for a second-listener failure, a remote rejection or a denied open must `Display` a fixed message, a field holding the pipe name, a SID, an SDDL string or an OS error kept out of `Display`.
  > - The `Display` impls on `PtyError` (hand-written), `ChannelError`, `StateError`, `AgentError`, `McpError` and `CoreError` (`thiserror`) use fixed messages. Variant fields that hold paths or payloads are excluded from `Display`.
- (.andromeda/security-plan.md:413) Full detail of a bind, DACL or first-instance failure goes only to the instance's 0600 `diagnostics/`, or for the `run` start refusal to stderr, and that stderr line holds no value on the NEVER-log floor.
  > - Full error detail goes only to `instances/<name>/diagnostics/` (files 0600; format owned by obs) or, for short-lived CLI verbs, to stderr. stderr never carries upstream text, tool `input`, drift-report values or any other value on the NEVER-log floor (Bootstrap phases, `logging-redaction-wire`).
- (.andromeda/security-plan.md:415) The hook's fail-open floor holds for every admission outcome the chunk asserts: no stderr byte and no non-zero exit from `hook`.
  > `hook` never writes to stderr and never exits non-zero: a security check failing in `hook` still ends in exit 0 with no body, so the dialog renders for the human;
- (.andromeda/security-plan.md:418) The channel's error format stands unchanged: the chunk adds no JSON-RPC code and no refusal word (`squatted-name`, exit 1 stand).
  > - **Channel:** JSON-RPC `error` for protocol faults, including `-32600` for frames over `MAX_FRAME` and `-32602` for `release`-with-`from`, and `result.refusal` for `not-delivered`/`control-character` and `not-delivered`/`empty-text`.
- (.andromeda/security-plan.md:484) The squatted-name start refusal and any new listener error must name no absolute path (the pipe path, the home or the instance directory) in an external error.
  > - NEVER expose absolute paths in any external error.

## Bindings
- **External-response sanitisation ↔ body §Authentication & Authorization (IPC access control (Windows))**: the listener the chunk asserts (protected user + SYSTEM DACL, remote clients rejected, first-pipe-instance kept) fails only through fixed-message errors with no path or type name, (.andromeda/security-plan.md:406) ↔ (.andromeda/security-plan.md:108).
- **`ChannelError` fixed `Display` ↔ body §Bootstrap phases (`error-sanitization-wire`)**: any new `viola-channel` error variant follows the fixed-message rule the bootstrap item wired, (.andromeda/security-plan.md:408) ↔ (.andromeda/security-plan.md:336).
- **Internal logging ↔ body §Bootstrap phases (`logging-redaction-wire`)**: detail of an admission failure lands only in 0600 `diagnostics/` or a floor-clean stderr line, (.andromeda/security-plan.md:413) ↔ (.andromeda/security-plan.md:340).
- **Hook fail-open ↔ body §Security Anti-Patterns (§Logging, §Universal)**: a refused or unreachable pipe never makes `hook` write stderr or exit non-zero, and no admission refusal blocks the human, (.andromeda/security-plan-detail/error-handling.md:8) ↔ (.andromeda/security-plan.md:480) and (.andromeda/security-plan.md:507).
