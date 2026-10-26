# Classified — security-plan · authentication-authorization.b1

## Answers
authentication-authorization#1.1 · no · the Cargo.toml change is only a Windows dev-dependency table, so no account, password or hashing surface moved
authentication-authorization#1.2 · no · the session adds witness tests for the existing OS check, leaving the all-or-nothing capability rule and the OS-user principal as they were
authentication-authorization#1.3 · yes · add-to · F1 and F2 add CI witnesses on windows-2025 for remote-client rejection and the DACL's anonymous-open denial, the controls themselves unchanged
authentication-authorization#1.4 · no · no Unix listener or socket-directory code changed, and the rule lives in viola-channel's Unix listener, which the session does not reach
authentication-authorization#1.5 · no · the euid peer check lives in viola-channel's Unix listener, which the session leaves untouched
authentication-authorization#1.6 · no · the report shows the client-side verification and its dated gaps still stand, and no product file (src/cmd/run.rs included) changed
authentication-authorization#1.7 · no · the report shows no product file or refusal word changed, so the -32602 release-from-driver refusal and its place before budget stand
authentication-authorization#1.8 · no · the viola-state home strict-modes check, replace_private and the 0700 / 0600 modes are unchanged, no product code in the reached root-bin files changed, and the new Windows cases test pipes, not an instance tree's DACL
