# Scope — 2026-10-10-viola-revive · viola revive

**Working entry** (`viola-0.1.0/working-route.md:115`, the head of the markerless tail, in
`### Epoch 4 — Session state & governance`): viola revive — resume a dead instance in place by its newest logged
session id in the recorded cwd, no launch replay, four preflight refusals — plus three CARRY blocks (folded below;
completeness check `route.py pins` → the run dir's trail: three `:115` rows, 1254, 922 and 842 chars, no
abstention). The entry carries no `BLOCKED-ON`, no `PREREQ` and no `WATCH`: the chunk has no acceptance gate.

**The operator's direction at take-up** (`inputs#I1`, the invocation, verbatim in the snapshot):
- no mutation run, no witness run and no workflow dispatch is planned;
- a live `claude` start needs the founder's cap for this chunk and none is given. The fake agent rehearses. If
  the measurement steps P4, P6, P7 and P12 need live starts, the review card states how many, on the headless
  pty rig, and the plan holds them as one step that waits for his number;
- the entry's open question (does a revived instance start with the wheel at `human`) is the founder's. It goes
  on the review card with each option priced and is not settled by the plan;
- the operator CI read at the end is the `ci.py conclusion` tool call (`leg = "operator"`);
- if the entry does not fit one builder window, the review card proposes the cut.

