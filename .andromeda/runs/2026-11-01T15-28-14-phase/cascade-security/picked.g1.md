### threat-model-summary#1.1 · values · threat-data · ## Threat Model Summary · lines 8–10
- **Type:** user-content (prompt text and assistant output)
  - **Where:** `instances/<ViolaName>/events.ndjson`. `prompt-submitted.data.text` holds every submitted prompt (driver, human and harness), and `turn-ended.data.last_assistant_message` holds each final assistant reply. The wrapper keeps the latest reply in memory for `last`. The same lines go out verbatim over MCP `wait`/`last` and over CLI `wait`/`last` (Standard Contracts: event `data` per kind. Occupied Resources: Filesystem).
  - **Volume:** Per-user, persistent and growing. The file is never rotated or truncated, and bounding its size is an open item (Occupied Resources: Filesystem). Instance directories are reused and appended to, so it builds up across sessions (Established Decisions: Session Liveness).
### threat-model-summary#1.2 · values · threat-data · ## Threat Model Summary · lines 11–13
- **Type:** user-content (tool-call arguments, plans, dialog questions and answers). This may incidentally include secrets or personal data the user's session touches.
  - **Where:** The `permission` event carries `{tool, input}`, where `input` is the tool call's full arguments as JSON. For Bash and Write-type tools that means shell commands and file bodies. `plan` carries the plan text and `question` carries questions, options and each option's description and preview text (`option_details`), which also go out over CLI `wait --json` and MCP `wait`. All of it is logged to `events.ndjson` and sent over the local-socket channel in `hook.dialog` (Standard Contracts: Event `data` per kind, `hook.dialog`).
  - **Volume:** Per-user, persistent, same retention as above.
### threat-model-summary#1.3 · values · threat-data · ## Threat Model Summary · lines 14–26
- **Type:** config (includes a stored executable command string)
  - **Where:**
    - `config.json` holds budget thresholds.
    - `instances/<ViolaName>/settings.json` is the per-session statusline override. It holds `{"statusLine":{"type":"command","command":"<pinned path> hook statusline"}}`, never the user's command.
    - `snapshot.json` stores `statusline_command`, the user-written shell string that `hook statusline` reads through `VIOLA_DIR` and runs through a shell. This is the only shell-out in viola.
    - `<home>/statusline-source.json` (the Claude settings shape; viola only reads it) and, for the default home only, the user's `~/.claude/settings.json` are the inputs that decide `statusline_command` at each start.
    - `snapshot.json` also stores `endpoint`, the socket path clients connect to, and `pinned_bin`.
    - `ledger/stamps.json` holds the verified CLI versions. A stamp is what switches on dialog answering.
    - `plugin/<version>-<hash>/hooks/hooks.json` and `.mcp.json` hold the absolute exec paths Claude Code runs for every hook and for MCP.
    - `marketplace/viola/.mcp.json`, written by `viola plugin install`, holds the absolute exec path Claude Code runs for MCP in a session that installed the plugin.
    - `bin/<version>-<hash>/viola(.exe)` is the pinned executable.
    - Sources: Occupied Resources (Filesystem, Claude Code integration names); Standard Contracts (Hook contract, Instance snapshot); Established Decisions (Deployment / Distribution, CLI Version Compatibility).
  - **Volume:** Per-user, persistent. These are integrity-sensitive more than confidentiality-sensitive, because several of them decide what code runs.
