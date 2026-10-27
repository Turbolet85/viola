# Apply — obs-plan · part 5 of 6

## Verdicts
- fanout-obs-plan--cascade-g4.md 1 · applied · .andromeda/obs-plan-detail/log-coverage.md:10#1
- fanout-obs-plan--cascade-g4.md 2 · applied · .andromeda/obs-plan-detail/log-coverage.md:10#2
- fanout-obs-plan--cascade-g4.md 3 · applied · .andromeda/obs-plan-detail/log-coverage.md:26+#1
- fanout-obs-plan--cascade-g4.md 4 · applied · .andromeda/obs-plan-detail/log-coverage.md:31#1
- fanout-obs-plan--cascade-g4.md 5 · applied · .andromeda/obs-plan-detail/log-coverage.md:32#1
- fanout-obs-plan--cascade-g4.md 6 · applied · .andromeda/obs-plan-detail/log-coverage.md:35#1
- fanout-obs-plan--cascade-g4.md 7 · applied · .andromeda/obs-plan-detail/log-coverage.md:36+#1
- fanout-obs-plan--cascade-g4.md 8 · folded into cascade-compare-obs-plan.md 1
- fanout-obs-plan--cascade-g4.md 9 · applied · .andromeda/obs-plan-detail/error-capture-reporting.md:15#1
- fanout-obs-plan--cascade-g4.md 10 · applied · .andromeda/obs-plan-detail/error-capture-reporting.md:15+#1
- fanout-obs-plan--cascade-g4.md 11 · applied · .andromeda/obs-plan-detail/error-capture-reporting.md:23#1
- fanout-obs-plan--cascade-new-error-capture-reporting.md 1 · applied · .andromeda/obs-plan-detail/error-capture-reporting.md:31+#1
- cascade-compare-obs-plan.md 1 · applied · .andromeda/obs-plan-detail/error-capture-reporting.md:8+#1
- fanout-obs-plan--cascade-g4.md 12 · applied · .andromeda/obs-plan-detail/slo-invariants-telemetry-budgets.md:11#1
- fanout-obs-plan--cascade-g4.md 13 · applied · .andromeda/obs-plan-detail/slo-invariants-telemetry-budgets.md:13#1
- fanout-obs-plan--cascade-g4.md 14 · applied · .andromeda/obs-plan-detail/slo-invariants-telemetry-budgets.md:14#1
- fanout-obs-plan--cascade-g4.md 15 · applied · .andromeda/obs-plan-detail/slo-invariants-telemetry-budgets.md:17+#1
- cascade-compare-obs-plan.md 6 · applied · .andromeda/obs-plan-detail/slo-invariants-telemetry-budgets.md:28#1
- cascade-compare-obs-plan.md 7 · applied · .andromeda/obs-plan-detail/slo-invariants-telemetry-budgets.md:30#1
- fanout-obs-plan--cascade-g4.md 16 · applied · .andromeda/obs-plan-detail/obs-anti-patterns.md:8#1, .andromeda/obs-plan-detail/obs-anti-patterns.md:8#2

## Edits
- line: .andromeda/obs-plan-detail/log-coverage.md:10
  old: «present only on a degrade: `hash-mismatch|unreadable|load-failed|not-built`»
  new: «present only on a degrade: `hash-mismatch|unreadable|load-failed|not-built`, or `strict-modes-failed`»
- line: .andromeda/obs-plan-detail/log-coverage.md:10
  old: «; D-36) |»
  new: «, the last when the write-rule check `pin_companions` makes before its held open refuses `bin/<key>/conpty/` or a companion in it, the child then on the inbox ConPTY with its exit unchanged and nothing written; D-36) |»
- after: .andromeda/obs-plan-detail/log-coverage.md:26
  anchor: «- `process-exit` exit 1: `already-live` (a `live` or a `stale` holder of the name, from `viola run` or from `viola revive`, which prints `run`'s own collision pair, or under `--json` the `{"v":1,"error":"already-live","detail":null}` document — revive's preflight refusals under `--json` are the same-shaped document with their code, the logged line unchanged (chunk 2026-10-17-cli-machine-contract); a `gone` holder is taken over with no exit-1 line), `squatted-name`, `pinned-hash-mismatch`, `batch-script-child`, `internal-error`, and from `viola revive`'s start-arm preflight `no-session` (no logged session, or an `--id` the log does not hold), `session-live` (one `claude agents --json` read lists the logged session id live in another `claude`; skipped under `--fork`; a failed, oversize or malformed read passes, fail open; chunk 2026-10-16-unwrapped-sessions-on-the-board) and `cwd-missing` (the snapshot records no directory, or one that is not an existing directory). The preflight reads the holder, then the session, then `session-live`, then the directory, and takes no later reading once one refuses; each refusal is one `process-exit{subject:"self", exit_code:1, detail}` in `run-<name>.ndjson` with no child started. `strict-modes-failed` is no longer one of them (chunk 2026-10-10-budget-governor): the start arm's instance check (`check_instance`) runs before the wrapper log opens and outside the preflight, so its refusal is the stderr `unable:` / `hint:` pair (or under `--json` the `strict-modes-failed` document, chunk 2026-10-17-cli-machine-contract) and exit 1 with no log opened, no `process-start`, no `process-exit` and no child.»
  add: « `viola run` and `viola revive`'s start arm also run the home-tree check before the wrapper log opens (`strict::check_home_tree` through `checked_exe` and `refuse_tree`: the home to the read rule, `check_instance`'s set, then `ledger/`, `bin/`, `plugin/`, `bin/<key>/`, the pinned copy, `plugin/<key>/`, its `hooks/` and `.claude-plugin/`, each that exists); its refusal is the home pair (a home-level refusal) or the instance pair (an instance-level one) on stderr and exit 1, with no log opened, no line and no child (revive under `--json` prints the `strict-modes-failed` document), so a refused home tree carries no `process-exit` detail either.»
