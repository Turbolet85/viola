# Curation — 2026-09-25-security-prerequisites

CLAUDE.md ecosystem curated:
  Tier 1 (CLAUDE.md USER:session-learnings):  none
  Tier 2 (.claude/rules/*):                   0 new
  Tier 3 (.claude/docs/session-learnings.md): none
  Extended: T2/testing.md: "2026-09-25: Every new guard test carries its remove-the-guard run …" + "re-Read the guard's site before each neutralising edit; the rustfmt hook reflow makes a pre-format anchor miss and the run reads green on the unmodified guard" (confidence 0.8)
    Proof: at /implement, remove-the-guard run (c) on `tests/contract_content_hash.rs`. The anchored Edit on the pre-format `#[case::abc(b"abc", "…15ad")]` line failed ("String to replace not found"), because the PostToolUse hook had reflowed the case over four lines. The chained nextest run read `5 tests run: 5 passed` on the unmodified file. A re-Read and re-anchored edit then produced the red reading (`rtg-c.log`, 4 passed / 1 failed). Earlier in the same session the hook reflowed `tests/channel_sqos_open.rs` twice (the `SQOS_OPEN` const) and the scratch probe once.
  Filters: 3 dup (test-only-target mutation verdict → test-plan §3/§10 + testing.md/verification-harness.md bodies this wrap; non-overlapped adoption hang → security-plan/arch + services/viola-channel.md; 0BSD per-crate exceptions → security-plan §Dependency Security + rules/security.md) · 0 task-specific · 0 conflict · 3 below threshold (host default toolchain stable-gnu 0.2; CARGO_HOME at D:/dev/rust/cargo 0.2; licence texts sourced from the registry 0.2) · 0 deferred
  No-other-home: "re-Read the guard's site before a neutralising edit (formatter reflow hazard)"
  CLAUDE.md size: 121/200 · T1 1.3 KB, 0 over 600 B
