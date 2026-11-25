# Outcome — 2026-11-25-placing-without-moving-the-view

**Run:** 2026-11-25T06-35-09Z-implement · **Outcome:** green (1 iteration) · Next: the operator pass WITHOUT R7 (entries 14–20 in this window, directive `inputs#I6`), then the wrap. v2-07 stays gated on R7 (entries 21–22, his, this evening): no ref written.

## Files changed
- listed (research): `src/run/window.rs` · `src/cmd/verify.rs` · `schemas/diag-line.v1.json` · `tests/cli_open.rs` · `tests/cli_verify.rs` · `tests/cli_version_gate.rs` · `tests/cross_exit_causes.rs` · new `scripts/r7-place.sh`
- recorded (`doc:2026-11-25-placing-without-moving-the-view/scope-record` rev 2): companion `tests/contract_diag_schema.rs` (serves `schemas/diag-line.v1.json`) · in-intent `tests/support/verify.rs` (serves step 13) · in-intent `tests/support/home.rs` (serves step 14) · companion `tests/cli_verify/stamps.rs` (serves `src/cmd/verify.rs`) · companion `tests/cli_verify/runs.rs` (serves `src/cmd/verify.rs`) — all `self`
- `gate.py scope`: `scope: clean — changed 13 · listed 8 · recorded 5 (companion 3 · mechanical 0 · in-intent 2 · widening 0) · absorbed 0 · excluded 234`

## Deviations, with their justification
1. **Four existing tests borrowed the trust form to fail verify rows** (research named none of them). `tests/cli_verify/stamps.rs` `verify_a_set_without_stop_fails_its_rows_and_still_stamps` and `verify_with_an_instance_logs_its_start_and_exit`, and `tests/cli_version_gate.rs`'s `stamp` helper (its `failing_row` case), moved the fake agent's `--trusted-root` off verify's cwd so Runs B–E met the trust form and were killed — the very path step 9 turns into an `untrusted-folder` refusal with no stamp. Per testing.md 2026-11-15 (a borrowed use is re-pointed, never deleted) each now takes its failing row from a spine set with no `SessionEnd` payload in the trusted folder: fast (6.4 s), fails `spine-hooks` alone, `stamped 2.1.0  28 pass  1 fail` (the oracle copied from the first run's output). The first case is renamed `verify_a_set_without_session_end_fails_its_rows_and_still_stamps`. `tests/cli_verify/runs.rs` `verify_leaves_neither_run_dir_behind::case_2_failing` still holds unchanged as a refusal (exit 1, its dirs gone); only its doc comment is corrected.
2. **`tests/contract_diag_schema.rs` gains one row** (`untrusted_folder`) so the acceptance «is in the line schema's `process-exit` detail enum (entry 6: `contract_diag_schema`)» has a case that reads it (testing.md 2026-10-06).
3. **Two test-support helpers**: `verify_in` (`tests/support/verify.rs`, a stamping verify typed in a given folder with no `--trusted-root`) and `Wrapper::start_as_user_in` (`tests/support/home.rs`, `start_as_user` typed in a given folder) — the runners had no cwd parameter. The step-13 case also runs under `VIOLA_NAME` and reads the `process-exit` `detail` `untrusted-folder` (step 9's last bullet), beyond the plan's listed reads.
4. **R7 script preconditions**: beside the plan's five subjects (`scratch`, `foot`, `hyprctl`, `locked`, `stand-in`) the script also refuses `precondition=jq` (it parses `hyprctl -j` and the snapshot with jq, as `scripts/r3-focus.sh` does) and, under `--rehearse` only, `precondition=fake-agent` (as r3 does). Both print under the plan's `r7: red precondition=<word>` form; neither is a product word. Named for the operator: a delegate may rule them out.
5. **R7 target rule reading**: «the lowest of 1–9 that is not A and that `hyprctl -j workspaces` lists with no window» is read as: no listed workspace of that id with `windows > 0` (an unlisted id holds no window either).
6. **The stand-in's `--version` line** is `r7 stand-in` (no version shape, so the CLI reads unverified).

## Gates
Entries 1–8 green (call 1), entries 9–12 after a QUIET host read (call 2): 9 recorded, 10–12 green. 13 `not run — defer` (gated on R7). 14–22 leg operator. Counts: entry 3 1413 run/passed (baseline 1403); entry 4 18 (baseline 8); entry 5 3 (baseline 0); entry 6 192 (baseline 189). Entry 8: `r7: green placement=placed active=unchanged` then `r7: rehearsed — not a live reading`, by `open`'s own exit-0 document (bash -x reading: `open_exit=0`, `placement` from `.ok.placement`), not the snapshot fallback. Record: `.andromeda/cache/chunks/2026-11-25-placing-without-moving-the-view/evidence/block.txt`.

Controls (`gate.py control`, `--timeout 900`, `evidence/control-1.txt`): `control: 3 holds · 0 timeout · 0 does not hold · 0 unreproduced · clone removed` — win-restore, win-stayed, vfy-confound each in place green, removed red.

No `gate.py guard` entry in the plan.

## Unproven until R7
On the real compositor: that the view stands where it was after a placing open; where keyboard focus lands after the restore; what a failed restore looks like live. The fakes prove the request sequence only (the recorded monitors reply reads the same active workspace before and after, so no restore is dispatched over the fakes).

## Process hygiene
| process | started by | final state |
|---|---|---|
| rehearsal wrappers, stand-in `sleep 900`, fake `foot`/`hyprctl` (entry 8, two by-hand rehearsals) | this run | terminated — census 2026-11-25 (comm viola, foot, viola-fake-agent, claude · args r7place, sleep 900, projects/viola/target): none of this run's among 15 running |
| test wrappers / fake agents of entries 3–6, 10–12 and the controls | this run (gates) | terminated — same census |
| the 15 census rows (overseer/escher/worker sessions, the product's `viola mcp` and `wait`) | not this run | left running — the host's own |

Left on disk outside the tree: `$TMPDIR/tmp.vuEFDme0uq/` (the by-hand rehearsal root in entry 8's form); the session scratchpad's `r7-hand-1/` (the first by-hand rehearsal, red on sun_path). Not removed (no recursive delete).

## Decisions & corrections
- The overseer's implement directive (`inputs#I6`): no real compositor, no real claude, R7's script only under `--rehearse`; the operator pass without R7; end the turn at the pre-CI push.
- Sweep hazard: a verify test that fails rows by `"--trusted-root", elsewhere` (the fake's trust root moved off the cwd) now reads the untrusted-folder refusal; find such borrowers by that literal, not by `Stop` skips.
- A by-hand rehearsal rooted under the session scratchpad overflowed `sun_path` through its runtime dir (detail chain «local socket name length exceeds capacity of sun_path»); a short `mktemp -d` root, as entry 8 uses, reads green.
- Bash guard refused one `cd` into the skill's references dir (re-issued with absolute paths).
