# Session memory: raise and revive — options study

_Research note, 2026-09-30. Asked by the founder through the Viola overseer
(`additional/viola-overseer/brief-session-memory.md`). This is a thinking task, not a chunk: it changes no product
code, master, route, matrix or playbook. Basis: commit `11f135c`, Claude Code `2.1.283`, Windows Terminal
`1.24.11911.0`, Windows 11 Pro 10.0.26200._

**Tags.** `[M: source]` means measured in this study, with the command or file it came from. `[M-prior: source]`
means measured earlier and carried here, not re-run. `[D: source]` means a vendor's docs or `--help` text says so
(docs lag the build, see Session Learnings). `[H]` means a hypothesis, still to be measured.

## 0. The answer in brief

- **Revive belongs in 0.1.0, as one small CLI chunk.** `viola revive <name> [-- extra args]` runs, in the
  terminal it is typed in, the ordinary `run` start order for `<name>`. The child is `claude --resume <newest
  agent_session_id>`, started in the recorded cwd. Before it starts, three preflight refusals apply: the name is
  still live, the cwd is gone, or the session is live elsewhere. It replays **no** launch spec, because the
  launch spec is code-bearing and belongs to a later chunk. Two probes are hard prerequisites: resuming an id
  that is live elsewhere (P4), and a claude child outliving its wrapper (P5).
- **Raise belongs in 0.2.0, with the graph, but it is more than "a probe".** On this host the foreground-lock
  timeout is infinite (`[M]` §1). A click on the graph therefore reaches a background server, which can never
  raise a window. The real decision is **who triggers the raise**, and every answer changes the security plan.
  The recommendation has three parts:
  - stay inside the user's terminal and record its window at start (R1);
  - resolve the tab by title (R2);
  - trigger the raise through a `viola://` handler that the foreground browser launches (R5).

  R3 (`viola open`, where viola opens the terminal itself) comes alongside as an opt-in launcher for exact
  identity, and brings windowed revive (V4) with it.
- **The overseer's lean, challenged in three places.**
  1. The brief's premise "the snapshot already keeps `agent_session_id`" is true of the architecture spec, not
     the code. The session id lives in `events.ndjson`, which is where revive should read it anyway.
  2. Revive in 0.1.0 should be narrower than "replay the launch".
  3. Raise's gating probes are four (P1, P2, P3, P9), not one.

## 1. The brief's facts, re-tested

| # | Claim | Verdict | Source |
|---|---|---|---|
| F1 | The instance snapshot keeps `pid`, `started_at`, `child_pid`, `agent_session_id` | **Corrected.** The spec keeps all four (`architecture.md` :253-255). The landed `InstanceSnapshot` has `endpoint, pid, started_at, pinned_bin, cli_verified, cli_version, wheel, budget_paused, links, child_pid` and **no `agent_session_id`**. | `[M: crates/viola-state/src/snapshot.rs:30-44 at 11f135c]` |
| F2 | The session id is still recorded somewhere | Yes: every `session-start` line in `events.ndjson` carries `data.agent_session_id` and `data.cause` (`startup · clear · resume · compact · unknown`). Replay recovers it by contract. | `[M: crates/viola-agent-claude/src/hook.rs:137; architecture.md :242, :163]` |
| F3 | No cwd, argv/launch spec or terminal identity is kept | Confirmed. The child's cwd is the wrapper's `std::env::current_dir()`, passed to `SpawnSpec.cwd` and persisted nowhere. | `[M: src/cmd/run.rs:153,199; crates/viola-pty/src/lib.rs:45-52]` |
| F4 | No working-route entry covers raise or revive | Confirmed. The only "window" hits are the ConPTY-resize and confirmation-window entries. | `[M: grep -i "raise\|revive\|resume\|foreground\|window" viola-0.1.0/working-route.md]` |
| F5 | The manual revive works (`viola run <name> -- claude --resume <id>`, SessionStart `source: resume`) | Carried. | `[M-prior: 2026-09-25, CLI 2.1.28x, vbr.ps1]` |
| F6 | viola owns no window; WT tabs have no per-tab HWND | Consistent with what was measured. All six open WT windows belong to **one** `WindowsTerminal` process (pid 63844). Each is a top-level `CASCADIA_HOSTING_WINDOW_CLASS` HWND whose tabs are UIA `TabItem` elements, not windows. A pid can therefore never pick a window. | `[M: UIA probe, §8]` |
| F7 | Under ConPTY/WT, `GetConsoleWindow` returns a pseudo window | **Not measurable from here.** The Bash tool's children have no console (`GetConsoleWindow: NULL`). The owner chain must be measured inside a real WT tab (P1). | `[M: probe §8]` / `[H]` |
| F8 | `SetForegroundWindow` is restricted by the foreground lock | Confirmed, and harder than assumed: `SPI_GETFOREGROUNDLOCKTIMEOUT` = **2147483647 ms** on this host, so the "lock timed out" exemption never fires. The remaining exemptions are: caller is the foreground process · caller was **started by** the foreground process · no foreground window · caller got the last input event · debugging. `AllowSetForegroundWindow` passes the right on until the next user input. | `[M: SystemParametersInfo 0x2000]`, `[D: learn.microsoft.com SetForegroundWindow]` |

