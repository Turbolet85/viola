# P5 review — 2026-11-16-send-confirmation · «Send confirmation» (viola-0.2.0, Epoch 3)

**The plan to read:** `.andromeda/cache/docs/2026-11-16-send-confirmation/plan.md` (the render of
`doc:2026-11-16-send-confirmation/plan` rev 3, 401 lines).
- No plan stands in a chunk's folder.
- Writing in that file changes no record until a `rec.py put`.
- `python -X utf8 ~/.claude/skills/andromeda-tools/scripts/rec.py diff --root . 2026-11-16-send-confirmation` diffs
  the chunk's records.

## Goal
1. **The trailing-whitespace match.** A driver's send whose text the CLI submits with a trailing run of whitespace
   removed is read back as the driver's own. The prompt is relabelled `driver`, the send is confirmed, and the wheel
   stays with the driver.
   - The tolerance is the delegate's answer to Q1 (`inputs#I2`, PROVISIONAL): a trailing run of ASCII space, TAB, CR
     and LF. The set is an assumption, since the CLI's own trim is unread.
   - The text is validated as received and typed as today; nothing is stripped.
   - Leading whitespace still claims nothing, and the founder's word is owed on it.
2. **The race.** The readiness-gate turn race is closed for every turn the wrapper can know of.

## Top acceptance criteria
- A send ending in such whitespace is confirmed as the driver's own and the wheel stays put. A refused character is
  still refused (`matrix#v2-18`).
- A turn whose `prompt-submitted` reaches the wrapper at any point before the paste refuses the send `not-delivered` /
  `turn-running`, with nothing typed, no link and no `send-issued`. A unit case reproduces a turn starting after the
  post-gate reads, and fails with the guard removed (`matrix#v2-19`).
- The predicate is pinned by two tables and a 512-case property. The fake agent's drift contract stays byte for byte,
  and no recorded fixture changes. `run --unit` and `pre-push` read `"ok":true`. CI on the pushed HEAD reads green.

## The lock (the relay's three asks, `inputs#I2`)
- **What it holds across.** The slot's flight lock is held across four things, in this order:
  - the post-gate human-typing and turn-running reads (`src/run/send.rs:121-130`, moved under it, not duplicated);
  - the link record;
  - the `send-issued` append;
  - the paste.

  It is released when the paste returns. In `append_hook_event` the claim, the session-start claim, the turn start
  or end and the unsent-human wheel move are decided under that same lock, released before the snapshot update and
  the append.
- **What a second sender sees while it is held.** It waits at its own arrival read (`src/run/send.rs:102`) until the
  paste returns. It then reads the first send in flight and is refused `not-delivered` / `turn-running`, nothing typed,
  which is today's answer. A hook line waits the same way, at most one link append, one `send-issued` append and the
  paste. A human key never waits: the stdin path takes no flight lock.
- **The test that fails without it.**
  `send_a_turn_started_after_the_post_gate_window_is_turn_running_and_types_nothing` (entry 6).
  - A forced-window parameter on the private `send_through` appends a harness-filed `prompt-submitted` after the gate.
  - With the guarded turn read removed, the text is pasted and the case reads red: the `locked-turn-read` control,
    fired under `gate.py control --timeout 120`.
  - The case is synchronous, with no thread or channel wait, so it cannot hang past a failure (`inputs#I1` item 4).
  - The new relabel helper reads its paste handshake by `recv_timeout`, never a bare `recv()`.

## The residual and its owner
A turn the CLI has started whose `UserPromptSubmit` has not yet reached the wrapper before the paste is beyond the
wrapper's reach:
- the gate reads the input box, which a turn's screen keeps (`crates/viola-agent-claude/src/screen.rs:664`);
- the hook latency is unmeasured.

It rides v2-19's claim as a dated note. **Owner: «Standing live rig with a screen model»** (Beyond the plan, row 2).

## Touchpoints (8 files, no new file)
`crates/viola-agent-claude/src/hook.rs` · `src/run/send/slot.rs` · `src/run/send.rs` · `src/run/send/relabel.rs` ·
`src/run/send/test_support.rs` · `src/bin/viola-fake-agent.rs` · `src/bin/viola-fake-agent/test_cases_opts.rs` ·
`tests/cli_send.rs`

