# P5 controls — 2026-10-13-epoch-4-survivors-ii

Validation check 4 (9). Each control was fired as its entry, through `gate.py run --plan
viola-0.1.0/chunks/2026-10-13-epoch-4-survivors-ii/plan.md --entry {n} --project-root {clone}`, with no `--run-dir`.
The clone is a scratch `git clone --shared` of the project at HEAD `64d7e5f7f620`
(`$TMPDIR/{scratchpad}/clone`). Each control input lived only in the clone and was reset before the next one.

## Unmodified, in the clone (it must read what the tree read)
- entry 8 · clone @ `64d7e5f7f620`, no input · red · `exit 0 ✗ (exit 1)`. FileNotFoundError: the chunk's
  `evidence/survivors.ndjson` is absent, as on the tree.
- entry 9 · clone @ `64d7e5f7f620`, no input · green · last line `0`, as on the tree.
- entry 10 · clone @ `64d7e5f7f620`, no input · green · exit 0, as on the tree.

## Entry 8 — the survivor ledger
- Must-pass: a 16-row ledger, one row per `owed` row of the earlier chunk's ledger, `form` `restatement`, `by` and
  `file_at_head` filled (`$TMPDIR/{scratchpad}/mk_ledger.py`). Green, last line `1`.
- Must-fail: the same ledger with its first row dropped (15 rows). Red, `last line '1' ✗ (last: '0')`.

## Entry 9 — no Run B constant, paste, guard, cap or log line removed from `typed.rs`
- Must-fail: the line `const POLL: Duration = Duration::from_millis(5);` removed. Red,
  `last line '0' ✗ (last: '1')`.
- Must-pass: a deadline restatement as the plan builds it. `framing_turns`' `let until = clock.now() +
  PROBE_DEADLINE;` became `let until = probe_until(clock);`, a `probe_until` function was added, and
  `Run::wait_pump`'s doc line was removed (`$TMPDIR/{scratchpad}/ctl_edit.py restate-deadline`). Green, last line `0`.

## Entry 10 — the preservation guard
- Must-fail: one comment line appended to `src/main.rs`. Red, `exit 0 ✗ (exit 1)`.
- Must-pass: one comment line appended to `src/cmd/verify/typed.rs`. Green, exit 0.
