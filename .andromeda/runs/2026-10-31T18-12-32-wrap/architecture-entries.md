## 2026-10-31-exit-cause-code-catalogue — exit 21's closed cause set and how each client reaches it
**Section:** Conventions (conv-refusal-reason, conv-exit-codes); Established Decisions ([MCP]); Standard Contracts (CLI `--json` output); Infrastructure Patterns → Project directory structure (`client.rs`, `viola-core`)
**Change:**
- conv-refusal-reason gains `viola_core::UnreachableCause` (`StrictModes` · `InstanceDead` · `UnwrappedName` · `EndpointLost` · `ServerVerify`, each word through `as_str`), exit 21's closed cause set: no serde path, the two clients write every word, and it is no `RefusalReason`.
- conv-exit-codes gains the CLI mapping: `client.rs` `answer_of` reads `Connect` as `Dead` and `Closed | Io` as `Lost`; the six no-endpoint arms (`send`, `wait`, `last`, `answer`, `pause`, `link`; `release` and `unlink` through `wheel_call` and `link_call`) read `Unreachable::without_endpoint`.
- conv-exit-codes gains the human hints: `src/human/hints.rs` holds the unwrapped-name and endpoint-lost hints, each last on stderr and plain ASCII; `hint_cause` keys `not-running` · `unwrapped-name` · `endpoint-lost` · `server-verify`.
- [MCP] gains `result::unreachable(cause)`: `of_reply` reads `Connect` as `InstanceDead` and `Closed | Io` as `EndpointLost`, `of_board` reads the board's refusal as `StrictModes`, and `tools.rs` `frame` reads a refused `check_instance` as `StrictModes`, no `live_endpoint` by `without_endpoint` and a failed `server_verified` as `ServerVerify`, each with no frame and no line, `isError` the record.
- Standard Contracts gains the writer: `client::unreachable_document` is the one writer of the exit-21 CLI document; `refuse_untrusted` and `src/cmd/plugin.rs` write `StrictModes` through it, and `list.rs`'s `refused_document` builds `strict-modes-failed` as a `Value` for `board_document`.
- Project directory structure: the closed `Unreachable` was `instance-dead` · `server-verify-failed`; now `Dead` · `Unwrapped` · `Lost` · `ServerUnverified`, each an `UnreachableCause` through `cause()`. `client.rs`'s comment adds `unreachable_document` and its refused check's document names `strict-modes-failed`; `viola-core`'s adds `UnreachableCause`.
**Why:** the one exit-21 document could not tell its causes apart; each is now a closed word with one CLI writer. No `RefusalReason` and no exit code was added. The cause crossing to a same-user driver is the founder's own word, live, through the overseer's dialog (`inputs#I2` Q1). The rows the contradiction check raised here resolved to apply on `inputs#I6` Q2, the overseer as delegate, PROVISIONAL.
**Kept:** `not-running` stays the hint cause's key, not renamed to the `instance-dead` the log reads; exit 1's document stays `refused_document`'s.
**Ref:** .andromeda/runs/2026-10-31T18-12-32-wrap/

## 2026-10-31-exit-cause-code-catalogue — `stale-heartbeat`, a stale holder's own code
**Section:** Conventions (conv-exit-codes); Established Decisions ([Session Liveness])
**Change:**
- `viola run`'s collision refusal: was `detail:"already-live"` for a live or a stale holder; now `already-live` for a live holder or a fresh replayed beat and `stale-heartbeat` for a stale holder, the human pairs as they stood.
- `viola revive`'s first preflight reading: was `detail:"already-live"`; now `already-live` or `stale-heartbeat`.
- [Session Liveness] gains `collision_check`'s two codes, printed and logged, and that stale heartbeat is exit 1's cause alone: a frame verb against a stale instance reads `instance-dead`, `live_endpoint` giving none for a stale wrapper.
**Why:** each exit-1 cause carries its own code, and `already-live` no longer stands for a stale holder. The [Session Liveness] row the contradiction check raised resolved to apply on `inputs#I6` Q2, the overseer as delegate, PROVISIONAL.
**Kept:** the `run.collision_check` span's `outcome` reads `already-live` for a live or a stale holder: its closed set says a holder stands, and it did not move.
**Ref:** .andromeda/runs/2026-10-31T18-12-32-wrap/

