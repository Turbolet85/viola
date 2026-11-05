@answer n=1 entity=authentication-authorization#1.1 v=no
why: no accounts or passwords exist, so the chunk adds or changes no credential hashing code.
@answer n=2 entity=authentication-authorization#1.2 v=yes
why: the all-or-nothing gate is the Unix IPC OS check that task item 2 keeps whole, so the Linux gate must stay the only one for the nine methods.
@answer n=3 entity=authentication-authorization#1.3 v=no
why: the Windows protected DACL arm is one task item 2 removes, so the chunk has no Windows listener left to honour it.
@answer n=4 entity=authentication-authorization#1.4 v=yes
why: the 0700 socket directory checked before bind and connect and the chmod 0600 after bind are the Unix IPC arms task item 2 keeps whole.
@answer n=5 entity=authentication-authorization#1.5 v=yes
why: the Unix directory verdict (check_socket_dir, socket_dir_verdict) is the check task item 2 keeps on Linux, and only its off-Unix arm goes.
@answer n=6 entity=authentication-authorization#1.6 v=yes
why: a refused socket directory must stay unchanged on Linux, which task item 2 requires to act exactly as before.
@answer n=7 entity=authentication-authorization#1.7 v=yes
why: check_endpoint_dir is the shared directory check before bind and at the client connect seams, named by task item 2.
@answer n=8 entity=authentication-authorization#1.8 v=yes
why: socket_dir_ready runs the directory check before bind and its socket-dir-refused start refusal is a Linux contract task items 2 and 19 keep.
@answer n=9 entity=authentication-authorization#1.9 v=yes
why: the leftover socket takeover on bind is Linux bind behaviour that task item 2 keeps exactly as before.
@answer n=10 entity=authentication-authorization#1.10 v=yes
why: the server-side peer euid check on every accept is a Unix IPC arm task item 2 keeps whole.
@answer n=11 entity=authentication-authorization#1.11 v=yes
why: the euid drop in accept_loop before any frame is the server-side peer check task item 2 keeps, and task item 3 binds its frame-before-verification ban.
@answer n=12 entity=authentication-authorization#1.12 v=yes
why: the Linux client identity checks (peer euid, peer pid, start time) and the before-frame rule bind every Linux client path under task items 2 and 3, while the Windows open and the macOS carve-out leave.
@answer n=13 entity=authentication-authorization#1.13 v=yes
why: the seam order of directory check, connect, euid decision and server verification is the Linux client path task items 2 and 3 keep.
@answer n=14 entity=authentication-authorization#1.14 v=no
why: the release-from-driver guard sits in the channel dispatch and release params, which the chunk does not edit and no task item names.
@answer n=15 entity=authentication-authorization#1.15 v=yes
why: the Unix home 0700, 0600 file creation and strict-modes refusal are checks task item 4 keeps refusing exactly as before on Linux.
@answer n=16 entity=authentication-authorization#1.16 v=yes
why: the board scan's check_read of the home is the Linux strict-modes read rule that task item 4 keeps.
@answer n=17 entity=authentication-authorization#1.17 v=yes
why: check_home_tree's home, instance and pinned-exe rules are the Linux strict-modes checks task item 4 keeps before the first read under the home.
@answer n=18 entity=authentication-authorization#2.1 v=no
why: the remote-client rejection witness is a Windows-only test on the windows-2025 leg, removed with its tests.
@answer n=19 entity=authentication-authorization#2.2 v=no
why: the anonymous-open denial witness is a Windows-only test on the windows-2025 leg, removed with its tests.
@answer n=20 entity=authentication-authorization#3.1 v=yes
why: the client's observed server pid read on Linux through SO_PEERCRED is the peer pid check task item 2 keeps, while the Windows and macOS readings leave.
@answer n=21 entity=authentication-authorization#3.2 v=yes
why: the observed peer euid decision, refused on Unix when unreadable, is the Linux peer check task item 2 keeps and task item 20 bounds.
@answer n=22 entity=authentication-authorization#3.3 v=yes
why: the closed ServerVerdict and the rule that only Matches writes a frame bind the Linux client path under task items 2 and 3.
@answer n=23 entity=authentication-authorization#3.4 v=yes
why: the instance-dead and unwrapped-name readings are closed instance-unreachable causes that task item 21 keeps as they are.
@answer n=24 entity=authentication-authorization#3.5 v=yes
why: link and unlink run the Linux client connect with the socket-directory check, euid decision and server check that task items 2 and 3 keep.
@answer n=25 entity=authentication-authorization#3.6 v=yes
why: the CLI exit 21 refusals with server-verify-failed and socket-dir-refused are closed Linux contracts that task items 2, 19 and 21 keep.
