# Scope — Diagnostics plane

**Marker:** `2026-09-24-diagnostics-plane` · **Version:** viola-0.1.0 · **Epoch:** Epoch 1 — Foundation

## Working-route entry (verbatim)
> Diagnostics plane — per-role JSON sinks with process-start service identity, closed event vocabulary and line schemas, owner-only per-instance detail files, diagnostics_level, torn-line-aware logs merge

Annotations folded (`route.py pins`: 2 blocks on line 17, both folded below — 1 CARRY, 1 PREREQ; no BLOCKED-ON / CONTEXT):
- **CARRY (verbatim):** chunk 2026-09-24-three-os-ci-headless-harness-skeleton shipped the first role lines with raw
  `tracing::info!`/`error!` in `src/run/mod.rs` (event/process/instance passed by hand) — migrate them to `obs_event!`;
  its panic hook writes the payload nowhere (no `detail-run.ndjson` yet) and `process-exit{subject:"self"}` has no
  `duration_ms`; the harness `logs` streams only the home-level diag source (events + detail sources and
  `--kind`/`--after` arrive with their producers)
- **PREREQ (verbatim):** close rust gate deferral (deferred since 2026-09-24-supply-chain-and-workflow-gates)

CI verdict for the last shipped sha (`4d8be5287b00a00c772e9d0a4264f08d6942f512`, read at Setup 5a via check-runs): all
9 check-runs `success` (test ×3, lint ×3, supply-chain, mutants, advisories). Nothing to fold.

**Operator-relayed notes (overseer, founder-delegated, pasted by the operator at P1 and flagged at the P5 review):**
- CI witness owed by the previous wrap: ci run `35983992260` (push) and nightly run `35984789181` (workflow_dispatch) are
  both `success` on `4d8be52`. Re-read at P1 via `gh run view`. **Record at this chunk's wrap.** It is not work this
  chunk builds.
- The write-guard fix below (item 8) is folded from that note. Its premise was closed at P3.

## What this chunk builds
1. **Closed event vocabulary in `viola-core` (`viola_core::obs`)**: the `ObsEvent` enum, one variant per closed kebab
   `event` value in obs-plan §3 Log format (tests set + accepted extensions D-01…D-05; no `a11y-violation`), with a
   kebab `Display`; the `ObsProcess` enum (`run|hook|mcp|ui|cli`, D-06); and the `obs_event!` `macro_rules!` that
   expands to `::tracing::event!` at the caller. The macro always attaches `event`, `process`, `instance` and `corr`,
   and its `message` is a static literal equal to the event name. viola-core gains no `tracing` dependency.
2. **Service identity**: `viola_core::{SERVICE_NAME, VERSION}` stays the single source. `process-start` carries
   `service_name`, `version`, `os`, `pid`. The process-global `OnceLock<ProcessCtx>` holds the role and the instance,
   so no call site passes them by hand.
   - [premise-corrected: `crates/viola-core/src/lib.rs:3-4` already defines both constants; the chunk reuses them, it
     does not add them]
3. **Per-role JSON sinks (`viola::obs` in the root bin)**: `viola_obs_init` builds the tracing-subscriber JSON layer
   with the mandatory builder settings (`log_internal_errors(false)`, `with_span_list(false)`, explicit writer,
   `with_ansi(false)`, `flatten_event(true)`, `with_current_span(false)`, `MillisUtc` timestamps). It uses one
   append-mode, 0600 file per role under `<home>/diagnostics/` (0700), with one `write_all` per line. Third-party
   targets are OFF.
   - The file-naming and open path covers all five roles (`run-<name>`, `hook-<name>`, `mcp`, `ui-<port>`,
     `cli-<name>`), and only `run` is exercised today. The other roles have no producer yet.
   - (verified: `src/cmd/mod.rs` `Command` has the single variant `Run`)
