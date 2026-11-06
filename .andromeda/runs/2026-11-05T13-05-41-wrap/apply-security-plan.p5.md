@apply_row n=1 ref_file=fanout-security-plan--cascade-g4.md ref_n=26 v=applied edit=E1
@apply_row n=2 ref_file=fanout-security-plan--cascade-g4.md ref_n=28 v=applied edit=E2
@apply_row n=3 ref_file=fanout-security-plan--cascade-g5.md ref_n=1 v=applied edit=E3
@apply_row n=4 ref_file=fanout-security-plan--cascade-g5.md ref_n=2 v=applied edit=E4
@apply_row n=5 ref_file=fanout-security-plan--cascade-g5.md ref_n=3 v=applied edit=E5
@apply_row n=6 ref_file=fanout-security-plan--cascade-g5.md ref_n=4 v=applied edit=E6
@apply_row n=7 ref_file=fanout-security-plan--cascade-g5.md ref_n=5 v=applied edit=E7
@apply_row n=8 ref_file=fanout-security-plan--cascade-g5.md ref_n=6 v=applied edit=E8
@apply_row n=9 ref_file=fanout-security-plan--cascade-g5.md ref_n=7 v=applied edit=E9
@apply_row n=10 ref_file=fanout-security-plan--cascade-g5.md ref_n=9 v=refused
why: a hand removal · fct-f5tdilgi
@apply_row n=11 ref_file=fanout-security-plan--cascade-g5.md ref_n=11 v=applied edit=E10
@apply_row n=12 ref_file=fanout-security-plan--cascade-g5.md ref_n=12 v=applied edit=E11
@apply_row n=13 ref_file=fanout-security-plan--cascade-g5.md ref_n=14 v=applied edit=E12
@fact id=fct-sawbqqxr expect_rev=1
old: per-OS listener hardening (Windows SDDL converted by windows-sys into interprocess `security_descriptor` + windows-sys SID lookup; Unix 0700 per-user socket directory
new: Linux socket hardening (0700 per-user socket directory
old: The SDDL, the per-user directory and the 0600 socket landed
new: The per-user directory and the 0600 socket landed
old: client: the SQOS open (windows-sys `CreateFileW` + `Stream::try_from(OwnedHandle)`), the peer pid
new: client: the peer pid
old: `peer_creds().ok()?.euid()` on Unix, none on Windows; the decision `peer_admitted`, on Linux and macOS, a refusal
new: `peer_creds().ok()?.euid()`; the decision `peer_admitted`, a refusal
old: the root bin and `viola-mcp`, built on Linux and Windows; every client
new: the root bin and `viola-mcp`; every client
old: the server check. The SQOS spike passed (Decisions Log `2026-09-25`); the client reuses the recipe `tests/channel_sqos_open.rs` pins.
new: the server check.
old: the read rule (Unix `mode & 0o077`; Windows no `READ_RIGHTS` grant to another SID) on the home
new: the read rule (`mode & 0o077`) on the home
old: `check_present` (the `conpty/` companions); `check_socket_dir`
new: `check_socket_dir`
old: never a chmod, off Unix `Ok(())`);
new: never a chmod);
old: `NotADirectory` refusals; on Windows, the protected user + SYSTEM DACL for a `--home` outside `%USERPROFILE%` and the owner + DACL check (windows-sys).
new: `NotADirectory` refusals.
old: re-hash of `bin/<version>-<hash>/viola(.exe)` before
new: re-hash of `bin/<version>-<hash>/viola` before
old: override is rewritten on Unix when the pinned path
new: override is rewritten when the pinned path
old: it is not written on Windows or for a pinned path
new: it is not written for a pinned path
old: absolute pinned `bin/<version>-<hash>/viola(.exe)` path
new: absolute pinned `bin/<version>-<hash>/viola` path

@fact id=fct-fdqq3i6o expect_rev=1
old: name (ASCII case-insensitive on Windows, exact on Unix) is removed
new: name (matched exactly) is removed
old: persistent set — Windows registry `Environment` value names, Unix `config.json` `claude_env_keep` — and
new: persistent set — `config.json`'s `claude_env_keep` — and
old: The strip, the registry reader and the logs handle
new: The strip and the logs handle

