# Journal — implement 2026-10-26-explicit-session-links

Each long run is entered here before it is waited on: what was launched, when (UTC), why, and how it ended.

## 1 · 2026-10-26T15:07Z · the whole gate block, first firing
- Call: `python -X utf8 {tools_dir}/gate.py run --plan viola-0.1.0/chunks/2026-10-26-explicit-session-links/plan.md --run-dir .andromeda/runs/2026-10-26T14-59-26-implement --marker 2026-10-26-explicit-session-links`, backgrounded.
- Why: P2's first full run after P1 wrote steps 1 to 11 (clippy already read clean by hand).
- Expected length: the block's pre-push (about 90 s at P5) and perf rows dominate; P5 baselines sum to about 4 min.
- Outcome: `entries 26 · green 22 · red 0 · recorded 1 · timeout 0 · not-run 3` (the three `operator` legs);
  orphans-check 113.83 s, pre-push 92.08 s (`"ok":true`, stage `linux-tests`, coverage 3133 passed, 0 failed,
  breaches none). Green on the first firing: no fix-loop iteration.

## 2 · 2026-10-26T15:10Z · the remove-the-guard controls and the restored re-run
- Three neutralise / run / restore controls by hand (each under 10 s), then `gate.py run … --only 2,3,8,9,10,13,15`
  in the foreground (about 25 s).
- Outcome: each control red on exactly its guarded cases; the restored tree `green 7 · red 0`. Record:
  `viola-0.1.0/chunks/2026-10-26-explicit-session-links/evidence/remove-the-guard.md`.

## 3 · 2026-10-26T15:13Z · operator pass step 1: the cross-target lints and the native gate
- Implement's report is written (`outcome-2026-10-26-explicit-session-links.md`, green + smoke); the operator pass
  runs on the operator's word in the session's opening request.
- Calls: the macOS-target and Windows-target check-only clippy (`target/maccheck`, `target/wincheck`), then
  `python -X utf8 {tools_dir}/gate.py run --plan viola-0.1.0/chunks/2026-10-26-explicit-session-links/plan.md --entry 23`
  (pre-push, about 90 s; no run dir, the pass runs under no skill).
- Outcome: both lints exit 0; entry 23 `green · exit 0 · 91.57s`. Hygiene (entry 24) `clean`. Record:
  `viola-0.1.0/chunks/2026-10-26-explicit-session-links/evidence/operator-pass.md`.
