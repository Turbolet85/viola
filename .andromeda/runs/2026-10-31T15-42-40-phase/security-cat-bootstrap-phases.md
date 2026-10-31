# security · Bootstrap phases — category read

## Carried
- (.andromeda/security-plan-detail/bootstrap-phases.md:8) The contract choice this chunk raises (a code in `detail` or in `error`) must be folded into the architecture text before the implementation that lands it.
  > Each arch amendment (Decisions Log) must be folded in before the first chunk that implements it:
- (.andromeda/security-plan-detail/bootstrap-phases.md:14) The socket-directory verification stays with its route entry: this chunk must give that refusal a code without building or moving the verification.
  > the `peer_creds` decision and the directory verification stay with their route entries.
- (.andromeda/security-plan-detail/bootstrap-phases.md:15) Telling strict modes, not running and server verification apart must keep this order and path for every client, and the strict-modes refusal must still write no line.
  > every client reads the snapshot only after the instance's strict-modes check (`viola_state::strict::check_instance`; the six CLI frame verbs and `link` / `unlink` through `client::refuse_untrusted`, the hook event and dialog paths through `trusted`, `viola mcp`'s frame tools directly; a refusal writes no line), then makes the liveness pre-check and the server check.
- (.andromeda/security-plan-detail/bootstrap-phases.md:16) P3's closing of the cause list must read each of these strict-modes checks and refusals, including `verify`'s refused home tree, and account for each one's code or named absence.
  > `check_home_tree` (the home tree at `run` and `revive` start and in `verify`); `check_diagnostics` (the diagnostics opens); `check_present` (the `conpty/` companions); and the `Readable` and `Symlink` refusals
- (.andromeda/security-plan-detail/bootstrap-phases.md:17) The pinned-hash-mismatch code and its `revive --json` document must report this re-hash's refusal and must leave the re-hash itself unweakened.
  > `viola-state`: SHA-256 re-hash of `bin/<version>-<hash>/viola(.exe)` before reuse.
- (.andromeda/security-plan-detail/bootstrap-phases.md:24) Any new `process-exit` log form and the new table's test homes and JUnit output must pass this CI canary scan against the NEVER-log floor.
  > The CI `test` job does run obs-plan §9's artifact canary scan: `viola-harness secret-scan` over test-home diagnostics, the harness capture except `run --mutants`' `target/agent-run/chunk.diff`, and the nextest JUnit report, checked against this plan's NEVER-log floor.
- (.andromeda/security-plan.md:9) The security-plan text this chunk's contract choice amends must not be hand-edited during the phase or implement runs.
  > _Do not edit during phase loop runs (`/andromeda-phase` / `/andromeda-implement`).
- (.andromeda/security-plan.md:29) `cwd-missing`'s code, hint, log line and document must not carry the recorded directory.
  > The snapshot's recorded `cwd`, an absolute host path that can carry the OS username, stands in the 0600 snapshot only, on no log line, error body or stdout.
- (.andromeda/security-plan.md:47) Start refusals must keep this check before any log opens, so a log form for a strict-modes cause must not move the check or open a log ahead of it without a P4 ruling.
  > beyond the content hash in the folder name the start holds the home tree, the pinned binary and the plugin folders to the strict-modes check (owner and mode, or owner and DACL) before any log opens.
- (.andromeda/security-plan.md:110) The socket-directory refusal must keep these three outcomes (`run` refused, clients told `instance-unreachable`, `hook` open) whatever code tells it apart.
  > a failed check refuses `run`, reports `instance-unreachable` to clients and fails `hook` open
- (.andromeda/security-plan.md:114) The server-verification cause's code must cover a mismatch, a replayed snapshot and a check that cannot be made with no frame sent and no default-connect fallback, and the owed euid and macOS gaps must stay with their route entry.
  > before the first frame of any method, the client runs the instance's strict-modes check, a refusal writing no frame and no line, and only then verifies the server against its one snapshot read: on Linux and Windows the connected peer's pid (Windows over an SQOS identification-level open) and that process's start time, a mismatch, a snapshot replayed from the log (no endpoint, pid or start time) or a check that cannot be made sending no frame, never falling back to a default connect; the client euid check and macOS (no peer pid; its euid and verified socket-directory checks) are owed to "Unix endpoint and home hardening".
- (.andromeda/security-plan.md:144) Every `revive --json` document, including those owed after a passed preflight, must hold only codes, closed words and session-shaped ids.
  > `revive --json` prints codes, closed words and session-shaped ids only, never a path or pid.
- (.andromeda/security-plan.md:160) The unwrapped-name cause must refuse such a row as a target, and its code and hint must not turn the row's name into a `ViolaName`.
  > an unwrapped row's `name` is never a `ViolaName` and never a valid `target`.
- (.andromeda/security-plan.md:164) The script-child cause's code must report this refusal with its `claude` shim carve-out unchanged.
  > on Windows a `.cmd`/`.bat` child is refused, except a `claude` shim, resolved to its fixed `claude.exe` sibling when that exists
