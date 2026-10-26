# Scope — Server verification before any frame (2026-10-26-server-verification-before-any-frame)

**Working entry (viola-0.1.0/working-route.md, Epoch 6 — Security hardening):** Server verification before any frame —
clients match the recorded wrapper pid and start time; CLI/MCP report unreachable, hooks fail open on mismatch.

**Invocation:** the operator's (`inputs#I2`), with the overseer's directive for this phase (`inputs#I1`): forced sample
at P2, every transcript and the run dir kept, a hard cascade failure stops the phase, a boundary widening held for the
founder's live word (he is asleep; a widening fork is held as such and never answered from a recorded direction).

## What it builds
security.md §Local trust boundary's clause "Before the FIRST frame of any method (including `hook.event`), verify the
server: pid (+ sysinfo start time) against a `snapshot.json` that passed strict-modes in the same process (macOS: euid +
dir only, recorded gap)", built in every client that writes a frame to a wrapper's endpoint, and with it the end of
the dated exceptions the founder ratified "until the Epoch 6 entries".

- The clients, every one of them (verified at P3: the product `Client::connect` / `connect_by` / `TokioClient::connect`
  sites, research.md Sweeps): CLI `viola send`, `wait`, `last`, `answer`, `pause`, `release` (`release` through
  `pause::wheel_call`); `viola hook`'s `hook.event` (every event, `deliver`) and `hook.dialog` (`pre-tool-use` ·
  `permission-request`, `ask`); `viola mcp`'s `send` / `wait` / `last` / `answer` tools (`frame`, Tokio client). No
  other product process writes a frame: `run`'s collision check reads the snapshot and opens no connection, `list` reads
  the disk, `revive` and `verify` hold no client.
- The check, before the first frame: the connected server's process id and its start time match the `pid` and
  `started_at` the instance's `snapshot.json` records. [premise-corrected: the server pid is the connected stream's
  `peer_creds().pid()` on both clients — Linux `SO_PEERCRED`, Windows `GetNamedPipeServerProcessId` on the
  SQOS-adopted client end (interprocess 2.4.4 source, research.md; not run on Windows here, CI's windows-2025 leg is its
  witness) — and the start time `viola_state::liveness::process_start_time` (sysinfo); macOS's `peer_creds()` gives
  no pid, and the euid and the socket-directory checks macOS's verification rests on are "Unix endpoint and home
  hardening"'s (its own freight: "the directory verification … and the peer euid check stay this entry's"), so this
  chunk's pid + start-time check is Linux and Windows, macOS keeping the recorded gap.]
- On a mismatch, or a check that cannot be made: CLI verbs report the instance unreachable (exit 21, the code the
  liveness pre-check uses today — verified: `src/cmd/client.rs:125`); MCP tools return `instance-unreachable`;
  `viola hook` fails open — exit 0, nothing on stdout or stderr, no frame written (the universal invariant).
- The dated exceptions this chunk acts on: the first (`hook.event`, Decisions Log `2026-09-28`), the fourth (`send`),
  fifth (`wait` / `last`), sixth (`answer` and `hook.dialog`), seventh (`pause` / `release`) and eighth (`viola mcp`'s
  frame tools); their text in security-plan is amended at the wrap (a master amendment, not a phase edit). Whether
  each closes WHOLE here or only its server-identity half — every one also names a strict-modes half that "Home and
  code-bearing file integrity" carries — is the boundary question below, settled at P4.

