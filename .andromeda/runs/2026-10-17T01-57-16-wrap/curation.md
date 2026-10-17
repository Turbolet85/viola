# Curation — 2026-10-17-end-to-end-path-binary-and-harness-legs

CLAUDE.md ecosystem curated:
  Tier 1 (CLAUDE.md USER:session-learnings):  none
  Tier 2 (.claude/rules/*):                   + testing.md: "A new test binary's nextest layer is proven by the layers' own selection counts … never read off its stem" (confidence 0.8)
    Proof: implement P2 block run 1 — `run --e2e` read `Starting 155 tests across 18 binaries` (`10.log`) and `run --integration --filter 'binary(path1_run_start)'` ran `path1_run_start_one_home_in_order` (`12.log`, 1 passed), so the plan-named `path1_run_start` / `path6_budget_harness` sat outside the E2E layer; block run 2, after the widening (`inputs#I2`), read 157 / 20 and `nextest-exit-4` (`10.2.log`, `12.2.log`). Signals: verified by measurement +0.4 · specific technical detail +0.2 · reached no other durable home +0.2 (the general rule is in no master, route annotation, playbook or hook; the masters carry the widened regex, not the proving rule).
  Tier 3 (.claude/docs/session-learnings.md): none
  Filters: 0 dup · 2 task-specific · 0 conflict (→ handoff) · 0 deferred (→ handoff) · 3 at or below 0.6
  No-other-home: "A new test binary's nextest layer is proven by the layers' own selection counts, never read off its stem"
  CLAUDE.md size: see the P7 report (health.py check 1)

## Decisions & corrections — each entry's disposition
- The overseer's word on the E2E prefix (`inputs#I2`) — not a learning: a ruling, carried by the test-plan amendments and their sidecar entries.
- Sweep hazard: a numbered `path<N>_` binary against `^(path|…)_` — applied (Tier 2, testing.md), the rule generalised to proving a binary's layer by the selection counts.
- A throwaway mini workspace needs a binary matching a `binary()` filterset — rejected, Filter 4: 0.6 (measured +0.4, detail +0.2), no conditional signal (the fact was amended into test-plan §3 → 5-command `run` step 1 this wrap).
- A boot step writing into the fake agent's exe dir runs after verify's stamp — rejected, Filter 4: 0.6 (measured +0.4, detail +0.2), no conditional signal (amended into test-plan §3 → 5-command `boot`, the `--agents-mode` write order).
- The operator's invocation rules for implement — not a learning: this chunk's own direction, honoured; no "from now on" scope.
- The operator's wrap invocation (context alarms, stop after P1 if P2 will not fit) — not a learning: this wrap's own direction.
- `evolve.py append` refused a hand-typed `ts` ahead of the clock — rejected, Filter 4: the tool itself refuses the act (no other home needed; −0.3 one-off).
- Journal stamps written before the clock was read — rejected, Filter 2 / Filter 4: task-specific to this run's journal, one-off (0.0).
- The census by `/proc/*/exe` (`comm` truncates at 15 chars) — rejected, Filter 1: the host-linux.md 2026-10-07 entry already carries the rule (decide whose a process is by `/proc/<pid>/exe`); the rule held, no recurrence.
