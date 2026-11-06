@answer n=1 entity=error-handling#1.1 v=no
why: the session only removed Windows-only path code from src/cmd/hook.rs and added no product surface, so the stdin cap, shape checks, fail-open words and empty stderr still hold
@answer n=2 entity=error-handling#1.2 v=no
why: the server-verify path and its hook-decision line are unchanged by the Windows-only removals
@answer n=3 entity=error-handling#1.3 v=no
why: the refused socket-directory path and its socket-dir-refused word are unchanged; the session added and renamed no refusal
@answer n=4 entity=error-handling#1.4 v=no
why: read_snapshot_or_replay and the channel-unreachable fail-open are unchanged; no path or word the session moved is part of this rule
@answer n=5 entity=error-handling#1.5 v=no
why: hook statusline without --capture reads no Windows path and the session added no statusline surface, so it still prints nothing
@answer n=6 entity=error-handling#1.6 v=no
why: the MCP frame tool's server-verify result and its instance-unreachable text are unchanged; the session added no tool or word
@answer n=7 entity=error-handling#1.7 v=no
why: the check_instance order and the socket-dir-refused result of the MCP frame tool are unchanged; the session added no surface
@answer n=8 entity=error-handling#1.8 v=no
why: the link and unlink -32603 fixed message and the exit 20 mapping are unchanged; the session retired no wrapper reply and added no verb
@answer n=9 entity=error-handling#1.9 v=no
why: sanitised_fault and its two callers in src/cmd/client.rs and crates/viola-mcp/src/result.rs keep their reading; the session removed only Windows path code
@answer n=10 entity=error-handling#1.10 v=no
why: the -32602 keep-data rule for the two bodies is unchanged; the session moved no fault body
@answer n=11 entity=error-handling#1.11 v=no
why: the wrapper fault fallback for an unknown code and its human line are unchanged; the session added no fault word
@answer n=12 entity=error-handling#1.12 v=no
why: is_code_word and the unknown and null fallbacks in reply_of and of_reply are unchanged by the session
@answer n=13 entity=error-handling#1.13 v=no
why: the by-design exception for a code word outside the closed list is unchanged; the session added no code word or reason
@answer n=14 entity=error-handling#1.14 v=no
why: the three -32602 causes and the closed channel-response enum value are unchanged; the session removed no cause word
@answer n=15 entity=error-handling#1.15 v=no
why: the session's listed changes do not touch accept_loop, admitted or peer_euid_of, so the silent drop before any frame stands
@answer n=16 entity=error-handling#1.16 v=no
why: the session added no failpoint and no -32603 driver in a scanned home; the unit tests named here are unchanged
@answer n=17 entity=error-handling#1.17 v=no
why: client::refuse_untrusted and the send and wait exit 21 path keep their order and output; the session removed only Windows-only code from those files
@answer n=18 entity=error-handling#1.18 v=no
why: the closed six-cause detail set of exit 21 is unchanged; the session added and removed no detail word
@answer n=19 entity=error-handling#1.19 v=no
why: CallError::ServerUnverified, NotSent::ServerUnverified and UnreachableCause::ServerVerify keep their names and the server-verify-failed word; the session renamed no type
@answer n=20 entity=error-handling#1.20 v=no
why: client::unreachable_document and list.rs refused_document remain the writers of the exit-21 document; the session changed neither writer's output
@answer n=21 entity=error-handling#1.21 v=no
why: the SocketDirRefused mapping to exit 21 and its one WARN process-exit line are unchanged; the session did not alter the socket-directory refusal path
@answer n=22 entity=error-handling#1.22 v=no
why: the unwrapped-name and endpoint-lost hints and their plain ASCII stderr lines are unchanged; the retired Git Bash warning is not one of them
@answer n=23 entity=error-handling#1.23 v=no
why: the socket-directory refusal lines hold only the parsed ViolaName, no path, pid or uid, and the session's Windows path removals leave them as written
@answer n=24 entity=error-handling#1.24 v=no
why: the exit-2 UNUSABLE_RESPONSE and UNUSABLE_TEXT stderr lines are not the retired Git Bash rewritten-path warning, so they still print on exit 2
@answer n=25 entity=error-handling#1.25 v=no
why: check_home_tree, update_stamps and StampsError::StrictModes keep their refusal exits and words; the session removed Windows-only home handling and changed no refusal
