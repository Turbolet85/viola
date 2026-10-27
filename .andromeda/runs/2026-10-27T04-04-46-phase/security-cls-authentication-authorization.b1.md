# Classified — security · authentication-authorization.b1

## Answers
authentication-authorization#1.1 · no · N/A, viola has no accounts or passwords (Personal v1), so the chunk hashes no credential
authentication-authorization#1.2 · yes · each served method and the hook.event notification sit behind their own params check, the wrapper's -32602 check before validate_paste_text that the chunk's channel rule requires of every method
authentication-authorization#1.3 · no · the Windows listener's protected DACL and accept_remote(false) are the Not needed set owned by Unix endpoint and home hardening; the chunk bounds frame input, not pipe access
authentication-authorization#1.4 · no · the Unix 0700 socket directory and the socket chmod are the Not needed set owned by Unix endpoint and home hardening
authentication-authorization#1.5 · no · the peer_creds euid check on accept is the Not needed set owned by Unix endpoint and home hardening
authentication-authorization#1.6 · yes · the before-first-frame server check and the strict-modes-before-read order bind the hook stdin read and the CLI send and answer text reads, which must sit after the instance check and before any frame; the SQOS open and the macOS gap are Not needed
authentication-authorization#1.7 · yes · the release params are a closed params struct in viola-core, so a from of another type is -32602 with data null and that refusal stays before budget acts; the string-from release-from-driver refusal is decision policy (Not needed), kept on the same params path
authentication-authorization#1.8 · yes · every read under the home (snapshot, events, stamps, budget, config, statusline-source.json) comes after that entry point's strict-modes check, and a refused instance reads no stdin, config or role file
authentication-authorization#1.9 · yes · check_home_tree with the pinned key runs before the bin/<key>/ and plugin/<key>/ join, so the closed key shape must sit before that join and the tree check before any bounded read
authentication-authorization#2.1 · no · the Windows listener's reject-remote witness covers accept_remote(false), in the Not needed listener set; the chunk adds no test on the pipe's remote-client bit
authentication-authorization#2.2 · no · the anonymous-open denial witnesses the Windows protected DACL, in the Not needed listener set; the chunk bounds frame input, not who may open the pipe
authentication-authorization#3.1 · no · the peer-pid read is an OS identity input the chunk neither changes nor reads, and the macOS no-peer-pid constant is Not needed
authentication-authorization#3.2 · yes · a replayed snapshot carries no endpoint, pid or started_at, and only a Matches verdict lets a frame be written, so the chunk's bounded snapshot and events replay must keep both
authentication-authorization#3.3 · yes · link and unlink run check_instance on instances/<driven> before client::start, so the ViolaName::try_new check on the driver and driven names must come before that join
authentication-authorization#3.4 · yes · the CLI frame verbs' exit-13 paste refusals come first and refuse_untrusted precedes the liveness and server checks, which binds the send and answer text reads (send's text read must move after refuse_untrusted)
authentication-authorization#3.5 · no · a hook passing the check at an exiting wrapper's own Windows pipe is a CARRY owed to Windows-only live measurements, which the Not needed list names
authentication-authorization#3.6 · no · the macOS server-verification cases are a CARRY owed to Unix endpoint and home hardening, which the Not needed list names
authentication-authorization#3.7 · yes · channel_link_names_are_validated_by_the_wrapper witnesses the chunk's ViolaName::try_new on the channel link and unlink params (bad, foreign and self-pair names read -32602 with no line)
