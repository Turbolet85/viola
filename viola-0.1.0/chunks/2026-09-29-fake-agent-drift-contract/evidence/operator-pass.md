# Operator pass — 2026-09-29-fake-agent-drift-contract

Driven in the implement session on the operator's word ("Go: run the operator pass now, entries 16-18"), under the
take-up direction "Judge local reds against a same-day control, with CI as the acceptance leg. Read CI through the ci.py
conclusion tool call."

## The pre-push guard, pushed through
Entry 12 (`agent-run.sh pre-push`) is red on its windows-tests stage only (linux-tests green: coverage 948/948,
playwright 1/1, `gate` green). The red is recorded `red — not this chunk's` with its two-sided `2d8bc53` basis in
`host-reds-two-sided.md`; the acceptance leg is CI (entry 18).

## Entry 16 — `gate.py hygiene` (`gate v1.5 · 018a3122`: overseer1's V35 deploy mid-run, announced to the operator)
- First firing: `hygiene: refused 3 files — P1 2 · P2 0 · P3 1`.
  - P3 `.andromeda/runs/2026-09-29T13-00-50-phase/baseline/control/src/leak.rs` (rust plane source): entry 9's one-line
    known-positive control (`const P: &str = "<cross-session-message";`). Disposition, per the operator's convention for
    phase-run controls: renamed to `leak.rs.txt`, bytes unchanged (sha256 `d73f4395…` before and after). The plan's
    entry-9 `baseline` note still names `src/leak.rs` (plan.md is immutable here).
  - P1 `.andromeda/runs/2026-09-29T13-00-50-phase/dryrun-p5.txt` ×4: the phase's copied dry-run listing carried the host's
    bash, repo, temp-log and skills paths. Rewritten to `<git-bash>` · `<repo>` · `<OS temp>` · `<skills>` (lines 2-4, 22,
    24; binary-mode, CRLF kept, read back).
  - P1 `evidence/host-reds-two-sided.md:8`: the control worktree's absolute path → `<repo parent>/viola-ctl-2d8bc53`.
- Second firing: `hygiene: clean — read 39 (runs 37 · evidence 2) · trails 12 not read · binary 0 not read by P1`;
  with this record written, `clean — read 40 (runs 37 · evidence 3)`.

## Entry 17 — the push
Pre-CI commit `055adf4`; `git diff --quiet && git diff --cached --quiet && git push origin HEAD`: exit 0,
`2d8bc53..055adf4 HEAD -> build/viola-0.1.0`; HEAD == `@{u}`.

## Entry 18 — `ci.py conclusion --sha HEAD --wait 1800` (the acceptance read)
`ci v1.0 · bf89b279` · polled 10× over 282 s · `055adf4b2291 verdict: green · checks 15/15 · wall 270 s · runs
ci#36583175440 completed/success`; exit 0. Both atoms held (`exit 0`, `contains verdict: green`). **The final HEAD run:
ci#36583175440.** No red to fold; no fix commit.

## The control worktree
The same-day control (`<repo parent>/viola-ctl-2d8bc53`, detached at `2d8bc53`) was removed after this read, on the
operator's word; the wrap's light gate mints its own control if it needs one.
