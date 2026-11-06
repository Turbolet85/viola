@answer n=1 entity=authentication-authorization#1.1 v=no
why: password hashing is N/A (no accounts or passwords) and the chunk touches no credential surface
@answer n=2 entity=authentication-authorization#1.2 v=no
why: the RBAC row is N/A under the task's not-needed list, and the chunk adds no served method
@answer n=3 entity=authentication-authorization#1.3 v=yes
why: the socket-directory wording pair (per-user 0700 $XDG_RUNTIME_DIR/viola/ or /tmp/viola-<uid>/) is the founder word item 15 owes
@answer n=4 entity=authentication-authorization#1.4 v=no
why: the directory verdict in viola-state strict.rs is not reached, and the wording the founder owes is not stated in it
@answer n=5 entity=authentication-authorization#1.5 v=no
why: the never-chmod rule of the directory check lives in viola-state strict.rs, which the chunk does not reach
@answer n=6 entity=authentication-authorization#1.6 v=no
why: check_endpoint_dir is a viola-state helper the chunk neither reaches nor names
@answer n=7 entity=authentication-authorization#1.7 v=no
why: socket_dir_ready in src/cmd/run.rs is not reached by any edited unit or named scope item
@answer n=8 entity=authentication-authorization#1.8 v=no
why: the harness cleanup unconnectable removes no socket file, a unit the harness edits leave alone, and item 17 concerns target/e2e-home removal only
@answer n=9 entity=authentication-authorization#1.9 v=yes
why: the live sessions of item 8 run through the channel's existing peer check, and the server drops a non-same-euid peer before any frame
@answer n=10 entity=authentication-authorization#1.10 v=yes
why: the accept-side drop filter and its peer_admitted decision are the peer check the live driver passes under item 8
@answer n=11 entity=authentication-authorization#1.11 v=yes
why: the peer_admitted read at the three client seams is the always-admit survivor of item 13, and the live driver passes it under item 8
@answer n=12 entity=authentication-authorization#1.12 v=yes
why: the seam order, peer euid decision before server verification, is what the item 13 survivor note and the item 8 live drives rest on
@answer n=13 entity=authentication-authorization#1.13 v=no
why: the release-from-driver guard is untouched (not-needed item 9), and the MCP tool-list disposition of item 16 is not this entity's rule
@answer n=14 entity=authentication-authorization#1.14 v=yes
why: the 0700 and write rule on bin/<version>-<hash>/ governs the keyed pinned binary that Measurement B (item 14) re-hashes before reuse
@answer n=15 entity=authentication-authorization#1.15 v=yes
why: the board's read rule, stated OPEN here, is the one item 15 owes the founder's word on
@answer n=16 entity=authentication-authorization#1.16 v=yes
why: check_home_tree's write rule on bin/<key>/ and plugin/<key>/ governs the keyed-binary folders Measurement B (item 14) concerns
@answer n=17 entity=authentication-authorization#2.1 v=yes
why: the observed server pid from peer_creds is the first input of the server-verification checks the item 8 live drives go through
@answer n=18 entity=authentication-authorization#2.2 v=yes
why: peer_euid_admitted(observed, own) is the always-admit decision of the item 13 survivor
@answer n=19 entity=authentication-authorization#2.3 v=yes
why: the ServerVerdict gate that lets a frame be written is the server-verification check the item 8 live drives pass
@answer n=20 entity=authentication-authorization#2.4 v=no
why: the no-endpoint liveness reading is neither a peer nor a server-verification check, and the scope names neither
@answer n=21 entity=authentication-authorization#2.5 v=no
why: link and unlink check ordering is untouched, the scope adds no verb, and the item 16 MCP disposition is not stated here
@answer n=22 entity=authentication-authorization#2.6 v=yes
why: the CLI frame-verb refusal on server-verify-failed is what a CLI live drive observes under item 8
@answer n=23 entity=authentication-authorization#2.7 v=yes
why: the MCP refusal mapping for server-verify-failed is what an MCP live drive observes under item 8
@answer n=24 entity=authentication-authorization#2.8 v=yes
why: tests/security_negatives_socket_dir.rs is a cfg(unix) gated file, one of the census files re-measured by the chunk, on the ubuntu-latest leg
@answer n=25 entity=authentication-authorization#2.9 v=yes
why: the peer euid decisions no test drives, with no seam to be added to drive them, is the note item 13 requires the chunk to hold
