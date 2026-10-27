# Outcome — 2026-10-27-pinned-copy-race-and-reproducible-builds

**green + smoke** — the chunk's gate block green on its first firing (21 green · 1 recorded · 0 red; the three
`operator` entries 23–25 are the operator pass's, below). Red 1 stopped under the plan's stop rule S2, its readings
for the operator. Run dir `.andromeda/runs/2026-10-27T02-08-46-implement/`; gate trail
`.andromeda/runs/2026-10-27T02-08-46-implement/gate-2026-10-27-pinned-copy-race-and-reproducible-builds.json`.

## Files changed
- `crates/viola-state/src/fs.rs` — `Placed { Written, Present }`, `place_private_new` and the private
  `place_private_new_with` (temp beside the target, mode before any byte, synced, closed by `into_temp_path`, then
  `TempPath::persist_noclobber`; `AlreadyExists` is `Present`, the target untouched); unit cases
  `place_private_new_places_the_bytes_once_at_the_mode_and_leaves_no_temp`,
  `…_over_a_present_name_is_present_and_leaves_it`, `…_under_a_missing_dir_fails`,
  `…_placed_copy_execs_at_once` (`cfg(unix)`), `…_placed_copy_runs_at_once` (`cfg(windows)`); the doc of
  `replace_private_shared` names its remaining users (the plugin files, the companions), its body unchanged.
- `crates/viola-state/src/pin.rs` — `pin_bytes`'s `NotFound` arm through the new private `place_absent` (`Written` →
  ok; `Present` → re-hash: equal ok, different `HashMismatch`, the file as found); module doc; `replace_private_shared`
  imported for `pin_companions` alone (`cfg(windows)`, its body unchanged); unit cases
  `place_absent_over_a_copy_another_start_placed` (rstest `same_bytes` / `other_bytes`),
  `place_absent_on_a_free_path_writes_the_copy`.
- `src/bin/viola-fake-agent.rs` — a failed hook spawn's receipt carries `spawn_os_error` (the raw OS code).
- `crates/viola-e2e/tests/mcp_tools.rs` — Path 7 boots the default `overseer` + `builder` pair unstamped; the race
  comment removed.
- `crates/viola-state/src/strict.rs` — the plain-home case reads an inherited, untrusted write grant by its right
  (`inherited_untrusted_writes`), whichever SID; its control: a home `instance_tree()` makes reads none.
