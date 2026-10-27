# Classified — security-plan · authentication-authorization.b1

## Answers
authentication-authorization#1.1 · no · the session adds no account or password surface, so the N/A stands
authentication-authorization#1.2 · no · the session changes no served method or params check, and spawn_os_error is a test-only fake-agent receipt field
authentication-authorization#1.3 · no · the Windows listener DACL lives in viola-channel, which the session does not reach
authentication-authorization#1.4 · no · the Unix socket directory and its 0600 chmod live in viola-channel, which the session does not reach
authentication-authorization#1.5 · no · the Unix peer euid check lives in viola-channel, which the session does not reach
authentication-authorization#1.6 · no · the client server verification lives in viola-channel and viola-state liveness, neither reached, and the strict-modes check it runs first keeps its product verdict
authentication-authorization#1.7 · no · the release from-check lives in the bin channel dispatch and viola-core params, neither reached by the session
authentication-authorization#1.8 · yes · add-to · F6: the plain-home refusal is checked by the write right whichever SID holds it, so its cfg(windows) witness names no fixed SID, and the pinned copy's first write is now place_private_new, no longer a replace
authentication-authorization#1.9 · no · check_home_tree and checked_exe keep their verdicts, since strict.rs's product code is unchanged and the pin change in pin.rs is not on the start's tree check
authentication-authorization#2.1 · no · the remote-client witness lives in tests/security_negatives_channel.rs, which the session does not reach
authentication-authorization#2.2 · no · the anonymous-open witness lives in tests/security_negatives_channel.rs, which the session does not reach
authentication-authorization#3.1 · no · the observed server pid is read in viola-channel client code, which the session does not reach
authentication-authorization#3.2 · no · the ServerVerdict and verify_server live in viola-state liveness, which the session does not reach
authentication-authorization#3.3 · no · link and unlink run check_instance first, and strict.rs keeps its product code, so the order holds
authentication-authorization#3.4 · no · client::refuse_untrusted still runs check_instance first, since strict.rs's product code is unchanged
authentication-authorization#3.5 · no · the exiting-wrapper pipe case lives in hook.rs and the sysinfo start-time read, neither changed by the session
authentication-authorization#3.6 · no · the macOS server-identity cases live in tests/security_negatives_server_identity.rs, which the session does not reach
authentication-authorization#3.7 · no · the link and unlink tests run through check_instance, whose product code is unchanged, and their files are not reached
