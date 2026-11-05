@answer n=1 entity=authentication-authorization#1.1 v=no
why: no accounts or passwords exist in v1, so the chunk reaches no hashing code.

@answer n=2 entity=authentication-authorization#1.2 v=no
why: summary of the all-or-nothing principal; the chunk changes no served method or params check, and the Linux IPC rows it points to bind on their own (#1.4, #1.10).

@answer n=3 entity=authentication-authorization#1.3 v=no
why: Windows listener DACL, SDDL and pipe options only; the arm leaves with the cfg(windows) code and no Linux code must honour it.

@answer n=4 entity=authentication-authorization#1.4 v=yes
why: the Unix arm stays whole and becomes unconditional; the 0700 socket directory checked before bind and connect and the chmod 0600 after bind must keep acting on Linux.

@answer n=5 entity=authentication-authorization#1.5 v=yes
why: the Unix directory verdict (symlink, non-directory, unix_read_verdict) becomes unconditional on Linux; only the off-Unix Ok(()) fallback leaves.

@answer n=6 entity=authentication-authorization#1.6 v=yes
why: the Linux socket-directory check must keep leaving a refused directory as found, and its witness test must still hold.

@answer n=7 entity=authentication-authorization#1.7 v=yes
why: check_endpoint_dir is the one helper the Linux bind and client seams must keep calling before connect (task item 2).

@answer n=8 entity=authentication-authorization#1.8 v=yes
why: socket_dir_ready's check before any snapshot, child or bind, and its socket-dir-refused exit 1, are Linux behaviour the chunk keeps.

@answer n=9 entity=authentication-authorization#1.9 v=no
why: the rule forbids removing a leftover socket file; the chunk adds no removal, the take-over bind stays under #1.4, and the rule sits over cleanup.rs, which the chunk does not edit.

@answer n=10 entity=authentication-authorization#1.10 v=yes
why: the Linux server's peer euid check on every accept, before any frame is read, is kept as is (task item 2).

@answer n=11 entity=authentication-authorization#1.11 v=yes
why: the euid drop in accept_loop before any stream worker or frame must keep acting on Linux; the cfg(unix) gate on that filter only loses its sibling.

@answer n=12 entity=authentication-authorization#1.12 v=yes
why: the Linux client identity check (peer euid, peer pid and start time, all before the first frame) is kept, and the macOS carve-out must not leak a skipped check into it (task items 2 and 3).

@answer n=13 entity=authentication-authorization#1.13 v=yes
why: the order directory check, connect, peer euid decision, then server verification must hold at each Linux client seam.

@answer n=14 entity=authentication-authorization#1.14 v=no
why: the release-from-driver guard sits in platform-neutral channel dispatch the chunk does not edit, and no task item names it; it is a re-check every change owes.

@answer n=15 entity=authentication-authorization#1.15 v=yes
why: the Unix home and file modes (0700 home, 0600 files, replace_private, place_private_new) and the Unix strict-modes refusal before the first read must keep refusing exactly what they refused on Linux (task item 4).

@answer n=16 entity=authentication-authorization#1.16 v=yes
why: the board's read-rule refusal of the home and its instances/ check on Linux is the Unix arm task item 4 keeps; the open note concerns the Windows and macOS reading, and the Linux behaviour is unchanged.

@answer n=17 entity=authentication-authorization#1.17 v=yes
why: the home tree check at Linux start (read rule on the home, write rule on ledger, bin, plugin and the pinned exe) must keep refusing before the wrapper log opens (task item 4).

@answer n=18 entity=authentication-authorization#2.1 v=no
why: a Windows-only witness for the remote-client bit; it leaves with the Windows cases.

@answer n=19 entity=authentication-authorization#2.2 v=no
why: a Windows-only witness for the anonymous-open denial; it leaves with the Windows cases.

@answer n=20 entity=authentication-authorization#3.1 v=yes
why: the Linux client's observed server pid (SO_PEERCRED) feeds the server check and must keep writing and logging nothing; only the macOS none branch leaves (task item 2).

@answer n=21 entity=authentication-authorization#3.2 v=yes
why: the Linux observed peer euid decision, refused when unreadable, must keep holding; the cfg!(unix) gate on it becomes unconditional, and no environment variable, flag or seam may switch it off (task items 2 and 20).

@answer n=22 entity=authentication-authorization#3.3 v=yes
why: on Linux only a Matches verdict may let a frame be written; the no-pid Matches branch is the macOS part that leaves (task item 2).

@answer n=23 entity=authentication-authorization#3.4 v=yes
why: the closed no-endpoint causes instance-dead and unwrapped-name are closed instance-unreachable causes that stay as they are (task item 21).

@answer n=24 entity=authentication-authorization#3.5 v=yes
why: link and unlink's Linux order (strict-modes check first, liveness pre-check, connect, peer euid, server check) is kept; only the macOS carve-out leaves (task items 2 and 3).

@answer n=25 entity=authentication-authorization#3.6 v=yes
why: the closed CLI refusals server-verify-failed and socket-dir-refused with exit 21 must stay exactly as on Linux (task items 2 and 19).
