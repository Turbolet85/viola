# Resume point — wrap of 2026-10-10-session-links

**Phase 1 is done. Phase 2 is next.** Written 2026-10-11T00:38Z by the session that ran implement (Part A), the
operator pass and this wrap's Phase 1. The operator directed the stop with the wrap's invocation (`inputs#I5`):
the reconcile runs in a fresh context, and the route directions come with the resume.

## How to resume

- Setup 2a applies: `chunk_dir/report.md` exists. Resume in THIS run dir
  (`.andromeda/runs/2026-10-11T00-33-08-wrap/`), reuse the report as it is, and start Phase 2 at its first step,
  the citation sweep (`cites.py apply --dry-run`, then without). No fan-out was made. No master, sidecar, key
  file, rule, leaf, route line or matrix entry was touched by this wrap.
- Re-fire Phase 1's two reads first, as a resume does: `gate.py scope` (read `clean` here: changed 13, listed
  13, recorded 0, base `f02d5486`) and `cites.py added --out <run dir>/new-text.md` (expect `unchanged`; the
  listing is the report's last section, pasted by `splice.py` at report line 366: 144 lines, 115 blocks).
- `inputs.py verify` read 5 entries, 0 drifted, 0 uncited once the report existed. This wrap snapped one:
  `inputs#I5`, its own invocation. The route directions given with the resume are a new input: snap them
  (`--step wrap`) before Phase 5 cites them.
- Setup facts read here: branch `build/viola-0.1.0`, 0 ahead of `origin/build/viola-0.1.0`, HEAD `3cae2ce`.
  The basis is `f02d548`, the parent of the one pre-CI commit `3cae2ce`. The pass made no fix commit.
  `state.yaml` read `last_wrap` 2026-10-10T23:17:26Z, `session_count` 62, `tree_db_refreshed_at`
  2026-10-10T22:37:14Z.
- The code-graph refresh fired at this Setup finished: rust 5311 nodes / 28724 edges in 38 s, ts 7 / 1.
  `.andromeda/cache/.refresh-done` was written at 2026-10-11T00:33:54Z; that is Phase 4's
  `tree_db_refreshed_at`. Do not fire it again unless a source file moves.
- The tree is dirty only with bookkeeping and this wrap's products: the friction ledger, the implement run
  dir's gate trail, `evidence/operator-pass.md` (its sections written after the pre-CI commit), the inputs
  manifest with `I5` and its copy, the report, this run dir.
- `gate.py hygiene --run-dir … --marker …` read `clean` after the report was written (read 59, base
  `f02d5486`). This file was written after that read: keep host paths out of it and re-read at P7.3c.
- Epoch 4 now: 10 entries, 8 complete, this chunk pending, 1 markerless ("The board: viola list"). It does not
  close at this wrap, so step 3b's consolidation does not fire and there is no evolve nudge, unless the route
  directions change the epoch's tail.

## What this session was, in order

1. `/andromeda-new-session`: orientation only. 14 of 14 health checks passed; one telemetry record.
2. `/andromeda-implement` on Part A (`inputs#I4`): steps 1 to 10 built, the red readings taken on two stub
   bodies first, then the built code, then two forced-on builds; the block fired once (13 green, the
   `binary(channel_links)` entry red as the hold, 3 operator entries not run); scope clean.
3. The operator pass on the operator's word (`inputs#I4`): `pre-push` green, hygiene clean twice, the pre-CI
   commit `3cae2ce`, the push (a fast-forward), CI `ci#38098238731` green on attempt 1, no fix commit.
4. This wrap's Setup and Phase 1 (`inputs#I5`).

No live `claude` was started. No mutation run, no witness run, no workflow dispatch. No ledger row, no stamp,
no rig home. Nothing of Part B was written, not even as a stub.

## For Phase 2's Validate: what needs the operator, and what must not be written as ratified

- **Part B is held, and the widening is NOT ratified.** The founder's word that step 11 asks for was not
  recorded in this conversation: no relay of it arrived. No master may be amended as if `link` / `unlink` were
  served, as if the two verbs existed, or as if a dated gap for them had been granted. The plan's last
  expected-amendment entry ("only when Part B lands") is **not carried**; the report says so with the reason.
- **What the masters may now say, and no more:** the two kinds exist; the wrapper appends `link` on an
  implicit link (an issued `send`, a taken `answer`, from another instance); the replay and the wrapper's
  start derive the set; the snapshot's `links` is filled. **No product path appends an `unlink` line**:
  `LinkSet::remove` exists and is unit-tested, with no caller. A sentence saying the wrapper appends `link`
  and `unlink` would be false for `unlink` today.
- **Four shapes landed on the operator's word, none ruled by the founder** (the playbook rule "Held items
  landed on the operator's word" applies as at the last wrap; each goes into the masters as the operator's
  answer or a planning lean, never as ratified):
  1. a refused `send` or `answer` records no link (`inputs#I2`, answer 1);
  2. a `from` equal to the wrapper's own name links nothing (the plan's lean, confirmed by the operator's
     approval word, `inputs#I3`);
  3. a `link` line's `source` is `wrapper` (the same lean and word);
  4. the record sits directly before `send-issued`, so a send that is issued and then reads `not-delivered`
     has still linked (the plan's note "That is issued"; `inputs#I3`).
  Two more of `inputs#I2`'s answers (the `link` result `{since, already_linked}`, an `unlink` of an absent
  pair) belong to Part B and did not land.
