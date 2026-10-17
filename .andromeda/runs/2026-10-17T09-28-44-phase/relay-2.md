# P4 forks — 2026-10-17-mcp-server-for-drivers (dialog unanswered)

The P4 dialog was shown at 2026-10-17T09:46Z. It came back with no option chosen: "No answer from the driver
within 58 min — do not proceed with this; end your turn and wait for instructions. (Held by viola, the hook
through which this session's driver answers its dialogs.)" Per the operator's take-up direction (`inputs#I1`), all
four forks are the founder's own word. A delegate's answer is recorded as a delegate's and does not ratify any of
them.

## Q1 — Boundary (playbook pattern "Boundary widening", escalate)
The new `viola mcp` process writes send/wait/last/answer frames to a wrapper. Its `list` tool would make the one
`claude agents --json` read that CLI `list` makes, and the security plan reads "`mcp` spawns none". Which crossing
do you ratify for this chunk?
1. **Eighth gap + agents read (Recommended).**
   - MCP frames go after the same liveness-only pre-check the CLI verbs use: an eighth dated gap, closed by
     "Server verification before any frame" and "Home and code-bearing file integrity".
   - MCP list reads agents as CLI list does.
   - Gains full CLI parity, so v1-38 can be claimed as written. It is the residual seven verbs already carry, and
     Epoch 6 closes all eight together.
   - Cost: one more dated gap, and a new process that spawns `claude`.
2. **Eighth gap, list no agents.**
   - Frames as in option 1, but MCP list reads only the disk board: STATUS unknown, no unwrapped rows, and the mcp
     process spawns nothing.
   - Narrower, but list stops matching `viola list --json`, so v1-38's "same results as the CLI" cannot be claimed
     for list.
3. **Full verification here.**
   - Full server verification before every MCP frame in this chunk: pid + start time against a strict-modes-checked
     snapshot, plus euid on Unix and SQOS identity on Windows.
   - No new gap, but Epoch 6 work moves forward for MCP only.
   - The chunk gets much larger, and until Epoch 6 the CLI verbs stay weaker than MCP.
4. **Hold frame tools to Epoch 6.**
   - Serve only `list` now. send/wait/last/answer wait for Epoch 6's verification.
   - No frame widening, but v1-38 cannot be claimed now and the driver surface stays CLI-only.

## Q2 — Live `claude` start cap
CI can prove the server through an rmcp stdio client, but the fake agent never loads `.mcp.json`, so only a real CLI
shows that a wrapped driver session actually gets the tools. How many live starts may this chunk make? The rig uses
2.1.287 first on PATH, because the installed bare `claude` is 2.1.296.
1. **One live start (Recommended).**
   - One wrapped claude 2.1.287 on the Linux dev host as the driver, beside a fake-agent instance as the driven one.
   - The CLI loads viola's MCP server from the plugin, lists exactly the five tools, and makes one `list` and one
     bounded `wait` call.
   - No dialog answered, no key typed. This measures the claim CI cannot reach.
2. **No live start.**
   - CI only: the rmcp client proves the server and the written `.mcp.json` proves the registration.
   - "A wrapped driver session gets the tools" then rests on the registration file, not on a measured CLI loading
     it.
3. **Two live starts.**
   - The single start above, plus a second start in which `viola mcp` refuses at strict-modes (exit 1).
   - It measures whether a failing registered server blocks the CLI's start. `.mcp.json` stayed empty because "a
     registered command that exits 2 blocks Claude Code" (`crates/viola-agent-claude/src/lib.rs:153`).

## Q3 — The v1-33 switch: which gap-list groups must close first (several may be chosen; no recommendation)
Carried from the board's card, "not priced until he names the gap-list groups". Group 1 (`list`) and the
terminal-replies stop are already closed at HEAD.
1. **Group 2: dialog hold.** A longer or configurable dialog hold (`DIALOG_DEADLINE` is a compiled 60 s,
   PROVISIONAL) and a record of an expired hold. No route entry names it yet.
2. **Group 3: screen read.** A verb that reads the screen: CLI modals, the screen before a dialog, the context
   percentage. No route entry names it yet.
3. **Group 4: allow <n>.** Answering a permission with one of its suggestions. This is a deliberate v1 limit today
   (`dialog.rs:173`), so a ruling rather than an entry.
4. **None, switch as built.** The overseer adapts its procedure and scripts to the product as it stands. Group 5,
   the start line and three scripts, lies outside this repository either way.

## Q4 — Which chunk claims v1-33
The claiming chunk re-words its acceptance to the Linux dev host and a key typed in the session terminal.
1. **New entry after the gaps (Recommended).**
   - Mint "Overseer switch to viola" at the end of Epoch 5, after any entries the chosen groups need.
   - It claims v1-33 and makes the switch live; this chunk does not claim it.
   - The switch stays separable from the MCP surface.
2. **This chunk.**
   - This chunk claims v1-33 and makes the switch live. Only possible with Q3 "None, switch as built".
   - It adds live starts and edits to the overseer's scripts outside this repository.
3. **Version done-check.** The Epoch 8 entry "Version done-check" claims it at the close of 0.1.0.
4. **Defer v1-33.** Mark v1-33 deferred out of 0.1.0 with a dated note; the switch is taken up in a later version.

## Answer
Received in the session message at 2026-10-17T11:18Z, verbatim:

> The founder's own words on your four P4 questions, live in the overseer's dialog at 2026-10-17T11:18Z: Q1 «Восьмой gap + agents» (option 1: an eighth dated gap, MCP list reads agents as CLI list does; widening ratified by him) · Q2 «Один старт» (one live claude 2.1.287 start) · Q3 «Гр. 2: удержание диалога» (Group 2 only must close before the switch) · Q4 «Новый вход после пробелов» (mint the switch entry after the gap entries; this chunk does not claim v1-33). Re-read your phase run dir from disk first, snapshot this answer as an input, then continue P4 → P5.

Authority: the founder's own answers to the four P4 forks, given live in the overseer's dialog and relayed in the
session message (relayed by the overseer).
- Q1: option 1, eighth gap + agents read.
- Q2: option 1, one live start.
- Q3: group 2 only.
- Q4: option 1, a new entry after the gap entries.
