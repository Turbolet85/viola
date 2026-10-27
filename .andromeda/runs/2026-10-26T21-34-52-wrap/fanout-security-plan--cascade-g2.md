proposals:
  - detector: D-cascade
    severity: warning
    file: .andromeda/security-plan-detail/input-validation.md
    near: 22
    section: "## Input Validation"
    change: "In the Event argument sentence, after «an unknown event, a clap error or an absent/invalid `VIOLA_NAME` exits 0 with nothing written anywhere», add that the event and dialog paths run the instance check next (`canonical` then `trusted` in `src/cmd/hook.rs`, before the diagnostics level and obs init). A `VIOLA_DIR` naming no existing instance directory, or an instance the check refuses, also exits 0 at once with no stdin read, no line and no frame, so the stdin cap, the one-object rule and the dialog classification apply only to a trusted instance."
    sidecar: "Hook stdin row: the instance check before stdin on the event and dialog paths; a refused or absent instance exits 0 with nothing read or written (chunk 2026-10-26-home-and-code-bearing-file-integrity)."
    rationale: "Report Symbols, src/cmd/hook.rs canonical (75-81) / trusted (83-86): the event and dialog paths take instance_of(…).and_then(canonical).filter(trusted) before read_diagnostics_level / viola_obs_init. A refused or absent instance returns exit 0 with no stdin read, no line, no frame and no SessionEnd append. Deviation 5 covers a VIOLA_DIR naming no directory."
  - detector: D-retired
    severity: warning
    file: .andromeda/security-plan-detail/input-validation.md
    near: 30
    section: "## Input Validation"
    change: "Retire the interim-gap clause that says `viola hook <event>`, on its nine `HookEvent` arms, does not canonicalise `VIOLA_DIR` or run the strict-modes check. Say instead that the nine arms, like `hook statusline`, canonicalise the instance directory and run `check_instance` (`canonical` and `trusted`, shared in `src/cmd/hook.rs`) before the diagnostics level, obs init and any stdin or snapshot use, and that a malformed, absent or refused instance is a silent no-op (exit 0, nothing written). Keep the `--capture` / `--shell` / `--answers` clauses unchanged: those arms still run ahead of the check."
    sidecar: "CLI arguments row: the hook.event strict-modes gap retired; the nine HookEvent arms check the instance first."
    rationale: "Report Retired (1): viola hook's hook.event snapshot read without strict-modes. Symbols: canonical and trusted moved from statusline.rs to hook.rs and are shared by both arms; the two --capture arms stay ahead of the check, unchanged."
    basis: .andromeda/security-plan-detail/input-validation.md:30
  - detector: D-retired
    severity: warning
    file: .andromeda/security-plan-detail/input-validation.md
    near: 30
    section: "## Input Validation"
    change: "Replace «`viola verify` reads `ledger/stamps.json` before any strict-modes check» with: `viola verify` runs `check_home_tree(home, None, None)` (the home, `ledger/`, `bin/`, `plugin/`) before its `cli` log, a refusal being the home pair with exit 1, and its `update_stamps` refuses before it creates `ledger/` or reads the stamps. Keep the `viola run` `read_stamps_strict` clause."
    sidecar: "CLI arguments row: verify's stamps-read gap retired; verify checks the home tree before its log."
    rationale: "Report Retired (2): verify's update_stamps reading ledger/stamps.json without strict-modes. Symbols: src/cmd/verify.rs (64-70, refuse_home 95-99) and viola_state::stamps StampsError; Deviation 1."
    basis: .andromeda/security-plan-detail/input-validation.md:30
  - detector: D-retired
    severity: warning
    file: .andromeda/security-plan-detail/input-validation.md
    near: 31
    section: "## Input Validation"
    change: "Retire the fourth to eighth interim gaps (CLI `send`, `wait` / `last`, `answer`, `pause` / `release` and `viola mcp`'s frame tools reading `snapshot.json` without the strict-modes check). Say instead that each of the six CLI frame verbs runs `client::refuse_untrusted` first: `check_instance` on the home and `instances/<name>`, and on a refusal exit 21 with the instance pair (or the `instance-unreachable` document under `--json`), nothing opened under the home and no line. Say that `viola mcp`'s frame tools run `check_instance` before `live_endpoint`, a refusal returning `instance-unreachable` with `detail: null`, no frame and no line. Keep the `send` text, `wait` / `last` and `answer` argument clauses."
    sidecar: "CLI arguments row: the fourth to eighth dated gaps retired; the six frame verbs and the MCP frame tools check the instance first."
    rationale: "Report Retired (4)-(8). Symbols: src/cmd/client.rs refuse_untrusted (20-41) is called by send (190-192), wait (43-45), last (30-32), answer (94-96), pause (32-34) and release (35-37); viola-mcp tools.rs frame runs check_instance at 337-339."
  - detector: D-cascade
    severity: warning
    file: .andromeda/security-plan-detail/input-validation.md
    near: 32
    section: "## Input Validation"
    change: "In the `revive` arguments clause, replace «so `viola revive` is not among this row's interim gaps» (the row now holds none) with: and its start arm also runs `check_home_tree` (through `run`'s `checked_exe`) before its wrapper log, a refusal being exit 1 with the home or instance pair, no log and no child (`--json` keeps `strict-modes-failed`)."
    sidecar: "CLI arguments row: revive's start arm runs the home-tree check before its log; its interim-gap phrase dropped."
    rationale: "Report Symbols, src/cmd/run.rs checked_exe (189-200) / refuse_tree (202-215): revive's start calls them before open_wrapper_log (revive.rs:125-135), and revive --json keeps strict-modes-failed. The row's dated gaps went 8 → 0 (Count 1)."
  - detector: D-cascade
    severity: warning
    file: .andromeda/security-plan-detail/input-validation.md
    near: 33
    section: "## Input Validation"
    change: "In the `link` / `unlink` arguments clause, replace «so neither verb is among this row's interim gaps» with: their `check_instance` runs through `client::refuse_untrusted`, the one check they share with the six frame verbs, and their behaviour is unchanged."
    sidecar: "CLI arguments row: link / unlink's instance check now runs through the shared refuse_untrusted."
    rationale: "Report Symbols: link_call (link.rs:94-95) moved onto refuse_untrusted with its behaviour unchanged. Count 2: the check_instance site moved from link.rs to client.rs:31. The row's interim gaps are retired (Count 1)."
  - detector: D-retired
    severity: warning
    file: .andromeda/security-plan.md
    near: 142
    section: "## Input Validation"
    change: "Drop «except the ratified dated interim gaps until the Epoch 6 entries land»: the home is resolved, canonicalised and passed through the strict-modes check before any snapshot, `statusline_command` or ledger read, with no exception."
    sidecar: "Input Validation broad line, CLI arguments: the dated interim-gap exception retired."
    rationale: "Report Retired: the eight founder-ratified dated strict-modes gaps (Count 1, pattern gap-until, security-plan.md:142 stale)."
    basis: .andromeda/security-plan.md:142
  - detector: D-cascade
    severity: warning
    file: .andromeda/security-plan-detail/input-validation.md
    near: 53
    section: "## Input Validation"
    change: "Replace «They have two product callers» (revive's preflight and list's board::read) with six: those two and, since chunk 2026-10-26-home-and-code-bearing-file-integrity, `viola_state::liveness::live_endpoint`, hook `connect`, statusline `serve` and `run`'s `collision_check`, each taken only after its entry point's strict-modes check has passed."
    sidecar: "Own state files row: the classified read and the replay have six product callers (was two)."
    rationale: "Report Count 5: replay product readers went from 2 (revive, list) to 6 (+ hook connect, statusline serve, run's collision_check, live_endpoint). Symbols: each entry point's check runs before the read (client.rs refuse_untrusted, tools.rs 337-339, hook.rs trusted, run.rs checked_exe before open_wrapper_log)."
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/input-validation.md
    near: 54
    section: "## Input Validation"
    change: "Replace the sentence saying «the three other snapshot readers» and `live_endpoint` stay on `read_snapshot`, with their move a CARRY owed to «Home and code-bearing file integrity». New text: `live_endpoint`, hook `connect`, statusline `serve` and `run`'s `collision_check` read `read_snapshot_or_replay`. On a replay, `live_endpoint` gives no endpoint, hook `connect` is `channel-unreachable` (fail open), `serve` runs nothing, and `collision_check` reads liveness from the beat alone (`beat_age` at most `STALE_AFTER` is `already-live`). `read_snapshot(` product callers under `src/`, outside test modules, went 3 → 0. This is the founder's boundary-widening ratification (`inputs#I3` Q1 of chunk 2026-10-26-home-and-code-bearing-file-integrity)."
    sidecar: "Own state files row: the four remaining snapshot readers moved onto read_snapshot_or_replay (F10)."
    rationale: "Report Symbols: liveness.rs 92-94, hook.rs connect 224-226, statusline.rs serve 128-132, run.rs collision_check 425-432. Count 5 marks input-validation.md:54 read-snapshot stale; Retired lists read_snapshot as a product reader under src/ (3 → 0)."
    basis: .andromeda/security-plan-detail/input-validation.md:54
    fact: F10
  - detector: D-retired
    severity: warning
    file: .andromeda/security-plan-detail/input-validation.md
    near: 54
    section: "## Input Validation"
    change: "In the **Integrity** clause, drop the parenthetical interim exceptions: `viola hook <event>` reading `endpoint`, `update_stamps`' locked read, and `send`, `wait`, `last`, `answer`, `pause` and `release` reading `endpoint`, `pid`, `started_at` and the heartbeat without the check. Every own state file is now read only after the strict-modes check, at every entry point."
    sidecar: "Own state files row: the integrity clause's dated exceptions retired."
    rationale: "Report Retired (1), (2) and (4)-(7). Count 1 marks gap-until input-validation.md:54 stale."
    basis: .andromeda/security-plan-detail/input-validation.md:54
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/input-validation.md
    near: 54
    section: "## Input Validation"
    change: "After the `read_stamps_strict` sentence, add: `update_stamps` runs `strict::check_stamps` before `create_private_dir(ledger/)` and again under its lock before the capped read. It returns `StampsError {StrictModes, State}`; a refusal reads and writes nothing, and `verify` maps `StrictModes` to the home pair."
    sidecar: "Own state files row: update_stamps checks the stamps before creating ledger/ and under its lock (F8)."
    rationale: "Report Symbols viola_state::stamps (StampsError 37-44) and Deviation 1: under the lock alone the check would follow the create, which narrows a widened ledger/ first. Count 17 marks stamps-err input-validation.md:54 stale."
    basis: .andromeda/security-plan-detail/input-validation.md:54
    fact: F8
  - detector: D-retired
    severity: warning
    file: .andromeda/security-plan.md
    near: 154
    section: "## Input Validation"
    change: "Drop «except the ratified dated gaps»: own state files are read only after the home strict-modes check."
    sidecar: "Input Validation broad line, own state files: the dated-gap exception retired."
    rationale: "Report Retired: the eight dated strict-modes gaps, now 0 (Count 1)."
  - detector: D-cascade
    severity: warning
    file: .andromeda/security-plan-detail/input-validation.md
    near: 77
    section: "## Input Validation"
    change: "After «the snapshot read» in the arm's order, add: that read is `read_snapshot_or_replay`, and a replayed snapshot carries no `statusline_command`, so nothing runs and nothing prints."
    sidecar: "Statusline shell-out row: serve reads the replay, and a replay runs nothing."
    rationale: "Report Symbols, src/cmd/hook/statusline.rs serve (128-132), with unit statusline_over_a_replayed_snapshot_runs_nothing (524-553). This is a second occurrence of F10, which is placed at the Own state files row."
    dependent-of: D-new-detail
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/input-validation.md
    near: 80
    section: "## Input Validation"
    change: "Replace «Owner/DACL checks on the `conpty/` folder: the third interim gap» with: Windows `pin_companions` runs `strict::check_present` on `bin/<key>/conpty/`, and on each companion that exists, before the held open. A refusal is `PinError::StrictModes`, with nothing written or loaded, recorded as `sideload_fallback` `strict-modes-failed`, and the child runs on the inbox ConPTY with its exit unchanged."
    sidecar: "DLL search row: the conpty/ owner/DACL leg built before the held open (F9); the third dated gap retired."
    rationale: "Report Retired (3) and Symbols viola_state::pin (pin_companions, PinError::StrictModes); run.rs:510 maps it to strict-modes-failed. Unit pin.rs 453-473 and integ conpty_sideload are green on windows-2025."
    fact: F9
  - detector: D-cascade
    severity: warning
    file: .andromeda/security-plan-detail/input-validation.md
    near: 80
    section: "## Input Validation"
    change: "Add `strict-modes-failed` to the closed `process-start.sideload_fallback` list (now `hash-mismatch` · `strict-modes-failed` · `unreadable` · `load-failed` · `not-built`), and add a strict-modes refusal to «A mismatch, an unreadable file or a failed load»."
    sidecar: "DLL search row: sideload_fallback 4 → 5 words (+strict-modes-failed)."
    rationale: "Report Schema / config: schemas/diag-line.v1.json line 70 adds strict-modes-failed. Count 7 marks sideload-fb input-validation.md:80 stale."
    basis: .andromeda/security-plan-detail/input-validation.md:80
  - detector: D-cascade
    severity: warning
    file: .andromeda/security-plan.md
    near: 170
    section: "## Input Validation"
    change: "After «the companions are loaded only after a full SHA-256 re-hash under a read-share-only handle», add: and only after an owner/DACL strict-modes check of the `conpty/` folder and each companion, made before that open."
    sidecar: "Input Validation broad line, DLL search: the companions' owner/DACL check before the held open."
    rationale: "Report Symbols viola_state::pin: pin_companions runs check_present before the held open. This is a second occurrence of F9, which is placed at the DLL search row."
    dependent-of: D-new-detail
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/data-protection.md
    near: 10
    section: "## Data Protection"
    change: "After the Windows line, add: measured on `windows-2025` (chunk 2026-10-26-home-and-code-bearing-file-integrity), the protected owner-only DACL of a home viola makes outside the profile reads `D:PAI(A;OICI;FA;;;LA)(A;OICI;FA;;;SY)`, where canonical SDDL spells the runner's RID-500 user `LA`."
    sidecar: "Files row: the protected DACL of a home outside the profile, as measured on windows-2025 (F17)."
    rationale: "Report Cross-project claims: measured on windows-2025 in ci#40400887151's log, and read back through icacls /save (Deviation 10)."
    fact: F17
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/data-protection.md
    near: 11
    section: "## Data Protection"
    change: "After «The strict-modes check runs at startup», add: measured on `windows-2025`, a home made with `create_dir_all` before viola is refused `Writable` because of an inherited Users `S-1-5-32-545` write ACE, so a home made before viola is one the product must refuse."
    sidecar: "Files row: a home made before viola is refused Writable on Windows (F18)."
    rationale: "Report Spec claims disproved: ci#40398601594, green case strict::win::tests::strict_instance_refuses_a_plain_home_made_before_viola_and_names_the_entry; Deviation 6 (CARRY 13's answer)."
    fact: F18
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/data-protection.md
    near: 15
    section: "## Data Protection"
    change: "After the `<hash>` re-hash item, add: `run`'s start pins the bytes it has already checked (`pin::pin_bytes`, keyed by `pin::pin_key` = `<VERSION>-<16 hex>`), and `pin_exe` delegates to `pin_bytes`. The `pin_exe` first-start race (CARRY 16) and reproducible builds (CARRY 11) are owed to the route entry «Pinned copy race and reproducible builds»."
    sidecar: "Code-bearing artefacts: pin_bytes / pin_key; the pin race and reproducible builds carved out (F19)."
    rationale: "Report Symbols viola_state::pin (pin_key 71-74, pin_bytes) and Deviation 3. inputs#I3 Q4 and inputs#I4 row 1 carve CARRYs 11 and 16 into the new route entry."
    fact: F19
  - detector: D-cascade
    severity: warning
    file: .andromeda/security-plan-detail/data-protection.md
    near: 16
    section: "## Data Protection"
    change: "Before «A mismatch is `PinError::HashMismatch`», add: before the held open, `strict::check_present` runs on `bin/<key>/conpty/` and on each companion. A refusal is `PinError::StrictModes`, recorded as `strict-modes-failed`: nothing is written, loaded or launched, and the child runs on the inbox ConPTY."
    sidecar: "Code-bearing artefacts, conpty companions: PinError::StrictModes beside HashMismatch."
    rationale: "Report Count 18 marks data-protection.md:16 stale (the PinError variants). This is a second occurrence of F9, which is placed at the DLL search row."
    basis: .andromeda/security-plan-detail/data-protection.md:16
    dependent-of: D-new-detail
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/data-protection.md
    near: 32
    section: "## Data Protection"
    change: "Add an item after it, **Diagnostics opens:** `check_diagnostics` runs in `open_role_file` and `write_detail` before `ensure_private_dir` narrows anything. A symlink is refused; a folder takes the read rule; a file takes the read rule on Unix and the write rule on Windows. A refusal is a failed open: the writer ends at `sink` and the detail line is dropped."
    sidecar: "Logs: the diagnostics opens refuse a symlink or a widened path before any narrowing (F7)."
    rationale: "Report Symbols src/obs.rs check_diagnostics (69-76), with units 610-673; viola_state::strict check_diagnostics (127-145)."
    fact: F7
  - detector: D-retired
    severity: warning
    file: .andromeda/security-plan-detail/bootstrap-phases.md
    near: 15
    section: "## Bootstrap phases (derive for route / setup-project)"
    change: "Replace «the snapshot read after the strict-modes check is owed to the route entry «Home and code-bearing file integrity» for every client but CLI `link` / `unlink`, which run …» with: every client reads the snapshot only after the instance's strict-modes check — the six CLI frame verbs and `link` / `unlink` through `client::refuse_untrusted`, the hook event and dialog paths through `trusted`, and `viola mcp`'s frame tools through `check_instance` — and then makes the liveness pre-check and the server check."
    sidecar: "auth-scaffolding-baseline, viola-channel client: the snapshot-read-after-strict-modes CARRY closed for every client."
    rationale: "Report Retired (1), (4)-(8); Symbols client.rs refuse_untrusted (20-41), hook.rs trusted (83-86), tools.rs 337-339. Count 1 marks the owner-title rows stale."
  - detector: D-cascade
    severity: warning
    file: .andromeda/security-plan-detail/bootstrap-phases.md
    near: 16
    section: "## Bootstrap phases (derive for route / setup-project)"
    change: "Extend the `viola-state` strict-modes item to the full set as built: the read rule (Unix `mode & 0o077`; on Windows, a `READ_RIGHTS` grant to another SID) on the home, the instance directory and `diagnostics/`; `check_home_tree` (the tree at `run` / `revive` start and in `verify`); `check_diagnostics`; `check_present`; and the `Readable` / `Symlink` refusals."
    sidecar: "auth-scaffolding-baseline, viola-state: the strict-modes set as built (tree, diagnostics, present path, read rule)."
    rationale: "Report Symbols viola_state::strict (unix_read_verdict 209-217, windows_read_verdict 279-296, check_home_tree 91-125, check_diagnostics 127-145, check_present 160-163). Count 3 lists the read-rule row bootstrap-phases.md:16."
    basis: .andromeda/security-plan-detail/bootstrap-phases.md:16
  - detector: D-retired
    severity: warning
    file: .andromeda/security-plan-detail/error-handling.md
    near: 8
    section: "## Error Handling"
    change: "Replace «strict-modes is owed to «Home and code-bearing file integrity»» with: the event and dialog paths run the instance check (`canonical` and `trusted`) before the diagnostics level and obs init, and a refused or absent instance exits 0 at once with no `config.json` read, no role file, no stdin read, no line, no frame and no SessionEnd append. The capture arms stay ahead of the check, unchanged."
    sidecar: "Error Handling, hook fail-open: the hook's strict-modes check built; a refused instance fails open with nothing written."
    rationale: "Report Retired (1) and Symbols src/cmd/hook.rs (75-86). Acceptance: hook refused by the instance check, exit 0, empty streams, no frame, no line, no SessionEnd."
  - detector: D-retired
    severity: warning
    file: .andromeda/security-plan-detail/security-anti-patterns.md
    near: 8
    section: "## Security Anti-Patterns (NEVER do these) › ### Authentication"
    change: "Drop the dated-exception sentences, which cover the strict-modes half owed to «Home and code-bearing file integrity» for `hook.event`, CLI `send`, `wait` / `last`, `answer`, `pause` / `release`, `hook.dialog` and `viola mcp`'s tool frames, and drop «no other frame or process may borrow any of them». The ban now admits no strict-modes exception; keep its euid and macOS clause."
    sidecar: "Anti-pattern, frame before verification: the eight dated strict-modes exceptions retired."
    rationale: "Report Retired (1), (4)-(8). Count 1 marks owner-title security-anti-patterns.md:8 stale."
    basis: .andromeda/security-plan-detail/security-anti-patterns.md:8
  - detector: D-cascade
    severity: warning
    file: .andromeda/security-plan-detail/security-anti-patterns.md
    near: 9
    section: "## Security Anti-Patterns (NEVER do these) › ### Authentication"
    change: "Generalise the `link` / `unlink` sentence to say every frame meets this ban in full. The six CLI frame verbs and `link` / `unlink` take `endpoint`, `pid` and `started_at` after `client::refuse_untrusted`'s `check_instance`, the hook event and dialog frames after `trusted`, and `viola mcp`'s tool frames after `check_instance`. Only every client's euid check and macOS's pid and start-time check stay owed to «Unix endpoint and home hardening»."
    sidecar: "Anti-pattern, frame before verification: every frame now meets the ban as link / unlink did."
    rationale: "Report Symbols: client.rs refuse_untrusted (20-41) shared by link/unlink and the six verbs, hook.rs trusted (83-86) and tools.rs 337-339. Untouched: the euid check and macOS stay owed to «Unix endpoint and home hardening»."
  - detector: D-retired
    severity: warning
    file: .andromeda/security-plan.md
    near: 439
    section: "## Security Anti-Patterns (NEVER do these) › ### Authentication"
    change: "Drop «except the dated gaps: `hook.event`, `hook.dialog`, the CLI `send`, `wait` / `last`, `answer`, `pause` / `release` frames and `viola mcp`'s tool frames verify the server against a snapshot read without the strict-modes check, owed to «Home and code-bearing file integrity»» and «no other frame or process may borrow them». Keep the clause that every client's euid check and macOS's pid and start-time check (no peer pid) are owed to «Unix endpoint and home hardening»."
    sidecar: "Anti-patterns broad line, § Authentication: the dated strict-modes exceptions retired."
    rationale: "Report Retired: the eight dated gaps, 8 → 0 (Count 1)."
note: "No picked entity can carry F20 (error-handling, the fixed-message StampsError / PinError::StrictModes list) or the unpicked rows error-handling.md:15 (no-line list), security-anti-patterns.md:17 (seed carve-out) and the Data Protection companion anti-pattern; they go to the new-detail pass. F11/F12 (behind input-validation#1.4 and error-handling#1.1) and F16 (board::scan, true as written at input-validation#1.6 Home resolution) belong to authentication-authorization."
