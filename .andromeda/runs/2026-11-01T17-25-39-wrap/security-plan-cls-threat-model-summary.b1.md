# Classified — security-plan · threat-model-summary.b1

## Answers
threat-model-summary#1.1 · no · the session writes no new line, field or event kind to `events.ndjson`, so the user-content row holds
threat-model-summary#1.2 · no · no tool-call, plan or dialog text is written, read or logged differently by any session change
threat-model-summary#1.3 · no · the session adds no config or snapshot field, and the `endpoint` and `pinned_bin` stored in `snapshot.json` are unchanged
threat-model-summary#1.4 · no · no credential is read, stored or stripped differently, and the session adds no secret
threat-model-summary#1.5 · no · the snapshot fields and `budget.json` are unchanged, and the session adds no operational metadata
threat-model-summary#2.1 · yes · contradict · the endpoint now sits in a verified 0700 per-user directory (`check_endpoint_dir`), disproving the `$TMPDIR`-or-`/tmp` socket and world-writable `/tmp` consequence, and the closed-cause and `server-verify-failed` reach move too
threat-model-summary#3.1 · no · no CLI send path, paste write or delivery-confirmation check is touched by the session
threat-model-summary#4.1 · no · the stdin classifier and the wheel rules are not reached by the session
threat-model-summary#5.1 · no · `src/cmd/hook.rs` keeps the strict-modes-first order, fail-open exit 0 and no-line rule for a refused instance; the new directory and euid refusals are separate checks this row does not enumerate
threat-model-summary#6.1 · yes · add-to · the MCP frame now runs the directory check after `live_endpoint` and the euid check beside `server_verified`, and this row's pre-frame check list does not yet carry them
threat-model-summary#7.1 · no · the MCP stdin `CappedLines` reader is not reached by the session
threat-model-summary#8.1 · no · the frame verbs, `run` and `revive`'s start still hold the instance and home to `check_instance` and `check_home_tree` with the exits this row states (21 and 1), and the new socket-dir refusal uses those same exits
threat-model-summary#9.1 · no · the home-tree check, the replay-reader order and `live_endpoint` are unchanged, and the board's read rule on the home is still a strict-modes check on the changed `board.rs`
threat-model-summary#10.1 · no · the pinned-binary placement path is not reached by any file in the session
threat-model-summary#11.1 · no · child spawn, PATH resolution and the `claude` and `verify` spawn rows are unchanged; the socket-dir check runs before `run`'s and `revive`'s spawn without altering what is spawned
threat-model-summary#12.1 · no · no dependency was added or bumped, `cargo deny check` stayed green, and the Tokio client stays behind the `tokio` feature, so the claim holds on the reached `Cargo.toml` and `tokio_client.rs`
threat-model-summary#13.1 · no · no CI step changed and the CI run read green, so the job list and the per-OS test jobs the row states still hold
