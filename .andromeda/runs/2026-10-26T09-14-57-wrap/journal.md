# Wrap journal — 2026-10-26-server-verification-before-any-frame

## 2026-10-26T11:34:38Z — halted at P7.1; the red goes back to implement

**Decision** (inputs#I13, the overseer's founder-delegated answer, 11:34Z): the open `pre-push` red (light gate entry 21:
every test passed, the coverage merge refused one cut-short `viola-fake-agent` profile) folds into this chunk — a red
met during a chunk folds into it — and goes back to implement. The fix targets the measured mechanism (a process killed
in its exit-time profile write) as the Epoch 5 cleanup did for the fake agent's hooks: the fake agent's own exit taken
out of the PTY's foreground group (`chunks/2026-10-20-epoch-5-cleanup/evidence/profraw-cause.md`). No fix work in this
window.

**Where the wrap stands:** P1–P6 done and uncommitted in the tree; P7.1 red; no commit, no flip, the master record
`pending`. What the tree holds from this wrap, all uncommitted: `chunks/…/report.md` (+ inputs I7–I13), the seven
masters' body and category-file edits and their sidecar entries (129 amendments), the leaves (34 findings), the curation
writes (`testing.md`, `testing-src.md`, `docs/session-learnings.md`), the working route's four CARRYs, `state.yaml`
(session 93), `.claude/session-handoff.md`, and this run dir. The evidence: `basis-loop.md` (the census and the parent
loop: no writer named, no basis), `cascade-dispositions.md`, `fanout-results.md`, `curation.md`.

**Resume point, in order:**
1. `/andromeda-implement` re-entry on this chunk for the fake-agent exit fix (a source edit in `src/bin/viola-fake-agent.rs`,
   test-only, behind `fake-agent`; a scope-record line for it), its gates and the operator pass with CI on the fix commit.
   That run must leave this wrap's uncommitted P1–P6 writes as they are.
2. `/andromeda-wrap-session` resumes from THIS run dir (`.andromeda/runs/2026-10-26T09-14-57-wrap/`), Setup 2a's resume:
   the report is reused as is (the fix enters through its addendum — an input read at the resume, or a fact Validate
   raises), the fan-out is reused (`fanout-results.md` carries every doc's verdict line), and the wrap continues at P7:
   step 0 (the new-text listing re-read — the fix moves source, so it reads `the listing differs`, and that HALT is the
   operator's to rule), then the light gate re-run, the drift and coverage gates (claimed 0), hygiene, the flip with
   compaction, the commit and the push.

## 2026-10-26T12:21Z — resumed at P7 step 0 (inputs#I16)
- Step 0: `the listing differs`, first at `src/bin/viola-fake-agent.rs`; the difference is the fake-agent fix's two
  files alone (`new-text-resume.md`). Ruled (inputs#I16): taken in through the report's addendum
  (`addendum-1.md`, spliced above `## New text, by line`, now at 172; `inputs.py verify` uncited 0).
- The fix's test-plan §7 clause, raised at the resume and answered «Amend now» (inputs#I17): main's own text at
  test-plan.md:670, its sweep, its sidecar entry, a re-run leaf batch (14 of 14 pass; 3 applied, 1 by hand, 7 declined)
  — `cascade-dispositions.md` §The resume pass; `fanout-results.md` test-plan 5.
- P6 re-written (state.yaml `last_wrap`, the code-graph stamp 12:12:01Z, the handoff).
- 12:21Z · P7.1 light gate — `gate.py run --plan viola-0.1.0/chunks/2026-10-26-server-verification-before-any-frame/plan.md --run-dir .andromeda/runs/2026-10-26T09-14-57-wrap --marker 2026-10-26-server-verification-before-any-frame`, backgrounded; the longest bounds entry 20 `run --perf` (3600 s) and entry 21 `pre-push` (7200 s), read at 1.86 s and 90.64 s at implement's re-entry.
  Result: `entries 24 · green 19 · red 1 (13) · recorded 1 · timeout 0 · not-run 3` (logs under
  `$TMPDIR/andromeda-gate/…/wrap-2026-10-26T09-14-57`, firing 2). The ASSERT: 21 `pre-push` green at `linux-tests`,
  89.84 s — the red that sent the wrap back is closed; 13 the guard `outside 3 files` — **red — the plan's wording: the
  guard's `--allow` list against `scope-record.md`'s three in-intent lines (`tests/contract_fake_agent_drift.rs`,
  `src/bin/viola-fake-agent.rs`, `tests/cli_fake_agent.rs`) · word: «put the guard's outside-the-plan line for
  tests/contract_fake_agent_drift.rs in the report; accepted as named.» — delegate the overseer (founder-delegated),
  2026-10-26 08:39Z (inputs#I5); «a red met during a chunk folds into it, and the fix targets the measured mechanism» —
  the operator, relayed verbatim by the wrap, 2026-10-26 11:34Z (inputs#I13)**; 1 recorded (tmpfs 25G avail); 22–24
  `leg = 'operator'` re-verified from `evidence/operator-pass.md` §7 (hygiene clean, `7948850a` pushed, CI
  `verdict: green · checks 15/15` ci#40318429755 on the final HEAD `7948850a`).
- 12:24Z · P7.2 drift = 0 (130 applied or resolved, 0 open); P7.3 `claimed 0 — gate no-op`; 3b not fired (Epoch 6: 5
  markerless); 3c `hygiene: clean`, `scope: clean — changed 20`; P7.4 `flip master-route.md:89 … pending -> complete`,
  `compacted 1 complete line(s) -> route-archive.md`.
