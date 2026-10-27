proposals:
  - detector: D-retired
    severity: warning
    file: .andromeda/security-plan-detail/input-validation.md
    near: 30
    section: '## Input Validation'
    change: 'Replace the passage from «Interim gaps (ratified, until the Epoch 6 entry "Home and code-bearing file integrity"):» through «and is otherwise a silent no-op (Decisions Log `2026-09-28`);» with the landed form: `viola hook <event>`, on its nine `HookEvent` arms (the event and dialog paths), takes the `instance_of` instance, canonicalises its directory (`canonical`) and runs `viola_state::strict::check_instance` on it (`trusted`; both in `src/cmd/hook.rs`, shared with `hook statusline`) before `read_diagnostics_level` and obs init, and still requires an absolute `VIOLA_DIR` ending `<home>/instances/<name>` and a `VIOLA_NAME` that passes `ViolaName::try_new`: a refused or absent instance, or a `VIOLA_DIR` naming an instance directory that does not exist, exits 0 at once with no `config.json` read, no role file, no stdin read, no line, no frame and no SessionEnd append (Hook stdin row); `hook statusline` keeps its order (canonicalise, `check_instance`, then its log and stdin); keep the following capture-arm text unchanged.'
    sidecar: 'input-validation CLI arguments row: the hook <event> interim strict-modes gap retired; the event and dialog paths run canonical + check_instance before obs init (chunk 2026-10-26-home-and-code-bearing-file-integrity).'
    rationale: 'The report''s Retired bullet retires gap (1) (`hook.event` read without strict-modes) and gap (6) (`hook.dialog`); Symbols: `src/cmd/hook.rs` `canonical` / `trusted` before `read_diagnostics_level` / `viola_obs_init`, a refused or absent instance exit 0 at once with nothing read or written; Deviation 5 (a `VIOLA_DIR` naming no directory writes nothing).'
    basis: .andromeda/security-plan-detail/input-validation.md:30
  - detector: D-retired
    severity: warning
    file: .andromeda/security-plan-detail/input-validation.md
    near: 30
    section: '## Input Validation'
    change: 'Replace «`viola verify` reads `ledger/stamps.json` before any strict-modes check (Decisions Log `2026-09-28`, stamps read);» with: `viola verify` runs `strict::check_home_tree(home, None, None)` (the home, `ledger/`, `bin/`, `plugin/`) before its `cli` log, a refusal printing the home pair, exit 1, no step line, no summary and no line; its `update_stamps` runs `strict::check_stamps` before `create_private_dir(ledger/)` and again under the lock before the capped read, a refusal `StampsError::StrictModes` that reads and writes nothing, mapped by `verify` to the same home pair;'
    sidecar: 'input-validation CLI arguments row: the verify stamps-read interim gap retired; check_home_tree before the cli log, check_stamps before create_private_dir and under the lock (StampsError::StrictModes).'
    rationale: 'The report''s Retired bullet retires gap (2) (`update_stamps` reading `ledger/stamps.json` without strict-modes); Symbols: `viola_state::stamps` `StampsError` and the check before `create_private_dir`, `src/cmd/verify.rs` `check_home_tree` before its `cli` log and `refuse_home`; Count 17 marks this file''s stamps-gap wording stale.'
    basis: .andromeda/security-plan-detail/input-validation.md:30
    dependent-of: D-retired
  - detector: D-retired
    severity: warning
    file: .andromeda/security-plan-detail/input-validation.md
    near: 31
    section: '## Input Validation'
    change: 'Replace the five interim gaps (from «A fourth interim gap (the founder''s ruling, live, 2026-10-04, relayed by the overseer)» through «read it the same way (`viola_state::liveness::live_endpoint`).») with the landed form: `viola send` (after its text read and its `refusal_of` checks), `viola wait`, `viola last`, `viola answer`, `viola pause` and `viola release` first run `client::refuse_untrusted` (`check_instance` on the absolute home and `instances/<name>`): a refusal is exit 21 with the instance pair, or under `--json` exactly `{"v":1,"error":"instance-unreachable","detail":null}`, opening nothing under the home and writing no line; only then do they resolve the endpoint, `pid` and `started_at` through `live_endpoint` (only a read snapshot proceeds; a replayed or absent one is no endpoint) and, on Linux and Windows, verify the serving process before the frame (IPC client-side server verification row); `viola mcp`''s `send` / `wait` / `last` / `answer` tools run `check_instance` before `live_endpoint`, a refusal `instance-unreachable` with `detail: null`, no frame and no line.'
    sidecar: 'input-validation CLI arguments row: the fourth to eighth interim strict-modes gaps (send, wait/last, answer, pause/release, the mcp frame tools) retired; refuse_untrusted / check_instance first, exit 21 instance pair, no line.'
    rationale: 'The report''s Retired bullet retires gaps (4) to (8); Symbols: `src/cmd/client.rs` `refuse_untrusted` called first by the six verbs, `viola-mcp` `tools.rs` `frame` running `check_instance` before `live_endpoint`, `live_endpoint` proceeding only on `Recovered::Snapshot`; Decisions: no line on a refused check and link''s instance pair for the six frame verbs (`inputs#I3` Q2, Q3).'
    basis: .andromeda/security-plan-detail/input-validation.md:31
  - detector: D-retired
    severity: warning
    file: .andromeda/security-plan-detail/input-validation.md
    near: 32
    section: '## Input Validation'
    change: 'Replace «so `viola revive` is not among this row''s interim gaps» with «and the start arm then runs `checked_exe` (`src/cmd/run.rs`, shared with `run`: the exe read once, its `pin_key`, then `check_home_tree` with the name and the key — the home, the instance set, `ledger/`, `bin/`, `plugin/`, `bin/<key>/`, the pinned `viola(.exe)`, `plugin/<key>/` and its `hooks/` and `.claude-plugin/`, each that exists) before its wrapper log opens, a refusal exit 1 with the home or instance pair, no log, line or child (`--json` keeps `strict-modes-failed`)».'
    sidecar: 'input-validation revive arguments: the reference to the row''s interim gaps retired; revive''s start runs checked_exe / check_home_tree before its wrapper log.'
    rationale: 'The row''s interim gaps are all retired (report Retired bullet), so a clause placing revive outside them describes a standing set that no longer exists; Symbols: `run`''s `checked_exe` / `refuse_tree` called by `revive`''s start (revive.rs 125-135) before `open_wrapper_log`.'
    basis: .andromeda/security-plan-detail/input-validation.md:32
    dependent-of: D-retired
  - detector: D-retired
    severity: warning
    file: .andromeda/security-plan-detail/input-validation.md
    near: 33
    section: '## Input Validation'
    change: 'Replace «and checked by `check_instance` before any snapshot read, so neither verb is among this row''s interim gaps» with «and checked by `check_instance` before any snapshot read through `client::refuse_untrusted`, the one check `link_call` shares with the six frame verbs».'
    sidecar: 'input-validation link/unlink arguments: the reference to the row''s interim gaps retired; link_call runs client::refuse_untrusted.'
    rationale: 'The row''s interim gaps are all retired (report Retired bullet); Symbols: `link_call` (link.rs 94-95) moved onto `refuse_untrusted` unchanged in behaviour; Count 2 (`check_instance(` site moved link.rs → client.rs).'
    basis: .andromeda/security-plan-detail/input-validation.md:33
    dependent-of: D-retired
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/input-validation.md
    near: 30
    section: '## Input Validation'
    change: 'After «then pass the `~/.viola/` strict-modes check before any snapshot, `statusline_command` or ledger read.» add: Since chunk 2026-10-26-home-and-code-bearing-file-integrity `check_instance` holds the home, the instance directory and its `diagnostics/` to the read rule (Unix `unix_read_verdict`: the write rule plus no group or other bit; Windows `windows_read_verdict`: the write rule plus no allow ACE, inherit-only included, granting a right in `READ_RIGHTS` `0x9000_0001` to a SID `trusted_sid` rejects), `instances/` and `snapshot.json` to the write rule, and `events.ndjson` to the write rule on Unix and the read rule on Windows, each that exists, a folder before what it holds; `run` / `revive`''s start and `viola verify` check the home tree (`check_home_tree`) before any log opens; on Unix a home that cannot be stat-ed (a path under a regular file) is a strict-modes refusal (`verify` exit 1 with the home pair, the frame verbs exit 21 with the instance pair), while on Windows such a path stats `NotFound` and the `cli` catch-site line stands.'
    sidecar: 'input-validation CLI arguments row, home path: the read rule as built, the checked set of check_instance, check_home_tree at run/revive start and verify, an unstattable home refused on Unix.'
    rationale: 'Report Symbols (`viola_state::strict`: `unix_read_verdict`, `windows_read_verdict`, `READ_RIGHTS`, `check_instance`''s widened set, `check_home_tree`); Count 3 (the read rule built); Count 13 (an unstattable home a strict-modes refusal on Unix, `NotFound` on Windows).'
    basis: .andromeda/security-plan-detail/input-validation.md:30
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/input-validation.md
    near: 22
    section: '## Input Validation'
    change: 'Extend «an unknown event, a clap error or an absent/invalid `VIOLA_NAME` exits 0 with nothing written anywhere» with «, and so does an instance directory that does not canonicalise (a `VIOLA_DIR` naming no existing directory included) or that `check_instance` refuses: `canonical` / `trusted` run before obs init and before stdin is read, so no `config.json` read, no role file, no line, no frame and no SessionEnd append; the two `--capture` arms stay ahead of that check, unchanged».'
    sidecar: 'input-validation Hook stdin row: the instance check before obs init and stdin joins the silent exit-0 cases.'
    rationale: 'Report Symbols (`src/cmd/hook.rs`: `instance_of(…).and_then(canonical).filter(trusted)` before `read_diagnostics_level` / `viola_obs_init`; the capture arms ahead of it, unchanged); Count 23 (a hook whose `VIOLA_DIR` names no existing directory writes nothing).'
    basis: .andromeda/security-plan-detail/input-validation.md:22
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/input-validation.md
    near: 53
    section: '## Input Validation'
    change: 'Replace «They have two product callers» and extend the sentence: the replay (`replay::read_snapshot_or_replay`) has six product readers, each taken only after its strict-modes check — `viola revive`''s preflight (one read handed to both its collision check and its recorded-directory reading), `viola list`''s `board::read`, and since chunk 2026-10-26-home-and-code-bearing-file-integrity the hook `connect` (`Replayed` / `Absent` is `channel-unreachable`, failing open), `hook statusline`''s `serve` (a replayed snapshot carries no `statusline_command`, so nothing runs), `run`''s `collision_check` (the caller''s one read: `Replayed` decides liveness from the beat alone, `already-live` within `STALE_AFTER`, else free; an error or `Absent` is free) and `viola_state::liveness::live_endpoint` (only a read snapshot proceeds; a replayed one carries no `endpoint`, `pid` or `started_at`).'
    sidecar: 'input-validation Own state files row: the replay''s product readers 2 → 6 (hook connect, statusline serve, run collision_check, live_endpoint join revive and list).'
    rationale: 'Report Count 5 (replay product readers 2 (`revive`, `list`) → 6) and Symbols (`live_endpoint`, hook `connect`, statusline `serve`, `collision_check` taking `Option<&Recovered>`); Deviation 4 (revive''s preflight reads once); the founder''s ratification of the move (`inputs#I3` Q1).'
    basis: .andromeda/security-plan-detail/input-validation.md:53
  - detector: D-retired
    severity: warning
    file: .andromeda/security-plan-detail/input-validation.md
    near: 54
    section: '## Input Validation'
    change: 'Replace the passage «The three other snapshot readers (…) still call `read_snapshot`, … is a CARRY owed to the route entry "Home and code-bearing file integrity".» with: «No product reader under `src/` calls `read_snapshot` any more (3 → 0; rule: `read_snapshot(` under `src/`, outside test modules, as measured at viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/report.md): the hook, `hook statusline`, `run`''s collision check and `live_endpoint` read through `read_snapshot_or_replay` (the readers of the paragraph above; `verification-matrix.json#v1-41` and test-plan §6 E5 steps 1–3 built), a move the founder ratified as a boundary widening (his own answer at P4, `inputs#I3` Q1 of chunk 2026-10-26-home-and-code-bearing-file-integrity).»'
    sidecar: 'input-validation Own state files row: the read_snapshot readers and their CARRY to "Home and code-bearing file integrity" retired; read_snapshot product callers 3 → 0.'
    rationale: 'Report Retired bullet (`read_snapshot` as a product reader under `src/`, 3 → 0); Count 5 marks input-validation.md:54 @c187, @c344, @c586, @c634 stale and Count 1 the owner-title @c757; Symbols remaining-caller facts (`read_snapshot(` product callers 3 → 0).'
    basis: .andromeda/security-plan-detail/input-validation.md:54
  - detector: D-retired
    severity: warning
    file: .andromeda/security-plan-detail/input-validation.md
    near: 54
    section: '## Input Validation'
    change: 'Replace «(interim exceptions, ratified, until Epoch 6: … — `pause` / `release` F-W1)» with «at every entry point, with no interim exception since chunk 2026-10-26-home-and-code-bearing-file-integrity: `viola hook <event>` and `hook statusline` after `check_instance` on the canonical instance directory, the CLI frame verbs and `link` / `unlink` after `client::refuse_untrusted`, `viola mcp`''s frame tools after `check_instance`, `run` and `revive`''s start after `check_home_tree`, and `viola verify`''s `update_stamps` after `check_stamps`, made before `create_private_dir(ledger/)` and again under the lock, a refusal (`StampsError::StrictModes`) reading and writing nothing».'
    sidecar: 'input-validation Own state files row, Integrity: the hook, update_stamps and six-verb interim exceptions retired; each reader named with its check.'
    rationale: 'Report Retired bullet (gaps (1), (2), (4) to (8)); Count 1 (`gap-until` input-validation.md:54 @c886 stale) and Count 17 (`stamps-err` input-validation.md:54 @c1015 stale); Symbols (`refuse_untrusted`, `check_home_tree`, `StampsError`).'
    basis: .andromeda/security-plan-detail/input-validation.md:54
    dependent-of: D-retired
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/input-validation.md
    near: 77
    section: '## Input Validation'
    change: 'After «The arm''s order: canonicalise the instance directory, the instance check, the log, stdin, the `budget.json` write, the snapshot read, the shell-out.» add: The snapshot read is `read_snapshot_or_replay` (since chunk 2026-10-26-home-and-code-bearing-file-integrity): a replayed snapshot carries no `statusline_command`, so nothing runs or prints; `canonical` and `trusted` (the instance check) live in `src/cmd/hook.rs`, shared with the event and dialog paths.'
    sidecar: 'input-validation Statusline shell-out row: the snapshot read through the replay; a replayed snapshot runs nothing.'
    rationale: 'Report Symbols (`src/cmd/hook/statusline.rs` `serve` (128-132) on `read_snapshot_or_replay`, unit `statusline_over_a_replayed_snapshot_runs_nothing`; `canonical` / `trusted` moved to hook.rs).'
    basis: .andromeda/security-plan-detail/input-validation.md:77
  - detector: D-retired
    severity: warning
    file: .andromeda/security-plan-detail/input-validation.md
    near: 80
    section: '## Input Validation'
    change: 'Replace «Owner/DACL checks on the `conpty/` folder: the third interim gap (§ `~/.viola/` access control, Decisions Log `2026-09-29`).» with «Owner/DACL: `pin::pin_companions` runs `strict::check_present` on `bin/<key>/conpty/` and on each companion that exists before the held open; a refusal is `PinError::StrictModes` with nothing written, and the child runs on the inbox ConPTY with `sideload_fallback` `strict-modes-failed`, the exit unchanged.»'
    sidecar: 'input-validation DLL/ConPTY row: the third interim gap retired; the conpty/ owner/DACL leg built (check_present, PinError::StrictModes).'
    rationale: 'Report Retired bullet gap (3); Symbols (`viola_state::pin`: Windows `pin_companions` runs `strict::check_present`, `PinError::StrictModes`; `sideload_outcome` maps it to `strict-modes-failed`); Count 18 (the `conpty/` leg built).'
    basis: .andromeda/security-plan-detail/input-validation.md:80
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/input-validation.md
    near: 80
    section: '## Input Validation'
    change: 'In «the closed `process-start.sideload_fallback`: `hash-mismatch` · `unreadable` · `load-failed` · `not-built`» add the word `strict-modes-failed`, reading `hash-mismatch` · `strict-modes-failed` · `unreadable` · `load-failed` · `not-built` (schemas/diag-line.v1.json), and add «a strict-modes refusal» to «A mismatch, an unreadable file or a failed load leaves the files as found».'
    sidecar: 'input-validation DLL/ConPTY row: sideload_fallback 4 → 5 words (+strict-modes-failed).'
    rationale: 'Report Schema / config (`schemas/diag-line.v1.json` line 70: the enum gains `strict-modes-failed`) and Count 7 (pattern `sideload-fb`: input-validation.md:80 stale where the word list is enumerated).'
    basis: .andromeda/security-plan-detail/input-validation.md:80
note: 'D-security-input: no new unvalidated external-input surface (every new site is a check); D-security-deps: no dependency added. Count 14 (`viola list`''s `board::scan` passing a 0750 home with no instance while `check_instance` refuses it with one) leaves line 34 true as written; its owner is a P5 route pin, so no proposal here.'
