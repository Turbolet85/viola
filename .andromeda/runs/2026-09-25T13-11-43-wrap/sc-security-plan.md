
## 2026-09-25-security-prerequisites — SQOS spike passed, SHA-256 crate picked, 0BSD per-crate exceptions
**Section:** §Authentication & Authorization (IPC client-side server verification row); §Data Protection (code-bearing artefacts); §Dependency Security (`deny.toml` additions, new `[licenses]` bullet); §Bootstrap phases (`viola-channel` client and `viola-state` re-hash bullets); §Security Decisions Log (the two Open questions removed; new `2026-09-25` entry).
**Change:**
- The client-verification row now states that the adoption spike passed (a same-user server reads `SecurityIdentification`), that `FILE_FLAG_OVERLAPPED` is required, and that the default connect reads `SecurityImpersonation` and stays banned.
- §Data Protection names `sha2 =0.11.0` (`default-features = false`) as the SHA-256 implementation.
- §Dependency Security records `allow` unchanged plus two per-crate `0BSD` exceptions (`doctest-file`, `recvmsg` via interprocess 2.4.4), never through `allow`; each further exception needs its own entry.
- Both bootstrap bullets cite the resolved decisions.
- The Decisions Log `2026-09-25` entry carries the measurements, conditions and ratification.
**Why:**
- The chunk's report: Dependencies; Cross-project claims (scratch probe, levels 1/2, the hang, interprocess `ReOpenFile`); Spec claims disproved #3; Expected amendments.
- CI run 36138441784 on `8e25ca7`: windows `test` leg PASSed the witness.
- The 0BSD exceptions widen a hardened boundary (playbook "Boundary widening", escalate). The escalation is resolved by the operator's ratification, recorded here: overseer, founder-delegated, at the phase P4 fork ("per-crate exceptions") and again in the wrap P2 directive item (2).
**Sweep** (cascade step 2, over the seven masters + CLAUDE.md + `.claude/rules/*` + `.claude/docs/session-learnings.md` + playbook + drift-base):
- Patterns: `open spike|spike \(Decisions Log|Spike: does|replacement SQOS open|must pass, or its replacement`, `open question.{0,40}(SHA|hash)|(SHA|hash).{0,60}open question|must first be picked|SHA-256 implementation is an open`, `no hash crate|pick.{0,20}hash`. All 0 hits after the apply.
- Known-positive control: the same patterns over `git show HEAD:.andromeda/security-plan.md` = 4 hits (:205, :265, :373, :375 and :609/:611 open questions), all amended.
- The detector's sweep found :265, which the report's site list missed.
- `windows-sys` restatements across the masters were read (29 lines): all consistent. security-plan :189/:202/:207/:374/:589 describe the same client open or unrelated DACL work, so no change.
- Leaves re-derived:
  - `.claude/docs/security-summary.md` (Open questions → a new "Resolved prerequisites" section);
  - `.claude/rules/security.md:27` (the licence allowlist + 0BSD exceptions);
  - `.claude/docs/stack.md` (Security-plan additions);
  - `.claude/docs/services/viola-channel.md:29`;
  - `.claude/docs/services/viola-state.md:30`.
- 0 hits in CLAUDE.md `GENERATED` blocks, the curation homes, playbook and drift-base.
