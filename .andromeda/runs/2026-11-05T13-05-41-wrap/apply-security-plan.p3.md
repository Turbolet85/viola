@apply_row n=1 ref_file=fanout-security-plan--cascade-g2.md ref_n=1 v=refused
why: a hand removal · fct-y6zteyy6
@apply_row n=2 ref_file=fanout-security-plan--cascade-g2.md ref_n=2 v=applied edit=E1
@apply_row n=3 ref_file=fanout-security-plan--cascade-g2.md ref_n=3 v=applied edit=E2
@apply_row n=4 ref_file=fanout-security-plan--cascade-g2.md ref_n=4 v=applied edit=E3
@apply_row n=5 ref_file=fanout-security-plan--cascade-g2.md ref_n=5 v=applied edit=E4
@apply_row n=6 ref_file=fanout-security-plan--cascade-g2.md ref_n=6 v=applied edit=E5
@apply_row n=7 ref_file=fanout-security-plan--cascade-g2.md ref_n=7 v=applied edit=E6
@apply_row n=8 ref_file=fanout-security-plan--cascade-g3.md ref_n=1 v=refused
why: a hand removal · fct-o3bl5eop
@apply_row n=9 ref_file=fanout-security-plan--cascade-g3.md ref_n=2 v=refused
why: a hand removal · fct-6fdpugfk
@apply_row n=10 ref_file=fanout-security-plan--cascade-g3.md ref_n=3 v=applied edit=E7
@apply_row n=11 ref_file=fanout-security-plan--cascade-g3.md ref_n=4 v=applied edit=E8
@apply_row n=12 ref_file=fanout-security-plan--cascade-g3.md ref_n=5 v=applied edit=E9
@apply_row n=13 ref_file=fanout-security-plan--cascade-g3.md ref_n=6 v=applied edit=E10
@apply_row n=14 ref_file=fanout-security-plan--cascade-g3.md ref_n=7 v=refused
why: a hand removal · fct-dyanhg3f
@apply_row n=15 ref_file=fanout-security-plan--cascade-g3.md ref_n=8 v=refused
why: a hand removal · fct-ka73bihm
@apply_row n=16 ref_file=fanout-security-plan--cascade-g3.md ref_n=9 v=refused
why: a hand removal · fct-ghctc7rk
@apply_row n=17 ref_file=fanout-security-plan--cascade-g3.md ref_n=10 v=applied edit=E11
@apply_row n=18 ref_file=fanout-security-plan--cascade-g3.md ref_n=11 v=applied edit=E12
@fact id=fct-2pxk6g7w expect_rev=1
old: `$XDG_RUNTIME_DIR/viola/` on Linux, `$TMPDIR/viola/` on macOS, and `/tmp/viola-<uid>/` only as a fallback
new: `$XDG_RUNTIME_DIR/viola/`, and `/tmp/viola-<uid>/` only as a fallback
old: chmod 0600 right after the bind, on every Unix OS. `ListenerOptionsExt::mode(0o600)` is not used: interprocess 2.4.4 returns `Unsupported` on macOS, which fails the bind, and ignoring it would leave the macOS socket at the umask mode. `try_overwrite`
new: chmod 0600 right after the bind. `ListenerOptionsExt::mode(0o600)` is not used: chmod after the bind is the one form. `try_overwrite`

@fact id=fct-a6h3q7dw expect_rev=1
old: through `unix_read_verdict`. Off Unix `check_socket_dir` returns `Ok(())`.
new: through `unix_read_verdict`.

