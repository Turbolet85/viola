# Outcome — 2026-11-20-a-new-cli-version-stamped-automatically

**Outcome: green + smoke + live round green.** Next → the OPERATOR PASS (entries 24 pre-push, 25 hygiene, 26 the
push, 27 the CI read), then the wrap.

## Files changed
- New: `src/run/auto_verify.rs`, `src/human/auto_verify.rs`.
- Modified (research's list): `crates/viola-agent-claude/src/ledger.rs`, `crates/viola-agent-claude/src/ledger/test_cases_stamps.rs`,
  `crates/viola-state/src/stamps.rs`, `src/obs.rs`, `schemas/config.v1.json`, `schemas/diag-line.v1.json`,
  `src/run/version_gate.rs`, `src/run/mod.rs`, `src/cmd/run.rs`, `src/human.rs`, `src/bin/viola-fake-agent.rs`,
  `src/bin/viola-fake-agent/test_cases_opts.rs`, `tests/support/fake.rs`, `tests/cli_version_gate.rs`,
  `tests/cli_controls_not_disableable.rs`, `tests/contract_bounded_readers.rs`, `crates/viola-e2e/src/rig/mod.rs`,
  `crates/viola-e2e/src/rig/isolate.rs`, `crates/viola-e2e/src/bin/viola-live-rig.rs`, `crates/viola-e2e/tests/rig_rehearsal.rs`.
- Out of the lists (scope record): `tests/support/home.rs` · in-intent · serves `tests/cli_version_gate.rs` · self;
  `tests/support/outer_pty.rs` · in-intent · serves `tests/cli_version_gate.rs` · self. `gate.py scope`: `scope: clean —
  changed 24 · listed 22 · recorded 2 (companion 0 · mechanical 0 · in-intent 2 · widening 0) · absorbed 0 · excluded 227`.

## Deviations, with justifications
1. **Step 6's re-read under the lock narrowed:** the start takes another start's verdict under the lock only when the
   entry reads `Attempted` now AND the gate had not read it `Attempted`. As written, that re-read alone stopped start 2's
   verify, so the plan's `attempted-arm` control (removing `decide`'s `EarlierRed` arm alone) could not read red; the
   narrowed form is what step 6 says it is for («another start just finished»). The control then read `holds`.
2. **The stop handler is installed before the `auto-verify` start line,** not after the spawn: a TERM read by a case at
   that line otherwise ended the start by the OS default with no `process-exit`, measured red in the integration layer.
3. **`AV` takes the verify arm's cwd and config seeds** (`verify_seed`, `marker_seed`), beyond step 19's named `trusted()`:
   the start's verify runs Runs B–E under that cwd, as `V`'s do; without the markers its runs meet the CLI's first-run
   screens. Read from `V` (mod.rs `verify_seed` / `marker_seed`).
4. **The rig's `end_session` takes a closed `Stop` (`CtrlC` · `NoKey` · `Term`)** in place of its `no_key` flag, and the
   `Session` gains `stop_term`: the plan's «the stop TERM» needed a third stop.
5. **Step 17, «one parse-rejected{config-json, malformed} line, none for v 2»:** a `v` 2 document already gets the
   diagnostics-level read's one `malformed` line (the same as `dialog_hold_s`'s `v_two` case); the rstest asserts one line
   for every case, the key reader adding none for `v` 2, and its doc comment says so.
6. **Step 22's Run F trust config:** the start's verify passes the fake no argument, and the rig's private config path
   holds the rig's run id; the rehearsal's `fake-args` names `<case>/cfg/.claude.json` and the case links `<case>/cfg` to
   the rig's scratch config once the rig has made it (Runs B–E are trusted by `--trusted-root <case scratch base>`).
7. **Case 16 (`…_verify_ended_by_its_own_stop_…`) and case 15 wait for verify to hold a child of its own** before the
   TERM (verify's stop is its `measure`'s first act); the start's pid is the outer PTY's own child (`OuterPty::child_pid`),
   not a sysinfo lookup, which read no pid.
8. **Enum renamed:** `Unstamped` → `NoStamp` in `src/human/auto_verify.rs` (clippy `enum_variant_names`); the L5 words
   are unchanged.
9. **No capability ref written:** the `coverage` view holds no row for this chunk (the plan claims none).

## Gate guard lines
No `gate.py guard` entry in this plan.

## Surfaced / stuck
None.

## Process hygiene
| process | started by | final state |
|---|---|---|
| the gate entries' cargo / nextest / test children (1–12, 16–23) | this run | terminated (the tool's entries exited; no survivor line) |
| two controls' clone firings | this run | terminated · `clone removed` |
| harness session `autostamp-smoke` (supervise, wrappers, fake agents) | the operator entries 13–15, this run | terminated (`cleanup`: `processes_gone: true`, `endpoint_gone: true`) |
| AV: `viola-live-rig`, its `viola run`, the start's `viola verify` and its `claude` children, the session `claude` | a leg (entry 21) | terminated (`census: 0`, the rig's own and entry 22's `census: 0`) |
| the 14 census rows of `gate.py census` (host Claude sessions, the overseer pair, the prototype-wrapped builders, the product's `viola wait`) | NOT DERIVED — not this run's | left running: the host's own sessions — their owners |

## Decisions & corrections
- The host read before the round (16:23:19Z) was CONTENDED naming the `escher` build; the run stopped without a start,
  per the implement directive §The host. The overseer's word (relayed by the operator, `inputs#I4`): the neighbour stopped
  at 16:48:26Z and is held; read the host since then. That read (16:48:26Z–16:48:48Z) was QUIET; the round fired.
- `hostwatch.py read --to now` is refused (`not a UTC stamp: 'now'`): the window's end is the clock's `date -u` stamp.
- The Bash guard refused a `cat` heredoc append to a source file (re-issued through Edit), a `cd` into the gate's log
  dir (re-issued with absolute paths), and a gate tool's output piped into `grep` (re-issued bare).
- A gate run is fired by `--only` on the non-live entries before the round: entry 23 (`live-stamp: unmoved`) is no leg
  but reads the sha file round entry 19 writes, so a bare block run before the round reads it red by construction.
