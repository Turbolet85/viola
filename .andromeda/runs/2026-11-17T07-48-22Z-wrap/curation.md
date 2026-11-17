# Curation — wrap 2026-11-17T07-48-22Z-wrap · chunk 2026-11-17-standing-live-rig

```
CLAUDE.md ecosystem curated:
  Tier 1 (CLAUDE.md USER:session-learnings):  none
  Tier 2 (.claude/rules/*):                   + verification-harness.md: "A change to what the fake agent draws under `--screens` reaches `cli_verify`'s `viola verify --record` recorded-screen cases … puts `cli_verify` in its gate block."
  Tier 3 (.claude/docs/session-learnings.md): none
  Filters: 0 dup · 2 task-specific / not a learning · 0 conflict · 0 deferred (1 recurrence → handoff)
  Extended: T2/verification-harness.md: "2026-10-10: On the Linux dev host the bare name `claude` can resolve to a newer CLI …" + "a plan's gate entry naming an installed CLI's path is read against the host's install before the review card"
  No-other-home: "a plan's gate entry naming an installed CLI's path is read against the host's install before the review card"
  No-other-home: "a fake-agent screen change puts `cli_verify` in the gate block"
  CLAUDE.md size: 134/200 · T1 3.9 KB, 0 over 600 B
```

## Applied
- T2 verification-harness.md, new entry 2026-11-17 (fake-agent screens → `cli_verify`) — confidence 0.8 (verified by measurement +0.4 · specific detail +0.2 · no other home +0.2).
  Proof: pre-push firing 1 read `cli_verify runs::verify_record_screens_keep_only_signature_rows` red after the fake's trust rows were drawn in place of the recorded `Screen.modal`; the plan's block did not select `cli_verify` (`chunks/2026-11-17-standing-live-rig/evidence/operator-14-firing-1.txt`; the report's Gate selection gap bullet).
- T2 verification-harness.md, extension of the 2026-10-10 entry (an installed CLI's path read against the host before the card) — confidence 0.8 (verified by measurement +0.4 · specific detail +0.2 · no other home +0.2).
  Proof: the stopped run's entry 10 `test -x "$HOME/.local/share/mise/installs/claude/2.1.325/bin/claude"` read red (exit 1) at the live round; the install holds `<version dir>/claude` with no `bin/` (`evidence/round-070702Z.txt`; outcome of run 2026-11-17T06-49-10Z-implement).

## Decisions & corrections, each weighed
Implement's:
- The operator's run rules (cap 2 + 1, ledger before start, «a red that moves a plan entry stops you») — not a learning: this chunk's run rules, task-specific (Filter 2).
- The live 2.1.325 trust form sets the fake's rows — not a learning: landed in test-plan contract/fake-agent-screens/trust-dialog at P2 (a master holds it).
- Permission-classifier refusal `[Undermining Oversight]` of a by-hand rehearsal script, not retried — not a learning: handled as the standing CLAUDE.md learning on classifier refusals reads (a verdict given, not routed around).
- Evolve refused an estimated `ts` — recurrence of CLAUDE.md «A timestamp written into any record … is read from `date -u`» → handoff Deferred learnings as `recurrence-despite-learning`.
- Pre-push red on two of the chunk's own tests outside the block — applied (the new T2 entry above; the WITHIN half is held by the contract lint itself).
- Bash-guard refusals at Setup (cd; pipe into grep) — not a learning: the guard working (each re-issued once).
- Kept live trust frame holds the scratch cwd path — not a learning: security-plan decision/live-session-screens-kept-local holds it.
After implement's:
- The overseer's wrap directive (inputs#I9) — applied through P2 (rows) and P5 (route); its two curation candidates are the two applied entries above.
- During the pass: none.
This wrap's conversation:
- `--respawn` takes comma-joined names — not a learning: pipeline telemetry (friction record), the friction stream is not curated.
