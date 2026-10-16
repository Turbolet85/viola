group: architecture-2
read: .claude/docs/services/viola-pty.md · 47 · «- `.andromeda/architecture.md` [PTY] · `.claude/docs/gotchas»
read: .claude/docs/services/viola-state.md · 49 · «- `.andromeda/architecture.md` §Standard Contracts (snapshot»
read: .claude/docs/services/viola-ui.md · 9 · «None: the `viola_ui` log target in `src/obs.rs`, the reserve»
read: .claude/docs/services/viola.md · 58 · «- `.andromeda/architecture.md` · `.claude/rules/api.md` · `.»
read: .claude/docs/stack.md · 66 · «5. Commit as `chore: bump {tool} to {version}`»
read: .claude/docs/workflow.md · 67 · «This project uses a **reference-based** CLAUDE.md architectu»

- leaf: .claude/docs/services/viola.md:6   class: stale
  leaf-text: «on Unix all twenty-two of `LedgerRow::ALL`, the last `resume-session`; on Windows nineteen, the first eighteen and `resume-session`, the last `resume-session`»
  source: .andromeda/architecture.md:88 «so a Unix host relies on all twenty-three rows and a Windows host on twenty, the first eighteen, `resume-session` and `agents-join`, the last two relied on every OS»
  replace-with: «on Unix all twenty-three of `LedgerRow::ALL`, the last `agents-join`; on Windows twenty, the first eighteen, `resume-session` and `agents-join`, the last `agents-join`»

- leaf: .claude/docs/services/viola.md:6   class: omission
  leaf-text: «after a long paste) —, Run C's dialogs in `-dialogs/`»
  source: .andromeda/architecture.md:88 «Once Run B's first turn has settled, before its framing pastes, verify makes ONE `claude agents --json` read (`typed::agents_read`: Run B's program, cwd the probe dir, the R8 strip»
  replace-with: «after a long paste) — and, once its first turn has settled and before its framing pastes, ONE `claude agents --json` read (`typed::agents_read`: Run B's program, cwd the probe dir, the R8 strip, `VERSION_DEADLINE`, none once a stop is read; its capture kept in memory, never written to the captures dir; no live start added; in CI the fake agent's `agents` mode answers it), Run C's dialogs in `-dialogs/`»

- leaf: .claude/docs/services/viola.md:21   class: stale
  leaf-text: «then a preflight of three readings in a fixed order, stopping at the first refusal (`already-live` → `no-session` → `cwd-missing`»
  source: .andromeda/architecture.md:90 «as the first of its four preflight readings (the name's holder, the logged session, `session-live`, the recorded directory; a later reading is not taken once an earlier one refuses)»
  replace-with: «then a preflight of four readings in a fixed order, stopping at the first refusal (`session-live` is one `claude agents --json` read: a row whose `sessionId` is the logged id about to be resumed refuses, `--fork` skips it, a failed read passes, its `agents-probe` pair in `run-<name>.ndjson`; `already-live` → `no-session` → `session-live` → `cwd-missing`»

- leaf: .claude/docs/services/viola.md:35   class: stale
  leaf-text: «`verify`'s seven child spawns are each logged as a `process-start`/`process-exit` pair at the call site: the `--version` read as `version-probe`, the print-mode probe as `verify-probe`, and each of the five interactive PTY runs (Run A, Run B, Run C, Run D, then Run E) as `verify-pty-probe`»
  source: .andromeda/architecture.md:341 «with eight spawn pairs between them — `version-probe` around the `--version` read, `verify-probe` around the print-mode probe»
  replace-with: «`verify`'s eight child spawns are each logged as a `process-start`/`process-exit` pair at the call site: the `--version` read as `version-probe`, the print-mode probe as `verify-probe`, each of the five interactive PTY runs (Run A, Run B, Run C, Run D, then Run E) as `verify-pty-probe`, and Run B's `claude agents --json` read as `agents-probe`»

- leaf: .claude/docs/services/viola.md:41   class: omission
  leaf-text: «`gate.rs` the bounded feed + `Gate`; `send.rs` the `send` module root»
  source: .andromeda/registries/contracts/architecture/project-directory-structure.md:35 «agents.rs: the bounded `claude agents --json` read (`read_logged`, `read_here`) shared by `list`, `revive` and verify's Run B;»
  replace-with: «`gate.rs` the bounded feed + `Gate`; `agents.rs` the bounded `claude agents --json` read (`read_logged`, `read_here`) shared by `list`, `revive` and verify's Run B; `send.rs` the `send` module root»

- leaf: .claude/docs/services/viola.md:45   class: omission
  leaf-text: «then `board::read` and `budget.json` through `read_budget_stamped`, the thresholds»
  source: .andromeda/architecture.md:237 «it then makes one `claude agents --json` read (`run::agents::read_here`, `claude` resolved by `run::resolve_program` from its current directory) and joins it by `viola_state::board::join`»
  replace-with: «then `board::read`, then one `claude agents --json` read (`run::agents::read_here`, `claude` resolved by `run::resolve_program` from its current directory) joined by `viola_state::board::join` (a row matching a wrapped item's `child_pid` or `agent_session_id` sets its `status` to `idle` or `busy`; every unjoined row is a read-only unwrapped item; a failed, oversize or malformed read leaves every `status` `unknown` and no unwrapped item), and `budget.json` through `read_budget_stamped`, the thresholds»

- leaf: .claude/docs/services/viola.md:48   class: omission
  leaf-text: «(incl. the `-p/--print` mode `verify` drives)»
  source: .andromeda/architecture.md:299 «and `agents` (chunk 2026-10-16-unwrapped-sessions-on-the-board), which answers `agents --json` from files beside its own executable»
  replace-with: «(incl. the `-p/--print` mode `verify` drives, and the `agents` mode, which answers `agents --json` from `agents-mode`, `agents-rows.json` and the live `sessions/<pid>.json` registry beside its own executable)»

- leaf: .claude/docs/services/viola-state.md:30   class: omission
  leaf-text: «and sums the skip counts of one `read_from` pass per instance;»
  source: .andromeda/architecture.md:237 «a row matching a wrapped item's `child_pid` or `agent_session_id` sets that item's `status` to `idle` or `busy`; every unjoined row is an unwrapped item»
  replace-with: «and sums the skip counts of one `read_from` pass per instance; the agents join (`JoinKeys`, `Observation`, `join`, `Item::unwrapped`) folds one `claude agents --json` reading in: a row matching a wrapped item's `child_pid` or `agent_session_id` sets that item's `status` to `idle` or `busy`, and every unjoined row is an unwrapped item;»

- leaf: .claude/docs/services/viola-state.md:6   class: omission
  leaf-text: «liveness (heartbeat + sysinfo pid/start time)»
  source: .andromeda/registries/contracts/architecture/project-directory-structure.md:69 «`Item::unwrapped`; sync, std only, no Claude shape; liveness's `start_times`, one process-table read; no type»
  replace-with: «liveness (heartbeat + sysinfo pid/start time; `liveness::start_times`, one process-table read)»

- leaf: .claude/docs/workflow.md:49   class: stale
  leaf-text: «all twenty-two on the Unix runners, nineteen on `windows-2025`, the first eighteen and `resume-session`»
  source: .andromeda/registries/contracts/architecture/ci-cd-approach.md:19 «all twenty-three on the Unix runners, twenty on `windows-2025`, the first eighteen, `resume-session` and `agents-join`;»
  replace-with: «all twenty-three on the Unix runners, twenty on `windows-2025`, the first eighteen, `resume-session` and `agents-join`»
