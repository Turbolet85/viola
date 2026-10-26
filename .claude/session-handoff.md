# Session Handoff

**Last Updated:** 2026-10-26T12:20:31Z
**Branch:** build/viola-0.1.0 · 0 ahead of origin/build/viola-0.1.0 as read at this wrap's resume (HEAD `7948850a` pushed by the operator pass; the wrap commit pushes after this file)
**Status:** clean
**Last Commit:** the wrap commit of 2026-10-26-server-verification-before-any-frame, on `7948850a` fix(2026-10-26-server-verification-before-any-frame) — the fake-agent exit fix (ci#40318429755: `verdict: green · checks 15/15`)

## Position
- Done: **2026-10-26-server-verification-before-any-frame** → `complete` (this wrap, resumed at P7 step 0 from
  `.andromeda/runs/2026-10-26T09-14-57-wrap/`, `inputs#I16`).
- Next: the working route's first markerless entry, **"Explicit session links"** (Epoch 6) → `/andromeda-phase`.

## Work done
- Every frame-writing client (the six CLI frame verbs, `viola hook`'s event and dialog paths, `viola mcp`'s four frame
  tools) checks the serving process's pid and start time against `snapshot.json` before its first frame on Linux and
  Windows: CLI exit 21 `instance-unreachable` with a new hint, MCP `instance-unreachable`, the hook fails open, each
  logged `server-verify-failed`. macOS stays on the recorded gap.
- Two folded test-only fixes: the ETXTBSY fix in the agents drift test (`inputs#I5`), and the wrap's P7.1 red — one
  cut-short `viola-fake-agent` coverage profile — closed by the fake agent ignoring SIGHUP from its exit's first step
  (`inputs#I13`; a session leader cannot leave its group, so not the directive's literal wording). Fix commit `7948850a`,
  pushed from a clean worktree (`inputs#I15`), CI green. Evidence: `chunks/…/evidence/fake-agent-exit-hangup.md`.

## Drift resolved
- 140 proposals (forced sample): 129 applied across all seven masters, 6 rejected, 5 escalated and resolved
  (`inputs#I9`); fork CARRY 7 / 8 recorded as the founder's own ruling (`inputs#I10`, `inputs#I11`).
- At the resume: test-plan §7 (`test-plan.md:670`) gains the fake agent's exit SIGHUP disposition (`inputs#I17`), its
  sidecar entry and its leaves (rules/verification-harness.md, plus the re-run leaf pass's two stale hook-path fixes in
  viola-channel.md and rules/events.md and testing.md's ETXTBSY planting rule). 130 amendments in all.
- Route: the P5 card's four re-pins as CARRYs on "Home and code-bearing file integrity" ×2, "Unix endpoint and home
  hardening", "Windows-only live measurements".

## Notes
- **Owed outside this repo:** the pipeline defects in the friction log for the founder's desk — F350, F351, the
  2026-10-26T07:01:56Z-b record, this wrap's records `2026-10-26T10:58:26Z-a`…`-j` and `2026-10-26T11:01:39Z-b`…`-d`
  (among them the check-7 contradiction-against-a-retired-line pattern for overseer1, `inputs#I9`), and implement's
  `2026-10-26T11:45:44Z-b` (a directive naming a mechanism a session leader cannot take).
- **Left in the tree:** the six listing dirs `cascade-{a11y,arch,design,obs,security,tests}/` in
  `.andromeda/runs/2026-10-26T06-27-48-wrap/` — their removal (`git rm -r`, a commit of its own) is ruled for
  viola-overseer (`rulings.md:697`).
- **Carried from earlier handoffs:** the founder's next batch, the CARRYs, the boundary-audit list, the founder's desk,
  the operator's-word items, the standing rules (in git at `d35644a5`'s `.claude/session-handoff.md`); no epoch boundary
  inside Epoch 6 (the founder's word, 2026-10-20T10:17Z); read `df -h /tmp` before a stamped-home gate.
- **Host recipe:** the macOS-target check-only clippy (`CARGO_TARGET_DIR=target/maccheck … --target aarch64-apple-darwin`)
  read green before this pass's push, beside `target/wincheck`.
- **Last failed command:** none open — the earlier `pre-push` red (entry 21) closed by the fake-agent exit fix.

## Session End Status
Completed normally at 2026-10-26 15:58:30
