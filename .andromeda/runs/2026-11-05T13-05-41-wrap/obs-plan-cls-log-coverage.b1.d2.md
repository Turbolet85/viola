@answer n=1 entity=log-coverage#1.1 v=yes mode=contradict
why: the session drops the sideload_fallback property, leaves the conpty-sideload and conpty pty_backend words, and narrows os to linux (touches 13, schema item 4).
@answer n=2 entity=log-coverage#1.2 v=no
why: the process-exit row names exit_source values and ties kill-fallback to no TerminateProcess, and exit_source is not among the schema changes, so the claim holds.
@answer n=3 entity=log-coverage#1.3 v=no
why: the channel-request fields are not named by any touch, the channel change being endpoint transport only.
@answer n=4 entity=log-coverage#1.4 v=no
why: the channel-response closed method, detail and error-code sets are not among the schema changes, so they hold.
@answer n=5 entity=log-coverage#1.5 v=no
why: the send event fields are untouched, and the send scenario has no OS arm moved (untouched item 6).
@answer n=6 entity=log-coverage#1.6 v=no
why: the dialog event fields are untouched, and the dialog-answer scenario has no OS arm moved (untouched item 6).
@answer n=7 entity=log-coverage#1.7 v=no
why: the hook_event enum and the hook fields are not among the schema changes and no touch names them.
@answer n=8 entity=log-coverage#1.8 v=no
why: the panic row's location form and thread field are not named by any touch and are not among the schema changes, so the claim holds.
@answer n=9 entity=log-coverage#1.9 v=no
why: the release-from-driver fields are not named by any touch, and the channel change is endpoint-level only.
@answer n=10 entity=log-coverage#1.10 v=no
why: the state-recovered details and file and scanned fields are not among the schema changes, and the viola-state change leaves the Unix refusals unchanged (touch 25).
@answer n=11 entity=log-coverage#1.11 v=no
why: the parse-rejected parser list and its details are not among the schema changes, and no touch removes a parser or detail.
@answer n=12 entity=log-coverage#2.1 v=yes mode=contradict
why: batch-script-child leaves the exit-1 detail list and the process-exit detail enum closes at 19, not 20 (touch 12, schema item 4).
@answer n=13 entity=log-coverage#2.2 v=yes mode=stale
why: the server-verify-failed parenthetical names Linux, Windows and macOS, which is now Linux alone (touches 1 and 11), while the exit-21 detail list itself holds.
@answer n=14 entity=log-coverage#2.3 v=no
why: wrapper-fault is not among the schema changes and no touch names it.
@answer n=15 entity=log-coverage#2.4 v=yes mode=stale
why: the server-verify-failed parenthetical names Linux, Windows and macOS, which is now Linux alone (touches 1 and 11), while the fail-open code list holds.
@answer n=16 entity=log-coverage#2.5 v=no
why: the parse-rejected per-parser details are not among the schema changes, and the ledger-stamps strict-modes refusal stays on Unix (touch 25).
@answer n=17 entity=log-coverage#2.6 v=no
why: the mcp-stdin oversize line is not touched and is not among the schema changes.
@answer n=18 entity=log-coverage#2.7 v=no
why: the refusal details are RefusalReason values that no touch changes.
@answer n=19 entity=log-coverage#2.8 v=no
why: the filesystem refusals other than the Windows owner/DACL arm are unchanged (touch 25), and strict-modes-failed stays in the hook-decision schema enum.
@answer n=20 entity=log-coverage#3.1 v=no
why: the subject enum and the spawn pairs are not among the schema changes, and no spawn is removed.