New facts this study measured or read:

- **M1.** `claude -p … --session-id <uuid>` is honoured: the result's `session_id` equals the given UUID.
  `[M: 2.1.283, print mode]`
- **M2.** `claude -p … --resume <id>` **from a different cwd** found the session, kept the **same** id and
  remembered the prior turn. The new turn was appended to the **original** project's transcript
  (`~/.claude/projects/<slug-of-a>/<id>.jsonl`); no project dir was created for the second cwd.
  `[M: 2.1.283, print mode]` The resumed process still runs its tools in the new cwd `[H]`.
- **M3.** `/compact` through `-p --resume <id>` kept the **same** id. The transcript gained one
  `compact_boundary` and one `isCompactSummary` record. `[M: 2.1.283, print mode]` Interactive compact is `[H]`,
  same.
- **M4.** `--help` (2.1.283) says:
  - `--fork-session`: "When resuming, create a new session ID instead of reusing the original", so a plain
    resume **reuses** the id;
  - `--bg` with `--resume`: "continues that session in the background under the same ID, or starts a copy and
    says so when the session is already running";
  - `stop`: "`claude --resume` works once it is stopped".

  The CLI therefore knows when a session is running, at least for `--bg`. `[D: claude --help]`
- **M5.** `claude agents --json` rows carry `cwd, kind, name, pid, sessionId, startedAt, status` (7 rows at
  measurement). `sessionId` can answer "is this id live anywhere?". `[M: claude agents --json]`
- **M6.** `wt.exe` exists only as the per-user app-execution alias
  `%LOCALAPPDATA%\Microsoft\WindowsApps\wt.exe`, which is not on this shell's PATH. `WT_SESSION` (a per-pane GUID)
  and `WT_PROFILE_ID` reach child processes. `[M]`
- **M7.** The UIA `TabItem.Name` is the title Claude sets (its session name, a leading status glyph printed here
  as `?`). A window's name is its active tab's title. `automationId` is empty, and tabs support
  `SelectionItemPattern`. A `WT_SESSION` GUID cannot be mapped to a tab by any public API `[H: none found in docs]`.
  `[M: UIA probe]`
- **M8.** `wt focus-tab` targets a tab **by index only**. `-w/--window` takes an integer id or a **name**
  ("If no window exists with the given window-id, then a new window will be created with that id/name"). `wt`
  parses `;` as a command separator. `[D: learn.microsoft.com Windows Terminal command-line arguments]`
- **M9.** The wrapper sets no job object and no console control handler: `grep JobObject|KILL_ON_JOB_CLOSE|
  CTRL_CLOSE|SetConsoleCtrlHandler` over `src/` and `crates/` found nothing. So whether `claude.exe` outlives a
  closed tab or a killed wrapper is **unmeasured** (P5). This matters for revive. `[M: grep]`

## 2. What "memory" is, and where it can live

Four records are in play. They need different homes, because two of them are code-bearing and one is only
recoverable from the log.

| Record | Needed by | Home | Why there |
|---|---|---|---|
| Session id history | revive | **`events.ndjson`** (already there: `session-start{cause, agent_session_id}`) | It survives a lost or unparseable snapshot (replay recovers it), and it keeps the whole chain, so an id from before a `/clear` stays reachable. The spec'd snapshot field `agent_session_id` (F1) can land as a cache, but revive should read the log. |
| Child cwd | revive, V4 | new snapshot field `cwd`, written at the first snapshot | It is only known at start, and no event carries it. Replay cannot rebuild it, so it joins the "treated as absent" list (architecture :242). It must stay out of `list` / `/api/sessions` / every error body (the no-absolute-paths rule on external surfaces). |
| Launch spec (argv after `--`) | V2+ replay | **not** the snapshot | Raw argv can hold a prompt positional (user content, which only `diagnostics/detail-*.ndjson` may hold) and code-bearing flags (`--settings`, `--plugin-dir`, `--mcp-config`, `--agents`, `--add-dir`, `--dangerously-skip-permissions`, `--permission-mode bypassPermissions`). If it is ever kept, it goes in a 0600 `instances/<name>/launch.json` holding a **closed-enum allowlist** (`--effort`, `--model`, `-n`), strict-modes checked like `statusline_command`. §3 V2 has the detail. |
| Terminal identity | raise | new snapshot field `terminal{kind, …}` (closed `kind`, bounded values) | It is only known at start. It is not a secret, but most of it comes from env vars (`WT_SESSION`, `TERM_PROGRAM`, `ITERM_SESSION_ID`, `TMUX_PANE`, `KITTY_WINDOW_ID`, `WEZTERM_PANE`, `WINDOWID`). Reading them **widens** the security rule "The only env vars outside `VIOLA_*` any viola build reads are the two test seams", so it needs a founder ruling. The Windows HWND path reads no env var. |

A gone instance's directory keeps its last snapshot until the next `run` under that name rewrites it
(architecture :93). Revive reads the gone snapshot first, through strict-modes like any snapshot read, and only
then starts the new wrapper, which writes its own.

