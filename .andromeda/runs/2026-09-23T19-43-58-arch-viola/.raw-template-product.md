## Quiz I Fields

### 1. Scale Intent [FIRST]
- **personal**: one founder, one Windows host, sessions on the founder's own subscription. No accounts, no hosting, and the GUI stays on 127.0.0.1. This fits the first consumer (the Andromeda overseer driving a builder) and the policy wording "ordinary, individual usage". The trade-off: hardening for other hosts (other terminals, a fresh untrusted folder, a different CLI version) can wait until someone else installs it.
- **startup**: a public open-source release for other individual Claude Code subscribers. Each user runs it locally, so there is still no server. This means signed binaries through GitHub Releases, winget, Homebrew or Scoop, a plugin marketplace entry, a CLI-version compatibility check, and defensive on-disk state from day one. Same product shape, but a much higher support and compatibility bar.
- **production**: a widely distributed tool that people rely on for unattended automation, with guarantees across Windows Terminal, conhost, the VS Code terminal, macOS and Linux, and tracking of every Claude Code point release. The catch is that the product sits on an unsupported surface (issue #85289 was closed NOT_PLANNED) and a fast-moving CLI, so it cannot honestly promise production stability.

### 2. Platform
- **Hybrid: a native CLI binary (`run` · `send` · `wait` · `hook` · `mcp` · `ui`), a Claude Code plugin (hooks plus a stdio MCP server) that calls that binary, and a minimal local web GUI served by the same binary** [inferred]. This is the category's standard shape (hcom, relay): one binary, no daemon, and the plugin makes setup a single install.
- Hybrid without the GUI (CLI and plugin only; the viewer comes later): less to build, but it drops decision D5.
- A desktop app hosting everything (the Tauri or Electron pattern of opcode and Orkas): richer UI, but it adds a GUI toolkit and a second process model. It also conflicts with "no GUI toolkit on any platform".

### 3. Primary Language Preference
- **Rust** [inferred, PROPOSED in the brief]: a static single binary with no interpreter, which matters because hooks spawn on every tool call. It is the category's trend (hcom, psmux, agent-console). The spike and prototype already proved `portable-pty` plus ConPTY in Rust, and Rust 1.95 is installed. The trade-off: upstream `portable-pty` lags on modern ConPTY flags, so a patched fork may be needed.
- **Go**: used by relay and many tmux orchestrators, and cross-compiles simply. However, `creack/pty` has no ConPTY, so Windows relies on separate packages (`go-pty`, `conpty`), and the Windows-first target would be the weaker path.
- **TypeScript/Node**: `node-pty` supports ConPTY well (used by Codeman, ccmanager, Happy). The costs are a Node runtime on the host and interpreter startup on every hook call, which is the complaint the brief designs against.

### 4. Target Users
- **Just me**: the founder's overseer and builder pairs across Pulse, Conductor and viola itself. The brief's first-consumer scope, and the fastest route to the first live test.
- **Individual Claude Code subscribers (public)**: developers who run their own linked sessions on their own subscription. This fits D1 ("a standalone product") and the permitted shape (the unmodified binary under the user's own login).
- **Developers building agent pipelines on top of viola**: other orchestrators that consume the CLI and MCP verbs as a transport. This puts a premium on stable verb and event contracts.
- Note: "my team sharing one subscription" is deliberately not offered. The legal page limits plans to ordinary, individual usage.

### 5. Growth Model
- **Modular monolith**: one binary with clear internal modules (pty · channel · hook · mcp · ui · state), and features grow as subcommands and verbs. This matches the proposed one-binary, no-daemon topology.
- **Plugin/extension system**: a core plus adapters (other driven agents, other viewers, driver policy packs). This only pays off if agent coverage (field 9) goes beyond Claude Code.
- **Monolith**: one flat codebase. Simplest for a personal tool, but the PTY, IPC and hook layers are platform-specific enough that boundaries help.
- (Microservices do not fit. There is no daemon by design, and the category's single-binary tools avoid them.)

### 6. Development Style
- **classic**: the founder writes and verifies by hand. No harness scaffolding. Live tests against the real CLI are run manually.
- **agent-driven**: the Andromeda pipeline builds viola. The harness includes the planned FAKE AGENT (a program that behaves like Claude Code and calls the hook commands) so CI runs on all three OSes with zero tokens, a headless browser verifies the GUI page, and structured logs plus a status endpoint let agents poll state. This fits the founder's default for their projects, and viola's own build loop becomes its first user.

### 7. Core Functionality
A cross-platform, Windows-first bridge that lets one interactive Claude Code session drive another on the user's own subscription. It wraps the unmodified `claude` CLI in a pseudo-terminal, types at turn boundaries, answers dialogs through hooks, holds a one-driver wheel the human can take at any moment, and shows active and linked sessions in a minimal local GUI. [inferred]

### 8. Viewer Surface and Reach
- **Local web page on 127.0.0.1 only; a phone or remote view later, behind authentication** [inferred, PROPOSED]: no GUI toolkit, verifiable with a headless browser, and a path to a phone view like Happy, Codeman and Remote Control. The cost is Host/Origin checks and a per-launch token from day one, to block DNS rebinding and cross-origin requests from browser tabs.
- **Terminal UI (ratatui), like relay `ui` and ccmanager**: simpler, with no browser attack surface. It cannot be viewed from a phone, and it cannot be verified with the headless-browser habit.
- **Both (TUI now, web page later)**: fast to ship and safe first. The cost is two viewers to maintain.
- **Web page with authenticated remote access in v1**: matches the Appendix A "push to the phone" escalation early. It widens the code-execution surface before the local core is proven.

### 9. Driven-Agent Coverage
- **Claude Code only**: every mechanism is measured on it (hooks answering dialogs, `Stop.last_assistant_message`, statusline `rate_limits`, `claude agents --json`). Smallest contract to keep in sync with a fast-moving CLI.
- **Claude Code first, with an agent-agnostic core** (a driven-agent adapter seam): leaves room for Codex, which has a supported `codex app-server` JSON-RPC (spoken by Orkas and AgentBridge), or Gemini CLI and OpenCode later. The cost is abstraction work before a second agent exists.
- **Multi-agent from v1** (Claude Code plus Codex): broader appeal, similar to AgentBridge. It roughly doubles the protocol surface, and Codex's authority model differs completely (a protocol rather than typed input).

### 10. GUI Control Scope in v1
- **Read-only**: sessions, links and a feed of recent events. Nothing on the page can type into a session, so the page is not a code-execution surface yet. This is the brief's "read-only first".
- **Read-only plus wheel controls** (take the wheel, pause, unlink, release): the human can take over from the page and not only by typing in the terminal. Needs the localhost hardening from field 8.
- **Full control** (also send prompts and answer dialogs from the page): a mission-control page like Codeman. It is the largest attack surface, and it becomes a second driver that R2 has to arbitrate.

### 11. Human Takeover Behavior (the Wheel)
- **Detect after the fact**: a submitted prompt the bridge did not send (per `UserPromptSubmit`) pauses automation, and `send` is refused until `release`. This is how the prototype behaves and what the Pulse live test passed. The trade-off: a human keystroke can still land inside a bridge paste already in flight.
- **Detect after the fact and also hold back human keys while a bridge send is in flight**: this prevents interleaved input. The cost is that the human's keyboard is briefly unresponsive, which is subtle against D4 ("take over at any moment").
- **Explicit take/release only**: the wheel changes only on an explicit command or GUI action, with no auto-pause. Predictable, but it ignores R2's "no button needed" and risks the driver typing over a human.

### 12. Budget Governor Behavior (v1)
The input is the statusline `rate_limits.five_hour` and `seven_day` (`used_percentage`, `resets_at`), taken by wrapping the user's single statusline command. It is present only for Pro and Max, only after the first response, and the per-model weekly window is not exposed (issue #91920).
- **Advisory**: record usage and show it in the GUI and the event log, and let the driver's policy decide. Least intrusive, but the weakest evidence of "ordinary individual usage".
- **Soft stop**: at a user-set threshold, pause automated `send`s and notify the human. The human can still type, and the driver waits. A conservative middle that works as a transparent usage safeguard.
- **Hard stop plus pacing**: refuse all bridge sends above the threshold and rate-limit automated turns below it. Strongest guard, but it can stall a pipeline mid-task. Because the per-model weekly window is missing, it can still run out unseen.

### 13. CLI Version Compatibility Posture
Viola depends on behaviors that were only measured, not documented: `ExitPlanMode` ignores a `PermissionRequest` allow, and newer builds re-check `updatedInput` against deny rules. The docs also lag the installed build.
- **Pin and refuse**: refuse to start against a Claude Code version outside a tested range. Safest, but every CLI release blocks users until viola is re-verified.
- **Warn and continue**: allow an untested version with a visible warning and an event-log entry. Keeps working through upgrades, but a silent behavior change can mis-answer a dialog.
- **Capability probe**: at startup, check the specific behaviors viola relies on (hook events fire, a dialog answer is honored) against the fake-agent contract, and degrade to "transport only, human answers dialogs" on a mismatch. Most robust, and the most to build.

### 14. Cross-Platform Release Bar for v1
- **Windows-only live support, with macOS and Linux built and CI-tested against the fake agent**: matches "Windows first" and the fact that every measurement so far is Windows-only (open item O5). Unix users get unverified-live builds.
- **All three OSes live-verified before v1**: fully honors D3 and avoids the category's weak spot (WSL-only Windows or tmux-only Unix). Needs macOS and Linux hosts and repeating the spike measurements there.
- **Windows only in v1, other OSes deferred**: the fastest path to the founder's own use. It departs from D3 ("cross-platform from the start") and lets platform-specific assumptions creep into the design.

## Pre-filled Values
- Platform: Hybrid, meaning a native CLI binary plus a Claude Code plugin (hooks plus MCP) plus a minimal local web GUI [inferred from: "Cross-platform CLI tool (single native binary) shipping with a Claude Code plugin (hooks + MCP server) and a minimal local web GUI"]
- Primary Language Preference: Rust [inferred from: brief §3.4 "One Rust binary" (PROPOSED); the prototype was built with portable-pty 0.8.1; Rust 1.95 installed]
- Development Style: agent-driven [inferred from: brief §3.4 fake-agent CI on all three OSes and headless-browser GUI verification "which is how the founder's projects verify UI by default"; viola's own Andromeda build loop is its first user]
- Core Functionality: see field 7 [inferred from: input.md "Core idea" and "Key aspects"]
- Viewer Surface: local web page on 127.0.0.1 only, phone view later behind authentication [inferred from: brief §3.4 "The GUI as a local web page… listens on 127.0.0.1 only"; ratatui listed as the alternative still on the table, so confirm]
- Growth Model: modular monolith, one binary with subcommands and no daemon [inferred from: brief §3.4 "One Rust binary, subcommands run · send · wait · hook · mcp · ui" and "No central daemon"]
- Target Users: at least the founder (the Andromeda overseer driving a builder on Pulse, Conductor and viola) [inferred from: brief §6 "The first consumer"; confirm whether a public release is also in scope, per D1 "a standalone product"]
- Fixed constraints, not asked: subscription only with no API key (D2), the unmodified `claude` binary, never touching credentials, both sessions stay interactive (D4), the budget governor ships in v1 [inferred from: brief §2 and §7 Policy]

## Quiz I Scope Note
Quiz I collects PRODUCT-level decisions: what, for whom, how it grows, and
how we develop it (classic vs agent-driven).
Technical decisions are collected later by:
- Quiz II (architectural forks: framework, database, deployment, API style,
  module boundaries, validation library, error handling).
- Specialist skills (tests / obs / security / design / a11y — their respective
  domains).
