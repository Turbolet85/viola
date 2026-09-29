# Codebase Research — 2026-09-29-verify-stamped-test-homes-and-harness

## Scope
- **Depth:** deep · **Reads:** 19 · **Globs/Greps:** 24 · **Graph queries:** 5 (rust plane)
- **Harness rules consulted:** `.claude/rules/verification-harness.md` — read in full, 6 Session Additions. The ones that apply: an unbuilt selector pinned as usage is a plan defect (2026-09-29); never pipe `boot` (2026-09-25); a `test(=name)` filter needs the full path (2026-09-27). `.claude/rules/testing.md` 19 Session Additions: wait on the exact line asserted; force a timing window with a test-only hold, never sample the race (2026-09-27); a timing red is never fixed by raising a bound (2026-09-28).
- **Platform issues consulted:**
  - Query: named pipe instance lifetime vs process exit (Windows).
  - Microsoft Learn `ExitProcess` (fetched) states the order: all handles are closed (step 6) BEFORE the exit status leaves `STILL_ACTIVE` (step 7), and the process object is signalled after that (step 8).
  - `GetExitCodeProcess` (fetched): the call "returns immediately"; `STILL_ACTIVE` until the process terminates.
  - Named Pipe Operations (search result): "An instance of a named pipe is always deleted when the last handle to the instance of the named pipe is closed."
  - No public tracker entry matched the runner signature. The documented order argues AGAINST "exit visible before the pipe closed" (see §Item 8).

## Files inspected
- `tests/support/home.rs` (full) — the interim `StampedHome { stamped: false }` (152-168); `Wrapper::boot` (189-212) passes neither `--cli-version` nor `--fixtures`; `stop_keep` (272-280) writes Ctrl-C and returns on `OuterPty::wait_exit`.
- `tests/support/outer_pty.rs` (17-134) — `wait_exit` polls `try_wait`, which is portable-pty's.
- `D:/dev/rust/cargo/registry/.../portable-pty-0.8.1/src/win/mod.rs` (20-107) — Windows `try_wait` is `GetExitCodeProcess` only (26-39). Only `wait` blocks on `WaitForSingleObject` (92-107).
- `tests/support/verify.rs` (120-145) — `verify(home, fixtures, version, before, after, env)` already runs `viola --home <h> verify -- <fake> --cli-version <v> --fixtures <f>`. This is the stamping call the seam can reuse.
- `tests/cli_verify.rs` (1-112, 300-320) — literal oracles `STEPS_PASS` / `ROW_IDS` (six rows) and `stamped 2.1.0  6 pass  0 fail` over a test-written set. With `VIOLA_NAME`, verify writes only the `self` pair (305-320).
- `tests/hook_fail_open.rs` (1-260) — the red case builds `StampedHome { stamped: false }` literally (192-196). `Env::Stopped` does `Wrapper::boot(…).stop_keep()` (199), then asserts events `[hook-invoked, hook-decision]` at 238.
- `tests/hook_events.rs` (355-395) — `hook_session_end_appends_directly_only_while_the_log_is_free` stops the same way (362). Its premise is also "the wrapper is gone".
- `crates/viola-e2e/src/harness/boot.rs` (full) — steps 1-3 and 5 only. There is no verify step, no `--unstamped`, no `events.ndjson` check in `readiness` (271-308), and `DEFAULT_CLI_VERSION = "2.1.0"` (21). Failure docs come from `failure(reason, instance, exit_code, missing)` (65-78).
- `crates/viola-e2e/src/harness/supervise.rs` (40-80) — `spawn_wrapper` passes `--cli-version <spec>` (58) and never `--fixtures`.
- `crates/viola-e2e/src/bin/viola-harness.rs` (full) — clap `Boot { session, instances, cli_version }` (30-37), `RunArgs` (74-97). An unknown flag is `Outcome::usage(cmd, "arguments")` (193-205).
- `crates/viola-e2e/tests/cli.rs` (50-90) — `run --e2e` and `boot --ui` are pinned as usage (58-73). Neither is built here.
- `crates/viola-e2e/tests/harness_lifecycle.rs` (grep) — five tests call `boot(&opts)` for real (85, 141, 171, 199, 212), so they will run step 4 once it lands.
- `src/bin/viola-fake-agent.rs` (14-493) — `DEFAULT_CLI_VERSION = "2.1.0"` (18). `--fixtures` joins `<cli_version>` (213-219). With no fixture, `fire` returns `no-fixture` and runs no hook (220-222). Print mode fires the four spine events (477-484).
- `src/cmd/verify.rs` (full) — spawns `run_bounded` twice (112 version, 127 probe) with no `process-start`/`process-exit`. The self pair only under an instance (50-61). The summary goes through `human::result` (142-145).
- `src/run/version_gate.rs` (1-205) — `run_bounded` (39-73). `version_gate` logs the `version-probe` pair around its own call (114-134). `stamps_verdict` matches `StampError::Malformed` (171).
- `crates/viola-agent-claude/src/ledger.rs` (215-297) — `StampError { Malformed }` (224-229); `verified` → `Result<bool, StampError>` (287-297).
- `crates/viola-agent-claude/src/hook.rs` (55-84) — `AgentError { Malformed }` (62-67), "the hook payload is not one JSON object".
- `crates/viola-channel/src/endpoint.rs` (1-90) — `endpoint_name` = FNV-1a 64 over name + `\0` + home, first 12 hex (29-37). `endpoint_path` → `\\.\pipe\viola-<h12>` (53-69).
- `src/cmd/run.rs` (250-253) — `bind_endpoint` calls `endpoint_path(name, home)` with the instance home.
- `crates/viola-channel/src/client.rs` (60-112) — `notify` logs `channel-request` (101-108) and then writes. It is only reachable on a client that has already connected.
- `crates/viola-channel/src/server.rs` / `server/win.rs` (grep) — pipe handles are non-inheritable on both ends (server.rs:79; win.rs:132-134 has a test).
- `schemas/diag-line.v1.json` (127) — `subject` enum `self|claude-child|version-probe|agents-probe|statusline-shell`. There is no code-side list (grep over src/crates/tests/scripts).
- `.andromeda/runs/2026-09-29T06-43-45-phase/ci/diag-windows-2025/viola-test-B3yvcs/` — the red's kept home, from ci#36532038635's `diag-windows-2025` artifact (see §Item 8).