### threat-model-summary#1.5 · values · threat-data · ## Threat Model Summary · lines 30–35
- **Type:** operational metadata. PII is limited to the OS username in paths.
  - **Where:**
    - Snapshots hold `pid`, `child_pid`, `agent_session_id`, `started_at` and the optional `cwd` (the start's spawn directory: an absolute host path that can carry the OS username; it lives in the 0600 snapshot only, on no log line, error body or stdout, and is omitted when absent).
    - `budget.json` holds subscription usage percentages and `resets_at`.
    - Sources: Standard Contracts (Instance snapshot); Occupied Resources (Filesystem).
  - **Volume:** Per-user, low sensitivity.
### threat-model-summary#2.1 · values · boundary-ipc · ## Threat Model Summary · lines 41–61
- **Vector:** Local IPC, the wrapper channel (JSON-RPC 2.0 over ndjson). This is the highest-impact surface.
  - **Entry point:**
    - One endpoint per `viola run`: named pipe `\\.\pipe\viola-<h12>` on Windows, socket file `$TMPDIR/viola-<h12>.sock` (else `/tmp`) on Unix.
    - Methods: `send`, `wait`, `last`, `answer`, `pause`, `release`, `link`, `unlink`, `hook.dialog`, and `hook.event` notifications.
    - Whoever can connect can type prompts into the driven Claude session (`send`) and allow PermissionRequest dialogs (`answer` with `behavior:"allow"`). That amounts to running arbitrary tools, and so arbitrary code, as the user. They can also read the whole conversation (`wait`, `last`).
    - Whoever can connect can also add or remove a driver link (`link` / `unlink`), whose effect is a `link` / `unlink` line in `events.ndjson` and the snapshot's links only; neither method reads the self-reported `from`.
    - Sources: Occupied Resources (IPC endpoints); Cross-cutting Patterns (Local endpoint trust boundary).
  - **Trust boundary:**
    - v1 intends to trust every process of the same OS user and no other. How that is enforced per OS (pipe DACL, socket file mode, directory permissions) is not decided in arch and is explicitly handed to security.
    - There is no per-connection authentication. The `from` parameter is self-reported by the caller (the CLI and MCP take it from their own `VIOLA_NAME`), so link records and driver identity are not authenticated.
    - The endpoint name is predictable: FNV-1a over `ViolaName` + home path, which is not secret (Conventions: Endpoint name).
    - Consequences:
      - On Unix with `TMPDIR` unset, the socket sits in world-writable `/tmp`, so another local user can create the path first.
      - On Windows, the named-pipe namespace is shared across the machine.
      - Clients (`hook`, `send`, `mcp`) connect to the `endpoint` recorded in the snapshot and act on whatever server answers there. A process holding that name while the real wrapper is gone could receive `hook.dialog` payloads and send back dialog decisions.
      - As built, on Linux and Windows every frame-writing client (CLI `send`, `wait`, `last`, `answer`, `pause`, `release`, `link`, `unlink`; `viola hook`'s event and dialog paths; `viola mcp`'s `send` / `wait` / `last` / `answer` tools) compares the connected peer's pid and that process's start time with the snapshot's `pid` and `started_at` before its first frame, and sends no frame on a mismatch or a check it cannot make. Each first runs the instance's strict-modes check (`check_instance`: the CLI verbs through `client::refuse_untrusted`, `viola hook` before its obs init, the `viola mcp` tools before `live_endpoint`), a refusal sending no frame, and then reads the snapshot through `read_snapshot_or_replay`, a replayed or absent snapshot giving no endpoint and so no frame; macOS, where the peer reports no pid, stays a recorded gap (owed to "Unix endpoint and home hardening").
      - `run`'s exclusive bind means a squatted name blocks startup (exit 1).
    - Input validation is limited to JSON-RPC parsing, the `v` check (`-32602`) and the `ViolaName` nutype.
    - That limit is the wrapper's read of a frame. A client's read of the answer is screened on its own: `reply_of` and `of_reply` take a `refusal` or a string `detail` only as a code word (`viola_core::is_code_word`) and a fault only through `viola_channel::sanitised_fault`, so nothing of the peer's but closed values reaches a CLI `--json` document or an MCP result; a code word outside the closed list stays visible by design.
    - An `instance-unreachable` on CLI `--json` or an MCP result names its cause in `detail`, one of five closed words from `viola_core::UnreachableCause` (no serde path, every word written by the two clients, no path, pid or upstream text): `strict-modes-failed`; `instance-dead` (no endpoint with the instance directory standing, or a failed connect); `unwrapped-name` (no endpoint with anything else at that path: absent, a regular file or a symlink); `endpoint-lost` (the wrapper lost with the call open); `server-verify-failed` (off macOS). The no-endpoint reading is `viola_state::liveness::without_endpoint`, a `symlink_metadata` that opens nothing and writes no line, with the strict-modes check first at every caller (its status at § Authentication & Authorization, the no-endpoint reading). MCP writes no line for any cause, `isError` the record.
    - It is open (arch defers it to security) whether a driver-originated `release` (params carrying `from`) should be refused. Established Decisions (MCP) records that an agent's Bash tool can call CLI `viola release` through `VIOLA_BIN`.
### threat-model-summary#5.1 · values · boundary-hook-stdin · ## Threat Model Summary · lines 83–90
- **Vector:** Hook stdin payloads from Claude Code, exec-form `viola hook <event>`.
  - **Entry point:** JSON on stdin for SessionStart, UserPromptSubmit, PreToolUse (`AskUserQuestion|ExitPlanMode`), PermissionRequest, Stop, SessionEnd, Notification, PostToolUse and PostToolUseFailure, plus `hook statusline` with statusline JSON (Established Decisions: Hook Transport; Occupied Resources).
  - **Trust boundary:**
    - Parsing is tolerant in `viola-agent-claude`: no `deny_unknown_fields`, serde_path_to_error for drift reports (Established Decisions: Validation).
    - Upstream text is treated as content, never as commands (R1).
    - Any failure fails open to the human: exit 0 with no body.
    - `hook statusline` runs `statusline_command` from `snapshot.json` through a shell. That snapshot is found through `VIOLA_DIR`, so its integrity is set by filesystem permissions on the viola home.
    - Every hook path (event, dialog and `hook statusline`) runs the instance's strict-modes check on the canonicalised `VIOLA_DIR` instance before anything under the home is read or opened, the `--capture` arms ahead of it; a refused or absent instance exits 0 with no `config.json` read, no role file, no stdin read, no line and no frame. `hook statusline` reads the snapshot through `read_snapshot_or_replay`, and a replayed snapshot carries no `statusline_command`, so nothing runs.
### threat-model-summary#6.1 · values · boundary-mcp · ## Threat Model Summary · lines 96–103
- **Vector:** MCP over stdio. The driver is an LLM agent.
  - **Entry point:** Tools `send`, `wait`, `last`, `answer` and `list`, called by the driver session's model (Established Decisions: MCP).
  - **Trust boundary:**
    - Tool input schemas (schemars) and the `ViolaName` newtype check the input.
    - Before its frame, each of `send`, `wait`, `last` and `answer` runs the instance's strict-modes check (`check_instance` on the home and the instance directory) ahead of the liveness pre-check and the server check; a refusal is `instance-unreachable` with `detail` `strict-modes-failed`, no frame and no line.
    - The driver model reads the driven session's output (`wait`/`last`), which arch classes as untrusted upstream text. The same model can then call `answer` on that session's PermissionRequest.
    - viola does not interpret that text (R1: mechanism, not policy). The decision to allow a permission rests with the driver model, outside viola.
    - Leaving `release`, `pause`, `link` and `unlink` off the MCP tool list is described as an affordance, not an enforcement.
### threat-model-summary#8.1 · values · boundary-cli · ## Threat Model Summary · lines 116–124
- **Vector:** CLI arguments, flags and environment.
  - **Entry point:**
    - All subcommands.
    - `--home <dir>` and `VIOLA_DIR` choose which state tree, snapshot, `endpoint`, `statusline_command` and ledger stamps are used (Cross-cutting Patterns: Config management).
    - `answer` takes its `response` JSON from stdin or `--file`.
    - `viola run <name> -- <program> <args>` spawns the program as given.
  - **Trust boundary:** Same-user invocation, clap parsing, the `ViolaName` nutype, and tolerant `config.json` parsing.
    - Before its first read or open under the home, `run`, `revive`'s start and `verify` hold the home tree to the strict-modes check (`check_home_tree`; a refusal exit 1, no log opened), and the frame verbs `send`, `wait`, `last`, `answer`, `pause`, `release`, `link` and `unlink` hold the instance to it (`check_instance`; a refusal exit 21), so a state tree chosen by `--home` or `VIOLA_DIR` that another user can write, or whose home, instance directory or `diagnostics/` another user can read, is refused.
    - `src/main.rs`'s `json_asked` reads `--json` from argv ahead of clap, as `role_of` reads its role, taking the flag only among the arguments before a `--`. It chooses only the form of the catch site's output where `prints_internal_error` is true: `{"v":1,"error":"internal-error","detail":null}` on stdout with stderr empty, in place of the human line `error: internal error`. `run`, `revive`'s start, `mcp` and `hook` print nothing at the catch site.
### threat-model-summary#9.1 · values · boundary-filesystem · ## Threat Model Summary · lines 130–139
- **Vector:** Filesystem state under `~/.viola/`.
  - **Entry point:**
    - Readers across processes: `snapshot.json`, `events.ndjson`, `budget.json`, `config.json`, `ledger/stamps.json`, `heartbeat`.
    - Code-bearing artefacts: `bin/<version>-<hash>/viola(.exe)` and `plugin/<version>-<hash>/hooks/hooks.json` / `.mcp.json`. Claude Code runs these on every hook event and for MCP. And `marketplace/viola/.mcp.json` (`viola plugin install`, never read back by viola), which Claude Code runs for MCP in a session that installed the plugin.
    - Sources: Occupied Resources (Filesystem); Established Decisions (Deployment / Distribution).
  - **Trust boundary:**
    - OS permissions on the user's home directory.
    - Readers tolerate torn lines (they skip and count them, and the next append heals a torn last line). The replay of the log for a snapshot that fails to parse, or carries a newer `v`, is taken by these product readers: `viola revive`'s preflight, `viola list` (`viola_state::board::read`), `viola_state::liveness::live_endpoint` (the CLI frame verbs and the `viola mcp` tools), `viola hook`'s `connect`, `hook statusline`'s `serve` and `run`'s collision check, each after the strict-modes check on the instance's files; a replayed snapshot carries no `endpoint`, `pid`, `started_at` or `statusline_command`, so it yields no frame and runs no command (Standard Contracts: Snapshot envelope).
    - The pinned binary and plugin folders are never deleted in v1.
    - Beyond the content hash in the folder name, `run` and `revive`'s start hold the home tree to the strict-modes check (Unix owner and mode, Windows owner and DACL) before the wrapper log opens (`check_home_tree`: the home, the instance's files, `ledger/`, `bin/`, `plugin/`, `bin/<version>-<hash>/`, the pinned `viola(.exe)`, `plugin/<version>-<hash>/`, its `hooks/` and `.claude-plugin/`), and on Windows `pin_companions` checks `bin/<version>-<hash>/conpty/` and each companion before the held open; `marketplace/` is off the integrity set.
