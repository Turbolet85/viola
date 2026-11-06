@apply_row n=1 ref_file=fanout-obs-plan--cascade-g1.md ref_n=1 v=applied edit=E1
@apply_row n=2 ref_file=fanout-obs-plan--cascade-g1.md ref_n=2 v=applied edit=E2
@apply_row n=3 ref_file=fanout-obs-plan--cascade-g1.md ref_n=3 v=applied edit=E3
@apply_row n=4 ref_file=fanout-obs-plan--cascade-g1.md ref_n=5 v=applied edit=E4
@apply_row n=5 ref_file=fanout-obs-plan--cascade-g1.md ref_n=6 v=applied edit=E5
@apply_row n=6 ref_file=fanout-obs-plan--cascade-g1.md ref_n=7 v=applied edit=E6
@apply_row n=7 ref_file=fanout-obs-plan--cascade-g1.md ref_n=8 v=applied edit=E7
@apply_row n=8 ref_file=fanout-obs-plan--cascade-g1.md ref_n=9 v=applied edit=E8
@apply_row n=9 ref_file=fanout-obs-plan--cascade-g1.md ref_n=10 v=applied edit=E9
@apply_row n=10 ref_file=fanout-obs-plan--cascade-g1.md ref_n=11 v=applied edit=E10
@apply_row n=11 ref_file=fanout-obs-plan--cascade-g1.md ref_n=12 v=applied edit=E11
@apply_row n=12 ref_file=fanout-obs-plan--cascade-g1.md ref_n=14 v=applied edit=E12
@apply_row n=13 ref_file=fanout-obs-plan--cascade-g1.md ref_n=15 v=applied edit=E13
@apply_row n=14 ref_file=fanout-obs-plan--cascade-g1.md ref_n=16 v=applied edit=E14
@apply_row n=15 ref_file=fanout-obs-plan--cascade-g1.md ref_n=18 v=applied edit=E15
@apply_row n=16 ref_file=fanout-obs-plan--cascade-g2.md ref_n=1 v=applied edit=E16
@apply_row n=17 ref_file=fanout-obs-plan--cascade-g2.md ref_n=2 v=applied edit=E17
@apply_row n=18 ref_file=fanout-obs-plan--cascade-g2.md ref_n=3 v=applied edit=E18
@fact after=fct-qat4n66a as=n1
- an OS target of Linux only: the core on any current distribution, CI on `ubuntu-latest`; the Windows and macOS code recoverable at the tag `multi-os-final` (`ecbe7ab3`);

@fact id=fct-drmiyyaa expect_rev=1
old: chaos fault telemetry, multi-OS compat, and binding
new: chaos fault telemetry, Linux platform compat, and binding

@fact id=fct-jzzdh6ip expect_rev=1
old: The ConPTY and openpty OS APIs behind it are boundary-only
new: The openpty OS API behind it is boundary-only

@fact id=fct-f3waaflx expect_rev=1
old: peer-euid (Unix), peer-pid
new: peer-euid, peer-pid
old: viola's stat and, on Windows, owner and DACL reads)
new: viola's stat reads)
old: viola's one `symlink_metadata`, off Unix no read)
new: viola's one `symlink_metadata`)

@fact id=fct-oiibon7o expect_rev=1
old: and writes its line; macOS compares no peer pid, the named carve-out from this rule, owned by no route entry, and there `server-verify-failed` means an euid mismatch alone (the Founder Direction 4 trigger below states them).
new: and writes its line (the Founder Direction 4 trigger below states it).

@fact id=fct-kv6frzyp expect_rev=1
old: (`canonical` + `trusted`: stat and, on Windows, owner and DACL reads)
new: (`canonical` + `trusted`: stat reads)

@fact id=fct-khwnpdov expect_rev=1
old: inside a 0700 directory (Windows: home DACL), and
new: inside a 0700 directory, and
old: the read rule (Unix: no group or other bit; Windows: no read grant to another SID), a file to the read rule on Unix and the write rule on Windows —
new: the read rule (no group or other bit), a file to the read rule —

@fact id=fct-h2wm2goy expect_rev=1
old: sha-256 mismatch, `.cmd`/`.bat` child, `socket-dir-refused`
new: sha-256 mismatch, `socket-dir-refused`

@fact id=fct-ycxx4ij4 expect_rev=1
old: `server-verify-failed` on Linux, Windows and macOS (on macOS an euid mismatch alone) and `socket-dir-refused`
new: `server-verify-failed` and `socket-dir-refused`

@fact id=fct-mxwnyc6t expect_rev=1
old: A server-verify mismatch (Linux, Windows and macOS, on macOS an euid mismatch alone) is one
new: A server-verify mismatch is one
old: with `detail` `socket-dir-refused`; macOS compares no peer pid (the named carve-out).
new: with `detail` `socket-dir-refused`.

