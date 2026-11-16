# Outcome — 2026-11-16-wait-wake-condition · run 2026-11-16T02-56-58-implement (re-entry on plan rev 8)

**Outcome: surfaced.** Step 2's bounded park waits are written, and entries 1–4 and 11 read green. The `no-false-wake`
control now reads `holds`, but `held-turn` still reads `timeout` under the 120 s kill. Plan rev 8 diagnosed the hang
as the unbounded `parked.recv()`. The measured cause is different: two existing tests hang in `std::thread::scope`'s
join of a waiter thread that has no deadline. Reaching `holds` would move step 2's wording ("each wait on `parked`
takes `recv_timeout(PARK_BOUND)`"; "the existing 5 s reply bounds unchanged") or step 9. The operator's direction was
"A red that moves a plan entry: stop BEFORE the operator pass", so this run stopped before the operator pass: no
hygiene entry 13, no pre-CI commit, no push, no `ci.py` read. No matrix ref was set, and no chunk report was written.

## The gap
- **Plan (rev 8):**
  - Step 9 / the acceptance say «Controls `held-turn` … each read `holds`», with no `timeout` word.
  - Revision 2's premise: `held-turn`'s timeout came from «the module's two existing tests, each blocked on an
    unbounded `parked.recv()`».
- **Measured** (`.andromeda/cache/chunks/2026-11-16-wait-wake-condition/evidence/control-2.txt`):
  `control: 1 holds · 1 timeout · 0 does not hold · 0 unreproduced · clone removed`.
  - `no-false-wake`: **holds**, 7.29 s. Its case now FAILS instead of hanging, because the bound reached it.
  - `held-turn`: **timeout** at 120.01 s.
    - The guard's four cases FAIL in 0.059 s each (`src/run/wait.rs:552`).
    - `wait_a_line_appended_and_signalled_during_the_park_wakes_it` and
      `wait_a_signalled_log_only_line_parks_again` each PANIC in seconds, at their 5 s `"woken"` reply bound
      (`src/run/wait.rs:495`, `:528`), then run SLOW past 60 s. After the panic, `std::thread::scope` joins the
      spawned waiter. That waiter is parked on `timeout_ms: 1_000_000_000` under the `Announcing` clock, which never
      moves, so the join never returns. No `parked` bound reaches that join.
- **What would close it** (phase's or the operator's call; implement authored none):
  - (a) The two existing tests give their waiter a reachable end: a finite `timeout_ms` on a clock that moves once
    the test is done, or a waiter outside `thread::scope` so a panic does not join it. These are test-module lines,
    but a change to step 2's text.
  - (b) Step 9 fires `held-turn` on an entry filtered to its own case (`test(/wait_a_turn_end_wakes_whatever_its_background_tasks/)`).
    This moves step 9 / adds an entry.
  - (c) The acceptance reads `held-turn`'s `timeout`, with every guard case FAIL in 0.06 s, as proof, and records
    `no-false-wake`'s `holds`. This moves the acceptance wording.

## Gates (this run)
- `entries 15 · green 5 · red 0 · recorded 0 · timeout 0 · not-run 10`, trail
  `.andromeda/runs/2026-11-16T02-56-58-implement/gate-2026-11-16-wait-wake-condition.json`:
  - 1 fmt green;
  - 2 clippy green;
  - 3 check green;
  - 4 green, `48 tests run: 48 passed, 960 skipped`;
  - 11 `run --unit` green, 21.63 s.
- Controls: the gate trail above, and `evidence/control-2.txt`.
- Not re-fired (rev 8): entries 5–10 and 12 stand from the earlier runs, and the four controls that read `holds`
  stand too.
- Not run: the operator entries 13–15.

## Files changed (whole chunk)
`src/run/wait.rs` (tests; this run added `PARK_BOUND` and bounded 7 `parked` waits) · `fixtures/fake-scripts/wait-held-turn.json` (new) ·
`tests/cli_wait_last.rs` · `tests/support/home.rs` · `tests/cli_verify.rs` · `.config/nextest.toml` (comment).

## Deviations
- None from step 2 as written.
- Scope: `scope: clean — changed 6 · listed 6 · recorded 0 (companion 0 · mechanical 0 · in-intent 0 · widening 0) ·
  absorbed 0 · excluded 253`.

## Process hygiene
| process | started by | final state |
|---|---|---|
| nextest / cargo of entries 1–4, 11 and the two controls (clone `~/.cache/andromeda-control/b9ef3011`) | this run | terminated. Census `gate.py census --comm viola --comm cargo-nextest --comm viola-fake-agent --args projects/viola/target` at 2026-11-16T03:01Z: 7 running, none from this repository's `target/` (the product install `~/.local/viola-product`, the lab prototype's builds, `~/.viola/bin/0.1.0-…` mcp). The tool removed the clone |

## Decisions & corrections
- The operator's direction was followed: no live start, no mutation run, stop before the operator pass on a red that
  moves the plan.
- Plan rev 8's premise was falsified by measurement. A park-wait bound does not end a test whose spawned waiter has no
  deadline inside `thread::scope`, because the panic waits for the join. The evidence is the removed log's two SLOW
  lines after their panics.
- The Bash guard refused one `cd` into the skill's references dir. It was re-issued with absolute paths.
