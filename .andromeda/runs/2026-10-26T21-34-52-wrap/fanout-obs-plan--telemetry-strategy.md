proposals:
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/telemetry-strategy.md
    near: 20
    section: '## 2. Telemetry Strategy'
    change: 'In the bounded-exemptions parenthesis, after «or a refused instance check on `viola link` / `viola unlink`, made before their role file opens» add: «, or a refused instance check on the six frame verbs `send` / `wait` / `last` / `answer` / `pause` / `release` (`client::refuse_untrusted`, exit 21) or on `viola hook`''s event and dialog paths (`canonical` / `trusted`, before `viola_obs_init`, exit 0), or a refused home-tree check (`strict::check_home_tree`) on `viola run`''s or a revived start (before `open_wrapper_log`, exit 1) or on `viola verify` (before its `cli` log, exit 1), each made before any role file opens»; and widen «a failed step-4 role-file open» to «a failed step-4 role-file open (a `diagnostics/` folder or role file that `strict::check_diagnostics` refuses included)».'
    sidecar: 'telemetry-strategy §2 panic-exemption list gains the new no-file refusal sites of chunk 2026-10-26-home-and-code-bearing-file-integrity (six frame verbs, hook event/dialog paths, run/revive start and verify home-tree checks, a strict-modes-refused diagnostics open).'
    rationale: 'Report Changes Symbols (client.rs refuse_untrusted called first by the six verbs; hook.rs canonical/trusted before viola_obs_init; run.rs checked_exe/refuse_tree before open_wrapper_log; verify.rs check_home_tree before its cli log; obs.rs check_diagnostics a refusal is exactly a failed open) and Counts item 20 (each new no-line refusal site, the exemption-list rows re-read); the line enumerates the no-file windows and names only revive, hook statusline, list and link/unlink.'
    basis: viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/report.md:15
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/telemetry-strategy.md
    near: 30
    section: '## 2. Telemetry Strategy'
    change: 'In the Vector 4 (hook stdin) row, after «asserted by absence (chunk 2026-10-10-budget-governor)» add: «, and so does the event and dialog paths'' (an instance `canonical` / `trusted` refuses returns exit 0 before `viola_obs_init`: no `config.json` read, no role file, no stdin read, no line, no frame, no SessionEnd append; a `VIOLA_DIR` naming no existing instance directory writes nothing either; chunk 2026-10-26-home-and-code-bearing-file-integrity)».'
    sidecar: 'telemetry-strategy §2 Vector 4 row: the strict-modes item''s no-line outcome now covers viola hook''s event and dialog paths, not only hook statusline (chunk 2026-10-26-home-and-code-bearing-file-integrity).'
    rationale: 'Report Changes Symbols src/cmd/hook.rs (the event and dialog paths take instance_of(…).and_then(canonical).filter(trusted) before read_diagnostics_level / viola_obs_init; a refused or absent instance returns exit 0 with no line, no frame, no SessionEnd append) and Counts item 23 (a VIOLA_DIR naming no directory writes nothing); the row names only hook statusline''s refused check.'
    basis: viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/report.md:16
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/telemetry-strategy.md
    near: 33
    section: '## 2. Telemetry Strategy'
    change: 'In the Vector 7 (filesystem) row, after the §6 detail catalog Filesystem refusals item add: «, §6 `process-start` `sideload_fallback` `strict-modes-failed` (a Windows x64 `conpty/` companion its owner/DACL check refuses, `PinError::StrictModes`; the child on the inbox ConPTY, exit unchanged); a strict-modes refusal at every other new entry point (the six frame verbs, the hook event and dialog paths, the MCP frame tools, `run` / `revive` start and `verify`) writes no line, asserted by absence».'
    sidecar: 'telemetry-strategy §2 Vector 7 row gains the sideload_fallback strict-modes-failed signal and the no-line strict-modes refusals of chunk 2026-10-26-home-and-code-bearing-file-integrity.'
    rationale: 'Report Changes Schema/config (diag-line.v1.json:70 sideload_fallback gains strict-modes-failed), Symbols run.rs sideload_outcome maps PinError::StrictModes to strict-modes-failed, Coverage of new surfaces (conpty leg instrumented by sideload_fallback; every other new check instrumentation n/a, no line by the founder''s word inputs#I3 Q2), Counts items 7 and 18; the trigger map names no coverage for either.'
    basis: viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/report.md:24
note: 'Body obs-plan.md:215 (Vector 7 «a strict-modes failure has a line only on the `ledger-stamps` read in `run`''s version gate») now also misses `process-start.sideload_fallback` `strict-modes-failed` — a body site outside this category file; line 33''s §6 `state-recovered` pointer and line 38 are true as written (Counts 6 vocabulary/pointer rows), and no line here states a retired dated gap.'
