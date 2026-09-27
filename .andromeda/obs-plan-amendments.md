# obs-plan — amendments

## 2026-09-24-three-os-ci-headless-harness-skeleton — fake agent's print-ban exemption
**Section:** §3 Bootstrap phases → obs-ci-gate-wire · §11 Obs Anti-Patterns → Logs
**Change:**
- Only `viola-e2e` omits `[lints] workspace = true` and carries its own `[lints.clippy]`.
- The fake agent is a `[[bin]]` of the root `viola` package. Lints are per package, so it inherits the root `[lints]` and is exempted by a crate-level `#![allow(clippy::print_stdout, clippy::print_stderr)]` in `src/bin/viola-fake-agent.rs`.
- The CI member-list assertion names only `viola-e2e`.
- The §11 carve-out lists that bin-level allow.
**Why:** `src/bin/viola-fake-agent.rs` is a root-package bin, so a separate lint table is impossible for it.
**Kept:** the §12 Decisions Log history entry stays as written.
**Ref:** .andromeda/runs/2026-09-24T07-05-59-wrap/

## 2026-09-24-supply-chain-and-workflow-gates — Platform: nightly.yml beside the single push/PR ci.yml
**Section:** §9 Pipeline integration → Platform
**Change:** `ci.yml` stays the single push/PR workflow. The scheduled `nightly.yml` sits beside it and runs only the weekly `cargo deny check advisories`.
**Why:** the operator's P4 decision put the weekly run in `nightly.yml`.
**Kept:** the other mentions of the single `ci.yml` stay unchanged, because they are still true.
**Ref:** .andromeda/runs/2026-09-24T09-41-13-wrap/

## 2026-09-24-diagnostics-plane — obs_event! field set, schema default-deny keyword
**Section:** §3 Observability Harness Contract (intro quote, Logging stack) · §8 PII Scrubbing & Compliance → Default-deny posture (+ Detail-file scope) · §11 Obs Anti-Patterns → Logs
**Change:**
- **§3 Logging stack:** `obs_event!` attaches `event`, plus `process` and `instance` from `ProcessCtx`. `corr` is a caller-supplied typed field; the macro has no dedicated arm, because such an arm is ambiguous with the generic `key = value` arm in `macro_rules`. Was: the macro always attaches `event` and `corr`.
- **§3 intro:** the arch harness pattern is re-stated from arch §Diagnostic output channels instead of the retired upstream quote.
- **§8 keyword:** default-deny is enforced by a top-level `unevaluatedProperties: false`, not per-event `additionalProperties: false`, which cannot see `allOf`/`if-then` properties. `diag-detail.v1.json` is self-contained, with the event enum inlined.
- **§11:** the raw-tracing ban's justification drops `corr` from what the macro guarantees.
**Why:** the shipped macro disproved the spec's field guarantee.
- The §8 pair was applied as routine: an accurate mechanism reconcile, invariant held.
- The §3/§11 `corr` pair was escalated and resolved with the operator: amend to shipped. The trade-off is closed by a CARRY on the wrapper-channel chunk: make `corr` `required` in `diag-line` for every corr-bearing event, with a negative test.
**Ref:** .andromeda/runs/2026-09-24T10-40-06-wrap/

## 2026-09-24-log-redaction-and-never-log-floor — anyhow scope follows the catch-site reporter
**Section:** §7 Error Capture & Reporting, Platform pick (:1117)
**Change:** the Platform pick now says anyhow 1.0.104 is used "at the root-bin dispatch edge and its catch-site reporter (`viola::obs::report_internal_error`) only". Was: the dispatch edge only.
**Why:** a cascade of this pass's arch amendment (same marker): the chunk's reporter in `src/obs.rs` now renders the chain, so the dispatch-only scope was stale.
**Kept:**
- §7 scrubbing layer 3 (`chain:[...]` only in `detail-<process>.ndjson`, only when an instance resolves) already matched the shipped behaviour and is unchanged.
- §1, the verbatim copy of obs-scope.md, keeps its pending wording by rule, including its quote of arch's retired "context chains only at dispatch"; §7 carries current truth.
**Ref:** .andromeda/runs/2026-09-24T11-37-52-wrap/

