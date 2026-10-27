proposals:
  - detector: D-retired
    severity: warning
    file: .andromeda/security-plan-detail/authentication-authorization.md
    near: 25
    section: '## Authentication & Authorization'
    change: 'Replace the sentence from «**Interim gap (ratified), its strict-modes half only:**» to «(Decisions Log `2026-09-28`).» with: **Strict-modes before the reference values, no dated gap (chunk 2026-10-26-home-and-code-bearing-file-integrity):** `viola hook`''s event and dialog paths take `instance_of(…).and_then(canonical).filter(trusted)` (`src/cmd/hook.rs`, `trusted` running `viola_state::strict::check_instance`) before `read_diagnostics_level` and obs init, a refused or absent instance returning exit 0 at once with no `config.json` read, no role file, no stdin read, no line, no frame and no SessionEnd append (the two `--capture` arms stay ahead of it); its shared `connect` then reads `read_snapshot_or_replay` (only `Recovered::Snapshot` proceeds; `Replayed` or `Absent` is `NotSent::Unreachable`, failing open with `channel-unreachable`), then `connect_by`, then `verify_server` on Linux and Windows (macOS the recorded gap).'
    sidecar: 'Auth & Authz: the `hook.event` strict-modes gap retired; the hook event/dialog instance check and the replay read stated as built.'
    rationale: 'Report **Retired** (1): `hook.event`''s snapshot read without strict-modes retired, each entry point now checking before its first read or open; Symbols `src/cmd/hook.rs` (`canonical` / `trusted`; `connect` on `read_snapshot_or_replay`); the readers on the replay are the founder''s own answer to P4 fork Q1 (`inputs#I3`).'
    basis: viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/report.md:26
  - detector: D-retired
    severity: warning
    file: .andromeda/security-plan-detail/authentication-authorization.md
    near: 25
    section: '## Authentication & Authorization'
    change: 'Replace the sentence from «**Second interim gap (the founder''s ruling, live, 2026-10-04, relayed by the overseer):**» to «It does not borrow the `hook.event` exception.» with: CLI `viola send` runs `cmd::client::refuse_untrusted` (`check_instance` on the absolute home and `instances/<name>`, after its text read) before anything under the home opens, then the liveness-only pre-check (`live_endpoint` over `read_snapshot_or_replay`: only a `Recovered::Snapshot` whose pid + start time are alive, whose heartbeat is live (`classify(beat_age, same_process)` = `live`) and whose `endpoint` is present, else exit 21), then, on Linux and Windows, checks the serving process''s identity before the frame (`client::connect`: the peer pid and its start time against the snapshot''s `pid` and `started_at` from the same `live_endpoint` read). The residual, a same-user process squatting a stale endpoint name, is closed on Linux and Windows and open on macOS; the pipe DACL and the 0700 socket directory still apply. In the next sentence drop «and this gap» («closed `send` params) and this gap.» → «closed `send` params).»).'
    sidecar: 'Auth & Authz: the CLI `send` strict-modes gap retired; `refuse_untrusted` before its liveness and server checks.'
    rationale: 'Report **Retired** (4) CLI `send`; Symbols `src/cmd/client.rs` `refuse_untrusted`, called first by `send` (after its text read); `live_endpoint` reads `read_snapshot_or_replay`, only `Recovered::Snapshot` proceeding.'
    basis: viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/report.md:15
    dependent-of: D-retired
  - detector: D-retired
    severity: warning
    file: .andromeda/security-plan-detail/authentication-authorization.md
    near: 25
    section: '## Authentication & Authorization'
    change: 'Replace the sentence from «**Third interim gap here, the fifth dated gap overall (the founder''s ruling, live, 2026-10-04, relayed by the overseer):**» to «they borrow neither the `hook.event` exception nor `send`''s.» with: CLI `viola wait` and `viola last` run `refuse_untrusted` first, then write their `wait` / `last` frame after the same liveness-only pre-check (else exit 21 `instance-unreachable`, `during:"connect"`) and, on Linux and Windows, the same server check through `client::connect`; the residual is the same, closed on Linux and Windows and open on macOS.'
    sidecar: 'Auth & Authz: the CLI `wait` / `last` strict-modes gap retired.'
    rationale: 'Report **Retired** (5) CLI `wait` / `last`; Symbols: `refuse_untrusted` called first by `wait` (43-45) and `last` (30-32).'
    basis: viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/report.md:15
    dependent-of: D-retired
  - detector: D-retired
    severity: warning
    file: .andromeda/security-plan-detail/authentication-authorization.md
    near: 26
    section: '## Authentication & Authorization'
    change: 'In «`wait` and `last` are served by the wrapper as of chunk 2026-10-04-wait-and-last, under the channel row''s controls (frame cap, depth, version, the `wait` / `last` params) and this gap.» drop «and this gap».'
    sidecar: 'Auth & Authz: `wait` / `last` served sentence no longer names a gap.'
    rationale: 'Report **Retired** (5): the gap the sentence names is closed.'
    basis: viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/report.md:26
    dependent-of: D-retired
  - detector: D-retired
    severity: warning
    file: .andromeda/security-plan-detail/authentication-authorization.md
    near: 26
    section: '## Authentication & Authorization'
    change: 'Replace the sentence from «**Fourth interim gap here, the sixth dated gap overall (the founder''s ruling F1, live, 2026-10-04, relayed by the overseer, the residual shown):**» to «These frames borrow neither the `hook.event` exception nor `send`''s nor `wait` / `last`''s.» with: CLI `viola answer` runs `refuse_untrusted` first, then writes its `answer` frame after the same liveness-only pre-check (`live_endpoint`; else exit 21 `instance-unreachable`), and `viola hook pre-tool-use|permission-request`, past the hook''s instance check, writes its `hook.dialog` frame after its payload shape check (`classify`), on the endpoint its event path reads; on Linux and Windows both check the serving process''s identity before the frame (`answer` through `client::connect`, `hook.dialog` through `src/cmd/hook.rs`''s shared `connect`). The residual, a process squatting the endpoint answering a dialog (a PermissionRequest `allow` included), is closed on Linux and Windows and open on macOS. In the next sentence drop «and this gap» («under the channel row''s controls and this gap.» → «under the channel row''s controls.»).'
    sidecar: 'Auth & Authz: the CLI `answer` / `hook.dialog` strict-modes gap retired.'
    rationale: 'Report **Retired** (6) CLI `answer` and `hook pre-tool-use|permission-request`''s `hook.dialog`; Symbols: `refuse_untrusted` first in `answer` (94-96); the hook dialog path behind `canonical` / `trusted`.'
    basis: viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/report.md:16
    dependent-of: D-retired
  - detector: D-retired
    severity: warning
    file: .andromeda/security-plan-detail/authentication-authorization.md
    near: 26
    section: '## Authentication & Authorization'
    change: 'Replace the sentence from «**Fifth interim gap here, the seventh dated gap overall (the founder''s ruling F-W1, live, 2026-10-04, relayed by the overseer, the pause-swallow residual shown):**» to «and neither checks the snapshot''s strict-modes.» with: CLI `viola pause` and `viola release` run `refuse_untrusted` first, then write their `pause` / `release` frame after the same liveness-only pre-check (`live_endpoint`; else exit 21 `instance-unreachable`); on Linux and Windows both then check the serving process''s identity before the frame through `client::connect`.'
    sidecar: 'Auth & Authz: the CLI `pause` / `release` strict-modes gap retired.'
    rationale: 'Report **Retired** (7) CLI `pause` / `release`; Symbols: `refuse_untrusted` first in `pause` (32-34) and `release` (35-37).'
    basis: viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/report.md:15
    dependent-of: D-retired
  - detector: D-retired
    severity: warning
    file: .andromeda/security-plan-detail/authentication-authorization.md
    near: 27
    section: '## Authentication & Authorization'
    change: 'Drop «These frames borrow none of the six earlier gaps.» and, in «`pause` and `release` are served by the wrapper as of chunk 2026-10-04-the-wheel, under the channel row''s controls and this gap.», drop «and this gap».'
    sidecar: 'Auth & Authz: the `pause` / `release` residual no longer names the earlier gaps.'
    rationale: 'Report **Retired** (7): no dated gap stands to borrow or name.'
    basis: viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/report.md:26
    dependent-of: D-retired
  - detector: D-retired
    severity: warning
    file: .andromeda/security-plan-detail/authentication-authorization.md
    near: 27
    section: '## Authentication & Authorization'
    change: 'Replace the sentence from «**Sixth interim gap here, the eighth dated gap overall (the founder''s own answer to P4 fork Q1, live, 2026-10-17, relayed by the overseer, `inputs#I2` of chunk 2026-10-17-mcp-server-for-drivers):**» to «These frames borrow none of the seven earlier gaps.» with: `viola mcp`''s `send` / `wait` / `last` / `answer` tools run `check_instance(&ctx.home, &instance_dir)` in `frame` before `live_endpoint` (a refusal returns `{"error":"instance-unreachable","detail":null}` with no frame and no line), then write their frames through the Tokio client after the same liveness-only pre-check (`viola_state::liveness::live_endpoint`; else `instance-unreachable`); on Linux and Windows each tool then checks the serving process''s identity after `TokioClient::connect` and before its frame (`server_verified`, the start-time read on `tokio::task::spawn_blocking`).'
    sidecar: 'Auth & Authz: the `viola mcp` frame tools strict-modes gap retired; `check_instance` in `frame` before `live_endpoint`.'
    rationale: 'Report **Retired** (8) `viola mcp`''s tools; Symbols `viola-mcp` `tools.rs` `frame` (337-339): `check_instance` before `live_endpoint`, a refusal `result::unreachable()`, no frame, no line.'
    basis: viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/report.md:14
    dependent-of: D-retired
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/authentication-authorization.md
    near: 27
    section: '## Authentication & Authorization'
    change: 'Before «**On mismatch**» add: **On a refused strict-modes check** (made before any snapshot value is read): no frame and no line, the mode or ACL left as found; the CLI frame verbs exit 21, under `--json` exactly `{"v":1,"error":"instance-unreachable","detail":null}` on stdout with stderr empty, in human mode the fixed instance-state `unable:` / `hint:` pair on stderr through `human::refuse` (as `link` / `unlink`); MCP returns `instance-unreachable` with `detail: null`; `hook` exits 0 with no body (the founder''s own answers to P4 forks Q2 and Q3, `inputs#I3` of chunk 2026-10-26-home-and-code-bearing-file-integrity).'
    sidecar: 'Auth & Authz: the refused strict-modes check''s outcome at the frame verbs, MCP and hook (no line, the instance pair).'
    rationale: 'Report Decisions & corrections: a refused check at every new entry point writes no line, as `link` / `unlink`, and the six frame verbs print `link`''s instance pair (`inputs#I3` Q2, Q3); Counts 8 and 20.'
    basis: viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/report.md:111
  - detector: D-retired
    severity: warning
    file: .andromeda/security-plan-detail/authentication-authorization.md
    near: 27
    section: '## Authentication & Authorization'
    change: 'In the **Collision check** sentence replace «reads the snapshot through `read_snapshot` and judges the recorded wrapper by its `pid` + OS start time and the heartbeat» with «takes the caller''s one `read_snapshot_or_replay` read (`Option<&Recovered>`): a `Snapshot` is judged by the recorded wrapper''s `pid` + OS start time and the heartbeat, a `Replayed` one by the beat alone (`beat_age ≤ STALE_AFTER` is `already-live`, else free), an error or `Absent` is free; `revive`''s preflight reads once and hands that read to both its collision check and its recorded-directory reading», the rest of the sentence unchanged.'
    sidecar: 'Auth & Authz: the collision check reads the caller''s one replay-aware read, not `read_snapshot`.'
    rationale: 'Report **Retired**: `read_snapshot` as a product reader under `src/` (3 → 0); Symbols `src/cmd/run.rs` `collision_check` (`Option<&Recovered>`); Count 5 lists this line stale.'
    basis: viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/report.md:18
    dependent-of: D-retired
  - detector: D-security-auth
    severity: warning
    file: .andromeda/security-plan-detail/authentication-authorization.md
    near: 34
    section: '## Authentication & Authorization'
    change: 'In `revive`''s parenthetical replace «which reads the home, `instances/`, the instance directory, its `snapshot.json` and its `events.ndjson`, each that exists and in that order» with «which reads the home (the read rule), `instances/` (the write rule), the instance directory (the read rule), its `diagnostics/` (the read rule), its `snapshot.json` (the write rule) and its `events.ndjson` (Unix the write rule, Windows the read rule), each that exists and a folder before what it holds».'
    sidecar: 'Auth & Authz: `check_instance`''s set widened (home, instance dir, `diagnostics/` to the read rule; Windows `events.ndjson` read rule).'
    rationale: 'Report Symbols `viola_state::strict`: `check_instance` (signature unchanged) now reads the home (READ), `instances/` (write), the instance directory (READ), `diagnostics/` (READ, new), `snapshot.json` (write), `events.ndjson` (Unix write; Windows READ); Count 2.'
    basis: viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/report.md:10
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/authentication-authorization.md
    near: 34
    section: '## Authentication & Authorization'
    change: 'After «at the start of `run`» add: (`cmd::run::checked_exe` reads the exe''s bytes once, takes `pin::pin_key` and runs `viola_state::strict::check_home_tree(home, Some(name), Some(&key))` before `open_wrapper_log`: the home (the read rule), `check_instance`''s set, `ledger/`, `bin/`, `plugin/`, `bin/<key>/`, the pinned `viola(.exe)`, `plugin/<key>/`, its `hooks/` and `.claude-plugin/` (the write rule), each that exists; through `cmd::run::refuse_tree` a `TreeRefused::HomeLevel` prints the home-state pair and an `InstanceLevel` the instance-state pair, exit 1, no log opened, no line, no child; `revive`''s start arm calls the same two before its log, `--json` keeping `strict-modes-failed`), `verify` (`check_home_tree(&home, None, None)`, the home, `ledger/`, `bin/` and `plugin/`, before its `cli` log; a refusal prints the home-state pair, exit 1, no step line, no summary, no line),'
    sidecar: 'Auth & Authz: `run` / `revive` start and `verify` named as strict-modes entry points through `check_home_tree`.'
    rationale: 'Report Symbols `src/cmd/run.rs` (`checked_exe`, `refuse_tree`; `run` 166-172, `revive` 125-135) and `src/cmd/verify.rs` (64-70, `refuse_home`); Count 9.'
    basis: viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/report.md:18
  - detector: D-retired
    severity: warning
    file: .andromeda/security-plan-detail/authentication-authorization.md
    near: 34
    section: '## Authentication & Authorization'
    change: 'Replace «proven by `cfg(unix)` cases, with no Windows case that widens an instance tree''s DACL yet, owed to the entry "Home and code-bearing file integrity"» with «proven by `cfg(unix)` cases and by `cfg(windows)` cases (an instance tree granted Users write refused `Writable` beside a `create_private_dir` tree that passes, a `diagnostics/` granted Users read refused), green on `windows-2025` (ci#40403389629)».'
    sidecar: 'Auth & Authz: the owed Windows widened-DACL case is built and green.'
    rationale: 'Report Spec claims disproved: «No case widens the DACL of an instance tree on Windows yet» now covered by the `cfg(windows)` cases green on ci#40403389629; Count 1 `owner-title`.'
    basis: viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/report.md:73
    dependent-of: D-retired
  - detector: D-retired
    severity: warning
    file: .andromeda/security-plan-detail/authentication-authorization.md
    near: 34
    section: '## Authentication & Authorization'
    change: 'In `revive`''s parenthetical drop «it is none of the dated interim gaps; » (before «proven by `cfg(unix)` cases»).'
    sidecar: 'Auth & Authz: `revive` no longer contrasted with dated gaps that no longer stand.'
    rationale: 'Report **Retired**: all eight dated gaps closed; the contrast describes them as standing.'
    basis: viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/report.md:26
    dependent-of: D-retired
  - detector: D-security-auth
    severity: warning
    file: .andromeda/security-plan-detail/authentication-authorization.md
    near: 35
    section: '## Authentication & Authorization'
    change: 'In the `link` / `unlink` parenthetical replace «`viola_state::strict::check_instance` on the absolute home and `instances/<driven>` before `client::start`» with «`cmd::client::refuse_untrusted` (`viola_state::strict::check_instance` on `std::path::absolute(home)` and `instances/<driven>`), shared with the six frame verbs, before `client::start`».'
    sidecar: 'Auth & Authz: `link` / `unlink` check moved onto the shared `refuse_untrusted`, unchanged in behaviour.'
    rationale: 'Report Symbols `src/cmd/client.rs`: `link_call` (link.rs:94-95) moved onto `refuse_untrusted` unchanged in behaviour; Count 10 `refuse-count` stale at this line.'
    basis: viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/report.md:15
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/authentication-authorization.md
    near: 35
    section: '## Authentication & Authorization'
    change: 'Between «proven by `cfg(unix)` cases only), » and «`mcp` (as landed» add: the six CLI frame verbs `send` (after its text read), `wait`, `last`, `answer`, `pause` and `release` (the same `refuse_untrusted` first, before `client::start`, any role file, log or snapshot value: exit 21, the instance-state `unable:` / `hint:` pair or exactly `{"v":1,"error":"instance-unreachable","detail":null}` under `--json`, no log, no line, the mode or ACL as found; the founder''s own answer to P4 fork Q3, `inputs#I3` of chunk 2026-10-26-home-and-code-bearing-file-integrity; proven on three OSes by `tests/security_negatives_strict_modes.rs`), '
    sidecar: 'Auth & Authz: the six CLI frame verbs join the strict-modes entry-point list.'
    rationale: 'Report Symbols `refuse_untrusted` called first by `send`, `wait`, `last`, `answer`, `pause`, `release`; Outcome (security) MET on all three CI legs; Count 2 `check-instance` extend at this line.'
    basis: viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/report.md:128
  - detector: D-retired
    severity: warning
    file: .andromeda/security-plan-detail/authentication-authorization.md
    near: 35
    section: '## Authentication & Authorization'
    change: 'In the `mcp` parenthetical replace «its frame tools then read `snapshot.json`''s `endpoint` under the eighth dated gap, without the check» with «its frame tools then run `check_instance` on the home and the target''s instance directory in `frame` before `live_endpoint`, a refusal returning `instance-unreachable` with `detail: null`, no frame and no line».'
    sidecar: 'Auth & Authz: the `mcp` entry no longer names the eighth dated gap.'
    rationale: 'Report **Retired** (8); Symbols `viola-mcp` `tools.rs` `frame` (337-339).'
    basis: viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/report.md:14
    dependent-of: D-retired
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/authentication-authorization.md
    near: 35
    section: '## Authentication & Authorization'
    change: 'Replace «every `hook <event>` including `hook statusline` (which runs `viola_state::strict::check_instance` on its canonicalised instance directory» with «every `hook <event>`, its event and dialog paths and `hook statusline` alike (each runs `viola_state::strict::check_instance` on its canonicalised instance directory through `src/cmd/hook.rs`''s shared `canonical` / `trusted`, the two `--capture` arms ahead of it; a `VIOLA_DIR` naming an instance directory that does not exist canonicalises to no session, so the hook writes nothing;», the rest of the parenthetical unchanged.'
    sidecar: 'Auth & Authz: every hook arm runs the instance check; a `VIOLA_DIR` naming no directory writes nothing.'
    rationale: 'Report Symbols `src/cmd/hook.rs` (`canonical` 75-81 and `trusted` 83-86 moved from `statusline.rs`, shared by both arms); Deviation 5 and Count 23 (`viola-dir-none`).'
    basis: viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/report.md:16
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/authentication-authorization.md
    near: 35
    section: '## Authentication & Authorization'
    change: 'In the `list` parenthetical, before «proven by `cfg(unix)` cases only), `link` / `unlink`», add: `board::scan` holds the home to the write rule (`check_path`) while `check_instance` now holds it to the read rule, so a home at 0750 passes the board with no instance and refuses it naming the instance (`ScanRefused::Instance`) with one (`board.rs` unchanged; its owner routed at this wrap''s P5); '
    sidecar: 'Auth & Authz: `viola list`''s home-check inconsistency recorded, owner routed at P5.'
    rationale: 'Report Count 14 (`board-scan`): the inconsistency is the new fact, `board.rs` unchanged; the directive''s outcome item (c) and Route asks for P5 give it an owner.'
    basis: viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/report.md:41
  - detector: D-retired
    severity: warning
    file: .andromeda/security-plan-detail/authentication-authorization.md
    near: 35
    section: '## Authentication & Authorization'
    change: 'In the `list` parenthetical drop «it is none of the dated interim gaps; » (before «proven by `cfg(unix)` cases only), `link` / `unlink`»).'
    sidecar: 'Auth & Authz: `list` no longer contrasted with dated gaps that no longer stand.'
    rationale: 'Report **Retired**: all eight dated gaps closed.'
    basis: viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/report.md:26
    dependent-of: D-retired
  - detector: D-retired
    severity: warning
    file: .andromeda/security-plan-detail/authentication-authorization.md
    near: 35
    section: '## Authentication & Authorization'
    change: 'In the `link` / `unlink` parenthetical drop «it is none of the dated interim gaps; » (before «proven by `cfg(unix)` cases only), `mcp`»).'
    sidecar: 'Auth & Authz: `link` / `unlink` no longer contrasted with dated gaps that no longer stand.'
    rationale: 'Report **Retired**: all eight dated gaps closed.'
    basis: viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/report.md:26
    dependent-of: D-retired
  - detector: D-retired
    severity: warning
    file: .andromeda/security-plan-detail/authentication-authorization.md
    near: 36
    section: '## Authentication & Authorization'
    change: 'In the hook parenthetical drop «; it is none of the dated interim gaps» (before «), and every CLI client verb»).'
    sidecar: 'Auth & Authz: the hook arm no longer contrasted with dated gaps that no longer stand.'
    rationale: 'Report **Retired**: all eight dated gaps closed.'
    basis: viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/report.md:26
    dependent-of: D-retired
  - detector: D-retired
    severity: warning
    file: .andromeda/security-plan-detail/authentication-authorization.md
    near: 36
    section: '## Authentication & Authorization'
    change: 'Replace «interim gap, ratified: until the Epoch 6 entry "Home and code-bearing file integrity" lands, `viola hook <event>` reads `snapshot.json`''s `endpoint` without this check, Decisions Log `2026-09-28`;» with «no dated gap remains: the eight the founder ratified (Decisions Log `2026-09-28` twice and `2026-09-29`, his rulings of 2026-10-04 and his answer of 2026-10-17) closed with chunk 2026-10-26-home-and-code-bearing-file-integrity, each entry point running this check before its first read or open; `viola hook <event>` reads `snapshot.json`''s `endpoint` only past its instance check;»'
    sidecar: 'Auth & Authz: the entry-point list''s first dated gap retired; no dated gap remains.'
    rationale: 'Report **Retired** (1), and the eight gaps 8 → 0 (Count 1, `gap-until` standing at this line).'
    basis: viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/report.md:28
    dependent-of: D-retired
  - detector: D-retired
    severity: warning
    file: .andromeda/security-plan-detail/authentication-authorization.md
    near: 36
    section: '## Authentication & Authorization'
    change: 'Replace «a second, ratified the same way: `viola verify`''s `update_stamps` reads `ledger/stamps.json` without it until the same entry adds it to this entry-point set, Decisions Log `2026-09-28`, stamps read» with «`viola verify`''s `update_stamps` runs `strict::check_stamps(home)` before `create_private_dir(ledger/)` and again under the lock before its capped read, a refusal `StampsError::StrictModes` reading and writing nothing, which `verify`''s `measure` maps to the home-state pair, exit 1», and in the parenthetical that follows replace «left this gap with chunk 2026-10-04-dialog-answers-by-dialog-id:» with «has read through the check since chunk 2026-10-04-dialog-answers-by-dialog-id:».'
    sidecar: 'Auth & Authz: the `update_stamps` strict-modes gap retired; `check_stamps` before the create and under the lock.'
    rationale: 'Report **Retired** (2); Symbols `viola_state::stamps` (`StampsError`, `check_stamps` before `create_private_dir` and under the lock); Deviation 1; Count 17 `stamps-err` stale at this line.'
    basis: viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/report.md:11
    dependent-of: D-retired
  - detector: D-retired
    severity: warning
    file: .andromeda/security-plan-detail/authentication-authorization.md
    near: 36
    section: '## Authentication & Authorization'
    change: 'Replace «a third, ratified by the founder live: on Windows x64 `viola run` loads the ConPTY companions `bin/<version>-<hash>/conpty/{OpenConsole.exe,conpty.dll}` under a `FILE_SHARE_READ`-only held handle and a full SHA-256 re-hash against the compiled pins, without the Windows owner/DACL check, until the same entry adds the `conpty/` folder and both files to the Windows set below, Decisions Log `2026-09-29`;» with «on Windows x64 `viola run` loads the ConPTY companions `bin/<version>-<hash>/conpty/{OpenConsole.exe,conpty.dll}` only after `pin_companions` runs `strict::check_present` (the write rule) on the `conpty/` folder and each companion that exists, then under a `FILE_SHARE_READ`-only held handle and a full SHA-256 re-hash against the compiled pins; a refusal is `PinError::StrictModes`, nothing written, and the child degrades to the inbox ConPTY with `process-start.sideload_fallback` `strict-modes-failed`, exit unchanged;»'
    sidecar: 'Auth & Authz: the ConPTY companions'' owner/DACL gap retired; `check_present` before the held open.'
    rationale: 'Report **Retired** (3); Symbols `viola_state::pin` (`pin_companions` runs `check_present`, `PinError::StrictModes`) and `run.rs:510` `sideload_outcome`; Count 18 `conpty-dacl` stale at this line; Outcome MET on `windows-2025`.'
    basis: viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/report.md:13
    dependent-of: D-retired
  - detector: D-retired
    severity: warning
    file: .andromeda/security-plan-detail/authentication-authorization.md
    near: 36
    section: '## Authentication & Authorization'
    change: 'Replace «a fourth, the founder''s ruling, live, 2026-10-04, relayed by the overseer: `viola send` reads `snapshot.json`''s `endpoint` (and its liveness fields) without this check until the same entry adds it;» with «`viola send` reads `snapshot.json`''s `endpoint` (and its liveness fields) only after `refuse_untrusted`;»'
    sidecar: 'Auth & Authz: the entry-point list''s `send` gap retired.'
    rationale: 'Report **Retired** (4).'
    basis: viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/report.md:26
    dependent-of: D-retired
  - detector: D-retired
    severity: warning
    file: .andromeda/security-plan-detail/authentication-authorization.md
    near: 36
    section: '## Authentication & Authorization'
    change: 'Replace «a fifth, the founder''s ruling, live, 2026-10-04, relayed by the overseer: `viola wait` and `viola last` read them the same way, without this check, until the same entry adds them;» with «`viola wait` and `viola last` the same way;»'
    sidecar: 'Auth & Authz: the entry-point list''s `wait` / `last` gap retired.'
    rationale: 'Report **Retired** (5).'
    basis: viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/report.md:26
    dependent-of: D-retired
  - detector: D-retired
    severity: warning
    file: .andromeda/security-plan-detail/authentication-authorization.md
    near: 37
    section: '## Authentication & Authorization'
    change: 'Replace «a sixth, the founder''s ruling F1, live, 2026-10-04, relayed by the overseer: `viola answer` reads them the same way, without this check, until the same entry adds it — `viola hook pre-tool-use|permission-request` stays under the first;» with «`viola answer` the same way, and `viola hook pre-tool-use|permission-request` past the hook''s instance check;»'
    sidecar: 'Auth & Authz: the entry-point list''s `answer` / `hook.dialog` gap retired.'
    rationale: 'Report **Retired** (6).'
    basis: viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/report.md:26
    dependent-of: D-retired
  - detector: D-retired
    severity: warning
    file: .andromeda/security-plan-detail/authentication-authorization.md
    near: 37
    section: '## Authentication & Authorization'
    change: 'Replace «a seventh, the founder''s ruling F-W1, live, 2026-10-04, relayed by the overseer: `viola pause` and `viola release` read them the same way, without this check, until the same entry adds them;» with «`viola pause` and `viola release` the same way;»'
    sidecar: 'Auth & Authz: the entry-point list''s `pause` / `release` gap retired.'
    rationale: 'Report **Retired** (7).'
    basis: viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/report.md:26
    dependent-of: D-retired
  - detector: D-retired
    severity: warning
    file: .andromeda/security-plan-detail/authentication-authorization.md
    near: 37
    section: '## Authentication & Authorization'
    change: 'Replace «an eighth, the founder''s own answer to P4 fork Q1, live, 2026-10-17, relayed by the overseer (`inputs#I2` of chunk 2026-10-17-mcp-server-for-drivers): `viola mcp`''s `send` / `wait` / `last` / `answer` tools read them the same way, through `viola_state::liveness::live_endpoint`, without this check, until the same entry adds them —» with «`viola mcp`''s `send` / `wait` / `last` / `answer` tools read them through `viola_state::liveness::live_endpoint` only after `check_instance` in `frame` —», the rest unchanged.'
    sidecar: 'Auth & Authz: the entry-point list''s `viola mcp` tools gap retired.'
    rationale: 'Report **Retired** (8); Symbols `viola-mcp` `tools.rs` `frame` (337-339).'
    basis: viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/report.md:14
    dependent-of: D-retired
  - detector: D-retired
    severity: warning
    file: .andromeda/security-plan-detail/authentication-authorization.md
    near: 37
    section: '## Authentication & Authorization'
    change: 'Replace «and the installed `command` gets no re-hash, its integrity owed to the route entry "Home and code-bearing file integrity")» with «and the installed `command` gets no re-hash: `marketplace/` is off the integrity set by the founder''s Epoch 5 batch (2026-10-20), CARRY 17 closed by that ruling with no work and no owner (`inputs#I4` row 2 of chunk 2026-10-26-home-and-code-bearing-file-integrity))».'
    sidecar: 'Auth & Authz: `marketplace/` integrity no longer owed to the closed entry; CARRY 17 closed by ruling.'
    rationale: 'Report Cross-project claims I4 row 2: CARRY 17 (the `marketplace/` tree''s integrity) closes with no work and no owner by the founder''s Epoch 5 batch; Count 1 `owner-title` (every row naming this entry as owner is stale).'
    basis: viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/report.md:67
    dependent-of: D-retired
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/authentication-authorization.md
    near: 38
    section: '## Authentication & Authorization'
    change: 'Replace «**Read rule:** also refuse when the home or `instances/<name>/` grants any group or other permission (`mode & 0o077 != 0`).» with «**Read rule:** also refuse when the home, `instances/<name>/` or `instances/<name>/diagnostics/` grants any group or other permission (`mode & 0o077 != 0`; `strict::unix_read_verdict`, the write rule plus that bit test, refused `Refused::Readable`). Before a role or detail file opens, `src/obs.rs`''s `check_diagnostics` runs `strict::check_diagnostics` on the existing `diagnostics/` folder and the file, before `ensure_private_dir` narrows anything: a symlink is refused (`Refused::Symlink`), a folder takes the read rule, a file the read rule on Unix and the write rule on Windows, an absent path passes; a refusal is a failed open, the writer ending at `std::io::sink`.»'
    sidecar: 'Auth & Authz: the Unix read rule as built (`unix_read_verdict`), extended to `diagnostics/`; the diagnostics opens'' symlink and mode check.'
    rationale: 'Report Symbols `viola_state::strict` (`unix_read_verdict` 209-217, `Refused::Readable` / `Symlink`, `check_diagnostics` 127-145) and `src/obs.rs` `check_diagnostics` (69-76); Count 3 `read-rule` re-read at this line.'
    basis: viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/report.md:20
  - detector: D-retired
    severity: warning
    file: .andromeda/security-plan-detail/authentication-authorization.md
    near: 39
    section: '## Authentication & Authorization'
    change: 'In the **Windows strict-modes check** list, after «(including the plugin folder''s `hooks/` and `.claude-plugin/` subfolders)», add «, on Windows x64 the `bin/<version>-<hash>/conpty/` folder and each companion in it (`OpenConsole.exe`, `conpty.dll`) that exists, before the held open (`check_present`)».'
    sidecar: 'Auth & Authz: the `conpty/` folder and companions join the Windows strict-modes set.'
    rationale: 'Report **Retired** (3): the gap was owed «until the same entry adds the `conpty/` folder and both files to the Windows set below»; Symbols `pin_companions` runs `check_present` on `bin/<key>/conpty/` and each companion.'
    basis: viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/report.md:13
    dependent-of: D-retired
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/authentication-authorization.md
    near: 39
    section: '## Authentication & Authorization'
    change: 'Replace «On failure `run` and `mcp` exit 1; every `hook` event prints nothing and exits 0; CLI client verbs report `instance-unreachable` (exit 21).» with «On failure `run`, `revive`, `verify` and `mcp`''s start exit 1 (`run` and `verify` the home-state pair, `run` the instance-state pair for an instance-level refusal); every `hook` event prints nothing and exits 0; CLI client verbs report `instance-unreachable` (exit 21) with the instance-state pair; `viola mcp`''s frame tools return `instance-unreachable` with `detail: null`; none writes a line.»'
    sidecar: 'Auth & Authz: the strict-modes failure outcomes gain `verify`, `revive`, the MCP frame tools and the pairs.'
    rationale: 'Report Symbols `refuse_tree` / `refuse_home` (exit 1, the pairs), `refuse_untrusted` (exit 21, the instance pair), `tools.rs` `frame`; Counts 8, 9, 20; Outcome (security) rows.'
    basis: viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/report.md:131
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/authentication-authorization.md
    near: 39
    section: '## Authentication & Authorization'
    change: 'After «A `--home` outside `%USERPROFILE%` gets an explicit protected user + SYSTEM DACL at creation (via windows-sys), or is refused if one cannot be set.» add: Measured on `windows-2025` (ci#40400887151, read back through `icacls /save`): such a home reads `D:PAI(A;OICI;FA;;;LA)(A;OICI;FA;;;SY)`, canonical SDDL spelling the runner''s user (local Administrator, RID 500) `LA`; a plain home a test made with `create_dir_all` before viola is refused `Writable`, the refused entry an inherited `S-1-5-32-545` (Users) write ACE.'
    sidecar: 'Auth & Authz: the protected `--home` DACL as read back on `windows-2025`, and a plain pre-made home''s refusal.'
    rationale: 'Report Cross-project claims (the measured SDDL) and Spec claims disproved (a `create_dir_all` home refused `Writable` on an inherited Users write ACE, ci#40398601594).'
    basis: viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/report.md:58
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/authentication-authorization.md
    near: 40
    section: '## Authentication & Authorization'
    change: 'After «so test the generic bits as well as the specific ones) to such a SID.» add: As built, `strict::windows_read_verdict` is the write rule plus any allow ACE granting a right in the pinned `READ_RIGHTS = 0x9000_0001` (`FILE_READ_DATA` | `GENERIC_READ` | `GENERIC_ALL`) to a SID `trusted_sid` rejects, inherit-only ACEs included, refused `Refused::Readable`; `check_instance` holds Windows `events.ndjson` to it.'
    sidecar: 'Auth & Authz: the Windows read rule as built (`windows_read_verdict`, `READ_RIGHTS`).'
    rationale: 'Report Symbols `viola_state::strict` (`windows_read_verdict` 279-296, `READ_RIGHTS`), Count 3 `read-rule`.'
    basis: viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/report.md:10
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/authentication-authorization.md
    near: 42
    section: '## Authentication & Authorization'
    change: 'After line 42 add a row: **Requirement:** `~/.viola/` access control, the checks as built; **Implementation:** `viola_state::strict` names each check a caller runs before its first read or open: `check_instance(home, instance_dir)` (above); `check_home_tree(home, name, key)` returning `TreeRefused { HomeLevel, InstanceLevel }` (the home, then with a name `check_instance`''s set, then `ledger/`, `bin/`, `plugin/`, then with a key `bin/<key>/`, the pinned `viola(.exe)`, `plugin/<key>/`, its `hooks/` and `.claude-plugin/`), called by `run` and `revive`''s start (`checked_exe`) and by `verify` (no name, no key); `check_diagnostics(path)` (the `diagnostics/` opens); `check_present(path)` (the write rule on one path that exists, an unstattable one refused: on Unix a home under a regular file is a strict-modes refusal, on Windows such a path stats `NotFound` and is skipped as absent); `check_stamps` (in `update_stamps` and `read_stamps_strict`); and the root bin''s `cmd::client::refuse_untrusted(home, name, json)`, the CLI frame verbs'' shared entry. Each refusal is a fixed `Refused` message (`Readable` and `Symlink` beside the write-rule variants) and opens nothing under the home; no `config.json` key, env var or flag switches one off. **Stack reference:** `crates/viola-state/src/{strict,stamps,pin}.rs`; root bin `src/cmd/{client,run,verify}.rs`, `src/obs.rs`; windows-sys 0.61.2.'
    sidecar: 'Auth & Authz: the tree checks (`check_home_tree`, `TreeRefused`, `check_diagnostics`, `check_present`, `refuse_untrusted`) registered.'
    rationale: 'Report Count 4 (`tree-checks`): the four new names stand in no master, a fact to register; Count 13 (an unstattable home a strict-modes refusal, Windows `NotFound`); `setting_does_not_disable_the_home_check` in `tests/cli_controls_not_disableable.rs`.'
    basis: viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/report.md:31
  - detector: D-retired
    severity: warning
    file: .andromeda/security-plan-detail/authentication-authorization.md
    near: 63
    section: '## Authentication & Authorization'
    change: 'Replace «taken with the `endpoint` from one `read_snapshot` call» with «taken with the `endpoint` from one `read_snapshot_or_replay` call, only a `Recovered::Snapshot` proceeding (a replayed snapshot carries no `endpoint`, `pid` or `started_at`, so `Replayed`, `Absent` or an error is no `LiveEndpoint`, and on the hook path `NotSent::Unreachable`; the readers on the replay are the founder''s own answer to P4 fork Q1, `inputs#I3` of chunk 2026-10-26-home-and-code-bearing-file-integrity), each after its caller''s strict-modes check».'
    sidecar: 'Auth & Authz: the verdict''s reference values come from the replay-aware read, never a replayed snapshot.'
    rationale: 'Report **Retired** `read_snapshot` as a product reader (3 → 0); Symbols `live_endpoint` (92-94) and hook `connect` (224-226); Count 5 `read-snapshot` stale at this line.'
    basis: viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/report.md:12
    dependent-of: D-retired
  - detector: D-retired
    severity: warning
    file: .andromeda/security-plan-detail/authentication-authorization.md
    near: 66
    section: '## Authentication & Authorization'
    change: 'Replace «No dated gap: both verbs run `viola_state::strict::check_instance` on the absolute home and `instances/<driven>` before `client::start`» with «Both verbs run `cmd::client::refuse_untrusted` (`viola_state::strict::check_instance` on the absolute home and `instances/<driven>`), shared with the six other frame verbs, before `client::start`», and replace «so they borrow none of the eight dated gaps.» with «as every frame verb''s do.»'
    sidecar: 'Auth & Authz: the link row no longer contrasts itself with the retired dated gaps; `refuse_untrusted` named.'
    rationale: 'Report **Retired** (all eight gaps) and Symbols (`link_call` moved onto `refuse_untrusted` unchanged in behaviour).'
    basis: viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/report.md:15
    dependent-of: D-retired
  - detector: D-security-auth
    severity: warning
    file: .andromeda/security-plan-detail/authentication-authorization.md
    near: 67
    section: '## Authentication & Authorization'
    change: 'Replace «root bin `src/cmd/link.rs` (`link_call`, shared by `src/cmd/unlink.rs`)» with «root bin `src/cmd/link.rs` (`link_call`, shared by `src/cmd/unlink.rs`) and `src/cmd/client.rs` (`refuse_untrusted`)».'
    sidecar: 'Auth & Authz: the link row''s stack reference names `refuse_untrusted`.'
    rationale: 'Report Count 2: `check_instance(` moved from `src/cmd/link.rs` to `src/cmd/client.rs` (`refuse_untrusted`, shared by link/unlink and the six verbs).'
    basis: viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/report.md:21
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/authentication-authorization.md
    near: 69
    section: '## Authentication & Authorization'
    change: 'Replace «and `link` / `unlink`''s strict-modes refusal (exit 21) comes before their liveness and server checks.» with «and every frame verb''s strict-modes refusal through `refuse_untrusted` (exit 21, the instance-state `unable:` / `hint:` pair through `human::refuse` or the same `instance-unreachable` document, no line) comes before its liveness and server checks, `send`''s after its text read.»'
    sidecar: 'Auth & Authz: the CLI refusal order covers all eight frame verbs'' strict-modes refusal.'
    rationale: 'Report Symbols `refuse_untrusted` called first by the six verbs and by `link_call`; Count 8 `instance-pair`.'
    basis: viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/report.md:15
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/authentication-authorization.md
    near: 70
    section: '## Authentication & Authorization'
    change: 'Replace «root bin `src/cmd/client.rs` (`connect`, `Unreachable::ServerUnverified`)» with «root bin `src/cmd/client.rs` (`connect`, `Unreachable::ServerUnverified`, `refuse_untrusted`)».'
    sidecar: 'Auth & Authz: the CLI refusal row''s stack reference names `refuse_untrusted`.'
    rationale: 'Report Symbols `src/cmd/client.rs` `refuse_untrusted` (20-41).'
    basis: viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/report.md:15
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/authentication-authorization.md
    near: 79
    section: '## Authentication & Authorization'
    change: 'After line 79 add a row: **Requirement:** `~/.viola/` access control, the tests holding the strict-modes controls; **Implementation:** `tests/security_negatives_strict_modes.rs` holds `frame_verb_refuses_a_widened_instance_before_any_log` (the six frame verbs, both modes, exit 21, no frame, no line, mode or ACL as found), `only_the_wrapper_writes_the_snapshot_and_only_verify_the_stamps` (byte-identical after every other writer-capable entry point, the wrapper stopped), `a_start_under_umask_000_makes_every_home_path_owner_only` (Unix), `a_home_another_user_may_read_refuses_run` (Unix 0750, Windows a Users read grant) and `a_home_viola_makes_outside_the_profile_takes_the_protected_dacl` (Windows); beside them `handlers_frame_tools_refuse_a_widened_instance_before_any_frame`, `hook_refused_by_the_instance_check_fails_open_and_writes_nothing`, `hook_statusline_a_widened_instance_runs_and_logs_nothing`, `run_refuses_a_widened_home_tree_before_its_log`, `verify_refuses_a_widened_ledger_before_any_step`, `run_on_a_companion_another_user_may_write_falls_back_to_conhost` (`windows-2025`), `setting_does_not_disable_the_home_check`, and the `strict` unit tables (`unix_read_verdict_*`, `windows_read_verdict_*`, `check_home_tree_*`, `check_diagnostics_*`, the `cfg(windows)` cases); green on three OSes (ci#40403389629). **Stack reference:** `tests/security_negatives_strict_modes.rs`, `crates/viola-mcp/tests/handlers.rs`, `tests/{hook_fail_open,hook_statusline,run_cli,cli_verify,conpty_sideload,cli_controls_not_disableable}.rs`, `crates/viola-state/src/strict.rs`.'
    sidecar: 'Auth & Authz: the strict-modes control tests registered.'
    rationale: 'Report Changes **Files** (tests) and New text, by line (the named test functions); Outcome (security) and (tests) rows MET on ci#40403389629.'
    basis: viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/report.md:134
note: 'D-security-input and D-security-deps fire nothing (no new external-input surface: the checks read only filesystem metadata, names already `ViolaName`; Dependencies none). The same retired gaps and `read_snapshot` wording stand in the body outside this file: security-plan.md:114, :118, :142, :154, :170 and :439.'
