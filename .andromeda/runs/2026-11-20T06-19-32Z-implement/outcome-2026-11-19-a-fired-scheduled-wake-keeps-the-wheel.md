# Outcome — 2026-11-19-a-fired-scheduled-wake-keeps-the-wheel

**Outcome:** green — offline block green (10/10 non-leg entries, last full firing), six controls hold, live round
COMPLETE (WS, HB, R3W green on claude 2.1.328, spare unspent). Next → the OPERATOR PASS (entries 35 pre-push, 36
hygiene, the operator's pre-CI commit, 37 push, 38 the CI read), then the wrap.

## Files changed
Listed (research): `src/run/wheel.rs` · `src/run/send/slot.rs` · `src/run/send/relabel.rs` ·
`crates/viola-agent-claude/src/ledger.rs` · `crates/viola-agent-claude/src/ledger/test_cases.rs` ·
`crates/viola-agent-claude/src/ledger/test_cases_checks.rs` · `crates/viola-agent-claude/src/ledger/test_cases_stamps.rs` ·
`src/cmd/verify.rs` · `src/run/version_gate.rs` · `src/bin/viola-fake-agent.rs` · `crates/viola-e2e/src/harness/run.rs` ·
`crates/viola-e2e/src/rig/mod.rs` · `crates/viola-e2e/src/rig/wake.rs` · `crates/viola-e2e/src/bin/viola-live-rig.rs` ·
`crates/viola-e2e/tests/rig_rehearsal.rs` · `tests/cli_wheel.rs` · `tests/tui_wheel.rs` · `tests/cli_verify.rs` ·
`tests/cli_verify/runs.rs` · `tests/support/verify.rs` · `tests/contract_ledger_probes.rs`.
Recorded: `tests/cli_verify/stamps.rs` · companion · serves step 7 · self.
Listed, read, unchanged (no relied-count pin found): `crates/viola-agent-claude/src/screen.rs`,
`tests/support/home.rs` (its dialogless count derives from `relied()`), `tests/contract_fixture_hygiene.rs`.

## Deviations (with justification)
- The control `mark-survives-turn-end` was fired under the id `mark-survives`: `gate.py control` refuses an id past 16
  chars (control-1.toml refused at exit 2, nothing made). Its edit and case are the plan's, unchanged.
- `tests/cli_verify/stamps.rs` (outside research's lists) moved its `stamped 2.1.0  7 pass  18 fail` to `7 pass  19
  fail`: a set without a Stop fails the 26th row too. Companion of step 7.
- The `r3w` rehearsal asserts no fake-agent receipt key count: its receipt holds `viola send`'s submit Enters and the
  `ws` verify's keys; the rig's own `keys: none` reading is the no-key witness.
- `HB` live is fired without `--model haiku` (no send, no turn): the plan names no child args for it.
- scope: `scope: clean — changed 22 · listed 21 · recorded 1 (companion 1 …)`.

## Gates
Offline (last full firing): fmt green · clippy green · `cargo nextest run -p viola-agent-claude` green ·
`run --unit` green · `run --integration --filter 'binary(cli_wheel) | binary(cli_verify)'` green ·
`run --e2e --filter 'binary(tui_wheel) | …'` green · `run --integration --filter 'binary(rig_rehearsal)'` green ·
`cargo nextest run -p viola-e2e --lib` green · `run --integration` green (574) · `run --e2e` green (324).
Reds met earlier and fixed (this chunk's own test pins and lints): clippy `double_ended_iterator_last` then
`filter_next` in `tests/tui_wheel.rs`; cli_verify summary totals and one row assertion; `rig_rehearsal` vh `rows 25`;
the `r3w` rehearsal's receipt key check.
Controls (`evidence/control-2.txt`): `control: 6 holds · 0 timeout · 0 does not hold · 0 unreproduced · clone removed`.
Live round (`evidence/round-064747Z.txt`, `round-064849Z.txt`, `round-064911Z.txt`): each `round: COMPLETE · legs
fired 1/1`; WS `pass-count: 26/26`, `stop-session-crons: pass`, `kept-stamp: written`; HB `hint-cells: 22`,
`hint-cells-attr: dim(22)`, `typed-attr: plain`, `hint-told-apart: yes`; R3W `wake: fired`, `wake-origin: harness`,
`wheel-after-fire: kept`, `send-2-reply: ok`, `send-1-wakes: 1`. Post-round operator entries 31 `census: 0`,
32 `live-stamps: unmoved` (`evidence/post-round-operator.txt`). Spare not fired — spare unspent.
Host reads before each start: QUIET, no neighbour named (06:47:36Z last-2-min read; 06:48:45Z; 06:49:06Z). The first
read over 06:41:20Z..06:47:30Z named escher (alive 5 s, 1 cargo) and was not fired into.
Watch `boot_that_never_gets_ready_times_out_and_stops_its_supervisor`: 3 green runs (33.log 2.42 s, 33.2.log 2.43 s,
33.3.log 2.64 s). cry-yx2svady `rig_rehearsal_reads_the_box_empty_after_x_and_del`: 6 green firings, no red.
Smoke: skipped — the plan lists no smoke entry; the boot-path stamping verify at 26 rows ran in `run --e2e`, green.

## Process hygiene
| process | started by | final state |
|---|---|---|
| claude 2.1.328 (WS's verify children, HB, R3W's wrapped child) | a leg | terminated — rig census 0 after each leg, 31 `census: 0` |
| viola-live-rig, viola (scratch wrappers) | a leg | terminated — `census: none running` at P4 |
| viola-fake-agent, viola-harness, cargo-nextest (gates, controls) | this run | terminated — census at P4 lists none |
| 8 `claude` processes on `latest` (the host's overseer and builder sessions) | NOT DERIVED — not this run's | left running: the operator's sessions |

## Decisions & corrections
- The overseer's word to fire the round, with a hostwatch read over the window since 06:41:20Z before each start
  (`inputs#I5`). A read naming escher (alive 5 s) was not fired into; a last-2-minute read naming no neighbour was.
- The gate tool's control-id form (≤16 chars) refused a plan-named control id; renamed, its edit kept.
- Bash guard refusals: a `cd` outside the root (twice), a cat heredoc to a file, a pipe of gate.py into grep — each
  re-issued once in the guard's form.
