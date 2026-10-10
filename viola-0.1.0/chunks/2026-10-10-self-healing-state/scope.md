# Scope — 2026-10-10-self-healing-state · Self-healing state

**Working entry** (`viola-0.1.0/working-route.md:113`, the head of the markerless tail, in
`### Epoch 4 — Session state & governance`): Self-healing state — torn-line healing, snapshot rebuild by log replay,
unknown kinds and fields counted and surfaced, newer-snapshot and newer-peer tolerance — plus three CARRY blocks
(folded below; completeness check `route.py pins` → the run dir's trail: three `:113` rows, 399, 416 and 698 chars,
no abstention). The entry carries no `BLOCKED-ON`, no `PREREQ` and no `WATCH`: the chunk has no acceptance gate.

**The operator's direction at take-up** (`inputs#I1`, the invocation, verbatim in the snapshot):
- no mutation run and no workflow dispatch is planned, local witness runs included; the gate block is unit plus
  smoke, and a guard test carries its one-off remove-the-guard demonstration;
- no live `claude` start is planned: no cap is given for this chunk;
- the operator CI read at the end is the `ci.py conclusion` tool call (`leg = "operator"`);
- the entry carries three CARRYs; if research shows it does not fit one builder window, the review card says so
  with a proposed split instead of a plan of the whole.

