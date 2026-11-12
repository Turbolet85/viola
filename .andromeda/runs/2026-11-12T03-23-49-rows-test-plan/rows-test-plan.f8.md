@row_add n=1 row=decision/driver-hints-never-name-release area=e2e-test-strategy rec=2,1,3
name: Driver-facing hints never name release
rule: A driver-facing refusal hint (human-typing, budget-paused) never contains release.
why: Drivers are refused release.
guard: no_hint_names_release, path6_the_refusal_hint_never_names_release
@row_add n=2 row=decision/snapshot-replay-from-log area=e2e-test-strategy rec=4
name: Snapshot replayed from the log
rule: Over an unreadable or newer snapshot the product readers replay instance state from the log the wrapper wrote, budget_paused, budget_override_until and links included.
guard: e5_the_replay_reads_what_the_wrapper_wrote, e5_the_replay_reads_the_link_the_wrapper_wrote
@row_add n=3 row=decision/error-bodies-hold-no-path area=e2e-test-strategy rec=6,7
name: Error bodies hold no path or upstream text
rule: CLI --json documents, MCP isError results and channel error.data contain no absolute path, no serde path and no Caused by, checked on each surface and by an enumeration of error documents with a census of writers.
guard: cli_error_documents_hold_no_path_or_upstream_text, channel_error_bodies, contract_error_bodies
@row_add n=4 row=contract/revive-json/refusal-document area=e2e-test-strategy rec=2,8
boundary: revive --json
item: refusal document
promise: An exit-1 refusal of the strict-modes check, the preflight or the start it shares with run prints {"v":1,"error":"<code>","detail":null} on stdout with empty stderr; without --json the human pair is unchanged.
@row_add n=5 row=decision/peer-euid-admission area=mocking-stubbing-discipline rec=9,10
name: Peer admitted only on the owning user's euid
rule: A channel peer is admitted only when its observed euid equals the server's own; an unreadable euid is refused, and a refused peer is dropped before any frame, with no line written.
guard: peer_euid_admitted_takes_only_the_owning_user_where_it_is_reported, serve_drops_a_refused_peer_before_any_frame
