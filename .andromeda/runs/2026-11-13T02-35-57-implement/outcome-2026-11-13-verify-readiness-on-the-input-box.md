# Outcome — 2026-11-13-verify-readiness-on-the-input-box

**Outcome: green on gate entries 1-29, and the live proof (30-36) passes on claude 2.1.323.** The founder's live-start
cap was given later («Даю, как в плане», relayed, `inputs#I7`). The one verify moved from 2.1.322 to 2.1.323, the
newest installed at the proof, on the overseer's word (`inputs#I8`). The operator pass then ran 37-43 (below). No
mutation run.

## The live proof (operator entries 30-36)
Lines from the gate tool: 30 and 31. Lines HAND-FIRED, NOT the gate tool's: 32, 33, 34, 35, 36. Each changes only the
version, 2.1.322 → 2.1.323. Every one of them is recorded with its whole output in `evidence/op-3N-*.txt`.
- 30 (tool): `30 probe recorded · exit 0 · 0.07s · 269 B → 30.log · history: unmoved · mise ls claude`. Newest
  installed: `claude  2.1.323  /etc/mise/conf.d/omarchy-tools.toml  latest`.
- 31 (tool): `31 build green · exit 0 · 8.81s · history: unmoved`. Hash
  `a17fd55cf04832edd6352518e46f4396f14673e5a0cf6baf4e0884bdb7f10c17  target/debug/viola`. Re-read identical at
  2026-11-13T06:08:22Z, right before the start.
- 32 (hand): `timeout -k 10 1800 bash -o pipefail -c 'target/debug/viola verify -- ~/.local/share/mise/installs/claude/2.1.323/claude > …/evidence/verify-2.1.323.out 2>&1'`.
  - Exit 1, expected: the one fail is `[21/24] statusline-rate-limits … fail`.
  - Last line: `stamped 2.1.323  23 pass  1 fail`.
  - The one verify of the cap: six `claude` starts and one agents read.
- 33 (hand): `24`, exit 0.
- 34 (hand): `0`, exit 1.
- 35 (hand): `stamped 2.1.323  23 pass  1 fail`, exit 0.
- 36 (hand): `true`, exit 0. The 2.1.323 stamp's `typed_probe` reads
  `{"ready_settle_ms": 1943, "turn_settle_ms": 315, "prompt_latency_ms": 66, "max_turn_gap_ms": 227, "compact_latency_ms": 7200, "box_only_guards": 3}`.
- **v2-29:** clause (1) met; clause (2) met with `box_only_guards` 3. The matrix row is set `implemented`
  (`matrix.py implement`); the wrap verifies.
- **Outside the cap's count:** two hand-run `claude` processes on 2.1.323 before the verify, `--version` and (by mistake)
  `--help`. Each was a print with no session, reported in `journal.md`.
- **Clean-up:** no `.viola-verify-*` dir left; census `none running`.
- **Stamps:** they now hold 2.1.296 · 2.1.302 · 2.1.312 · 2.1.319 · 2.1.323. 2.1.322 was never stamped, and 2.1.296's
  entry is kept.

## Gate block
Trail `.andromeda/runs/2026-11-13T02-35-57-implement/gate-2026-11-13-verify-readiness-on-the-input-box.json`, three
firings (journal: `.andromeda/runs/2026-11-13T02-35-57-implement/journal.md`).
- Firing 1: `entries 43 · green 25 · red 3 (8,9,29) · recorded 1 · timeout 0 · not-run 14`. One cause for all three:
  `cmd::verify::typed::pty::tests::settled_on_the_box_alone_settles_at_the_quiet_instant`, `assertion left == right
  failed, left: "❯ ", right: "❯"` at `src/cmd/verify/typed/pty.rs:291`. That was the new test's own oracle: vt100 keeps the
  trailing space it was fed. Fixed to `"❯ "`. No product code changed.
- Firing 2 (`--only 8,9,29`): `entries 43 · green 3 · red 0 · recorded 0 · timeout 0 · not-run 40`.
- Firing 3 (the whole block): `entries 43 · green 28 · red 0 · recorded 1 · timeout 0 · not-run 14`.

Readings (firing 3 logs):
- 1: `tmpfs 32G 9.0G 23G 29% /tmp`.
- 7: `608 tests run: 608 passed`.
- 8: `2628 tests run: 2628 passed`.
- 10: `511 tests run: 511 passed (5 slow)`.
- 11: `281 tests run: 281 passed`.
- 13: `error: package ID specification rustix@1.1.5 did not match any packages`, exit 101.
- 14: `advisories ok, bans ok, licenses ok, sources ok`, with three duplicate warnings (syn, thiserror,
  thiserror-impl) that were already there.
- 18: `deny-probes: 14/14 banned, control clean`, the 14 the plan predicted.
- 20: `rmcp-features: server transport-io`.
- 22: `release-check: viola only`.
- 24: boot green, stamped.
- 29: `"stage":"linux-tests"`, ok, `3420 tests run: 3420 passed (5 slow)`.

No entry was deferred. Every changed surface's gate ran.

