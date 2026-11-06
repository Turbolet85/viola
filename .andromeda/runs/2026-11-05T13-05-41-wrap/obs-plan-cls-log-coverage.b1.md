@answer n=1 entity=log-coverage#1.1 v=yes mode=contradict
why: the session drops the process-start sideload_fallback property and the conpty-sideload and conpty pty_backend values, so pty_backend reads openpty only
@answer n=2 entity=log-coverage#1.2 v=no
why: exit_source keeps handle-wait and kill-fallback, and the entity ties kill-fallback to no TerminateProcess, so its claim holds
@answer n=3 entity=log-coverage#1.3 v=no
why: the channel-request field list names no OS, endpoint or retired word, and the session changes none of those fields
@answer n=4 entity=log-coverage#1.4 v=no
why: the channel-response field list, its closed method list and its -32602 detail faults carry no OS or retired word, and the session leaves them unchanged
@answer n=5 entity=log-coverage#1.5 v=no
why: the send field list names no OS or retired word, and the session changes no send field
@answer n=6 entity=log-coverage#1.6 v=no
why: the dialog field list names no OS or retired word, and the session changes no dialog field
@answer n=7 entity=log-coverage#1.7 v=no
why: the hook-invoked and hook-decision fields and their hook_event words carry no OS or retired word, and the session changes none of them
@answer n=8 entity=log-coverage#1.8 v=no
why: the panic fields name only a workspace-relative or dependency location, which the session does not change
@answer n=9 entity=log-coverage#1.9 v=no
why: the release-from-driver fields name no OS or retired word, and the session changes none of them
@answer n=10 entity=log-coverage#1.10 v=no
why: the state-recovered detail and file words and the scanned pattern carry no OS or retired word, and the session changes none of them
@answer n=11 entity=log-coverage#1.11 v=no
why: the parse-rejected parser and detail words name no OS or retired word, and the session changes none of them
@answer n=12 entity=log-coverage#2.1 v=yes mode=contradict
why: the session removes batch-script-child from the process-exit detail enum, which drops from 20 to 19 values, so the exit-1 list and the Windows Ctrl-C wording of the interrupted stop are stale
@answer n=13 entity=log-coverage#2.2 v=yes mode=contradict
why: the session retires the macOS euid-only carve-out and the Windows check, so the Windows and macOS euid-mismatch-alone parenthetical on server-verify-failed is no longer true
@answer n=14 entity=log-coverage#2.3 v=no
why: exit 20 holds the single word wrapper-fault, which the session does not change
@answer n=15 entity=log-coverage#2.4 v=yes mode=contradict
why: the session retires the macOS euid-only carve-out and the Windows check, so the Windows and macOS euid-mismatch-alone parenthetical on hook-decision server-verify-failed is no longer true
@answer n=16 entity=log-coverage#2.5 v=no
why: the parse-rejected details per parser name no Windows-only arm, and the strict-modes-failed word on ledger-stamps stays on the Unix check
@answer n=17 entity=log-coverage#2.6 v=no
why: the mcp-stdin oversize parse-rejected line is a log detail the session does not change
@answer n=18 entity=log-coverage#2.7 v=no
why: the refusal details are the arch RefusalReason words, which the session does not change
@answer n=19 entity=log-coverage#2.8 v=no
why: the filesystem refusal sites and the socket-dir-refused detail name no DACL or Windows arm, and the Unix checks they list stay unchanged
@answer n=20 entity=log-coverage#3.1 v=no
why: the child and shell spawn subject enum and the verify and agents-probe pairs name no OS, ConPTY or kill-fallback tie, and the session changes none of them
