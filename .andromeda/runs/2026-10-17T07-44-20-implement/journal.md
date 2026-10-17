# Journal — 2026-10-17-cli-output-discipline implement run

- 2026-10-17T08:3xZ · P1 · remove-the-guard batch started in the background: `python3 $TMPDIR/{scratchpad}/controls.py`
  (13 controls; each applies one neutralisation, runs its filtered `agent-run.sh run`, restores the file byte for byte
  and checks the restore); its log `$TMPDIR/{scratchpad}/controls.log`, its readings copied to
  `chunks/2026-10-17-cli-output-discipline/evidence/red-03-remove-the-guard.txt`.
- 2026-10-17T08:0xZ · batch done: 13 controls red on their own cases; the paste control's first firing read all 8
  cases red (host COLORTERM inherited), the helper fixed, the second firing read the 3 new cases red.
- 2026-10-17T08:0xZ · P2 · the whole block started in the background: `gate.py run --plan
  viola-0.1.0/chunks/2026-10-17-cli-output-discipline/plan.md --run-dir .andromeda/runs/2026-10-17T07-44-20-implement
  --marker 2026-10-17-cli-output-discipline` (21 entries, 3 operator entries not fired; pre-push bounded 7200 s).
- 2026-10-17T08:00Z · block done: green 18 · red 0 · not-run 3 (operator). P3 smoke by hand, cleaned. Outcome written.
- 2026-10-17T08:04Z · operator pass: pre-push green, hygiene clean, pre-CI commit 94a3d25, entry 20 pushed
  (origin/build/viola-0.1.0 6d8b662c→94a3d25e, written after the push). CI read (entry 21) started in the background:
  `ci.py conclusion --sha HEAD --wait 1800`.
- 2026-10-17T08:13Z · CI ci#39015978414 red on test (windows-2025) alone: help_lists_only_served_verbs (the
  `viola.exe` usage word). Oracle fixed; entries 1,2,3,8 green; pre-push green; hygiene clean; fix commit 4a8a0ca pushed.
- 2026-10-17T08:26Z · CI ci#39016750772 green 15/15. Ready for the wrap.