## Graph impact
- **run_bounded** — 3 production call sites: `measure()` @ `src/cmd/verify.rs:112`, `:127`; `version_gate()` @ `src/run/version_gate.rs:116`, plus 2 tests (217, 237). Logging inside `run_bounded` would double `run`'s existing pair. Logging at the verify call sites leaves `run` untouched.
- **StampError** — refs only in `crates/viola-agent-claude/src/ledger.rs` (287, 288, 293, 909, 916) and `src/run/version_gate.rs` (16, 171). The fold touches two files.
- **verified** (ledger.rs) — one production caller, `stamps_verdict()` @ `src/run/version_gate.rs:169`, plus the ledger unit tests.
- **StampedHome / stamped_home / booted_wrapper** — graph refs by file: `channel_endpoint.rs` 4, `cli_fake_agent.rs` 5, `cli_instance_state.rs` 10, `cli_version_gate.rs` 2, `hook_events.rs` 3, `hook_fail_open.rs` 2, `support/home.rs` 6.
  - The graph misses `tui_channel_fds.rs`, which takes `booted_wrapper` as an rstest fixture argument (`:16`, `:19`). grep finds it, so it is an index gap through the macro and rides the list.
  - Literal `StampedHome { … stamped: false }` constructors: `cli_fake_agent.rs:630`, `cli_version_gate.rs:61`, `hook_events.rs:26`, `hook_fail_open.rs:192`.
  - Re-derived: the refs query in `tree-query-{marker}.json`, plus `grep -n 'StampedHome\|stamped_home\|booted_wrapper\|Wrapper::boot\|stamped:' tests/*.rs`.

