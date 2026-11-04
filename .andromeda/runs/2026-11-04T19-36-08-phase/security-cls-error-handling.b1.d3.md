@answer n=1 entity=error-handling#1.1 v=yes
why: hook exits 0 with no body and no stderr on every OS, a failing security check included (item 11), and it names the Unix socket-dir and peer euid checks
@answer n=2 entity=error-handling#1.2 v=yes
why: Unix server-verify-failed on hook carries the closed word, under the client verification and macOS euid-only carve-out (items 5, 6)
@answer n=3 entity=error-handling#1.3 v=yes
why: socket-dir refusal on hook rests on the per-user 0700 socket directory and the client socket-dir check before connect (items 3, 5)
@answer n=4 entity=error-handling#1.4 v=yes
why: the Linux peer pid and start-time verification reads the snapshot pid and started_at fields that a replayed snapshot lacks (item 5)
@answer n=5 entity=error-handling#1.5 v=yes
why: the statusline shell-out (item 8) runs the recorded statusline_command, and a replayed snapshot must still run nothing
@answer n=6 entity=error-handling#1.6 v=yes
why: the MCP frame tool's server-verify-failed under the macOS whole-check carve-out and the Linux peer-pid check, with instance-unreachable details (items 5, 6, 9)
@answer n=7 entity=error-handling#1.7 v=yes
why: the home strict-modes check runs before any frame and is refused before the first read on Unix (item 7), with the endpoint socket-dir check (item 5)
@answer n=8 entity=error-handling#1.8 v=yes
why: the -32603 fixed message with null data is a channel error.data closed code that the diagnostics schema work must keep (item 9)
@answer n=9 entity=error-handling#1.9 v=yes
why: sanitised_fault is the fault reader that keeps channel error.data to closed codes with fixed text (item 9)
@answer n=10 entity=error-handling#1.10 v=yes
why: the rule that -32602 keeps data in two bodies only is the closed-data rule for channel error.data (item 9)
@answer n=11 entity=error-handling#1.11 v=yes
why: a fault code outside the five reads as the wrapper fault output word on --json and MCP, a closed word every OS must share (item 9)
@answer n=12 entity=error-handling#1.12 v=yes
why: is_code_word closes the refusal and detail words on reply_of and of_reply, the closed words item 9 requires on every OS
@answer n=13 entity=error-handling#1.13 v=yes
why: the by-design exception keeps an out-of-list code word visible in the document and exit 14, which the closed-code work must not sanitise away (items 9, 11)
@answer n=14 entity=error-handling#1.14 v=yes
why: channel-response detail is the closed enum in schemas/diag-line.v1.json, which the chunk brings into line on both Unix OSes (item 9)
@answer n=15 entity=error-handling#1.15 v=yes
why: the accept-loop euid drop before any frame, with EOF and no line, is the peer euid check the chunk runs on both Unix OSes (item 4)
@answer n=16 entity=error-handling#1.16 v=yes
why: the chunk's new counted FAKE_AGENT_HOOK_PANIC seam must not drive a -32603 in a scanned home (item 14, G2 exemption)
@answer n=17 entity=error-handling#1.17 v=yes
why: the CLI frame verbs refuse on the strict-modes check before any frame, with exit 21 and the --json document (items 7, 11)
@answer n=18 entity=error-handling#1.18 v=yes
why: exit 21's closed set of six instance-unreachable details is named in item 9 and reaches the --json and MCP documents
@answer n=19 entity=error-handling#1.19 v=yes
why: the server-verify-failed word for a refused peer euid on the three clients is the Unix server-verify outcome (items 4, 5, 9)
@answer n=20 entity=error-handling#1.20 v=yes
why: client::unreachable_document is the single writer of the exit-21 --json document that the schema work brings into line (item 9)
@answer n=21 entity=error-handling#1.21 v=yes
why: socket-dir-refused on a CLI frame verb, exit 21 with no channel-request, follows the socket directory check before connect (items 3, 5)
@answer n=22 entity=error-handling#1.22 v=no
why: the human stderr hints for unwrapped-name and endpoint-lost are not among the external-error surfaces item 9 names, and the chunk does not change them
@answer n=23 entity=error-handling#1.23 v=yes
why: socket-directory refusal lines must hold no path, pid or uid, which the Unix socket-dir check on both OSes now exercises (items 3, 5, 9)
@answer n=24 entity=error-handling#1.24 v=yes
why: the exit-2 paths write no --json document, an exception the chunk's --json schema work must keep (items 9, 11)
@answer n=25 entity=error-handling#1.25 v=yes
why: the home strict-modes refusal before the log opens, including an unstattable home on Unix, is the home check item 7 strict-modes on Unix (exit 1)
