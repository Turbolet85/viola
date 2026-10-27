# Implement outcome — 2026-10-26-home-and-code-bearing-file-integrity

**Outcome: green + smoke, and CI green on all three OSes.** The operator pass ran in this session on the
operator's own request: pre-CI commit `af6c67ef`, fix commits `f65a4ae8` and `e3468500`, each pushed through entry 25
after pre-push and hygiene; the last CI read `e346850044ac verdict: green · checks 15/15 · wall 715 s · runs
ci#40403389629 completed/success`. Next → `/andromeda-wrap-session`.

Run dir `.andromeda/runs/2026-10-26T19-02-07-implement/` · journal `journal.md` beside this file · gate trail
`gate-2026-10-26-home-and-code-bearing-file-integrity.json` (three full firings, two targeted).

## Gates

- Firing 1 (19:24Z): `entries 26 · green 16 · red 6 (4,7,10,11,12,23) · recorded 1 · timeout 0 · not-run 3`.
- Firing 2 (20:31Z): `entries 26 · green 21 · red 1 (4) · recorded 1 · timeout 0 · not-run 3`; `--only 2,4` →
  `green 2 · red 0`.
- Firing 3 (20:36Z): `entries 26 · green 22 · red 0 · recorded 1 · timeout 0 · not-run 3` — entry 1 (`df -h /tmp`)
  recorded `tmpfs 32G 7.4G 24G 24% /tmp`; entries 24–26 `not run — leg operator`.
- Entry 22 (`run --perf`, hook rows with the instance check in the timed path), hyperfine mean / max over 30 runs:
  session-start 2.0 / 2.5 ms · user-prompt-submit 2.0 / 2.4 · stop 2.0 / 2.4 · session-end 2.0 / 2.4 ·
  pre-tool-use 2.3 / 3.3 ms; `perf passed 7, failed 0` (P5 baseline max 2.6 / 2.5 / 2.6 / 2.6 / 2.9 ms).
- Entry 23 (`pre-push`): `"ok":true` at stage `linux-tests`, coverage `passed 3220, failed 0`.
- Whole workspace suite outside the tool, same tree: `3220 tests run: 3220 passed, 0 skipped`.
- No deferral: Rust delta in every gate's language; no `defer` key in the block.

### Entry 15 — the preservation guard's pass line (read from `15.3.log`, whole)

```
guard: allowed 30 — crates/viola-mcp/src/tools.rs, crates/viola-mcp/tests/handlers.rs, crates/viola-state/src/liveness.rs, crates/viola-state/src/pin.rs, crates/viola-state/src/stamps.rs, crates/viola-state/src/strict.rs, schemas/diag-line.v1.json, src/cmd/answer.rs, src/cmd/client.rs, src/cmd/hook.rs, src/cmd/hook/statusline.rs, src/cmd/last.rs, src/cmd/link.rs, src/cmd/pause.rs, src/cmd/release.rs, src/cmd/revive.rs, src/cmd/run.rs, src/cmd/send.rs, src/cmd/verify.rs, src/cmd/wait.rs, src/obs.rs, tests/chaos_snapshot_replay.rs, tests/cli_controls_not_disableable.rs, tests/cli_verify.rs, tests/conpty_sideload.rs, tests/hook_fail_open.rs, tests/hook_statusline.rs, tests/run_cli.rs, tests/security_negatives_strict_modes.rs, tests/support/home.rs · companions excluded 5 — tests/cli_machine_contract.rs (serves step 1), tests/cli_revive.rs (serves step 5), tests/cli_wait_last.rs (serves step 3), tests/contract_diag_schema.rs (serves step 1), tests/tui_env_strip.rs (serves step 1) · changed 35 · excluded 0
guard: clean
```

Each excluded companion: a standing test whose home or oracle the change reaches — `cli_machine_contract`,
`contract_diag_schema` and `tui_env_strip` made their home with `create_dir_all` (0755), which the read rule now
refuses (step 1), so each makes it through `viola_state::fs::create_private_dir`; `cli_wait_last`'s two catch-site
cases used a regular-file home or a 0755 home, now refused by the frame verbs' check (step 3), so each holds a
viola-made home with a regular file where `diagnostics/` goes; `cli_revive`'s two `cwd-missing` cases cut the
snapshot while the stopped wrapper's beat was fresh, which the replayed collision check now reads as `already-live`
(step 5), so each ages the beat past `STALE_AFTER` first.

### Remove-the-guard controls (testing.md 2026-09-25)