## 2026-09-24-observability-gates — raw-tracing ban: level-macro path ban plus a fail-closed raw-`event!` grep
**Section:** §3 Bootstrap phases (logger-stack-install exemption bullets; obs-ci-gate-wire bullet 1) · §8 PII Scrubbing integration point 1 · §9 Pipeline integration Lint / typecheck row · §10 Build / deploy failure conditions · §11 Logs (raw-tracing rule enforcement) · §12 Decisions Log (new D-33)
**Change:**
- The inner-`#[allow(clippy::disallowed_macros)]` exemption is retired.
- `clippy.toml` `disallowed-macros` bans `tracing::{info,warn,error,debug,trace}` by path.
- Raw `event!` is caught by a fail-closed grep in `scripts/lint-probes.sh`, which is proven both ways and runs on the Linux lint leg.
- `obs_event!`'s inner allow stays but has no effect.
- A `lint-probes.sh` failure is now a build failure.
- D-33 records the supersession of D-25's clause; D-25 is left as written.
**Why:** as measured on clippy 1.98.1, no allow placement inside the macro, at the call site or on the calling fn exempts the inner `event!`. Only a caller-crate `#![allow]` does, and with `tracing::event` listed, all 16 `obs_event!` sites failed. The overseer (founder-delegated) ratified the replacement at /implement P1.
**Kept:** the §11 ban on all six `tracing::{event,…}` macros stays; only its enforcement was amended.
**Ref:** .andromeda/runs/2026-09-24T13-07-17-wrap/

## 2026-09-24-quality-gates — mutation consumer, unscanned uploads admissible by content, nightly fuzz
**Section:** §8 PII Scrubbing integration point 6 (new sub-bullet) · §9 Platform · §9 Pipeline integration (Mutation row)
**Change:**
- §8 item 6 records two uploads outside the secret scan, admissible by content: `mutants-verdict-<os>.json` (repo-relative source locations and outcomes only; `mutants.out/` never uploaded) and the nightly `fuzz/artifacts/` on failure (from the synthetic corpus; a non-synthetic seed drops that upload first).
- §9 Platform: `nightly.yml` runs advisories and the fuzz time-box; was "only" advisories.
- §9 Mutation consumer: the per-leg verdicts merged by `mutants-verdict`'s union gate.
**Why:** the escalations over the two unscanned uploads were resolved at wrap P2 by the overseer, who ratified both by content, with the verdict file stated as repo-relative only.
**Ref:** .andromeda/runs/2026-09-24T14-48-15-wrap/

## 2026-09-24-workspace-tree-and-code-graph-planes — supply-chain artifact admitted by content (operator ratification)
**Section:** §8 PII Scrubbing → Integration points, item 6 (Verification), "Unscanned uploads, admissible by content"
**Change:** a third inventory bullet covers the ci.yml `supply-chain` artifact (`deny.json`, `deny-fuzz.json` for the new `fuzz/Cargo.lock` audit, `zizmor.json`; `if: always()`, 7 days).
- It holds cargo-deny and zizmor diagnostics only, with no absolute path, argv, log content or user input: as measured at this chunk, 0 absolute paths in all three files.
- The guard stays binding: a member that could carry a host path or log content is scanned before upload, or dropped.
- This also closes the pre-existing omission: `deny.json` and `zizmor.json` had uploaded unscanned since chunk 2026-09-24-supply-chain-and-workflow-gates without an inventory entry.
**Why:** it is the playbook's never-routine Boundary widening class, so it was escalated at wrap P2 and ratified by the overseer (founder-delegated, logged for founder review): admissible by content as measured, with the guard clause binding. The secret scan runs before any diagnostics-bearing upload, not before every upload; §8 item 6 holds the unscanned set.
**Ref:** .andromeda/runs/2026-09-24T16-23-20-wrap/