## Patterns detected
- **Verify against the fake agent** (`tests/support/verify.rs:128-144`): the `-- <fake> --cli-version <v> --fixtures <dir>` form. The stamped seam reuses it with `workspace_path("fixtures/claude")` and `2.1.283`.
- **Per-test fixtures** (`tests/cli_instance_state.rs:364-375`): a test writes its own set and passes `--fixtures` as `Wrapper::boot` extra args. It relies on the fake agent's DEFAULT version finding `<fixtures>/2.1.0/`.
- **Harness failure document** (`boot.rs:65-78`): `{"v":1,"cmd":"boot","ok":false,"reason","instance","exit_code","missing"}`. `verify-failed` takes the same shape.
- **Staged readiness** (`boot.rs:271-308`): the role file, then the snapshot and endpoint, then the heartbeat. The `events.ndjson` lines 1-3 check appends a stage.
- **Harness `endpoint_gone`** (verification-harness.md §cleanup): the endpoint is gone when a client connect returns NotFound. That is the condition "stopped" should mean.

## Conventions to follow
- **Literal oracles** (`tests/cli_verify.rs:22-37`): row ids, counts and versions are test literals, never imported from the product.
- **Exit-aware bounded waits** (`tests/support/home.rs:219-244`): `Watch` plus `WITHIN` (7 s), failing at once on `try_wait()`.
- **Fixed-message thiserror** (`hook.rs:62-67`, `ledger.rs:224-229`): the fold keeps a fixed `Display`.
- **Spawn pair shape** (`version_gate.rs:114-134`): `process-start{subject}` before, and `process-exit{subject, child_exit_status, duration_ms}` after.

## Item 8 — the folded red, closed against ci#36532038635
- **Witness.** The job log (`ci/job-109287650444.log:514-534`) shows the red. The kept home `viola-test-B3yvcs` is the only home whose events match the failing case's window (FAIL logged 06:39:42.314, 0.382 s):
  - `run-builder.ndjson`: `process-exit{subject:"self", exit_code:0}` at **06:39:42.258Z**.
  - `hook-builder.ndjson`: `hook-invoked` at **06:39:42.283Z**, `channel-request` (`method:"hook.event"`, conn `hook-4540-…`) at .284, then `hook-decision` with NO `detail`. `deliver` returned true: the notification was written.
  - The fake agent fired no hook of its own (no `--fixtures`, so `no-fixture`). The hook lines are the test's `run_hook`.
- **VERIFIED:** the red stands at ci#36532038635 / `31dd995` / `test (windows-2025)`.
- **FALSIFIED — the overseer's hypothesis** ("a pipe-name collision with a concurrent test"). The endpoint name carries the home (`endpoint.rs:29-37`, called with the instance home at `run.rs:253`). Two unique `viola-test-*` homes share a pipe only on a 48-bit hash-prefix collision, and the witness shows the hook reached its OWN home's endpoint.
- **Established:** 25 ms after the wrapper's last line, and after `stop_keep` had seen its exit, the stopped wrapper's own `\\.\pipe\viola-<h12>` still accepted a connect and a write.
- **Not established:** why the endpoint outlived the observed exit.
  - (a) portable-pty's `try_wait` is `GetExitCodeProcess` alone. Microsoft's documented `ExitProcess` order has handles closed before the status is set, which argues against it.
  - (b) another handle to the pipe instance. Our handles are non-inheritable.
  - Both are unmeasured candidates.
- **Test-side meaning:** `stop_keep`'s "stopped" (an exit code) is not "endpoint gone" (a connect reads NotFound). `hook_events.rs:362` carries the same premise.
- **Product side:** no boundary change. The hook reached the same user's, same home's wrapper. The one product consequence is narrow: a `hook.event` written into an exiting wrapper reads "delivered", so `SessionEnd`'s direct-append fallback (`hook.rs:148-150`) is skipped. It is recorded, not in this chunk's scope.

## New files to create
- `tests/contract_ledger_probes.rs` — every committed `fixtures/claude/<ver>/` set through `viola verify` against the fake agent; literal row ids exactly once each; `<n>` equals the literal row count.

