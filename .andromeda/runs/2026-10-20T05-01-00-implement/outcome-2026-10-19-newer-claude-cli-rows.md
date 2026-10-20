# Implement outcome — 2026-10-19-newer-claude-cli-rows (gated mode, steps 8–12)

**Outcome: green on the gated part, with one red surfaced for the operator's word.** Every gate is green except
entry 24, the preservation guard. It reads red on the one recorded companion edit alone (see Surfaced).
- The block cleared on the founder's own answers (`inputs#I6`): Fork 1 option 1 and Fork 2 option 1.
- The live round used **13 of the 19 starts**: footer 1, record 6, `--local-live` 6. The 6 held for a re-run were not
  spent. Step 13 did not run (option 1).
- The macOS readiness flake (steps 6(c) and 7) still reads **unmet**: no CI run has located its phase yet. This run
  does not touch it.
- The chunk stays `pending`. **Next:** the operator pass, then stop before the wrap (`inputs#I7`).

## Inputs
- `inputs#I6`: the founder's own answers, snapped from `.andromeda/runs/2026-10-20T05-01-00-implement/relay-1.md`.
- `inputs#I7`: the operator's invocation.

## Files changed (`gate.py scope`: clean — changed 37 · listed 36 · recorded 1 (companion 1) · absorbed 29 · excluded 76)
- `crates/viola-agent-claude/src/screen.rs`:
  - `SIGNATURES.input_box` is `["for agents", "(shift+tab to cycle)"]`, with the doc comment updated;
  - `signatures_are_the_measured_literals` moved;
  - the `resting_footer` cases: `holds_any_reads_the_resting_footer` (8 rows),
    `verdict_over_the_2_1_302_resting_footer_alone_is_ready` and
    `verdict_over_the_resting_footer_beside_a_modal_is_input_not_ready`.
- `tests/contract_ledger_probes.rs`: `STAMPED` is `["2.1.287", "2.1.302"]`.
- `tests/contract_fixture_hygiene.rs`: `SIGNATURE_LITERALS` gains `"(shift+tab to cycle)"`. This is a **companion**,
  recorded in `scope-record.md`.
