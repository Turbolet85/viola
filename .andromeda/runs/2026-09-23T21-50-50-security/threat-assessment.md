## 1. Data Classification

- **Type:** user-content (prompt text and assistant output)
  - **Where:** `instances/<ViolaName>/events.ndjson`. `prompt-submitted.data.text` holds every submitted prompt (driver, human and harness), and `turn-ended.data.last_assistant_message` holds each final assistant reply. The wrapper keeps the latest reply in memory for `last`. The same lines go out verbatim over SSE `/api/events` (`data:` is the event line), over MCP `wait`/`last` and over CLI `wait`/`last` (Standard Contracts: event `data` per kind, SSE feed. Occupied Resources: Filesystem).
  - **Volume:** Per-user, persistent and growing. The file is never rotated or truncated, and bounding its size is an open item (Occupied Resources: Filesystem). Instance directories are reused and appended to, so it builds up across sessions (Established Decisions: Session Liveness).

- **Type:** user-content (tool-call arguments, plans, dialog questions and answers). This may incidentally include secrets or personal data the user's session touches.
  - **Where:** The `permission` event carries `{tool, input}`, where `input` is the tool call's full arguments as JSON. For Bash and Write-type tools that means shell commands and file bodies. `plan` carries the plan text and `question` carries questions and options. All of it is logged to `events.ndjson`, streamed over SSE, and sent over the local-socket channel in `hook.dialog` (Standard Contracts: Event `data` per kind, `hook.dialog`).
  - **Volume:** Per-user, persistent, same retention as above.

- **Type:** config (includes a stored executable command string)
  - **Where:**
    - `config.json` holds budget thresholds and the GUI port.
    - `instances/<ViolaName>/settings.json` is the per-session statusline override.
    - `snapshot.json` stores `statusline_command`, the user-written shell string that `hook statusline` reads through `VIOLA_DIR` and runs through a shell. This is the only shell-out in viola.
    - `snapshot.json` also stores `endpoint`, the socket path clients connect to, and `pinned_bin`.
    - `ledger/stamps.json` holds the verified CLI versions. A stamp is what switches on dialog answering.
    - `plugin/<version>-<hash>/hooks/hooks.json` and `.mcp.json` hold the absolute exec paths Claude Code runs for every hook and for MCP.
    - `bin/<version>-<hash>/viola(.exe)` is the pinned executable.
    - Sources: Occupied Resources (Filesystem, Claude Code integration names); Standard Contracts (Hook contract, Instance snapshot); Established Decisions (Deployment / Distribution, CLI Version Compatibility).
  - **Volume:** Per-user, persistent. These are integrity-sensitive more than confidentiality-sensitive, because several of them decide what code runs.

- **Type:** credential (transient, inherited from the environment, never stored by viola)
  - **Where:** The `viola run` wrapper process's environment holds `CLAUDE_CODE_MESSAGING_TOKEN`, `CLAUDE_CODE_MESSAGING_SOCKET` and other `CLAUDE*` parent-identity variables, which are stripped from the child's environment (R8 strip list, Occupied Resources: Environment variables). The Claude subscription credentials belong to the unmodified `claude` CLI, and viola makes no model API calls (Stack: AI/ML N/A). Cross-cutting Patterns (Config management): "There are no secrets in v1."
  - **Volume:** Transient, in process memory only.

- **Type:** operational metadata. PII is limited to the OS username in paths.
  - **Where:**
    - `/api/info` exposes `pid`, `bind`, `viola_home` (for example `C:/Users/<user>/.viola`) and `verified_cli_versions`.
    - Snapshots hold `pid`, `child_pid`, `agent_session_id` and `started_at`.
    - `budget.json` holds subscription usage percentages and `resets_at`.
    - `/api/sessions` shows unwrapped session identifiers from `claude agents --json`.
    - Sources: Standard Contracts (Service info, GUI list envelope, Instance snapshot); Occupied Resources (Filesystem).
  - **Volume:** Per-user, low sensitivity.

- **Type:** payment / health / multi-user PII: none. Stack lists no payment, identity or health SDKs, there is no database and no accounts (Stack; Established Decisions: Personal v1).

## 2. Attack Surface

