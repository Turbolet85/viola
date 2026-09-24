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
- Agent surface: exactly `boot · run · status · cleanup · logs`. `supervise`, `ui-restart` and `gate` are internal subcommands, forwarded unchanged. A 6th agent command needs a test-plan amendment.
- Every command prints exactly one JSON document on stdout starting `{"v":1,"cmd":"<command>","ok":<bool>,…}`; exit 0 ok · 1 failure · 2 usage. A human-only message without the JSON document is forbidden.

## boot
- Steps in order: `cargo build --workspace --features fake-agent` → a not-yet-existing home under `target/e2e-home/viola-session-*/home` → copy the fake agent to `target/agent-run/<session>/bin/claude[.exe]` on a harness-scoped `PATH` (per child via `Command::env`) → `viola verify --home` against it unless `--unstamped` → detached `viola-harness supervise` owning one portable-pty per instance → optional `viola ui` → token exchange (303 + `HttpOnly; SameSite=Strict`, cookie stored 0600, never printed).
- Readiness is a bounded 100 ms file-state probe: snapshot has `endpoint/pid/started_at/child_pid`, heartbeat < 5 s, `events.ndjson` lines 1–3 = `wheel{cause:start}` → `budget-gate` → `session-start{source:hook}`; UI `/health` ok and `/ready` `"ready"`. Failure reasons are the closed list in test-plan §3.
- After UI readiness, byte-compare every embedded `/assets/*` with the repo file (`stale-embedded-assets`).
- No nested `cargo run` inside nextest: tests use the `harness_session` fixture or the rstest chain `home → fake_agent_path → stamped_home → booted_wrapper`.

## status / cleanup / logs
- `status`: `viola list --json` + `/ready` + cookie-gated `/api/info` and `/api/sessions`; `api_sessions_equal_list` deep-equals `.ok.items`; exit 0 only for `state:"ready"`.
- `cleanup`: `stop.request` → Ctrl-C into each outer PTY → `child.wait()` 10 s → `kill()`; verify endpoint gone (CLI exit 21, socket path absent), port free, `ui/<port>.url` removed. Idempotent (no session → `ok:true, cleaned:[]`). Kill targets verified by pid + start time. `AGENT_RUN_KEEP_HOMES=1` (CI) keeps the home (`home_removed:"kept"`).
- `logs`: merge `events.ndjson` (`src:"events"`, byte `offset`), home `diagnostics/*.ndjson` and `instances/*/diagnostics/detail-*.ndjson` (with `instance`); torn lines emitted as `{"torn":true}`, never dropped; filters `--instance --kind --process --after`.

## Fake agent (`viola-fake-agent`, feature `fake-agent`)
- Answers `--version` like the real CLI; reads hook commands from the `plugin/` and `settings.json` files `run` wrote (never PATH) and runs the real pinned `viola hook`.
- Scripted turns gated on the `--control` file by byte offset; receipts record prompts (text, hex, `bare_esc`), keystrokes, env names, Unix fds and hook invocations. Modes: `--suppress-prompt-submit`, `--local-command-mode`, `--inject-harness-turn`, `--exit-no-eof`, `--vt100-panic-bytes`, `--report-version`, `statusline-echo`, `agents --json`. Exits on `\x03`.
- It must not drift from recorded `viola verify` fixtures (contract suite).

## Exemptions
- `viola-harness` and the fake agent print by design: they omit `[lints] workspace = true` and carry their own `[lints.clippy]` without `print_stdout` / `print_stderr`.

## Session Additions
_This section is owned by `/wrap-session`. setup-project preserves content added here on re-run._
