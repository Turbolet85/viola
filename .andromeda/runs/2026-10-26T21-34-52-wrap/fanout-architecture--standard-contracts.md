proposals:
  - detector: D-arch-resources
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 24
    section: '## Standard Contracts'
    change: 'Snapshot envelope: replace «As landed the fallback is library code in `viola-state` with two product readers» with six product readers, each reached after the instance''s strict-modes check has passed (`check_instance`, or on `run` / `revive`''s start `check_home_tree` holding its set): keep the `viola revive` preflight and `viola list` `board::read` text as written, then add the four since chunk 2026-10-26-home-and-code-bearing-file-integrity (the founder''s boundary-widening ratification, that chunk''s `inputs#I3` Q1): `viola_state::liveness::live_endpoint`, which proceeds only on `Recovered::Snapshot` and returns `None` for a replayed snapshot (no `endpoint`, `pid` or `started_at`), an absent one or an error, so no frame is written; `hook`''s `connect`, where a replayed or absent snapshot is `NotSent::Unreachable` (fail open, `channel-unreachable`, SessionEnd''s direct append as before); `hook statusline`''s `serve`, where a replayed snapshot has no `statusline_command`, so nothing runs or prints; and `run`''s `collision_check`, which takes its caller''s one read (`revive`''s preflight reads once for its collision check and its recorded-directory reading) and over a replayed snapshot reads liveness from the beat alone (a beat at most `STALE_AFTER` old is `already-live`, else free; an error or an absent snapshot is free).'
    sidecar: 'standard-contracts Snapshot envelope: the replay''s product readers 2 → 6 (+ `live_endpoint`, hook `connect`, statusline `serve`, `run`''s `collision_check`), each with its replayed-snapshot outcome (chunk 2026-10-26-home-and-code-bearing-file-integrity, Count 5).'
    rationale: 'Report Changes, Counts 5: «replay product readers 2 (`revive`, `list`) → 6 (+ hook `connect`, statusline `serve`, `run`''s `collision_check`, `live_endpoint`)»; Symbols liveness / hook / statusline / run bullets give each reader''s outcome over a replayed snapshot.'
    basis: 'viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/report.md:32'
  - detector: D-retired
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 25
    section: '## Standard Contracts'
    change: 'Snapshot envelope: replace «`read_snapshot` keeps its results, and its three product callers (`src/cmd/hook.rs` once, …; `run`''s collision check; `src/cmd/hook/statusline.rs`; … the CLI frame verbs and `viola mcp` read the snapshot inside `viola_state::liveness::live_endpoint`) still read every such snapshot as absent.» with: `read_snapshot` keeps its results but has no product caller under `src/` since chunk 2026-10-26-home-and-code-bearing-file-integrity (the rule: `read_snapshot(` under `src/`, outside test modules; 3 → 0): `hook`''s `connect`, `run`''s collision check and `hook statusline`''s `serve` read through `read_snapshot_or_replay`, and so does `viola_state::liveness::live_endpoint`, the CLI frame verbs'' and `viola mcp`''s snapshot read.'
    sidecar: 'standard-contracts Snapshot envelope: `read_snapshot` as a product reader under `src/` retired (3 → 0), its three callers and `live_endpoint` on the replay (chunk 2026-10-26-home-and-code-bearing-file-integrity, Retired).'
    rationale: 'Report Retired: «`read_snapshot` as a product reader under `src/` (3 → 0)»; Symbols remaining-caller facts: `read_snapshot(` product callers 3 → 0, the only `src/cmd` hit inside `mod tests`.'
    basis: 'viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/report.md:26'
  - detector: D-retired
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 109
    section: '## Standard Contracts'
    change: 'Session liveness: replace «all from its one `read_snapshot` call, so a frame-writing client takes the liveness-only pre-check first and the server check after it» with: all from its one `viola_state::replay::read_snapshot_or_replay` call, of which only `Recovered::Snapshot` proceeds (a replayed or absent snapshot, or an error, is `None`); a frame-writing client takes the instance''s strict-modes check first (CLI `client::refuse_untrusted`, the MCP frame tools'' `check_instance`), then this liveness-only pre-check, then the server check.'
    sidecar: 'standard-contracts Session liveness: `live_endpoint` reads through the replay, a replayed snapshot `None`; the strict-modes check precedes the liveness pre-check (chunk 2026-10-26-home-and-code-bearing-file-integrity).'
    rationale: 'Report Symbols: `live_endpoint(instance_dir)` reads `replay::read_snapshot_or_replay` (92-94): only `Recovered::Snapshot` proceeds; `Replayed`, `Absent` or an error → `None`; `refuse_untrusted` called first by the six verbs, `check_instance` before `live_endpoint` in tools.rs; Retired names `read_snapshot` as a product reader.'
    basis: 'viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/report.md:12'
    dependent-of: D-retired
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 51
    section: '## Standard Contracts'
    change: 'Channel methods, after the server-check item: add one item — Before that read, each of these clients runs `viola_state::strict::check_instance` on the home and `instances/<name>` (chunk 2026-10-26-home-and-code-bearing-file-integrity): CLI `send`, `wait`, `last`, `answer`, `pause` and `release` through `client::refuse_untrusted`, shared with `link` / `unlink`; `hook`''s event and dialog paths through `trusted`, before `read_diagnostics_level` and obs init; the MCP frame tools in `tools.rs` `frame`. A refused check writes no frame and no line and opens nothing under the home: the CLI verbs exit 21 (under `--json` the `instance-unreachable` document, else the instance pair), the MCP tools return `instance-unreachable` with `detail: null`, and `hook` exits 0 at once with no `config.json` read, no stdin read, no frame and no SessionEnd append.'
    sidecar: 'standard-contracts Channel methods: the instance strict-modes check made by every frame-writing client before its liveness pre-check and server check, its refusal per surface (chunk 2026-10-26-home-and-code-bearing-file-integrity).'
    rationale: 'Report Symbols: `refuse_untrusted` (client.rs 20-41) called first by the six verbs, `link_call` on it; hook.rs `instance_of(…).and_then(canonical).filter(trusted)` before obs init, refused → exit 0, no line, no frame, no SessionEnd append; tools.rs 337-339 `check_instance` before `live_endpoint` → `instance-unreachable`; Retired gaps (1), (4)-(8).'
    basis: 'viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/report.md:15'
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 93
    section: '## Standard Contracts'
    change: 'CLI `--json` output, after the `link` / `unlink` item: add one item — `viola send`, `wait`, `last`, `answer`, `pause` and `release` share that refusal through `client::refuse_untrusted` (chunk 2026-10-26-home-and-code-bearing-file-integrity; the founder''s word, that chunk''s `inputs#I3` Q2 / Q3): a refused strict-modes check on the home and `instances/<name>`, made first (`send` after its text read), prints exactly `{"v":1,"error":"instance-unreachable","detail":null}` on stdout under `--json`, else the instance pair through `human::refuse`, exit 21, with nothing opened under the home and no line in any log.'
    sidecar: 'standard-contracts CLI `--json` output: the six frame verbs'' refused strict-modes check prints link''s `instance-unreachable` document / instance pair, exit 21 (chunk 2026-10-26-home-and-code-bearing-file-integrity, Count 8).'
    rationale: 'Report Symbols `src/cmd/client.rs`: refused → under `--json` `refused_document("instance-unreachable")` (exactly `{"v":1,"error":"instance-unreachable","detail":null}`), else `human::refuse` of `human::instance_state_pair(name)`; `Some(21)`; opens nothing under the home and writes no line; Count 8.'
    basis: 'viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/report.md:15'
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 35
    section: '## Standard Contracts'
    change: 'Ledger stamps envelope, after its reader sentence: add — The writer refuses first (chunk 2026-10-26-home-and-code-bearing-file-integrity): `update_stamps` returns `Result<(), StampsError>` (`StrictModes` · `State`, fixed messages) and runs `strict::check_stamps(home)` before it creates `ledger/` and again under the lock before its capped read, a refusal reading and writing nothing; `viola verify` runs `check_home_tree(home, None, None)` (the home, `ledger/`, `bin/`, `plugin/`) before its `cli` log, and a refusal there or a `StampsError::StrictModes` prints the home pair, exit 1, with no step line and no summary.'
    sidecar: 'standard-contracts Ledger stamps envelope: `update_stamps`'' strict-modes check before the create and under the lock (`StampsError`), verify''s home-tree check before its log (chunk 2026-10-26-home-and-code-bearing-file-integrity, Count 17).'
    rationale: 'Report Symbols `viola_state::stamps` (StampsError, check_stamps before `create_private_dir(ledger/)` and under the lock) and `src/cmd/verify.rs` (check_home_tree 64-70, refuse_home, StrictModes → home pair); Retired gap (2).'
    basis: 'viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/report.md:11'
note: 'D-arch-decisions clean (Dependencies none, no locked decision contradicted); Count 14 (`board::scan`''s home-check inconsistency) has no site in this file (line 81 names no check); body architecture.md:100''s exit-21 list (six frame verbs, Count 8) is the Conventions category''s.'
