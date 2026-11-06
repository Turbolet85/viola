@answer n=1 entity=established-decisions#20.1 v=no
why: the rmcp 3.4.1 stdio bound and CappedLines sit in viola-mcp and rmcp, which the session does not reach and the task keeps unchanged
@answer n=2 entity=established-decisions#21.1 v=yes mode=contradict
why: the session removed the Git Bash path rewriting, so the rationale that exec form skips Git Bash path rewriting no longer holds and shell startup alone remains
@answer n=3 entity=established-decisions#22.1 v=yes mode=stale
why: src/cmd/hook.rs is reached whole by the Windows-remnant removal, and the connect sequence this entity states lives in that file and is not shown to hold on the changed file
@answer n=4 entity=established-decisions#23.1 v=yes mode=stale
why: src/cmd/hook.rs is reached whole by the Windows-remnant removal, and the fail-open, no-stderr and hold-read rules live in that file, with no report showing they hold on it
@answer n=5 entity=established-decisions#24.1 v=no
why: the dialog-expired append lives in the wrapper under src/run, which the session does not reach and the task does not name
@answer n=6 entity=established-decisions#24.2 v=no
why: the 671 s live hold is a dated measurement at its own chunk, and the session does not move that reading
@answer n=7 entity=established-decisions#24.3 v=yes mode=stale
why: src/cmd/hook.rs is reached whole, and the server-check fail-open rule this entity states lives in that file, not shown to hold on the changed file
@answer n=8 entity=established-decisions#24.4 v=yes mode=stale
why: src/cmd/hook.rs is reached whole, and the socket-dir-refused and server-verify-failed fail-open rule lives in that file, not shown to hold on it
@answer n=9 entity=established-decisions#24.5 v=yes mode=stale
why: src/cmd/hook.rs is reached whole, and the instance-check ordering this entity states lives in that file, not shown to hold on the changed file
@answer n=10 entity=established-decisions#25.1 v=yes mode=contradict
why: the task moves the dialog-hold ceiling owner from Version done-check to 0.2.0's first entry and records that the hold was not read, so the first live read at Version done-check no longer stands
@answer n=11 entity=established-decisions#26.1 v=no
why: the wheel rule lives in src/run/wheel.rs, which the session does not reach, and the session adds no harness prefix or wheel change
@answer n=12 entity=established-decisions#27.1 v=no
why: the link rule lives in src/run/links.rs and the wrapper, which the session does not reach, and the link verbs are unchanged
@answer n=13 entity=established-decisions#27.2 v=yes mode=stale
why: the explicit verbs' shared refusal path refuse_untrusted lives in src/cmd/client.rs, which the session reaches whole
@answer n=14 entity=established-decisions#27.3 v=no
why: the no-span and no-line rule for link and unlink lives in the wrapper, which the session does not reach, and the dispatch span names are unchanged
@answer n=15 entity=established-decisions#27.4 v=no
why: LinkSet::line lives in src/run/links.rs, which the session does not reach
@answer n=16 entity=established-decisions#28.1 v=yes mode=add-to
why: the budget gate's send refusal rests on the statusline-rate-limits row, which F1 now reads as the account's plan, and the open question whether the gate stays, F2, belongs beside this entity
@answer n=17 entity=established-decisions#29.1 v=yes mode=stale
why: the session's live round read statusline-rate-limits 23 pass 1 fail on the unchanged CLI 2.1.287 (F1), so the stamped 24 pass 0 fail counts and the row's pure CLI reading are stale
@answer n=18 entity=established-decisions#30.1 v=yes mode=stale
why: check_home_tree and the update_stamps check_stamps live in src/strict.rs and stamps.rs, both reached whole, and the claim is not shown to hold on them
@answer n=19 entity=established-decisions#30.2 v=no
why: the Ended::Refused codes are written by the verify module, which the session does not reach, and no code is added or changed
@answer n=20 entity=established-decisions#31.1 v=yes mode=stale
why: run::version_gate lives in src/run/version_gate.rs, which the session reaches whole, and the start order and version gate step this entity states are not shown to hold on the changed file
@answer n=21 entity=established-decisions#31.2 v=no
why: the collision_check span and its log outcomes live in unreached run code, and the session changes no log field or event kind
@answer n=22 entity=established-decisions#31.3 v=yes mode=stale
why: the server-identity seams cite src/cmd/client.rs connect and src/cmd/hook.rs connect, both reached whole, so the claim is not shown to hold on them
@answer n=23 entity=established-decisions#31.4 v=yes mode=stale
why: client::Unreachable::without_endpoint lives in src/cmd/client.rs, which the session reaches whole
@answer n=24 entity=established-decisions#31.5 v=no
why: the run collision check's replayed-snapshot rule lives in unreached run code, and the session moves no heartbeat or snapshot behaviour
@answer n=25 entity=established-decisions#31.6 v=no
why: the revive preflight reading lives in code that no task-named reached file holds, and the session adds or changes no preflight reading
