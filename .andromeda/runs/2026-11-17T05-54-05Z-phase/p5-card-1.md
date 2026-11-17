# P5 card 1 — 2026-11-17-standing-live-rig «Standing live rig with a screen model»

**Plan:** `doc:2026-11-17-standing-live-rig/plan` rev 6, rendered for reading at
`.andromeda/cache/docs/2026-11-17-standing-live-rig/plan.md` (364 lines). No plan stands in a chunk's folder; writing in
that file changes no record until a `put`. `python -X utf8 ~/.claude/skills/andromeda-tools/scripts/rec.py diff --root .
2026-11-17-standing-live-rig` is a diff of the chunk's records.

## Goal
One committed test-only rig, `viola-live-rig` in `viola-e2e`, that every later live round reuses:
- a vt100 screen model of its own, with readiness through the product's `input_box_up`;
- trust and input-box reads, kept as closed words;
- kept screens under the gitignored `target/live-rig/`;
- the founder's granted keys only.

It owns its isolation: a `/tmp` scratch root, a private config dir, a 0600 credentials copy behind the 20-minute
token check and removed on every exit, and `~/.claude.json` read before and after each start. It owns its processes:
its own group, a watchdog, and a `/proc` census that must read 0 at every exit.

The fake agent gains DEL erase and a trust selector. Live: R1 (trust + DEL) and R2 (whitespace trim at submit), cap
2 + 1 spare (inputs#I2).

## Top acceptance criteria (12 in the plan)
- A1/A2 — the rehearsal reads the trust dialog passed on a read selected row, and the box `empty` after `x` + DEL.
- A4/A8 — on every exit path (pass, refusal, child exit 1, SIGINT, SIGTERM, watchdog) no credentials copy, no scratch
  root, no marked process; a hook outliving the fake agent reads `census: 1 killed`.
- A5 — 19 min 59 s of token refuses, uncounted; 20 min 1 s starts.
- A6 + entry 9 — no credential, host path or config value in any rig output; no tracked file outside `.andromeda/`
  but the rehearsal source holds the canary token (inputs#I2 item 4).
- A9 — `live` refuses under `CI`; `spent` and `cap` refusals uncounted.
- A10/A11 — R1 and R2 on claude 2.1.325, census 0, credentials removed, scratch absent; R1's words equal the
  rehearsal's.

## Touchpoints
- New: `crates/viola-e2e/src/bin/viola-live-rig.rs`, `crates/viola-e2e/src/rig/`, `crates/viola-e2e/tests/rig_rehearsal.rs`.
- Modify: `crates/viola-e2e/Cargo.toml`, `crates/viola-e2e/src/lib.rs`, `src/bin/viola-fake-agent/input.rs`,
  `src/bin/viola-fake-agent.rs`.

## Test commands (16 entries)
- **Lint, tests and probes:** fmt, clippy, the rig unit cases (`-E 'test(/^rig::/)'`), `rig_rehearsal`,
  `cli_fake_agent`, `contract_fake_agent_drift`, release-check, `cargo deny`, and the canary probe.
- **Live round:** the round probe (2.1.325 by path), then the live legs R1 and R2. Each leg's stop is
  `viola-live-rig census --kill`.
- **Operator pass:** pre-push, hygiene, the clean-tree push, and the `ci.py` read.
- **Controls:** five one-shot remove-the-guard controls at `--timeout 60`: `rig-census`, `rig-cred-rm`, `rig-token`,
  `rig-keygrant`, `fake-del`.
- **No mutation run.**

## Capability — concretization preview (never-weaken checkpoint)
**matrix#v2-32** (e2e), the one claim.
- Current: «A live round runs on the standing rig with no rig written for it; its kept screens show the CLI's trust
  and modal dialogs read as the live CLI drew them, and a rehearsal on the fake agent reads the same erase and dialog
  screens the live run reads.»
- Proposed: «R1 and R2 on claude 2.1.325 are fired by `viola-live-rig live` alone, with no script written into the
  chunk's evidence. R1's kept frames under `target/live-rig/` and its summary read the trust dialog (`trust`,
  `selected`, `options`) and the box erase (`box-text: empty` after `x` + DEL) as the live CLI drew them. The fake
  agent's rehearsal of the same arms (`rig_rehearsal` A1, A2) reads the same four words.»
- **The narrowing to look at:** «trust and modal dialogs» becomes the trust dialog alone. The external-imports modal
  is read by the model's `modal` word in rehearsal only; no live start is shaped to raise it, because the founder's
  cap has no start for it. If that reads as weakening, the alternative is to claim nothing and leave v2-32 pooled with
  a dated note. My recommendation is to claim it, with a notes line naming the imports modal as unread live.

## Directive item 10 — the record
Union lines, as printed:
```
architecture: rows 948 · packs 10 · picked 251 (124 by every draw) · standing 14 · 72645 B
security-plan: rows 604 · packs 7 · picked 152 (94 by every draw) · standing 12 · 42082 B
design-system: rows 315 · packs 4 · picked 89 (41 by every draw) · standing 0 · 21280 B
layout-templates: rows 222 · packs 3 · picked 76 (50 by every draw) · standing 0 · 15738 B
test-plan: rows 1333 · packs 14 · picked 314 (156 by every draw) · standing 25 · 79293 B
obs-plan: rows 1024 · packs 11 · picked 313 (167 by every draw) · standing 21 · 76210 B
a11y-plan: rows 235 · packs 3 · picked 61 (41 by every draw) · standing 14 · 17285 B
rows-union: 7 master(s) · picked 1256 of 4681 · rule any
```

**How the rows were read.** By search, never whole: the union sums to 324 533 B, past the 250 000 B line. The 11
cited rows are each found in a rows file: 4 in security-rows, 4 in tests-rows, 3 in arch-rows. None was looked up
outside the union.

**Retyped handles and re-spawns.** `[RETYPED]` lines, as printed:
- `arch-pick-p4.d2 · [RETYPED] line 88: decision/anyhow-only-in-root-bin-crate is no row of the pack — read as decision/anyhow-only-in-root-bin, the pack's row 88`
- `tests-pick-p9 · [RETYPED] line 72: decision/synthetic-fixture-text-only is no row of the pack — read as decision/synthetic-fixture-text, the pack's row 72`
- `tests-pick-p9.d3 · [RETYPED] line 12: decision/synthetic-fixture-text-only is no row of the pack — read as decision/synthetic-fixture-text, the pack's row 12`

There were 0 re-spawns and 0 dead draws. Five history files hold `not a marker` rows for
`2026-11-17T05-21-35Z-wrap`, an entry no chunk made.

**Non-zero exits, refusals and printed verdicts.**
- Bash-guard refusals, each re-issued once:
  - two `cd`s into a run dir, re-issued as subshells;
  - one tool output piped into a filter, re-issued bare.
- The stamp-ahead hook refused the plan's estimated `Generated:` stamp (+4 min). It was re-written from `date -u`. Its
  line: `Stamp ahead of the clock: 2026-11-17T06:45:00Z (+4 min) … never estimate a time.`
- Baseline reds, each read for its cause:
  - entry 3 (first form) matched an unrelated test whose name holds `rig`; re-spelt `test(/^rig::/)`;
  - entry 4 (first form, `--e2e`) read `error: operator didn't match any binary names`: `rig_rehearsal` is no
    E2E-prefixed binary, so it was re-spelt `--integration`;
  - the re-spelt 3 and 4 read red on their absent inputs (nextest exit 4; `"ok":false`), as expected before implement.
- No `UNPARSED`, `INDETERMINATE` or `NOT DERIVED` line was printed.

**Where a letter was wrong or silent.**
- The Grep tool is not loaded in this session, so searches ran as bash `grep`.
- The tracked store journal would carry this plan's own canary token, so entry 9 excludes `.andromeda/`. Its
  known-positive controls are in `controls-entry9.md`: a tracked canary file reads red; the rehearsal source alone
  and `.andromeda/` read green.
- One validation-1 intent gap was found and closed: the scope was narrowed to the founder's split (scope rev 4,
  citing inputs#I2).
- No scheduled wake or timer was set, and no hook refused one.

## Beyond the plan
| # | proposal | authority |
|---|---|---|
| 1 | New route entry «first readings on the standing rig», placed directly after row 2's entry: R3 (the fired wake's `source`, «a real wake reads 1», the hook-latency window) and R4 (the live `viola verify --record`). The CARRYs cry-skycqoql, cry-iugxdgc4 and cry-mqvgnhlo move to it from this entry. | the founder's own answer, inputs#I2 item 1 |
| 2 | New route entry, second in Epoch 4 (right after this one), WHAT-only: a wrapper a test boots does not outlive that test, killed or not. Its ground is the five mutant-build wrappers at about 99 % of a core for 5 h 50 min on 2026-11-17. Its requirement row comes by the wrap's dialogue. | the founder's own answer, inputs#I2 item 3 |

Apply? (yes / review / cancel)