@fact id=fct-ju3c6nzj expect_rev=1
old: **Windows client open:** open the pipe through windows-sys ` CreateFileW(..., SECURITY_SQOS_PRESENT | SECURITY_IDENTIFICATION | FILE_FLAG_OVERLAPPED, ...) `, then adopt the handle with `interprocess::os::windows::named_pipe::local_socket::Stream::try_from(OwnedHandle)`. The adoption spike passed (Decisions Log `2026-09-25`): after adoption a same-user server reads `SecurityIdentification`, as measured at chunk 2026-09-25-security-prerequisites (research.md §Measured facts, CI run 36138441784 windows-2025), and `tests/channel_sqos_open.rs` pins the recipe. `FILE_FLAG_OVERLAPPED` is required: a non-overlapped handle adopted this way hangs its first exchange (same measurement). The client never falls back to interprocess's default connect, which opens without SQOS and lets the server impersonate (measured: `SecurityImpersonation`) (§ Authentication). **Windows identity check:** the connected stream's peer pid (the observed server pid, below) must equal the snapshot `pid`, and sysinfo's process start time must equal snapshot `started_at`. **Unix identity check:** `peer_creds().euid()`
new: **Linux identity check:** `peer_creds().euid()`
old: a refused euid reads `server-verify-failed`), and where available (Linux/FreeBSD) `peer_creds().pid()` must equal the snapshot `pid`, with the sysinfo start-time match (built on Linux, the `SO_PEERCRED` peer pid). **macOS:** `peer_creds()` gives no pid, so verification there is euid + the verified 0700 socket directory, with no pid check. This is accepted for v1 (macOS is CI-only, brief O5) and recorded as a known gap for the public version. **Timing:**
new: a refused euid reads `server-verify-failed`), and `peer_creds().pid()` (the `SO_PEERCRED` peer pid) must equal the snapshot `pid`, with the sysinfo start-time match. **Timing:**
old: then on Linux and Windows checks the serving process's identity before the frame (macOS the recorded gap):
new: then on Linux checks the serving process's identity before the frame:
old: is closed on Linux and Windows and open on macOS; the pipe DACL and the 0700 socket directory still apply.
new: is closed; the 0700 socket directory still applies.
old: then, on Linux and Windows, check the serving process's identity
new: then, on Linux, check the serving process's identity
old: The residual is the same, closed on Linux and Windows and open on macOS.
new: The residual is the same, closed.
old: on Linux and Windows both check the serving process's identity before the frame (`answer`
new: on Linux both check the serving process's identity before the frame (`answer`
old: a PermissionRequest `allow` included), is closed on Linux and Windows and open on macOS.
new: a PermissionRequest `allow` included), is closed.
old: on Linux and Windows both then check
new: on Linux both then check
old: or answering a `release`, is closed on Linux and Windows and open on macOS.
new: or answering a `release`, is closed.
old: on Linux and Windows each tool then checks
new: on Linux each tool then checks
old: (sync + `tokio` feature: the SQOS open and the peer pid,
new: (sync + `tokio` feature: the peer pid,
old: the `frame` of `viola-mcp`); windows-sys 0.61.2; sysinfo 0.39.6
new: the `frame` of `viola-mcp`); sysinfo 0.39.6

