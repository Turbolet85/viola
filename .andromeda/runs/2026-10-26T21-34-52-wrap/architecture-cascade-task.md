# Task — architecture · 2026-10-26-home-and-code-bearing-file-integrity

## The session
The session built the home's strict-modes check before the first read or open at every entry point: the six frame verbs (`send` · `wait` · `last` · `answer` · `pause` · `release`), the `viola hook` event and dialog paths, the `viola mcp` frame tools, `viola run` / `viola revive`'s start, `viola verify`, `update_stamps` and the diagnostics opens.
It added a read rule beside the write rule (Unix `mode & 0o077` on the home, `instances/<name>/` and `diagnostics/`; Windows a read grant to another SID through `READ_RIGHTS`), the home-tree check `check_home_tree`, `check_diagnostics`, `check_present`, and built the Windows `conpty/` companions' owner/DACL leg (fallback word `strict-modes-failed`).
It moved four snapshot readers onto the replay (`live_endpoint`, hook `connect`, statusline `serve`, `run`'s `collision_check`), so `read_snapshot` has no product caller under `src/` (3 → 0) and the replay's product readers go 2 → 6.
It retired all eight founder-ratified dated strict-modes gaps that were owned by this route entry; a refused check writes no line anywhere (the founder's word).
It measured on `windows-2025` the protected DACL of a home viola makes outside the profile, the refusal of a home made before viola, and a hard-link propagation hazard of the test ConPTY seed.
It changed no dependency, crate, config key, env var or flag; `schemas/diag-line.v1.json` gains one enum word.

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
- The `sideload_fallback` word list goes 4 → 5 (adds `strict-modes-failed`, now `hash-mismatch` · `strict-modes-failed` · `unreadable` · `load-failed` · `not-built`): the [PTY] decision rows that list the fallback causes (established-decisions.md:17) and the body's ConPTY companions line (architecture.md:319).
- The ConPTY companions are no longer loaded under the re-hash alone: `pin_companions` checks `bin/<key>/conpty/` and each existing companion's owner/DACL before the held open; every row that describes the companion load as re-hash-only or names the missing owner/DACL leg is stale (architecture.md:319, established-decisions.md:225, stack-and-technologies.md:19).
- Rows that name the route entry "Home and code-bearing file integrity" as the owner of a gap, a CARRY or a not-yet-built case (2 in architecture) are stale: all eight dated strict-modes gaps it owned are closed.
- CLI exit code 21's list (architecture.md:100, conventions.md exit-code rows) names only `viola list`, `viola plugin install`, `viola link`, `viola unlink` for a refused strict-modes check; `send`, `wait`, `last`, `answer`, `pause` and `release` now join it.
- CLI exit code 1 now also covers `viola run` / `viola revive`'s start refusing a widened home tree and `viola verify` refusing a widened home, `ledger/`, `bin/` or `plugin/` (conventions.md exit-code rows).
- The `instances/<ViolaName>/` line's list of checkers (architecture.md:325) names `viola list`, `viola revive`, `hook statusline`, `viola link`, `viola unlink`; the six frame verbs, the hook event/dialog paths, the `mcp` frame tools and `run`'s start now check too.
- The `check_instance(` call-site count line (occupied-resources.md:112): still 3 product sites, but the files moved — `src/cmd/hook/statusline.rs` → `src/cmd/hook.rs` (`trusted`), `src/cmd/link.rs` → `src/cmd/client.rs` (`refuse_untrusted`, shared by link/unlink and the six verbs), `src/cmd/revive.rs` unchanged.
- What `check_instance` reads is widened (the home and the instance directory to the read rule, `diagnostics/` added to the set, Windows `events.ndjson` to the read rule): every row stating what that check covers is re-read against the new set.
- The sentence that no case widens the DACL of an instance tree on Windows (occupied-resources.md:113) is disproved: `cfg(windows)` cases now do this, and they are green on `windows-2025`.
- `read_snapshot` as a product reader under `src/` goes 3 → 0: rows saying a reader stays on `read_snapshot`, or naming its 3 callers, are stale (established-decisions.md:79 ×2, :110, :207, standard-contracts.md:109).
- The replay's product readers go 2 (`revive`, `list`) → 6 (+ `live_endpoint`, hook `connect`, statusline `serve`, `collision_check`): the Snapshot envelope contract (architecture.md:152, standard-contracts.md:25) and the `state-recovered` emitter-list rows (established-decisions.md:46, occupied-resources.md:104, standard-contracts.md:25).
- Instance snapshot `pid` / `started_at` server check (architecture.md:157): every frame-writing client now runs the strict-modes instance check before the liveness pre-check and the server check, and no longer runs only liveness; the macOS peer-pid gap stays.
- Ledger stamps rows (architecture.md:154, :332, occupied-resources.md:126, standard-contracts.md:31): `update_stamps` now checks strict modes before `create_private_dir(ledger/)` and again under the lock, and returns `StampsError`; a refusal reads and writes nothing; the single-writer and over-cap-as-absent claims stay true.
- The pinned copy line (architecture.md:318) and [Deployment / Distribution]: `run`'s start now reads the exe's bytes once, checks the home tree (including `bin/<key>/`, the pinned exe, `plugin/<key>/`, `hooks/`, `.claude-plugin/`) before its log, and pins those same bytes (`pin_bytes`, `pin_key`).
- The home-level `diagnostics/` line (architecture.md:323) does not name the instance detail file `detail-<process>.ndjson` (the directive's outcome item; `v1-17`'s by-construction clause leans on it).
- Diagnostic output channels (architecture.md:412): the role-file and detail-file opens now refuse a symlinked or widened `diagnostics/`, and a refusal is a failed open that ends at `sink`.
- [Session Liveness] (established-decisions.md:203-207): `collision_check` now takes the caller's one read; on a replayed snapshot it decides from the beat alone (`already-live` or free), and an error or an absent snapshot reads as free.
- The hook fail-open contract and the `VIOLA_NAME` / `VIOLA_DIR` rows (architecture.md:296, :298): the hook event and dialog paths now return exit 0 with nothing done on a refused or absent instance, before any `config.json` read or obs init.
- MCP results (conventions.md mcp-results rows, architecture.md:104): a refused instance check on a frame tool is now an `instance-unreachable` result with `detail: null`; no frame is written and no line.
- CLI `--json` output (architecture.md:225, standard-contracts.md cli-json rows): the `instance-unreachable` document (exit 21) is now also the six frame verbs' strict-modes refusal.
- The Hook contract's statusline row (architecture.md:240, standard-contracts.md:99-103): `serve` reads the replay, and a replayed snapshot carries no `statusline_command`, so nothing runs or prints.
- The error-handling catch-site row (established-decisions.md:213): on Unix an unstattable home (a path under a regular file) is now a strict-modes refusal where the `cli` catch-site line stood; the reporter itself is unchanged.
- `viola list`'s board-scan rows (5 in architecture): true as written, but `check_instance` now holds the home to the read rule while `scan`'s own home check uses the write rule. The result is inconsistent (`board.rs` unchanged).
- The `seed_conpty` carve-out rows (architecture.md:343, :370): `seed_conpty` now creates the home and the seed dir with `create_private_dir`. A test that widens a home tree above `conpty/` starts that home unseeded.
- `schemas/diag-line.v1.json` (architecture.md:359) gains the enum word: the row naming it stays true, and any row that enumerates its values must be re-read.

## New facts
- F1 · occupied-resources · `viola_state::strict` has a read rule beside the write rule: Unix refuses any group or other bit (`mode & 0o077`); Windows refuses any allow ACE (inherit-only ACEs included) that grants `READ_RIGHTS` = `0x9000_0001` (`FILE_READ_DATA` | `GENERIC_READ` | `GENERIC_ALL`) to an untrusted SID. `Refused` gains `Readable` and `Symlink`, each with a fixed message.
- F2 · occupied-resources · `check_instance` holds the home, the instance directory and its `diagnostics/` to the read rule. It holds `instances/` and `snapshot.json` to the write rule, and `events.ndjson` to the write rule on Unix and the read rule on Windows. Only entries that exist are checked, a folder before what it holds.
- F3 · occupied-resources · `check_home_tree(home, name?, key?)` checks the home (read rule). With a name it then runs `check_instance`'s set. It then checks `ledger/`, `bin/` and `plugin/`. With a key it also checks `bin/<key>/`, the pinned exe, `plugin/<key>/`, its `hooks/` and `.claude-plugin/`. A refusal is `TreeRefused` `HomeLevel` or `InstanceLevel`.
- F4 · conventions · `viola run` and `viola revive`'s start check the home tree before `open_wrapper_log` (`checked_exe` / `refuse_tree`). On a refusal they exit 1 with the home pair, or with the instance pair for an instance-level refusal; no log is opened, no line written and no child started. `revive --json` keeps `strict-modes-failed`.
- F5 · conventions · `viola verify` runs `check_home_tree(home, None, None)` before its `cli` log. A refusal exits 1 with the home pair, no step line, no summary and no line; `measure` maps `StampsError::StrictModes` to the same pair.
- F6 · conventions · The six frame verbs call `client::refuse_untrusted` first (`send` after its text read). A refused instance exits 21 with `link`'s instance pair, or under `--json` exactly `{"v":1,"error":"instance-unreachable","detail":null}`; nothing under the home is opened and no line is written.
- F7 · conventions · `viola mcp`'s frame tools run `check_instance` before `live_endpoint`. A refusal returns `instance-unreachable` with `detail: null`, and no frame or line is written.
- F8 · established-decisions · The `viola hook` event and dialog paths take `instance_of` → `canonical` → `trusted` (`check_instance`) before `read_diagnostics_level` and obs init. A refused or absent instance exits 0 with no `config.json` read, no role file, no stdin read, no line, no frame and no SessionEnd append. The `--capture` arms stay ahead of it.
- F9 · occupied-resources · A `VIOLA_DIR` naming an instance directory that does not exist canonicalises to no session, so `viola hook` writes nothing.
- F10 · standard-contracts · `update_stamps` returns `StampsError { StrictModes, State }` (fixed messages) and runs `check_stamps` before creating `ledger/` and again under its lock. A refusal reads and writes nothing.
- F11 · established-decisions · On the replay, `live_endpoint` proceeds only on a real snapshot (a replay → no endpoint), hook `connect` treats a replay or absence as unreachable (fail open), and statusline `serve` runs nothing. `state.snapshot_recover` now sits under `run.start` before `run.collision_check` and under `hook.handle`.
- F12 · established-decisions · `revive`'s preflight reads the snapshot once and hands that read to both its collision check and its recorded-directory reading: one replay, one `state-recovered` line.
- F13 · established-decisions · `pin::pin_key(bytes)` names the pin key `<VERSION>-<16 hex>` once, and `pin_bytes(home, bytes)` pins already-checked bytes. `pin_exe` keeps its signature and delegates to it; `verify` calls `pin_exe`, and `run`'s start calls `pin_bytes`.
- F14 · established-decisions · On Windows, `pin_companions` runs `check_present` on `bin/<key>/conpty/` and each existing companion before the held open. A refusal is `PinError::StrictModes`, writes nothing, and runs the child on the inbox ConPTY with `sideload_fallback` `strict-modes-failed`; the exit code is unchanged.
- F15 · occupied-resources · The diagnostics opens (`open_role_file`, `write_detail`) run `strict::check_diagnostics` before `ensure_private_dir` narrows anything. A symlink is refused, a folder takes the read rule, and a file takes the read rule on Unix and the write rule on Windows; a refusal is a failed open that ends at `sink`, and the detail line is dropped.
- F16 · occupied-resources · Measured on `windows-2025`: the protected owner-only DACL of a home viola makes outside the profile reads `D:PAI(A;OICI;FA;;;LA)(A;OICI;FA;;;SY)`, where canonical SDDL spells the runner's local Administrator (RID 500) `LA`.
- F17 · occupied-resources · Measured on `windows-2025`: a home made with `create_dir_all` before viola is refused `Writable`, on an inherited `S-1-5-32-545` (Users) write ACE; a home a test made first is one the product must refuse.
- F18 · occupied-resources · Measured on Windows: a path under a regular file stats `NotFound`, so the strict-modes check skips it as absent and the catch-site line stands there. On Unix the same home is a strict-modes refusal.
- F19 · occupied-resources · Measured on `windows-2025`: an inheritable `icacls` grant on a seeded home's `bin/` (or the home) propagates through `seed_conpty`'s hard links into the shared `target/conpty-seed/<key>/` companion and widens every seeded home until it is removed.
- F20 · occupied-resources · `viola list`'s `board::scan` checks the home with the write rule, then runs `check_instance` per instance, which holds the home to the read rule. A 0750 home therefore passes an empty board but is refused (`ScanRefused::Instance`) once it holds an instance. The owner is a P5 route pin (proposed: a CARRY on "Unix endpoint and home hardening").
- F21 · new · The `pin_exe` first-start race and reproducible builds (CARRYs 16 and 11) move to a new Epoch 6 route entry, "Pinned copy race and reproducible builds", right after this one. CARRY 17 (`marketplace/` integrity) is closed by the founder's ruling with no work.

## Untouched
- Endpoint naming (FNV-1a over `ViolaName` + home), the IPC endpoint paths and their permissions: the server identity check itself is unchanged.
- The channel methods' `params` and `ok` payloads, the frame envelope, `MAX_FRAME` and the JSON-RPC error codes: no wire shape changed.
- The normalised event kinds and the event `data` per kind, the `state-recovered` event and its detail words included: only who emits it moved.
- The budget governor and the `budget.json` checked read (`read_budget`): unchanged.
- The `marketplace/` tree and `viola plugin install`'s own strict-modes check: off the integrity set by ruling, unchanged.
- The capability ledger rows, `LedgerRow::relied()` and the stamped CLI versions: no row added or re-stamped.
- The stack, crate versions, workspace members and `deny.toml`: no `Cargo.toml` or `Cargo.lock` change.
- The wheel contract (`wheel` on every start with `driver`, `pause` → `wheel:"human"`): re-read true; only test-plan's E5 signal text moves.
- The instance snapshot's field set and its single writer (the wrapper): no field added or removed.