## 2026-10-31-exit-cause-code-catalogue — `viola revive --json` documents every refusal of its start
**Section:** Conventions (conv-exit-codes); Established Decisions ([Session Liveness], [CLI Conventions]); Occupied Resources (Binaries); Standard Contracts (CLI `--json` output, body and detail)
**Change:**
- The exit-1 `{"v":1,"error":"<code>","detail":null}` document: was `revive --json` refusing its strict-modes check or its preflight, five codes; now also the start it shares with `run`, ten codes: `strict-modes-failed` · `already-live` · `no-session` · `session-live` · `cwd-missing` · `stale-heartbeat` · `batch-script-child` · `pinned-hash-mismatch` · `squatted-name` · `internal-error` (Occupied Resources: was one of the five codes; now one of the ten).
- The sentence that a refusal `run`'s start makes after a passed preflight prints no document, owed to the working-route entry "Exit-cause code catalogue", is gone from the body, conv-exit-codes and [CLI Conventions].
- [CLI Conventions] states a `claude` not on `PATH` or an anyhow error of the start as `internal-error`. Binaries' exit 1 adds a refusal of the start after a passed preflight.
- [Session Liveness]: a refused home-tree check under `viola revive --json` prints the `strict-modes-failed` document in place of either pair.
- Standard Contracts gains `Launch`'s `json: bool` (`run` false, `revive` `args.json`): with it true, `refused_with` prints `refused_document` in place of the human pair for `batch-script-child`, `pinned-hash-mismatch` and `squatted-name`, and a program not found, which has no pair, prints `internal-error`, each before `spawn_child`.
- Standard Contracts gains: an anyhow error from `trusted`, `checked_exe`, `open_wrapper_log`, `preflight` or `start` prints the `internal-error` document and returns the same error, so the catch site logs its chain and its line.
**Why:** the post-preflight gap, carried since chunk 2026-10-17-cli-machine-contract, is closed. `run` has no `--json`, so its causes reach the log and the hint, its start also through `revive --json`: the founder's own word, live, through the overseer's dialog, claiming with named carve-outs (`inputs#I2` Q4). The rows the contradiction check raised here resolved to apply on `inputs#I6` Q2, the overseer as delegate, PROVISIONAL.
**Kept:** a `pump_child` error prints no document, because the child has held the terminal; a document from `main` at `Started` time was rejected for that reason.
**Ref:** .andromeda/runs/2026-10-31T18-12-32-wrap/

## 2026-10-31-exit-cause-code-catalogue — the `cli` catch site's `internal-error` document
**Section:** Conventions (conv-exit-codes); Standard Contracts (CLI `--json` output, body); Infrastructure Patterns → Build system
**Change:**
- The body gains `{"v":1,"error":"internal-error","detail":null}` (exit 1): a `cli` verb at the `main` catch site with `--json` among the arguments before a `--`, printed on stdout by `cmd::internal_error_document` in place of `error: internal error`, stderr empty.
- conv-exit-codes states that document at the catch site's line and gains how it is read: `src/main.rs` `json_asked`, from argv before a `--` as `role_of` reads argv; where `prints_internal_error` is true it prints `src/cmd/mod.rs` `internal_error_document` in place of the human line.
- Build system: the catch site was `error: internal error` for every `cli` verb; now that line without `--json`, and under `--json` the document on stdout in its place, stderr empty.
**Why:** a driver asking for `--json` gets a document at the catch site, exit 1 and the human line staying: the founder's own word, live, through the overseer's dialog (`inputs#I2` Q1). The conv-exit-codes row the contradiction check raised resolved to apply on `inputs#I6` Q2, the overseer as delegate, PROVISIONAL.
**Kept:** without `--json` the human line stands; `run`, `revive`'s start, `mcp` and `hook` stay silent at that site.
**Ref:** .andromeda/runs/2026-10-31T18-12-32-wrap/

