# P5 review card 1 — «The 2.1.328 recorded fixture set» (chunk 2026-11-19-the-2-1-328-recorded-fixture-set)

**Plan:** `doc:2026-11-19-the-2-1-328-recorded-fixture-set/plan` rev 2. To open it in an editor, use its render
`.andromeda/cache/docs/2026-11-19-the-2-1-328-recorded-fixture-set/plan.md`.
- No plan stands in the chunk's folder.
- Writing in that file changes no record until a `put`.
- `python -X utf8 ~/.claude/skills/andromeda-tools/scripts/rec.py diff --root . 2026-11-19-the-2-1-328-recorded-fixture-set`
  diffs the chunk's records.

**Inputs:** `inputs#I1` (the overseer's phase directive) · `inputs#I2` (the P3-block relay: the founder's
«3 + 1 запасной (Recommended)», read 10:09:58Z, and the overseer's answers 3–8).

## Goal
Record claude 2.1.328 as the 2.1.325 set was recorded:
- `R4`: one clean `viola verify --record`;
- `R4M`: one `claude -p` through `viola-mcp-tap`.

Both run by path, under the rig's PRIVATE config. The output lands whole, as a plain copy, in
`fixtures/claude/2.1.328/`. `STAMPED` gains `"2.1.328"`, and the drift, hygiene, ledger-probe and MCP-exchange suites
read the new set.

C3 is taken in as F2 option A. It is a test-only mend of the rig's `CX` arm: the session transcript is looked up
again after the `/context` send, and a transcript found then is counted from its start. Then one `L2` start.

## What 2.1.328 changes: T1–T3, read statically at P4 (no start)
- **T1 holds.**
  - The trust dialog still reads `confirmLabel:"Yes, I trust this folder"` and `cancelLabel … "No, exit"`, so the
    fake's `TRUST_OPTIONS` stands.
  - The imports and browser-tools labels are unchanged (2 → 2 hits each), and so is the `for agents` footer.
  - The new decline text is a message shown after declining, never a dialog row.
  - The one new `No, exit` is a cancel label of a different dialog.
- **T2 holds:** the migration gate is `migrationVersion===et`, with `et=15`, the same as 2.1.325's `Ge=15`. The rig's
  seed `Migration(15)` (isolate.rs:684) still skips the migrations.
- **T3 holds as far as a static read reaches:** `server/discover` has 79 hits in both bundles. The live opening is read
  by `mcp_recorded_exchange` after the landing.
- No fork: nothing changed that a compiled literal, the seed or the tap's `fixture` depends on.

## Top acceptance criteria
- `fixtures/claude/2.1.328/` is a byte copy of `R4`'s record and `R4M`'s tap fixture: the `diff -r` and `cmp`
  entries print nothing.
- The hygiene, drift, `STAMPED` 24-pass probes and MCP-exchange suites are green over every set, 2.1.328 included.
- The resume variant is recorded (`screen-e: box`, `SessionStart.resume-1.json`).
- The `CX` mend is held by two inline unit cases and the `cx-relook` control. The `cx` rehearsal reads
  `context-transcript-after: absent` over the fake.
- `R4` and `R4M` read green within the founder's cap.
- `L2`'s readings are recorded as read. `context-transcript-after: absent` is a recorded red that does not stop the
  chunk (`inputs#I2` items 2, 6).
- The whole integration and E2E layers, pre-push and CI read green.
- **No capability is claimed.** None of the 18 pooled caps names a fixture set, so there is no concretization preview.

## Touchpoints
- New: `fixtures/claude/2.1.328/`.
- Modify:
  - `crates/viola-e2e/src/rig/mod.rs`: `transcript_after_send`, `arm_cx`'s one call and its
    `context-transcript-after` word, and two unit cases;
  - `crates/viola-e2e/tests/rig_rehearsal.rs`: the `cx` rehearsal;
  - `tests/contract_ledger_probes.rs`: `STAMPED`.
- The product graph is untouched.

## Live starts (the founder's cap, `inputs#I2` item 1)
One ledger file, `.andromeda/cache/chunks/2026-11-19-the-2-1-328-recorded-fixture-set/evidence/fixture-starts.ndjson`.
- **Order and flags:** `R4 --cap 1 --spare 1` → census → `R4M --cap 2` → census → `L2 --cap 3` → census. The last
  census is keyed `operator`, so a resumed round still fires it.
- **Before the round:** a hand read of `hostwatch.py read --last 15`. A CONTENDED reading that names a neighbour
  stops; this repository's own stall is no neighbour.
- **On every start:** the atom `lacks user-config: unread` STAYS, its reason in the note (`inputs#I2` item 4).
- **Spare:** `R4-2` only on a cause `R4` itself showed, after a revision. `R4M` and `L2` have no spare.

## Test commands
25 entries. The dry-run parses and exits 0. `gate.py baseline` read: seen green 5 · not seen 0 · new (key) 7 · leg 13.

Baselines written:
- 3: green, 316 passed;
- 5: green, 27 passed;
- 15, 16, 17: red, inputs absent, the subject this chunk builds;
- 18: green, 56 passed;
- 19: green, 127 passed.

`cx-relook` is the remove-the-guard control: a one-shot run at implement on entry 3, `--timeout 600`.

## Checks
| check | reading |
|---|---|
| planlint | 0 hits |
| 12 (d) | the 3 «unchanged» claims read true: the product graph, and `CX`'s other keys by name |
| research lists | `scope lists: parsed 4 items (new 1 · modify 3 · derived 0)` |
| inputs | 2 unchanged |
| 4 (7) | `target/live-rig/…` and the ledger file read ignored |
| 11 (b) | the 2.1.325 report records no harm to the host: the rig kills only its own group |
| 6 (size) | 290 prose lines |
| Validation-1 | aligned with the scope's C3 premise closure: the mended `CX` start, taken on the founder's word |

## Block (directive item 11, verbatim where a tool printed it)
**The union** (P2, `rows-union` trail):
```
architecture: rows 967 · packs 10 · picked 249 (115 by every draw) · standing 1 · 71020 B
security-plan: rows 617 · packs 7 · picked 143 (87 by every draw) · standing 3 · 42734 B
design-system: rows 317 · packs 4 · picked 87 (69 by every draw) · standing 0 · 20176 B
layout-templates: rows 222 · packs 3 · picked 67 (45 by every draw) · standing 0 · 13991 B
test-plan: rows 1347 · packs 14 · picked 284 (189 by every draw) · standing 17 · 71309 B
obs-plan: rows 1028 · packs 11 · picked 479 (293 by every draw) · standing 1 · 110025 B
a11y-plan: rows 235 · packs 3 · picked 77 (59 by every draw) · standing 11 · 20963 B
rows-union: 7 master(s) · picked 1386 of 4733 · rule any
```

**Rows read by search** (sum 350218 B, over the 250000 B line). Every cited row is in its rows file:
- tests: `decision/replay-every-cli-version-fixture`, `decision/ledger-probe-contract-suite`,
  `decision/live-failing-reading-not-verification` and `contract/fake-agent-screens/trust-dialog`, 1 hit each;
- security: `decision/live-rig-context-arm-keeps-no-transcript`, 1 hit.

**[RETYPED]** (P2 pick checks):
```
arch-pick-p4 · [RETYPED] line 16: decision/fake-agent-test-seam is no row of the pack — read as decision/fake-agent-test-seams, the pack's row 16
tests-pick-p6 · [RETYPED] line 49: decision/hook-fails-open-silently is no row of the pack — read as decision/hook-fails-open, the pack's row 49
tests-pick-p9 · [RETYPED] line 1: decision/ci-homes-kept-until-gate-steps is no row of the pack — read as decision/ci-homes-kept-until-gates, the pack's row 1
```

**Re-spawns** (P2 history batch):
```
layouts-history · [CITES] UNRESOLVED: 2026-10-16-resume-and-compact-ledger-rows — history:28
a11y-history · [CITES] UNRESOLVED: 2026-11-13-interrupted-verify-cleanup — history:31
VERDICT: 5 of 7 pass · failed: layouts-history [CITES]; a11y-history [CITES]
batch history-r1 · 2 agents · 2 with every named path · …
VERDICT: 2 of 2 pass · failed: none
re-spawned 2 agent(s) · CITES 2 · wall 97 s
```

**Non-zero exits and refusals in this window:**
- **Bash guard, twice:** it refused a `cd` into a references dir and into the run dir. Each call was re-issued once
  with absolute paths.
- **Baseline reds:** entries 15 and 16 exit 2, entry 17 exits 1; all expected, on absent inputs.

No `UNPARSED`, `INDETERMINATE` or `NOT DERIVED` line in this window.

**Where a letter was wrong or silent:**
1. **Resume rung (c) against P4's rows read.** Setup's resume rung (c) says the rows files are "read there once,
   whole". P4 says that past 250000 B they are read by search. I read them by search, so the P4 rule governed.
2. **The `user-config` atom.** Relay item 4 keeps `lacks user-config: unread`, against verification-harness.md
   2026-11-19 (no atom on a nothing-to-read word). The relay governs, and its reason stands beside the atom on every
   start.
3. **The host read's form.** No letter says what form the host read takes. It is a hand step in the plan (step 7), not
   an entry: the reader lives outside the repository and its verdict words are not pinned.
4. **History file markers.** The history-batch check printed `not a marker: **amd-…**` on every tests-history item.
   The file names amendment ids, not chunk markers, and the check passed it all the same (5 of 7, tests not among the
   failed).
5. **Missed decline text.** My first T1 byte search for the decline text used a straight apostrophe and found nothing,
   because the bundle uses a curly one. The text was read from the trust dialog's surrounding bytes instead.

## Beyond the plan
1. **(conditional) The primed two-send `CX` start.** Proposed only if `L2` reads `context-transcript-after: absent`.
   It sends `reply with the single word ok`, waits for `turn-ended`, then sends `/context`. It is a carry proposal for
   the wrap's route adaptation, owner named there, and its start is a further cap of the founder's (`inputs#I2` item 2).
2. **(conditional) Listing `/context` as a local command.** A product change to `LOCAL_COMMANDS`, proposed only if
   `L2`'s reading supports it. It is a carry proposal for the wrap, with its owner named there (`inputs#I2` item 6).

Apply? (yes / review / cancel)