4. **`diagnostics_level`**: `config.json` key `diagnostics_level` (`"info"` default | `"debug"`) applied through a
   `filter::Targets` layer built in code. It is never `RUST_LOG`, `EnvFilter` or an env var. The level changes volume
   only, never the field allow-list.
   - `config.json` has no reader yet (verified: `grep -rn 'config.json' src crates --include=*.rs` → 0 hits).
   - This chunk adds the minimal, bounded, tolerant read of that one key: it checks `v`, skips and counts unknown
     keys, and falls back to the default when the file or the key is absent.
   - A failed or partial parse emits `parse-rejected{parser:"config-json", detail, count}` (obs D-05/D-28) after
     `process-start`. The full config surface belongs to later chunks.
5. **Owner-only per-instance detail files**: the `viola::obs` detail writer for
   `<home>/instances/<name>/diagnostics/detail-<process>.ndjson` (0600 files in 0700 dirs), opened lazily. Its lines
   are hand-formatted with the same `timestamp`/`level`/`target`/`message`/`event`/`process`/`instance` keys, one
   `write_all` each, and never pass through the subscriber.
   - CARRY: the panic hook writes its payload and a `Backtrace::force_capture()` backtrace to `detail-run.ndjson` when
     an instance resolves. The home-level panic line stays payload-free.
   - [added at P3 per obs extract, obs-plan §7 Panic hooks; `src/main.rs:32-36` maps a caught panic straight to
     `ExitCode::from(1)` with no role line] After a caught main-thread panic in `run`, the catch site writes
     `process-exit{subject:"self", exit_code:1, detail:"internal-error"}` before exiting 1.
6. **Line schemas + the CARRY migration**:
   - `schemas/diag-line.v1.json` (hand-maintained): `corr` / `instance` are optional keys and a literal `null` is
     rejected.
   - [added at P3 per obs extract, obs-plan §8 Default-deny posture → Detail-file scope; tests extract §6] It gains its
     sibling `schemas/diag-detail.v1.json`, because detail files get their first producer here.
   - A tests-owned check asserts that the schema's `event` enum equals the obs-plan §3 values, pinned as literals in
     the test. Verified against obs-plan §3 log-format-schema-emit and test-plan §11 Unit (no product-list oracle).
   - CARRY: the raw `tracing::info!`/`error!` role lines in `src/run/mod.rs` are migrated to `obs_event!`, and
     `process-exit{subject:"self"}` gains `duration_ms`.
     `grep -rn -E 'tracing::(info|error|warn|debug|trace|event)!' src crates --include=*.rs` → 6 call sites, all in
     `src/run/mod.rs`.
7. **Torn-line-aware logs merge**: the harness `logs` command merges every `diagnostics/*.ndjson` line
   (`{"src":"diag","file","record"}`) with every instance detail line (`{"src":"diag","file","instance","record"}`). A
   torn line is emitted as `{"torn":true,"offset":n}`, and `--instance` / `--process` filters are supported.
   - The `events.ndjson` source and the `--kind` / `--after` filters stay with the events producer.
   - (verified: `grep -rln 'events.ndjson' src crates --include=*.rs` → 0 files)
   - An unknown `--kind` / `--after` already exits 2 `reason:"usage"` through clap
     (`crates/viola-e2e/src/bin/viola-harness.rs:77-85` → `Outcome::usage`, `harness/mod.rs:43-47`).