## Where a planned row or a letter fell short
- **Plan step 3, read literally, is wrong.** It says `settled` and `box_wait` read `SIGNATURES.input_box_up` in place of
  `holds_any`. But `holds_any` also matched the modal literals, and `input_box_up` is false on a modal screen. The literal
  swap would hold Run A's modal screen to `GATE_MAX_WAIT` (8.5 s more per verify, in every stamped-home test). It would
  also hold a modal box wait to `PROBE_DEADLINE`. Built instead: `settled` and `box_wait` end on a quiet screen where the
  input box is up or a modal literal stands (`signed()` in `src/cmd/verify/typed/pty.rs`). The existing case
  `box_wait_ends_on_a_quiet_literal_with_its_rows`, which waits on the trust modal, stays green unchanged.
- **Plan step 2's fault case holds only in part.** It says "fault reads a recorded box row with no finding". A kept rule
  row is made only of seam characters, so its seam check reads the neighbouring live prompt row's whole text. So
  `--record` refuses a screen whose input box holds a home path, a username or an email. Such a screen is refused, never
  kept. Both cases are now pinned:
  - `screen_fault_reads_a_recorded_box_row_with_no_finding`
  - `screen_fault_refuses_a_box_whose_prompt_row_holds_the_home`
- **Research missed a file.** `fuzz/fuzz_targets/vt100_feed.rs` builds a `Signatures` literal and is in neither of
  research's lists. It was edited as a companion and recorded in `doc:…/scope-record` rev 1. No gate entry builds the fuzz
  workspace. On the operator's request after the block, it was compile-checked on its pinned nightly, which was already
  installed: `RUSTUP_AUTO_INSTALL=0 timeout -k 10 600 cargo +nightly-2026-09-20 check --offline --manifest-path
  fuzz/Cargo.toml --bins`, exit 0. It checked all six targets, `vt100_feed` included, with 0 warnings and 0 errors.
  Its last line: `Finished dev profile [unoptimized + debuginfo] target(s) in 5.13s`. Output:
  `.andromeda/runs/2026-11-13T02-35-57-implement/fuzz-check.txt`.
- **matrix#v2-29 was not written.** Its acceptance is the live verify on claude 2.1.322: entries 32-36, the
  `[NN/24]` lines, the `stamped 2.1.322` line and `box_only_guards ≥ 1`. No unit test satisfies that as written, so no
  `ref` and no `implemented` were set. The plan's own words ("the cap stays `implemented`, not verified") suggest that
  `implemented` was meant. That write is the operator's call: it is one
  `matrix.py implement … --id v2-29` with a unit-test ref.
- **No remove-the-guard control was run** for the new predicate's guard cases. The plan names none, and the operator
  ruled out a mutation run.

## Files changed
In research's lists:
- `crates/viola-agent-claude/src/screen.rs`
- `crates/viola-agent-claude/src/ledger.rs`
- `crates/viola-agent-claude/src/ledger/test_cases.rs`
- `src/cmd/verify/typed/turns.rs`
- `src/cmd/verify/typed/pty.rs`
- `src/cmd/verify/typed/runs.rs`
- `src/run/gate.rs`
- `tests/contract_fixture_hygiene.rs`
- `deny.toml`
- `Cargo.lock` (rustix 1.1.5 → 1.1.6)

Recorded (scope record): `` - `fuzz/fuzz_targets/vt100_feed.rs` · companion · serves `crates/viola-agent-claude/src/screen.rs` · self ``.

Scope read: `scope: clean — changed 11 · listed 10 · recorded 1 (companion 1 · mechanical 0 · in-intent 0 · widening 0) · absorbed 0 · excluded 228`.

## The operator pass (37-43)
- 37, fired as written (`gate.py hygiene`):
  - First reading: `hygiene: refused 2 files — P1 2 · P2 0 · P3 0`. Both files were `in-root`:
    `evidence/op-31-build-hash.txt` and the run dir's `fuzz-check.txt`.
  - `respell` wrote `fuzz-check.txt` (`×4 · −136 B`). It refused `op-31-build-hash.txt` with
    `:9: an in-root path not spelled as git spells the root (or the bare root) — nothing written`, so that file was
    respelled by hand, and the file says so.
  - Re-fired: `hygiene: clean — read 244 (runs 227 · evidence 8 · inputs 9)`.
  - Land: `8 new · 0 changed · 0 gone · 9 kept · tx 876`.
- 29, re-run before the push (`--entry 29`): `29 probe green · exit 0 · 98.21s · bash scripts/agent-run.sh pre-push`.
- The pre-CI commit, the push (38) and the CI reads (39-43) follow; their lines are in the console report and the
  journal, written after this commit.

## Process hygiene
Census: `census: 15 running`. None of them is under `projects/viola/target`.

| process | started by | final state |
|---|---|---|
| gate entries' children (cargo, nextest, viola-harness, fake agent) | this run | terminated (entry 28 read `processes_gone` and `endpoint_gone` true; no census row under `projects/viola/target`) |
| 4 overseer `claude`, 5 prototype `viola run` and their `claude` children, `~/.viola/bin/…/viola mcp`, two `viola wait` | NOT DERIVED | left running: not this run's (other trees and sessions) |

## Decisions & corrections
- Operator direction (this invocation's arguments), carried out as given:
  - build steps 1-6 and run entries 1-29;
  - the founder's live-start cap is not on record, so no live `claude` start, and entries 30-36 not fired;
  - stop before entry 30;
  - no mutation run;
  - journal each long run before waiting on it.
- Guard refusal: one Bash guard refusal of a `cd` into the skill's references dir (it would have moved the session's
  cwd). Re-issued once with absolute paths, and it ran.