@fact id=fct-65crhg3m expect_rev=1
old: and `events.ndjson` to the write rule on Unix and the read rule on Windows, each that exists
new: and `events.ndjson` to the write rule, each that exists
old: proven by `cfg(unix)` cases and by `cfg(windows)` cases (an instance tree granted Users write refused beside a `create_private_dir` tree that passes, a `diagnostics/` granted Users read refused, a plain home made before viola refused `Writable` (read by the inherited write right whichever SID holds it, with a viola-made-home control)), green on `windows-2025` as measured at ci#40403389629; the start arm's
new: proven by `cfg(unix)` cases; the start arm's
old: proven on all three OSes; the board holds the home to the read rule on every OS, so a home another user may read (a 0750 home on Unix) is refused
new: proven on Linux; the board holds the home to the read rule, so a home another user may read (a 0750 home) is refused
old: proven for `link` / `unlink` by `cfg(unix)` cases only, for the six frame verbs on all three OSes)
new: proven for `link` / `unlink` by `cfg(unix)` cases, for the six frame verbs on Linux)
old: detail:"strict-modes-failed"}`); on Windows x64, `viola run`'s ConPTY companions `bin/<version>-<hash>/conpty/{OpenConsole.exe,conpty.dll}` (`pin_companions` runs `viola_state::strict::check_present` on the `conpty/` folder and on each companion that exists before the `FILE_SHARE_READ`-only held open and the full SHA-256 re-hash against the compiled pins; a refusal writes nothing and leaves the child on the inbox ConPTY with `sideload_fallback` `strict-modes-failed`);
new: detail:"strict-modes-failed"}`);
old: `strict::unix_read_verdict`). This is the Unix counterpart of the Windows read rule, so a pre-existing home
new: `strict::unix_read_verdict`), so a pre-existing home
old: (exit 21). **Windows:** `%USERPROFILE%` inheritance (user + SYSTEM + Administrators) is accepted. A `--home` outside `%USERPROFILE%` gets an explicit protected user + SYSTEM DACL at creation (via windows-sys), or is refused if one cannot be set. **Windows strict-modes check:** at the same entry points as Unix, read the owner and DACL of the home, `instances/<name>/`, `bin/`, `plugin/`, `ledger/`, the `bin/<version>-<hash>/` and `plugin/<version>-<hash>/` folders the process uses or writes (including the plugin folder's `hooks/` and `.claude-plugin/` subfolders), and the trusted files themselves (`snapshot.json`, `ledger/stamps.json`, the pinned `viola.exe`), matching the Unix list (each checked separately, because Windows bypass-traverse makes a subdirectory reachable regardless of the home's ACL, and a file can carry an explicit, non-inherited ACE that no directory check sees) (windows-sys `GetNamedSecurityInfoW` + `GetAce`).
new: (exit 21).
old: Refuse when the security descriptor cannot be read, when the DACL is NULL (a NULL DACL grants everyone full access yet holds no ACE for `GetAce` to reject), when the volume lacks persistent ACLs (`GetVolumeInformationW` without `FILE_PERSISTENT_ACLS`, for example a `--home` on FAT32/exFAT), when the owner is not the user SID or Administrators, or when any allow ACE grants write (`FILE_WRITE_DATA`/`FILE_ADD_FILE`, `FILE_APPEND_DATA`/`FILE_ADD_SUBDIRECTORY`, `FILE_DELETE_CHILD`, `GENERIC_WRITE` or `GENERIC_ALL`), delete, `WRITE_DAC` or `WRITE_OWNER` to a SID other than the user, SYSTEM or Administrators (for example `Users`, `Authenticated Users`, `Everyone`); inherit-only ACEs count for this write rule too, because files and subdirectories viola creates inherit them (an object-inherit write grant to `Users` on `bin/`, `plugin/`, `instances/<name>/` or `ledger/` would make the pinned exe, `hooks.json`, `snapshot.json` or `stamps.json` writable by other users between checks). `CREATOR OWNER` (S-1-3-0) is treated as the user for both the write and read rules, because it resolves to the creating user. **Read rule:** on the home, `instances/<name>/` and `instances/<name>/diagnostics/`, and on each existing `events.ndjson` before the process opens it (the file is appended to across sessions and never replaced, so an explicit ACE on it survives, and bypass-traverse makes it readable by its predictable path whatever the directory ACLs say), also refuse any allow ACE, including an inherit-only ACE with object-inherit, that grants read (`READ_RIGHTS` = `0x9000_0001`: `FILE_READ_DATA`, `GENERIC_READ` or `GENERIC_ALL`, judged by `strict::windows_read_verdict`; inherit-only ACEs usually keep unmapped generic bits, as `icacls (OI)(CI)(IO)(F)` stores `GENERIC_ALL`, so test the generic bits as well as the specific ones) to such a SID. Files viola creates inherit these DACLs, so a readable directory exposes `events.ndjson` and `diagnostics/`. A refusal names the property it refused:
new: A refusal names the property it refused:
old: This is the Windows counterpart of Unix's 0600 file creation. The outcomes match Unix but one: an unstattable home (a path under a regular file) is a strict-modes refusal on Unix (`verify` exits 1 with the home pair, the frame verbs 21 with the instance pair), while on Windows such a path stats `NotFound`, passes as absent, and the catch-site line stands. This covers
new: An unstattable home (a path under a regular file) is a strict-modes refusal (`verify` exits 1 with the home pair, the frame verbs 21 with the instance pair). This covers
old: a start under umask 000, the Windows protected DACL read back, a read-granted
new: a start under umask 000, a read-granted
old: green on all three OSes as measured at ci#40403389629.
new: green on Linux as measured at ci#42030025050.
old: std `OpenOptionsExt`; windows-sys 0.61.2
new: std `OpenOptionsExt`

