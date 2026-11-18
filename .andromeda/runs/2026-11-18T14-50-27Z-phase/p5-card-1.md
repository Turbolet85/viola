# P5 review card 1 — 2026-11-18-browser-tools-prompt-modal «Browser-tools prompt read as a modal»

Written 2026-11-18T15:57:56Z by /andromeda-phase P5, in run dir `.andromeda/runs/2026-11-18T14-50-27Z-phase`. The
operator is away, so the overseer carries this card. Its word is taken as text: `yes` / `review` / `cancel`.

The plan to open in an editor is `.andromeda/cache/docs/2026-11-18-browser-tools-prompt-modal/plan.md`, the render of
`doc:2026-11-18-browser-tools-prompt-modal/plan` rev 2. No plan stands in the chunk's folder. Writing in that file
changes no record until a `rec.py put`. `python -X utf8 ~/.claude/skills/andromeda-tools/scripts/rec.py diff --root .
2026-11-18-browser-tools-prompt-modal` shows a diff of the chunk's records.

## Goal
- Compile the 2.1.325 browser-tools offer's confirm label, `Yes, use my browser`, as the third modal literal. The
  label stands twice in the install file, the same count as the trust literal, measured at P4. With it:
  - verify settles on that screen at once and pastes nothing;
  - `run`'s pre-send gate refuses `input-not-ready` at once instead of waiting 8.5 s;
  - nothing is ever typed into the screen.
- Add the 24th relied row, `browser-tools-prompt`, stamped `pass` · `absent` · `fail`, per the founder's Q0 answer
  (`inputs#I3` item 1). `absent` counts toward verified, on this row alone. A run whose first screen reads `none`
  makes the row `fail`, never `absent`: a reworded prompt stays honest.
- Prove it on CI over the fake agent. A new `--start-screen <phase>` option makes the fake draw a test-written
  synthetic screen. `cli_verify` is in the gate block.
- Prove it live by the founder's cap: `BP` (the prompt must stand), then `BK` (its control), plus the spare `BP-2`.
  Keys: none. The stop is a no-key group kill.

## Top acceptance criteria
- The four readings of the row are pinned: pass, absent, fail, and `absent` on any other row read as unverified. The
  `bt-absent` and `bt-none` controls hold.
- `modal-signature` reads only the trust and imports literals, held by the `bt-modalsig` control.
- The gate refuses on its first verdict, held by the `bt-literal` control.
- `viola send` exits 13 with `input-not-ready` over the fake standing on the prompt. The fake's receipt holds no key
  and no prompt line, so NOTHING was sent.
- Verify over the fake on the prompt stamps `pass` and pastes nothing. The plain fake's 24th step line ends `absent`,
  and its summary reads `stamped 2.1.0  24 pass  0 fail`.
- The version gate reads a 23-id stamp as unverified, `absent` as verified and `fail` as unverified.
- The `vp` and `vk` rehearsals pass in CI.
- `BP` GREEN: `browser-tools-prompt: pass`, no `screen-*: none`, `keys: none`, the stop not Ctrl-C, every teardown
  word.
- `BK` GREEN: `failing: none`, `rows: 24`, `pass-count: 24/24`, B–F `box`, `browser-tools-prompt: absent`, every
  teardown word.
- A RED stops implement before the operator pass.
- CI green on the pre-CI commit.

## Touchpoints
- 24 files to modify, no new file (`## Codebase touchpoints`):
  - the agent crate: `screen.rs`, `ledger.rs` and three `test_cases*`;
  - the root bin: `verify.rs`, `typed/{pty,turns,run}.rs`, `run/{version_gate,gate}.rs`;
  - the fake agent;
  - tests: `cli_send`, `cli_verify` (with its `runs` and `stamps`), `cli_version_gate`, `contract_ledger_probes`,
    `support/home.rs`;
  - viola-e2e: `harness/run.rs`, `rig/{mod,isolate,screen}.rs`, `tests/rig_rehearsal.rs`.
- No committed fixture.
- `Arm::V`'s Ctrl-C stop is untouched. The new arms `VP` / `VK` stop with no key.
- `cache-a` is read by key NAME at verify's end.

## Test commands
21 entries:
- 1–2: fmt, clippy;
- 3: the agent crate;
- 4: `run --unit`;
- 5: the five changed integration binaries;
- 6–7: `run --integration`, `run --e2e`;
- 8–14: the live round — the install by path, the census, the build hash, `BP --cap 1 --spare 1`, the census,
  `BK --cap 2 --spare 1`, the census, one ledger `evidence/prompt-starts.ndjson`;
- 15–21: the operator pass — the `screen-smoke` boot, status and cleanup, pre-push, hygiene, the clean-tree push and
  `ci.py --wait 1800`.

Four one-shot controls are named with their `--timeout`: `bt-literal` 900 s, and `bt-absent`, `bt-none` and
`bt-modalsig` 600 s each. There is no mutation run and no release check (only test-only code changes).

## Capabilities claimed
None. The entry claims no new requirement (CARRY `cry-vlr2pc4f`), and the pool of 18 planned caps holds none it
builds. There is no concretization preview.

## Decisive leans (confirm-class; each in the plan)
1. The literal is the confirm label, as the two standing modal literals are. The cancel words also stand in the footer
   (4 hits).
2. `none` on any first screen makes the row `fail`.
3. Verify's summary keeps `n pass  m fail`, where `n` counts the rows that count toward verified, `absent` among them.
   The step line itself prints `absent`, and the stamp holds `absent`, never `pass`.
4. `cache-a` is read once, at verify's end: the rig cannot pause between runs. With `screen-b` it says whether Run A
   armed the prompt.