8. **Write-guard repair (`.claude/settings.json` PreToolUse `Edit|MultiEdit|Write|NotebookEdit` hook),
   folded from the operator-relayed overseer note.**
   - Claim, verbatim: "the .claude/settings.json PreToolUse write guard blocks NOTHING (overseer1 relay W154,
     re-measured by the overseer): under Git Bash path=${path//\//} deletes forward slashes. Measured fix: replace it
     with path=$(printf "%s" "$path" | tr "\\\\" "/");"
   - Acceptance, verbatim: "the stdin-payload probe against the RENDERED command, run with Git Bash, gives exit 2 for
     target/x.rs, target\x.rs and both absolute forms, exit 0 for src/x.rs and src\x.rs. The probe is
     D:\dev\projects\additional\viola-overseer\guard_probe.py (pass the Git Bash path)."
   - VERIFIED at P3 (research.md §Measured facts), under GNU bash 5.2.37 (x86_64-pc-msys):
     - The shipped form exits 0 on all six probe paths.
     - The fixed form exits 2/2/2/2 on the four `target` forms and 0/0 on the `src` forms.
     - `${path//\\//}` turned `a/b\c/d` into `ab\cd`, which confirms the forward-slash deletion mechanism.
9. **PREREQ, closing the Rust gate deferral**: this chunk carries a `.rs` delta, so the Rust unit suite
   (`scripts/agent-run.sh run --unit`) runs and passes at this chunk's gates. The deferral from
   2026-09-24-supply-chain-and-workflow-gates (zero `.rs` delta) is closed by that run, not carried again.

## Boundaries (not this chunk)
- `#[instrument(skip_all, fields(..))]` everywhere, veil `Redact` payload types, fixed thiserror `Display`s, and routing
  content-bearing records (anyhow chains, serde drift reports) to detail files are **Log redaction and never-log
  floor**. This chunk builds the detail sink and the panic-payload routing only.
- The panic-hook-first gate, G1–G4 CI steps (zero-panic, schema-conformance, bare-instrument, abort-panic), canary
  secret scan, `clippy.toml` `disallowed-macros` (no `clippy.toml` exists yet), and
  `print_stdout`/`print_stderr`/`dbg_macro` workspace lints are **Observability gates**.
  - The `#[allow(clippy::disallowed_macros)]` inside `obs_event!`'s expansion is written here, because the macro's
    shape is this chunk's (obs-plan §3 logger-stack-install). The ban that makes it matter lands there.
- Home strict-modes check before the `diagnostics/` open (obs init step 4) → **Home and code-bearing file integrity**
  (security extract: walking-skeleton `--home` gap owned there).
- `corr`/`conn` population on channel, dialog and send lines, and `Span` hand-off into threads/tasks belong to the
  chunks that build those producers (channel, hook, send). This chunk defines the vocabulary they emit into.
- Worker-thread `catch_unwind` (PTY pump, handle-wait) → the PTY seam chunk: `run` has no worker thread yet
  (`src/cmd/run.rs:33-35` waits on the child in `main`).
- `liveness-changed`, `sse-*` emitters belong to viola-ui chunks. The `events.ndjson` writer belongs to the wrapper/events
  chunks.
- Log rotation: N/A in v1 (no rotation library).

## Surfaces / contracts touched
- New: `crates/viola-core/src/obs.rs` (re-exported from `lib.rs`); a root-bin `src/obs.rs` (`viola::obs`).
- Modified: `src/main.rs` (panic hook → detail file, run catch-site exit line), `src/run/mod.rs` and `src/cmd/run.rs`
  (migrate to `obs_event!` / `viola_obs_init`).
- New: `schemas/diag-line.v1.json`, `schemas/diag-detail.v1.json`.
- Modified: the harness `logs` command (`crates/viola-e2e/src/harness/logs.rs`).
- Modified: `.claude/settings.json` (item 8). New: `scripts/guard-probe.py`, the repo-local rendered-guard regression
  probe (operator's P4 choice, because the external probe asserts the shipped form and cannot witness the fix).
- New: `tests/contract_diag_schema.rs` (schema conformance); `viola_core::MAX_FRAME` (the bounded config read, per
  security-plan §Input Validation). Both were added at P5 validation-1 (intent-incomplete, justified by the plan).
- Specs read: obs-plan §3 (service identity, logging stack, log format, file location, paste-to-AI merge) / §6 Log
  Coverage / §7 Error Capture / §8 Default-deny; test-plan §3 Log format + `logs` command; architecture §Occupied
  Resources (home tree, `config.json`) / §Cross-cutting Patterns → Config management; security-plan §Bootstrap
  `logging-redaction-wire`.

## Premises (closed at P3 — research.md §Measured facts)
- Only the `run` role has a producer today. (VERIFIED: `src/cmd/mod.rs` `enum Command { Run }`.)
- No `config.json` reader exists yet. (VERIFIED: 0 hits, command above.)
- [premise-corrected: `crates/viola-core/src/lib.rs:3-4` defines `SERVICE_NAME` / `VERSION`] `viola_core` has the
  identity constants already; only the `obs` module is absent.
- No `events.ndjson` producer exists yet. (VERIFIED: 0 files, command above.)
- Item 8's mechanism claim. (VERIFIED: probe + mechanism probe, research.md.)
