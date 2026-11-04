@answer n=1 entity=error-handling#1.1 v=yes
why: the hook contract (exit 0, empty streams, fail-open) and the strict-modes check before the config read bind the Unix hook path and the macOS carve-out (items 6, 7, 11)
@answer n=2 entity=error-handling#1.2 v=yes
why: a failed server check writes no frame, exits 0 and reads server-verify-failed on the hook path, which the client verification of item 5 and the hook contract of item 11 must keep
@answer n=3 entity=error-handling#1.3 v=yes
why: a refused socket directory on the hook path writes no frame, exits 0 and reads socket-dir-refused, the socket-directory rule of items 3 and 5
@answer n=4 entity=error-handling#1.4 v=yes
why: a replayed or absent snapshot fails open as channel-unreachable with no frame, a fail-open hook case the Unix suite must hold under item 11
@answer n=5 entity=error-handling#1.5 v=yes
why: statusline without capture runs nothing over a replayed snapshot, a limit on the one Unix shell-out that item 8 adds
@answer n=6 entity=error-handling#1.6 v=yes
why: MCP frame tools send no frame on a refused peer euid, and on macOS the whole check is euid alone, the carve-out of item 6 and the client check of item 5
@answer n=7 entity=error-handling#1.7 v=yes
why: MCP runs the instance strict-modes check and the socket-directory check before any frame, the home and socket-dir rules of items 3, 5 and 7
@answer n=8 entity=error-handling#1.8 v=yes
why: the link and unlink fault mapping (-32603 internal error, data null, exit 20) is a closed-word external error rule that item 9 covers for every --json document
@answer n=9 entity=error-handling#1.9 v=yes
why: sanitised_fault fixes the message and null data for the five known codes, the external-error sanitising item 9 requires on every OS
@answer n=10 entity=error-handling#1.10 v=yes
why: -32602 keeps data in only two bodies, the closed external codes item 9 names for CLI --json and channel error.data
@answer n=11 entity=error-handling#1.11 v=yes
why: an unknown fault code reads the fixed wrapper fault word, a closed word every --json document must carry on every OS under item 9
@answer n=12 entity=error-handling#1.12 v=yes
why: is_code_word bounds the refusal and detail words, the closed-word rule item 9 keeps identical across the OSes
@answer n=13 entity=error-handling#1.13 v=yes
why: a code word outside the closed list stays visible with exit 14, an exception the closed-codes rule of item 9 must hold on Unix
@answer n=14 entity=error-handling#1.14 v=yes
why: ProtocolError cause words are the closed channel-response detail enum in diag-line.v1.json, which item 9 requires to match on every OS
@answer n=15 entity=error-handling#1.15 v=yes
why: the accept loop drops a peer whose euid is refused before any frame is read, the server-side peer check of item 4
@answer n=16 entity=error-handling#1.16 v=yes
why: the Unix suite's new counted panic seam (FAKE_AGENT_HOOK_PANIC, item 14) must not drive a -32603 in a scanned home
@answer n=17 entity=error-handling#1.17 v=yes
why: CLI frame verbs run the instance strict-modes check before any frame, the strict-modes-before-first-read rule of item 7
@answer n=18 entity=error-handling#1.18 v=yes
why: exit 21's six-cause detail set is the closed instance-unreachable set item 9 names, including server-verify-failed and socket-dir-refused of item 5
@answer n=19 entity=error-handling#1.19 v=yes
why: a refused peer euid reads server-verify-failed in each client, the word items 4 and 5 use on Unix
@answer n=20 entity=error-handling#1.20 v=yes
why: each cause word is written only by the two clients' writers, which the closed-word diagnostics of item 9 must keep on every OS
@answer n=21 entity=error-handling#1.21 v=yes
why: a refused socket directory on a CLI frame verb is exit 21 with socket-dir-refused and no frame, the client directory check of item 5
@answer n=22 entity=error-handling#1.22 v=no
why: the unwrapped-name and endpoint-lost stderr hints are human text that no item of the chunk names or changes
@answer n=23 entity=error-handling#1.23 v=yes
why: the socket-directory refusal lines hold no path, pid or uid, the NEVER-log and no-absolute-path floor that items 3, 9 and 10 apply on every OS
@answer n=24 entity=error-handling#1.24 v=yes
why: the exit-2 paths of answer and send write no document, a --json exception that item 9 and the exit set of item 11 must hold on Unix
@answer n=25 entity=error-handling#1.25 v=yes
why: run and revive refuse on the home strict-modes check before the log opens, the Unix home-under-a-regular-file case of item 7
