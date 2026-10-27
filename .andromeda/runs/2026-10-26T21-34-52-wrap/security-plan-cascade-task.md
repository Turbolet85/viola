# Task — security-plan · 2026-10-26-home-and-code-bearing-file-integrity

## The session
Built the Epoch 6 entry "Home and code-bearing file integrity": the home's strict-modes check now runs before the first read or open at every entry point (the six CLI frame verbs, the hook event and dialog paths, the `viola mcp` frame tools, `run`, `revive`'s start, `verify`, `update_stamps`, the diagnostics opens, the Windows `conpty/` companions).
Retired all eight founder-ratified dated strict-modes gaps (hook.event; verify's `update_stamps`; the Windows ConPTY companions' owner/DACL leg; CLI `send`; `wait` / `last`; `answer` and `hook.dialog`; `pause` / `release`; the `viola mcp` tools): dated gaps 8 → 0.
Built the read rule (Unix `mode & 0o077`; Windows a read grant to another SID) on the home, the instance dir and `diagnostics/`, and widened `check_instance`'s set; added `check_home_tree`, `check_diagnostics`, `check_present`, `refuse_untrusted` and a symlink refusal.
Moved four snapshot readers (`live_endpoint`, hook `connect`, statusline `serve`, `run`'s `collision_check`) onto `read_snapshot_or_replay` on the founder's boundary-widening word; `read_snapshot` product callers under `src/` 3 → 0.
Measured on `windows-2025`: the protected owner-only DACL of a home outside the profile, a pre-existing `create_dir_all` home refused `Writable`, and a path under a regular file stats `NotFound`; CI green 15/15 on three OSes.

## Reached files
- crates/viola-state/src/strict.rs
- crates/viola-state/src/stamps.rs
- crates/viola-state/src/liveness.rs
- crates/viola-state/src/pin.rs
- crates/viola-mcp/src/tools.rs
- src/cmd/client.rs
- src/cmd/link.rs
- src/cmd/send.rs
- src/cmd/wait.rs
- src/cmd/last.rs
- src/cmd/answer.rs
- src/cmd/pause.rs
- src/cmd/release.rs
- src/cmd/hook.rs
- src/cmd/hook/statusline.rs
- src/cmd/run.rs
- src/cmd/revive.rs
- src/cmd/verify.rs
- src/obs.rs
- schemas/diag-line.v1.json
- tests/security_negatives_strict_modes.rs
- tests/chaos_snapshot_replay.rs
- crates/viola-mcp/tests/handlers.rs
- tests/hook_fail_open.rs
- tests/hook_statusline.rs
- tests/run_cli.rs
- tests/cli_verify.rs
- tests/conpty_sideload.rs
- tests/cli_controls_not_disableable.rs
- tests/support/home.rs
- tests/cli_revive.rs
- tests/cli_wait_last.rs
- tests/contract_diag_schema.rs
- tests/cli_machine_contract.rs
- tests/tui_env_strip.rs
- viola-0.1.0/verification-matrix.json
- viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/scope-record.md
- viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/evidence/remove-the-guard.md
- viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/evidence/operator-pass.md

## Touches
- Authentication & Authorization, IPC client-side server verification: the sentence calling the snapshot read without strict-modes a dated gap owed to "Home and code-bearing file integrity" (with `link` / `unlink` the only exception) is retired; every client now checks the instance first. The euid check and macOS stay owed to "Unix endpoint and home hardening".
- Authentication & Authorization, `~/.viola/` access control: "with its dated interim gaps" is retired; the read rule ("a home another user can read") is now built in code.
- Every security-plan site that names "Home and code-bearing file integrity" as the owner of a gap, CARRY or unbuilt case (pattern owner-title, 15 security-plan rows) is stale; the gap-until sites are security-plan.md body, authentication-authorization.md:36, input-validation.md:30 and input-validation.md:54.
- Input Validation, CLI arguments / stdin row: the exception "except the ratified dated interim gaps until the Epoch 6 entries land" is retired.
- Input Validation, Own state files on read row: "except the ratified dated gaps" is retired; `ledger/stamps.json` is now read only after the check, by both `run` and `verify`.
- Security Anti-Patterns § Authentication: the frame-before-verification ban's list of dated gaps (`hook.event`, `hook.dialog`, CLI `send`, `wait` / `last`, `answer`, `pause` / `release`, `viola mcp` tool frames) is retired; only the euid and macOS clause remains.
- Snapshot readers: every site that says a reader "stays on `read_snapshot`" or names its three callers is stale (authentication-authorization.md:25, :27, :63; input-validation.md:54). Product replay readers went from 2 to 6, and `read_snapshot` product callers from 3 to 0.
- `check_instance`: still 3 call sites under `src/`, but their files moved (statusline.rs to hook.rs, link.rs to client.rs `refuse_untrusted`) and the set widened: home, instance dir and `diagnostics/` take the read rule, `diagnostics/` is new in the set, and Windows `events.ndjson` takes the read rule. Each of the 11 security-plan check-instance rows that says what the check reads needs a re-read.
- Read-rule wording that calls the rule not yet in code (authentication-authorization.md:18, :38, :40; bootstrap-phases.md:16) is now built.
- `update_stamps`: sites that say it reads `ledger/stamps.json` without strict-modes are stale (authentication-authorization.md:36, input-validation.md:54, :90); it now refuses before creating `ledger/` and before reading.
- ConPTY companions: the owner/DACL leg is now built (authentication-authorization.md:36, data-protection.md:16 for the `PinError` variants, input-validation.md:81). The Input Validation DLL search row and the Data Protection anti-pattern on the companions gain the check before the held open.
- `process-start.sideload_fallback` enum grew from 4 words to 5 (+`strict-modes-failed`); input-validation.md:80, which lists the fallback causes, is stale.
- `human::refuse(` sites under `src/` went from 15 to 18 (authentication-authorization.md:35 at its count).
- No-line exemption lists (authentication-authorization.md:27, :34, :35; error-handling.md:15) gain the new silent refusal sites: the six frame verbs, the hook event and dialog paths, the MCP frame tools, `verify`, `run` / `revive`'s tree check, and the diagnostics opens.
- The six frame verbs now join `revive` and `link` / `unlink` in printing the instance pair (exit 21) on a strict-modes refusal. `run` and `verify` print the home pair (exit 1). Revisit any list that names which verbs print which pair.
- `board::scan` (`viola list`) rows (3 in security-plan) are still true as written, but they now run into the widened `check_instance`. See the new facts.
- Threat Model, Filesystem vector and boundary-filesystem: "nothing checks integrity beyond the content hash in the folder name" no longer holds. The pinned copy, `bin/<key>/`, `plugin/<key>/` and their subfolders are now mode/DACL-checked at `run` / `revive` start.
- Threat Model, MCP boundary and CLI boundary: a refusal from the `viola mcp` tools and the CLI frame verbs now comes from the instance's strict-modes check as well as from liveness.
- Bootstrap auth-scaffolding-baseline: the strict-modes checks in `viola-state` are now the full set (tree, diagnostics, present-path, read rule).
- Anti-pattern "NEVER skip the Windows owner + DACL strict-modes check at startup": `run`'s start now checks the whole home tree before its log. Measured on `windows-2025`: the `--home`-outside-profile protected DACL.
- Anti-pattern "NEVER run `statusline_command` from a `snapshot.json` whose home fails strict-modes": statusline `serve` now reads the replay, and a replayed snapshot carries no `statusline_command`, so nothing runs.
- Anti-pattern "NEVER create `diagnostics/` files readable by other users": these files are now also refused on open (symlink, modes) before any narrowing.
- Error Handling, the `Display` fixed-message list: new fixed messages on `Refused::Readable`, `Refused::Symlink`, `PinError::StrictModes` and the new `StampsError`.
- Seed carve-out (security-anti-patterns.md:17): viola now creates the seeded home and the seed dir (`create_private_dir`) before `bin/<key>/conpty/`. Re-read the home-creation step.
- `revive` (Input Validation revive row): its start arm also runs the home-tree check before its log; its preflight now takes one snapshot read for both the collision check and the recorded-directory reading.
- CARRY 17 (the integrity of the `marketplace/` tree), which named this entry as its owner, is closed by ruling: `marketplace/` is off the integrity set, by the founder's Epoch 5 word.

## New facts
- F1 · authentication-authorization · `check_home_tree(home, name, key)` returns `TreeRefused {HomeLevel, InstanceLevel}`. It checks the home (read rule), then `check_instance`'s set, then `ledger/`, `bin/`, `plugin/`, `bin/<key>/`, the pinned `viola(.exe)`, `plugin/<key>/`, its `hooks/` and its `.claude-plugin/` (write rule), each one that exists. `run` and `revive`'s start run it before opening the wrapper log: exit 1 with the home or instance pair, no log, no line, no child.
- F2 · authentication-authorization · `viola verify` runs `check_home_tree(home, None, None)` before its `cli` log. A refusal is the home pair, exit 1, with no step line and no summary.
- F3 · authentication-authorization · `refuse_untrusted` (client.rs) runs `check_instance` first in `send`, `wait`, `last`, `answer`, `pause`, `release` and `link` / `unlink`. A refusal exits 21 with the instance pair, or exactly `{"v":1,"error":"instance-unreachable","detail":null}` under `--json`; it opens nothing under the home and writes no line.
- F4 · authentication-authorization · The read rule as built: `unix_read_verdict` refuses `mode & 0o077`. `windows_read_verdict` refuses any allow ACE (inherit-only included) granting `READ_RIGHTS` 0x9000_0001 (FILE_READ_DATA, GENERIC_READ, GENERIC_ALL) to an untrusted SID.
- F5 · authentication-authorization · `check_instance` now applies the read rule to the home, the instance directory and `diagnostics/` (new in the set), and on Windows to `events.ndjson`, each folder before what it holds.
- F6 · authentication-authorization · `Refused` gains `Readable` and `Symlink` (variants 5 → 7, fixed messages), so the read rule and the symlink check do not borrow `Writable`.
- F7 · data-protection · `check_diagnostics` runs in `open_role_file` and `write_detail` before `ensure_private_dir`: a symlink is refused, a folder takes the read rule, and a file takes the read rule on Unix and the write rule on Windows. A refusal is a failed open: the writer ends at `sink` and the detail line is dropped.
- F8 · input-validation · `update_stamps` runs `check_stamps` before `create_private_dir(ledger/)` and again under the lock. It returns `StampsError {StrictModes, State}`, and a refusal reads and writes nothing; `verify` maps `StrictModes` to the home pair.
- F9 · input-validation · Windows `pin_companions` runs `check_present` on `bin/<key>/conpty/` and on each companion before the held open. A refusal is `PinError::StrictModes`, recorded as `sideload_fallback` `strict-modes-failed`: the child runs on the inbox ConPTY with its exit unchanged.
- F10 · input-validation · `live_endpoint`, hook `connect`, statusline `serve` and `run`'s `collision_check` read `read_snapshot_or_replay`. On a replay: no endpoint for `live_endpoint`, `channel-unreachable` (fail open) for hook `connect`, nothing run for `serve`, and liveness from the beat alone for `collision_check`. This is the founder's boundary-widening ratification (`inputs#I3` Q1).
- F11 · authentication-authorization · Hook event and dialog paths check the instance (`canonical` plus `trusted`) before reading the diagnostics level and before obs init. A refused or absent instance exits 0 with no `config.json` read, no role file, no stdin read, no line, no frame and no SessionEnd append. The capture arms stay ahead of the check, unchanged.
- F12 · authentication-authorization · A hook whose `VIOLA_DIR` names an instance directory that does not exist canonicalises to no session and writes nothing.
- F13 · authentication-authorization · The `viola mcp` frame tools run `check_instance` before `live_endpoint`. A refusal returns `instance-unreachable` with `detail: null`, no frame and no line.
- F14 · authentication-authorization · Founder's word (`inputs#I3` Q2 and Q3): a refused check at every new entry point writes no line (no `strict-modes-failed` is logged), and the six frame verbs print `link`'s instance pair.
- F15 · authentication-authorization · An unstattable home (a path under a regular file) is a strict-modes refusal on Unix: `verify` gives exit 1 with the home pair, and the frame verbs give exit 21 with the instance pair. On Windows such a path stats `NotFound`, passes as absent, and the catch-site line stands.
- F16 · authentication-authorization · `viola list`'s `board::scan` runs `check_path` (write rule) on the home, then `check_instance` (now the read rule) per instance. A 0750 home therefore passes with no instance and is refused, naming the instance, when it holds one. The owner is a CARRY on "Unix endpoint and home hardening".
- F17 · data-protection · Measured on `windows-2025`: the protected owner-only DACL of a home viola makes outside the profile reads `D:PAI(A;OICI;FA;;;LA)(A;OICI;FA;;;SY)` (`LA` is the runner's RID-500 user).
- F18 · data-protection · Measured on `windows-2025`: a home made with `create_dir_all` before viola is refused `Writable` (an inherited Users `S-1-5-32-545` write ACE). A home made before viola is one the product must refuse.
- F19 · data-protection · `run`'s start pins the bytes it already checked (`pin_bytes`, key `pin_key` = `<VERSION>-<16 hex>`), and `pin_exe` delegates to it. The `pin_exe` first-start race (CARRY 16) and reproducible builds (CARRY 11) move to the new route entry "Pinned copy race and reproducible builds".
- F20 · error-handling · New fixed-message `StampsError` in `viola-state`: "the capability stamps failed the strict-modes check" and "... could not be written". `PinError::StrictModes`: "the pinned copy failed the strict-modes check".
- F21 · authentication-authorization · `tests/security_negatives_strict_modes.rs` witnesses the strict-modes refusals on all three OSes: the six verbs, umask 000, the Windows protected DACL read back, a read-granted pre-existing home, and single writers with the wrapper stopped. `tests/chaos_snapshot_replay.rs` witnesses the replayed readers (E5 steps 1–3).

## Untouched
- Paste-text and dialog free-text validation (`validate_paste_text`, `control-character`, `empty-text`, CR/LF typing) are unchanged: `viola-core` was not edited.
- Channel framing (`MAX_FRAME` before `read_line`, `-32600`, `-32602`, depth limit) and the Windows listener SDDL, Unix socket directory and per-accept peer euid check are unchanged: `viola-channel` was not edited.
- The client euid check and macOS's server check (no peer pid) are still owed to "Unix endpoint and home hardening": this session did not touch them.
- Dependency Security (cargo-deny, the 0BSD exceptions, zizmor, pinning, `conpty-vendor.sh`, CI workflow jobs, `release-check.sh`, `fuzz/`) is unchanged: no `Cargo.toml`, `Cargo.lock`, script or workflow changed.
- API Security (rmcp features `server` + `transport-io`, the banned rmcp features) is unchanged.
- Secret Management (the R8 strip, `env -i` pre-push, CI artifact canary scan classes) is unchanged.
- The Windows DLL search restriction (`restrict_dll_search` as the second statement of `main`) and the companions' SHA-256 re-hash pins are unchanged: `viola-pty` and `src/conpty.rs` were not edited.
- The `viola plugin install` `check_path` on the home and the `marketplace/` dirs, and the exec-form absolute-path ban, are unchanged.
- The terminal colour facts and `list`'s `escape_row` are unchanged.
- The persistent-environment names (Windows registry) row, the `config.json` budget thresholds and dialog hold, and the `claude_env_keep` floor are unchanged.
- The driver-originated `release` refusal (`-32602`, `release-from-driver`) in the wrapper's handler is unchanged.
- The session-id argv shape (`resume_args`) and `claude` child resolution (the `.cmd`/`.bat` refusal) are unchanged.
- The accepted risks and residuals (driver prompt injection, verify's trusted runs, the scratch footer reading, the live `plugin install` round, local availability) are unchanged.
- The `MAX_FRAME` constant and the fuzz/property parser-surface list are unchanged.