- **One shape no input words, taken at implement:** a `link` line with no string `ts` gets
  `since: "unknown"`. Report, Deviations 4. It is the operator's to accept or send back; if kept, architecture's
  [Links] or the snapshot contract may want one clause for it.
- **One plan claim listed as disproved by measurement** (report, Spec claims disproved): the plan's note that
  its `schema-check` entry reads the new cases' lines. Validate check 6 must end it DISPOSED. It is a plan
  note, and no master sentence was read for it. Before disposing, grep the masters and the test-plan and
  obs-plan key files for a sentence that says G4 or `schema-check` reads the test homes on the dev host: if
  one does, that sentence is the drift; if none does, the disposition is "plan note only, nothing to amend",
  with a learning for phase (below).
- **A fact the detectors may trip on:** the previous handoff and the masters say the test for `answer`,
  `wait`, `last` and `pause` under a closed gate is owed and name this entry as its owner. It landed
  (`path6_a_closed_gate_leaves_answer_wait_last_and_pause_alone`). The three sentences to change are at
  `architecture.md:72`, `a11y-plan.md:823` and `test-plan.md:863` (read at `f02d548` plus this chunk; the
  citation sweep may move the numbers).

## Findings for the masters and the route (the report carries each; this is the list)

- Sites of the six carried expected amendments, each with its pattern and hit count: report, "Expected
  amendments (from plan)". The site script was a scratch file: one pattern per call, matches counted per
  master and per file under `.andromeda/registries/`.
- **Order facts worth one clause each in architecture:** in `send` the record comes after the second wheel
  and running-turn reads and before `ctx.issue`; in `answer` it comes after the kind check and before the
  `dialog-answered` line, under the slot's state lock, so a link that cannot be recorded answers `-32603`,
  writes no `dialog-answered` line and leaves the dialog held.
- **The start reads the log twice** before the endpoint serves (`WaitFeed::rebuild`, then
  `LinkSet::restore`), each one pass through `read_from`. security-plan's row "Own state files on read" is the
  expected home.
- **`EventKind` order:** `Link` and `Unlink` are the last two variants. `.claude/rules/events.md` lists the
  kinds in another order (it already named both); that list is prose, not declaration order.
- **`.claude/rules/events.md` says "`links` replays empty until the link kinds land"** (its Snapshots and
  liveness bullet), and `.claude/rules/api.md` lists `link`, `unlink` among the channel methods. The first is
  now false and is a leaf to re-derive; the second was already the contract's list and stays true of the
  contract, with both methods still unserved.
