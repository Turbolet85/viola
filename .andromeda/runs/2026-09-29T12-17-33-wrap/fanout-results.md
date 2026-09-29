# Fan-out results — wrap of 2026-09-29-sideloaded-conpty

Seven Explore doc-agents, one parallel batch, prompt from `amendment-flow.md` §Fan-out verbatim (report
`viola-0.1.0/chunks/2026-09-29-sideloaded-conpty/report.md`). Stripping removed only each return's trailing `#`
commentary block (its substance is in the verdict lines below). Entity probe over the returns: 0 entities.

## Verdicts
- architecture — 14 proposals (D-arch-resources 5 · D-arch-decisions 9)
- security-plan — 12 proposals (D-security-input 4 · D-security-auth 2 · D-security-deps 6); stripped note: no
  contradiction found, every gap doc-side; the Threat Model Summary lines 114/120/134/160-161 restate retired claims but
  are the verbatim copy (not proposed); the auth library check passes (sha2 digest, no dependency added); the two
  Decisions Log proposals should land as one `2026-09-29` entry.
- design-system — 0 proposals (`proposals: []`, no stripping change): every Coverage row `tokens n/a`, no UI element.
- layout-templates — 0 proposals (`proposals: []`, stripped note only): no user-facing surface; `viola run` still prints
  nothing while the child runs (layout-templates.md:355, 489-490); the cli Tooling line (:322) still accurate.
- test-plan — 6 proposals (D-tests-coverage 6); stripped note: D-tests-framework no drift (nextest via the harness,
  tempfile unit homes fit §2), D-tests-obs-harness no drift (additive field + new string value only).
- obs-plan — 6 proposals (D-obs-instrumentation 6); D-obs-stack and D-obs-pii no drift.
- a11y-plan — 2 proposals (D-a11y-surface 2); stripped note: D-a11y-obs-schema no drift (a11y rows validate against
  `a11y-row.v1.json`, never diag-line); a11y-plan:793 holds on both backends; §1 line 94 verbatim and accurate.

## Parsed lists + dispositions

### architecture
A1. D-arch-resources · Occupied Resources → Filesystem (viola home) · register `bin/<version>-<hash>/conpty/{OpenConsole.exe,conpty.dll}` (Windows x64; embedded; write-if-absent via `pin_companions`; FILE_SHARE_READ held re-hash; mismatch left as found → inbox ConPTY, `sideload_fallback`) · basis architecture.md:388
   → **apply** (playbook "Accurate this-chunk addition"; expected amendment §Occupied Resources → Filesystem)
A2. D-arch-resources · Occupied Resources → Repository · register `vendor/conpty/<version>/x64/{conpty.dll,OpenConsole.exe}` (MIT nupkg, pins in `src/conpty.rs` text, `.gitattributes` binary, `scripts/conpty-vendor.sh` sole writer/verifier) · basis :401
   → **apply** (same rule; expected amendment → Repository)
A3. D-arch-resources · Occupied Resources → Repository · register test-side `target/conpty-seed/<key>/` (per-run seed by `seed_conpty`) · basis :409-428
   → **apply** (same rule — a target/ subtree the tests write; report Harness/gate surface)
A4. D-arch-resources (dependent-of D-arch-resources) · Infrastructure Patterns → Deployment model · bin/ also holds `conpty/` on Windows x64, preloaded before spawn · basis :455
   → **apply** (dependent of A1)
A5. D-arch-resources (dependent-of D-arch-resources) · Infrastructure Patterns → Project directory structure · add `vendor/conpty/<version>/x64/`, `scripts/conpty-vendor.sh`, `src/conpty.rs`, viola-pty `sideload` · basis :486-532
   → **apply** (dependent of A1/A2; merged with A14 at apply — same tree)
