proposals:
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/error-capture-reporting.md
    near: 23
    section: '## 7. Error Capture & Reporting'
    change: 'Extend the line-23 window''s list (the link / unlink clause kept as written): the window also holds the instance check (`client::refuse_untrusted`, `check_instance` on the home and `instances/<name>`) that `viola send` (after its text read), `wait`, `last`, `answer`, `pause` and `release` run first, before their `cli` role file opens, whose refusal is link''s (exit 21, the instance-state pair or under `--json` `{"v":1,"error":"instance-unreachable","detail":null}`, no frame, no line); the instance check (`canonical` then `trusted`, `src/cmd/hook.rs`) that `viola hook`''s event and dialog paths run after the two `--capture` arms and before `read_diagnostics_level` / `viola_obs_init`, whose refusal (a refused instance, or a `VIOLA_DIR` naming an instance directory that does not exist) is exit 0 at once with no `config.json` read, no role file, no stdin read, no line and no frame; `viola verify`''s home-tree check (`check_home_tree` on the home, `ledger/`, `bin/` and `plugin/`) before its `cli` log, whose refusal is the home-state pair, exit 1, no step line and no summary; and the home-tree check `viola run` and `viola revive`''s start arm make (`checked_exe`, then `check_home_tree` with the name and the pin key) before `open_wrapper_log`, whose refusal is the home-state or instance-state pair, exit 1, no log opened, no line and no child (revive `--json` keeps `strict-modes-failed`); each writes no line, as link / unlink, by the founder''s own word (inputs#I3 Q2).'
    sidecar: 'error-capture-reporting: the pre-init no-file window gains the six frame verbs'' instance check, the hook event/dialog instance check, verify''s and run/revive''s home-tree checks, each refusing with no line (chunk 2026-10-26-home-and-code-bearing-file-integrity, inputs#I3 Q2).'
    rationale: 'The report''s Changes add `refuse_untrusted` called first by the six frame verbs (client.rs:20-41; opens nothing under the home and writes no line), the hook''s `canonical`/`trusted` before `viola_obs_init` (hook.rs:75-86), verify''s `check_home_tree` before its `cli` log (verify.rs:64-70) and run/revive''s `checked_exe`/`refuse_tree` before `open_wrapper_log` (run.rs:166-172, 189-215; revive.rs:125-135); Count 20 names this file''s exemption row to be re-read for the new no-line sites and Count 23 the hook''s nonexistent-`VIOLA_DIR` silence; line 23 names only revive, hook statusline, mcp and link / unlink.'
    basis: viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/report.md:15
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/error-capture-reporting.md
    near: 23
    section: '## 7. Error Capture & Reporting'
    change: 'After the line-23 item, add one item: after its init, `viola mcp`''s frame tools (`send`, `wait`, `last`, `answer`) run `check_instance` on the home and the instance directory before `live_endpoint` (`crates/viola-mcp/src/tools.rs` `frame`); a refusal returns `instance-unreachable` with `detail: null`, writes no frame and no line, and its `isError` code is its only record, as the liveness pre-check''s.'
    sidecar: 'error-capture-reporting: the MCP frame tools'' strict-modes check refuses as `instance-unreachable` with no line (chunk 2026-10-26-home-and-code-bearing-file-integrity).'
    rationale: 'The report''s Changes: `viola-mcp` `tools.rs` `frame` (337-339) runs `check_instance` before `live_endpoint`, a refusal `result::unreachable()` (`instance-unreachable`, `detail: null`), no frame, no line; Coverage marks it no line; this check runs after mcp''s role file is open, so the line-23 pre-init window cannot hold it and the file states no MCP frame-tool refusal.'
    basis: viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/report.md:14
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/error-capture-reporting.md
    near: 15
    section: '## 7. Error Capture & Reporting'
    change: 'After the main-thread catch-site item, add one item: on Unix a home under a regular file (an unstattable home) no longer reaches the `cli` catch site (`process-exit{detail:"internal-error"}`, `error: internal error`) on the checked verbs; it is a strict-modes refusal before any log (`verify` exit 1 with the home-state pair, the six frame verbs exit 21 with the instance-state pair), and on Windows, where such a path stats `NotFound` and the check skips it as absent, the catch-site line stands (a per-OS oracle, `cli_verify`''s `verify_under_a_regular_file_refuses_the_home`).'
    sidecar: 'error-capture-reporting: the catch site''s reach narrowed, an unstattable home a strict-modes refusal on Unix, the catch-site line on Windows (chunk 2026-10-26-home-and-code-bearing-file-integrity, Count 13).'
    rationale: 'Report Count 13: an unstattable home is now a strict-modes refusal (Unix verify exit 1 home pair, the frame verbs exit 21 instance pair) where the `cli` catch-site line stood, Windows unchanged; it names obs-plan.md:496 as the catch-site reporter unchanged and «the narrowed reach is new text»; Deviation 8 and the directive item (b) (inputs#I5) ask the behaviour change be stated.'
    basis: viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/report.md:40
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/error-capture-reporting.md
    near: 8
    section: '## 7. Error Capture & Reporting'
    change: 'After the line-8 item, add one item: a role file or detail file is opened only through `src/obs.rs` `check_diagnostics`, which runs `strict::check_diagnostics` on the existing `diagnostics/` folder and the file in `open_role_file` and `write_detail` before `ensure_private_dir` narrows anything (a symlink refused, a folder the read rule, a file the read rule on Unix and the write rule on Windows, an absent path passing); a refusal is exactly a failed open, so the role writer ends at `std::io::sink` and the content-bearing detail line (the chain, the panic payload) is dropped.'
    sidecar: 'error-capture-reporting: the role and detail files open only past `check_diagnostics`; a refused path is a failed open, the detail line dropped (chunk 2026-10-26-home-and-code-bearing-file-integrity).'
    rationale: 'The report''s Changes: `src/obs.rs` new private `check_diagnostics(dir, file)` (69-76) runs `strict::check_diagnostics` (strict.rs:127-145) before `ensure_private_dir` in `open_role_file` and `write_detail`, a refusal an `io::Error`, «exactly a failed open (the writer ends at `std::io::sink`, a detail line dropped)»; the file states the detail line is written whenever an instance resolves, with no condition on the diagnostics path.'
    basis: viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/report.md:20
note: 'obs-plan.md:537 (body §7, outside this file) names the liveness pre-check and server-verify mismatch as MCP''s no-line `instance-unreachable` causes and owes the frame tools'' strict-modes check beside them; no passage of this file states a retired dated gap as standing (D-retired none); D-obs-stack and D-obs-pii find nothing (no dependency change, no new logged field, every new refusal fixed-form with no path or pid).'
