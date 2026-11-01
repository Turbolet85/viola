## 2026-11-01-unix-endpoint-and-home-hardening — exit 21's closed causes, five to six
**Section:** §7 Error Capture & Reporting (error-capture-reporting.md: start refusals under `--json`, the exit-21 document); §1 Obs Scope Summary (obs-scope-summary.md: the `viola-core` entity)
**Change:**
- Exit 21's `detail` was one of five closed words of `viola_core::UnreachableCause`; now one of six: `strict-modes-failed`, `instance-dead`, `unwrapped-name`, `endpoint-lost`, `server-verify-failed`, `socket-dir-refused`, still with no serde path. The `viola-core` entity's closed set reads the same six.
- `refused_with` prints the exit-1 document in place of the human pair for `socket-dir-refused` too, beside `batch-script-child`, `pinned-hash-mismatch` and `squatted-name`, all before `spawn_child`.
**Why:** `UnreachableCause::SocketDir` is built as the sixth cause, with no serde path and no `RefusalReason`; `start` calls `refused_with` with the word before any snapshot, child or `Server::bind`, `revive`'s start sharing it. The word is the founder's own, `inputs#I2` Q1, live, 2026-11-01 before 16:27 Z, through the overseer's dialog, given after the widening was shown to him; these rows read as inside it on the answer card's confirm, the overseer's as the founder's delegate, PROVISIONAL, superseded by the founder's own word in the batch at Epoch 7's boundary.
**Ref:** .andromeda/runs/2026-11-01T17-25-39-wrap/

