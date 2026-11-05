@answer n=1 entity=error-handling#1.1 v=yes
why: hook exits 0 with no body and no stderr on a failed security check, and its Unix socket-dir and peer euid checks are kept by task items 19 and 2

@answer n=2 entity=error-handling#1.2 v=yes
why: the Linux server-verify-failed refusal of a hook event with exit 0, named by task items 2 and 19

@answer n=3 entity=error-handling#1.3 v=yes
why: the socket-dir-refused refusal of a hook event, named by task item 2 as a Unix check the chunk keeps whole

@answer n=4 entity=error-handling#1.4 v=yes
why: the hook fails open at exit 0 over a replayed snapshot lacking the pid and start time that the Linux server check reads (task items 19 and 2)

@answer n=5 entity=error-handling#1.5 v=yes
why: the statusline shell-out finds no command over a replayed snapshot, the cfg(unix) arm made unconditional by task item 5

@answer n=6 entity=error-handling#1.6 v=yes
why: the mcp frame tool's server-verify-failed refusal is the Linux client verification named by task item 2 (the macOS carve-out leaves by item 3)

@answer n=7 entity=error-handling#1.7 v=yes
why: the mcp frame tool's strict-modes check and endpoint socket-dir check, both named by task items 4 and 2

@answer n=8 entity=error-handling#1.8 v=yes
why: the typed exit codes 1 and 20 of link and unlink, which task item 19 keeps unchanged

@answer n=9 entity=error-handling#1.9 v=no
why: sanitised_fault is an untouched wire-fault reader the chunk neither edits nor calls, and the scope names no fault code or body it carries; a re-check every change owes

@answer n=10 entity=error-handling#1.10 v=no
why: the -32602 data bodies of the same untouched fault reader are not named by the scope; a re-check every change owes

@answer n=11 entity=error-handling#1.11 v=no
why: the fixed wrapper fault fallback for an out-of-list fault code sits in untouched reader code the scope does not name; a re-check every change owes

@answer n=12 entity=error-handling#1.12 v=no
why: is_code_word is an untouched closed-word check and the scope names no word or value it admits; a re-check every change owes

@answer n=13 entity=error-handling#1.13 v=yes
why: exit 14 for an out-of-list code word, within the typed exit range 10 to 14 that task item 19 keeps

@answer n=14 entity=error-handling#1.14 v=no
why: ProtocolError::cause and the channel-response detail enum sit on untouched wire code and the scope names no detail it carries; a re-check every change owes

@answer n=15 entity=error-handling#1.15 v=yes
why: the accept loop's server-side peer euid refusal is the euid drop that task item 2 says the Unix arms keep whole

@answer n=16 entity=error-handling#1.16 v=no
why: the scanned-home absence of a -32603 drives no chunk test or seam and the scope names no internal-error path; a re-check every change owes

@answer n=17 entity=error-handling#1.17 v=yes
why: the CLI strict-modes check before any frame, exit 21 with no line, is the Unix check that task item 4 must keep refusing exactly as before

@answer n=18 entity=error-handling#1.18 v=yes
why: the closed exit-21 cause set, including server-verify-failed and socket-dir-refused, is named by task items 19, 2 and 21

@answer n=19 entity=error-handling#1.19 v=yes
why: server-verify-failed is the euid refusal word the Linux client keeps (task item 2), and the macOS carve-out it loses leaves by item 3

@answer n=20 entity=error-handling#1.20 v=yes
why: the two clients write the closed cause words, including server-verify-failed and socket-dir-refused, which the scope names as refusals to keep (items 2 and 21)

@answer n=21 entity=error-handling#1.21 v=yes
why: socket-dir-refused on the exit 21 CLI frame path is the socket-dir check that task item 2 keeps on Linux

@answer n=22 entity=error-handling#1.22 v=yes
why: the exit 21 stderr hints for the closed causes unwrapped-name and endpoint-lost, which the scope keeps as they are (items 19 and 21)

@answer n=23 entity=error-handling#1.23 v=yes
why: the socket-dir refusal's lines hold no path, pid or uid, which task item 21 says removed variants must not let into an external error

@answer n=24 entity=error-handling#1.24 v=yes
why: the exit 2 paths of answer and send write the fixed unusable line, and exit 2 is one of the typed exit codes task item 19 keeps

@answer n=25 entity=error-handling#1.25 v=yes
why: the home strict-modes check run by viola run, revive and verify before any log, a Unix check task item 4 must keep refusing exactly as before
