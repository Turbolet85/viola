proposals:
  - detector: D-retired
    severity: warning
    file: .andromeda/security-plan-detail/bootstrap-phases.md
    near: 15
    section: "## Bootstrap phases (derive for route / setup-project)"
    change: "Line 15 should read: `viola-channel` client: the SQOS open (windows-sys `CreateFileW` + `Stream::try_from(OwnedHandle)`) and the peer pid (`Client::server_pid`, `TokioClient::server_pid`, interprocess `peer_creds().pid()`); the verdict, pid + sysinfo start time against the snapshot's `pid` and `started_at`, lives in `viola-state` liveness (`verify_server`, `ServerVerdict`), called by the root bin and `viola-mcp`, built on Linux and Windows; every client reads the snapshot only after the strict-modes check (chunk 2026-10-26-home-and-code-bearing-file-integrity): CLI `send`, `wait`, `last`, `answer`, `pause`, `release`, `link` and `unlink` run `viola_state::strict::check_instance` on the home and the instance (`refuse_untrusted`) before any role file or log opens and before any snapshot value is read, `viola mcp`'s frame tools run it before `live_endpoint`, and `viola hook`'s event and dialog paths before obs init, then the liveness pre-check and the server check; `live_endpoint` and hook `connect` proceed only on a snapshot read whole, a replayed one reading as unreachable. The SQOS spike passed (Decisions Log `2026-09-25`); the client reuses the recipe `tests/channel_sqos_open.rs` pins."
    sidecar: "bootstrap-phases auth-scaffolding-baseline: the viola-channel client bullet no longer owes the snapshot read after the strict-modes check to 'Home and code-bearing file integrity' (retired: every client checks the instance first; the readers take the snapshot whole, never a replay)."
    rationale: "Report Retired bullet: the eight dated strict-modes gaps (hook.event, CLI send, wait/last, answer and hook.dialog, pause/release, viola mcp's tools) are retired, each now running the strict-modes check before its first read or open; Symbols: refuse_untrusted at the six verbs and link_call, tools.rs frame check_instance before live_endpoint, hook.rs trusted before obs init, live_endpoint and hook connect on read_snapshot_or_replay with only Recovered::Snapshot proceeding. Line 15 still states the read as owed to this entry for every client but link/unlink."
    basis: viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/report.md:26
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/bootstrap-phases.md
    near: 16
    section: "## Bootstrap phases (derive for route / setup-project)"
    change: "Line 16 should read: `viola-state`: home 0700/0600 creation (the pinned `bin/<version>-<hash>/viola` is the only 0700 file) and the strict-modes check, built (chunk 2026-10-26-home-and-code-bearing-file-integrity): the write rule on the trusted files, the read rule on the home, `instances/<name>/` and its `diagnostics/` (Unix `mode & 0o077`; Windows any allow ACE, inherit-only included, granting a right in `READ_RIGHTS` (`FILE_READ_DATA` | `GENERIC_READ` | `GENERIC_ALL`) to a SID `trusted_sid` rejects), a symlinked `diagnostics/` folder or file refused (`check_diagnostics`) before obs opens it; `check_home_tree` (the home, the instance's set, `ledger/`, `bin/`, `plugin/`, and with a key `bin/<key>/`, the pinned copy and `plugin/<key>/`) before any log opens in `run`, `revive`'s start and `verify`; on Windows, the protected user + SYSTEM DACL for a `--home` outside `%USERPROFILE%` and the owner + DACL startup check (windows-sys)."
    sidecar: "bootstrap-phases auth-scaffolding-baseline: the viola-state strict-modes bullet gains the read rule as built (diagnostics/ added, the Windows read-grant rule), the diagnostics symlink check and check_home_tree before run's, revive's and verify's logs."
    rationale: "Report Symbols (strict.rs): unix_read_verdict / windows_read_verdict with READ_RIGHTS, check_instance's set widened to the home, the instance dir and diagnostics/ under the read rule, check_diagnostics (symlink refused), check_home_tree and TreeRefused; run.rs checked_exe / revive.rs and verify.rs 64-70 call it before the log; Counts 3 and 4 (the four new names stand in no master — a fact to register)."
    basis: viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/report.md:10
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/bootstrap-phases.md
    near: 17
    section: "## Bootstrap phases (derive for route / setup-project)"
    change: "Line 17 should read: `viola-state`: SHA-256 re-hash of `bin/<version>-<hash>/viola(.exe)` before reuse, `run`'s start reading the exe's bytes once (`pin_key`, `pin_bytes`) and holding `bin/<key>/` and the pinned copy to the write rule (`check_home_tree`) before its log; on Windows `pin_companions` runs `strict::check_present` on `bin/<key>/conpty/` and on each companion that exists before the held open, a refusal being `PinError::StrictModes` with nothing written and the child on the inbox ConPTY, `process-start.sideload_fallback` `strict-modes-failed`. The hash crate is `sha2 =0.11.0` (Decisions Log `2026-09-25`)."
    sidecar: "bootstrap-phases auth-scaffolding-baseline: the pinned-binary bullet gains the home-tree write rule on bin/<key>/ and the pinned copy, and the ConPTY companions' owner/DACL leg (check_present, PinError::StrictModes, sideload_fallback strict-modes-failed)."
    rationale: "Report Symbols (pin.rs: pin_key, pin_bytes, PinError::StrictModes, pin_companions check_present; run.rs checked_exe and sideload_outcome mapping to strict-modes-failed), Schema (diag-line sideload_fallback gains strict-modes-failed) and Retired gap (3), the companions' owner/DACL check, now built; Count 18."
    basis: viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/report.md:13
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/bootstrap-phases.md
    near: 18
    section: "## Bootstrap phases (derive for route / setup-project)"
    change: "Line 18's last sentence should read: `ledger/stamps.json` is written only by `viola verify`, under its `.lock`, with 0600; `update_stamps` runs `strict::check_stamps` before it creates `ledger/` and again under the lock before the capped read, a refusal (`StampsError::StrictModes`) reading and writing nothing, and `verify` holds the home tree to `check_home_tree` before its `cli` log."
    sidecar: "bootstrap-phases auth-scaffolding-baseline: the stamps single-writer sentence gains update_stamps' strict-modes check before ledger/ is created and under the lock (StampsError), and verify's home-tree check before its log."
    rationale: "Report Symbols (stamps.rs: StampsError, check_stamps before create_private_dir(ledger/) and again under the lock; verify.rs check_home_tree 64-70 before its cli log), Deviation 1 (the check before the create, the CARRY 14 trap) and Retired gap (2); Count 17."
    basis: viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/report.md:11
note: "D-security-input, D-security-auth and D-security-deps hold for this category: the chunk adds checks rather than input surfaces, no auth library or secret source, and no dependency (report Dependencies: none); line 24's secret-scanning-ci-gate is untouched."