- **The route.** The operator's directions arrive with the resume. What they need to settle, from this
  conversation:
  1. where steps 12 to 15 live while step 11 holds (the plan leaves it to this wrap), and so whether this
     record flips `complete` on Part A;
  2. the previous wrap's CARRY on this entry (the folded budget test) is discharged by this chunk;
  3. the dead-code allow on `LinkSet::remove` ends with the `unlink` method: it belongs on whatever entry
     owns Part B;
  4. the widening itself is still owed to the founder, with its three places and two forms (plan step 11,
     `scope.md` "Held for the review card").
- **For "The board: viola list"** (the next markerless entry): the snapshot now carries real `links`; the
  plan's Boundaries say v1 `list` shows no link column.

## For the handoff's founder and operator sections

- **New for the founder at this wrap:** the four shapes above landed on the operator's word; the widening of
  the explicit verbs is owed his own word and Part B waits on it; `since: "unknown"` for a line without a
  `ts`.
- **Carried from the last handoff and unchanged by this chunk:** the four budget items landed on the
  operator's word; the three relied-on dependences with no ledger row; everything under "For the founder
  (carried)", "Owed, no entry minted", "Standing rules", "Before a census run", "After a `cargo clean`" and the
  `windows-mutants` note. This chunk added mutable code to the root package (`src/run/links.rs`, 105 product
  lines, and blocks in `send.rs`, `dialog.rs`, `cmd/run.rs`), `viola-state` (`apply_link_line`) and
  `viola-core`, and ran no mutation run: add it to the note for whoever dispatches next.
- **Changed by this chunk in the last handoff's lists:** the route line "Session links (a case for `answer`,
  `wait`, `last` and `pause` under a closed gate…)" is done; `v1-13` and `v1-36` carry dated ledger notes from
  the phase and are advanced, not proven; "Not measured" loses nothing and gains this chunk's (below).
- **Not measured by this chunk:** any mutation score; anything against the real `claude`; anything of Part B;
  the link code under two wrappers of one home (each case boots one `builder`); a log holding an `unlink`
  written by a product path (none exists); `viola revive` over a log with links on a live session (the unit
  and root cases restart through `run`, not `revive`).
- **Operator desk, from this session (none committed, none removed):** scratch files in this session's
  scratchpad (the kept copies of `send.rs` and `dialog.rs` from the forced-on runs, the run logs, three
  scripts); the gate logs under `$TMPDIR/andromeda-gate/2026-10-10-session-links/` (the implement run's and the
  push entry's); `target/wincheck/`, used again; `target/run-archive/` entries of this session's harness runs;
  `target/agent-run/artifacts/`. Six `viola-test-*` homes dated 2026-10-08 stand on the test-home base and are
  earlier chunks'. Nothing depends on any of them.
- **Last failed command:** none.

## Curation candidates (Phase 3; the conversation that held them is gone at the resume)

Each is a fact from this session with its evidence. The filters and the tier are Phase 3's.

1. **A plan step that puts an operation in the unheld part while its only caller sits behind the stop makes
   dead code under `-D warnings`.** Here: `LinkSet::remove` (step 4) against step 12's `unlink`. Implement
   built it with `#[cfg_attr(not(test), allow(dead_code))]`. A plan with a held part should either place such
   an operation with its caller or say the allow. (A phase-side learning; friction record
   `2026-10-11T00:16:16Z-b`.)
2. **A plan's forced-on edit must name a handle the target holds.** Step 10 worded "a gate evaluation added
   to `DialogSlot::answer`"; the slot holds no gate. The snapshot's `budget_paused` stood in. (Friction record
   `2026-10-11T00:16:16Z-c`.)
3. **On the Linux dev host `schema-check` (G4) reads kept homes only**: a root test's home is removed when the
   test ends, so a chunk's entry reads the booted smoke home and the kept live homes, and its figures can
   equal the previous chunk's. A plan note must not say that entry validates the new cases' lines; the cases'
   own schema read and CI's G4 step do. (Friction record `2026-10-11T00:20:26Z-b`; candidate home:
   `verification-harness.md`.)