- `tests/path1_run_start.rs` — unchanged (its temporary timing record removed; HEAD's bytes).
- New, in `viola-0.1.0/chunks/2026-10-27-pinned-copy-race-and-reproducible-builds/evidence/`: `preconditions.md`,
  `repro-unstamped.py`, `race-before.md`, `race-after.md`, `red-green.md`, `repro-builds.sh`,
  `reproducible-builds.md`, `path1-phases.md`, and the operator pass's record.

## The readings the criteria rest on
- **The product race, two-sided** (`evidence/race-before.md`, `evidence/race-after.md`): the unstamped default pair
  failed 4 of 24 boots at the base, every failure's SessionStart hook `ran:false, spawn_os_error:26` (`ETXTBSY`) —
  the CARRY's hypothesis now read on the product run; 0 of 24 after; the single-instance control 0 of 24 both sides.
- **The forced window** (`evidence/red-green.md`): `place_private_new_placed_copy_execs_at_once` green; with the
  guard removed inside `place_private_new_with` (the handle held across the placement) red, panic line
  `the placed copy spawned: Os { code: 26, kind: ExecutableFileBusy, message: "Text file busy" }`; restored, green.
- **Measurement B** (`evidence/reproducible-builds.md`): six cold release builds of one tree gave two files,
  `57bce92d…` (8 179 264 B) and `ff63f38e…` (8 179 272 B); where they met, 34 sections, 3 unequal —
  `.note.gnu.build-id`, `.symtab`, `.strtab` — every loaded section equal; 6 strings lines, three
  `anon.<hash>.<n>.llvm.<number>` local symbols. Cause **not explained**; acting on it is the founder's.
- **Red 1** (`evidence/path1-phases.md`): Path 1 ≈ 6.07 s on the dev host, ≈ 4.88 s (80 %) the `stamped_home`
  verify, 0.93 s the four product starts, 0.16 s (2.7 %) the test's own whole-file I/O. **Stop rule S2 fired:** no
  step is cut, no bound moved; the readings are for the operator.

## Deviations from the plan, with their justifications
1. **Step 10 under stop rule S2** — the plan's own branch: the dominant cost is the `stamped_home` verify and the
   product starts (Path 1's asserted contract); `tests/path1_run_start.rs` is left as HEAD has it and no "after"
   readings were taken (nothing changed to read after). The red's owner question goes back to the operator.
2. **Step 10's readings in two sets of five** — the first set's loop kept the wrong JUnit file (an old archive), so
   only its fifth JUnit time stands; a second set of five was fired with each JUnit kept. Both sets are in the record.
3. **Step 9 fired three times** — the first firing's comparison was defective (the `true` control resolved to the
   shell builtin; `objcopy -O binary --only-section` hashes non-loaded sections as empty); the second stopped at
   `.symtab`, which `objcopy --dump-section` will not dump; the script now reads every section at its `readelf -S`
   offset and size (cross-checked against `objcopy` on one section). All three firings' file pairs are recorded.
4. **Step 9's second control reads 35 of 38 unequal, not "all"** — `.interp`, `.note.ABI-tag` and `.fini` hash equal
   between `viola` and `/usr/bin/true`; recorded as measured.
5. **Entry 8's filter** selects `tests::place_private_new_` and `tests::pin_` only, so step 6's `place_absent_*`
   cases ran in entry 7 (`run --unit`, green) and not in entry 8, whose note names steps 5–6. Not edited (plan.md is
   read-only here).
6. **`replace_private_shared`'s doc comment** in `fs.rs` (a listed file) no longer names the pinned copy among its
   users; its body is unchanged.
- The scope record: none needed — `gate.py scope`: `scope: clean — changed 5 · listed 5 · recorded 0`.

## Guard entry 14's pass line (from its log, whole)
```
guard: allowed 5 — crates/viola-e2e/tests/mcp_tools.rs, crates/viola-state/src/fs.rs, crates/viola-state/src/pin.rs, crates/viola-state/src/strict.rs, src/bin/viola-fake-agent.rs · companions excluded 0 · changed 5 · excluded 0
guard: clean
```
Every allowed file is one of the plan's Files to modify; none excluded.

## Gate readings worth naming
- Entry 22 `pre-push`: `ok:true`, stage `linux-tests`, coverage `3227 passed, 0 failed`, breaches none (P5 baseline
  3220: +4 Unix placement cases, +3 `place_absent` rows).
- Entry 10: `mcp_tools_path7_unverified_cli_answer` PASS 0.663 s on the default pair; `path1_run_start_one_home_in_order`
  PASS 6.218 s.
- Entry 21 `run --perf`: `perf passed 7, failed 0`; hyperfine means 1.9–2.3 ms per hook row.
- Matrix: no capability is claimed by this chunk (`matrix.py show`: claimed 0), so no ref was written.

## Surfaced
- Nothing spec↔reality. For the operator: Red 1 under S2 (its owner), and measurement B's two-file reading (the
  founder's to act on).

## Process hygiene
| process | started by | final state |
|---|---|---|
| 96 harness sessions (`viola-harness supervise`, 2 or 1 `viola run`, their fake agents) of steps 3 and 8 | this run | terminated — every successful session by `cleanup` (`processes_gone:true`); the 4 failed boots by boot's own abort; the `ps` listing read whole after each step held none under `projects/viola/target` |
| the smoke session `p-pin-smoke` | this run (entry 16) | terminated — entry 20 `processes_gone:true`, `endpoint_gone:true` |
| `run --perf`'s `perf-<pid>` session | a gate entry (21) | terminated — its own cleanup inside the entry |
| test wrappers of entries 7–10, 22 | gate entries | terminated — each test's `Drop` guard; the census holds none |
| final census (`gate.py census --comm viola --comm viola-harness --comm viola-fake-agen --comm claude --args projects/viola/target`) | — | 16 running, none this repository's build: the `viola-lab` prototype wrappers and their `claude` children, the overseer sessions, this session's own `~/.local/viola-product` / `~/.viola` bridge and `viola wait` pollers, the desktop `claude` — none started by this run |

Residue (not processes): the 4 failed boots' homes under `target/e2e-home/viola-session-*` and 96
`target/agent-run/race-*` session dirs, all under `target/` (ignored).

## Decisions & corrections
- The operator's directive for this run: «Every test or control that spawns viola run or a wrapper gets a kill
  deadline … Journal each long run in the run dir before you wait on it, and read process listings whole» — applied:
  every long run is in `journal.md` before its wait, with its bounds.
- Guard refusals: the stamp-ahead hook refused two estimated clock stamps (re-issued from `date -u`); the Bash guard
  refused one `cd` into the scratchpad (re-issued as an Edit).
- The permission classifier refused measurement B's third launch with no verdict; per the standing learning
  (CLAUDE.md, «a transient outage»), read-only work filled the wait and the same call then ran.
- Recurrence of a standing learning: verification-harness.md 2026-10-09 (a file hash is no proof two builds load the
  same code) — read again, the loaded sections equal while the files differ.

## Next
The operator pass: entry 23 (hygiene), entry 22 re-fired before the push, the pre-CI commit, entry 24 (the push),
entry 25 (the CI read) — then `/andromeda-wrap-session`.
