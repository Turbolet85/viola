group: architecture+security-plan
read: .claude/rules/api.md · 41 · «_This section is owned by `/wrap-session`. setup-project pre»

- leaf: .claude/rules/api.md:23   class: omission
  leaf-text: «and that process's OS start time with the snapshot's `pid` and `started_at` from its one snapshot read (»
  source: .andromeda/architecture-detail/standard-contracts.md:63 «Before that comparison each client runs the instance's strict-modes check, and a refused check writes no frame and no line: `client::refuse_untrusted` for `send`, `wait`, `last`, `answer`, `pause`»
  replace-with: «and that process's OS start time with the snapshot's `pid` and `started_at` from its one snapshot read, after the instance's strict-modes check (`client::refuse_untrusted` for the eight CLI verbs, the hook's `trusted` for `hook.event` and `hook.dialog`, `strict::check_instance` in the MCP frame tools; a refusal writes no frame and no line), that read being `replay::read_snapshot_or_replay` and only a real snapshot going on to the comparison (»

- leaf: .claude/rules/api.md:28   class: omission
  leaf-text: «and their refused strict-modes check on the home and `instances/<driven>` prints exactly»
  source: .andromeda/architecture-detail/standard-contracts.md:111 «`viola send`, `viola wait`, `viola last`, `viola answer`, `viola pause` and `viola release` print the same `{"v":1,"error":"instance-unreachable","detail":null}` on stdout under `--json` (exit 21)»
  replace-with: «and their refused strict-modes check on the home and `instances/<driven>`, like the refused check of `viola send` / `wait` / `last` / `answer` / `pause` / `release` on the home and `instances/<name>` (each through `client::refuse_untrusted`), prints exactly»

- leaf: .claude/rules/api.md:29   class: omission
  leaf-text: «1 internal / a `run` start refusal, a `revive` strict-modes or preflight refusal, a `verify` failing row, refusal or Ctrl-C / TERM stop»
  source: .andromeda/architecture-detail/conventions.md:40 «Exit `1` also covers `viola run`'s start and `viola revive`'s start arm refusing the home tree, checked before the wrapper log opens (`checked_exe`, then `refuse_tree`, in `src/cmd/run.rs`)»
  replace-with: «1 internal / a `run` start refusal (its refused home-tree check before the wrapper log included: the home or the instance pair, no log, no child), a `revive` strict-modes, home-tree or preflight refusal, a `verify` failing row, refusal (its refused home-tree check of the home, `ledger/`, `bin/` and `plugin/` before its `cli` log, and `update_stamps`'s `StampsError::StrictModes`, each the home-state pair, included) or Ctrl-C / TERM stop»

- leaf: .claude/rules/api.md:29   class: stale
  leaf-text: «the client's own exit-13 refusals, and for `link` / `unlink` the instance check of the driven instance (exit 21), come first»
  source: .andromeda/architecture-detail/conventions.md:52 «The client's own refusals (exit 13) and the strict-modes instance check (exit 21, `client::refuse_untrusted`: the target instance for `send`, `wait`, `last`, `answer`, `pause` and `release`»
  replace-with: «the client's own exit-13 refusals and the strict-modes instance check (exit 21, `client::refuse_untrusted`: the target instance for `send`, `wait`, `last`, `answer`, `pause` and `release`, the driven instance for `link` / `unlink`) come first»

- leaf: .claude/rules/api.md:29   class: omission
  leaf-text: «and `viola link` / `viola unlink`'s refused instance check of the driven instance (`check_instance` on the home and `instances/<driven>`, before any role file or log opens»
  source: .andromeda/architecture-detail/conventions.md:58 «Exit `21` also covers the refused strict-modes check of `send`, `wait`, `last`, `answer`, `pause` and `release` (`client::refuse_untrusted`: `check_instance` on the home and `instances/<name>`»
  replace-with: «and `viola link` / `viola unlink`'s refused instance check of the driven instance and `send` / `wait` / `last` / `answer` / `pause` / `release`'s of the target instance (`client::refuse_untrusted`: `check_instance` on the home and `instances/<driven>` or `instances/<name>`, the verb's first step, after its text read for `send`, before any role file or log opens»

- leaf: .claude/rules/api.md:30   class: retired
  leaf-text: «Interim ratified gaps until Epoch 6: `viola hook <event>` shape-checks `VIOLA_DIR` only (`hook statusline` canonicalises its instance directory and runs `check_instance` before its log;»
  source: .andromeda/security-plan-detail/input-validation.md:30 «`viola hook <event>` on its nine `HookEvent` arms and `hook statusline`, each outside `--capture`, canonicalise the instance directory and run `viola_state::strict::check_instance` (`canonical` and»
  replace-with: «`viola hook <event>`'s event and dialog paths and `hook statusline` canonicalise the instance directory and run `check_instance` (`canonical` and `trusted`, shared in `src/cmd/hook.rs`) before the diagnostics level, obs init and any stdin, a malformed, absent or refused instance a silent exit 0 (`hook statusline` before its log;»

- leaf: .claude/rules/api.md:30   class: retired
  leaf-text: «`verify` reads `ledger/stamps.json` before strict-modes — security-plan Decisions Log `2026-09-28`;»
  source: .andromeda/security-plan-detail/input-validation.md:54 «`viola verify`'s `update_stamps` runs `strict::check_stamps` before it creates `ledger/` and again under its lock before the capped read: a refusal is `StampsError::StrictModes`»
  replace-with: «`viola verify` runs `check_home_tree` on the home, `ledger/`, `bin/` and `plugin/` before its `cli` log (a refusal the home pair, exit 1), and its `update_stamps` runs `strict::check_stamps` before it creates `ledger/` and again under its lock before the capped read (a refusal `StampsError::StrictModes`, reading and writing nothing, mapped to the home pair);»

- leaf: .claude/rules/api.md:30   class: retired
  leaf-text: «read `snapshot.json` without strict-modes after a liveness-only pre-check (`live_endpoint` → `LiveEndpoint { endpoint, pid, started_at }` from one `read_snapshot`)»
  source: .andromeda/security-plan-detail/input-validation.md:31 «`viola send` (after its text read), `wait`, `last`, `answer`, `pause` and `release` each run `client::refuse_untrusted` first: `viola_state::strict::check_instance` on the home and `instances/<name>`»
  replace-with: «run `client::refuse_untrusted` first (`check_instance` on the home and `instances/<name>`, after its text read for `send`; a refusal exit 21, nothing opened under the home, no line), then a liveness-only pre-check (`live_endpoint` → `LiveEndpoint { endpoint, pid, started_at }` from one `replay::read_snapshot_or_replay`, only on a real snapshot)»

- leaf: .claude/rules/api.md:30   class: retired
  leaf-text: «the gap now only its strict-modes half, owed to "Home and code-bearing file integrity", the client euid check and macOS owed to "Unix endpoint and home hardening"»
  source: .andromeda/security-plan.md:114 «the client runs the instance's strict-modes check, a refusal writing no frame and no line, and only then verifies the server against its one snapshot read»
  replace-with: «with no dated gap, the client euid check and macOS owed to "Unix endpoint and home hardening"»

- leaf: .claude/rules/api.md:30   class: retired
  leaf-text: «(`viola_state::liveness::live_endpoint`, else `{"error":"instance-unreachable","detail":null}`; the eighth dated gap, the founder's own answer, `inputs#I2` Q1 of chunk 2026-10-17-mcp-server-for-drivers)»
  source: .andromeda/security-plan-detail/input-validation.md:31 «`viola mcp`'s `send` / `wait` / `last` / `answer` tools run `check_instance` before `live_endpoint`; a refusal returns `instance-unreachable` with `detail: null`, no frame and no line.»
  replace-with: «(`check_instance` on the home and the target instance first, then `viola_state::liveness::live_endpoint`, a refusal of either `{"error":"instance-unreachable","detail":null}` with no frame)»

- leaf: .claude/rules/api.md:34   class: omission
  leaf-text: «(on `send` with one client-side `send-refused` line), before any frame.»
  source: .andromeda/architecture-detail/conventions.md:77 «A frame tool (`send`, `wait`, `last`, `answer`) first runs `viola_state::strict::check_instance` on the home and the target instance, before `live_endpoint`: a refusal returns»
  replace-with: «(on `send` with one client-side `send-refused` line), before any frame; a frame tool runs `viola_state::strict::check_instance` on the home and the target instance before `live_endpoint`, a refusal being `isError:true` `{"error":"instance-unreachable","detail":null}`, text `refused: instance-unreachable`, with no frame and no line.»
