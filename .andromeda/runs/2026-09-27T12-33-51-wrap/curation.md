# Curation — 2026-09-27-wrapper-channel (resumed wrap)

Scope: this session (the resumed wrap) + the report's *Decisions & corrections*. The implement / operator-pass
conversation was in the prior window and is gone; corrections only it held are not curated.

CLAUDE.md ecosystem curated:
  Tier 1 (CLAUDE.md USER:session-learnings):  none
  Tier 2 (.claude/rules/*):
    + testing.md: "cargo-mutants runs only the mutated package's tests, so a crate's security property needs a test inside that crate" (0.8)
      Proof: report Decisions & corrections "Measured: cargo-mutants tests only the mutated package" — the viola-channel DACL/redaction mutants needed crate-level tests (report Outcome; v1-20 refs).
    + testing.md: "cargo-mutants mutates `const` initializers; `|`→`^` over disjoint flags is equivalent — one literal pinned by a test" (0.8)
      Proof: report Decisions & corrections "Measured: cargo-mutants 27.1.0 mutates `const` initializers"; report Reverted bullet (flag consts with `|` reverted to literals).
    + testing.md: "a scoped tracing subscriber as sole dispatcher caches an off-thread callsite as disabled — capture through a process-global subscriber" (0.8)
      Proof: report Decisions & corrections "Measured: a scoped tracing subscriber as the sole dispatcher caches a callsite first hit on another thread as disabled".
  Tier 3 (.claude/docs/session-learnings.md): none
  Filters: 0 dup · 0 task-specific · 0 conflict · 3 deferred (→ handoff) · 3 rejected by other homes (a Linux reader sees an append part-way → test-plan §2 this wrap; idle WSL VM memory → test-plan §12 this wrap; "the child holds nothing of viola's" → test-plan §6 E2 this wrap)
  Deferred (max-3 cap): the `"777"` digit-substring sweep hazard over a document carrying a git sha (0.8); the founder's
    ruling "a boundary widening halts for a live answer; an earlier direction never ratifies it" (0.7 — its home is the
    playbook's Boundary widening rule, proposed at the wrap card); the overseer's "a red found now folds into this chunk
    even outside its diff; a green re-run never closes a red" (0.7).
  No-other-home: the three Tier-2 entries (other signals 0.6 each: measurement +0.4, specific detail +0.2)
