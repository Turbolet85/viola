# obs-plan — archived amendment originals

Writer = wrap P7 only · read by NO loop skill · cold history, never cited for current truth; each run's originals under its own heading.

# Consolidated at the 2026-09-27-wrapper-channel wrap — 12 re-worded · 0 pruned

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

## 2026-09-26-local-linux-pre-push-gate — local union before the push; no clone path in the `pre-push` document
**Section:** §8 PII Scrubbing item 6 (Verification) · §9 CI Integration → Mutation row
**Change:**
- §8 item 6: the local `viola-harness pre-push` document and the `ubuntu-latest` leg verdict it copies back from the WSL2 clone are never uploaded and carry no absolute path (no Windows repo/home path, no Linux clone/home path); a `pre_push_` unit test and the gate's verdict path-grep entry assert it.
- §9 Mutation row: before the operator push the local `pre-push` computes the same union over the same two legs (equal `base` required) — a filter, never the verdict of record.
**Why:** chunk 2026-09-26-local-linux-pre-push-gate report Harness / gate surface, Coverage of new surfaces (PII), Expected amendments 12-13. Raised by the orchestrator at validate check 5 (the obs doc-agent returned `proposals: []` and flagged #12/#13/#17 as outside its detectors).
**Sweep:** the test-plan entry's 23 patterns (+4 hand-controlled). obs-plan rows :1208 area (new bullet) and :1254 amended. Expected amendment #17: the nine "env vars are not a configuration channel" / "never reads env" restatements (:39, :249, :604, :606, :1139, :1279, :1385, :1415, :1565) each read — every one bans env AS CONFIGURATION (a level, a service name, a backtrace switch, CI metadata), none is exhaustive over what a `viola` build reads, and the seam configures nothing: no change. :1405 (`VIOLA_*` may not widen fields) true. Leaves: `docs/obs-summary.md` re-derived (PII bullet); `rules/observability.md` states neither claim — no change. Bind test-plan §3 ↔ obs-plan §3: obs §3 lists no internal harness subcommand — no change. Full row list: `.andromeda/runs/2026-09-27T00-51-08-wrap/sweep-dispositions.md`.

## 2026-09-27-instance-state-and-start-order — `already-live` covers the stale refusal; `gone` is taken over
**Section:** §3 Logging stack (run-file collision append) · §4 Scenario 1 (`run.collision_check` outcome; product event order) · §6 `detail` catalog
**Change:** `already-live` is logged for a `live` AND a `stale` holder of the name; a `gone` holder is taken over with no exit-1 line (`run.collision_check` outcome `free`); the product order's `session-start` joins with "Hooks to normalised events" (`run` writes `wheel` + `budget-gate` today).
**Why:** chunk report Refusals and Spec claims 5; Expected amendment 7. Proposal O1 (a "spans not yet declared" status line under §4 Scenario 1) rejected as a Sequencing deferral — the spans are owed by "Wrapper channel", pinned there as a CARRY at route-resolve.
**Sweep:** rows :620, :856, :1073 this pass's text; :869 amended (fold); :72, :185, :307, :421 in §1 (verbatim — no change); :767, :1310, :1322 state the flip threshold of a running process — true. Leaf re-derived: `docs/obs-summary.md` (:32 — the flip is tested through `classify` with an injected age). Full rows: `runs/2026-09-27T06-12-23-wrap/cascade-sweep.md`.

## Registry migration (U35) — 2026-09-29

<!-- U35 · obs-plan.md · ## 3. Observability Harness Contract · sha256 9a9a560ec8a2259b29c0acb7ad30fb3709ebd86cea9d2077563645da5a4a96be -->

## 3. Observability Harness Contract

_[ALL tiers — central agent-driven mechanism; Section consumed by
setup-project to materialize OTel SDK init + log sinks + service
identity + snapshot generation]_

The architecture commits to the harness pattern (arch §Cross-cutting Patterns → Diagnostic output channels):
- `hook` writes only its decision body to stdout and nothing to stderr.
- `run` writes nothing to the terminal except the child's own output, with one exception before any child is spawned: a start refusal (`.cmd`/`.bat` child, live or stale name, tampered pinned copy, squatted endpoint) writes exactly two fixed stderr lines (e.g. `unable: <name>'s command is a .cmd or .bat script` / `hint: pass the real executable, not a .cmd or .bat shim`, no path or pid) through `src/human.rs` `refuse` → `write_refusal`, one `write_all` on the locked stderr, a closed pipe swallowed — the root bin's only human-stderr writer, called only by `run`. While the child runs, `run` writes nothing of its own.
- Every role's codes-only process log goes to its home-level role file under `diagnostics/`, and content-bearing detail goes only to the instance's `diagnostics/detail-<role>.ndjson`.
- `mcp` writes only MCP frames to stdout.
- `ui` and short-lived CLI verbs may use stderr for their human output.
- The logger and the format are owned by obs.

This section specifies the concrete contract.

### Product mode — Normal vs Inverted (self-observing)

**Normal.** The self-observing-inversion trigger did not fire (obs-scope §5; Founder Direction 3: "Normal mode, not inverted: viola is not a telemetry product."). `events.ndjson` is a product audit log, not a collector. The Normal contract below applies, with the Minimal-tier exporter carve-outs from obs-scope §6.

### OTel SDK init

- **SDK packages:** **No OTel SDK in v1.** The OTel API role is filled by **tracing 0.1.44** (the facade in every instrumentable crate) and **tracing-subscriber 0.3.23** (the formatter and sink).
  - Researched and deferred to the root-bin edge only: `opentelemetry` / `opentelemetry_sdk` 0.33.0 and the `tracing-opentelemetry` 0.34.0 bridge. With no exporter they would add only dependency and cargo-mutants surface, and bridged span IDs would have no consumer.
  - If either is ever adopted, the Resource must be built with `Resource::builder_empty()`, never `Resource::builder()`: the latter's `EnvResourceDetector` reads `OTEL_*` env vars.
  - Cargo entry for every instrumentable crate: `tracing = "0.1.44"`.
  - Cargo entry for the root bin only: `tracing-subscriber = { version = "0.3.23", default-features = false, features = ["fmt", "json", "registry", "std"] }`. `ansi` and `tracing-log` are off (D-11). The `chrono` feature is off too: it only enables `ChronoUtc` / `ChronoLocal`, and `MillisUtc` uses the root bin's direct `chrono` dependency (D-26).
- **Init order** (one `viola_obs_init(role)` in the root bin, run in obs-scope §3 order):
  1. The **first statement in `main`** is `std::panic::set_hook(viola_panic_hook)`. The hook writes to a `OnceLock<Arc<File>>` that is empty until step 4, so it never falls back to stderr.
  2. clap parse.
  3. Resolve the home (`--home` → grandparent of `VIOLA_DIR` → `~/.viola/`), the instance and the role (`run` / `hook` / `mcp` / `ui` / `cli`). The instance source is fixed per role:
     - `run`: its own `ViolaName` argument. An inherited `VIOLA_NAME` (a `viola run` started from inside a wrapped session) never overrides it, so a wrapper never writes into another instance's `run-<name>.ndjson`;
     - `hook` and `mcp`: `VIOLA_NAME` only;
     - `cli`: the caller's `VIOLA_NAME`, then the target argument (Trace context propagation);
     - `ui`: no instance. Its file name needs `<port>`, and the GUI port can come from `config.json` (security Data Classifications, config values). For `ui` only, the step-5 `config.json` parse therefore runs here and its result is reused at step 5. `<port>` resolves flag → `config.json` → `47319` (arch precedence flags > config > defaults). A `ui` panic during that parse falls in the §7 pre-init window (D-29).
  4. **Home strict-modes first, diagnostics init after** (overseer fix pass 2, B2). When viola creates the home (a not-yet-existing `--home`, as the tests harness passes), it creates it first: 0700 on Unix, and on Windows the explicit protected user + SYSTEM DACL that security requires for a `--home` outside `%USERPROFILE%`. The role's security strict-modes check then runs on the home (owner, mode / DACL, not a symlink; security `~/.viola/` access control). Nothing under the home, `diagnostics/` included, is created or opened before that check passes. A refused home gets no diagnostics line: the role exits with its security outcome (`hook` fails open with exit 0), and a panic in this window falls in the §7 pre-init window (D-29). Roles with no strict-modes check in security still get the diagnostics checks below. Only then create `<home>/diagnostics/` with `DirBuilderExt::mode(0o700)` on Unix; on Windows it inherits the already-verified home DACL. Before the first write, `diagnostics/` and every opened diagnostics file (role file, and later the detail file) get the same checks as the home: on Unix, `lstat` owner == `geteuid()`, `mode & 0o077 == 0`, not a symlink (files opened with `O_NOFOLLOW`); on Windows, owner and DACL. A failed check is handled like a failed open (below), so a pre-existing foreign, permissive or symlinked diagnostics file is never written. Open the role file with `OpenOptions::new().append(true).create(true)` and `OpenOptionsExt::mode(0o600)` on Unix. If the open fails, the writer becomes `std::io::sink` (or stderr for `mcp` only, D-09). The instance detail file `<home>/instances/<name>/diagnostics/detail-<process>.ndjson` is **not** opened here. It is opened lazily, with the same append / 0600 / 0700 rules, by the first content-bearing record (chain, drift report), and the panic hook opens it itself when it is still closed. A failed detail open drops only the detail line, never the home-level line, and writes nothing to stderr. The error-free `hook` path therefore keeps exactly one log `open` (D-29).
  5. Parse `<home>/config.json` once for the process. `diagnostics_level` falls back to `"info"` when the file is missing or unreadable or the value is unknown. Then install the subscriber with an explicit writer (below). No subscriber exists during the parse, so its skipped-key count and failure `detail` are held in memory. A panic during the parse is still logged, because the role file exists after step 4.
  6. `obs_event!(ProcessStart, …)`, then `parse-rejected{parser:"config-json", detail, count}` if the step-5 parse failed or skipped keys (D-28).

  Per-role anchors:
  - `run`: steps 1–6 finish **before** `run.collision_check`, so exit-1 causes are logged. The panic hook is live before `pty.spawn`.
  - `ui`: before `axum::serve` binds `127.0.0.1:<port>`.
  - `mcp`: before the rmcp 3.4.1 stdio transport starts.
  - `hook`: one `config.json` read plus exactly one log `open`, then appends, which fits the `max < 1.0 s` gate.
- **Init body sketch (≤ 5 lines):**
  ```rust
  let sub = tracing_subscriber::fmt().json().flatten_event(true).with_current_span(false)
      .with_span_list(false).with_ansi(false).log_internal_errors(false).with_max_level(tracing::Level::DEBUG)
      .with_timer(MillisUtc).with_writer(diag_writer /* BoxMakeWriter over Arc<File> | sink */)
      .finish().with(viola_targets(cfg.diagnostics_level)); // filter::Targets = the only configurable level gate
  tracing::subscriber::set_global_default(sub)?;
  ```
  `MillisUtc` is a crate-local `FormatTime` that writes `chrono::Utc::now().to_rfc3339_opts(SecondsFormat::Millis, true)`. It is needed because the default `SystemTime` timer writes microseconds and `ChronoUtc::rfc_3339()` writes `+00:00` with automatic precision (obs-research, tracing-subscriber, finding 1).

### Service identity

- **service.name:** hardcoded `"viola"` as `viola_core::SERVICE_NAME`. This is the arch-fixed binary name, already served as `"name":"viola"` by `/health` and `/api/info`. It is never read from `OTEL_SERVICE_NAME` / `SERVICE_NAME`: arch says env vars are not a configuration channel.
- **service.version:** compile-time `pub const VERSION: &str = env!("CARGO_PKG_VERSION");` in viola-core. Every crate uses `version.workspace = true` and reads this one constant, so the log `version`, channel `sender`, snapshot `writer` and `/health.version` cannot drift apart.
- **deployment.environment:** N/A. viola is local-only with no hosting (arch Surfaces). No env-var fallback exists: env vars are not a configuration channel.
- **Resource attributes:** no OTel Resource is built in v1. The identity is emitted as fields on `process-start`: `service_name`, `version`, `os` (`windows|macos|linux` from `std::env::consts::OS`), `pid`. Every line carries the role (`process`) and the instance (`instance`). If the deferred bridge is ever adopted, use `Resource::builder_empty()` with these same values.

### Logging stack

- **Library:** tracing 0.1.44 (facade; Tokio-free, no C) + tracing-subscriber 0.3.23 (`fmt` JSON formatter; meets security Vector 9's `>=0.3.20`).
  - Every line is emitted through **one crate-local macro `obs_event!`**. It is defined in viola-core as a `macro_rules!` that expands to `::tracing::event!`, so viola-core itself stays I/O-free and tracing-free.
  - The macro always attaches `event` (the `ObsEvent` enum's kebab `Display`), plus `process` and `instance` from a process-global `OnceLock<ProcessCtx>`. Each is key absence when it has no value.
  - `corr` is an ordinary typed `key = value` field that the caller passes on the events that carry one. It is never `?`/`%`, and it is absent when null. The macro has no dedicated `corr` arm, because in `macro_rules` such an arm is ambiguous with the generic `key = value` arm. The schema confines `corr` to the corr-bearing events and types it `number|string`. As measured at chunk 2026-09-24-diagnostics-plane (`crates/viola-core/src/obs.rs`, `tests/contract_diag_schema.rs`).
  - `message` is a static literal equal to the event name, so it never carries interpolated content.
- **Format:** structured JSON-per-line. `on_event` formats into a thread-local buffer and makes one `write_all` per line on an append-mode handle, which meets the arch "one `write` per line" rule. The line is codes-only and stays well under 4 KiB. Every role file can have concurrent writers in separate processes, and the same rule covers all of them:
  - `hook-<name>.ndjson`, shared by hook processes;
  - `mcp.ndjson`, shared by one `viola mcp` per Claude session;
  - `cli-<name>.ndjson`, shared by concurrent short-lived verbs;
  - `run-<name>.ndjson` and `ui-<port>.ndjson`, briefly: a second `viola run <name>` on a live or `stale` name, or `viola ui` on a live port, opens the same file at init step 4. It appends its `process-start` and its exit-1 `process-exit` (for example `detail:"already-live"`, Scenario 1) next to the holder's lines. A `gone` name is taken over instead, with no exit-1 line. The concurrent-append check covers this collision path too (D-29);
  - the instance detail files `detail-hook.ndjson`, `detail-cli.ndjson` and `detail-mcp.ndjson`, shared by those same processes. Their lines (chains, drift reports, one-line backtraces) are not codes-only and can exceed 4 KiB, so the size argument below does not apply to them. They rely only on the single `write_all` and the torn-line tolerance, and the concurrent-append check must include a detail line larger than 4 KiB (D-28). As landed for the hook files: the check (8 concurrent `viola hook` processes: 16 whole lines in `hook-<name>.ndjson`, 8 in `detail-hook.ndjson`, all 3 CI OSes) carries no line over 4 KiB, and its over-4 KiB half is `hook_panics_append_whole_lines_over_4_kib_side_by_side`: 8 concurrent forced panics through the `fake-agent` seam `FAKE_AGENT_HOOK_PANIC` leave 8 whole role lines and 8 whole `detail-hook.ndjson` lines each over 4 096 B, all 3 CI OSes (chunk 2026-09-28-hook-perf-gate). The panic payload + backtrace is the only hook detail line that large.

  Each line is one `write_all` of one complete line on an append-mode handle. POSIX `O_APPEND` fixes the offset per write; on Windows, `append(true)` opens with `FILE_APPEND_DATA` only. Non-interleaving at this size is an assumption, not a POSIX guarantee, because the 4 KiB `PIPE_BUF` rule applies to pipes. Two things back it:
  - readers tolerate a torn line: the harness emits `{"torn":true}`, and G2 skips it through `fromjson?`;
  - a tests-owned concurrent-append check on windows-2025, macos-latest and ubuntu-latest asserts that every line of each shared file parses. This is a multi-platform-exporter-compat item. A torn line fails that check.
- **Mandatory builder settings beyond the tests line** (obs-research, tracing-subscriber, findings 2–4). Each is additive and renames or removes no required field:
  - `.log_internal_errors(false)`: the default `true` falls back to `eprintln`, which would break the Hook contract and the `run` terminal rule;
  - `.with_span_list(false)`: closes the `spans:[…]` leak path;
  - `.with_writer(...)` always explicit: the default writer is stdout;
  - `.with_ansi(false)`;
  - `with_file(false)` / `with_line_number(false)` left at their defaults, so no source paths.
- **Sink:** **file-only, one file per role.** There is no stdout sink on any role, and no pretty or dev sink. The files are:
  - `run`: `<home>/diagnostics/run-<name>.ndjson`;
  - `hook`: `<home>/diagnostics/hook-<name>.ndjson` (shared by concurrent hook processes; a write failure is swallowed);
  - `mcp`: `<home>/diagnostics/mcp.ndjson`, with stderr JSON only when the file cannot be opened (D-09);
  - `ui`: `<home>/diagnostics/ui-<port>.ndjson`;
  - `cli`: `<home>/diagnostics/cli-<name>.ndjson`, only when an instance resolves (D-06).

  Content-bearing records (full anyhow chains, serde_path_to_error drift reports, panic payload + backtrace) go **only** to `<home>/instances/<name>/diagnostics/detail-<process>.ndjson` (0600/0700), and only when an instance resolves (D-08).
  - They never pass through the tracing subscriber, whose one writer holds only the home-level role file.
  - The root-bin `viola::obs` detail writer formats each detail line itself, as the panic hook does: same `timestamp` (`MillisUtc` format), `level`, `target`, `message`, `event`, `process`, `instance`, and `corr` / `conn` keys. It writes each line with one `write_all` on the lazily opened detail handle.
  - No `obs_event!` call ever carries `chain`, `drift_report`, `panic_payload` or `backtrace`, and G4's `diag-line.v1.json` rejects a home-level line that does (D-30).
- **Agent-mode flag:** **none.** JSON-to-file is the only mode (obs-scope §3).
  - The level comes from `config.json` key `diagnostics_level` (`"info"` default | `"debug"`), applied through a `tracing_subscriber::filter::Targets` layer built in code.
  - It is never read from `RUST_LOG`, and `EnvFilter` / the `env-filter` feature is never enabled.
  - Level changes volume only. Every level uses the same field allow-list, so no setting can disable redaction (security Anti-Patterns).
  - Third-party targets (`rmcp`, `axum`, `tower_http`, `hyper`, `portable_pty`, `notify`, `sysinfo`, `interprocess`) are `OFF` in every sink, because their records lack the required fields. Their failures are captured at viola's seam from the returned `Result`s (D-11).

### Log format JSON schema

**Binding. Reproduced verbatim from test-plan.md §3 Test Harness Contract → Log format (re-synced in overseer fix pass 2, 2026-09-24, after test-plan amendments D-21 / 693e083 and fix pass 2, and again in fix pass 3 for Z7). Obs aligns to tests, not vice versa.**

```markdown
- **Format:** JSON-per-line. There are two streams:
  - **Event stream:** product events in `instances/<name>/events.ndjson`, exactly the arch Event line `{"v":1,"ts","instance","kind","source":"hook|wrapper|cli","data"}`.
  - **Process logs:** emitted by tracing-subscriber 0.3.23 `fmt().json().flatten_event(true).with_current_span(false)`, writing codes-only lines to `<home>/diagnostics/{run-<name>,hook-<name>,mcp,ui-<port>,cli-<name>}.ndjson`. Content-bearing detail (chains, drift reports, panic payload and backtrace) goes only to `<home>/instances/<name>/diagnostics/detail-<process>.ndjson` (obs-plan D-08). Files are 0600, dirs 0700, with one `write` per line.
- **Required fields:**
  - For process logs: `timestamp` (RFC 3339 UTC with ms and `Z`), `level` (`DEBUG|INFO|WARN|ERROR`), `target`, and `fields.message` (or the flattened `message`).
  - `event`, a closed kebab-case enum. A new value needs a Decisions Log entry, like the §3 closed enums. Each value fixes what its `corr` holds:
    - `channel-request`, `channel-response`: `corr` = the JSON-RPC request `id`; null for an id-less `hook.event` notification and on a response to a frame that yielded no id (`error_code` -32700 or -32600)
    - `dialog-raised`, `dialog-answered`: `corr` = `dialog_id`
    - `hook-invoked`, `hook-decision`: `corr` = `dialog_id` for dialog hooks, otherwise null. A dialog `hook-invoked` may still be null (fail-open before `dialog_id` is known, D-07), and so may a `hook-decision` that carries `detail`
    - `send-issued`, `send-confirmed`, `send-refused`: `corr` = the send `cursor`. A send refused before `send-issued` (`human-typing`, `budget-paused`, `input-not-ready`, `turn-running`) has no arch cursor, so its wrapper-side `send-refused` carries `corr` = the target's `events.ndjson` end offset at the moment of refusal, a log join key only, never returned to the caller (obs-plan D-28). Wrapper-side `send-*` lines also carry `conn` and `rpc_id`, the originating `send` request's connection and JSON-RPC `id`, so `(conn, rpc_id)` joins them to that call's `channel-*` lines (obs-plan D-30). The client-side `send-refused{side:"client"}` has `corr` null.
    - `release-from-driver`: `corr` = the JSON-RPC request `id` (obs-plan D-01; the wrapper's `-32602` refusal of a `release` that carries `from`)
    - `process-start`, `process-exit`, `http-request`, `panic`: `corr` = null
    - `liveness-changed`, `state-recovered`, `sse-opened`, `sse-closed`, `parse-rejected`: `corr` = null (obs-plan D-02 … D-05)
    - `schemas/diag-line.v1.json` requires `corr` exactly where the rules above always define it: `dialog-raised`, `dialog-answered`, `release-from-driver`; `channel-request` unless `method` is `hook.event`; `channel-response` unless `error_code` ∈ {-32700, -32600}; `send-*` when `side` is `wrapper`; `hook-decision` when `hook_event` ∈ {`pre-tool-use`, `permission-request`} and no `detail`. `hook-invoked` never requires it.
  - `process` (`run|hook|mcp|ui|cli`; `cli` is a short-lived verb with a resolved instance, writing `cli-<name>.ndjson`, obs-plan D-06) and `instance` (a `ViolaName` or null).
  - `corr` is copied unchanged as a JSON number or string. It is never renamed (for example to `correlation_id`).
  - **Null encoding:** a null `corr` or `instance` is written as key **absence** (tracing has no null field value; obs-plan D-12). The harness and every `jq` / jaq assertion treat an absent key and `null` as the same value (`.corr == null` holds for both), and obs-plan's `schemas/diag-line.v1.json` rejects a literal `null`.

  obs-plan may add fields but must not rename or remove these. The harness greps on them.
- **Harness-side a11y rows (not part of this `event` enum):** `event:"a11y-violation"` is not a value of the product `event` enum above, has no `ObsEvent` variant, and is never emitted by a product process.
  - The a11y Playwright fixture writes one row per failing check into `e2e-web/test-results/a11y/*.ndjson`.
  - Rows follow a tests-owned schema of their own, `e2e-web/schemas/a11y-row.v1.json`. It reuses the diag-line field names without renaming any: `event` is const `"a11y-violation"`, `process` is const `"ui"`, `corr` and `instance` are absent, and the a11y plan's additive fields (a11y-plan §3 Structured violation JSON schema) are declared there.
  - The same schema-conformance check body that backs obs-plan G4 validates these files against that schema. G2, G4 and `schemas/diag-line.v1.json` never read them.
  - This revises the fix-pass-2 Y3 entry (overseer fix pass 3, Z7).
- **Constraints:**
  - The hook trace is the `hook-<name>.ndjson` file. A failed write there is swallowed, so the hook still exits 0 and never writes to stderr.
  - No line may contain the GUI token, a `Cookie` header, `?t=`, or any stripped `CLAUDE*` value. This is enforced by the secret-scan test in §6.
- **Agent parsing:** lines parseable with `jq -c 'select(.event=="dialog-raised")'` or equivalent; NEVER multi-line stack traces. A panic in a hook is caught and logged as one `level:"ERROR", event:"panic"` line.
```

**Obs extensions.** None of these renames or removes a tests field. Each enum value has its own Decisions Log entry (§12). All are ACCEPTED at review (2026-09-24, overseer, founder-delegated) and carried to test-plan.md as the tests amendment D-21, which the overseer applies right after this run.
- **Accepted `event` values (tests amendment D-21):**
  - `release-from-driver`: `corr` = JSON-RPC `id` (D-01)
  - `liveness-changed`: `corr` = null (D-02)
  - `state-recovered`: `corr` = null (D-03)
  - `sse-opened`, `sse-closed`: `corr` = null (D-04)
  - `parse-rejected`: `corr` = null (D-05)
- **Not a product `event` value: `a11y-violation`** (overseer fix pass 3, Z7, revising fix pass 2).
  - It is **not** in the product `event` enum and has **no** `ObsEvent` variant in `viola_core::obs`. No product process emits it.
  - It is a harness-only row that the a11y Playwright fixture writes into `e2e-web/test-results/a11y/*.ndjson`.
  - The row has its own tests-owned schema, `e2e-web/schemas/a11y-row.v1.json`, which reuses the diag-line field names without renaming any (tests Log format → Harness-side a11y rows).
  - `schemas/diag-line.v1.json`, G2 and G4 never contain or read it.
- **Accepted `process` value:** `cli`, for short-lived verbs with a resolved instance (D-06; tests amendment D-21 adds it to the `process` enum and to the harness `--process` filter).
- **Additive fields:** catalogued per event in Section 6.
- **Null encoding:** tracing 0.1.44 has no null field value; an `Option::None` field records nothing. A null `corr` / `instance` is therefore represented by **key absence**, and `jq '.corr'` yields `null` either way. The panic hook's hand-written line follows the same rule. It never writes `corr`, and it omits `instance` when none resolved (always for `ui`). Both emitters are therefore schema-identical: `schemas/diag-line.v1.json` declares `corr` (number | string) and `instance` (string) as optional keys — except that `corr` is required on the lines §3's per-event rules always define it for — and rejects a literal `null` for either, which catches an emitter that drifts (D-26). Key absence is ACCEPTED (D-12, review 2026-09-24); the tests amendment D-21 states "absent = null" for `corr` / `instance`, so harness readers treat a missing key and a literal `null` identically.

Illustrative line (one physical line on disk):
```json
{"timestamp":"2026-09-24T03:12:07.412Z","level":"INFO","target":"viola_channel::server","message":"channel-response","event":"channel-response","corr":7,"conn":"cli-4812-1790219525118-1","process":"run","instance":"builder","method":"send","result_class":"refusal","refusal":"human-typing","duration_ms":3}
```

### Log file location

- **Path:** `<home>/diagnostics/{run-<name>,hook-<name>,mcp,ui-<port>,cli-<name>}.ndjson`.
  - `<home>` is resolved `--home` → grandparent of `VIOLA_DIR` → `~/.viola/`. The basenames and root follow tests (binding); `cli-<name>` is accepted (D-06) and added to the tests file list by amendment D-21. Arch names only `instances/<name>/diagnostics/`, so the home-level root plus instance detail files (D-08) go to arch as amendment request D-22.
  - Content-bearing detail goes to `<home>/instances/<name>/diagnostics/detail-<process>.ndjson` (D-08), which satisfies security Vector 2: only instance-scoped diagnostics may hold user-derived content.
  - Home-level files are **codes-only by construction**: they carry no user content, token, cookie or `CLAUDE*` value.
  - Permissions: files 0600 in 0700 directories on Unix; the home DACL (user, SYSTEM, Administrators) on Windows. The tests secret-scan test enforces both.
- **Rotation:** **N/A in v1, and no rotation library.** Retention is the lifetime of the session home (tests `logs`; obs-scope §5 audit-log-retention). `events.ndjson` must never be rotated by a generic library, because byte offsets are the `wait after`, `send cursor` and `Last-Event-ID` cursors.
  - Future rotation of `diagnostics/` only: the candidate is logroller 0.1.12. Phase 5 must first verify 0600 creation, keep `xz2` off (`lzma-sys` is a C-building crate), keep `flate2` on its pure-Rust backend, and accept the duplicate `thiserror 1.x`.
  - tracing-appender 0.2.5 is rejected: its `non_blocking` writer drops lines, and it pulls in `time` (RUSTSEC-2026-0009 range).

### Snapshot / paste-to-AI integration

N/A: there is no pulse-class receiver in the stack. The paste-to-AI workflow runs through the tests harness instead of a tail of a single log file:
- `scripts/agent-run.sh logs` / `scripts/agent-run.ps1 logs` merges `events.ndjson` (`{"src":"events","instance","offset","record"}`) with every `diagnostics/*.ndjson` (`{"src":"diag","file","record"}`) and every instance detail line from `instances/*/diagnostics/detail-*.ndjson` (`{"src":"diag","file","instance","record"}`; the added `instance` tells apart the same `detail-<process>.ndjson` basename across instances, per tests `logs`). Torn lines appear as `{"torn":true,"offset":n}`. The merged output is filterable by `--instance`, `--kind`, `--process`, `--after`.
- `agent-run status` is the aggregate health snapshot: `state`, `last_error` closed enum, `api_sessions_equal_list`.
- Canonical agent queries:
  - To follow one send across every side: `agent-run logs | jq -c 'select(.record.instance=="B" and (.record.event|IN("send-issued","send-confirmed","send-refused")) and .record.corr==412)'`, where `B` is the send's target.
    - A send `cursor` is a byte offset into the target's own `events.ndjson`, so it is unique per target instance only. Two wrappers with identical start sequences (common in E2E) can both hold a send at 412, so `corr` alone mixes sends.
    - `send-*` lines with a cursor `corr` are emitted only by the target wrapper (`side:"wrapper"`, D-27), so the jq `instance` predicate is the target's. The caller-side `send-refused{side:"client"}` has `corr` null and is read from the caller's `channel-response{conn}` and `process-exit` instead.
    - The harness `--instance` filter is still not used. It would also drop the caller's `channel-*` lines, which carry the caller's instance (see Trace context propagation).
    - The `event` filter keeps out an unrelated `channel-request` whose JSON-RPC `id` happens to be 412.
    - The `channel-*` lines of the same send are reached from a matched send line: its `(conn, rpc_id)` equals their `(conn, corr)` (D-30). When refusals at the same offset must be told apart, add `and .record.rpc_id==<id>` to the query.
    - The hook side (`hook-invoked{hook_event:"user-prompt-submit"}`, `corr` null) joins by the wrapper's `instance` + `hook_event` + `timestamp` inside `run.confirm_window`;
  - `jq -c 'select(.record.level=="ERROR" and (.file|startswith("detail-")|not))'` lists every error and panic once. The detail-file duplicate of a panic line is excluded (§10).
- The harness `logs` glob must also cover `instances/*/diagnostics/detail-*.ndjson` (D-08). This additive harness change is part of tests amendment D-21.

### Trace context propagation

- **HTTP boundaries:** no W3C `traceparent` in v1. The browser sends no propagation header (obs-scope §3), and axum-tracing-opentelemetry 0.39.1 is rejected: it needs the deferred SDK, its W3C `traceparent` extraction has no sender, and its optional `/metrics` route would be a new listener. (Its MSRV 1.91 fits the raised 1.96 workspace floor, D-22, so MSRV is no longer a reason.)
  - Correlation on the `ui` side runs on the SSE `id:` cursor `<ViolaName>:<offset>` and `Last-Event-ID`.
  - `sse-opened` records `last_event_id_present` / `last_event_id_valid` and `resume_instances` (a count).
- **gRPC boundaries:** N/A (there is no gRPC in the stack).
- **IPC boundaries** (viola-channel JSON-RPC over interprocess 2.4.4 local sockets):
  - `corr` carries the JSON-RPC `id` on channel lines, `dialog_id` on dialog and hook lines, and the send `cursor` on send lines (tests).
  - **Additive `conn` field (D-10).** The client generates `conn = "<process>-<pid>-<t0>-<n>"`, with `t0` the process's start instant in Unix epoch milliseconds (captured once at init; Low sensitivity like `pid`) and `n` a per-process connection counter (D-32), and sends it in `params` alongside `v` / `sender` / `from`. Both sides log it on `channel-request` / `channel-response`. It is wire-safe under arch Mixed-version tolerance, because readers skip unknown fields. A server receiving a frame without `conn` from an older peer logs `srv_conn = "srv-<accept counter>"` instead, and so does a server receiving a `conn` outside the shape `^[a-z]+-\d+-\d+-\d+$` or over 64 bytes (a peer-supplied value is logged only in that shape). The server strips `conn` from `params` before dispatch: it is never an input to a method.
  - **Join key.** The merged-log join key for channel lines is `(conn, corr)`, and `instance` is deliberately **not** part of it (D-25).
    - `conn` is unique across the session home (`<process>-<pid>-<t0>-<n>`, D-10, D-32). `<pid>` alone is not unique: PIDs are recycled, quickly on Windows. Every `hook` process opens exactly one connection (`n` = 1) and sends the same first JSON-RPC `id`, so without `t0` two hook calls from a reused PID would share `(conn, corr)`.
    - Each side logs its **own** instance. `cli` and `mcp` log the caller's `VIOLA_NAME`, which is also `from`. `run` logs the wrapper's `ViolaName`. So the two sides of one call carry different `instance` values.
    - `srv_conn` lines from older peers have no client-side counterpart. They join only within the server's own file.
  - **`cli` instance precedence** (init step 3): the caller's `VIOLA_NAME` comes first. The explicit target `ViolaName` argument becomes `instance`, and names the `cli-<name>.ndjson` file, only when `VIOLA_NAME` is unset. So `viola send --to B` run by driver A logs `instance:"A"` to `cli-A.ndjson`.
  - `agent-run logs --instance B` therefore shows only B's side of a call. To reach the caller, take `conn` from B's `channel-request` and query with no `--instance` filter, for example `agent-run logs | jq -c 'select(.record.conn=="cli-4812-1790219525118-1")'`.
  - **Send cursor on early refusals.** The arch `send` result returns `cursor` only on success or `unconfirmable`; a refusal result carries `refusal` + `detail` only. For a send refused before `send-issued` (`human-typing`, `budget-paused`, `input-not-ready`, `turn-running`), the wrapper's `send-refused` carries `corr` = the target's `events.ndjson` end offset at the moment of refusal. This value is a log join key only. It is never returned to the caller and is not a `wait after` value. After `send-issued`, every `send-*` line carries the issued cursor. The cursor alone is not a unique send key. Refusals at an unchanged end offset can share it, and a later issued send can reuse it.
  - **Send ↔ channel link (D-30).** Every wrapper `send-*` line also carries `conn` (or `srv_conn` for an older peer) and `rpc_id`, the JSON-RPC `id` of the `send` request it answers. The pair `(conn, rpc_id)` on a send line equals `(conn, corr)` on that call's `channel-request` / `channel-response`, on both sides. So the caller reaches the wrapper's send lines, and the wrapper's send lines reach the caller, without timestamp guessing. One send is identified by target `instance` + `corr` + `conn` + `rpc_id`.
  - `from` is logged with the constant companion `from_trust:"self-reported"` and is never treated as identity.
  - JSON-RPC notifications (`hook.event`) have no `id`, so their `channel-request` has a null `corr`. They join through `hook_event` + `instance` + `timestamp`.
- **Claude Code → hook stdin:** there is no inbound context. Correlate by `instance` + `dialog_id` (dialog hooks) or `instance` + `hook_event` + `timestamp`.
- **Internal async boundaries:**
  - `viola-ui` / `viola-mcp` (Tokio): spawned futures use `tracing::Instrument::instrument(span)` / `.in_current_span()`.
  - `run` std threads (PTY pump, handle-wait thread, channel server workers): the thread closure takes a cloned `tracing::Span` and runs inside `span.in_scope(...)`.
  - Because spans are not serialized, `corr`, `conn` and `instance` are always passed as **explicit values** into the thread or task and set on each `obs_event!`. They are never recovered from span context.

### Heartbeat ticks

_[Standard+ tier. The stall-detection intent is kept, with viola-specific mechanisms instead of 10–30 s tick lines (D-13).]_

- **Tick interval:**
  - `run`: the existing `<instance>/heartbeat` file, touched every **1 s** (arch contract). It is not logged per tick: a 1 s line into a never-rotated file would grow without bound.
  - `ui`: `Sse::keep_alive` every **15 s**. This is a tests perf gate, observed by the tests at `advance(15 s)` and not logged per keep-alive.
  - `mcp`, `hook` and short-lived CLI verbs: no tick (request-driven).
- **Tick event format:**
  - Transition-only `liveness-changed` (D-02), emitted by the long-lived reader `ui` when a watched instance's state changes. Fields: `subject_instance`, `liveness_from`, `liveness_to` (`live|stale|gone`), `heartbeat_age_ms`, `pid_alive` (sysinfo 0.39.6 pid + start-time check).
  - Short-lived readers (`list`) expose `liveness` in `--json` and `/api/sessions` instead, and the agent reads it from the `agent-run status` snapshot.
- **Stall detection:** the agent reads `agent-run status` (`state: degraded` when any item is `stale`), or `agent-run logs --process ui | jq 'select(.record.event=="liveness-changed")'`. The flip threshold is `heartbeat_age_ms > 5000` (4.9 s live / 5.1 s stale or gone).

### Bootstrap phases (derive for route / setup-project)

_[ALL tiers — explicit derivation hint for downstream consumers
per D26 chain. route reads this to plan bootstrap phase ordering;
setup-project reads this to materialize phase scaffolding.]_

- **otel-sdk-install:** **no-op in v1.** No `opentelemetry*` crate is added. The phase records only the deferral (D-12) and adds `cargo deny` `[[bans]]` entries for `opentelemetry-otlp`, `opentelemetry-stdout`, `sentry` and `tracing-appender` on the workspace, so they cannot arrive transitively without a Decisions Log entry.
- **logger-stack-install:**
  - Add `tracing = "0.1.44"` to the root bin, viola-pty, viola-channel, viola-state, viola-agent-claude, viola-mcp and viola-ui.
  - Add `tracing-subscriber 0.3.23` (`default-features = false`, `fmt,json,registry,std`; no `chrono` feature, D-26) to the root bin only.
  - Add tower-http 0.7.1 feature `trace` to viola-ui.
  - viola-core (`viola_core::obs`): `obs_event!` and the `ObsEvent` / `ObsProcess` enums. The macro expands to `::tracing::event!` at the caller, so viola-core itself gains no `tracing` dependency.
  - Root bin (`viola::obs`), the only crate that depends on tracing-subscriber: `MillisUtc`, `viola_obs_init` and `viola_panic_hook`.
    - Add a direct `chrono = { version = "0.4.45", default-features = false, features = ["clock", "std"] }` (already in the tree). `MillisUtc` calls `chrono::Utc::now()` itself, and tracing-subscriber's `chrono` feature does not re-export chrono.
  - Raw-tracing ban. The mechanism is split, because no allow inside `obs_event!` can exempt its inner `::tracing::event!`. As measured on clippy 1.98.1 at chunk 2026-09-24-observability-gates (`.andromeda/runs/2026-09-24T12-21-11-implement/clippy-disallowed-macros-measurement.md`), clippy reports a disallowed macro expanded inside an exported macro at the caller crate's level. Only a crate-level `#![allow]` in the caller silences it.
    - One workspace `clippy.toml` `disallowed-macros` bans the level macros `tracing::{info,warn,error,debug,trace}` by path. `tracing::event` is not listed there, because `obs_event!` expands to it.
    - Raw `event!` is caught by a fail-closed grep in `scripts/lint-probes.sh`: `\bevent!\s*[({[]` in any `*.rs` outside `crates/viola-core/src/obs.rs`. `\b` does not match inside `obs_event!`. The inner `#[allow(clippy::disallowed_macros)]` in `obs_event!` stays in place but has no effect.
    - The gate is proven both ways by `scripts/lint-probes.sh`, in throwaway crates carrying the repo's `clippy.toml` and lint table:
      - a raw `tracing::info!` fails and a real `obs_event!` call lints clean;
      - the grep finds a planted raw `tracing::event!`, passes `obs_event!`, and is clean over the tree.
  - Verify with `cargo check` of the sync crates without Tokio, plus `cargo deny check` on all three targets.
- **service-identity-wire:** `viola_core::{SERVICE_NAME, VERSION}` as the single source for `process-start.version`, `sender`, `writer` and `/health.version`. Workspace `version.workspace = true`.
- **log-format-schema-emit:** commit `schemas/diag-line.v1.json`, a hand-maintained JSON Schema of the Section 3 / Section 6 line. Its `event` enum must equal the `ObsEvent` variants, and a tests-owned check asserts that equality. The harness greps the same fields.
- **trace-context-propagate-wire:** `corr` population per the tests table, plus the additive `conn` in viola-channel client `params` and server logging (D-10), plus `Span` hand-off into std threads and Tokio tasks.
- **heartbeat-tick-wire:** the `liveness-changed` transition emitter in viola-ui's sessions reader, and the `sse-opened` / `sse-closed` guard in the `/api/events` stream (D-02, D-04).
- **pii-scrubbing-wire:** merge with security's `logging-redaction-wire` bootstrap phase:
  - `#[instrument(skip_all, fields(...))]` everywhere;
  - veil 0.3.0 `#[derive(Redact)]` on payload types, plus `cargo deny` `[[bans.features]] crate = "veil" deny = ["toggle"]`;
  - the tower-http custom `make_span_with` using `uri.path()` only;
  - fixed thiserror `Display`s;
  - the instance-scoped `detail-<process>.ndjson` routing.
- **obs-ci-gate-wire:** CI steps per Section 9:
  - `clippy.toml` `disallowed-macros` (raw `tracing::{info,warn,error,debug,trace}` outside `viola_core::obs`), plus the fail-closed raw-`event!` grep in `scripts/lint-probes.sh`, which runs in the `ci.yml` `lint` job on the Linux leg (source text is OS-independent). The split is explained under logger-stack-install;
  - `[workspace.lints.clippy]` `print_stdout` / `print_stderr` / `dbg_macro` = `deny` (§11 Logs), plus `[lints] workspace = true` in **every product** member's `Cargo.toml` (the root bin `viola` and the product `crates/viola-*` crates). A product member without it inherits no workspace lints, so the ban silently does not apply there. **Exempt (overseer fix pass 2, B5):** `viola-harness` (`crates/viola-e2e`) and the fake agent (`viola-fake-agent`) must print: the harness prints exactly one JSON document per command on stdout, and the fake agent emulates the claude CLI on stdout/stderr (tests §3, §4). Cargo cannot override single lints under `workspace = true`, so `viola-e2e` omits it and carries its own `[lints.clippy]` table without `print_stdout` / `print_stderr`. The fake agent is a `[[bin]]` of the root `viola` package (feature `fake-agent`), and lints are set per package, so it inherits the root's `[lints]` and is exempted by a crate-level `#![allow(clippy::print_stdout, clippy::print_stderr)]` in `src/bin/viola-fake-agent.rs`. A CI assertion lists members: every product member has `workspace = true`, and only `viola-e2e` lacks it. Prove it both ways: a `println!` in a `hook` path fails clippy, and the output modules' local `#[allow]` passes;
  - the SHA-pinned PCRE2 ripgrep install step, then G1 (bare `#[instrument]`) and G3 (`panic = "abort"`);
  - G2 (zero `event:"panic"`), G4 (`id: schema-conformance`), the secret scan (`id: secret-scan`, `if: always()`) and the scan-gated uploads, in the §9 step order;
  - SHA-pinned `actions/upload-artifact` v7.0.1.

Route order: otel-sdk-install (no-op record) → logger-stack-install → service-identity-wire → log-format-schema-emit → trace-context-propagate-wire → heartbeat-tick-wire → pii-scrubbing-wire → obs-ci-gate-wire.
- **Merged phase position:** `pii-scrubbing-wire` + security's `logging-redaction-wire` run as **one** phase at the `pii-scrubbing-wire` slot. It must follow logger-stack-install, because the sinks, `obs_event!` and the detail-file routing target must exist. It must precede obs-ci-gate-wire, because G1, the secret-scan/canary step and the veil `toggle` ban verify it. `logging-redaction-wire` is security's only logging contribution at Minimal tier, so obs imposes no order on security's other, non-logging phases.
- **Ownership:** obs owns the merged phase and its mechanical gates: `#[instrument(skip_all)]` + G1, the veil `toggle` cargo-deny ban (D-19), the TraceLayer `make_span_with`, and the `detail-<process>.ndjson` routing. Security owns the NEVER-log floor list that those gates enforce. The secret-scan test body and canary value stay tests-owned.

---

<!-- U35 · obs-plan.md · ## 12. Obs Decisions Log · sha256 06bb6485589ab0da7cb80f2284984b66b69f200b79778b4db9ff47f9e66cf43e -->

## 12. Obs Decisions Log

_Records key decisions during plan generation + manual additions
between phase loops._

**Initial entry:**

`2026-09-24` — Initial obs plan generated by `/andromeda-obs`
- **Tier:** Standard (1) with Minimal-tier exporter carve-outs. Justified by obs-scope §6:
  - 6 telemetry surface entries across 4 process roles, 7 instrumentable crates, 7 must-trace paths plus 5 edge flows, and 19 triggers;
  - tests tier Comprehensive (2), within ±1;
  - security Minimal with no compliance triggers, so there is no Comprehensive driver;
  - the exporter and reporter drop to Minimal shapes because upstream bans egress, new listeners, stdout on 3 of 4 roles, and Tokio / C crates in the sync graph.
- **Key decisions:**
  - **OTel SDK:** none in v1. The tracing 0.1.44 facade fills the API role; opentelemetry / opentelemetry_sdk 0.33.0 and tracing-opentelemetry 0.34.0 are deferred to the root-bin edge. Chosen because (obs-research OTel SDK Core):
    - with no exporter the SDK adds only dependency and mutation surface;
    - `with_current_span(false)` and the tests' `corr` model leave bridged span IDs without a consumer;
    - `Resource::builder()` reads `OTEL_*` env vars.
  - **Structured Logger:** tracing-subscriber 0.3.23 `fmt().json().flatten_event(true).with_current_span(false)`, plus `.with_span_list(false).with_ansi(false).log_internal_errors(false).with_timer(MillisUtc)` and an explicit writer. Chosen because:
    - it is the tests-bound emitter (Founder Direction 1) and meets security's `>=0.3.20`;
    - one `write_all` per event satisfies the arch one-write-per-line rule;
    - the four extra settings close research-found stderr, stdout and span-field leak paths and fix the millisecond timestamp.
  - **Exporter:** a tracing-subscriber 0.3.23 JSON **file sink** (`MakeWriter` over a std `OpenOptions::append` `File`, 0600 in 0700) at `<home>/diagnostics/<role>.ndjson`, consumed by `scripts/agent-run.{sh,ps1} logs|status`. Chosen because:
    - opentelemetry-otlp 0.33.0 brings Tokio via `reqwest::blocking` and env-configured endpoints;
    - opentelemetry-stdout 0.33.0 writes to reserved stdout and is "unsuitable for production";
    - the file sink has the same schema on all three OSes.
- **Open questions** (Phase 5 / other specialists):
  - tests alignment on D-01…D-06 enum values, D-12 absent-means-null and the D-08 `logs` glob: RESOLVED at review; carried as tests amendment D-21;
  - arch acknowledgment of the additive `conn` params field (D-10) and of `diagnostics_level` in `config.json` (D-15);
  - the D-08 diagnostics root and the sysinfo MSRV inconsistency (D-18): RESOLVED at review; carried as arch amendment request D-22 (floor 1.96).

**Subsequent entry format (for manual additions or re-runs):**

`2026-09-24` — D-01 `event` enum extension: `release-from-driver`
- **Decision:** Add `release-from-driver` (corr = JSON-RPC `id`; fields `conn`, `from`, `from_trust`). The wrapper emits it when `release` arrives from a driver rather than the CLI.
- **Rationale:** Founder Direction 4: "must log as its own event, not a generic wrapper fault -32602". Tests requires a Decisions Log entry for new values.
- **Impact:** §3 schema extensions, §4 Scenario 5, §6. Tests harness enum list (Phase 5).
- **By:** `/andromeda-obs` initial run

`2026-09-24` — D-02 `event` enum extension: `liveness-changed`
- **Decision:** Add `liveness-changed` (corr null), emitted by `ui` on live↔stale↔gone transitions of watched instances.
- **Rationale:** obs-scope §3 gap (b) and heartbeat ticks ("log transitions on change only"). Chaos trigger "stale heartbeat vs live pid". Design "live → stale" counter.
- **Impact:** §3 Heartbeat, §5, §10. The tests harness enum.
- **By:** `/andromeda-obs` initial run

`2026-09-24` — D-03 `event` enum extension: `state-recovered`
- **Decision:** Add `state-recovered` (corr null; `detail` ∈ `torn-line-healed|snapshot-replayed|snapshot-unsupported-v`; `file` basename; `offset`; `v_seen`).
- **Rationale:** chaos-instrumentation trigger and tests E5. An enum value is chosen over an additive `detail` on `process-*`, because heals happen mid-process with no lifecycle event to attach to.
- **Impact:** §4 E5, §5, §6. The tests harness enum.
- **By:** `/andromeda-obs` initial run

`2026-09-24` — D-04 `event` enum extension: `sse-opened`, `sse-closed`
- **Decision:** Add both (corr null), emitted by the `ui.sse_stream` `Drop` guard with the resume and close fields.
- **Rationale:** obs-scope §4 path 7 marks the SSE lifecycle as pending. TraceLayer cannot observe stream end (obs-research tower-http). Design "TAPE connecting → open" span and closed-with-cause counter.
- **Impact:** §4 Scenario 7, §5. The tests harness enum.
- **By:** `/andromeda-obs` initial run

`2026-09-24` — D-05 `event` enum extension: `parse-rejected`
- **Decision:** Add `parse-rejected` (corr null; `parser` closed enum; `detail`; `count`) for tolerant-parser rejections without an existing carrier: `claude agents --json`, vt100 under `catch_unwind`, `config.json` skipped keys, malformed `Last-Event-ID`, framing beyond `MAX_FRAME`.
- **Rationale:** creator-explicit trigger (per-parser parse-failure counts) and security Vector 8 ("unknown plus a counter").
- **Impact:** §5, §6, §7. The tests harness enum.
- **By:** `/andromeda-obs` initial run

`2026-09-24` — D-06 `process` enum extension: `cli` + `cli-<name>.ndjson`
- **Decision:** Short-lived verbs with a resolved instance log as `process="cli"` to `<home>/diagnostics/cli-<name>.ndjson`. Without an instance they write no file.
- **Rationale:** security Vector 2 and tests Coverage Triggers require `send-refused{control-character}` "at both client and wrapper". The tests `process` enum (`run|hook|mcp|ui`) has no value for the CLI client.
- **Impact:** §3 sinks, §4 surface table. The tests harness `--process` filter and file list (Phase 5).
- **By:** `/andromeda-obs` initial run

`2026-09-24` — D-07 `hook-invoked` `corr` timing
- **Decision:**
  - Dialog hooks emit `hook-invoked` once `dialog_id` is known (after the `hook.dialog` response), with `invoked_at` preserving the true start instant.
  - Non-dialog hooks and pre-`dialog_id` fail-open paths emit `hook-invoked{corr:null}` immediately.
  - The wrapper's `dialog-raised` gives live visibility during the wait.
- **Rationale:** obs-scope §3 gap (c). The tests table fixes `corr = dialog_id` for dialog hooks, and `hook.dialog` returns `dialog_id` only with the response.
- **Impact:** §4 Scenario 4, §6.
- **By:** `/andromeda-obs` initial run

`2026-09-24` — D-08 Diagnostics directory root + instance detail files
- **Decision:**
  - Home-level files follow tests verbatim (`<home>/diagnostics/…`) and are codes-only by construction.
  - Content-bearing records (full anyhow chains, drift reports, panic payload and backtrace) go only to `<home>/instances/<name>/diagnostics/detail-<process>.ndjson`, and only when an instance resolves.
- **Rationale:** reconciles tests (binding paths) with arch and security Vector 2 ("only `instances/<name>/diagnostics/` may hold user content"). `ui-<port>` has no instance.
- **Impact:** §3, §7, §8. The tests harness `logs` glob must add `instances/*/diagnostics/detail-*.ndjson` (Phase 5). Arch Diagnostics-location wording.
- **By:** `/andromeda-obs` initial run

`2026-09-24` — D-09 `mcp` sink
- **Decision:** `mcp` always writes `<home>/diagnostics/mcp.ndjson`: a home always resolves via `--home` → `VIOLA_DIR` grandparent → `~/.viola/`. JSON-per-line on stderr is only a fallback when the file cannot be opened. Never stdout.
- **Rationale:** tests assumes a file (binding). Arch's "stderr unless `VIOLA_DIR`" predates default-home resolution for diagnostics. rmcp community practice writes to stderr, never stdout.
- **Impact:** §3 sinks. Arch Output channel rules (Phase 5 acknowledgment).
- **By:** `/andromeda-obs` initial run

`2026-09-24` — D-10 Additive `conn` connection discriminator
- **Decision:** The client adds `conn = "<process>-<pid>-<n>"` to channel `params`, and both sides log it. Servers receiving frames without it log `srv_conn`.
- **Rationale:** JSON-RPC ids are monotonic per connection (obs-scope §2 gap). obs-research: no library provides a discriminator, and W3C traceparent has no consumer. The field is wire-safe under arch Mixed-version tolerance. `conn` is correlation only, never identity.
- **Impact:** §3 propagation, §4 Scenarios 2–5. Arch Standard Contracts (additive params field; needs arch acknowledgment). Security note: `pid` is Low sensitivity and permitted in diagnostics.
- **By:** `/andromeda-obs` initial run

`2026-09-24` — D-11 Third-party log targets OFF; `tracing-log` feature off
- **Decision:**
  - The `rmcp`, `axum`, `tower_http`, `hyper`, `portable_pty`, `notify`, `sysinfo` and `interprocess` targets are `OFF`.
  - tracing-subscriber is built with `default-features = false`, which drops the `tracing-log` bridge and `ansi`.
  - Their failures are captured from returned `Result`s at the viola seam as viola events (`process-exit`, `state-recovered`, `http-request`, `sse-closed{close_cause:"tail-error"}`).
- **Rationale:** bridged and third-party records lack the tests-required `event` / `corr` / `process` / `instance`, and rmcp debug events can carry frame content (obs-research). The researched fallback, json-subscriber 0.3.0, is kept for revisit if chaos tests show a seam-capture gap.
- **Impact:** §3 Logging stack, §6 per-module levels.
- **By:** `/andromeda-obs` initial run

`2026-09-24` — D-12 No `trace_id` / `span_id`; OTel SDK deferred; null-as-absence
- **Decision:**
  - No W3C trace context and no `trace_id` / `span_id` fields; correlation is `corr` + `conn`.
  - The OTel SDK and bridge are deferred.
  - Null `corr` / `instance` is encoded as key absence (tracing has no null value).
- **Rationale:** tests binds `with_current_span(false)`. obs-research: bridged IDs have no consumer. tracing's `Option` None records nothing.
- **Impact:** §2, §3, §6, §7. Tests must confirm absent == null, or obs switches to json-subscriber 0.3.0.
- **By:** `/andromeda-obs` initial run

`2026-09-24` — D-13 Heartbeat: transitions only, no periodic tick lines
- **Decision:** No 10–30 s `subsystem.tick` lines. The 1 s heartbeat file, `liveness-changed` transitions and the `sse-opened` / `sse-closed` lifecycle replace them.
- **Rationale:** obs-scope §3 Heartbeat ticks. The never-rotated files would grow without bound (security Vector 7).
- **Impact:** §3, §10, §11 SLO.
- **By:** `/andromeda-obs` initial run

`2026-09-24` — D-14 R8 strip logging: count + known names only
- **Decision:** Log `env_stripped_count` and `env_stripped_known`, the names drawn only from viola's compile-time list of the 14 known variables. Unknown `CLAUDE*` names are counted but not named. Values are never logged.
- **Rationale:** obs-scope §5 Vector 6 asks Phase 3 to confirm that names are admissible. Known names are not secrets, and arbitrary env names are untrusted input.
- **Impact:** §4 Scenario 1 / E2, §8.
- **By:** `/andromeda-obs` initial run

`2026-09-24` — D-15 Level control
- **Decision:** Levels come from `config.json` `diagnostics_level` (`info` default | `debug`) through an in-code `filter::Targets`. There is no `RUST_LOG`, no `EnvFilter` and no CLI flag in v1.
- **Rationale:** arch: env vars are not a configuration channel; config precedence is flags > config > defaults. Security: no knob may disable redaction; level changes volume only.
- **Impact:** §3, §6. Arch `config.json` key list (acknowledgment).
- **By:** `/andromeda-obs` initial run

`2026-09-24` — D-16 Hook without `VIOLA_NAME`: no line
- **Decision:** A hook with no instance writes no file and no line, exits 0 and leaves stdout/stderr empty. Asserted by absence.
- **Rationale:** tests E1 "silent no-op". Obs-scope §2: "Where no instance directory resolves … no log and no stderr, by contract". This overrides the Vector 4 trigger's "missing `VIOLA_NAME`" logging, which is unrealizable without a file name.
- **Impact:** §4 edge flows.
- **By:** `/andromeda-obs` initial run

`2026-09-24` — D-17 Frontend telemetry: web-vitals not vendored in v1
- **Decision:** No web-vitals 6.2.2 and no @opentelemetry/sdk-trace-web 2.11.0. The browser side uses DOM state words plus the native `performance` entries read by Playwright.
- **Rationale:** obs-research: web-vitals is optional and no tests perf gate needs Core Web Vitals. The browser SDK needs an OTLP POST (405, no listener) and a bundler (no JS build step).
- **Impact:** §4 surface table. Revisit with the v1.x authenticated remote view.
- **By:** `/andromeda-obs` initial run

`2026-09-24` — D-18 Flag: sysinfo 0.39.6 MSRV vs workspace
- **Decision:** Flag only; obs does not own the fix. crates.io lists `rust-version = 1.95` for sysinfo 0.39.6, above the workspace `rust-version = "1.89"`, and arch says "The obs stack must compile at MSRV 1.89".
- **Rationale:** obs-research Heartbeat / Tick Pattern key detail.
- **Impact:** Arch Stack: either raise the workspace MSRV or pin an older sysinfo. `liveness-changed.pid_alive` depends on it.
- **By:** `/andromeda-obs` initial run

`2026-09-24` — D-19 veil 0.3.0 adoption with `toggle` banned
- **Decision:** `#[derive(Redact)]` on channel, hook and MCP payload types as the second scrubbing layer. cargo-deny `[[bans.features]] crate = "veil" deny = ["toggle"]`.
- **Rationale:** obs-research PII Scrubbing Library. `toggle` allows env-var disabling of redaction, which the security Anti-Patterns ban.
- **Impact:** §8, §9 lint stage, `pii-scrubbing-wire` (merged with security's `logging-redaction-wire`).
- **By:** `/andromeda-obs` initial run

`2026-09-24` — D-20 Exit-cause `detail` codes
- **Decision:**
  - exit 1: `already-live|squatted-name|pinned-hash-mismatch|batch-script-child|internal-error`;
  - exit 21: `instance-dead|strict-modes-failed|server-verify-failed`, plus `during:"connect"|"call"`;
  - exit 20: `wrapper-fault`.
  - No path or pid in any code.
- **Rationale:** Founder Direction 4.
- **Impact:** §6 detail catalog, §4 Scenarios 1 and 3.
- **By:** `/andromeda-obs` initial run

`2026-09-24` — D-21 Tests amendment request (test-plan.md §3), accepted at review
- **Decision:** test-plan.md §3 is amended, and the overseer applies it right after this run:
  - the closed `event` enum gains `release-from-driver` (D-01), `liveness-changed` (D-02), `state-recovered` (D-03), `sse-opened` / `sse-closed` (D-04) and `parse-rejected` (D-05), each with its `corr` rule from §3;
  - the `process` enum and the harness `--process` filter gain `cli`, and the process-log file list gains `cli-<name>.ndjson` (D-06);
  - `corr` / `instance`: "absent = null" (D-12);
  - the harness `logs` glob adds `instances/*/diagnostics/detail-*.ndjson` (D-08).
- **Rationale:** Phase 3.5 review (overseer, founder-delegated). Founder Direction 1 makes tests the owner of the field and event lists, so the additions travel as a tests amendment rather than an obs-only extension.
- **Impact:** §3 schema extensions, §3 Log file location, §6. Blocking for `/andromeda-implement` until test-plan.md carries it.
- **By:** Phase 3.5 review

`2026-09-24` — D-22 Arch amendment request (architecture.md), accepted at review
- **Decision:** architecture.md is amended:
  - Diagnostics location: `<home>/diagnostics/{run-<name>,hook-<name>,mcp,ui-<port>,cli-<name>}.ndjson` for codes-only process logs, plus `<home>/instances/<name>/diagnostics/detail-<process>.ndjson` for content-bearing detail (D-08). Arch currently names only `instances/<name>/diagnostics/`.
  - Workspace `rust-version` rises from 1.89 to 1.96, matching security's toolchain floor of >= 1.96. sysinfo 0.39.6 (MSRV 1.95) then fits, which resolves D-18.
- **Rationale:** Phase 3.5 review (overseer, founder-delegated).
- **Impact:** §3 Log file location, §3 Trace context propagation (axum-tracing-opentelemetry is no longer rejected on MSRV), §4 Heartbeat (`liveness-changed.pid_alive`).
- **By:** Phase 3.5 review

`2026-09-24` — Review 1: A/B/C accepted as tests amendment D-21; D-08 and the 1.96 floor (D-18) accepted as arch amendment D-22; tier Standard with Minimal exporter carve-outs confirmed (overseer, founder-delegated).

`2026-09-24` — D-23 `tracing-log` off: what portable-pty / notify failures still surface
- **Decision:** With the bridge off (D-11) and no hand-installed `LogTracer`, `log` records from portable-pty 0.8.1 and notify 8.2.0 are dropped. The failure classes that obs-research flags are still captured from returned values or handles:
  - `pty.spawn` / ConPTY spawn failures are returned as `Err` from the seam. They become `process-exit{subject:"self", exit_code:1, detail:"internal-error"}`, with the chain in `detail-run.ndjson`.
  - Child exit without EOF is detected by the handle-wait thread (`exit_source`), independent of `log`.
  - notify watcher errors reach the event handler as `Err(notify::Error)`. They surface through `state.tail_notify` as `sse-closed{close_cause:"tail-error"}`.
  - Accepted loss: only third-party `log` context emitted without a returned error. It has no `event` / `corr` / `process`.
- **Rationale:** obs-research, tracing-subscriber `tracing-log` finding. D-11 answered the field-shape and rmcp-content concern, not the dropped-failure concern.
- **Impact:**
  - The chaos cases "child exit without EOF" and "notify watcher error" must each find a viola line.
  - If one does not, that is a seam gap, handled through D-11's revisit path. Never re-enable `tracing-log` and never install `LogTracer` by hand.
  - Affects §3 Logging stack and §6 per-module levels.
- **By:** `/andromeda-obs` iteration 1

`2026-09-24` — D-24 D-12 json-subscriber fallback closed
- **Decision:** D-12's fallback ("tests must confirm absent == null, or obs switches to json-subscriber 0.3.0") is closed. D-21 (accepted) states "absent = null" for `corr` / `instance`. D-11's separate seam-gap revisit stays open.
- **Rationale:** the condition named in D-12's Impact is met.
- **Impact:** §3 Null encoding, §6.
- **By:** `/andromeda-obs` iteration 1

`2026-09-24` — D-25 Iteration-1 contract corrections
- **Decision:**
  - The cross-process channel join key is `(conn, corr)`, not `(instance, conn, corr)`. Each side logs its own instance.
  - `cli` `instance` / file precedence is the caller's `VIOLA_NAME`, then the target argument.
  - `obs_event!` and the enums live in viola-core. `MillisUtc`, `viola_obs_init` and the panic hook live in the root bin, with a direct `chrono` dependency. The `disallowed-macros` exemption is an allow attribute inside the macro expansion.
  - The fmt builder ceiling is `Level::DEBUG`, and `Targets` is the only level gate.
  - The CI harness output goes to its own `harness-<os>` artifact.
- **Rationale:**
  - `(instance, conn, corr)` could not join MCP- or CLI-originated calls.
  - The default INFO ceiling made `diagnostics_level:"debug"` a no-op.
  - upload-artifact v4+ rejects a second upload of the same artifact name.
- **Impact:** §3 (Init body sketch, Snapshot / paste-to-AI, Trace context propagation, Bootstrap phases), §9.
- **By:** `/andromeda-obs` iteration 1

`2026-09-24` — D-26 Iteration-3 contract corrections
- **Decision:**
  - The panic hook's hand-written line encodes a null `corr` / `instance` by key absence, like `obs_event!`. `schemas/diag-line.v1.json` declares both as optional keys and rejects a literal `null`.
  - The one-`write_all` append rule and torn-line tolerance cover every multi-writer file: `hook-<name>.ndjson`, `mcp.ndjson` and `cli-<name>.ndjson`. A concurrent-append check on windows-2025, macos-latest and ubuntu-latest is a multi-platform-exporter-compat item.
  - tracing-subscriber 0.3.23 is built without its `chrono` feature. That feature only enables `ChronoUtc` / `ChronoLocal`, which viola does not use; `MillisUtc` uses the direct `chrono` 0.4.45 dependency (D-25). This narrows the obs-research catalog's feature list.
- **Rationale:**
  - Two emitters with different null shapes would force a schema that accepts both and hides drift.
  - The earlier 4 KiB claim covered `hook` only and relied on a pipe rule.
  - An unused feature adds dependency surface with no caller.
- **Impact:** §3 (OTel SDK init, Logging stack, Null encoding, Bootstrap phases) and §7 Panic hooks.
- **By:** `/andromeda-obs` iteration 3

`2026-09-24` — D-27 Iteration-4 contract corrections
- **Decision:**
  - The CI uploads are gated on `steps.secret-scan.outcome` (`== 'success'` for `diag-<os>` / `harness-<os>`, `== 'failure'` for the hit report), never on a bare `always()` / `failure()`. The scan step runs `if: always()` and also covers the `agent-run` capture in `target/agent-run/`. Uploading the harness artifact is admissible under the same synthetic-input rule as detail files.
  - `send-*` lines with a cursor `corr` are emitted only by the target wrapper. The send join is target `instance` + `corr`.
  - A main-thread panic in `mcp` / `ui` exits 1 after `process-exit{detail:"internal-error"}`. Tokio task panics are contained.
  - G3 also catches `'abort'`, unquoted `panic=abort` (rustflags) and `CARGO_PROFILE_*_PANIC` in workflow files.
  - §11 Logs bans the raw print / `dbg!` macros (clippy `print_stdout` / `print_stderr` / `dbg_macro`) and a hand-installed `LogTracer`.
- **Rationale:**
  - A bare `always()` / `failure()` still runs after a failed scan.
  - A byte-offset cursor collides across instances.
  - The §7 catch site had no exit defined for `mcp` / `ui`.
  - Cargo accepts more spellings of the abort strategy than the old G3 matched.
- **Impact:** §3 Snapshot / paste-to-AI, §7 Panic hooks, §8 Integration points, §9, §11 Logs.
- **By:** `/andromeda-obs` iteration 4

`2026-09-24` — D-28 Iteration-5 contract corrections
- **Decision:**
  - G4 is the schema-conformance gate over real CI output. Home-level lines are checked against `diag-line.v1.json` and detail lines against `diag-detail.v1.json`, and non-JSON lines are skipped and counted. The check body and validator crate are tests-owned; the schemas are obs-owned.
  - Integration-test viola homes live under `target/e2e-home/`, so G2, G4, the secret scan and the `diag-<os>` upload cover them.
  - `config.json` is parsed once at init step 5, before the subscriber exists. A failed or partial parse falls back to `info` and is logged as `parse-rejected{parser:"config-json"}` right after `process-start`.
  - The pre-clap role is the first argument that is not a global flag or a flag's value, not `argv[1]`.
  - The detail files written by multi-process roles (`detail-hook`, `detail-cli`, `detail-mcp`) fall under the one-`write_all` rule, the torn-line tolerance and the concurrent-append check, with no size guarantee.
  - A `send` refused before `send-issued` logs `corr` = the target's `events.ndjson` end offset at refusal. It is a join key only.
- **Rationale:**
  - §10 counted conformance failures that no gate measured.
  - A `hook` panic exits 0, so an unscanned integration home hides it.
  - The level filter needs the config before the subscriber exists, while `parse-rejected{config-json}` needs the subscriber.
  - `viola --home <dir> hook …` puts `--home` in `argv[1]`.
  - Detail lines are content-bearing and can exceed 4 KiB.
  - The arch `send` refusal result has no cursor, yet Scenarios 2, 5 and 6 logged `corr:<cursor>` on refusals.
- **Impact:** §3 (OTel SDK init, Logging stack, Trace context propagation), §7 Panic hooks, §9 (Pipeline integration, Gate commands, Step order). Tests: the G4 check and the integration home layout.
- **By:** `/andromeda-obs` iteration 5

`2026-09-24` — D-29 Iteration-7 contract corrections
- **Decision:**
  - The init-step-3 instance source is fixed per role. `run` uses its own argument and ignores an inherited `VIOLA_NAME`. `hook` and `mcp` use `VIOLA_NAME` only. `cli` keeps the D-25 precedence.
  - `ui` resolves `<port>` (flag → `config.json` → 47319) at step 3 by running the step-5 `config.json` parse early. Its pre-init panic window includes that parse.
  - `detail-<process>.ndjson` is opened lazily by the first content-bearing record or by the panic hook, never at step 4. A failed detail open drops only the detail line.
  - `run-<name>.ndjson` and `ui-<port>.ndjson` fall under the one-`write_all` rule and the concurrent-append check, because a colliding second `run` / `ui` appends to them.
  - G2 runs `if: always()`, like G4.
- **Rationale:**
  - Step 3 allowed `VIOLA_NAME` or the argument with no per-role rule, so a nested `viola run B` could log as its parent instance.
  - Step 4 opened `ui-<port>.ndjson` before step 5 parsed the `config.json` that can hold the port.
  - Step 4 named only the role file, which left the detail file's open point undefined, along with the `hook` one-`open` claim that depends on it.
  - Scenario 1 logs exit-1 collision causes into the live process's file.
  - A skipped G2 leaves the §10 panic count unmeasured on exactly the failing runs.
- **Impact:** §3 (OTel SDK init, Logging stack), §7 Panic hooks, §9 Step order, §10 exemptions. Tests: the concurrent-append check adds the collision path.
- **By:** `/andromeda-obs` iteration 7

`2026-09-24` — D-30 Iteration-8 contract corrections
- **Decision:**
  - Every wrapper `send-*` line carries `conn` (or `srv_conn`) and `rpc_id`, the JSON-RPC `id` of the originating `send` request. `(conn, rpc_id)` on a send line joins `(conn, corr)` on that call's `channel-*` lines.
  - Detail-file lines never pass through the tracing subscriber. The root-bin `viola::obs` detail writer formats them itself, as the panic hook does.
  - Perf-gate hook runs use `VIOLA_NAME` and a home under `target/e2e-home/`, and G2 covers that home.
  - The §5 quantile snippet reads lines with `-R` + `fromjson?`.
- **Rationale:**
  - Send lines carried only the cursor and channel lines only the JSON-RPC `id`, so no field joined them. D-28's end-offset `corr` also lets refusals at the same offset collide.
  - The subscriber has one writer, the home-level role file. A `chain` or `drift_report` emitted through it would land in a codes-only file.
  - Without `VIOLA_NAME` the hook is the D-16 no-op, so the `max < 1.0 s` gate timed no log open and no channel call, and a perf-job hook panic went unseen.
  - `jq -s` aborts with exit 2 on a torn line.
- **Impact:** §3 (Logging stack Sink, Snapshot / paste-to-AI, Trace context propagation), §5, §6 catalog, §10 budget table. Tests: the perf-job hook fixture.
- **By:** `/andromeda-obs` iteration 8

`2026-09-24` — D-31 No error reporter in v1; iteration-9 contract corrections
- **Decision:**
  - There is no error reporter in v1. This is the Minimal-tier carve-out from obs-scope §6, alongside the no-exporter decision in the initial entry. sentry 0.49.3 / sentry-tracing 0.49.3 are inadmissible. Capture stays local: thiserror 2.0.20 fixed `Display`, anyhow 1.0.104 at the root-bin edge, the hand-written panic hook, and the §7 `catch_unwind` sites. A future reporter needs the §7 preconditions plus a security Decisions Log entry for the egress.
  - `parse-rejected.detail` is a closed per-parser list (§6 detail catalog), enforced by `diag-line.v1.json`.
  - The Scenario 2, 5 and 6 field lists carry `conn` / `rpc_id` on wrapper `send-*` lines (D-30).
  - The §5 Counter derivation reads home-level files only, with `-R` + `fromjson?`.
  - The scan step writes its hit report to `target/secret-scan/`, and §9 step 5 uploads it.
- **Rationale:**
  - The reporter carve-out was traced only to §7 prose.
  - The `detail` catalog was declared closed, but `parse-rejected` had no value list.
  - D-30 updated the §6 catalog but not the scenario field lists that `/andromeda-implement` reads.
  - `jq -s` exits 2 on a torn line, and detail files repeat lines.
- **Impact:** §2 (Errors row, invariants, trigger map), §4 Scenario 2, §5, §6 detail catalog, §8 Integration points.
- **By:** `/andromeda-obs` iteration 9

`2026-09-24` — D-32 Iteration-10 contract corrections
- **Decision:**
  - `conn` becomes `"<process>-<pid>-<t0>-<n>"`, where `t0` is the process's start instant in Unix epoch milliseconds, captured once at init. It stays an additive, log-only string (D-10). `srv_conn` is unchanged.
  - The `junit-<os>` upload runs `if: always() && steps.secret-scan.outcome == 'success'`. The `perf-<os>` upload runs `if: always()`.
  - The hyperfine perf gates run in the §9 step-1 block of the per-OS job, before the secret scan.
  - A failed step-4 role-file open is a fourth bounded exemption from the zero-unlogged-panics invariant. The failure condition keys on a `process-start` present in a home-level role file.
- **Rationale:**
  - PIDs are recycled, quickly on Windows. Each `hook` process opens one connection (`n` = 1) and sends the same first JSON-RPC `id`, so the D-25 `(conn, corr)` join could merge two hook calls.
  - An `if:` without a status function gets an implicit `success()`, so the JUnit XML and the perf sample were lost on exactly the failing runs.
  - A separate perf job would leave its hook home unscanned and not uploaded, or would collide on the `diag-<os>` artifact name.
  - The panic hook writes only to the step-4 handle, so `sink` and the `mcp` stderr fallback get no panic line.
- **Impact:** §3 (Log format illustrative line, Trace context propagation), §9 (Telemetry artifact handling, Step order), §10 exemptions. Arch: the D-10 `conn` acknowledgment covers the new format.
- **By:** `/andromeda-obs` iteration 10

`2026-09-24` — overseer fix pass 2, 2026-09-24 (cross-plan findings "obs vs upstreams" and "a11y P3.5", founder-delegated; each item checked against the cited line before the edit)
- **Decision:**
  - B1 (as ruled): in CI, homes are kept until the gate steps have run (`AGENT_RUN_KEEP_HOMES=1`, set by `ci.yml`, honoured by harness `cleanup` and the rstest homes). Tests states the same. §9 Step order.
  - B2 (as ruled): the home strict-modes check, including a viola-created home, runs first, and diagnostics init runs after it. `diagnostics/` and every diagnostics file get the same owner / mode (DACL) / symlink checks as the home, and a failed check is handled like a failed open. §3 Init order step 4. The init order is an arch amendment, routed with the D-22 intent cap.
  - B3: the verbatim tests log-format copy is re-synced to test-plan.md §3 after 693e083 and fix pass 2 (`cli`, the detail files, the D-21 events, null as absence, the pre-issue `corr`, `conn` / `rpc_id`, `a11y-violation`).
  - B5: the print ban is scoped to the product crates. `viola-harness` and the fake agent are exempt through their own `[lints.clippy]` table, with a CI member-list assertion. §3 obs-ci-gate-wire and §11 Logs.
  - B8: the `logs` detail-line wrapper carries `instance`, per tests. §3 Snapshot / paste-to-AI.
  - B9: "verify never in CI" is replaced by "verify runs in CI only against the fake agent". §4 `verify`.
  - B11: citations of design "telemetry hints" that design-system.md does not contain are dropped or re-anchored to real design states (readback words, `TAPE` states, `stale`, the ATIS `skipped` box). §1 (telemetry surfaces and the path source), §5 metrics.
  - Y3: `a11y-violation` joins the accepted `event` values (a11y-plan D-A11Y-09, D-21 route). It is a harness artifact only, never in `diagnostics/`.
- **Rationale:** the overseer's audit "obs vs upstreams" (2026-09-24 05:10) and the a11y P3.5 items (05:52). Obs aligns to tests, and tests is upstream.
- **Impact:** §1, §3 (Init order, Log format, obs-ci-gate-wire, Snapshot / paste-to-AI), §4, §5, §9, §11. The arch amendment for B2 is routed with D-22. Architecture, security and a11y plans are unchanged.
- **By:** manual edit, overseer fix pass 2, 2026-09-24 (founder-delegated).

`2026-09-24` — overseer fix pass 3, 2026-09-24 (cross-plan findings "a11y vs upstreams", founder-delegated; checked against the cited lines first)
- **Decision:**
  - Z7: `a11y-violation` leaves the product `event` enum. It has no `ObsEvent` variant, no product process emits it, and `schemas/diag-line.v1.json` does not contain it. It is a harness-only row with its own tests-owned schema, `e2e-web/schemas/a11y-row.v1.json`, which reuses the diag-line field names without renaming any.
  - This revises fix pass 2 Y3, whose "accepted `event` value" bullet is replaced. The verbatim tests log-format copy is re-synced again so it carries the tests bullet "Harness-side a11y rows".
- **Rationale:** a product-enum value would force an `ObsEvent` variant, and the D-26 schema coverage, for a row no product code writes (finding Z7).
- **Impact:** §3 (Log format copy, Obs extensions), §6 (Log format pointer), §8 (Default-deny posture). The a11y plan's §3 wording and D-A11Y-09 still say "closed-enum value … tests + obs amendment"; they are left for the overseer to reconcile.
- **By:** manual edit, overseer fix pass 3, 2026-09-24 (founder-delegated).

`2026-09-24` — D-33 Raw-tracing ban: path ban plus raw-`event!` grep (supersedes D-25's `disallowed-macros` exemption clause)
- **Decision:**
  - D-25's clause "The `disallowed-macros` exemption is an allow attribute inside the macro expansion" is superseded.
  - `clippy.toml` bans `tracing::{info,warn,error,debug,trace}` by path.
  - Raw `event!` is caught by the fail-closed grep in `scripts/lint-probes.sh`, which proves itself both ways.
  - `obs_event!`'s inner allow stays but has no effect.
- **Rationale:** as measured on clippy 1.98.1 at chunk 2026-09-24-observability-gates (`.andromeda/runs/2026-09-24T12-21-11-implement/clippy-disallowed-macros-measurement.md`), every allow placement inside the macro, at the call site and on the calling fn still reported the ban. Only a crate-level `#![allow]` in the caller silences it, which would disable the ban for that crate. With `tracing::event` listed, all 16 `obs_event!` call sites failed.
- **Impact:** §3 logger-stack-install and obs-ci-gate-wire, §8 PII Scrubbing item 1, §9 Lint row, §10 failure conditions, §11 Logs.
- **By:** operator decision at /implement P1 (overseer, founder-delegated), applied by wrap-session 2026-09-24.

`2026-09-25` — D-34 R8 strip logging names every stripped and kept name (supersedes D-14)
- **Decision:**
  - D-14 ("names drawn only from viola's compile-time list of the 14 known variables; unknown `CLAUDE*` names are counted but not named") is superseded.
  - `process-start{subject:"claude-child"}` carries `env_stripped_count`, `env_stripped_known` (every stripped name, comma-joined) and `env_kept` (every kept name, comma-joined). Names only: no value is read by the strip, the registry reader or any log, and no env map is passed to a span or event.
  - The strip removes every inherited `CLAUDE*` name (ASCII case-insensitive on Windows, exact on Unix) except the persistent set — Windows `HKCU`/`HKLM` `Environment` value names, Unix `config.json` `claude_env_keep` — and always the 11-name identity floor `IDENTITY_FLOOR` measured on the Windows host. There is no 14-name list.
- **Rationale:** operator ruling 1 at chunk 2026-09-25-pty-wrapper-on-windows (P4): each stripped and kept name is named in the log. The S6 "14" list was never enumerated by any artifact (the chunk's research fact 7). `CLAUDE*` names are not secrets; their values stay never-log (§8).
- **Impact:** §4 Scenario 1 / E2, §6 `process-start` catalog, §8 data classification row.
- **By:** operator ruling (overseer, founder-delegated), applied by wrap-session 2026-09-25.

`2026-09-29` — D-35 `verify-probe` joins the closed spawn `subject` enum
- **Decision:**
  - `viola verify` logs each of its two child spawns as a `process-start` / `process-exit` pair to `cli-<name>.ndjson` (only when `VIOLA_NAME` resolves), codes only: `subject:"version-probe"` for the `--version` read and the new `subject:"verify-probe"` for the print-mode probe; `process-exit` carries `child_exit_status` and `duration_ms`.
  - The pairs sit at verify's call sites; the shared `run_bounded` stays unlogged, so `run`'s version gate keeps exactly one `version-probe` pair.
  - `schemas/diag-line.v1.json` `$defs.subject.enum` is `self|claude-child|version-probe|verify-probe|agents-probe|statusline-shell`.
- **Rationale:** §6 Child / shell spawns names every spawn as a pair; verify's two spawns wrote none (an overseer ruling at the capability-ledger wrap). A closed value needs this entry (§8 default-deny).
- **Impact:** §4 `verify`, §6 `process-start` catalog and Boundary-call wrappers. §1 keeps its verbatim wording.
- **By:** chunk 2026-09-29-verify-stamped-test-homes-and-harness, applied by wrap-session 2026-09-29.

`2026-09-29` — D-36 Sideloaded ConPTY telemetry: `run.conpty_sideload`, `pty_backend` `conpty-sideload` and the closed `sideload_fallback`
- **Decision:**
  - On Windows, `viola run`'s start step `run.conpty_sideload` runs between `run.pin_copy` and `run.version_gate`, with `outcome` ∈ `loaded|hash-mismatch|unreadable|load-failed|not-built` and `search_restricted` (bool).
  - `pty_backend` gains the value `conpty-sideload` (read from `viola_pty::pty_backend()`, which replaces the const `PTY_BACKEND`); it stays `{ "type": "string" }` in `schemas/diag-line.v1.json`.
  - `process-start{subject:"claude-child"}` gains the additive `sideload_fallback`, present only when the outcome is not `loaded`, closed in the schema's `process-start` properties as `hash-mismatch|unreadable|load-failed|not-built` (`diag-detail.v1.json` unchanged).
  - Codes only: no companion path or hash reaches a home-level line; the degrade stays on the existing info-level lines (no `warn`), with no `process-exit{exit_code:1}` and no panic.
- **Rationale:** the backends must be told apart and a degrade recorded without a byte on the human's terminal; D-35 is the closed-set widening template, and a closed value needs this entry (§8 default-deny).
- **Impact:** §4 Scenario 1 (chain, attributes, log fields), §6 `process-start` catalog. §1 keeps its verbatim wording.
- **By:** chunk 2026-09-29-sideloaded-conpty, applied by wrap-session 2026-09-29.

(Append new entries at the bottom; do not modify historical
entries.)