@fact id=fct-5ckzuzgq expect_rev=1
old: in 0700 directories (Windows: home DACL), enforced
new: in 0700 directories, enforced
old: has a line in three places: the `ledger-stamps` read in `run`'s version gate, on Windows x64 a refused `conpty/` companion (`PinError::StrictModes`), which `process-start` carries as `sideload_fallback` `strict-modes-failed`, the exit unchanged; and a refused `update_stamps`
new: has a line in two places: the `ledger-stamps` read in `run`'s version gate, and a refused `update_stamps`

@fact id=fct-3og7ujbn expect_rev=1
old: pass `cargo deny` on all three targets.
new: pass `cargo deny` on its one target, `x86_64-unknown-linux-gnu`.

@fact id=fct-yfoyoh37 expect_rev=1
old: without EOF (ConPTY);
new: without EOF;

@fact id=fct-7bdi4wod expect_rev=1
old: The same file sink and line schema on every OS. Platform differences must still yield identical fields:
new: One file sink and line schema on Linux (any current distribution, CI on `ubuntu-latest`), each field fixed:
old: permission application: Windows DACL vs Unix 0600/0700;
new: permission application: Unix 0600/0700;
old: concurrent hook writers: Windows append vs POSIX `O_APPEND`;
new: concurrent hook writers: POSIX `O_APPEND`;
old: named pipe vs Unix socket in channel logs;
new: the Unix socket in channel logs;
old: ConPTY vs openpty in `pty` spans.
new: openpty in `pty` spans;
old: the server check's reads: the peer pid, Linux `SO_PEERCRED` vs Windows `GetNamedPipeServerProcessId` vs macOS none (`SERVER_PID_REPORTED` false, so that verdict is `Matches` with no comparison: the named carve-out, owned by no route entry); the peer euid, Unix `peer_creds` (a mismatch reads `server-verify-failed`, on macOS its one meaning) vs Windows none (`PEER_EUID_REPORTED` false, the DACL and the pid comparison the controls).
new: the server check's reads: the peer pid by `SO_PEERCRED` and the peer euid by `peer_creds` (a mismatch of either reads `server-verify-failed`; an unreadable euid is refused).

@fact id=fct-n7xftqkr expect_rev=1
old: `pinned-hash-mismatch`, `batch-script-child`, `socket-dir-refused`
new: `pinned-hash-mismatch`, `socket-dir-refused`
old: The server-verify fail is written on Linux, Windows and macOS (on macOS an euid mismatch alone) as
new: The server-verify fail is written as
old: with no pid, start time, path or endpoint; macOS compares no peer pid (the named carve-out).
new: with no pid, start time, path or endpoint.
old: reads that one word. One carve-out stands outside the distinct-code rule: macOS compares no peer pid, named and owned by no route entry.
new: reads that one word.

@fact id=fct-n6l5o3fm expect_rev=1
old: at WARN on Linux, Windows and macOS (macOS compares no peer pid, the recorded gap, so there the word means an euid mismatch alone) |
new: at WARN |

@fact id=fct-ne5xp5cg expect_rev=1
old: §9 secret-scan-before-upload, §4 Scenario 1 / §6 `process-start.sideload_fallback` `strict-modes-failed` (a `conpty/` companion its owner/DACL check refuses: the child on the inbox ConPTY, exit unchanged), §10 exemptions
new: §9 secret-scan-before-upload, §10 exemptions

@fact id=fct-6isebxud expect_rev=1
old: (0600/0700 vs home DACL)
new: (0600/0700)
old: check on windows-2025 / macos-latest / ubuntu-latest)
new: check on ubuntu-latest)
old: §9 per-OS artifacts |
new: §9 artifacts (their names in `matrix.os` form, on the one `ubuntu-latest` leg) |
@pattern id=obs2-01
fixed: multi-OS compat
@pattern id=obs2-02
fixed: ConPTY and openpty OS APIs
@pattern id=obs2-03
fixed: off Unix no read
@pattern id=obs2-04 ignore_case=true
fixed: macOS compares no peer pid
@pattern id=obs2-05
fixed: on Windows, owner and DACL reads
@pattern id=obs2-06
fixed: no read grant to another SID
@pattern id=obs2-07
fixed: `.cmd`/`.bat` child
@pattern id=obs2-08
fixed: on macOS an euid mismatch alone
@pattern id=obs2-09 ignore_case=true
fixed: Linux, Windows and macOS
@pattern id=obs2-10
fixed: Windows: home DACL
@pattern id=obs2-11
fixed: a refused `conpty/` companion
@pattern id=obs2-12
fixed: on all three targets
@pattern id=obs2-13
fixed: without EOF (ConPTY)
@pattern id=obs2-14
fixed: same file sink and line schema on every OS
@pattern id=obs2-15
fixed: named pipe vs Unix socket
@pattern id=obs2-16
fixed: `pinned-hash-mismatch`, `batch-script-child`
@pattern id=obs2-17
fixed: the recorded gap, so there the word means an euid mismatch alone
@pattern id=obs2-18
fixed: the child on the inbox ConPTY
@pattern id=obs2-19
fixed: windows-2025 / macos-latest / ubuntu-latest
@pattern id=obs2-20
fixed: 0600/0700 vs home DACL
