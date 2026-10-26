# Classified — security-plan · authentication-authorization.b1

## Answers
authentication-authorization#1.1 · no · the lock delta is tokio's version and checksum alone, so no password-hashing crate arrived and the N/A claim holds
authentication-authorization#1.2 · no · the session changed no source and added no IPC method, so the all-or-nothing OS-user capability model stands
authentication-authorization#1.3 · no · the lock delta is tokio's version and checksum alone, so interprocess 2.4.4 and windows-sys 0.61.2 stand and the Windows SDDL listener is untouched
authentication-authorization#1.4 · no · the lock delta is tokio's version and checksum alone, so interprocess 2.4.4 stands and the Unix socket directory and its mode checks are untouched
authentication-authorization#1.5 · no · the session changed no viola-channel source and no security-relevant tokio API use, so the per-accept peer euid check stands
authentication-authorization#1.6 · no · the lock delta is tokio's version and checksum alone, so windows-sys 0.61.2 and sysinfo 0.39.6 stand and client server verification is untouched
authentication-authorization#1.7 · no · the session changed no source or release dispatch, so the driver-originated release refusal stands
authentication-authorization#1.8 · no · the lock delta is tokio's version and checksum alone, so tempfile 3.27.0 and windows-sys 0.61.2 stand and no home or file-mode code changed
authentication-authorization#2.1 · no · the lock delta is tokio's version and checksum alone, so windows-sys 0.61.2 stands and the remote-client witness test is unchanged
authentication-authorization#2.2 · no · the lock delta is tokio's version and checksum alone, so windows-sys 0.61.2 stands and the DACL witness test is unchanged
