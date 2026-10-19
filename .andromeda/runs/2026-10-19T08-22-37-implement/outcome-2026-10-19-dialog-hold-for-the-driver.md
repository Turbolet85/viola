# Implement outcome — 2026-10-19-dialog-hold-for-the-driver

**Outcome: green + smoke + live round green.** The gate block reads `entries 23 · green 20 · red 0 · recorded 0 ·
timeout 0 · not-run 3` (the three `operator` legs, 21–23), second firing; trail
`.andromeda/runs/2026-10-19T08-22-37-implement/gate-2026-10-19-dialog-hold-for-the-driver.json`. Clippy (entry 2)
closes the PREREQ deferred since 2026-10-18-plugin-precedence-live-readings. The live round
(`evidence/live-round.md`) ran inside the founder's cap (`inputs#I2` Q3): 1 `viola verify` (`stamped 2.1.287  24 pass
0 fail`) and 1 wrapped start. The one dialog was answered by `viola answer` at 671.3 s. The wrapper logged
`dialog-answered{deadline_hit:false, duration_ms:671301}`, the hook `hook-decision{decision_emitted:true,
deadline_hit:false, duration_ms:671302}`, `last` equals `beta`, `dialog-expired` 0, and `list`'s `dialog_pending` was
true at 300 s. Reds 0, none re-fired. No key was typed into any dialog.

Next → the OPERATOR PASS (plan entries 21 hygiene, 22 push, 23 `ci.py conclusion`; `.claude/rules/ci.md` order),
then `/andromeda-wrap-session`.

## Files changed (14, all on research's Files to modify) + new

- `crates/viola-core/src/lib.rs`: `DIALOG_HOLD_DEFAULT` / `DIALOG_HOLD_MIN` / `DIALOG_HOLD_MAX` (60 / 60 / 3600 s,
  PROVISIONAL) replace `DIALOG_DEADLINE`. `DialogHold` (nutype `u16`, 60–3600, `duration()`, `Default`).
  `EventKind::DialogExpired` (`dialog-expired`, `{dialog_id, dialog_kind}`): `ALL` holds 16 kinds and `WAIT_WAKE` 6.
  Pins renamed and grown as step 1–2 name them.
- `src/run/dialog.rs`: `read_hold` (the `read_thresholds` twin). The slot stores the hold, `await_answer`'s deadline
  and `deadline_ms` read it, and an expiry (`deadline_hit`) appends one `dialog-expired` line through
  `feed.appending` and emits `dialog-answered{deadline_hit:true}` with no `from`. Unit cases:
  `read_hold_takes_…`, `read_hold_falls_back_…`, `hook_dialog_holds_until_the_configured_hold_…`,
  `hook_dialog_expiry_writes_one_…`, `hook_dialog_hand_back_writes_no_dialog_expired_line`.
- `src/cmd/run.rs`: `read_hold(&home)` after `read_thresholds`, its rejections logged, `.hold_for(hold.duration())`.
- `src/cmd/hook.rs`: `dialog_deadline` = `DIALOG_HOLD_MAX + REPLY_GRACE` (3605 s). Pins at `DIALOG_HOLD_MAX + 5 s` and
  `< 3615 s`.
- `plugin/hooks/hooks.json`: the two dialog `timeout`s go from 75 to 3615. The spine 5s are unchanged.
- `crates/viola-agent-claude/src/lib.rs`: the rendered literal reads 3615.
  `hooks_json_dialog_timeouts_outlast_the_longest_hold` (renamed) asserts a margin of at least 10 s over
  `DIALOG_HOLD_MAX`.
- `src/obs.rs` (`CONFIG_KEYS: [&str; 6]`) and `schemas/config.v1.json` (`dialog_hold_s` integer 60–3600).
- `src/cmd/wait.rs` (`DIALOG_KINDS: [&str; 4]`, `dialog_form`,
  `wait_line_form_dialog_expired_takes_the_dialog_form`) and `src/human/wait.rs` (the
  `dialog-expired  builder  dialog 7  cursor 49400` pin).
- `src/run/wait.rs` (`#[case::dialog_expired]`), `crates/viola-state/tests/state_events.rs` (16 kinds, the new row),
  `crates/viola-state/src/events.rs` (the in-file kind list).
- `tests/cli_controls_not_disableable.rs`: `SETTINGS: [_; 10]` with `dialog_hold_s`, and the booted
  `setting_does_not_unbound_the_dialog_hold` over 0, 3601, -1, `"3600"` and `false`. Each case gets exactly one
  `parse-rejected{config-json, malformed}`, and the doc comment states the booted tier's limit.