## 3. Revive: options

### Common failure modes (every option meets them)

| Failure | What happens | Handling |
|---|---|---|
| **Id live elsewhere.** The user already resumed it by hand, a `--bg` copy exists, or the **old child survived its wrapper** (M9, P5) | Two `claude` processes on one transcript `[H: what interactive resume does is unmeasured, P4]` | Preflight: `claude agents --json` row with `sessionId == id` → refuse `session-live` (naming the pid). If `pid == old child_pid`, the hint says the old child is still running. viola never kills a session it did not spawn in this process. `--fork` (→ `--fork-session`) is the opt-in escape. |
| **Id rotated by `/clear`** | The newest `session-start` has a new id; the pre-clear conversation is under the old one `[H: /clear rotates; spec'd at architecture :163]` | The default is the **newest** id (what was on screen). `--id <uuid>` picks an older one, which must appear in this instance's log (it is never a free UUID). `viola revive <name> --list` prints the chain `(ts, cause, id)`. |
| **Compact** | Same id `[M3, print mode]`; interactive `[H]` | Nothing to do. |
| **Cwd gone** (worktree removed, drive unmounted) | Resume would still find the transcript (M2), but tools, `CLAUDE.md` and project settings would come from the wrong place | Refuse `cwd-missing`. The human can run the manual form (F5) wherever they choose. |
| **Transcript gone** (`cleanupPeriodDays`, `--no-session-persistence`, a different `CLAUDE_CONFIG_DIR` in the new terminal) | `claude --resume` fails in the child, visibly `[H: its exit/message shape, P8]` | viola does not pre-check the transcript layout, because it is internal to Claude Code (brief §4). It lets the CLI fail on screen and logs the exit. A later ledger row could add the check. |
| **Name live or stale** | — | `run`'s existing collision refusal (architecture :93), unchanged. |
| **CLI upgraded since** | Resume across versions `[H: works]` | The version gate reruns as usual; on an unverified build revive is transport-only. |
| **State the wrapper held** | `pending_dialog` is lost. The wheel restarts at `driver` (architecture :70), so linked drivers resume sending at once. Links survive through log replay. | **Open question for the founder:** should a revived instance start with the wheel at `human`? A revive is a human act, and the human may want to look before drivers type. |
| **Env differs** | The revived child gets the **new** terminal's env through R8, not the original's | This is documented, not fixed: env is never replayed (security rule). |

### V1 — Print the recipe (the human runs it)

- **How.** `viola revive <name> --print` (or `--json`) prints the resume command: `cd <cwd>` plus
  `viola run <name> -- claude --resume <id>`. It starts nothing.
- **Remember.** `cwd` (snapshot); the id comes from the log.
- **Per OS.** Identical everywhere. The only per-OS part is quoting in the printed line (PowerShell versus
  POSIX), or it prints argv as JSON only.
- **Security.** Nothing is replayed by viola; the human reads and runs it. The printed cwd is an absolute path
  in a success payload, which the error-sanitisation rule does not cover, but it should never enter an error
  body.
- **Failures.** All the common ones, reported rather than enforced. The preflight can still warn.
- **Size.** ½ chunk; it is a subset of V2.

### V2 — Revive in place (recommended for 0.1.0)

- **How.** `viola revive <name> [--id <uuid>] [--fork] [-- extra claude args]` runs the **ordinary `run` start
  order** in the current terminal, with the child cwd set to the recorded `cwd` and child argv
  `--resume <id> [--fork-session] [extra args]`.
  - The program is **re-resolved now** (npm-shim → `claude.exe`, `.cmd`/`.bat` refusal), never replayed from a
    stored path.
  - The env is the current terminal's, through R8.
  - The preflight refusals are `instance-live`, `cwd-missing`, `session-live` and `no-session` (no
    `session-start` ever logged).
- **Remember.** `cwd` in the snapshot. The id chain is already in the log. **No launch spec in 0.1.0**: the
  human re-adds flags after `--`. A later chunk can add the closed-enum `launch.json` (allowlisted
  `--effort`/`--model`/`-n`, never `--settings`/`--plugin-dir`/`--mcp-config`/`--agents`/`--add-dir`/any
  permission-bypass flag, never a prompt positional). That file is the second code-bearing record after
  `statusline_command`, so it takes strict-modes at every read and a security-plan Decisions Log entry.
- **Per OS.**
  - Windows: as `run`, with the ConPTY sideload.
  - macOS/Linux: as `run` over openpty.

  The per-OS surface is only the cwd existence check.
