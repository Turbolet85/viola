# Viola — prior art (2026-09-23)

Companion to `viola-brief.md`. Built from two web research briefs run the same day, then checked:
every repository row below was verified through the GitHub API (exists · archived or not · last push ·
stars · licence), and the claims that position viola were spot-checked in the READMEs. A claim marked
*(reported)* comes from a research brief and was NOT confirmed by that check.

## 1. Tools that control coding-agent terminal sessions

| Project | Status (gh, 2026-09-23) | Windows | How input reaches the agent | How state is read | Agent drives agent? |
|---|---|---|---|---|---|
| coder/agentapi | **ARCHIVED**, last push 2026-09-13, MIT, 1.5k★ | native exe shipped *(reported)* | HTTP API → keystrokes | README: *"runs an in-memory terminal emulator… translates API calls into appropriate terminal keystrokes and parses the agent's outputs into individual messages"* | **yes** — README: *"a backend in an MCP server that lets one agent control another coding agent"* |
| awslabs/cli-agent-orchestrator | active, 2026-09-23, Apache-2.0, 1.3k★ | no — README requires *"tmux 3.3 or later"* | tmux | HTTP API + terminal streaming *(reported)* | **yes** — supervisor → workers, Handoff/Assign/SendMessage over MCP *(reported)* |
| Jedward23/Tmux-Orchestrator | stale, last push 2025-07-14, no licence, 1.8k★ | no (tmux scripts) | `tmux send-keys` | agents read panes, scheduled polling *(reported)* | yes — orchestrator → PM → engineer *(reported)* |
| smtg-ai/claude-squad | active, 2026-08-20, AGPL-3.0, 8.5k★ | needs tmux → WSL *(reported)* | tmux session per agent | tmux pane + diff | no — a human drives many |
| kbwo/ccmanager | active, 2026-09-13, MIT, 1.2k★ | yes, no tmux *(reported)* | own PTY | status-change hooks *(reported)* | no |
| receptron/mulmoterminal | active, 2026-09-23, MIT, 226★ | macOS + Windows *(reported)* | a real PTY per grid cell; tmux optional, for persistence (README) | README: cells coloured *working / done / needs you*; via Claude Code hooks *(reported)* | no — a human drives many |
| slopus/happy | active, 2026-09-22, MIT, 23.9k★ | not stated | wraps `claude` as a subprocess, relays from the phone | streams terminal state to a mobile app *(reported)* | no — a human on a phone |
| siteboon/claudecodeui | active, 2026-09-23, AGPL-3.0, 13.8k★ | installer shipped *(reported)* | reads and writes the same `~/.claude` sessions *(reported)* | the same files | no |
| winfunc/opcode (ex-Claudia) | active, 2026-09-18, AGPL-3.0, 22.4k★ | yes, WebView2 *(reported)* | Tauri GUI over local sessions | GUI-level | no |
| BloopAI/vibe-kanban | active, 2026-09-19, Apache-2.0, 28.2k★ | yes *(reported)* | a branch + terminal per task | not detailed | no — a human plans a board |
| imbue-ai/sculptor | active, 2026-09-22, MIT, 232★ | no build *(reported)* | a container per agent | not detailed | no |
| stravu/crystal | last push 2026-02-26, MIT, 3.1k★ — renamed closed-source Nimbalyst *(reported)* | not stated | own terminal per worktree *(reported)* | not detailed | no |
| conductor.build | Mac app, no public repo *(reported)* | no — Mac only *(reported)* | integrated terminal per worktree | not published | no |

## 2. Events, messaging, and Anthropic's own surfaces

