# Scope — Three-OS CI and headless harness skeleton

**Marker:** `2026-09-24-three-os-ci-headless-harness-skeleton` · **Version:** viola-0.1.0 · **Epoch:** Epoch 1 — Foundation (first chunk)

**Working entry (verbatim):** Three-OS CI and headless harness skeleton — SHA-pinned actions, pinned 1.96-floor
toolchain, nextest, five agent-run commands, minimal fake agent, per-role JSON line, mutation gate

**Intent anchors:** F-01 (3-OS CI from the first commit), F-02 (five agent-run commands), F-06 (mutation gate from
chunk 1), F-19 (toolchain floor 1.96), plus the minimal slices of F-03 (fake agent) and F-04 (JSON log line) that the
harness needs to have something real to boot and read. Founder rule: chunk 1 = 3-OS CI + the headless harness; no
crate without a real consumer and a test.

No annotations stand on the entry (`route.py pins`: 0 freight blocks). Setup's CI read found no checks for `13b7ee3`
(no workflow exists yet), so there is no CI verdict to fold.

## What this chunk builds

1. **Cargo workspace born.** Root `Cargo.toml` = the `viola` bin package + `[workspace] members = ["crates/*"]`,
   `resolver = "3"`, edition 2024, `rust-version = "1.96"`, `publish = false` on every member, shared
   `[workspace.dependencies]`, every profile `panic = "unwind"`; `Cargo.lock` committed.
2. **Toolchain pin.** `rust-toolchain.toml` pins ONE exact current stable (≥ 1.96) with `rustfmt` + `clippy`; the
   declared floor (1.96) is what the workspace states. The host toolchain is brought to the same pin.
3. **nextest.** `.config/nextest.toml` with `[profile.ci]` (junit path, `retries = 0`, slow-timeout, `fail-fast =
   false`), `[profile.mutants]` (`fail-fast = true`) and the `fixed-port` test group + default-profile override.
