proposals:
  - detector: D-arch-resources
    severity: warning
    section: '§Occupied Resources → Filesystem (verify''s interactive probe dirs)'
    change: 'Replace «All are removed on every exit path, passing or failing» with: all are removed when verify passes, fails, or is stopped by Ctrl-C / Ctrl-Break (Windows console control events 0 and 1) or SIGINT / SIGTERM (Unix); the stop handler `cmd::verify::stop`, installed by `viola verify` alone, only sets one process-global flag, which verify reads at fixed points, killing and reaping the running child before the drop guards remove the dirs this run created (never by a pattern or a listing); SIGKILL, a crash, a failed handler install (ignored, fail-open) and a Windows close / logoff / shutdown keep the OS default and leave the dirs to be removed by hand.'
    sidecar: 'arch §Occupied Resources → Filesystem: verify''s interactive probe dirs are removed on a pass, a fail and a Ctrl-C / Ctrl-Break / SIGINT / SIGTERM stop (cmd::verify::stop); SIGKILL, a crash and a Windows close / logoff / shutdown leave them (chunk 2026-10-13-interrupted-verify-cleanup).'
    rationale: 'The report''s Symbols / APIs bullet lands `cmd::verify::stop`, the root package''s one signal / console-control handler, a process-global OS registration registered in no arch section, and states its effect on the registered dirs: a stop removes the print probe''s, Run A''s and Run B''s dirs and leaves planted foreign dirs (Outcome, first criterion), while SIGKILL, a crash and a Windows close / logoff / shutdown keep their default and leave the dirs by hand. The body''s «every exit path» names no exception; the report''s Expected amendments item 1 carries this site (architecture `:413`).'
    basis: 'src/cmd/verify/stop.rs:16-20 (install), src/cmd/verify/stop.rs:43-56 (Unix sigaction), src/cmd/verify/stop.rs:58-75 (Windows console routine); architecture.md:413'
  - detector: D-arch-resources
    severity: warning
    section: §Occupied Resources → Filesystem (`ledger/probes/<pid>/`)
    change: 'Replace «removed whole by a drop guard on every exit path» with: removed whole by a drop guard when verify passes, fails or is stopped by Ctrl-C / Ctrl-Break / SIGINT / SIGTERM (`cmd::verify::stop`); only this run''s `<pid>` dir is removed, and SIGKILL, a crash and a Windows close / logoff / shutdown leave it to be removed by hand.'
    sidecar: 'arch §Occupied Resources → Filesystem: `ledger/probes/<pid>/` removal names the stop and its exceptions (chunk 2026-10-13-interrupted-verify-cleanup).'
    rationale: 'The same retired claim («every exit path», no exception) restated for the print probe''s dir; the report''s Outcome states the print probe''s dir is gone after a stop while a planted `ledger/probes/0/` stands, and its Symbols bullet that SIGKILL and a crash leave the dirs by hand. Report Expected amendments item 1 lists architecture `:412` under `verify-dirs`.'
    basis: 'architecture.md:412; src/cmd/verify.rs:242-244 (the stop read after the probe dir is dropped)'
    dependent-of: D-arch-resources
  - detector: D-arch-resources
    severity: warning
    section: §Occupied Resources → Filesystem (`diagnostics/`, the `cli-<name>.ndjson` producers)
    change: 'In verify''s pair, add: a verify stopped before it finished ends `process-exit{subject:"self", exit_code:1, detail:"interrupted"}` (`detail` a member of `schemas/diag-line.v1.json`''s closed `process-exit.detail` enum, now 12 members), and a spawn pair whose child was killed at the stop closes with `process-exit{subject, duration_ms}` and no `child_exit_status`; every spawn pair is still closed.'
    sidecar: 'arch §Occupied Resources → Filesystem diagnostics/: verify''s interrupted self exit (`detail:"interrupted"`) and a stopped child''s `process-exit` without `child_exit_status` (chunk 2026-10-13-interrupted-verify-cleanup).'
    rationale: 'The report''s Schema / config bullet adds `interrupted` to `process-exit.detail` (11 → 12) and its Symbols bullet states `verify` logs `run::log_self_exit(1, Some("interrupted"))` and that a stopped child''s `process-exit` carries `duration_ms` and no `child_exit_status`; the body registers each verify spawn pair as closing `process-exit{subject, child_exit_status, duration_ms}` and lists no interrupted self exit, so the new logged shape is unregistered.'
    basis: 'schemas/diag-line.v1.json:82; src/cmd/verify.rs:69-78'
  - detector: D-arch-resources
    severity: warning
    section: §Conventions → CLI exit codes (the `1` item)
    change: 'Add to verify''s exit-1 causes: `viola verify` stopped by Ctrl-C / Ctrl-Break / SIGINT / SIGTERM before it finished — no stamp written, stdout empty, stderr exactly the pair `unable: verify was stopped before it finished` / `hint: run viola verify again to stamp this CLI`, and `process-exit{subject:"self", exit_code:1, detail:"interrupted"}` when `VIOLA_NAME` holds a valid name.'
    sidecar: 'arch §Conventions CLI exit codes: exit 1 covers an interrupted `viola verify` and its fixed stderr pair (chunk 2026-10-13-interrupted-verify-cleanup).'
    rationale: 'The exit-1 list enumerates verify''s exit-1 outcomes (a failing row with its stamp written, and verify''s refusal pairs) and is the definition Occupied Resources → Binary, subcommands and exit codes defers to; the report''s Symbols bullet adds a third outcome with its own pair and self exit, and its Outcome states stdout empty, stderr exactly the pair, exit 1 and no `ledger/stamps.json`. Report Expected amendments item 3 (`verify-exit1`, architecture `:137`).'
    basis: 'architecture.md:137; src/cmd/verify.rs:69-78'
    dependent-of: D-arch-resources
  - detector: D-arch-resources
    severity: warning
    section: §Occupied Resources → Claude Code integration names (child flags bullet)
    change: 'Replace «`viola verify`''s Runs C and D hold the same literal at two sites in `src/cmd/verify/typed.rs` from before that constant; moving them onto it is owed to the working-route entry "Interrupted verify cleanup"» with: `viola verify`''s Runs C and D read the same constant, and verify''s default program reads `viola_agent_claude::PROGRAM` (both moved at chunk 2026-10-13-interrupted-verify-cleanup); no owed move remains.'
    sidecar: 'arch §Occupied Resources → Claude Code integration names: verify reads `SETTINGS_FLAG` and `PROGRAM`; the owed-move clause naming "Interrupted verify cleanup" retired (chunk 2026-10-13-interrupted-verify-cleanup).'
    rationale: 'The report''s Symbols bullet states the default program reads `viola_agent_claude::PROGRAM` and Runs C and D''s `--settings` read `viola_agent_claude::statusline::SETTINGS_FLAG`; the registry still records the two literals in typed.rs as owed to this very entry. Report Expected amendments item 2 (`owed-literals`, architecture `:371`).'
    basis: 'architecture.md:371'
  - detector: D-arch-decisions
    severity: warning
    section: §Infrastructure Patterns → Crate dependency direction
    change: 'In the root bin bullet, add to windows-sys''s Windows-only roles `Win32_System_Console` `SetConsoleCtrlHandler` for `src/cmd/verify/stop.rs`''s console routine (Ctrl-C and Ctrl-Break set the stop; every other event keeps the OS default), and to libc''s Unix-only roles `sigaction` (SIGINT and SIGTERM, `SA_RESTART`) for the same module''s signal handler; neither is a new dependency or feature.'
    sidecar: 'arch Crate dependency direction: the root''s windows-sys `SetConsoleCtrlHandler` and libc `sigaction` roles for `cmd::verify::stop` (chunk 2026-10-13-interrupted-verify-cleanup).'
    rationale: 'The report''s Dependencies bullet adds no crate or feature but states the handler uses the root package''s existing libc (Unix) and windows-sys (Windows, the workspace feature list''s `Win32_System_Console`); the contract enumerates the root''s libc role as `backtrace` + `dladdr` and its windows-sys roles as the registry reader and the panic-frame APIs only, so the landed use is outside the recorded allowance. Report Expected amendments item 4 (`root-ffi`, crate-dependency-direction.md:11).'
    basis: 'registries/contracts/architecture/crate-dependency-direction.md:11; src/cmd/verify/stop.rs:43-56; src/cmd/verify/stop.rs:58-75'
  - detector: D-arch-resources
    severity: warning
    section: §Infrastructure Patterns → CI/CD approach
    change: 'Replace «a nine-item `matrix.include` … The root package `viola` is four of the nine, split by file (`src/cmd/run.rs`; `src/main.rs` with `src/conpty.rs`; `src/run/env.rs`; `src/panic_frames.rs`)» with a ten-item `matrix.include` in which the root package `viola` is five of the ten, the fifth `label: viola-cmd-verify-stop`, `files: src/cmd/verify/stop.rs`; the «all nine jobs» of run 38036448183 is a past run''s reading and stays.'
    sidecar: 'arch CI/CD approach: `windows-mutants.yml` matrix 9 → 10 items, root `viola` 4 → 5 (`viola-cmd-verify-stop`, `src/cmd/verify/stop.rs`) (chunk 2026-10-13-interrupted-verify-cleanup).'
    rationale: 'The report''s Harness / gate surface and Counts bullets land a new matrix item, job `mutants (viola-cmd-verify-stop)` (matrix 9 → 10, root 4 → 5), forced by `tests/contract_windows_mutation_scope.rs`; the contract''s current-state description of the matrix still reads nine and four. The report classed ci-cd-approach.md:2 only by its past-run «all nine jobs», but the same line also holds this present-tense count and file list, which its sweep did not list.'
    basis: '.github/workflows/windows-mutants.yml:48-50; registries/contracts/architecture/ci-cd-approach.md:2'
note: 'Not proposed, outside both detectors'' sections: the Project directory structure tree (§Infrastructure Patterns) lists `verify/typed.rs` but not the new `verify/stop.rs`; and [CLI Version Compatibility] (architecture.md:91) still describes Run B''s Ctrl-C ending and verify''s stamping without the stop path (no contradiction, an omission). No new IPC method, endpoint, event kind, port, socket, env var, crate or dependency (report Interfaces / Dependencies).'
