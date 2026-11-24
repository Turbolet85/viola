# Founder forks — 2026-11-24-viola-up («`viola up`», chk-eq4ejvc6) · P3 end · 2026-11-24T04:52:36Z

Written in the away form (the directive, `inputs#I1`): the operator is away, no dialog was raised. Each fork of his
carries its options in order, each priced, the recommendation marked, and the arm this phase takes meanwhile, marked
PROVISIONAL until his own word. Technical forks carry the lean taken, for the overseer's relay.

## Boundary widenings first (playbook escalate rows `rul-r74hshif` · `rul-p2zl2kgl`)
- **U2 is a boundary widening** of security-plan `decision/open-terminal-direct-spawn` («starts in it only the pinned
  viola run with claude»): `up`'s terminal would start the pinned `viola revive`, and revive's argv would gain a hidden
  window flag, as `decision/run-window-flags-checked` gave `run` («A boundary widening of the started run's argv»).
  U2's arm (b) is a further widening of `decision/window-record-kept-to-snapshot-and-event-log` (the recorded
  workspace would leave the snapshot onto a child's argv). A delegate cannot settle either: U2 takes its most
  conservative arm that builds the entry, (a), PROVISIONAL.
- **No MCP tool** (the directive's conservative arm): `up` is CLI-only; the MCP tool set stays `send · wait · last ·
  answer · list`. An MCP `up` would be a further widening, not proposed.
- Nothing else in this plan widens a hardened boundary: `up` reads only what `viola list` and `revive` already read,
  plus the host's boot time (`/proc/stat` `btime`, a bounded read of a kernel file, no env var, no config key).

## The window (the directive's item 3, last sentence)
- One implement window holds the entry: one new verb file, revive's hidden flag, the fake `foot`'s per-spawn record,
  one integration test binary and the census rows. No split is proposed.

## Measurements (P3, research rev 1 — `doc:2026-11-24-viola-up/research` §Measurements)
- **Survives a reboot on this host** (`findmnt`, 2026-11-24): the home on `/home` (btrfs) — `snapshot.json`,
  `snapshot.json.lock`, `heartbeat`, `events.ndjson`, `diagnostics/`; NOT the runtime dir `/run/user/1000` (tmpfs) —
  the socket dir and every endpoint go (also at a logout); `/tmp` (tmpfs) — every test home goes. The boot time reads
  from `/proc/stat` `btime` (2026-11-23T21:04:31Z at this reading).
- **«Was live before the host went down» vs «ended on purpose»:** the snapshot is never removed at a wrapper's end; the
  heartbeat just stops; a wrapper ended by a signal writes no line (it installs no handler). After a reboot every
  wrapper reads `gone` (pid absent or reused: start time differs). Two disk facts separate the cases: (1) the heartbeat
  mtime against `btime` — older than this boot means alive in an earlier boot; (2) a `session-end` line after the
  newest `session-start` — written on an explicit end, and appended by the hook even with the wrapper gone
  (src/cmd/hook.rs:590-600). **Unmeasured, live only:** whether claude 2.1.330 runs its SessionEnd hook on a graceful
  shutdown — if it does, a cleanly rebooted session reads «ended on purpose». SessionEnd's `reason` is dropped at the
  parse (crates/viola-agent-claude/src/hook.rs:238), so no word tells `/exit` from `logout`.
- **revive for one instance:** checks, then holder → logged session → `session-live` (one 5 s `claude agents --json`
  read, fail-open) → recorded cwd; resumes the newest logged id with the recorded model, effort, parent and role; runs
  **no window step** (`Launch.window: None`, src/cmd/revive.rs:259) and takes no window flag; holds the terminal it is
  typed in. It replays no workspace.
- **A recorded window after a reboot:** the address dies with the compositor; the workspace id names the same numbered
  workspace; `placement` records how the last step ended, not a place. «In its recorded workspace and placement» can
  only mean: the new window moved to the recorded workspace id. A hand-started session records no window.
- **Parent-first order:** the snapshot's `lineage.parent`, a label that authenticates nothing; a parent naming no
  selected instance makes a root (missing or never-live alike); a cycle is taken once from its first name, as the board
  draws it (src/human/board/rows.rs:20-24).
- **Bounds and wall time:** per start, the agents read 5 s, the version gate 5 s, the window step 5 s; `up` reuses
  `open`'s live wait `OPEN_LIVE_BOUND` 30 s per instance (no new bound). A fleet of N sequential starts costs at most
  N × 30 s at the live wait (about 1 s each over the fake agent). The first start after a CLI update runs the automatic
  verify (six probe runs, each up to 120 s) under `ledger/verify.lock`: it can outlast the 30 s wait, and that
  instance then reads `not-live` while its verify goes on; the starts after it, sequential, meet a stamped CLI.
- **Stays the fake's, unmeasured live:** real `foot` windows on real Hyprland moved to a recorded workspace; a real
  `claude --resume` after a real reboot; the graceful-shutdown SessionEnd behaviour; the auto-verify cost after a CLI
  update.

## Rows this plan reaches (searched in all seven masters by the counted literals and «never», «only», «replays no»)
- Widened, his (U2): security `decision/open-terminal-direct-spawn`; layouts `contract/viola-revive/no-window`
  («viola revive never places a window»); architecture `decision/revive-resumes-in-place` («replays no launch spec but
  the recorded model and effort» — the recorded window's workspace joins it under U2(a)); security
  `decision/run-window-flags-checked` (its twin for revive).
- Grown by the verb (no widening): design `contract/help-output/verb-groups` · `contract/viola-help/verb-groups`;
  layouts `contract/cli-verbs/help` · `contract/viola-help/verb-groups`; test-plan `contract/viola-help/verb-table`;
  a11y `contract/cli-help/usage-line` · `contract/cli-help/group-labels` (the `setup:` row: `open, run, revive, up,
  verify, plugin install`); architecture `contract/cli-verbs/revive` (the hidden flag), `contract/cli-exit-codes/1` ·
  `contract/cli-exit-codes/21` and design / layouts / a11y / obs exit rows (`up` joins exit 1 and exit 21);
  architecture `decision/lineage-label-only` (a new reader: `up` orders by the parent, outside every wheel, link, send
  and answer path the census covers); obs `decision/cli-log-only-under-viola-name` · a new `contract/cli-log-file/up`;
  security `decision/state-tree-strict-modes-before-use` / tests `decision/strict-modes-before-any-log` and
  `contract/strict-modes-refusal/no-line` (`up` named under the check it really runs: the board scan); tests
  `decision/single-writer-state-files` (`up` writes neither file).
- Time-bound lists, NOT grown and NOT split (the operator's fork F1 of run `2026-11-24T00-26-24Z-wrap`): a11y
  `decision/time-bounds-never-on-human` · `decision/time-bounds-fall-on-the-driver` (rev 6, full). `up` mints no
  bound; whether its reuse of `OPEN_LIVE_BOUND` must be named in a list waits on F1 (Expected amendments).
- Read, not reached: architecture `decision/new-capabilities-as-verbs` («and, where a driver needs them, as MCP tools» —
  none added); security `decision/window-step-moves-own-terminal-window` (the wrapper's own step, unchanged);
  `decision/window-origin-authenticates-nothing`; `decision/lineage-parent-authenticates-nothing` (order only, never a
  path); tests `decision/window-placement-proven-live` (placement proven only live — the gate below); the playbook's
  `rul-o2tr55aj` and `rul-bznresq5` (no held item, no founder answer to a P4 fork on record for this chunk).

## His forks — each with the arm taken meanwhile (PROVISIONAL)

### U1 — Which instances `up` takes («was live before the host went down»)
1. **(Recommended)** An instance whose heartbeat is older than this boot AND whose events log holds no `session-end`
   after its newest `session-start`; plus `viola up <name>…`, which takes the named instances whose heartbeat is older
   than this boot whatever the session-end reading. Cost: if claude writes SessionEnd at a graceful shutdown, a cleanly
   rebooted fleet selects nothing by default until the named form is used — measured only at a real reboot (R4b).
2. Every instance whose heartbeat is older than this boot. Cost: it also re-opens sessions closed on purpose in an
   earlier boot, each a real `claude` start.
3. Only named instances (`viola up <name>…`). Cost: no one command restores the whole fleet; v2-12's «every session»
   reads only with every name typed.
- **Arm taken: 1, PROVISIONAL** (the fewest starts of the three).

### U2 — What `up`'s terminal runs (a boundary widening, above)
1. **(Recommended, conservative)** `foot` with the closed argv `--app-id=viola.<name> --title=<name>
   --working-directory=<recorded cwd> <pinned viola> --home <home> revive <name> --origin <cli|agent>`, one element per
   value, no shell; revive gains the hidden `--origin` alone, and its window step moves the window to the workspace its
   OWN snapshot recorded (the record never leaves the snapshot onto an argv). Cost: a hand-started session has no
   recorded workspace (U3), and the parent's workspace cannot be handed in.
2. As 1, and revive also takes the hidden `--place <N>`, `up` reading the recorded workspace (or the re-opened parent's)
   and putting it on the argv. Cost: a second widening (the window record onto a child's argv).
3. No terminal: `up --dry-run` prints the plan and the operator types each re-open. Cost: does not build the entry.
- **Arm taken: 1, PROVISIONAL.**

### U3 — A session started by hand (no window record)
1. **(Recommended under U2.1)** Re-open it in its new terminal left where the compositor puts it (placement
   `unplaced`); its new window is recorded, so the next `up` has a workspace for it. Cost: today every fleet root is
   hand-started, so the first `up` after the first reboot leaves the roots on the current workspace.
2. Re-open it on its re-opened parent's workspace (as `viola open` does). Cost: needs U2.2's `--place`.
3. Skip it, its line naming why. Cost: the hand-started roots never come back by `up`.
- **Arm taken: 1, PROVISIONAL.**

### U4 — A start that fails among the fleet
1. **(Recommended)** Go on with the rest, its children included (each child's own record decides its window); the
   failed one's line names why; exit 1 at the end.
2. Skip the failed one's descendants (their lines `parent-not-live`). Cost: one failed root keeps a whole tree down.
3. Stop the fleet at the first failure. Cost: one bad instance blocks every later one.
- **Arm taken: 1, PROVISIONAL.**

### U5 — `up` from a driver session
- `viola up` run from an agent's shell (a `VIOLA_NAME` set) starts every selected instance of the home — the reach
  `viola revive <name>` already gives that shell for one name, now for the fleet; its windows record origin `agent`.
  No MCP tool. **Arm taken: CLI-only, no MCP tool, PROVISIONAL** (the directive's conservative arm).

## Technical forks — the lean taken (the overseer's relay may correct any)
- **T1 argument surface:** `viola up [<name>…] [--dry-run] [--json]`. `--dry-run` prints the plan, one line per
  instance with the word it would end on (`would-open` or a skip word), and starts nothing. Leaned: the directive asks
  whether `up` can print its plan; a dry form costs one branch.
- **T2 order and pacing:** sequential, parents first, each awaited live (or its terminal's end) under `OPEN_LIVE_BOUND`
  before the next. Leaned per research: the verify lock and the pinned-copy race favour one start at a time, and the
  fake `hyprctl` answers one terminal at a time.
- **T3 the per-instance line:** `<name>  <word>[  <placement>]`, the words a closed set: `live` (re-opened, with its
  placement word as `viola open` prints it), `already-live` and `stale-heartbeat` (skipped), `no-session`,
  `cwd-missing`, `session-live` (each found by `up`'s own pre-read before any spawn: the logged chain, the recorded
  cwd, ONE `claude agents --json` read for the whole fleet), `not-live` (spawned, not live within the bound),
  `would-open` (dry). [corrected after card 1, `inputs#I2` change 4: the plan builds three NEW words — `would-open`,
  `not-taken`, `boot-time-unread` — and one new pair; they are fork U7's, below, never a technical lean.] A refusal
  revive itself prints in its terminal surfaces to `up` only as `not-live`.
- **T4 exits:** 0 when every taken instance reads `live`, `already-live` or `stale-heartbeat` (or the dry form ran); 1
  when any reads a refusal word or `not-live`, or `foot` is not found (`terminal-not-found`, before any spawn); 21 when
  the board scan refuses (a home, `instances/` or any instance another user may read — `viola list`'s ratified
  whole-board rule); 2 usage. `--json`: one document `{"v":1,"ok":{"instances":[{"name","outcome","placement"}]}}` (exit
  0 or 1), or the `instance-unreachable` / refusal document for a whole refusal.
- **T5 logging:** as `viola list`: no role file without a parsed `VIOLA_NAME`; under one, the caller's
  `cli-<name>.ndjson` gets the self pair and one `process-start` / `process-exit{subject:"terminal"}` pair per spawn,
  no instance name, address, pid or workspace on a line. Each re-opened instance's own records are revive's start's.
- **T6 the boot-time read:** `/proc/stat` through `take(MAX_FRAME)`, one read per `up`, in `viola-state`'s liveness
  module; a beat at or after `btime` is this boot's.
- **T7 the fake `foot`:** one argv record per spawn (an appended `foot-spawns.ndjson`, `{pid, argv}` a line), so a
  fleet's terminals are each readable; its `foot-argv` stays written, so `cli_open`'s reader is unchanged. [the file
  name corrected after card 1 to the plan's]

## What `up` writes to disk, what outlives a SIGKILL, what it lets one session do to another (the directive's item 6)
- **Writes:** no snapshot, no event line and no stamp of its own; under a `VIOLA_NAME` its `cli` lines (T5). Each
  re-opened instance's revive start writes that instance's usual records (wheel, lineage, launch, under `--origin` the
  window line, session-start, the snapshot, the heartbeat, its `run` log and trust-record line).
- **SIGKILL of `up`:** every terminal it already spawned lives on (its own process group); instances not yet reached
  stay gone; `up` leaves nothing half-written; a second `up` skips the live ones.
- **Across sessions:** `up` learns only what `viola list` shows; it starts other instances only through revive's own
  start, recording origin `cli` or `agent`; it changes no other instance's records itself.

## Live readings, each the operator's to grant at the host (none fired by this phase)
- **R4a — real windows, no `claude`:** under a scratch home, the fake agent as `claude` on a scratch PATH (no real
  `claude` reachable), real `foot` and real Hyprland; a planted pre-boot fleet of 3 (a root with a recorded workspace,
  its child with one, a hand-started root with none), wrappers ended and heartbeats aged before `btime`; ONE `viola up`.
  Starts 0 real `claude`; opens 3 `foot` windows and moves 2 to their recorded workspaces; the script closes each window
  it opened by ending its wrapper, on every exit path, found a second way by its `app-id` when its first reading is
  empty; every inherited `VIOLA_*` name unset. Needs the host unlocked (`hyprctl locked` `false`). Proves «in its
  recorded workspace», the order and the lines on a real compositor.
- **R4b — a real reboot:** **this reading needs a real reboot.** Under a scratch home, 2 real `claude` sessions started
  by `viola open` (a parent and its child: 2 real starts), the host rebooted, then ONE `viola up` (2 real `claude`
  resumes; 4 starts in all). Reads: each resumed the logged id in place, in its recorded workspace, parent first; and
  whether the graceful shutdown wrote `session-end` (U1's open measurement). Leaves 2 resumed sessions and windows,
  closed by `/exit` in each, then the scratch home named for removal.
- v2-12 is claimed GATED on R4a and R4b; the fakes prove the selection, the order, the skip, the lines, the argv handed
  to each terminal and each re-opened start's records.

## Can the entry be built whole on these arms?
- **Yes.** On U1.1, U2.1, U3.1, U4.1 and U5 (CLI-only) the entry builds whole now and its claim waits on R4a and R4b.
  The phase goes on to P4 without waiting, every row resting on these arms marked PROVISIONAL with this file named.

---

# Revision after P5 card 1 — 2026-11-24T05:03:52Z (the overseer's review, `inputs#I2`)

The overseer, as delegate, took T1, T2 and T4–T7 on their leans and T3's lean for the line's form (`inputs#I2`
§answers). Six changes followed; three of them are new forks of his, U6–U8. The live readings above are superseded by
§R4 as revised below.

## R4 as revised (changes 1, 3 and 5)
- **The live root lies outside every repository** (change 1): `~/.cache/viola-r4/<stamp>/`. `findmnt -no
  TARGET,FSTYPE -T ~/.cache` reads `/home btrfs` (a reboot keeps it); `git -C ~/.cache rev-parse` reads «not a git
  repository». Every real `claude`'s cwd is a folder under that root. (A real `claude` whose cwd is inside a
  repository under a private config meets the external-imports modal and reaches no session — the committed rig keeps
  its scratch root «outside every repository», crates/viola-e2e/src/rig/isolate.rs:1-4.) The rehearsal stays under
  `target/r4/`.
- **R4a's stand-in agent** (change 3): built by `CARGO_TARGET_DIR=target/r4-agent cargo build --features fake-agent
  --bin viola-fake-agent` — its own target dir, so the release-shaped `target/debug/viola` R4a runs is never swapped.
  The script copies `target/r4-agent/debug/viola-fake-agent` to `<root>/bin/claude` (0700) with a `fake-args` beside
  it (`--fixtures <repo>/fixtures/claude` and `--screens`). A precondition reads it present and its `--version`
  answering exit 0 with a version line, else `r4: red precondition=stand-in-agent`. Every child runs under `PATH=<root>/bin:/usr/bin:/bin`; before the first start the script reads `command -v claude` under that PATH and
  requires `<root>/bin/claude`, and requires `/usr/bin/claude` and `/bin/claude` absent, else
  `r4: red precondition=real-claude-reachable`. (The host's real `claude` is `~/.local/share/mise/command-wrappers/bin/claude`,
  read 2026-11-24, on no such PATH.) Its sessions start unverified (no stamp for the stand-in's version under the
  scratch home), which `up` does not need.
- **R4a's two workspaces are parameters of his grant** (change 5): `r4-up.sh <root> <viola> windows <ws-parent>
  <ws-child>`; the plan's 7 and 8 were placeholders. What the screen shows: three `foot` windows titled `r4p<stamp>`,
  `r4c<stamp>` and `r4h<stamp>`, each running the stand-in agent's idle screen; the first two moved to his two
  workspaces, the third left on the current one; all three closed by the script about 20 s after the `up` (each start
  about 1 s over the stand-in, the reads and the teardown).

## His forks, added after card 1 — each with the arm taken meanwhile (PROVISIONAL)

### U6 — What R4b needs, and whether it runs (change 2)
What a real session needs to reach its first turn under a private config, from the committed rig
(crates/viola-e2e/src/rig/isolate.rs): a private config dir seeded `{"hasCompletedOnboarding": true}` with the
start's cwd trusted (`seed_config(true)`, :475-486) and the used-config markers (`seed_used_markers`, :491-500);
`.credentials.json`, a copy of his credentials, placed 0600 in that private config dir, only inside a token window of
at least 20 minutes (`TOKEN_FLOOR`, :21; `token_window_open`, :101-111), removed on every exit path (:660-693); a cwd
outside every repository. The keys the rig may type are down, enter, x and del, each granted on argv
(crates/viola-e2e/src/rig/keys.rs:1-3); with the cwd trusted by the seed, R4b types no key.
1. **(Recommended)** Two short copies, none at rest across the reboot: `plant` copies the credentials in, starts the
   parent and child by `viola open … --model haiku --effort low -- claude`, sends each ONE prompt by `viola send`
   (the literal `Reply with the single word ready.`: one short turn each), then removes the copy before it returns;
   `after` copies them in again (a new token-window read), runs ONE `viola up`, reads, ends both wrappers and removes
   the copy on every exit path; `r4-up.sh <root> <viola> purge` removes the copy and the scratch root by hand if
   `after` never runs. Cost: 4 real `claude` starts as his cap (2 before, 2 resumes), 2 short turns.
2. As 1, with the copy left at rest, 0600, in the scratch root across the reboot. Cost: his credentials on disk
   outside his own config for as long as the reboot takes.
3. No R4b in this chunk: v2-12 stays gated, its reboot reading moved to «The fleet on the founder's host» (v2-28).
   Cost: the claim does not close here.
- **Arm taken, PROVISIONAL: neither copy is built.** `plant` and `after` are written to refuse —
  `r4: red precondition=r4b-not-granted`, nothing started, nothing copied — and the script holds no code path that
  copies a credential on a delegate's word. His U6 answer comes in through a revision that builds the arm he names.
  The gated entry still needs BOTH green lines (`phase=windows` and `phase=after`): v2-12 cannot verify until R4b runs.

### U7 — The new words, the new pair and the line's form (change 4)
- New closed words: `would-open` (the dry form's), `not-taken` (a named instance the rule does not take),
  `boot-time-unread` (the exit-1 code and `process-exit` detail when `/proc/stat` cannot be read).
- New pair: `unable: the host's boot time cannot be read` / `hint: viola revive <name> resumes one instance by hand`.
- New hint, after the lines when any reads `not-live` (U8): `hint: an instance not live may still be verifying a new
  claude version; viola list shows when it is live`.
- The line's form: `<name>  <word>`, or `<name>  live  <placement>`; the `--json` document
  `{"v":1,"ok":{"instances":[{"name","outcome","placement"}]}}`.
1. **(Recommended)** These forms as written.
2. Other words or wording, his.
- **Arm taken: 1, PROVISIONAL.**

### U8 — The first `up` after a Claude Code update (change 6)
Read from the code: a start whose CLI version is unstamped runs the automatic verify only with `config.json`'s
`verify_new_cli` on (absent reads off; src/run/auto_verify.rs:22-66). It runs after the bind, and no snapshot names
the endpoint while it runs (src/cmd/run.rs:421-431), so `live_endpoint` reads none. With the key **on**: the first
instance reads `not-live` at 30 s while its verify goes on (six probe runs, each up to 120 s), its window stays open,
and it goes live when the verify ends; every later start finds `ledger/verify.lock` held, says the verifying-elsewhere
line and starts **unverified** (transport-only) at once (src/run/auto_verify.rs:139-144), so it reads `live`; a start
after the verify ended reads the new stamp. `up` exits 1 (the first `not-live`). With the key **off**: every start is
unverified and reads `live`; `up` exits 0.
1. **(Recommended — adds no bound and no act)** Leave it, and say so in the hint (U7's `not-live` hint).
2. `up` runs `viola verify` once before its first spawn when the CLI is unstamped. Cost: a new act — six real probe
   runs on his host before any session comes back.
3. A longer live wait for the first instance. Cost: a new time bound, behind his fork F1.
- **Arm taken: 1, PROVISIONAL.** The report's «unproven» lines carry this reading: read from the code, never run.

## Can the entry be built whole on these arms? (revised)
- **Yes.** On U6's refusing arm, U7.1 and U8.1 the entry builds whole now; R4a can be granted at once; R4b waits on
  U6, and the claim waits on both.
