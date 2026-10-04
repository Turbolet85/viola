# Operator pass — 2026-10-04-running-turn-refusal

Fired by hand in the implement session, on the operator's word ("Run the operator pass with the ci.py
conclusion read (leg=operator) as usual. Tally the .profraw WATCH from every pre-push you run.").

- Entry 17 (`bash scripts/agent-run.sh pre-push`) on the uncommitted tree before the pre-CI commit: exit 0,
  `"ok":true`, `"stage":"linux-tests"` (coverage 1472 passed / 0 failed, playwright 1/0, gate no breaches), 0
  corrupt-profile lines. WATCH: post-fix green 4/4 consecutive (`watch-profraw.md`, run 5).
- Entry 18 (`gate.py hygiene`): first read `hygiene: refused 3 files — P1 3` — phase P4's raw `gate.py` listings
  in `.andromeda/runs/2026-10-04T21-38-24-phase/` (`.baseline.txt`, `.dry2.txt`, `.dry3.txt`: untracked, named only
  in the gate tools' delta inventories, the same class `2026-10-04-wait-and-last` removed). Removed; re-read exit 0 ·
  `hygiene: clean — read 49 (runs 47 · evidence 2 · inputs 0) · trails 13 not read · copies 0 not read by P1 — 0 host
  paths kept · binary 0 not read by P1`.