- **Security.**
  - It reuses every `run` control, so nothing new is exec'd.
  - The new inputs, `cwd` from the snapshot and the id from the log, both come from 0600 own-state files read
    under strict-modes (Epoch 6's Home-integrity chunk covers the snapshot read; until then this read sits in the
    same interim as `run`'s).
  - The id is validated as a UUID before it reaches argv. `--id` must match a logged id.
  - The cwd is used only as `SpawnSpec.cwd`, never joined with anything.
- **Size.** 1 chunk, with P4/P5 as its measurement step and `cwd` riding in its first snapshot.

### V3 — Pre-assign the session id at `run`

- **How.** `viola run` adds `--session-id <uuid v4>` when the user's argv has none of `--resume`/`-r`,
  `--continue`/`-c`, `--session-id`, `--fork-session` or `--from-pr` (M1 shows the flag is honoured). viola then
  knows the id before the first hook, including a run where hooks never fire (an unwrapped-plugin misconfig or a
  hook failing open).
- **Remember.** The injected id, in the snapshot's first write. `/clear` rotation still needs the log.
- **Per OS.** Identical.
- **Security.** No new input: viola generates the UUID from `getrandom`. **Premise change:** viola adds an
  argument the human did not type. The pass-through premise is about bytes on the terminal, not argv, but no
  argv injection exists today, so it is the founder's call. It is also a ledger row (`--session-id` accepted in
  interactive mode, plus its conflicts with `--resume`, P11).
- **Failures.** A user alias or a wrapper script that already passes an id (collision → the CLI error is shown).
  It only helps the "no hook ever fired" corner.
- **Size.** ½–1 chunk. **Not recommended for 0.1.0.** The log already carries the id whenever hooks work, and
  the corner it covers is small.

### V4 — Revive into a terminal viola opens

- **How.** `viola revive <name> --window` launches the terminal itself, with V2's preflight and child:
  - Windows: `wt.exe -w viola.<name> new-tab --title <name> --suppressApplicationTitle -d <cwd> <pinned viola>
    run <name> -- <claude.exe> --resume <id>`;
  - Unix: a terminal adapter (see R3).
- **Remember.** V2's records, plus which launcher opened it (`terminal.kind = "viola-opened"` and the window
  name), which is exactly what R3 needs for raise.
- **Per OS.**
  - Windows: `wt` by the alias's absolute path (M6), with a conhost fallback via `CreateProcess` +
    `CREATE_NEW_CONSOLE` when WT is absent.
  - macOS: `osascript` for Terminal.app/iTerm2, with a TCC Automation prompt on first use `[H]`.
  - Linux: `$TERMINAL`/`x-terminal-emulator` `-e`, or kitty/wezterm/tmux CLIs.
- **Security.** This is the one option that builds a **command line for another program's parser**:
  - `wt` splits on `;` (M8), so a cwd holding `;` needs `\;` escaping. A `ViolaName` cannot hold one
    (`^[a-z][a-z0-9-]{0,31}$`, `[M: crates/viola-core/src/lib.rs:11,53-56]`).
  - AppleScript needs string escaping.
  - Every such adapter is a new injection surface and needs a fuzz target or a closed quoting function.
  - The launcher path is resolved to an absolute path. The alias lives in a user-writable dir, which is
    same-user trust, the class already accepted for PATH `claude` resolution.
- **Failures.** WT not installed or the alias disabled. `windowingBehavior` settings change `-w` defaults `[D]`.
  An elevated WT cannot be driven from non-elevated viola (UIPI) `[H]`.
- **Size.** 1–2 chunks (Windows), plus 1 per Unix family. It belongs with R3 in 0.2.0.

### V5 — Automatic or GUI revive

- **How.**
  - (a) A watcher respawns a wrapper whose terminal closed.
  - (b) A "revive" button on the graph (0.2.0) calls V4 through the `ui` server.
- **Assessment.**
  - (a) breaks **no daemon** (architecture: "There is no daemon") and **the human wins**: closing a window may
    be deliberate, and a resurrecting session overrides that.
  - (b) needs a state-changing GUI route, which is exactly the reserved v1.x brake mechanism (cookie +
    per-launch proof + cross-origin denial). The spawned terminal also meets the foreground lock: it opens behind
    the browser `[H, P2]`.
- **Size.** (a) 3+ chunks, rejected. (b) 1 chunk after V4 and the brake auth, a 0.2.x candidate.

## 4. Raise: options

Raise has two independent halves, and every option below is a pairing of the two:

- **Locate:** which window, and which tab or pane, belongs to `<name>`.
- **Trigger:** which process calls the foreground API, and whether Windows lets it (F8).

### The trigger problem, stated once

The graph page runs in the browser. A click reaches the `viola ui` server, a **background** process: it is
neither the foreground process, nor started by it, nor the receiver of the last input. With the lock timeout at
∞ (F8), `SetForegroundWindow` from the server **always** degrades to a taskbar flash `[H: behaviour inferred from
the documented rules plus the measured timeout; P2 confirms]`. The paths that can work:

- **T1 — accept the flash.** Honest and zero-risk, but it does not do what the founder asked.
- **T2 — the target raises itself.** Windows Terminal is one process for all its windows (F6). When a
  `wt -w <name> focus-tab …` request reaches it while WT **is** the foreground process (the user clicked in some
  WT window), WT activates its own window legitimately `[H, P9]`. From the browser, WT is not foreground → flash.
