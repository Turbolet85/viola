## User Input
Viola: a standalone, cross-platform (Windows first) bridge that lets one interactive Claude Code session drive another on the user's own subscription. It wraps the unmodified claude CLI in a pseudo-terminal, types into the driven session at turn boundaries, answers its dialogs through hooks, holds a one-driver wheel the human can take at any moment, and shows the active and linked sessions in a minimal local GUI. The full brief (decisions, measurements, open items, the first consumer) is refs/viola-brief.md; prior art is refs/viola-prior-art.md. The throwaway spike and prototype that proved the mechanisms live outside this repo in D:\dev\projects\additional\viola-lab and are reference material only, not this project's code.

## Extracted
- Product type: Cross-platform CLI tool (single native binary) shipping with a Claude Code plugin (hooks + MCP server) and a minimal local web GUI
- Core idea: A bridge that lets one interactive Claude Code session drive another on the user's own subscription by wrapping the unmodified `claude` CLI in a pseudo-terminal.
- Key aspects:
  - Authority through typed input only — types at turn boundaries (bracketed paste + Enter); hooks report events and answer dialogs (AskUserQuestion, permission, plan approval)
  - One-driver wheel — the human typing takes the wheel and pauses automation; authority flows driver → driven, events flow back; identity by instance (BRIDGE_NAME), never by folder
  - Windows first (ConPTY), then macOS/Linux; no central daemon, ndjson state on disk; GUI on 127.0.0.1 only; first consumer is the Andromeda overseer driving an Andromeda builder
  - Constraints: subscription only (no API key), unmodified binary, never touches credentials; budget governor in v1; viola-lab spike/prototype are reference only, not reused code

## Reference Materials

The following are the creator's reference materials from the project-root `refs/` directory, folded in at full fidelity.

### refs/viola-brief.md

# Viola — the brief (preliminary)

