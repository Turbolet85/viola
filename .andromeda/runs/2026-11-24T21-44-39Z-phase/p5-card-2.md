# P5 review, card 2 — 2026-11-24-adopt-s-resume-folder «Adopt's resume folder»

**Answers** card 1's `review`. The review is `inputs#I2` (`../additional/viola-overseer/relay-adopt-folder-p5-review.md`,
snapped `--step phase:P5`): the overseer's relay, holding the operator's own answers and the overseer's technical ones.

**Open the plan in an editor:** `.andromeda/cache/docs/2026-11-24-adopt-s-resume-folder/plan.md`, the render of
`doc:2026-11-24-adopt-s-resume-folder/plan` rev 4.
- No plan stands in the chunk's folder.
- Writing in that file changes no record until a `put`.
- `python -X utf8 ~/.claude/skills/andromeda-tools/scripts/rec.py diff --root . 2026-11-24-adopt-s-resume-folder`
  is a diff of the chunk's records.

## Goal (rev 4)
The folder did not decide the R5a reds. The evidence (`inputs#I2`):
- the ledger's Run E resumes a session from a sibling folder (M3);
- the operator resumed a session from a non-sibling folder by hand on 2.1.331;
- the overseer's one measurement firing of the rig's arm `a` read green, typed in a folder the session was not made in.

B and C can no longer be read apart (the scratch folder is gone), and nothing more is asked of him.

The entry builds adopt's not-resumed end. An adopt whose resumed child ends before any `session-start` reaches the
wrapper ends exit 1 with:
```
unable: <name>'s session did not resume
hint: check the session id - claude --resume <id> must open it by hand, under the same claude config
```
- Under `--json`: `{"v":1,"error":"not-resumed","detail":null}`, stderr empty.
- In the run log: one `process-exit{subject:"self", exit_code:1, detail:"not-resumed"}`.

The spawn folder stays the caller's: adopt needs no particular folder.

## The answers folded
- **Fork 2 — the operator's own answer:** «Да, печатать отказ (Recommended)». The pair, or the document, is written
  after the child has released the terminal, for adopt's not-resumed end only; `run` and `revive` are unchanged. The
  rows it departs from are Expected amendments written as his, citing `inputs#I2`.
- **The hint — the operator's own answer:** «Проверь id руками (Recommended)».
  - `<id>` is that literal word, so the session id is never printed.
  - **Dash form:** the tree's hints are ASCII-only (no em dash in `src/human/hints.rs`; the CLI rule reads «ASCII
    only»), so the hint uses ` - ` where the relay's text has `—`.