- **T3 — a `viola://raise/<name>` URL handler.** The browser is the foreground process and launches the handler.
  The handler is "started by the foreground process", so it may call `SetForegroundWindow`, or
  `AllowSetForegroundWindow(<WT pid>)` and then ask WT (T2) `[H, P2(d)]`.
  - The browser shows an "Open viola?" prompt; Chrome can remember "always allow" per origin `[H]`.
  - It needs an HKCU protocol registration (`HKCU\Software\Classes\viola`). That registry entry is code-bearing,
    and **any** web page can invoke the scheme.
  - So the handler takes exactly one `ViolaName` (validated by `try_new`, anything else → exit 0, nothing done)
    and does **nothing but raise**: no revive, no send, no argument beyond the name. The registered command is
    the absolute pinned binary path, rewritten at `viola ui` start like the plugin files.
- **T4 — the CLI.** `viola raise <name>` typed in a terminal: the WT process is foreground, but the caller is a
  grandchild (WT → shell → viola), so neither exemption plainly applies `[H, P2(a)]`. Through T2 it works when
  the target is also a WT window.

### R1 — Stay inside; record the host window at start (native identity)

- **How.** At start the wrapper records the window hosting **its own** console:
  - Windows: `GetConsoleWindow()` → owner/root-owner → the WT top-level HWND `[H, P1]`, plus that window's
    process id and start time (HWNDs are recycled). Also `WT_SESSION`, if the env ruling allows.
  - conhost: `GetConsoleWindow()` is the real window.

  Raise then validates the record (the HWND still exists, its class and owning pid/start time still match) and
  runs `ShowWindow(SW_RESTORE)` + the trigger (T3/T4) + a tab select (via R2, since the HWND names the window,
  not the tab).
- **Remember.** Snapshot `terminal{kind:"windows-terminal"|"conhost"|…, hwnd, host_pid, host_started_at,
  wt_session?, tty?}`.
- **Per OS.**
  - Windows as above.
  - macOS: `ttyname(0)` + `TERM_PROGRAM`/`ITERM_SESSION_ID`. Terminal.app AppleScript can select the tab whose
    `tty` matches, and iTerm2 by session id (TCC Automation prompt) `[H]`.
  - Linux: `WINDOWID` (X11 terminals set it) and `_NET_ACTIVE_WINDOW` via `wmctrl`/xdotool-like calls on X11.
    Wayland refuses focus stealing without an xdg-activation token. tmux/kitty/wezterm have their own pane ids
    and CLIs (`tmux select-pane -t $TMUX_PANE`, `kitty @ focus-window --match id:$KITTY_WINDOW_ID`,
    `wezterm cli activate-pane --pane-id $WEZTERM_PANE`) `[H: all]`.
- **Pass-through.** Kept intact: viola still runs where the human started it.
- **Security.**
  - The recorded values are untrusted (any parent can set env vars): bounded length, charset-checked, a closed
    `kind`. They are used only as arguments to a raise adapter, never as a path or a command.
  - The env reads need a ruling (§2).
  - The Win32 calls (`GetConsoleWindow`, `GetWindowThreadProcessId`, UIA) add `windows-sys` features and no C
    build.
  - UIA is COM. The `uiautomation` crates are the likely route, and their deny/licence status is unmeasured
    `[H]`.
- **Failures.**
  - A tab dragged to another window (WT tear-out): the HWND is stale while the tab lives on, so fall back to the
    R2 search.
  - WT restarted: the HWND is gone.
  - Split panes: the tab is found, but not the pane.
  - Minimised or on another virtual desktop `[H: SetForegroundWindow does not switch desktops]`.
  - Elevated WT (UIPI).
  - viola started over SSH or in WSL: no window at all, `kind:"unknown"`.
- **Size.** 2 chunks on Windows (record + raise with T3), +1 per Unix family.

### R2 — Stay inside; find the tab by its title (beacon search)

- **How.** Nothing is recorded at start. Raise enumerates WT windows via UIA (M7), finds the `TabItem` whose
  title contains the session's name, calls `SelectionItemPattern.Select()` (switching the tab without focus
  `[H, P3]`), then raises the window through a trigger. The title is **set by Claude**: `-n <name>` "shown in the
  prompt box, /resume picker, and terminal title" `[D: --help]`; the glyph prefix changes with activity (M7).
- **Remember.** Nothing new, or the Claude session name if it differs from the viola name.
- **Per OS.**
  - Windows: UIA.
  - macOS: AppleScript tab/session names.
  - Linux X11: `_NET_WM_NAME`, which covers windows but not tabs in most terminals.
- **Pass-through.** Intact. The beacon comes from the child's own title bytes, and viola writes zero bytes
  (its invariant).
- **Security.** It reads other windows' titles, which is user-visible and same-user. A title is untrusted text:
  it is matched only, never logged beyond a hash, and never executed. Uniqueness is the weak point: two sessions
  both called `builder` in two projects.
- **Failures.**
  - The user renamed the session (`/rename`) or the profile suppresses application titles.
  - A title collision picks the wrong tab, so R1's HWND should narrow the search first.
  - A title change mid-search.
- **Size.** 1 chunk (Windows). This is R1's tab resolver, not a standalone answer.

### R3 — viola opens the terminal (the model-changing option)

