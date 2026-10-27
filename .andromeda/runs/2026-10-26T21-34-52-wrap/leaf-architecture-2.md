group: architecture-2
read: .claude/docs/services/viola-mcp.md · 39 · «- `.andromeda/architecture.md` [MCP] · `.claude/rules/api.md»
read: .claude/docs/services/viola-pty.md · 47 · «- `.andromeda/architecture.md` [PTY] · `.claude/docs/gotchas»
read: .claude/docs/services/viola-state.md · 49 · «- `.andromeda/architecture.md` §Standard Contracts (snapshot»
read: .claude/docs/services/viola.md · 58 · «- `.andromeda/architecture.md` · `.claude/rules/api.md` · `.»

- leaf: .claude/docs/services/viola-mcp.md:22   class: omission
  leaf-text: «a frame tool (`send`, `wait`, `last`, `answer`) whose connected server is not the recorded wrapper»
  source: .andromeda/architecture-detail/conventions.md:77 «A frame tool (`send`, `wait`, `last`, `answer`) first runs `viola_state::strict::check_instance` on the home and the target instance, before `live_endpoint`»
  replace-with: «a frame tool (`send`, `wait`, `last`, `answer`) first runs `viola_state::strict::check_instance` on the home and the target instance, before `live_endpoint` (a refusal returns the `instance-unreachable` result with `detail: null`, text `refused: instance-unreachable`, and writes no frame and no line); a frame tool whose connected server is not the recorded wrapper»

- leaf: .claude/docs/services/viola-state.md:6   class: stale
  leaf-text: «all from its one `read_snapshot` call)»
  source: .andromeda/architecture-detail/standard-contracts.md:127 «all from its one `replay::read_snapshot_or_replay` call and only on a real snapshot (a replayed or absent one, or a failed read, gives `None`)»
  replace-with: «all from its one `replay::read_snapshot_or_replay` call and only on a real snapshot: a replayed or absent one, or a failed read, gives `None`)»

- leaf: .claude/docs/services/viola-state.md:6   class: stale
  leaf-text: «read by `viola revive`'s preflight through `read_snapshot_or_replay`;»
  source: .andromeda/architecture-detail/standard-contracts.md:24 «is library code in `viola-state` with six product readers, each after the instance's strict-modes check has passed: `viola revive`'s preflight takes it in its second reading»
  replace-with: «read through `read_snapshot_or_replay` by six product readers, each after the instance's strict-modes check: `viola revive`'s preflight (one read for both its collision check and its recorded directory), `viola list`'s `board::read`, `liveness::live_endpoint`, `viola hook`'s `connect`, `hook statusline`'s `serve` and `run`'s collision check; no product code under `src/` calls `read_snapshot`;»

- leaf: .claude/docs/services/viola-state.md:6   class: stale
  leaf-text: «the instance check `strict::check_instance` (the home, `instances/`, the instance directory, its `snapshot.json` and `events.ndjson`, each that exists)»
  source: .andromeda/architecture-detail/occupied-resources.md:136 «runs `viola_state::strict::check_instance` over the home (the read rule), `instances/` (the write rule), the instance directory (the read rule), its `diagnostics/` (the read rule)»
  replace-with: «the instance check `strict::check_instance` (the home (the read rule), `instances/` (the write rule), the instance directory (the read rule), its `diagnostics/` (the read rule), its `snapshot.json` (the write rule) and its `events.ndjson` (the write rule on Unix, the read rule on Windows), each that exists, a folder before what it holds)»

- leaf: .claude/docs/services/viola-state.md:6   class: omission
  leaf-text: «the home strict-modes check (Unix modes; Windows owner + DACL) with 0700/0600 creation.»
  source: .andromeda/architecture-detail/occupied-resources.md:143 «`viola_state::strict` holds two rules. The read rule is the write rule plus, on Unix, a refusal of any group or other bit (`mode & 0o077`) and, on Windows, a refusal of any allow ACE»
  replace-with: «the home strict-modes check (Unix modes; Windows owner + DACL) with 0700/0600 creation, in two rules — the write rule, and the read rule (`unix_read_verdict`, `windows_read_verdict`: on Unix no group or other bit, on Windows no allow ACE granting a right in `READ_RIGHTS` to a SID `trusted_sid` rejects) —, and beside `check_instance` the checks `check_home_tree(home, name, key)` (the home; with a name `check_instance`'s set; `ledger/`, `bin/`, `plugin/`; with a key `bin/<key>/`, the pinned `viola`, `plugin/<key>/`, its `hooks/` and `.claude-plugin/`; each that exists; refusing as `TreeRefused::HomeLevel` or `TreeRefused::InstanceLevel`; called by `run`'s and `revive`'s starts and by `verify` with no name and no key), `check_diagnostics` (a symbolic link refused; run by `open_role_file` / `write_detail` before anything is narrowed), `check_present` and `check_stamps`.»

- leaf: .claude/docs/services/viola-state.md:21   class: omission
  leaf-text: «is the ONE writer path of `ledger/stamps.json` (0700 `ledger/`,»
  source: .andromeda/architecture-detail/standard-contracts.md:41 «`viola_state::stamps::update_stamps` runs `strict::check_stamps` on the home before it creates `ledger/` and again under its lock before its capped read; a refusal reads and writes nothing and returns»
  replace-with: «is the ONE writer path of `ledger/stamps.json` (`strict::check_stamps` on the home before it creates `ledger/` and again under its lock before its capped read, a refusal reading and writing nothing and returning `StampsError::StrictModes` — its error `StampsError` (`StrictModes` · `State`), each variant with a fixed message —; 0700 `ledger/`,»

- leaf: .claude/docs/services/viola-state.md:22   class: retired
  leaf-text: «(not landed yet — Epoch 6 "Home and code-bearing file integrity"; until then, ratified interim gaps: `hook <event>`'s snapshot read, and `verify`'s `update_stamps` reading the stamps, security-plan Decisions Log `2026-09-28` — `run`'s version gate reads them through `stamps::read_stamps_strict`, the `strict` module's `check_stamps` first, a refusal `cli_verified:false`; `send` / `wait` / `last` / `answer` / `pause` / `release`'s liveness-only snapshot reads, the founder's 2026-10-04 rulings; and the Windows x64 ConPTY companions loaded under the held-handle re-hash alone, Decisions Log `2026-09-29`)»
  source: .andromeda/architecture-detail/occupied-resources.md:135 «These entry points check these files before they use them: `viola list`; `viola revive`; `viola run`'s start (through `check_home_tree` with its name); `viola hook`'s statusline, event and dialog»
  replace-with: «(`run`'s and `revive`'s starts and `verify` through `check_home_tree`, `verify`'s `update_stamps` through `check_stamps`, `run`'s version gate through `stamps::read_stamps_strict` (the `strict` module's `check_stamps` first, a refusal `cli_verified:false`), `hook`'s event, dialog and statusline paths through `trusted`, the CLI frame verbs `send` / `wait` / `last` / `answer` / `pause` / `release` and `link` / `unlink` through `client::refuse_untrusted`, `viola mcp`'s frame tools through `check_instance`, and on Windows x64 the ConPTY companions through `check_present` before the held-handle re-hash)»

- leaf: .claude/docs/services/viola-state.md:24   class: omission
  leaf-text: «`pin_companions(&Pinned, subdir, &[Companion])` — each file written if absent»
  source: .andromeda/architecture-detail/established-decisions.md:256 «On Windows x64, before the held open, `pin_companions` runs the owner/DACL check (`viola_state::strict::check_present`) on `bin/<version>-<hash>/conpty/` and on each companion that exists:»
  replace-with: «`pin_companions(&Pinned, subdir, &[Companion])` — first the owner/DACL check (`strict::check_present`) on `bin/<version>-<hash>/conpty/` and on each companion that exists (a refusal is `PinError::StrictModes`, writes nothing, and the child runs on the inbox ConPTY with `sideload_fallback` `strict-modes-failed`), then each file written if absent»

- leaf: .claude/docs/services/viola-state.md:24   class: omission
  leaf-text: «`pin_exe` / `verify` unchanged;»
  source: .andromeda/architecture-detail/established-decisions.md:255 «`viola_state::pin::pin_key(bytes)` names the pinned copy's key, `<version>-<16 hex>`, in one place, and `pin_bytes(home, bytes)` pins bytes already read; `pin_exe(home, exe)` delegates to it.»
  replace-with: «`pin_key(bytes)` names the pinned copy's key, `<version>-<16 hex>`, in one place, and `pin_bytes(home, bytes)` pins bytes already read (`run`'s start pins the very bytes its home-tree check keyed); `pin_exe(home, exe)` delegates to it; `verify` unchanged;»

- leaf: .claude/docs/services/viola-state.md:28   class: stale
  leaf-text: «whose two product callers are `viola revive`'s preflight and `viola list`'s `board::read`»
  source: .andromeda/architecture-detail/standard-contracts.md:24 «The other four are `viola_state::liveness::live_endpoint` (Session liveness, below); `viola hook`'s `connect`, which reads a replayed or absent snapshot as unreachable, so the hook fails open;»
  replace-with: «whose six product callers are `viola revive`'s preflight, `viola list`'s `board::read`, `liveness::live_endpoint`, `viola hook`'s `connect` (a replayed or absent snapshot unreachable, so the hook fails open), `hook statusline`'s `serve` (no `statusline_command` in a replayed snapshot) and `run`'s collision check (a replayed snapshot judged by the heartbeat alone)»

- leaf: .claude/docs/services/viola.md:6   class: omission
  leaf-text: «prints the step lines and `stamped` summary and writes the stamps through `viola_state::stamps::update_stamps`;»
  source: .andromeda/architecture-detail/established-decisions.md:216 «Before its `cli` log opens, `viola verify` runs `viola_state::strict::check_home_tree(home, None, None)` (the home, `ledger/`, `bin/` and `plugin/`, each that exists); a refusal prints the home pair»
  replace-with: «prints the step lines and `stamped` summary and writes the stamps through `viola_state::stamps::update_stamps` (its `StampsError::StrictModes` refusal printing the home-state pair, exit 1); before its `cli` log opens it runs `viola_state::strict::check_home_tree(home, None, None)` (the home, `ledger/`, `bin/` and `plugin/`, each that exists), a refusal printing the home pair and exiting 1 with no step line, no summary and no line;»

- leaf: .claude/docs/services/viola.md:6   class: omission
  leaf-text: «the panic hook, the detail writer, the catch-site reporter `report_internal_error`)»
  source: .andromeda/architecture-detail/occupied-resources.md:129 «`open_role_file` and `write_detail` (`src/obs.rs`) run `strict::check_diagnostics` on an existing `diagnostics/` folder and on the role or detail file before `ensure_private_dir` narrows anything:»
  replace-with: «the panic hook, the detail writer, the catch-site reporter `report_internal_error`; `open_role_file` and `write_detail` run `strict::check_diagnostics` on an existing `diagnostics/` folder and on the role or detail file before `ensure_private_dir` narrows anything — a symbolic link refused, a folder to the read rule, a file to the read rule on Unix and the write rule on Windows, an absent path passing — and a refusal is a failed open: the writer ends at `std::io::sink`, a detail line is dropped)»

- leaf: .claude/docs/services/viola.md:20   class: omission
  leaf-text: «`run` start order (the home is canonicalised by `canonical_home` once the wrapper log has created it, before the collision check, the pin and the endpoint bind):»
  source: .andromeda/architecture-detail/established-decisions.md:229 «Before its wrapper log opens, every start (`run`'s and a revived one) checks the home tree: `checked_exe` reads the exe's bytes once and takes their pin key»
  replace-with: «`run` start order (before the wrapper log opens, every start, `run`'s and a revived one, checks the home tree: `checked_exe` reads the exe's bytes once and takes their pin key, then `viola_state::strict::check_home_tree` checks the home, the instance's set, `ledger/`, `bin/`, `plugin/`, `bin/<key>/`, the pinned copy, `plugin/<key>/` and its `hooks/` and `.claude-plugin/`, each that exists — a refusal is exit 1 with the home pair, or the instance pair for an instance-level refusal, no log opened, no line and no child —, and the pin step pins those same bytes through `pin_bytes`; the home is canonicalised by `canonical_home` once the wrapper log has created it, before the collision check, the pin and the endpoint bind):»

- leaf: .claude/docs/services/viola.md:20   class: stale
  leaf-text: «`outcome` `loaded|hash-mismatch|unreadable|load-failed|not-built`»
  source: .andromeda/architecture-detail/established-decisions.md:224 «`outcome` `loaded` · `hash-mismatch` · `strict-modes-failed` · `unreadable` · `load-failed` · `not-built`, never a refusal»
  replace-with: «`outcome` `loaded|hash-mismatch|strict-modes-failed|unreadable|load-failed|not-built`»

- leaf: .claude/docs/services/viola.md:20   class: omission
  leaf-text: «A `live` or `stale` name → exit 1 (`already-live`); a `gone` one is taken over at once.»
  source: .andromeda/architecture-detail/established-decisions.md:227 «`run`'s collision check takes its caller's one snapshot read (`replay::read_snapshot_or_replay`): over a replayed snapshot, which holds no `pid` or `started_at`, it decides from the heartbeat alone»
  replace-with: «A `live` or `stale` name → exit 1 (`already-live`); a `gone` one is taken over at once. The collision check takes its caller's one snapshot read (`replay::read_snapshot_or_replay`): over a replayed snapshot, which holds no `pid` or `started_at`, it decides from the heartbeat alone, a beat at most `STALE_AFTER` old refusing `already-live` and an older one free; a failed read or no snapshot is free.»

- leaf: .claude/docs/services/viola.md:21   class: omission
  leaf-text: «first the strict-modes check (`trusted`: `check_instance` before the wrapper log opens; a refusal is the state-files pair and exit 1 with no log opened and no line),»
  source: .andromeda/architecture-detail/conventions.md:40 «Exit `1` also covers `viola run`'s start and `viola revive`'s start arm refusing the home tree, checked before the wrapper log opens (`checked_exe`, then `refuse_tree`, in `src/cmd/run.rs`)»
  replace-with: «first the strict-modes check (`trusted`: `check_instance` before the wrapper log opens; a refusal is the state-files pair and exit 1 with no log opened and no line), on the start arm then the home-tree check (`checked_exe`, then `refuse_tree`: the home, the instance's `check_instance` set, `ledger/`, `bin/`, `plugin/`, and for the running binary's pin key `bin/<key>/`, the pinned `viola`, `plugin/<key>/` and its `hooks/` and `.claude-plugin/`, each that exists; a refusal prints the home-state pair, or the instance-state pair when the refused path is the instance's, opens no log, writes no line and starts no child, and under `--json` is the `strict-modes-failed` document),»

- leaf: .claude/docs/services/viola.md:23   class: omission
  leaf-text: «writes `budget.json` only with a `rate_limits` object, then runs the command»
  source: .andromeda/architecture-detail/standard-contracts.md:119 «reads the snapshot through `replay::read_snapshot_or_replay` (a replayed one holds no `statusline_command`, so no command runs), then runs the command.»
  replace-with: «writes `budget.json` only with a `rate_limits` object, reads the snapshot through `replay::read_snapshot_or_replay` (a replayed one holds no `statusline_command`, so no command runs), then runs the command»

- leaf: .claude/docs/services/viola.md:32   class: stale
  leaf-text: «its event and dialog paths share one connect (`read_snapshot`, then `Client::connect_by`, then `verify_server`)»
  source: .andromeda/architecture-detail/established-decisions.md:122 «The event and dialog paths, once their instance check has passed ([Hook Contract]), share one connect: `replay::read_snapshot_or_replay` (a replayed or absent snapshot is an endpoint not reached:»
  replace-with: «its event and dialog paths, once their instance check (`trusted`, before the diagnostics level is read and obs is initialised; a refused or absent instance exits 0 at once with no `config.json` read, no role file, no stdin read, no line, no frame and no SessionEnd append) has passed, share one connect (`replay::read_snapshot_or_replay` — a replayed or absent snapshot is an endpoint not reached: the hook fails open with `channel-unreachable` —, then `Client::connect_by`, then `verify_server`)»

- leaf: .claude/docs/services/viola.md:42   class: omission
  leaf-text: «(`viola pause|release <name> [--json]`: `client::connect`»
  source: .andromeda/architecture-detail/conventions.md:58 «Exit `21` also covers the refused strict-modes check of `send`, `wait`, `last`, `answer`, `pause` and `release` (`client::refuse_untrusted`: `check_instance` on the home and `instances/<name>`»
  replace-with: «(`viola pause|release <name> [--json]`: first `client::refuse_untrusted` (`check_instance` on the home and `instances/<name>`; a refusal exits 21 with the instance-state `unable:` / `hint:` pair, or under `--json` `{"v":1,"error":"instance-unreachable","detail":null}` with stderr empty, nothing under the home opened and no line written), then `client::connect`»

- leaf: .claude/docs/services/viola.md:43   class: omission
  leaf-text: «(`viola answer <name> <dialog_id> [--file] [--json]`: the response»
  source: .andromeda/architecture-detail/conventions.md:58 «Exit `21` also covers the refused strict-modes check of `send`, `wait`, `last`, `answer`, `pause` and `release` (`client::refuse_untrusted`: `check_instance` on the home and `instances/<name>`»
  replace-with: «(`viola answer <name> <dialog_id> [--file] [--json]`: first `client::refuse_untrusted` (the strict-modes instance check of the home and `instances/<name>`; exit 21 with the instance-state pair, or the `instance-unreachable` document under `--json`, no line), then the response»

- leaf: .claude/docs/services/viola.md:44   class: omission
  leaf-text: «(stdin / `--file` under `take(MAX_FRAME + 1)`, client paste check,»
  source: .andromeda/architecture-detail/conventions.md:58 «(`client::refuse_untrusted`: `check_instance` on the home and `instances/<name>`, the verb's first step, after its text read for `send`)»
  replace-with: «(stdin / `--file` under `take(MAX_FRAME + 1)`, then `client::refuse_untrusted` (the strict-modes instance check of the home and `instances/<name>`, the verb's first step after its text read; exit 21 with the instance-state pair, or the `instance-unreachable` document under `--json`, no line), client paste check,»

- leaf: .claude/docs/services/viola.md:45   class: omission
  leaf-text: «over `src/cmd/client.rs` (`connect`, the verified connection»
  source: .andromeda/registries/contracts/architecture/project-directory-structure.md:32 «`refuse_untrusted`, the instance strict-modes check (`check_instance` on the absolute home and `instances/<name>`) that every CLI frame verb (send, wait, last, answer, pause, release)»
  replace-with: «over `src/cmd/client.rs` (`refuse_untrusted`, the instance strict-modes check (`check_instance` on the absolute home and `instances/<name>`) that every CLI frame verb (send, wait, last, answer, pause, release) and `link` / `unlink` run first (refused: exit 21 with the instance pair, or the `instance-unreachable` document under `--json`; nothing opened under the home, no line); `connect`, the verified connection»