@fact id=fct-pq4mk5dy expect_rev=1
old: `instances/` to `check_path` on every OS, the first failure
new: `instances/` to `check_path`, the first failure
old: the board's read rule on every OS stands on
new: the board's read rule stands on

@fact id=fct-mhqlyrp7 expect_rev=1
old: the pinned `viola(.exe)`,
new: the pinned `viola`,

@fact id=fct-gfc5nchi expect_rev=1
old: `peer_creds().pid()`): on Linux `SO_PEERCRED`, on Windows `GetNamedPipeServerProcessId` on the SQOS-adopted client end, on macOS none. `SERVER_PID_REPORTED`, a `const bool` re-exported by `viola-channel`, is false on macOS. Reading the pid
new: `peer_creds().pid()`, Linux `SO_PEERCRED`). Reading the pid

@fact id=fct-zpaxdj2d expect_rev=1
old: `peer_euid_admitted(observed, own, reported)` admits when not reported or when observed equals own (`!reported || observed == Some(own)`); `PEER_EUID_REPORTED` is `cfg!(unix)`, so a peer whose euid cannot be read is refused on Unix, and on Windows no euid is read and the DACL and the pid comparison stay the controls. `Client::server_euid`
new: `peer_euid_admitted(observed, own)` admits only when observed equals own (`observed == Some(own)`), so a peer whose euid cannot be read is refused. `Client::server_euid`

@fact id=fct-5ryfhirx expect_rev=1
old: lets a frame be written. Where no peer pid is reported (`SERVER_PID_REPORTED` false, macOS) the verdict is `Matches` with no comparison.
new: lets a frame be written.

@fact id=fct-p7bllbag expect_rev=1
old: the peer euid decision on Unix and, on Linux and Windows, the server check (macOS compares no pid or start time, the named carve-out).
new: the peer euid decision and the server check (peer pid and start time).

@fact id=fct-ezmg3kua expect_rev=1
old: a symlink to a 0700 one, on `ubuntu-latest` and `macos-latest`, none on Windows.
new: a symlink to a 0700 one, on CI's `ubuntu-latest` leg.

@fact id=fct-h63tjgiv expect_rev=1
old: against an impostor server on Linux and Windows, compiled out on macOS with the file;
new: against an impostor server, the file ungated;
old: a symlink to a 0700 one, on Linux and macOS.
new: a symlink to a 0700 one.

@pattern id=sec3-01
fixed: `$TMPDIR/viola/` on macOS
@pattern id=sec3-02
fixed: Off Unix `check_socket_dir` returns
@pattern id=sec3-03
fixed: closed on Linux and Windows and open on macOS
@pattern id=sec3-04
fixed: (macOS the recorded gap)
@pattern id=sec3-05
fixed: the write rule on Unix and the read rule on Windows
@pattern id=sec3-06
fixed: This is the Unix counterpart of the Windows read rule
@pattern id=sec3-07
fixed: `SERVER_PID_REPORTED` false, macOS
@pattern id=sec3-08
fixed: `!reported || observed == Some(own)`
@pattern id=sec3-09
fixed: macOS compares no pid or start time
@pattern id=sec3-10
fixed: compiled out on macOS with the file
@pattern id=sec3-11
fixed: none on Windows
@pattern id=sec3-12
fixed: read rule on every OS