- **Fork 1, technical (the overseer, a delegate's):** the folder stays the caller's and nothing new is read. The
  wording «the operator types adopt in the session's folder» is gone from the plan.
- **Fork 3, technical (the overseer, a delegate's):** nothing kept by default; «under the same claude config» is all
  the hint says.

## What changed between revs
- **rev 1 → 2 (before card 1):** planlint's check-2 HIT fixed — a non-amendment bullet moved out of the Expected
  amendments list.
- **rev 2 → 3 (before card 1):** the six `baseline` keys written (entries 3–8).
- **rev 3 → 4 (this review, tx 4904):**
  1. The hint literal in step 2, the Goal and the cli acceptance criterion. Step 7's exit-table row reads «the step-2
     hint», so it follows.
  2. Fork 1's wording, in the Metadata, the Goal and the Constraints. The Constraints gain «a hint naming a folder —
     rejected by the operator's answer».
  3. The Goal's «B and C are asked of the operator» is replaced by `inputs#I2`'s measurements; nothing is asked.
  4. Step 8 gains one line: the live arm changes nothing in where it types.
  5. The Metadata's fork line and the Expected-amendments note now cite `inputs#I2`.

  No gate entry, touchpoint or test name changed, so the six baselines stand.

## Top acceptance criteria (unchanged but the hint text)
- A not-resumed adopt exits 1 with exactly the pair above, or under `--json` exactly the document, stderr empty
  (entry 5).
- Its run log holds the `not-resumed` self exit and the child's exit, schema-valid, with no session id (entry 5).
- A resumed adopt still ends exit 0. A name a not-resumed start left is taken over (entry 4, step 6).
- `not-resumed` is in the exit table with its own hint, and its writer is named (entry 4).
- The rehearsal prints `r5a … start=ok`, `r5b` unchanged, and `r5c: green not-resumed=yes started=child ended=yes`
  (entry 7, fake agent only).

## Touchpoints, test commands, capabilities — as card 1
- **Touchpoints:** nine files to modify, none new.
- **Gates:** 14 entries.
  - Baselines: 3 green (1401), 4 green (75), 6 green, 8 recorded (QUIET).
  - 5 red, vacuous until step 6. 7 red on `start=ok` until step 8.
  - One control: `not-resumed`, `--timeout 900`.
- **Capabilities:** no claim; serves the gated `matrix#v2-13`.

## The checks on rev 4 — verbatim where a tool printed it
- `gate.py run --dry-run`: `entries 14 · green 0 · red 0 · recorded 0 · timeout 0 · not-run 14`, exit 0 (the fence
  parses).
- `inputs.py verify`:
```
I1 unchanged · copy no-repo · ../additional/viola-overseer/phase-adopt-folder-directive.md
I2 unchanged · copy no-repo · ../additional/viola-overseer/relay-adopt-folder-p5-review.md
inputs: 2 entries — unchanged 2 · drifted 0 · vanished 0 · broken 0 · altered 0 · unreachable 0 · n/a 0 · uncited 0 · unparsed 0
```
- planlint, whole:
```
planlint v1.25 · 1ced29bd
fence: 14 entries · plan: doc:2026-11-24-adopt-s-resume-folder/plan · research: doc:2026-11-24-adopt-s-resume-folder/re…
check 9 · CI-outcome reads: 1 · leg operator exempt: 1 · Platform issues consulted: none · `none — no runner-only bull…`
check 9 · runner-only scope bullet: not read — the letter's
check 4 (5) · leg = 'live' entries: 0 · Harness rules consulted: none · `none — no live leg in this chunk (the one liv…`
check 4 (5) · leg entries with a content atom: 3 · note without `atom from`: 0
check 4 (13) · operator CI-outcome reads: 1 · not ci.py conclusion: 0
check 4 (13) · job-log reads by pattern: 0
check 4 (14) · base-less git change reads outside the operator and round legs: 0
check 4 (15) · grep → wc -l counts asserting exit 0 and last line 0: 0
check 4 (16) · guards whose exclusions leave a new file in: 0 of 0 with an exclusion
check 4 (18) · entries reading a master's or a base's path the store now holds: 0
check 4 (19) · absence probes against the test names their own plan adds: 0 · probes read 0
check 4 (17) · entries reading a path the wrap writes: 0 of 9 asserting — listed, P5's read
check 4 (17) · a path the wrap writes, read from inside a program an entry runs (a suite, the ledger's tool): not read
check 2 · ledger-note entries (matrix#): 0
check 2 · entries naming no master and no matrix#: 0
check 5 · placeholders outside code: 0
check 12 (c) · step references the plan's steps lack: 0 of 4
check 12 (d) · «unchanged» claims on a name another part of the plan names: 6 of 7 — listed, P5's read
  line 60 · «unchanged» · run (step 3) · pump_child (step 3)
  line 86 · «unchanged» · src/cmd/run.rs (step 1)
  line 115 · «unchanged» · process-exit (step 1) · ok (Test Commands)
  line 128 · «unchanged» · src/cmd/run.rs (step 1) · pump_child (step 1)
  line 289 · «unchanged» · r5b (Test Commands)
  line 291 · «unchanged» · run (step 1) · revive (step 1) · pump_child (step 1)
  a claim on a name no other step, Files to modify or Test Commands names is not listed — P5's read
check 4 (4) · criterion evidence paths the plan names nowhere else: 0 of 0
hits: 0 — resolved by its check (validation.md 2 · 5 · 9 · 4 (4), (5), (13)–(16), (18)–(19) · 12 (c)); 0 is not a pass
```
  - 12 (d) is read: every «unchanged» claim is on behaviour (`pump_child`'s mapping for `run` and `revive`, the `r5b`
    line), none on a file the plan changes.
  - Rev 3's «unchanged» line on the live arms is now step 8's «nothing changes in where it types».
- `gate.py scope --lists-only`: `scope lists: parsed 9 items (new 0 · modify 9 · derived 0)`, exit 0.
- **Validation-1:** aligned. The scope's corrected premises (A falsified, the gap verified) carry into the plan, and
  `inputs#I2` closes the open causes.

## Non-zero exits and refusals since card 1
- none. The relay's snap exited 0, and rev 4's `put` and the four checks exited 0.
- From before card 1, unchanged:
  - `rec.py put --edit-file` exit 2 at the scope closure.
  - The Bash guard's two `cd` refusals and one pipe-filter refusal, each re-issued in a passing form.
  - `gate.py baseline` backgrounded past 120 s.
  - The two `[RETYPED]` picker lines.
  - The 50 `not a marker` history rows.

## Beyond the plan
- none

**Apply? (yes / review / cancel)**
