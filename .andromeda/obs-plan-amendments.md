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

## 2026-09-24-workspace-tree-and-code-graph-planes — supply-chain artifact admitted by content (operator ratification)
**Section:** §8 PII Scrubbing → Integration points, item 6 (Verification), "Unscanned uploads, admissible by content"
**Change:** A third inventory bullet covers the ci.yml `supply-chain` artifact (`deny.json`, `deny-fuzz.json` for the new `fuzz/Cargo.lock` audit, `zizmor.json`; `if: always()`, 7 days).
- It holds cargo-deny and zizmor diagnostics only, with no absolute path, argv, log content or user input: as measured at this chunk, 0 absolute paths in all three files. The one raw pattern hit was the `s:/` inside `https://`.
- The guard stays binding: a member that could carry a host path or log content is scanned before upload, or dropped.
- This also closes the pre-existing omission: `deny.json` and `zizmor.json` had uploaded unscanned since chunk 2026-09-24-supply-chain-and-workflow-gates without an inventory entry.
**Why:** chunk 2026-09-24-workspace-tree-and-code-graph-planes (report Changes: Schema/config `deny-fuzz.json`; Coverage row). D-obs-pii proposed it at severity escalate, and it is the playbook's never-routine Boundary widening class. It was escalated at wrap P2 and ratified by the overseer (founder-delegated, logged for founder review): "admissible by content as measured (0 absolute paths in deny.json, deny-fuzz.json, zizmor.json); the guard clause stays binding".
**Sweep** (same pass `sweep.py`, the `unscanned uploads` / `two unscanned` families plus a leaf grep for `before any upload` and `mutants-verdict`): obs-plan hits after the apply number 0 stale (the §9 Platform line :1232 only points to §8 item 6). Leaves re-derived: `.claude/docs/obs-summary.md` (the secret-scan row now reads "before any diagnostics-bearing upload", pointing to §8 item 6 for the unscanned set) and `.claude/docs/commands.md` (the secret-scan line, same correction). Both had said "before any upload", over-broad since the quality-gates ratification.

## 2026-09-24-epoch-1-cleanup — §9 Mutation row names the per-leg unviable rule
**Section:** §9 Pipeline integration → the Mutation row
**Change:** after "a mutant is red only when no leg caught it", the row adds that before that union a leg whose unviable mutants outnumber its caught ones is red at its own run (test-plan §10 Mutation gate).
**Why:** a cross-master citation of test-plan's mutation verdict, which this chunk amended (test-plan sidecar, same marker). Flagged out of detector scope by the obs-plan doc-agent and folded by the cascade step-2 sweep. obs-plan §1 (the verbatim obs-scope copy) was not touched: the new playbook rule "Verbatim scope copy", appended by this wrap, keeps it out of every sweep.
**Sweep:** the same pass pattern (`sweep.txt`) found 1 obs-plan hit, `:1253`, amended. The streaming change adds no obs event, field or sink. Its lines carry repo-relative names and durations only, and `mutants.out/` stays un-uploaded (obs-plan §8), so no other obs section moves.

## 2026-09-25-pty-wrapper-on-windows — env_kept, D-34 supersedes D-14, batch-script-child from resolution, run refusal stderr
**Section:** §3 diagnostic output channels preamble · §4 Scenario 1 (`run.pin_copy` outcome, claude-child `process-start` fields) · §4 Edge flow E2 · §6 additive field catalog (`process-start`) · §7 Platform pick · §8 data classification (R8 row) · §11 Logs (print-macro allow list) · §12 Decisions Log (new D-34)
**Change:**
- `process-start{subject:"claude-child"}` gains `env_kept` (kept `CLAUDE*` names, names only) in §4 and the §6 catalog; D-34 supersedes D-14: every stripped and kept name is logged, the 14-name compile-time list is gone, values are never read.
- `batch-script-child` is decided by program resolution before the strip plan and `pty.spawn`, not by `run.pin_copy`.
- `run` writes two fixed stderr lines before any spawn on the `.cmd`/`.bat` refusal; the refusal fn is the `run` path's one local `print_stderr` allow.
- §7: `PtyError`'s fixed `Display` is hand-written (no thiserror in `viola-pty`).
**Why:** chunk 2026-09-25-pty-wrapper-on-windows report Changes (Schema / config, Symbols), expected amendment 5, operator ruling 1; PtyError exception ratified at wrap P2 (E2). Rejected as sequencing (playbook rule 1): making `viola-pty` / `viola-agent-claude` tracing-free in §3 — obs §3/§4 require seam spans (`pty.spawn` and the viola-pty seam operations), no `#[instrument]` exists workspace-wide yet, and "Wrapper channel" owns the first spans; a route CARRY there names `pty.spawn` and where the seam spans live.
**Sweep:** patterns as the architecture entry of this chunk plus `D-14|env_stripped_known|pin_copy.{0,40}batch`: obs :556, :857, :868, :968, :1057, :1119, :1176, :1389 amended; :1556-1560 (the D-14 entry, historical — "do not modify historical entries") no change; §1 :52 (verbatim scope copy, playbook rule) no change; the generic "fixed thiserror `Display`" statements :503, :798, :1132, :1198 stay true (the thiserror types keep fixed `Display`s; `PtyError` is fixed too), no change. Leaves: `.claude/docs/obs-summary.md`, `.claude/rules/observability.md` recomputed, no change; `.claude/docs/gotchas.md` and `services/viola-agent-claude.md` now cite D-34.

## 2026-09-26-ci-chunk-base-and-union-verdict — chunk.diff out of the secret scan and the harness upload; compiling-leg union
**Section:** §8 PII Scrubbing → Integration points, item 6 (`harness-<os>` bullet) · §9 Telemetry artifact handling (`agent-run logs` / `harness-<os>` row) · §9 Pipeline integration (Mutation row) · §9 Step order and conditions, step 3
**Change:**
- The scan applies the Critical-class patterns to `target/agent-run/*` except exactly `target/agent-run/chunk.diff` (the mutation leg's diff, repository source text by construction); the `harness-<os>` upload excludes the same file (`!target/agent-run/chunk.diff`), so it is never scanned and never uploaded; a `chunk.diff` elsewhere under the capture is still scanned.
- Mutation row: each mutant judged only by the legs whose `#[cfg]`s compile its line (every leg when none does); a missed obs-code mutant on its compiling leg stays red.
**Why:** chunk 2026-09-26-ci-chunk-base-and-union-verdict report Symbols/APIs (`secret_scan::scan`, `union`), Harness / gate surface, Spec claims disproved 2, Expected amendments (obs-plan). The Mutation row was raised by the orchestrator (Validate check 5): no obs detector owns gate semantics. The three D-obs-pii proposals carried detector severity `escalate`; applied as routine because the scan narrows only for a file the upload also drops (no boundary widening) and the overseer's recorded wrap direction named the exclusion on both sides.
**Sweep:** the test-plan entry's 12 patterns; obs-plan rows :1206, :1240, :1253, :1285 amended; the §12 D-27 entry (:1669) is history, no change. Leaves: `docs/obs-summary.md:29` and `rules/observability.md` state no scan scope or union — no change. Fanned 3 proposals (D-obs-pii, 2 `dependent-of`) + 1 orchestrator-raised, all applied with text re-derived from the report.
