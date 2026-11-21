# Outcome — 2026-11-21-mcp-calls-bounded (implement run 2026-11-21T21-51-10Z-implement)

**green** (2 fix iterations) — Next: the OPERATOR PASS in this same window (entries 30–36: the smoke boot / status /
cleanup, `pre-push`, hygiene, the push, the CI conclusion), then the wrap in a fresh window.

## Files changed (12; `gate.py scope`: clean — changed 12 · listed 12 · recorded 0)
New: `crates/viola-e2e/src/rig/mcpcall.rs`. Modified: `crates/viola-core/src/lib.rs`,
`crates/viola-agent-claude/src/lib.rs`, `crates/viola-mcp/{Cargo.toml, src/lib.rs, src/tools.rs, src/result.rs,
tests/handlers.rs}`, `tests/contract_error_bodies.rs`, `crates/viola-e2e/src/rig/mod.rs`,
`crates/viola-e2e/src/bin/viola-live-rig.rs`, `crates/viola-e2e/tests/rig_rehearsal.rs`.

## Deviations, with their justifications (scope record: none — gate.py scope clean, 0 recorded)
1. The arm canonicalises the fake agent's path (`--fake`, else `--program`), refusing `fake` when it cannot: the live
   entries pass `target/harness/debug/viola-fake-agent` relative, and `viola run` resolves its child from the scratch
   cwd, where a relative path names nothing. Step 10 named the path, not its spelling.
2. A `tool_result` with no `is_error` key reads `call-error: false` (a non-boolean `unread`): step 9 named the words
   from `is_error` and not an absent key. The live starts' `call-error: false` came from the client's own line.
3. The cut rehearsal takes the arm word `mc` (`rehearse mc` → `Arm::MB`, label `rehearse-MC`, `rig::REHEARSE_MC`),
   step 12 naming the label's shape but no word; `rehearse-MB` comes from `mb` by the rig's own `rehearse-{arm:?}`.
4. One summary key beyond step 10's list, `stdout-skipped` (the reader's count of lines of no known shape); no entry
   asserts it. MB `0`, MB8 `0`, MC `0`.
5. A third rehearsal case beyond step 12's two: `rig_rehearsal_mb_live_without_a_fake_refuses_before_its_ledger_line`
   (the live arm's `refused: fake`, exit 3, no ledger line, no scratch) — step 10's refusal witnessed.
6. `tests/contract_error_bodies.rs`: the census asked nothing (green untouched); the MCP `instance-unreachable` row's
   `detail` list gained `call-deadline` so the row names what its writer now writes. The CLI row is unchanged.
7. The non-live entries fired by `--only 1,…,13` ahead of the round, 27 (`--operator`), 28 and 29 after it
   (verification-harness.md 2026-11-20; the plan's own order).

## Gates
- Entries 1–13: green (the first firing's clippy red, `unnecessary_lazy_evaluations` at
  `crates/viola-e2e/src/rig/mcpcall.rs:135`, fixed by `then_some`; the re-run 13/13 green).
- Control `call-deadline` on entry 6, `--timeout 300`: holds (`evidence/control-1.txt`; all four cases red at the
  5 s backstop with the guard removed).
- Round (the overseer's word `inputs#I6`; hostwatch `--from 2026-11-21T22:13:23Z` QUIET): `round: COMPLETE · legs
  fired 3/3 · … red 0` (`evidence/round-222200Z.txt`); MB · MB8 · MC each green; spare unspent.
- 27 `census: 0` (green, `history: unmoved`) · 28 recorded (`evidence/live-round.md`) · 29 `run --integration` green,
  608 passed, 0 failed.
- No `gate.py guard` entry in the block.
- Carry `cry-ujze4atl`: `rig_rehearsal_reads_the_box_empty_after_x_and_del` ran green in entry 29; its red not met →
  re-pinned whole.

## Surfaced / stuck
none.

## Process hygiene
| process | started by | final state |
|---|---|---|
| viola-live-rig · viola run rig-mb · viola-fake-agent · claude -p · viola mcp (MB, MB8, MC) | a leg | terminated (each start's `census: 0`; the round's survivors none; entry 27 `census: 0`) |
| the `call-deadline` control's clone and its firings | this run | terminated, `clone removed` |
| gate entries 1–13, 28, 29 (cargo, nextest, harness) | this run | terminated (each exited) |

`gate.py census --comm viola --comm viola-live-rig --comm viola-fake-agent --comm claude --args
projects/viola/target/debug/viola`: 13 running, none this run's — the host's overseer sessions, the viola-lab
prototype's wrappers and their `claude` children, the product's `viola wait`, and the real home's pinned
`~/.viola/bin/0.1.0-…/viola mcp`; none runs this repository's `target/debug/viola`.

## Decisions & corrections
- The overseer's word (`inputs#I6`, verbatim `relay-1.md`): read hostwatch from 2026-11-21T22:13:23Z; the read was
  `QUIET - 0 s stalled on IO`; the round fired as one call. Its own entry 14 (`read --last 15`) read CONTENDED for
  stalls at 22:01–22:02 and 22:06–22:07, inside the neighbour's window the word named as ended and this repository's
  gate run — recorded, the round not withheld.
- Guard: `bash-guard.sh` refused a `cat` heredoc with a file target (the `arm_mb` insert); re-issued once through the
  Write tool into the scratchpad.
- `gate.py run --only 1-13` refused at exit 2 (a range is no form): re-issued as a comma list.
- MC's `outlived: yes`: at the cut call's closing line `viola mcp` still ran — a reading, never an atom (`inputs#I2`
  item 5).