All sixteen on this host read red with the guard removed and green with it on, every file restored byte-equal:
`chunks/2026-10-26-home-and-code-bearing-file-integrity/evidence/remove-the-guard.md`. The Windows-only
`conpty/` leg's control is not measurable here (CI's `windows-2025` leg runs its witnesses).

## The operator pass and CI (`chunks/{marker}/evidence/operator-pass.md`)

| commit | CI | reading |
|---|---|---|
| `af6c67ef` pre-CI | ci#40398601594 `red`, 14/15 | `test (windows-2025)` 2 of 3175: `verify_under_a_regular_file_refuses_the_home` (a path under a regular file stats `NotFound` on Windows); the protected-DACL read-back (`Get-Acl` printed nothing) |
| `f65a4ae8` fix | ci#40400887151 `red`, 14/15 | `test (windows-2025)` 2 of 3175: the DACL read back `D:PAI(A;OICI;FA;;;LA)(A;OICI;FA;;;SY)` against a SID oracle (`LA` = the runner's local Administrator); `run_rust_log_changes_nothing` (a `strict-modes-failed` fallback leaked from `run_refuses_a_widened_home_tree_before_its_log`'s seeded home — in both runs) |
| `e3468500` fix | ci#40403389629 `green`, 15/15 | Windows `3175 passed`; kept homes: 264 sideloads loaded, 3 `hash-mismatch`, 1 `strict-modes-failed` (the companion test's own) |

Every red was this chunk's own test code; the product checks read as specified on every leg.

## Smoke (P3)

Fired — the boot path changed (`run`, `hook`, every frame verb). By hand: `agent-run.sh cleanup` → `boot --session
p3-hci-smoke --instance builder` (`ok:true`) → `status` (`state:"ready"`, builder `live`, `cli_verified:true`) →
`viola --home <the session home> last builder --json` through the new instance check
(`{"v":1,"ok":{"last_assistant_message":null,"ts":null}}`, exit 0) → `cleanup` (`processes_gone:true,
endpoint_gone:true, home_removed:true`). The plan's smoke entries 16–21 also read green in firing 3.

## Matrix

`v1-45`, `v1-17`, `v1-41` → `implemented`, refs in `viola-0.1.0/verification-matrix.json` (trail `matrix-*.json`
in this run dir). Verified at the wrap, after CI's three legs.

## Files changed (35)

Product: `crates/viola-state/src/{strict,stamps,liveness,pin}.rs` · `crates/viola-mcp/src/tools.rs` ·
`src/cmd/{client,link,send,wait,last,answer,pause,release,hook,run,revive,verify}.rs` · `src/cmd/hook/statusline.rs` ·
`src/obs.rs` · `schemas/diag-line.v1.json`.
Tests: new `tests/security_negatives_strict_modes.rs`, `tests/chaos_snapshot_replay.rs`; modified
`crates/viola-mcp/tests/handlers.rs`, `tests/{hook_fail_open,hook_statusline,run_cli,cli_verify,conpty_sideload,
cli_controls_not_disableable}.rs`, `tests/support/home.rs`; companions (scope record) `tests/{cli_revive,
cli_wait_last,contract_diag_schema,cli_machine_contract,tui_env_strip}.rs`.
Records: `chunks/{marker}/scope-record.md` (5 companion lines), `chunks/{marker}/evidence/remove-the-guard.md`,
`viola-0.1.0/verification-matrix.json`.

## Deviations from the plan, with their justification

1. **Step 2:** `update_stamps` runs `check_stamps` before `create_private_dir` as well as inside its lock. Under the
   lock alone the check comes after the create, which narrows an existing widened `ledger/` to 0700 first (the
   CARRY 14 trap); its unit case reads a 0777 `ledger/` and a 0666 stamps file refused and left as found.
   The refusal is a new `StampsError` (`StrictModes` · `State`, fixed messages); `verify` maps `StrictModes` to the
   home pair.
2. **Step 1:** `Refused` gained `Readable` and `Symlink` (fixed messages), so the read rule and the symlink check
   do not borrow `Writable`. `check_home_tree` returns `TreeRefused::{HomeLevel, InstanceLevel}(Refused)`;
   `strict::check_present` (the write rule on one path that exists) serves step 10; `DIAGNOSTICS` is a constant.
3. **Step 8:** `pin_exe` keeps its signature (verify still calls it) and delegates to a new `pin_bytes(home,
   bytes)`, which `run`'s start calls with the bytes `checked_exe` read; `pin::pin_key` names the key once.
