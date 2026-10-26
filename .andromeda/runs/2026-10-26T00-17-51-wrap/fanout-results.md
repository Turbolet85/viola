# Fan-out results — 2026-10-25-windows-endpoint-admission · wrap

Run dir `.andromeda/runs/2026-10-26T00-17-51-wrap/`. Every file passed `check` and the split (re-spawns included:
stage1-r1, strong-r1, newdetail-r1). The files are the record; this lists verdicts and dispositions only.

## Verdicts
- architecture: 1 proposals · fanout-architecture.md · 1 cascade proposals · fanout-architecture--cascade.md · new-detail fanout-architecture--cascade-new-cross-cutting-patterns.md (1), fanout-architecture--cascade-new-occupied-resources.md (0) · 3 facts · 2 placed · 1 no home · (sampler) lost 0 · beside a placed fact 1
  - note: no other drift; the new test pipes stay in the registered `viola-test-chan-<pid>-<label>` namespace (architecture.md:261, true). F1 (the windows-sys dev-dependency) no home in occupied-resources: its home is the key file Crate dependency direction.
- security-plan: 0 proposals · fanout-security-plan.md · 4 cascade proposals · fanout-security-plan--cascade.md · new-detail fanout-security-plan--cascade-new-dependency-security.md (2) · 5 facts · 5 placed · 0 no home · (sampler) lost 0 · beside a placed fact 1
  - note: body line 434 («The default DACL grants read to Everyone and anonymous») measured true by the default-descriptor control; no body drift.
- design-system: 0 proposals · fanout-design-system.md · 0 cascade proposals · 0 facts · (sampler) lost 0
- layout-templates: 0 proposals · fanout-layout-templates.md (whole master, no cascade)
- test-plan: 2 proposals · fanout-test-plan.md · 13 cascade proposals · fanout-test-plan--cascade.md · new-detail fanout-test-plan--cascade-new-test-strategy.md (1) · 10 facts · 10 placed · 0 no home · (sampler) lost 1 · beside a placed fact 1
- obs-plan: 0 proposals · fanout-obs-plan.md · 0 cascade proposals · fanout-obs-plan--cascade.md · 0 facts · (sampler) lost 0
- a11y-plan: 0 proposals · fanout-a11y-plan.md · 0 cascade proposals · 0 facts · (sampler) lost 0

Fixes removed nothing (no `PREAMBLE`, `COMMENTARY` or escape fix was owed; every failure was re-spawned).

## Dispositions
Opening rule (a source the report does not carry): 0 rejected — every `basis` is a master line, the report, its New
text section (`Cargo.toml:168`, `src/main.rs:281`, `:640`, `tests/security_negatives_channel.rs:124`, `:206`, `:218`,
`:229`, `:245`, `:272` all within its listed ranges) or the chunk's evidence the report cites.