## 2026-09-24-epoch-1-cleanup — §9 Mutation row names the per-leg unviable rule
**Section:** §9 Pipeline integration → the Mutation row
**Change:** after "a mutant is red only when no leg caught it", the row adds that before that union a leg whose unviable mutants outnumber its caught ones is red at its own run (test-plan §10 Mutation gate).
**Why:** the row cites test-plan's mutation verdict, which this chunk amended (test-plan sidecar, same marker). obs-plan §1 (the verbatim obs-scope copy) was not touched: the playbook rule "Verbatim scope copy", appended by this wrap, keeps it out of every sweep.
**Kept:** the streaming change adds no obs event, field or sink; its lines carry repo-relative names and durations only, and `mutants.out/` stays un-uploaded (§8), so no other obs section moves.
**Ref:** .andromeda/runs/2026-09-25T11-29-18-wrap/

## 2026-09-25-pty-wrapper-on-windows — env_kept, D-34 supersedes D-14, batch-script-child from resolution, run refusal stderr
**Section:** §3 diagnostic output channels preamble · §4 Scenario 1 (`run.pin_copy` outcome, claude-child `process-start` fields) · §4 Edge flow E2 · §6 additive field catalog (`process-start`) · §7 Platform pick · §8 data classification (R8 row) · §11 Logs (print-macro allow list) · §12 Decisions Log (new D-34)
**Change:**
- `process-start{subject:"claude-child"}` gains `env_kept` (kept `CLAUDE*` names, names only) in §4 and the §6 catalog; D-34 supersedes D-14: every stripped and kept name is logged, the 14-name compile-time list is gone, values are never read.
- `batch-script-child` is decided by program resolution before the strip plan and `pty.spawn`; was by `run.pin_copy`.
- `run` writes two fixed stderr lines before any spawn on the `.cmd`/`.bat` refusal; the refusal fn is the `run` path's one local `print_stderr` allow.
- §7: `PtyError`'s fixed `Display` is hand-written (no thiserror in `viola-pty`).
**Why:** an operator ruling at the chunk; the `PtyError` exception was ratified at wrap P2.
**Kept:**
- Making `viola-pty` / `viola-agent-claude` tracing-free in §3 was rejected as sequencing (playbook rule 1): obs §3/§4 require seam spans (`pty.spawn` and the viola-pty seam operations), no `#[instrument]` exists workspace-wide yet, and "Wrapper channel" owns the first spans; a route CARRY there names `pty.spawn` and where the seam spans live.
- The D-14 entry is historical and unchanged; §1 (verbatim scope copy) is unchanged.
- The generic "fixed thiserror `Display`" statements stay true: the thiserror types keep fixed `Display`s, and `PtyError` is fixed too.
**Ref:** .andromeda/runs/2026-09-25T17-43-18-wrap/

