## 2026-10-26-home-and-code-bearing-file-integrity — the dated strict-modes gaps retired: every client checks the instance before the snapshot
**Section:** §Authentication & Authorization (authentication-authorization.md: the IPC client-side server verification row and its stack reference; security-plan.md: the server verification summary) · §Bootstrap phases (bootstrap-phases.md: the `viola-channel` client) · §Threat Model Summary (threat-model-summary.md: the Local IPC vector)
**Change:**
- The server verification row: was six interim gaps here (the first and the fourth to eighth dated gaps: `hook.event`, CLI `send`, `wait` / `last`, `answer` with `hook.dialog`, `pause` / `release`, `viola mcp`'s four tools), each reading the snapshot without the strict-modes check until "Home and code-bearing file integrity", per 2026-10-26-server-verification-before-any-frame — the dated gaps keep only their strict-modes half; now **Strict-modes check first, no dated gap**: every client runs the instance's strict-modes check before it reads the snapshot, then (CLI and MCP) the liveness-only pre-check, then on Linux and Windows the server check (macOS the recorded gap). The residuals stay closed on Linux and Windows and open on macOS.
- New: on a strict-modes refusal, made before the liveness and server checks, no frame and no line is written, no `strict-modes-failed` line included; the CLI frame verbs exit 21 with the instance-state `unable:` / `hint:` pair (under `--json` the `instance-unreachable` document, `detail: null`); `viola hook` exits 0 with no body; MCP returns `instance-unreachable` with `detail: null`.
- The stack reference adds `viola-state` `strict::check_instance`, called by `cmd::client::refuse_untrusted`, `cmd::hook`'s `trusted` and `viola-mcp`'s `frame`.
- The `viola-channel` client, the threat model and the master summary: was the snapshot read after the check owed to that entry for every client but CLI `link` / `unlink`; now every client reads the snapshot only after `check_instance`, a refusal sending no frame and writing no line; the client euid check and macOS stay owed to "Unix endpoint and home hardening".
**Why:** the report's Retired line: the eight founder-ratified dated gaps, each "until" this entry, now each run the strict-modes check before the first read or open. A refused check writing no line, as `link` / `unlink`'s, is the founder's own answer to P4 fork Q2 (`inputs#I3`, live by dialog, relayed by the overseer). The companion rewrites of the retired lines were confirmed at the P2 halt by the overseer, founder-delegated (`inputs#I6`).
**Ref:** .andromeda/runs/2026-10-26T21-34-52-wrap/

## 2026-10-26-home-and-code-bearing-file-integrity — the six CLI frame verbs share `link` / `unlink`'s instance check
**Section:** §Authentication & Authorization (authentication-authorization.md: the strict-modes entry points' `link` / `unlink` clause and gap list; the `viola link` / `viola unlink` row; the CLI refusal row) · §Input Validation (input-validation.md: CLI arguments / stdin) · §Error Handling (error-handling.md: a new item)
**Change:**
- `client::refuse_untrusted` runs `check_instance` on the home and `instances/<name>` first in `send` (after its text read), `wait`, `last`, `answer`, `pause` and `release`, and in `link` / `unlink` (was `link_call`'s own call), each before `client::start`, so before any role file, log or snapshot value.
- A refusal exits 21: the instance pair (`human::instance_state_pair`) through `human::refuse`, or under `--json` exactly `{"v":1,"error":"instance-unreachable","detail":null}`; nothing opened under the home, no line. Proven for `link` / `unlink` by `cfg(unix)` cases only, for the six frame verbs on all three OSes.
- The fourth to seventh dated gaps (`send`, `wait` / `last`, `answer`, `pause` / `release` reading the snapshot without the check) leave the strict-modes entry points and the CLI arguments row, per 2026-10-04-confirmed-send-with-cl-1-records — the fourth dated gap: viola send before server verification (F3), 2026-10-04-wait-and-last — the fifth dated gap: CLI wait / last after the liveness-only pre-check, 2026-10-04-dialog-answers-by-dialog-id — the sixth dated gap: answer and hook.dialog frames, and their validation, and 2026-10-04-the-wheel — the seventh dated gap: pause and release frames, and their validation (their other claims stand); the row now holds a **Frame verbs** clause: the check, then `live_endpoint`, then on Linux and Windows the server check.
- The CLI refusal row: was only `link` / `unlink`'s strict-modes refusal coming before the liveness and server checks; now every frame verb's; its stack reference adds `refuse_untrusted`.
- The link row: was "so they borrow none of the eight dated gaps"; now "the order every CLI frame verb takes"; its stack reference adds `src/cmd/client.rs`.
**Why:** the report's Retired line (gaps 4 to 7). The six verbs printing `link`'s instance pair is the founder's own answer to P4 fork Q3, and no line on a refused check his answer to Q2 (`inputs#I3`, relayed by the overseer).
**Ref:** .andromeda/runs/2026-10-26T21-34-52-wrap/

## 2026-10-26-home-and-code-bearing-file-integrity — `viola mcp`'s frame tools check the instance before `live_endpoint`
**Section:** §Authentication & Authorization (authentication-authorization.md: the server verification row's MCP clause; the strict-modes entry points' `mcp` clause and gap list) · §Input Validation (input-validation.md: CLI arguments / stdin) · §Error Handling (error-handling.md: a new item) · §Threat Model Summary (threat-model-summary.md: the MCP tools vector)
**Change:**
- Was the eighth dated gap: the `send` / `wait` / `last` / `answer` tools read `snapshot.json`'s `endpoint` through `live_endpoint` without the check, per 2026-10-17-mcp-server-for-drivers — the eighth dated gap: MCP frame tools after the liveness-only pre-check (its start check through `check_path` stands); now each runs `check_instance` on the home and the instance directory before `live_endpoint`.
- A refused check, like a replayed or absent snapshot, sends no frame and returns `isError` `instance-unreachable` with `detail: null`, text `refused: instance-unreachable`, and no log line.
- The MCP trust boundary gains the check, ahead of the liveness pre-check and the server check.
**Why:** the report's Retired line (gap 8) and its Symbols (`tools.rs` `frame`); no line on a refused check is the founder's own answer to P4 fork Q2 (`inputs#I3`, relayed by the overseer).
**Ref:** .andromeda/runs/2026-10-26T21-34-52-wrap/

## 2026-10-26-home-and-code-bearing-file-integrity — the server check's reference values and the collision check read through the replay
**Section:** §Authentication & Authorization (authentication-authorization.md: the IPC client-side server verification row's Collision check and stack reference; the verdict row)
**Change:**
- The verdict: was the reference values taken with the `endpoint` from one `read_snapshot` call; now from one `read_snapshot_or_replay` call made after the instance's strict-modes check, only a parsed snapshot giving them: a replayed or absent one is no `LiveEndpoint` for the CLI frame verbs and `viola mcp`, and `NotSent::Unreachable`, failing open, for `viola hook`'s `connect`.
- The collision check: was reading the snapshot through `read_snapshot`, per 2026-10-26-server-verification-before-any-frame — run's collision check: the server check retired as moot (its moot server check stands); now it takes the caller's one read through `read_snapshot_or_replay` (`run` after its home-tree check; `revive`'s preflight reads once and hands that read to both its collision check and its recorded-directory reading) and judges a parsed snapshot by `pid` + OS start time and the heartbeat, a replayed one by the beat alone (a `beat_age` within `STALE_AFTER` is `already-live`, else free), an error or an absent snapshot free. Its citation: was `src/cmd/run.rs:383`; now `src/cmd/run.rs:418`.
- The stack reference adds `replay::read_snapshot_or_replay`, read by `live_endpoint` and `cmd::hook`'s `connect`.
**Why:** the founder's own answer to P4 fork Q1 (`inputs#I3`, live by dialog, relayed by the overseer), his boundary-widening ratification, under the overseer's founder-delegated PROVISIONAL card confirm at the P2 halt (`inputs#I6`). One read per revive keeps one replay and one `state-recovered` line (the report's Deviation 4); the old citation no longer named the check.
**Supersedes:** 2026-10-26-explicit-session-links — the collision check's code citation re-pointed
**Ref:** .andromeda/runs/2026-10-26T21-34-52-wrap/

## 2026-10-26-home-and-code-bearing-file-integrity — `check_instance`'s set and the read rule as built
**Section:** §Authentication & Authorization (authentication-authorization.md: `~/.viola/` access control, the strict-modes check's `revive` clause, the Unix and Windows read rules; security-plan.md: `~/.viola/` access control) · §Bootstrap phases (bootstrap-phases.md: `viola-state`)
**Change:**
- `check_instance`: was reading the home, `instances/`, the instance directory, its `snapshot.json` and its `events.ndjson` in that order; now it holds the home, the instance directory and its `diagnostics/` to the read rule, `instances/` and `snapshot.json` to the write rule, and `events.ndjson` to the write rule on Unix and the read rule on Windows, each that exists, a folder before what it holds.
- The Unix read rule: was the home and `instances/<name>/`; now also `diagnostics/` (`strict::unix_read_verdict`). The Windows read rule names `READ_RIGHTS` = `0x9000_0001` and `strict::windows_read_verdict`.
- A refusal names its property: `Refused` holds `Readable` ("a trusted state path is readable by another user") and `Symlink` ("a trusted state path is a symbolic link"), each fixed, neither borrowing `Writable`.
- `viola-state` bootstrap: was the Unix strict-modes check, its read rule on the home and `instances/<name>/`; now `viola_state::strict`, the read rule on both OSes over the home, the instance and its `diagnostics/`, with `check_home_tree`, `check_diagnostics`, `check_present` and the `Readable` / `Symlink` refusals.
- The master's access control: was refusing a trusted path another user can write, or a home another user can read, with its dated interim gaps; now also a home, instance directory or `diagnostics/` another user can read (Unix any group or other bit; Windows a read-right allow ACE to another SID, inherit-only included) or a symlinked `diagnostics/` folder or file, before the first read or open under the home.
**Why:** the report's Symbols (`strict.rs`) and its Count 3, the read rule built; the two variants give fixed messages that name the refused property (Deviation 2).
**Ref:** .andromeda/runs/2026-10-26T21-34-52-wrap/

## 2026-10-26-home-and-code-bearing-file-integrity — the Windows legs measured: the protected DACL, a plain home refused, an unstattable home
**Section:** §Authentication & Authorization (authentication-authorization.md: `~/.viola/` access control, the `revive` clause's proof and the outcomes paragraph) · §Data Protection (data-protection.md: Files (`~/.viola/`))
**Change:**
- `revive`'s check: was proven by `cfg(unix)` cases only, a Windows case widening an instance tree's DACL owed to this entry; now also by `cfg(windows)` cases (an instance tree granted Users write refused beside a `create_private_dir` tree that passes, a `diagnostics/` granted Users read refused, a plain home made before viola refused `Writable`), green on `windows-2025`.
- The outcomes: was "The outcomes match Unix"; now they match but one: an unstattable home (a path under a regular file) is a strict-modes refusal on Unix (`verify` exit 1 with the home pair, the frame verbs 21 with the instance pair), while on Windows such a path stats `NotFound`, passes as absent, and the catch-site line stands.
- The witnesses named: `tests/security_negatives_strict_modes.rs` (the six frame verbs, a start under umask 000, the Windows protected DACL read back, a read-granted pre-existing home refusing `run`, single writers with the wrapper stopped) and `tests/chaos_snapshot_replay.rs` (E5 steps 1–3), green on all three OSes.
- Data protection, new: the protected owner-only DACL of a home viola makes outside `%USERPROFILE%` reads `D:PAI(A;OICI;FA;;;LA)(A;OICI;FA;;;SY)`, canonical SDDL spelling the runner's user (the local Administrator, RID 500) `LA`; a plain home made before viola (`create_dir_all`) is refused `Writable`, the refused entry an inherited Users (`S-1-5-32-545`) write ACE.
**Why:** measured on `windows-2025` at the chunk's CI (the report's Cross-project claims and Spec claims disproved). The plain home's SDDL itself was not recorded; the refusal and the refused ACE are asserted instead (Deviation 6). A path under a regular file needs a per-OS oracle (Deviations 8 and 10).
**Ref:** .andromeda/runs/2026-10-26T21-34-52-wrap/

## 2026-10-26-home-and-code-bearing-file-integrity — `viola list`'s home-check inconsistency, a CARRY on "Unix endpoint and home hardening"
**Section:** §Authentication & Authorization (authentication-authorization.md: `~/.viola/` access control, the strict-modes entry points' `list` clause)
**Change:**
- New: `check_path` holds the home to the write rule while `check_instance` holds it to the read rule, so a 0750 home passes a board with no instance and is refused, naming the instance (`ScanRefused::Instance`), once it holds one; the inconsistency is a CARRY owed to the route entry "Unix endpoint and home hardening".
**Why:** the report's Count 14 (`board.rs` unchanged; the wrap directive, `inputs#I5`, asks an owner). The owner is the overseer's answer to the P2 halt's Q3, founder-delegated (`inputs#I6`): the home's checks belong to that entry. P5 pins the CARRY.
**Ref:** .andromeda/runs/2026-10-26T21-34-52-wrap/

## 2026-10-26-home-and-code-bearing-file-integrity — `viola hook`'s event and dialog paths check the instance before obs init
**Section:** §Authentication & Authorization (authentication-authorization.md: the strict-modes entry points' `hook <event>` clause and gap list) · §Input Validation (input-validation.md: Hook stdin; CLI arguments / stdin, Home path) · §Error Handling (error-handling.md: the hook fail-open item) · §Threat Model Summary (threat-model-summary.md: the Hook stdin vector)
**Change:**
- Was the first dated gap: `viola hook <event>` read `snapshot.json`'s `endpoint` without the check and its nine `HookEvent` arms neither canonicalised `VIOLA_DIR` nor ran it (Decisions Log `2026-09-28`), per 2026-09-27-hooks-to-normalised-events — `hook.event` served; the hook's interim gap, ratified by the founder; hook stdin row (its other claims stand); now the event and dialog paths canonicalise the instance directory and run `check_instance` (`canonical`, then `trusted`, shared in `src/cmd/hook.rs` with `hook statusline`) before the diagnostics level and obs init.
- A refused or absent instance exits 0 at once: no `config.json` read, no role file, no stdin read, no line, no frame, no SessionEnd append. The two `--capture` arms stay ahead of the check.
- On every hook path a `VIOLA_DIR` naming an instance directory that does not exist canonicalises to no instance, so the hook writes nothing.
- The error-handling item: was "strict-modes is owed to" this entry; now the check as above. The threat model: every hook path runs it before anything under the home is read or opened.
**Why:** the report's Retired line (gap 1), its Symbols (`hook.rs`) and its Count 23; no line on a refused check is the founder's own answer to P4 fork Q2 (`inputs#I3`, relayed by the overseer).
**Ref:** .andromeda/runs/2026-10-26T21-34-52-wrap/

## 2026-10-26-home-and-code-bearing-file-integrity — `ledger/stamps.json` checked before `ledger/` is made, and `verify`'s home tree before its log
**Section:** §Authentication & Authorization (authentication-authorization.md: the strict-modes entry points' `update_stamps` clause and gap list; the failure outcomes) · §Input Validation (input-validation.md: CLI arguments / stdin, Home path; Own state files on read, Integrity; security-plan.md: the CLI arguments line) · §Error Handling (error-handling.md: two new items) · §Security Anti-Patterns (security-anti-patterns.md: a new ban)
**Change:**
- Was the second dated gap: `update_stamps` read `ledger/stamps.json` without the check (Decisions Log `2026-09-28`, stamps read), per 2026-09-28-capability-ledger-and-viola-verify — `run`'s and `verify`'s stamps reads before strict-modes, a ratified interim gap until Epoch 6; now it runs `strict::check_stamps` before `create_private_dir(ledger/)` and again under its lock before the capped read; a refusal, `StampsError::StrictModes` (its other error `StampsError::State`), reads and writes nothing.
- `viola verify` runs `check_home_tree` on the home, `ledger/`, `bin/` and `plugin/` before its `cli` log; that refusal, or `StampsError::StrictModes`, exits 1 with the home pair, no step line, no summary and no line. On failure `run`, `verify` and `mcp` exit 1 (was `run` and `mcp`).
- `StampsError`'s fixed message: "the capability stamps failed the strict-modes check"; a write failure its own fixed message.
- New ban: never let `viola verify` read or write `ledger/stamps.json` before the check passes, nor run the check after a create, a mode set or a log open that narrows the path it checks.
- The master's CLI arguments line: was "except the ratified dated interim gaps until the Epoch 6 entries land"; now `ledger/stamps.json` is checked before `ledger/` is created and again under its lock, a refusal reading and writing nothing.
**Why:** the report's Retired line (gap 2) and its Deviation 1: checked under the lock alone, the check comes after the create, which narrows a widened `ledger/` to 0700 first, so it is made before the create too, by the plan's order rule.
**Ref:** .andromeda/runs/2026-10-26T21-34-52-wrap/

## 2026-10-26-home-and-code-bearing-file-integrity — the ConPTY companions' owner/DACL check before the held open
**Section:** §Authentication & Authorization (authentication-authorization.md: the strict-modes entry points' ConPTY clause and gap list) · §Data Protection (data-protection.md: Code-bearing artefacts) · §Input Validation (input-validation.md: DLL search order + sideloaded ConPTY companions; security-plan.md: its line) · §Error Handling (error-handling.md: a new item) · §Threat Model Summary (threat-model-summary.md: Filesystem state)
**Change:**
- Was the third dated gap: on Windows x64 `run` loaded `bin/<version>-<hash>/conpty/{OpenConsole.exe,conpty.dll}` under the held handle and the re-hash alone, without the owner/DACL check (Decisions Log `2026-09-29`), per 2026-09-29-sideloaded-conpty — vendored Microsoft ConPTY binaries and the third interim gap (founder live) (its other claims stand); now `pin_companions` runs `strict::check_present` on `bin/<key>/conpty/` and each companion that exists before the `FILE_SHARE_READ`-only held open. A refusal is `PinError::StrictModes` (fixed message "the pinned copy failed the strict-modes check"): nothing written, loaded or launched from that folder, the child on the inbox ConPTY, its exit unchanged.
- `process-start.sideload_fallback`: was four words; now five, `hash-mismatch` · `strict-modes-failed` · `unreadable` · `load-failed` · `not-built`; `run` maps `PinError::StrictModes` to `strict-modes-failed`.
- New: `run`'s start pins the bytes it has already checked (`pin::pin_bytes`, under the key `pin::pin_key`, `<VERSION>-<16 hex>`); `pin_exe`, which `verify` calls, delegates to it; the `pin_exe` first-start race and reproducible builds are owed to "Pinned copy race and reproducible builds".
**Why:** the report's Retired line (gap 3) and its Counts 7 and 18; one `strict-modes-failed` in the Windows leg's kept homes, the companion test's own. The pin race's new owner is the founder's own answer to P4 fork Q4 (`inputs#I3`) and his yes on the P5 card's Beyond-the-plan row 1 (`inputs#I4`), each relayed by the overseer. `pin_exe` keeps its signature for `verify` (Deviation 3).
**Ref:** .andromeda/runs/2026-10-26T21-34-52-wrap/

## 2026-10-26-home-and-code-bearing-file-integrity — `marketplace/` off the integrity set, no owner
**Section:** §Authentication & Authorization (authentication-authorization.md: the strict-modes entry points' `viola plugin install` clause) · §Threat Model Summary (threat-model-summary.md: Filesystem state)
**Change:**
- The installed `command` gets no re-hash: was its integrity owed to the route entry "Home and code-bearing file integrity"; now `marketplace/` is off the integrity set, with no owner. The threat model's integrity line says so too.
**Why:** CARRY 17 closed by ruling: the founder's own yes on the P5 card's Beyond-the-plan row 2 (`inputs#I4`, relayed by the overseer), his Epoch 5 batch (2026-10-20) having made `marketplace/` off the integrity set his word.
**Ref:** .andromeda/runs/2026-10-26T21-34-52-wrap/

## 2026-10-26-home-and-code-bearing-file-integrity — the home tree checked at `run` and `revive`'s start, before the wrapper log
**Section:** §Authentication & Authorization (authentication-authorization.md: a new row, `~/.viola/` access control, the home tree at start) · §Input Validation (input-validation.md: CLI arguments / stdin, the `revive` arguments) · §Error Handling (error-handling.md: a new item) · §Threat Model Summary (threat-model-summary.md: CLI arguments; Filesystem state) · security-plan.md (the Filesystem state vector)
**Change:**
- New row: `viola_state::strict::check_home_tree(home, name, key)` checks the home (read rule), with a name `check_instance`'s set, then `ledger/`, `bin/` and `plugin/`, then with a key `bin/<key>/`, the pinned `viola(.exe)`, `plugin/<key>/`, its `hooks/` and its `.claude-plugin/` (write rule), each that exists, and returns `TreeRefused { HomeLevel, InstanceLevel }`.
- `run` and `revive`'s start run it through `checked_exe` (the exe's bytes read once, its `pin_key` taken) before the wrapper log opens; a refusal exits 1 with the home or the instance pair (`refuse_tree`; `revive --json` the `strict-modes-failed` document), no log, no line and no child.
- The threat model: was "Nothing checks integrity beyond the content hash in the folder name"; now beyond it the start holds the home tree to the strict-modes check (Unix owner and mode, Windows owner and DACL) before the wrapper log, and `pin_companions` checks the `conpty/` folder; the CLI boundary: `run`, `revive`'s start and `verify` hold the home tree, the frame verbs the instance, so a tree chosen by `--home` or `VIOLA_DIR` that another user can write or read is refused.
- The master's Filesystem state vector: was "nothing checks integrity beyond the content hash"; now the start holds the home tree, the pinned binary and the plugin folders to the strict-modes check before any log opens.
**Why:** the report's Symbols (`run.rs` `checked_exe` / `refuse_tree`, `revive.rs`) and its acceptance criterion: `run` refuses a 0770 home, a widened `instances/<name>`, `bin/` or pinned copy, exit 1 with its pair before any log.
**Ref:** .andromeda/runs/2026-10-26T21-34-52-wrap/

## 2026-10-26-home-and-code-bearing-file-integrity — the diagnostics opens refuse a symlink or a widened path
**Section:** §Bootstrap phases (bootstrap-phases.md: `viola-state`) · §Data Protection (data-protection.md: a new Logs item) · §Error Handling (error-handling.md: a new item)
**Change:**
- New: `open_role_file` and `write_detail` run `viola_state::strict::check_diagnostics` on the existing `diagnostics/` folder and on the role or detail file before `ensure_private_dir` narrows anything: a symlink is refused; a folder takes the read rule; a file the read rule on Unix and the write rule on Windows; an absent path passes.
- A refusal is a failed open: the role writer ends at `std::io::sink` and the detail line is dropped.
**Why:** the report's Symbols (`src/obs.rs`'s `check_diagnostics`, `strict.rs`'s `check_diagnostics`): the check comes before anything is narrowed.
**Ref:** .andromeda/runs/2026-10-26T21-34-52-wrap/

## 2026-10-26-home-and-code-bearing-file-integrity — the snapshot readers move onto `read_snapshot_or_replay`
**Section:** §Error Handling (error-handling.md: two new items) · §Input Validation (input-validation.md: Own state files on read; Statusline shell-out; security-plan.md: the Own state files line) · §Threat Model Summary (threat-model-summary.md: Filesystem state) · §Authentication & Authorization (security-plan.md: the server verification summary)
**Change:**
- The classified read and the replay: was two product callers (`viola revive`'s preflight, `viola list`'s `board::read`); now six, each after its entry point's strict-modes check: also `viola_state::liveness::live_endpoint` (the CLI frame verbs and `viola mcp`'s tools), hook `connect` (one read for the event and dialog paths), statusline `serve` and `run`'s collision check.
- No product caller under `src/` calls `read_snapshot` (the rule: `read_snapshot(` under `src/`, outside test modules); was the three other readers and `live_endpoint` staying on it, their move a CARRY owed to this entry. The integrity clause: read only after the check, at every entry point (was its dated exceptions).
- Over a replayed snapshot, which carries no `endpoint`, `pid`, `started_at` or `statusline_command`: `live_endpoint` gives no endpoint, so no frame; hook `connect` writes no frame and fails open as `channel-unreachable`, a SessionEnd appended through `append_session_end`; `hook statusline` runs and prints nothing, exit 0, empty stderr; the collision check judges by the heartbeat alone.
- The master's Own state files line and server verification summary: a replayed snapshot yields no frame and runs no command.
**Why:** the founder's own answer to P4 fork Q1 (`inputs#I3`, live by dialog 2026-10-26T18:11Z, relayed by the overseer), his ratification of the boundary widening: hook, MCP and CLI processes read `events.ndjson` on a new basis. The answer card's confirm at the P2 halt is the overseer's, founder-delegated, PROVISIONAL (`inputs#I6`); the ratification stays his `inputs#I3`. The report: `read_snapshot(` product callers 3 → 0, replay readers 2 → 6.
**Supersedes:** 2026-10-26-server-verification-before-any-frame — the snapshot readers are three, and stay on read_snapshot
**Ref:** .andromeda/runs/2026-10-26T21-34-52-wrap/

## 2026-10-26-home-and-code-bearing-file-integrity — the frame ban met in full by every frame
**Section:** §Security Anti-Patterns (security-anti-patterns.md: the frame-before-verification ban; security-plan.md: the Authentication ban)
**Change:**
- The detail ban: was dated exceptions holding the strict-modes half for `hook.event`, `hook.dialog`, CLI `send`, `wait` / `last`, `answer`, `pause` / `release` and `viola mcp`'s tool frames, with `link` / `unlink` alone meeting it in full, per 2026-10-26-server-verification-before-any-frame — the frame ban's exceptions and the threat narrowed to the strict-modes half (its euid and macOS owners stand); now every frame meets the ban in full: the CLI frame verbs and `link` / `unlink` after `cmd::client::refuse_untrusted`'s `check_instance`, `viola hook`'s `hook.event` and `hook.dialog` after its instance check (`trusted`), `viola mcp`'s tool frames after `check_instance`, each verifying the server on Linux and Windows.
- The master ban: was the strict-modes exceptions owed to "Home and code-bearing file integrity" beside the euid and macOS gaps; now the dated gaps are only every client's euid check and macOS's pid and start-time check, owed to "Unix endpoint and home hardening"; no other frame or process may borrow them.
**Why:** the report's Retired line: the eight dated strict-modes gaps closed, so no frame takes its reference values from an unchecked snapshot.
**Ref:** .andromeda/runs/2026-10-26T21-34-52-wrap/