A6. D-arch-decisions · Stack and Technologies → PTY layer row · the Windows x64 host is the vendored Microsoft ConPTY 1.24.260710001 (not a crate); windows-sys covers the System32 restriction + absolute-path preload · basis :17
   → **apply** (accurate this-chunk addition; the report's Dependencies bullet)
A7. D-arch-decisions · Established Decisions → [PTY] · As-built clause: restriction in `main`, preload by absolute path, `pty_backend()` values, fail-open fallback, H2 pair 0/200 vs 14/200 beside 13/200, the sideload preamble + DA1 stall, uncovered spawns · basis :47
   → **apply** (expected amendment [PTY]; disproved-claim 2 disposition, arch half)
A8. D-arch-decisions · [Session Liveness] "As landed" chain · insert `run.conpty_sideload` after pinned copy and plugin · rationale cites `src/cmd/run.rs:168-179`
   → **reject — re-derivation tell** (a source location the report does not carry); the fact re-enters as **R1** (check 5).
A9. D-arch-decisions (dependent-of D-arch-decisions) · [Session Liveness] prose start order · second site
   → **reject** with its primary A8 (dependent-of group); re-enters inside **R1**.
A10. D-arch-decisions · [Deployment / Distribution] · embedded companions, write-if-absent, fail-open on mismatch, first-start cost ~1.2 s handled test-side · basis :99
   → **apply** (expected amendment; disproved-claim 1 disposition, arch half — written with its measured scope: this host, parallel suite)
A11. D-arch-decisions (dependent-of D-arch-decisions) · [Snapshot writer] · companions join the `replace_private_shared` users; bounded error-32 held-open retry · basis :51
   → **apply** (accurate this-chunk addition; report Symbols/APIs + Deviations)
A12. D-arch-decisions (dependent-of D-arch-decisions) · Stack → Content hash row · sha2 also re-hashes the companions · basis :23
   → **apply**
A13. D-arch-decisions (dependent-of D-arch-decisions) · Infrastructure Patterns → CI/CD approach · windows-2025-only `ConPTY vendor verification` step · basis :573
   → **apply** (the runner-tool clause dropped: the report names the host's tools, not the runner's)
A14. D-arch-decisions (dependent-of D-arch-decisions) · Project directory structure · viola-pty comment names `sideload` · basis :501
   → **apply** (merged with A5)

### security-plan
S1. D-security-input (escalate) · Input Validation → boundary table · new row: sideloaded companions + DLL search order (restriction in `main`, held-handle SHA-256 pin, absolute-path preload, closed `sideload_fallback`, codes only) · basis :219-239
   → **apply — escalation resolved**: playbook "Boundary widening" (escalate) + "what ratifies it"; the widening (commit + embed; held-handle re-hash before Epoch 6) was shown at the phase P4 forks and answered by the founder live 2026-09-29 10:41:12, relay the Viola overseer (plan Provenance; this wrap's directive confirms). The DLL-search restriction is a control, not a widening. Recorded as the founder's.
S2. D-security-input (dependent-of) · Security Anti-Patterns → Universal · bans: bare-name DLL load before the restriction; relative-path preload; companion load without the held-handle re-hash; overwrite/delete of a failing companion · basis :576-584
   → **apply** (with S1; the "never load a DLL by bare name" wording re-derived — portable-pty's own bare-name load is the one the preload satisfies)
S3. D-security-input (dependent-of) · Data Protection → Code-bearing artefacts · companions join the trusted-file list · basis :265-269
   → **apply** (with S1; expected amendment)
S4. D-security-input (dependent-of) · Security Anti-Patterns → Data Protection · the bin/ re-hash ban extends to the companions · basis :528
   → **apply** (with S1)
S5. D-security-auth · Authentication & Authorization → `~/.viola/` access control · the companions and the third dated interim gap · basis :207
   → **apply — re-derived**: the companions are NOT added to the strict-modes lists as checked (they are not checked until Epoch 6 — that is the gap); the row names them as Epoch 6's to add, and the third gap. Windows only (no Unix companion). Founder ratification as S1.
S6. D-security-auth (dependent-of) · Security Decisions Log · third interim gap bullet · basis :719-747
   → **apply** (merged into one `2026-09-29` entry with S12)
S7. D-security-deps (escalate) · Dependency Security → Audit tool · the vendored binaries sit outside every lockfile graph; their own gate `conpty-vendor.sh --verify/--probe` · basis :307-311
   → **apply — escalation resolved** (founder ratification as S1)
S8. D-security-deps (dependent-of) · Dependency Security → Pinning · four pins, one textual home `src/conpty.rs` · basis :323-331
   → **apply**
S9. D-security-deps (dependent-of) · Dependency Security → CI integration · the windows-2025 vendor-verification step · basis :335-349
   → **apply**
S10. D-security-deps (dependent-of) · Bootstrap phases → dep-audit-tooling-install · add `conpty-vendor.sh` · basis :391
   → **apply**
S11. D-security-deps (dependent-of) · Bootstrap phases → dep-security-ci-gate · add the step · basis :407
   → **apply**
S12. D-security-deps (dependent-of) · Security Decisions Log · `2026-09-29` vendored Microsoft ConPTY binaries, founder live 10:41:12 relay the Viola overseer · basis :588-755
   → **apply** (expected amendment; one entry carrying S6's bullet too)

### test-plan
T1. D-tests-coverage · §5 → Module ↔ PTY row · H2 pair ci#36563868040 (sideload 0/200, inbox 14/200, image windows-2025-vs2026 20260828.587, no rate) beside 13/200; root `viola run` tests on the sideload, the three uncovered spawners inbox; `conpty_sideload` + the two-sided unit test · basis :927
   → **apply** (expected amendment §5)
T2. D-tests-coverage · §9 → Pipeline structure · the `ConPTY vendor verification` step + the `conpty_sideload` binary · basis :1448
   → **apply** (expected amendment §9)
T3. D-tests-coverage (dependent-of) · §9 → Build failure conditions · vendor verify/probe failure · basis :1484
   → **apply**
T4. D-tests-coverage (escalate) · §5 → Setup / teardown lifecycle · Windows x64 seeded-home carve-out (`seed_conpty`), first-start write covered only by unseeded starts · basis :931
   → **ESCALATE** (report Decisions & corrections: "The test-home convention needs a ruling for seeded homes"; disproved-claim 3)
T5. D-tests-coverage (escalate, dependent-of) · §3 → Test data bootstrap → Mechanism · `seed_conpty` · basis :738
   → **ESCALATE** with T4 (group)
T6. D-tests-coverage (escalate, dependent-of) · §7 → Seed strategies (On-disk product state) · second exception · basis :1352
   → **ESCALATE** with T4 (group)

### obs-plan
O1. D-obs-instrumentation · §4 Scenario 1 → Must-trace spans · `run.conpty_sideload` (Windows) after `run.pin_copy` · basis cites `src/cmd/run.rs:168-179`, `:660-670`
   → **reject — re-derivation tell**; re-enters as **R2** (check 5).
O2-O6. dependents of O1 (§4 attributes for `run.conpty_sideload` [basis cites `src/cmd/run.rs:279-294`]; §4 `pty.spawn` `pty_backend` values; §4 `process-start{claude-child}` `sideload_fallback`; §6 `process-start` catalog; §12 D-36)
   → **reject** with O1 (dependent-of group); each re-enters inside **R2**.

### a11y-plan
Y1. D-a11y-surface · §3 → Keyboard test harness (Tooling) · the Windows preamble names both hosts: inbox (fact 4) and sideloaded `ESC[1t ESC[c ESC[?1004h ESC[?9001h` with DA1; oracle on both backends · basis :624
   → **apply** (expected amendment a11y §3, the preamble differs; disproved-claim 2 disposition, a11y half; "the host answers DA1" re-derived — the terminal, or the piped test driver, answers it; viola stays silent)
Y2. D-a11y-surface (dependent-of) · §6 → CLI equivalent · credits both hosts' own bytes · basis :947
   → **apply**

**T4-T6 resolved** (AskUserQuestion): "Carve-out + Epoch 6 CARRY" — the operator's word, the overseer agreeing ("The seed
is test-side and byte-identical, the unseeded starts keep the product write covered, and the Epoch 6 CARRY keeps the
DACL reason from lapsing silently"). Applied to §5, §3, §7 + a §12 `2026-09-29` Log entry; the CARRY pins at P5.

## Orchestrator raises (Validate check 5 — expected amendments no surviving proposal covers)
- **R1** architecture [Session Liveness] — `run.conpty_sideload` in both start-order sites, after the pinned copy and plugin,
  before the strip plan and version gate (report Symbols/APIs `src/cmd/run.rs` bullet; Acceptance 6). The chain's
  "program resolution → strip plan → collision" order is corrected in the same pass: the report places the strip plan
  after the sideload step. → **apply, routine**.
- **R2** obs-plan §4 Scenario 1 (chain + `run.conpty_sideload` attributes + `pty_backend` values + `sideload_fallback`
  on `process-start`), §6 `process-start` catalog, §12 D-36 — all from the report's Schema/config + Symbols/APIs +
  Counts. §1 untouched (playbook "Verbatim scope copy"). → **apply, routine**.

## Checks
1. Playbook: apply rows under "Accurate this-chunk addition"; S1/S7 groups under "Boundary widening" + "what ratifies it" —
   resolved by the founder's live ratification (shown at phase P4, answered 10:41:12, relay the Viola overseer). No new
   widening met.
2. Cross-contradiction: none (S6 + S12 land as one Log entry; A5 + A14 as one tree edit).
3. Intent-consistency: every scope-record line (companion 3 · in-intent 10) holds its `serves`; the report's deviations are
   justified by measurement or the overseer's word; the seeded homes are the one open intent question → T4 escalation.
4. Absence: each applied row read at its basis line before editing (long lines by window).
5. Expected amendments: all 12 entries covered (10 by proposals, 2 by R1/R2).
6. Disproved claims: (1) first-start cost → A10 + T4's resolution; (2) preamble/DA1 → A7 + Y1 + a P5 pin (the owner
   entry); (3) test-home convention → T4 escalation.
