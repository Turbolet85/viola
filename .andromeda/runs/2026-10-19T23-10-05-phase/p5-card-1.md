# P5 review card 1 — 2026-10-19-newer-claude-cli-rows ("Newer claude CLI rows")

Plan: `viola-0.1.0/chunks/2026-10-19-newer-claude-cli-rows/plan.md` · research and scope beside it · run dir
`.andromeda/runs/2026-10-19T23-10-05-phase/`.

## Goal
The four rows claude 2.1.302 fails are one stop, measured from the CLI's own records (`inputs#I3`). After Run B's long
paste, verify waited its full 120 s for the input-box literal `"for agents"` and never saw it: 2.1.302 hides that
footer hint about 10 s after a client attaches, and 2.1.296 never hides it. `run`'s pre-send gate reads the same
literal, so the remedy is a product reliance: a second, live-measured input-box literal, recorded so CI replays it.

## What runs now, with no live start (steps 1–7)
- **The input-box row.** `TypedRun` keeps the screen Run B settles on after the long paste's turn, and
  `input-box-signature` reads it, so the row names the cause instead of passing by timing.
- **The fake agent** gains `--long-turn-screen <phase>`.
- **The reproduction case** `verify_pastes_nothing_more_once_the_long_turn_screen_shows_a_modal`: a modal after the
  long turn stops Run B at once.
  - Predicted `19 pass  5 fail` on Unix and `16 pass  5 fail` on Windows; the literals are copied from the first run.
  - Its red-before reading must reproduce the overseer's `20 pass  4 fail` four-row set.
- **The macOS readiness flake, measured first.**
  - A stall report in `wait_ready`: macOS `sample`, Linux `/proc` wchan.
  - The self → version-probe gap read from macOS diag artifacts.
  - Then a fix at the measured phase, with a witness red and then green.
  - If no reading locates the phase, the criterion reads unmet and the red stays open. It is not closed by a green
    re-run.

## Gated (steps 8–13): runs only after the founder's own word on Fork 1 and Fork 2
- The 2.1.302 footer reading: 1 start, a `tmux` capture of the bottom three rows at about 15 s and 25 s, one
  `viola list` at about 2 s.
- The measured literal added beside `"for agents"`.
- `viola verify --record` on 2.1.302 (6 starts): `stamped 2.1.302  24 pass  0 fail` and `fixtures/claude/2.1.302/`.
- That set placed on the stamped list.
- `--local-live` (6 starts).
- A 2.1.296 verify (6 starts) under his option 2 only.

The block, verbatim: the founder's own word on the live-start cap and on Fork 2
(`.andromeda/runs/2026-10-19T23-10-05-phase/founder-forks.md`). A stand-in, stub or recorded artifact never stands in
for the live round.

## Top acceptance criteria
- The reproduction case and its `20 pass  4 fail` red-before reading (`evidence/reproduction.md`).
- The `long_turn` unit cases.
- Every 2.1.287 fake-agent stamp still 24/0, and the `--tag-turn-screen` case unchanged.
- Run B's six pastes, six starts and eight spawn pairs unchanged.
- No new obs vocabulary.
- macOS `test` green on its first attempt, with its cause named.
- Gated:
  - the footer evidence;
  - the measured literal's `resting_footer` cases;
  - `stamped 2.1.302  24 pass  0 fail` with the set committed;
  - `--local-live` passed;
  - the transcript delta inside the accepted class.

## Touchpoints
- `ledger.rs`, `typed.rs`, `viola-fake-agent.rs`, `tests/cli_verify.rs`, `tests/support/home.rs`.
- Gated: `screen.rs`, `tests/contract_ledger_probes.rs`, `fixtures/claude/2.1.302/`.
- Provisional, step 7's fix site only: `pin.rs`, `src/cmd/run.rs`, `.config/nextest.toml`.

## Test commands
35 entries:
- lint;
- unit, plus the `long_turn` filter;
- the gated `resting_footer` filter;
- one contiguous gated live round of 11 entries (probes, counts, the footer reading, the record round, `--local-live`,
  the option-2 verify and their censuses);
- the fixture guard;
- integration;
- the reproduction case;
- the contract and `cross_list_rows` binaries;
- the preservation guard, `cargo deny`, the smoke, G2, G4 and `pre-push`;
- the three operator entries.

Baselines at HEAD `50e2f76167ee`:
- green: 3, 21 (114 passed), 23 (90 passed), 24, 25, the smoke entries, and 32 (`pre-push`, 88.6 s);
- red as predicted (the subject not yet built): 7, 8, 22 (0 tests selected) and 20 (no 2.1.302 set);
- the controls are in `.andromeda/runs/2026-10-19T23-10-05-phase/p5-controls.md`.

## Matrix
No claim. `v1-34` still needs the harness-prefix and R8 identity-floor rows and the Windows readings. `v1-03` and
`v1-21` are not advanced to their acceptance. A dated not-claimed note goes on `v1-34`.

## Where the plan departs from the direction (`inputs#I4`), for your word
1. **No gated arm at the wrap.** `route.py flip --to gated` refuses without a claimed gated cap below verified
   (route-contract, the `flip` row), and no cap is claimable. So after implement's non-live work the chunk stays
   `pending` until the founder's word runs steps 8–13. It does not wrap gated.
2. **The reproduction uses a modal stop, not the resting footer itself.** A literal-less footer makes `box_wait`
   wait 120 s, past every test kill line.
3. **No synthetic `send` gate case now.** `src/run/send.rs:666` already pins a verified screen with no literal. The
   measured row's `Ready` case lands with step 9.
4. **Fork 2's text was corrected before his answer.** The measurement's cwd is `.viola-footer-<pid>/` under the
   repo root, because an untrusted dir would raise the trust modal. The live forks file now differs from its snapshot
   `inputs#I4`.
5. **Size.** The plan is about 630 lines, of which the fence is about 300; the prose is inside the ~400 band.

## Beyond the plan
none
