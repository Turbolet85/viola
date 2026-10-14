# P5 review card 1 — 2026-10-13-owed-ledger-rows · Owed ledger rows

Plan: `viola-0.1.0/chunks/2026-10-13-owed-ledger-rows/plan.md` (13 steps, 11 acceptance criteria, a 26-entry fence;
about 320 lines of prose, inside the 150–400 band).

## Goal
Three probed, stamped capability-ledger rows (`v1-34`), all measured from Run B's own session with no added start:
- `statusline-override`: a `--settings` statusLine runs in the status line's place;
- `statusline-shell`: a statusline command runs through `/bin/sh`;
- `statusline-rate-limits`: the payload carries two whole-number `rate_limits` windows. This is the shape the shipped
  `budget-paused` refusal rests on.

Run B is started with a probe `--settings` file whose command is the pinned `viola` with a new hidden capture arm,
`hook statusline --capture <dir> --shell "$0"`. The arm files each payload at 0600 and a closed shell verdict
(`bin-sh` | `other`). The founder's fork-3 widening is `inputs#I1`.

## The founder's four answers (`inputs#I1`, relayed by the operator from the overseer's dialog, 22:52:37Z)
1. The cut: the statusline trio is built here.
2. The `agents --json` row goes to its consumer.
3. Run B gains the probe `--settings`.
4. The cap is ten live starts.

## Decided by the artifacts (leaned; shown for your check)
- **A per-OS relied row set.** The three rows are relied on only where the status line is wrapped (Unix,
  `statusline::WRAPPED_HERE`). A Unix host prints and stamps `[NN/21]`; a Windows host keeps `[NN/18]`, and its stamp
  verdict reads its eighteen. Shipping them on Windows would leave every Windows home unverified forever, and passing
  them there would print `pass` for a shape never measured.
- **The fake agent hands `"$0"` over as `/bin/sh`.** It never runs a shell (test-plan §7). This replays the 2.1.287
  live launch reading; the live round stamps the real CLI's shell.

## Top acceptance criteria
- `LedgerRow::ALL` holds 21 rows; `relied()` holds 21 on Unix and 18 on Windows. An eighteen-id stamp reads
  unverified on Unix and verified on Windows.
- Nine `check_statusline_` cases read 9 passed.
- The record round on 2.1.287 (5 starts) prints `stamped 2.1.287  21 pass  0 fail`. The `run --local-live` round
  (5 starts) reads `"suite":"local-live","passed":1,"failed":0`. Both censuses are clean.
- `fixtures/claude/2.1.287/` gains exactly `StatusLine.default-1.json` and `StatusLine.rate-limits-1.json`, validated
  against the new `schemas/claude-statusline-fixture.v1.json` and replayed byte for byte.
- The capture arm prints nothing, exits 0 on every path, files only the closed verdict and opens no role file. The
  verify run keeps six spawn pairs.
- No other source changes (the preservation guard), `cargo deny` passes, and CI is green on all three OSes.

## Touchpoints
- **Modified:** `ledger.rs`, `statusline.rs`, `src/cmd/hook.rs`, `src/cmd/verify.rs`, `typed.rs`, `version_gate.rs`,
  the fake agent, eight test files and the harness `run.rs`.
- **New:** the statusline schema, two recorded fixtures, `evidence/`.

## Test commands
The fence holds:
- lint ×4 and the full unit run;
- the nine-case filter;
- the live round: probe, record (5 starts), census, probe, `run --local-live` (5 starts), census;
- the fixture guard, the integration binaries and the preservation guard;
- `cargo deny`, the smoke boot, status and cleanup, G2, G4 and `pre-push`;
- the operator's hygiene, push and CI read.

Baselines:
- new and red as predicted: the 9-case filter (0 selected) and the fixture guard (no variants yet);
- new and green: integration (129/0), the preservation guard, `cargo deny` and smoke.

The controls are in `p5-controls.md`.

## Concretization preview
None: no capability is claimed. `v1-34` stays pooled. The harness-prefix and R8 identity-floor rows stay owed with no
entry, and `--resume` and `/compact` move. Its not-claimed note is written at step 4.

## Beyond the plan
1. **New route entry, "Resume and compact ledger rows", directly after "Owed ledger rows".** It carries this entry's
   CARRY 2 (`claude --resume <id>`, chunk 2026-10-10-viola-revive) and CARRY 3 (`/compact`, the same chunk) verbatim,
   and CARRY 1's cost frame verbatim. The source is the founder's fork 1 (`inputs#I1`).
2. **The `claude agents --json` join row and its probe move onto "Unwrapped sessions on the board"**, with this
   entry's CARRY 6 verbatim. The source is the founder's fork 2 (`inputs#I1`).
