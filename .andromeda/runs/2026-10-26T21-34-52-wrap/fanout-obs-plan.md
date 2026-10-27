proposals:
  - detector: D-obs-instrumentation
    severity: warning
    section: '§3 → OTel SDK init'
    change: 'Per-role anchors, `run`: before the log init, `run` and `viola revive`''s start arm run `checked_exe` (the exe''s bytes read once, `pin_key`, then `strict::check_home_tree(home, Some(name), Some(key))`: the home to the read rule, the `check_instance` set, `ledger/`, `bin/`, `plugin/`, `bin/<key>/`, the pinned copy, `plugin/<key>/` with its `hooks/` and `.claude-plugin/`, each that exists); a refusal (`TreeRefused`) opens no log, writes no `process-start`, `process-exit` or any other line, starts no child and leaves the modes as found, exit 1 with the home pair (`HomeLevel`) or the instance pair (`InstanceLevel`) through `refuse_tree`; only after a passed check do steps 1–6 run, still before `run.collision_check`.'
    sidecar: 'OTel SDK init `run` anchor: the home-tree check (`checked_exe` / `check_home_tree`) runs before the log init of `run` and of revive''s start; a refusal writes no line (chunk 2026-10-26-home-and-code-bearing-file-integrity).'
    rationale: 'Report Changes → Symbols: `src/cmd/run.rs` `checked_exe` (189-200) and `refuse_tree` (202-215), called by `run` (166-172) and revive''s start (revive.rs:125-135) before `open_wrapper_log`, exit 1, no log, no line, no child; Count 9; the founder''s P4 Q2 (`inputs#I3`): a refused check writes no line. The anchor names only revive''s `check_instance` before the log init.'
    basis: 'src/cmd/run.rs:189-215'
  - detector: D-obs-instrumentation
    severity: warning
    section: '§3 → OTel SDK init'
    change: 'Per-role anchors, `hook`: every arm without `--capture` (the event and dialog paths as well as `viola hook statusline`) takes `instance_of(…).and_then(canonical).filter(trusted)` (`src/cmd/hook.rs`, `check_instance` on the canonicalised instance) BEFORE `read_diagnostics_level` and the log init; a refused or absent instance, a `VIOLA_DIR` naming an instance directory that does not exist included, returns exit 0 at once: no `config.json` read, no role file, no stdin read, no line, no frame, no SessionEnd append; the two `--capture` arms stay ahead of it unchanged, and `FAKE_AGENT_HOOK_PANIC` keeps its place after the log init.'
    sidecar: 'OTel SDK init `hook` anchor: the event and dialog arms take statusline''s instance check before the log init; a refused or absent instance writes nothing (chunk 2026-10-26-home-and-code-bearing-file-integrity).'
    rationale: 'Report Changes → Symbols, `src/cmd/hook.rs`: `canonical` (75-81) and `trusted` (83-86) moved from statusline.rs and shared by both arms, taken before `read_diagnostics_level` / `viola_obs_init`; Deviation 5 and Count 23 (a `VIOLA_DIR` naming no existing directory writes nothing). The anchor states the pre-init check for `viola hook statusline` only.'
    basis: 'src/cmd/hook.rs:75-86'
  - detector: D-obs-instrumentation
    severity: warning
    section: '§3 → OTel SDK init'
    change: 'Per-role anchors, `cli`: `send`, `wait`, `last`, `answer`, `pause` and `release` take `viola link` / `viola unlink`''s form through the one shared `client::refuse_untrusted` (`check_instance` on the absolute home and `instances/<name>`, before `client::start` and any log; `send` after its text read): a refusal opens no role file, writes no `process-start`, no `process-exit` and no line in any log, and leaves the modes as found (exit 21, the fixed instance-state pair, or `{"v":1,"error":"instance-unreachable","detail":null}` under `--json`).'
    sidecar: 'OTel SDK init `cli` anchor: the six frame verbs share link''s pre-log instance check (`client::refuse_untrusted`), no line on a refusal (chunk 2026-10-26-home-and-code-bearing-file-integrity).'
    rationale: 'Report Changes → Symbols, `src/cmd/client.rs` `refuse_untrusted` (20-41), called first by send (send.rs:190-192), wait (43-45), last (30-32), answer (94-96), pause (32-34), release (35-37), `link_call` moved onto it; Coverage of new surfaces: instrumentation n/a (no line by the founder''s word, `inputs#I3` Q2). The anchor names only link / unlink.'
    basis: 'src/cmd/client.rs:20-41'
  - detector: D-obs-instrumentation
    severity: warning
    section: '§3 → OTel SDK init'
    change: 'Per-role anchors, `cli`: `viola verify` runs `strict::check_home_tree(home, None, None)` (the home to the read rule, `ledger/`, `bin/`, `plugin/`) before its `cli` log: a refusal is exit 1 with the home pair (`refuse_home`), no step line, no summary and no line; `update_stamps` runs `check_stamps` before `create_private_dir(ledger/)` and again under its lock, and its `StampsError::StrictModes` maps to the same pair with nothing read or written.'
    sidecar: 'OTel SDK init `cli` anchor: `verify`''s home-tree check before its log and `update_stamps`'' pre-read refusal, no line (chunk 2026-10-26-home-and-code-bearing-file-integrity).'
    rationale: 'Report Changes → Symbols, `src/cmd/verify.rs` (64-70, `refuse_home` 95-99, `StampsError::StrictModes` 288-296) and `viola_state::stamps` (`StampsError` 37-44); Deviation 1; Coverage of new surfaces: instrumentation n/a (no log opened).'
    basis: 'src/cmd/verify.rs:64-70'
  - detector: D-obs-instrumentation
    severity: warning
    section: '§1 Obs Scope Summary → Telemetry surfaces (cli `viola hook <event>` and `viola hook statusline`)'
    change: 'Exporter: … without `VIOLA_NAME` the hook is a silent no-op, and so is a hook whose instance the instance check refuses or whose `VIOLA_DIR` names an instance directory that does not exist (exit 0, no file opened, no line, no frame) …'
    sidecar: '§1 hook surface: a refused or non-existent instance is the same silent no-op as no `VIOLA_NAME` (chunk 2026-10-26-home-and-code-bearing-file-integrity).'
    rationale: 'Report Changes → Symbols, `src/cmd/hook.rs`: a refused or absent instance returns exit 0 at once with no line; a `VIOLA_DIR` naming a missing instance directory canonicalises to no session (Deviation 5, Count 23).'
    basis: 'src/cmd/hook.rs:75-86'
  - detector: D-obs-instrumentation
    severity: warning
    section: '§1 Obs Scope Summary → Telemetry triggers (security plan, Vector 7)'
    change: 'Required telemetry signal: … a strict-modes failure has a line only on the `ledger-stamps` read in `run`''s version gate and on a `conpty/` companion its owner/DACL check refuses (`process-start.sideload_fallback:"strict-modes-failed"`, the child on the inbox ConPTY, exit unchanged); every other refused check writes no line: the six frame verbs and `viola link` / `unlink`, the hook event, dialog and statusline arms, the `viola mcp` frame tools, `verify`, `run` / `revive`''s home-tree check, and a refused `diagnostics/` open (the writer ends at `std::io::sink`, a detail line dropped).'
    sidecar: '§1 Vector 7: the companions'' `strict-modes-failed` fallback is a second strict-modes line; the new no-line refusal sites listed (chunk 2026-10-26-home-and-code-bearing-file-integrity).'
    rationale: 'Report Changes → Schema / config (`sideload_fallback` gains `strict-modes-failed`), Symbols (`pin_companions` → `PinError::StrictModes`, run.rs:510; `src/obs.rs` `check_diagnostics` 69-76), Count 7 and Count 20 (each new no-line refusal site); Expected amendments name the §1 Vector 7 list.'
    basis: 'src/cmd/run.rs:510'
  - detector: D-obs-instrumentation
    severity: warning
    section: '§3 Observability Harness Contract'
    change: '… the root bin''s human-stderr refusal writer, called by `run` (7 sites), `verify` (6), `list` (2), `revive` (1), `plugin install` (1) and `client` (1, `client::refuse_untrusted`, the one site `viola link` / `viola unlink` and the six frame verbs `send`, `wait`, `last`, `answer`, `pause`, `release` share), as re-derived by `grep -rcE ''human::refuse\('' src` at chunk 2026-10-26-home-and-code-bearing-file-integrity; …'
    sidecar: '§3 refusal writer: `human::refuse(` sites 15 → 18 by `grep -rcE` (run 7, verify 6, client 1 replacing link 1) (chunk 2026-10-26-home-and-code-bearing-file-integrity).'
    rationale: 'Report Count 10 (pattern `refuse-count`): obs-plan.md:311 per-file figures stale; at HEAD `grep -rcE ''human::refuse\('' src` gives verify 6, run 7, list 2, revive 1, plugin 1, client 1 against the base''s link 1, list 2, plugin 1, revive 1, run 5, verify 5.'
  - detector: D-obs-instrumentation
    severity: warning
    section: '§3 Observability Harness Contract'
    change: '`run`''s start refusals (before any child) gain a fifth, a home tree another user could write or read: the home pair for a home-level refusal, the instance pair for an instance-level one, exit 1, written before any log is opened; `viola revive`''s start arm reaches it through the same `checked_exe` / `refuse_tree`, and `viola revive` adds its three refusals to `run`''s five.'
    sidecar: '§3 `run` stderr refusals: the home-tree refusal (home or instance pair, exit 1, no log) added (chunk 2026-10-26-home-and-code-bearing-file-integrity).'
    rationale: 'Report Changes → Symbols, `src/cmd/run.rs` `checked_exe` / `refuse_tree` (`HomeLevel` → `human::home_state_pair()`, `InstanceLevel` → `human::instance_state_pair(name)`, through `human::refuse`) before `open_wrapper_log`; Count 9. obs-plan.md:311 lists four `run` start refusals and «`run`''s four».'
    basis: 'src/cmd/run.rs:202-215'
  - detector: D-retired
    severity: warning
    section: '§4 Span / Trace Coverage → Scenario: `wait` / `last` event-driven readback'
    change: 'Replace the closing clause «`strict-modes-failed` cannot occur on them, its check owed to the route entry "Home and code-bearing file integrity"» with: a home or `instances/<name>` that `check_instance` refuses ends a CLI `wait` / `last` before any log (`client::refuse_untrusted`): exit 21, the instance pair or `{"v":1,"error":"instance-unreachable","detail":null}` under `--json`, no line; MCP''s returns `instance-unreachable` with no line.'
    sidecar: '§4 wait/last: the «strict-modes-failed cannot occur» gap retired; a refused instance check is exit 21 with no line (chunk 2026-10-26-home-and-code-bearing-file-integrity).'
    rationale: 'Report Retired: dated gap (5) CLI `wait` / `last` and (8) the `viola mcp` tools now run the strict-modes check before their first read; Count 11 (pattern `cannot-occur`): obs-plan.md:361 stale.'
    basis: 'src/cmd/wait.rs:43-45'
  - detector: D-retired
    severity: warning
    section: '§7 Error Capture & Reporting → Error classes captured'
    change: 'MCP rejects: … `instance-unreachable` (a frame tool''s refused `check_instance`, made before `live_endpoint`, the liveness pre-check''s, or a frame tool''s server-verification mismatch on Linux and Windows) is logged by no line, its outcome kept inside `mcp.tool_call`, the `isError` code its only record.'
    sidecar: '§7 MCP rejects: a frame tool''s refused instance check joins the no-line `instance-unreachable` causes (chunk 2026-10-26-home-and-code-bearing-file-integrity).'
    rationale: 'Report Retired: dated gap (8), `viola mcp`''s send / wait / last / answer tools, closed; Symbols: `tools.rs` `frame` runs `check_instance` before `live_endpoint`, a refusal returns `instance-unreachable`, `detail: null`, no frame, no line; Count 20.'
    basis: 'crates/viola-mcp/src/tools.rs:337-339'
  - detector: D-obs-instrumentation
    severity: warning
    section: '§4 Span / Trace Coverage → Edge flows (E5)'
    change: 'E5: `state-recovered{detail:"snapshot-replayed"|"snapshot-unsupported-v"}` at WARN, written by `viola_state::replay::read_snapshot_or_replay`, one line per call; its product readers are six: `revive` (one read for both its collision check and its recorded-directory reading), `list`, the hook `connect`, the statusline `serve`, `run`''s `collision_check` (the caller''s one read) and `liveness::live_endpoint`; a `Replayed` or `Absent` read gives no endpoint, no frame and no command run, and `read_snapshot` has no product caller under `src/`.'
    sidecar: '§4 E5: the replay''s product readers 2 → 6 named, `read_snapshot` product callers 3 → 0 (chunk 2026-10-26-home-and-code-bearing-file-integrity).'
    rationale: 'Report Count 5 (replay product readers 2 → 6; `read_snapshot(` product callers 3 → 0), Count 6 (the four new readers on the emitter list), Deviation 4 (revive reads once: «one replay, one line»); Symbols: liveness.rs 92-94, hook.rs 224-226, statusline.rs 128-132, run.rs 425-432. obs-plan.md:394 says only that the other snapshot readers write no line.'
    basis: 'crates/viola-state/src/liveness.rs:92-94'
  - detector: D-obs-instrumentation
    severity: warning
    section: '§4 Span / Trace Coverage → Edge flows (`verify` / `plugin install`)'
    change: '… with `VIOLA_NAME` set they log `process-start` / `process-exit` to `cli-<name>.ndjson`; `verify` first runs `check_home_tree` on the home, `ledger/`, `bin/` and `plugin/` before its `cli` log, and a refusal (or `update_stamps`'' `StampsError::StrictModes`) is exit 1 with the home pair and no line …'
    sidecar: '§4 verify edge flow: the home-tree check before its log, a refusal with no line (chunk 2026-10-26-home-and-code-bearing-file-integrity).'
    rationale: 'Report Changes → Symbols, `src/cmd/verify.rs` 64-70 (refused → `refuse_home()`, exit 1, no step line, no summary, no line) and 288-296; Coverage of new surfaces: instrumentation n/a (no log opened).'
    basis: 'src/cmd/verify.rs:64-70'
note: 'D-obs-stack and D-obs-pii find no drift (no dependency change; every new refusal prints a fixed pair or a codes-only document and writes no line). The `sideload_fallback` enum''s new word `strict-modes-failed` (Count 7) and the `state.snapshot_recover` span placement under `run.start` / `hook.handle` (Count 6, Deviation 4) stand in the §6 / §4 detail files (log-coverage, span-trace-coverage), which are the cascade''s. The new no-line exit-1 / exit-21 refusals rest on the founder''s P4 Q2 (`inputs#I3`); §1''s Founder Direction 4 sentence («each cause behind exit 21 and exit 1 gets a distinct `detail` code») already stood beside revive''s and link''s no-line refusals and is not proposed here.'