**CI at take-up** (Setup 5a, the last wrap's flip through HEAD is one sha): `2f1efe3` reads green, 15/15 checks,
`ci#38041076462`, wall 464 s. Nothing red or not-green is carried.

**The cut research found (P3; the operator's fourth note).** The entry does not fit one builder window as written,
and two of its parts cannot be built now without a ruling this chunk does not hold. The plan covers the state
library and the writer (items 1 to 5 below, as closed). It leaves out, and the review card proposes as their own
route entries or CARRYs: the readers' behaviour on an unreadable or newer snapshot (item 2's second half), the
shown half of the counts (item 3's second half) and the killed-verify remover (item 6). Each left-out part is named
below with its reason. **The operator's word at the P5 review (`inputs#I2`): the cut stands**, scope items 1 to 5
here. The new entry for the killed-verify leftover dirs waits for the founder's word, which the operator brings to
the wrap's route step, and the five CARRYs for the replay's readers and the shown counts are proposed there. The
six points of the review card stand as planned, and `crates/viola-e2e/.viola-verify-227786-plan` is the operator's
own and is left alone.

## What this chunk builds

### 1. Torn-line healing in the `events.ndjson` log
- Source: the entry's title and its second CARRY. Architecture's State Store decision
  (`.andromeda/architecture.md:50`) says "Readers heal a torn last line" and names healing and `state-recovered` as
  owed to this entry; security-plan (`:239`) says the same of the healing and its count.
- Today, re-read at P1 and P3 at HEAD `2f1efe3`: `viola_state::events::read_from`
  (`crates/viola-state/src/events.rs:136`) is the one reader, with two product callers, `rebuild` and `scan`
  (`src/run/wait.rs:115`, `:197`). `Skipped { oversize, malformed }` (`:127-129`) is its count, and
  `LoggedLines::skipped` has no product caller (code-graph: one call site, the test helper at `events.rs:473`). It
  skips an unterminated last line unreturned, unrewritten and uncounted (`:208-213`), and skips and counts
  over-long and non-object lines.
- [premise-corrected: research.md §Scope premise closure — architecture names no form; test-plan §6 Chaos suite
  names the observable] The heal is on the writer's side and is one byte. All three append functions share one
  write path (`write_line`, `events.rs:257-261`; callers `:68`, `:89`, `:109`), which today appends straight after
  a torn tail, so the new line is glued to the fragment and lost (the unit test
  `events_append_keeps_prior_bytes_and_writes_one_line` pins that today). After this chunk an append to a log whose
  last byte is not LF starts with an LF in the same single write, under the lock it already holds: "the next
  append starts on a fresh line" (test-plan §6 Chaos suite). Nothing is truncated, replaced or rewritten, and every
  offset handed out before stays the start of the same line. A reader never writes.
- The offset equality the plan rests on: for a log of length L whose last byte is not LF, the appended line starts
  at L + 1, and `append_event_at` gives that value to its builder and returns it (a `send`'s `cursor`). For an
  absent, empty or LF-ended log it is L, as today.
- The record: the healing process writes one `state-recovered` line, `detail` `torn-line-healed`, `file`
  `events.ndjson` (a basename), `offset` L, at WARN, with no `corr` key (obs-plan §6 Additive field catalog, D-03;
  §6 Log levels mapping). `ObsEvent::StateRecovered` and the schema's fields already stand
  (`crates/viola-core/src/obs.rs:28`, `schemas/diag-line.v1.json:94-96`); no product code emits the event today
  (search `StateRecovered` over `src` and `crates`: the enum and one test at `src/obs.rs:692`). One heal writes one
  line, so the cadence is bounded by the number of torn tails.
- The count: the reader's count takes the contract's three names, `unknown_kinds`, `unknown_fields`, `torn_lines`
  (architecture GUI list envelope, `:220`; test-plan §4 viola-state). `torn_lines` counts an over-long line
  (test-plan §4: "counted as torn"), a terminated line that is not one JSON object, and the unterminated last line
  of the read. So a torn tail reads 1 before the heal and 1 after it.
- The reader's two count names of today are not logged and are not logged after: obs-plan names no process-log
  carrier for a `wait` or `last` read's counts (the obs extract, Constraints, the `viola.parse.skipped` item). The
  counts are the reader's return value.

### 2. Snapshot classification and the log replay (library half)
- Source: the entry's title; v1-41's requirement; architecture §Standard Contracts, Snapshot envelope (`:247`):
  a snapshot with a higher `v` or one that fails to parse is ignored and the state rebuilt by replay, which
  recovers only `links`, `agent_session_id`, `wheel`, `budget_paused` and `budget_override_until`; obs-plan E5
  (`:728`).
- Today: `read_snapshot` returns `None` for a missing, unreadable, unparseable and unsupported-`v` snapshot alike
  (`crates/viola-state/src/snapshot.rs:100-110`), and no replay exists.
- The chunk builds, in `viola-state`: a read that tells the four cases apart (present, absent, unreadable,
  unsupported `v` with the `v` it saw), and a replay of `events.ndjson` that returns the log-derived fields: `wheel`
  from the last `wheel` record, `budget_paused` and `budget_override_until` from the last `budget-gate` record,
  `agent_session_id` from the last `session-start` record that carries one (a `/clear` mints a new one mid-log),
  and `links`. A function that reads the snapshot and falls back to the replay writes the `state-recovered` line:
  `snapshot-replayed` for an unreadable one, `snapshot-unsupported-v` with `v_seen` for a newer one, `file`
  `snapshot.json`. The replay writes no file: only the wrapper writes `snapshot.json`.
- [premise-corrected: research.md §Scope premise closure — `EventKind` holds 13 kinds, none of them `link` or
  `unlink`] `links` replays as empty here. Architecture lists fifteen kinds; the two link kinds have no writer and
  no `EventKind` variant at HEAD, and they land with the route entry "Session links". The replay's link half is
  owed there.
- [premise-corrected: research.md §Scope premise closure — four callers, each under a founder-dated pre-check or
  the start arbiter] No product reader takes the fallback in this chunk. `read_snapshot` has four product callers:
  the CLI verbs' liveness pre-check (`src/cmd/client.rs:96`), the hook's two endpoint reads (`src/cmd/hook.rs:254`,
  `:469`) and `run`'s collision check (`src/cmd/run.rs:294`). The first three stand under the dated exceptions of
  security.md, whose words fix the pre-check ("the snapshot's pid + start time alive, the heartbeat live, an
  `endpoint` present; else exit 21"); a client that connected on a rebuilt endpoint and a heartbeat alone would
  change that outcome, which is a boundary matter for the founder, not a plan detail. The collision check reads
  "free" for an unreadable snapshot and the endpoint lock then decides; architecture's "liveness rests on the
  heartbeat alone" would refuse a restart that test-plan E5 says must not be refused. So what each reader does
  with the replay is not built here. The first consumer is the next entry, "viola revive" (its CARRY,
  `working-route.md:115`: this entry lands the replay its id read depends on).
- Verified: the strict-modes exceptions stand untouched. No caller's read changes.

### 3. Unknown kinds and fields counted
- Source: the entry's title; v1-41; the CLAUDE.md learning (readers skip and count, never `deny_unknown_fields`).
- The chunk builds the two counts in the reader. A line whose `kind` is not one this binary knows is not returned
  and is counted in `unknown_kinds`. A known-kind line that carries a top-level key outside the six of the event
  line, or a `data` key outside its kind's contract (architecture §Standard Contracts, Event `data` per kind,
  `:290-304`), is returned and counted once in `unknown_fields`. The known kinds and their `data` keys live in
  `viola-core`, beside `EventKind`.
- Leaned, with its artifact: the `data` keys are counted, not only the six top-level keys. An added field of a
  newer writer lands in `data` (api.md: "Adding a field/kind is additive"), and architecture's per-kind list is
  closed, so a count that read only the envelope would never rise for the case it exists for.
- The equality that keeps the count honest: every line this binary's own writers append reads zero on all three
  counts. Verified at P3 for the writers' key sets (research.md §Patterns detected); the plan pins it with a test
  over product-written logs.
- [premise-corrected: research.md §Scope premise closure — no surface exists at HEAD] "Surfaced" and "shown" name
  surfaces that are later entries: `list --json` ("The board: viola list") and `/api/sessions` with the page
  (Epoch 8). `wait` and `last` have fixed line forms with no skip word (layout-templates, Output structure). So
  this chunk counts and returns; showing is owed to those entries.

### 4. Newer-snapshot and newer-peer tolerance
- Source: the entry's title; v1-41; its ledger note of 2026-09-27.
- [premise-corrected: research.md §Scope premise closure — landed with chunk 2026-09-27-wrapper-channel] The
  newer-peer refusal exists and is tested: `params.v` above `PROTOCOL_V` is `ProtocolError::UnsupportedVersion`
  (`crates/viola-channel/src/server.rs:256-257`), answered `-32602` `"unsupported protocol version"` with
  `data` `{supported, wrapper}` (`crates/viola-channel/src/lib.rs:81`, `:92`), pinned at unit level
  (`answer_refuses_a_newer_version_with_supported_and_wrapper`, `server.rs:598`) and over a real endpoint
  (`tests/channel_endpoint.rs:124-129`). Nothing is built for it here.
- The newer-snapshot half is item 2's classification: a higher `v` is told apart and recorded with `v_seen`.

### 5. The crate-level `viola-state` suite (CARRY 1)
- Source: the first CARRY, from chunk 2026-09-27-instance-state-and-start-order (proposals T17/T18 rejected to
  this CARRY at that wrap): the crate-level `crates/viola-state/tests/` suite in which every event kind
  round-trips `{"v":1,"ts","instance","kind","source","data"}` (test-plan §4 viola-state; §5 On-disk).
- Re-verified: `crates/viola-state/tests/` does not exist at HEAD `2f1efe3`.
- [premise-corrected: research.md §Scope premise closure — `read_from` landed with chunk 2026-10-04-wait-and-last]
  The CARRY's "this entry, which lands the reader" reads "this entry, which lands the healing behind the reader".
- Verified: "every event kind" is the 13 variants of `viola_core::EventKind`
  (`crates/viola-core/src/lib.rs:49-63`). The suite writes each through the crate's own writer and reads it back
  through the crate's own reader, on a real temp dir, with zero on all three counts.

## What this chunk does not build (the proposed cut)

### 6. A killed `viola verify`'s leftover probe dir (CARRY 3)
- Source: the third CARRY, from chunk 2026-10-07-live-rows-and-paste-shapes-on-the-dev-host (the overseer's
  disposition at its wrap, on its route card): a killed `viola verify` leaves the probe dir of the run it was in;
  a leftover's name carries a pid and no start time, so a remover needs an owner record with pid and start time
  first.
- The CARRY's mechanism claim, in its own words: `Its dirs are held by a drop guard and verify handles no signal
  (read at that chunk's research M8: src/cmd/verify/typed.rs, no signal handling in verify.rs, verify/typed.rs or
  main.rs).` Verified at P3: `ProbeDir` and `TrustedDir` remove their dir in `Drop` (`src/cmd/verify.rs:304-308`,
  `src/cmd/verify/typed.rs:347-351`), Run A's dir is a `tempfile` guard (`typed.rs:68`), and a search of
  `src/cmd/verify.rs`, `src/cmd/verify/typed.rs` and `src/main.rs` for `signal|ctrlc|SIGTERM|SIGINT` printed no
  file.
- One addition to the CARRY, found at P3: a kill leaves three classes of dir, not one. Beside
  `<cwd>/.viola-verify-<pid>/` and its `-dialogs` and `-plan` siblings there is `<home>/ledger/probes/<pid>/`
  (`verify.rs:259-262`), which holds raw hook captures, and Run A's `viola-verify-*` dir in the OS temp root.
- [premise-corrected: research.md §Scope premise closure — no rule exists and the dir rules are the founder's]
  Not built here. Security-plan holds no owner record and no remover for a probe dir (the security extract,
  Constraints, last item); the probe dirs' placement and removal are the founder's live rulings (2026-10-05); and a
  deleter acting on a record it reads from disk, in the cwd and the OS temp root, is the class the founder ratified
  for the test side as a boundary widening (2026-10-07). It needs his rule before a plan. The review card proposes
  it as a route entry of its own. It also sits in `src/cmd/verify*`, not `viola-state`.
- The CARRY's count of standing dirs, re-derived at P3 (`ls -d .viola-verify-*` at the repository root, then under
  `crates/viola-e2e/`): the root holds none today, where the CARRY named eleven; `crates/viola-e2e/` holds one,
  `.viola-verify-227786-plan`, a name the handoff does not list (it lists `.viola-verify-2676638-plan/` there).
  Whatever stands is the operator's. Nothing here touches it.

### What else is owed elsewhere
- The readers' fallback (item 2): `viola revive` for the session id; "The board: viola list" for `wheel`,
  `budget_paused` and the liveness reading; "Session links" for the replay's `links`; "Budget governor" for
  `budget_override_until`'s writer; the client pre-check's behaviour on an unreadable snapshot with the Epoch 6
  entry "Server verification before any frame".
- Test-plan §6 Scenario E5 as written needs `link`, a budget pause and `list --json`, none of which exist, and its
  `wheel:"human"` after a second `viola run` disagrees with the landed start, which appends `wheel{driver, start}`
  on every start (`src/cmd/run.rs:432-441`; architecture `:306`). E5 is not built here and the disagreement is
  named on the review card.
- Test-plan §6 Chaos suite: the torn-append case's "next append starts on a fresh line" half is built here; its
  `skipped.torn_lines` reading on `list --json` and `/api/sessions`, and the whole forward-compat case's
  two-surface reading, are owed where those surfaces land. The counts those readings need are built here.

## The capability
- `verification-matrix.json#v1-41` (Crash-safe, self-healing, version-tolerant state; method `e2e`; unclaimed).
  With the cut it is advanced and not proven: "shown" has no surface and no reader heals by replay yet. P4 reads
  the claim; the lean is no claim and one dated ledger note. No working-route line names the id.

## Boundaries
- No mutation run, no workflow dispatch, no local witness run, no live `claude` start (`inputs#I1`).
- The UI's rebuild "by replaying and tailing" is Epoch 8's; `viola revive`, the board and the budget governor are
  their own entries.
- No Epoch 6 hardening: server verification, home integrity and the seven dated exceptions stand as written.
- Universal invariants that bound every item: `events.ndjson` is never truncated; only the wrapper writes
  `snapshot.json`; one `write` per ndjson line; 0700 dirs and 0600 files set explicitly; every external reader goes
  through `Read::take(MAX_FRAME)`; no `deny_unknown_fields` on viola's own formats; logs only through `obs_event!`;
  external errors carry codes and fixed messages; user content only in `diagnostics/detail-*.ndjson`.