### threat-model-summary#13.1 · values · threat-infra · ## Threat Model Summary · lines 178–183
- **CI/CD:**
  - GitHub Actions `ci.yml`, triggered on push and PR, on a native matrix of `windows-2025`, `macos-latest` and `ubuntu-latest`; plus `nightly.yml` (weekly `schedule` + `workflow_dispatch`) for `cargo deny check advisories` over the root and fuzz lockfiles, and a time-boxed fuzz run.
  - Jobs: fmt, clippy `-D warnings`, `cargo check` of the sync crates, the cargo-modules orphans gate per lib/bin target, the per-OS `rmcp features` lint step (`scripts/rmcp-features.sh`: rmcp's features in `viola`'s release graph exactly `server transport-io`, rmcp present), the ubuntu supply-chain job (`cargo deny check`, the `fuzz/Cargo.lock` advisories and sources audit, the sole-root tokio ban then `scripts/channel-tokio-free.sh`, the ban probes, zizmor), tests against the fake agent with recorded fixtures (per OS under coverage), the per-OS `perf` job (`agent-run run --perf`), MSRV 1.96, fuzz corpus replay, and the per-OS release build (`cargo build --release --locked --bin viola` through `scripts/release-check.sh`, which refuses any test-only binary). No job of `ci.yml` runs mutation since 2026-09-28: mutation testing moved to the epoch-boundary code audit.
  - No deploy stage, no release artefacts and no secrets in v1. The real `claude` CLI runs only locally; `viola verify` runs in CI only against the fake agent, and its real-CLI probe and `--record` only locally.
  - v1.x adds a dist 0.33.0 release workflow with cargo-auditable 0.7.6, signing, winget/Homebrew/Scoop and `self_update` 1.3.0.
  - Sources: Infrastructure Patterns (CI/CD approach); Stack (Release).