## Boundaries
- "Home and code-bearing file integrity" (the entry after "Explicit session links") carries the strict-modes half for the
  same verbs: `send`, `wait` / `last`, `answer`, `pause` / `release`, the MCP frame tools and the hook client's
  snapshot read each get a strict-modes check there (verified: its line's CARRYs name each verb, "this entry adds
  `send` to the strict-modes entry points", and the hook client's "runs the strict-modes check … before that read").
  security.md's clause requires the snapshot the verification reads to have "passed strict-modes in the same process",
  so verification here either takes the strict-modes check with it or stands on a snapshot read without one until that
  entry — P4's to lean on that freight.
- "Explicit session links" sits between the two and stays held (the founder's «Не делать до Эпохи 6»); nothing here
  serves `link` / `unlink`.
- Windows pipe admission (the listener DACL, remote rejection, first-instance) landed with
  2026-10-25-windows-endpoint-admission; the client's SQOS open already stands. Unix socket hardening is "Unix endpoint
  and home hardening".
- No boundary is widened. A widening found on the way — a client that would write a frame on a new basis, a reader
  added off the trusted-file list — halts for the founder's live word (`inputs#I1` item 5).

## Folded freight (working-route.md, the entry's own line — nine CARRYs)
1. CARRY (chunk 2026-09-27-hooks-to-normalised-events, operator ruling at its P4 fork 2; the gap ratified by the founder,
   live, 2026-09-28, Decisions Log `2026-09-28`): `viola hook` sends `hook.event` to the endpoint `snapshot.json`
   records with no server verification; this entry adds it to the hook client, with its fail-open-on-mismatch case
   (exit 0, nothing on stdout or stderr), closing that dated exception. → "What it builds", hook client.
2. CARRY (chunk 2026-09-29-verify-stamped-test-homes-and-harness): on Windows a `viola hook` fired within milliseconds
   of the wrapper's exit reached the exiting wrapper's own pipe and wrote its `hook.event` (measured at
   ci#36532038635, 25 ms after the wrapper's exit line; "why the pipe outlives the observed exit is not established");
   the notification reads as delivered, so `SessionEnd`'s direct-append fallback is skipped in that window — a product
   consequence no entry owned; tests now wait for the endpoint to be gone (test-plan §3 `run` step 2). This entry owns
   that consequence (verified: the CARRY sits on this line). What P3 established: the exiting wrapper IS the recorded
   process, so a pid + start-time check that passes there is the right verdict, and the check is not built to close the
   window; whether sysinfo still lists a terminated Windows process whose pipe handle is open is unmeasured (research.md
   Open questions) — P4 names the window as unclosed or plans its measurement. The CARRY's mechanism text, kept
   verbatim: "why the pipe outlives the observed exit is not established" [inferred: not re-derived — a Windows-only
   timing this host cannot reach; it stays the CARRY's statement, never this chunk's].
3. CARRY (chunk 2026-10-04-confirmed-send-with-cl-1-records; the founder's ruling, live, 2026-10-04, the fourth dated
   gap): CLI `viola send` writes its `send` frame after a liveness-only pre-check (snapshot pid + start time alive,
   heartbeat live, `endpoint` present), no server identity check — removed here (security-plan IPC client-side server
   verification row; §Anti-Patterns Authentication).
4. CARRY (chunk 2026-10-04-wait-and-last; the fifth dated gap): `viola wait` / `viola last`, the same pre-check —
   removed here with `send`'s.
5. CARRY (chunk 2026-10-04-dialog-answers-by-dialog-id; the founder's ruling F1, the sixth dated gap): `viola answer`
   after the same pre-check, and `viola hook pre-tool-use|permission-request` writing `hook.dialog` after only its
   payload shape check — removed here with `send`'s.
6. CARRY (chunk 2026-10-04-the-wheel; the founder's ruling F-W1, the seventh dated gap): `viola pause` / `viola release`
   after the same pre-check — removed here with `send`'s.
7. CARRY (chunk 2026-10-10-self-healing-state, the operator's direction at its wrap,
   `viola-0.1.0/chunks/2026-10-10-self-healing-state/inputs/I4-relay-1.md.txt`): what a client's pre-check does with an
   unreadable or newer `snapshot.json` is undecided and unchanged — `read_snapshot` reads either as absent, so
   `live_endpoint` finds no endpoint and the verb exits 21; the classified read and the log replay stand as library
   code no client calls. "A client that connected on a replayed endpoint and a heartbeat alone would change the dated
   pre-check's outcome: a boundary matter, the founder's to rule before this entry plans it." → **held**: this chunk
   keeps today's outcome (unreadable or newer reads as absent, exit 21 / fail open) unless the founder's own live word
   rules otherwise; the plan goes up to it behind a stop rule and the fork is raised at P4 as his, never leaned.
8. CARRY (chunk 2026-10-11-the-board-viola-list, the review card the founder approved,
   `viola-0.1.0/chunks/2026-10-11-the-board-viola-list/inputs/I3-relay-3.md.txt` and
   `…/inputs/I8-board-card-owners.md.txt`): the board landed its half of the self-healing carry; the five other
   snapshot readers (`src/cmd/client.rs`, `run`'s collision check, `src/cmd/hook.rs` twice,
   `src/cmd/hook/statusline.rs`) still call `read_snapshot` and take no replay, which keeps
   `verification-matrix.json#v1-41` open and E5's steps 1–3 unbuilt. [premise-corrected: `src/cmd/client.rs` holds no
   `read_snapshot(` call at HEAD (`grep -c`, 0) — the CLI verbs' snapshot read moved to
   `viola_state::liveness::live_endpoint` (`crates/viola-state/src/liveness.rs:75`) at chunk
   2026-10-17-mcp-server-for-drivers; under `src/` four product callers remain (`hook.rs:246`, `hook.rs:494`,
   `statusline.rs:142`, `run.rs:379`), research.md Sweeps.] Those readers' move to the classified read is owed here
   and is bound to item 7's hold — moving a client to the replay is the change item 7 names.
9. CARRY (chunk 2026-10-17-mcp-server-for-drivers, its P5 card's row 4,
   `viola-0.1.0/chunks/2026-10-17-mcp-server-for-drivers/inputs/I3-relay-3.md.txt`; the founder's own answer to its P4
   fork Q1, `…/inputs/I2-relay-2.md.txt`): the eighth dated gap — `viola mcp`'s `send` / `wait` / `last` / `answer`
   write their frames through the Tokio client after `viola_state::liveness::live_endpoint`, checking neither the
   server's identity nor the snapshot's strict-modes; this entry adds the server verification to them, closing that gap
   with the other seven. → "What it builds", MCP tools.

## CI since the last wrap (Setup 5a, `ci-no-marker.json` in the run dir)
Every sha from the last wrap's flip (`6bdfdda8`) through HEAD (`6bdfdda8`), newest first:
- `6bdfdda8` in progress · 15/15 checks · ci#40283761039 in_progress, its oldest open check `test (windows-2025)` at
  185 s — **CI 6bdfdda8: verdict not yet available**, never read as green; no wall-clock yet. The pre-CI commit under it
  (`948f7f01`) read green at ci#40278510123 (the handoff).