4. **Red-first on stubs, as done here:** stub the innermost bodies that hold the behaviour (the rule, the
   set's one write path) and leave the call sites in place; then every presence case reads red in one run per
   filter and every absence case reads green, which is the list that needs forced-on runs. Two forced-on
   builds covered five guards' cases. Restore scratch edits from a kept copy and read SHA-256 back, never by a
   reverse Edit: the format hook reflows after every write.
5. **`clippy::cloned_ref_to_slice_refs`** fires on `assert_eq!(v, [x.clone()])` in test code: bind the
   one-element array once. Found by a hand lint probe before the block, which saved a red block run.
6. **`recurrence-despite-learning: host-linux.md` Transports:** one inline python heredoc for a source edit
   (step 1). It landed whole.
7. **`recurrence-despite-learning: a time written without the clock`**, caught before it was pasted: the
   report's process-list time was first typed from memory and then replaced by a `date -u` reading taken with
   the measurement.
8. **A harness `run` document names its archive** (`"archived":"target/run-archive/<n>"`), which identifies a
   one-shot reading in `evidence/` without a clock time.
9. **A designed red can be told from a real one by `gate.py show --n <n>`**: the held entry's log held
   nextest's own "no binary names matched", the text of its baseline, and no test ran.
10. **Reading a CI run's three test logs in one script** (the run's attempt, each job's steps, `PASS` counts
    by binary and by name prefix, the G4 documents) took one call and gave every figure of the pass's record.
11. **The gate tool prints nothing until its block ends**, so a backgrounded block shows an empty output file
    while it runs; the entry in flight is read from the process list.

Nothing in this conversation was a user correction of the agent's work, and no "from now on" convention was
given. The operator's two directions are `inputs#I4` and `inputs#I5`.

## Counts the masters may state, with their rules (report: Counts / qualifiers moved)

- `EventKind::ALL`: 15 (the array's length). test-plan states 13 once.
- The 20 s kill filter line: 15 binaries (the `binary(` names on `.config/nextest.toml:32`). The test-plan key
  file states "fourteen" with the list of names; `cli_links` joins it after `cli_instance_state`.
- Root waits: 28 sites in 20 files, unchanged (`Instant::now() + WITHIN` under `tests/`).
- Product callers of `Snapshots::update`: 5. No master states the old 4.
- `tests/cli_budget.rs`: 10 cases by nextest's count (9 before); `tests/cli_links.rs`: 5; `state_replay`: 9
  (7 before); `state_events`: 4, unchanged; the four-prefix inline cases: 34.
- Unit suite on the dev host: 1899. CI: 2322, 2317, 2321.

## What Phase 7's light gate should know

- **The `binary(channel_links)` entry reads red by construction** while step 11 holds, and the light gate will
  meet it. It is not a `defer`, not a `leg`, and not a red of another chunk's cause, so none of the ASSERT's
  arms fits it as written. The plan's own prose calls it the hold. Take it to the operator before the gate:
  the form that fits the contract is `--skip {n}:{reason}` with the reason from the report ("step 11 holds
  Part B; `tests/channel_links.rs` is not built"), which is then a recorded treatment. Never read it as green
  and never let it stop the flip without the operator's word.
- The three `leg = 'operator'` entries are never re-run: they are re-verified from
  `evidence/operator-pass.md` (hygiene clean twice; the push's `history moved` line; `ci#38098238731`,
  attempt 1, `verdict: green`, 15/15, on `3cae2ce`, the final HEAD of the pass).
- The preservation guard names `f02d54865815` and stays green for as long as no held path moves.
- Timings here: the block about 90 s, of which `pre-push` 63 s; the smoke session is `p-links-smoke`.
- Coverage gate: the chunk claims no capability (`claimed 0 — gate no-op`). `v1-13` and `v1-36` already carry
  the phase's dated notes; no ledger note is listed in the plan for P7.3.
- `walk-class rust 3` was printed by the block's header (three contract tests walk the tree). No entry of this
  block carries a `defer`, so nothing is voided by it.
