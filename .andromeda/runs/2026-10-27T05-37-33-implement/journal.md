# Journal — /andromeda-implement · 2026-10-27-bounded-inputs-at-every-boundary

Every long run is entered here before it is waited on: what it runs, what it spawns, and the kill deadline
that bounds it (the operator's direction for this run: every test or control that spawns `viola run`,
`viola mcp` or a wrapper gets a kill deadline; process listings read whole).

## 1 · 2026-10-27T05:54:31Z — the whole gate block (P2, first firing)
- Call: `python -X utf8 {tools_dir}/gate.py run --plan viola-0.1.0/chunks/2026-10-27-bounded-inputs-at-every-boundary/plan.md --run-dir .andromeda/runs/2026-10-27T05-37-33-implement --marker 2026-10-27-bounded-inputs-at-every-boundary`, backgrounded.
- Entries 1–24 (25–27 are `leg = 'operator'`, not fired). Logs under `$TMPDIR/andromeda-gate/2026-10-27-bounded-inputs-at-every-boundary/`.
- Kill deadlines: every entry under GNU `timeout`, 1800 s default; entry 24 (`pre-push`) its own 7200 s.
  Inside: the harness's `run --unit` / `run --e2e` / `pre-push` run nextest's `ci` profile (30 s × 4 per test;
  `mcp_tools`, `chaos` and the other listed binaries 10 s × 2). Entries 8–10 run nextest's default profile, which
  has no per-test kill: their outer bound is the entry's 1800 s, and each spawning test carries its own deadline
  (the new `serve_ends_the_session_at_a_line_past_max_frame` kills its child at 5 s; the root wrapper tests'
  `WITHIN` 7 s waits; the PTY drop kills a wrapper).
- Spawns: wrappers (`viola run` over the fake agent) in root integration tests and the e2e binaries; `viola mcp`
  children in `mcp_tools` (the new case kills its child at 8 s); one harness smoke session `p-bounded-smoke`
  (entries 18–23), ended by entry 23's `cleanup`.
- After: `gate.py census` over `viola` / `viola-fake-agent` / `viola-harness`, read whole.
- Result: `entries 27 · green 23 · red 0 · recorded 1 · timeout 0 · not-run 3` (the 3 operator entries).

## 2 · 2026-10-27T05:59:17Z — the P3 smoke, by hand
- `bash scripts/agent-run.sh boot --session p-bounded-p3 --unstamped --instance builder` under `timeout 300`,
  stdout to a scratchpad file (never piped); then `status` under `timeout 60`.
- One `viola mcp` over the booted home: `initialize`, `notifications/initialized`, `tools/list`, then stdin
  closed; under `timeout 20` (the kill deadline), `VIOLA_*` removed, `PATH` the session's.
- `bash scripts/agent-run.sh cleanup --session p-bounded-p3` under `timeout 120`, then `gate.py census`.
- Result: boot ready, status ready, `viola mcp` exit 0 with the five tools, cleanup `processes_gone:true`; no bound hit.

## 3 · 2026-10-27T06:01:22Z — operator pass: the pre-push re-run (entry 24's command)
- `timeout 7200 bash scripts/agent-run.sh pre-push`, backgrounded, output to a scratchpad file (read whole).
- Kill deadline: `timeout 7200` (entry 24's own); inside, nextest's `ci` profile per test (30 s × 4).
- Spawns: the instrumented suite's wrappers and `viola mcp` children, each test reaping its own.
- Result: exit 0, `pre-push` `ok:true` at `linux-tests`, coverage 3251 passed, 0 failed, breaches none.
- Then: hygiene clean; pre-CI commit `607792cb`; entry 26 pushed it (`history moved: refs/remotes/origin/build/viola-0.1.0 4b0bf776→607792cb`).

## 4 · 2026-10-27T06:04:25Z — operator pass: the CI read (entry 27, as written)
- `python -X utf8 ~/.claude/skills/andromeda-phase/../andromeda-tools/scripts/ci.py conclusion --sha HEAD --wait 2400`,
  backgrounded; its own wait bound 2400 s (entry 27's `timeout` 2580). Spawns nothing local: it reads GitHub
  through `gh`.
- Result: `607792cb8178 verdict: green · checks 15/15 · wall 719 s · runs ci#40458923095 completed/success`; the final census `census: 8 running`, none under this repository's `target/`.
