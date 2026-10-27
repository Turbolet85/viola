# Classified — security · authentication-authorization.b1

## Answers
authentication-authorization#1.1 · no · no accounts or passwords in v1, so the chunk hashes no credential
authentication-authorization#1.2 · no · the chunk adds no served method or frame; the all-or-nothing surface is untouched
authentication-authorization#1.3 · no · the Windows pipe's SDDL and remote-client rules belong to the listener, which the chunk neither edits nor opens an endpoint for
authentication-authorization#1.4 · no · the Unix socket directory and its chmod rules belong to the listener, out of scope as «Unix endpoint and home hardening»'s
authentication-authorization#1.5 · no · the Unix accept-time peer euid check is the channel server's and is not reached
authentication-authorization#1.6 · no · client-side server verification runs before a frame; the chunk writes no frame and edits no client, connect or collision-check unit
authentication-authorization#1.7 · no · the release-from-driver refusal is channel frame validation, and no send, answer or frame path changes
authentication-authorization#1.8 · yes · the pinned viola's 0700 mode and replace_private's mode-before-bytes, sync_all and persist rule bind the absent-copy arm of pin_bytes / pin_exe and the replace helper the chunk edits; the ConPTY companion checks bind pin_companions
authentication-authorization#1.9 · yes · the home-tree check before the pin and before any log, which the chunk re-asserts for run's and revive's start and for verify, binds where the start's pin_bytes call and verify's pin_exe call sit
authentication-authorization#2.1 · no · the Windows remote-client rejection witness tests the listener's pipe, which the chunk neither edits nor opens
authentication-authorization#2.2 · no · the pipe DACL's anonymous-open witness tests the listener's descriptor, not reached
authentication-authorization#3.1 · no · the observed server pid is read in viola-channel's client, untouched and unread by the pin path
authentication-authorization#3.2 · no · the ServerVerdict and verify_server gate frame writes, and the chunk writes no frame
authentication-authorization#3.3 · no · link / unlink server verification is not reached; neither verb is edited
authentication-authorization#3.4 · no · the CLI frame verbs' exit-21 refusal on a mismatch is untouched, and the chunk writes no frame
authentication-authorization#3.5 · no · an exiting wrapper's Windows pipe is a Windows-only carry on the hook connect, which the chunk does not reach
authentication-authorization#3.6 · no · the macOS server-verification cases sit on the server check, which the chunk does not touch
authentication-authorization#3.7 · no · the link / unlink channel and CLI tests are not reached by the pin fix or its witness