| Surface | Status | What it gives | What it leaves out for viola |
|---|---|---|---|
| disler/claude-code-hooks-multi-agent-observability | active, last push 2026-02-08, 1.5k★ | hooks → HTTP → server → SQLite → WebSocket → dashboard; every hook type *(reported)* | observation only — no control path |
| Dicklesworthstone/mcp_agent_mail | active, 2026-09-22, 2.2k★ | agent mailboxes and advisory file leases over MCP, git-backed *(reported)* | advisory by design — *"no agent can directly command another"* *(reported)* |
| steipete/claude-code-mcp | **ARCHIVED**, 2026-05-15, 1.3k★ | Claude Code exposed as an MCP tool for another agent | one-shot processes; defaults to skipping permissions *(reported)* — the anti-pattern viola avoids |
| Cross-session messaging (SendMessage / ListAgents) | shipped (docs) | named local peers, text delivery, idle wake | *"a command in the message's text… arrives as plain text. Claude Code never executes it"* |
| Agent teams | experimental (docs) | lead + teammates, mailboxes, a shared task list | teammates cannot approve or reconfigure on another's word; in-process teammates do not survive `/resume` |
| Channels | research preview (docs) | MCP push into a running session; a "permission relay" lets a remote approver allow or deny tool use *(reported)* | cannot carry a slash command; events only while the session is open |
| Remote Control | shipped (docs) | a human steers a local session from phone or web, `/clear` and `/compact` included | human-only; no programmatic entry point |
| Background sessions (`--bg`, `attach`, `logs`, `agents --json`) | shipped (measured, CLI 2.1.280) | detach, reattach, read recent output, list every session as JSON | no way to send input to a running session |
| **issue anthropics/claude-code#85289** | **closed NOT_PLANNED, 2026-09-15** (gh, verified) | asks for exactly viola's mechanism: *"no supported, first-party way for an authorized local process to deliver a turn or a control command into a specific running session while preserving the live interactive window"* | — the gap, stated by its latest requester and declined |

## 3. What viola borrows

- **Hooks as the state source.** At least two tools classify sessions from Claude Code's own signals
  rather than the screen *(reported for ccmanager and MulmoTerminal)*; the spike measured the same
  signals first-hand (brief §4.1).
- **Wrap the real, unmodified CLI and never touch the login.** Happy and claudecodeui keep the user's
  own subscription sign-in by launching the real binary — exactly the shape the legal page permits.
- **A modest GUI.** MulmoTerminal's grid — one cell per session, coloured working / done / needs you —
  is the closest analogue to viola's minimal page; viola adds the links between sessions.
- **Anthropic's own trust language.** "A command in a message never executes", "cannot approve on
  another's word" is viola's R1, already worded by the platform.

## 4. What viola avoids

- **Screen parsing as the primary channel.** agentapi — the one open tool that let an agent drive an
  agent through keystrokes and a terminal emulator — is archived; its README had to explain what
  happens when the wrapped TUI changes *(the related capture breakage on a Claude Code point release is
  reported)*. Viola parses no screen for content (R7).
- **tmux as the ownership primitive.** Three of the tools that let an agent drive an agent need tmux,
  which has no native Windows build. Viola owns its PTY on every OS.
- **Blanket approval.** Auto-yes flags and skip-permissions defaults remove the one checkpoint a human
  relied on. Viola answers each prompt by policy and escalates the rest.
- **Overclaiming.** The GUI shows only what the hooks and the wrapper observed.
- **Fragile shared state.** Mailbox and lease files broke in more than one of these systems
  *(reported)*: viola's on-disk state parses defensively and heals itself from day one.

## 5. Where viola genuinely differs

No surveyed tool combines all of: cross-platform with Windows first · its own PTY · hooks both for
EVENTS and for ANSWERING dialogs · one agent driving another while both stay live, watchable
terminals · an explicit wheel rule for when the human and the driver both reach for the keyboard (no
precedent found) · the user's own subscription through the unmodified binary. Each piece exists
somewhere; the combination does not — and #85289 shows the platform will not supply it.

## 6. Corrections the verification made to the research briefs

- One brief quoted the legal page as saying subscription OAuth in the Agent SDK *"is not permitted"*.
  The page's actual words: developers of products *"including those using the Agent SDK, should use
  API key authentication"*, and *"Anthropic does not permit third-party developers to offer Claude.ai
  login into their own applications"*. The verbatim text is in the brief §7.
- MulmoTerminal's use of hooks and cli-agent-orchestrator's delegation verbs were not found by the
  README spot-check; both stay *(reported)*.
- Issue #85289 states that messaging is not available on native Windows, while `ListAgents` lists peers
  on this Windows host; sending was not measured (brief O3).
