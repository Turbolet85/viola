# Scope — 2026-10-26-explicit-session-links: "Explicit session links"

The working entry (Epoch 6 — Security hardening): "Explicit session links — `viola link` / `unlink` verbs and their
channel methods, held for the founder's word on the boundary widening", with three CARRYs, folded below.

## Take-up

- The founder's word at the take-up is «Взять, жду P4»: take this entry up; he picks the widening's form himself at
  P4, asked as a live dialog, his answer relayed verbatim (`inputs#I1`, `inputs#I2` item 5). `[inferred]` That is
  the word the hold names: the founder's own word, shown the widening as itself, naming `viola link` and
  `viola unlink` and one form. It is asked at P4, so this scope plans every form up to it, and the plan's steps for
  the verbs follow the form he names.
- The operator's directive for this phase (the overseer, founder-delegated, `inputs#I2`): a forced sample at P2;
  every subagent and Workflow transcript kept and the run dir left whole; a stop, with the run dir, at a hard
  cascade failure or agents writing nothing or doing the orchestrator's work; the P2 report items; the widening's
  form asked of the founder at P4; no epoch boundary inside Epoch 6; `df -h /tmp` read before a stamped-home gate.

## What it builds (the entry and its first CARRY, the earlier plan's steps 12 to 15)

The CARRY re-pinned from "The board: viola list" at its wrap, by the review card the founder approved (chunk
2026-10-11-the-board-viola-list, `chunks/2026-10-11-the-board-viola-list/inputs/I3-relay-3.md.txt`; owners
`chunks/2026-10-11-the-board-viola-list/inputs/I8-board-card-owners.md.txt`; placement
`chunks/2026-10-11-the-board-viola-list/inputs/I7-relay-3.md.txt`,
`chunks/2026-10-11-the-board-viola-list/inputs/I9-relay-4.md.txt`), its text as it stood there: chunk
2026-10-10-session-links (the operator's direction at its wrap,
`chunks/2026-10-10-session-links/inputs/I6-relay-2.md.txt`) made that plan's steps 12 to 15 one carry on this entry.
Owed:

- **The two channel methods.** The wrapper answers `link` and `unlink` `-32601` today. Served: `link` records the
  pair and answers `{since, already_linked}`; `unlink` removes it and answers the empty object; an `unlink` of an
  absent pair appends nothing; a `driven` that is not the serving wrapper's name and a `driver` equal to `driven`
  are `-32602` (the operator's answers, `chunks/2026-10-10-session-links/inputs/I2-relay-2.md.txt`,
  `chunks/2026-10-10-session-links/inputs/I3-relay-3.md.txt`).
- **The three result lines** (`link … since …Z`, `link … already linked`, `unlink …`).
- **The two verbs**, `viola link <driver> <driven>` and `viola unlink <driver> <driven>`, with `--json`; the verb's
  instance is the driven one.
- **Their root cases**, with `tests/channel_links.rs` (that plan's step 15): test-plan §6 Scenario E3's steps 1 and 3
  and the raw-channel name negative are still owed with the verbs.
- **The `unlink` product path.** No product path appends an `unlink` line: `LinkSet::remove` (`src/run/links.rs`) is
  unit-tested, has no product caller and carries `#[cfg_attr(not(test), allow(dead_code))]`, which ends when the
  `unlink` method lands.
- **`v1-13`'s `link` half** (`verification-matrix.json#v1-13`): every human `link` line derivable from its machine
  result, and the `--json` / MCP result carrying the same facts. `[inferred]` The `release --budget` half landed with
  chunk 2026-10-10-budget-governor and the MCP surface with chunk 2026-10-17-mcp-server-for-drivers, so whether this
  chunk can prove the whole acceptance, the MCP half included, is P3's to read (the cap names an MCP result; the MCP
  tools are `send · wait · last · answer · list`, none of them `link`).

## The hold (as the entry states it)

- The explicit verbs wait for the founder's own word on a boundary widening, given after it is shown to him as
  itself and recorded as an input; no earlier direction, the operator's included, stands in for it (playbook
  "Boundary widening — what ratifies it"). Until his word names `viola link` and `viola unlink` and one of the three,
  nothing of steps 12 to 15 is planned or built, and no master says either verb or method is served.
