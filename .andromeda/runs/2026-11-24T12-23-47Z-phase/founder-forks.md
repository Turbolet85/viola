# Founder forks — «Switching a session's model and effort» (`2026-11-24-switching-a-session-s-model-and-effort`)

## Block 1 — P3's end, 2026-11-24T12:58:51Z (the phase directive item 5, `inputs#I1`)

**Window.** One builder window, this phase run (`.andromeda/runs/2026-11-24T12-23-47Z-phase/`). The take-up landed at
tx 4749, the row read picked 1973 of 4975 rows (7 masters, none at 0) and research landed at
`doc:2026-11-24-switching-a-session-s-model-and-effort/research` rev 1. Nothing was live: no start, window, key,
signal, real session or `~/.claude*` read. The operator is away, so no dialog was raised.

### Measurements (research M1–M6; code, recorded fixtures, tests — nothing live)
- **M1 — the ways a running session's model or effort can change.**
  - (a) A restart that resumes it with a new pair. `viola revive --model/--effort` does this today, but only on a GONE
    instance: a live name is refused by `run`'s collision check, and a session a live claude holds is refused
    `session-live`. The resume rests on the relied ledger row `resume-session`: «--resume <id> reopens the session
    under that id». It writes only viola's events log and snapshot.
  - (b) A command typed into the session. The ledger's 7 local commands hold no `/model` or `/effort`, and no row
    measures one. What it writes is UNMEASURED; only the cap's own text (the operator's 2026-11-09 ruling) says
    `/model` writes the user-scope model key.
  - (c) A settings write. No row; UNMEASURED. viola writes none today, and the run-start test holds the user's
    settings byte- and mtime-equal.
- **M2 — revive and the recorded pair.** A given half overrides the recorded half, which overrides the default
  (`tests/cli_revive.rs`, `revive_hands_the_recorded_pair`).
- **M3 — where the pair is recorded.** Every start appends one `launch {model, effort}` events line, and the
  snapshot holds it. On a replay the last `launch` line wins, so a switch that appends one is replayed by a later
  resume. A later `launch` line also CLEARS the replayed window record, unless a `window` line follows it.
- **M4 — the closed sets.**
  - Model: a SHAPE, not a list (1–64 bytes `[a-z][a-z0-9.-]*`).
  - Effort: the five words `low · medium · high · xhigh · max`.
  - Default: `DEFAULT_PAIR = (opus, medium)`, then `config.json`'s `default_model` / `default_effort`.
  - All are checked by clap at the verb and re-checked by the wrapper.
- **M5 — mid-turn or with a dialog.**
  - The wrapper already knows the turn, the wheel, typing and the one pending dialog, and `send` refuses
    `turn-running`.
  - No code path ends a running session child today. The PTY seam's kill sends SIGHUP first.
  - What the real CLI does on that signal (is the transcript whole for the resume?) is UNMEASURED.
- **M6 — read-back.** None. The pair is «never read back or verified» (architecture, launch-pair-handed-not-verified),
  and the recorded resume SessionStart carries no `model` key. So «takes effect for its next turn» rests on the real
  CLI alone; that reading is carried to «The fleet on the founder's host» (`cry-s4qhj2e3`).

