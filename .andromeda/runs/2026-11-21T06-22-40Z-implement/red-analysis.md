# Red analysis — S1 (`HF`) of «Harness-prefix and identity-floor ledger rows»

**Written:** 2026-11-21T07:05:31Z · **Ask:** inputs#I8 (the overseer's relay, snapped) · **Ground:** the tree, this
run's evidence (`evidence/live-round.md`, the kept stamp `target/live-rig/floor-stamp.json`, the rig's kept
`target/live-rig/296568-1795244334/`), and the installed 2.1.328 bundle read as strings (no start). Names only, never
a value; no plan change, no start.

## A. The two unknown names

### `CLAUDE_CODE_CHILD_NOTES_DIR`
- **Where it is set, for whom.** The CLI's child-env builder (the one that also sets `CLAUDECODE`,
  `CLAUDE_CODE_SESSION_ID`, `CLAUDE_CODE_CHILD_SESSION`, `CLAUDE_CODE_SESSION_ATTENDED`, `CLAUDE_PID`, `CLAUDE_EFFORT`)
  adds it to every child it spawns that is not display-only — hooks and the Bash tool's shells — keyed by the
  session's own id. The plugin-process launcher (a plugin's commands, its LSP servers) adds it too, keyed by the
  launching session ids, beside `CLAUDE_CODE_LAUNCHED_BY_SESSION_IDS`.
- **What kind of value.** A per-session directory: `<the CLI's temp dir>/…/<the parent session's id>/started`. The
  CLI creates it owner-only, and its reader accepts it only when the last part is `started` and the part above
  parses as a session id. So the parent session's id is part of the value.
- **What an inherited copy would let a wrapped child do.** It would hand the child the parent's session id and the
  parent's own notes directory, a channel back into the parent session's state. A wrapped `claude` would then read
  as that parent's child. The CLI files the name with the identity names in its own lists (the child-env scrub set
  holding `CLAUDE_CODE_SESSION_ID`, `CLAUDE_CODE_CHILD_SESSION`, `CLAUDE_PID`, and the background-launch keep set
  beside `CLAUDE_CODE_LAUNCHED_BY_SESSION_IDS`).
- **Lean: the floor.** It carries the parent's session id and a path into the parent's state: an identity name by
  the floor's own test.

### `CLAUDE_PLUGIN_DATA`
- **Where it is set, for whom.** The plugin launchers set it beside `CLAUDE_PLUGIN_ROOT` (and `CLAUDE_PROJECT_DIR`)
  for a plugin's own processes: its hooks (S1's capture plugin is a plugin, hence the hook child carried it), its
  stdio MCP servers and its commands.
- **What kind of value.** A per-plugin directory: `<the plugins dir>/data/<the plugin's id, sanitised>`, created on
  demand. It is keyed by the plugin, not by the session; no session id or parent identity is part of it.
- **What an inherited copy would let a wrapped child do.** It would read and write that plugin's persistent data
  dir. That is the plugin's state, not the parent session's identity or channel. A wrapped child's own plugins get
  their own value from the child CLI anyway.
- **Lean: the non-identity list** (making four names). It is a per-plugin data path of the same kind as
  `CLAUDE_PLUGIN_ROOT`, already on that list.

### Does either reach a `viola run` child today?
No, unless kept. `plan_strip` (`crates/viola-agent-claude/src/lib.rs`) removes every inherited `CLAUDE*` name
unless `config.json`'s `claude_env_keep` keeps it and it is not on `IDENTITY_FLOOR`. So the question is only about
`claude_env_keep`:
- On the floor, a user's keep entry for the name is overridden (stripped all the same).
- On the non-identity list, it is a keepable name like any other.
The same holds for `viola verify`'s children and the `revive` start.

A sibling to watch, names only: `CLAUDE_CODE_LAUNCHED_BY_SESSION_IDS` travels with `CLAUDE_CODE_CHILD_NOTES_DIR` and
carries session ids. It is on no list. S1's hook child did not carry it, so it did not read unknown.

## B. `no-prompt`

### B.1 What stands on disk from S1 (no sender text read)
- **The sender's exit.** It was 0. The kept stamp reads `cross_session_probe.outcome: no-prompt`, a word written only
  when the sender ended `Exited`; any other exit reads `sender-failed` or `sender-deadline` (`ledger::cross_session_reading`).
  `latency_ms` is null: no arrival.
- **The sender's runtime.** Not kept. verify's process log goes to the rig's scratch home, which is removed
  (`scratch: absent`). The whole S1 wall is 73.45 s, with the 30 s window inside it.
- **The sender's own hook captures.** There are none. The plan gives the sender no `--plugin-dir` (step 12), so the
  capture arm never ran for it, and no PreToolUse or tool name was captured.
- **Run B's captures in the window.** Gone with the probe dir (removed on every exit). What survives as closed words:
  - `local-command-clear: pass` and Run B ended as it always does. So after the window the box guard read the box
    up: no modal stood in Run B at that read. A held-message approval dialog standing there would have blocked the
    paste.
  - The rig's two kept frames read `modal: none`.
  - There is no reading of a Notification.