fanout-architecture.md
1. apply — check 1: accurate this-chunk addition (the report's Dependencies: the new root `[target.'cfg(windows)'.dev-dependencies]` windows-sys table); check 5: the plan's expected amendment architecture §Infrastructure Patterns → Crate dependency direction; inputs#I7 item 6 · as: in the key file `.andromeda/registries/contracts/architecture/crate-dependency-direction.md`, the root `viola` package's entry states its test-only `cfg(windows)` dev-dependency on windows-sys (the workspace pin) with BOTH features `Wdk_Storage_FileSystem` and `Win32_System_IO` — `NtQueryInformationFile` and its `IO_STATUS_BLOCK` argument need both — for the pipe-type read-back of `tests/security_negatives_channel.rs`; both already reach the Windows graph by unification, so `Cargo.lock` does not move and no release build carries them; no chunk name or date in the text

fanout-test-plan.md
1. apply-merged → fanout-test-plan--cascade.md 2 — the same line, `.andromeda/test-plan.md:46` (§1 `viola-channel` broad line); one rewrite
2. apply-merged → fanout-test-plan--cascade.md 13 — the same line, `.andromeda/test-plan.md:606` (§6 Security control negatives broad line); one rewrite

fanout-architecture--cascade.md
1. reject — check 1, the fact stated once: the tokio advisory and its owner are security-plan's (Dependency Security, fanout-security-plan--cascade-new-dependency-security.md 2); the stack row's tokio 1.53.1 pin stays true (Dependencies: none bumped) · check 7: architecture-newgroup-stack-and-technologies.md · distinct

fanout-architecture--cascade-new-cross-cutting-patterns.md
1. reject — check 1, the fact stated once: the Windows boundary's CI witnesses land in security-plan §Authentication & Authorization (fanout-security-plan--cascade.md 1, 2), which the cross-cutting body line names as the owner of per-OS enforcement · check 7: architecture-newgroup-cross-cutting-patterns.md · distinct

fanout-security-plan--cascade.md
1. apply — check 1: accurate this-chunk addition; check 5: the plan's expected amendment security-plan §Authentication & Authorization (IPC access control (Windows)) · check 7: security-plan-newgroup-authentication-authorization.md · distinct · as: a new detail group below the IPC access control (Windows) record, no chunk name: the remote-client rejection is asserted on CI's `test (windows-2025)` by `security_negative_viola_listener_rejects_remote_clients` (a client end of a bound listener, its `NamedPipeType` read through `NtQueryInformationFile` / `FilePipeLocalInformation`, the `FILE_PIPE_REJECT_REMOTE_CLIENTS` bit set), beside two test-owned control pipes that tell the bit apart (accept-remote clear, reject-remote set)
2. apply — check 1; check 5 (same entry) · check 7: security-plan-newgroup-authentication-authorization.md · distinct · as: in the same group, no chunk name: the DACL's effect is asserted too — `security_negative_viola_listener_denies_an_anonymous_open`, the listener bound first, then a `GENERIC_READ` open under `ImpersonateAnonymousToken` denied `ERROR_ACCESS_DENIED` (5), beside `security_negative_control_default_pipe_admits_an_anonymous_read` (the same open admitted on a default-descriptor pipe)
3. reject — check 1, the fact stated once: the witnesses stand in authentication-authorization (1, 2) and test-plan §1; the threat model's trust-boundary line 48 already hands per-OS enforcement to security · check 7: security-plan-newgroup-threat-model-summary.md · distinct
4. reject — check 1: bootstrap-phases derives route / setup phases; a CI-witness status and a run's re-reading («re-read as passing») are status and provenance away from their owner, which is authentication-authorization (1, 2); a dependent whose primary is placed elsewhere

fanout-security-plan--cascade-new-dependency-security.md
1. reject — check 1, Registry over-reach (playbook): windows-sys is already the stack's pinned crate; the root's dev-dependency features are realization inside it, recorded once in architecture's key file Crate dependency direction (fanout-architecture.md 1) · check 7: security-plan-newgroup-dependency-security.md · distinct
2. apply — check 1: accurate this-chunk addition (the report's Outcome: the full CI row's one red, not this chunk's, with its owner by the founder's yes on the P5 card, inputs#I6) · check 7: security-plan-newgroup-dependency-security.md · distinct · as: ONE broad line, a new `values · deps-supply-chain` group, no run ids and no chunk name: the open advisory RUSTSEC-2026-0386 (`unsound`) against the locked tokio 1.53.1 fails CI's `supply-chain` job; it is owned by the working-route entry "tokio RUSTSEC-2026-0386 lock bump" (a lock bump to `>=1.53.3`), with no ignore-list widening — RUSTSEC-2017-0008 stays the only advisory ignore

fanout-test-plan--cascade.md
1. apply — check 6: the report's first disproved claim (test-plan §1 Test Scope Summary, the `viola-channel` untestable zone); check 5: the plan's expected amendment test-plan §1 · as: `.andromeda/test-plan-detail/test-scope-summary.md:11` in place, its «since chunk …» provenance dropped
2. apply — check 6; check 5 (same entry); fanout-test-plan.md 1 rides with it · as: `.andromeda/test-plan.md:46` in place, the broad line: remote pipe clients and the Windows pipe DACL asserted on `test (windows-2025)`; only the other-euid peer cannot be driven on hosted runners and is asserted at the peer-credential check boundary
3. apply — check 5: the plan's expected amendment test-plan §4 Unit Test Strategy · check 7: test-plan-newgroup-unit-test-strategy.md · distinct · as: no chunk name
4. apply — check 1: accurate this-chunk addition (the witness) · check 7: test-plan-newgroup-unit-test-strategy.md · distinct
5. apply — check 1: the founder's own yes on the P5 card's Beyond-the-plan row «Same-class captures left out, no owner» (inputs#I6; inputs#I7 item 5) · check 7: test-plan-newgroup-unit-test-strategy.md · distinct · as: no chunk name; the three captures named by file, never observed red, no owner
6. reject — check 1, Registry over-reach (playbook): the Module ↔ IPC Tools line already names windows-sys 0.61.2, true; the root's dev-dependency features are recorded once in architecture's Crate dependency direction (fanout-architecture.md 1) · check 7: test-plan-newgroup-integration-test-strategy.md · distinct
7. apply — check 1: accurate this-chunk addition, written as a pointer (the fact's home is §6) · as: append to the Module ↔ IPC Test approach one clause, no chunk name: the pipe's admission itself — remote clients rejected, an anonymous open denied — is asserted in §6 Security control negatives (`tests/security_negatives_channel.rs`)
8. apply — check 5: the plan's expected amendment test-plan §6 Security control negatives · check 7: test-plan-newgroup-e2e-test-strategy.md · distinct · as: no chunk name
9. apply — check 5 (same entry) · check 7: test-plan-newgroup-e2e-test-strategy.md · distinct
10. apply — check 5 (same entry) · check 7: test-plan-newgroup-e2e-test-strategy.md · distinct · as: no chunk name
11. reject — the case's internal call order (bind before impersonating, revert in the same call) is the test code's own (`tests/security_negatives_channel.rs` `anonymous_read`), not the strategy's; the learning is P3 curation's (the report's Decisions & corrections) · check 7: test-plan-newgroup-e2e-test-strategy.md · distinct
12. reject — check 1, Registry over-reach (playbook): the `viola-test-chan-<pid>-<label>` namespace is registered in architecture §Occupied Resources (architecture.md:261, true); the per-case labels are realization · check 7: test-plan-newgroup-e2e-test-strategy.md · distinct
13. apply — check 5: the plan's expected amendment test-plan §6 (the broad line); fanout-test-plan.md 2 rides with it · as: `.andromeda/test-plan.md:606` in place, naming the Windows listener rejecting remote clients and denying an anonymous open, each beside its controls

fanout-test-plan--cascade-new-test-strategy.md
1. apply — check 1: the founder's own answer to P4 fork 4 (inputs#I5: plain `cargo test` unsupported, its known-cause reds recorded with no owner) · check 7: test-plan-newgroup-test-strategy.md · distinct · as: no chunk name; nextest is the runner, plain `cargo test` is not supported, and under it the three named root-bin reds are known-cause and recorded with no owner

(sampler) cascade-compare-architecture.md
S1. reject (sampler) — check 1, Registry over-reach and the fact stated once: the root's dev-dependency features are recorded in architecture's key file Crate dependency direction (fanout-architecture.md 1); the stack table's Wrapper IPC row names the layer's windows-sys role, true · check 7: architecture-newgroup-stack-and-technologies.md · distinct

(sampler) cascade-compare-security-plan.md
S1. reject (sampler) — check 7: security-plan-newgroup-dependency-security.md · twin · fanout-security-plan--cascade-new-dependency-security.md 2 — its fact is that proposal's

(sampler) cascade-compare-test-plan.md
S1. apply (sampler) — check 1: accurate this-chunk addition (the five cases' test-owned control pipes and the anonymous-token principal) · check 7: test-plan-newgroup-mocking-stubbing-discipline.md · distinct · as: the Fake servers row gains, in its two cells and no chunk name, windows-sys `CreateNamedPipeW` test-owned control pipes (Windows only: accept-remote, reject-remote, the default descriptor) beside the real listener in the listener-admission negatives, and `ImpersonateAnonymousToken` standing in for a foreign principal; no call-order detail
S2. apply (sampler) — check 1: accurate this-chunk addition (the V1 controls asserted in §6) · check 7: test-plan-newgroup-test-strategy.md · distinct · as: the trigger map's IPC vector (V1) row also traces to §6 Security control negatives (root `security_negatives_channel.rs`, the Windows listener's remote-client rejection and DACL denial), no chunk name

## Checks 2–6
- check 2, cross-contradiction: two pairs edit the same line in one direction (test-plan.md:46 and :606) — merged, no opposition.
- check 3, intent-consistency: scope record none (`gate.py scope` clean, 0 recorded); the three deviations are justified in the report and inside the intent (test code only). No intent amendment.
- check 4, absence-needs-evidence: no proposal claims an absence or a caught-ALL; the newgroup readers read each target file whole.
- check 5, the plan's Expected amendments (wrap): test-plan §1 → cascade 1, 2 · test-plan §6 → cascade 8, 9, 10, 13 · test-plan §4 → cascade 3, 4 · architecture Crate dependency direction → fanout-architecture.md 1 · security-plan §Authentication & Authorization → security cascade 1, 2 · `matrix#v1-44` notes → written at phase P5, nothing owed at P7.3 · the testing.md curation candidate → P3. Every entry covered; nothing raised by main. `citation-dispositions.md`: 0 rows, no `claim false`.
- check 6, disproved claims: test-plan §1 untestable zone → cascade 1, 2 · the plan's one-feature step 5 → no master site; the landed text names both features (fanout-architecture.md 1) and the plan-premise correction is P3's · the CARRY's capture mechanism → cascade 3, 4 (the helper and its witness) and P3's testing.md 2026-09-27 extension.
- check 7: ten verdict files, 22 verdicts — 21 distinct, 1 twin; no duplicate, no contradiction, no «no fitting category».

Counts: 29 proposals (stage1 3 · strong readers and new-detail passes 22 · sampler 4, the `## Lost` items) → 17 apply · 2 apply-merged · 10 reject · 0 escalate.