**CI at take-up** (Setup 5a, the last wrap's flip through HEAD is one sha): `00c73fd` reads green, 15/15 checks,
`ci#38048391313`, wall 438 s. Nothing red or not-green is carried.

**The cut research found (P3; the operator's fifth note).** The entry does not fit one builder window as written,
and one of its four refusals cannot be built now. The plan covers items 1 to 5 and 7 below, with three of the four
refusals. It leaves out, and the review card proposes as a CARRY on the route entry "The board: viola list": the
`session-live` refusal with the two measurements that serve it (the study's P4 and P12). The live readings that
stay with this chunk (the resume itself, the `/compact` half of P6, P7) are one plan step that waits for the
founder's number. Each left-out part is named below with its reason.

## What this chunk builds

### 1. The verb: `viola revive <name> [--id <uuid>] [--fork] [-- args]` and `--list`
- Source: the entry's title and its first CARRY (founder ruling 2026-10-01, relayed by the overseer,
  `e2b-route-adaptation.md` C; the study `refs/session-memory-options.md` §3 V2 and §6). The relay file
  `e2b-route-adaptation.md` is named and not found in this tree or under any chunk's `inputs/`; the study is
  tracked in this repository and was read at P1 (§0, §3 common failure modes and V2, §5, §6).
- The entry states: revive runs the ordinary `run` start order in the terminal it is typed in; the program is
  re-resolved at revive time; the child's cwd is the snapshot's recorded `cwd`; the id chain is read from the
  log; no launch spec is replayed; `--list` exists.
- Verified (study §3 V2): the child's argv is `--resume <id> [--fork-session] [extra args]`. `--fork` maps to
  the CLI's `--fork-session`, and the human re-adds any other flag after `--`. As landed, `child_launch` puts
  `--plugin-dir <dir>` ahead of every argument (`src/run/mod.rs:124-128`), so the order is the plugin flag, the
  resume flags, then the human's.
- Verified (study §3 V2, Security): the id is checked as a UUID before it reaches argv, and `--id` must be an
  id this instance's own log holds. Every `session_id` in the committed fixtures is a UUID (research.md,
  finding 2); the hook parser's own unit tests use `"s-1"`, which is test data and not a product shape.
- Verified (study §3; research.md finding 4): the default id is the newest logged one, and `--list` prints the
  chain `(ts, cause, id)`. A `/clear` rotates the id, as recorded on 2.1.287.
- Verified: nothing named `revive` exists at HEAD `00c73fd` (a search of `src`, `crates`, `plugin`, `fixtures`
  and `tests` printed no file).
- [premise-corrected: research.md finding 10 — the program is a name resolved at revive time, never an
  argument] The program is `claude`, resolved against the reviving process's `PATH` by `run`'s own resolver. The
  words after `--` are the child's arguments only.
- [premise-corrected: research.md finding 11 — `role_of` files every first word but `run` and `hook` under `cli`]
  The start arm of revive is a wrapper: it logs as process `run` into `run-<name>.ndjson` and takes the
  wrapper's panic rules, and `role_of` names `revive` beside `run`. `--list` writes no process-log file of its
  own. No sixth process value is added.
- [premise-corrected: research.md finding 12 — exit 1 has no `--json` document and `run` takes none] Revive
  takes no `--json` in this chunk. The machine forms are the route entry "CLI machine contract"'s.

### 2. The recorded cwd: a new snapshot field
- Source: the first CARRY ("the child cwd the snapshot's recorded `cwd` (study §6 puts that field in this
  scope)").
- Re-verified at P1 and P3: `InstanceSnapshot` (`crates/viola-state/src/snapshot.rs:39-57`) holds `endpoint`,
  `pid`, `started_at`, `pinned_bin`, `cli_verified`, `cli_version`, `wheel`, `budget_paused`, `links`,
  `child_pid`, `pending_dialog`. It holds no `cwd` and no `agent_session_id`.
- Verified (study §2; `src/cmd/run.rs:406-443`): the wrapper writes `cwd` in its first snapshot, from the
  directory its child is spawned in. No event carries it, so the replay cannot rebuild it. It is shown on no
  external surface (no `list`, no `/api/sessions`, no error body, no process-log line) and is used only as the
  child's spawn cwd, never joined with anything.
- Verified (architecture §Conventions, Protocol versioning and Data model conventions; the arch extract): the
  field is optional and additive, omitted on write when absent, and the snapshot's `v` stays 1.
- [premise-corrected: research.md findings 1 and 5 — the entry names no outcome; the nearest closed code is
  `cwd-missing`] A revive with no recorded cwd to use is refused `cwd-missing`, the same as a recorded
  directory that is gone. That covers a snapshot written by a binary older than this chunk, an unreadable or
  newer snapshot (the replay arm yields no `cwd`), and a `cwd` that is not valid UTF-8 and so was never written.
- A widening the wrap amends: security-plan §Input Validation (Child executable resolution row) says the
  child's cwd is the spawner's current directory. The founder ratified the recorded cwd as the child's spawn
  directory live on 2026-10-10, shown to him as a widening (`inputs#I3`, the overseer as relay), with the
  strict-modes check run before the value is used (item 4).

### 3. The id read: the first reader of the log replay
- Source: the third CARRY (chunk 2026-10-10-self-healing-state, citing that chunk's own fourth input
  snapshot): the replay landed as library code with no caller, and this entry is its first reader.
- Re-verified at P1 and P3: `viola_state::snapshot::read_snapshot_classified`
  (`crates/viola-state/src/snapshot.rs:123`) and `viola_state::replay::read_snapshot_or_replay`
  (`crates/viola-state/src/replay.rs:84`) exist, and neither has a caller outside `viola-state` (the code-graph
  query, 65 rows). `read_snapshot` has four product callers (`src/cmd/client.rs:96`, `src/cmd/run.rs:294`,
  `src/cmd/hook.rs:254`, `:469`), as the CARRY says.
- [premise-corrected: research.md finding 1 — `replay` keeps one id and the snapshot arm returns none] The
  CARRY's claim holds as far as it goes: `replay` returns the last `agent_session_id` a `session-start` line
  carried (`replay.rs:63-67`), and `read_snapshot_or_replay` writes one `state-recovered` line on its replay arm
  (`:96-102`). But the snapshot holds no id, so the id comes from the log on every arm, and nothing returns the
  chain. This chunk adds the chain reader beside `replay`, over the same `read_from` and its three counts, and
  revive calls `read_snapshot_or_replay` for the snapshot's side (the `cwd`, and the `state-recovered` line when
  the snapshot is unreadable or newer).
- The four product callers' behaviour on an unreadable or newer snapshot is not this entry's: it is owed to the
  entries the last wrap's CARRYs name ("The board: viola list", "Server verification before any frame").

