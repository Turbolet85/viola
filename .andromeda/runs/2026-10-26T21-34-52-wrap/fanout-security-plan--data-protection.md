proposals:
  - detector: D-retired
    severity: warning
    file: .andromeda/security-plan-detail/data-protection.md
    near: 16
    section: '## Data Protection'
    change: 'Add the owner/DACL leg to the ConPTY companions item: before the held open, `pin_companions` runs `strict::check_present` on `bin/<key>/conpty/` and on each companion that exists (the write rule, an unstattable path refused); a refusal is `PinError::StrictModes`, nothing written, overwritten or deleted, nothing loaded or launched from that folder, and the child runs on the inbox ConPTY with `process-start.sideload_fallback` `strict-modes-failed`, exit unchanged; only then is each opened with `FILE_SHARE_READ` only and re-hashed in full against the compiled pin, a mismatch staying `PinError::HashMismatch` (`sideload_fallback` `hash-mismatch`); keep the rest of the item unchanged.'
    sidecar: 'data-protection code-bearing artefacts, ConPTY companions: the re-hash-alone gap retired; check_present on conpty/ and each companion before the held open, PinError::StrictModes -> sideload_fallback strict-modes-failed (chunk 2026-10-26-home-and-code-bearing-file-integrity).'
    rationale: 'The report''s Retired bullet retires gap (3) (Windows x64 ConPTY companions loaded under the re-hash alone, without the owner/DACL check); Count 18 names this file''s line 16 stale (`PinError` variants); Symbols: `viola_state::pin` `PinError::StrictModes`, `pin_companions` running `strict::check_present`; `run.rs:510` maps it to `strict-modes-failed`; Schema: the `sideload_fallback` enum gains `strict-modes-failed`; Outcome: the `conpty/` criterion MET on `windows-2025`.'
    basis: .andromeda/security-plan-detail/data-protection.md:16
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/data-protection.md
    near: 11
    section: '## Data Protection'
    change: 'Replace «The strict-modes check runs at startup» with: the strict-modes check runs at every entry point before its first read or open under the home (Authentication & Authorization, `~/.viola/` access control) — `viola run` and `viola revive`''s start through `checked_exe` / `check_home_tree(home, Some(name), Some(key))` before the wrapper log, `viola verify` through `check_home_tree(home, None, None)` before its `cli` log, the six frame verbs through `client::refuse_untrusted`, `viola hook`''s event and dialog paths and `hook statusline` through `check_instance`, `viola mcp`''s frame tools through `check_instance` before `live_endpoint`; a trusted path another user can write is refused (`Writable`, the write rule), and the home, the instance directory and its `diagnostics/` another user can read are refused too (`Readable`, the read rule: Unix `mode & 0o077 != 0`, Windows any allow ACE granting a right in `READ_RIGHTS` = `0x9000_0001` to a SID `trusted_sid` rejects, inherit-only ACEs included; on Windows `events.ndjson` to the read rule too), each that exists, a folder before what it holds; a refusal opens nothing under the home and writes no line.'
    sidecar: 'data-protection Files: the strict-modes check now runs at every entry point before any read or open, the read rule (Readable) on the home, the instance dir and diagnostics/ (chunk 2026-10-26-home-and-code-bearing-file-integrity).'
    rationale: 'Report Chunk line (the home''s strict-modes check before every read or open at every entry point); Symbols: `viola_state::strict` `Refused::Readable`, `unix_read_verdict`, `windows_read_verdict`, `READ_RIGHTS`, `check_instance`''s widened set, `check_home_tree`; `src/cmd/client.rs` `refuse_untrusted`; `src/cmd/hook.rs` `canonical` / `trusted`; `tools.rs` 337-339; `run.rs` `checked_exe`; `verify.rs` 64-70; Counts 2, 3, 4, 20; Decisions: no line on a refused check (`inputs#I3` Q2).'
    basis: .andromeda/security-plan-detail/data-protection.md:11
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/data-protection.md
    near: 10
    section: '## Data Protection'
    change: 'After the Windows sub-item add one item: as built and witnessed at chunk 2026-10-26-home-and-code-bearing-file-integrity, a start under umask 000 leaves every home path owner-only on Unix (0700 directories, 0600 files, the pinned `viola` 0700); on Windows the protected owner-only DACL of a home viola makes outside the profile reads `D:PAI(A;OICI;FA;;;LA)(A;OICI;FA;;;SY)` on `windows-2025` (canonical SDDL spells the runner''s local Administrator, RID 500, `LA`), read back through `icacls /save`; a home made before viola with `create_dir_all` (an inherited `S-1-5-32-545` Users write ACE) is refused `Writable`, and a pre-existing home another user may read (Unix 0750, Windows a Users read grant) refuses `viola run`, exit 1.'
    sidecar: 'data-protection Files: umask-independent owner-only modes and the measured protected DACL of a home outside the profile recorded; a pre-existing widened or readable home refused (chunk 2026-10-26-home-and-code-bearing-file-integrity).'
    rationale: 'Report Cross-project claims (measured on `windows-2025`: the protected DACL `D:PAI(A;OICI;FA;;;LA)(A;OICI;FA;;;SY)`, `LA` for RID 500); Spec claims disproved (a `create_dir_all` home refused `Writable`, the inherited Users write ACE); Deviations 6, 7 (d), 10; Outcome tests criterion (umask-independent modes, the Windows `--home` outside the profile, a Users-read dir refused exit 1); Decisions: `icacls /save`.'
    basis: .andromeda/security-plan-detail/data-protection.md:10
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/data-protection.md
    near: 15
    section: '## Data Protection'
    change: 'Append to the pinned-binary item: the folder name is `pin::pin_key(bytes)` (`<VERSION>-<16 hex>`); `viola run`''s start (and `viola revive`''s) reads the exe''s bytes once (`checked_exe`), runs `check_home_tree` with that key (`bin/<key>/`, the pinned `viola(.exe)`, `plugin/<key>/` and its `hooks/` and `.claude-plugin/`, the write rule, each that exists) before its wrapper log, and pins those same bytes (`pin_bytes`; `pin_exe` keeps its signature and delegates to it); a refused tree is exit 1 with the home or instance pair (`refuse_tree`), no log, line or child; `pin_exe`''s first-start race and reproducible builds are owed to the route entry «Pinned copy race and reproducible builds» (CARRYs 16 and 11, the founder''s own word, `inputs#I3` Q4).'
    sidecar: 'data-protection code-bearing artefacts, pinned binary: pin_key / pin_bytes, the home tree with the key checked before the wrapper log; the pin race carved to its own route entry (chunk 2026-10-26-home-and-code-bearing-file-integrity).'
    rationale: 'Symbols: `viola_state::pin` `pin_key` (71-74), `pin_bytes`, `pin_exe` delegating; `src/cmd/run.rs` `checked_exe` (189-200), `refuse_tree` (202-215), `run` (166-172) and `revive`''s start (revive.rs:125-135) before `open_wrapper_log`; `check_home_tree` (strict.rs 91-125); Deviation 3; Count 22 (`pin_key` / `pin_bytes` new, `pin_exe`''s race and reproducible builds carved out); Decisions: CARRYs 11 and 16 carved (`inputs#I3` Q4, `inputs#I4` row 1).'
    basis: .andromeda/security-plan-detail/data-protection.md:15
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/data-protection.md
    near: 17
    section: '## Data Protection'
    change: 'Append to the stamps item: `update_stamps` runs `strict::check_stamps(home)` before `create_private_dir(ledger/)` and again under the lock before the capped read; a refusal is `StampsError::StrictModes` (fixed message «the capability stamps failed the strict-modes check»), reading and writing nothing, and `viola verify` maps it to the home pair, exit 1, `stamps.json` unchanged; the single writers of `ledger/stamps.json` (`viola verify`) and `snapshot.json` (the instance''s own wrapper) are witnessed byte-identical after every other writer-capable entry point, the wrapper stopped.'
    sidecar: 'data-protection code-bearing artefacts, ledger/stamps.json: check_stamps before create_private_dir and under the lock (StampsError::StrictModes); single writers witnessed with the wrapper stopped (chunk 2026-10-26-home-and-code-bearing-file-integrity).'
    rationale: 'Symbols: `viola_state::stamps` `StampsError { StrictModes, State }` (37-44), `update_stamps` checking before `create_private_dir` and under the lock, `verify.rs` 288-296; Deviation 1 (the check before the create, the CARRY 14 trap); Count 17; Count 16 (single writers with the wrapper stopped); Outcome criteria (`verify` refuses a widened `ledger/`, `stamps.json` unchanged; single writers byte-identical, `only_the_wrapper_writes_the_snapshot_and_only_verify_the_stamps`).'
    basis: .andromeda/security-plan-detail/data-protection.md:17
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/data-protection.md
    near: 20
    section: '## Data Protection'
    change: 'New item after the interactive probe dirs: **Diagnostics files (`diagnostics/`):** before `ensure_private_dir` narrows anything, `open_role_file` and `write_detail` (`src/obs.rs`) run `strict::check_diagnostics` on the existing `diagnostics/` folder and on the role or detail file: a symbolic link is refused (`Symlink`), a folder is held to the read rule, a file to the read rule on Unix (no group or other bit) and the write rule on Windows, an absent path passes; a refusal is exactly a failed open (the writer ends at `std::io::sink`, a detail line dropped), and the path is left as found.'
    sidecar: 'data-protection: new Diagnostics files item — check_diagnostics (symlink + read rule) before any diagnostics open, a refusal a failed open ending at sink (chunk 2026-10-26-home-and-code-bearing-file-integrity).'
    rationale: 'Symbols: `viola_state::strict` `check_diagnostics` (127-145), `Refused::Symlink`, `DIAGNOSTICS`; `src/obs.rs` `check_diagnostics` (69-76) in `open_role_file` and `write_detail`, units 610-673 (`open_role_file_refuses_a_widened_diagnostics_path_and_leaves_it`, `open_role_file_refuses_a_symlinked_role_file`, `write_detail_refuses_a_widened_diagnostics_folder_and_leaves_it`); Coverage row «diagnostics opens».'
    basis: .andromeda/security-plan-detail/data-protection.md:20
note: 'D-security-input, D-security-auth and D-security-deps hold for this file (no new external input unvalidated, no auth library or secret source, Dependencies none); line 18 (`marketplace/`) stays true as written — CARRY 17 closed by ruling (`inputs#I4` row 2), `marketplace/` off the integrity set, so no proposal there.'
