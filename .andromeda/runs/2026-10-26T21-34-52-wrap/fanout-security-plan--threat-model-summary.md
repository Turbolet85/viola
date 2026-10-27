proposals:
  - detector: D-retired
    severity: warning
    file: .andromeda/security-plan-detail/threat-model-summary.md
    near: 56
    section: "## Threat Model Summary"
    change: "As built, on Linux and Windows every frame-writing client (CLI `send`, `wait`, `last`, `answer`, `pause`, `release`, `link`, `unlink`; `viola hook`'s event and dialog paths; `viola mcp`'s `send` / `wait` / `last` / `answer` tools) first passes the instance's strict-modes check (`check_instance`: the home, the instance directory and its `diagnostics/` to the read rule, `instances/` and `snapshot.json` to the write rule, `events.ndjson` to the write rule on Unix and the read rule on Windows; the CLI verbs through `client::refuse_untrusted`, the hook through `trusted`, the MCP tools before `live_endpoint`), then compares the connected peer's pid and that process's start time with the snapshot's `pid` and `started_at` before its first frame, and sends no frame on a refused check, a mismatch, a check it cannot make or a replayed snapshot (which carries no `endpoint`, `pid` or `started_at`); a refused check writes no line. No dated strict-modes gap remains; macOS, where the peer reports no pid, stays a recorded gap (owed to 'Unix endpoint and home hardening')."
    sidecar: "boundary-ipc: the eight dated strict-modes gaps retired; every frame-writing client now runs check_instance before its snapshot read and first frame, a replayed snapshot is no endpoint (chunk 2026-10-26-home-and-code-bearing-file-integrity)."
    rationale: "Report Retired: gaps (1)-(8) each now run the strict-modes check before their first read or open; Symbols: client.rs refuse_untrusted (20-41) for the six verbs and link_call, hook.rs canonical/trusted (75-86), tools.rs 337-339, liveness.rs 92-94 (only Recovered::Snapshot proceeds); the founder's P4 answer Q2 (no line on a refused check, inputs#I3)."
    basis: .andromeda/security-plan-detail/threat-model-summary.md:56
  - detector: D-retired
    severity: warning
    file: .andromeda/security-plan-detail/threat-model-summary.md
    near: 124
    section: "## Threat Model Summary"
    change: "Readers tolerate torn lines (they skip and count them, and the next append heals a torn last line). The replay of the log for a snapshot that fails to parse, or carries a newer `v`, has six product readers, each taking it after the strict-modes check on the instance's files: `viola revive`'s preflight (one read handed to both its collision check and its recorded-directory reading), `viola list` (`viola_state::board::read`), `viola hook`'s `connect`, `hook statusline`'s `serve`, `run`'s `collision_check` and `viola_state::liveness::live_endpoint` (the CLI frame verbs and `viola mcp`'s frame tools); `read_snapshot` has no product reader under `src/`. A replayed snapshot carries no `endpoint`, `pid`, `started_at` or `statusline_command`: it is no live endpoint, the hook fails open unreachable, the statusline runs nothing, and `run`'s collision check reads liveness from the beat alone (the founder's boundary-widening ratification, chunk 2026-10-26-home-and-code-bearing-file-integrity `inputs#I3` Q1; Standard Contracts: Snapshot envelope)."
    sidecar: "boundary-filesystem: the snapshot replay's product readers 2 -> 6 (hook connect, statusline serve, run collision_check, live_endpoint added), read_snapshot product readers 3 -> 0 (chunk 2026-10-26-home-and-code-bearing-file-integrity)."
    rationale: "Report Retired: read_snapshot as a product reader under src/ (3 -> 0); Counts 5 (replay product readers 2 -> 6); Symbols liveness.rs 92-94, hook.rs connect 224-226, statusline.rs serve 128-132, run.rs collision_check 425-432, Deviation 4 (revive reads once); Decisions: inputs#I3 Q1."
    basis: .andromeda/security-plan-detail/threat-model-summary.md:124
  - detector: D-retired
    severity: warning
    file: .andromeda/security-plan-detail/threat-model-summary.md
    near: 126
    section: "## Threat Model Summary"
    change: "Beyond the content hash in the folder name, the strict-modes check holds the code-bearing tree: `run`'s and `revive`'s start run `check_home_tree` before any log (the home to the read rule, the instance's `check_instance` set, then `ledger/`, `bin/`, `plugin/`, `bin/<key>/`, the pinned `viola(.exe)`, `plugin/<key>/`, its `hooks/` and `.claude-plugin/` to the write rule, each that exists; a refusal is exit 1 with the home or instance pair, no log, no line, no child); `verify` runs it on the home, `ledger/`, `bin/` and `plugin/` before its log, and `update_stamps` refuses a widened `ledger/` before reading; on Windows x64 each `bin/<key>/conpty/` companion passes `check_present` (owner/DACL) before its held open, a refusal leaving the child on the inbox ConPTY with `sideload_fallback` `strict-modes-failed`. `marketplace/` stays off the integrity set (the founder's Epoch 5 word; CARRY 17 closed by ruling, `inputs#I4` row 2)."
    sidecar: "boundary-filesystem: the code-bearing tree is now under the strict-modes check (check_home_tree at run/revive/verify start, the conpty/ owner/DACL leg), not the content hash alone (chunk 2026-10-26-home-and-code-bearing-file-integrity)."
    rationale: "Report Retired gap (3) (ConPTY companions under the re-hash alone) and gap (2) (update_stamps); Symbols strict.rs check_home_tree (91-125) / TreeRefused, run.rs checked_exe/refuse_tree (189-215), revive.rs 125-135, verify.rs 64-70, stamps.rs StampsError, pin.rs pin_companions check_present; Counts 4, 17, 18; inputs#I4 row 2."
    basis: .andromeda/security-plan-detail/threat-model-summary.md:126
    dependent-of: D-retired
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/threat-model-summary.md
    near: 125
    section: "## Threat Model Summary"
    change: "Writes under `instances/<name>/diagnostics/` (`open_role_file`, `write_detail`) first run `strict::check_diagnostics` on the existing folder and the file, before anything is narrowed: a symbolic link is refused, a folder is held to the read rule, a file to the read rule on Unix and the write rule on Windows; a refusal is exactly a failed open (the writer ends at a sink, a detail line dropped)."
    sidecar: "boundary-filesystem: diagnostics/ opens refuse a symlinked or widened folder or file (check_diagnostics) (chunk 2026-10-26-home-and-code-bearing-file-integrity)."
    rationale: "Report Symbols: strict.rs check_diagnostics (127-145), obs.rs check_diagnostics (69-76) in open_role_file and write_detail; Coverage of new surfaces: diagnostics opens."
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/threat-model-summary.md
    near: 86
    section: "## Threat Model Summary"
    change: "The event and dialog paths take the instance from `VIOLA_DIR`, canonicalise it and pass it through `check_instance` before any stdin read, `config.json` read, role file or frame: a refused instance, or a `VIOLA_DIR` naming no existing directory, exits 0 at once with no body and writes nothing (no line, no frame, no SessionEnd append); the two `--capture` arms stay ahead of the check, unchanged."
    sidecar: "boundary-hook-stdin: the hook event and dialog paths run the instance check before reading stdin; a refused or absent instance fails open writing nothing (chunk 2026-10-26-home-and-code-bearing-file-integrity)."
    rationale: "Report Symbols: src/cmd/hook.rs canonical/trusted (75-86) before read_diagnostics_level / viola_obs_init; Deviation 5 (a VIOLA_DIR naming no directory writes nothing); Counts 23."
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/threat-model-summary.md
    near: 111
    section: "## Threat Model Summary"
    change: "Trust boundary: same-user invocation, clap parsing, the `ViolaName` nutype, tolerant `config.json` parsing, and the strict-modes check on the chosen home before anything under it is read or opened: the six frame verbs and `link` / `unlink` (`refuse_untrusted`: exit 21, the instance pair or the `instance-unreachable` document), `run` and `revive`'s start (`check_home_tree`, exit 1), `verify` (exit 1, the home pair), `viola hook` (exit 0, nothing) and `viola mcp`'s frame tools (`instance-unreachable`, `detail: null`); a refused check writes no line, and an unstattable home (a path under a regular file) is a refusal on Unix."
    sidecar: "boundary-cli: the home chosen by --home / VIOLA_DIR passes the strict-modes check at every entry point before any read (chunk 2026-10-26-home-and-code-bearing-file-integrity)."
    rationale: "Report Symbols client.rs refuse_untrusted, run.rs/revive.rs checked_exe, verify.rs 64-70, hook.rs trusted, tools.rs 337-339; Counts 8, 9, 13; Decisions inputs#I3 Q2/Q3."
note: "The body's broad line .andromeda/security-plan.md:47 restates the retired claim «nothing checks integrity beyond the content hash in the folder name» (the body's Filesystem vector) — owned by the body's reconciler, not this category file; D-security-input, D-security-auth and D-security-deps hold (no new input surface unvalidated, no auth library change, Dependencies: none)."