4. **Headless harness — the five commands.** `scripts/agent-run.sh` and `scripts/agent-run.ps1` as identical thin
   shims over `cargo run -q -p viola-e2e --bin viola-harness -- <command>`, forwarding exit code and stdout unchanged.
   `viola-harness` implements `boot · run · status · cleanup · logs`, each printing exactly one JSON document
   `{"v":1,"cmd":…,"ok":…}` and exiting 0 / 1 / 2 (success / failure / usage). Session record under
   `target/agent-run/<session>/session.json`; homes under `target/e2e-home/`; `cleanup` idempotent.
   - (verified P3: no product verb exists at HEAD) "skeleton" = the contract surface (argument grammar, JSON envelopes, typed exits, session record,
     home placement, idempotent cleanup, `run`'s suite selection and summary) is real now; the steps that depend on
     product verbs not yet built (`viola run`, `viola verify`, `viola ui`, `viola list`, `viola last`, the supervisor's
     outer PTYs) land with the chunks that build those verbs. What `boot` spawns and what readiness means in this
     chunk is a P4 decision against research.
5. **Minimal fake agent.** A `viola-fake-agent` bin behind the root package's `fake-agent` feature, excluded from the
   release build — (verified P3 vs test-plan §7) just enough surface for `boot` to place it as `claude[.exe]` and for a session to start
   (e.g. a version answer and a clean exit on Ctrl-C/stdin close); scripted modes, control file, receipts and
   fixture replay are the next chunk's.
6. **Per-role JSON line.** (verified P3; the writing role follows the boot-depth decision) At least one product process role writes a one-line JSON record carrying the
   harness-grepped fields (`timestamp`, `level`, `target`, `message`, `event`, `process`, `instance` / `corr` as
   absent keys) to `<home>/diagnostics/<role>.ndjson`, so `agent-run logs` streams a real product line; the panic
   hook is the first statement of `main`. The full sinks, event vocabulary, schemas and detail files are the
   Diagnostics-plane chunk's.
7. **Mutation gate.** `agent-run run --mutants`: base from `AGENT_RUN_CHUNK_BASE` or `git merge-base HEAD
   origin/main`; unreachable base → exit 1 `reason:"base-missing"` (never an empty diff); `git diff <base>...HEAD`
   → `target/agent-run/chunk.diff`; `cargo mutants --in-diff … --test-tool=nextest` under `NEXTEST_PROFILE=mutants`;
   verdict from `mutants.out/outcomes.json` (`missed == 0 && timeout == 0`), exit codes 1/4/5/6/70 →
   `reason:"mutants-exit-<code>"`. Runs locally through the harness and in CI.
   - [premise-corrected: no `main`/`origin/main` exists locally or on the remote, and test-plan §9 runs the mutation
     job on pull_request only while this project pushes one long-lived build branch — so neither the documented base
     fallback nor the PR-only job would fire per chunk] The per-chunk base source and the CI trigger that makes the
     gate fire on this branch workflow are a P4 decision (resolved at P4: `AGENT_RUN_CHUNK_BASE` → merge-base with
     origin/main → base-missing locally; CI job on push with `github.event.before` and on PR with the PR base sha).
   - (val-1, intent-incomplete) /implement never commits, so the committed-only `<base>...HEAD` diff would mutate
     nothing. The chunk diff is the working tree plus untracked files from `merge-base(base, HEAD)`, which equals the
     three-dot form on a committed tree with an ancestor base.
8. **Three-OS CI.** `.github/workflows/ci.yml` on push + pull_request: matrix `windows-2025`, `macos-latest`,
   `ubuntu-latest`, `fail-fast: false`; `permissions: {}` at the top and `contents: read` per job; every `uses:`
   pinned by full commit SHA; toolchain read from `rust-toolchain.toml`; nextest (+ cargo-mutants) installed
   version-pinned; each OS runs the build and the harness `run`; the ubuntu PR mutation job checks out with
   `fetch-depth: 0` and sets `AGENT_RUN_CHUNK_BASE` from the PR base sha; `AGENT_RUN_KEEP_HOMES=1` on every leg;
   `target/agent-run/artifacts/` uploaded per OS as `agent-run-${{ matrix.os }}`.
   - (resolved P3) `Swatinem/rust-cache` rides this chunk (test-plan §9 job table; SHA canonical in security-plan);
     the release-build step belongs to the Workspace-tree chunk ("release build free of test binaries").

## Boundaries (later Epoch 1 chunks own these)

- Fake agent scripted modes, control file, receipts, fixture chain, scrub-and-schema walk, proptest seeds →
  *Fake agent and test-data fixtures*.
- cargo-deny, zizmor, weekly advisory run, tokio-free sync-crate check → *Supply-chain and workflow gates*.
- Per-role sinks with service identity, closed event vocabulary, line schemas, detail files, `diagnostics_level`,
  torn-line-aware logs merge → *Diagnostics plane*.
- Redaction / never-log floor → *Log redaction*. Zero-panic, schema, canary scan, print/log lint bans →
  *Observability gates*.
- Fatal fmt/clippy lint in CI, the MSRV 1.96 job, per-OS coverage floors, fuzz/property replay, the `gate` verdict
  per job → *Quality gates*. (verified P3; refusal shape is a P4 decision) `agent-run run`'s `--coverage`, `--perf`, `--fuzz-replay` and `--browser`
  selectors therefore exist only as grammar here (or are refused cleanly), not as working suites.
- Arch tree amendment and code-graph planes → *Workspace tree and code-graph planes* (the Rust plane builds at this
  chunk's wrap per setup-project).
- No product crate (`viola-core`, `-pty`, `-channel`, `-state`, `-agent-claude`, `-mcp`, `-ui`) is created without a
  consumer and a test in this chunk.

## Surfaces / contracts touched

- test-plan §3 (5-command implementation, Log format, PID file, Test data bootstrap, Bootstrap phases), §9 (CI
  Integration), §10 (Mutation gate, zero-flakiness).
- architecture §Infrastructure Patterns (build system, directory tree, CI/CD approach), §Occupied Resources.
- security-plan §Dependency Security (SHA pins, `permissions: {}`, toolchain floor).
- obs-plan §3 (logging stack, panic hook ordering) — only the one-line minimum.
