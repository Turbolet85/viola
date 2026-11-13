# Session Handoff

**Last Updated:** 2026-11-13T07:05:42Z
**Branch:** build/viola-0.2.0 · 0 ahead of origin/build/viola-0.2.0 as read at this wrap's Setup (this wrap's push lands its commit)
**Status:** clean
**Last Commit:** 2026-11-13-verify-readiness-on-the-input-box — verify reads readiness off the input box; supply-chain green

## Position
- Done: 2026-11-13-verify-readiness-on-the-input-box — the input box reads up on a footer literal or its own rows
  (`Signatures::input_box_up`, verify and `viola run`'s gate); live verify on claude 2.1.323: `stamped 2.1.323  23 pass
  1 fail` (statusline-rate-limits), `box_only_guards` 3; three reasoned rmcp ignores + rustix 1.1.6, CI ci#43377140230
  green; v2-29 verified.
- Next: «Epoch 8 cleanup», then «Rows-proposal rule work» (`/andromeda-phase`).

## Work done
- Code + the operator pass (pre-CI `68a4efba`, pushed); wrap: 16 row operations in 4 masters (TX 882–885), 10 leaf
  edits, 3 Tier-2 learnings, 1 CARRY on «Status-line interception removed» (the box-only fixture).

## Drift resolved
- The readiness, recorded-screen, box-wait and stamps-measured rows (architecture, security-plan, test-plan, a11y-plan)
  and the advisory rows (security-plan retired 2, added `rmcp-advisory-ignores`, `rustix-advisory-floor`; test-plan
  `except` ×2), all under the founder's own F1/F2 word (`inputs#I9`, 2026-11-13T06:32:35Z); the answer card's confirm
  was the overseer's as delegate — PROVISIONAL as a confirm only.
- Rejected: 2 at Validate, 1 by fit (test-plan `decision/screen-signatures-only`, `not_master`; it stands true).

## Notes
- The founder's F1 «Утвердить ignore» and F2 «Утвердить добавление» supersede `inputs#I5`'s delegate answers; F2 is
  ratified as a boundary widening.
- The unpin of `vb.sh`'s 2.1.296 shim is the operator's (the P5 card's «Beyond the plan» row; no route entry).
- **Curation conflict — user review:** a live-start cap counts every `claude` process, so a version is read via `mise
  ls claude`, never by running the binary — against verification-harness.md's 2026-10-10 entry «reads `claude --version`
  by path before its first start». Not applied.
- **Owned by 0.2.0** (carried): the 3600 s hold ceiling, PROVISIONAL; the live round's two reds; matrix#v1-27 deferred.
- **Carried, unchanged:** `chunks/2026-10-26-explicit-session-links`'s card confirm and P2 halt;
  `chunks/2026-10-27-bounded-inputs-at-every-boundary`'s P2 halt Q1 and Path 1's runner-variance reading;
  `obs-scope-summary#20.5` and `slo-invariants-telemetry-budgets#2.2` label `max` readings the report labels `mean`.
- **Deferred learnings (carried):** as at the 2026-11-06 wrap (eight items, unchanged).
- **Host:** python `yaml` is absent; read `df -h /tmp` before a stamped-home gate. Health check 13 warns
  `agent-run.ps1` missing: a false red on a Linux-only tree. claude 2.1.323 is `latest` and stamped.
- **Last failed command:** none.
