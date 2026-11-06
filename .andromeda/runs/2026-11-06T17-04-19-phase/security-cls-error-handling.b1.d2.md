@answer n=1 entity=error-handling#1.1 v=no
why: hook's silent exit-0 contract; hook is named by no scope item and no hook code changes (not_needed 2)
@answer n=2 entity=error-handling#1.2 v=no
why: hook server-check failure path; hook is not a route the live driver takes and no hook code changes
@answer n=3 entity=error-handling#1.3 v=no
why: hook socket-directory refusal; hook is not named in the scope and no hook code changes
@answer n=4 entity=error-handling#1.4 v=no
why: hook replayed-snapshot connect failure; hook is not named in the scope and no hook code changes
@answer n=5 entity=error-handling#1.5 v=no
why: hook statusline without capture; statusline is not named in the scope and no hook code changes
@answer n=6 entity=error-handling#1.6 v=yes
why: the live driver over MCP passes the named server-verification check, and this rule fixes the result it reads when that check refuses
@answer n=7 entity=error-handling#1.7 v=yes
why: the MCP frame tool's strict-modes and endpoint-directory prechecks run before any frame on the MCP route the live driver takes
@answer n=8 entity=error-handling#1.8 v=no
why: wrapper link and unlink failure replies; the chunk re-words those dispositions without building or changing the methods
@answer n=9 entity=error-handling#1.9 v=no
why: sanitised_fault and its two callers; neither reply_of nor of_reply is named and no fault reader changes
@answer n=10 entity=error-handling#1.10 v=no
why: -32602 data retention in sanitised_fault; no fault reader changes and no -32602 path is named
@answer n=11 entity=error-handling#1.11 v=no
why: fault codes outside the closed five; no fault reader changes and no wrapper fault path is named
@answer n=12 entity=error-handling#1.12 v=no
why: is_code_word refusal reading in reply_of and of_reply; no reply shape changes (not_needed 4)
@answer n=13 entity=error-handling#1.13 v=no
why: closed-list code-word exception; no reply shape changes (not_needed 4)
@answer n=14 entity=error-handling#1.14 v=no
why: ProtocolError cause words and channel-response detail; no channel code is reached and no reply shape changes (not_needed 4)
@answer n=15 entity=error-handling#1.15 v=yes
why: the accept-loop peer admission is the named peer check the live driver passes before any frame
@answer n=16 entity=error-handling#1.16 v=yes
why: no counted panic or failpoint may drive a -32603 in a scanned home, which the G2 zero-panics gate the chunk must bring green counts
@answer n=17 entity=error-handling#1.17 v=yes
why: CLI send, wait, last, answer, pause and release run the strict-modes precheck before any frame on the CLI route the live driver takes, and release is named in the scope
@answer n=18 entity=error-handling#1.18 v=no
why: the exit-21 closed detail set reads only on a refused call, which a healthy live run does not make and the scope does not name
@answer n=19 entity=error-handling#1.19 v=yes
why: a refused peer euid reads server-verify-failed in three clients, the word the live driver's named peer and server-verification checks return
@answer n=20 entity=error-handling#1.20 v=no
why: refusal-document writers for exit 21 and MCP; no writer is reached and the board's read rule is row-level, not a refusal document
@answer n=21 entity=error-handling#1.21 v=no
why: socket-directory refusal on CLI frame verbs; the socket-directory wording founder item is a matrix record, not this refusal path
@answer n=22 entity=error-handling#1.22 v=no
why: unwrapped-name and endpoint-lost stderr hints; no hint text is reached
@answer n=23 entity=error-handling#1.23 v=no standing=S2
why: S2 convention, log lines carry no absolute path, pid or uid, which the scope names as the NEVER-log floor for recorded readings
@answer n=24 entity=error-handling#1.24 v=no
why: exit-2 UNUSABLE lines of answer and send; the scope does not name an exit-2 path for the live run
@answer n=25 entity=error-handling#1.25 v=yes
why: viola run's home strict-modes precheck before the wrapper log opens binds the live viola run the scope names, whose home must pass it