Written 2026-09-23 by the founder's second overseer session, from a design discussion with the
founder, and updated the same day with the spike's results (§4.1). It is the input `/andromeda-arch`
reads from `refs/`; its companion is `viola-prior-art.md`. Every item carries its standing:
**DECIDED** (the founder said so), **PROPOSED** (the overseer proposed it and the founder did not
object; the arch dialogue confirms or replaces it), **MEASURED** (on the founder's host, dated),
**DOCS** (official Claude Code docs, fetched 2026-09-23), **OPEN** (the spike or the dialogue decides).

The spike and the prototype are reference material, not this project's code: they live in
`D:\dev\projects\additional\viola-lab\` (`spike\`, `prototype\`, `prototype-goal.txt`), moved
out of this folder 2026-09-23 at the founder's word so the build starts on an empty codebase.

## 1. The problem

Today the founder is the transport and the operator between two Claude Code sessions: an
**overseer** (verifies the build against artifacts, writes relays) and a **builder** (runs the
Andromeda pipeline skills on a managed project). Relays are copied by hand, the builder's reviews,
forks and go-aheads are answered by hand, and skills and `/clear` are invoked by hand. The founder is
the bottleneck, and a third project in parallel with Pulse and Conductor makes that worse.

The founder's own words:

- *«я хочу сделать какой то механизм который позволит оверсиру самому управлять билдером, но проблема
  в том что я не хочу апи использовать а утилизировать подписку»* — a mechanism that lets the
  overseer drive the builder itself, on the subscription, never the API.
- *«чтоб как то можно было связывать агентов в двух терминалах через наш буфер какой то, чтоб это было
  легко и без танцев с бубном»* — link agents in two terminals through our own buffer, easy, no hacks.
- *«Еще бы неплохо было какой то минимальный гуи чтоб было видно какие есть активные сессии и какие из
  них связанны»* — a minimal GUI showing which sessions are active and which are linked.

Context, not a requirement: the founder expects Opus 5.5 at medium/high effort to cost about half of
what Opus 5 at max did, which is what frees the capacity for a third project. That saving is an
expectation and was not yet measured on this host when this brief was written.

## 2. Decisions

- **D1 DECIDED — a standalone product**, not a per-project configuration: a general bridge between
  agents running in terminals. The project folder is `viola`.
- **D2 DECIDED — the subscription only.** It drives the official `claude` CLI under the user's own
  login. No API key.
- **D3 DECIDED — cross-platform from the start**: Windows, macOS, Linux. Windows is the founder's host,
  so it is the first target and the hardest one.
- **D4 DECIDED — both agents stay interactive terminal sessions** the human can watch and take over at
  any moment. That rules out a design where the driven agent runs headless only.
- **D5 DECIDED — a minimal GUI**: the active sessions and the links between them.

## 3. The concept

### 3.1 The constraint everything follows from

User authority in a Claude Code session comes from one place only: that session's own input. Messages
from other sessions, channel events and hook output all arrive as text the model reads, not as input
the CLI executes. **So the bridge must own the terminal input of the session it drives** — which is
what a terminal multiplexer does, and why the tmux-style approach is the base.

- DOCS: cross-session messages — *"a command in the message's text, such as `/compact`, arrives as
  plain text. Claude Code never executes it"*; the receiving model is told the message came from
  another session and treats it as untrusted content.
- DOCS: channels (MCP push into a running session) and agent teams cannot start a slash command either;
  Remote Control has no programmatic interface.
- Consequence for the first consumer: the Andromeda pipeline skills are all
  `disable-model-invocation: true` (user-only), so a message can neither start one nor answer its
  review. Typed input can.

### 3.2 The parts (PROPOSED)

1. **`bridge run <name> -- claude …`** — a thin pseudo-terminal wrapper. The human sees and types into
   the terminal exactly as before; the session gains a second input, a local channel through which
   the bridge types. A tmux reduced to what agents need, native on every OS.
2. **Events, not screen-scraping.** Hooks post events to the bridge: `Stop` (the turn ended, with its
   final text), `UserPromptSubmit` (a prompt was submitted, with its text), `PreToolUse` on
   `AskUserQuestion` (a question with its options), `PermissionRequest` (a permission dialog is about
   to open), `Notification` (the dialog is waiting). The same hooks ANSWER both dialogs (MEASURED,
   §4.1). Nobody parses the rendered screen for content.
3. **The agent-facing interface**: an MCP server (`send`, `wait`, `last`) plus the same verbs as a
   CLI. Hooks and MCP ship together as one Claude Code plugin, so setup is one install.
4. **`bridge ui`** — a local web page served by the same binary: the sessions, the links, a feed of
   recent events. Read-only first; take-the-wheel, pause and unlink come next.

What the founder types:

```
terminal 1:  bridge run builder  -- claude --effort medium
terminal 2:  bridge run overseer -- claude --effort high
```

### 3.3 The rules (PROPOSED)

- **R1 Authority flows one way, events the other.** The driver types into the driven session; the
  driven session only reports events. A driven agent that picks up an injected instruction from a web
  page it read can never command its driver.
- **R2 One wheel.** Exactly one driver at a time. A submitted prompt the bridge did not send means the
  human took the wheel, and automation pauses by itself — no button needed.
- **R3 Type only at a turn boundary**, as one bracketed paste followed by Enter. Never send Esc: it
  interrupts the turn. Input typed mid-turn is queued by the CLI, which is not a state the driver
  should create by accident.
- **R4 Mechanism, not policy.** The bridge carries, logs (ndjson on disk, so it survives restarts on
  either side) and holds the wheel. It never decides what to answer. What a driver may answer on its
  own belongs to the driver's side (Appendix A is the first consumer's).
- **R5 Identity by instance, never by location.** The wrapper sets `BRIDGE_NAME` in the child's
  environment; hooks read it; a session started without the wrapper makes every hook a silent no-op.
  MEASURED motivation: this host's own global `SessionEnd` and `PostToolUseFailure` hooks pick the file
  they write by the session's working directory, so a second session opened in the same folder writes
  into another instance's record.
- **R6 Use what exists, build only what is missing.** Session discovery (`claude agents --json`) and
  plain messaging between sessions already exist. Viola adds: authority through input, answers to
  dialogs, the wheel, turn-synchronised events, and the links.
- **R7 Keys only for what must be typed.** Keystrokes carry user turns alone — skill starts, relays,
  review answers. The dialogs are answered through the hook contract: `PreToolUse` answers
  `AskUserQuestion` and approves a plan, `PermissionRequest` answers a permission prompt (a suggestion
  included) or sends a plan back with feedback, and the answered dialog never renders (MEASURED, §4.1
  S3 and S7). That shrinks the keystroke surface — the part called brittle wherever this has been
  tried — to plain text plus Enter.
- **R8 A wrapper strips its parent's session identity.** Started from inside a Claude session — the
  normal case once an overseer launches a builder — the child would inherit that session's id, its
  Remote Control bridge and its messaging socket. The wrapper removes them (MEASURED, §4.1).

### 3.4 Stack direction (PROPOSED)

- **One Rust binary**, subcommands `run` · `send` · `wait` · `hook` · `mcp` · `ui`. The plugin's hooks
  call the binary, not a Python or shell one-liner, so there is no interpreter dependency and no shell
  difference between platforms.
- **One pseudo-terminal layer on every OS** — the `portable-pty` crate (ConPTY on Windows, openpty on
  Unix) — rather than tmux on Unix and something else on Windows: one code path, one behaviour.
- **Local channels behind one layer**: named pipes on Windows, Unix sockets elsewhere.
- **No central daemon**: each wrapper owns its channel, state lives on disk, and `bridge ui` is a viewer
  that reads the disk and `claude agents --json`. Fewer moving parts, nothing to die mid-flight.
- **CI on all three OSes against a FAKE AGENT** — a small program that behaves like Claude Code
  (prints, waits for input, calls the hook commands). Tests spend no tokens and do not flake; the real
  CLI runs only in local live tests.
- **The GUI as a local web page** rather than a native window: no GUI toolkit on any platform, and an
  agent can verify it through a headless browser, which is how the founder's projects verify UI by
  default. It listens on 127.0.0.1 only: a page that can type into sessions is code execution, so any
  remote view (the phone, later) needs authentication. The alternative on the table is a terminal UI
  (ratatui): simpler, but not viewable from a phone.

## 4. Measured facts — 2026-09-23, Windows 11, Claude Code 2.1.280

- **CLI surface** (`claude --help`): `-p --input-format stream-json --output-format stream-json`
  (realtime streaming input) · `--permission-prompts host|none` · `--effort low|medium|high|xhigh|max`
  per session · `-n/--name` · `--remote-control [name]` · `--bg` with `claude attach <id>`,
  `claude logs <id>` (recent terminal output), `claude stop <id>`, `claude agents --json` ·
  `claude setup-token` ("requires Claude subscription"). No subcommand sends input to a running session.
- **`claude agents --json`** lists every active session on the host, interactive and background:
  `name` · `kind` · `status` (idle/busy) · `cwd` · `pid` · `sessionId` · `startedAt`. Six were listed
  at measurement, the founder's builder and both overseers among them.
- **Messaging between sessions works locally**: a session lists its peers by name and can message
  them. Received text is framed *"Another Claude session sent a message: `<agent-message from=…>`"*,
  and the frame states that it carries no user authority.
- **The builder's transcript** (`~/.claude/projects/<slug>/<session>.jsonl`) records each
  `AskUserQuestion` call with its full options, and each skill invocation with its arguments — the
  founder's relays ride as a skill's arguments. DOCS: the entry format *"is internal to Claude Code and
  changes between versions, so scripts that parse these files directly can break on any release"* —
  usable for reading on demand, never as the bridge's contract.
- **DOCS on hooks**: every hook receives `session_id`, `transcript_path`, `cwd`, `hook_event_name`,
  `permission_mode`; the default hook timeout is 600 s.
- **DOCS on headless mode**: `/skill-name args` in a `-p` prompt expands before it runs; a session
  started interactively can be continued with `-p --resume <id>` and reopened interactively after.
- **The docs lag the installed build.** One docs sweep found no stream-json input, another found no
  `claude attach`; both are in 2.1.280's own help. Design against the installed CLI, measured.
- **The host**: no tmux, no WezTerm; WSL holds only the `docker-desktop` distro; Rust 1.95 and
  Python 3.14 are installed.

### 4.1 The spike — run r1, 2026-09-23 (portable-pty 0.8.1 + vt100 0.15.2, Haiku 4.5 on Claude Max)

Method: `viola-lab/spike/` hosts `claude.exe` in a ConPTY through `portable-pty`, keeps a vt100 screen
model, types command files into it, and loads a test plugin with `--plugin-dir` whose hooks log every
event with the wrapper's `BRIDGE_NAME`. The session ran with `--setting-sources project,local`, so the
founder's global settings and hooks were not loaded into it.

- **S1 PASS — the TUI runs, and typed input carries user authority.** It rendered correctly in the
  screen model, switched bracketed-paste mode on by itself, and re-rendered at the new width after a
  resize. `/spike-ping hello` typed plus Enter started a `disable-model-invocation: true` skill (reply
  `PONG hello`). A bracketed multi-line paste plus Enter arrived as ONE prompt, newlines inside.
- **S2 PASS — hooks fire with the wrapper's identity.** 23 events of 8 kinds, every one carrying
  `bridge_name=spike-a`: SessionStart · UserPromptSubmit (carries `prompt`) · PreToolUse ·
  PermissionRequest · PostToolUse · Notification (`permission_prompt`, fired AFTER PermissionRequest) ·
  Stop (carries `last_assistant_message` — the turn's final text, so a driver needs no transcript) ·
  SessionEnd (carries `reason`).
- **S3 PASS — both dialogs are answerable without keys.** A `PreToolUse` hook returning
  `permissionDecision: allow` with `updatedInput.answers = {question: label}` answered AskUserQuestion:
  no dialog rendered, the tool result read *"User answered Claude's questions: … → Blue"*, the reply was
  `CHOSE Blue`. A `PermissionRequest` hook returning `decision.behavior: allow` approved a Bash call: no
  dialog, the screen read *"Allowed by PermissionRequest hook"*. Keys work as well (Enter on the dialog).
- **S4 — no trust prompt appeared** for the new sandbox folder (see O2).
- **S5 PASS — waking without polling.** A background `wait` that exits on the next `Stop` completed
  when the driven turn ended, and the driving session's harness woke on its exit.
- **S6 (found by the spike) — the parent's identity must be stripped.** The spike was launched from
  inside a Claude session whose environment held 14 `CLAUDE*` variables, among them `CLAUDECODE`,
  `CLAUDE_CODE_SESSION_ID`, `CLAUDE_CODE_BRIDGE_SESSION_ID` and `CLAUDE_CODE_MESSAGING_SOCKET`/`_TOKEN`.
  The wrapper removed them, and the child registered in `claude agents --json` as a session of its own,
  with its own id and pid.
- **S7 (run r2, plan mode) — the plan-approval dialog, and how every dialog's options map.** Leaving plan
  mode fires `PreToolUse` and `PermissionRequest` for `ExitPlanMode`, with `tool_input = {plan,
  planFilePath}` and no `permission_suggestions`; the dialog offers *Yes, auto-accept edits · Yes,
  manually approve edits · Tell Claude what to change*. APPROVE: only `PreToolUse` with
  `permissionDecision: allow` approves it — no dialog, and the session continues with edits needing
  approval (the second option). `PermissionRequest` `allow` is IGNORED for this tool: measured twice,
  plain and with a mode switch, and the dialog rendered both times. REVISE: `PermissionRequest` `deny`
  with a `message` sent the plan back, and the model rewrote it as the message asked (a file renamed);
  keys work too (option 3, typed feedback, Enter). An ordinary permission dialog's middle option arrives
  as a suggestion — *Yes, and switch to accept edits for this session* is `permission_suggestions:
  [{type: setMode, mode: acceptEdits, destination: session}]` — so its options are: allow · allow plus
  suggestion n · deny; picking the suggestion left the session in `acceptEdits` (the `Stop` payload's
  `permission_mode`). Notification types seen: `permission_prompt`, `idle_prompt`. A caution: two
  identical revise messages in a row made the model stop re-presenting the plan — feedback must say what
  to change.
- **S8 (run r3) — free text in a dialog.** `AskUserQuestion` takes any string as an answer, not only an
  option label — the *Other* path: `answers = {question: "A triangle, drawn with rounded corners"}`
  reached the model verbatim. A note on a chosen option rides `annotations = {question: {notes: …}}` in
  the same `updatedInput`, and the model received it (a first run that seemed to show otherwise was an
  instrument defect — the spike's hook did not forward `annotations`; fixed and re-measured). Free text
  on the other dialogs is `deny` plus `message` (S7). Not measured: *approve with this feedback*
  (shift+tab on a plan's third option) and `multiSelect` answers.
- **Timing: the screen lags the hooks.** A snapshot taken right after a `Stop` could still show the
  turn's spinner. Hooks are the source of truth; the screen is an eventually consistent view.
- **Windows details.** `claude` on PATH is an npm shim with the real binary behind it
  (`…\@anthropic-ai\claude-code\bin\claude.exe`), so the wrapper must resolve it. ConPTY did not close
  the output stream when the child exited, so exit is detected on the process, never on EOF.

## 5. Open

- **O1 Pass-through to a real terminal — LARGELY PASSED 2026-09-23** (the founder, Windows PowerShell,
  the prototype's `viola run builder -- claude`, a render stress prompt): Cyrillic, emoji, CJK, box
  tables, a ~300-character wrapped line, code blocks, a folded long output, a diff and a question dialog
  all rendered as without the wrapper, and a multi-line paste arrived as one prompt. Still open: one
  artifact — after the question dialog closed, the next line began with a long run of spaces, the bullet
  pushed right. NOT REPRODUCED: the same `/spike-ask` dialog, run once without the wrapper and once
  under `viola run`, left no such run either time (founder's screenshots) — a one-off after a long mixed
  turn, kept as a watch item rather than a bug. ANSI colours CONFIRMED by eye: the coloured statusline
  renders through the wrapper.
- **O7 A fourth dialog answer: "Chat about this".** The unwrapped question dialog offers, besides the
  options and *Type something* (the Other path, S8), a *Chat about this* choice that declines the question
  and returns to conversation. Its hook mapping is unmeasured — probably `deny` with a message, like a
  plan's *Tell Claude what to change*; the prototype does not offer it yet.
- **O8 The budget governor's source — FOUND.** The statusline's input JSON carries the plan limits as
  official fields, `rate_limits.five_hour.used_percentage` and `rate_limits.seven_day.used_percentage`
  (the founder's own statusline script reads them). A statusline command is one setting per user, so
  viola would take them by wrapping the user's statusline — record the figures, then hand over to the
  user's script unchanged.
- **O2 A truly untrusted folder.** No workspace-trust prompt appeared in the spike, whose folder sits in
  an already-used tree; the first-run prompt in a fresh location is unmeasured.
- **O3 Messaging on native Windows.** `ListAgents` lists peers on this host, yet the author of issue
  #85289 states that messaging is not available on native Windows; sending a peer message was not
  measured.
- **O4 `SessionStart` → `initialUserMessage`**, named in #85289 as a documented way to seed the first
  turn of a starting session — unmeasured.
- **O5 macOS and Linux** — every measurement in §4.1 is Windows-only so far.
- **O6 Limits** — the largest paste the input box takes whole; hook latency per tool call (the spike's
  hooks start a Python interpreter each time; the product's hooks call the native binary).

## 6. The first consumer and the first live test

- **The consumer**: the founder's Andromeda overseer driving an Andromeda builder — on Pulse, on
  Conductor, and on viola itself once the transport works (its own build loop is its first user, the
  way Conductor serves Pulse).
- **The first live test**: the overseer sends `/andromeda-new-session` to the builder, waits for
  `Stop`, and reads the dashboard. When that passes, the foundation stands.
- **PASSED 2026-09-23 on Pulse** (the prototype, `viola run builder-v` in the founder's window, the
  overseer driving from its own session through the `viola` CLI): the skill started from typed input;
  the dashboard arrived through `Stop` and its claims checked against Pulse's artifacts (0 ahead, 57
  complete / 0 pending, coverage 21/22 with P-075 unclaimed, CLAUDE.md 156 lines, both rule sizes); the
  answer to *Ready to continue?* arrived; the founder typing into the window took the wheel, `send` was
  refused, `release` returned it.
- **Found by that test — a driver's shell can rewrite the text it sends.** From Git Bash (the shell
  of Claude's Bash tool on Windows, so every overseer's) a leading-slash argument is taken for a path:
  `/andromeda-new-session` arrived as `C:/Program Files/Git/andromeda-new-session` (42 bytes sent for
  22), and the builder correctly refused to treat it as a command. `MSYS_NO_PATHCONV=1` fixed it by
  hand. The product must not depend on that: take the text from stdin or a file, and warn when an
  argument carries a rewritten-path prefix.

## 7. For the arch dialogue

- The GUI is a UI surface, so `/andromeda-design` runs and a11y's UI machinery switches on; the page
  itself is minimal.
- Security is a first-class concern: the bridge types into sessions that can run commands. Localhost
  only by default, authentication for anything remote, and the event log as the audit trail.
- **Policy — verified 2026-09-23 at code.claude.com/docs/en/legal-and-compliance.** What permits
  viola's shape, verbatim: *"Nor does it prevent an end user from signing in to the unmodified Claude
  Code binary with their own Claude subscription"*. What bounds it: *"The Claude Code binary must not be
  modified"*; *"developers may not collect, store, or intermediate Claude.ai credentials or session
  tokens — sign-in to a Claude account must complete through Anthropic's own flow"*; developers of
  products *"including those using the Agent SDK, should use API key authentication"* — which is why
  viola wraps the CLI rather than hosting the SDK. On volume: *"Advertised usage limits for Pro and Max
  plans assume ordinary, individual usage of Claude Code and the Agent SDK"* — so a budget governor
  belongs in the first version, not a later one. Naming: the Claude Code name and logo may not be part
  of the product's own name or logo.
- **The gap is confirmed, not assumed.** github.com/anthropics/claude-code/issues/85289 — a request for
  a supported local way to deliver a prompt or a control command into a running session — was closed
  NOT_PLANNED on 2026-09-15; it is the latest filing of the same need (#53049, #27441, #65586, #65606).

## Appendix A — the first consumer's decision rights (the driver's policy, not viola's scope)

Recorded here because it was worked out in the same discussion and has no other home yet. The
Andromeda pipeline halts for a human by design, so a driver that answers must know which halts are its
own to answer.

- **The driving overseer may answer by itself**: a session-start go-ahead; starting the next skill on
  the pipeline's own priority ladder; a plan approval, only after its own verification against the
  artifacts; a fork that a recorded founder direction already settles; an operator action on the host
  such as launching the app from a recipe.
- **Always the founder's** (a push to the phone; the builder simply waits): trajectory; a widening of a
  security boundary; a third consecutive re-pin; deferring a capability; a version change or new
  intent; anything irreversible; any disagreement between what the builder says and the artifacts;
  anything that needs human eyes on a GUI.
- The pipeline already has the bridge between autonomy and authority: a gate is satisfied by a
  RECORDED operator pre-direction naming both the entry and its disposition. Autonomy grows by the
  founder recording directions, not by the driver deciding more.
- Rollout: transport only (the founder approves every answer) → routine classes delegated →
  escalations only. A step that moves the rate of caught errors is reverted.


### refs/viola-prior-art.md

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