- line: .andromeda/obs-plan-detail/log-coverage.md:31
  old: «four of the seven).»
  new: «four of the seven). The event and dialog paths run the instance check too (`canonical`, then `trusted`) before `read_diagnostics_level` and obs init, so a refused or absent instance, a `VIOLA_DIR` naming no existing directory included, writes no `hook-decision` and no other line either; no hook arm writes `strict-modes-failed`.»
- line: .andromeda/obs-plan-detail/log-coverage.md:32
  old: «decision_emitted:false, detail:"server-verify-failed"}`.»
  new: «decision_emitted:false, detail:"server-verify-failed"}`. `channel-unreachable` is also `connect`'s code when `read_snapshot_or_replay` returns a replayed or absent snapshot (a replay carries no `endpoint`, `pid` or `started_at`): `NotSent::Unreachable`, the hook fails open, and SessionEnd takes its direct append.»
- line: .andromeda/obs-plan-detail/log-coverage.md:35
  old: «and the nine hook events run no instance check;»
  new: «and neither does the refused instance check of the hook event and dialog paths, made before obs init;»
- after: .andromeda/obs-plan-detail/log-coverage.md:36
  anchor: « As landed its call site is `viola_state::board::read` (`viola list`, chunk 2026-10-11-the-board-viola-list): after its loop, one `parse-rejected{parser:"state-entry", detail:"symlink-ignored"|"malformed", count}` per nonzero count of `board::scan` (links ignored; a non-directory or a name that is no `ViolaName`), naming no entry, into `cli-<name>.ndjson` under a `VIOLA_NAME` for `viola list`, and into `mcp.ndjson` for `viola mcp`'s `list` tool (through `list::board_document`, chunk 2026-10-17-mcp-server-for-drivers); `board::scan` itself counts and logs nothing.»
  add: « The other refused strict-modes checks write no line either — no `process-start`, no `process-exit`, no `strict-modes-failed`: `send`, `wait`, `last`, `answer`, `pause` and `release` through `client::refuse_untrusted` (exit 21, the instance pair, or under `--json` the `instance-unreachable` document); `viola mcp`'s frame tools (`instance-unreachable`, `detail: null`); `viola verify`'s home-tree check (exit 1, the home pair); and the home-tree check of `viola run` and `viola revive`'s start arm (exit 1, the home or the instance pair).»
- after: .andromeda/obs-plan-detail/error-capture-reporting.md:8
  anchor: «- When an instance resolves, it also writes `{"event":"panic", ..., "panic_payload":"<escaped>", "backtrace":[...frames]}` as one line to `detail-<process>.ndjson`. The backtrace is raw, never-symbolised frames from `src/panic_frames.rs` — `RtlCaptureStackBackTrace` + `GetModuleHandleExW`/`GetModuleFileNameW` on Windows, `libc::backtrace` + `dladdr` on Unix — up to 62 strings `0x<ip> <module path> base=0x<base> +0x<offset>` (or `0x<ip> ?`), enough for an offline resolver against the pinned copy. It is **not** gated by `RUST_BACKTRACE`, because env vars are not a configuration channel. Symbolising in the panic path was retired: `Backtrace::force_capture().to_string()` cost 351.4 ms of a 402.7 ms hook run for 72 frames on the Windows runner and pushed the forced-panic hook to 1.50 s against the 1.0 s spine bound; the raw capture measured 23.7 ms (as measured at chunk 2026-09-28-capability-ledger-and-viola-verify, CI runs 36436266196, 36435153705 and 36448654074).»
  add: «- A role file or a detail file is opened only past `check_diagnostics` (`src/obs.rs`, in `open_role_file` and `write_detail`), which runs `strict::check_diagnostics` on the existing `diagnostics/` folder and on the file before `ensure_private_dir` narrows anything: a symlink is refused, a folder is held to the read rule, a file to the read rule on Unix and the write rule on Windows, and an absent path passes. A refusal is exactly a failed open: the role writer ends at `std::io::sink`, and the detail line is dropped.»