- **Net.** Disk tells "the sender exited 0 and nothing arrived; no dialog stood in Run B after it" — and nothing
  more. The three causes (a model that did not call the tool · a tool it could not reach · a target the CLI could not
  resolve) are not told apart by anything S1 kept.

### B.2 From the bundle
- **Is the send tool offered, and deferred?** The tool is named `SendMessage` and carries `shouldDefer: true`.
  A deferred tool is loaded through the tool-search tool, which is not on the sender's one-tool allow-list. The
  strings place the tool-search tool in some default-allowed lists, but they do not show plainly whether a `-p`
  session with `--allowedTools SendMessage` alone may call it. So T4 is not settled by strings: the cause MAY be the
  plan's own argv, and no start has measured it. Whether the tool is offered at all is gated on a feature check the
  minified strings do not resolve; one branch of the roster code reads a `cold_no_identity` state.
- **How `to` is resolved (T3).** The tool's own prompt addresses a local Claude session by its NAME from a
  `ListAgents` listing (`"worker"`, or `"worker [ref]"` when a listing shows one). It says a session that is not
  listed cannot be looked up, and allows a session id only as "the one exception" when the id is in the user's own
  message — which the sender prompt does. Resolving any of these needs the session registry: either form is a
  lookup. Run B was listed under the rig's private config (`agents-join: pass`); whether the sender reads the same
  registry, and resolves an id rather than a name, is unmeasured.
- **The form that needs no lookup.** The socket address: `uds:<socket path>`, or a bare `…/*.sock` path (research's
  host read of the same bundle). Run B's own socket under the runtime dir, by its pid, would need verify to read
  that path — a read viola does not make today.

### B.3 The cheapest closed-word reading for a next start
1. **The sender's own tool calls, by NAME only.** Give the sender the capture plugin with a matcher for the send tool
   and the tool-search tool; read only `tool_name` from its PreToolUse captures. Words: `send-called` ·
   `send-not-called` · `search-called` (the deferred load seen).
   - Widens: the sender's argv gains `--plugin-dir` (`decision/verify-children-direct-spawn`, the sender's row W1);
     the capture arm's hook rows gain a third probe plugin kind; one more `measured` key
     (`contract/ledger-stamps/content`).
   - The founder has not seen this reading.
2. **The send tool's result class.** From its PostToolUse capture: `ok` · `error` · `none`, a closed class with no
   text kept. Widens the same rows, plus a read of `tool_response`'s shape (content, read only as a class).
   The founder has not seen it.
3. **The sender's exit class with its wall time.** Already kept as the closed word; adding `sender_ms` to
   `cross_session_probe` widens only the stamp contract.

Read 1 alone separates "never called" (the model, or a tool it could not reach) from "called and nothing arrived"
(the target or delivery). Read 2 separates an addressing error from a delivered message that Run B dropped. None of
them reads sender text.

## C. What a next start would be — priced, nothing decided

### (i) The identity row alone lands now; the cross-session row leaves for its own entry
- **Stays in the tree:**
  - all of phase A (the capture arm's env file, the canary test);
  - `identity-floor`, `env_reading`, `NON_IDENTITY_HOOK_NAMES` (grown by his classification);
  - the record's `Env.recorded-1.json` and its user-word check, the fake agent's env replay;
  - the hygiene env class, the rig's `HF` / `HG` labels and its `identity-floor` / `env-unknown` lines;
  - `harness_prefix` (a pure refactor of `prompt_origin`, harmless).
- **Cut out:**
  - `CrossSessionPrefix`, `CROSS_SESSION_SETTINGS` and Run B's `--settings`;
  - the sender texts, `SENDER_DEADLINE`, the cross-session turn and `FramingEnd`'s kill end;
  - the fake sender, receiver and inbox, the `cross-session-1` synthetic variant and stem;
  - the drift test's cross-session keying, the five `cross_session_prefix_*` cases, `verify-sender` (schema and
    spawn lists), and the rig's two cross-session lines.
  - Every relied count moves 29 → 28.
- **Size:** a phase revision of steps 7, 9, 10 (part), 12, 14 (part), 16 (part), 18 (part), 20, 29 and the counts
  — about a third of the diff unwound.
- **Starts:** one base re-recording start. The founder classifies the two names first; with both on a list,
  `identity-floor` reads pass if no new name appears. The cap is the founder's (`HG`'s ground was this case, but the
  red was two rows).

### (ii) Both rows stay
- **Path:** a diagnostic start carrying B.3's reads 1–2 (that start re-records too, if every row then passes), then
  the mend (e.g. the tool-search tool on the allow-list, a name or socket address in `to`), then a re-recording start.
- **Starts:** at least 2 base starts beyond `HF` — one only if the diagnostic start's own reading is already green
  after a mend made before it on a guess, which the B.3 readings exist to avoid. A third if the diagnostic reading
  names a cause whose mend needs its own proof.
- **Unmeasured even then:** the model's own behaviour run to run (a refusal reads the same word); the user's
  managed, repo or local `crossSessionInbound` and the CLI's `receive` switch (each a red by design); the sender's
  leavings (a registry entry, an input-history line, a socket while it runs); and every other CLI version
  (2.1.329 on PATH).