4. **Step 5:** `collision_check` takes the caller's one read of the snapshot (`Option<&Recovered>`): `run`'s start
   reads once before it, and `revive`'s preflight reads once and hands that read to both its collision check and its
   recorded-directory reading. A replay in each would have written two `state-recovered` lines in one revive (a
   red, `cli_revive`'s «one replay, one line»). Consequence: the `state.snapshot_recover` span now sits under
   `run.start` before `run.collision_check`, and under `hook.handle` on the hook path; the two span-order oracles
   (`start_opens_the_scenario_one_spans_under_run_start`, `hook_refuses_an_unverified_server_inside_hook_handle`)
   now list it.
5. **Step 4:** a `VIOLA_DIR` naming an instance directory that does not exist canonicalises to no session, so the
   hook writes nothing (statusline already did); `hook_fail_open`'s cases that expect lines run in an instance
   directory viola made (`Env::Made`), and a `no_instance_dir` row pins the silence. The widened-instance rows are
   one function over both matrices' events (stop, session-end, pre-tool-use, permission-request) with a live
   wrapper, rather than one row added to each `rstest` matrix.
6. **Step 11:** the plain home's canonical SDDL could not be recorded from a passing CI run (no artifact or
   passing-test output carries it). The cfg(windows) case
   `strict_instance_refuses_a_plain_home_made_before_viola_and_names_the_entry` asserts the refusal `Writable` and
   that the refused entry is an inherited `S-1-5-32-545` write ACE; a failure prints every ACE. The answer to
   CARRY 13 it records: a home a test made first is one the product must refuse.
7. **Step 13:** (a) the `send` case's unwidened control runs `last` (an unscripted fake agent's `send` waits out the
   readiness gate past `WITHIN`); (b) the MCP tools are driven over stdio with an `initialize` handshake, probed first
   on the built binary; (d) the read-granted pre-existing home refusing `viola run` runs on every OS (Unix 0750,
   Windows a Users read grant); the protected-DACL read-back is Windows-only, through PowerShell `Get-Acl`.
8. **Step 14:** `cli_verify`'s catch-site case split in two: a home under a regular file is now the home pair
   (`verify_under_a_regular_file_refuses_the_home`), and the catch-site line is forced by a regular file where
   `ledger/` goes in a home viola made.
9. **Steps 12–16:** `tests/support/home.rs` gained `widen` · `grant_read` · `unwiden` · `still_widened` (Unix
   modes; Windows `icacls` with its exit asserted). The G4 and G10 witnesses (`run_refuses_a_widened_home_tree_
   before_its_log`, `a_start_over_a_replayed_snapshot_and_a_fresh_beat_refuses_already_live`) spawn through
   `support::cli::Running`, bounded by `WITHIN`, after the remove-the-guard hang (below).

10. **After CI (the two fix commits):** `verify_under_a_regular_file_refuses_the_home` carries a per-OS oracle — the
    home pair on Unix, the catch-site line on Windows, where a path under a regular file stats `NotFound`, so the home's
    check skips it as absent (testing.md 2026-09-24); the Windows `--home` DACL read-back goes through `icacls /save`
    and takes `LA` for a user SID ending `-500`; `run_refuses_a_widened_home_tree_before_its_log` starts its home
    UNSEEDED — a seeded home's ConPTY companions are hard links to one shared seed file, and `widen`'s inheritable
    grant on the home or `bin/` reached every seeded home through them (one foreign start fell back
    `strict-modes-failed` inside that case's window in both red runs; one, the companion test's own, in the green run).

## Surfaced — for the wrap's reconcile (nothing authored here)

1. **v1-17, the by-construction clause:** `architecture.md:323` (§Occupied Resources) names both diagnostics roots and
   the four role sinks, but the instance detail file only by its folder; `detail-<process>.ndjson` is not named in the
   architecture. The ref says so; the plan's expected §Occupied Resources amendment is where it lands.