## 2026-10-31-exit-cause-code-catalogue — the exit-21 document carries its cause in `detail`
**Section:** Conventions (conv-exit-codes, conv-mcp-results); Established Decisions ([Message Broker / IPC], [MCP]); Occupied Resources (Binaries, Filesystem); Standard Contracts (channel methods, CLI `--json` output, body and detail)
**Change:**
- The body: was `{"v":1,"error":"instance-unreachable","detail":null}`; now `"detail":"<cause>"`, one of `strict-modes-failed` · `instance-dead` · `unwrapped-name` · `endpoint-lost` · `server-verify-failed`, written by `client::unreachable_document`. Standard Contracts' detail gains the five closed causes.
- `viola plugin install`'s refused check: was the `instance-unreachable` document through `client::refused_document`; now with `detail` `strict-modes-failed` through `client::unreachable_document`.
- A refused strict-modes check: was `detail: null`; now `strict-modes-failed`, on the frame verbs, `link` / `unlink` and the board (conv-exit-codes, Binaries, Filesystem, Standard Contracts) and on MCP's frame tools (conv-mcp-results, [MCP], Filesystem). MCP `list`'s board refusal names it too, its only one.
- A failed server check: was the same null document; now `detail` `server-verify-failed`. The cause was without exit code, `--json` code or `RefusalReason` of its own; now without exit code or `RefusalReason`, its `--json` word the document's `detail`.
- conv-mcp-results: `instance-unreachable` was `detail: null`; now `detail` its cause word, the five.
- [Message Broker / IPC]: a `wait` whose endpoint vanishes reads `instance-unreachable` with `detail` `endpoint-lost`.
**Why:** a same-user driver can now tell exit 21's causes apart. The founder's own word, live, 2026-10-31 before 16:52Z, through the overseer's dialog, given after this widening was shown to him (`inputs#I2` Q1). The overseer's confirm, as delegate, that these rows lie inside it is `inputs#I6`, PROVISIONAL; the ratification stays the founder's. Standard Contracts' row of the five causes, which the contradiction check raised, resolved to apply on `inputs#I6` Q2, PROVISIONAL.
**Kept:** `v` stays 1, `error` stays `instance-unreachable` on all five, and MCP's result stays the document less `v`, its text `refused: instance-unreachable`.
**Ref:** .andromeda/runs/2026-10-31T18-12-32-wrap/

## 2026-10-31-exit-cause-code-catalogue — the socket-directory and macOS carve-outs
**Section:** Conventions (conv-exit-codes)
**Change:**
- Gains an absence: the socket-directory refusal has no code, `bind_endpoint`'s `create_private_dir` failure being an anyhow error at the catch site. OPEN, its code PROVISIONAL, owner the route entry "Unix endpoint and home hardening".
- Gains an exception: `server-verify-failed` is claimed off macOS only; `v1-18`'s named carve-out leaves macOS's server check with that route entry.
**Why:** `v1-18`'s requirement lists a socket-directory refusal the code does not hold, as measured for the chunk. Both stay with "Unix endpoint and home hardening": the founder's own word, live, through the overseer's dialog, claiming with named carve-outs (`inputs#I2` Q4). Leaving the socket-directory code out of this chunk is the row of `inputs#I3`, delegate the overseer, PROVISIONAL, and its mark stands at that item.
**Ref:** .andromeda/runs/2026-10-31T18-12-32-wrap/

## 2026-10-31-exit-cause-code-catalogue — `viola verify`'s refusal codes on its `process-exit`
**Section:** Conventions (conv-exit-codes); Established Decisions (dec-cli-version-compatibility); Occupied Resources (Binaries, Filesystem)
**Change:**
- Conventions, the decision and Filesystem each gain: the `process-exit` `verify` writes under a parsed `VIOLA_NAME` carries its `Ended::Refused` code, `batch-script-child` · `program-not-found` · `pinned-hash-mismatch` · `version-unreadable` · `strict-modes-failed` · `unclean-fixture`, beside `interrupted`.
- Each also states what stays without one: a refused home tree writes no line, a failing row is no refusal, the fixture's `<file>` enters no code, and `verify` takes no `--json`.
- Binaries: `verify`'s home-state exit was before its `cli` log and any step line for both refusals; now that holds of `check_home_tree`'s refusal alone, and `update_stamps`' refusal of `ledger/stamps.json` carries `detail:"strict-modes-failed"` on its `process-exit` under a parsed `VIOLA_NAME`.
**Why:** `verify` has no `--json`, so its causes reach the log and the hint: the founder's own word, live, through the overseer's dialog (`inputs#I2` Q4). The Binaries rewrite, a row no companion carried, is `inputs#I6` Q8, the overseer as delegate, PROVISIONAL.
**Ref:** .andromeda/runs/2026-10-31T18-12-32-wrap/