- **How.** It adds a launcher verb, `viola open <name> [--cwd <dir>] -- claude …`, which starts
  `viola run <name> -- …` **inside a terminal it creates**:
  - Windows: `wt.exe -w viola.<name> new-tab --title <name> --suppressApplicationTitle -d <cwd> <pinned viola>
    run <name> -- <claude.exe> …`. **One named WT window per session** makes the window addressable by name,
    and raise becomes `wt -w viola.<name> focus-tab -t 0` (T2) or T3 + the recorded HWND. One shared window
    with tabs would bring back index-only tab focus (M8), so it is avoided.
  - macOS: iTerm2 `create window … command` returns a session id; Terminal.app `do script` returns a tab
    reference with its `tty` `[H]`.
  - Linux: kitty `@ launch` and `wezterm cli spawn` return ids, `tmux new-window -P -F '#{pane_id}'` returns a
    pane id, gnome-terminal returns nothing (fall back to title + X11) `[H]`.
- **Remember.** `terminal{kind:"viola-opened", launcher, window_name, …}`, exact by construction.
- **The trade-off against the pass-through premise.** The brief's premise is D4 plus O1: the human runs
  `viola run` in **their** terminal, and the passed O1 render check was measured there (WT, PowerShell).
  - R3 **does not change the byte path**: the inner `viola run` is identical, so O1 still holds `[H: unless the
    new window's profile differs; the profile is WT's default]`.
  - What it changes: viola now chooses the terminal program and its profile, so it depends on each terminal's
    CLI. Every such behaviour (window naming, `focus-tab`, title suppression) is an undocumented-behaviour
    **ledger row** with a probe (Critical Warnings).
  - Sessions the human starts by hand stay on R1/R2's best effort.
  - Closing the window is still the human's act; nothing resurrects it.

  Net: R3 **adds** a front door and keeps `run` as-is. Replacing `run` with it would break D4's "the human
  starts it where they like", so R3 must stay opt-in.
- **Security.** V4's quoting surface (`;` in `wt`, AppleScript strings), the absolute launcher path, and a
  launched command line carrying only the pinned viola path, the name, the cwd and the user's own `--` args.
  Launching from `viola ui` would be a state-changing route (brake auth); launching from the CLI needs no new
  auth.
- **Failures.**
  - WT missing or the alias off.
  - `windowingBehavior` or a user's `-w` naming collision `[D]`.
  - A named window that the user renamed, or merged a tab into.
  - WT's window-name persistence across a WT restart `[H, P9]`.
- **Size.** 2 chunks on Windows (`open` + raise-by-name, with V4 riding the same adapter), +1–2 for Unix.

### R4 — viola owns the terminal surface (embedded terminal in the graph page)

- **How.** The 0.2.0 React page embeds xterm.js panes fed from each wrapper's PTY output. Raise means focusing
  the pane in the page, which is total and cross-platform, with no foreground lock inside the page.