## 2026-11-01-unix-endpoint-and-home-hardening — MCP's frame tools: the socket-directory and euid refusals
**Section:** §7 Error Capture & Reporting (error-capture-reporting.md: the MCP rejects, the cause mapping); §1 Obs Scope Summary (obs-scope-summary.md: the mcp surface notes, the Vector 5 trigger); §4 Span / Trace Coverage (span-trace-coverage.md: viola-mcp's server check); §7 Error Capture & Reporting (obs-plan.md: the MCP rejects line)
**Change:**
- `frame` runs `check_endpoint_dir` on the recorded endpoint after `live_endpoint`; a refusal returns `instance-unreachable` with `detail` `socket-dir-refused` through `UnreachableCause::SocketDir`, which no `RefusalReason` carries, and writes no frame and no line, the `isError` result its only record.
- The cause mapping was a failed `server_verified` alone as `ServerVerify`; now a refused `peer_admitted` or a failed `server_verified`, and a refused `check_endpoint_dir` is `SocketDir`.
- The span chain reads `check_instance`, `live_endpoint`, `check_endpoint_dir`, then after `TokioClient::connect`, inside `mcp.tool_call`, the euid decision `peer_admitted(client.server_euid())` and `server_verified`. A refused euid, a mismatch or a check that cannot be made returns `server-verify-failed`, no frame and no log line.
- `instance-unreachable`'s closed causes in the mcp notes gain `socket-dir-refused`; Vector 5's no-line calls gain a refused socket directory and a refused server euid.
- The master's no-line list gains a refused socket directory (made after the liveness pre-check) and a refused euid; the mismatch was Linux and Windows; now Linux, Windows and macOS, on macOS an euid mismatch alone.
**Why:** `frame` maps `check_endpoint_dir` to `SocketDir` and a refused euid or failed check to `ServerVerify`, with no line by rule. The words are the founder's own, `inputs#I2` Q1 and Q2, live, 2026-11-01 before 16:27 Z, through the overseer's dialog, given after the widening was shown to him; these rows read as inside them on the answer card's confirm, the overseer's as the founder's delegate, PROVISIONAL, superseded by the founder's own word in the batch at Epoch 7's boundary.
**Ref:** .andromeda/runs/2026-11-01T17-25-39-wrap/

## 2026-11-01-unix-endpoint-and-home-hardening — `Refused::NotADirectory`'s fixed message
**Section:** §7 Error Capture & Reporting (error-capture-reporting.md: the error bodies)
**Change:**
- `Refused::NotADirectory`'s message reads fixed, `a trusted state path is not a directory`, and names no path.
- It is a variant of `Refused`, no derive of its own, so it adds nothing to the 9 `thiserror` derives.
**Why:** `socket_dir_verdict` refuses a path that is no directory through this new variant; the chunk's re-count of `thiserror::Error` reads 9, not moved.
**Kept:** the line stating the 9 derives and its as-measured chunk, unedited.
**Ref:** .andromeda/runs/2026-11-01T17-25-39-wrap/

## 2026-11-01-unix-endpoint-and-home-hardening — `socket-dir-refused` in the log catalogue, `process-exit` 19 to 20, `hook-decision` 7 to 8
**Section:** §6 Log Coverage (log-coverage.md: exit 1's words, the schema's count, exit 21's words, the `hook-decision` fail-open codes)
**Change:**
- `process-exit` exit 1 gains `socket-dir-refused`: the socket directory of `viola run` or `viola revive`'s start refused or not creatable, before any snapshot, child or bind.
- `schemas/diag-line.v1.json` closed `process-exit`'s `detail` at 19 values, as measured at chunk 2026-10-31-exit-cause-code-catalogue; now at 20, as measured at this chunk.
- `process-exit` exit 21's list gains `socket-dir-refused`.
- The `hook-decision` fail-open codes gain `socket-dir-refused`, eight now. `viola hook statusline` was three of the seven; now three of the eight, the same three. The clause «four of the seven» on what the arm wrote before chunk 2026-10-10-budget-governor leaves.
**Why:** the word joins both `detail` enums in the schema, 19 to 20 and 7 to 8, each counted from the enum's members, and no new statusline path exists. The word is the founder's own, `inputs#I2` Q1, live, 2026-11-01 before 16:27 Z, through the overseer's dialog, given after the widening was shown to him; these rows read as inside it on the answer card's confirm, the overseer's as the founder's delegate, PROVISIONAL, superseded by the founder's own word in the batch at Epoch 7's boundary.
**Ref:** .andromeda/runs/2026-11-01T17-25-39-wrap/

## 2026-11-01-unix-endpoint-and-home-hardening — `server-verify-failed` reaches macOS, as an euid mismatch alone
**Section:** §6 Log Coverage (log-coverage.md: the CLI's and the hook's lines); §5 Metric Coverage (metric-coverage.md: `viola.wait.outcome`); §1 Obs Scope Summary (obs-scope-summary.md: the Vector 4, multi-OS and Founder Direction 4 triggers); §2 Telemetry Strategy (telemetry-strategy.md: the Vector 4 row); §4 Span / Trace Coverage (obs-plan.md: the `wait` / `last` line)
**Change:**
- The CLI's eight frame verbs and `viola hook` were a server that is not the recorded wrapper or a check that cannot be made, on Linux and Windows; now also a server of another euid (Unix), on Linux, Windows and macOS, where it means an euid mismatch alone.
- The metric row, the Vector 4 trigger and row, and the Founder Direction 4 trigger read the same reach; MCP's `wait` tool still writes no line.
- The multi-OS trigger's peer-pid read keeps macOS none (`SERVER_PID_REPORTED` false, that verdict `Matches` with no comparison), and adds the peer euid: Unix `peer_creds`, a mismatch reading `server-verify-failed`, on macOS its one meaning; Windows none (`PEER_EUID_REPORTED` false, the DACL and the pid comparison the controls). Was no `server-verify-failed` line on macOS, a recorded gap owned by "Unix endpoint and home hardening"; now the named carve-out, owned by no route entry.
- The master's `wait` / `last` line reads the recorded-wrapper case on Linux and Windows and the other-euid case on Linux and macOS, macOS comparing no peer pid.
**Why:** a refused euid is built as `CallError::ServerUnverified` at `connect` and `NotSent::ServerUnverified` at the hook, so the client reads this word on Linux and macOS. It is the founder's own word, `inputs#I2` Q2, live, 2026-11-01 before 16:27 Z, through the overseer's dialog, given after the widening was shown to him; these rows read as inside it on the answer card's confirm, the overseer's as the founder's delegate, PROVISIONAL, superseded by the founder's own word in the batch at Epoch 7's boundary.
**Kept:** macOS compares no peer pid, a true claim: the Vector 4 row keeps its «recorded gap» wording beside the new reach.
**Ref:** .andromeda/runs/2026-11-01T17-25-39-wrap/

## 2026-11-01-unix-endpoint-and-home-hardening — a refused socket directory is no strict-modes failure
**Section:** §6 Log Coverage (log-coverage.md: the filesystem refusals); §1 Obs Scope Summary (obs-scope-summary.md: the Vector 7 trigger); §2 Telemetry Strategy (telemetry-strategy.md: the Vector 7 row); §1 Obs Scope Summary (obs-plan.md: the Vector 7 trigger)
**Change:**
- `socket-dir-refused` reads a closed `detail` code on `process-exit` and `hook-decision`: the socket directory of a start, or a recorded endpoint's directory, refused as a symlink, as no directory, for another owner or for a foreign mode bit.
- `check_socket_dir` takes the strict-modes read verdict yet writes under its own word: one `process-exit{subject:"self", exit_code:1}` at `viola run` and `viola revive`'s start, one `process-exit{exit_code:21, during:"connect"}` at WARN with no `channel-request` at the eight CLI frame verbs, one `hook-decision` at WARN at `viola hook`; only `viola mcp`'s `frame` writes no line, `isError` its record.
- The Vector 7 trigger counts it outside the strict-modes line sites, and the Vector 7 row's §10 exemptions name it as no such exemption. The master reads: its line, where it has one, carries its own `detail`.
**Why:** `socket_dir_verdict` ends in `unix_read_verdict`, and the rows reading such refused checks as lineless would misstate these lines. The word is the founder's own, `inputs#I2` Q1, live, 2026-11-01 before 16:27 Z, through the overseer's dialog, given after the widening was shown to him; these rows read as inside it on the answer card's confirm, the overseer's as the founder's delegate, PROVISIONAL, superseded by the founder's own word in the batch at Epoch 7's boundary.
**Ref:** .andromeda/runs/2026-11-01T17-25-39-wrap/

## 2026-11-01-unix-endpoint-and-home-hardening — the frame verbs' socket-directory refusal and its line
**Section:** §5 Metric Coverage (metric-coverage.md: `viola.wait.outcome`); §1 Obs Scope Summary (obs-scope-summary.md: the `wait` / `last` and dialog paths' fields); §4 Span / Trace Coverage (span-trace-coverage.md: the short-lived verbs, `send`, `wait` / `last`, the dialog and `pause` / `release` scenarios; obs-plan.md: the `wait` / `last` line)
**Change:**
- A frame verb (`send`, `wait`, `last`, `answer`, `pause`, `release`, `link`, `unlink`) meeting a refused socket directory writes one `process-exit{exit_code:21, detail:"socket-dir-refused", during:"connect"}` at WARN inside its dispatch span, no `channel-request` line and no frame. MCP's tools write no line.
- The check lives in `cmd::client::connect`, on the recorded endpoint's parent, and changes no mode: a refused directory stays as found.
- `send.client`'s and `wait` / `last`'s exit-21 words gain the word; the exit-21 `during` `connect` list gains it, `endpoint-lost` alone staying `call`; `wait.block` also closes on this refusal.
- `send`, `answer` and `pause` / `release` each gain a refusal item with no `channel-request` / `channel-response` line.
- The metric row counts this line by the Counter row's `detail` grouping; the dialog path's `detail` field and the `wait` `exit_code` field carry the word.
**Why:** `connect` keeps its 8 callers, and the same check at each writes this line. The word is the founder's own, `inputs#I2` Q1, live, 2026-11-01 before 16:27 Z, through the overseer's dialog, given after the widening was shown to him; these rows read as inside it on the answer card's confirm, the overseer's as the founder's delegate, PROVISIONAL, superseded by the founder's own word in the batch at Epoch 7's boundary.
**Ref:** .andromeda/runs/2026-11-01T17-25-39-wrap/

## 2026-11-01-unix-endpoint-and-home-hardening — `run`'s start refusals and the `human::refuse` census
**Section:** §11 Obs Anti-Patterns (obs-anti-patterns.md: the print-macro ban); §3 Observability Harness Contract (obs-plan.md: the `run` bullet)
**Change:**
- The census of `human::refuse(` call sites was `run` 7; now `run` 8, in both files, with `verify` 6, `list` 2, `revive` 1, `plugin install` 1 and `client` 1 unmoved. The master's re-derivation was at chunk 2026-10-26-home-and-code-bearing-file-integrity; now at this chunk.
- The master's start refusals gain a socket directory `socket_dir_ready` refuses (Unix, exit 1, before any snapshot, child or `Server::bind`).
- `viola revive`'s three added refusals were counted on `run`'s five; now on `run`'s six.
**Why:** `refuse_socket_dir` adds the one site, `human::refuse(` under `src/` moving 18 to 19 by the chunk's grep; `revive`'s start shares the refusal.
**Ref:** .andromeda/runs/2026-11-01T17-25-39-wrap/

## 2026-11-01-unix-endpoint-and-home-hardening — the OS call sites the new checks add
**Section:** §1 Obs Scope Summary (obs-scope-summary.md: the OS APIs entity)
**Change:**
- The server check's reads were peer-pid and start-time; now peer-euid (Unix), peer-pid and start-time, a failed check logged as `server-verify-failed` by the CLI and the hook and by no line on MCP.
- `check_socket_dir` (viola's one `symlink_metadata`, off Unix no read) is a call site, its refusal written under `socket-dir-refused`, MCP by no line.
- The wrapper's read of a client's euid in `accept_loop` writes no line.
**Why:** the chunk adds `peer_euid_of` on both sides and `check_socket_dir` at the start, the client seams and the hook. The words are the founder's own, `inputs#I2` Q1 and Q2, live, 2026-11-01 before 16:27 Z, through the overseer's dialog, given after the widening was shown to him; these rows read as inside them on the answer card's confirm, the overseer's as the founder's delegate, PROVISIONAL, superseded by the founder's own word in the batch at Epoch 7's boundary.
**Ref:** .andromeda/runs/2026-11-01T17-25-39-wrap/

## 2026-11-01-unix-endpoint-and-home-hardening — Founder Direction 4: the socket-directory carve-out retired, macOS's named and unowned
**Section:** §1 Obs Scope Summary (obs-scope-summary.md: the cli surface note, the Founder Direction 4 trigger); §2 Telemetry Strategy (telemetry-strategy.md: the Founder Direction 4 row)
**Change:**
- The cli note was two carve-outs, the socket-directory cause and macOS's server check, both with the route entry "Unix endpoint and home hardening"; now a refused socket directory has its own code and writes its line, and macOS comparing no peer pid is the one named carve-out, owned by no route entry, where `server-verify-failed` means an euid mismatch alone.
- The trigger's distinct codes gain `socket-dir-refused` on exit 21 and on exit 1.
- The trigger was: the socket-directory cause has no code (`bind_endpoint`'s `create_private_dir` failure an anyhow error at the catch site), with an OPEN leaving its code to that entry. Now the cause has its code whatever failed, a refused check, a failed create or a lost race reading that one word; the OPEN is gone.
- The row sets it outside the strict-modes exception: the logged `detail` `socket-dir-refused`, no path, pid or uid, exit 21 at the eight CLI frame verbs and exit 1 at `run` and `revive`'s start.
**Why:** the check and code owed to that route entry are built, and only macOS's pid gap stays. One code for a failed create and a refused check is a lean the P5 word took, `inputs#I3`, the overseer's word as the founder's delegate, PROVISIONAL. The new word and macOS's reading are the founder's own, `inputs#I2` Q1 and Q2, live, 2026-11-01 before 16:27 Z, through the overseer's dialog, given after the widening was shown to him; these rows read as inside them on the answer card's confirm, the overseer's as the founder's delegate, PROVISIONAL, superseded by the founder's own word in the batch at Epoch 7's boundary.
**Ref:** .andromeda/runs/2026-11-01T17-25-39-wrap/

## 2026-11-01-unix-endpoint-and-home-hardening — the hook's timed path holds both new checks, no perf value moved
**Section:** §1 Obs Scope Summary (obs-scope-summary.md: the hook notes, the latency budget); §10 SLO Invariants & Telemetry Budgets (slo-invariants-telemetry-budgets.md: the spine hooks row)
**Change:**
- The hook notes add: the socket-directory check and the peer-euid check on the hook `connect` sit in the timed path too, the rows under `SPINE_DEADLINE` and at or under the recorded band with them, as measured at this chunk's implement outcome.
- The row's timed path was the server check; now the socket-directory check on the recorded endpoint's directory, the peer euid decision on Unix, then the server check. Its fail-open was `server-verify-failed`; now `hook-decision{detail:"socket-dir-refused"}` on the directory and `hook-decision{detail:"server-verify-failed"}` on the euid or the server, exit 0.
- The row's macOS clause was no comparison; now no pid comparison.
**Why:** the chunk's `run --perf` read a mean of 2.0, 2.0, 2.0, 2.0 and 2.3 ms over the five hook rows and a highest max of 2.8 ms, under `SPINE_DEADLINE`, both checks timed; the predicted rise did not show. The row's checks and fail-open words follow the founder's own word, `inputs#I2` Q1 and Q2, live, 2026-11-01 before 16:27 Z, through the overseer's dialog, given after the widening was shown to him; that row reads as inside them on the answer card's confirm, the overseer's as the founder's delegate, PROVISIONAL, superseded by the founder's own word in the batch at Epoch 7's boundary.
**Kept:** the band and its as-measured cites stay, max 2.5 / 2.4 / 2.4 / 2.4 / 3.3 ms and max 2.4–3.3 ms, a true record of earlier readings; the row keeps «the recorded gap» for macOS, a true claim.
**Ref:** .andromeda/runs/2026-11-01T17-25-39-wrap/

## 2026-11-01-unix-endpoint-and-home-hardening — the wrapper's drop of a refused peer writes no line
**Section:** §1 Obs Scope Summary (obs-scope-summary.md: the ipc-internal notes); §4 Span / Trace Coverage (span-trace-coverage.md: the surface table); §2 Telemetry Strategy (telemetry-strategy.md: the Structured Logs row); §6 Log Coverage (obs-plan.md: IPC invocation)
**Change:**
- A stream whose peer euid is refused is dropped on the accept thread (`accept_loop`, filtering each accepted stream by `peer_admitted`), before its worker thread and before any frame. The peer reads end-of-file.
- No span and no line, by rule: no frame exists to key a line on and `channel-response`'s `detail` is closed. Nothing reaches the outer terminal and the wheel does not move.
- The Structured Logs row's every-refusal coverage now excepts this peer and MCP's `frame` on a refused socket directory or server euid, `isError` its record.
**Why:** the wrapper's decision is built and tested over an injected decision; the drop writing no line is a lean the P5 word took, `inputs#I3`, the overseer's word as the founder's delegate, PROVISIONAL.
**Ref:** .andromeda/runs/2026-11-01T17-25-39-wrap/

## 2026-11-01-unix-endpoint-and-home-hardening — the start's socket-directory step, no span and one line on a refusal
**Section:** §1 Obs Scope Summary (obs-scope-summary.md: the `viola run` start path's fields); §4 Span / Trace Coverage (span-trace-coverage.md: the start scenario's must-trace spans and exit-1 `detail`)
**Change:**
- The start's exit-1 `detail` codes gain `socket-dir-refused`, on `run` and on a start made by `viola revive`: one `process-exit{subject:"self", exit_code:1}` before any snapshot, child or bind, no path or pid.
- A fourth start step carries no span and writes a line only when it refuses: `socket_dir_ready` checks the endpoint's parent, creates it only when absent, then checks again, so a failed create, a lost race and a refused check read one code.
- A `Server::bind` failure is not this refusal and keeps its own outcome.
**Why:** `Bound` gives `bind_endpoint` a third outcome and `start` refuses through `refused_with` before any snapshot, child or `Server::bind`. The word is the founder's own, `inputs#I2` Q1, live, 2026-11-01 before 16:27 Z, through the overseer's dialog, given after the widening was shown to him; these rows read as inside it on the answer card's confirm, the overseer's as the founder's delegate, PROVISIONAL, superseded by the founder's own word in the batch at Epoch 7's boundary.
**Ref:** .andromeda/runs/2026-11-01T17-25-39-wrap/

## 2026-11-01-unix-endpoint-and-home-hardening — a refused socket directory at `viola hook`
**Section:** §1 Obs Scope Summary (obs-scope-summary.md: the Vector 4 trigger); §4 Span / Trace Coverage (span-trace-coverage.md: the hook's server check and socket-directory check, the dialog scenario's fail-open paths); §2 Telemetry Strategy (telemetry-strategy.md: the Vector 4 row)
**Change:**
- The logged fail-open paths gain a refused socket directory, which, though its check shares the read verdict, is no silent path: one `hook-decision{detail:"socket-dir-refused"}` at WARN inside `hook.handle`; exit 0, empty streams, no frame.
- On the dialog path `hook-invoked{corr:null}` comes first, then the decision with `decision_emitted:false`, with no `hook.dialog` frame and so no `dialog-raised`.
- `connect`'s chain was `read_snapshot_or_replay`, `connect_by`, `verify_server`; now `read_snapshot_or_replay`, `check_endpoint_dir`, `connect_by`, the euid decision `peer_admitted(client.server_euid())`, `verify_server`. A refused euid reads `server-verify-failed`, no `channel.request`, fail open.
**Why:** `NotSent::SocketDir` and `Asked::SocketDir` are built into the hook's `connect`. One `hook-decision` line as the record is a lean the P5 word took, `inputs#I3`, the overseer's word as the founder's delegate, PROVISIONAL. The word is the founder's own, `inputs#I2` Q1, live, 2026-11-01 before 16:27 Z, through the overseer's dialog, given after the widening was shown to him; these rows read as inside it on the answer card's confirm, the overseer's as the founder's delegate, PROVISIONAL, superseded by the founder's own word in the batch at Epoch 7's boundary.
**Ref:** .andromeda/runs/2026-11-01T17-25-39-wrap/

## 2026-11-01-unix-endpoint-and-home-hardening — the socket-directory refusal's PII reading and clean-body witnesses
**Section:** §8 PII Scrubbing & Compliance (pii-scrubbing-compliance.md: the refusal bullets, the error-body check, the live witnesses)
**Change:**
- A socket-directory refusal carries no path, pid or uid in its lines and documents, and its human pair names only the parsed `ViolaName`: `tests/security_negatives_socket_dir.rs` reads them with `assert_names_nothing` and `digit_runs`, on Linux and macOS (the file is `cfg(unix)`, with no case on Windows).
- The live witnesses of clean error bodies were three; now four, with `cli_error_documents_of_a_loosened_socket_directory_hold_no_path_or_upstream_text` in `tests/cli_error_bodies.rs`.
**Why:** the new binary's cases ran green on `ubuntu-latest` and `macos-latest`, it holds no Windows case, and the new `tests/cli_error_bodies.rs` case calls `assert_error_body_clean`.
**Kept:** the server-verification refusal's bullet, on Linux and Windows with its file compiled out on macOS: those negatives stay compiled out there.
**Ref:** .andromeda/runs/2026-11-01T17-25-39-wrap/

## 2026-11-01-unix-endpoint-and-home-hardening — the checks' order before any frame, by span scenario
**Section:** §4 Span / Trace Coverage (span-trace-coverage.md: the short-lived verbs' server check, `send`, `wait` / `last`, the dialog and `pause` / `release` scenarios)
**Change:**
- The CLI order before any frame was the snapshot comparison; now `check_endpoint_dir` on the recorded endpoint, the connect, the euid decision `peer_admitted(client.server_euid())`, then `verify_server`, ending in `server-verify-failed` on a refused euid, a mismatch or a check that cannot be made.
- Each scenario's parenthesis was Linux and Windows, macOS on the recorded gap; now the peer pid and its start time on Linux and Windows, `server-verify-failed` on Linux, Windows and macOS, on macOS an euid mismatch alone, macOS comparing no peer pid, its named carve-out.
- `send`, `wait` / `last`, the dialog path and `pause` / `release` state a refused directory or euid opening no `channel.request`.
- `send`'s client-side server-verification item, `send.client`'s exit-21 word and `wait` / `last`'s client `process-exit` read that reach, `server-verify-failed` for a process of another euid or not the recorded wrapper.
**Why:** `src/cmd/client.rs` `connect` runs the checks in this order for all 8 callers, and the hook and MCP hold the same three. The words are the founder's own, `inputs#I2` Q1 and Q2, live, 2026-11-01 before 16:27 Z, through the overseer's dialog, given after the widening was shown to him; these rows read as inside them on the answer card's confirm, the overseer's as the founder's delegate, PROVISIONAL, superseded by the founder's own word in the batch at Epoch 7's boundary.
**Ref:** .andromeda/runs/2026-11-01T17-25-39-wrap/

## 2026-11-01-unix-endpoint-and-home-hardening — the client euid check's place at the three seams
**Section:** §4 Span / Trace Coverage (span-trace-coverage.md: the client euid check)
**Change:**
- Each client seam (`src/cmd/client.rs` `connect`, `src/cmd/hook.rs` `connect`, `crates/viola-mcp/src/tools.rs` `frame`) reads the server's euid after the connect and before the pid comparison, through `Client::server_euid` / `TokioClient::server_euid`, which write and log nothing.
- `peer_admitted` refuses a peer of another euid, and a peer it cannot read, where euids are reported (`PEER_EUID_REPORTED`, Unix). On Windows no euid is read, and the DACL and the pid comparison stay the controls.
**Why:** `peer_euid_admitted` is `!reported || observed == Some(own)` and `PEER_EUID_REPORTED` is `cfg!(unix)`, as built. The decisions are committed tests; each seam's wiring was read only once, through forced refusals in the run, as one account can make no foreign peer.
**Ref:** .andromeda/runs/2026-11-01T17-25-39-wrap/

## 2026-11-01-unix-endpoint-and-home-hardening — the socket-directory refusal's captured-span cases
**Section:** §4 Span / Trace Coverage (span-trace-coverage.md: the refusal's place)
**Change:**
- Two captured-span cases assert the place: `connect_refuses_a_loosened_socket_directory_inside_its_dispatch_span` (`src/cmd/client.rs`), which opens the span by hand (`wait.block`) around `connect`, where the check sits, and `hook_refuses_a_loosened_socket_directory_inside_hook_handle` (`src/cmd/hook.rs`).
- Their span recorder `Seen` stands in the test module `refused_seen` of `src/cmd/client.rs`, out of the macOS-gated `refused_inside`.
**Why:** the recorder moved so the two new cases that run on macOS can read it; the check sits in `connect`, whose eight callers are unedited, so the client case opens its span by hand.
**Ref:** .andromeda/runs/2026-11-01T17-25-39-wrap/

## 2026-11-01-unix-endpoint-and-home-hardening — the socket-directory refusal's human pair
**Section:** §4 Span / Trace Coverage (span-trace-coverage.md: the human pair)
**Change:**
- The pair is named by pointer to design-system §Surface: cli, printed on start and frame-verb refusals in plain ASCII with `hint:` last and no SGR, naming only the parsed `ViolaName`.
**Why:** the pair is new with the chunk. Its wording is a planning form, approved by `inputs#I3`, the overseer's word as the founder's delegate, PROVISIONAL, the founder's wording owed at Epoch 7's boundary batch.
**Kept:** the wording itself and any status of it stay out of this file, the disposition naming the pair by pointer alone.
**Ref:** .andromeda/runs/2026-11-01T17-25-39-wrap/

## 2026-11-01-unix-endpoint-and-home-hardening — OTel SDK init: the `cli` chain after the log opens
**Section:** §3 Observability Harness Contract (registries/contracts/obs-plan/otel-sdk-init.md: the `cli` per-role anchor)
**Change:**
- After a passed instance check the log opens with `process-start`, followed by the liveness pre-check and the server check; now the liveness pre-check, the socket-directory check on the recorded endpoint (refused: `process-exit{exit_code:21, detail:"socket-dir-refused", during:"connect"}` at WARN, no frame), the peer euid decision and the server check.
**Why:** `src/cmd/client.rs` `connect` runs `check_endpoint_dir`, `Client::connect`, `peer_admitted(client.server_euid())`, then `verify_server`, for all 8 callers. The words are the founder's own, `inputs#I2` Q1 and Q2, live, 2026-11-01 before 16:27 Z, through the overseer's dialog, given after the widening was shown to him; this row reads as inside them on the answer card's confirm, the overseer's as the founder's delegate, PROVISIONAL, superseded by the founder's own word in the batch at Epoch 7's boundary.
**Ref:** .andromeda/runs/2026-11-01T17-25-39-wrap/
