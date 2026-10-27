proposals:
  - detector: D-retired
    severity: warning
    section: Authentication & Authorization
    change: "IPC client-side server verification row: before the first frame of any method the client reads its one snapshot only after the instance's strict-modes check (CLI send, wait, last, answer, pause, release, link and unlink through refuse_untrusted; viola hook's event and dialog paths before obs init; viola mcp's frame tools through check_instance), a refused check writing no frame and no line (CLI exit 21 instance pair or the instance-unreachable document, MCP instance-unreachable with detail null, hook exit 0 with no body); then on Linux and Windows it verifies the connected peer's pid (Windows over an SQOS identification-level open) and that process's start time, a mismatch, a replayed snapshot (no endpoint, pid or started_at) or a check that cannot be made sending no frame, never falling back to a default connect; no strict-modes gap remains, and only the client euid check and macOS (no peer pid; its euid and verified socket-directory checks) stay owed to «Unix endpoint and home hardening»."
    sidecar: "Authentication & Authorization, server verification row: the dated strict-modes gap (snapshot read without the check, link/unlink excepted) retired by chunk 2026-10-26-home-and-code-bearing-file-integrity; every client now checks the instance first, a replayed snapshot sends no frame; euid and macOS still owed to «Unix endpoint and home hardening»."
    rationale: "Report Retired bullet: the eight dated strict-modes gaps (hook.event, hook.dialog, CLI send, wait/last, answer, pause/release, viola mcp tools) each now run the strict-modes check before the first read or open; Symbols: refuse_untrusted called first by the six verbs, hook.rs canonical/trusted before obs init, tools.rs check_instance before live_endpoint; live_endpoint and hook connect read read_snapshot_or_replay, Replayed is None/Unreachable; founder's P4 answers inputs#I3 (no line on a refused check, link's instance pair)."
    basis: "src/cmd/client.rs:20-41"
  - detector: D-retired
    severity: warning
    section: Authentication & Authorization
    change: "~/.viola/ access control row: drop «with its dated interim gaps»; the strict-modes check refuses a trusted path another user can write, or a home, instance directory or diagnostics/ another user can read (Unix any group or other bit; Windows any allow ACE granting a READ_RIGHTS right to another SID, inherit-only included), and a symlinked diagnostics path, in every process that reads a trusted file, before its first read or open under the home (run and revive's start over the home tree, verify, the frame verbs, hook, the MCP frame tools, the diagnostics opens), with no dated gap."
    sidecar: "Authentication & Authorization, ~/.viola/ access control row: «dated interim gaps» retired and the read rule stated as built (home, instance dir, diagnostics/; Windows READ_RIGHTS), chunk 2026-10-26-home-and-code-bearing-file-integrity."
    rationale: "Report Retired bullet (all eight gaps closed); Symbols: strict.rs unix_read_verdict / windows_read_verdict, check_instance's widened set (home, instance dir, diagnostics/ to the read rule), check_home_tree, check_diagnostics (symlink refused); run/revive checked_exe before open_wrapper_log, verify check_home_tree before its cli log, obs.rs check_diagnostics in open_role_file and write_detail."
    basis: "crates/viola-state/src/strict.rs:91-125"
    dependent-of: D-retired
  - detector: D-retired
    severity: warning
    section: Input Validation
    change: "CLI arguments / stdin row: drop «except the ratified dated interim gaps until the Epoch 6 entries land»; the home is resolved, canonicalised and passed through the strict-modes check before any snapshot, statusline_command or ledger read, with no dated gap (update_stamps checks ledger/stamps.json before creating ledger/ and again under its lock, a refusal reading and writing nothing)."
    sidecar: "Input Validation, CLI arguments / stdin row: the dated interim strict-modes gaps retired, chunk 2026-10-26-home-and-code-bearing-file-integrity."
    rationale: "Report Retired bullet items (2) update_stamps and (4)-(7) the CLI frame verbs; Counts 1 names security-plan.md:142 as a standing gap-until hit gone stale; Symbols: stamps.rs StampsError, check_stamps before create_private_dir and under the lock."
    basis: ".andromeda/security-plan.md:142"
    dependent-of: D-retired
  - detector: D-retired
    severity: warning
    section: Input Validation
    change: "Own state files on read row: drop «except the ratified dated gaps»; every own state file is read only after the home strict-modes check, no dated gap remaining; a snapshot that fails to read is replayed from the log by its readers (run's collision check, revive, list, hook connect, hook statusline, live_endpoint), and a replayed snapshot carries no endpoint, pid, started_at or statusline_command, so no frame is written and no command runs from it."
    sidecar: "Input Validation, own state files on read row: the dated gaps retired and the snapshot readers on the replay (no endpoint, pid, started_at or statusline_command from a replay), chunk 2026-10-26-home-and-code-bearing-file-integrity."
    rationale: "Report Retired bullet (the eight gaps; read_snapshot as a product reader under src/ 3 → 0); Symbols: live_endpoint, hook connect, statusline serve and collision_check read read_snapshot_or_replay, a Replayed snapshot proceeding to no frame and no command; Counts 5; founder's boundary-widening ratification inputs#I3 Q1."
    basis: "crates/viola-state/src/liveness.rs:92-94"
    dependent-of: D-retired
  - detector: D-retired
    severity: warning
    section: Security Anti-Patterns (NEVER do these) › Authentication
    change: "Channel-frame ban: drop the strict-modes exception (hook.event, hook.dialog, the CLI send, wait / last, answer, pause / release frames and viola mcp's tool frames verifying against a snapshot read without the check); every client's frame follows a snapshot that passed the strict-modes check in the same process, and the only dated gaps left are every client's euid check and macOS's pid and start-time check (no peer pid), owed to «Unix endpoint and home hardening»; no other frame or process may borrow them."
    sidecar: "Security Anti-Patterns › Authentication, channel-frame ban: the strict-modes gaps of the hook, CLI and MCP frames retired; euid and macOS gaps kept, chunk 2026-10-26-home-and-code-bearing-file-integrity."
    rationale: "Report Retired bullet items (1), (4)-(8): hook.event, CLI send, wait/last, answer and hook.dialog, pause/release, viola mcp tools each now run the strict-modes check before the first read or open; Outcome criteria (security) rows 1-3 MET on three OSes (ci#40403389629)."
    basis: "crates/viola-mcp/src/tools.rs:337-339"
    dependent-of: D-retired
  - detector: D-retired
    severity: warning
    section: Input Validation
    change: "DLL search order + sideloaded ConPTY companions row: the companions are loaded only after the owner/DACL strict-modes check on bin/<key>/conpty/ and on each companion that exists (check_present, before the held open) and a full SHA-256 re-hash under a read-share-only handle, by absolute path; any failure, a refused check included (process-start sideload_fallback strict-modes-failed, nothing written), degrades to the inbox ConPTY, recorded only as codes, never a refusal or an exit change."
    sidecar: "Input Validation, DLL search order + ConPTY companions row: the owner/DACL leg built beside the re-hash (PinError::StrictModes → sideload_fallback strict-modes-failed), chunk 2026-10-26-home-and-code-bearing-file-integrity."
    rationale: "Report Retired bullet item (3): Windows x64 ConPTY companions loaded under the re-hash alone, without the owner/DACL check; Symbols: pin_companions runs strict::check_present on bin/<key>/conpty/ and each companion before the held open, refusal PinError::StrictModes; sideload_outcome maps it to strict-modes-failed (run.rs:510); Schema: sideload_fallback gains strict-modes-failed; Outcome MET on windows-2025."
    basis: "src/cmd/run.rs:510"
