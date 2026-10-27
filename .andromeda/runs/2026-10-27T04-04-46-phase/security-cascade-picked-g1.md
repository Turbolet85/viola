# security · picked accounting · group 1 of 3

## Picked
threat-model-summary#1.1 · not in scope · the user-content data class and the unbounded `events.ndjson` retention; the chunk bounds a read of the file, never its content or growth (the retention item an arch-owned open risk)
threat-model-summary#1.2 · not in scope · the data class of dialog content and where it travels; the chunk's `option_details` string-only rule is the hook parse's type rule, not this classification
threat-model-summary#1.3 · carried · names `config.json`, `snapshot.json`'s `statusline_command`, the two statusline sources and `ledger/stamps.json`, the code-deciding files whose reads the chunk caps and enumerates
threat-model-summary#1.4 · not in scope · the transient inherited credential and the R8 strip; no input bound the chunk adds or re-measures reads it
threat-model-summary#1.5 · not in scope · operational metadata classification (pids, `cwd`, usage percentages); the chunk bounds no value of it
threat-model-summary#2.1 · carried · the IPC frame boundary's input half and its served methods are a row of the enumeration, and its input validation is what the chunk bounds
threat-model-summary#3.1 · carried · the paste boundary whose rule the chunk re-verifies at every client and in the wrapper, `empty-text` included
threat-model-summary#4.1 · carried · the terminal boundary is a row of the threat-model set the enumeration is checked against, its closed classifier list the type rule and no bound refusing a human byte
threat-model-summary#5.1 · carried · hook stdin is a capped boundary the chunk bounds: tolerant parse, content never a command, fail open, the instance check before any stdin read
threat-model-summary#6.1 · carried · MCP tool inputs are a boundary the chunk bounds, schemars advisory and each handler checking raw arguments before its frame
threat-model-summary#7.1 · carried · CLI arguments, `--home` / `VIOLA_DIR` and `answer`'s stdin / `--file` are boundaries the chunk bounds, after the strict-modes check
threat-model-summary#8.1 · carried · the home's cross-process readers are the files the chunk caps, torn-line tolerance and the replayed snapshot's missing fields re-asserted
threat-model-summary#10.1 · carried · `claude agents --json` and `--version` output are child-output readers the chunk caps, parsing tolerant and failure `unknown`
authentication-authorization#1.2 · carried · every served method and `hook.event` sits behind its own params check, the closed-params property the chunk proves per method
authentication-authorization#1.5 · carried · `crates/viola-channel/src/server.rs` is reached whole; the peer drop comes before any frame is read, ahead of the frame cap
authentication-authorization#1.6 · carried · `client.rs` and `tokio_client.rs` are reached whole; the server check precedes the first frame, the strict-modes check precedes the snapshot read, and `send`'s text read precedes `refuse_untrusted`
authentication-authorization#1.7 · carried · `release`'s `{budget?, from?}` params are a channel method's params whose closed type check the chunk proves, `-32602` refusal shapes included
authentication-authorization#1.8 · carried · every bounded read under the home comes after the strict-modes check, and the statusline sources' and `budget.json`'s controls are the capped, tolerant read
authentication-authorization#1.9 · carried · `src/cmd/run.rs` and `src/cmd/revive.rs` are reached whole; `check_home_tree` precedes every start read and `checked_exe` takes the pinned key the join uses
authentication-authorization#3.2 · not in scope · the closed `ServerVerdict` lives in `viola-state` liveness, which the chunk only uses; the scope names no server-verify behaviour (peer-credential hardening is out)
authentication-authorization#3.3 · not in scope · `link` / `unlink`'s server-check order in `src/cmd/link.rs`, which the chunk does not reach; their name check is carried through its witness
authentication-authorization#3.4 · carried · the client's exit-13 paste refusals come before the strict-modes, liveness and server checks, the client-side half of the paste rule
authentication-authorization#3.7 · carried · `channel_link_names_are_validated_by_the_wrapper` witnesses the channel `link` / `unlink` names before a path join, a per-boundary witness the proof names