## 2026-10-31-exit-cause-code-catalogue — the code-word rule and the exit-cause table
**Section:** Cross-cutting Patterns (Sanitised error surfaces); Occupied Resources (Filesystem, test-only)
**Change:**
- Sanitised error surfaces gains: every exit-21 cause and exit-1 code passes `is_code_word` and is a row of `tests/contract_error_bodies.rs`, carrying no path, pid or upstream text.
- Occupied Resources gains the test-only `tests/cross_exit_causes.rs`, the exit-cause table, a root `cross_` binary, with four checks over the table itself: `no_two_codes_share_a_hint`, `shared_hints_reads_a_planted_duplicate`, `no_hint_names_release` and `every_code_is_a_code_word`.
**Why:** the chunk's new words each pass `is_code_word` and stand as named rows, and the table holds each cause to its code and its hint, green at the chunk's report.
**Kept:** the table's row states no test count.
**Ref:** .andromeda/runs/2026-10-31T18-12-32-wrap/

## 2026-10-31-exit-cause-code-catalogue — `without_endpoint`, the no-endpoint reading
**Section:** Established Decisions ([Session Liveness], its values and pending); Standard Contracts (Session liveness); Infrastructure Patterns → Project directory structure (`viola-state`)
**Change:**
- [Session Liveness] and Session liveness gain `viola_state::liveness::without_endpoint`: where a client has no live endpoint it reads the instance directory by `symlink_metadata`, a directory `InstanceDead`, anything else (absent, a regular file, a symlink) `UnwrappedName`. It opens nothing and writes no line, behind the strict-modes check at every caller (`client::Unreachable::without_endpoint`, `viola mcp`'s `frame`).
- Session liveness states that `live_endpoint` gives none for a stale wrapper, so a frame verb against a stale instance reads `instance-dead`.
- [Session Liveness]'s pending gains: OPEN, this reading of a missing endpoint is PROVISIONAL, owner the open decision on the unwrapped-name reading.
- `viola-state`'s comment adds the no-endpoint read beside `liveness::live_endpoint`.
**Why:** a missing endpoint now has two causes, told apart by what stands at the instance directory. That reading is `inputs#I2` Q2, delegate the overseer, PROVISIONAL, its mark standing at the pending item; it is not the founder's own answer.
**Ref:** .andromeda/runs/2026-10-31T18-12-32-wrap/

## 2026-10-31-exit-cause-code-catalogue — exit 21's causes on `process-exit`
**Section:** Occupied Resources (Filesystem, `diagnostics/`)
**Change:**
- The `cli-<name>.ndjson` frame verbs' exit 21: `detail:"instance-dead"` was with `during`; now with `during:"connect"`.
- Gains exit 21 `detail:"unwrapped-name"` + `during:"connect"`, and `detail:"endpoint-lost"` + `during:"call"` for a wrapper lost with the call open.
- Gains: `schemas/diag-line.v1.json`'s `process-exit` `detail` enum holds `stale-heartbeat`, `unwrapped-name`, `endpoint-lost`, `program-not-found`, `version-unreadable` and `unclean-fixture` among its words.
**Why:** `instance-dead` no longer carries `during:"call"`: a wrapper lost with the call open is its own cause, and each logged cause is one `process-exit` with its own `detail`, the schema admitting the six words.
**Ref:** .andromeda/runs/2026-10-31T18-12-32-wrap/
