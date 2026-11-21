# P5 review — revision 1 of «Harness-prefix and identity-floor ledger rows»

Marker `2026-11-20-harness-prefix-and-identity-floor-ledger-rows` · viola-0.2.0 · revision run
`.andromeda/runs/2026-11-21T07-47-46Z-phase` · plan rev 10 (tx 3577) · scope rev 3 · research rev 3.

**Open the plan** at `.andromeda/cache/docs/2026-11-20-harness-prefix-and-identity-floor-ledger-rows/plan.md` (rendered
from the store, byte-equal). No plan stands in the chunk's folder; writing in that file changes no record until a
`put`. `python -X utf8 ~/.claude/skills/andromeda-tools/scripts/rec.py diff --root . 2026-11-20-harness-prefix-and-identity-floor-ledger-rows`
shows a diff of the chunk's records.

## Goal
The `identity-floor` row lands ALONE (v2-26): every `CLAUDE*` name the CLI adds to a hook child is on the R8 identity
floor (now thirteen names) or the compiled non-identity list (now four), read as NAMES only. Relied set 27 → 28. The
cross-session row leaves this chunk whole for an entry of its own (`inputs#I10` items 1, 5).

## The cut (R1–R6) — nothing kept behind a flag
- **Seven files back to their `b3eb184c` text**, read by the guard entry 3 (`git diff --quiet b3eb184c -- …` → `cut:
  whole`): `crates/viola-agent-claude/src/hook.rs` · `src/cmd/verify/typed.rs` · `src/cmd/verify/typed/runs.rs` ·
  `src/cmd/verify/typed/turns.rs` · `src/bin/viola-fake-agent/agents.rs` · `schemas/diag-line.v1.json` ·
  `tests/contract_diag_schema.rs`. Brought back by edits, never `git checkout --`.
- Removed from the rest: the `CrossSessionPrefix` row and its constants/types/readings (`ledger.rs`, its test files),
  `ALLOWED_TOOLS_FLAG`/`SEND_TOOL` (`lib.rs`), `SENDER_DEADLINE`/`sender_cwd` (`verify.rs`), the fake agent's sender
  and receiver, the three cross-session `cli_verify` tests, every `verify-sender` pair, the `cross-session` hygiene
  stem, the rig's cross lines. Entry 4's name census must read exit 1, no output.
- Stays: phase A whole; phase B's identity half (`env_reading`, `ENV_RECORDED`, the Env replay, `write_env_set`, the two
  `identity_floor_*` cases, the canary test, the rig's `HF`/`HF-2`/`HG` labels).

