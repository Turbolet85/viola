## 2026-09-29-t15-07-57-wrap — registry migration (U35): the security-plan Decisions Log leaves the body
**Section:** §Security Decisions Log · §Authentication & Authorization · §Data Protection · §Dependency Security (CI integration; after the omitted Supply chain integrity note) · §Security Anti-Patterns → Code Patterns, Universal
**Change:**
- The log moved verbatim to security-plan-amendments-archive.md: 14 entries, from the `2026-09-23` initial entry through the `2026-09-29` cross-session-message tag.
- Authentication & Authorization: the accepted risk that a driver LLM can be prompt-injected into `answer` `allow` (viola's part is only the closed `behavior` enum and the `unverified-cli` gate).
- Data Protection: the accepted local-availability risk — another local user can hold GUI connections or SSE streams open (no bound in v1), and unbounded `events.ndjson` growth can exhaust local disk (availability only).
- Dependency Security → CI integration: `scripts/release-check.sh` judges the build's own artifact records and refuses any executable other than `viola` and any `test-support` / `fake-agent` artifact (`--probe` 5/5). `test-support` is enabled only by the root package's `[dev-dependencies]`.
- Dependency Security → CI integration: G2's one exact-path exemption (`src/cmd/hook/seam.rs:<digits>`, whole-string compare), its look-alike `--probe` run before every check, and the rule that the perf arm's own check exempts nothing.
- Dependency Security: the v1.x release prerequisites (dist attestations and signing, cargo-auditable + `cargo audit bin`, self_update `signatures`, zizmor cache-poisoning, no `rust-cache`).
- Anti-Patterns → Code Patterns: `run --mutants` wipes only the guarded host mutation scratch (`scratch-refused` / `scratch-wipe-failed`, no fallback, the path never printed).
- Anti-Patterns → Universal: no non-`null` dialog decision while `run`'s stamps read skips strict-modes (the PREREQ on "Dialog answers by dialog_id").
**Why:** a Decisions Log is keyed by time, so it is history, not current truth. Its in-force items now stand in the body.
**Ref:** .andromeda/runs/2026-09-29T15-07-57-wrap/