## 2026-09-26-ci-chunk-base-and-union-verdict — chunk.diff out of the secret scan and the harness upload; compiling-leg union
**Section:** §8 PII Scrubbing → Integration points, item 6 (`harness-<os>` bullet) · §9 Telemetry artifact handling (`agent-run logs` / `harness-<os>` row) · §9 Pipeline integration (Mutation row) · §9 Step order and conditions, step 3
**Change:**
- The scan applies the Critical-class patterns to `target/agent-run/*` except exactly `target/agent-run/chunk.diff` (the mutation leg's diff, repository source text by construction); the `harness-<os>` upload excludes the same file (`!target/agent-run/chunk.diff`), so it is never scanned and never uploaded; a `chunk.diff` elsewhere under the capture is still scanned.
- Mutation row: each mutant judged only by the legs whose `#[cfg]`s compile its line (every leg when none does); a missed obs-code mutant on its compiling leg stays red.
**Why:** the Mutation row was raised by the orchestrator, since no obs detector owns gate semantics. The scan-scope proposals carried severity `escalate` but were applied as routine: the scan narrows only for a file the upload also drops (no boundary widening), and the overseer's recorded wrap direction named the exclusion on both sides.
**Kept:** the §12 D-27 entry is history and unchanged.
**Ref:** .andromeda/runs/2026-09-26T20-59-23-wrap/

## 2026-09-26-local-linux-pre-push-gate — local union before the push; no clone path in the `pre-push` document
**Section:** §8 PII Scrubbing item 6 (Verification) · §9 CI Integration → Mutation row
**Change:**
- §8 item 6: the local `viola-harness pre-push` document and the `ubuntu-latest` leg verdict it copies back from the WSL2 clone are never uploaded and carry no absolute path (no Windows repo/home path, no Linux clone/home path); a `pre_push_` unit test and the gate's verdict path-grep entry assert it.
- §9 Mutation row: before the operator push the local `pre-push` computes the same union over the same two legs (equal `base` required) — a filter, never the verdict of record.
**Why:** raised by the orchestrator: both fall outside the obs detectors.
**Kept:**
- The nine "env vars are not a configuration channel" / "never reads env" restatements stay: each bans env as configuration (a level, a service name, a backtrace switch, CI metadata), none is exhaustive over what a `viola` build reads, and the seam configures nothing.
- `VIOLA_*` may not widen fields stays true.
- Obs §3 lists no internal harness subcommand, so the test-plan §3 binding moves nothing there.
**Ref:** .andromeda/runs/2026-09-27T00-51-08-wrap/

## 2026-09-27-instance-state-and-start-order — `already-live` covers the stale refusal; `gone` is taken over
**Section:** §3 Logging stack (run-file collision append) · §4 Scenario 1 (`run.collision_check` outcome; product event order) · §6 `detail` catalog
**Change:** `already-live` is logged for a `live` AND a `stale` holder of the name; a `gone` holder is taken over with no exit-1 line (`run.collision_check` outcome `free`); the product order's `session-start` joins with "Hooks to normalised events" (`run` writes `wheel` + `budget-gate` today).
**Why:** the chunk's refusals disproved the spec. A "spans not yet declared" status line under §4 Scenario 1 was rejected as a Sequencing deferral: the spans are owed by "Wrapper channel", pinned there as a CARRY at route-resolve.
**Kept:** §1 (verbatim scope copy) is unchanged; the statements of a running process's flip threshold stay true.
**Ref:** .andromeda/runs/2026-09-27T06-12-23-wrap/

## 2026-09-27-wrapper-channel — corr required rules in the line schema; mis-shaped conn logged as srv_conn
**Section:** §3 Log format (Required fields: the `corr` rules; Null encoding) · §3 IPC boundaries (additive `conn`)
**Change:**
- `channel-*` corr is null for an id-less `hook.event` and on a response to a frame with no id (-32700 / -32600); a dialog `hook-invoked` (fail-open before `dialog_id`, D-07) and a `hook-decision` with `detail` may be null.
- `schemas/diag-line.v1.json` requires `corr` on `dialog-raised`, `dialog-answered`, `release-from-driver`, `channel-request` unless `hook.event`, `channel-response` unless -32700/-32600, `send-*` when `side` is `wrapper`, and `hook-decision` for `pre-tool-use`/`permission-request` without `detail`; never on `hook-invoked`. Was "optional keys" for `corr` everywhere.
- A peer `conn` outside `^[a-z]+-\d+-\d+-\d+$` or over 64 bytes is logged as `srv_conn` too; the server strips `conn` before dispatch.
**Why:** the chunk made `corr` required where §3 always defines it and disproved the plan's unconditional rule on dialog hooks; the test-plan §3 Log format is the bound twin and carries the same text.
**Kept:** §1 (the verbatim obs-scope copy) and the D-10 / D-26 Decisions Log entries.
**Ref:** .andromeda/runs/2026-09-27T12-33-51-wrap/
