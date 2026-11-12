@row_add n=1 row=decision/tui-passthrough-never-parses-screen area=6-e2e-test-strategy rec=1
name: Passthrough verdicts never parse the rendered screen
rule: viola run passthrough tests detect exit only via child wait, drain the master reader on its own thread, and take verdicts from events.ndjson, the receipt and the exit status, never from a parse of the rendered screen.
except: The forced vt100 panic chaos case byte-searches the drained master reader's stream for the child's own bytes.
@row_add n=2 row=decision/endpoint-read-from-snapshot area=6-e2e-test-strategy rec=4
name: Endpoint read only from the snapshot
rule: Wrapper channel tests drive the sync client, a raw interprocess stream and jsonschema, and read the endpoint only from snapshot.json.
@row_add n=3 row=decision/mcp-stdio-test-owns-child area=6-e2e-test-strategy rec=6
name: MCP stdio test owns the child
rule: MCP stdio tests run an rmcp client over a viola mcp child the test spawns and owns, closing its stdin and awaiting its exit, never rmcp's TokioChildProcess, beside a replay of the recorded MCP exchange.
@row_add n=4 row=decision/cross-user-peers-untestable area=6-e2e-test-strategy rec=7,8
name: Cross-OS-user peers skipped as untestable
rule: Cross-OS-user peers are skipped as untestable, covered by the peer-credential boundary unit test and same-user socket modes read-back; the euid wiring at the three client seams is held by no committed test.
@row_add n=5 row=contract/viola-send/empty-text area=6-e2e-test-strategy rec=9,10
boundary: viola send
item: empty-text
promise: A text whose typed text is empty once its trailing newlines are removed is refused at once, exit 13 not-delivered/empty-text, and never issued.
@row_add n=6 row=contract/viola-answer/unknown-dialog area=6-e2e-test-strategy rec=11,12
boundary: viola answer
item: 13 unknown-dialog
promise: An answer to an unknown dialog id exits 13 {"refusal":"not-delivered","detail":"unknown-dialog"}.
@row_add n=7 row=contract/viola-answer/unverified-cli area=6-e2e-test-strategy rec=14,13
boundary: viola answer
item: 12 unverified-cli
promise: An answer on an unverified or unstamped CLI is refused exit 12 unverified-cli with no decision body, the dialog left to the human; MCP answer refuses alike.
@row_add n=8 row=decision/snapshot-replay-from-log area=6-e2e-test-strategy rec=15,16,17,18
name: Unreadable snapshot replayed from the log
rule: Over a truncated, unreadable or newer-version snapshot, a start and the product readers replay instance state from the event log, links, the budget pause, override end and the agent session id included, with no dialog pending, and write no file.
guard: a_start_over_an_unreadable_snapshot_replays_the_log, e5_the_replay_reads_what_the_wrapper_wrote
@row_add n=9 row=decision/start-collision-by-heartbeat area=6-e2e-test-strategy rec=15,17,19
name: Start collision read from the heartbeat
rule: Over a replayed snapshot the start collision check reads liveness from the heartbeat alone: a fresh beat refuses the start already-live (exit 1), a beat older than STALE_AFTER does not.
guard: a_start_over_a_replayed_snapshot_and_a_fresh_beat_refuses_already_live
@row_add n=10 row=decision/hook-fails-open area=6-e2e-test-strategy rec=20,21
name: Hook fails open silently
rule: The hook exits 0 with empty streams and no frame on oversize stdin, malformed JSON, a clap error, a channel failure, a forced panic, a missing VIOLA_NAME, no instance directory, a refused instance or socket directory, or a server that is not the recorded wrapper.
guard: hook_fails_open_silently_within_the_spine_bound, hook_dialog_tier_fails_open_silently
@row_add n=11 row=contract/json-rpc-errors/-32603 area=6-e2e-test-strategy rec=22,23
boundary: JSON-RPC errors
item: -32603
promise: Every -32603 carries exactly the message "internal error" and data null.
@row_add n=12 row=decision/umask-independent-modes area=6-e2e-test-strategy rec=24,25
name: Umask-independent modes
rule: Under any umask, home files are 0600, directories 0700 and the pinned viola binary 0700, never group- or world-writable, and a clean start keeps the socket private.
guard: security_negative_a_clean_start_under_an_open_umask_keeps_the_socket_private