### Rows reached (read whole; searched over all seven masters for «replays no», «launch spec», «served-verb», «product caller», «time bound», «channel method», «signal», «settings», «setup:», «Eight time bounds»)
- **Reached:**
  - security-plan `decision/new-listener-needs-log-entry`: a new channel method needs a Security Decisions Log entry.
  - architecture `decision/revive-resumes-in-place`: «replays no launch spec but the recorded model and effort».
  - architecture `decision/launch-pair-handed-not-verified`.
  - architecture `decision/revive-pair-from-record`.
  - architecture `contract/event-data/launch` and `contract/instance-snapshot/launch`.
  - the help rows: design-system `contract/help-output/verb-groups` and `contract/viola-help/verb-groups`;
    layout-templates `contract/cli-verbs/help` and `contract/viola-help/setup-row`; test-plan
    `contract/viola-help/verb-table`; a11y-plan `contract/cli-help/served-verbs-only` and `contract/cli-help/usage-line`.
  - a11y-plan `decision/time-bounds-never-on-human` and `decision/time-bounds-fall-on-the-driver` (each «Eight time
    bounds»).
  - test-plan `decision/pid-start-time-verified-kill` and `contract/viola-run-start/settings`.
  - security-plan `decision/adopt-signals-no-foreign-process` and `decision/launch-pair-argv-crossing`.
  - architecture `decision/focus-has-no-driver-form` and `contract/cli-verbs/up` (the no-MCP-tool precedent).
- **Read, not reached:**
  - «product caller» (0 hits in any master).
  - architecture `decision/send-time-bound`: a switch is no send.
  - the obs `--settings` never-logged row: no `--settings` is passed.
- **Playbook escalate rows read before this block:** all 4 — Boundary widening; what ratifies it; held items landed
  on the operator's word; a widening the founder answered himself.

### Forks
**F1 — the mechanism (his: a boundary widening on every arm that switches a RUNNING session).** Options, in order:
1. **In-place restart served by the session's own wrapper** — RECOMMENDED · PROVISIONAL meanwhile.
   - What happens: a new wrapper channel method (`switch`) checks the session is idle (F3). The wrapper then ends ITS
     OWN claude child (it started it): the PTY seam's kill, after a pid + start-time check, within a bounded wait.
     It appends `launch {model, effort}` and, where the instance has a window record, its `window` line again (M3).
     Then it spawns `claude --resume <the snapshot's session id> --model M --effort E` in the same PTY, directory and
     terminal. Window, socket, wheel holder and links stay.
   - Crossings: a new channel method (Security Decisions Log entry); a wrapper ending its own live child (today only
     on a panic); one new time bound (F6).
   - Types nothing into a session and writes nothing outside viola's own state.
   - Cost: the pump must outlive one child (`crates/viola-pty/src/pump.rs`, `src/cmd/run.rs` `pump_child`), the
     largest code change of the entry.
2. **End, then re-open in a new terminal.**
   - What happens: the CLI verb signals the wrapper's child — a process the verb itself did not start. Once the
     wrapper is gone, it spawns a foot terminal running the pinned `viola revive <name> --model M --effort E
     --origin cli`, as `viola up` does.
   - Crossings: a CLI process signalling another process's child; a window closed and a new one opened.
   - Cost: smaller code; the window moves and the human's terminal closes under them.
3. **Type `/model` and an effort command into the session.**
   - Crossings: types into a live session and relies on unmeasured CLI behaviour (M1 (b)). The cap's text says
     `/model` writes the user-scope model key, which breaks the entry's «no side effect on the user's settings».
   - Not buildable within the hint.
4. **Record only: no running session switched.**
   - The verb refuses a live instance and records the pair for the next `revive` or `up`, which already replays it
     (M2, M3).
   - No widening, but the entry («switches a running session») is not built whole.

Read from the history files after the block was first written (arch and security histories, this run): arm 1 respawns
inside the wrapper and does NOT pass through `run::start` — no automatic verify, no trust-flag write into the Claude
config, no `CLAUDE_CONFIG_DIR` read, no new name on the start lists — but its mid-session `launch` line breaks
architecture's «once at every start» promise (`contract/wrapper-event-appends/launch`), an Expected amendment. Arm 2
DOES pass through `run::start` (revive): it inherits the start's trust-flag write into the Claude config
(`start-writes-trust-record-flag-only`, skipped for an already-trusted folder), the automatic verify on an unstamped
CLI, and `wheel-starts-with-driver` (the wheel back to the driver, a human hold dropped). So arm 1 stays the
recommendation; arm 2's settings-side effect is one more reason against it.

