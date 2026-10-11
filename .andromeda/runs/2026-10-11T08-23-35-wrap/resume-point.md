# Resume point — wrap of 2026-10-11-the-board-viola-list

**Phase 1 is done. Phase 2 is next.** Written 2026-10-11T08:28Z by the session that ran implement, the operator pass
and this wrap's Setup and Phase 1. The operator directed the stop with the wrap's invocation (`inputs#I5`): this is
the epoch-close wrap, and its reconcile and consolidation run in a fresh window.

## How to resume

- Setup 2a applies: `chunk_dir/report.md` exists. Resume in THIS run dir
  (`.andromeda/runs/2026-10-11T08-23-35-wrap/`), reuse the report as it is, and start Phase 2 at its first step, the
  citation sweep (`cites.py apply --dry-run`, then without). No fan-out was made. No master, sidecar, key file, rule,
  leaf, route line, playbook or drift-base entry, or matrix entry was touched by this wrap.
- Re-fire Phase 1's two reads first, as a resume does: `gate.py scope` (read `clean` here: changed 13, listed 13,
  recorded 0, base `995dc55a`) and `cites.py added --out <run dir>/new-text.md` (expect `unchanged`; the listing is
  the report's last section, pasted by `splice.py` at report line 350: 221 lines, 194 blocks).
- `inputs.py verify` read 5 entries, 0 drifted, 0 uncited once the report existed. This wrap snapped one:
  `inputs#I5`, its own invocation. Any direction given with the resume is a new input: snap it (`--step wrap`)
  before a phase cites it.
- Setup facts read here: branch `build/viola-0.1.0`, 0 ahead of `origin/build/viola-0.1.0`, HEAD `acad7af`. The
  basis is `995dc55a3d6b`, the parent of the one pre-CI commit `f1ee634`; the pass made one fix commit, `acad7af`.
  `state.yaml` read `last_wrap` 2026-10-11T00:59:13Z, `session_count` 63, `tree_db_refreshed_at`
  2026-10-11T00:33:54Z.
- The code-graph refresh fired at this Setup finished: rust 5488 nodes / 29946 edges in 40 s, ts 7 / 1.
  `.andromeda/cache/.refresh-done` holds no time; its mtime reads 2026-10-11 08:24:25Z (`TZ=UTC stat -c %y`), and
  that is Phase 4's `tree_db_refreshed_at` (2026-10-11T08:24:25Z). Do not fire it again unless a source file moves.
- The tree is dirty only with bookkeeping and this wrap's products: the friction ledger, the implement run dir's
  gate trail and `journal.md`, `evidence/operator.md` (its sections after the pre-CI commit), the inputs manifest
  with `I5` and its copy, the report, this run dir.
- `gate.py hygiene --run-dir … --marker …` read `clean` after the report was written (read 64, base `995dc55a`). This
  file was written after that read: it holds no host path; re-read at P7.3c.
- **This wrap closes Epoch 4** (`route.py epoch`: Epoch 4, entries 10, markerless 0, complete 9, pending 1 = this
  chunk). P7 step 3b's sidecar consolidation fires (`references/consolidation.md`), before the flip. Two sidecars
  were over the 120 000 B whole-read bound at the last wrap: `architecture-amendments.md` (186 843 B) and
  `test-plan-amendments.md` (140 677 B). The evolve nudge applies at P7.6: Epoch 4 has no
  `*-evolve-diagnose/proposals.md` (the four diagnoses target Epochs 1, 2, 2b, 3).

## What this session was, in order

1. `/andromeda-new-session`: orientation only. 14 of 14 health checks passed; one telemetry record.
2. `/andromeda-implement` (`inputs#I4`): steps 1 to 9 built. The block fired 6 times (`journal.md`): firing 1 red on
   one harness unit case after the morning's reboot (below), firings 2 to 6 green. Six remove-the-guard readings, all
   red, all restored by copy with SHA-256 read back (`evidence/red-green.md`). One hand smoke of `viola list`
   (`evidence/smoke.md`). Scope clean.
3. The operator pass on the operator's word (`inputs#I4`): `pre-push` green, hygiene clean, pre-CI commit `f1ee634`,
   push, CI `ci#38123340339` **red** (`test (windows-2025)`, four cases), fix commit `acad7af`, push, CI
   `ci#38124062614` **green** 15/15 (`evidence/operator.md`).
4. This wrap's Setup and Phase 1 (`inputs#I5`).

No live `claude` was started. No mutation run, witness run or workflow dispatch. No ledger row, no stamp, no rig home.

## For Phase 2's Validate: what needs the operator, and what must not be written as ratified

- **Landed on the operator's word, none ruled by the founder** (the playbook rule "Held items landed on the
  operator's word" applies as at the last wraps; each goes into a master as the operator's answer or a planning
  form, never as ratified):
  1. a failed strict-modes check of the home, `instances/` or one instance refuses the whole board, exit 21, no log
     opened, no line written, the home's mode as found (`inputs#I2`);
  2. the five board forms of plan step 3 (figure rounded down; whole-minute age and `read unknown`; bare skip
     counts; `human - budget-paused`; the empty board's three lines) — planning forms (`inputs#I3`);
  3. the home pair's words come from design-system; `list` is its first writer in source.
- **Two things implement decided that no input words** (report, Deviations 1 and 4): WHEEL at 23 (the plan said 22;
  its own widest-word rule gives 23), and the two failure paths (an unlistable `instances/` is exit 1; a per-instance
  read error gives no row). Both are the operator's to accept or send back.
- **Four disproved claims** (report, "Spec claims disproved by measurement"); Validate check 6 must end each
  DISPOSED:
  1. `test-plan.md:591` says `list` keeps `\n` and `\t`; design-system.md:747 and layout-templates.md:580 say the row
     escapes them, and the landed rows do. A real master contradiction: the test-plan line is the drift.
  2. The plan's WHEEL 22 — plan text; check whether any master states a WHEEL width before disposing.
  3. research.md's "the budget readings are Unix root cases" — research text; the Path 6 list case passed on
     `windows-2025`. Read what security-plan and architecture say of the statusline arm's Windows behaviour before
     disposing.
  4. `registries/contracts/test-plan/status-endpoint-shape.md` line 19, "while `viola list` / `viola ui` are
     unbuilt" — `viola list` is built; the `list` member is `null` because `status` runs no `viola list`. A key-file
     amendment (key files are master text), with its owner by title.
- **Callers that moved** (report, Remaining-caller facts and Counts): the snapshot fallback now has 2 product readers,
  `check_instance` 3, `read_thresholds` 2, `read_budget_stamped` 2 paths; the skip counts have their first product
  reader. The "one product reader / one caller" hits are listed in Counts (architecture 6, security-plan 3, test-plan
  1, obs-plan 1); not every hit is about these functions — each is read.
- **security-plan.md:245** says `list` resolves `claude` for `claude agents --json` from PATH. As landed `list`
  spawns nothing. That sentence describes the full contract; the amendment states what is landed and leaves the
  contract, with its owner (the founder's matters, scope.md "Held for the review card").

## The red in implement's run, and the light gate

- **Firing 1's red** (`run --unit`, not this chunk's): `viola-e2e
  harness::cleanup::tests::remove_session_home_removes_a_boot_created_parent` failed `AlreadyExists` at
  `crates/viola-e2e/src/harness/cleanup.rs:366`, its `fs::create_dir_all(ws.e2e_home())` over the link
  `target/e2e-home` while the link dangled (the host booted 2026-10-11 07:18:11Z; the tmpfs target was re-made by a
  parallel case at 07:50:10Z). Std's `create_dir_all` over a dangling link returns `AlreadyExists` — measured
  two-sided in a scratch program. This chunk edits no `crates/viola-e2e` file (preservation guard green). The report's
  Outcome writes it `red — not this chunk's: … → owner: this wrap's P5 pin (not yet minted)`. **P5 owes the pin** (a
  CARRY on a harness-owning entry, a minted prior-debt entry, or a `residuals.md` line) — and decides whether the
  basis needs a run of the parent tree in a clean worktree; P7.1's ASSERT needs basis and owner both.
- The light gate fires the whole block fresh. The base exists now, so entry 3 should read green; a red there with the
  same diagnostic is the same cause (the base gone again — read `test -L target/e2e-home && findmnt -n -o FSTYPE -T
  target/e2e-home/` first; the standing rule says stop and report).
- The three `leg = 'operator'` entries are never re-run: they are re-verified from `evidence/operator.md` (hygiene
  clean before both commits; the two pushes' `history moved` lines; `ci#38124062614` `verdict: green`, 15/15, on
  `acad7afc865a`, the final HEAD of the pass).
- The preservation guard names `995dc55a3d6b`.
- Timings here: the block about 90 s (`pre-push` about 65 s); the smoke session is `p-board-smoke`.
- Coverage gate: the chunk claims no capability (`claimed 0 — gate no-op`). `v1-37` and `v1-41` carry the phase's
  dated notes; the plan lists no ledger note for P7.3.
- `walk-class rust 3` is printed by the block's header; no entry carries a `defer`.

## For the route (P5)

- No CARRY on this entry needs discharging beyond what scope.md folded: self-healing-state's freight is in full here
  (the counts on `list --json`, the chaos readings, E5's `list` step), budget-governor's board half, cli-output-tokens'
  escaper. Every held part has its proposed owner on the review card (scope.md "Held for the review card"), approved
  by the founder (`inputs#I3`); P5 places each as the card says.
- The pin for firing 1's red (above).
- **For the harness's owner:** the four Windows reds came from a fixture fact: `seed_conpty` makes an unstamped home
  with an inherited DACL on Windows x64, so any reader running the strict-modes check on the home refuses it there.

## For the handoff's founder and operator sections

- **New for the founder at this wrap:** `viola list` landed for wrapped sessions only, uncoloured, no unwrapped row,
  no link; the exit-21 board refusal and the five board forms stand on the operator's word; WHEEL at 23; Epoch 4
  closes.
- **Carried from the last handoff, unchanged by this chunk:** the explicit `viola link` / `unlink` hold; the five
  implicit-link shapes; the budget items; the three relied-on dependences with no ledger row; everything under "For
  the founder (carried)", "Owed, no entry minted", "Standing rules", "Before a census run", "After a `cargo clean`".
  Add to the `windows-mutants` note: this chunk added mutable code to the root package (`src/cmd/list.rs`,
  `src/human.rs` board writers) and to `viola-state` (`board.rs`, `budget.rs`), and ran no mutation run.
- **Not measured by this chunk:** any mutation score; anything against the real `claude`; the refusal cases on
  Windows (`cfg(unix)` only); a board over two live wrappers in one home (each case boots one `builder`); a `list`
  under a home whose `config.json` holds a bad threshold (the code path is `viola run`'s, unit-tested there).
- **Operator desk, from this session (none committed, none removed):** scratch files in this session's scratchpad
  (the kept copies of `board.rs`, `budget.rs`, `list.rs`, `cli_list.rs`; run logs; `sites.py`, `lines.py`, the
  dangling-link probe); the gate logs under `$TMPDIR/andromeda-gate/2026-10-11-the-board-viola-list/`;
  `target/wincheck/`, used again; `target/run-archive/` entries; `target/agent-run/artifacts/`. The harness session
  `p-board-hand` was cleaned. Nothing depends on any of them.
- **Last failed command:** none.

## Curation candidates (Phase 3; the conversation that held them is gone at the resume)

Each is a fact from this session with its evidence. The filters and the tier are Phase 3's.

1. **On Windows x64 an unstamped root-test home fails the strict-modes check**: `tests/support/home.rs` `seed_conpty`
   creates it with `fs::create_dir_all` (the runner's inherited DACL) before `viola run`. A root case that runs a
   reader with the home check (`list`, `revive`, `hook statusline`) starts from `stamped_home`. CI only: the dev host
   cannot see it (`seed_conpty` is a no-op off Windows x64). Candidate home: `testing.md` (Test data).
2. **`std::fs::create_dir_all` over a dangling symlink returns `AlreadyExists`.** After a reboot empties the tmpfs
   behind `target/e2e-home`, a test calling `create_dir_all` on the link fails until something re-makes the target.
   Candidate home: `verification-harness.md` or `host-linux.md`.
3. **Lint the Windows target on the host before the push** (the 2026-10-10 testing.md command): it caught a
   `cfg(unix)`-only constant read dead (exit 101) that the host clippy passed. A recurrence-worthy confirmation of an
   existing rule (extension candidate).
4. **A tree-equality assertion over a viola home compares hashes, never bytes:** the home holds the pinned `viola`
   copy, and one red printed it whole (a 64 MB tool result). Candidate home: `testing.md`.
5. **The stamp-ahead hook reads a unit test's synthetic instants dated today as record stamps.** Date a test oracle's
   fixed instants in the past (here 2025-03-01). Candidate home: `testing.md` or the memory note on the hook.
6. **`recurrence-despite-learning: a time written ahead of or without the clock`**, four times this session (three
   journal lines, one evidence window), each corrected by measurement.
7. **The gate trail's own `ts` per firing** dates a journal line exactly; a firing's start is its trail stamp minus
   the sum of its entries' seconds.
8. **`gh api …/jobs/<id>/logs` refuses a log with terminal escapes** ("pass --allow-escape-sequences"); `gh run view
   <run> --log-failed` gives the failed steps' log in one call.
9. **The masters word the `list` row escape as "C0/C1 … including `\n` and `\t`"**, never "row mode" (0 hits), and
   the kill line's count lives in a test-plan key file, not the body: a site sweep needs both forms.

Nothing in this conversation was a user correction of the agent's work, and no "from now on" convention was given.
The operator's two directions this session are `inputs#I4` and `inputs#I5`.

## Counts the masters may state, with their rules (report: Counts / qualifiers moved)

- The 20 s kill filter line: 18 binaries (the `binary(` names on `.config/nextest.toml:32`); 15 at the basis.
- Root test files: 41 (`tests/*.rs`); 39 at the basis.
- Unit suite on the dev host: 1972 (firing 3's `run --unit` log). CI `test (windows-2025)`: 2402 on `f1ee634`.
- `escape_message` product callers: 6, unchanged.
- `EventKind::ALL`: 15, unchanged.
