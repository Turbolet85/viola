# Curation — wrap 2026-11-17T14-02-45Z of 2026-11-17-supply-chain-green-and-booted-tests-end

CLAUDE.md ecosystem: no new learnings this session.
  Filters: 3 dup · 0 task-specific · 0 conflict · 0 deferred · 1 below threshold

## Dispositions — implement's *Decisions & corrections*
- Bash-guard refusals (two `cd`, one `cat` heredoc with a file target): not a learning — the setup Bash guard refuses the very act, and `host-linux.md` already states both forms.
- Sweep hazard (a shared helper taken by `#[path]` carries its own `#[cfg(test)]` cases into every taker; an rstest table is untakeable by viola-e2e): rejected — Filter 1 duplicate: this wrap landed it in `test-plan · decision/rstest-case-tables` (`except`, tx 2354) and the cascade wrote it into `.claude/rules/testing.md`'s generated body.
- Instrument validity (a process-and-dir kill witness cannot separate a tied verify from an untied one; the stamp's absence does): rejected — Filter 1 duplicate: `test-plan · decision/tied-verify-kill-witness` (tx 2354) and `.claude/rules/testing.md`'s generated body (this wrap's cascade).
- Leftovers by construction (a SIGKILLed verify leaves its empty Run A dir, removed by hand): rejected — Filter 1 duplicate of `.claude/rules/security.md`'s body («SIGKILL, a crash and a failed handler install leave them to be removed by hand»).

## Dispositions — `## After implement`'s *Decisions & corrections after implement*
- The pass: none recorded.
- The wrap directive's rulings (`inputs#I3`): not learnings — each is this wrap's work (the standing row, the v2-37 note, the TERM leaf, the route placement, the boundary carry).

## Dispositions — the conversation
- The pre-push total counts each rstest case as one test, so a case table turned into one labelled `#[test]` drops the count by (cases − 1) per binary that takes it (measured: 3510 → 3462, the close's Outcome, final): rejected — Filter 4, 0.6 (verified by measurement +0.4, specific technical detail +0.2, reached no other durable home +0.2, could be task-specific −0.2), exactly at threshold.
- The ci profile's 20 s kill does not hold for `harness_lifecycle` (its readiness-timeout case reads 20.644 s): not a learning — a route-resolve finding (P5), owned there.
- The cascade pattern id bound (16 chars): not a learning — tool telemetry, recorded as friction.
