# arch · Stack and Technologies — category read

## Carried
- (.andromeda/architecture-detail/stack-and-technologies.md:11) Every code, document and hint change on the CLI verbs must stay on std threads, with an async runtime only inside `mcp`.
  > Hybrid: std threads for `run`, `hook`, `send`, `wait` and every other CLI verb; Tokio 1.53.3 built only inside `mcp`
- (.andromeda/architecture-detail/stack-and-technologies.md:12) The catalogue must account for this closed verb list: each verb's exit-21 and exit-1 causes join the table or are named outside it by a reason.
  > Subcommands `run · revive · send · wait · last · list · answer · hook · mcp · verify · pause · release · link · unlink · plugin install`
- (.andromeda/architecture-detail/stack-and-technologies.md:15) The endpoint-lost-mid-call code must read the same through both channel clients, the sync one the CLI uses and the feature-gated Tokio one `viola-mcp` uses, the Tokio flavour staying confined there.
  > interprocess 2.4.4 `local_socket` (sync API; the Tokio flavour, `interprocess/tokio`, only in `viola-channel`'s feature-gated `tokio_client::TokioClient`, used by `viola-mcp` alone
- (.andromeda/architecture-detail/stack-and-technologies.md:17) A log form a cause gains must be a process-log line under the existing schema files, carrying no `v` key, its version held by the schema filename.
  > The exception is process-log lines (home-level role files and instance detail files): they carry no `v`, and their version lives in the schema filename (`schemas/diag-line.v1.json`, `schemas/diag-detail.v1.json`), because obs's default-deny field list admits no `v` key
- (.andromeda/architecture-detail/stack-and-technologies.md:18) The strict-modes causes' negative cases must provoke the refusal in each OS's own form, modes and owner on Unix, owner and DACL on Windows.
  > the strict-modes check and private-dir creation through libc (Unix modes and owner) and windows-sys (owner + DACL, and the protected user + SYSTEM DACL on a home created outside the user's profile, `Win32_UI_Shell` `GetUserProfileDirectoryW`; a DACL that cannot be set fails the creation)
- (.andromeda/architecture-detail/stack-and-technologies.md:20) Every new or moved `--json` document must keep its declared key order through serde_json's `preserve_order`, and the table must assert the document in that order.
  > `preserve_order` keeps a printed document's declared key order (e.g. `{"v":1,"cmd":…,"ok":…}`)
- (.andromeda/architecture-detail/stack-and-technologies.md:21) A cause's log form must be a one-line JSON process log in `diagnostics/`, its line format taken from the obs plan and not decided here.
  > One-line JSON process logs into the viola home's `diagnostics/`; the line format and levels are owned by the obs plan
- (.andromeda/architecture-detail/stack-and-technologies.md:24) Each distinct cause must be a typed value callers can branch on, and a cause's path, pid or context chain must stay in the owner-only instance detail file, never in a code.
  > Typed errors that callers can branch on; context chains built at dispatch and recorded only in the owner-only instance detail file
- (.andromeda/architecture-detail/stack-and-technologies.md:26) The not-running and stale-heartbeat causes must be told apart on the heartbeat as primary signal with pid and start time, `claude agents --json` only enriching and never becoming a cause of its own.
  > sysinfo 0.39.6 (pid + process start time) + `claude agents --json` | Enriches the primary signal, the heartbeat file
- (.andromeda/architecture-detail/stack-and-technologies.md:27) The MCP face of the catalogue must live on these five tools over the pinned rmcp, adding no tool to carry a code.
  > rmcp 3.4.1 (features `server`, `transport-io`), minor pinned; schemars 1.2.2 (tool input schemas) | stdio MCP server implementing spec 2026-07-28 (compatible with 2025-11-25): tools `send · wait · last · answer · list`.
- (.andromeda/architecture-detail/stack-and-technologies.md:27) Assertions moved in `crates/viola-e2e/tests/mcp_tools.rs` must keep this client shape, the test owning the `viola mcp` child.
  > the test-only `viola-e2e` takes rmcp `client` alone, never `transport-child-process` (which kills the server 3 s after a close), its E2E owning the `viola mcp` child and awaiting its exit
- (.andromeda/architecture-detail/stack-and-technologies.md:31) `tests/cross_exit_causes.rs` must run and pass on all three native OSes against the fake agent, an OS-specific case compiled and tested on its own runner.
  > Build, lint and test on all three OSes against the fake agent, on native runners.
- (.andromeda/architecture-detail/stack-and-technologies.md:31) A green local gate must not stand as the chunk's verdict; the CI run does.
  > a pre-push filter, never the verdict of record
- (.andromeda/architecture-detail/stack-and-technologies.md:32) New document and hint writers and every new log line must pass these lints, printing through the existing seams and never by a direct print or tracing level macro.
  > rustfmt, clippy (`-D warnings`, with the workspace `print_stdout` / `print_stderr` / `dbg_macro` bans and `clippy.toml` `disallowed-macros` on the tracing level macros), `cargo check`;
- (.andromeda/architecture-detail/stack-and-technologies.md:32) Any new `process-exit` `detail` value must pass the G4 schema conformance run.
  > jsonschema 0.57.0 (test harness `schema-check`, G4)
- (.andromeda/architecture-detail/stack-and-technologies.md:39) The server-verification code must hold what the server check can read per OS, and macOS, which reports no peer pid, must remain a named gap and not a table case that claims the check.
  > Wrapper IPC: both clients read the serving process's pid from the connected stream through interprocess's `StreamCommon::peer_creds().pid()` (Linux `SO_PEERCRED`, Windows `GetNamedPipeServerProcessId` on the SQOS-adopted client end, none on macOS), the identity each frame-writing client checks before its first frame (§Standard Contracts, Session liveness).
- (.andromeda/architecture-detail/stack-and-technologies.md:45) The server-verification code must carry the mismatch's cause alone, neither the observed peer pid nor its start time crossing a client or the log.
  > Process liveness: sysinfo's process start time is also the observed side of the server check each frame-writing client makes before its first frame (the connected peer's pid and its start time against the snapshot's `pid` and `started_at`; §Standard Contracts, Session liveness).
- (.andromeda/architecture.md:65) Distinct causes must reach the three surfaces as branchable values, and anyhow's chains must stay off every role line, stdout and stderr.
  > Rationale: callers branch on refusal variants across three surfaces, and anyhow's chains serve only the owner's post-mortem, never a role line, stdout or stderr.
- (.andromeda/architecture.md:100) The typed exit set must not grow or shift: causes are told apart inside `1` and `21`, each cause keeping the exit this line gives it.
  > - **CLI exit codes:** `0` ok · `1` internal error, and the start, revive-preflight and verify refusals (a refused home-tree check of `run`, `viola revive`'s start or `verify` among them) · `2` usage error · `10` `human-typing` · `11` `budget-paused` · `12` `unverified-cli` · `13` `not-delivered` · `14` `unknown` refusal · `20` protocol fault from the wrapper · `21` instance unreachable, and a refused strict-modes check of `viola list`, `viola plugin install`, `viola link`, `viola unlink` or a CLI frame verb (`send`, `wait`, `last`, `answer`, `pause`, `release`).
- (.andromeda/architecture.md:104) An MCP result carrying an exit-cause code must stay an `isError: true` result shaped like CLI `--json`, never a JSON-RPC error.
  > - **MCP:** a success is a tool result with `isError: false` and the channel `ok` payload as `structuredContent`; a refusal, an unreachable instance, a wrapper protocol fault and an argument that fails re-validation (`invalid-params`, no frame written) are `isError: true` results shaped like CLI `--json`; JSON-RPC errors are only the MCP connection's own faults: an unknown tool name (`-32602`) and an internal handler fault (`-32603`).
- (.andromeda/architecture.md:226) Telling the exit-21 causes apart moves this one document, so the chosen shape must amend this line and hold in every reader of it.
  > - `{"v":1,"error":"instance-unreachable","detail":null}` (exit 21)
- (.andromeda/architecture.md:228) The exit-1 document's form must be the one new start-refusal documents take: written by `client::refused_document`, codes only, stderr empty.
  > - `{"v":1,"error":"<code>","detail":null}` (exit 1): `viola revive --json` refusing its strict-modes check or its preflight, `<code>` one of `strict-modes-failed` · `already-live` · `no-session` · `session-live` · `cwd-missing`, written by `client::refused_document` (codes only: no path, pid or id), stderr empty.
- (.andromeda/architecture.md:228) This chunk owes the document for each start refusal after a passed preflight, and the line must be amended when it lands.
  > A refusal `run`'s start makes after a passed preflight (`batch-script-child`, `pinned-hash-mismatch`, `squatted-name`, a `claude` not on `PATH` as `internal-error`) prints no document as landed; its document is owed to the working-route entry "Exit-cause code catalogue".
- (.andromeda/architecture.md:229) `viola plugin install`'s refusal prints the exit-21 document and must join the table or be named outside it, moving with any change to that document.
  > its refused strict-modes check prints the `instance-unreachable` document (exit 21) through `client::refused_document`.
- (.andromeda/architecture.md:320) A home-tree refusal must keep writing no line, so its code reaches `--json` and the hint while its log absence stands as a named decision.
  > a refusal exits 1 with the home-state `unable:` / `hint:` pair (the instance-state pair for a refusal inside the instance), opens no log, writes no line and starts no child
- (.andromeda/architecture.md:327) A refused instance-file check must keep writing no line, the catalogue naming that absence and never moving the check behind a log.
  > a folder before what it holds; a refused check writes no line.
- (.andromeda/architecture.md:361) Any `process-exit` `detail` value added must land in the default-deny schema and stay pinned by its contract test.
  > - `schemas/diag-line.v1.json` and `schemas/diag-detail.v1.json`: the obs-owned default-deny line schemas for home-level process-log lines and instance detail lines. Each is self-contained (no cross-file `$ref`), and both are pinned by `tests/contract_diag_schema.rs`.
- (.andromeda/architecture.md:412) Code the chunk adds on the `run`, `send` and channel paths must use std threads and blocking I/O.
  > - **Tokio containment.** New code on the `run`, `hook`, `send` or channel-server paths uses std threads and blocking I/O. Only `viola-mcp` may build a runtime.

## Bindings
- **Logging and process-log version ↔ obs plan and the body's schema line**: the log forms new causes take sit under the obs-owned default-deny schema and pass G4, while the stack only fixes the one-line JSON form and the no-`v` rule (.andromeda/architecture-detail/stack-and-technologies.md:17), (.andromeda/architecture-detail/stack-and-technologies.md:21), (.andromeda/architecture-detail/stack-and-technologies.md:32), (.andromeda/architecture.md:361)
- **Error types ↔ security plan's error rules and the body's [Error Handling]**: typed, branchable causes on the surfaces with chains kept to the owner-only detail file are the stack's side of the security plan's no-path, no-pid rule (.andromeda/architecture-detail/stack-and-technologies.md:24), (.andromeda/architecture.md:65)
- **Serialization ↔ the body's CLI `--json` output**: the key order the stack's feature keeps is the order of every document the contract names, moved or new (.andromeda/architecture-detail/stack-and-technologies.md:20), (.andromeda/architecture.md:226), (.andromeda/architecture.md:228)
- **Concurrency and Wrapper IPC ↔ the body's Tokio containment**: the mid-call reading is written once per client, the sync client under the containment rule and the Tokio client inside `viola-mcp` alone (.andromeda/architecture-detail/stack-and-technologies.md:11), (.andromeda/architecture-detail/stack-and-technologies.md:15), (.andromeda/architecture.md:412)
- **Server-check inputs ↔ the body's instance snapshot and the route entry "Unix endpoint and home hardening"**: the peer pid and start time the stack reads are compared with the snapshot's fields, the macOS gap staying with that entry (.andromeda/architecture-detail/stack-and-technologies.md:39), (.andromeda/architecture-detail/stack-and-technologies.md:45), (.andromeda/architecture.md:158)
- **Process liveness ↔ the body's Session liveness**: the agents read never errors, so it can give no cause in the catalogue (.andromeda/architecture-detail/stack-and-technologies.md:26), (.andromeda/architecture.md:246)
- **CI matrix ↔ test plan's Exit-cause matrix**: the table's per-OS cases run on the matrix's native runners, the CI run being the verdict the chunk reads (.andromeda/architecture-detail/stack-and-technologies.md:31)
- **Exit-1 document ↔ design system's `hint:` lines**: each code matches the human hint of the same cause, and the document's stderr stays empty under `--json` (.andromeda/architecture.md:228)