**F2 — the verb's name and argument surface (technical).** Proposed:
- Usage: `viola switch <name> [--model <MODEL>] [--effort <EFFORT>] [--json]`. At least one of the two flags is
  required, else exit 2. A half not given keeps the session's RECORDED half, not the default.
- The help row: `setup:` grows to `open, run, revive, adopt, up, switch, verify, plugin install`, which amends the
  seven help rows named above.
- Its human line: `<name>  switched  <model> <effort>`. Under `--json`: `{"v":1,"ok":{"name","model","effort"}}`.

**F3 — mid-turn, holding a dialog, with the human at the keys; whose wheel (his: it ends and resumes his session).**
Options:
1. **Refuse** — RECOMMENDED · PROVISIONAL.
   - Words: `human-typing` (exit 10), `not-delivered` / `turn-running` and `not-delivered` / `dialog-pending`
     (exit 13), and `unverified-cli` (exit 12) when the CLI's stamp does not count the relied `resume-session` row.
   - Read under the wrapper's one flight lock, as send reads them.
   - Either wheel holder may switch. The holder is unchanged, and no `wheel` line is written: the wrapper does not
     restart.
2. **Wait** for the turn end or the dialog's end, up to a bound — a further time bound on the driver.
3. **Interrupt** the turn — ends work in flight and is unmeasured (M5).

**F4 — records, lines and words (technical).** Proposed, beside the existing tables:
- **Events:** one `launch` line per switch (and the `window` line again where one stands). No new event kind.
- **Process logs:** `run-<name>.ndjson` gets one `child-exit` / `process-start` pair for the ended and the resumed
  child, as at a start. The CLI verb writes its own `process-start` / `process-exit` into `cli-<name>.ndjson`.
- **Closed details:** two new `process-exit` details, `child-not-ended` (the bounded wait passed) and
  `resume-failed` (the respawn failed), each in `schemas/diag-line.v1.json`.
- No path, pid, session id or model text in any line.