- **Vector:** Local IPC, the wrapper channel (JSON-RPC 2.0 over ndjson). This is the highest-impact surface.
  - **Entry point:**
    - One endpoint per `viola run`: named pipe `\\.\pipe\viola-<h12>` on Windows, socket file `$TMPDIR/viola-<h12>.sock` (else `/tmp`) on Unix.
    - Methods: `send`, `wait`, `last`, `answer`, `pause`, `release`, `link`, `unlink`, `hook.dialog`, and `hook.event` notifications.
    - Whoever can connect can type prompts into the driven Claude session (`send`) and allow PermissionRequest dialogs (`answer` with `behavior:"allow"`). That amounts to running arbitrary tools, and so arbitrary code, as the user. They can also read the whole conversation (`wait`, `last`).
    - Sources: Occupied Resources (IPC endpoints); Cross-cutting Patterns (Local endpoint trust boundary).
  - **Trust boundary:**
    - v1 intends to trust every process of the same OS user and no other. How that is enforced per OS (pipe DACL, socket file mode, directory permissions) is not decided in arch and is explicitly handed to security.
    - There is no per-connection authentication. The `from` parameter is self-reported by the caller (the CLI and MCP take it from their own `VIOLA_NAME`), so link records and driver identity are not authenticated.
    - The endpoint name is predictable: FNV-1a over `ViolaName` + home path, which is not secret (Conventions: Endpoint name).
    - Consequences:
      - On Unix with `TMPDIR` unset, the socket sits in world-writable `/tmp`, so another local user can create the path first.
      - On Windows, the named-pipe namespace is shared across the machine.
      - Clients (`hook`, `send`, `mcp`) connect to the `endpoint` recorded in the snapshot and act on whatever server answers there. A process holding that name while the real wrapper is gone could receive `hook.dialog` payloads and send back dialog decisions.
      - `run`'s exclusive bind means a squatted name blocks startup (exit 1).
    - Input validation is limited to JSON-RPC parsing, the `v` check (`-32602`) and the `ViolaName` nutype.
    - It is open (arch defers it to security) whether a driver-originated `release` (params carrying `from`) should be refused. Established Decisions (MCP) records that an agent's Bash tool can call CLI `viola release` through `VIOLA_BIN`.

- **Vector:** CLI input, the `send` text written into the PTY.
  - **Entry point:** A bridge send is one `ESC[200~…ESC[201~` + CR write of text from stdin, `--file` or MCP `send.text` (Established Decisions: Human Takeover / Wheel).
  - **Trust boundary:** Arch does not say whether sent text is checked for an embedded bracketed-paste terminator (`ESC[201~`) or other control sequences. Text containing one would end the paste early, and the rest would reach the `claude` TUI as typed keystrokes. The only post-hoc check is delivery confirmation, which compares `prompt-submitted` text with the sent text (Established Decisions: Delivery Confirmation).

- **Vector:** Local HTTP on loopback, view-only GET + SSE.
  - **Entry point:**
    - `viola ui` on `127.0.0.1:47319`.
    - Routes: `/`, `/assets/*`, `/health`, `/ready`, `/api/info`, `/api/sessions`, `/api/links`, and SSE `/api/events`, which carries raw event lines including prompts, assistant output and permission `input`.
    - `/api/events` accepts a client-supplied `Last-Event-ID` cursor (`<ViolaName>:<offset>,…`).
    - Reserved for v1.x, not served in v1: `POST /api/sessions/{name}/pause` and `POST /api/sessions/{name}/unlink`.
    - Sources: Occupied Resources (Network, HTTP routes); Standard Contracts (SSE feed).
  - **Trust boundary:**
    - Checks in place: bind to 127.0.0.1 only, never `0.0.0.0` or `::`; a Host allowlist of `127.0.0.1:<port>` and `localhost:<port>` against DNS rebinding; GET only, with 405 for any other method.
    - There is no token and no CSRF protection (Established Decisions: GUI Control Scope).
    - Loopback TCP is reachable by every local process, including other OS users' processes. So this surface does not follow the same-OS-user boundary set for the IPC endpoint: another local user can read the full event feed.
    - Arch does not specify a CORS policy.
    - Cursor parsing: instance names in `Last-Event-ID` are declared as `ViolaName`, whose charset is `[a-z0-9-]`, but arch does not say the header is parsed through the newtype before it reaches filesystem lookups.
    - The page renders untrusted upstream text: prompts, `last_assistant_message`, plan text and tool `input` (Cross-cutting Patterns: Untrusted upstream text). The frontend framework, and so its output-encoding behaviour, is deferred to design.
    - For the v1.x POST routes, arch requires that "no state-changing route is reachable cross-origin or without per-launch proof that the caller is the local user", with the mechanism owned by security.

- **Vector:** Hook stdin payloads from Claude Code, exec-form `viola hook <event>`.
  - **Entry point:** JSON on stdin for SessionStart, UserPromptSubmit, PreToolUse (`AskUserQuestion|ExitPlanMode`), PermissionRequest, Stop, SessionEnd, Notification, PostToolUse and PostToolUseFailure, plus `hook statusline` with statusline JSON (Established Decisions: Hook Transport; Occupied Resources).
  - **Trust boundary:**
    - Parsing is tolerant in `viola-agent-claude`: no `deny_unknown_fields`, serde_path_to_error for drift reports (Established Decisions: Validation).
    - Upstream text is treated as content, never as commands (R1).
    - Any failure fails open to the human: exit 0 with no body.
    - `hook statusline` runs `statusline_command` from `snapshot.json` through a shell. That snapshot is found through `VIOLA_DIR`, so its integrity is set by filesystem permissions on the viola home.