- (.andromeda/security-plan.md:166) The `no-session` code must keep its place before any spawn, and each revive hint and refusal line in the table must hold none of these values.
  > an `--id` the log does not hold is `no-session` before any spawn; no refusal line or hint holds the recorded directory, a pid or a logged id.
- (.andromeda/security-plan.md:341) Every new exit-21 and exit-1 code must reach MCP `structuredContent` only as one of these code fields.
  > MCP `structuredContent` carries only `refusal`/`detail`/`error` codes.
- (.andromeda/security-plan.md:342) The `internal-error` cause must keep a fixed message on its external surfaces, with the full error confined to diagnostics.
  > Map it to a fixed message before it joins the chain, and write the full error only to `instances/<name>/diagnostics/` when an instance is resolved
- (.andromeda/security-plan.md:345) No new code, hint or `--json` document may carry upstream text, and each surface named here must stay free of it.
  > Outside those payloads, user content (prompts, `last_assistant_message`, plan, question and option text), tool `input`, the `statusline_command` string, and serde_path_to_error drift reports (paths and values) go only to `instances/<name>/diagnostics/`. They never go to stderr log or error lines, CLI `--json` error output, MCP `isError` results or channel `error.data`.
- (.andromeda/security-plan.md:346) New codes must hold no absolute path and no internal type name, so a code word must not be a Rust variant or type name, and a pid may stand only in `diagnostics/`.
  > Operational metadata (TMS §1): absolute paths, which carry the OS username (the only PII in scope), and internal type names never go to CLI `--json` error output, MCP `isError` results or channel `error.data`. Pids, `agent_session_id`, `started_at` and `budget.json` values may be written to `diagnostics/`.
- (.andromeda/security-plan.md:410) Every external form of a new code must meet this sanitisation floor.
  > No anyhow context chains, no absolute paths, no upstream text (prompts, tool `input`, assistant output), no serde_path_to_error paths or values, and no internal type names.
- (.andromeda/security-plan.md:413) This row names the MCP `instance-unreachable` body the chunk moves: a P4 ruling on the code's place must amend it, and the rest of the row must hold (a code word through `is_code_word`, a `refused: <code>` text, `-32603` for any other fault).
  > MCP `isError` results carry only the `structuredContent` codes from Conventions (`refusal`/`detail`, or `error: instance-unreachable | wrapper-fault | invalid-params` — `instance-unreachable` with `detail: null`, `wrapper-fault` with `{code,message,data}` as `viola_channel::sanitised_fault` closes them, never the peer's own, `invalid-params` with a closed field word), a `refusal` that is no code word (`viola_core::is_code_word`) reading `unknown` and a string `detail` that is none reading null, `of_reply` here and `reply_of` on CLI `--json` alike, and every `isError` text is `refused: <code>`. Any other handler fault, a reply of no known shape or a panicked call is the JSON-RPC error `-32603` `internal error` with `data: null` (chunk 2026-10-17-mcp-server-for-drivers).
- (.andromeda/security-plan.md:416) Each cause's `hint:` line goes to stderr and must stay under this floor, including the unwrapped-name hint.
  > stderr never carries upstream text, tool `input`, drift-report values or any other value on the NEVER-log floor (Bootstrap phases, `logging-redaction-wire`).
- (.andromeda/security-plan.md:418) `hook` must stay outside the new per-cause surfaces: a catalogued cause reaching it must add no stderr, body or non-zero exit.
  > `hook` never writes to stderr and never exits non-zero: a security check failing in `hook` still ends in exit 0 with no body, so the dialog renders for the human

## Bindings
- **Code contract ↔ architecture, design-system and test-plan**: the document and MCP body a P4 ruling moves must change together in these plans and in this plan's MCP row, the architecture fold coming before implementation, (.andromeda/security-plan-detail/bootstrap-phases.md:8), (.andromeda/security-plan.md:413).
- **Client check order ↔ the body's Authentication & Authorization**: the detail's strict-modes-then-liveness-then-server order and the body's no-frame, no-fallback rule bind each exit-21 code together, (.andromeda/security-plan-detail/bootstrap-phases.md:15), (.andromeda/security-plan.md:114).
- **Socket-directory refusal ↔ route entry "Unix endpoint and home hardening"**: this chunk may code that refusal, while the verification and the euid and macOS gaps stay with the entry, (.andromeda/security-plan-detail/bootstrap-phases.md:14), (.andromeda/security-plan.md:110).
- **NEVER-log floor ↔ obs-plan §9 and `schemas/diag-line.v1.json`**: a new `process-exit` `detail` value must join the log's closed list and pass the canary scan, holding nothing the floor bars, (.andromeda/security-plan-detail/bootstrap-phases.md:24), (.andromeda/security-plan.md:346).
- **Sanitised surfaces ↔ `tests/contract_error_bodies.rs` and test-plan's Exit-cause matrix**: each new code stays inside the error-body census and is a case of `tests/cross_exit_causes.rs`, its document and hint held to the floor, (.andromeda/security-plan.md:410), (.andromeda/security-plan.md:416).
