# Implement outcome — 2026-10-19-newer-claude-cli-rows

**Outcome: green — on the non-live part only, as the operator directed: plan steps 1–7, no live start; steps 8–13
gated on the founder's word.** Two acceptance criteria are open, by design, and are not claimed:
- **The macOS readiness flake** (steps 6(c) and 7). The phase is not named yet: the recorded runs bound the stalled
  span but cannot place it. Step 7 does not run until a CI run of the operator pass names the phase.
- **The gated criterion** (steps 8–13). It waits on the founder's word on Fork 1 and Fork 2
  (`.andromeda/runs/2026-10-19T23-10-05-phase/founder-forks.md`).

The chunk stays `pending`. **Next:** the operator pass. Entry 33 (`gate.py hygiene`) runs before the pre-CI commit,
entry 34 (the push) and then entry 35 (`ci.py conclusion`). Its `diag-macos-latest` and `junit-macos-latest`
artifacts feed steps 6(b), 6(c) and 7. Only after that come the founder's word on the gated part and then the wrap.

## Files changed (`gate.py scope`: clean — changed 5 · listed 5 · recorded 0)
- `crates/viola-agent-claude/src/ledger.rs`:
  - `TypedRun::long_turn`;
  - `check(InputBoxSignature)` now reads `ready`, `turn` and `long_turn`;
  - three unit cases (`check_input_box_signature_long_turn_holding_the_literal_passes`, `…_holding_a_modal_fails`,
    `…_of_none_fails`);
  - `clean_typed()` gains a `long_turn` holding the input-box row, so every other `input-box-signature` case reads it.
- `src/cmd/verify/typed.rs`: `framing_turns` keeps the rows `box_wait` settles on after the long paste's turn in
  `typed.long_turn`. They stay none when that wait fails or the turn never ended. Nothing reaches `measured`.
- `src/bin/viola-fake-agent.rs`:
  - `--long-turn-screen <phase>`: `Screen.<phase>` drawn after the long paste's turn alone, after its Stop, in
    `end_turn`;
  - two parse cases named `opts_long_turn_screen_…`, and the all-flags parse case extended.
- `tests/cli_verify.rs`: `verify_pastes_nothing_more_once_the_long_turn_screen_shows_a_modal`.
- `tests/support/home.rs`: `wait_ready`'s stall report (`note_stall`, `wrapper_pid`, `process_state`, per OS).
- Chunk folder (version workspace, excluded from the scope set):
  - `evidence/reproduction.md`;
  - `evidence/macos-run-start.md`;
  - `macos-run-start.py`, step 6(b)'s scratch reader, which the plan puts in the chunk folder, never in `scripts/`.

