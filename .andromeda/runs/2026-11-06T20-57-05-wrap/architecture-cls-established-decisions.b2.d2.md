@answer n=1 entity=established-decisions#20.1 v=no
why: rmcp stays 3.4.1 and its stdio reader is untouched, so the bound in viola-mcp still holds on the changed Cargo.lock.
@answer n=2 entity=established-decisions#21.1 v=yes mode=contradict
why: the Git Bash path-rewriting reason in the exec-form rationale is gone with the Windows remnants, leaving shell startup as the live reason (fct-zhr3wx2i).
@answer n=3 entity=established-decisions#22.1 v=no
why: the event and dialog connect sequence (replay, check_endpoint_dir, connect_by, peer and server checks) is unchanged, and the live round re-witnessed the 0700 socket directory.
@answer n=4 entity=established-decisions#23.1 v=no
why: the fail-open contract, the PROVISIONAL dialog-hold ceiling and its 3615 s timeout stand unchanged; only the ceiling's owner moves, carried by #25.1.
@answer n=5 entity=established-decisions#24.1 v=no
why: the dialog-expired line and its WaitFeed append are unchanged by the session.
@answer n=6 entity=established-decisions#24.2 v=no
why: the 671 s hold is a fixed historical measurement from chunk 2026-10-19, and the session's 2.1.287 verify run held no dialog.
@answer n=7 entity=established-decisions#24.3 v=no
why: the no-hook.dialog path for a failing server check is unchanged, and the live round did not touch it.
@answer n=8 entity=established-decisions#24.4 v=no
why: the socket-dir-refused and server-verify-failed fail-open outcomes keep their codes and exit 0, with no change in the hook path.
@answer n=9 entity=established-decisions#24.5 v=no
why: the instance check ahead of diagnostics and obs initialisation is unchanged, and no config or strict-modes read moved.
@answer n=10 entity=established-decisions#25.1 v=yes mode=contradict
why: the hold is not read at Version done-check, so that owner moves to 0.2.0's first entry and the ceiling stays PROVISIONAL (fct-4nt6qtyp).
@answer n=11 entity=established-decisions#26.1 v=no
why: the wheel's byte classification and send window are unchanged, and viola-pty's pump.rs changed by comment edits only.
@answer n=12 entity=established-decisions#27.1 v=no
why: link recording and the LinkSet rule are unchanged, and no channel method or event kind was added.
@answer n=13 entity=established-decisions#27.2 v=no
why: refuse_untrusted, the connect checks and the exit-21 refusals are unchanged, and the client.rs edit removes Windows remnants only.
@answer n=14 entity=established-decisions#27.3 v=no
why: no span or process-log line for link or unlink was added or changed.
@answer n=15 entity=established-decisions#27.4 v=no
why: links.rs is not reached, so the Utc::now stamp at its cited line is unchanged.
@answer n=16 entity=established-decisions#28.1 v=yes mode=add-to
why: F2 (whether the budget gate stays and what it stands on, the founder's F-06i question, owner 0.2.0's intent) joins this entry, since the budget-paused refusal rests on the statusline-rate-limits row.
@answer n=17 entity=established-decisions#29.1 v=yes mode=add-to
why: F1 (the statusline-rate-limits row reads the account's plan state, five_hour unknown on this account) joins the rate-limits row, and the 2.1.287 stamp now reads 23 pass 1 fail, so the 24-pass count is stale.
@answer n=18 entity=established-decisions#30.1 v=no
why: the strict-modes check order and the StampsError::StrictModes refusal hold, the dev home's stamps are not moved, and stamps.rs changed only for Windows remnants.
@answer n=19 entity=established-decisions#30.2 v=no
why: the Ended::Refused codes and the no-line refusal of a refused home tree are unchanged.
@answer n=20 entity=established-decisions#31.1 v=no
why: pid plus start-time liveness and the heartbeat decision are unchanged, and liveness.rs is not reached.
@answer n=21 entity=established-decisions#31.2 v=no
why: the collision_check log lines and the run.collision_check outcome set are unchanged.
@answer n=22 entity=established-decisions#31.3 v=no
why: the connect seams and the verify_server order hold, and no verb was added or removed, so the eight callers stand.
@answer n=23 entity=established-decisions#31.4 v=no
why: without_endpoint lives in liveness.rs, which is not reached, and its directory rule is unchanged.
@answer n=24 entity=established-decisions#31.5 v=no
why: the replayed-snapshot heartbeat decision in run's collision check is unchanged.
@answer n=25 entity=established-decisions#31.6 v=no
why: the revive preflight reading order and its single snapshot read are unchanged.
