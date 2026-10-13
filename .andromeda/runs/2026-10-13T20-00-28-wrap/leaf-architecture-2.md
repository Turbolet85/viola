group: architecture-2
read: .claude/docs/services/viola-state.md · 49 · «- `.andromeda/architecture.md` §Standard Contracts (snapshot»
read: .claude/docs/services/viola-ui.md · 42 · «- `.claude/rules/api.md` · `.claude/rules/frontend.md` · `.c»
read: .claude/docs/services/viola.md · 58 · «- `.andromeda/architecture.md` · `.claude/rules/api.md` · `.»
read: .claude/docs/stack.md · 70 · «5. Commit as `chore: bump {tool} to {version}`»
read: .claude/docs/workflow.md · 67 · «This project uses a **reference-based** CLAUDE.md architectu»
read: .claude/rules/events.md · 39 · «_This section is owned by `/wrap-session`. setup-project pre»

- leaf: .claude/docs/services/viola.md:6   class: omission
  leaf-text: «checks the seventeen ledger rows, prints the step lines and `stamped` summary and writes the stamps through `viola_state::stamps::update_stamps`;»
  source: .andromeda/architecture.md:91 «It is ended by Ctrl-C, Ctrl-C again after 500 ms, then a kill at `PROBE_DEADLINE`; a run stopped by Ctrl-C or TERM (`cmd::verify::stop`) is ended by a kill with no key, starts no later run and writes no stamp.»
  replace-with: «checks the seventeen ledger rows, prints the step lines and `stamped` summary and writes the stamps through `viola_state::stamps::update_stamps`; a verify stopped by Ctrl-C or TERM (`cmd::verify::stop`: Unix `SIGINT` / `SIGTERM`, Windows `CTRL_C_EVENT` / `CTRL_BREAK_EVENT`, installed by `viola verify` alone, its handler only setting one process-global flag that every wait of the run reads) kills and reaps the running child with no key, starts no later run, writes no stamp and exits 1 (`unable: verify was stopped before it finished` / `hint: run viola verify again to stamp this CLI`, stdout empty), its drop guards removing only the dirs that run created; SIGKILL, a crash, a failed handler install and a Windows close, logoff or shutdown keep the OS default and leave the dirs to be removed by hand;»

- leaf: .claude/docs/services/viola.md:11   class: omission
  leaf-text: «`GetModuleFileNameW` for `src/panic_frames.rs`);»
  source: .andromeda/registries/contracts/architecture/crate-dependency-direction.md:11 «plus `Win32_System_Console` `SetConsoleCtrlHandler` for `src/cmd/verify/stop.rs`'s console routine, where Ctrl-C and Ctrl-Break set verify's stop and every other event keeps the OS default»
  replace-with: «`GetModuleFileNameW` for `src/panic_frames.rs`, and `SetConsoleCtrlHandler` (`Win32_System_Console`) for `src/cmd/verify/stop.rs`'s console routine, where Ctrl-C and Ctrl-Break set verify's stop and every other event keeps the OS default);»

- leaf: .claude/docs/services/viola.md:11   class: omission
  leaf-text: «libc (Unix only: `backtrace` + `dladdr` for the raw panic frames);»
  source: .andromeda/registries/contracts/architecture/crate-dependency-direction.md:11 «and `sigaction` for the same module's `SIGINT` / `SIGTERM` handler, `SA_RESTART`); neither role added a dependency or a feature (chunk 2026-10-13-interrupted-verify-cleanup)»
  replace-with: «libc (Unix only: `backtrace` + `dladdr` for the raw panic frames, and `sigaction` for `src/cmd/verify/stop.rs`'s `SIGINT` / `SIGTERM` handler, `SA_RESTART`; neither stop role added a dependency or a feature);»

- leaf: .claude/docs/services/viola.md:35   class: omission
  leaf-text: «and each of the four interactive PTY runs as `verify-pty-probe` (obs-plan §6).»
  source: .andromeda/architecture.md:406 «a child killed at a Ctrl-C or TERM stop closes its pair with `duration_ms` and no `child_exit_status`, a stopped run writes only the pairs it started, and its self exit reads `exit_code:1, detail:"interrupted"`»
  replace-with: «and each of the four interactive PTY runs as `verify-pty-probe` (obs-plan §6); a child killed at a Ctrl-C or TERM stop closes its pair with `duration_ms` and no `child_exit_status`, a stopped run writes only the pairs it started, and its self exit reads `exit_code:1, detail:"interrupted"`.»

- leaf: .claude/docs/services/viola.md:47   class: tree
  leaf-text: «- **Verify / version gate / panic frames:** `src/cmd/verify.rs`, `src/run/version_gate.rs`, `src/panic_frames.rs`»
  source: .andromeda/registries/contracts/architecture/project-directory-structure.md:33 «verify/stop.rs: verify's Ctrl-C / TERM stop (Unix SIGINT / SIGTERM, Windows Ctrl-C / Ctrl-Break)»
  replace-with: «- **Verify / version gate / panic frames:** `src/cmd/verify.rs`, `src/cmd/verify/typed.rs`, `src/cmd/verify/stop.rs` (verify's Ctrl-C / TERM stop), `src/run/version_gate.rs`, `src/panic_frames.rs`»