## The floor (F1–F2) and the counts (C1)
- `IDENTITY_FLOOR: [&str; 13]` (+ `CLAUDE_CODE_CHILD_NOTES_DIR`, `CLAUDE_CODE_LAUNCHED_BY_SESSION_IDS`, the sibling on
  the bundle's reading alone); `NON_IDENTITY_HOOK_NAMES: [&str; 4]` (+ `CLAUDE_PLUGIN_DATA`); both pinned against
  literal copies; `tui_env_strip` strips the thirteen with two new canaries joining `CRITICAL`.
- **Every relied-count literal at 28**: `[NN/28]` step lines, `28 pass  0 fail`, `ROW_IDS` 28, `LEDGER_ROWS`/`LIVE_PASS`,
  `rows` 28; hook count 46 → 44, Stop count 11 → 10, role indices back to `b3eb184c`; `FLOOR_ROWS` 24.

## The round — new order, the whole-round command fires
`gate.py run --chunk … --run-dir {implement run dir} --live-legs`; the span holds only `round`/`live` entries, so it
fires whole, in this order:
1. the 2.1.328 install read by path (entry 13);
2. the census (14);
3. the plain build + hash (15);
4. `HG`'s record and kept stamp read absent (16);
5. **the live home's stamp sha RE-TAKEN right before `HG`** into its own file `evidence/live-stamp-HG.sha256`, written
   fresh at each firing (17) — `HF`'s `evidence/live-stamp.sha256` stays in the evidence as taken;
6. **`HG`** (18): `viola-live-rig -- live HG --cli 2.1.328 --deadline 1500 --cap 2 --spare 1 --ledger
   …/evidence/floor-starts.ndjson` — the SAME ledger file, its first line `HF`'s, so `HG` is base line 2; `HF-2`
   unspent unless a rig refusal before the ledger line or a CONTENDED neighbour;
7. the two name diffs (19, 20), valid only before the landing.

Then by `--operator`/`--entry`: the census (21), the kept stamp at `28 ok` (22), the env names report-only (23),
`live-stamp: unmoved` **against the HG reading** (24). The runtime-dir entry left with the sender.

**Why the stamp reading moved** (`inputs#I11` item 1): on the founder's word the overseer ran ONE `viola verify` with
the pinned product for PATH's claude 2.1.330 (07:23:25Z–07:24:08Z, `stamped 2.1.330  24 pass  0 fail`), which added
that version's entry to `~/.viola/ledger/stamps.json`. Nothing of this chunk wrote it. The driver session now runs
2.1.330; `HG` still runs 2.1.328 by path.

## Top acceptance criteria
- (identity) `matrix#v2-26` — `HG` live on 2.1.328: `identity-floor: pass`, `env-unknown: 0`, `pass-count: 28/28`,
  kept stamp `28 ok`, every `env_probe.added` name on the thirteen or the four; `security_negatives_env_capture` green;
  after the landing `contract_ledger_probes` reads 2.1.328 at `28 pass  0 fail`.
- (cut) `cut: whole` and the census empty; every count 28.
- (floor/rows/capture) the literal copies, the strip oracle, the three one-shot controls (`kept-names`,
  `floor-unknown`, `floor-grows`) each «holds».
- (live) `HG` green within the founder's cap; after-reads `28 ok`, `live-stamp: unmoved`, name diff exactly 2 lines.
- (fixtures/tests) `diff -r` empty after the landing; unit/integration/E2E `"ok":true`; pre-push and CI green.

## Touchpoints
35 modify entries (34 files and the 2.1.328 fixture dir), 2 new (`tests/security_negatives_env_capture.rs` stays; `fixtures/claude/2.1.328/Env.recorded-1.json`
from `HG`). The set grows 31 → 32 files.

## Test commands — 35 entries; check set re-run on rev 10
- `gate.py run --dry-run`: 35 parse, exit 0 · planlint: 0 hits (12 (c) INDETERMINATE: lettered steps, read by hand —
  every cited step exists) · `inputs.py verify`: 11 entries, 0 unparsed (`I5` uncited) · `scope --lists-only`: parsed
  26 · `scope --sync`: **PASS · delta +4 −1**.
- **Baselines** (this run dir's gate trail; `baseline` written on every `new` non-leg entry, entry 5 marked `new`):
  3 red, 4 red (140 lines, 22 files), 9 red — the cut not yet made; 5, 8, 10 green; 22, 23, 24, 25 red on absent
  inputs (no `HG` yet).
- **Entry 9's red, its cause read** (`gate.py show --n 9`): 62 run, 57 pass, **5 fail — the five stamped-home cases,
  each failing on the one row `cross-session-prefix`**, the 29th row the cut R2 removes. The cut not yet made on the
  tree: the red the plan predicts, said so in its baseline.
- **Clone controls** (`.andromeda/runs/2026-11-21T07-47-46Z-phase/p5-controls.md`, clone at `b3eb184c`): 3, 4, 22, 24,
  25 fired as themselves — each green on its pass input and red on its fail input; 19/20 (round legs the tool will not
  fire by `--entry`) typed unchanged: 19 holds its two names, 20 reads `2` on pass and `3` on a stray name; 22/23's
  key paths read over `HF`'s real kept stamp (`29 short`; `unknown` the two names now classified).

Research: lists synced to the revision's delta (+4 −1), rev 3; kept: `fixtures/claude/*/*.json` (covers the dropped
`UserPromptSubmit.cross-session-1.json`); not re-derived since the take-up's rev 2.

## Validation-1
Aligned with scope rev 3 and the working entry: the one row lands, the harness-prefix row leaves by the founder's
answer 1 (`[premise-corrected]` in scope), one more start `HG`, the live home untouched. No divergence.

## Concretization preview (the never-weaken checkpoint)
- **v2-26 — re-claimed**, its concretization moved from `S1`/`HF`/29 to `HG`/28:
  - *now:* «… `S1` (rig label `HF`) prints `identity-floor: pass` and `env-unknown: 0` … each on the eleven-name
    identity floor or the compiled three-name non-identity list (`CLAUDE_PLUGIN_ROOT`, `CLAUDE_PROJECT_DIR`,
    `CLAUDE_ENV_FILE`) … `contract_ledger_probes` reading 2.1.328 at `29 pass 0 fail`. A red `S1` verifies nothing.»
  - *proposed:* «verify's relied ledger row `identity-floor` reads `pass` on claude 2.1.328 live in the rig's scratch
    home: `HG` (base line 2 of the floor ledger, `--cap 2 --spare 1`) prints `identity-floor: pass`, `env-unknown: 0`
    and `pass-count: 28/28`, its kept stamp `target/live-rig/floor-stamp-2.json` reads `28 ok`, and its
    `measured.env_probe.added` lists the CLI's own `CLAUDE*` names a hook child carried, names only, each on the
    thirteen-name identity floor or the compiled four-name non-identity list (`CLAUDE_PLUGIN_ROOT`,
    `CLAUDE_PROJECT_DIR`, `CLAUDE_ENV_FILE`, `CLAUDE_PLUGIN_DATA`). No value reaches any capture, log, stamp or
    fixture: `security_negatives_env_capture` is green in CI (no canary value's bytes under the test home, the record
    dir or the user home; kept names absent from stamp and record). The recorded `Env.recorded-1.json` in
    `fixtures/claude/2.1.328/` is replayed by the fake agent in CI, `contract_ledger_probes` reading 2.1.328 at
    `28 pass  0 fail`. A red `HG` verifies nothing.»
  - Not weakened: the same outcome, the floor grown by the founder's own classification, one row fewer by his split.
- **v2-25 — un-claimed after the word** (`matrix.py unclaim`), then a dated `matrix.py note` naming the founder's
  answer 1 (`inputs#I10`) and the entry minted at this chunk's wrap. The revised plan no longer proves it.

## F-R1 — recorded, not asked
The three test-plan rows outside §5 (`r8-claude-env-strip`, `claude-env-strip`, `literal-test-oracles`) are reworded
with the other floor rows, the floor by NAME, no count where a row can avoid one — inside the founder's ratification of
the thirteen-name floor, the overseer's reading as his delegate (`inputs#I11` item 2). Authority `delegate overseer`,
**PROVISIONAL for the epoch batch**; it holds nothing. Written beside the fork in
`.andromeda/runs/2026-11-20T18-17-18Z-phase/founder-forks.md` §«Revision 1» and in the plan's Expected amendments.

## Beyond the plan
- (standing — `.andromeda/runs/2026-11-20T18-17-18Z-phase/p5-card-4.md`): none; that card's section held no row.
- **A new working-route entry, proposed for the wrap's route-resolve:** «Cross-session prefix ledger row — the
  `no-prompt` cause settled first», placed directly after «Harness-prefix and identity-floor ledger rows» in Epoch 4.
  It carries the `cross-session-prefix` row, its sender, the compiled `crossSessionInbound` accept literal and every
  part `red-analysis.md` lists under «cut out»; its phase first settles why `HF`'s sender reached no prompt; v2-25
  goes to it (`inputs#I10` items 1, 5).

---
Apply? (yes / review / cancel)