- **Vector:** MCP over stdio. The driver is an LLM agent.
  - **Entry point:** Tools `send`, `wait`, `last`, `answer` and `list`, called by the driver session's model (Established Decisions: MCP).
  - **Trust boundary:**
    - Tool input schemas (schemars) and the `ViolaName` newtype check the input.
    - The driver model reads the driven session's output (`wait`/`last`), which arch classes as untrusted upstream text. The same model can then call `answer` on that session's PermissionRequest.
    - viola does not interpret that text (R1: mechanism, not policy). The decision to allow a permission rests with the driver model, outside viola.
    - Leaving `release`, `pause`, `link` and `unlink` off the MCP tool list is described as an affordance, not an enforcement.

- **Vector:** CLI arguments, flags and environment.
  - **Entry point:**
    - All subcommands.
    - `--home <dir>` and `VIOLA_DIR` choose which state tree, snapshot, `endpoint`, `statusline_command` and ledger stamps are used (Cross-cutting Patterns: Config management).
    - `answer` takes its `response` JSON from stdin or `--file`.
    - `viola run <name> -- <program> <args>` spawns the program as given.
  - **Trust boundary:** Same-user invocation, clap parsing, the `ViolaName` nutype, and tolerant `config.json` parsing.

- **Vector:** Filesystem state under `~/.viola/`.
  - **Entry point:**
    - Readers across processes: `snapshot.json`, `events.ndjson`, `budget.json`, `config.json`, `ledger/stamps.json`, `heartbeat`.
    - Code-bearing artefacts: `bin/<version>-<hash>/viola(.exe)` and `plugin/<version>-<hash>/hooks/hooks.json` / `.mcp.json`. Claude Code runs these on every hook event and for MCP.
    - Sources: Occupied Resources (Filesystem); Established Decisions (Deployment / Distribution).
  - **Trust boundary:**
    - OS permissions on the user's home directory.
    - Readers tolerate torn lines and replay the log when a snapshot fails to parse (Standard Contracts: Snapshot envelope).
    - The pinned binary and plugin folders are never deleted in v1.
    - Nothing checks integrity beyond the content hash in the folder name.

- **Vector:** Child process spawning and PATH resolution.
  - **Entry point:**
    - `list`, `mcp` and `ui` spawn `claude agents --json`, resolving `claude` from their own PATH with npm-shim → `claude.exe` resolution.
    - `run` resolves the child executable and runs it once with `--version`.
    - `run` puts the pinned copy's folder first on the child's PATH.
    - Sources: Standard Contracts (Session liveness); Established Decisions (CLI Version Compatibility, Deployment / Distribution).
  - **Trust boundary:** Whatever the calling process's PATH resolves to. Output parsing is tolerant, and failure becomes `unknown`.

- **Vector:** Supply chain (build and CI).
  - **Entry point:**
    - Cargo dependencies. Arch notes interprocess has a single main maintainer and rmcp releases nearly weekly with its minor version pinned.
    - Third-party GitHub Actions: `dtolnay/rust-toolchain@stable` and `Swatinem/rust-cache@v2.9.2`.
    - Future v1.x public distribution and `self_update`.
    - Sources: Stack; Infrastructure Patterns (Build system, CI/CD approach).
  - **Trust boundary:** `cargo deny check` (licences, C-crate bans, tokio ban), `Cargo.lock`, and `publish = false`. In v1 there is no release, no signing and no deploy stage.

- **Public internet API / file upload / OAuth / WebSocket / webhook:** none. Stack and Conventions list no public listener, no WebSocket ("No WebSocket and no polling endpoints in v1") and no hosting.

## 3. Auth Model

- **Approach:** none for v1 (as of 2026-09-23). The only principal is the local OS user.
  - Reason: Project Intent and Established Decisions (Personal v1) describe a single-user local tool with "no accounts, no hosting". The GUI is view-only.
  - Access control therefore depends entirely on OS-level isolation: who can connect to each IPC endpoint and who can read or write `~/.viola/`. Cross-cutting Patterns (Local endpoint trust boundary) sets the target principal as "every process of the same OS user and no other".
  - Arch has decided no enforcement mechanism for that boundary on any OS. That decision is outstanding and belongs to this specialist.
  - The loopback HTTP GUI currently has no principal check, so it does not match the same-OS-user boundary (Section 2).