- line: .andromeda/obs-plan-detail/error-capture-reporting.md:15
  old: «only after its instance check has passed; a refused check on the start arm is the stderr pair and exit 1 with no process log opened.»
  new: «only after its instance check and its home-tree check (`check_home_tree`, through `checked_exe`) have passed; a refused check on the start arm, or a refused home tree on `viola run`, is a stderr pair (on a refused home tree the home pair, or the instance pair for an instance-level refusal) and exit 1 with no process log opened.»
- after: .andromeda/obs-plan-detail/error-capture-reporting.md:15
  anchor: «  - **Main-thread catch site (every role):** everything after `set_hook` (clap parse, init and role dispatch) runs inside one `std::panic::catch_unwind` in the root bin's `main`. An `Err` maps to the role's exit below, so no role ends through an uncaught unwind (Rust's default exit 101). The role is taken from argv before clap runs: the first argument that is neither a global flag nor a flag's value (for example `--home <path>`) and that names a role subcommand (`run`, `hook`, `mcp`; `role_of` in `src/main.rs` names `revive` beside `run`, except `revive` followed (after any `--home` pair) by `--list` before a `--`, which is `cli`; a `--list` after `--` is the child's word; any other verb is `cli`). `viola revive`'s start arm logs as process `run` (the `run-<name>.ndjson` role file and `run`'s detail sink), opened by the log setup it shares with `run` only after its instance check has passed; a refused check on the start arm is the stderr pair and exit 1 with no process log opened. `viola revive <name> --list` is `cli`, so its dispatch `Err` or panic reaches the `cli` catch site (`process-exit{detail:"internal-error"}`, exactly `error: internal error`); its refused strict-modes check opens no log and writes no line (the stderr pair, or under `--json` the `strict-modes-failed` document, exit 1), and its `no-session` is `process-exit{exit_code:1, detail:"no-session"}` in the caller's `cli-<VIOLA_NAME>.ndjson` under a name, with the stderr pair or the `no-session` document (chunk 2026-10-17-cli-machine-contract). So `viola --home <dir> hook stop` is still a `hook`, and a pre-init panic in it still exits 0. Reading `argv[1]` alone would misclassify that form as `cli` and exit 1 (D-28).»
  add: «  - **Catch-site reach:** on Unix a home under a regular file is a strict-modes refusal with no line — `viola verify` exits 1 with the home pair, the frame verbs exit 21 with the instance pair — so it never reaches the `cli` catch site; on Windows such a path stats `NotFound`, the check skips it as absent, and the catch-site line `process-exit{detail:"internal-error"}` stands.»
- line: .andromeda/obs-plan-detail/error-capture-reporting.md:23
  old: «writes no line in any log and leaves the directory's mode as found.»
  new: «writes no line in any log and leaves the directory's mode as found. The window also holds the instance check the hook event and dialog paths run (`canonical`, then `trusted`) before `read_diagnostics_level` and obs init, the six frame verbs' `client::refuse_untrusted` before their `cli` role file opens, `viola verify`'s home-tree check before its `cli` role file opens, and the home-tree check of `viola run` and `viola revive`'s start arm before the wrapper log opens; each of their refusals writes no line.»
- after: .andromeda/obs-plan-detail/error-capture-reporting.md:31
  anchor: «<!-- /detail -->»
  add: «»
  add: «<!-- detail: values · errors -->»
  add: «»
  add: «- MCP rejects on a refused instance check: `viola mcp`'s frame tools (`send`, `wait`, `last`, `answer`) run `check_instance` on the home and the instance directory in `frame` (`crates/viola-mcp/src/tools.rs`) before `live_endpoint`; a refusal returns `instance-unreachable` with `detail: null`, the `isError` code of the liveness pre-check, and writes no frame and no line.»
  add: «»
  add: «<!-- /detail -->»
- line: .andromeda/obs-plan-detail/slo-invariants-telemetry-budgets.md:11
  old: «which runs before their `cli` role file opens);»
  new: «which runs before their `cli` role file opens, and the instance check of the hook event and dialog paths, which runs before obs init, and the six frame verbs' `client::refuse_untrusted`, which runs before their `cli` role file opens, and `viola verify`'s home-tree check, which runs before its `cli` role file opens, and the home-tree check of `viola run` and `viola revive`'s start arm, which runs before the wrapper log opens);»
- line: .andromeda/obs-plan-detail/slo-invariants-telemetry-budgets.md:13
  old: «3. `hook` without `VIOLA_NAME` (D-16);»
  new: «3. `hook` without `VIOLA_NAME` (D-16), or whose `VIOLA_DIR` names an instance directory that does not exist: it canonicalises to no session and runs no obs init;»
- line: .andromeda/obs-plan-detail/slo-invariants-telemetry-budgets.md:14
  old: «4. a failed step-4 role-file open (§3 init step 4).»
  new: «4. a failed step-4 role-file open (§3 init step 4), a refusal of `check_diagnostics` included (§7: a symlinked or widened `diagnostics/` folder or role file).»
- after: .andromeda/obs-plan-detail/slo-invariants-telemetry-budgets.md:17
  anchor: «    7. a refused instance check on either `viola revive` arm (start or `--list`) or on `viola hook statusline` (chunk 2026-10-10-budget-governor), or a refused `viola list` scan of the home, `instances/` or an instance (chunk 2026-10-11-the-board-viola-list, the operator's answer at its planning), or `viola mcp`'s refused check of its home or `diagnostics/` through `viola_state::strict::check_path` (chunk 2026-10-17-mcp-server-for-drivers, its plan's lean, approved by the operator at P5), or `viola plugin install`'s refused check of its home or a `marketplace/` tree dir through `check_path` (chunk 2026-10-17-optional-plugin-install), or a refused `check_instance` of the driven instance on `viola link` / `viola unlink`: it opens no role file and writes no line (revive: the stderr pair, or under `--json` the `strict-modes-failed` document, and exit 1; statusline: exit 0 and nothing printed; list and plugin install: exit 21, the `--json` error document or the stderr pair; link and unlink: exit 21, the fixed instance-state pair or under `--json` the `instance-unreachable` document; mcp: exit 1 with nothing on stdout or stderr), so a panic there writes none, by §7's rule; no forced-panic case reads it. That a refused check writes no line on `viola hook statusline` and on `viola list` is the founder's own answer, 2026-10-13, relayed verbatim by the Viola overseer (the operator's answer at those chunks' planning, superseded 2026-10-13).»
  add: «    8. a refused strict-modes check on the hook event and dialog paths (exit 0, nothing printed), on `send`, `wait`, `last`, `answer`, `pause` or `release` (exit 21, the instance pair or under `--json` the `instance-unreachable` document), on `viola verify`'s home tree (exit 1, the home pair) or on the home tree of `viola run` or `viola revive`'s start arm (exit 1, the home or the instance pair): it opens no role file and writes no line, so a panic there writes none, by §7's rule.»
- line: .andromeda/obs-plan-detail/slo-invariants-telemetry-budgets.md:28
  old: «`hook-decision{detail:"server-verify-failed"}`, exit 0) |»
  new: «`hook-decision{detail:"server-verify-failed"}`, exit 0); the timed path also holds the hook's instance strict-modes check (`trusted`, before obs init), and that one `snapshot.json` read is `read_snapshot_or_replay` (a replayed or absent snapshot fails open as `channel-unreachable`), inside the same gate (max 2.4–3.3 ms over the five hook rows, as measured at chunk 2026-10-26-home-and-code-bearing-file-integrity's `run --perf`) |»
- line: .andromeda/obs-plan-detail/slo-invariants-telemetry-budgets.md:30
  old: «and the gate passes vacuously.»
  new: «and the gate passes vacuously; so it does when `VIOLA_DIR` names no existing instance directory or the instance fails the strict-modes check (the hook then opens no file and makes no channel call), so the timed instance must pass that check. Logger init's one `open` follows the instance check's and `check_diagnostics`' metadata reads.»
- line: .andromeda/obs-plan-detail/obs-anti-patterns.md:8
  old: «and `link`'s / `unlink`'s instance-check refusal»
  new: «and the instance-check refusal of `link` / `unlink` and of `send` / `wait` / `last` / `answer` / `pause` / `release`»
- line: .andromeda/obs-plan-detail/obs-anti-patterns.md:8
  old: «callers `run` 5 · `verify` 5 · `list` 2 · `revive` 1 · `plugin install` 1 · `link` 1, the one `link_call` site both verbs share»
  new: «callers `run` 7 · `verify` 6 · `list` 2 · `revive` 1 · `plugin install` 1 · `client` 1, the one `refuse_untrusted` site `link` / `unlink` and the six frame verbs share»

## Patterns
[[pattern]]
id = "obs5-01"
regex = 'hash-mismatch[|`, ]+unreadable[|`, ]+load-failed[|`, ]+(or |and )?`?not-built`?(?!, or `strict-modes-failed`)'

[[pattern]]
id = "obs5-02"
fixed = 'the nine hook events run no instance check'
ignore_case = true

[[pattern]]
id = "obs5-03"
fixed = '`run` 5 · `verify` 5'

[[pattern]]
id = "obs5-04"
fixed = 'the one `link_call` site both verbs share'
