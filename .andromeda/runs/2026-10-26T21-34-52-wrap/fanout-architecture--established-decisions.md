proposals:
  - detector: D-retired
    severity: warning
    file: .andromeda/architecture-detail/established-decisions.md
    near: 79
    section: "## Established Decisions"
    change: "Replace the bullet (dec-message-broker-ipc) with: A verified client meets an unreadable or newer snapshot through the replay (`read_snapshot_or_replay`, §Standard Contracts, Snapshot envelope); a replayed snapshot carries no `endpoint`, `pid` or `started_at`, so `live_endpoint` returns none (the client's `instance-unreachable`) and the hook's connect reads the endpoint unreachable (`channel-unreachable`, fail open); no product reader under `src/` stays on `read_snapshot` (the founder's boundary-widening ratification, `inputs#I3` Q1 of chunk 2026-10-26-home-and-code-bearing-file-integrity)."
    sidecar: "established-decisions: the readers' move onto read_snapshot_or_replay stated as built; the owed-to-Home-and-code-bearing-file-integrity clause and «every reader stays on read_snapshot» retired (both occurrences on the line)."
    rationale: "Report Retired: read_snapshot as a product reader under src/ (3 → 0) and the dated gaps retired; Symbols: live_endpoint and hook connect read read_snapshot_or_replay, only Recovered::Snapshot proceeds; Counts 5 lists established-decisions.md:79 ×2 stale."
    basis: .andromeda/architecture-detail/established-decisions.md:79
  - detector: D-retired
    severity: warning
    file: .andromeda/architecture-detail/established-decisions.md
    near: 110
    section: "## Established Decisions"
    change: "In the first sentence (dec-hook-transport) replace «`read_snapshot`, then `Client::connect_by`» with «`read_snapshot_or_replay`, then `Client::connect_by`», and add after it: a replayed or absent snapshot is an unreachable endpoint (`NotSent::Unreachable`, `channel-unreachable`, fail open), SessionEnd's direct append as before."
    sidecar: "established-decisions: the hook's shared connect reads read_snapshot_or_replay; a replayed or absent snapshot is unreachable."
    rationale: "Report Symbols, src/cmd/hook.rs: hook connect reads read_snapshot_or_replay, Replayed / Absent → NotSent::Unreachable (fail open, channel-unreachable, SessionEnd's direct append as before); Counts 5 lists established-decisions.md:110 stale."
    basis: .andromeda/architecture-detail/established-decisions.md:110
    dependent-of: D-retired
  - detector: D-retired
    severity: warning
    file: .andromeda/architecture-detail/established-decisions.md
    near: 207
    section: "## Established Decisions"
    change: "Replace the bullet (dec-session-liveness) with: The snapshot's `pid` and `started_at` are also the server identity: `viola_state::liveness::live_endpoint` returns them with the endpoint from one `read_snapshot_or_replay` (`LiveEndpoint`), only a read snapshot (`Recovered::Snapshot`) proceeding, a replayed or absent one or an error giving none; every frame-writing client first runs the strict-modes check on the home and the instance (`check_instance`: the six CLI frame verbs `send`, `wait`, `last`, `answer`, `pause` and `release` through `cmd::client::refuse_untrusted`, as `link` / `unlink`; `viola mcp`'s frame tools; the hook's event and dialog paths), a refusal writing no frame and no line, then the liveness-only pre-check, then, on Linux and Windows, compares the connected peer's pid and its OS start time with them before its first frame (`verify_server`; §Standard Contracts, Session liveness)."
    sidecar: "established-decisions: live_endpoint reads the replay; every frame-writing client takes the strict-modes check before the liveness pre-check and the server check."
    rationale: "Report Retired: the eight dated strict-modes gaps (CLI send, wait/last, answer and hook.dialog, pause/release, the MCP tools, hook.event) each now run the check first, and read_snapshot as a product reader 3 → 0; Symbols: liveness.rs, client.rs refuse_untrusted, tools.rs frame; Counts 5 lists established-decisions.md:207 stale."
    basis: .andromeda/architecture-detail/established-decisions.md:207
    dependent-of: D-retired
  - detector: D-retired
    severity: warning
    file: .andromeda/architecture-detail/established-decisions.md
    near: 225
    section: "## Established Decisions"
    change: "In [Deployment / Distribution], after «At every start each companion is opened with `FILE_SHARE_READ` only and re-hashed in full against its pin, and the handles are held until the spawn.» state that before the held open `pin_companions` runs `strict::check_present` (the write rule) on `bin/<key>/conpty/` and on each companion that exists, a refusal being `PinError::StrictModes` with nothing written; and read «Unlike the exe, a mismatched companion never refuses the start» as «Unlike the exe, a mismatched companion, or one its strict-modes check refuses, never refuses the start», the child then on the inbox ConPTY (`sideload_fallback` `hash-mismatch` or `strict-modes-failed`)."
    sidecar: "established-decisions: the ConPTY companions' owner/DACL leg (check_present before the held open, PinError::StrictModes) stated as built; the re-hash-alone load retired."
    rationale: "Report Retired (3): Windows x64 ConPTY companions loaded under the re-hash alone, without the owner/DACL check; Symbols viola_state::pin: pin_companions runs strict::check_present, PinError::StrictModes; sideload_outcome maps it to strict-modes-failed; Counts 18 lists established-decisions.md:225 stale."
    basis: .andromeda/architecture-detail/established-decisions.md:225
  - detector: D-retired
    severity: warning
    file: .andromeda/architecture-detail/established-decisions.md
    near: 17
    section: "## Established Decisions"
    change: "In [PTY] read «An absent, tampered, unreadable or unloadable sideload fails open to kernel32's inbox ConPTY» as «An absent, tampered, unreadable or unloadable sideload, or one whose `conpty/` dir or companion fails its strict-modes check (`strict-modes-failed`), fails open to kernel32's inbox ConPTY»."
    sidecar: "established-decisions: [PTY]'s fail-open causes gain the companions' strict-modes refusal (sideload_fallback strict-modes-failed)."
    rationale: "Report Schema / config: process-start.sideload_fallback enum gains strict-modes-failed; Counts 7 (sideload-fb) lists established-decisions.md:17 stale where the fallback causes are enumerated."
    basis: .andromeda/architecture-detail/established-decisions.md:17
    dependent-of: D-retired
  - detector: D-retired
    severity: warning
    file: .andromeda/architecture-detail/established-decisions.md
    near: 205
    section: "## Established Decisions"
    change: "In the ConPTY sideload step's outcome list read «`outcome` `loaded` · `hash-mismatch` · `unreadable` · `load-failed` · `not-built`» as «`outcome` `loaded` · `hash-mismatch` · `strict-modes-failed` · `unreadable` · `load-failed` · `not-built`», and «`pin_companions` + the absolute-path pre-load» as «`pin_companions` (its strict-modes check, then the held re-hash) + the absolute-path pre-load»."
    sidecar: "established-decisions: run.conpty_sideload's outcome list gains strict-modes-failed."
    rationale: "Report Symbols, src/cmd/run.rs: sideload_outcome maps PinError::StrictModes to the outcome strict-modes-failed; Schema: the sideload_fallback enum now hash-mismatch, strict-modes-failed, unreadable, load-failed, not-built."
    basis: .andromeda/architecture-detail/established-decisions.md:205
    dependent-of: D-retired
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/established-decisions.md
    near: 227
    section: "## Established Decisions"
    change: "New bullet after the [Deployment / Distribution] entry: Before the wrapper log opens, `run` and `revive`'s start read the exe's bytes once (`checked_exe`: `pin::pin_key`, then `strict::check_home_tree(home, Some(name), Some(&key))`: the home under the read rule, `check_instance`'s set, `ledger/`, `bin/`, `plugin/`, then `bin/<key>/`, the pinned `viola(.exe)`, `plugin/<key>/`, its `hooks/` and `.claude-plugin/` under the write rule, each that exists); a refusal (`refuse_tree`) is exit 1 with the home pair (`TreeRefused::HomeLevel`) or the instance pair (`InstanceLevel`), no log opened, no line and no child (`revive --json` keeps `strict-modes-failed`); the pin then writes the bytes already checked (`pin::pin_bytes`, which `pin_exe` delegates to)."
    sidecar: "established-decisions: run / revive start's home-tree strict-modes check over the pinned copy and plugin folder, before the wrapper log."
    rationale: "Report Symbols: strict.rs check_home_tree / TreeRefused, run.rs checked_exe / refuse_tree called by run and revive's start before open_wrapper_log, pin.rs pin_key / pin_bytes; Outcome: run refuses a 0770 home, widened instances/<name>, bin/ or pinned copy, exit 1 before any log."
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/established-decisions.md
    near: 206
    section: "## Established Decisions"
    change: "New sentence after «A gone instance's directory is reused at once, …» ([Session Liveness]): `collision_check` takes its caller's one snapshot read (`Option<&Recovered>`): a replayed snapshot, which carries no `pid` or `started_at`, decides from the beat alone (a beat at most `STALE_AFTER` old is `already-live`, else the name is free), and an error or an absent snapshot leaves the name free; `revive`'s preflight reads once and hands that read to both its collision check and its recorded-directory reading, so one revive replays once."
    sidecar: "established-decisions: the collision check over a replayed snapshot (the beat alone) and revive's one read."
    rationale: "Report Symbols, src/cmd/run.rs: collision_check takes the caller's ONE read, Replayed → liveness from the beat alone, error or Absent → free; revive's preflight reads once for both; Deviation 4."
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/established-decisions.md
    near: 125
    section: "## Established Decisions"
    change: "New bullet after it ([Hook Contract]): The event and dialog paths take the instance from `instance_of(…)`, canonicalise it and run `check_instance` on the home and that instance (`canonical` · `trusted`, shared with `hook statusline`) before the diagnostics level is read or obs is initialised; a refused instance, or a `VIOLA_DIR` naming an instance directory that does not exist, exits 0 at once with no body: no `config.json` read, no role file, no stdin read, no line, no frame and no SessionEnd append (no line on a refused check: the founder's own answer, `inputs#I3` Q2 of chunk 2026-10-26-home-and-code-bearing-file-integrity). The two `--capture` arms stay ahead of it, unchanged."
    sidecar: "established-decisions: the hook's event and dialog paths run the instance check first and fail open silently on a refusal or a missing instance directory."
    rationale: "Report Symbols, src/cmd/hook.rs: canonical / trusted before read_diagnostics_level / viola_obs_init, a refused or absent instance exit 0 at once with nothing read or written; Deviation 5 (VIOLA_DIR naming no directory writes nothing, Count 23); Retired (1), (6)."
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/established-decisions.md
    near: 92
    section: "## Established Decisions"
    change: "New sentence at the end of the [MCP] paragraph: Each frame tool (`send`, `wait`, `last`, `answer`) runs `check_instance` on the home and the target's instance directory before `live_endpoint`; a refusal is the `instance-unreachable` result with `detail: null`, no frame and no line (chunk 2026-10-26-home-and-code-bearing-file-integrity)."
    sidecar: "established-decisions: viola mcp's frame tools run the instance strict-modes check before the liveness pre-check."
    rationale: "Report Symbols: viola-mcp tools.rs frame runs check_instance(&ctx.home, &instance_dir) before live_endpoint, a refusal result::unreachable(), no frame, no line; Retired (8)."
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/established-decisions.md
    near: 197
    section: "## Established Decisions"
    change: "New sentence at the end of the [CLI Version Compatibility] entry: Before its `cli` log `viola verify` runs `strict::check_home_tree(&home, None, None)` (the home under the read rule, `ledger/`, `bin/`, `plugin/`, each that exists); a refusal prints the home pair, exit 1, no step line, no summary and no line. The one stamps writer `update_stamps` runs `strict::check_stamps` before it creates `ledger/` and again under its lock before the capped read, returning `StampsError` (`StrictModes` · `State`); a refusal reads and writes nothing, and verify maps it to the same home pair."
    sidecar: "established-decisions: verify's home-tree check before its log and update_stamps' strict-modes check before any read or create."
    rationale: "Report Symbols: verify.rs check_home_tree before its cli log, refuse_home; stamps.rs StampsError, check_stamps before create_private_dir(ledger/) and under the lock; Retired (2); Deviation 1."
  - detector: D-arch-decisions
    severity: escalate
    file: .andromeda/architecture-detail/established-decisions.md
    near: 213
    section: "## Established Decisions"
    change: "New bullet after the [Error Handling] entry: A third open divergence, not an exception: `viola-state` holds `StampsError` (`StrictModes` · `State`; fixed messages, `From<io::Error>`, returned by `update_stamps`) beside `StateError` (chunk 2026-10-26-home-and-code-bearing-file-integrity); the rule stands, and its fold or ratification is open."
    sidecar: "established-decisions: viola-state's StampsError recorded as an open divergence from one enum per workspace crate."
    rationale: "Report Symbols viola_state::stamps: new pub enum StampsError with fixed messages, update_stamps now returns it (was StateError) — a second error enum in viola-state against [Error Handling]'s locked «one enum per workspace crate», unrecorded as an exception or divergence."
    basis: .andromeda/architecture-detail/established-decisions.md:213
note: "D-arch-resources finds nothing to register in this category (the new symbols, the six verbs' instance pair and the check_instance call-site count belong to §Occupied Resources / §Conventions); if the StampsError divergence is accepted, the body's [Error Handling] line (architecture.md:65, «two open divergences») and §Inherited Defaults Errors owe the same companion."