- **Approach for v1.x brake (state-changing GUI routes):** a per-launch local secret proving the caller is the local user (API-key style, bound to one `viola ui` launch).
  - Reason: Established Decisions (GUI Control Scope) requires "per-launch proof that the caller is the local user" and no cross-origin reachability.
  - There are no user accounts, so session cookies with login, JWT or OAuth have no identity provider or user store to bind to.
- **Provider:** self-implemented. Stack contains no auth framework or identity SDK: axum and tower-http are listed without any auth layer.
- **Scope:** per-OS-user for v1 (IPC and filesystem). Per-launch for the v1.x brake token. Remote or phone access behind authentication is a later version and is N/A in this assessment (Project Intent: Scale path; Stack: Mobile N/A).
- **Development Style context:** agent-driven (Cross-cutting Patterns). The main automated callers are LLM driver sessions using MCP and the CLI, with no interactive login. That matches an ambient OS-user trust model rather than a credential flow.

## 4. Infrastructure

- **Hosting:** local-only. Installed with `cargo install --path .`. No Docker, Compose, Kubernetes, serverless or PaaS (Infrastructure Patterns: Deployment model; Established Decisions: Hosting).
- **Database:** none. State is embedded flat files: ndjson append logs plus atomically replaced JSON snapshots, with std `File::lock` on sibling `.lock` files, under `~/.viola/` or `--home` (Stack: Database; Established Decisions: Database / State Store). Nothing is encrypted, and all of it is plaintext for readability by design.
- **Networking:** local only.
  - TCP loopback `127.0.0.1:47319`, never `0.0.0.0` or `::`.
  - Per-instance named pipes (Windows) or Unix domain sockets (`$TMPDIR` or `/tmp`); no Linux abstract namespace.
  - MCP over stdio.
  - No outbound network calls by viola itself. The `claude` child handles the network to Anthropic on its own.
  - Sources: Occupied Resources (Network, IPC endpoints); Stack (AI/ML N/A).
- **CI/CD:**
  - GitHub Actions `ci.yml`, triggered on push and PR, on a native matrix of `windows-2025`, `macos-latest` and `ubuntu-latest`.
  - Jobs: fmt, clippy `-D warnings`, `cargo check` of the sync crates, `cargo deny check` (on ubuntu), tests against the fake agent with recorded fixtures, and `cargo build --release`.
  - No deploy stage, no release artefacts and no secrets in v1. The real `claude` CLI and `viola verify` run only locally.
  - v1.x adds a dist 0.33.0 release workflow with cargo-auditable 0.7.6, signing, winget/Homebrew/Scoop and `self_update` 1.3.0.
  - Sources: Infrastructure Patterns (CI/CD approach); Stack (Release).

## 5. Compliance Triggers

None: no compliance-regulated data detected.
- No payment data (no payment SDK in Stack).
- No health data.
- No children's data.
- No collection of personal data from third parties. The tool has no accounts and no hosting, and all logged content is the operator's own Claude session content kept on their own machine (Established Decisions: Personal v1; Project Intent).
- The v1.x public distribution in Project Intent (Scale path) still keeps all data local on each user's machine, so it adds no trigger in this assessment.

## 6. Security Tier

**Tier: Minimal (0), with targeted elevations for the local privilege boundary.**

**Justification:** viola is a local-only, single-user tool. It has no accounts, no public network listener, no database, no stored credentials and no regulated data (Sections 1, 3, 4, 5), which rules out Standard's user-account and HTTPS concerns and Hardened's compliance drivers.

It still goes beyond a plain Minimal profile in three ways:
- **Local IPC endpoint.** It is effectively a code-execution interface: `send` types prompts, and `answer` can allow PermissionRequest tool calls. It currently has no decided per-OS enforcement of its same-OS-user boundary, and its names are predictable in shared namespaces (Windows pipes, `/tmp`).
- **Loopback HTTP GUI.** It streams persistent, never-rotated prompt and assistant content, including raw tool arguments, to any local process without authentication. It renders that untrusted upstream text in a browser, and it gains state-changing routes in v1.x.
- **Code-bearing state files.** `snapshot.json`'s `statusline_command`, `hooks.json` exec paths, the pinned binaries and ledger stamps decide what code runs and whether dialogs are answered.

So the baseline is Minimal (dependency audit, input validation on config and external payloads, error sanitization). On top of it, specific local-boundary items are assessed:
- IPC endpoint access control and server impersonation.
- Bracketed-paste breakout in `send.text`.
- GUI output encoding.
- GUI cross-user and cross-origin readability.
- v1.x per-launch brake auth.
- Integrity of `~/.viola/`.

Each of these traces to Section 2.