- New: `evidence/live-pty.py`, `evidence/live-ledger.py` (byte copies, `cmp` 0), `evidence/live-sessions.ndjson`
  (3 rows: verify, start, census), `evidence/live-round.md`.

`gate.py scope`: `clean — changed 14 · listed 14 · recorded 0`.

## Deviations from the plan, with justifications

1. **The hold reaches the slot through `DialogSlot::hold_for(hold)`, not as an argument of `DialogSlot::new`** (step
   4). As first written, the eighth argument tripped `clippy::too_many_arguments` and needed an `#[expect]`. It also
   forced an edit to `src/run/wheel.rs`, whose test helper calls `new` and which research's lists and Graph impact
   omit. That edit was recorded as a companion in `scope-record.md`, and the preservation guard (entry 11) read red on
   it in the first block firing. The cause was removed: `new` keeps its seven arguments and starts at
   `DIALOG_HOLD_DEFAULT`, the start calls `.hold_for(...)`, `src/run/wheel.rs` was restored to HEAD (the guard
   green), the lint exemption is gone, and the now-empty scope record was deleted. The in-file caller
   (`src/cmd/run.rs`'s `methods`) and the `slot` helper take the default unchanged. The configured-hold case drives
   `hold_for`.
2. **`dialog_hold_takes_whole_seconds_from_sixty_to_thirty_six_hundred` is a literal table `#[test]`, not an
   rstest** (step 1). `crates/viola-core` has no rstest dev-dependency, and the preservation guard forbids a
   manifest edit. The case holds the same 8 values (60, 61, 3599 and 3600 accepted; 0, 59, 3601 and `u16::MAX`
   refused), in the form of its neighbour `percent_admits_zero_to_one_hundred_and_nothing_else`.
3. **The live round's census** keyed first on the bare text `target/debug/viola` and listed the overseer pair's
   prototype bridge (another tree's build of the same name). It was re-keyed on this repository's path, which read
   none running before and after. Nothing was done to the other tree's processes.

## Surfaced / stuck

Nothing surfaced, nothing stuck. No matrix capability is claimed (`matrix.py show --chunk`: 0), so no matrix write;
`v1-21` stays pooled per the plan.

- **WATCH** (`send_budget_paused…` case_4): not advanced. Only nextest ran, with no plain `cargo test`; it stands 0/3.
- **PREREQ** (clippy deferred since 2026-10-18-plugin-precedence-live-readings): closed. Entry 2 is green on this
  chunk's Rust delta.

## Process hygiene

| process | started by | final state |
|---|---|---|
| `viola-harness supervise` + booted wrapper + fake agent (session `p-dhd-smoke`) | a leg (gate entries 15, 19) | terminated: entry 19 cleanup `processes_gone` and `endpoint_gone` true |
| `viola verify` and its five interactive `claude` 2.1.287 children | this run | terminated: exit 0 at 08:38:55Z |
| `live-pty.py` → `viola run builder` → `claude` 2.1.287 | this run | terminated: stop file, 3 presses, exit 0, not killed |
| background `list` / `answer` waiters | this run | terminated: each exited 0 |

Measured: `gate.py census --args projects/viola/target/debug/viola --args viola-live-hold --args hold-live-` read
`census: none running` after the stop (trail `census-2026-10-19-dialog-hold-for-the-driver.json`). The overseer
pair's prototype bridge processes (`~/dev/projects/additional/viola-lab/prototype/`) were running before and after.
They are not this run's: left running, theirs to stop.

## Decisions & corrections

- The founder's cap for this chunk, restated by the operator in the `/implement` invocation (2026-10-19): «ONE viola
  verify plus ONE wrapped claude 2.1.287 start (his dialog 2026-10-19T08:04Z); one dialog answered by viola answer at
  >= 660 s; no key typed into any dialog; ledger each start with date -u before it; a red never re-fired … No mutation
  run. Then the operator pass with the ci.py conclusion read (leg=operator), and stop ready for the wrap.» Kept: 1 + 1,
  answer at 671.3 s, 0 keys, a `date -u` before each ledger row, 0 reds, no mutation run.
- Guard refusal: the Bash guard refused one `cd` into the skill's references dir. It was re-issued once with an
  absolute path (`guard-refused:1`).
- A standing learning recurred: host-linux.md 2026-10-07 (a process named `viola` is not this repository's by its
  name), in the census above.
- The plan template's preservation guard reads red on any recorded companion edit, a carried operator item in the
  handoff. It recurred here, and was resolved in the design (deviation 1) rather than by a record.
