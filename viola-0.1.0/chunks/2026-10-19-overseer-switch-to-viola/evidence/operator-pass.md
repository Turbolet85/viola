# Operator pass — 2026-10-19-overseer-switch-to-viola

On the operator's direction (this session, 2026-10-19 after 18:40Z): entry 20 until clean, the pre-CI commit
(`git add -A`), entry 21 through `gate.py run --operator 21`, entry 22 as written. Fired by the builder under the
product.

## Entry 20 — `gate.py hygiene` (as written), first firing, exit 0

    hygiene: clean — read 71 (runs 51 · evidence 3 · inputs 17) · trails 16 not read · copies 15 not read by P1 — 1 host paths kept · binary 0 not read by P1

Re-fired once after this file was written (so the commit's evidence is read too), exit 0:

    hygiene: clean — read 72 (runs 51 · evidence 4 · inputs 17) · trails 16 not read · copies 15 not read by P1 — 1 host paths kept · binary 0 not read by P1

## The pre-CI commit

`git add -A`, then `chore(2026-10-19-overseer-switch-to-viola): operator pre-CI commit, for the run this chunk's
verdict reads`, on `build/viola-0.1.0` over `f23586fb`. Its sha is recorded at entry 21 below (a commit cannot
carry its own sha).