## Test commands (16 entries; base `86fc64d0b4fb`)
| # | entry | baseline |
|---|---|---|
| 1–3 | fmt · clippy · check | seen green |
| 4 | hook unit cases | green, 117 passed |
| 5 | the send module | green, 126 passed |
| 6 | the race case, by name | red, predicted: no tests to run |
| 7 | the trimmed-prompt cases, by name | red, predicted: no tests to run |
| 8 | the fake agent's unit cases | green, 33 passed |
| 9 | the booted trimmed send, by name | red, predicted: no tests to run |
| 10 | `cli_send` · `channel_paste_validation` · `cli_fake_agent` · `contract_fake_agent_drift` | green, 84 passed |
| 11 | the fixture-unchanged probe | green, 0; control red at 1 in a clone (`p5-controls.md`) |
| 12–13 | `run --unit` · `pre-push` | seen green |
| 14–16 | hygiene · push · CI read | operator legs |

- No smoke: no boot-path touchpoint changes.
- No mutation run (`inputs#I1` item 3).
- No live start: the cap is 0 (`inputs#I2`).
- Controls (implement's one-shot, `--timeout 120`, each on its filtered entry): `trail-trim` (entry 7),
  `locked-turn-read` (entry 6), `fake-trim` (entry 9).

## Concretization preview (the never-weaken checkpoint)
- **v2-18.**
  - **Now:** «A driver's send whose text ends in whitespace the CLI trims is confirmed as the driver's own, the wheel
    stays with the driver and the send succeeds; a refused character is still refused, never removed.»
  - **Proposed:** «A driver's send whose text ends in a trailing run of ASCII space, TAB, CR or LF, submitted by the
    CLI without that run, is confirmed as the driver's own.
    - The `prompt-submitted` is relabelled `driver` with its text as submitted, the send answers `ok`, no `wheel`
      record is written, and a later send after the turn ends is accepted.
    - A refused character is still refused, never removed.
    - Held by the trimmed-prompt unit cases (`trail-trim` holds), the booted case on the fake agent (`fake-trim`
      holds) and the wrapper's paste-validation cases.»
  - **The narrowing, named:**
    - the whitespace set is ASCII by assumption, the CLI's own trim set unread;
    - the leading side is not tolerated (PROVISIONAL, `inputs#I2`);
    - the requirement's own words are «ends in a space (or any whitespace the CLI trims)».
- **v2-19.**
  - **Now:** «A turn that starts while a send waits on the readiness gate never receives the send's text: the send is
    refused as a running turn or typed only once that turn is over, and a test reproduces the race.»
  - **Proposed:** «A turn whose `prompt-submitted` reaches the wrapper at any point before the paste — the gate's wait
    included — refuses the send `not-delivered` / `turn-running` with nothing typed, no link and no `send-issued`.
    - The gate-wait cases (`src/run/send.rs:606-635`) and a unit case reproducing a turn started after the post-gate
      reads (`locked-turn-read` holds) prove it.»
  - **Note line:** the hook-latency window is unmeasured and beyond the wrapper's reach; its owner is «Standing live
    rig with a screen model».
  - **The narrowing, named:** a turn the CLI started that the wrapper has not heard of (Q2, the overseer's technical
    answer, `inputs#I2`).

## Expected amendments (wrap)
- security-plan · decision/delivery-confirmation-exact-match. This is a boundary widening, PROVISIONAL (`delegate
  overseer`), so the wrap escalates it in full.
- security-plan · decision/typed-text-line-ending-normalisation: its why is now stale.
- architecture · decision/exact-prompt-match.
- architecture · contract/hook-event/driver-relabel.
- architecture · contract/channel-methods/hook-event-driver-origin.
- test-plan · decision/send-confirmation-exact-match.
- test-plan · decision/send-refusal-order (the reads under the lock).
- test-plan · contract/fake-agent-cli/argv-only-options (`--trim-prompt-end`).

## The record (directive item 7)
- **The union, verbatim:**
  ```
  architecture: rows 943 · packs 10 · picked 175 (114 by every draw) · standing 31 · 48135 B
  security-plan: rows 599 · packs 6 · picked 110 (71 by every draw) · standing 15 · 32549 B
  design-system: rows 314 · packs 4 · picked 62 (49 by every draw) · standing 12 · 14151 B
  layout-templates: rows 221 · packs 3 · picked 57 (36 by every draw) · standing 0 · 12034 B
  test-plan: rows 1329 · packs 14 · picked 318 (197 by every draw) · standing 51 · 76070 B
  obs-plan: rows 1021 · packs 11 · picked 164 (105 by every draw) · standing 29 · 40301 B
  a11y-plan: rows 235 · packs 3 · picked 63 (49 by every draw) · standing 15 · 17689 B
  rows-union: 7 master(s) · picked 949 of 4662 · rule any
  ```
- **How the rows were read:** the rows files whole, sum 240 929 B, under the 250 000 B line, each read once at P2's
  checkpoint. One page of `tests-rows.md` passed the Read cap and was read in two consecutive ranges, lines 1–303 and
  304–382.
- **Every cited row is in the union:** yes. The 27 handles the plan, the scope or this card cite are each a whole
  line of their master's rows file. None needed a `rec.py find`, and no standing list was needed.
- **`[RETYPED]`:**
  `obs-pick-p8 · [RETYPED] line 50: decision/run-worker-panic-ends-session is no row of the pack — read as decision/run-worker-panic-ends-run, the pack's row 50`.
  It is not a row the plan uses.
- **Re-spawns:** none. All 167 agents passed first try (7 task writers, 153 pickers, 7 history agents), with no dead
  draw.
- **Non-zero exits, refusals and verdict lines:**
  - The baseline runs of entries 6, 7 and 9 exited 4 (`error: no tests to run`), the predicted red.
  - Entry 11's control in the clone read red, last line 1, as it should.
  - One Bash call was refused by the bash guard: a `rec.py get` piped into `cut`. It was re-issued bare and ran.
  - planlint check 2 printed one HIT (`matrix#v2-19` in a wrap ledger note on an unverified cap), resolved by moving
    the residual to the claim's `--note-file`.
  - No `UNPARSED`, `INDETERMINATE` or `NOT DERIVED` line from any tool. `gate.py scope` read `parsed 8 items`, and
    `inputs.py verify` read `uncited 0 · unparsed 0`.
- **Where a letter was wrong or silent:**
  - The playbook's `rul-p2zl2kgl` asks a widening to halt in a dialog for a live answer. The directive (item 6) and
    the letters' away form send it to `founder-forks.md` instead. The letters say nothing on which wins; the
    directive was followed, and the widening stays the wrap's full escalation.
  - `validation.md` check 4 (9) asks a must-pass reading for a literal the verdict turns on. The plan's
    `contains 1 test run: 1 passed` was re-spelt `contains run: 1 passed` (and `run: 4 passed`) from nextest's own
    printed summary seen in the baseline logs (`117 tests run: 117 passed`). That form holds whatever nextest prints
    for one test, the singular form being unread.
  - The phase letter's Setup reads the evolve system from the phase copy; it is byte-identical to new-session's
    (`cmp`).

## Beyond the plan
1. **«Standing live rig with a screen model»** (an existing entry, its working-route title verbatim): a
   Carry-forward. Read live, on the interactive input box, exactly which whitespace the CLI trims at submit
   (leading, trailing, inside a paste) (`inputs#I2`).
2. **«Standing live rig with a screen model»**: owner of v2-19's residual. Measure the hook-latency window: a turn the
   CLI starts whose `UserPromptSubmit` has not reached the wrapper before a send's paste.
3. **The founder's word owed on Q1** (`.andromeda/runs/2026-11-16T04-10-47-phase/founder-forks.md`):
   - «trailing whitespace only» stands PROVISIONAL on the delegate's answer;
   - «either end» is held there as the candidate, the overseer leaning to it once row 1's reading lands.

Apply? (yes / review / cancel)
