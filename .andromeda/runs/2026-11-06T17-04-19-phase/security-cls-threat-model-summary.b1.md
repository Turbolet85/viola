@answer n=1 entity=threat-model-summary#1.1 v=yes
why: the live driver's prompts land in events.ndjson, and the NEVER-log floor of the live readings keeps user content out of them
@answer n=2 entity=threat-model-summary#1.2 v=yes
why: a live dialog hold writes permission tool input and question text to events.ndjson and hook.dialog, which the readings must keep out
@answer n=3 entity=threat-model-summary#1.3 v=yes
why: the pinned bin/<version>-<hash>/viola is what Measurement B re-keys by content hash, and the live start writes snapshot.json and reads statusline-source
@answer n=4 entity=threat-model-summary#1.4 v=yes
why: CLAUDE* stripped values are the NEVER-log floor the live readings must hold to, and the dev-host crossings bound what a live run may read
@answer n=5 entity=threat-model-summary#1.5 v=yes
why: the live start's snapshot holds pid, child_pid and the absolute cwd, which the live readings and the /proc fd measurement must not carry
@answer n=6 entity=threat-model-summary#2.1 v=yes
why: the live driver's send and wait pass the channel's peer and server-verification checks, and v1-13/v1-36 keep link, unlink and release off MCP
@answer n=7 entity=threat-model-summary#3.1 v=yes
why: the live driver's send uses the bracketed-paste framing and delivery confirmation this entity states
@answer n=8 entity=threat-model-summary#4.1 v=yes
why: needs 11 extends the stdin classifier's non-editing list only by a live-measured reply, and the headless rig's terminal replies sit on that list
@answer n=9 entity=threat-model-summary#5.1 v=yes
why: the dialog hold read near 3600 s observes the PermissionRequest hook's fail-open outcome, while hook parsing and the statusline shell-out stay unchanged
@answer n=10 entity=threat-model-summary#6.1 v=yes
why: the live driver's MCP send, wait, last and answer meet the strict-modes, socket-dir and euid checks, and release, link and unlink stay off the MCP list (needs 8, 16)
@answer n=11 entity=threat-model-summary#7.1 v=no
why: the MCP stdin CappedLines reader is unchanged (not_needed 3) and no scope item names an over-long driver line
@answer n=12 entity=threat-model-summary#8.1 v=yes
why: viola run must resolve the child executable and spawn it as given (needs 5), and the live runs use their own --home under check_home_tree and check_instance
@answer n=13 entity=threat-model-summary#9.1 v=yes
why: Measurement B's pinned binary keying and the never-delete rule on bin/ and plugin/ sit under the strict-modes check on run's home tree (needs 14)
@answer n=14 entity=threat-model-summary#10.1 v=yes
why: Measurement B re-keys the pinned copy and re-hashes it before reuse, the placement path place_private_new's ETXTBSY-safe rule governs
@answer n=15 entity=threat-model-summary#11.1 v=yes
why: viola run resolves the child executable and hands portable-pty an absolute path (needs 5), and the live run resolves claude from the calling PATH (needs 8)
@answer n=16 entity=threat-model-summary#12.1 v=yes
why: the portable-pty pin, cargo deny over deny.toml and the sole-root tokio ban are the supply-chain trust-boundary line the chunk must make agree with the code (needs 1, 18)
@answer n=17 entity=threat-model-summary#13.1 v=yes
why: the CI/CD line names the one-leg ubuntu-latest matrix, the supply-chain job and the release check that refuses test-only binaries (needs 7, 18)