5. `FLOOR_ROWS` stays at 21.
6. A RED `BP` stops the round, so `BK` is not fired after it. A spare or a re-fire waits on a plan revision.

## Expected amendments the wrap meets (escalations named)
- **Answer card, on the founder's own answer:** `absent` counts toward verified. This touches architecture ·
  `decision/version-verified-all-relied-rows` and test-plan · `decision/version-gate-needs-all-relied-rows`
  (`inputs#I3` item 1, relayed verbatim).
- **Plain boundary-widening escalation (`rul-r74hshif` / `rul-p2zl2kgl`):** the test-only rig reads the key NAME
  `cachedChromeExtensionInstalled` in its own private Claude config. This touches architecture ·
  `decision/claude-first-no-adapter-trait` except and its twin in CLAUDE.md. The answer allowing it is the overseer's
  technical answer 5 (delegate overseer), not the founder's, so under `rul-bznresq5` it is no card item. It halts at
  the wrap for the founder's live answer.
- The rest are routine:
  - the step and summary lines in four masters;
  - the stamp words and `first-screen`;
  - the third literal;
  - the fake's option;
  - the rig's arms;
  - the 23 → 24 counts.

## The directive's verbatim block (item 12)
- The union, as `rec.py rows-union` printed it (2026-11-18T15:18:45Z, exit 0):
  ```
  architecture: rows 966 · packs 10 · picked 170 (107 by every draw) · standing 38 · 51104 B
  security-plan: rows 614 · packs 7 · picked 121 (91 by every draw) · standing 10 · 36871 B
  design-system: rows 316 · packs 4 · picked 82 (68 by every draw) · standing 25 · 20012 B
  layout-templates: rows 222 · packs 3 · picked 42 (27 by every draw) · standing 2 · 9578 B
  test-plan: rows 1346 · packs 14 · picked 260 (147 by every draw) · standing 36 · 66229 B
  obs-plan: rows 1027 · packs 11 · picked 198 (130 by every draw) · standing 45 · 49891 B
  a11y-plan: rows 235 · packs 3 · picked 76 (54 by every draw) · standing 20 · 20339 B
  rows-union: 7 master(s) · picked 949 of 4726 · rule any
  ```
- Rows were read BY SEARCH, never whole. The sum is 254 024 B, past the 250 000 B bound.
- Every row the plan cites is in the union or its standing list. A per-handle grep over the seven `-rows.md` and
  `-standing.md` files printed no `NOT IN UNION` line.
- `[RETYPED]` lines (`fanout.py check`, verbatim):
  ```
  design-pick-p1.d3 · [RETYPED] line 58: contract/cli-separators/field-separator is no row of the pack — read as contract/cli-glyphs/field-separator, the pack's row 58
  tests-pick-p9 · [RETYPED] line 30: decision/synthetic-fixture-text-only is no row of the pack — read as decision/synthetic-fixture-text, the pack's row 30
  tests-pick-p9.d2 · [RETYPED] line 68: decision/synthetic-fixture-text-only is no row of the pack — read as decision/synthetic-fixture-text, the pack's row 68
  tests-pick-p9.d2 · [RETYPED] line 88: decision/fixture-hygiene-run-time-walk is no row of the pack — read as decision/fixture-hygiene-walk, the pack's row 88
  obs-pick-p8 · [RETYPED] line 50: decision/run-worker-panic-ends-session is no row of the pack — read as decision/run-worker-panic-ends-run, the pack's row 50
  obs-pick-p8.d2 · [RETYPED] line 39: decision/claude-agents-rows-never-logged-verbatim is no row of the pack — read as decision/claude-agents-rows-never-logged, the pack's row 39
  ```
- Re-spawns: `task · 6 of 7 pass · failed: layouts-task [NOT_NEEDED_REACHED]`, then `task-r1 · 1 of 1 pass`. The
  census printed `re-spawned 1 agent(s) · NOT_NEEDED_REACHED 1 · wall 45 s` and
  `respawn-kept 98 % · 57 of 58 predecessor lines, 1 strong re-spawn file(s)`. No draw was dead (`draws-dead 0`).
- Non-zero exits, refusals and `UNPARSED` / `INDETERMINATE` / `NOT DERIVED` lines in this window:
  - none from any tool;
  - the bash guard refused three calls that `cd`'d into a run or reference dir. Each was re-issued once with absolute
    paths.
- Every place a letter was wrong or silent:
  - Research said the rig offers no no-key stop. In fact `Session::stop_no_key` exists (`rig/mod.rs:643`, used by
    `IM` / `VC` / `TC`). Corrected in the plan.
  - The census atom source `viola-live-rig.rs:161` from the prior plan is stale. The print is at `:165`; corrected.
  - The founder-forks text said `cache-a` reads «after Run A». The rig can only read at verify's end; the plan names it
    so (lean 4).
  - The directive is silent on how verify's summary counts `absent` (lean 3).
- Baselines on the untouched tree, each green:

  | entry | baseline |
  |---|---|
  | 3 | 628 passed |
  | 4 | nextest-unit 2740 |
  | 5 | 113 |
  | 6 | nextest-integration 547 |
  | 7 | nextest-e2e 321 |

- Checks:
  - planlint: `hits: 0`;
  - `scope lists: parsed 19 items`;
  - `inputs: 4 entries — unchanged 2 · … n/a 2 · uncited 0 · unparsed 0`;
  - check 12 (d): 4 claims listed, each read and holding;
  - check 11 (b): the prior rig chunk's report records no host harm;
  - **size WARN**: the plan prose is about 440 lines, past the ~400 guide.

## Beyond the plan
none

Apply? (yes / review / cancel)