## Gates (trail `.andromeda/runs/2026-10-19T23-48-08-implement/gate-2026-10-19-newer-claude-cli-rows.json`)
- **Red-before:** entry 22 alone (step 2's arm not yet extended) read red. The stdout was the overseer's 2.1.302
  verdict, row for row, with `stamped 2.1.0  20 pass  4 fail` (`evidence/reproduction.md`).
- **Full block:** `entries 35 · green 19 · red 0 · recorded 0 · timeout 0 · not-run 16`.
  - Not run, as designed: the `gated:` defers 8 and 20, the leg round 9–19 and the operator entries 33–35.
  - Entry 7: `5 tests run: 5 passed`.
  - Entry 21: `115 tests run: 115 passed`, P5's 114 plus the new case.
  - Entry 22: the reproduction case green, at `19 pass  5 fail`.
  - Entry 23: `90 passed`.
  - Entry 24, the preservation guard: green.
  - Entry 32, `pre-push`: green in 86.3 s, `"stage":"linux-tests"`.
- **Remove-the-guard:** with the `long_turn` operand removed, `3 passed, 2 failed`. The two failures were the modal
  and None cases. Then the operand was restored, and entries 1, 2, 6, 7, 22 and 24 re-fired green
  (`evidence/reproduction.md`).
- **Deferrals:** none. Rust changed, so every Rust gate ran.
- **WATCH** (the truncated `viola` `.profraw` at `pre-push`'s coverage merge): `pre-push` ran it green once in this
  run. No red. This is a fact for the wrap's tally.
- **Matrix:** `matrix.py show --chunk` reads `claimed by 2026-10-19-newer-claude-cli-rows: 0`. No `ref` was written,
  as the plan said ("No verification-matrix claim").

## Smoke (P3)
Recorded from P2, not re-driven: entries 26–31 ran green.
- `boot` stamped through `viola verify`, the changed Run B path with the long turn's screen read, and came up ready.
- `status` read `ready`, `cli_verified:true`.
- G2 and G4 were green.
- `cleanup` read `processes_gone:true, endpoint_gone:true`.

## Deviations from the plan, with justifications
1. **Step 6(a): where the wrapper's pid comes from.** The plan gives `sample <pid>` but not where the pid comes from.
   `tests/support/outer_pty.rs` exposes none and lies outside the preservation guard's (entry 24) allowed paths. So
   `home.rs` finds the pid with one sysinfo read instead: the `viola` (exe or `argv[0]` equal to the built binary)
   whose arguments hold the boot's fake-agent control file. No out-of-list file was edited.
2. **Step 6(a): `sample` is bounded and writes to a file.** It runs as `/usr/bin/sample <pid> 1 -mayDie -file
   <temp>/viola-root-watch/stall-<pid>.sample.txt` under a 4 s bound, and the test kills only that child of its own
   past the bound. Only the `Call graph:` block is kept, at most 120 `stack` lines. A bound was needed to stay inside
   the 20 s CI kill line once the 7 s `WITHIN` has passed. The stall code is compiled per OS: macOS, Linux, and a
   no-op elsewhere. Entry 3 (the Windows-target clippy) is green.
3. **Step 5: no literal moved.** None of the plan's candidates (the no-framing and no-turn-stop summaries) moved.
   - In those runs the long turn draws the `turn` screen, or the run never reaches it.
   - The `--tag-turn-screen` case stays at `21 pass  3 fail`.
   - Entry 21 read 115/115 with every earlier literal unchanged.
4. **Step 4: the Windows literal is still a forecast.** `stamped 2.1.0  16 pass  5 fail` (21 − 5) was not run here;
   CI's `windows-2025` job reads it first. The Unix literal is measured.
5. **Step 2: the cases' form.** The three cases are named `#[test]` functions, each holding `long_turn`. The modal case
   loops over a literal list of the two modal rows, as the existing case beside it does.
6. **Steps 6(c) and 7: not done** (see below). This is the plan's own rule, not a skip.
7. **Research's `"for agents"` sweep: not run.** Research ties it to the literal change, which is step 9 and gated.

## What stays open
- **macOS flake, step 6(b)** (`evidence/macos-run-start.md`). On ci#39321117796:
  - Attempt 1 had three gaps of about 5.83 s: the job's first three wrapper starts, all at 20:41:41.949Z. It also had
    one gap of 2.48 s and the two stalls (pids 16375 and 16377) with no line after `process-start{self}`.
  - Attempt 2 peaked at 355 ms; Ubuntu at 806 ms; Windows at 1.09 s.
  - Concurrency does not explain the gaps. The slow and stalled starts had 1–4 other starts within 1 s; the fast ones
    had a median of 5 and up to 22.
  - `run.start`'s steps 1–6 log nothing on success, so the phase is **NOT NAMED**. `pin_exe` stays a hypothesis.
- **Step 7 is not run** (by the plan's own rule), and the criterion reads unmet. The 6(a) report lands in a stalled
  test's panic message, so in the `junit-macos-latest` artifact (the watch file under the runner's temp dir is not
  uploaded). The operator pass's CI runs are its first chance to fire.
- **Gated (steps 8–13):** not started. No live start was made and no `defer` was voided.

## Process hygiene
`gate.py census --comm viola --comm viola-fake-agent --comm claude --comm sample --args projects/viola/target/`
(trail `census-2026-10-19-newer-claude-cli-rows.json`) read `census: 20 running`. None is this run's: no row's
command line holds `projects/viola/target/`.

| process | started by | final state |
|---|---|---|
| gate entries' children (cargo, nextest, test binaries, fake agents, verify children) | this run | terminated (every entry exited; no survivors line) |
| smoke session `p-ncr-smoke` (supervisor, wrapper pid 999946, child pid 999954) | this run | terminated (`cleanup` read `processes_gone:true`, `endpoint_gone:true`) |
| `/usr/bin/sample` | — | never started (no boot stalled) |
| the 20 census rows: other trees' bridges and builders, the overseers, `~/.local/viola-product/bin/viola wait`, `~/.viola/bin/…/viola mcp`, this session's own `claude` | not this run (the operator's host) | left running — not this run's to stop |

## Decisions & corrections
- **The operator's ruling for this run:** "steps 1–7 of Newer claude CLI rows (the non-live part); steps 8–13 stay
  gated on the founder's word — no live start. Stop with your report; the operator pass and the wrap wait for my next
  message." (operator, 2026-10-20, this session).
- **Guard refusal:** the Bash guard refused a `cd` twice. One was into the implement skill's `references/`, the other
  into the chunk's `inputs/`. This is a recurrence of the carried host-linux.md §Paths learning. The first was
  re-issued with absolute paths; the read the second served was dropped for the CI artifacts.
- **Hook refusal:** the `stamp-ahead.py` PostToolUse hook refused an estimated time, a few minutes ahead of the clock, written into
  `evidence/reproduction.md` while the clock read 00:00:52Z. It was corrected to the measured bound (taken before
  2026-10-20T00:00:52Z, the `date -u` read just after).
- **Measured, not a rule:** a macOS stall report can only travel in the panic message to the JUnit. The CI
  diagnostics artifact keeps only `diagnostics/` dirs, and `viola-root-watch/` is under the runner's temp dir.