@fact id=fct-io4zn2wb expect_rev=1
old: parse and, on Unix, the socket-directory and peer euid checks and, on Linux and Windows, the server verification (macOS the recorded gap), and fails open
new: parse and the socket-directory, peer euid and server verification checks, and fails open

@fact id=fct-2qzjtoe6 expect_rev=1
old: euid is refused, on macOS the whole of the check; the peer
new: euid is refused; the peer

@fact id=fct-j463drna expect_rev=1
old: On Unix a home under a regular file (unstattable) is such a refusal (the frame verbs: exit 21, the instance pair); on Windows that path stats `NotFound` and the `cli` catch-site line stands.
new: A home under a regular file (unstattable) is such a refusal (the frame verbs: exit 21, the instance pair).

@fact id=fct-g4i32qdx expect_rev=1
old: returns) and `viola_state::pin::PinError::StrictModes` carry fixed
new: returns) carries fixed
old: check`, a write failure its own fixed message, and `the pinned copy failed the strict-modes check`; `run` maps `PinError::StrictModes` to the `sideload_fallback` outcome `strict-modes-failed`.
new: check`, and a write failure its own fixed message.

@fact id=fct-zu3fn6nz expect_rev=1
old: its refusals `AgentError::BatchScriptChild` and `AgentError::ProgramNotFound`, each a fixed message;
new: its one refusal `AgentError::ProgramNotFound`, a fixed message (a given path taken as given, a bare name searched on `PATH`);

@fact id=fct-wyrvqwn5 expect_rev=1
old: or a mode or grant another user may use
new: or a mode another user may use

@fact id=fct-izf47ccx expect_rev=1
old: the server's pid (Windows: the connected stream's peer pid, interprocess `peer_creds().pid()`, which calls `GetNamedPipeServerProcessId` on the SQOS-adopted client end) or euid + pid (Linux/FreeBSD `peer_creds()`;
new: the server's euid + pid (`peer_creds()`;
old: before the server verification) plus sysinfo
new: before the server verification, an unreadable euid refused) plus sysinfo
old: against `snapshot.json` (on macOS, where `peer_creds()` gives no pid: euid + the verified 0700 socket directory only, per the recorded known gap; the euid check is never skipped), and NEVER
new: against `snapshot.json`, and NEVER
old: each verifying the server on Linux and Windows and, by the peer euid alone, on macOS (the CLI verbs through `cmd::client::connect`); only macOS's pid and start-time check stays absent, the named carve-out, owned by no route entry.
new: each verifying the server (the CLI verbs through `cmd::client::connect`); no carve-out stands.

@fact id=fct-zvkglgpg expect_rev=1
old: (`LedgerRow::relied()`, counter `/24` on Unix; on Windows, where the status line is not wrapped, the first eighteen, `resume-session`, `agents-join` and `local-command-compact`, counter `/21`)
new: (`LedgerRow::relied()`, all twenty-four rows on every build, counter `/24`)
old: `20 pass  4 fail` on Unix (`17 pass  4 fail` on Windows) records
new: `20 pass  4 fail` records
old: and, on every OS, one holding
new: and one holding

@fact id=fct-bg5w767t expect_rev=1
old: the IPC DACL or socket-directory check
new: the socket-directory check

@fact id=fct-lvhmwli7 expect_rev=1
old: `peer_euid_admitted(observed, own, reported)`
new: `peer_euid_admitted(observed, own)`

@pattern id=sec5-01
fixed: per-OS listener hardening
@pattern id=sec5-02
fixed: ASCII case-insensitive on Windows
@pattern id=sec5-03
fixed: on Linux and Windows, the server verification
@pattern id=sec5-04
fixed: on macOS the whole of the check
@pattern id=sec5-05
fixed: on Windows that path stats `NotFound`
@pattern id=sec5-06
fixed: maps `PinError::StrictModes` to the `sideload_fallback` outcome
@pattern id=sec5-07
fixed: its refusals `AgentError::BatchScriptChild`
@pattern id=sec5-08
fixed: a mode or grant another user may use
@pattern id=sec5-09
fixed: only macOS's pid and start-time check stays absent
@pattern id=sec5-10
fixed: on Windows, where the status line is not wrapped
@pattern id=sec5-11
fixed: the IPC DACL or socket-directory check
@pattern id=sec5-12
fixed: peer_euid_admitted(observed, own, reported)
