# Fan-out results — 2026-09-24-three-os-ci-headless-harness-skeleton

| Doc | Verdict | Proposals | Disposition |
|---|---|---|---|
| architecture | drift | 18 (9 D-arch-resources, 9 D-arch-decisions) | 18 applied (routine: accurate this-chunk addition / expected amendments 1–3). The child-env half of #5 (PATH / CARGO_TARGET_DIR / NEXTEST_PROFILE) was not registered: registry over-reach. Raw: `.raw-fanout-architecture.md` |
| security-plan | drift | 5 | 3 D-security-deps applied (action set + toolchain source, expected amendment 2). D-security-input (`--home` unvalidated) and D-security-auth (R8 strip) rejected as sequencing deferrals (playbook rule 1), owned by route entries "Home and code-bearing file integrity" / "CLI machine contract" (CARRY pins at P5) and "PTY wrapper on Windows" (already names the strip). The D-security-auth basis also cited `src/run/mod.rs`, a re-derivation tell. Raw: `.raw-fanout-security-plan.md` |
| design-system | clean | 0 | — (every surface `tokens n/a`) |
| layout-templates | clean | 0 | — (`viola run` + `--home` already in §Surface: cli) |
| test-plan | drift | 21 (18 D-tests-obs-harness, 3 D-tests-framework) | 21 applied (routine; expected amendments 4–6 + Spec claims disproved 1–4), plus 2 orchestrator-raised dependents (line 537 E2E command feature, line 541 "detached") and a §12 Decisions Log entry for the new closed values. Raw: the conversation return; the summary is in the test-plan sidecar |
| obs-plan | drift | 2 D-obs-stack | 2 applied (the fake agent's lint exemption). The detector's code-side notes route to CARRY pins at P5: raw `tracing` macros → `obs_event!` (Diagnostics plane); `duration_ms` / `run.start` span (PTY wrapper). Raw: `.raw-fanout-obs-plan.md` |
| a11y-plan | clean | 0 | — |

**Validate checks:**
1. **Playbook:** 44 routine (accurate this-chunk addition), 2 rejected (sequencing deferral), 0 escalated. D-security-input's `escalate` severity is overridden by the playbook's routine verdict, and its owner exists in the route.
2. **Cross-contradiction:** none. arch §Diagnostic output channels and obs §3 now agree on the home-level role file.
3. **Intent:** consistent (scope val-1 amendment already covered the diff form; operator P4 decisions cover the triggers).
4. **Absence-evidence:** every "0 hits" claim carries its grep in the sidecars.
5. **Expected amendments 1–6:** all proposed or raised.
6. **Disproved claims 1–5:** 1–4 by test-plan amendments. 5 (key order) needs no spec change (the spec was right; the code was fixed); it routes to curation.

**Escalations:** 0. **Drift = 0:** yes.
