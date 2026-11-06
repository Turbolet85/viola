@answer n=1 entity=authentication-authorization#2.10 v=yes mode=stale
why: the Windows-remnant removal reached tests/security_negatives_socket_dir.rs, where the socket-directory refusal test this entity names lives, and no report shows its claim still holds on the changed file
@answer n=2 entity=authentication-authorization#2.11 v=no
why: the client seam sits in viola-channel and tests/channel_links.rs, neither reached by the change, and no viola-state edge moved
