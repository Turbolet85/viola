---
paths:
  - "scripts/agent-run.*"
  - "crates/viola-e2e/src/**"
  - "crates/viola-e2e/Cargo.toml"
  - "tests/support/**"
  - "src/bin/viola-fake-agent.rs"
  - "fixtures/fake-scripts/**"
---

# Verification Harness Rules

Path-scoped rules for the agent-driven harness driver: the `scripts/agent-run.*` shims, `viola-harness`, the rstest fixture chains and the fake agent.

**Authoritative sources (bound):**
- `.andromeda/test-plan.md` §3 — the 5-command contract, status shape, log format, session record, test data bootstrap
- `.andromeda/obs-plan.md` §3 — the log sinks, the `event` enum, `corr` / `conn`, null-as-absence

A change on one side keeps the other in sync; a log-format break is a harness break.

## Shims
- `scripts/agent-run.sh` and `scripts/agent-run.ps1` are identical thin shims: `cargo run -q -p viola-e2e --bin viola-harness -- <command> [flags]`, forwarding exit code and stdout unchanged. All logic lives once, in Rust.
- Agent surface: exactly `boot · run · status · cleanup · logs`. `supervise`, `ui-restart`, `gate`, and the CI gate bodies `schema-check` (G4) and `secret-scan` are internal subcommands, forwarded unchanged by both shims (test-plan §3 Internal harness subcommands). A 6th agent command needs a test-plan amendment.
- Every command prints exactly one JSON document on stdout starting `{"v":1,"cmd":"<command>","ok":<bool>,…}`; exit 0 ok · 1 failure · 2 usage. A human-only message without the JSON document is forbidden.