2. **Behaviour the masters do not yet state:** an unstattable home (one under a regular file) is now a strict-modes
   refusal — `verify` exit 1 with the home pair, the frame verbs exit 21 with the instance pair — where the `cli`
   catch-site line stood; a hook whose `VIOLA_DIR` names no existing directory writes nothing; the
   `state.snapshot_recover` span inside `run.start` and `hook.handle` (obs-plan §4 Scenario 1's span list); the
   `sideload_fallback` word `strict-modes-failed` (schema enum updated; D-36 class, its Decisions Log entry).
3. **A latent inconsistency outside this chunk's files:** `viola_state::board::scan` checks the home with
   `check_path` (the write rule) and each instance with `check_instance` (now the read rule on the home). A 0750 home
   with no instance passes the board; with one it refuses naming that instance (`ScanRefused::Instance`), not the home.
   `board.rs` is not in research's lists; not changed.
4. **Windows-only, unmeasured here:** every `cfg(windows)` case compiles under the Windows-target clippy (entry 4)
   but runs only on CI's `windows-2025` leg.

The plan's Expected amendments (security-plan, architecture, obs-plan, test-plan, design-system / layout-templates,
a11y-plan) stand as written; carried out of this chunk by the founder's word: the new entry «Pinned copy race and
reproducible builds» (CARRYs 11 and 16).

## Process hygiene

Census at P4 (`gate.py census --comm viola --comm viola-fake-agent --comm viola-harness --args projects/viola/target
--args viola-e2e-home-1000`): 7 running, none this run's — five `viola run` / `wait` processes from
`~/dev/projects/additional/viola-lab/prototype/target/debug/viola`, the bridge's `~/.viola/bin/…/viola mcp` and a
`~/.local/viola-product/bin/viola wait`.

| process | started by | final state |
|---|---|---|
| gate firings' harness sessions `p-hci-smoke` (wrapper + fake agent) | a gate entry (17) | terminated — entry 21 `processes_gone:true, endpoint_gone:true` |
| P3 smoke session `p3-hci-smoke` | this run, by hand | terminated — `cleanup` `processes_gone:true, endpoint_gone:true` |
| `viola mcp` probe (scratchpad `in.ndjson`) | this run | terminated — exited at stdin EOF |
| controls: G4's four hung test processes `run_cli-b93b0de638fe67df` (3046386, 3046387, 3046388, 3046390) | this run (controls.py) | terminated — SIGTERM on the overseer's word (20:19Z, 20:27Z) |
| controls: eight `viola run` wrappers and fake agents those tests started (3046454, 3046457, 3046491, 3046492, 3046459, 3046493, 3046428, 3046468) | this run (controls.py) | terminated — SIGTERM after `/proc` exe + home check, 20:27:44Z |
| controls: G10's wrapper 3328731 and fake agent 3328741 | this run (controls.py) | terminated — SIGTERM after the same check, 20:28:25Z |
| controls: G10's test process 3327778 | this run (controls.py) | terminated — exited by itself once its child was gone |
| controls.py (first run) | this run | terminated — SIGINT 20:28:03Z on the overseer's word; second run exited 0 |

## Decisions & corrections

- The overseer's technical answer, 20:19Z: kill only the two hung test pids 3046386, 3046387; stop the script before
  G10; bound G4 and G10's witnesses; re-fire G4 (journal). The overseer's technical answer, 20:27Z: steps 1–5, killing
  only processes whose `/proc` exe and home path prove them this run's, never viola-builder's wrapper or anything under
  `~/.viola`; `run.rs` restored and the census empty before step 4 (journal).
- Correction (mine): the 20:03Z status named two hung pids from a listing cut at 30 rows (`| head -30`); two more were
  hidden. A recurrence of the standing rule «never read a result summary through truncation»
  (code-writing-discipline.md, Shell discipline).
- Correction (mine): the script stopped inside G10, not before it — G5–G9 finished during steps 1–2.
- A `/proc/<pid>/exe` reading `… (deleted)` after a rebuild was matched with that suffix removed, printed per row.
- Hook refusals: `stamp-ahead.py` refused a guessed journal time (19:45Z written, clock 19:30Z) — corrected from
  `date -u`; `bash-guard.sh` refused two calls carrying a `cat` heredoc with a file target and one `cd` that would
  move the session cwd — each re-issued in the allowed form.
- Measured on `windows-2025`: an inheritable `icacls` grant on a seeded home's `bin/` (or the home) propagates through
  `seed_conpty`'s hard links into the shared seed companion, widening every seeded home until it is removed; a test
  that widens a home tree above `bin/<key>/conpty/` starts that home unseeded.
- Measured on `windows-2025`: canonical SDDL spells the runner's user (local Administrator, RID 500) `LA`.
- Remove-the-guard rule (testing.md 2026-09-25) applied: a control whose neutralised guard lets a product process
  start needs a bounded witness (measured: a 27-minute hang under nextest's default profile, which has no kill line).
