proposals:
  - detector: D-arch-resources
    severity: escalate
    file: .andromeda/architecture-detail/occupied-resources.md
    near: 8
    section: "## Occupied Resources"
    change: "Replace «It rebuilds the path by the same rule only when the snapshot is missing or unreadable.» with: each of those readers reads the snapshot through `viola_state::replay::read_snapshot_or_replay`, and a replayed snapshot carries no `endpoint`, `pid` or `started_at`: `live_endpoint` (the CLI frame verbs and `mcp`'s frame tools) returns none, `hook`'s `connect` is unreachable (fail open, SessionEnd's direct append as before) and `hook statusline` runs and prints nothing; `run`'s collision check takes the caller's one read and, over a replayed snapshot, reads liveness from the beat alone (a beat at most `STALE_AFTER` old is `already-live`, else the name is free), an absent or unreadable snapshot leaving the name free (chunk 2026-10-26-home-and-code-bearing-file-integrity)."
    sidecar: "res-ipc-endpoints: the readers of the recorded endpoint over a replayed snapshot (no endpoint: unreachable / fail open; run's collision check from the beat alone) replace the path-rebuild sentence."
    rationale: "Report Symbols: live_endpoint, hook connect, statusline serve and run's collision_check moved onto read_snapshot_or_replay (Count 5, read_snapshot( product callers 3 → 0); escalated because the report does not say whether any reader still rebuilds the path by rule."
    basis: "viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/report.md:12"
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/occupied-resources.md
    near: 14
    section: "## Occupied Resources"
    change: "After «checks that the process serving it is the snapshot's wrapper», add: each of these processes first runs `viola_state::strict::check_instance` on the home and its instance, before it reads the snapshot (`hook` through `src/cmd/hook.rs` `trusted`, the CLI frame verbs through `client::refuse_untrusted`, `mcp`'s frame tools in `tools.rs` `frame`); a refused check sends no frame and writes no line (Filesystem, `instances/<ViolaName>/`)."
    sidecar: "res-ipc-endpoints: the instance strict-modes check now precedes the server check for every frame-writing client."
    rationale: "Report Retired: gaps (1), (4)–(8) closed, each frame writer now runs the strict-modes check before its first read or open (Symbols: refuse_untrusted, hook trusted, tools.rs frame)."
    basis: "viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/report.md:26"
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/occupied-resources.md
    near: 29
    section: "## Occupied Resources"
    change: "Add to the subcommand exit codes: `send`, `wait`, `last`, `answer`, `pause` and `release` exit 21 when their instance check refuses (`client::refuse_untrusted`: human the instance-state `unable:` / `hint:` pair, under `--json` the `instance-unreachable` document; no role file, no line), as `link` / `unlink` do; `verify` exits 1 with the home-state pair, before its `cli` log and any step line, when `check_home_tree` refuses its home, `ledger/`, `bin/` or `plugin/` or `update_stamps` refuses `ledger/stamps.json`; `run` and `revive`'s start exit 1 before any log when the home tree check refuses (`revive --json` keeps `strict-modes-failed`); on Unix an unstattable home (a path under a regular file) is such a refusal (`verify`'s home pair, the frame verbs' instance pair), while on Windows that path stats `NotFound` and `verify` keeps the `cli` catch-site exit 1 (chunk 2026-10-26-home-and-code-bearing-file-integrity)."
    sidecar: "res-binaries: the six frame verbs' exit 21, verify / run / revive start's exit 1 on a refused strict-modes check, the unstattable home per OS."
    rationale: "Report Symbols (client.rs refuse_untrusted, verify.rs 64-70 / 288-296, run.rs checked_exe / refuse_tree) and Counts 8, 9, 13."
    basis: "viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/report.md:15"
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/occupied-resources.md
    near: 65
    section: "## Occupied Resources"
    change: "Add: `hook`'s event, dialog and statusline arms canonicalise the instance directory `VIOLA_DIR` names and run the instance check before reading `config.json` or opening a role file; a `VIOLA_DIR` naming an instance directory that does not exist canonicalises to no session, so the hook writes nothing and exits 0 (chunk 2026-10-26-home-and-code-bearing-file-integrity)."
    sidecar: "res-env-vars: VIOLA_DIR naming no existing instance directory makes the hook write nothing."
    rationale: "Report Symbols (src/cmd/hook.rs canonical / trusted) and Count 23, a behaviour no master states (the directive's outcome item b, inputs#I5)."
    basis: "viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/report.md:16"
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/occupied-resources.md
    near: 103
    section: "## Occupied Resources"
    change: "After «a refused `link` / `unlink` instance check opens no role file and writes no line», add: so does the instance check of `send`, `wait`, `last`, `answer`, `pause` and `release` (the one `client::refuse_untrusted` the link verbs share), made before `client::start`; and in the opening `run` sentence add that `run`'s own start, like `revive`'s start arm, opens no process log and writes no line when its home tree check (`check_home_tree`) refuses (chunk 2026-10-26-home-and-code-bearing-file-integrity)."
    sidecar: "res-filesystem diagnostics/: the six frame verbs' and run's start refused checks join the no-line list."
    rationale: "Report Count 20 (no-line, occupied-resources.md:103 re-read for the new sites) and the founder's P4 Q2 (inputs#I3): a refused check writes no line, as link / unlink."
    basis: "viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/report.md:47"
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/occupied-resources.md
    near: 104
    section: "## Occupied Resources"
    change: "In `verify`'s sentence, add: written only after `check_home_tree(home, None, None)` passed (the home, `ledger/`, `bin/`, `plugin/`); a refused check is exit 1 with the home-state pair and no step line, no summary and no line (chunk 2026-10-26-home-and-code-bearing-file-integrity)."
    sidecar: "res-filesystem diagnostics/: verify's cli lines follow its home tree check."
    rationale: "Report Symbols src/cmd/verify.rs (64-70, 95-99) and Count 20."
    basis: "viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/report.md:19"
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/occupied-resources.md
    near: 105
    section: "## Occupied Resources"
    change: "In `mcp.ndjson`'s list, «none for a frame tool whose server check fails» gains: and no line and no frame for a frame tool whose `check_instance` refuses (the `instance-unreachable` result, `detail: null`); in `hook-<name>.ndjson`'s, add: the event and dialog paths write no line when their instance check refuses or no instance resolves (no `hook-invoked`, no `hook-decision`, no SessionEnd append)."
    sidecar: "res-filesystem diagnostics/: mcp frame tools and hook event/dialog paths write no line on a refused instance check."
    rationale: "Report Symbols (tools.rs 337-339, hook.rs) and Count 20 (no-line, occupied-resources.md:105)."
    basis: "viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/report.md:14"
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/occupied-resources.md
    near: 105
    section: "## Occupied Resources"
    change: "Add: a replayed snapshot read writes its one `state-recovered` line into the role file of the reading process, now also from `run`'s start (`run-<name>.ndjson`; the `state.snapshot_recover` span under `run.start` before `run.collision_check`), the hook's `connect` and `hook statusline` (`hook-<name>.ndjson`; the span under `hook.handle`) and `live_endpoint` (the CLI frame verbs' `cli-<name>.ndjson`, and `mcp.ndjson` for its frame tools), beside `list` and `revive` (chunk 2026-10-26-home-and-code-bearing-file-integrity)."
    sidecar: "res-filesystem diagnostics/: the four new replay readers' state-recovered lines and span placement."
    rationale: "Report Count 5 (replay product readers 2 → 6), Count 6 (state.snapshot_recover placement; the directive's outcome item b) and Coverage (the four readers on the replay, instrumentation state-recovered)."
    basis: "viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/report.md:33"
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/occupied-resources.md
    near: 105
    section: "## Occupied Resources"
    change: "After «dir 0700, files 0600 on Unix», add: before `ensure_private_dir` narrows anything, `open_role_file` and `write_detail` (`src/obs.rs` `check_diagnostics`) run `strict::check_diagnostics` on the existing `diagnostics/` folder and the role or detail file: a symbolic link is refused, a folder is held to the read rule, a file to the read rule on Unix (no group or other bit) and the write rule on Windows, and an absent path passes; a refusal is a failed open (the writer ends at `std::io::sink`, a detail line dropped)."
    sidecar: "res-filesystem diagnostics/: the role and detail file opens are strict-modes checked, symlinks refused."
    rationale: "Report Symbols src/obs.rs (69-76) and viola_state::strict check_diagnostics (127-145)."
    basis: "viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/report.md:20"
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/occupied-resources.md
    near: 105
    section: "## Occupied Resources"
    change: "Replace «Content-bearing detail stays in `instances/<ViolaName>/diagnostics/`.» with «Content-bearing detail stays in the instance's `instances/<ViolaName>/diagnostics/detail-<role>.ndjson`.»"
    sidecar: "res-filesystem diagnostics/: the home-level entry names the instance detail file."
    rationale: "Report Count 12 and the directive's outcome item a (inputs#I5): the instance detail file is unnamed at the diagnostics/ entry; v1-17's by-construction clause leans on it (Outcome, capability row)."
    basis: "viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/report.md:39"
  - detector: D-arch-resources
    severity: warning
    file: .andromeda/architecture-detail/occupied-resources.md
    near: 111
    section: "## Occupied Resources"
    change: "Replace «Four readers check these files before they use them (`viola list`, `viola revive`, `hook statusline`, and `viola link` / `viola unlink` sharing one check).» with: these readers check these files before they use them: `viola list`; `viola revive`; `viola run`'s start (through `check_home_tree`); `viola hook` (its event, dialog and statusline arms, sharing `src/cmd/hook.rs` `trusted`); the CLI frame verbs `send`, `wait`, `last`, `answer`, `pause`, `release`, `link` and `unlink` (sharing `client::refuse_untrusted`); and `viola mcp`'s four frame tools (`tools.rs` `frame`)."
    sidecar: "res-filesystem instances/: the checked readers widened from four to every entry point that reads an instance."
    rationale: "Report Symbols (refuse_untrusted, hook trusted, tools.rs frame, checked_exe) and Retired (the eight dated gaps closed); expected amendment «architecture §Occupied Resources (the checked readers ...)»."
    basis: "viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/report.md:77"
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/occupied-resources.md
    near: 111
    section: "## Occupied Resources"
    change: "After the `viola list` sentence, add: `board::scan`'s `check_path` holds the home to the write rule while `check_instance` per instance now holds it to the read rule, so on Unix a home at 0750 passes a board with no instance and refuses one with an instance, naming the instance (`ScanRefused::Instance`); `board.rs` is unchanged, and the inconsistency is owed to the working-route entry «Unix endpoint and home hardening» (the report's route ask for P5)."
    sidecar: "res-filesystem instances/: viola list's home-check inconsistency recorded with its owner."
    rationale: "Report Count 14 and the directive's outcome item c (inputs#I5): give viola list's home-check inconsistency an owner (Route asks for P5)."
    basis: "viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/report.md:41"
  - detector: D-arch-resources
    severity: warning
    file: .andromeda/architecture-detail/occupied-resources.md
    near: 112
    section: "## Occupied Resources"
    change: "Replace «over the home, `instances/`, the instance directory, its `snapshot.json` and its `events.ndjson`, each that exists and in that order» with: over the home (the read rule: owner-only with no group or other bit on Unix, no read grant to another SID on Windows), `instances/` (the write rule), the instance directory (read rule), its `diagnostics/` (read rule), its `snapshot.json` (write rule) and its `events.ndjson` (the write rule on Unix, the read rule on Windows), each that exists, a folder before what it holds; and add that `revive`'s start arm also runs `run`'s home tree check before its wrapper log opens."
    sidecar: "res-filesystem instances/: check_instance's set widened (diagnostics/, the read rule on the home and instance dir, Windows events.ndjson)."
    rationale: "Report Symbols viola_state::strict (check_instance now reads the home, the instance dir and diagnostics/ to the read rule; Windows events.ndjson to the read rule) and Count 2."
    basis: "viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/report.md:10"
    dependent-of: D-arch-resources
  - detector: D-arch-resources
    severity: warning
    file: .andromeda/architecture-detail/occupied-resources.md
    near: 112
    section: "## Occupied Resources"
    change: "Replace «`check_instance` has these three product call sites (`src/cmd/hook/statusline.rs`, `src/cmd/revive.rs` and `src/cmd/link.rs`, the one site both link verbs share; the rule: `check_instance(` under `src/`).» with «`check_instance` has three product call sites (`src/cmd/hook.rs` `trusted`, shared by the hook's event, dialog and statusline arms; `src/cmd/revive.rs`; and `src/cmd/client.rs` `refuse_untrusted`, shared by `link` / `unlink` and the six frame verbs; the rule: `check_instance(` under `src/`), beside `viola-mcp`'s `tools.rs` `frame` and `viola_state::strict::check_home_tree`, which holds the same set for `run`'s start, outside that rule.»"
    sidecar: "res-filesystem instances/: check_instance's call-site files moved (statusline.rs → hook.rs, link.rs → client.rs), count 3 by its rule."
    rationale: "Report Remaining-caller facts and Count 2: check_instance( under src/ 3 → 3, files moved; the stale count line occupied-resources.md:112."
    basis: "viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/report.md:21"
    dependent-of: D-arch-resources
  - detector: D-arch-resources
    severity: warning
    file: .andromeda/architecture-detail/occupied-resources.md
    near: 112
    section: "## Occupied Resources"
    change: "After the `viola hook statusline` sentence, add: the hook's event and dialog paths run the same check (`src/cmd/hook.rs` `canonical` / `trusted`, shared with the statusline arm) before `read_diagnostics_level` and obs init, a refused or absent instance returning exit 0 at once with no `config.json` read, no role file, no stdin read, no line, no frame and no SessionEnd append, the two `--capture` arms staying ahead of it; in the `link` / `unlink` sentence add: through `client::refuse_untrusted`, which `send`, `wait`, `last`, `answer`, `pause` and `release` call first too, with the same exit 21 and pair; and `viola mcp`'s frame tools run `check_instance` before `live_endpoint`, a refusal the `instance-unreachable` result with `detail: null`, no frame and no line."
    sidecar: "res-filesystem instances/: the hook event/dialog paths, the six frame verbs and the mcp frame tools join the checks made before any log."
    rationale: "Report Symbols (hook.rs, client.rs, tools.rs) and Count 20 (occupied-resources.md:112 @c446/@c1336 → extend); the founder's P4 Q2 / Q3 (inputs#I3)."
    basis: "viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/report.md:16"
    dependent-of: D-arch-resources
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/occupied-resources.md
    near: 113
    section: "## Occupied Resources"
    change: "Replace the line with: The check is proven on Unix (widened modes, and the read rule on the home, the instance directory and `diagnostics/`) and on Windows by `cfg(windows)` cases green on `windows-2025` (ci#40403389629): an instance tree granted Users write is refused `Writable` beside a `create_private_dir` tree that passes, a `diagnostics/` granted Users read is refused, and a home made with `create_dir_all` before viola is refused `Writable` on an inherited `S-1-5-32-545` write ACE."
    sidecar: "res-filesystem instances/: the owed Windows DACL case is built and green; the owed sentence retired."
    rationale: "Report Spec claims disproved: occupied-resources.md:113 «No case widens the DACL of an instance tree on Windows yet» is now covered (Symbols strict.rs cfg(windows) cases 708-830)."
    basis: "viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/report.md:73"
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/occupied-resources.md
    near: 113
    section: "## Occupied Resources"
    change: "New record after the line: `viola run`'s start and `viola revive`'s start arm run `src/cmd/run.rs` `checked_exe` before `open_wrapper_log`: it reads the exe's bytes once, takes `viola_state::pin::pin_key` (`<VERSION>-<16 hex>`) and runs `strict::check_home_tree(home, Some(name), Some(&key))`, which checks the home (read rule), `check_instance`'s set, `ledger/`, `bin/`, `plugin/`, then `bin/<key>/`, the pinned `viola(.exe)`, `plugin/<key>/`, its `hooks/` and `.claude-plugin/` (write rule), each that exists; a refusal (`TreeRefused::HomeLevel` → the home-state pair, `InstanceLevel` → the instance-state pair, through `run::refuse_tree`) is exit 1 with no log opened, no line and no child, and the start then pins through `pin::pin_bytes` with the bytes it checked. `viola verify` runs `check_home_tree(home, None, None)` (the home, `ledger/`, `bin/`, `plugin/`) before its `cli` log, a refusal exit 1 with the home-state pair (chunk 2026-10-26-home-and-code-bearing-file-integrity)."
    sidecar: "res-filesystem: the home tree check of run / revive's start and verify (check_home_tree, TreeRefused, checked_exe) registered."
    rationale: "Report Symbols (strict.rs check_home_tree / TreeRefused, run.rs checked_exe / refuse_tree, pin.rs pin_key / pin_bytes, verify.rs 64-70) and Count 4: the new tree checks stand in no master."
    basis: "viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/report.md:18"
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/occupied-resources.md
    near: 126
    section: "## Occupied Resources"
    change: "After «The ONE writer is `viola_state::stamps::update_stamps`, called only by `viola verify`», add: it runs `strict::check_stamps(home)` before `create_private_dir(ledger/)` and again under the lock before the capped read; a refusal (`StampsError::StrictModes`, a fixed message) reads and writes nothing, and `verify` maps it to the home-state pair, exit 1 (chunk 2026-10-26-home-and-code-bearing-file-integrity)."
    sidecar: "res-filesystem ledger/: update_stamps refuses on the strict-modes check before reading or creating."
    rationale: "Report Symbols viola_state::stamps (StampsError, check before create and under the lock), Retired gap (2) and Count 17 (stamps-err, occupied-resources.md:126)."
    basis: "viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/report.md:11"
note: "The body's own §Occupied Resources lines owe the same facts outside this file: architecture.md:319 (the conpty owner/DACL leg, `sideload_fallback` `strict-modes-failed`), :323 (name `detail-<role>.ndjson`, Count 12) and :325 (the checked readers list); D-arch-decisions found no new library, runtime or contradicted decision (Dependencies: none)."
