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

## 2026-09-24-log-redaction-and-never-log-floor — anyhow scope follows the catch-site reporter
**Section:** §7 Error Capture & Reporting, Platform pick (:1117)
**Change:** :1117 now says anyhow 1.0.104 is used "at the root-bin dispatch edge and its catch-site reporter (`viola::obs::report_internal_error`) only".
**Why:** cascade step 2 of the arch amendment in this pass (architecture-amendments, same marker). :1117 restated the dispatch-only scope while the chunk's reporter in `src/obs.rs` now renders the chain (report Changes → Symbols / APIs). §7 scrubbing layer 3 (`chain:[...]` only in `detail-<process>.ndjson`, only when an instance resolves) already matched the shipped behaviour and is unchanged.
**Sweep:** the arch entry's patterns and dispositions; in obs: amended :1117. No change at :54: it sits in §1, the verbatim copy of obs-scope.md that keeps its pending wording by rule (obs :485), the same disposition an earlier arch entry gave §1 hits; its quote of arch's retired "context chains only at dispatch" stays, and §7 carries current truth.
**Leaves re-derived:** none needed — `.claude/docs/obs-summary.md` and `.claude/rules/observability.md` carry neither sentence (grep `anyhow|dispatch edge|platform pick|local error capture` → 0 hits in obs-summary; observability.md :19 already states the chain goes only to detail files).

## 2026-09-24-observability-gates — raw-tracing ban: level-macro path ban plus a fail-closed raw-`event!` grep
**Section:** §3 Bootstrap phases (logger-stack-install exemption bullets; obs-ci-gate-wire bullet 1) · §8 PII Scrubbing integration point 1 · §9 Pipeline integration Lint / typecheck row · §10 Build / deploy failure conditions · §11 Logs (raw-tracing rule enforcement) · §12 Decisions Log (new D-33)
**Change:**
- The inner-`#[allow(clippy::disallowed_macros)]` exemption is retired.
- `clippy.toml` `disallowed-macros` bans `tracing::{info,warn,error,debug,trace}` by path.
- Raw `event!` is caught by a fail-closed grep in `scripts/lint-probes.sh`, which is proven both ways and runs on the Linux lint leg.
- `obs_event!`'s inner allow stays but has no effect.
- A `lint-probes.sh` failure is now a build failure.
- D-33 records the supersession of D-25's clause; D-25 is left as written.
**Why:** as measured on clippy 1.98.1 at this chunk (`.andromeda/runs/2026-09-24T12-21-11-implement/clippy-disallowed-macros-measurement.md`), no allow placement inside the macro, at the call site or on the calling fn exempts the inner `event!`. Only a caller-crate `#![allow]` does, and with `tracing::event` listed, all 16 `obs_event!` sites failed. The operator ratified the replacement at /implement P1 (overseer, founder-delegated). Report: "Spec claims disproved" 1, Deviations 1–2.
**Sweep (cascade step 2):**
- Masters, 7 of 7:
  - `tracing::{event,`: 1 hit, obs `:1378`, the §11 rule; no change, since the ban on all six stays true and only its enforcement was amended;
  - `allow attribute inside`: 2 hits, obs `:1641` (D-25, historical, no change) and `:1779` (the new D-33);
  - `inner \`::tracing::event!\``: 1 hit, the amended `:783`;
  - `exemption mechanism`: 0 hits;
  - `disallowed-macros`: every hit is amended text or D-25/D-33.
- Leaves: `.claude/rules/observability.md:24` (the rule stated clippy as the whole enforcement) was re-derived. `.claude/docs/commands.md:45` was re-derived. `obs-summary.md:54` (print bans only) needs no change.
- Curation homes, playbook and drift-base: 0 hits.

## 2026-09-24-quality-gates — mutation consumer, unscanned uploads admissible by content, nightly fuzz
**Section:** §8 PII Scrubbing integration point 6 (new sub-bullet) · §9 Platform · §9 Pipeline integration (Mutation row)
**Change:**
- §8 item 6 records two uploads outside the secret scan, admissible by content: `mutants-verdict-<os>.json` (repo-relative source locations and outcomes only; `mutants.out/` never uploaded) and the nightly `fuzz/artifacts/` on failure (from the synthetic corpus; a non-synthetic seed drops that upload first).
- §9 Platform: `nightly.yml` runs advisories and the fuzz time-box, no longer "only" advisories.
- §9 Mutation consumer: the per-leg verdicts merged by `mutants-verdict`'s union gate.
**Why:** chunk 2026-09-24-quality-gates. The D-obs-pii escalations were resolved at wrap P2 by the overseer's ruling "Ratify both by content", with the verdict file stated as repo-relative only.
**Sweep:** `mutants\.out|outcomes\.json` 2 hits after the apply (`:1209` new text; the Mutation row's new consumer text). `runs only the weekly` 0. `nightly` 1 (the amended Platform). `.claude/docs/obs-summary.md` and `.claude/rules/observability.md` recomputed with no change (neither names the upload inventory, the nightly workflow or the mutation consumer).