### 4. Three preflight refusals built, with a strict-modes check ahead of them
- Source: the entry's title and first CARRY: `instance-live` · `cwd-missing` · `session-live` · `no-session`,
  "each with an exit code and hint that join Epoch 6's exit-cause catalogue" (the route entry "Exit-cause code
  catalogue").
- [premise-corrected: research.md, Patterns detected — the landed name is `already-live`] `instance-live` is
  `run`'s landed collision refusal, reused whole: its two stderr pairs (a `live` holder, a `stale` holder) and
  its process-log detail `already-live`. No rename.
- Verified (study §3; the arch, design and layouts extracts): `no-session` is "no `session-start` with an id
  ever logged", and `cwd-missing` is item 2's. Both are start refusals of the `run` kind: exit 1, a fixed
  `unable:` line and one `hint:` line on stderr, a closed `process-exit` detail, no path and no pid. The two
  detail codes are new values of a closed schema enum (research.md, Patterns detected).
- The hint lines' wording is the founder's by precedent (design-system's amendment history, 2026-10-08): the
  review card shows the proposed words.
- [premise-corrected: research.md finding 5 — `run`'s start runs no strict-modes check on the home or the
  snapshot, and no dated gap names revive] Revive runs the strict-modes check on the instance's directory, its
  `snapshot.json` and its `events.ndjson` before it uses a value from either, as security-plan §Authentication
  & Authorization requires of every entry point that reads `snapshot.json`. A failed check is exit 1 with the
  landed detail `strict-modes-failed`. No eighth dated gap is asked for. `run`'s own unchecked collision read
  stays as it is, under its Epoch 6 owner.
- The other start refusals reach revive unchanged because it runs the same order: a `.cmd`/`.bat` child, a
  tampered pinned copy, a squatted endpoint.

### 5. The fake-agent E2E
- Source: the first CARRY: "a fake-agent E2E kills a wrapper and revives it, asserting
  `session-start{cause:"resume"}` with the same id".
- Bound by `inputs#I2`: the wrapper it kills is one the test started, by its exact pid; no test here sends a
  kill at a process it did not start.
- [premise-corrected: research.md findings 2 and 3 — no recorded SessionStart has source `resume`, and every
  start of the fake agent reports the fixture's one id] The fake agent learns `--resume <id>` and
  `--fork-session` as argv options. Under `--resume <id>` it fires the recorded `SessionStart.default` payload
  with `source` set to `resume` and `session_id` set to the id it was given (the operator's word at P4,
  `inputs#I3`; test-plan §7, which lets the fake agent set one field of one event today, is amended at the
  wrap). Without `--resume` every payload stays the recorded bytes.
- [premise-corrected: research.md finding 8 — the pin is `#[cfg(windows)]`] The second CARRY's result stands
  for Windows. On Linux and macOS the E2E is the first reading that a wrapper's child ends with it; a child
  that outlives its killed wrapper there is a finding for the founder, never a test to weaken.
- Verified (research.md finding 9): the kill lands only after the start's hook has ended and the fake agent is
  idle, so no instrumented process is in its own exit at the kill.

### 6. The measurement step, held for the founder's number
- Source: the first CARRY ("Measurement step: P4 · P6 · P7 · P12 (study §5); P5 is the Epoch 2b cleanup
  chunk's, read from its result").
- Bound by `inputs#I1`: no live start runs without the founder's number. The plan holds the live readings as
  one step that waits for it, on the headless pty rig.
- [premise-corrected: research.md finding 4 — the `/clear` half of P6 is recorded] P6's `/clear` half needs no
  start: the recorded `clear-1` pair on 2.1.287 shows a new id, and it was read live on 2026-10-08. Its
  `/compact` half has no reading.
- What stays with this chunk, three live starts: the resume itself (does `claude --resume <id>` under `viola
  revive` report a SessionStart of source `resume` with the same id; its only reading is the study's carried F5
  of 2026-09-25), the `/compact` half of P6 inside that session, and P7 (a resume from another cwd).
- What leaves with `session-live` (item 8): P4 and P12, one more live start.

### 7. P5, read and not re-measured
- Source: the second CARRY (chunk 2026-10-02-epoch-2b-cleanup, its `evidence/p5-child-survival.md`).
- Re-verified at P1: the evidence file exists and reads as the CARRY says (the fake agent gone at the first
  0.01 s sample in two readings; `claude` 2.1.283 gone at 0.50 s and 1.27 s, its session's rows 1 → 0; nothing
  holding the home survived), and `tests/run_cli.rs:601` holds `run_child_ends_when_its_wrapper_is_terminated`.
- [premise-corrected: research.md finding 8] The CARRY's claim, in its own words: `terminating the wrapper by
  pid takes its child with it`, and `The study's job-object hypothesis was not observed`. Both are Windows
  readings of 2026-10-03 on CLI 2.1.283, and the test is Windows-only. Nothing is re-measured here; item 5's
  E2E adds the fake-agent reading on the two Unix legs.
- The tab-close leg stays owed to the founder (founder-attended, in a Linux terminal on the dev host).

## What this chunk does not build (the proposed cut)

### 8. The `session-live` refusal, with P4 and P12
- Source: the entry's title and first CARRY; study §3 (the first failure row) and §6 ("P4 decides whether
  `session-live` is a refusal or a warning").
- [premise-corrected: research.md finding 6 — everything it reads is routed to a later entry] Not built here.
  The refusal reads `claude agents --json` for a row whose `sessionId` is the id. No such call exists in the
  product; its join-field ledger row, its `viola verify` probe, the fake agent's stub and the parser's property
  and fuzz targets are pinned by three CARRYs on the route entry "The board: viola list", which stands after
  this one. Taking them here pulls a ledger row, a new verify child and its live starts into this chunk.
  Security-plan's accepted PATH risk for that call names `list`, `mcp` and `ui` only, and the design and layout
  rules forbid the pid the study's wording names.
- Until it lands, a revive of an id that is live elsewhere is not refused by viola. The review card says so and
  proposes the CARRY.

### What else is owed elsewhere
- Launch-spec replay (`launch.json`), V3's pre-assigned session id and V4's opened terminal: the founder's
  ruling of 2026-10-01 keeps them out of 0.1.0 (the incubator `viola-0.2.0-incubator/` is gitignored,
  `.gitignore:6`, and is never routed here).
- Test-plan §6 Scenario E5 as a whole: it needs `link`, a budget pause and `list --json`, each a later entry
  (test-plan `:948`). This chunk supplies the reader that takes the replay.
- The transcript pre-check: viola does not read the CLI's transcript layout; a resume of a gone transcript
  fails in the child, on screen (study §3).
- No kill-on-close job object: the P5 result did not observe a surviving child.
- A ledger row for `--resume`: research.md finding 7. The founder's word at P4 (`inputs#I3`, live, relayed by
  the operator): revive lands with no row now, and the wrap records `--resume` as a relied-on shape with no
  row, owed on a route entry. No row is built here and the seventeen stand.

## Open for the founder (not settled here)
- **Does a revived instance start with the wheel at `human`?** (study §3, the "State the wrapper held" row; the
  entry's own last sentence). As landed, every start appends `wheel{holder:"driver", cause:"start"}`
  (`src/cmd/run.rs:435`), and architecture [Human Takeover / Wheel] says the wheel starts with `driver` at every
  start. Bound by `inputs#I1`: the review card carries the question with each option priced; the plan changes
  nothing about the start holder.

## The capability
- No requirement line and no matrix entry names revive (a search of `viola-0.1.0/requirements.md` and the
  37 unclaimed heads printed none). P4 reads the pool for a cap this chunk makes fully verifiable; the lean at
  take-up is no claim. `v1-41` (advanced and not proven at the last wrap) gains its first reader of the replay
  here; whether that proves it is P4's read of its acceptance.

## Boundaries
- No mutation run, no witness run, no workflow dispatch; no live `claude` start without the founder's number
  (`inputs#I1`).
- No Epoch 6 hardening beyond revive's own read: server verification, home integrity and the seven dated
  exceptions stand as written, and revive borrows none of them.
- No new env var, config key, test seam, listener or channel method. The fake agent's new options are argv.
- `viola release` stays a human verb; nothing here hints it to a driver.
- Universal invariants that bound every item: the human always wins; spawn children directly, never through a
  shell; `events.ndjson` is never truncated; only the wrapper writes `snapshot.json`; 0700 dirs and 0600 files
  set explicitly; every external reader goes through `Read::take(MAX_FRAME)`; names only through
  `ViolaName::try_new`; external errors carry codes and fixed messages, never a path (the recorded cwd
  included); stdout is reserved; each relied-on CLI behaviour is a ledger row with a probe.