- The widening crosses the local trust boundary in three places (that chunk's `scope.md`, "Held for the review card,
  unsettled"): (a) the wrapper first serves two more channel methods; (b) the two verbs would write their frame after
  the liveness-only pre-check, with no server identity; (c) they would read `snapshot.json`'s `endpoint`, `pid` and
  `started_at` without the strict-modes check.
- His three options, quoted from that plan's step 11: "the two methods served by the wrapper, and the two verbs
  writing their frame after the liveness-only pre-check with the snapshot read under no strict-modes check, as a dated
  gap of their own until the Epoch 6 entries "Server verification before any frame" and "Home and code-bearing file
  integrity""; "the instance check ahead of the snapshot read as `viola revive` takes it"; "no explicit verbs before
  Epoch 6".
- His earlier word came in the Epoch 4 boundary batch (his own answer, 2026-10-13, relayed verbatim by the Viola
  overseer): «Не делать до Эпохи 6», the third option. This entry moved by it into Epoch 6, after "Windows endpoint
  admission" and "Server verification before any frame".
- `[inferred]` Both of those entries are now complete (master records `2026-10-25-windows-endpoint-admission` and
  `2026-10-26-server-verification-before-any-frame`). Crossing (b) is therefore no longer what the first option
  describes: every frame-writing client now checks the serving process's pid and start time against `snapshot.json`
  before its first frame on Linux and Windows (macOS the recorded gap), so the two verbs would take that check through
  `client::connect` like `send` does. What stays open is (a) and (c): the first option's dated gap would now cover
  the strict-modes half only, until "Home and code-bearing file integrity". The third option's literal reading
  ("before Epoch 6") is spent by the calendar of the route. P3 reads the client path at HEAD; P4 asks the founder with
  the options re-stated against HEAD, each priced, the three originals named.
- `[inferred]` The second option ("the instance check ahead of the snapshot read as `viola revive` takes it") is
  `viola_state::strict::check_instance` run before the snapshot read. Whether it can be taken for these two verbs
  alone without taking the strict-modes work of "Home and code-bearing file integrity" for the eight other clients
  (that entry's CARRYs) is P3's to read.

## Standing shapes (Part A of the earlier chunk, landed)

- Landed and standing: the kinds `link` and `unlink`, the implicit link on an issued `send` and a taken `answer` from
  another instance, the wrapper's `LinkSet` restored from the log at every start, the replay's link half, and the
  snapshot's `links` as `{driver, driven, since}` items.
- Five shapes of Part A landed on the operator's word and are not ruled by the founder (architecture [Links]).

## The second CARRY — a logged link name shown

Chunk 2026-10-11-the-board-viola-list (the operator's rule for that chunk,
`chunks/2026-10-11-the-board-viola-list/inputs/I1-relay-1.md.txt` item 3, and the review card the founder approved,
`chunks/2026-10-11-the-board-viola-list/inputs/I3-relay-3.md.txt`,
`chunks/2026-10-11-the-board-viola-list/inputs/I8-board-card-owners.md.txt`): the first surface that shows a logged
link name — these verbs' result lines, or `/api/links` — parses it through `ViolaName::try_new` again; the board shows
no link. For this entry's own board: that plan's Boundaries say v1 `list` shows no link column.

## The third CARRY — the help row

Chunk 2026-10-17-cli-output-discipline (its P5 review card's row, approved by the operator's word,
`chunks/2026-10-17-cli-output-discipline/inputs/I3-relay-3.md.txt`): `viola --help` gains its `handoff:  link, unlink`
row between `wheel:` and `setup:` (layout-templates §Output structure — `viola --help`) when this entry serves the
verbs; the `HELP` literal of `help_is_the_grouped_verb_table` and `help_lists_only_served_verbs` move with it.

## Boundaries

- No ledger row, no `viola verify` child, no re-stamp and no live `claude` start `[inferred]`: a link is recorded
  from viola's own parameters, and the earlier chunk's scope found no file of `crates/viola-agent-claude`,
  `fixtures/claude` or `src/cmd/verify` touched; P3 re-reads it.
- No `link` / `unlink` MCP tool `[inferred]`: the entry names the verbs and the channel methods only; whether `v1-13`
  needs one is the claim question above, not a build of this entry.
- No link column on `viola list`.
- The strict-modes check of the other eight clients stays "Home and code-bearing file integrity"'s.

## CI read at Setup (the last wrap's flip through HEAD)

- `8fc3d1df` — green, checks 15/15, wall 762 s, `ci#40322505434`.
- `086320a0` — green, checks 15/15, wall 670 s, `ci#40322327024`.
No red to fold.
