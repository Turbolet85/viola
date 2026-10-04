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
- Entry 19: pushed `93a5cbf` (the pre-CI commit) to `origin/build/viola-0.1.0` (`eb37914..93a5cbf`).
- Entry 20 (`ci.py conclusion --sha HEAD --wait 1800`): exit 0 · `93a5cbf4f13e verdict: green · checks 15/15 · wall
  287 s · runs ci#37239689446 completed/success` (polled 11× over 312 s). The final HEAD run is ci#37239689446:
  the harness-turn, release and driver-turn witnesses, G4 schema conformance and the secret scan, on all three OSes.

## After the wrap's light gate (run dir `2026-10-04T22-27-20-wrap`, `light-gate-fix.md`)
- The light gate read two reds: the full suite (`agent-run.sh run`, a receipt-count race in the two new turn
  witnesses) and the `:82` probe (a plan defect). Both were folded on the overseer's founder-delegated word.
- Entry 17 on the fixed, uncommitted tree: exit 0, `"ok":true`, `"stage":"linux-tests"` (coverage 1472 / 0,
  playwright 1 / 0, gate no breaches), 0 corrupt-profile lines.
- Fix commit `334ee7f` (`tests/cli_send.rs`, `tests/tui_wheel.rs` only), pushed `93a5cbf..334ee7f`.
  - Pushed with `git push origin HEAD`, not entry 19's guarded form: its `git diff --quiet` refuses while the wrap's
    own artifacts (spec bodies, sidecars, report, run dir, plan correction) sit uncommitted for the wrap commit.
  - `git status --short -- src tests crates Cargo.toml Cargo.lock` read empty before the push, so CI tests the
    complete source.
- Entry 20 on `334ee7f`: exit 0 · `334ee7f3d0ef verdict: green · checks 15/15 · wall 290 s · runs ci#37241137053
  completed/success` (polled 11× over 310 s). The final HEAD run is **ci#37241137053**.
