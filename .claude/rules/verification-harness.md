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
- Steps in order: `cargo build --workspace --features viola/fake-agent` with `CARGO_TARGET_DIR=target/harness` (the CLI's `<bin dir>` is `target/harness/debug`: a running `target/debug/viola-harness.exe` cannot be relinked on Windows) → a not-yet-existing home under `target/e2e-home/viola-session-*/home` → copy the fake agent to `target/agent-run/<session>/bin/claude[.exe]` on a harness-scoped `PATH` (per child via `Command::env`) → `viola verify --home` against it unless `--unstamped` → `viola-harness supervise` as an ordinary child (outlives `boot`, stops only via `stop.request`) owning one portable-pty per instance (a stdin pipe per wrapper until `viola-pty` exists) → optional `viola ui` → token exchange (303 + `HttpOnly; SameSite=Strict`, cookie stored 0600, never printed).
- Readiness is a bounded 100 ms file-state probe. Interim (until snapshot/heartbeat/events exist): `diagnostics/run-<name>.ndjson` has `process-start` for `subject:"self"` and `subject:"claude-child"`, both pids alive by pid + start time. Target: snapshot has `endpoint/pid/started_at/child_pid`, heartbeat < 5 s, `events.ndjson` lines 1–3 = `wheel{cause:start}` → `budget-gate` → `session-start{source:hook}`; UI `/health` ok and `/ready` `"ready"`. Failure reasons are the closed list in test-plan §3.
- After UI readiness, byte-compare every embedded `/assets/*` with the repo file (`stale-embedded-assets`).
- No nested `cargo run` inside nextest: tests use the `harness_session` fixture or the rstest chain `home → fake_agent_path → stamped_home → booted_wrapper`.

## status / cleanup / logs
- `status`: `viola list --json` + `/ready` + cookie-gated `/api/info` and `/api/sessions`; `api_sessions_equal_list` deep-equals `.ok.items`; exit 0 only for `state:"ready"`.
- `cleanup`: `stop.request` → Ctrl-C into each outer PTY (into each wrapper's stdin pipe, then closed, until `viola-pty`) → `child.wait()` 10 s → `kill()`; `processes_gone` proves supervisor, wrappers and children gone (endpoint/port/url fields `null` until those surfaces exist); verify endpoint gone (CLI exit 21, socket path absent), port free, `ui/<port>.url` removed. Idempotent (no session → `ok:true, cleaned:[]`). Kill targets verified by pid + start time. `AGENT_RUN_KEEP_HOMES=1` (CI) keeps the home (`home_removed:"kept"`).
- `logs`: merge `events.ndjson` (`src:"events"`, byte `offset`), home `diagnostics/*.ndjson` and `instances/*/diagnostics/detail-*.ndjson` (with `instance`); torn lines emitted as `{"torn":true}`, never dropped; filters `--instance --kind --process --after`.

## Fake agent (`viola-fake-agent`, feature `fake-agent`)
- Answers `--version` like the real CLI. It reads hook commands from `<plugin-dir>/hooks/hooks.json` (the `--plugin-dir` `run` passes). Only an absolute exec-form `command` is spawned: no shell, no PATH, matchers not yet evaluated. A non-absolute command is receipted `command_absolute:false, ran:false` and never run. The payload is `<fixtures>/<cli-version>/<Event>.<variant>.json`, with only `prompt` set for UserPromptSubmit.
- Scripted turns (`{"v":1,"steps":[…]}`, `schemas/fake-script.v1.json`) gated on the `--control` file by byte offset.
- Receipt ndjson `"v":1` + kebab `kind`: `start`, `env` (names only), `fds` (Unix), `key`, `prompt` (`text`, `hex`, `bare_esc`, `origin`, `submit`), `hook`, `step`.
- Modes built: `--suppress-prompt-submit`, `--local-command-mode`, `--inject-harness-turn`, `--exit-no-eof`, `--report-version`. `--vt100-panic-bytes`, `statusline-echo` and `agents --json` land with their consumers. Exits on `\x03`.
- Root fixture chain: `tests/support/` (`home` → `fake_agent_path` → `stamped_home` interim, no stamps → `booted_wrapper`). Homes live under `target/e2e-home/viola-test-*`, kept per `AGENT_RUN_KEEP_HOMES` / `AGENT_RUN_KEEP_FAILED`. The `viola_e2e::fixtures` copy lands with its first E2E consumer.
- `run --mutants` classifies `chunk.diff` first. A diff with no `.rs` path gives `verdict:"no-rust-delta"` and never runs cargo-mutants. A Rust delta deletes a stale `outcomes.json`, then gives `verdict:"counted"`. `--leg <name>` also writes `artifacts/mutants-verdict-<name>.json` (repo-relative names and outcomes only) and defers survivors to `gate --mutants-legs`, the CI union.
- `run --coverage` (one instrumented `cargo llvm-cov nextest` run as suite `coverage`, then doctest) and `run --fuzz-replay` (Linux only) drive their tools through the `run_with` runner seam, so the harness's own tests use a stand-in runner and never nest `cargo llvm-cov` or `cargo fuzz` inside nextest.
- It must not drift from recorded `viola verify` fixtures (contract suite).

## Exemptions
- `viola-harness` and the fake agent print by design. `viola-e2e` omits `[lints] workspace = true` and carries its own `[lints.clippy]` without `print_stdout` / `print_stderr`; the fake agent is a `[[bin]]` of the root package (lints are per package), so it takes a crate-level `#![allow(clippy::print_stdout, clippy::print_stderr)]`. `tests/contract_lints.rs` asserts that every product member inherits the workspace lints and only `viola-e2e` opts out.

## Session Additions
_This section is owned by `/wrap-session`. setup-project preserves content added here on re-run._
- 2026-09-24: `run --mutants` mutates only lines the chunk diff touches (`--in-diff`), so a mutant CI missed on an earlier chunk is never regenerated by a later chunk that adds only a test — witness that kill another way (the new test's PASS line in that runner's job log), never by a green `mutants` job.
