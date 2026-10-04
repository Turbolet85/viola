# Operator pass — 2026-10-04-confirmed-send-with-cl-1-records

Run 2026-10-04 on the overseer's word ("Run the operator pass now (entries 25-27: hygiene, the pre-CI commit, push,
the ci.py read with leg=operator --wait); the 3-OS CI is the v1-12 witness. Fold any red now. Report and stop before
the wrap."), on the Linux dev host. The native `pre-push` (entry 24) read `ok:true`, stage `linux-tests`, on this tree
at /implement (gate trail `.andromeda/runs/2026-10-04T06-10-38-implement/`, both full runs), and again on the folded
tree before the push (below).

| entry | command (as the plan lists it) | exit | reading |
|---|---|---|---|
| 25 | `python -X utf8 …/andromeda-tools/scripts/gate.py hygiene` | 0 | first read `hygiene: refused 3 files — P1 3` (the three `.dryrun*.log` raw `gate.py --dry-run` listings phase P4 left in `.andromeda/runs/2026-10-04T05-41-59-phase/`, cited nowhere): removed; re-read `hygiene: clean — read 42 (runs 42 · evidence 0) · trails 14 not read · binary 0 not read by P1` |
| — | the pre-CI commit (`git add -A`, then `chore(2026-10-04-confirmed-send-with-cl-1-records): operator pre-CI commit, for the run this chunk's verdict reads`) | 0 | `a6abfc2` on `build/viola-0.1.0` |
| — | a red met and folded: the fix commit `fix(2026-10-04-confirmed-send-with-cl-1-records): keep the paste_text fuzz seeds byte-exact` | 0 | `306c4ae`; before it, entries 1 · 15 · 24 re-ran green on the folded tree (`fuzz-replay` passed 5; `pre-push` `ok:true`); hygiene after it `clean` |
| 26 | `git diff --quiet && git diff --cached --quiet && git push origin HEAD` | 0 | `51b5338..306c4ae  HEAD -> build/viola-0.1.0`; 0 ahead after |
| 27 | `python -X utf8 …/andromeda-tools/scripts/ci.py conclusion --sha HEAD --wait 1800` | 0 | `306c4ae757ae verdict: green · checks 15/15 · wall 295 s · runs ci#37183365088 completed/success` |

## The red folded
The pre-CI commit's `git add` warned that `fuzz/corpus/paste_text/multi-line` had its CRLF replaced: under the
repository's `* text=auto eol=lf`, the committed blob held `\n` where the seed holds `\r\n` (33 bytes, not 34), so the
corpus in history was not the seed written and a fresh checkout would replay a different input. `.gitattributes`
gained `fuzz/corpus/paste_text/** binary` (as `vendor/conpty/**` already is); the corpus was re-staged and its blob
read back as the 34 seed bytes, CRLF included. Recorded in `scope-record.md` (`.gitattributes` · in-intent).

## The three-OS witness (ci#37183365088)
Every job `success` (15/15), `fuzz-replay` included. Read from each test job's log (`gh run view --job <id> --log`):
every one of the 15 tests below PASSED on each OS, and no job logged a FAIL or TIMEOUT line.

| job | `cli_send` (6) | `channel_paste_validation` (2) | `chaos_feed_panic` (1) | controls `send` rows (5) | `channel_debug_level_…` (1) | G2 | G4 | secret scan |
|---|---|---|---|---|---|---|---|---|
| `test (windows-2025)` 111380211392 | 6 PASS | 2 PASS | 1 PASS | 5 PASS | PASS | `g2: clean` | `ok:true` 126 files / 755 lines | `ok:true`, 0 hits |
| `test (macos-latest)` 111380211453 | 6 PASS | 2 PASS | 1 PASS | 5 PASS | PASS | `g2: clean` | `ok:true` 117 / 719 | `ok:true`, 0 hits |
| `test (ubuntu-latest)` 111380211410 | 6 PASS | 2 PASS | 1 PASS | 5 PASS | PASS | `g2: clean` | `ok:true` 117 / 719 | `ok:true`, 0 hits |

**v1-12** — `send_window_second_concurrent_send_is_refused_turn_running` PASSED on windows-2025 (10.495 s), macos-latest
(10.504 s) and ubuntu-latest (10.664 s): the matrix acceptance's three-OS witness.

The chaos test's home is outside `target/e2e-home` (F4), so G2 / G4 / the secret scan read clean without counting its
deliberate panic; the test itself asserts that panic line and its `parse-rejected` line present.

## Held for the wrap
- **F3** (the fourth dated gap: `viola send`'s frame without strict-modes or peer identity) — the wrap halts for the
  founder's live ruling. Only the planned liveness pre-check was built.
- **F4** (the chaos test's home outside G2, G4 and the secret scan) — founder-visible beside F3.
- **Overseer direction:** "the main.rs panic-line gap in send (no "error: internal error" on a panic) needs a named
  owner, the first chunk allowed to touch main.rs." `src/main.rs` `role_of` files `send` as `Role::Other`, so a panic
  inside `viola send` exits 1 without the `cli` role's stderr line (obs-plan §7); an ordinary error prints it from the
  `Send` dispatch arm. `src/main.rs` is under this chunk's held-boundary probe.

## After the pass
The wrap's E3 resolution (the overseer, founder-delegated: "the contract is right; fold detail:{code,message,data} with a
unit test now") changed `src/cmd/send.rs` after `ci#37183365088`: the `--json` wrapper fault now carries the arch
contract's `detail` object. The wrap's light gate (`.andromeda/runs/2026-10-04T06-44-39-wrap/`) re-ran the whole block
over it green, `pre-push` included; the CI run on the wrap commit is its three-OS witness.

