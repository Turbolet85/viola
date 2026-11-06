@carried n=1 label=architecture/standard-contracts fact=fct-dtnnvvl3
why: v1-36's link-set-on-the-board clause must be deferred with its reason or re-worded against the `list` item shape, which holds no link set.
quote: `items` entry: `{name, wrapped, liveness, status, wheel?, budget_paused?, dialog_pending, cli_version?, cli_verified?}`.
@carried n=1 run=2 label=architecture/standard-contracts fact=fct-smoe4pw5
quote: Its `ok` payload (CLI `--json` and MCP `list`) is `{items, budget, skipped}`, with the item, `budget` and `skipped` shapes of the `list` payload above.
@carried n=1 run=3 label=architecture fact=fct-cqamt3xw
quote: its `ok` payload (CLI `--json` and MCP `list`) is `{items, budget, skipped}`
@carried n=2 label=architecture/standard-contracts fact=fct-vgnixbh3
why: the link set must stay held in the instance snapshot's `links`, the place v1-36's disposition names as where the set lives, and the live driver session's implicit link must land in it.
quote: `links` is the set derived by replaying `link`/`unlink` events, as `{driver, driven, since}`. `since` is the `link` line's `ts`.
@carried n=2 run=2 label=architecture/standard-contracts fact=fct-fr2pme2q
quote: The set changes on an implicit link and on a served `link` / `unlink` call: `LinkSet::link` adds the pair through `record`, and `LinkSet::unlink` removes it through `remove`.
@carried n=2 run=3 label=architecture fact=fct-hv3qoyp2
quote: `links` is the set derived by replaying `link`/`unlink` events, as `{driver, driven, since}`, `since` the `link` line's `ts`
@carried n=3 label=architecture/standard-contracts fact=fct-ubupynfq
why: v1-13's disposition must read the built CLI half of `link`, whose result exists only on the CLI, with no MCP result.
quote: `viola link <driver> <driven> --json` prints `{"v":1,"ok":{"since":…,"already_linked":…}}` (`since` a string, `already_linked` a boolean) and `viola unlink <driver> <driven> --json` prints `{"v":1,"ok":{}}`, each with exit 0. Their refused strict-modes check on the home and `instances/<driven>`, made before any role file or log opens, prints exactly `{"v":1,"error":"instance-unreachable","detail":"strict-modes-failed"}` on stdout (exit 21), with stderr empty and no line in any log. A refused socket directory at either verb, met in `connect` after that check, prints the same document with `detail` `socket-dir-refused` (exit 21), with one `process-exit` at WARN.
@carried n=3 run=2 label=architecture fact=fct-ax5zoxh3
quote: `link` `{driver, driven}` → `{since, already_linked}` and `unlink` `{driver, driven}` → `{}`, each sent to the driven instance's wrapper
@carried n=4 label=architecture/standard-contracts fact=fct-ekks4dfz
why: the `StampsError` divergence must take a fold or a ratification, and this contract's error text must follow whichever lands, matching the re-measured derive count.
quote: Its error is `StampsError` (`StrictModes` · `State`), each variant with a fixed message.
@carried n=4 run=2 label=architecture fact=fct-vf75c7sp
quote: `viola-mcp`'s `ServeError` beside `McpError` and `viola-state`'s `StampsError` beside `StateError` are open divergences, with `PinError`, `StampsReadError` and `Refused` the three the census adds, [Error Handling])
@carried n=5 label=architecture/standard-contracts fact=fct-ikwybol4
why: the always-admit `peer_admitted` survivor must stay a dated note and an untestable zone, with no seam added to kill it, while the client's peer euid decision keeps refusing an unreadable or foreign peer.
quote: The peer euid decision is `viola_channel`'s: `peer_euid_admitted(observed, own)` is `observed == Some(own)`, so a peer it cannot read is refused; `peer_admitted(observed)` is that decision over this process's `own_euid()`.
@carried n=5 run=2 label=architecture/standard-contracts fact=fct-2tmeycxu
quote: Before its first frame, the client of `send`, `wait`, `last`, `answer`, `pause`, `release`, `link`, `unlink`, `hook.event` and `hook.dialog`, and of the MCP `send` / `wait` / `last` / `answer` tools, reads the connected stream's peer euid (`Client::server_euid`, `TokioClient::server_euid`) and refuses another euid or one it cannot read (`peer_admitted`)
@carried n=5 run=3 label=architecture/standard-contracts fact=fct-a6y4blve
quote: so a frame-writing client takes the instance's strict-modes check first, then the liveness-only pre-check, then the socket-directory check on the recorded endpoint, then on its connected stream the peer euid decision, then the server check. The closed decision enum `ServerVerdict` (no crate error enum) is `Matches` · `Mismatch` · `Unverifiable`.
@carried n=5 run=4 label=architecture fact=fct-ld365mzq
quote: the connected peer's pid and its OS start time against these two fields, a mismatch writing no frame.
@carried n=6 label=architecture/standard-contracts fact=fct-rvenzu5e
why: the founder's word on the socket-directory pair's wording must land in every contract line naming the refused socket directory, together.
quote: Between that read and the connect each client checks the recorded endpoint's directory (`viola_state::strict::check_endpoint_dir`), and a refused directory writes no frame: `src/cmd/client.rs` `connect` (`CallError::SocketDirRefused`, one `process-exit`), the hook's `connect` (`NotSent::SocketDir`, one `hook-decision`) and MCP's `frame` (`UnreachableCause::SocketDir`, no line).
@carried n=6 run=2 label=architecture/standard-contracts fact=fct-qybkldvd
quote: A refused socket directory at any of the six, met in `connect` after that check, prints the same document with `detail` `socket-dir-refused` (exit 21), with one `process-exit` at WARN and no `channel-request`.
@carried n=6 run=3 label=architecture/standard-contracts fact=fct-dt7d6rx2
quote: The exit-21 document's `detail` is one of six closed causes: `strict-modes-failed`, `instance-dead`, `unwrapped-name`, `endpoint-lost`, `server-verify-failed` and `socket-dir-refused`; its `v` is 1 and its `error` `instance-unreachable` on all six.
@carried n=6 run=4 label=architecture/standard-contracts fact=fct-zxepdepr
quote: With it true, `refused_with` (`src/cmd/run.rs`) prints `refused_document` in place of the human pair for `pinned-hash-mismatch`, `squatted-name` and `socket-dir-refused`, and a program not found, which has no human pair, prints `internal-error`; each returns before `spawn_child`.
@carried n=6 run=5 label=architecture fact=fct-ksmkl4jd
quote: `<cause>` one of `strict-modes-failed` · `instance-dead` · `unwrapped-name` · `endpoint-lost` · `server-verify-failed` · `socket-dir-refused` (`viola_core::UnreachableCause`)
@carried n=7 label=architecture/standard-contracts fact=fct-ng4l52uh
why: the founder's word on the board's read rule must land where the board's one refusal is stated, now read for Linux alone.
quote: `src/cmd/list.rs`'s `refused_document` builds `strict-modes-failed` as a `Value` for `board_document`, the board's only refusal.
@carried n=7 run=2 label=architecture/standard-contracts fact=fct-smoe4pw5
quote: MCP `list` (chunk 2026-10-17-mcp-server-for-drivers) builds the same board through `list::board_document` (scan → read → document, the provider `viola mcp` hands `viola_mcp::serve`) and returns the document's `ok`, or `instance-unreachable` with `detail` `strict-modes-failed` for the board's refusal, its only one
@carried n=8 label=architecture/standard-contracts fact=fct-2rdzhblg
why: the live «overseer drives a builder through viola» session must drive the builder through `send` (with `from`, recording the implicit link) and `wait`, inside the budget gate, and read their payloads as stated.
quote: `send` `{text, from?}` → `{submitted_at, cursor}` when confirmed, or `{confirmed:false, detail:"unconfirmable", cursor}`. `cursor` is the end offset of `events.ndjson` when the send was accepted, before the paste. A `send` reads the wrapper's budget gate once at its arrival ([Budget Governor]); a closed gate is the refusal `budget-paused` with detail `five-hour` or `seven-day`, before `send-issued`, with no cursor, nothing typed and no flight reserved, and a gate change that cannot be recorded there answers `-32603`. Since chunk 2026-10-10-session-links a `send` whose `from` is a name other than the wrapper's own records the link `from → this instance` ([Links]) after the second wheel and running-turn reads and directly before `send-issued`
@carried n=8 run=2 label=architecture/standard-contracts fact=fct-kzj34g66
quote: `wait` `{after?, timeout_ms?, from?}` → `{event:<ndjson event line>, cursor}`, where `cursor` is the end offset of that event's line, or `{timed_out:true}`.
@carried n=8 run=3 label=architecture fact=fct-ekt3xwun
quote: a `from` naming another instance records the link before `send-issued`.
@binding n=1
**standard-contracts `list` payload ↔ established-decisions GUI Control Scope and the verification matrix (v1-36)**: v1-36's link-set clause is deferred with its reason or re-worded, on the founder's word, reading that the board's items carry no link set (architecture/standard-contracts fct-dtnnvvl3) while the set lives in the snapshot's `links` (architecture/standard-contracts fct-vgnixbh3) and the deferred 0.2.0 page's intent names one (architecture fct-7wwoxbip).
@binding n=2
**standard-contracts CLI `link` result ↔ established-decisions MCP (v1-13)**: v1-13's MCP-result-for-`link` and budget-release clause is deferred with its reason or re-worded, on the founder's word, its CLI half the built `link` / `unlink` result (architecture/standard-contracts fct-ubupynfq) and `link` and `release` staying CLI-only (architecture fct-tkwxvt3j).
@binding n=3
**standard-contracts ledger stamps ↔ established-decisions Error Handling, Conventions error types and Inherited Defaults Errors**: a fold of `StampsError` re-words the `update_stamps` error text and a ratification names it as ratified, and the re-measured `thiserror::Error` derive count agrees across all four (architecture/standard-contracts fct-ekks4dfz) (architecture fct-sha7o35z) (architecture fct-nhsg4k7g) (architecture fct-vf75c7sp).
@binding n=4
**standard-contracts peer euid decision ↔ occupied-resources IPC endpoints and test-plan §1**: the always-admit `peer_admitted` survivor stays a dated note at matrix#v1-44 and the untestable zone, no seam added, while the Linux endpoint's same-euid admission and the channel-handle no-leak measurement under `/proc/<child pid>/fd` are stated on the same endpoint (architecture/standard-contracts fct-ikwybol4) (architecture fct-wular7qm).
@binding n=5
**standard-contracts socket-directory refusals ↔ occupied-resources IPC endpoints and conventions exit codes**: the founder's word on the socket-directory pair's wording lands together in the client check, the CLI documents, the closed exit-21 causes, `run` / `revive`'s refusal, the endpoint row and the exit-code table (architecture/standard-contracts fct-rvenzu5e) (architecture/standard-contracts fct-dt7d6rx2) (architecture fct-wular7qm) (architecture fct-jdhp6aqu).
@binding n=6
**standard-contracts channel methods ↔ established-decisions Links and the live session**: the live driver session's `send` with a `from` records the implicit link before `send-issued` and its `wait` reads the builder's turns, inside the directive's two headless sessions (architecture/standard-contracts fct-2rdzhblg) (architecture fct-3iyhqr2o).