- **Assessment.**
  - Human keystrokes would travel browser → server → wrapper. The GUI becomes a **typing surface**, which is
    code execution (brief §3.2's own line: "a page that can type into sessions is code execution"). It needs
    full per-launch auth on every write, the wheel's byte-provenance model re-derived for GUI bytes, and
    `viola ui` becomes a hot path.
  - O1 fidelity (Cyrillic/emoji/CJK/box/wrap) must be re-measured in xterm.js.
  - It keeps D4 in letter (interactive, takeable) but moves where the human types.
- **Size.** 5+ chunks plus a security-plan rewrite. Not for 0.2.0. It is noted because it is the only option
  where raise is exact on every OS.

### R5 — Browser-launched raise handler (the trigger, paired with R1/R2/R3)

- **How.** T3 above. The graph node's "raise" is a plain `<a href="viola://raise/<name>">`, not a fetch.
  `viola raise <name>` (the handler target, also usable by hand) loads the snapshot's `terminal` record and runs
  the locate path (R1 → R2 fallback, or R3's window name), then `SetForegroundWindow` under the "started by the
  foreground process" exemption.
- **Remember.** Nothing beyond R1/R3. The handler registration lives in HKCU (Windows), in `Info.plist`
  `CFBundleURLTypes` in an app bundle (macOS, which viola lacks, so this is `[H]` and costly), or in an
  `x-scheme-handler` `.desktop` file (Linux).
- **Security.**
  - This is the new external entry point: any site can fire it.
  - The input is exactly one `ViolaName`; the action is raise only; it has no output, no state change, and exit
    0 always.
  - The worst case is that a hostile page brings one of the user's windows forward.
  - It avoids adding a **write route** to the view-only `ui` server.
  - It needs a security-plan Decisions Log entry (a new registered command, rewritten with the absolute pinned
    path at each `viola ui` start, removed by an uninstall verb).
- **Per OS.** Windows is straightforward. macOS needs an app bundle for URL schemes `[H]`. Linux needs a
  `.desktop` file plus `xdg-mime` `[H]`. **Fallback everywhere: T1** (the flash) or a copyable
  `viola raise <name>` line.
- **Size.** 1–2 chunks on Windows including the security entry, and 1 per Unix family.

### Raise comparison

| Option | Locate accuracy (Windows) | Keeps pass-through | New security surface | Chunks (Windows / +Unix) |
|---|---|---|---|---|
| R1 record host window | window exact, tab by title | yes | env-read ruling · untrusted record | 2 / +1 each |
| R2 title search | tab by title, collisions | yes | reads titles | 1 / +1 |
| R3 viola opens it | exact (named window) | yes for `run`; adds a launcher | launcher quoting · ledger rows | 2 / +1–2 |
| R4 embedded terminal | exact | moves typing to the GUI | GUI becomes a typing surface | 5+ |
| R5 URL handler (trigger) | — (pairs with R1–R3) | yes | registered handler, any site can fire it | 1–2 / +1 each |

## 5. What must be measured before choosing

Every item says who can run it. **FA** means founder-attended: it opens windows or needs real focus, which an
agent in a no-console tool shell cannot do (F7).

| # | Question | Method | Gates |
|---|---|---|---|
| P1 | Does `GetConsoleWindow()` → `GetWindow(GW_OWNER)` / `GetAncestor(GA_ROOTOWNER)` yield the hosting WT top-level HWND (a) in a plain WT tab, (b) under `viola run` (the wrapper's own console), and (c) in conhost? | Run the read-only probe script from this study (§8) **inside** each (FA, about 1 min). Record class, owner HWND, whether it matches the tab's window from the UIA list, and its pid. | R1 |
| P2 | Which callers can actually foreground a WT window? (a) CLI in another WT tab, (b) a child of a background process (a `viola ui` stand-in), (c) `wt -w <name> focus-tab -t 0` from (a) and (b), (d) a `viola://`-style handler launched by Chrome, with and without `AllowSetForegroundWindow(<WT pid>)`. | A throwaway handler registered under a probe scheme name in HKCU, removed after (FA). Record raised versus flashed. | trigger choice (T1–T4, R5) |
| P3 | Does UIA `SelectionItemPattern.Select()` on a tab of a **non-foreground** WT window switch the tab without stealing focus? | Probe script (FA, observation only). | R2 |
| P4 | Interactive `claude --resume <id>` while that id is **live** in another terminal: refused, copied (as `--bg` does, M4), or two writers on one transcript? | Two throwaway interactive sessions in `target/`, `--safe-mode --model haiku` (FA or `viola run` with the harness). Read `claude agents --json` during. | V2 `session-live` rule |
| P5 | When the WT tab closes or the wrapper is killed, does the `claude` child die? Does its session keep a `claude agents --json` row? | First the fake agent under `viola run` (ConPTY mechanics, CI-safe), then the real CLI: close the tab or `Stop-Process` the wrapper, then `Get-Process -Id <child_pid>` + agents rows. The inner ConPTY close should end attached clients `[H]`, but with no job object (M9) a grandchild could survive. | V2 preflight; a possible job-object fix in `run` |
| P6 | Interactive `/clear` and `/compact`: the `session_id` in the next SessionStart payload (new id versus same)? | Read `session-start` lines of a live instance's `events.ndjson` after each command, or extend the `viola verify` probe with a `-p --resume` compact (M3 covers print mode). | the V2 "newest id" default |
| P7 | Interactive resume from a different cwd: found (M2 says yes in print mode)? Which project's `CLAUDE.md`/settings load? | Interactive throwaway pair, as P4. | V2 `cwd-missing` strictness |
| P8 | Resume of a deleted or aged-out transcript: exit code and message; the default `cleanupPeriodDays`. | Throwaway session, delete its `.jsonl`, `claude -p --resume`. Plus docs. | V2 failure text |
| P9 | `wt -w <name>`: is the name kept across a WT restart? Two opens of one name? Does `focus-tab` on a named window raise it when WT is foreground, and when it is not? | FA, with WT closed and reopened. | R3, T2 |
| P10 | Unix: Terminal.app tab-by-tty and iTerm2 session-id AppleScript (plus the TCC prompt), X11 activation, GNOME Wayland refusal, kitty/wezterm/tmux CLIs. | The Linux laptop (Epoch 7's live-confirmation slot). No macOS host, so macOS stays `[H]` until one exists. | R1/R3 Unix legs |
| P11 | `--session-id` in interactive mode, and its conflicts with `--resume`/`--continue`/`--fork-session`. | `claude -p` combinations, cheap. | V3 |
| P12 | After `/clear`, does `claude agents --json`'s `sessionId` follow the new id? (The `list` join and the V2 preflight both rely on it.) | The P6 session, read agents rows. | V2, `list` join ledger row |

## 6. Recommendation and version split

**Revive: V2, in 0.1.0, one chunk.**

- **Scope.**
  - `viola revive <name> [--id] [--fork] [-- args]` and `--list`;
  - snapshot `cwd`;
  - the id chain read from the log;
  - preflight refusals `instance-live` · `cwd-missing` · `session-live` · `no-session`;
  - an exit code and hint for each (they join the Epoch 6 exit-cause catalogue);
  - a fake-agent E2E that kills a wrapper and revives it, asserting `session-start{cause:"resume"}` with the same
    id.
- **Measurement step inside the chunk:** P4, P5, P6, P7, P12.
  - P5 decides whether `run` also needs a kill-on-close job object. If the child can outlive its wrapper, that
    is a real bug for 0.1.0 whether or not revive ships: a driverless `claude` keeps running with no wheel.
  - P4 decides whether `session-live` is a refusal or a warning.
- **Placement.** After *Self-healing state* (Epoch 4), which lands the log replay that revive's id read
  depends on. Alternatively in Epoch 5 beside the CLI machine contract, which owns the exit codes. A route
  amendment is the overseer's and the founder's, not this study's.
- **Kept out of 0.1.0:**
  - launch-spec replay (`launch.json`, a security-plan entry);
  - V3's argv injection (a premise change);
  - V4 (it rides R3).

**Raise: R1 + R2 + R5, in 0.2.0 with the graph; R3 (+ V4) alongside as an opt-in launcher.**

- **Gating probes:** P1, P2, P3, P9, all founder-attended and together about 20 minutes. P2 is the decisive
  one. If Chrome-launched handlers do **not** get the foreground right, R5 falls back to T1 (flash) and the graph
  should say so plainly rather than pretend.
- **Security-plan work:**
  - the env-read ruling (or R1 Windows-only, which reads no env);
  - the protocol handler entry;
  - untrusted-record handling for `terminal`.
- **Unix legs** follow Epoch 7's pattern: Linux measured on the laptop, macOS `[H]` until a host exists.
- **Not recommended:** R4 (it turns the GUI into a typing surface), and raise from the `ui` server itself (the
  foreground lock makes it a flash, and it would need a write route).

**Against the overseer's lean.**

- It holds in shape (revive 0.1.0 CLI, raise 0.2.0 after probing), with three amendments.
  1. F1: the snapshot does not hold the id yet. Revive should read the log, and the chunk should say so rather
     than assume a field.
  2. Revive's 0.1.0 scope is **resume by id in place, no replay**; launch replay is its own later chunk.
  3. Raise has four gating probes, and its real decision is the trigger, which is a security-plan decision, not
     only a UI one.
- One more consideration: **P5 is worth running soon, independent of both features.** It asks whether the
  child can outlive the wrapper, which touches "the human always wins" today.

## 7. Size summary

| Option | Chunks | Version |
|---|---|---|
| V1 print recipe | ½ (subset of V2) | — |
| **V2 revive in place** | **1** | **0.1.0** |
| V2+ launch-spec replay | 1 (+ security entry) | 0.2.x, if asked |
| V3 pre-assigned id | ½–1 | only if P11/field data show hookless runs matter |
| V4 revive into an opened terminal | 1–2 (shares R3's adapter) | 0.2.0 with R3 |
| V5a auto-respawn / V5b GUI revive | 3+ / 1 | rejected / 0.2.x after the brake auth |
| **R1 host-window record** | **2** Windows, +1 per Unix family | **0.2.0** |
| **R2 title resolver** | **1** | **0.2.0** |
| R3 `viola open` | 2 Windows, +1–2 Unix | 0.2.0, opt-in |
| R4 embedded terminal | 5+ | not planned |
| **R5 URL-handler trigger** | **1–2** (+ security entry), +1 per Unix family | **0.2.0** |

## 8. Probes run in this study

Every CLI probe ran with `--safe-mode --model haiku`, and each working directory sat under
`target/probe-sm/{a,b}`.

1. `claude --version` → `2.1.283 (Claude Code)`; `claude --help` and `claude agents --help` read (M4).
2. `claude agents --json` → 7 rows, field set M5. The output went to the session scratchpad, outside the repo.
3. A read-only PowerShell/UIA probe, `console-owner.ps1`, **written to the session scratchpad, not `target/`**:
   the repo's PreToolUse hook blocks writes under `target/` ("edit to a generated directory"), and it was not
   routed around. The probe reads `GetConsoleWindow`, its owner chain, `GetForegroundWindow`,
   `SPI_GETFOREGROUNDLOCKTIMEOUT` and the WT windows' UIA tab names. It changes no focus (F6–F8, M7).
4. `wt.exe` location and the WT package version via `Get-AppxPackage` (M6).
5. Three print-mode sessions on one pre-assigned UUID:
   - `--session-id` from `a/` (M1);
   - `--resume` from `b/` (M2);
   - `/compact` via `--resume` from `a/`. Its first attempt was mangled by Git Bash into a path, which is the
     known `/skill` Session Learning; it was re-run under `MSYS2_ARG_CONV_EXCL='*'` (M3).
6. Transcript inspection: the `.jsonl` file list and `grep -o` over record-type markers only (M2, M3).

**Residue.** The cleanup `rm` of `target/probe-sm/` and of
`~/.claude/projects/D--dev-projects-viola-target-probe-sm-a/` was **denied by the permission layer**, twice for
the `target/` path. Both are still on disk and are left for the operator to remove (`target/` is gitignored, so
nothing reaches the commit). They hold only the probes' JSON results and the throwaway
transcript. The pre-existing `~/.claude/projects/C--Users-turbo--viola-record-ledger-probes-16940/` is not from
this study.

**Sources.**
- [Windows Terminal command line arguments](https://learn.microsoft.com/en-us/windows/terminal/command-line-arguments)
- [SetForegroundWindow](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setforegroundwindow)
- `claude --help` (2.1.283)
