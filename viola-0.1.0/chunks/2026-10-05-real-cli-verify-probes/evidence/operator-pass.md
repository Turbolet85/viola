# Operator pass — 2026-10-05-real-cli-verify-probes

Driven by the implementer on the operator's word at the `/andromeda-implement` invocation ("run the operator pass with
the ci.py conclusion read (leg=operator)"), after the block read 18 green, 0 red (`pre-push`, entry 21, included).

## Entry 22 — hygiene (by hand), 2026-10-05
- **First read: `refused 9 files — P1 9`.**
  - Eight are phase run-dir raw gate listings (`dryrun*.out` / `baseline*.out` in the
    `2026-10-05T00-16-17`, `07-05-09` and `09-09-05` phase dirs). Each carried the host logs dir under the temp root once,
    and the skill path under the user home twice.
  - One is a line of this chunk's own `screen-probe-2.1.288.md`, a literal planted `/home/…` path.
- **The rewrite.** Each file was rewritten byte-wise, line count kept, every file kept: the logs dir to
  `$TMPDIR/andromeda-gate/…` and the skill path to `~/.claude/…`, the spellings the gate tool itself prints. The evidence
  lines were reworded, the planted path and the cause's `…/home/ledger/…` described rather than spelled.
- **Final read:** exit 0, `hygiene: clean — read 77 (runs 68 · evidence 5 · inputs 4)`. Atoms `exit 0` ✓ ·
  `contains hygiene: clean` ✓.

## The pre-CI commit and entry 23 — the push
- `012fc50` `chore(2026-10-05-real-cli-verify-probes): operator pre-CI commit, for the run this chunk's verdict reads`
  (the whole tree, after hygiene read clean again).
- Entry 23: `git diff --quiet && git diff --cached --quiet && git push origin HEAD` → exit 0,
  `caae9ec..012fc50  HEAD -> build/viola-0.1.0`.

## Entry 24 — the CI conclusion
- `ci.py conclusion --sha HEAD --wait 1800` → exit 0: `012fc5089498 verdict: green · checks 15/15 · wall 422 s · runs
  ci#37296910661 completed/success` (polled 15× over 436 s). Atoms `exit 0` ✓ · `contains verdict: green` ✓.
- **The CI read covers:**
  - the ten-row stamp and the screen replay;
  - the trust-root fake and the full gate;
  - all three OSes, Windows ConPTY included.

