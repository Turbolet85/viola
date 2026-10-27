proposals:
  - detector: D-cascade
    severity: warning
    file: .andromeda/obs-plan-detail/log-coverage.md
    near: 10
    section: '## 6. Log Coverage'
    change: 'In the `process-start` row, the `sideload_fallback` word list reads `hash-mismatch|strict-modes-failed|unreadable|load-failed|not-built` (five words, in the order of `schemas/diag-line.v1.json`); the rest of the cell is unchanged.'
    sidecar: 'log-coverage#1.1: `sideload_fallback` 4 → 5 words, +`strict-modes-failed` (chunk 2026-10-26-home-and-code-bearing-file-integrity).'
    rationale: 'Report Changes, Schema / config: `schemas/diag-line.v1.json` line 70, the `process-start.sideload_fallback` enum gains `strict-modes-failed`; Counts 7 names log-coverage.md:10 stale where the word list is enumerated.'
    basis: '.andromeda/obs-plan-detail/log-coverage.md:10'
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/log-coverage.md
    near: 10
    section: '## 6. Log Coverage'
    change: 'In the same `process-start` cell, after `D-36`, add: `strict-modes-failed` is the degrade when `pin_companions`''s `strict::check_present` (the owner/DACL write rule, run before the held open) refuses `bin/<key>/conpty/` or a companion that exists (`PinError::StrictModes`): the child runs on the inbox ConPTY, the exit unchanged, nothing written (chunk 2026-10-26-home-and-code-bearing-file-integrity).'
    sidecar: 'log-coverage#1.1: the cause of `sideload_fallback` `strict-modes-failed` added, a refused `conpty/` companion (F11).'
    rationale: 'Report Symbols, `viola_state::pin` (Windows `pin_companions` runs `check_present`, a refusal is `PinError::StrictModes`, nothing written) and `src/cmd/run.rs` (`sideload_outcome` maps it to `strict-modes-failed`, run.rs:510); Outcome, the `conpty/` criterion MET on `windows-2025` with exit unchanged.'
    fact: F11
  - detector: D-cascade
    severity: warning
    file: .andromeda/obs-plan-detail/log-coverage.md
    near: 26
    section: '## 6. Log Coverage'
    change: 'Add a continuation line after line 26: `viola run` and `revive`''s start arm also run `strict::check_home_tree` (through `checked_exe` / `refuse_tree`: the home to the read rule, `check_instance`''s set, `ledger/`, `bin/`, `plugin/`, `bin/<key>/`, the pinned copy, `plugin/<key>/`, its `hooks/` and `.claude-plugin/`) before `open_wrapper_log`; a refusal is the home pair (home level) or the instance pair (instance level) on stderr and exit 1, with no log opened, no line and no child (revive under `--json` keeps the `strict-modes-failed` document), so a refused home tree has no `process-exit` detail code either (chunk 2026-10-26-home-and-code-bearing-file-integrity).'
    sidecar: 'log-coverage#2.1: `run` and revive''s start refuse a home tree before the wrapper log, exit 1 with no line, so the refusal carries no exit-1 detail code.'
    rationale: 'Report Symbols, `src/cmd/run.rs`: `checked_exe` / `refuse_tree` (189-215), called by `run` (166-172) and `revive`''s start (revive.rs:125-135) before `open_wrapper_log`: exit 1, no log opened, no line, no child; the entity states only revive''s `check_instance` as running before the log. Line 26 is near the 2 000-char cap, so the fact goes on its own continuation line.'
    basis: '.andromeda/obs-plan-detail/log-coverage.md:26'
  - detector: D-cascade
    severity: warning
    file: .andromeda/obs-plan-detail/log-coverage.md
    near: 31
    section: '## 6. Log Coverage'
    change: 'After «…four of the seven).» add: the event and dialog paths take `canonical` then `trusted` (the instance check) before `read_diagnostics_level` and obs init, so a refused or absent instance, a `VIOLA_DIR` naming no existing directory included, writes no `hook-decision` and no other line either, and no hook arm writes `strict-modes-failed` (chunk 2026-10-26-home-and-code-bearing-file-integrity; the founder''s own answer, `inputs#I3` Q2).'
    sidecar: 'log-coverage#2.4: the hook event and dialog paths'' refused instance check writes no `hook-decision`; no hook arm writes `strict-modes-failed`.'
    rationale: 'Report Symbols, `src/cmd/hook.rs`: `canonical` (75-81) and `trusted` (83-86) shared by both arms; a refused or absent instance returns exit 0 at once with no line, no frame, no SessionEnd append; Deviation 5 (a `VIOLA_DIR` naming no directory writes nothing); Decisions (`inputs#I3` Q2: no line on a refused check).'
    basis: '.andromeda/obs-plan-detail/log-coverage.md:31'
  - detector: D-cascade
    severity: warning
    file: .andromeda/obs-plan-detail/log-coverage.md
    near: 32
    section: '## 6. Log Coverage'
    change: 'After the `server-verify-failed` sentence add: `channel-unreachable` is also hook `connect`''s code over a snapshot `read_snapshot_or_replay` returns as `Replayed` or `Absent` (a replay carries no `endpoint`, `pid` or `started_at`): `NotSent::Unreachable`, fail open, SessionEnd''s direct append as before (chunk 2026-10-26-home-and-code-bearing-file-integrity).'
    sidecar: 'log-coverage#2.4: hook `connect` over a replayed or absent snapshot fails open as `channel-unreachable`.'
    rationale: 'Report Symbols, `src/cmd/hook.rs`: hook `connect` (224-226) reads `read_snapshot_or_replay`; `Replayed` / `Absent` → `NotSent::Unreachable` (fail open, `channel-unreachable`, SessionEnd''s direct append as before); Counts 5 (`read_snapshot(` product callers 3 → 0).'
    basis: '.andromeda/obs-plan-detail/log-coverage.md:32'
  - detector: D-cascade
    severity: warning
    file: .andromeda/obs-plan-detail/log-coverage.md
    near: 35
    section: '## 6. Log Coverage'
    change: 'Replace «and the nine hook events run no instance check» with «and the hook event and dialog paths'' refused instance check (`trusted`, before obs init) writes no line either, chunk 2026-10-26-home-and-code-bearing-file-integrity»; the rest of the line unchanged.'
    sidecar: 'log-coverage#2.7: the hook events now run an instance check, whose refusal writes no line (contradiction resolved).'
    rationale: 'Report Symbols, `src/cmd/hook.rs`: the event and dialog paths take `instance_of(…).and_then(canonical).filter(trusted)` before `read_diagnostics_level` / `viola_obs_init`; a refused instance writes no line; Retired item (1), `hook.event`''s snapshot read without strict-modes.'
    basis: '.andromeda/obs-plan-detail/log-coverage.md:35'
  - detector: D-cascade
    severity: warning
    file: .andromeda/obs-plan-detail/log-coverage.md
    near: 36
    section: '## 6. Log Coverage'
    change: 'Add a continuation line after line 36: the chunk''s other refused strict-modes checks write no line either (no `process-start`, no `process-exit`, no `strict-modes-failed`; the founder''s own answer, `inputs#I3` Q2, chunk 2026-10-26-home-and-code-bearing-file-integrity): `send` / `wait` / `last` / `answer` / `pause` / `release` through `client::refuse_untrusted` (exit 21, link''s instance pair, or under `--json` the `instance-unreachable` document); `viola mcp`''s frame tools (`instance-unreachable`, `detail: null`); `viola verify`''s `check_home_tree` and `update_stamps`''s `StampsError::StrictModes` (exit 1, the home pair); `run` and `revive`''s start `check_home_tree` (exit 1, the home or instance pair) — so on `process-exit` exit 21 still no arm writes the word.'
    sidecar: 'log-coverage#2.7: the six frame verbs, the MCP frame tools, verify and run / revive''s start join the no-line filesystem refusals.'
    rationale: 'Report Symbols (`refuse_untrusted` 20-41 and its six callers; tools.rs 337-339; verify.rs 64-70 and 288-296; run.rs 166-172, revive.rs 125-135), Counts 20 (each new no-line refusal site; log-coverage.md:35 an exemption-list row) and Decisions (`inputs#I3` Q2). Line 35 is near the 2 000-char cap, so the list goes on its own continuation line.'
    basis: '.andromeda/obs-plan-detail/log-coverage.md:35'
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/log-coverage.md
    near: 36
    section: '## 6. Log Coverage'
    change: 'Add a continuation line after line 36: `open_role_file` and `write_detail` (`src/obs.rs`) run `strict::check_diagnostics` on the existing `diagnostics/` folder and on the role or detail file before `ensure_private_dir` narrows anything: a symlink is refused (`symlink_metadata`), a folder is held to the read rule, a file to the read rule on Unix and the write rule on Windows; a refusal is a failed open, so the writer ends at `std::io::sink` and a detail line is dropped (chunk 2026-10-26-home-and-code-bearing-file-integrity).'
    sidecar: 'log-coverage#2.7: the diagnostics opens refuse a symlinked or widened `diagnostics/` path as a failed open (F10).'
    rationale: 'Report Symbols, `src/obs.rs`: private `check_diagnostics(dir, file)` (69-76) in `open_role_file` and `write_detail`, a refusal an `io::Error`, exactly a failed open (the writer ends at `std::io::sink`, a detail line dropped); `viola_state::strict::check_diagnostics` (127-145). It sits beside the filesystem refusals and the `symlink-ignored` rule of this entity.'
    fact: F10
  - detector: D-cascade
    severity: warning
    file: .andromeda/obs-plan-detail/error-capture-reporting.md
    near: 15
    section: '## 7. Error Capture & Reporting'
    change: 'Replace «only after its instance check has passed; a refused check on the start arm is the stderr pair and exit 1 with no process log opened» with «only after its home-tree check (`check_home_tree` through `checked_exe`) and its instance check have passed; a refused check on the start arm, or a refused home tree on `viola run`, is the stderr pair (the home pair, or the instance pair at instance level) and exit 1 with no process log opened».'
    sidecar: 'error-capture-reporting#2.1: run / revive''s start open their log only after the home-tree check; its refusal opens none.'
    rationale: 'Report Symbols, `src/cmd/run.rs`: `checked_exe` (189-200) and `refuse_tree` (202-215: `HomeLevel` → the home pair, `InstanceLevel` → the instance pair) called by `run` (166-172) and `revive`''s start (revive.rs:125-135) before `open_wrapper_log`: exit 1, no log opened.'
    basis: '.andromeda/obs-plan-detail/error-capture-reporting.md:15'
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/error-capture-reporting.md
    near: 15
    section: '## 7. Error Capture & Reporting'
    change: 'Add a sub-bullet after line 15: `  - **Catch-site reach (chunk 2026-10-26-home-and-code-bearing-file-integrity):** on Unix a home under a regular file is a strict-modes refusal with no line — `verify` exit 1 with the home pair, the frame verbs exit 21 with the instance pair — where the `cli` catch-site `process-exit{detail:"internal-error"}` line stood; on Windows such a path stats `NotFound`, the check skips it as absent and the catch-site line stands.`'
    sidecar: 'error-capture-reporting#2.1: the `cli` catch-site line no longer reports an unstattable home on Unix (F14).'
    rationale: 'Report Counts 13 (an unstattable home a strict-modes refusal; on Windows it stats `NotFound` and the catch-site line stands), Deviation 8 (`verify_under_a_regular_file_refuses_the_home`, per-OS oracle) and Decisions (a path under a regular file stats `NotFound` on Windows).'
    fact: F14
  - detector: D-cascade
    severity: warning
    file: .andromeda/obs-plan-detail/error-capture-reporting.md
    near: 23
    section: '## 7. Error Capture & Reporting'
    change: 'Before «, whose refusal (exit 21 …» keep the link sentence and append to the window list: «and the instance check the hook event and dialog paths run (`canonical` then `trusted`) before `read_diagnostics_level` and obs init, the six frame verbs'' `client::refuse_untrusted` before their `cli` role file, `viola verify`''s `check_home_tree` before its `cli` log, and `run` and `revive`''s start `check_home_tree` before `open_wrapper_log` (chunk 2026-10-26-home-and-code-bearing-file-integrity), each refusal writing no line (`inputs#I3` Q2)».'
    sidecar: 'error-capture-reporting#3.1: the pre-init window gains the hook event/dialog, frame-verb, verify and run / revive start checks.'
    rationale: 'Report Symbols (hook.rs `canonical` / `trusted` before obs init; `refuse_untrusted` called first by the six verbs, opening nothing under the home; verify.rs 64-70 before its `cli` log; run.rs 166-172, revive.rs 125-135 before `open_wrapper_log`); the task''s Touches (§7 panic window before init step 4).'
    basis: '.andromeda/obs-plan-detail/error-capture-reporting.md:23'
  - detector: D-cascade
    severity: warning
    file: .andromeda/obs-plan-detail/slo-invariants-telemetry-budgets.md
    near: 11
    section: '## 10. SLO Invariants & Telemetry Budgets'
    change: 'In exemption 1, before the closing «)», add: «, and the hook event and dialog paths'' instance check before obs init, the six frame verbs'' `client::refuse_untrusted` before their `cli` role file, `viola verify`''s `check_home_tree` before its `cli` role file, and `run` / `revive`''s start `check_home_tree` before `open_wrapper_log` (chunk 2026-10-26-home-and-code-bearing-file-integrity)».'
    sidecar: 'slo#1.1 exemption 1: the pre-init window gains the chunk''s new checks before any role file.'
    rationale: 'Report Symbols (each check runs before its process''s role file or wrapper log opens: hook.rs 75-86, client.rs 20-41, verify.rs 64-70, run.rs 166-172, revive.rs 125-135); Counts 20 (slo-invariants-telemetry-budgets.md:17 an exemption-list row).'
    basis: '.andromeda/obs-plan-detail/slo-invariants-telemetry-budgets.md:11'
  - detector: D-cascade
    severity: warning
    file: .andromeda/obs-plan-detail/slo-invariants-telemetry-budgets.md
    near: 13
    section: '## 10. SLO Invariants & Telemetry Budgets'
    change: 'Exemption 3 reads: «`hook` without `VIOLA_NAME` (D-16), or whose `VIOLA_DIR` names an instance directory that does not exist: it canonicalises to no session and runs no obs init (chunk 2026-10-26-home-and-code-bearing-file-integrity)».'
    sidecar: 'slo#1.1 exemption 3: a hook whose `VIOLA_DIR` names no existing directory has no role file either.'
    rationale: 'Report Symbols, `src/cmd/hook.rs` (a `VIOLA_DIR` naming an instance directory that does not exist canonicalises to no session, so the hook writes nothing) and Deviation 5.'
    basis: '.andromeda/obs-plan-detail/slo-invariants-telemetry-budgets.md:13'
  - detector: D-cascade
    severity: warning
    file: .andromeda/obs-plan-detail/slo-invariants-telemetry-budgets.md
    near: 14
    section: '## 10. SLO Invariants & Telemetry Budgets'
    change: 'In exemption 4, after «a failed step-4 role-file open» add: «, `strict::check_diagnostics` refusing the `diagnostics/` folder or the role file included (a symlink or a widened mode; chunk 2026-10-26-home-and-code-bearing-file-integrity)».'
    sidecar: 'slo#1.1 exemption 4: a role-file open refused by `check_diagnostics` is a failed open.'
    rationale: 'Report Symbols, `src/obs.rs`: `check_diagnostics` (69-76) runs in `open_role_file`; a refusal is an `io::Error`, exactly a failed open, the writer ending at `std::io::sink`; Counts 20 (the diagnostics opens a new no-line site).'
    basis: '.andromeda/obs-plan-detail/slo-invariants-telemetry-budgets.md:14'
  - detector: D-cascade
    severity: warning
    file: .andromeda/obs-plan-detail/slo-invariants-telemetry-budgets.md
    near: 17
    section: '## 10. SLO Invariants & Telemetry Budgets'
    change: 'Add item 8 after exemption 7: «8. a refused strict-modes check at the entry points of chunk 2026-10-26-home-and-code-bearing-file-integrity — the hook event and dialog paths (exit 0, nothing printed), `send` / `wait` / `last` / `answer` / `pause` / `release` (exit 21, the instance pair or the `instance-unreachable` document), `viola verify` (exit 1, the home pair) and `run` / `revive`''s start home tree (exit 1, the home or instance pair): it opens no role file and writes no line, so a panic there writes none, by §7''s rule; that a refused check writes no line is the founder''s own answer (`inputs#I3` Q2).»'
    sidecar: 'slo#1.1: exemption 8 added, the chunk''s refused checks before any role file.'
    rationale: 'Report Symbols (hook.rs, client.rs `refuse_untrusted`, verify.rs 64-70, run.rs / revive.rs `checked_exe`) and Decisions (`inputs#I3` Q2: a refused check at every new entry point writes no line); Counts 20 names slo-invariants-telemetry-budgets.md:17. Exemption 7''s line is near the 2 000-char cap, so the sites go in a new item.'
    basis: '.andromeda/obs-plan-detail/slo-invariants-telemetry-budgets.md:17'
  - detector: D-cascade
    severity: warning
    file: .andromeda/obs-plan-detail/obs-anti-patterns.md
    near: 8
    section: '## 11. Obs Anti-Patterns (NEVER do these) › ### Logs'
    change: 'Replace «and `link`''s / `unlink`''s instance-check refusal» with «and the instance-check refusal of `link` / `unlink` and of `send` / `wait` / `last` / `answer` / `pause` / `release`», and «callers `run` 5 · `verify` 5 · `list` 2 · `revive` 1 · `plugin install` 1 · `link` 1, the one `link_call` site both verbs share» with «callers `run` 7 · `verify` 6 · `list` 2 · `revive` 1 · `plugin install` 1 · `client` 1, the one `refuse_untrusted` site `link` / `unlink` and the six frame verbs share»; the rule in the parenthesis unchanged.'
    sidecar: 'obs-anti-patterns#1.1: `human::refuse(` callers 15 → 18 by its rule (run 7, verify 6, client 1).'
    rationale: 'Report Symbols, remaining-caller facts and Counts 10: `human::refuse(` under `src/` 15 → 18 (verify 6, run 7, list 2, revive 1, plugin 1, client 1); `link_call` moved onto `refuse_untrusted`, shared by link/unlink and the six verbs. The report reads this row as «no figure», but the entity carries the per-file figures, now stale.'
    basis: '.andromeda/obs-plan-detail/obs-anti-patterns.md:8'
note: F7 (error-capture-reporting, the MCP frame tools' refused check_instance) fits no picked entity — its broad line is obs-plan.md:537 (MCP rejects), for the new-detail pass; F3 and F8 (span-trace-coverage) bear on log-coverage#2.1's already-live and exit-1 wording; obs-plan.md:667's diagnostics build-failure condition is worth a re-read against check_diagnostics.
