# Curation — wrap 2026-09-28T09-46-16 · chunk 2026-09-28-cli-output-tokens

CLAUDE.md ecosystem curated:
  Tier 1 (CLAUDE.md USER:session-learnings):  + "A tool call refused with no verdict by the permission classifier is a transient outage, not a gate to route around: fill the wait with read-only work and retry the same call." (confidence 0.7)
    Proof: the operator's implement argument ("The shell outage at new-session was transient: retry the shell first; if it still refuses, wait a few minutes and retry rather than working around a gate"); measured: 6 no-verdict refusals at new-session and 2 at implement Setup, the third implement call ran after read-only Setup reads (friction record 2026-09-28T09:11:55Z-b).
  Tier 2 (.claude/rules/*):                   extended verification-harness.md: "2026-09-25: On the Windows host, a diff that carries `#[cfg(unix)]` bodies cannot pass plain `run --mutants` …" + "a shared body whose only killing test is OS-gated … the union is the verdict" (confidence 0.8)
    Proof: gate `run --mutants --file src/cmd/run.rs` red on the Windows host (5 tested, 1 missed: `refuse_stale`, killer `run_refuses_a_stale_name` is `#[cfg(unix)]`; run-archive 169); pre-push and CI unions killed it (ubuntu caught `refuse_stale`, windows caught `refuse_batch_script`; ci#36404931982 `mutants-verdict` breaches []); operator ruling at implement and at this wrap's E1.
  Tier 3 (.claude/docs/session-learnings.md): none
  Filters: 0 dup · 1 task-specific (the `human::` grep call-site hazard) · 0 conflict · 0 deferred · 1 below threshold (the ConPTY SGR-parameter oracle, exactly 0.6; its fact is amended into test-plan §5, so no conditional signal)
  Extended: T2/verification-harness.md: "2026-09-25: On the Windows host, a diff that carries `#[cfg(unix)]` bodies …" + "a shared body whose only killing test is OS-gated"
  CLAUDE.md size: 124/200 · T1 1.8 KB, 0 over 600 B
