@answer n=1 entity=span-trace-coverage#1.1 v=no
why: CLIENT span-kind census; the chunk declares no new span and renames none (not-needed item 9), and pty.spawn stays as it is
@answer n=2 entity=span-trace-coverage#1.2 v=no
why: SERVER span-kind census; the chunk touches no server span
@answer n=3 entity=span-trace-coverage#1.3 v=no
why: INTERNAL span-kind census; the chunk declares no span and the paste rule is about bytes, not a span name
@answer n=4 entity=span-trace-coverage#2.1 v=yes
why: the live dialog read runs the answer verb, whose dispatch span and cli-<name>.ndjson process lines are part of its evidence (items 10 and 12)
@answer n=5 entity=span-trace-coverage#2.2 v=yes
why: the portable-pty target OFF with failures captured at the seam is the level-table rule the portable-pty bump must keep (item 5)
@answer n=6 entity=span-trace-coverage#2.3 v=yes
why: the hook-invoked and hook-decision lines on the dialog path are the evidence the live dialog hold read rests on (item 12)
@answer n=7 entity=span-trace-coverage#2.4 v=yes
why: the never-params-or-result-bodies rule is what keeps question, answer and tool input unlogged, which the dialog read's evidence states (item 12)
@answer n=8 entity=span-trace-coverage#2.5 v=no
why: the wrapper's accept-thread filter is not reached by the chunk and its no-line rule is not a scope item
@answer n=9 entity=span-trace-coverage#2.6 v=no
why: rmcp's OFF target is not named by the scope (only portable_pty's is), and no chunk code touches MCP tracing
@answer n=10 entity=span-trace-coverage#3.1 v=no
why: the cli server-verify check sits in cmd::client::connect, which the chunk does not reach, and its Linux-only peer check is unchanged
@answer n=11 entity=span-trace-coverage#3.2 v=no
why: the client euid check in the client seams; none of those files are reached and no scope item names it
@answer n=12 entity=span-trace-coverage#3.3 v=no
why: the per-verb captured-span refusal tests for the euid refusal are not the socket-directory wording or another scope-named behaviour
@answer n=13 entity=span-trace-coverage#3.4 v=yes
why: it names the two captured-span cases that witness the socket-directory refusal the chunk ratifies or re-words (item 14)
@answer n=14 entity=span-trace-coverage#3.5 v=no
why: the instance check ordering for the six frame verbs is not named by any scope item and no reached file holds it
@answer n=15 entity=span-trace-coverage#3.6 v=yes
why: the cli socket-dir-refused process-exit detail code and its WARN line are the value item 14 ratifies or re-words
@answer n=16 entity=span-trace-coverage#3.7 v=yes
why: the human socket-directory pair's wording (plain ASCII, hint last, ViolaName only) is the wording item 14 ratifies or re-words
@answer n=17 entity=span-trace-coverage#3.8 v=no
why: the hook's server-verify refusal and fail-open ordering are not named by the scope and the chunk does not move the euid or verify path
@answer n=18 entity=span-trace-coverage#3.9 v=yes
why: it names the socket-dir-refused hook-decision detail code on the dialog path, the code item 14 carries
@answer n=19 entity=span-trace-coverage#3.10 v=no
why: the channel-response detail word for -32602 is a protocol fault word that no scope item names
@answer n=20 entity=span-trace-coverage#3.11 v=yes
why: the MCP frame tools return the socket-dir-refused detail code, the value item 14 ratifies or re-words
@answer n=21 entity=span-trace-coverage#3.12 v=no
why: the clients' reply screening is in no scope item and no reached file
@answer n=22 entity=span-trace-coverage#4.1 v=yes
why: the run start sequence carries the socket_dir_ready refusal (socket-dir-refused) and the pty.spawn span the portable-pty bump reaches (items 6 and 14)
@answer n=23 entity=span-trace-coverage#4.2 v=yes
why: the pty.spawn pty_backend attribute from viola_pty::pty_backend() is the value item 1 cuts to openpty alone
@answer n=24 entity=span-trace-coverage#4.3 v=yes
why: the process-start claude-child fields with pty_backend and the socket-dir-refused detail are what the live run and the process-start row rest on (items 1, 6 and 14)
@answer n=25 entity=span-trace-coverage#4.4 v=yes
why: the handle-wait claude-child process-exit with exit_source kill-fallback, never reader EOF, is the exit path item 6 requires