## Files to modify
- `tests/support/home.rs` — the stamped seam, the explicitly-unstamped form, `Wrapper::boot` version/fixtures, a stop that waits for the endpoint to be gone
- `tests/support/verify.rs` — reused or extended for the committed-set stamping call
- `tests/support/fake.rs` — the root chain's `RECORDED_CLI_VERSION` constant (P4 step 1)
- `tests/support/mod.rs` — module wiring if a helper moves
- `tests/hook_fail_open.rs` — case_08 on the corrected stop and the item-8 witness
- `tests/hook_events.rs` — the same stop premise (362); its fixtures are written at the old default version (38)
- `tests/cli_version_gate.rs` — unstamped constructor (61); default-version literals (76-115)
- `tests/cli_fake_agent.rs` — `stamped_home` asserts `!stamped` (593); the literal constructor (630); default-version literal (199)
- `tests/cli_instance_state.rs` — fixtures written at the old default (368); `stamped_home` consumers
- `tests/channel_endpoint.rs` — `stamped_home` / `booted_wrapper` consumer
- `tests/tui_channel_fds.rs` — `booted_wrapper` consumer
- `tests/run_cli.rs` — the default `--version` answer literal (538)
- `tests/tui_passthrough.rs` — default-version literals (193, 249)
- `tests/cli_verify.rs` — a verify-with-instance case for the new spawn pairs
- `tests/contract_diag_schema.rs` — accepts the new probe `subject`, still rejects an unknown one
- `schemas/diag-line.v1.json` — the new `subject` enum value
- `src/bin/viola-fake-agent.rs` — `DEFAULT_CLI_VERSION` → the recorded version and its unit test
- `src/cmd/verify.rs` — `version-probe` and probe-child spawn pairs
- `src/run/version_gate.rs` — the `StampError` import and match move to `AgentError`
- `crates/viola-agent-claude/src/ledger.rs` — `StampError` removed; `verified` returns `AgentError`
- `crates/viola-agent-claude/src/hook.rs` — `AgentError` gains the stamps variant (or moves to lib.rs)
- `crates/viola-agent-claude/src/lib.rs` — re-export if `AgentError` moves
- `crates/viola-e2e/src/harness/boot.rs` — step 4, `--unstamped`, `verify-failed`, readiness line 3, `DEFAULT_CLI_VERSION`
- `crates/viola-e2e/src/harness/supervise.rs` — `--fixtures <root>/fixtures/claude`
- `crates/viola-e2e/src/harness/run.rs` — the `--local-live` selection and its `live-in-ci` refusal
- `crates/viola-e2e/src/bin/viola-harness.rs` — `--unstamped` and `--local-live` flags
- `crates/viola-e2e/tests/cli.rs` — usage/refusal rows for the new flags
- `crates/viola-e2e/tests/harness_lifecycle.rs` — boots now stamp; unstamped and verify-failed cases
- `.claude/docs/gotchas.md` — the Windows dying-pipe window: a hook fired within ms of wrapper exit can still reach the exiting wrapper's pipe; `viola hook` still exits 0 (P4 overseer direction)
- `.claude/docs/services/viola.md` — the same note beside the hook's fail-open paths

## Open questions
- Should `Wrapper::boot` pass `--cli-version` and `--fixtures` for every consumer (the fake agent then fires SessionStart and every hook in every booted wrapper, and each consumer's event counts move), or only for the stamped form, keeping consumer behaviour as is? → blocks: plan-decision
- Item 8's fix and witness. The fix candidate is test-side: `stop_keep` returns only once the endpoint is gone (connect NotFound, the harness `endpoint_gone` rule). A deterministic forced-window witness needs the endpoint held reachable after the reported exit, which no existing seam gives. Or the cause is first measured on the runner with temporary measurement pushes (the H2 precedent). → blocks: plan-decision
- `2026-09-29-h2-conpty-resize-probe`'s arch wording ("owned by the real-CLI verify entry") could read as naming this entry's `run --local-live`. The handoff pins H2 to "First live test and self-drive". The plan states that `--local-live` does not claim the H2 measurement. → blocks: plan-decision
