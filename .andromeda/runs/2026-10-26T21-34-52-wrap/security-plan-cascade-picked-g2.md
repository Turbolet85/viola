# security-plan · picked accounting · group 2 of 2

## Picked
input-validation#1.1 · not touched · send.rs gains only refuse_untrusted after its text read (190-192); validate_paste_text and the empty-text check still run before any frame, and viola-core is untouched
input-validation#1.2 · not touched · answer.rs:94-96 adds the instance check, which checks no response; the exact set is still refused before any frame on both sides
input-validation#1.4 · proposed · the hook event and dialog paths now run the instance check before stdin, so a refused or absent instance joins the exit-0-nothing-written list
input-validation#1.5 · not touched · tools.rs 337-339 adds check_instance in frame before live_endpoint; the raw-argument checks and invalid-params words stay as stated
input-validation#1.6 · proposed · its hook, verify and fourth-to-eighth interim gaps are retired, and the revive and link clauses still point at those gaps
input-validation#1.7 · not touched · the report lists the config.json thresholds and dialog hold as unchanged; a refused hook reads no config.json, which the row's read rule still covers
input-validation#1.9 · not touched · run.rs's changes (checked_exe, refuse_tree, collision_check, sideload_outcome) leave hold_pump_start as stated
input-validation#1.10 · not touched · the report says FAKE_AGENT_HOOK_PANIC keeps its place after viola_obs_init, so the row's order and fail-open still hold
input-validation#1.12 · proposed · the replay's callers went from 2 to 6, the readers' read_snapshot CARRY and the integrity exceptions are retired, and update_stamps now checks first
input-validation#1.14 · not touched · the report lists the claude agents --json read and child output bounds as unchanged
input-validation#1.16 · not touched · revive still reads the recorded cwd after check_instance (revive.rs:151), now from one shared read, still only as the spawn directory
input-validation#1.17 · not touched · the report lists the session-id argv shape (resume_args) as unchanged
input-validation#1.18 · not touched · run.rs's changes do not include the statusline source read, which stays as stated
input-validation#1.19 · proposed · statusline serve's snapshot read is now read_snapshot_or_replay, and a replay runs nothing
input-validation#1.20 · proposed · the conpty/ owner/DACL leg is built (PinError::StrictModes) and sideload_fallback gains strict-modes-failed
input-validation#2.0 · not touched · Count 17 still finds the over-cap-as-absent stamps read here true; update_stamps keeps its capped read after the check
data-protection#1.1 · proposed · it carries F17, F18 and F19, and its companion item gains the PinError::StrictModes check before the held open
data-protection#3.1 · proposed · it carries F7, the check_diagnostics refusal on the role and detail file opens
bootstrap-phases#1.1 · proposed · the client snapshot-read CARRY is retired, and the viola-state strict-modes item is now the full built set
error-handling#1.1 · proposed · the hook's strict-modes check owed to the entry is now built, with the refused instance exiting 0 with nothing written
error-handling#1.2 · not touched · the server-verify-failed path and SessionEnd's direct append are unchanged; the new instance check comes before them and writes nothing
security-anti-patterns#1.1 · proposed · the frame ban's dated strict-modes exceptions are retired, and every frame now meets it as link/unlink did
security-anti-patterns#3.1 · not touched · home.rs gains only the widen/grant_read/unwiden/still_widened helpers and seed_conpty's create_private_dir; the keepers and the removal rule are untouched
