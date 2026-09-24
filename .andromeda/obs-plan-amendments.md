# obs-plan — amendments

## 2026-09-24-three-os-ci-headless-harness-skeleton — fake agent's print-ban exemption
**Section:** §3 Bootstrap phases → obs-ci-gate-wire · §11 Obs Anti-Patterns → Logs
**Change:**
- Only `viola-e2e` omits `[lints] workspace = true` and carries its own `[lints.clippy]`.
- The fake agent is a `[[bin]]` of the root `viola` package. Lints are per package, so it inherits the root `[lints]` and is exempted by a crate-level `#![allow(clippy::print_stdout, clippy::print_stderr)]` in `src/bin/viola-fake-agent.rs`.
- The CI member-list assertion names only `viola-e2e`.
- The §11 carve-out lists that bin-level allow.

**Why:** report Changes > Files (`src/bin/viola-fake-agent.rs` is a root-package bin) and Deviation 7: a separate lint table is impossible for it. Sweep `only these two lack|these two members omit|own \[lints\.clippy\]` over all 7 masters:
- obs-plan.md:790 and :1373 amended;
- obs-plan.md:1748 no change (§12 Decisions Log history, B5);
- 0 in the other masters.

The leaf `.claude/rules/verification-harness.md` §Exemptions was re-derived.

## 2026-09-24-supply-chain-and-workflow-gates — Platform: nightly.yml beside the single push/PR ci.yml
**Section:** §9 Pipeline integration → Platform
**Change:** `ci.yml` stays the single push/PR workflow. The scheduled `nightly.yml` sits beside it and runs only the weekly `cargo deny check advisories`.

**Why:** chunk 2026-09-24-supply-chain-and-workflow-gates. It is a plan-carried expected amendment, raised by the orchestrator under Validate check 5; no detector proposed it. The P4 operator decision put the weekly run in `nightly.yml`. Sweep: see architecture-amendments.md, same entry heading. For this master, :1217 was amended. :406, :777, :1239, :1378 and :1443 were left unchanged, because they are still true.

## 2026-09-24-diagnostics-plane — obs_event! field set, schema default-deny keyword
**Section:** §3 Observability Harness Contract (intro quote, Logging stack) · §8 PII Scrubbing & Compliance → Default-deny posture (+ Detail-file scope) · §11 Obs Anti-Patterns → Logs
**Change:**
- **§3 Logging stack:** `obs_event!` attaches `event`, plus `process` and `instance` from `ProcessCtx`. `corr` is a caller-supplied typed field; the macro has no dedicated arm, because such an arm is ambiguous with the generic `key = value` arm in `macro_rules`. As measured at the chunk.
- **§3 intro:** the arch harness pattern is re-stated from arch §Diagnostic output channels instead of the retired upstream quote.
- **§8 keyword:** default-deny is enforced by a top-level `unevaluatedProperties: false`, not per-event `additionalProperties: false`, which cannot see `allOf`/`if-then` properties. `diag-detail.v1.json` is self-contained, with the event enum inlined.
- **§11:** the raw-tracing ban's justification drops `corr` from what the macro guarantees.
**Why:** chunk 2026-09-24-diagnostics-plane, report Spec claims disproved #1 and Symbols/Reverted bullets.
- The §8 pair was applied as routine: accurate mechanism reconcile, invariant held.
- The §3/§11 `corr` pair was escalated and resolved WITH the operator: amend to shipped. The trade-off is closed by a CARRY on the wrapper-channel chunk: make `corr` `required` in `diag-line` for every corr-bearing event, with a negative test.

Sweep over all seven masters and the leaves:
- **Patterns:** `additionalProperties`; `attaches .event.|always attaches|attaches .{0,20}.corr.|guarantees .event., .corr.`.
- **Amended:** obs :605, :1201, :1202, :1366.
- **No change:** obs :1203 is the new text.
- **Leaf re-derived:** `.claude/rules/observability.md:24`.
- **Result:** 0 retired-claim hits remain in the masters.
- **Control:** `unevaluatedProperties` fired.
