## Domain Overview
Viola belongs to a fast-growing category of **coding-agent session orchestrators and bridges**. These are local tools that start, watch and steer interactive CLI agents such as Claude Code, Codex, Gemini CLI and OpenCode, so that a human, or another agent, does not have to copy text between terminals or answer every prompt by hand. Viola's niche is narrower than most. One interactive Claude Code session holds real user authority (typed input, answered dialogs) over another live, watchable session, on the user's own subscription, with native Windows support.

Nearby products, as of 2026-09:
- **hcom** (aannoo/hcom, Rust single binary, MIT, about 500 stars): agents message, watch, spawn, fork and kill each other across terminals. Hooks write activity to SQLite and inject messages between tool calls or wake idle agents. It has native Windows install. Its messages arrive as text, not typed input, so they carry no user authority.
- **relay** (fuad-daoud/relay, Go, MIT): automates the plan/report handoff between a Claude Code "planner" and a "builder". It is a plugin (MCP plus SessionStart hook), and the planner wakes on a background `relay wait`. The builder runs **headless `-p`** with no dialogs. Linux and macOS only.
- **Codeman** (julian3xl/codeman, Node 22 + Fastify + xterm.js, MIT): a self-hosted "mission control". It puts each CLI in a real PTY inside tmux, receives hook events at `/api/hook-event`, and has an orchestrator loop. Windows needs WSL.
- **AgentBridge** (raysonmeng/agent-bridge and forks, MIT): keeps Claude Code and Codex as live peers through a localhost daemon, built on Claude Code Channels on one side and the Codex app-server on the other.
- **coder/agentapi**: an HTTP API that turns calls into keystrokes against an in-memory terminal emulator. It was the closest "agent drives agent" precedent and is now **deprecated and archived**, replaced by Coder's own harness ("Coder Agents").
- Human-drives-many managers (context only): ccmanager (TypeScript, own PTY, no tmux), claude-squad (tmux), Orkas (Electron; speaks each CLI's own protocol, e.g. Codex app-server JSON-RPC), psmux (a native Windows tmux clone in Rust on ConPTY; Claude Code agent teams spawn panes inside it).

## Common Approaches
- **Architecture patterns**
  - **Terminal ownership**:
    - tmux `send-keys` (claude-squad, Tmux-Orchestrator, cli-agent-orchestrator, Codeman). This rules out native Windows.
    - An own PTY per session (ccmanager, MulmoTerminal, psmux, agentapi).
    - No PTY at all: hook plus message injection (hcom), or headless `-p` / stream-json workers (relay, many CI-style orchestrators).
  - **State source**: Claude Code hooks posting to a local endpoint (Codeman, hcom, disler's observability stack) have largely replaced screen scraping. Terminal emulators such as vt100, alacritty_terminal or xterm.js headless are kept only for rendering and snapshots.
  - **Process topology**:
    - A single binary with no daemon, keeping state in a file or DB (hcom, relay).
    - A localhost daemon or server (AgentBridge, Codeman, agentapi).
    - A desktop app hosting everything (Orkas, opcode, vibe-kanban).
  - **Local-first everywhere**: state lives on disk, and the servers bind to localhost.
- **Interaction models**:
  - A CLI verb set, e.g. `send`, `wait`, `pull`, `status`.
  - An MCP server exposing the same verbs to the driving agent, usually packaged with its hooks as a Claude Code plugin.
  - A viewer, which comes in three forms:
    - a browser dashboard (Codeman, claudecodeui, disler's dashboard);
    - a TUI (relay `ui`, ccmanager, agent-console);
    - a desktop app (Tauri or Electron).
  - A phone or remote view appears in Happy, Codeman and Claude's own Remote Control and Channels.
- **Common workflows and data flows**:
  - Driver issues a command → bridge delivers it (typed keys, injected message, or a fresh headless process) → hooks report `UserPromptSubmit`, `PreToolUse`, `PermissionRequest`, `Notification` and `Stop` → an event log or DB → the driver wakes, usually via a blocking `wait` command that the Claude Code harness runs in the background, or via a channel push.
  - Dialogs are answered through hooks. A `PreToolUse` hook answers `AskUserQuestion` by returning `updatedInput`, and `PermissionRequest` returns allow or deny. The pattern is documented and widely used for headless and CI automation, and newer builds re-check a rewritten `updatedInput` against the deny rules.

## Technology Landscape
**Languages/Runtimes:**
- **Rust** is common for single-binary, cross-platform terminal tooling: hcom, psmux, agent-console, and the terminal emulators alacritty and wezterm. It gives a static binary with no interpreter, which matters for hooks that spawn once per tool call.
- **Go** is used by relay and many tmux-based orchestrators. Windows PTY support there comes from separate packages (e.g. `aymanbagabas/go-pty`, `UserExistsError/conpty`). The long-standing `creack/pty` has no ConPTY.
- **TypeScript/Node** is used by Codeman, ccmanager, claudecodeui and Happy, usually with Microsoft's `node-pty`, which supports ConPTY. It needs a Node runtime on the host.
- **Python** appears mostly in hook scripts and prototypes (`pywinpty` on Windows). The per-hook interpreter startup cost is a known complaint.

**Frameworks:** (all backend or runtime, none frontend)
- **Rust**: `tokio` (async runtime; the only async runtime `interprocess` supports) plus `axum` or `hyper` for a local HTTP/SSE/WebSocket server. `ratatui` if the view is a TUI.
- **Go**: the standard `net/http`; `bubbletea` for TUIs.
- **Node**: Fastify or Express (Codeman uses Fastify).
- **Desktop shells** (context only, as seen in competitors): Tauri (opcode, MonoCode), Electron (Orkas, polycode).

**Data storage:**
- Append-only **ndjson/JSONL event logs** are common because they are crash-tolerant, greppable, and double as an audit trail. Claude Code's own transcripts are JSONL, but their format is explicitly unstable.
- **Embedded SQLite** is used by hcom and disler's observability stack when queries, subscriptions or concurrent writers matter.
- **tmux itself** acts as the persistence layer in the tmux-based tools.
- Git-backed mailboxes appear in mcp_agent_mail.
- No domain tool uses a server database.

**Key libraries/services:**
- **PTY**:
  - `portable-pty` (part of the wezterm monorepo, 0.9.x). Upstream lags on modern ConPTY flags (PASSTHROUGH_MODE for Windows 11 22H2+, WIN32_INPUT_MODE, RESIZE_QUIRK), so patched forks exist (`portable-pty-psmux`, `psmux/portable-pty-patched`).
  - Alternatives: `alacritty_terminal::tty`, `winpty-rs`, `node-pty`.
- **Screen model**: `vt100` crate, `alacritty_terminal`, `@xterm/headless`.
- **IPC**: the `interprocess` crate (one "local socket" API over Windows named pipes and Unix domain sockets, with an opt-in tokio feature). Tokio also has named pipes and UDS directly.
- **MCP**:
  - Official Rust SDK `rmcp`, now on a 3.x line (reported).
  - Community `rust-mcp-sdk` 2.x, which targets the MCP 2026-07-28 stateless spec.
  - Both do stdio, which Claude Code uses for local MCP servers.
- **Claude Code surfaces**:
  - Hooks: about 30 lifecycle events; the payload includes `last_assistant_message` on `Stop`.
  - Plugins (hooks plus MCP in one install).
  - `claude agents --json`, `--bg`, `attach`, `logs`.
  - Statusline JSON: `rate_limits.five_hour` and `seven_day`, each with `used_percentage` and `resets_at`.
  - Cross-session messaging, Channels, Remote Control.

**Deployment:**
- Tools ship as a native binary through GitHub Releases, with cross-builds via cargo-dist or goreleaser, `cargo install`, winget, Homebrew or Scoop. Node tools ship as npm global packages.
- The Claude Code half is distributed as a **plugin** (a marketplace or `--plugin-dir`).
- Nothing is hosted. At most a localhost web server runs from the same binary.
- WSL-only Windows support is still common and is a stated weak spot in the category.

## Recent Trends
- **The first-party multi-session surface keeps growing, but still has no input channel.**
  - Agent View / `claude agents` and background sessions (`--bg`) arrived as a research preview in May 2026.
  - Channels (MCP push into a live session, with a permission relay) arrived in March 2026.
  - Cross-session messaging (ListAgents/SendMessage) also exists.
  - None of these can type a slash command or carry user authority. Issue #85289, which asked for exactly that, was closed NOT_PLANNED on 2026-09-15.
- **Subscription enforcement against third-party harnesses.**
  - Server-side blocking of unofficial OAuth began 2026-01-09.
  - The legal page was clarified on 2026-02-19.
  - From 2026-04-04, subscriptions stopped covering OpenClaw, OpenCode and similar tools.
  - Wrapping the **unmodified official binary under the user's own login** is the pattern that remains permitted. The "ordinary, individual usage" wording is the remaining pressure point for automation.
- **Screen scraping is falling out of favor.** agentapi (the keystroke plus terminal-emulator approach) was deprecated and archived in Sept 2026. Hook-driven state is now the norm in new tools such as hcom, Codeman and ccmanager.
- **Rust single binaries and native Windows are gaining ground.** Examples: hcom, psmux (native tmux for Windows on ConPTY), agent-console ("no tmux"). WSL-only tools are increasingly seen as a limitation.
- **Plugin packaging (hooks plus an MCP server)** has become the standard way for a bridge to attach to Claude Code (relay, hcom-style integrations).
- **A "background wait" as the wake-up primitive.** The driver runs a blocking command in the background and ends its turn, and the harness wakes it when the command exits (relay's documented pattern). It is an alternative to Channels push.
- **Contrast with other vendors.** OpenAI's Codex offers a supported programmatic protocol (`codex app-server` JSON-RPC) that tools like Orkas and AgentBridge speak. Claude Code's interactive session has no equivalent.
- **Proliferation.** Curated lists (awesome-agent-orchestrators, awesome-cli-coding-agents) now track dozens of orchestrators. The trends they note are state machines over fire-and-forget triggers, and sandbox or container isolation.

## Considerations for This Project
- **Windows ConPTY is the hard part, and upstream `portable-pty` lags it.**
  - Passthrough, win32-input-mode and resize quirks depend on the Windows build and need flags that upstream does not expose. Maintained patched forks exist and are a real option to weigh.
  - Other known traps (already measured in the brief): the npm shim in front of `claude.exe`, EOF not arriving on child exit, and the outer terminal's own input mode.
  - Byte-level pass-through fidelity has to be verified per terminal host: Windows Terminal, conhost, the VS Code terminal.
- **The whole contract rides on a fast-moving CLI.**
  - Hook payloads are documented. Several behaviors Viola depends on are only measured, not documented: `ExitPlanMode` ignores `PermissionRequest` allow, and newer builds re-check `updatedInput` against deny rules.
  - The docs lag the installed build.
  - Decision needed: how to detect incompatible CLI versions at startup, and how the fake-agent CI stays in sync with real CLI behavior.
- **Arbitrating the "wheel" is subtle.**
  - The human's keystrokes and the bridge's paste share one input stream, and a human keystroke can land in the middle of a bridge paste.
  - Mid-turn input is queued by the CLI.
  - `UserPromptSubmit` is the only reliable signal of who submitted a prompt, and the screen lags the hooks.
  - Trade-off: detect the human after the fact (pause on an unexpected prompt), or also gate raw key input while a bridge send is in flight.
- **Budget governance has thin inputs, and the policy stakes are real.**
  - Statusline `rate_limits` is present only for Pro/Max, only after the first response, and each window may be absent.
  - The per-model weekly window shown in `/usage` (Opus/Sonnet/Fable) is **not** exposed (issue #91920, open), and that window often runs out first.
  - Only one statusline command is allowed per user, so wrapping it touches user config.
  - After the 2026 harness crackdown, a transparent, conservative governor also serves as evidence of "ordinary individual usage".
- **The local GUI and IPC are a code-execution surface.**
  - A page on 127.0.0.1 that can type into sessions is exposed to DNS rebinding and cross-origin requests from any browser tab unless it checks Host/Origin and uses a per-launch token.
  - Named pipe ACLs on Windows and socket file permissions on Unix need the same care.
  - A driven session's `Stop` text flows into the driver as content, so prompt injection travels upstream as text. R1 is mitigated only if the driver's policy treats that text as untrusted.
  - Other trade-offs to settle in the quiz: web page vs `ratatui` TUI (phone view later vs simplicity), and ndjson vs embedded SQLite (hcom's choice) for concurrent writers and self-healing state.

Sources:
- [awesome-agent-orchestrators](https://github.com/andyrewlee/awesome-agent-orchestrators)
- [aannoo/hcom](https://github.com/aannoo/hcom)
- [fuad-daoud/relay](https://github.com/fuad-daoud/relay)
- [julian3xl/codeman](https://github.com/julian3xl/codeman)
- [raysonmeng/agent-bridge](https://github.com/raysonmeng/agent-bridge)
- [coder/agentapi](https://github.com/coder/agentapi)
- [Coder Tasks deprecation](https://coder.com/docs/ai-coder/tasks)
- [Orkas](https://github.com/Orkas-AI/Orkas)
- [psmux](https://github.com/psmux/psmux)
- [kbwo/ccmanager](https://github.com/kbwo/ccmanager)
- [portable-pty docs](https://docs.rs/portable-pty)
- [portable-pty-psmux](https://lib.rs/crates/portable-pty-psmux)
- [interprocess](https://docs.rs/interprocess)
- [MCP Rust SDK](https://github.com/modelcontextprotocol/rust-sdk)
- [rust-mcp-sdk](https://crates.io/crates/rust-mcp-sdk)
- [Claude Code statusline docs](https://code.claude.com/docs/en/statusline)
- [claude-code#91920](https://github.com/anthropics/claude-code/issues/91920)
- [Channels reference](https://code.claude.com/docs/en/channels-reference)
- [Agent View overview](https://claudefa.st/blog/guide/agents/agent-view)
- [VentureBeat on harness crackdown](https://venturebeat.com/technology/anthropic-cracks-down-on-unauthorized-claude-usage-by-third-party-harnesses)
- [MindStudio on OpenClaw ban](https://www.mindstudio.ai/blog/anthropic-openclaw-ban-oauth-authentication)
- [Handle approvals and user input](https://code.claude.com/docs/en/agent-sdk/user-input)
- [microsoft/terminal#1173 ConPTY passthrough](https://github.com/microsoft/terminal/issues/1173)
