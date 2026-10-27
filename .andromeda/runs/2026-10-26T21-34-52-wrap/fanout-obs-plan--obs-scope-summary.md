proposals:
  - detector: D-retired
    severity: warning
    file: .andromeda/obs-plan-detail/obs-scope-summary.md
    near: 370
    section: "## 1. Obs Scope Summary"
    change: "Line 370's second sentence reads: As landed a strict-modes failure is not logged: every hook arm but the two `--capture` arms runs the instance check — `viola hook statusline` and, since chunk 2026-10-26-home-and-code-bearing-file-integrity, the event and dialog paths (`canonical` / `trusted`, shared in `src/cmd/hook.rs`, before `read_diagnostics_level` / `viola_obs_init`) — and a refused check, or an instance directory that does not exist, returns exit 0 with no `config.json` read, no role file, no stdin read, no line, no frame and no SessionEnd append (chunk 2026-10-10-budget-governor; the founder's own answer, 2026-10-13, relayed verbatim by the Viola overseer, the operator's answer at its planning superseded 2026-10-13; for the event and dialog paths the founder's own answer, `inputs#I3` Q2 of chunk 2026-10-26-home-and-code-bearing-file-integrity)."
    sidecar: "obs-scope-summary Vector 4 trigger: `viola hook statusline` is no longer the one hook arm with an instance check — the event and dialog paths run it too and a refusal leaves no line (chunk 2026-10-26-home-and-code-bearing-file-integrity, Retired (1) and (6))."
    rationale: "The report's Retired bullet retires the dated gaps (1) `hook.event`'s snapshot read without strict-modes and (6) `hook.dialog`; Symbols `src/cmd/hook.rs` moves `canonical` / `trusted` onto both arms before obs init, a refusal exit 0 with no line; line 370 still names statusline as the one arm."
    basis: viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/report.md:16
  - detector: D-obs-instrumentation
    severity: warning
    file: .andromeda/obs-plan-detail/obs-scope-summary.md
    near: 397
    section: "## 1. Obs Scope Summary"
    change: "Line 397 reads: Refusal telemetry for strict-modes failures, pinned-exe hash mismatches and ignored symlinks. As landed a strict-modes failure has a line in two places: the `ledger-stamps` read in `run`'s version gate, and on Windows x64 a `conpty/` companion its owner/DACL check refuses (`pin_companions`' `strict::check_present`, `PinError::StrictModes`), recorded as `process-start.sideload_fallback` `strict-modes-failed`, the child on the inbox ConPTY, exit unchanged. Every other strict-modes refusal leaves no line, by this named list: `viola revive`'s instance check and its start's home-tree check, and `viola run`'s home-tree check (`check_home_tree`, before `open_wrapper_log`; exit 1); `viola verify`'s home-tree check and `update_stamps`' `StampsError::StrictModes` (exit 1, the home pair); `viola hook`'s instance check on every non-capture arm (exit 0); the instance check (`refuse_untrusted`) of `send`, `wait`, `last`, `answer`, `pause` and `release`, and `viola link` / `viola unlink`'s of the driven instance (each made before any role file opens; exit 21); `viola mcp`'s frame tools (`instance-unreachable`, `detail: null`); the `diagnostics/` opens (`strict::check_diagnostics` in `open_role_file` / `write_detail`: a symlink or a widened folder or file is a failed open, the writer ends at `std::io::sink`, a detail line dropped); the checked read of `budget.json` (`read_budget_stamped`, which the budget gate takes through `read_budget` and `viola list` takes directly, where a refused file is no reading) (chunk 2026-10-10-budget-governor); and `viola list`'s scan of the home, `instances/` and each instance, which refuses the whole board with exit 21 (chunk 2026-10-11-the-board-viola-list). The sites of chunk 2026-10-26-home-and-code-bearing-file-integrity write no line by the founder's own answer (`inputs#I3` Q2)."
    sidecar: "obs-scope-summary Vector 7 trigger: the strict-modes no-line refusals named as a list (the six frame verbs, hook event/dialog, MCP frame tools, verify, run/revive home tree, diagnostics opens join it) and the `conpty/` companion refusal's `sideload_fallback` `strict-modes-failed` added as a second logged site; the stale «Five» dropped."
    rationale: "Report Coverage of new surfaces: each new check is instrumentation n/a, no line (inputs#I3 Q2), except the `conpty/` leg, instrumented by `process-start.sideload_fallback` `strict-modes-failed` (Schema / config, diag-line.v1.json line 70); Count 20 lists the new no-line sites; line 397 still says a strict-modes failure has a line only on the ledger-stamps read and counts five no-line refusals."
    basis: viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/report.md:90
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/obs-scope-summary.md
    near: 487
    section: "## 1. Obs Scope Summary"
    change: "In line 487, after `viola link` / `viola unlink`'s sentence, add: `send`, `wait`, `last`, `answer`, `pause` and `release` take the same form through `refuse_untrusted` (`src/cmd/client.rs`, shared with `link` / `unlink`): a refused instance check carries no logged `detail` (no line), its output the fixed instance-state `unable` / `hint:` pair on stderr, or under `--json` the `instance-unreachable` document with `detail` null, exit 21 (the founder's own answers, `inputs#I3` Q2 and Q3 of chunk 2026-10-26-home-and-code-bearing-file-integrity). `viola run`'s and a revive start's home-tree strict-mode fail (`check_home_tree`, before the wrapper log) and `viola verify`'s are exit 1 with no logged `detail` (no line): the home pair, or for an instance-level refusal of `run` the instance pair; revive `--json` keeps `strict-modes-failed`."
    sidecar: "obs-scope-summary Founder Direction 4 trigger: the six frame verbs' strict-mode fail (no line, the instance pair or the instance-unreachable document, exit 21) and run / revive start / verify's home-tree refusal (exit 1, no line) added."
    rationale: "Report Symbols `src/cmd/client.rs` (`refuse_untrusted` called first by the six verbs, link_call moved onto it), `src/cmd/run.rs` (`checked_exe` / `refuse_tree` before `open_wrapper_log`), `src/cmd/verify.rs` (`refuse_home`); Count 8 and 9."
    basis: viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/report.md:15
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/obs-scope-summary.md
    near: 300
    section: "## 1. Obs Scope Summary"
    change: "In line 300, after «A revive whose instance check refuses writes no line: the check precedes the log (chunk 2026-10-10-budget-governor).» add: A `run` or a revive start whose home-tree check (`check_home_tree`: the home, the instance set, `ledger/`, `bin/`, `plugin/`, `bin/<key>/`, the pinned copy, `plugin/<key>/`) refuses writes no line either: exit 1, no log opened, no child (chunk 2026-10-26-home-and-code-bearing-file-integrity)."
    sidecar: "obs-scope-summary run start path: the home-tree strict-modes refusal of `run` and a revive start added to the no-line exits."
    rationale: "Report Symbols `src/cmd/run.rs`: `run` (166-172) and revive's start call `checked_exe` / `refuse_tree` before `open_wrapper_log`, exit 1, no log, no line, no child; Count 20 names obs-scope-summary.md:300 as an exemption row to extend."
    basis: viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/report.md:18
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/obs-scope-summary.md
    near: 299
    section: "## 1. Obs Scope Summary"
    change: "Line 299's must-trace spans read `run.start` › `state.snapshot_recover` › `run.collision_check` › `run.pin_copy` › …: `collision_check` takes the caller's one `read_snapshot_or_replay` read (`Option<&Recovered>`), so the replay's span sits under `run.start` before `run.collision_check`; over a replayed snapshot the collision check reads liveness from the beat alone (chunk 2026-10-26-home-and-code-bearing-file-integrity)."
    sidecar: "obs-scope-summary run start path: `state.snapshot_recover` placed under `run.start` before `run.collision_check`."
    rationale: "Report Deviation 4 and Count 6: `state.snapshot_recover` now sits under `run.start` before `run.collision_check`; Symbols `src/cmd/run.rs` `collision_check` over `Option<&Recovered>`; the directive's outcome item (b) names this span placement as a behaviour change not yet stated."
    basis: viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/report.md:97
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/obs-scope-summary.md
    near: 326
    section: "## 1. Obs Scope Summary"
    change: "Line 326's must-trace spans read `hook.handle` › `state.snapshot_recover` › `channel.request(hook.dialog)` › …: hook `connect` reads `read_snapshot_or_replay`, so the replay's span sits under `hook.handle` on the hook path (chunk 2026-10-26-home-and-code-bearing-file-integrity)."
    sidecar: "obs-scope-summary dialog path: `state.snapshot_recover` placed under `hook.handle`."
    rationale: "Report Deviation 4 and Count 6: `state.snapshot_recover` sits under `hook.handle` on the hook path; Symbols `src/cmd/hook.rs` `connect` (224-226) reads `read_snapshot_or_replay`."
    basis: viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/report.md:16
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/obs-scope-summary.md
    near: 371
    section: "## 1. Obs Scope Summary"
    change: "After line 371 add: A snapshot `read_snapshot_or_replay` replays (it carries no `endpoint`, `pid` or `started_at`), or an absent one, is the channel-failure fail-open: hook `connect` returns `NotSent::Unreachable` (`channel-unreachable`, SessionEnd's direct append as before), and `viola hook statusline` over a replayed snapshot has no `statusline_command`, so nothing runs or prints; the replay's one `state-recovered` line (E5) is its only record (chunk 2026-10-26-home-and-code-bearing-file-integrity)."
    sidecar: "obs-scope-summary Vector 4 trigger: a replayed or absent snapshot on the hook path is the `channel-unreachable` fail-open; statusline runs nothing over it."
    rationale: "Report Symbols `src/cmd/hook.rs` connect: Replayed / Absent → `NotSent::Unreachable`; `src/cmd/hook/statusline.rs` serve over a replayed snapshot runs nothing; Count 5 (replay product readers 2 → 6)."
    basis: viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/report.md:17
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/obs-scope-summary.md
    near: 190
    section: "## 1. Obs Scope Summary"
    change: "Line 190 reads: Where no instance directory resolves — no `VIOLA_NAME`, a `VIOLA_DIR` naming an instance directory that does not exist (it canonicalises to no session), or an instance the instance check refuses — file logging is impossible or refused. That case gets no log and no stderr, by contract (chunk 2026-10-26-home-and-code-bearing-file-integrity)."
    sidecar: "obs-scope-summary hook surface: a `VIOLA_DIR` naming no existing directory and a refused instance join the no-log case."
    rationale: "Report Deviation 5 and Count 23: a `VIOLA_DIR` naming an instance directory that does not exist canonicalises to no session, so the hook writes nothing; Symbols `src/cmd/hook.rs`: a refused or absent instance returns exit 0 before obs init."
    basis: viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/report.md:98
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/obs-scope-summary.md
    near: 189
    section: "## 1. Obs Scope Summary"
    change: "In line 189, after the logger-init sentence, add: The instance check that precedes logger init and `check_diagnostics`' metadata reads inside the role-file open sit in the timed path; `run --perf` keeps the five hook rows under `SPINE_DEADLINE` with them (max 2.5 / 2.4 / 2.4 / 2.4 / 3.3 ms, chunk 2026-10-26-home-and-code-bearing-file-integrity)."
    sidecar: "obs-scope-summary hook surface: the instance check and the diagnostics check measured inside the hook latency budget."
    rationale: "Report Outcome (obs): every refused check writes no line; `run --perf` keeps the five hook rows under `SPINE_DEADLINE` with the check in the timed path (max 2.5 / 2.4 / 2.4 / 2.4 / 3.3 ms); Symbols `src/obs.rs` check_diagnostics in `open_role_file`."
    basis: viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/report.md:137
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/obs-scope-summary.md
    near: 158
    section: "## 1. Obs Scope Summary"
    change: "In line 158, after `viola link` / `viola unlink`'s sentence, add: `send`, `wait`, `last`, `answer`, `pause` and `release` run the same instance check (`refuse_untrusted`) first, and `viola verify` its home-tree check (`check_home_tree(&home, None, None)`: the home, `ledger/`, `bin/`, `plugin/`) before its `cli` log; a refused check opens none (chunk 2026-10-26-home-and-code-bearing-file-integrity)."
    sidecar: "obs-scope-summary cli surface exporter: the six frame verbs and `verify` open their `cli` log only after their strict-modes check passes."
    rationale: "Report Symbols `src/cmd/client.rs` (`refuse_untrusted` opens nothing under the home and writes no line; called first by the six verbs) and `src/cmd/verify.rs` (check_home_tree before its cli log, refused: no line)."
    basis: viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/report.md:19
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/obs-scope-summary.md
    near: 379
    section: "## 1. Obs Scope Summary"
    change: "After line 379 add: The frame tools (`send` · `wait` · `last` · `answer`) run `check_instance` on the home and the instance before `live_endpoint`, and `live_endpoint` reads a replayed snapshot as no endpoint; either returns `instance-unreachable` with `detail: null`, no frame and no line (chunk 2026-10-26-home-and-code-bearing-file-integrity)."
    sidecar: "obs-scope-summary Vector 5 trigger: the MCP frame tools' strict-modes refusal and a replayed snapshot return `instance-unreachable` with no line."
    rationale: "Report Symbols `viola-mcp` tools.rs `frame` (337-339): check_instance before live_endpoint, refusal `result::unreachable()`, no frame, no line; `viola_state::liveness::live_endpoint`: only `Recovered::Snapshot` proceeds."
    basis: viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/report.md:14
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/obs-scope-summary.md
    near: 292
    section: "## 1. Obs Scope Summary"
    change: "In line 292, after the 0600 / 0700 sentence, add: Before `ensure_private_dir` narrows anything, `open_role_file` and `write_detail` run `strict::check_diagnostics` on the existing `diagnostics/` folder and the role or detail file: a symlink is refused, a folder takes the read rule, a file the read rule on Unix (no group or other bit) and the write rule on Windows, an absent path passes; a refusal is a failed open, so the writer ends at `std::io::sink` and a detail line is dropped (chunk 2026-10-26-home-and-code-bearing-file-integrity)."
    sidecar: "obs-scope-summary log file location: the diagnostics opens' strict-modes check (symlink, read rule) before any narrowing added."
    rationale: "Report Symbols `src/obs.rs` new private `check_diagnostics(dir, file)` (69-76) and `viola_state::strict::check_diagnostics` (127-145)."
    basis: viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/report.md:20
note: "D-obs-stack and D-obs-pii found no drift (no dependency change; no new line touches user data). Lines 370, 397 and 487 hold strict-modes no-line lists the report's pattern rows do not name (Count 20 names only obs-scope-summary.md:300); their edits come from reading the file."