**F5 — an MCP tool (his: a driver switching another session spends the operator's usage differently).** Options:
1. CLI-only — RECOMMENDED · PROVISIONAL, as `focus` and `up`.
2. An MCP tool beside the five.

**F6 — the new time bound (MOVED to his forks by the review of card 1, `inputs#I3`: a new bound and its value were
his in the earlier chunks; see Block 2).** As first proposed:
- One bound: the switch's wait for the ended child to exit, proposed 10 s, on viola's own child.
- It lands as a row of its own beside the two «Eight time bounds» rows, which are not grown and not split.

**F7 — one implement window, or a split (his).**
- On F1 arm 1, one window holds it, tight: the pump's respawn, the wrapper method, the verb, the help row and the
  tests over the fake agent.
- If he would rather split:
  1. The wrapper's in-place respawn and the channel method, proven by a channel test.
  2. The CLI verb, its help row and words.
- RECOMMENDED: no split.

**F8 — live readings (his, each his to grant; none is fired by this phase).**
- **R6a:** a switch of a real idle session of his, reading the model the next turn runs on, and his settings files
  byte-equal before and after (the settings read itself is his).
- **R6b:** a switch refused mid-turn on a real session.
- The fakes carry the argv, the record, the replay and the refusals. They cannot carry «takes effect for its next
  turn».
- No cap is claimed (below), so these readings gate no claim here and are carried to «The fleet on the founder's
  host» unless he rules otherwise.

### Claims
- **v2-31: not claimed.** Its requirement names the console surface, which this chunk does not build («the console
  reaches it through its actions»). Its first leg's «takes effect» is the real CLI's (M6).
- **v2-30: not claimed.** A switch adds no clause it names. Its `up` and `adopt` legs wait on R4 and R5, and «runs on»
  is the fleet entry's.
- One dated notes line goes on each cap at P5 step 4.

### Directive item 6
- **What lands on disk:**
  - the instance's own events log: one `launch` line, plus a `window` line where one stands;
  - its snapshot's `launch`;
  - `run-<name>.ndjson` and `cli-<name>.ndjson` lines;
  - the CLI's own transcript of the resumed session (its existing residual);
  - nothing under `~/.claude*`, and no settings file.
- **What outlives a SIGKILL:**
  - every line already written: one write per line, and the snapshot is replaced atomically;
  - a SIGKILL of the wrapper between ending the child and the resume leaves the instance `gone` with the NEW pair
    recorded only if the `launch` line was written, so the plan orders the `launch` append AFTER the old child's
    exit and BEFORE the spawn, and `viola revive` then resumes on the new pair.
- **What one session can change of another's:** on F1 arm 1, a caller of the same OS user (the only principal) can
  end and resume another instance's session on another model and effort. On F5 arm 1, CLI-only, a driver model has
  no tool for it; it could still run the CLI through its own shell.
- **The operator's spend:** a switch moves every later turn of that session to the new model. «takes effect» is
  never read back (M6).

### Can the entry be built whole on these arms?
- **Yes:** F1 arm 1, F3 arm 1 and F5 arm 1, each PROVISIONAL, with F2, F4 and F6 as technical proposals for the
  overseer's relay.
- The phase goes on to P4 without waiting, as directive item 5 says. Each of his arms is marked PROVISIONAL in the
  plan's `Expected amendments (wrap)` with the escalate pattern it meets: «Boundary widening» for F1 and F5.


## Block 2 — the review of card 1 folded, 2026-11-24T13:09:55Z (`inputs#I3`, the delegate overseer's review word)

**Answered (a delegate technical word, `inputs#I3`):**
- F2, the verb's surface: as proposed.
- F4, the records, lines and words: as proposed.

**F6 — the reap bound (his: a new time bound and its value).** Options, in order:
1. **A new compiled bound `RESPAWN_REAP_BOUND`, 10 s** — TAKEN, PROVISIONAL on the review's word. It falls on
   viola's own child, never the human. It lands as an a11y row of its own beside the two «Eight time bounds» rows,
   which are not grown.
2. **Reuse the pump's existing `KILL_WAIT`, 2 s** (`crates/viola-pty/src/pump.rs:18`, the worker-panic reap).
   - No new bound and no a11y row.
   - Cost: a claude child that takes more than 2 s to exit on SIGHUP is read `NotEnded`. Its signal has already been
     sent, so the session still ends shortly after, as below.

**Correction to F1's pricing, found reading the pump (`crates/viola-pty/src/pump.rs:370-381`):**
- A `NotEnded` read does NOT leave the session as it was. The SIGHUP has been sent, so a child that exits after the
  bound ends the wrapper as any child exit does, with the old pair recorded (no `launch` line was appended).
- `viola revive` then resumes it on the OLD pair. The plan's criterion is reworded to that.

**The code-graph query, recorded now** (`.andromeda/runs/2026-11-24T12-23-47Z-phase/tree-query-2026-11-24-switching-a-session-s-model-and-effort.json`, 2 queries):
- `pump_with_paste`'s product callers:
  - `pump()` @ `crates/viola-pty/src/pump.rs:293`;
  - `pump_child` @ `src/cmd/run.rs:823`;
  - verify's typed run `Run::spawn` @ `src/cmd/verify/typed/run.rs:152`, a touchpoint card 1 missed, now in the plan.
- `pump_child`'s callers: `run` @ `src/cmd/run.rs:305`, `revive` @ `src/cmd/revive.rs:149`, `adopt` @
  `src/cmd/adopt.rs:107`.
- `kill_and_reap`'s one caller: the panic arm of `pump_open` @ `crates/viola-pty/src/pump.rs:381`.
- Line numbers are the editor's (the trace's 0-indexed line + 1).
- What each relies on that a pump outliving its child changes is in the plan's Implementation notes, «The pump's
  callers».