- `fixtures/claude/2.1.302/`: 29 files derived by entry 13's `viola verify --record`.
- Chunk folder:
  - `footer-reading.sh` (step 8);
  - `scope-record.md`;
  - `evidence/footer-2.1.302.md`, `evidence/live-round.md` (the start ledger and the round's readings), and
    `evidence/round-050522Z.txt`, `round-051156Z.txt`, `round-051400Z.txt`.

## Gates (trail `.andromeda/runs/2026-10-20T05-01-00-implement/gate-2026-10-19-newer-claude-cli-rows.json`)
- **The live round**, fired leg by leg with `--live-legs --entry {11,13,16}`, each `--void`ed on the cleared block:
  - `round: COMPLETE · legs fired 1/1` each time;
  - footer reading `green · 28.9s`;
  - record round `green · 69.2s`, `stamped 2.1.302  24 pass  0 fail`;
  - `--local-live` `green · 63.71s`, `local-live` passed 1.
  - Censuses 12 and 14 were green with no output. Entry 17 is unreachable by `--entry` without leg 18; its loop was
    run by hand and printed nothing (`evidence/live-round.md`).
- **Whole block:** `entries 35 · green 20 · red 1 (24) · recorded 0 · timeout 0 · not-run 14 · voided 2 (8,20)`.
  - Entry 8 (`resting_footer`): 10 passed.
  - Entry 20 (the fixture guard): green.
  - Entry 21: 115 passed.
  - Entry 23: 90 passed, the 2.1.302 set replayed and stamped by the fake agent.
  - Entry 32 `pre-push`: green in 88.75 s, `"stage":"linux-tests"`.
  - Entries 26–31 (the smoke): green.
- **Re-fire after the refused control:** entries 1, 2, 6 and 8 green on the restored tree.
- **WATCH** (the truncated `.profraw` at `pre-push`'s coverage merge): `pre-push` ran green once in this run, with no
  red. This is a fact for the wrap's tally.
- **Deferrals:** none.
- **Matrix:** `claimed by 2026-10-19-newer-claude-cli-rows: 0`. No ref was written, as the plan says.

## Smoke (P3)
Recorded from P2, not re-driven: entries 26–31 green.
- `boot` stamped through `viola verify` with the new literal set.
- `status` read ready.
- G2 and G4 were green.
- `cleanup` read `processes_gone` and `endpoint_gone` true.

## Surfaced
1. **Entry 24 (the preservation guard) reads red.**
   - Its only hit is `tests/contract_fixture_hygiene.rs` (`git diff --name-only` with the guard's own pathspecs), the
     recorded companion.
   - The new literal had to join the hygiene walk's own copy of the signature literals. Without it, the recorded
     2.1.302 screens would read `NotASignatureRow`.
   - Research did not list that file: its `for agents` sweep was left to implement, and the sweep found it at
     `tests/contract_fixture_hygiene.rs:357`. The guard's allow-list holds only research's lists.
   - This is the carried "For the operator's word" item, "the plan template's preservation guard reads red on any
     recorded companion edit". The wrap's light gate will read the same red. The operator's word decides it.
2. **The remove-the-guard reading for the `resting_footer` cases is owed** (testing.md 2026-09-25).
   - The new literal was neutralised in `screen.rs`. Firing entry 8 on it was then refused by the auto-mode
     permission classifier (`[Security Test Removal]`). It was not retried and not routed around.
   - The literal was restored, read back at `screen.rs:41`, and entries 1, 2, 6 and 8 were re-fired green.
   - The red reading needs the operator's permission to run.

## Deviations from the plan, with justifications
1. **Step 9, the literal's choice.** Taken literally, "the shortest text the resting input box keeps" admits any
   substring. Implement read it as the shortest phrase on the measured row that meets three conditions:
   - it is independent of the permission mode (`auto mode on` is the user's mode word);
   - it is not the rule row;
   - it is on no other text the 2.1.302 bundle holds (the tip "Hit shift+tab to cycle between …" and the Skills
     dialog's "enter/space to cycle, …"; the bundle was read by path, with no start).

   The choice is `(shift+tab to cycle)`. The record round validated it at 24/0. The reasons are in
   `evidence/footer-2.1.302.md` and the cases in `holds_any_reads_the_resting_footer`.
2. **Step 9, the extra cases.** Beyond the plan's two, there are the beside-a-modal verdict case and the rule row,
   blank row, tip and Skills-hint rows (all `false`), which pin the parentheses' purpose.
3. **Step 8's "last three rows"** were taken as the pane's rows 21–23 (`capture-pane -S 21 -E 23`). The recorded
   screens show the footer as the bottom row.
4. **Step 8, an extra version read.** The script reads `claude --version` by path before its start
   (verification-harness.md 2026-10-10). It starts no session.
5. **Transcripts.** The footer reading wrote none, since no prompt was typed; the plan expected one. The delta is in
   the accepted class (`evidence/live-round.md`).
6. **The `cli_verified:true` criterion** was read from the record home's `ledger/stamps.json` (2.1.302, all 24 rows
   `pass`). `stamps_verdict` reads that input as `(true, None)` (its unit case). No product run read it, because
   that would be another live start.
7. **A named limit of CI's replay:** the recorded 2.1.302 `ready` and `turn` screens hold both literals on one row
   (the hint is still drawn inside 10 s). So CI's fake agent never draws the hint-less resting row. The unit cases
   alone cover it.

## What stays open
- Steps 6(c) and 7 (the macOS flake): not named and not run. The criterion reads unmet; the operator pass's CI runs
  are its next chance.
- `send` on a verified 2.1.302 after `viola list` is not asserted live. The footer reading names the exposure: the
  hint was gone at 15 s, and whether `list` caused it cannot be separated from the start's own 10 s.

## Process hygiene
`gate.py census --comm viola --comm viola-fake-agent --comm claude --comm tmux --args projects/viola/target/` (trail
`census-2026-10-19-newer-claude-cli-rows.json`) read `census: 17 running`. None is this run's: no row's command line
holds `projects/viola/target/`, and no `tmux` is left.

| process | started by | final state |
|---|---|---|
| the footer reading's tmux server, `viola run m302` and its `claude` 2.1.302 | a leg (entry 11) | terminated (the script's `kill-session`; census 12 empty; no tmux in the census) |
| the record round's verify children (print probe, Runs A–E) | a leg (entry 13) | terminated (verify reaps its children; census 14 empty; no survivors line) |
| `--local-live`'s harness and verify children | a leg (entry 16) | terminated (no survivors line; entry 17's loop by hand empty) |
| gate entries' children (cargo, nextest, test binaries, fake agents) | this run | terminated (every entry exited) |
| smoke session `p-ncr-smoke` | this run | terminated (`cleanup` `processes_gone:true`, `endpoint_gone:true`) |
| the 17 census rows: the overseers, their bridges and builders, `~/.local/viola-product/bin/viola wait`, `~/.viola/bin/…/viola mcp`, this session's own `claude` | not this run (the operator's host) | left running — not this run's to stop |

## Decisions & corrections
- **The founder's word (`inputs#I6`):** Fork 1 option 1 (19 starts) and Fork 2 option 1 (one scratch start, footer
  rows only), read 2026-10-20T04:59:04Z. It is his own ratification of the footer-rows crossing.
- **The operator's ruling for this run (`inputs#I7`):** snap the answer first, ledger every live start with the
  clock's time before making it, then the operator pass; stop before the wrap.
- **Classifier refusal:** the remove-the-guard control (Surfaced 2) was refused with a verdict
  (`[Security Test Removal]`). Not retried, and the guard was restored.
- **`hyprctl locked` read `true`** at the first ledger row. No step focused a window or typed a key, so the rule
  did not stop the round.
