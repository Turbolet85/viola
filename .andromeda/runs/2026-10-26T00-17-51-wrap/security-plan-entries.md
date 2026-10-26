## 2026-10-25-windows-endpoint-admission — the Windows pipe's admission controls gain their CI witnesses
**Section:** Authentication & Authorization (authentication-authorization.md: a new `values · auth` group below the IPC access control (Windows) record)
**Change:**
- IPC access control (Windows), the remote-client rejection's witness: asserted on CI's `test (windows-2025)` by `security_negative_viola_listener_rejects_remote_clients`, which opens a client end of a bound `viola_channel::Server::bind` listener and reads its `NamedPipeType` through `NtQueryInformationFile` / `FilePipeLocalInformation`, the `FILE_PIPE_REJECT_REMOTE_CLIENTS` bit (2) set; two test-owned control pipes tell the bit apart, `security_negative_control_accept_remote_pipe_reads_the_bit_clear` (accept-remote, clear) and `security_negative_control_reject_remote_pipe_reads_the_bit_set` (reject-remote, set). Was unasserted; now asserted.
- IPC access control (Windows), the DACL's effect's witness: beyond its text read-back, the protected user + SYSTEM DACL is asserted on `test (windows-2025)` by `security_negative_viola_listener_denies_an_anonymous_open`: with the listener bound first, a `GENERIC_READ` open under `ImpersonateAnonymousToken` is denied `ERROR_ACCESS_DENIED` (5); its control `security_negative_control_default_pipe_admits_an_anonymous_read` shows the same open admitted on a pipe with the default descriptor (null security attributes). Was unasserted; now asserted.
- Both rows' Stack reference: `tests/security_negatives_channel.rs` (Windows only); windows-sys 0.61.2.
**Why:** the chunk's Windows cases passed on CI's `test (windows-2025)`, moving the remote-client rejection and the DACL's effect from unasserted to asserted; the pipe-type read-back and the anonymous-token open are the founder's own choices on the chunk's P4 forks, given live by dialog in the overseer session. The record's own `accept_remote(false)` clause measured true and now has its witness.
**Kept:** the IPC access control (Windows) record's text stands as written; the witnesses sit in their own group below it, with no chunk name.
**Ref:** .andromeda/runs/2026-10-26T00-17-51-wrap/

## 2026-10-25-windows-endpoint-admission — the supply-chain red on RUSTSEC-2026-0386 and its owner
**Section:** Dependency Security (dependency-security.md: a new `values · deps-supply-chain` group after the last one)
**Change:**
- One item: the open advisory RUSTSEC-2026-0386 (`unsound`) against the locked tokio `=1.53.1` fails CI's `supply-chain` job. Its owner is the working-route entry "tokio RUSTSEC-2026-0386 lock bump": a lock bump of `tokio` to `>=1.53.3`, with no ignore-list widening, so RUSTSEC-2017-0008 stays the only advisory ignore.
**Why:** the report's CI read: the full row red with `supply-chain` its one red, on RUSTSEC-2026-0386 against the unchanged tokio 1.53.1, not this chunk's change. The owning route entry is the overseer's founder-delegated answer, relayed by the operator, confirmed by the founder's own yes on the P5 review card, live by dialog in the overseer session. The lock bump's own wrap retires this item.
**Kept:** the `values · deps` item's tokio `=1.53.1` pin stands: no dependency was bumped and `Cargo.lock` did not move.
**Ref:** .andromeda/runs/2026-10-26T00-17-51-wrap/
