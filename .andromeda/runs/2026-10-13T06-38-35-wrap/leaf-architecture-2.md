group: architecture-2
read: .claude/docs/services/viola-state.md · 49 · «- `.andromeda/architecture.md` §Standard Contracts (snapshot»
read: .claude/docs/services/viola-ui.md · 42 · «- `.claude/rules/api.md` · `.claude/rules/frontend.md` · `.c»
read: .claude/docs/services/viola.md · 58 · «- `.andromeda/architecture.md` · `.claude/rules/api.md` · `.»
read: .claude/docs/stack.md · 70 · «5. Commit as `chore: bump {tool} to {version}`»
read: .claude/docs/workflow.md · 67 · «This project uses a **reference-based** CLAUDE.md architectu»
read: .claude/rules/events.md · 39 · «_This section is owned by `/wrap-session`. setup-project pre»

- leaf: .claude/rules/events.md:34   class: stale
  leaf-text: «No surface shows the counts yet.»
  source: .andromeda/architecture.md:50 «`viola list` is the first surface that shows them (chunk 2026-10-11-the-board-viola-list): `list --json`'s `skipped` `{unknown_kinds, unknown_fields, torn_lines}`»
  replace-with: «`viola list` is the first surface that shows them: `list --json`'s `skipped` `{unknown_kinds, unknown_fields, torn_lines}` and the BAY line's `skipped a - b - c`, each summed over every scanned instance, gone ones included.»

- leaf: .claude/docs/services/viola.md:6   class: stale
  leaf-text: «called by `run`'s start refusals and by `viola verify`»
  source: .andromeda/registries/contracts/architecture/project-directory-structure.md:21 «called by `run`, `revive`, `verify`, `send`, `wait`, `last`, `list`, `answer`, `pause`, `release` and the `main` catch site»
  replace-with: «called by `run`, `revive`, `verify`, `send`, `wait`, `last`, `list`, `answer`, `pause`, `release` and the `main` catch site»

- leaf: .claude/docs/services/viola.md:34   class: stale
  leaf-text: «(`role_of` files every first word but `run`, `hook` and a leading `-` flag under `cli`: `send`, `wait`, `last`, `list`, `answer`, `pause`, `release`, `verify`)»
  source: .andromeda/architecture.md:139 «`src/main.rs` `role_of` files every first word but `run`, `revive`, `hook` and a leading `-` flag under `cli`»
  replace-with: «(`role_of` files every first word but `run`, `revive`, `hook` and a leading `-` flag under `cli`: `send`, `wait`, `last`, `list`, `answer`, `pause`, `release`, `verify`; `revive` is a wrapper beside `run`, and its start arm logs as process `run`)»

- leaf: .claude/docs/services/viola.md:43   class: stale
  leaf-text: «exits 0/1/2/12/13/20/21»
  source: .andromeda/architecture.md:136 «then `human-typing`, `unverified-cli`, `not-delivered`/`unknown-dialog`»
  replace-with: «exits 0/1/2/10/12/13/20/21»

- leaf: .claude/docs/services/viola.md:11   class: omission
  leaf-text: «libc (Unix only: `backtrace` + `dladdr` for the raw panic frames).»
  source: .andromeda/registries/contracts/architecture/crate-dependency-direction.md:11 «tempfile (`=3.27.0`, workspace-pinned: `viola verify`'s Run A temp dir only)»
  replace-with: «libc (Unix only: `backtrace` + `dladdr` for the raw panic frames); tempfile (`=3.27.0`, workspace-pinned: `viola verify`'s Run A temp dir only).»

- leaf: .claude/docs/services/viola.md:20   class: omission
  leaf-text: «→ exclusive endpoint bind (taken → exit 1 `squatted-name`) → first snapshot (with `endpoint`) + heartbeat → `wheel` + `budget-gate` events → child spawn»
  source: .andromeda/architecture.md:93 «→ endpoint bind → the budget gate at start (span `run.budget_eval` with `trigger: start`; it reads `budget.json`, restores no override and writes nothing;»
  replace-with: «→ exclusive endpoint bind (taken → exit 1 `squatted-name`) → the budget gate at start (span `run.budget_eval`, `trigger: start`: reads `budget.json`, restores no override, writes nothing) → first snapshot (with `endpoint`; its `budget_paused` that verdict) + heartbeat (the per-session settings override, Unix only, is rewritten after the first snapshot and before the spawn) → `wheel` + `budget-gate` events → child spawn»

- leaf: .claude/docs/services/viola.md:45   class: omission
  leaf-text: «- **Wait / last verbs:**»
  source: .andromeda/architecture.md:291 «CLI `list` reads `<home>/instances/` (`viola_state::board::scan`, then `board::read`) and `budget.json` (through `read_budget_stamped`) and contacts no wrapper»
  replace-with: «- **List verb:** `src/cmd/list.rs` (`viola list [--json]`: caller-side, no channel frame and no wrapper contacted; `viola_state::board::scan` — the strict-modes check of the home, `instances/` and each instance before any log, whose refusal refuses the whole board with exit 21 and no log — then `board::read` and `budget.json` through `read_budget_stamped`, the thresholds through `run::budget::read_thresholds` at each run; the `--json` `{items, budget, skipped}` document or the human board as one stdout write; exits 0 / 21 / 1 / 2); **Wait / last verbs:**»

- leaf: .claude/docs/services/viola-state.md:6   class: stale
  leaf-text: «`ledger/stamps.json` reads, `config.json` parsing, and»
  source: .andromeda/architecture.md:400 «and by `viola list` at each run through the same `run::budget::read_thresholds`»
  replace-with: «`ledger/stamps.json` reads (`config.json` is read by the root bin, `run::budget::read_thresholds`, not here), and»

- leaf: .claude/docs/services/viola-state.md:11   class: omission
  leaf-text: «windows-sys (DACL APIs, Windows) with strict-modes.»
  source: .andromeda/registries/contracts/architecture/crate-dependency-direction.md:5 «libc (Unix only: the `strict` module's owner + mode check) and windows-sys (Windows only: the `strict` module's owner + DACL check»
  replace-with: «windows-sys (Windows only: the `strict` module's owner + DACL check, and the protected user + SYSTEM DACL `fs::create_private_dir` gives the topmost folder it creates outside the user's profile, `Win32_UI_Shell`); libc (Unix only: the `strict` module's owner + mode check); dev-dependency tracing-subscriber (unit-test line capture only, no product edge).»

- leaf: .claude/docs/services/viola-ui.md:32   class: tree
  leaf-text: «- **Router, middleware, handlers:** `crates/viola-ui/src/`»
  source: .andromeda/architecture.md:378 «Landed so far: `viola`, `viola-core`, `viola-pty`, `viola-channel`, `viola-agent-claude`, `viola-state`, `viola-e2e`.»
  replace-with: «- **Router, middleware, handlers:** `crates/viola-ui/src/` (not created yet: the crate is created by the first chunk that consumes it)»

- leaf: .claude/docs/services/viola-ui.md:33   class: tree
  leaf-text: «- **Embedded page:** `crates/viola-ui/assets/`»
  source: .andromeda/architecture.md:378 «Landed so far: `viola`, `viola-core`, `viola-pty`, `viola-channel`, `viola-agent-claude`, `viola-state`, `viola-e2e`.»
  replace-with: «- **Embedded page:** `crates/viola-ui/assets/` (not created yet: it lands with the `viola-ui` crate)»

- leaf: .claude/docs/workflow.md:49   class: stale
  leaf-text: «CI uses the fake agent (including `viola verify` against its print mode)»
  source: .andromeda/registries/contracts/architecture/ci-cd-approach.md:20 «`viola verify` runs in CI only against the fake agent: its `-p/--print` mode and its interactive `--screens --turn-stop --trusted-root --dialogs --framing` modes»
  replace-with: «CI uses the fake agent (including `viola verify` against its `-p/--print` mode and its interactive `--screens --turn-stop --trusted-root --dialogs --framing` modes: all four typed runs and all seventeen ledger rows)»

- leaf: .claude/docs/stack.md:13   class: omission
  leaf-text: «for the `TerminateProcess` kill fallback and the host console's raw/VT modes (`Win32_System_Console`);»
  source: .andromeda/architecture.md:17 «the host console's raw/VT modes (`Win32_System_Console`) and its input read (`ReadConsoleW`, behind `viola_pty::host_stdin()`)»
  replace-with: «for the `TerminateProcess` kill fallback, the host console's raw/VT modes (`Win32_System_Console`) and its input read (`ReadConsoleW`, behind `viola_pty::host_stdin()`);»

- leaf: .claude/docs/stack.md:13   class: omission
  leaf-text: «(Windows x64; the inbox ConPTY is the fail-open fallback)»
  source: .andromeda/architecture.md:17 «the inbox ConPTY is the fail-open fallback and the host on other Windows targets»
  replace-with: «(Windows x64; the inbox ConPTY is the fail-open fallback and the host on other Windows targets)»

- leaf: .claude/docs/stack.md:18   class: omission
  leaf-text: «(`viola_state::fs::replace_private`, mode set on the temp file before the write)»
  source: .andromeda/architecture.md:22 «(`viola_state::fs::replace_private`: the mode is set on the temp file before the write, then `sync_all` and `persist`)»
  replace-with: «(`viola_state::fs::replace_private`: the mode is set on the temp file before the write, then `sync_all` and `persist`)»
