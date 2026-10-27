proposals:
  - detector: D-retired
    severity: warning
    file: .andromeda/security-plan-detail/threat-model-summary.md
    near: 56
    section: "## Threat Model Summary"
    change: "In the as-built consequence, replace «The snapshot is read without the strict-modes check (owed to «Home and code-bearing file integrity»), except by CLI `link` / `unlink`, which read it only after `check_instance` passed on the driven instance» with: every one of these clients first runs the instance's strict-modes check (`check_instance`: the CLI verbs through `client::refuse_untrusted`, `viola hook` through `canonical` + `trusted` before obs init, the `viola mcp` tools before `live_endpoint`) and then reads the snapshot through `read_snapshot_or_replay`, a replayed or absent snapshot giving no endpoint and so no frame (chunk 2026-10-26-home-and-code-bearing-file-integrity, no dated gap left); keep «and macOS, where the peer reports no pid, stays a recorded gap (owed to «Unix endpoint and home hardening»)» as it stands."
    sidecar: "threat-model-summary 2.1: the snapshot read without strict-modes retired for every frame-writing client; macOS gap kept."
    rationale: "Report Retired: the eight founder-ratified dated strict-modes gaps, each now running the check before its first read or open; Symbols: `refuse_untrusted` (client.rs 20-41), hook.rs `canonical`/`trusted` (75-86), tools.rs 337-339, `live_endpoint` and hook `connect` on `read_snapshot_or_replay`."
    basis: "viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/report.md:26"
  - detector: D-cascade
    severity: warning
    file: .andromeda/security-plan-detail/threat-model-summary.md
    near: 87
    section: "## Threat Model Summary"
    change: "Add after the `hook statusline` trust line: every hook path (event, dialog and `hook statusline`) runs the instance's strict-modes check on the canonicalised `VIOLA_DIR` instance before anything under the home is read or opened, and a refused or absent instance exits 0 with no `config.json` read, no role file, no stdin read, no line and no frame (the `--capture` arms stay ahead of the check); `hook statusline` reads the snapshot through `read_snapshot_or_replay`, and a replayed snapshot carries no `statusline_command`, so nothing runs."
    sidecar: "threat-model-summary 5.1: the hook paths' instance check and statusline's replay read added to the hook-stdin trust boundary."
    rationale: "Report Symbols: src/cmd/hook.rs `canonical` (75-81) and `trusted` (83-86) ahead of `read_diagnostics_level` / `viola_obs_init` on the event and dialog paths; src/cmd/hook/statusline.rs `serve` (128-132) on `read_snapshot_or_replay`, a replayed snapshot running nothing."
    basis: "viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/report.md:16"
  - detector: D-cascade
    severity: warning
    file: .andromeda/security-plan-detail/threat-model-summary.md
    near: 96
    section: "## Threat Model Summary"
    change: "Add after the input-check line: before its frame, each of `send`, `wait`, `last` and `answer` runs the instance's strict-modes check (`check_instance` on the home and the instance directory) ahead of the liveness pre-check and the server check, so a refusal (`instance-unreachable`, `detail: null`, no frame, no line) now comes from the instance's strict-modes check as well as from liveness."
    sidecar: "threat-model-summary 6.1: the MCP frame tools' instance strict-modes check added to the MCP trust boundary."
    rationale: "Report Symbols: `viola-mcp` tools.rs `frame` (337-339) runs `check_instance` before `live_endpoint`, a refusal returning `result::unreachable()`."
    basis: "viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/report.md:14"
  - detector: D-cascade
    severity: warning
    file: .andromeda/security-plan-detail/threat-model-summary.md
    near: 111
    section: "## Threat Model Summary"
    change: "Extend the CLI trust boundary with the home's strict-modes check before the first read or open: `run`, `revive`'s start and `verify` check the home tree (`check_home_tree`; a refusal exit 1, no log), and the frame verbs `send`, `wait`, `last`, `answer`, `pause`, `release`, `link` and `unlink` check the instance (`check_instance`; a refusal exit 21), so a state tree chosen by `--home` or `VIOLA_DIR` that another user can write, or whose home, instance or `diagnostics/` another user can read, is refused."
    sidecar: "threat-model-summary 7.1: the CLI boundary gains the home-tree and instance strict-modes checks."
    rationale: "Report Symbols: run.rs `checked_exe` / `refuse_tree` (189-215) before `open_wrapper_log`, revive.rs 125-135, verify.rs 64-70, client.rs `refuse_untrusted` called first by the six verbs and `link_call`."
    basis: "viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/report.md:18"
  - detector: D-cascade
    severity: warning
    file: .andromeda/security-plan-detail/threat-model-summary.md
    near: 124
    section: "## Threat Model Summary"
    change: "Replace «has two product readers, `viola revive`'s preflight and `viola list` (…), each taking it after the strict-modes check on the instance's files; no other product reader takes it» with: its product readers are `viola revive`'s preflight, `viola list` (`viola_state::board::read`), `viola_state::liveness::live_endpoint` (the CLI frame verbs and the `viola mcp` tools), `viola hook`'s `connect`, `hook statusline`'s `serve` and `run`'s collision check, each taking it after the strict-modes check on the instance's files; a replayed snapshot carries no `endpoint`, `pid`, `started_at` or `statusline_command`, so it yields no frame and runs no command (the founder's boundary-widening word, `inputs#I3` Q1 of chunk 2026-10-26-home-and-code-bearing-file-integrity)."
    sidecar: "threat-model-summary 8.1: the replay's product readers named after four readers moved onto `read_snapshot_or_replay`."
    rationale: "Report Counts 5: replay product readers 2 (`revive`, `list`) → 6 (+ hook `connect`, statusline `serve`, `run`'s `collision_check`, `live_endpoint`); Decisions: the founder's P4 Q1 ratification."
    basis: "viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/report.md:32"
  - detector: D-cascade
    severity: warning
    file: .andromeda/security-plan-detail/threat-model-summary.md
    near: 126
    section: "## Threat Model Summary"
    change: "Replace «Nothing checks integrity beyond the content hash in the folder name.» with: beyond the content hash in the folder name, `run` and `revive`'s start hold the home tree to the strict-modes check (Unix owner and mode, Windows owner and DACL) before the wrapper log opens (`check_home_tree`: the home, the instance, `ledger/`, `bin/`, `plugin/`, `bin/<key>/`, the pinned `viola(.exe)`, `plugin/<key>/`, its `hooks/` and `.claude-plugin/`), and on Windows `pin_companions` checks `bin/<key>/conpty/` and each companion before the held open; `marketplace/` stays off the integrity set (CARRY 17 closed by ruling)."
    sidecar: "threat-model-summary 8.1: the code-bearing artefacts now mode/DACL-checked at start; the content-hash-only line retired."
    rationale: "Report Symbols: strict.rs `check_home_tree` (91-125), run.rs `checked_exe` before `open_wrapper_log`, pin.rs `pin_companions` running `check_present`; Expected amendments: CARRY 17 closed by ruling (`inputs#I4` row 2)."
    basis: "viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/report.md:10"
  - detector: D-retired
    severity: warning
    file: .andromeda/security-plan-detail/authentication-authorization.md
    near: 25
    section: "## Authentication & Authorization"
    change: "Replace the «Interim gap (ratified), its strict-modes half only» and «Second interim gap» paragraphs with one statement, no dated gap: every client runs the instance's strict-modes check before it reads the snapshot (`viola hook`'s event and dialog paths through `canonical` + `trusted` in src/cmd/hook.rs before obs init; CLI `send` through `client::refuse_untrusted` after its text read), then reads it through `read_snapshot_or_replay`, only a parsed snapshot giving reference values, then on Linux and Windows the server check before the frame (macOS the recorded gap); the hook.event and `send` gaps are retired (chunk 2026-10-26-home-and-code-bearing-file-integrity). Keep «`send` is served by the wrapper as of chunk 2026-10-04-confirmed-send-with-cl-1-records, under the channel row's controls (…)» without «and this gap»."
    sidecar: "authentication-authorization 1.6: the hook.event and `send` dated strict-modes gaps retired."
    rationale: "Report Retired (1) and (4); Symbols: hook.rs `canonical`/`trusted` and hook `connect` (224-226) on `read_snapshot_or_replay`, client.rs `refuse_untrusted` called first by `send` (send.rs:190-192)."
    basis: "viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/report.md:26"
  - detector: D-retired
    severity: warning
    file: .andromeda/security-plan-detail/authentication-authorization.md
    near: 26
    section: "## Authentication & Authorization"
    change: "Retire the third, fourth and fifth interim gaps here (CLI `wait` / `last`; CLI `answer` and `hook.dialog`; CLI `pause` / `release`): each now runs the instance's strict-modes check first (`client::refuse_untrusted`; the hook dialog path through `trusted`) before the liveness pre-check and the server check; keep each «served by the wrapper as of chunk …, under the channel row's controls» sentence without «and this gap», and drop the «borrow neither / none of the earlier gaps» clauses."
    sidecar: "authentication-authorization 1.6: the `wait` / `last`, `answer` / `hook.dialog` and `pause` / `release` dated gaps retired."
    rationale: "Report Retired (5), (6), (7); Symbols: `refuse_untrusted` called first by wait (43-45), last (30-32), answer (94-96), pause (32-34), release (35-37)."
    basis: "viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/report.md:15"
    dependent-of: D-retired
  - detector: D-retired
    severity: warning
    file: .andromeda/security-plan-detail/authentication-authorization.md
    near: 27
    section: "## Authentication & Authorization"
    change: "Retire the sixth interim gap here (the eighth overall, `viola mcp`'s `send` / `wait` / `last` / `answer` tools): each tool now runs `check_instance` on the home and the instance directory before `live_endpoint`, a refusal returning `instance-unreachable` with `detail: null`, no frame and no line; keep the after-`TokioClient::connect` server check (`server_verified`) as it stands."
    sidecar: "authentication-authorization 1.6: the `viola mcp` tools' dated strict-modes gap retired."
    rationale: "Report Retired (8); Symbols: tools.rs `frame` (337-339)."
    basis: "viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/report.md:14"
    dependent-of: D-retired
  - detector: D-cascade
    severity: warning
    file: .andromeda/security-plan-detail/authentication-authorization.md
    near: 27
    section: "## Authentication & Authorization"
    change: "In **Collision check**, replace «reads the snapshot through `read_snapshot`» with: takes the caller's one read through `read_snapshot_or_replay` (`run` after its home-tree check; `revive`'s preflight reads once and hands that read to both its collision check and its recorded-directory reading), a parsed snapshot judged by its `pid` + OS start time and the heartbeat, a replayed one by the beat alone (`beat_age` within `STALE_AFTER` → `already-live`, else free), an error or an absent snapshot free; it still opens no connection and writes no frame, so the peer-pid server check stays moot."
    sidecar: "authentication-authorization 1.6: the collision check reads the caller's one replay-aware read."
    rationale: "Report Symbols: run.rs `collision_check` takes `Option<&Recovered>` (425-432), `Replayed` → liveness from the beat alone; Deviations 4: revive reads once for both; Counts 5: `read_snapshot(` product callers 3 → 0."
    basis: "viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/report.md:18"
  - detector: D-cascade
    severity: warning
    file: .andromeda/security-plan-detail/authentication-authorization.md
    near: 28
    section: "## Authentication & Authorization"
    change: "Extend the stack reference: `viola-state` `strict::check_instance` (called by the root bin's `cmd::client::refuse_untrusted` and `cmd::hook`'s `trusted`, and by `viola-mcp`'s `frame`) and `replay::read_snapshot_or_replay` (read by `live_endpoint` and `cmd::hook`'s `connect`) beside the liveness functions."
    sidecar: "authentication-authorization 1.6: stack reference names the instance check and the replay-aware read."
    rationale: "Report Symbols: client.rs `refuse_untrusted`, hook.rs `trusted`, tools.rs 337-339, liveness.rs 92-94 and hook.rs 224-226 on `read_snapshot_or_replay`."
    basis: "viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/report.md:21"
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/authentication-authorization.md
    near: 27
    section: "## Authentication & Authorization"
    change: "Add after **On mismatch**: **On a strict-modes refusal** (ahead of the liveness and server checks) no line is written at any of the new entry points, no `strict-modes-failed` included, as `link` / `unlink` before them, and the six frame verbs print `link`'s instance-state pair (exit 21); `viola hook` exits 0 with no body and no line; MCP returns `instance-unreachable` with `detail: null` (the founder's word, `inputs#I3` Q2 and Q3 of chunk 2026-10-26-home-and-code-bearing-file-integrity)."
    sidecar: "authentication-authorization 1.6: a refused strict-modes check writes no line; the six frame verbs print link's instance pair."
    rationale: "Report Decisions: the founder's P4 answers Q2 (no line on a refused check, as link) and Q3 (link's instance pair for the six frame verbs)."
    basis: "viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/report.md:111"
    fact: F14
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/authentication-authorization.md
    near: 34
    section: "## Authentication & Authorization"
    change: "In `revive`'s clause, replace «which reads the home, `instances/`, the instance directory, its `snapshot.json` and its `events.ndjson`, each that exists and in that order» with: which holds the home, the instance directory and its `diagnostics/` (new in the set) to the read rule, `instances/` and `snapshot.json` to the write rule, and `events.ndjson` to the write rule on Unix and the read rule on Windows, each that exists, a folder before what it holds."
    sidecar: "authentication-authorization 1.8: `check_instance`'s set widened by the read rule and `diagnostics/`."
    rationale: "Report Symbols: strict.rs `check_instance` (signature unchanged) now reads the home (READ), `instances/` (write), the instance directory (READ), `diagnostics/` (READ, new), `snapshot.json` (write), `events.ndjson` (Unix write; Windows READ)."
    basis: "viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/report.md:10"
    fact: F5
  - detector: D-cascade
    severity: warning
    file: .andromeda/security-plan-detail/authentication-authorization.md
    near: 34
    section: "## Authentication & Authorization"
    change: "Replace «proven by `cfg(unix)` cases, with no Windows case that widens an instance tree's DACL yet, owed to the entry «Home and code-bearing file integrity»» with: proven by `cfg(unix)` cases and by `cfg(windows)` cases green on `windows-2025` (an instance tree granted Users write refused beside a `create_private_dir` tree that passes, a `diagnostics/` granted Users read refused, a plain home made before viola refused `Writable`)."
    sidecar: "authentication-authorization 1.8: the Windows widened-instance-tree case built; its owed clause retired."
    rationale: "Report Spec claims disproved: «No case widens the DACL of an instance tree on Windows yet» now covered by the `cfg(windows)` cases green on ci#40403389629; Symbols: strict.rs `cfg(windows)` cases 708-830."
    basis: "viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/report.md:73"
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/authentication-authorization.md
    near: 34
    section: "## Authentication & Authorization"
    change: "Add after `list`'s clause: `board::scan` runs `check_path` (the write rule) on the home and then `check_instance` per instance, which now holds the home to the read rule, so a 0750 home passes the board with no instance and is refused, naming the instance (`ScanRefused::Instance`), once it holds one; `board.rs` is unchanged and the inconsistency is a CARRY on «Unix endpoint and home hardening»."
    sidecar: "authentication-authorization 1.8: `viola list`'s home-check inconsistency recorded with its owner."
    rationale: "Report Counts 14: `board::scan` against the widened `check_instance`; Route asks: the owner a CARRY on the markerless entry «Unix endpoint and home hardening»."
    basis: "viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/report.md:41"
    fact: F16
  - detector: D-cascade
    severity: warning
    file: .andromeda/security-plan-detail/authentication-authorization.md
    near: 35
    section: "## Authentication & Authorization"
    change: "In `link` / `unlink`'s clause, say the check now runs through `client::refuse_untrusted` (`check_instance` on the absolute home and `instances/<name>`), which the six frame verbs `send`, `wait`, `last`, `answer`, `pause` and `release` call first too, each before `client::start`, so before any role file or log opens and before any snapshot value is read, with the same exit-21 refusal shapes."
    sidecar: "authentication-authorization 1.8: the six frame verbs join link / unlink's instance check."
    rationale: "Report Symbols: client.rs `refuse_untrusted` (20-41), called first by send, wait, last, answer, pause, release; `link_call` (link.rs:94-95) moved onto it unchanged in behaviour."
    basis: "viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/report.md:15"
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/authentication-authorization.md
    near: 35
    section: "## Authentication & Authorization"
    change: "In `mcp`'s clause, replace «its frame tools then read `snapshot.json`'s `endpoint` under the eighth dated gap, without the check» with: its frame tools then run `check_instance` on the home and the instance directory before `live_endpoint`, a refusal returning `instance-unreachable` with `detail: null`, no frame and no line."
    sidecar: "authentication-authorization 1.8: the `viola mcp` frame tools run the instance check before `live_endpoint`."
    rationale: "Report Symbols: tools.rs `frame` (337-339); Retired (8)."
    basis: "viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/report.md:14"
    fact: F13
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/authentication-authorization.md
    near: 35
    section: "## Authentication & Authorization"
    change: "In the `hook <event>` clause, add: the event and dialog paths take the same instance check as `hook statusline` (`canonical` plus `trusted`, now shared in src/cmd/hook.rs) before the diagnostics level is read and before obs init; a refused or absent instance exits 0 with no `config.json` read, no role file, no stdin read, no line, no frame and no SessionEnd append; the two `--capture` arms stay ahead of the check, unchanged."
    sidecar: "authentication-authorization 1.8: the hook event and dialog paths check the instance before obs init."
    rationale: "Report Symbols: hook.rs `canonical` (75-81) and `trusted` (83-86) moved from statusline.rs, the event and dialog paths taking `instance_of(…).and_then(canonical).filter(trusted)` before `read_diagnostics_level` / `viola_obs_init`."
    basis: "viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/report.md:16"
    fact: F11
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/authentication-authorization.md
    near: 35
    section: "## Authentication & Authorization"
    change: "Add beside the hook clause: a `VIOLA_DIR` naming an instance directory that does not exist canonicalises to no session, so the hook writes nothing (as `hook statusline` already did)."
    sidecar: "authentication-authorization 1.8: a hook whose VIOLA_DIR names no directory writes nothing."
    rationale: "Report Symbols (hook.rs) and Deviations 5: a `VIOLA_DIR` naming an instance directory that does not exist canonicalises to no session."
    basis: "viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/report.md:98"
    fact: F12
  - detector: D-retired
    severity: warning
    file: .andromeda/security-plan-detail/authentication-authorization.md
    near: 36
    section: "## Authentication & Authorization"
    change: "Retire «interim gap, ratified: until the Epoch 6 entry … lands» and the second to fifth gaps on this line: `viola hook <event>` now checks the instance before reading `snapshot.json`; `viola verify`'s `update_stamps` runs `check_stamps` before `create_private_dir(ledger/)` and again under its lock, a refusal reading and writing nothing; Windows `pin_companions` checks `bin/<key>/conpty/` and each companion (`check_present`) before the held open, a refusal the inbox ConPTY with `sideload_fallback` `strict-modes-failed`; `viola send`, `wait` and `last` check the instance through `client::refuse_untrusted` before any snapshot value is read (chunk 2026-10-26-home-and-code-bearing-file-integrity, dated gaps 8 → 0, by the Retired list); keep `run`'s version-gate sentence (`read_stamps_strict`) as it stands."
    sidecar: "authentication-authorization 1.8: the hook.event, update_stamps, ConPTY owner/DACL, send and wait / last dated gaps retired."
    rationale: "Report Retired (1)–(5); Counts 1 (pattern gap-until, authentication-authorization.md:36 stale); Symbols: stamps.rs `StampsError`, pin.rs `pin_companions` / `PinError::StrictModes`, client.rs `refuse_untrusted`."
    basis: "viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/report.md:28"
  - detector: D-retired
    severity: warning
    file: .andromeda/security-plan-detail/authentication-authorization.md
    near: 37
    section: "## Authentication & Authorization"
    change: "Retire the sixth, seventh and eighth gaps on this line (`viola answer`, with `hook pre-tool-use|permission-request`; `viola pause` / `viola release`; the `viola mcp` frame tools): each now runs the instance's strict-modes check before any snapshot value is read; keep `mcp`'s own start check and `viola plugin install`'s check as they stand."
    sidecar: "authentication-authorization 1.8: the answer / hook.dialog, pause / release and viola mcp dated gaps retired."
    rationale: "Report Retired (6), (7), (8); Symbols: `refuse_untrusted` in answer, pause, release; hook.rs `trusted` on the dialog path; tools.rs 337-339."
    basis: "viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/report.md:26"
    dependent-of: D-retired
  - detector: D-cascade
    severity: warning
    file: .andromeda/security-plan-detail/authentication-authorization.md
    near: 37
    section: "## Authentication & Authorization"
    change: "In `viola plugin install`'s clause, replace «the installed `command` gets no re-hash, its integrity owed to the route entry «Home and code-bearing file integrity»» with: the installed `command` gets no re-hash, `marketplace/` being off the integrity set by the founder's Epoch 5 word (CARRY 17 closed by ruling, no owner)."
    sidecar: "authentication-authorization 1.8: the marketplace/ integrity owner retired; CARRY 17 closed by ruling."
    rationale: "Report Expected amendments: CARRY 17 closed by ruling (`inputs#I4` row 2, the founder's Epoch 5 batch: «`marketplace/` off the integrity set»); Counts 1: every owner-title row stale."
    basis: "viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/report.md:67"
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/authentication-authorization.md
    near: 38
    section: "## Authentication & Authorization"
    change: "Extend the Unix **Read rule** as built: `strict::unix_read_verdict` refuses the write rule's failures plus `mode & 0o077 != 0`, held on the home, `instances/<name>/` and its `diagnostics/`; its Windows counterpart `strict::windows_read_verdict` refuses any allow ACE, inherit-only included, granting a right in `READ_RIGHTS` 0x9000_0001 (`FILE_READ_DATA` | `GENERIC_READ` | `GENERIC_ALL`) to a SID other than the user, SYSTEM, Administrators or `CREATOR OWNER`."
    sidecar: "authentication-authorization 1.8: the read rule built (`unix_read_verdict`, `windows_read_verdict`, `READ_RIGHTS`)."
    rationale: "Report Symbols: strict.rs `unix_read_verdict` (209-217) and `windows_read_verdict` (279-296) with the pinned `READ_RIGHTS`; Counts 3: the read rule built."
    basis: "viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/report.md:10"
    fact: F4
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/authentication-authorization.md
    near: 39
    section: "## Authentication & Authorization"
    change: "Extend «On failure `run` and `mcp` exit 1» with `verify`: `viola verify` runs `check_home_tree(home, None, None)` (the home, `ledger/`, `bin/`, `plugin/`) before its `cli` log, a refusal the home pair, exit 1, with no step line and no summary; its `update_stamps` `StampsError::StrictModes` maps to the same pair."
    sidecar: "authentication-authorization 1.8: `viola verify` checks the home tree before its log."
    rationale: "Report Symbols: verify.rs 64-70 (`check_home_tree(&home, None, None)`), `refuse_home` 95-99, `measure` 288-296."
    basis: "viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/report.md:19"
    fact: F2
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/authentication-authorization.md
    near: 40
    section: "## Authentication & Authorization"
    change: "Add after the Windows read rule: a refusal names the property it refused, `Refused` gaining `Readable` («a trusted state path is readable by another user») and `Symlink` («a trusted state path is a symbolic link»), fixed messages, so neither the read rule nor the symlink check borrows `Writable`."
    sidecar: "authentication-authorization 1.8: `Refused` gains `Readable` and `Symlink`."
    rationale: "Report Symbols: strict.rs `Refused` gains `Readable` and `Symlink`, fixed messages; Deviations 2."
    basis: "viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/report.md:95"
    fact: F6
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/authentication-authorization.md
    near: 41
    section: "## Authentication & Authorization"
    change: "Add after «The outcomes match Unix»: one outcome differs by OS: on Unix an unstattable home (a path under a regular file) is a strict-modes refusal (`verify` exit 1 with the home pair, the frame verbs exit 21 with the instance pair); on Windows such a path stats `NotFound`, passes as absent, and the catch-site line stands."
    sidecar: "authentication-authorization 1.8: an unstattable home is a refusal on Unix, absent on Windows."
    rationale: "Report Counts 13 and Decisions (measured on Windows: a path under a regular file stats `NotFound`); Deviations 8: the per-OS oracle."
    basis: "viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/report.md:40"
    fact: F15
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/authentication-authorization.md
    near: 41
    section: "## Authentication & Authorization"
    change: "Add the witnesses: `tests/security_negatives_strict_modes.rs` proves the strict-modes refusals on all three OSes (the six frame verbs, a start under umask 000, the Windows protected DACL read back, a read-granted pre-existing home refusing `run`, and single writers with the wrapper stopped), and `tests/chaos_snapshot_replay.rs` the replayed readers (E5 steps 1–3)."
    sidecar: "authentication-authorization 1.8: the strict-modes and replay witnesses named."
    rationale: "Report Outcome: `binary(security_negatives_strict_modes)` and `binary(chaos_snapshot_replay)` green on all three CI legs (ci#40403389629)."
    basis: "viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/report.md:128"
    fact: F21
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/authentication-authorization.md
    near: 42
    section: "## Authentication & Authorization"
    change: "Add a new entity after this one — Requirement: `~/.viola/` access control, the home tree at start; Implementation: `viola_state::strict::check_home_tree(home, name, key)` returns `TreeRefused { HomeLevel, InstanceLevel }`; it checks the home (read rule), then with a name `check_instance`'s set, then `ledger/`, `bin/`, `plugin/`, then with a key `bin/<key>/`, the pinned `viola(.exe)`, `plugin/<key>/`, its `hooks/` and its `.claude-plugin/` (write rule), each that exists; `run` and `revive`'s start run it (through `checked_exe`) before the wrapper log opens, a refusal exit 1 with the home or the instance pair (`refuse_tree`), no log, no line, no child; Stack reference: `viola-state` `strict`; root bin `src/cmd/run.rs` (`checked_exe`, `refuse_tree`), `src/cmd/revive.rs`."
    sidecar: "authentication-authorization: new entity for `check_home_tree` at `run` / `revive` start."
    rationale: "Report Symbols: strict.rs `TreeRefused` (83-89), `check_home_tree` (91-125); run.rs `checked_exe` (189-200), `refuse_tree` (202-215), run 166-172, revive.rs 125-135."
    basis: "viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/report.md:18"
    fact: F1
  - detector: D-cascade
    severity: warning
    file: .andromeda/security-plan-detail/authentication-authorization.md
    near: 63
    section: "## Authentication & Authorization"
    change: "Replace «taken with the `endpoint` from one `read_snapshot` call» with: taken with the `endpoint` from one `read_snapshot_or_replay` call made after the instance's strict-modes check, only a parsed snapshot (`Recovered::Snapshot`) giving reference values — a replayed or absent snapshot is no `LiveEndpoint` for the CLI frame verbs and `viola mcp`, and `NotSent::Unreachable` (fail open) for `viola hook`'s `connect`."
    sidecar: "authentication-authorization 3.2: the verdict's reference values read through `read_snapshot_or_replay`."
    rationale: "Report Symbols: liveness.rs 92-94 (`live_endpoint` reads `read_snapshot_or_replay`, only `Recovered::Snapshot` proceeds), hook.rs 224-226 (`Replayed` / `Absent` → `NotSent::Unreachable`); Counts 5: `read_snapshot(` product callers 3 → 0."
    basis: "viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/report.md:12"
  - detector: D-cascade
    severity: warning
    file: .andromeda/security-plan-detail/authentication-authorization.md
    near: 66
    section: "## Authentication & Authorization"
    change: "Say both verbs run `check_instance` through `client::refuse_untrusted` (shared with the six frame verbs), and replace «so they borrow none of the eight dated gaps» with: the order every CLI frame verb now takes, the eight dated gaps being retired."
    sidecar: "authentication-authorization 3.3: link / unlink's check now `refuse_untrusted`, shared; the eight-gap clause retired."
    rationale: "Report Symbols: `link_call` (link.rs:94-95) moved onto `refuse_untrusted` unchanged in behaviour; Retired: the eight dated gaps."
    basis: "viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/report.md:15"
  - detector: D-cascade
    severity: warning
    file: .andromeda/security-plan-detail/authentication-authorization.md
    near: 67
    section: "## Authentication & Authorization"
    change: "Add `src/cmd/client.rs` (`refuse_untrusted`, where the `check_instance` call now sits) to the stack reference beside `src/cmd/link.rs`."
    sidecar: "authentication-authorization 3.3: stack reference names `client::refuse_untrusted`."
    rationale: "Report Symbols, remaining-caller facts: `src/cmd/link.rs:98` → `src/cmd/client.rs:31` (`refuse_untrusted`, shared by link/unlink and the six verbs)."
    basis: "viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/report.md:21"
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/authentication-authorization.md
    near: 69
    section: "## Authentication & Authorization"
    change: "Replace «and `link` / `unlink`'s strict-modes refusal (exit 21) comes before their liveness and server checks» with: and every frame verb's strict-modes refusal comes before its liveness and server checks: `client::refuse_untrusted` runs `check_instance` first in `send` (after its text read), `wait`, `last`, `answer`, `pause`, `release` and `link` / `unlink`, a refusal exiting 21 with the instance-state `unable:` / `hint:` pair, or under `--json` the `instance-unreachable` document with `detail: null`, opening nothing under the home and writing no line."
    sidecar: "authentication-authorization 3.4: every CLI frame verb's strict-modes refusal precedes its liveness and server checks."
    rationale: "Report Symbols: client.rs `refuse_untrusted` (20-41) called first by the six verbs and `link_call`; Decisions: the founder's P4 Q3 (link's instance pair for the six frame verbs)."
    basis: "viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/report.md:15"
    fact: F3
  - detector: D-cascade
    severity: warning
    file: .andromeda/security-plan-detail/authentication-authorization.md
    near: 70
    section: "## Authentication & Authorization"
    change: "Extend the stack reference's `src/cmd/client.rs` entry with `refuse_untrusted` (the strict-modes refusal) beside `connect` and `Unreachable::ServerUnverified`."
    sidecar: "authentication-authorization 3.4: stack reference names `refuse_untrusted`."
    rationale: "Report Symbols: client.rs `refuse_untrusted` (20-41)."
    basis: "viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/report.md:15"
note: "Not mine, for the orchestrator: F7–F10, F17–F20 (data-protection, input-validation, error-handling) and the body's broad lines (security-plan.md:114 «a dated gap owed to Home and code-bearing file integrity», :118 «with its dated interim gaps», :47 «nothing checks integrity beyond the content hash») move with the same retirements; authentication-authorization.md:35's refuse-count hit (@c931) shows no figure in the picked text, so no count edit is proposed; the D-security-input / -auth / -deps detectors find no drift (no new external input, no auth library or secret change, no dependency)."
