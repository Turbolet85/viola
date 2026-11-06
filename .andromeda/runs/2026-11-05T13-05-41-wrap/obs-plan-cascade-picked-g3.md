@account n=1 entity=span-trace-coverage#3.1 v=proposed
why: it states the Linux-and-Windows pid comparison and the macOS euid-only carve-out, both retired with the Windows and macOS code
@account n=2 entity=span-trace-coverage#3.2 v=proposed
why: it names PEER_EUID_REPORTED and a Windows no-euid control, both gone (peer_euid_admitted is observed == Some(own))
@account n=3 entity=span-trace-coverage#3.4 v=proposed
why: it places the span recorder out of a macOS-gated module, and no target_os macos gate stands after the chunk
@account n=4 entity=span-trace-coverage#3.8 v=proposed
why: the hook server check repeats the Linux-and-Windows comparison and the retired macOS carve-out
@account n=5 entity=span-trace-coverage#3.11 v=proposed
why: the MCP server check repeats the Linux-and-Windows comparison and the retired macOS carve-out
@account n=6 entity=span-trace-coverage#4.1 v=proposed
why: its span chain carries run.conpty_sideload, retired with the ConPTY sideload
@account n=7 entity=span-trace-coverage#4.2 v=proposed
why: it carries the batch-script-child refusal, the run.conpty_sideload row, the named-pipe endpoint kind and three pty_backend words, all retired
@account n=8 entity=span-trace-coverage#4.3 v=proposed
why: its detail set holds batch-script-child and it describes sideload_fallback in process-start and the schema, both removed from the schema
@account n=9 entity=span-trace-coverage#4.4 v=not_touched
why: the report keeps child exit detected on the handle wait and changes no exit_source word, so the cleanup rule still holds
@account n=10 entity=span-trace-coverage#5.1 v=not_touched
why: the report keeps pin_bytes and the pinned copy re-hash and removes neither place_private_new nor PinError::HashMismatch
@account n=11 entity=span-trace-coverage#6.1 v=proposed
why: the send server-check parenthetical repeats the Linux-and-Windows comparison and the retired macOS carve-out
@account n=12 entity=span-trace-coverage#6.3 v=proposed
why: the server-verification refusal item names Linux, Windows and macOS and the macOS euid-only reading
@account n=13 entity=span-trace-coverage#6.4 v=proposed
why: the exit-21 server-verify-failed clause names Linux, Windows and macOS and the macOS euid-only reading
@account n=14 entity=span-trace-coverage#7.1 v=proposed
why: the wait/last server-check parenthetical repeats the Linux-and-Windows comparison and the retired macOS carve-out
@account n=15 entity=span-trace-coverage#7.3 v=proposed
why: the client process-exit item names Linux, Windows and macOS and the macOS euid-only reading
@account n=16 entity=span-trace-coverage#7.4 v=not_touched
why: it names no OS arm and no removed symbol, and the refusal order it states stands on the changed client files
@account n=17 entity=span-trace-coverage#8.1 v=proposed
why: the answer server-check parenthetical repeats the Linux-and-Windows comparison and the retired macOS carve-out
@account n=18 entity=span-trace-coverage#9.1 v=proposed
why: the pause/release server-check parenthetical repeats the Linux-and-Windows comparison and the retired macOS carve-out
@account n=19 entity=span-trace-coverage#10.3 v=proposed
why: it ties the statusline-shell pair to shell_argv being None on Windows, and shell_argv now returns [&str; 3]
@account n=20 entity=span-trace-coverage#11.1 v=proposed
why: it states the Windows case-insensitive strip, the HKCU/HKLM persistent set and the registry reader, all removed
@account n=21 entity=span-trace-coverage#15.1 v=not_touched
why: the report changes no append or heal path, so the torn-append line still holds
@account n=22 entity=span-trace-coverage#17.1 v=proposed
why: it carries the Windows console-control stop, the WRAPPED_HERE relied rule with 21 on Windows, and Unix-only and Windows-close qualifiers, all retired
@account n=23 entity=metric-coverage#3.1 v=proposed
why: it names Linux, Windows and macOS and the macOS euid-only reading for the server-verification refusal
@account n=24 entity=metric-coverage#3.2 v=not_touched
why: the instance strict-modes refusal it counts names no OS arm and the report leaves it unchanged