## boot
- Steps in order: `cargo build --workspace --features viola/fake-agent` with `CARGO_TARGET_DIR=target/harness` (the CLI's `<bin dir>` is `target/harness/debug`: a running `target/debug/viola-harness.exe` cannot be relinked on Windows) → a not-yet-existing home under `target/e2e-home/viola-session-*/home` → copy the fake agent to `target/agent-run/<session>/bin/claude[.exe]` on a harness-scoped `PATH` (per child via `Command::env`) → `viola verify --home` against it unless `--unstamped` → `viola-harness supervise` as an ordinary child (outlives `boot`, stops only via `stop.request`) owning one portable-pty outer PTY per instance via `viola-pty` (master writer kept until the child exits, one drain thread per master) → optional `viola ui` → token exchange (303 + `HttpOnly; SameSite=Strict`, cookie stored 0600, never printed).
- Readiness is a bounded 100 ms file-state probe. Interim (until snapshot/heartbeat/events exist): `diagnostics/run-<name>.ndjson` has `process-start` for `subject:"self"` and `subject:"claude-child"`, both pids alive by pid + start time. Target: snapshot has `endpoint/pid/started_at/child_pid`, heartbeat < 5 s, `events.ndjson` lines 1–3 = `wheel{cause:start}` → `budget-gate` → `session-start{source:hook}`; UI `/health` ok and `/ready` `"ready"`. Failure reasons are the closed list in test-plan §3.
- After UI readiness, byte-compare every embedded `/assets/*` with the repo file (`stale-embedded-assets`).
- No nested `cargo run` inside nextest: tests use the `harness_session` fixture or the rstest chain `home → fake_agent_path → stamped_home → booted_wrapper`.

## status / cleanup / logs
- `status`: `viola list --json` + `/ready` + cookie-gated `/api/info` and `/api/sessions`; `api_sessions_equal_list` deep-equals `.ok.items`; exit 0 only for `state:"ready"`.
- `cleanup`: `stop.request` → Ctrl-C into each outer PTY, pressed again every 500 ms until the child exits (a Ctrl-C into a child not yet raw is swallowed) → `child.wait()` 10 s → `kill()`; `processes_gone` proves supervisor, wrappers and children gone (endpoint/port/url fields `null` until those surfaces exist); verify endpoint gone (CLI exit 21, socket path absent), port free, `ui/<port>.url` removed. Idempotent (no session → `ok:true, cleaned:[]`). Kill targets verified by pid + start time. `AGENT_RUN_KEEP_HOMES=1` (CI) keeps the home (`home_removed:"kept"`).
- `logs`: merge `events.ndjson` (`src:"events"`, byte `offset`), home `diagnostics/*.ndjson` and `instances/*/diagnostics/detail-*.ndjson` (with `instance`); torn lines emitted as `{"torn":true}`, never dropped; filters `--instance --kind --process --after`.

## Fake agent (`viola-fake-agent`, feature `fake-agent`)
- Answers `--version` like the real CLI. It reads hook commands from `<plugin-dir>/hooks/hooks.json` (the `--plugin-dir` `run` passes). Only an absolute exec-form `command` is spawned: no shell, no PATH, matchers not yet evaluated. A non-absolute command is receipted `command_absolute:false, ran:false` and never run. The payload is `<fixtures>/<cli-version>/<Event>.<variant>.json`, with only `prompt` set for UserPromptSubmit.
- Scripted turns (`{"v":1,"steps":[…]}`, `schemas/fake-script.v1.json`) gated on the `--control` file by byte offset.
- Receipt ndjson `"v":1` + kebab `kind`: `start`, `env` (names only), `fds` (Unix), `key`, `prompt` (`text`, `hex`, `bare_esc`, `origin`, `submit`), `hook`, `step`.
- Modes built: `--suppress-prompt-submit`, `--local-command-mode`, `--inject-harness-turn`, `--exit-no-eof`, `--report-version`. `--vt100-panic-bytes`, `statusline-echo` and `agents --json` land with their consumers. Exits on `\x03`.
- Root fixture chain: `tests/support/` (`home` → `fake_agent_path` → `stamped_home` interim, no stamps → `booted_wrapper`). Homes live under `target/e2e-home/viola-test-*`, kept per `AGENT_RUN_KEEP_HOMES` / `AGENT_RUN_KEEP_FAILED`. The `viola_e2e::fixtures` copy lands with its first E2E consumer.
- `run --mutants` classifies `chunk.diff` first. A diff with no `.rs` path gives `verdict:"no-rust-delta"` and never runs cargo-mutants. A diff whose `.rs` paths are all test targets (`test_target`: `tests/`, `benches/`, `examples/` at the root or under `crates/<member>/`) gives `verdict:"test-only-rust-delta"` with `rust_files`, and never builds or runs cargo-mutants either. Any other Rust delta, a mixed one included, deletes a stale `outcomes.json`, then gives `verdict:"counted"`. `--leg <name>` also writes `artifacts/mutants-verdict-<name>.json` (repo-relative names and outcomes only) and defers survivors to `gate --mutants-legs`, the CI union. More unviable than caught mutants (`unviable-exceeds-caught`: the mutant builds failed) is red at the leg itself and never deferred. cargo-mutants' outcome lines stream live to the harness's stderr, so a stalled leg names its mutant in the job log. A booted-session test must stop its session in a `Drop` guard: under a mutated `cleanup` a leaked supervisor locks its `.exe` on Windows, and every later mutant build fails.
- `run --coverage` (one instrumented `cargo llvm-cov nextest` run as suite `coverage`, then doctest) and `run --fuzz-replay` (Linux only) drive their tools through the `run_with` runner seam, so the harness's own tests use a stand-in runner and never nest `cargo llvm-cov` or `cargo fuzz` inside nextest.
- It must not drift from recorded `viola verify` fixtures (contract suite).

## Exemptions
- `viola-harness` and the fake agent print by design. `viola-e2e` omits `[lints] workspace = true` and carries its own `[lints.clippy]` without `print_stdout` / `print_stderr`; the fake agent is a `[[bin]]` of the root package (lints are per package), so it takes a crate-level `#![allow(clippy::print_stdout, clippy::print_stderr)]`. `tests/contract_lints.rs` asserts that every product member inherits the workspace lints and only `viola-e2e` opts out.

## Session Additions
_This section is owned by `/wrap-session`. setup-project preserves content added here on re-run._
- 2026-09-24: `run --mutants` mutates only lines the chunk diff touches (`--in-diff`), so a mutant CI missed on an earlier chunk is never regenerated by a later chunk that adds only a test — witness that kill another way (the new test's PASS line in that runner's job log), never by a green `mutants` job.
- 2026-09-25: On the Windows host, a diff that carries `#[cfg(unix)]` bodies cannot pass plain `run --mutants` (those bodies are compiled out, so their mutants grade missed): run it as `run --mutants --leg windows-2025` and take the verdict from the CI union of both legs.
- 2026-09-25: Never pipe `agent-run.sh boot` (`boot | cut`, `| head`): the supervisor it leaves running inherits boot's stdout, so the pipe never reaches EOF and the call hangs until its tool bound — write each harness step's output to a file.
