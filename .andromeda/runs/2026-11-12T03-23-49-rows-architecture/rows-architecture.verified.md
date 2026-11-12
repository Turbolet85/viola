@row_add n=1 row=decision/mechanism-not-policy area=design-philosophy rec=fct-v73h4for@1,fct-pdkjglgn@1
name: Mechanism, not policy: a general bridge
rule: viola carries input, answers dialogs when told to, logs everything and holds the wheel, but never decides what to answer; driver-side policy and anything Andromeda-specific stay outside the binary, so every feature is a general bridge verb.
why: Decision rights belong to the driver, so viola stays reusable for any overseer/builder pair.
@row_check n=1 row=decision/mechanism-not-policy sha=6c07c3c363f6 v=holds judge=sonnet head=8834195b0683
at: src/run/wheel.rs:7
quote: //! thread, so no human byte waits on the disk. viola records who holds the wheel; it never decides
@row_add n=2 row=decision/send-confirmed-after-the-fact area=design-philosophy rec=fct-24sxrnlw@1,fct-i4ogppls@1
name: Every send confirmed after the fact
rule: A send is confirmed only by a matching `prompt-submitted` within the compiled confirmation window (never in `config.json`), or for a ledger-listed local command by its measured post-condition; otherwise `not-delivered`; the ledger decides, never a leading slash.
why: A turn boundary per the hooks does not prove the input box is ready.
except: A ledger-listed local command whose post-condition is none, not yet measured, or whose CLI is unverified returns `ok` with `confirmed: false` and detail `unconfirmable`.
@row_check n=2 row=decision/send-confirmed-after-the-fact sha=5863a44e81c2 v=holds judge=sonnet head=8834195b0683
at: src/run/send.rs:156
quote: None => ctx.not_delivered(Some(cursor), NotDelivered::NoPromptSubmitted),
at: src/run/send.rs:152
quote: return ctx.unconfirmable(cursor, slot.clock.now());
@row_add n=3 row=decision/capability-ledger area=design-philosophy rec=fct-24sxrnlw@1,fct-7vnaqpcq@1
name: Version-stamped capability ledger
rule: Any behaviour of the `claude` CLI that viola relies on is a row in a version-stamped capability ledger; a CLI version is verified only when its stamp holds every relied ledger row `pass`, and `viola verify` re-stamps it.
why: The docs lag the installed build, and several load-bearing behaviours are measured, not documented.
@row_check n=3 row=decision/capability-ledger sha=200d05902fcd v=holds judge=sonnet head=8834195b0683
at: crates/viola-agent-claude/src/ledger.rs:1237-1239
quote: Ok(LedgerRow::relied() .iter() .all(|row| rows[row.id()] == "pass"))
at: src/cmd/verify.rs:273
quote: let written = update_stamps(home, |existing| {
@row_add n=4 row=decision/human-always-wins-wheel area=design-philosophy rec=fct-5pxykib3@1
name: The human always wins the wheel
rule: Human keystrokes are never blocked, refused or delayed past the current atomic paste; automation is refused with a typed reason instead of competing with the human.
@row_check n=4 row=decision/human-always-wins-wheel sha=c0388bd7117e v=holds judge=sonnet head=8834195b0683
at: src/run/wheel.rs:111
quote: pub(crate) fn human_typing(&self) -> Option<Option<HumanTyping>> {
at: src/run/send.rs:99
quote: RefusalReason::HumanTyping,
@row_add n=5 row=decision/no-daemon-disk-is-shared-truth area=design-philosophy rec=fct-efluvsb5@1
name: No daemon; the disk is the shared truth
rule: Every wrapper owns its own local-socket endpoint, every other verb is a separate process owning no endpoint, and all state shared between processes lives in ndjson logs and atomic snapshots that survive a crash on either side.
@row_check n=5 row=decision/no-daemon-disk-is-shared-truth sha=a49d8af12bac v=holds judge=sonnet head=8834195b0683
at: src/cmd/run.rs:500
quote: match Server::bind(&endpoint) {
@row_add n=6 row=overview/stack-and-technologies area=stack-and-technologies rec=fct-zegjs5rw@3
text: viola is one native Rust binary: a std-thread core with Tokio built only inside `mcp`, clap for the CLI, portable-pty behind viola's own `pty` seam, vt100 for readiness, interprocess local sockets carrying JSON-RPC 2.0 over ndjson, ndjson append logs and atomically replaced JSON snapshots with no database, broker, container or hosting, rmcp for the stdio MCP server, and GitHub Actions on `ubuntu-latest` against the fake agent.
@row_check n=6 row=overview/stack-and-technologies sha=59efb7cfff25 v=holds judge=sonnet head=8834195b0683
at: Cargo.toml:240
quote: tokio = { version = "=1.53.3", default-features = false }
at: Cargo.toml:197
quote: clap = { version = "=4.6.7", default-features = false, features = [
at: Cargo.toml:217
quote: portable-pty = "=0.9.0"
at: Cargo.toml:248
quote: vt100 = "=0.16.2"
at: Cargo.toml:207
quote: interprocess = "=2.4.4"
at: Cargo.toml:223
quote: rmcp = { version = ">=3.4.1, <3.5", default-features = false, features = ["server", "transport-io"] }
at: crates/viola-state/src/fs.rs:83
quote: tmp.persist(path).map(|_| ()).map_err(|e| e.error.into())
at: .github/workflows/ci.yml:193
quote: runs-on: ubuntu-latest
@row_add n=7 row=decision/personal-v1-no-accounts area=established-decisions rec=fct-hcu7z4ke@2
name: Personal v1 without accounts
rule: v1 runs on the founder's own subscription with no accounts, and its on-disk state is parsed defensively.
why: The later public version for individual Claude Code subscribers must not need a rewrite.
@row_check n=7 row=decision/personal-v1-no-accounts sha=1ed0486648d0 v=unverifiable judge=sonnet head=8834195b0683
why: not a code claim
@row_add n=8 row=decision/claude-first-no-adapter-trait area=established-decisions rec=fct-7tdo3mcl@2
name: Claude-first, one agent crate, no adapter trait
rule: All Claude-specific logic (hook JSON parsing, dialog answer mapping, the identity strip, child program resolution, the Claude half of the CLI version gate) lives only in `viola-agent-claude`, which stays pure; the other layers speak only normalised events.
why: A trait designed before a second agent exists would encode guesses; isolating the agent in one crate keeps the later extraction mechanical.
@row_check n=8 row=decision/claude-first-no-adapter-trait sha=df890a388e3b v=holds judge=sonnet head=8834195b0683
at: crates/viola-agent-claude/src/lib.rs:185
quote: pub fn resolve_program(
@row_add n=9 row=decision/human-key-never-held-behind-resize area=established-decisions rec=fct-omwrdnlc@3
name: Human key never held behind a resize
rule: viola never holds, queues or re-orders a human key behind a resize.
@row_check n=9 row=decision/human-key-never-held-behind-resize sha=c973df8864e7 v=holds judge=sonnet head=8834195b0683
at: crates/viola-pty/src/pump.rs:164
quote: spawn_worker(tx, None, move || copy(input, human));
at: crates/viola-pty/src/pump.rs:189
quote: pty.resize(size)?;
@row_add n=10 row=decision/no-da1-answer area=established-decisions rec=fct-omwrdnlc@3
name: viola never answers DA1
rule: viola never answers a DA1 query itself.
@row_check n=10 row=decision/no-da1-answer sha=0683d863cd83 v=holds judge=sonnet head=8834195b0683
searched: DA1 · in src/**/*.rs, crates/viola-pty/src/**/*.rs, crates/viola-agent-claude/src/**/*.rs (only a doc comment and test cases in src/run/wheel.rs classify DA1 input; nothing writes a reply)
searched: \?[0-9;]*c" · in src/**/*.rs, crates/viola-pty/src/**/*.rs, crates/viola-agent-claude/src/**/*.rs (only test inputs in src/run/wheel.rs)
@row_add n=11 row=contract/not-delivered-details/input-not-ready area=established-decisions rec=fct-hsjl3wtp@1
boundary: not-delivered refusal details
item: input-not-ready
promise: A screen not ready at the readiness gate's bound is refused `not-delivered` with this detail, nothing typed.
@row_check n=11 row=contract/not-delivered-details/input-not-ready sha=3f1405caafae v=holds judge=sonnet head=8834195b0683
at: src/run/send.rs:129-130
quote: if readiness != Readiness::Ready { return ctx.not_delivered(None, NotDelivered::InputNotReady);
@row_add n=12 row=contract/not-delivered-details/no-prompt-submitted area=established-decisions rec=fct-i4ogppls@1
boundary: not-delivered refusal details
item: no-prompt-submitted
promise: A send with no matching `prompt-submitted` within the compiled confirmation window is refused `not-delivered` with this detail.
@row_check n=12 row=contract/not-delivered-details/no-prompt-submitted sha=cef8ad312b56 v=holds judge=sonnet head=8834195b0683
at: src/run/send.rs:156
quote: None => ctx.not_delivered(Some(cursor), NotDelivered::NoPromptSubmitted),
@row_add n=13 row=decision/send-text-line-endings area=established-decisions rec=fct-qevnbocy@1
name: Send text line-ending normalisation
rule: A `send` types its text with every CR LF pair and every other CR as one LF, and without its trailing CR and LF characters.
@row_check n=13 row=decision/send-text-line-endings sha=a4f6cbfafa18 v=holds judge=sonnet head=8834195b0683
at: crates/viola-agent-claude/src/hook.rs:232-234
quote: let text = text.trim_end_matches(['\r', '\n']); if text.contains('\r') { Cow::Owned(text.replace("\r\n", "\n").replace('\r', "\n"))
@row_add n=14 row=contract/not-delivered-details/empty-text area=established-decisions rec=fct-qevnbocy@1
boundary: not-delivered refusal details
item: empty-text
promise: A send whose typed text is left empty is refused `not-delivered` with this detail, nothing issued.
@row_check n=14 row=contract/not-delivered-details/empty-text sha=982e39431ea7 v=holds judge=sonnet head=8834195b0683
at: src/cmd/send.rs:187
quote: .then_some(NotDelivered::EmptyText)
@row_add n=15 row=decision/torn-last-line-healed area=established-decisions rec=fct-73drmylz@2
name: Torn log line healed by the next append
rule: Each event line is one `write`; the next append heals a torn last line, and readers count it and never rewrite the log.
@row_check n=15 row=decision/torn-last-line-healed sha=b8e769401c5d v=holds judge=sonnet head=8834195b0683
at: crates/viola-state/src/events.rs:335
quote: file.write_all(&[b"\n", bytes].concat())?;
@row_add n=16 row=decision/private-mode-before-write area=established-decisions rec=fct-2sdvn2pr@1
name: File mode set before any byte is written
rule: A replaced private file's temp file has its mode set before any byte is written, then is written, synced and persisted.
@row_check n=16 row=decision/private-mode-before-write sha=6c89bc9a1509 v=holds judge=sonnet head=8834195b0683
at: crates/viola-state/src/fs.rs:79-83
quote: let mut tmp = tempfile::NamedTempFile::new_in(parent)?; restrict(tmp.path(), mode)?; tmp.write_all(bytes)?; tmp.as_file().sync_all()?; tmp.persist(path).map(|_| ()).map_err(|e| e.error.into())
@row_add n=17 row=decision/pinned-copy-never-replaced area=established-decisions rec=fct-2sdvn2pr@1
name: Pinned copy never replaced
rule: The pinned copy is never replaced: an absent copy is written by a no-clobber placement of a closed temp file.
@row_check n=17 row=decision/pinned-copy-never-replaced sha=79026967058c v=holds judge=sonnet head=8834195b0683
at: crates/viola-state/src/fs.rs:118
quote: match tmp.into_temp_path().persist_noclobber(path) {
@row_add n=18 row=decision/interprocess-local-socket-channel area=established-decisions rec=fct-w2igrcse@2
name: interprocess local sockets behind the channel seam
rule: interprocess `local_socket` carries the channel, one endpoint per `viola run`, used only behind viola's own `channel` seam.
why: One API over Unix sockets, sync and Tokio flavours, and no daemon.
@row_check n=18 row=decision/interprocess-local-socket-channel sha=0d247b999a48 v=holds judge=sonnet head=8834195b0683
at: crates/viola-channel/src/server.rs:11
quote: use interprocess::local_socket::traits::ListenerExt as _;
@row_add n=19 row=decision/channel-blocks-never-polls area=established-decisions rec=fct-w2igrcse@2
name: Nothing polls the channel
rule: `hook` blocks on the channel for a dialog answer and `wait` until the next driver-relevant event, so nothing polls.
@row_check n=19 row=decision/channel-blocks-never-polls sha=68edd50fcfa3 v=holds judge=sonnet head=8834195b0683
at: src/run/wait.rs:8
quote: use std::sync::{Condvar, Mutex, MutexGuard, PoisonError};
at: crates/viola-channel/src/client.rs:100
quote: match rx.recv_timeout(deadline.saturating_duration_since(Instant::now())) {
@row_add n=20 row=decision/hand-rolled-json-rpc-over-ndjson area=established-decisions rec=fct-lhug4cke@1
name: JSON-RPC 2.0 over ndjson, hand-rolled with serde
rule: The channel speaks a JSON-RPC 2.0 envelope over ndjson, hand-rolled with serde; requests carry an `id`, hook events are notifications without one.
why: The same shape as MCP makes `mcp` a thin adapter; jsonrpsee is Tokio-bound and would drag a runtime into `hook`.
@row_check n=20 row=decision/hand-rolled-json-rpc-over-ndjson sha=93ac7bdbd7a8 v=holds judge=sonnet head=8834195b0683
at: crates/viola-channel/src/client.rs:178
quote: let frame = json!({"jsonrpc": "2.0", "id": id, "method": method, "params": params});
at: crates/viola-channel/src/client.rs:109
quote: pub fn notify(
@row_add n=21 row=decision/self-describing-frames-no-handshake area=established-decisions rec=fct-lhug4cke@1
name: Self-describing frames, no handshake
rule: Every channel `params` carries `v` and the sender's `CARGO_PKG_VERSION`, with no handshake.
why: Mixed binary versions are a measured fact, so every frame self-describes.
@row_check n=21 row=decision/self-describing-frames-no-handshake sha=6ef4b5afdc7d v=holds judge=sonnet head=8834195b0683
at: crates/viola-channel/src/client.rs:174-175
quote: params.insert("v".to_owned(), PROTOCOL_V.into()); params.insert("sender".to_owned(), VERSION.into());
at: crates/viola-channel/src/client.rs:114-115
quote: params.insert("v".to_owned(), PROTOCOL_V.into()); params.insert("sender".to_owned(), VERSION.into());
@row_add n=22 row=decision/list-is-caller-side-read area=established-decisions rec=fct-lhug4cke@1,fct-4ghn4t2o@1
name: list is a caller-side read
rule: `list` is not a channel method: the caller reads the disk and one `claude agents --json`, joined into the board.
@row_check n=22 row=decision/list-is-caller-side-read sha=abf75c6dc44d v=holds judge=sonnet head=8834195b0683
at: src/cmd/run.rs:140
quote: "send" => send::send(&self.send, &self.name, &self.instance_dir, call),
at: src/cmd/run.rs:149
quote: _ => self.dispatch(call.method, call.params),
at: src/cmd/list.rs:120
quote: let mut board = board::read(scan, at);
@row_add n=23 row=decision/rmcp-stdio-mcp-server area=established-decisions rec=fct-tkwxvt3j@1
name: rmcp stdio MCP server
rule: rmcp, its minor pinned, is what serves MCP, over stdio.
why: The official SDK.
@row_check n=23 row=decision/rmcp-stdio-mcp-server sha=23df60710a27 v=holds judge=sonnet head=8834195b0683
at: Cargo.toml:223
quote: rmcp = { version = ">=3.4.1, <3.5", default-features = false, features = ["server", "transport-io"] }
@row_add n=24 row=contract/mcp-tool-result/refusal area=established-decisions rec=fct-tkwxvt3j@1
boundary: MCP tool results
item: refusal
promise: A refusal is an `isError: true` tool result mapped from the channel's `result.refusal`.
@row_check n=24 row=contract/mcp-tool-result/refusal sha=34a2e52ef9af v=holds judge=sonnet head=8834195b0683
at: crates/viola-mcp/src/result.rs:21
quote: failed(reason, json!({"refusal": reason, "detail": detail}))
at: crates/viola-mcp/src/result.rs:40
quote: let mut result = CallToolResult::structured_error(body);
at: crates/viola-mcp/src/result.rs:83
quote: Ok(refused(reason, detail))
@row_add n=25 row=decision/release-cli-only area=established-decisions rec=fct-tkwxvt3j@1
name: release is CLI-only
rule: `release` is never an MCP tool; only the CLI offers it.
why: A driver is never offered a way to hand the wheel back to automation: an affordance, not an enforcement.
@row_check n=25 row=decision/release-cli-only sha=0af4646db0b0 v=holds judge=sonnet head=8834195b0683
at: crates/viola-mcp/src/tools.rs:37
quote: pub const TOOL_NAMES: [&str; 5] = ["send", "wait", "last", "answer", "list"];
at: src/cmd/mod.rs:78
quote: Release(release::ReleaseArgs),
@row_add n=26 row=decision/pause-link-unlink-cli-only area=established-decisions rec=fct-tkwxvt3j@1
name: pause, link and unlink are CLI-only
rule: `pause`, `link` and `unlink` are offered only on the CLI, never as MCP tools.
why: They are human controls over the wheel and the link topology.
@row_check n=26 row=decision/pause-link-unlink-cli-only sha=c695cdc166c3 v=holds judge=sonnet head=8834195b0683
at: crates/viola-mcp/src/tools.rs:37
quote: pub const TOOL_NAMES: [&str; 5] = ["send", "wait", "last", "answer", "list"];
at: src/cmd/mod.rs:76
quote: Pause(pause::PauseArgs),
at: src/cmd/mod.rs:80
quote: Link(link::LinkArgs),
at: src/cmd/mod.rs:82
quote: Unlink(link::LinkArgs),
@row_add n=27 row=decision/mcp-stdin-line-cap area=established-decisions rec=fct-tkwxvt3j@1
name: MCP stdin read under a line cap
rule: The MCP server's stdin is read under a `MAX_FRAME` line cap, newline counted; a longer line is refused whole, never truncated or skipped, and ends the session with one `parse-rejected{parser:"mcp-stdin"}` line holding no byte of it.
why: rmcp's stdio transport reads each line unbounded, so the bound sits in the reader viola hands rmcp.
@row_check n=27 row=decision/mcp-stdin-line-cap sha=3f59018f7447 v=holds judge=sonnet head=8834195b0683
at: crates/viola-mcp/src/lib.rs:77
quote: let stdin = stdin::CappedLines::new(stdin, MAX_FRAME);
at: crates/viola-mcp/src/stdin.rs:60
quote: if !self.fits(self.held.len() + segment.len()) {
at: crates/viola-mcp/src/stdin.rs:63
quote: self.cut.store(true, Ordering::Relaxed);
at: crates/viola-mcp/src/lib.rs:98
quote: parser = "mcp-stdin",
@row_add n=28 row=decision/exec-form-hooks area=established-decisions rec=fct-zhr3wx2i@2
name: Exec-form command hooks
rule: Hooks are exec-form `command` hooks naming the pinned viola copy with args `hook <event>`.
why: Exec form skips shell startup.
@row_check n=28 row=decision/exec-form-hooks sha=55f29194413f v=holds judge=sonnet head=8834195b0683
at: plugin/hooks/hooks.json:4
quote: {"hooks": [{"type": "command", "command": "@@VIOLA_BIN@@", "args": ["hook", "session-start"], "timeout": 5}]}
at: crates/viola-agent-claude/src/lib.rs:119
quote: const BIN_TOKEN: &str = "@@VIOLA_BIN@@";
@row_add n=29 row=contract/hook-tiers/sync-spine area=established-decisions rec=fct-zhr3wx2i@2
boundary: hook tiers
item: sync spine
promise: SessionStart, UserPromptSubmit and Stop run synchronously with no reply body within viola's spine deadline; SessionEnd inside its 1.5 s budget, never raised.
@row_check n=29 row=contract/hook-tiers/sync-spine sha=5943baa7a8fb v=holds judge=sonnet head=8834195b0683
at: plugin/hooks/hooks.json:4
quote: {"hooks": [{"type": "command", "command": "@@VIOLA_BIN@@", "args": ["hook", "session-start"], "timeout": 5}]}
at: plugin/hooks/hooks.json:7
quote: {"hooks": [{"type": "command", "command": "@@VIOLA_BIN@@", "args": ["hook", "user-prompt-submit"], "timeout": 5}]}
at: plugin/hooks/hooks.json:16
quote: {"hooks": [{"type": "command", "command": "@@VIOLA_BIN@@", "args": ["hook", "stop"], "timeout": 5}]}
at: plugin/hooks/hooks.json:19
quote: {"hooks": [{"type": "command", "command": "@@VIOLA_BIN@@", "args": ["hook", "session-end"]}]}
at: src/cmd/hook.rs:56
quote: const CONNECT_DEADLINE: Duration = Duration::from_millis(750);
@row_add n=30 row=decision/lost-endpoint-loses-event area=established-decisions rec=fct-zhr3wx2i@2
name: A lost endpoint loses the event, not the turn
rule: When a hook cannot reach its endpoint, the hook event is lost and the turn goes on.
@row_check n=30 row=decision/lost-endpoint-loses-event sha=5f171205cc28 v=holds judge=sonnet head=8834195b0683
at: src/cmd/hook.rs:116-117
quote: let Some(instance) = instance.as_ref().and_then(canonical).filter(trusted) else { return Ok(ExitCode::SUCCESS);
at: src/cmd/hook.rs:235
quote: Client::connect_by(endpoint, "hook", connect_by).map_err(|_| NotSent::Unreachable)?;
@row_add n=31 row=decision/dialog-hold-bound area=established-decisions rec=fct-4nt6qtyp@3
name: Dialog hold bounds a sync dialog hook
rule: A sync dialog hook waits at most the dialog hold (`dialog_hold_s`, 60 to 3600 s, default 60), after which the dialog renders for the human.
state: provisional
@row_check n=31 row=decision/dialog-hold-bound sha=ab61c081c108 v=holds judge=sonnet head=8834195b0683
at: crates/viola-core/src/lib.rs:19
quote: pub const DIALOG_HOLD_DEFAULT: Duration = Duration::from_secs(60);
at: crates/viola-core/src/lib.rs:22
quote: pub const DIALOG_HOLD_MIN: Duration = Duration::from_secs(60);
at: crates/viola-core/src/lib.rs:27
quote: pub const DIALOG_HOLD_MAX: Duration = Duration::from_secs(3600);
at: src/run/dialog.rs:153
quote: const HOLD_KEY: &str = "dialog_hold_s";
@row_add n=32 row=decision/wheel-byte-source-claim area=established-decisions rec=fct-dl246x3d@1
name: Byte-source claim of the wheel
rule: Any human editing key since the last turn boundary moves the wheel to the human and `send` is refused `human-typing`; only a closed non-editing list of keys never counts.
why: `viola run` owns the real stdin, so it knows byte provenance.
@row_check n=32 row=decision/wheel-byte-source-claim sha=5c199a734cda v=holds judge=sonnet head=8834195b0683
at: src/run/wheel.rs:322-323
quote: if self.keys.feed(&buf[..n]) { self.wheel.human_input();
at: src/run/wheel.rs:42
quote: (Input::Human, Wheel::Driver, _) => Some((Wheel::Human, WheelCause::HumanInput)),
at: src/run/dialog.rs:431
quote: RefusalReason::HumanTyping,
@row_add n=33 row=decision/explicit-wheel-release area=established-decisions rec=fct-dl246x3d@1
name: Explicit wheel release
rule: The wheel returns from the human to the driver only through `viola release`.
@row_check n=33 row=decision/explicit-wheel-release sha=42507e84287c v=holds judge=sonnet head=8834195b0683
at: src/run/wheel.rs:45
quote: (Input::Release, Wheel::Human, _) => Some((Wheel::Driver, WheelCause::Release)),
@row_add n=34 row=decision/link-recording area=established-decisions rec=fct-3iyhqr2o@1
name: How a driver-to-driven link is recorded
rule: A link `driver → driven` is recorded when a `send` or `answer` is issued from inside a wrapped session to another instance, or by an explicit `viola link <driver> <driven>`; `viola unlink` removes it.
@row_check n=34 row=decision/link-recording sha=b257ad661eaa v=holds judge=sonnet head=8834195b0683
at: src/run/send.rs:145
quote: links.record(from).map_err(|_| ProtocolError::Internal)?;
at: src/run/dialog.rs:455
quote: links.record(from).map_err(|_| ProtocolError::Internal)?;
at: src/cmd/mod.rs:80
quote: Link(link::LinkArgs),
at: src/run/links.rs:108
quote: let line = self.line(EventKind::Unlink, driver);
@row_add n=35 row=decision/links-are-log-events area=established-decisions rec=fct-3iyhqr2o@1
name: Links are events in the log
rule: Links are events in the log, and the snapshot's link set is derived by replaying them.
why: The log stays the single source of truth for the link set.
@row_check n=35 row=decision/links-are-log-events sha=66ba4247f12b v=holds judge=sonnet head=8834195b0683
at: src/run/links.rs:65
quote: let line = self.line(EventKind::Link, driver);
at: crates/viola-state/src/replay.rs:89
quote: (Some(EventKind::Link), None) => {
at: crates/viola-state/src/replay.rs:91
quote: links.push(json!({"driver": driver, "driven": driven, "since": since}));
@row_add n=36 row=decision/budget-soft-stop area=established-decisions rec=fct-3hy3yfb4@1
name: Budget soft stop at the turn boundary
rule: Once a configured threshold is crossed (defaults `five_hour` at 90 %, `seven_day` at 85 %), automated `send` is refused `budget-paused`; human keys are never blocked and a running turn finishes.
why: The per-model weekly window is hidden, so thresholds are set low.
@row_check n=36 row=decision/budget-soft-stop sha=3a44fe23a698 v=holds judge=sonnet head=8834195b0683
at: src/run/budget.rs:30-31
quote: const FIVE_HOUR_DEFAULT: u8 = 90; const SEVEN_DAY_DEFAULT: u8 = 85;
at: src/run/send/budget.rs:11
quote: RefusalReason::BudgetPaused,
@row_add n=37 row=decision/budget-pause-lift area=established-decisions rec=fct-3hy3yfb4@1
name: Budget pause lifts on reset or release
rule: A budget pause lifts when `resets_at` passes or on `viola release --budget`.
@row_check n=37 row=decision/budget-pause-lift sha=02221e463b8a v=holds judge=sonnet head=8834195b0683
at: src/run/budget.rs:157
quote: (*used >= f64::from(threshold.into_inner()) && later(resets_at, now)).then(|| Crossed {
at: src/run/wheel.rs:213-214
quote: if budget { let lifted = gate.lift().map_err(|_| ProtocolError::Internal)?;
@row_add n=38 row=decision/budget-gate-fails-open area=established-decisions rec=fct-3hy3yfb4@1
name: Budget gate never polls and fails open
rule: The budget gate never polls and fails open.
@row_check n=38 row=decision/budget-gate-fails-open sha=18f7e17cf397 v=holds judge=sonnet head=8834195b0683
at: src/run/budget.rs:179
quote: let paused = reading.and_then(|reading| {
at: src/run/budget.rs:203
quote: pub(crate) enum Trigger {
@row_add n=39 row=decision/global-statusline-untouched area=established-decisions rec=fct-3hy3yfb4@1
name: Global statusline config untouched
rule: The user's global statusline config is never touched.
@row_check n=39 row=decision/global-statusline-untouched sha=2e010527a7fc v=holds judge=sonnet head=8834195b0683
searched: settings\.json · in src crates plugin (only reads: src/cmd/run.rs:540 builds the path to read; writes appear only in tests)
searched: USER_SETTINGS · in src crates (one product use, src/cmd/run.rs:540, a read)
@row_add n=40 row=decision/verify-never-types-into-dialog area=established-decisions rec=fct-7vnaqpcq@1
name: viola verify never answers a CLI dialog
rule: No `viola verify` run writes a byte into a CLI dialog.
@row_check n=40 row=decision/verify-never-types-into-dialog sha=556c6e2d7e6f v=holds judge=sonnet head=8834195b0683
at: src/cmd/verify/typed/runs.rs:124
quote: if !run.paste_text(prompt) {
at: src/cmd/verify/typed/runs.rs:139
quote: run.end_by_kill();
@row_add n=41 row=absence/graphical-frontend area=established-decisions rec=fct-jn2hd4ac@2,fct-7wwoxbip@2
capability: A web or graphical frontend, page or graphical control surface
@row_check n=41 row=absence/graphical-frontend sha=df45b74d20e9 v=holds judge=sonnet head=8834195b0683
searched: axum|actix|warp|tauri|egui|iced|TcpListener|ratatui|crossterm · in Cargo.toml crates/*/Cargo.toml src/**/*.rs crates/*/src/**/*.rs (no match)
searched: web frontend or page route · in plugin/ src/ (none; stdio MCP only)
@row_add n=42 row=decision/liveness-heartbeat-and-pid area=established-decisions rec=fct-aos27ydg@1
name: Heartbeat-first session liveness
rule: The heartbeat file is primary; pid plus process start time decide `gone`, and `claude agents --json` only enriches it.
why: The prototype's heartbeat proved reliable, and a bare pid is unsafe because pids get reused.
@row_check n=42 row=decision/liveness-heartbeat-and-pid sha=9ad8fcc3e433 v=holds judge=sonnet head=8834195b0683
at: crates/viola-state/src/liveness.rs:28-29
quote: if !same_process { return Liveness::Gone;
at: crates/viola-state/src/liveness.rs:32
quote: Some(age) if age <= STALE_AFTER => Liveness::Live,
@row_add n=43 row=decision/no-start-over-live-instance area=established-decisions rec=fct-aos27ydg@1
name: No start over a live or stale instance
rule: A `viola run` or `viola revive` whose `ViolaName` is `live` or `stale` refuses to start.
@row_check n=43 row=decision/no-start-over-live-instance sha=79f6e595c943 v=holds judge=sonnet head=8834195b0683
at: src/cmd/run.rs:454
quote: Liveness::Live => Some(refused_with(json, "already-live", || refuse_live(name))),
at: src/cmd/run.rs:455
quote: Liveness::Stale => Some(refused_with(json, "stale-heartbeat", || {
at: src/cmd/revive.rs:212
quote: || collision_check(name, &instance_dir, read.as_ref().ok(), json),
@row_add n=44 row=decision/revive-in-place area=established-decisions rec=fct-aos27ydg@1,fct-efluvsb5@1
name: Revive resumes in place through the start path
rule: A revive resumes the `gone` instance in place through the same start as `run`, its `events.ndjson` appended to, never truncated.
@row_check n=44 row=decision/revive-in-place sha=d18de3bbc3b2 v=holds judge=sonnet head=8834195b0683
at: src/cmd/revive.rs:154
quote: Ok(launch) => start(home, &launch, &persistent, &exe),
at: crates/viola-state/src/events.rs:330
quote: let mut file = open_private_append(&instance_dir.join(EVENTS))?;
@row_add n=45 row=decision/chrono-utc-millisecond-timestamps area=established-decisions rec=fct-qx4of5bv@1
name: chrono timestamps, RFC 3339 UTC, milliseconds
rule: chrono writes every timestamp viola writes as RFC 3339 UTC at millisecond precision.
why: chrono is already in the tree through rmcp.
@row_check n=45 row=decision/chrono-utc-millisecond-timestamps sha=2fc5080e37d6 v=holds judge=sonnet head=8834195b0683
at: crates/viola-state/src/lib.rs:31
quote: at.to_rfc3339_opts(SecondsFormat::Millis, true)
at: src/obs.rs:96
quote: at.to_rfc3339_opts(SecondsFormat::Millis, true)
@row_add n=46 row=decision/tolerant-resets-at area=established-decisions rec=fct-qx4of5bv@1
name: External resets_at parsed tolerantly
rule: An external `resets_at` is parsed tolerantly, and a malformed value becomes `unknown`, never an error.
why: External clocks must never break parsing.
@row_check n=46 row=decision/tolerant-resets-at sha=c248d6842568 v=holds judge=sonnet head=8834195b0683
at: crates/viola-agent-claude/src/statusline.rs:65
quote: Some(Value::String(text)) => DateTime::parse_from_rfc3339(text)
at: crates/viola-agent-claude/src/statusline.rs:74
quote: _ => Reading::UNKNOWN,
@row_add n=47 row=decision/versioned-tolerant-own-formats area=established-decisions rec=fct-caebuvjq@1
name: Own formats versioned-tolerant
rule: Every channel `params`, event and snapshot carries `v`, and readers skip and count unknown kinds and fields.
why: `deny_unknown_fields` on own formats was rejected because mixed binary versions are real.
@row_check n=47 row=decision/versioned-tolerant-own-formats sha=f4b670af6972 v=holds judge=sonnet head=8834195b0683
at: crates/viola-channel/src/client.rs:114
quote: params.insert("v".to_owned(), PROTOCOL_V.into());
at: crates/viola-state/src/events.rs:32
quote: pub v: u32,
at: crates/viola-state/src/snapshot.rs:84
quote: v: u32,
at: crates/viola-state/src/events.rs:262
quote: self.skipped.unknown_kinds += 1;
at: crates/viola-state/src/events.rs:266
quote: self.skipped.unknown_fields += 1;
@row_add n=48 row=decision/flat-cargo-workspace area=established-decisions rec=fct-brnw2t5e@1
name: Flat Cargo workspace with the bin at the root
rule: One Cargo workspace with a flat `crates/` layout, the root `Cargo.toml` being both the `viola` bin package and the workspace; `fuzz/` is its own workspace.
why: The compiler enforces that `viola-pty` cannot see `viola-agent-claude`; `cargo install --path .` needs the bin at the root.
@row_check n=48 row=decision/flat-cargo-workspace sha=6ff0d8b6cb5b v=holds judge=sonnet head=8834195b0683
at: Cargo.toml:180-182
quote: [workspace] members = ["crates/*"] exclude = ["fuzz"]
at: Cargo.toml:1-2
quote: [package] name = "viola"
at: fuzz/Cargo.toml:20
quote: [workspace]
@row_add n=49 row=decision/no-path-lookup area=established-decisions rec=fct-c3wssf3l@2,fct-le5sc2tk@1
name: Hooks and MCP never search PATH
rule: Hooks and MCP never search PATH; an installed plugin's `command` is an absolute path.
@row_check n=49 row=decision/no-path-lookup sha=e0072e826643 v=holds judge=sonnet head=8834195b0683
at: crates/viola-agent-claude/src/marketplace.rs:45
quote: NAME: {"command": exe_fwd, "args": ["--home", home_fwd, "mcp"]},
at: src/cmd/plugin.rs:115
quote: for (rel, content) in marketplace::marketplace_files(&fwd(&exe), &fwd(&home)) {
at: src/cmd/run.rs:475
quote: for (rel, content) in viola_agent_claude::plugin_files(&pinned.path_fwd) {
@row_add n=50 row=decision/optional-plugin-install area=established-decisions rec=fct-le5sc2tk@1
name: Optional MCP-only plugin install
rule: `viola plugin install` (a local marketplace) is optional, for the MCP verbs in an unwrapped driver, and the installed plugin carries only `.mcp.json` and no hooks.
@row_check n=50 row=decision/optional-plugin-install sha=f63ef3878bb6 v=holds judge=sonnet head=8834195b0683
at: crates/viola-agent-claude/src/marketplace.rs:24
quote: pub fn marketplace_files(exe_fwd: &str, home_fwd: &str) -> [(&'static str, String); 3] {
at: crates/viola-agent-claude/src/marketplace.rs:51
quote: ("viola/.mcp.json", document(&mcp)),
@row_add n=51 row=decision/cli-output-conventions area=established-decisions rec=fct-nvft5tqc@2
name: Human text by default, --json for agents
rule: The CLI prints human text by default and `--json` for agents, with a typed exit code per refusal.
why: Agents need machine-parseable refusals.
@row_check n=51 row=decision/cli-output-conventions sha=326013f129f1 v=holds judge=sonnet head=8834195b0683
at: src/cmd/send.rs:36
quote: json: bool,
at: src/cmd/send.rs:67-71
quote: RefusalReason::HumanTyping => 10, RefusalReason::BudgetPaused => 11, RefusalReason::UnverifiedCli => 12, RefusalReason::NotDelivered => 13, RefusalReason::Unknown => 14,
@row_add n=52 row=decision/ci-against-fake-agent area=established-decisions rec=fct-bvzxxcy7@2
name: CI on ubuntu-latest against the fake agent
rule: GitHub Actions on `ubuntu-latest` runs the tests against the fake agent; the real CLI runs only in local live tests.
why: No tokens spent, no flakes in CI, and no cross-compilers (cross 0.2.5 is stale).
@row_check n=52 row=decision/ci-against-fake-agent sha=b716b4eaa3a0 v=holds judge=sonnet head=8834195b0683
at: .github/workflows/ci.yml:18
quote: os: [ubuntu-latest]
at: tests/run_cli.rs:97
quote: fn run_with_fake_agent_writes_start_and_exit_lines(#[from(home)] tmp: TestHome) {
@row_add n=53 row=decision/fake-agent-after-double-dash area=established-decisions rec=fct-bvzxxcy7@2
name: Fake agent named after --
rule: The fake agent is substituted by naming its executable after `--` (`viola run <name> -- <fake agent> <args>`).
@row_check n=53 row=decision/fake-agent-after-double-dash sha=c92ea3c554d9 v=holds judge=sonnet head=8834195b0683
at: tests/run_cli.rs:65
quote: args.extend(["run", name, "--", program].map(OsString::from));
@row_add n=54 row=decision/child-program-resolution area=established-decisions rec=fct-bvzxxcy7@2,fct-7tdo3mcl@2
name: viola resolves the child program itself
rule: viola resolves the child program to an absolute path itself, a given path as given and a bare name searched on `PATH`, and spawns that.
@row_check n=54 row=decision/child-program-resolution sha=7f5043caa093 v=holds judge=sonnet head=8834195b0683
at: crates/viola-agent-claude/src/lib.rs:192-195
quote: if given.is_absolute() || given.components().count() > 1 { Ok(cwd.join(given)) } else { find_on_path(program, search, is_file).ok_or(AgentError::ProgramNotFound)
at: src/run/mod.rs:147
quote: viola_agent_claude::resolve_program(program, cwd, search, &|p: &Path| p.is_file())
@row_add n=55 row=decision/test-isolated-viola-home area=established-decisions rec=fct-bvzxxcy7@2
name: Each test in its own viola home
rule: Each test runs in its own viola home (`--home`), so parallel tests never share an endpoint, log or `budget.json`.
@row_check n=55 row=decision/test-isolated-viola-home sha=e0d448705216 v=holds judge=sonnet head=8834195b0683
at: tests/run_cli.rs:64
quote: let mut args: Vec<OsString> = vec!["--home".into(), home.into()];
at: crates/viola-e2e/src/harness/supervise.rs:59
quote: let mut args: Vec<OsString> = vec!["--home".into(), spec.home.clone().into()];
@row_add n=56 row=decision/no-hosting area=established-decisions rec=fct-mg4wkqys@1,fct-hcu7z4ke@2
name: No hosting
rule: viola has no hosting; Docker, serverless and PaaS do not apply.
why: v1 is local-only.
@row_check n=56 row=decision/no-hosting sha=ec89a32ce065 v=holds judge=sonnet head=8834195b0683
searched: Dockerfile* · docker-compose* · serverless.yml · fly.toml · Procfile · vercel.json · netlify.toml · app.yaml · wrangler.toml · render.yaml · in repository root glob
searched: TcpListener|0\.0\.0\.0|axum|actix|warp:: · in Cargo.toml, crates/*/Cargo.toml, src/**/*.rs, crates/*/src/**/*.rs
@row_add n=57 row=decision/no-model-api-calls area=established-decisions rec=fct-bdlakgwt@1
name: No model API calls
rule: The binary makes no model API calls; the model is reached only through the unmodified `claude` CLI.
why: D2, subscription only through the unmodified `claude` CLI.
@row_check n=57 row=decision/no-model-api-calls sha=e9ae011299ab v=holds judge=sonnet head=8834195b0683
searched: reqwest|ureq|hyper|isahc|curl|anthropic|attohttpc|TcpStream · in Cargo.toml, crates/*/Cargo.toml, src/**/*.rs, crates/*/src/**/*.rs (no HTTP client or API host; only comments)
searched: Command::new( · in src/**/*.rs (the child is the resolved `claude` program only, src/cmd/run.rs:268)
@row_add n=58 row=decision/viola-name-prefix area=established-decisions rec=fct-5ohkl26i@1
name: viola prefix for names
rule: The product, the binary and identifiers use the `viola` prefix, environment variables included (`VIOLA_NAME`, `VIOLA_DIR`, `VIOLA_BIN`, newtype `ViolaName`).
why: One name everywhere removes the need to translate between prototype and product.
except: Ratified: test seams `FAKE_AGENT_PUMP_DELAY_MS`, `FAKE_AGENT_HOOK_PANIC` (fake-agent builds only); `NO_COLOR`, `TERM`, `COLORTERM`, read never set, deciding only SGR bytes.
@row_check n=58 row=decision/viola-name-prefix sha=19d9d53e0d3f v=holds judge=sonnet head=8834195b0683
at: crates/viola-core/src/lib.rs:52
quote: pub struct ViolaName(String);
at: src/cmd/hook.rs:109-110
quote: std::env::var_os("VIOLA_NAME"), std::env::var_os("VIOLA_DIR"),
at: src/run/mod.rs:125
quote: ("VIOLA_BIN".into(), OsString::from(&pinned.path_fwd)),
at: src/cmd/run.rs:63
quote: let ms = std::env::var("FAKE_AGENT_PUMP_DELAY_MS")
at: src/cmd/hook/seam.rs:10
quote: panic_on(std::env::var_os("FAKE_AGENT_HOOK_PANIC"));
at: src/human/style.rs:95
quote: no_color: std::env::var_os("NO_COLOR").is_some(),
@row_add n=59 row=decision/four-interfaces-one-binary area=conventions rec=fct-ynzkvjxk@1
name: Four interfaces from one binary
rule: The binary exposes four interfaces: CLI verbs, MCP tools over stdio, the hook stdin/stdout contract, and the wrapper channel; there is no REST write surface, GraphQL, gRPC or tRPC.
@row_check n=59 row=decision/four-interfaces-one-binary sha=75abb33cdcda v=holds judge=sonnet head=8834195b0683
searched: axum|hyper|tonic|actix|warp|graphql|trpc|reqwest|tungstenite|rocket|poem|prost · in Cargo.toml and crates/*/Cargo.toml (none a dependency; only a comment at Cargo.toml:208)
searched: PROTOCOL_V · in crates/viola-channel/src/lib.rs (the wrapper channel: `pub const PROTOCOL_V: u64 = 1;` at line 28)
searched: pub enum McpError · in crates/viola-mcp/src/lib.rs (the MCP stdio server crate, line 32)
@row_add n=60 row=contract/protocol-version/v area=conventions rec=fct-hhwkhtaq@1
boundary: protocol versioning
item: v
promise: An integer starting at 1, carried in every channel `params`, ndjson event line and snapshot.
@row_check n=60 row=contract/protocol-version/v sha=6f64d35f47f7 v=holds judge=sonnet head=8834195b0683
at: crates/viola-channel/src/client.rs:114
quote: params.insert("v".to_owned(), PROTOCOL_V.into());
at: crates/viola-state/src/events.rs:32
quote: pub v: u32,
at: crates/viola-state/src/snapshot.rs:93
quote: v: SNAPSHOT_V,
@row_add n=61 row=decision/protocol-version-bump-rule area=conventions rec=fct-hhwkhtaq@1
name: When the protocol version bumps
rule: Adding a field or event kind is additive and needs no bump of `v`; removing a field or changing its meaning bumps `v`.
@row_check n=61 row=decision/protocol-version-bump-rule sha=58acb9161800 v=unverifiable judge=sonnet head=8834195b0683
why: not a code claim
@row_add n=62 row=contract/channel-params/sender area=conventions rec=fct-gajhyfaa@1
boundary: channel params
item: sender
promise: Every channel `params` carries the sending binary's `CARGO_PKG_VERSION`.
@row_check n=62 row=contract/channel-params/sender sha=0d93a98a8124 v=holds judge=sonnet head=8834195b0683
at: crates/viola-channel/src/client.rs:115
quote: params.insert("sender".to_owned(), VERSION.into());
at: crates/viola-channel/src/client.rs:175
quote: params.insert("sender".to_owned(), VERSION.into());
@row_add n=63 row=contract/snapshot/writer area=conventions rec=fct-gajhyfaa@1
boundary: snapshot
item: writer
promise: Holds the `CARGO_PKG_VERSION` of the binary that wrote the snapshot.
@row_check n=63 row=contract/snapshot/writer sha=b487d576fd5a v=holds judge=sonnet head=8834195b0683
at: crates/viola-state/src/snapshot.rs:95
quote: writer: VERSION.to_owned(),
@row_add n=64 row=contract/channel-response/refusal area=conventions rec=fct-wfq6mlyt@1,fct-lhug4cke@1
boundary: channel response
item: refusal
promise: A refusal is a normal outcome, not an error: `{"jsonrpc":"2.0","id":<n>,"result":{"refusal":"<reason>","detail":<string|null>}}`.
@row_check n=64 row=contract/channel-response/refusal sha=e4dc7daec02e v=holds judge=sonnet head=8834195b0683
at: src/run/send.rs:298
quote: Ok(json!({"refusal": refusal.as_str(), "detail": detail}))
at: crates/viola-channel/src/frame.rs:95
quote: json!({"jsonrpc": "2.0", "id": id, "result": result})
@row_add n=65 row=contract/channel-response/ok area=conventions rec=fct-5z4gbncz@1
boundary: channel response
item: ok
promise: Success is `{"jsonrpc":"2.0","id":<n>,"result":{"ok":<payload>}}`.
@row_check n=65 row=contract/channel-response/ok sha=307b6fd3f23d v=holds judge=sonnet head=8834195b0683
at: src/run/send.rs:224
quote: Ok(json!({"ok": {"confirmed": false, "detail": "unconfirmable", "cursor": cursor}}))
at: crates/viola-channel/src/frame.rs:95
quote: json!({"jsonrpc": "2.0", "id": id, "result": result})
@row_add n=66 row=contract/channel-response/error area=conventions rec=fct-h4oc7sqh@1,fct-lhug4cke@1
boundary: channel response
item: error
promise: Reserved for protocol faults: `{"jsonrpc":"2.0","id":<n|null>,"error":{"code":<int>,"message":"<text>","data":<object|null>}}`.
@row_check n=66 row=contract/channel-response/error sha=3b768a0d17a5 v=holds judge=sonnet head=8834195b0683
at: crates/viola-channel/src/frame.rs:100
quote: json!({"jsonrpc": "2.0", "id": id, "error": fault.body()})
at: crates/viola-channel/src/lib.rs:99
quote: json!({"code": self.code(), "message": self.message(), "data": data})
@row_add n=67 row=contract/channel-fault-codes/-32700 area=conventions rec=fct-h4oc7sqh@1
boundary: channel protocol fault codes
item: -32700
promise: Parse error.
@row_check n=67 row=contract/channel-fault-codes/-32700 sha=bee4fa0513d5 v=holds judge=sonnet head=8834195b0683
at: crates/viola-channel/src/lib.rs:71
quote: Self::Parse => -32700,
@row_add n=68 row=contract/channel-fault-codes/-32600 area=conventions rec=fct-h4oc7sqh@1
boundary: channel protocol fault codes
item: -32600
promise: Invalid request.
@row_check n=68 row=contract/channel-fault-codes/-32600 sha=5e2e39a5346a v=holds judge=sonnet head=8834195b0683
at: crates/viola-channel/src/lib.rs:72
quote: Self::InvalidRequest => -32600,
@row_add n=69 row=contract/channel-fault-codes/-32601 area=conventions rec=fct-h4oc7sqh@1
boundary: channel protocol fault codes
item: -32601
promise: Unknown method.
@row_check n=69 row=contract/channel-fault-codes/-32601 sha=b8e5616205f8 v=holds judge=sonnet head=8834195b0683
at: crates/viola-channel/src/lib.rs:73
quote: Self::MethodNotFound => -32601,
@row_add n=70 row=contract/channel-fault-codes/-32602 area=conventions rec=fct-h4oc7sqh@1,fct-hhwkhtaq@1
boundary: channel protocol fault codes
item: -32602
promise: Bad params or unsupported `v`; a wrapper receiving a higher `params.v` than it supports answers it with `data.supported` and `data.wrapper` (its `CARGO_PKG_VERSION`).
@row_check n=70 row=contract/channel-fault-codes/-32602 sha=6ef0231deec5 v=holds judge=sonnet head=8834195b0683
at: crates/viola-channel/src/lib.rs:74
quote: Self::UnsupportedVersion | Self::InvalidParams | Self::ReleaseFromDriver => -32602,
at: crates/viola-channel/src/lib.rs:95
quote: Self::UnsupportedVersion => json!({"supported": PROTOCOL_V, "wrapper": VERSION}),
at: crates/viola-channel/src/server.rs:239
quote: return Err(ProtocolError::UnsupportedVersion);
@row_add n=71 row=contract/channel-fault-codes/-32603 area=conventions rec=fct-h4oc7sqh@1
boundary: channel protocol fault codes
item: -32603
promise: Internal error.
@row_check n=71 row=contract/channel-fault-codes/-32603 sha=5bef489f1e98 v=holds judge=sonnet head=8834195b0683
at: crates/viola-channel/src/lib.rs:75
quote: Self::Internal => -32603,
@row_add n=72 row=contract/refusal-reason/unknown area=conventions rec=fct-xscdpgpr@1
boundary: RefusalReason
item: `unknown` refusal reason
promise: The catch-all reason (`#[serde(other)]`), its detail a code word or null; an unrecognised code word is opaque text to a reader.
@row_check n=72 row=contract/refusal-reason/unknown sha=8910c355c4d5 v=holds judge=sonnet head=8834195b0683
at: crates/viola-core/src/lib.rs:165
quote: #[serde(other)]
at: src/cmd/client.rs:84
quote: Some(Value::String(detail)) => is_code_word(detail).then(|| detail.clone()),
@row_add n=73 row=decision/refusal-details-closed-set area=conventions rec=fct-xscdpgpr@1
name: Refusal details from a closed set
rule: Each refusal reason's `detail` comes from a closed set per reason, extended only in `viola-core`.
@row_check n=73 row=decision/refusal-details-closed-set sha=9407cd669aed v=holds judge=sonnet head=8834195b0683
at: crates/viola-core/src/lib.rs:209
quote: pub enum NotDelivered {
at: crates/viola-core/src/lib.rs:235
quote: pub enum HumanTyping {
at: crates/viola-core/src/lib.rs:251
quote: pub enum BudgetWindowKind {
@row_add n=74 row=decision/first-refusal-only area=conventions rec=fct-xscdpgpr@1
name: Only the first refusal returned
rule: Of several refusals only the first is returned, in a fixed order per verb, `control-character` first.
@row_check n=74 row=decision/first-refusal-only sha=6fbace78f8d7 v=holds judge=sonnet head=8834195b0683
at: src/run/send.rs:88-89
quote: if let Err(detail) = viola_core::validate_paste_text(text) { return ctx.not_delivered(None, detail);
@row_add n=75 row=decision/control-characters-refused area=conventions rec=fct-xscdpgpr@1
name: Control characters refused, never stripped
rule: Any C0 character but LF, CR and TAB, DEL or a C1 character is refused, never stripped.
@row_check n=75 row=decision/control-characters-refused sha=246b619678e2 v=holds judge=sonnet head=8834195b0683
at: crates/viola-core/src/lib.rs:366-367
quote: '\n' | '\r' | '\t' => false, '\u{0}'..='\u{1f}' | '\u{7f}'..='\u{9f}' => true,
@row_add n=76 row=contract/send-ok-payload/unconfirmable area=conventions rec=fct-xscdpgpr@1,fct-24sxrnlw@1
boundary: send ok payload
item: unconfirmable
promise: Only an `ok` payload's detail, with `confirmed: false`, never a refusal detail.
@row_check n=76 row=contract/send-ok-payload/unconfirmable sha=204be885f41d v=holds judge=sonnet head=8834195b0683
at: src/run/send.rs:224
quote: Ok(json!({"ok": {"confirmed": false, "detail": "unconfirmable", "cursor": cursor}}))
@row_add n=77 row=contract/viola-hook-exit-codes/0 area=conventions rec=fct-nppkdemz@1
boundary: viola hook exit codes
item: 0
promise: Always 0, including on clap parse errors, channel failures and panics caught at the edge; exit 2 is forbidden.
@row_check n=77 row=contract/viola-hook-exit-codes/0 sha=649cd5cb0c66 v=holds judge=sonnet head=8834195b0683
at: src/main.rs:51
quote: Err(_) if role == Role::Hook => return Outcome::Unparsed,
at: src/main.rs:133
quote: (Role::Hook, _) => ExitCode::SUCCESS,
@row_add n=78 row=contract/mcp-tool-results/is-error-true area=conventions rec=fct-5nrjpmo2@1
boundary: MCP tool results
item: isError true
promise: A refusal, an unreachable instance, a wrapper protocol fault and an argument that fails re-validation (`invalid-params`, no frame written) are `isError: true` results shaped like CLI `--json`.
@row_check n=78 row=contract/mcp-tool-results/is-error-true sha=3b6d25a2a7d9 v=holds judge=sonnet head=8834195b0683
at: crates/viola-mcp/src/result.rs:40
quote: let mut result = CallToolResult::structured_error(body);
at: crates/viola-mcp/src/result.rs:21
quote: failed(reason, json!({"refusal": reason, "detail": detail}))
at: crates/viola-mcp/src/result.rs:31
quote: error("instance-unreachable", Value::from(cause.as_str()))
at: crates/viola-mcp/src/result.rs:61
quote: .map(|detail| error("wrapper-fault", detail))
at: crates/viola-mcp/src/result.rs:36
quote: error("invalid-params", Value::from(field))
@row_add n=79 row=decision/one-error-enum-per-crate area=conventions rec=fct-nhsg4k7g@2
name: One error enum per crate
rule: Each crate has one error enum named `<Crate>Error`, each a `thiserror` enum except `PtyError`, whose `Display` and `Error` are hand-written with fixed messages.
except: Ratified exceptions: `ServeError` beside `McpError`, `StampsError` beside `StateError`, `PinError`, `StampsReadError` and `Refused`.
@row_check n=79 row=decision/one-error-enum-per-crate sha=8fd874afa886 v=holds judge=sonnet head=8834195b0683
at: crates/viola-pty/src/lib.rs:81
quote: impl fmt::Display for PtyError {
at: crates/viola-pty/src/lib.rs:96
quote: impl std::error::Error for PtyError {
at: crates/viola-channel/src/lib.rs:33
quote: pub enum ChannelError {
at: crates/viola-state/src/lib.rs:22
quote: pub enum StateError {
at: crates/viola-agent-claude/src/lib.rs:162
quote: pub enum AgentError {
at: crates/viola-mcp/src/lib.rs:32
quote: pub enum McpError {
@row_add n=80 row=decision/anyhow-only-in-root-bin area=conventions rec=fct-nhsg4k7g@2
name: anyhow only in the root bin crate
rule: `anyhow` appears only in the root bin crate.
@row_check n=80 row=decision/anyhow-only-in-root-bin sha=fb17008cd883 v=holds judge=sonnet head=8834195b0683
at: src/obs.rs:256
quote: pub(crate) fn report_internal_error(error: &anyhow::Error, sink: Option<&DetailSink>) {
searched: anyhow · in crates/*/Cargo.toml (none)
searched: anyhow · in crates/**/*.rs and fuzz/**/*.rs (none)
@row_add n=81 row=decision/crate-naming area=conventions rec=fct-md27i7yu@1
name: Crate naming
rule: Crates are named `viola-<area>`, kebab-case.
@row_check n=81 row=decision/crate-naming sha=13c90c974f1a v=holds judge=sonnet head=8834195b0683
at: crates/viola-core/Cargo.toml:2
quote: name = "viola-core"
at: crates/viola-state/Cargo.toml:2
quote: name = "viola-state"
at: crates/viola-channel/Cargo.toml:2
quote: name = "viola-channel"
@row_add n=82 row=decision/standard-rust-naming area=conventions rec=fct-md27i7yu@1
name: Standard Rust naming
rule: Modules and files are snake_case, types UpperCamelCase, functions and variables snake_case, and constants SCREAMING_SNAKE_CASE, as standard Rust.
@row_check n=82 row=decision/standard-rust-naming sha=a93e3c0a046c v=unverifiable judge=sonnet head=8834195b0683
why: not a code claim; the standard Rust casing is a language convention the compiler's lints enforce, with no single line that does it
@row_add n=83 row=decision/json-field-names-snake-case area=conventions rec=fct-osx5axgl@1
name: JSON field names in snake_case
rule: JSON field names are snake_case in every viola format.
except: `last_assistant_message`, `resets_at` and `used_percentage` stay as Claude emits them.
@row_check n=83 row=decision/json-field-names-snake-case sha=9e1b6600de6e v=holds judge=sonnet head=8834195b0683
at: crates/viola-core/src/lib.rs:342
quote: pub used_percentage: Reading<f64>,
at: crates/viola-core/src/lib.rs:350
quote: pub five_hour: Reading<BudgetWindow>,
searched: rename_all = "camelCase" · in src/**/*.rs and crates/**/*.rs (none; every rename_all is "kebab-case")
@row_add n=84 row=decision/wire-enum-values-kebab-case area=conventions rec=fct-vnzcvfvg@1
name: Enum string values on the wire in kebab-case
rule: Enum string values on the wire are kebab-case, covering event kinds, refusal reasons, details and wheel holders (`human`, `driver`).
@row_check n=84 row=decision/wire-enum-values-kebab-case sha=fe0615c143b0 v=holds judge=sonnet head=8834195b0683
at: crates/viola-core/src/lib.rs:159-160
quote: #[serde(rename_all = "kebab-case")] pub enum RefusalReason {
at: crates/viola-core/src/lib.rs:152
quote: Self::DialogExpired => "dialog-expired",
at: crates/viola-state/src/snapshot.rs:22-23
quote: #[serde(rename_all = "kebab-case")] pub enum Wheel {
@row_add n=85 row=contract/normalised-event-kinds/session-start area=conventions rec=fct-5pojuecq@1
boundary: normalised event kinds
item: session-start
promise: `session-start` is a normalised event kind.
@row_check n=85 row=contract/normalised-event-kinds/session-start sha=43c6e61dd45a v=holds judge=sonnet head=8834195b0683
at: crates/viola-core/src/lib.rs:137
quote: Self::SessionStart => "session-start",
@row_add n=86 row=contract/normalised-event-kinds/turn-ended area=conventions rec=fct-5pojuecq@1
boundary: normalised event kinds
item: turn-ended
promise: `turn-ended` is a normalised event kind.
@row_check n=86 row=contract/normalised-event-kinds/turn-ended sha=c4833c5ac192 v=holds judge=sonnet head=8834195b0683
at: crates/viola-core/src/lib.rs:139
quote: Self::TurnEnded => "turn-ended",
@row_add n=87 row=contract/normalised-event-kinds/prompt-submitted area=conventions rec=fct-5pojuecq@1
boundary: normalised event kinds
item: prompt-submitted
promise: `prompt-submitted` is a normalised event kind.
@row_check n=87 row=contract/normalised-event-kinds/prompt-submitted sha=36a8fcec27f6 v=holds judge=sonnet head=8834195b0683
at: crates/viola-core/src/lib.rs:138
quote: Self::PromptSubmitted => "prompt-submitted",
@row_add n=88 row=contract/normalised-event-kinds/question area=conventions rec=fct-5pojuecq@1
boundary: normalised event kinds
item: question
promise: `question` is a normalised event kind.
@row_check n=88 row=contract/normalised-event-kinds/question sha=a5e4c7ed7bd1 v=holds judge=sonnet head=8834195b0683
at: crates/viola-core/src/lib.rs:147
quote: Self::Question => "question",
@row_add n=89 row=contract/normalised-event-kinds/permission area=conventions rec=fct-5pojuecq@1
boundary: normalised event kinds
item: permission
promise: `permission` is a normalised event kind.
@row_check n=89 row=contract/normalised-event-kinds/permission sha=895543a11d22 v=holds judge=sonnet head=8834195b0683
at: crates/viola-core/src/lib.rs:148
quote: Self::Permission => "permission",
@row_add n=90 row=contract/normalised-event-kinds/plan area=conventions rec=fct-5pojuecq@1
boundary: normalised event kinds
item: plan
promise: `plan` is a normalised event kind.
@row_check n=90 row=contract/normalised-event-kinds/plan sha=e371dfc4a363 v=holds judge=sonnet head=8834195b0683
at: crates/viola-core/src/lib.rs:149
quote: Self::Plan => "plan",
@row_add n=91 row=contract/normalised-event-kinds/session-end area=conventions rec=fct-5pojuecq@1
boundary: normalised event kinds
item: session-end
promise: `session-end` is a normalised event kind.
@row_check n=91 row=contract/normalised-event-kinds/session-end sha=5396fcb75c9b v=holds judge=sonnet head=8834195b0683
at: crates/viola-core/src/lib.rs:140
quote: Self::SessionEnd => "session-end",
@row_add n=92 row=contract/normalised-event-kinds/activity area=conventions rec=fct-5pojuecq@1
boundary: normalised event kinds
item: activity
promise: `activity` is a normalised event kind.
@row_check n=92 row=contract/normalised-event-kinds/activity sha=cb2220661b4f v=holds judge=sonnet head=8834195b0683
at: crates/viola-core/src/lib.rs:141
quote: Self::Activity => "activity",
@row_add n=93 row=contract/normalised-event-kinds/link area=conventions rec=fct-5pojuecq@1
boundary: normalised event kinds
item: link
promise: `link` is a normalised event kind.
@row_check n=93 row=contract/normalised-event-kinds/link sha=f7e13c214203 v=holds judge=sonnet head=8834195b0683
at: crates/viola-core/src/lib.rs:150
quote: Self::Link => "link",
@row_add n=94 row=contract/normalised-event-kinds/unlink area=conventions rec=fct-5pojuecq@1
boundary: normalised event kinds
item: unlink
promise: `unlink` is a normalised event kind.
@row_check n=94 row=contract/normalised-event-kinds/unlink sha=51514bcd4b18 v=holds judge=sonnet head=8834195b0683
at: crates/viola-core/src/lib.rs:151
quote: Self::Unlink => "unlink",
@row_add n=95 row=contract/normalised-event-kinds/wheel area=conventions rec=fct-5pojuecq@1
boundary: normalised event kinds
item: wheel
promise: `wheel` is a normalised event kind.
@row_check n=95 row=contract/normalised-event-kinds/wheel sha=d4f94070c2d5 v=holds judge=sonnet head=8834195b0683
at: crates/viola-core/src/lib.rs:142
quote: Self::Wheel => "wheel",
@row_add n=96 row=contract/normalised-event-kinds/budget-gate area=conventions rec=fct-5pojuecq@1
boundary: normalised event kinds
item: budget-gate
promise: `budget-gate` is a normalised event kind.
@row_check n=96 row=contract/normalised-event-kinds/budget-gate sha=f99ba8736263 v=holds judge=sonnet head=8834195b0683
at: crates/viola-core/src/lib.rs:143
quote: Self::BudgetGate => "budget-gate",
@row_add n=97 row=contract/normalised-event-kinds/send-issued area=conventions rec=fct-5pojuecq@1
boundary: normalised event kinds
item: send-issued
promise: `send-issued` is a normalised event kind.
@row_check n=97 row=contract/normalised-event-kinds/send-issued sha=2ba255425de2 v=holds judge=sonnet head=8834195b0683
at: crates/viola-core/src/lib.rs:144
quote: Self::SendIssued => "send-issued",
@row_add n=98 row=contract/normalised-event-kinds/send-confirmed area=conventions rec=fct-5pojuecq@1
boundary: normalised event kinds
item: send-confirmed
promise: `send-confirmed` is a normalised event kind.
@row_check n=98 row=contract/normalised-event-kinds/send-confirmed sha=75d760ee8244 v=holds judge=sonnet head=8834195b0683
at: crates/viola-core/src/lib.rs:145
quote: Self::SendConfirmed => "send-confirmed",
@row_add n=99 row=contract/normalised-event-kinds/send-refused area=conventions rec=fct-5pojuecq@1
boundary: normalised event kinds
item: send-refused
promise: `send-refused` is a normalised event kind.
@row_check n=99 row=contract/normalised-event-kinds/send-refused sha=dbacc270e6cf v=holds judge=sonnet head=8834195b0683
at: crates/viola-core/src/lib.rs:146
quote: Self::SendRefused => "send-refused",
@row_add n=100 row=contract/normalised-event-kinds/dialog-expired area=conventions rec=fct-5pojuecq@1
boundary: normalised event kinds
item: dialog-expired
promise: `dialog-expired` is a normalised event kind.
@row_check n=100 row=contract/normalised-event-kinds/dialog-expired sha=6b9731f28270 v=holds judge=sonnet head=8834195b0683
at: crates/viola-core/src/lib.rs:152
quote: Self::DialogExpired => "dialog-expired",
@row_add n=101 row=decision/event-kinds-agent-neutral area=conventions rec=fct-5pojuecq@1
name: Event kinds added only in viola-core
rule: A new normalised event kind is added only in `viola-core`, never with a Claude-specific name.
@row_check n=101 row=decision/event-kinds-agent-neutral sha=29362849790a v=holds judge=sonnet head=8834195b0683
at: crates/viola-core/src/lib.rs:57
quote: pub enum EventKind {
searched: enum EventKind · in crates/**/*.rs (only viola-core defines it)
@row_add n=102 row=contract/hook-to-kind-map/sessionstart area=conventions rec=fct-ao3imeon@1
boundary: Hook to kind map
item: SessionStart
promise: SessionStart maps to `session-start`; `data.cause` is one of `startup` · `clear` · `resume` · `compact` · `unknown`, and `data.agent_session_id` holds the new session id.
@row_check n=102 row=contract/hook-to-kind-map/sessionstart sha=17368d674a43 v=holds judge=sonnet head=8834195b0683
at: crates/viola-agent-claude/src/hook.rs:70
quote: Self::SessionStart => Some(EventKind::SessionStart),
at: crates/viola-agent-claude/src/hook.rs:159-164
quote: let cause = known .source .as_deref() .filter(|s| SESSION_CAUSES.contains(s)) .unwrap_or("unknown"); json!({"cause": cause, "agent_session_id": known.session_id})
at: crates/viola-agent-claude/src/hook.rs:109
quote: const SESSION_CAUSES: [&str; 4] = ["startup", "clear", "resume", "compact"];
@row_add n=103 row=contract/hook-to-kind-map/userpromptsubmit area=conventions rec=fct-ao3imeon@1
boundary: Hook to kind map
item: UserPromptSubmit
promise: UserPromptSubmit maps to `prompt-submitted`.
@row_check n=103 row=contract/hook-to-kind-map/userpromptsubmit sha=f3e476ff17c7 v=holds judge=sonnet head=8834195b0683
at: crates/viola-agent-claude/src/hook.rs:71
quote: Self::UserPromptSubmit => Some(EventKind::PromptSubmitted),
@row_add n=104 row=contract/hook-to-kind-map/pretooluse-askuserquestion area=conventions rec=fct-ao3imeon@1
boundary: Hook to kind map
item: PreToolUse AskUserQuestion
promise: PreToolUse `AskUserQuestion` maps to `question`.
@row_check n=104 row=contract/hook-to-kind-map/pretooluse-askuserquestion sha=c1e2031a79ef v=holds judge=sonnet head=8834195b0683
at: crates/viola-agent-claude/src/dialog.rs:67
quote: Self::AskUserQuestion => DialogKind::Question,
at: crates/viola-agent-claude/src/dialog.rs:32
quote: Self::Question => EventKind::Question,
@row_add n=105 row=contract/hook-to-kind-map/pretooluse-exitplanmode area=conventions rec=fct-ao3imeon@1
boundary: Hook to kind map
item: PreToolUse ExitPlanMode
promise: PreToolUse `ExitPlanMode` maps to `plan`.
@row_check n=105 row=contract/hook-to-kind-map/pretooluse-exitplanmode sha=32ca0cf2099e v=holds judge=sonnet head=8834195b0683
at: crates/viola-agent-claude/src/dialog.rs:68
quote: Self::ExitPlanMode => DialogKind::Plan,
at: crates/viola-agent-claude/src/dialog.rs:34
quote: Self::Plan => EventKind::Plan,
@row_add n=106 row=contract/hook-to-kind-map/permissionrequest area=conventions rec=fct-ao3imeon@1
boundary: Hook to kind map
item: PermissionRequest
promise: PermissionRequest maps to `permission`, except one that repeats the armed `question` / `plan` dialog's tool with an equal `tool_input`: that is the dialog's continuation and raises no event.
@row_check n=106 row=contract/hook-to-kind-map/permissionrequest sha=34e950ddcb79 v=holds judge=sonnet head=8834195b0683
at: crates/viola-agent-claude/src/dialog.rs:103-105
quote: None if event == HookEvent::PermissionRequest => ( DialogKind::Permission, json!({"tool": name, "input": input}),
at: crates/viola-agent-claude/src/dialog.rs:114
quote: continuation: tool.is_some() && event == HookEvent::PermissionRequest,
at: src/run/dialog.rs:243-247
quote: if raise.continuation && let Some(answer) = continued(&mut state, &raise) { return Ok(answer); }
@row_add n=107 row=contract/hook-to-kind-map/stop area=conventions rec=fct-ao3imeon@1
boundary: Hook to kind map
item: Stop
promise: Stop maps to `turn-ended`.
@row_check n=107 row=contract/hook-to-kind-map/stop sha=ccec2712686d v=holds judge=sonnet head=8834195b0683
at: crates/viola-agent-claude/src/hook.rs:72
quote: Self::Stop => Some(EventKind::TurnEnded),
@row_add n=108 row=contract/hook-to-kind-map/sessionend area=conventions rec=fct-ao3imeon@1
boundary: Hook to kind map
item: SessionEnd
promise: SessionEnd maps to `session-end`.
@row_check n=108 row=contract/hook-to-kind-map/sessionend sha=3ae01f6674d1 v=holds judge=sonnet head=8834195b0683
at: crates/viola-agent-claude/src/hook.rs:73
quote: Self::SessionEnd => Some(EventKind::SessionEnd),
@row_add n=109 row=contract/hook-to-kind-map/notification area=conventions rec=fct-ao3imeon@1
boundary: Hook to kind map
item: Notification
promise: Notification maps to `activity`, which is log-only and never wakes `wait`.
@row_check n=109 row=contract/hook-to-kind-map/notification sha=54681e62a344 v=holds judge=sonnet head=8834195b0683
at: crates/viola-agent-claude/src/hook.rs:74-75
quote: Self::Notification | Self::PostToolUse | Self::PostToolUseFailure => { Some(EventKind::Activity)
at: crates/viola-core/src/lib.rs:99-105
quote: pub const WAIT_WAKE: [Self; 6] = [ Self::TurnEnded, Self::Question, Self::Permission, Self::Plan, Self::SessionEnd, Self::DialogExpired,
@row_add n=110 row=contract/hook-to-kind-map/posttooluse area=conventions rec=fct-ao3imeon@1
boundary: Hook to kind map
item: PostToolUse
promise: PostToolUse maps to `activity`, which is log-only and never wakes `wait`.
@row_check n=110 row=contract/hook-to-kind-map/posttooluse sha=75a035624ccf v=holds judge=sonnet head=8834195b0683
at: crates/viola-agent-claude/src/hook.rs:74-75
quote: Self::Notification | Self::PostToolUse | Self::PostToolUseFailure => { Some(EventKind::Activity)
at: crates/viola-core/src/lib.rs:99-105
quote: pub const WAIT_WAKE: [Self; 6] = [ Self::TurnEnded, Self::Question, Self::Permission, Self::Plan, Self::SessionEnd, Self::DialogExpired,
@row_add n=111 row=contract/hook-to-kind-map/posttoolusefailure area=conventions rec=fct-ao3imeon@1
boundary: Hook to kind map
item: PostToolUseFailure
promise: PostToolUseFailure maps to `activity`, which is log-only and never wakes `wait`.
@row_check n=111 row=contract/hook-to-kind-map/posttoolusefailure sha=8a2fcdd2fd0b v=holds judge=sonnet head=8834195b0683
at: crates/viola-agent-claude/src/hook.rs:74-75
quote: Self::Notification | Self::PostToolUse | Self::PostToolUseFailure => { Some(EventKind::Activity)
at: crates/viola-core/src/lib.rs:99-105
quote: pub const WAIT_WAKE: [Self; 6] = [ Self::TurnEnded, Self::Question, Self::Permission, Self::Plan, Self::SessionEnd, Self::DialogExpired,
@row_add n=112 row=contract/hook-to-kind-map/hook-statusline area=conventions rec=fct-ao3imeon@1
boundary: Hook to kind map
item: hook statusline
promise: `hook statusline` emits no event.
@row_check n=112 row=contract/hook-to-kind-map/hook-statusline sha=d9b69ed10feb v=holds judge=sonnet head=8834195b0683
at: src/cmd/hook/statusline.rs:126-127
quote: let budget_written = reading.is_some_and(|reading| write_budget(&instance.home, &reading, Utc::now()).is_ok());
at: src/cmd/hook/statusline.rs:143
quote: let _ = io.out.write_all(&stdout).and_then(|()| io.out.flush());
@row_add n=113 row=decision/fake-agent-test-seams area=conventions rec=fct-qqxg3ika@1
name: Fake-agent test seam variables
rule: The test seams `FAKE_AGENT_PUMP_DELAY_MS` (capped at 5 s) and `FAKE_AGENT_HOOK_PANIC` (fires only on the exact value `1`) are read only under the `fake-agent` feature and are absent from release builds.
why: Neither product plumbing nor harness-only, so each carries neither prefix; a ratified exception.
@row_check n=113 row=decision/fake-agent-test-seams sha=1a6b187e97b7 v=holds judge=sonnet head=8834195b0683
at: src/cmd/run.rs:61-67
quote: #[cfg(feature = "fake-agent")] fn hold_pump_start() { let ms = std::env::var("FAKE_AGENT_PUMP_DELAY_MS") .ok() .and_then(|v| v.parse::<u64>().ok()); if let Some(ms) = ms { std::thread::sleep(std::time::Duration::from_millis(ms.min(5_000)));
at: src/cmd/hook/seam.rs:16-18
quote: #[cfg(feature = "fake-agent")] fn panic_on(value: Option<OsString>) { if value.as_deref() == Some(OsStr::new("1")) {
@row_add n=114 row=decision/on-disk-file-naming area=conventions rec=fct-2ejf4aiq@1
name: On-disk file naming
rule: On-disk files are lower-case, `.ndjson` for logs, `.json` for snapshots and `.lock` for lock files, one lock file per guarded file named `<name>.lock`.
@row_check n=114 row=decision/on-disk-file-naming sha=ae12114f424d v=holds judge=sonnet head=8834195b0683
at: crates/viola-state/src/events.rs:18
quote: pub const EVENTS: &str = "events.ndjson";
at: crates/viola-state/src/events.rs:19
quote: const EVENTS_LOCK: &str = "events.ndjson.lock";
at: crates/viola-state/src/budget.rs:18
quote: const BUDGET_LOCK: &str = "budget.json.lock";
@row_add n=115 row=decision/cli-flag-and-subcommand-style area=conventions rec=fct-7ikxzg5k@1
name: CLI flag and subcommand style
rule: CLI flags are kebab-case long options, subcommands are single lower-case words, and verb arguments mirror the channel `params`.
@row_check n=115 row=decision/cli-flag-and-subcommand-style sha=2420b6ae571a v=holds judge=sonnet head=8834195b0683
at: src/cmd/mod.rs:53
quote: #[arg(long, global = true)]
at: src/cmd/revive.rs:36
quote: #[arg(long, conflicts_with_all = ["id", "fork", "extra"])]
at: src/cmd/mod.rs:62
quote: Run(run::RunArgs),
@row_add n=116 row=contract/cli-verbs/send area=conventions rec=fct-7ikxzg5k@1
boundary: CLI verbs
item: send
promise: `viola send <target>` takes its text from stdin or `--file`.
@row_check n=116 row=contract/cli-verbs/send sha=4f7d7460b801 v=holds judge=sonnet head=8834195b0683
at: src/cmd/send.rs:159-162
quote: let text = match &args.file { Some(path) => File::open(path).ok().and_then(read_text), None => read_text(io::stdin().lock()), };
@row_add n=117 row=contract/cli-verbs/wait area=conventions rec=fct-7ikxzg5k@1
boundary: CLI verbs
item: wait
promise: `viola wait <target> [--after <cursor>] [--timeout-ms <n>]`.
@row_check n=117 row=contract/cli-verbs/wait sha=fd50469130e4 v=holds judge=sonnet head=8834195b0683
at: src/cmd/wait.rs:25-26
quote: #[arg(long, value_name = "CURSOR")] after: Option<u64>,
at: src/cmd/wait.rs:28-29
quote: #[arg(long, value_name = "MS")] timeout_ms: Option<u64>,
@row_add n=118 row=contract/cli-verbs/last area=conventions rec=fct-7ikxzg5k@1
boundary: CLI verbs
item: last
promise: `viola last <target>`.
@row_check n=118 row=contract/cli-verbs/last sha=40115ee8773c v=holds judge=sonnet head=8834195b0683
at: src/cmd/last.rs:22-23
quote: #[arg(value_parser = super::send::parse_name)] pub(super) name: ViolaName,
@row_add n=119 row=contract/cli-verbs/answer area=conventions rec=fct-7ikxzg5k@1
boundary: CLI verbs
item: answer
promise: `viola answer <target> <dialog_id>` takes the `response` object as JSON from stdin or `--file`.
@row_check n=119 row=contract/cli-verbs/answer sha=603475fa187d v=holds judge=sonnet head=8834195b0683
at: src/cmd/answer.rs:86-89
quote: let response = match &args.file { Some(path) => File::open(path).ok().and_then(read_response), None => read_response(io::stdin().lock()), };
@row_add n=120 row=contract/cli-verbs/pause area=conventions rec=fct-7ikxzg5k@1
boundary: CLI verbs
item: pause
promise: `viola pause <target>`.
@row_check n=120 row=contract/cli-verbs/pause sha=3d7d534fab80 v=holds judge=sonnet head=8834195b0683
at: src/cmd/pause.rs:24-25
quote: #[arg(value_parser = super::send::parse_name)] pub(super) name: ViolaName,
@row_add n=121 row=contract/cli-verbs/release area=conventions rec=fct-7ikxzg5k@1
boundary: CLI verbs
item: release
promise: `viola release <target> [--budget]`.
@row_check n=121 row=contract/cli-verbs/release sha=f08ed28c0dd1 v=holds judge=sonnet head=8834195b0683
at: src/cmd/release.rs:27-28
quote: #[arg(long)] budget: bool,
@row_add n=122 row=contract/cli-verbs/link area=conventions rec=fct-7ikxzg5k@1
boundary: CLI verbs
item: link
promise: `viola link <driver> <driven>`.
@row_check n=122 row=contract/cli-verbs/link sha=3bee666b24b4 v=holds judge=sonnet head=8834195b0683
at: src/cmd/link.rs:24
quote: pub(super) driver: ViolaName,
at: src/cmd/link.rs:27
quote: pub(super) driven: ViolaName,
@row_add n=123 row=contract/cli-verbs/unlink area=conventions rec=fct-7ikxzg5k@1
boundary: CLI verbs
item: unlink
promise: `viola unlink <driver> <driven>`.
@row_check n=123 row=contract/cli-verbs/unlink sha=7dee2cb99c25 v=holds judge=sonnet head=8834195b0683
at: src/cmd/mod.rs:82
quote: Unlink(link::LinkArgs),
@row_add n=124 row=contract/cli-verbs/revive area=conventions rec=fct-7ikxzg5k@1
boundary: CLI verbs
item: revive
promise: `viola revive <name> [--id <ID>] [--fork] [-- <child args>]` and `viola revive <name> --list` send no channel frame; `--list` conflicts with `--id`, `--fork` and the trailing arguments.
@row_check n=124 row=contract/cli-verbs/revive sha=1549a850ecff v=holds judge=sonnet head=8834195b0683
at: src/cmd/revive.rs:36
quote: #[arg(long, conflicts_with_all = ["id", "fork", "extra"])]
at: src/cmd/revive.rs:116-118
quote: if args.list { return list(home, &args.name, args.json); }
@row_add n=125 row=contract/cli-verbs/from area=conventions rec=fct-7ikxzg5k@1
boundary: CLI verbs
item: from
promise: Like the MCP server, the CLI adds `from` from its own `VIOLA_NAME` when set.
@row_check n=125 row=contract/cli-verbs/from sha=c83598343410 v=holds judge=sonnet head=8834195b0683
at: src/cmd/client.rs:209
quote: pub(crate) fn own_name() -> Option<ViolaName> {
at: src/cmd/send.rs:203-204
quote: if let Some(from) = own_name() { params.insert("from".to_owned(), from.as_ref().into());
at: crates/viola-mcp/src/tools.rs:319
quote: params.insert("from".to_owned(), Value::from(AsRef::<str>::as_ref(from)));
@row_add n=126 row=decision/viola-name-instance-key area=conventions rec=fct-tnxtyezf@1
name: ViolaName as the instance key
rule: The instance key is `ViolaName`: ASCII `[a-z0-9-]`, 1 to 32 characters, starting with a letter, enforced by nutype.
why: No database, so no UUID or serial keys.
@row_check n=126 row=decision/viola-name-instance-key sha=ef89418338d6 v=holds judge=sonnet head=8834195b0683
at: crates/viola-core/src/lib.rs:52
quote: pub struct ViolaName(String);
at: crates/viola-core/src/lib.rs:389
quote: bytes.len() <= 32
@row_add n=127 row=decision/endpoint-name-stable-hash area=conventions rec=fct-zcqzutrw@1
name: Endpoint name from a stable hand-written hash
rule: The endpoint name is `viola-` plus the first 12 hex characters of a hand-written FNV-1a 64-bit hash over `ViolaName`, a NUL and the absolute viola home path; std `DefaultHasher` is forbidden.
why: `DefaultHasher` output is not stable across Rust releases, and mixed binary versions must agree on the name.
@row_check n=127 row=decision/endpoint-name-stable-hash sha=20e58925dcb8 v=holds judge=sonnet head=8834195b0683
at: crates/viola-channel/src/endpoint.rs:30
quote: let hash = fnv1a(name.bytes().chain([0]).chain(home.bytes()));
at: crates/viola-channel/src/endpoint.rs:19
quote: fn fnv1a(bytes: impl IntoIterator<Item = u8>) -> u64 {
@row_add n=128 row=decision/timestamp-format area=conventions rec=fct-u2cu7pdd@1
name: Timestamp format
rule: Timestamps are always RFC 3339 UTC with milliseconds and a `Z` suffix, as in `2026-09-23T19:43:58.123Z`.
@row_check n=128 row=decision/timestamp-format sha=ad1c29b897eb v=holds judge=sonnet head=8834195b0683
at: crates/viola-state/src/lib.rs:31
quote: at.to_rfc3339_opts(SecondsFormat::Millis, true)
@row_add n=129 row=decision/absent-optional-fields-omitted area=conventions rec=fct-u5xw5irn@1
name: Absent optional fields omitted on write
rule: viola omits an absent optional field on write rather than writing `null`.
@row_check n=129 row=decision/absent-optional-fields-omitted sha=684ecf515199 v=holds judge=sonnet head=8834195b0683
at: crates/viola-state/src/snapshot.rs:40
quote: #[serde(default, skip_serializing_if = "Option::is_none")]
@row_add n=130 row=decision/missing-equals-null-on-read area=conventions rec=fct-u5xw5irn@1
name: Missing and null read the same
rule: Readers treat a missing field and a `null` field the same.
@row_check n=130 row=decision/missing-equals-null-on-read sha=ba1698feb281 v=holds judge=sonnet head=8834195b0683
at: crates/viola-state/src/snapshot.rs:40
quote: #[serde(default, skip_serializing_if = "Option::is_none")]
@row_add n=131 row=decision/unparseable-external-values-unknown area=conventions rec=fct-iskzo3uv@1
name: Unparseable external values become unknown
rule: An unparseable external value becomes the string `"unknown"` in viola's own records, never an error, covering `resets_at`, `used_percentage` and a missing statusline window.
@row_check n=131 row=decision/unparseable-external-values-unknown sha=2be7ea468d32 v=holds judge=sonnet head=8834195b0683
at: crates/viola-agent-claude/src/statusline.rs:43
quote: return Reading::UNKNOWN;
at: crates/viola-agent-claude/src/statusline.rs:54
quote: _ => Reading::UNKNOWN,
at: crates/viola-agent-claude/src/statusline.rs:74
quote: _ => Reading::UNKNOWN,
@row_add n=132 row=decision/ndjson-line-discipline area=conventions rec=fct-2ajbpybj@1
name: ndjson line discipline
rule: Each ndjson line is one complete JSON object plus `\n` in a single `write` call; multi-line text travels as an escaped JSON string on one line.
@row_check n=132 row=decision/ndjson-line-discipline sha=1b056f6c435a v=holds judge=sonnet head=8834195b0683
at: crates/viola-state/src/events.rs:335
quote: file.write_all(&[b"\n", bytes].concat())?;
at: crates/viola-state/src/events.rs:64
quote: /// Appends `line` to `<instance_dir>/events.ndjson` with a single `write_all`.
@row_add n=133 row=absence/graphical-interface area=standard-contracts rec=fct-ygzlcqsl@2
capability: A web or graphical surface
@row_check n=133 row=absence/graphical-interface sha=e2aaf7e5f73a v=holds judge=sonnet head=8834195b0683
searched: egui|tauri|winit|iced|ratatui · in Cargo.toml, crates/*/Cargo.toml, src/main.rs, src/cmd/mod.rs
searched: enum Command (every subcommand: Run Revive Send Wait Last Answer List Pause Release Link Unlink Verify Plugin Hook Mcp, no gui or ui) · in src/cmd/mod.rs
@row_add n=134 row=decision/list-without-pagination area=standard-contracts rec=fct-oy7hacpg@1
name: list without pagination
rule: The `list` payload has no pagination; lists are returned whole.
why: Lists are personal-scale.
@row_check n=134 row=decision/list-without-pagination sha=d443728f56bc v=holds judge=sonnet head=8834195b0683
at: src/cmd/list.rs:178
quote: json!({"v": 1, "ok": {"items": board.items, "budget": budget, "skipped": board.skipped}})
@row_add n=135 row=contract/list-payload/skipped area=standard-contracts rec=fct-oy7hacpg@1
boundary: list payload
item: skipped
promise: `skipped` counts records a reader could not interpret, so the board can show them.
@row_check n=135 row=contract/list-payload/skipped sha=f4fe0ab5c257 v=holds judge=sonnet head=8834195b0683
at: crates/viola-state/src/events.rs:258
quote: self.skipped.torn_lines += 1;
at: src/cmd/list.rs:178
quote: json!({"v": 1, "ok": {"items": board.items, "budget": budget, "skipped": board.skipped}})
@row_add n=136 row=contract/list-payload/items area=standard-contracts rec=fct-beanu3w4@1,fct-oy7hacpg@1
boundary: list payload
item: items
promise: Each `items` entry is `{name, wrapped, liveness, status, wheel?, budget_paused?, dialog_pending, cli_version?, cli_verified?}`.
@row_check n=136 row=contract/list-payload/items sha=677440008fe6 v=holds judge=sonnet head=8834195b0683
at: crates/viola-state/src/board.rs:168-176
quote: pub name: String, pub wrapped: bool, pub liveness: RowLiveness, pub status: SessionStatus, #[serde(skip_serializing_if = "Option::is_none")] pub wheel: Option<Wheel>, #[serde(skip_serializing_if = "Option::is_none")] pub budget_paused: Option<bool>, pub dialog_pending: bool,
at: crates/viola-state/src/board.rs:177-180
quote: #[serde(skip_serializing_if = "Option::is_none")] pub cli_version: Option<String>, #[serde(skip_serializing_if = "Option::is_none")] pub cli_verified: Option<bool>,
@row_add n=137 row=contract/list-payload/liveness area=standard-contracts rec=fct-beanu3w4@1,fct-oy7hacpg@1
boundary: list payload
item: liveness
promise: An entry's `liveness` is `live` or `stale`; gone instances are omitted.
@row_check n=137 row=contract/list-payload/liveness sha=b1d93dad8209 v=holds judge=sonnet head=8834195b0683
at: crates/viola-state/src/board.rs:276-278
quote: Liveness::Live => Some(RowLiveness::Live), Liveness::Stale => Some(RowLiveness::Stale), Liveness::Gone => None,
@row_add n=138 row=contract/list-payload/wrapped area=standard-contracts rec=fct-beanu3w4@1,fct-oy7hacpg@1
boundary: list payload
item: wrapped
promise: `wrapped: false` marks a read-only unwrapped row from `claude agents --json`, never a valid `target`.
@row_check n=138 row=contract/list-payload/wrapped sha=28b8d8bbf30b v=holds judge=sonnet head=8834195b0683
at: crates/viola-state/src/board.rs:194
quote: wrapped: false,
@row_add n=139 row=contract/event-line/shape area=standard-contracts rec=fct-jzfs52nf@1,fct-anjjkm64@1
boundary: ndjson event line
item: shape
promise: Each line of an instance's `events.ndjson` is a JSON object with `v`, `ts`, `instance`, `kind`, `source` and `data`.
@row_check n=139 row=contract/event-line/shape sha=59b49099a4d0 v=holds judge=sonnet head=8834195b0683
at: crates/viola-state/src/events.rs:140
quote: const LINE_KEYS: [&str; 6] = ["v", "ts", "instance", "kind", "source", "data"];
@row_add n=140 row=contract/event-line/source area=standard-contracts rec=fct-lzgtihuf@1,fct-anjjkm64@1
boundary: ndjson event line
item: source
promise: `source` is one of `hook` · `wrapper`.
@row_check n=140 row=contract/event-line/source sha=4871b8777dfa v=stale judge=sonnet head=8834195b0683
at: crates/viola-state/src/events.rs:26
quote: Wrapper,
at: crates/viola-state/src/events.rs:27
quote: Hook,
shows: The event line's `source` has only the values `wrapper` and `hook`; no code emits `cli`.
cell: promise
value: `source` is one of `hook` · `wrapper`.
@row_add n=141 row=contract/event-line/data area=standard-contracts rec=fct-lzgtihuf@1,fct-anjjkm64@1
boundary: ndjson event line
item: data
promise: `data` is kind-specific, uses only viola's normalised fields, and never embeds a raw Claude payload.
@row_check n=141 row=contract/event-line/data sha=e1ed5f316b9a v=unverifiable judge=sonnet head=8834195b0683
searched: data: Value · in crates/viola-state/src/events.rs (a free-form serde_json Value, no code there that restricts it to normalised fields)
why: no evidence found
@row_add n=142 row=contract/snapshot-envelope/unreadable-snapshot area=standard-contracts rec=fct-vufggymy@1
boundary: snapshot envelope
item: unreadable snapshot
promise: A snapshot whose `v` is higher than the reader supports, or that fails to parse, is ignored and state is rebuilt by replaying `events.ndjson`, recovering only `links`, `agent_session_id`, `wheel`, `budget_paused`, `budget_override_until`; the replay writes no file.
@row_check n=142 row=contract/snapshot-envelope/unreadable-snapshot sha=815aa01f354b v=holds judge=sonnet head=8834195b0683
at: crates/viola-state/src/snapshot.rs:140-141
quote: if v > u64::from(SNAPSHOT_V) { return SnapshotRead::Unsupported { v_seen: v };
at: crates/viola-state/src/replay.rs:154-157
quote: SnapshotRead::Unreadable => ReplayCause::Unreadable, SnapshotRead::Unsupported { v_seen } => ReplayCause::Unsupported { v_seen }, }; let state = replay(instance_dir)?;
at: crates/viola-state/src/replay.rs:59
quote: Some(EventKind::Wheel) => replayed.wheel = holder(&data["holder"]),
at: crates/viola-state/src/replay.rs:61
quote: replayed.budget_paused = data["paused"].as_bool();
@row_add n=143 row=contract/ledger-stamps/content area=standard-contracts rec=fct-scthzguh@1
boundary: ledger stamps envelope
item: content
promise: Per CLI version, the envelope holds each ledger row's `pass` or `fail` and the probe's measured values, of which `run` reads none.
@row_check n=143 row=contract/ledger-stamps/content sha=ac35ed5d4d06 v=holds judge=sonnet head=8834195b0683
at: crates/viola-agent-claude/src/ledger.rs:1125
quote: let rows: Map<String, Value> = results
at: crates/viola-agent-claude/src/ledger.rs:1130
quote: json!(if *pass { "pass" } else { "fail" }),
at: crates/viola-agent-claude/src/ledger.rs:1141-1142
quote: "measured": { "largest_hook_payload": measured,
at: crates/viola-agent-claude/src/ledger.rs:1236
quote: let rows = &doc["data"]["versions"][version]["rows"];
@row_add n=144 row=contract/ledger-stamps/verified-version area=standard-contracts rec=fct-scthzguh@1
boundary: ledger stamps envelope
item: verified version
promise: A CLI version is verified only when every relied ledger row reads `pass`.
@row_check n=144 row=contract/ledger-stamps/verified-version sha=87d990c1a197 v=holds judge=sonnet head=8834195b0683
at: crates/viola-agent-claude/src/ledger.rs:1237-1239
quote: Ok(LedgerRow::relied() .iter() .all(|row| rows[row.id()] == "pass"))
@row_add n=145 row=contract/ledger-stamps/malformed-envelope area=standard-contracts rec=fct-scthzguh@1
boundary: ledger stamps envelope
item: malformed envelope
promise: A malformed envelope makes `run`'s gate record `cli_verified:false`.
@row_check n=145 row=contract/ledger-stamps/malformed-envelope sha=75a6db519be7 v=holds judge=sonnet head=8834195b0683
at: crates/viola-agent-claude/src/ledger.rs:1233-1234
quote: if !shaped { return Err(AgentError::StampsMalformed);
at: src/run/version_gate.rs:208
quote: Err(_) => (false, Some("malformed")),
@row_add n=146 row=contract/instance-snapshot/data area=standard-contracts rec=fct-q5tbsmkw@1
boundary: instance snapshot
item: data
promise: `data` is `{endpoint?, pid, started_at, pinned_bin, statusline_command?, cli_version?, cli_verified, wheel, budget_paused, budget_override_until?, pending_dialog?, links, child_pid?, agent_session_id?, cwd?}`.
@row_check n=146 row=contract/instance-snapshot/data sha=1d261aa1c54d v=holds judge=sonnet head=8834195b0683
at: crates/viola-state/src/snapshot.rs:39
quote: pub struct InstanceSnapshot {
at: crates/viola-state/src/snapshot.rs:72
quote: pub agent_session_id: Option<String>,
@row_add n=147 row=contract/instance-snapshot/writer area=standard-contracts rec=fct-q5tbsmkw@1
boundary: instance snapshot
item: writer
promise: `instances/<ViolaName>/snapshot.json` is written only by that instance's wrapper and rewritten on every change to a field of its `data`.
@row_check n=147 row=contract/instance-snapshot/writer sha=023c0caad9a6 v=holds judge=sonnet head=8834195b0683
at: src/run/snapshot.rs:47
quote: write_snapshot(&self.instance_dir, snapshot)
@row_add n=148 row=contract/instance-snapshot/started-at area=standard-contracts rec=fct-q5tbsmkw@1
boundary: instance snapshot
item: started_at
promise: `started_at` is the wrapper's own OS start time in whole seconds rendered RFC 3339, so a reader can compare it with the same field of the recorded `pid`.
@row_check n=148 row=contract/instance-snapshot/started-at sha=c2e4f86d5d33 v=holds judge=sonnet head=8834195b0683
at: crates/viola-state/src/liveness.rs:62
quote: DateTime::from_timestamp(secs, 0).map(crate::timestamp)
at: src/cmd/run.rs:600
quote: started_at: own_start(pid).context("own start time unreadable")?,
@row_add n=149 row=contract/instance-snapshot/endpoint area=standard-contracts rec=fct-q5tbsmkw@1
boundary: instance snapshot
item: endpoint
promise: `endpoint` is present from the first snapshot, because the bind precedes it; only a replay rebuild lacks it.
@row_check n=149 row=contract/instance-snapshot/endpoint sha=168ffe0c37e4 v=holds judge=sonnet head=8834195b0683
at: src/cmd/run.rs:598
quote: endpoint: Some(endpoint),
at: src/cmd/run.rs:500
quote: match Server::bind(&endpoint) {
@row_add n=150 row=contract/instance-snapshot/spawn-order area=standard-contracts rec=fct-q5tbsmkw@1
boundary: instance snapshot
item: spawn order
promise: The first snapshot is written before the spawn without `child_pid`; the rewrite straight after the spawn adds it.
@row_check n=150 row=contract/instance-snapshot/spawn-order sha=ffa371f4534c v=holds judge=sonnet head=8834195b0683
at: src/cmd/run.rs:608
quote: child_pid: None,
at: src/cmd/run.rs:652
quote: snapshots.update(|s| s.child_pid = child_pid)?;
@row_add n=151 row=contract/instance-snapshot/pid-and-started-at area=standard-contracts rec=fct-ld365mzq@2,fct-q5tbsmkw@1
boundary: instance snapshot
item: pid and started_at
promise: `pid` and `started_at` identify the wrapper process for the liveness check and for the server check every frame-writing client makes before its first frame; a mismatch with the connected peer writes no frame.
@row_check n=151 row=contract/instance-snapshot/pid-and-started-at sha=ac7e6af24578 v=holds judge=sonnet head=8834195b0683
at: src/cmd/client.rs:180
quote: let verdict = verify_server(live.pid, &live.started_at, client.server_pid());
at: src/cmd/hook.rs:239
quote: let verdict = verify_server(snapshot.pid, &snapshot.started_at, client.server_pid());
at: crates/viola-mcp/src/tools.rs:363
quote: tokio::task::spawn_blocking(move || verify_server(live.pid, &live.started_at, observed))
@row_add n=152 row=contract/instance-snapshot/child-pid area=standard-contracts rec=fct-n4ricmno@1,fct-q5tbsmkw@1
boundary: instance snapshot
item: child_pid
promise: `child_pid` is the pid of the spawned agent child and exists only to join `claude agents --json` rows to this instance.
@row_check n=152 row=contract/instance-snapshot/child-pid sha=be27307744af v=holds judge=sonnet head=8834195b0683
at: src/cmd/run.rs:650
quote: let child_pid = viola_pty::Pty::child_pid(&pty);
@row_add n=153 row=contract/instance-snapshot/agent-session-id area=standard-contracts rec=fct-n4ricmno@1,fct-q5tbsmkw@1
boundary: instance snapshot
item: agent_session_id
promise: `agent_session_id` is the one from the newest `session-start` event and exists only to join `claude agents --json` rows to this instance.
@row_check n=153 row=contract/instance-snapshot/agent-session-id sha=ad2d7d1da84f v=holds judge=sonnet head=8834195b0683
at: src/run/send/slot.rs:226
quote: snapshots.update(|snapshot| snapshot.agent_session_id = Some(id.to_owned()))?;
@row_add n=154 row=contract/instance-snapshot/cwd area=standard-contracts rec=fct-27cqwcgd@1,fct-q5tbsmkw@1
boundary: instance snapshot
item: cwd
promise: `cwd` is the directory the start spawned its child in, written into the first snapshot when valid UTF-8 and omitted otherwise; its one reader is `viola revive`, which uses it only as the revived child's spawn directory.
@row_check n=154 row=contract/instance-snapshot/cwd sha=479cd154bb68 v=holds judge=sonnet head=8834195b0683
at: src/cmd/run.rs:610
quote: cwd: recorded.spawn_dir.to_str().map(str::to_owned),
at: src/cmd/revive.rs:260
quote: .cwd
@row_add n=155 row=decision/spawn-directory-confined-to-snapshot area=standard-contracts rec=fct-27cqwcgd@1,fct-q5tbsmkw@1
name: Spawn directory confined to the snapshot
rule: The spawn directory host path lives only in the owner-only (0600) instance snapshot, on no event, log line, error body or stdout.
@row_check n=155 row=decision/spawn-directory-confined-to-snapshot sha=18eb96ec44bd v=holds judge=sonnet head=8834195b0683
at: src/cmd/run.rs:610
quote: cwd: recorded.spawn_dir.to_str().map(str::to_owned),
@row_add n=156 row=contract/instance-snapshot/statusline-command area=standard-contracts rec=fct-st4h3pj7@1,fct-q5tbsmkw@1
boundary: instance snapshot
item: statusline_command
promise: `statusline_command` is the user's statusline command that `run` read from the named source at start, omitted when the source gave none; no event carries it, and `hook statusline` reads it and never changes the snapshot.
@row_check n=156 row=contract/instance-snapshot/statusline-command sha=7c9f2fdb8588 v=holds judge=sonnet head=8834195b0683
at: src/cmd/run.rs:611
quote: statusline_command: recorded.statusline_command,
at: src/cmd/hook/statusline.rs:130
quote: Ok(Recovered::Snapshot(snapshot)) => snapshot.statusline_command,
@row_add n=157 row=contract/instance-snapshot/cli-version area=standard-contracts rec=fct-gbttz3aj@1,fct-q5tbsmkw@1
boundary: instance snapshot
item: cli_version
promise: `cli_version` comes from `run`'s version gate.
@row_check n=157 row=contract/instance-snapshot/cli-version sha=889fdf18a78d v=holds judge=sonnet head=8834195b0683
at: src/cmd/run.rs:603
quote: cli_version: gate.cli_version,
@row_add n=158 row=contract/instance-snapshot/cli-verified area=standard-contracts rec=fct-gbttz3aj@1,fct-q5tbsmkw@1
boundary: instance snapshot
item: cli_verified
promise: `cli_verified` comes from `run`'s version gate.
@row_check n=158 row=contract/instance-snapshot/cli-verified sha=12c6b2e3cd6e v=holds judge=sonnet head=8834195b0683
at: src/cmd/run.rs:602
quote: cli_verified: gate.cli_verified,
@row_add n=159 row=contract/instance-snapshot/pending-dialog area=standard-contracts rec=fct-yqoy7mew@1,fct-q5tbsmkw@1
boundary: instance snapshot
item: pending_dialog
promise: `{dialog_id, kind}` while a `hook.dialog` is open, written before its event line and reset when that append fails, so a reader that sees the line sees it; removed on answer, expiry or a `null` answer on a wheel move; never rebuilt from the log.
@row_check n=159 row=contract/instance-snapshot/pending-dialog sha=c90abc6e369c v=holds judge=sonnet head=8834195b0683
at: src/run/dialog.rs:297
quote: self.write_pending(Some(PendingDialog {
at: src/run/dialog.rs:307
quote: self.write_pending(None);
at: src/run/dialog.rs:360
quote: self.write_pending(None);
@row_add n=160 row=contract/list-payload/dialog-pending area=standard-contracts rec=fct-yqoy7mew@1
boundary: list payload
item: dialog_pending
promise: `dialog_pending` comes only from the snapshot's `pending_dialog`, so a reader that falls back to replay reports `dialog_pending: false`.
@row_check n=160 row=contract/list-payload/dialog-pending sha=3210eedf1732 v=holds judge=sonnet head=8834195b0683
at: crates/viola-state/src/board.rs:239
quote: dialog_pending: snapshot.pending_dialog.is_some(),
at: crates/viola-state/src/board.rs:294
quote: dialog_pending: false,
@row_add n=161 row=contract/wrapper-channel-frames/envelope area=standard-contracts rec=fct-5wbp3lzt@1,fct-kdiruxnb@1
boundary: wrapper channel frames
item: envelope
promise: Frames are JSON-RPC 2.0, one per line, and a request's `params` carry `v` and `sender`.
@row_check n=161 row=contract/wrapper-channel-frames/envelope sha=638a52e195a4 v=holds judge=sonnet head=8834195b0683
at: crates/viola-channel/src/frame.rs:95
quote: json!({"jsonrpc": "2.0", "id": id, "result": result})
at: crates/viola-channel/src/frame.rs:45-46
quote: line.push('\n'); writer.write_all(line.as_bytes()).map_err(ChannelError::Io)
at: crates/viola-channel/src/client.rs:114
quote: params.insert("v".to_owned(), PROTOCOL_V.into());
at: crates/viola-channel/src/client.rs:115
quote: params.insert("sender".to_owned(), VERSION.into());
@row_add n=162 row=contract/wrapper-channel-frames/ok-result area=standard-contracts rec=fct-5wbp3lzt@1,fct-kdiruxnb@1
boundary: wrapper channel frames
item: ok result
promise: A success is answered as `result` holding `{"ok": <payload>}`.
@row_check n=162 row=contract/wrapper-channel-frames/ok-result sha=dee80c9f28b8 v=holds judge=sonnet head=8834195b0683
at: src/run/send.rs:215
quote: Ok(json!({"ok": {"submitted_at": submitted_at, "cursor": cursor}}))
at: crates/viola-channel/src/frame.rs:95
quote: json!({"jsonrpc": "2.0", "id": id, "result": result})
@row_add n=163 row=contract/wrapper-channel-frames/refusal-result area=standard-contracts rec=fct-5wbp3lzt@1,fct-kdiruxnb@1
boundary: wrapper channel frames
item: refusal result
promise: A refusal is answered as `result` holding `{"refusal": <reason>, "detail": <detail>}`.
@row_check n=163 row=contract/wrapper-channel-frames/refusal-result sha=fb7bf632dd22 v=holds judge=sonnet head=8834195b0683
at: src/run/dialog.rs:146
quote: json!({"refusal": reason.as_str(), "detail": detail})
at: crates/viola-channel/src/frame.rs:95
quote: json!({"jsonrpc": "2.0", "id": id, "result": result})
@row_add n=164 row=contract/wrapper-channel-frames/unsupported-protocol-version area=standard-contracts rec=fct-5wbp3lzt@1,fct-kdiruxnb@1
boundary: wrapper channel frames
item: unsupported protocol version
promise: An unsupported protocol version is a JSON-RPC error `-32602` with message `unsupported protocol version` and `data` `{supported, wrapper}`.
@row_check n=164 row=contract/wrapper-channel-frames/unsupported-protocol-version sha=0e19d27cbaef v=holds judge=sonnet head=8834195b0683
at: crates/viola-channel/src/server.rs:238-239
quote: if params.v > PROTOCOL_V { return Err(ProtocolError::UnsupportedVersion);
at: crates/viola-channel/src/lib.rs:84
quote: Self::UnsupportedVersion => "unsupported protocol version",
at: crates/viola-channel/src/lib.rs:95
quote: Self::UnsupportedVersion => json!({"supported": PROTOCOL_V, "wrapper": VERSION}),
@row_add n=165 row=contract/wrapper-channel-frames/max-frame area=standard-contracts rec=fct-q4hzbk47@1
boundary: wrapper channel frames
item: MAX_FRAME
promise: A frame is one line of at most `MAX_FRAME` bytes, its `\n` counted; a longer line is answered `-32600` and the connection is closed.
@row_check n=165 row=contract/wrapper-channel-frames/max-frame sha=fc97af48b963 v=holds judge=sonnet head=8834195b0683
at: crates/viola-channel/src/frame.rs:24-26
quote: io::Read::take(&mut *reader, cap) .read_until(b'\n', &mut line) .map_err(ChannelError::Io)?;
at: crates/viola-channel/src/server.rs:162
quote: Err(ChannelError::Oversize) => Some(refuse_oversize(srv_conn)),
at: crates/viola-channel/src/server.rs:268-269
quote: frame: error_response(None, ProtocolError::InvalidRequest), close: true,
@row_add n=166 row=contract/wrapper-channel-frames/id-less-frames area=standard-contracts rec=fct-q4hzbk47@1
boundary: wrapper channel frames
item: id-less frames
promise: Only `hook.event` may be id-less; any other frame without an `id` is not dispatched and is logged `parse-rejected{parser:"channel-frame", detail:"malformed"}`.
@row_check n=166 row=contract/wrapper-channel-frames/id-less-frames sha=e21522abfbcc v=holds judge=sonnet head=8834195b0683
at: crates/viola-channel/src/server.rs:202-204
quote: if request.id.is_none() && method != Some(NOTIFICATION) { reject_frame("malformed"); return None;
at: crates/viola-channel/src/server.rs:257-258
quote: parser = "channel-frame", detail = detail,
@row_add n=167 row=contract/wrapper-channel-frames/conn area=standard-contracts rec=fct-q4hzbk47@1
boundary: wrapper channel frames
item: conn
promise: A client may add `conn` to `params`; the server logs it and strips it before dispatch, so no method takes it as input and it is never identity.
@row_check n=167 row=contract/wrapper-channel-frames/conn sha=1c07abecd412 v=holds judge=sonnet head=8834195b0683
at: crates/viola-channel/src/server.rs:193
quote: let peer = Peer::of(params.conn.as_deref(), srv_conn);
at: crates/viola-channel/src/server.rs:196-197
quote: if let Some(fields) = request.params.as_object_mut() { fields.remove("conn");
@row_add n=168 row=contract/channel-methods/v-and-sender area=standard-contracts rec=fct-vi22dxmq@1
boundary: channel methods
item: v and sender
promise: Every channel method's `params` carries `v` and `sender`.
@row_check n=168 row=contract/channel-methods/v-and-sender sha=2d481623c3a1 v=holds judge=sonnet head=8834195b0683
at: crates/viola-channel/src/client.rs:174
quote: params.insert("v".to_owned(), PROTOCOL_V.into());
at: crates/viola-channel/src/client.rs:175
quote: params.insert("sender".to_owned(), VERSION.into());
at: crates/viola-channel/src/client.rs:114-115
quote: params.insert("v".to_owned(), PROTOCOL_V.into()); params.insert("sender".to_owned(), VERSION.into());
@row_add n=169 row=contract/channel-methods/from area=standard-contracts rec=fct-vi22dxmq@1
boundary: channel methods
item: from
promise: `from` is the caller's `VIOLA_NAME` when set, else omitted; `send`, `wait`, `last`, `answer` and `pause` take it absent, `null` or a valid `ViolaName` and answer anything else `-32602` with `data: null`, never `-32600`.
@row_check n=169 row=contract/channel-methods/from sha=be8badd48064 v=holds judge=sonnet head=8834195b0683
at: src/cmd/client.rs:210
quote: let raw = std::env::var_os("VIOLA_NAME")?.into_string().ok()?;
at: src/cmd/send.rs:203-204
quote: if let Some(from) = own_name() { params.insert("from".to_owned(), from.as_ref().into());
at: src/run/send/params.rs:15-20
quote: match params.get("from") { None | Some(Value::Null) => Ok(None), Some(Value::String(from)) => ViolaName::try_new(from.clone()) .map(Some) .map_err(|_| ProtocolError::InvalidParams), Some(_) => Err(ProtocolError::InvalidParams),
at: src/run/wait.rs:138
quote: parse_from(params)?;
at: src/run/wait.rs:174
quote: parse_from(params)?;
at: src/run/dialog.rs:419
quote: let from = parse_from(params)?;
at: src/run/wheel.rs:189
quote: parse_from(call.params)?;
@row_add n=170 row=contract/channel-methods/send-budget-paused area=standard-contracts rec=fct-ekt3xwun@1,fct-vi22dxmq@1
boundary: channel methods
item: send budget-paused
promise: A closed budget gate refuses `send` `budget-paused` before `send-issued`, with nothing typed.
@row_check n=170 row=contract/channel-methods/send-budget-paused sha=69edcd04b0f1 v=holds judge=sonnet head=8834195b0683
at: src/run/send.rs:103-109
quote: if let Some(budget) = slot.budget.get() { let crossed = budget .evaluate(Trigger::Send, Utc::now()) .map_err(|_| ProtocolError::Internal)?; if let Some(crossed) = crossed { return ctx.budget_paused(&crossed);
at: src/run/send.rs:147
quote: let cursor = ctx.issue(text.len())?;
@row_add n=171 row=contract/channel-methods/send-from-link area=standard-contracts rec=fct-ekt3xwun@1,fct-vi22dxmq@1
boundary: channel methods
item: send from link
promise: A `send` whose `from` names another instance records the link before `send-issued`.
@row_check n=171 row=contract/channel-methods/send-from-link sha=185c824dc5a2 v=holds judge=sonnet head=8834195b0683
at: src/run/send.rs:142-147
quote: if let (Some(links), Some(from)) = (slot.links.get(), ctx.from.as_ref()) && from != name { links.record(from).map_err(|_| ProtocolError::Internal)?; } let cursor = ctx.issue(text.len())?;
@row_add n=172 row=contract/channel-methods/wait-pending-dialog area=standard-contracts rec=fct-rgtvcxfi@1,fct-vi22dxmq@1
boundary: channel methods
item: wait pending dialog
promise: For `wait`, a pending dialog comes first unless `after` is past its line.
@row_check n=172 row=contract/channel-methods/wait-pending-dialog sha=f21209304e89 v=holds judge=sonnet head=8834195b0683
at: src/run/wait.rs:139-147
quote: let mut from = match after { Some(after) => after, None => { let feed = self.feed(); match feed.pending_start { Some(start) => start, None => end_offset(instance_dir).map_err(|_| ProtocolError::Internal)?,
@row_add n=173 row=contract/channel-methods/wait-wake-kinds area=standard-contracts rec=fct-rgtvcxfi@1,fct-vi22dxmq@1
boundary: channel methods
item: wait wake kinds
promise: Only `turn-ended` · `question` · `permission` · `plan` · `session-end` · `dialog-expired` wake `wait`, and a parked `wait` never polls the file.
@row_check n=173 row=contract/channel-methods/wait-wake-kinds sha=9aa0a67bd1f6 v=holds judge=sonnet head=8834195b0683
at: crates/viola-core/src/lib.rs:99-106
quote: pub const WAIT_WAKE: [Self; 6] = [ Self::TurnEnded, Self::Question, Self::Permission, Self::Plan, Self::SessionEnd, Self::DialogExpired, ];
at: src/run/wait.rs:201-205
quote: if EventKind::WAIT_WAKE .iter() .any(|k| Some(k.as_str()) == kind) { return Ok(Some(line));
at: src/run/wait.rs:158-167
quote: while feed.generation == seen { if deadline.is_some_and(|d| self.clock.now() >= d) { span.record("timed_out", true); return Ok(json!({"ok": {"timed_out": true}})); } feed = self .appended .wait_timeout(feed, STEP)
@row_add n=174 row=contract/channel-methods/last-after-seen-line area=standard-contracts rec=fct-5chyftpi@1,fct-vi22dxmq@1
boundary: channel methods
item: last after a seen line
promise: A caller who has seen a `turn-ended` line on disk always reads that turn from `last`, its value updated under the same lock hold as the append and rebuilt once from the whole `events.ndjson` at `run` start.
@row_check n=174 row=contract/channel-methods/last-after-seen-line sha=f707c5d03e9c v=holds judge=sonnet head=8834195b0683
at: src/run/wait.rs:76-80
quote: let mut feed = self.feed(); append()?; feed.generation = feed.generation.wrapping_add(1); if line.kind == EventKind::TurnEnded { feed.newest = Some(Turn::of(&line.data, &line.ts));
at: src/run/wait.rs:115-117
quote: for line in read_from(instance_dir, 0)? { let line = line?; if line.value["kind"] == EventKind::TurnEnded.as_str() {
at: src/cmd/run.rs:327
quote: let highest_dialog = wait.rebuild(&instance_dir)?;
@row_add n=175 row=contract/answer-response/question area=standard-contracts rec=fct-er6xfhrr@1
boundary: answer response
item: question
promise: A `question` response is `{answers:{<question>:<text>}, annotations?}`.
@row_check n=175 row=contract/answer-response/question sha=c9f3385b71ec v=holds judge=sonnet head=8834195b0683
at: crates/viola-agent-claude/src/dialog.rs:213-214
quote: if let Some(answers) = object.get("answers") { return question_response(answers, object.get("annotations"));
@row_add n=176 row=contract/answer-response/permission area=standard-contracts rec=fct-er6xfhrr@1
boundary: answer response
item: permission
promise: A `permission` response is `{behavior:"allow"|"deny", message?}`, with no suggestion slot.
@row_check n=176 row=contract/answer-response/permission sha=cd51d99c7782 v=holds judge=sonnet head=8834195b0683
at: crates/viola-agent-claude/src/dialog.rs:222-228
quote: Some("allow") => Ok(Self::Permission { behavior: Permit::Allow, message, }), Some("deny") => Ok(Self::Permission { behavior: Permit::Deny, message,
at: crates/viola-agent-claude/src/dialog.rs:188-191
quote: Permission { behavior: Permit, message: Option<String>, },
@row_add n=177 row=decision/permission-answer-without-suggestions area=standard-contracts rec=fct-er6xfhrr@1
name: Permission answers without suggestions
rule: A permission answer has no suggestion slot, so an offered `permission_suggestions` entry cannot be picked.
why: A deliberate v1 limit.
@row_check n=177 row=decision/permission-answer-without-suggestions sha=03b84ca1ad12 v=holds judge=sonnet head=8834195b0683
at: crates/viola-agent-claude/src/dialog.rs:188-191
quote: Permission {         behavior: Permit,         message: Option<String>,     },
@row_add n=178 row=contract/answer-response/plan area=standard-contracts rec=fct-er6xfhrr@1
boundary: answer response
item: plan
promise: A `plan` response is `{behavior:"approve"|"revise", message?}`.
@row_check n=178 row=contract/answer-response/plan sha=ddc22ee913cc v=holds judge=sonnet head=8834195b0683
at: crates/viola-agent-claude/src/dialog.rs:230
quote: Some("approve") => Ok(Self::Plan {
at: crates/viola-agent-claude/src/dialog.rs:234
quote: Some("revise") => Ok(Self::Plan {
@row_add n=179 row=contract/channel-methods/answer-from-link area=standard-contracts rec=fct-er6xfhrr@1,fct-vi22dxmq@1
boundary: channel methods
item: answer from link
promise: An `answer` whose `from` names another instance records the link once every refusal has passed and its kind matched, before the answer is taken; a link that cannot be recorded answers `-32603` and the dialog stays held; others record nothing.
@row_check n=179 row=contract/channel-methods/answer-from-link sha=1cdba60b5fd4 v=holds judge=sonnet head=8834195b0683
at: src/run/dialog.rs:449-456
quote: if response.kind() != pending.kind { return Err(ProtocolError::InvalidParams); } if let (Some(links), Some(from)) = (self.links.get(), from.as_ref()) && *from != self.name { links.record(from).map_err(|_| ProtocolError::Internal)?;
at: src/run/dialog.rs:468
quote: pending.answer = Some(response);
@row_add n=180 row=contract/channel-methods/release-from-driver area=standard-contracts rec=fct-3nvzp4be@1,fct-vi22dxmq@1
boundary: channel methods
item: release from driver
promise: A `release` with a string `from` is refused `-32602` `"invalid params"` with `data: {"reason":"release-from-driver"}` plus one `release-from-driver` obs line; this check comes first.
@row_check n=180 row=contract/channel-methods/release-from-driver sha=4080e795d41e v=holds judge=sonnet head=8834195b0683
at: src/run/wheel.rs:204-207
quote: if let Some(Value::String(from)) = params.get("from") { log_release_from_driver(call, from); return Err(ProtocolError::ReleaseFromDriver); }
at: crates/viola-channel/src/lib.rs:96
quote: Self::ReleaseFromDriver => json!({"reason": "release-from-driver"}),
at: crates/viola-channel/src/lib.rs:74
quote: Self::UnsupportedVersion | Self::InvalidParams | Self::ReleaseFromDriver => -32602,
at: src/run/wheel.rs:238-240
quote: obs_event!( INFO, ObsEvent::ReleaseFromDriver,
@row_add n=181 row=contract/channel-methods/release-type-check area=standard-contracts rec=fct-3nvzp4be@1,fct-vi22dxmq@1
boundary: channel methods
item: release type check
promise: A `release` whose `from` is of a non-string type, or whose `budget` is not a bool (`null` included), is `-32602` with `data: null`; this check comes first.
@row_check n=181 row=contract/channel-methods/release-type-check sha=aafe02e68c73 v=holds judge=sonnet head=8834195b0683
at: src/run/wheel.rs:208-212
quote: parse_from(params)?; let budget = match params.get("budget") { None => false, Some(budget) => budget.as_bool().ok_or(ProtocolError::InvalidParams)?, };
at: src/run/wheel.rs:213
quote: if budget {
@row_add n=182 row=contract/channel-methods/release-budget-true area=standard-contracts rec=fct-3nvzp4be@1,fct-vi22dxmq@1
boundary: channel methods
item: release budget true
promise: `budget:true` leaves the wheel and lifts the gate: `{wheel, budget_paused:false, override_until:<the crossed window's resets_at>}` when a pause was lifted, `{wheel, budget_paused:false}` and nothing recorded when none stood.
@row_check n=182 row=contract/channel-methods/release-budget-true sha=1b0ed27a161c v=holds judge=sonnet head=8834195b0683
at: src/run/wheel.rs:213-219
quote: if budget { let lifted = gate.lift().map_err(|_| ProtocolError::Internal)?; let mut ok = json!({"wheel": self.holder().as_str(), "budget_paused": false}); if let Some(end) = lifted { ok["override_until"] = json!(end);
at: src/run/budget.rs:311-313
quote: let Some(crossed) = &recorded.paused else { return Ok(None); };
@row_add n=183 row=contract/channel-methods/plain-release area=standard-contracts rec=fct-3nvzp4be@1,fct-vi22dxmq@1
boundary: channel methods
item: plain release
promise: A `release` with no `budget` or `budget:false` lifts no pause and answers `budget_paused` as the gate's last recorded state; a `release` on a driver-held wheel changes nothing.
@row_check n=183 row=contract/channel-methods/plain-release sha=6a07f666c32b v=holds judge=sonnet head=8834195b0683
at: src/run/wheel.rs:221-222
quote: settled(self.apply(Input::Release, true))?; Ok(json!({"ok": {"wheel": self.holder().as_str(), "budget_paused": gate.paused()}}))
at: src/run/wheel.rs:46
quote: (Input::Human | Input::Release, _, _) => None,
@row_add n=184 row=contract/channel-methods/wheel-move-durability area=standard-contracts rec=fct-ufpqhup6@1,fct-vi22dxmq@1
boundary: channel methods
item: wheel move durability
promise: `pause` and `release` are answered only after the move's `wheel` record has landed, `-32603` if it did not.
@row_check n=184 row=contract/channel-methods/wheel-move-durability sha=f561d7e69a52 v=holds judge=sonnet head=8834195b0683
at: src/run/wheel.rs:190
quote: settled(self.apply(Input::Pause, true))?;
at: src/run/wheel.rs:221
quote: settled(self.apply(Input::Release, true))?;
at: src/run/wheel.rs:230-231
quote: Some(outcome) if outcome.recv().unwrap_or(false) => Ok(()), Some(_) => Err(ProtocolError::Internal),
@row_add n=185 row=contract/channel-methods/budget-release-durability area=standard-contracts rec=fct-ufpqhup6@1,fct-vi22dxmq@1
boundary: channel methods
item: budget release durability
promise: A `release` with `budget:true` moves no wheel and is answered only after its `budget-gate` line and the snapshot's `budget_paused` and `budget_override_until` have landed; else `-32603` and the pause stands.
@row_check n=185 row=contract/channel-methods/budget-release-durability sha=2770cc78fd26 v=holds judge=sonnet head=8834195b0683
at: src/run/budget.rs:319-321
quote: self.record(&standing)?; *recorded = standing; Ok(Some(end))
at: src/run/budget.rs:332-334
quote: append_event(&self.instance_dir, &line)?; self.snapshots.update(|snapshot| { snapshot.budget_paused = standing.paused.is_some();
at: src/run/wheel.rs:214
quote: let lifted = gate.lift().map_err(|_| ProtocolError::Internal)?;
@row_add n=186 row=contract/channel-methods/link-params-check area=standard-contracts rec=fct-ax5zoxh3@1,fct-vi22dxmq@1
boundary: channel methods
item: link params check
promise: For `link` and `unlink`, a missing, non-string or invalid name, a `driven` that is not the wrapper's own name and a `driver` equal to `driven` are each `-32602` with `data: null`.
@row_check n=186 row=contract/channel-methods/link-params-check sha=19d66008b4db v=holds judge=sonnet head=8834195b0683
at: src/run/links.rs:88-96
quote: let named = |key: &str| { params[key] .as_str() .and_then(|name| ViolaName::try_new(name.to_owned()).ok()) .ok_or(ProtocolError::InvalidParams) }; let (driver, driven) = (named("driver")?, named("driven")?); if driven != self.name || driver == driven { return Err(ProtocolError::InvalidParams);
at: src/run/links.rs:73
quote: let driver = self.driver_of(params)?;
at: src/run/links.rs:80
quote: let driver = self.driver_of(params)?;
@row_add n=187 row=contract/channel-methods/hook-event-driver-origin area=standard-contracts rec=fct-nf7gdkwl@1,fct-vi22dxmq@1
boundary: channel methods
item: hook.event driver origin
promise: A `prompt-submitted` equal to the in-flight `send`'s typed text is appended with `origin:"driver"`, and only then settles that send.
@row_check n=187 row=contract/channel-methods/hook-event-driver-origin sha=6c4e84a42d99 v=holds judge=sonnet head=8834195b0683
at: src/run/send/slot.rs:198-203
quote: let prompt = line.kind == EventKind::PromptSubmitted && line.data["text"] .as_str() .is_some_and(|text| slot.claim(text)); if prompt { line.data["origin"] = json!("driver");
at: src/run/send/slot.rs:228-230
quote: let appended = feed.appending(&line, || append_event(instance_dir, &line)); if claimed { slot.settle(appended.is_ok().then(|| line.ts.clone()));
@row_add n=188 row=contract/channel-methods/hook-dialog-unverified-cli area=standard-contracts rec=fct-vwuy3ine@1,fct-vi22dxmq@1
boundary: channel methods
item: hook.dialog unverified CLI
promise: On an unverified CLI the wrapper replies `null` to `hook.dialog` at once.
@row_check n=188 row=contract/channel-methods/hook-dialog-unverified-cli sha=0188f68b5011 v=holds judge=sonnet head=8834195b0683
at: src/run/dialog.rs:251
quote: let hold = self.cli_verified
at: src/run/dialog.rs:271-272
quote: if !hold { return Ok(reply(dialog_id, None));
@row_add n=189 row=contract/channel-methods/hook-dialog-continuation area=standard-contracts rec=fct-vwuy3ine@1,fct-vi22dxmq@1
boundary: channel methods
item: hook.dialog continuation
promise: A PermissionRequest repeating the armed PreToolUse dialog's tool with an equal `tool_input` is that dialog's continuation: no event, no new `dialog_id`, answered only from an armed `plan` revise, otherwise `null`.
@row_check n=189 row=contract/channel-methods/hook-dialog-continuation sha=71c7f1e4bcfb v=holds judge=sonnet head=8834195b0683
at: src/run/dialog.rs:243-246
quote: if raise.continuation && let Some(answer) = continued(&mut state, &raise) { return Ok(answer);
at: src/run/dialog.rs:483
quote: let revise = match &arm.outcome {
at: src/run/dialog.rs:486
quote: behavior: Verdict::Revise,
@row_add n=190 row=decision/list-reads-state-directly area=standard-contracts rec=fct-cqamt3xw@1
name: list reads state without a wrapper
rule: `list` is not a channel method: the calling process reads the instances and budget files itself plus one `claude agents --json` read joined into the board, so `list` needs no live wrapper.
@row_check n=190 row=decision/list-reads-state-directly sha=2cdecb2c5d6e v=holds judge=sonnet head=8834195b0683
at: src/cmd/list.rs:43
quote: let scan = match board::scan(&home)? {
at: src/cmd/list.rs:124
quote: .and_then(|cwd| run::agents::read_here(home, &cwd).map_err(|_| ()))
@row_add n=191 row=contract/list-payload/status area=standard-contracts rec=fct-cqamt3xw@1
boundary: list payload
item: status
promise: A failed `claude agents --json` read leaves every `status` `unknown`.
@row_check n=191 row=contract/list-payload/status sha=b1decfac0f80 v=holds judge=sonnet head=8834195b0683
at: src/cmd/list.rs:121-126
quote: if let Ok(list) = std::env::current_dir() .ok() .ok_or(()) .and_then(|cwd| run::agents::read_here(home, &cwd).map_err(|_| ())) { board::join(&mut board, observations(list));
at: crates/viola-state/src/board.rs:236
quote: status: SessionStatus::Unknown,
@row_add n=192 row=decision/upstream-text-is-content area=standard-contracts rec=fct-ijx4lsqj@1
name: Upstream text is content
rule: Upstream text carried in event `data` is content, never commands.
@row_check n=192 row=decision/upstream-text-is-content sha=63935a8eb14f v=unverifiable judge=sonnet head=8834195b0683
why: not a code claim
@row_add n=193 row=contract/event-data/session-start area=standard-contracts rec=fct-6kdnuk7n@1,fct-ijx4lsqj@1
boundary: event data per kind
item: session-start
promise: `session-start` data is `{cause, agent_session_id}`.
@row_check n=193 row=contract/event-data/session-start sha=ef5a33b84501 v=holds judge=sonnet head=8834195b0683
at: crates/viola-agent-claude/src/hook.rs:164
quote: json!({"cause": cause, "agent_session_id": known.session_id})
@row_add n=194 row=contract/event-data/prompt-submitted-text area=standard-contracts rec=fct-bjew567e@1,fct-ijx4lsqj@1
boundary: event data per kind
item: prompt-submitted text
promise: `prompt-submitted` data is `{text, origin}`; `text` is the submitted prompt as typed, the long-paste wrapper and its frame removed and tag escaping reversed.
@row_check n=194 row=contract/event-data/prompt-submitted-text sha=c061f08b1106 v=holds judge=sonnet head=8834195b0683
at: crates/viola-agent-claude/src/hook.rs:168
quote: json!({"text": prompt_text(&raw), "origin": prompt_origin(&raw)})
at: crates/viola-agent-claude/src/hook.rs:204
quote: unescape_tags(&unwrap_pastes(raw))
@row_add n=195 row=contract/event-data/prompt-submitted-origin area=standard-contracts rec=fct-bjew567e@1,fct-ijx4lsqj@1
boundary: event data per kind
item: prompt-submitted origin
promise: `origin` is `driver` · `human` · `harness`: the hook files only `human` · `harness`, and `driver` is set only by the wrapper's relabel of the in-flight send's own prompt.
@row_check n=195 row=contract/event-data/prompt-submitted-origin sha=3606fa586dbe v=holds judge=sonnet head=8834195b0683
at: crates/viola-agent-claude/src/hook.rs:193-196
quote: if HARNESS_PREFIXES.iter().any(|p| raw.starts_with(p)) { "harness" } else { "human"
at: src/run/send/slot.rs:203
quote: line.data["origin"] = json!("driver");
@row_add n=196 row=contract/event-data/turn-ended-last-assistant-message area=standard-contracts rec=fct-nh2dddb3@1,fct-ijx4lsqj@1
boundary: event data per kind
item: turn-ended last_assistant_message
promise: `turn-ended` data is `{last_assistant_message, background_tasks}`; `last_assistant_message` is a string or `null` and is the source `last` rebuilds from.
@row_check n=196 row=contract/event-data/turn-ended-last-assistant-message sha=af1c3097773d v=holds judge=sonnet head=8834195b0683
at: crates/viola-agent-claude/src/hook.rs:171
quote: "last_assistant_message": known.last_assistant_message,
at: crates/viola-agent-claude/src/hook.rs:102
quote: last_assistant_message: Option<String>,
@row_add n=197 row=contract/event-data/turn-ended-background-tasks area=standard-contracts rec=fct-nh2dddb3@1,fct-ijx4lsqj@1
boundary: event data per kind
item: turn-ended background_tasks
promise: `background_tasks` is the Stop payload's `background_tasks` array length, a count only with no element read or carried, or `null` when the key is absent, `null` or not an array.
@row_check n=197 row=contract/event-data/turn-ended-background-tasks sha=043935ef48f4 v=holds judge=sonnet head=8834195b0683
at: crates/viola-agent-claude/src/hook.rs:172
quote: "background_tasks": known.background_tasks.map(|tasks| tasks.len()),
at: crates/viola-agent-claude/src/hook.rs:104
quote: background_tasks: Option<Vec<IgnoredAny>>,
@row_add n=198 row=contract/event-data/question area=standard-contracts rec=fct-7tqstti5@1
boundary: event data
item: `question`
promise: `{dialog_id, questions:[{question, header, options:[<label>], option_details:[{description, preview}], multi_select}]}`; each `question` string is a key in `answer`'s `answers`.
@row_check n=198 row=contract/event-data/question sha=2b311b12f309 v=holds judge=sonnet head=8834195b0683
at: crates/viola-agent-claude/src/dialog.rs:157-163
quote: Ok(json!({ "question": text, "header": question["header"].as_str(), "options": options, "option_details": option_details, "multi_select": question["multiSelect"].as_bool().unwrap_or(false), }))
at: crates/viola-agent-claude/src/dialog.rs:131
quote: Ok(json!({"questions": questions}))
at: src/run/dialog.rs:288
quote: data.insert("dialog_id".to_owned(), json!(dialog_id));
@row_add n=199 row=contract/event-data/question-option-details area=standard-contracts rec=fct-7tqstti5@1
boundary: event data
item: `question` option_details
promise: `option_details` is parallel to `options`, one per option, each field the option's string or `null`; a missing or non-string value never refuses the dialog.
@row_check n=199 row=contract/event-data/question-option-details sha=b8d90b6d4af8 v=holds judge=sonnet head=8834195b0683
at: crates/viola-agent-claude/src/dialog.rs:151-156
quote: let option_details: Vec<Value> = raw .iter() .map( |o| json!({"description": o["description"].as_str(), "preview": o["preview"].as_str()}), ) .collect();
@row_add n=200 row=decision/option-text-never-in-human-line area=standard-contracts rec=fct-7tqstti5@1
name: Upstream option text kept out of human lines
rule: Question option text is upstream content: carried into the event log, `hook.dialog`, CLI `wait --json` and the MCP `wait`, never into a human line.
@row_check n=200 row=decision/option-text-never-in-human-line sha=3c87009753dc v=holds judge=sonnet head=8834195b0683
at: crates/viola-agent-claude/src/dialog.rs:161
quote: "option_details": option_details,
searched: options|option_details|questions · in src/human/**/*.rs (no match; the human wait line prints only kind, name, dialog id and cursor, src/human/wait.rs:81)
@row_add n=201 row=contract/event-data/permission area=standard-contracts rec=fct-6tq6etq5@1
boundary: event data
item: `permission`
promise: `{dialog_id, tool, input}`; `input` is the tool call's arguments as a JSON value, carried as content, not the hook payload.
@row_check n=201 row=contract/event-data/permission sha=f97b1cd79d9f v=holds judge=sonnet head=8834195b0683
at: crates/viola-agent-claude/src/dialog.rs:105
quote: json!({"tool": name, "input": input}),
@row_add n=202 row=contract/event-data/plan area=standard-contracts rec=fct-kjgokxdm@1
boundary: event data
item: `plan`
promise: `{dialog_id, plan}`, with the plan as text.
@row_check n=202 row=contract/event-data/plan sha=19f33d264550 v=holds judge=sonnet head=8834195b0683
at: crates/viola-agent-claude/src/dialog.rs:135
quote: Ok(json!({"plan": plan}))
at: src/run/dialog.rs:288
quote: data.insert("dialog_id".to_owned(), json!(dialog_id));
@row_add n=203 row=contract/event-data/session-end area=standard-contracts rec=fct-qnym535j@1
boundary: event data
item: `session-end`
promise: `{}`.
@row_check n=203 row=contract/event-data/session-end sha=da2ab4d34b5e v=holds judge=sonnet head=8834195b0683
at: crates/viola-agent-claude/src/hook.rs:174
quote: HookEvent::SessionEnd | HookEvent::PreToolUse | HookEvent::PermissionRequest => json!({}),
@row_add n=204 row=contract/event-data/activity area=standard-contracts rec=fct-dyfp6yle@1
boundary: event data
item: `activity`
promise: `{}`, plus `tool` when the hook names one.
@row_check n=204 row=contract/event-data/activity sha=ea711d754981 v=holds judge=sonnet head=8834195b0683
at: crates/viola-agent-claude/src/hook.rs:175-179
quote: HookEvent::Notification | HookEvent::PostToolUse | HookEvent::PostToolUseFailure => { match known.tool_name { Some(tool) => json!({"tool": tool}), None => json!({}), }
@row_add n=205 row=contract/event-data/link area=standard-contracts rec=fct-sbsvt5hx@1
boundary: event data
item: `link`
promise: `{driver, driven}`.
@row_check n=205 row=contract/event-data/link sha=756541fc400d v=holds judge=sonnet head=8834195b0683
at: src/run/links.rs:123
quote: let data = json!({"driver": driver, "driven": driven});
at: src/run/links.rs:65
quote: let line = self.line(EventKind::Link, driver);
@row_add n=206 row=contract/event-data/unlink area=standard-contracts rec=fct-sbsvt5hx@1
boundary: event data
item: `unlink`
promise: `{driver, driven}`.
@row_check n=206 row=contract/event-data/unlink sha=6a38e1509e19 v=holds judge=sonnet head=8834195b0683
at: src/run/links.rs:123
quote: let data = json!({"driver": driver, "driven": driven});
at: src/run/links.rs:108
quote: let line = self.line(EventKind::Unlink, driver);
@row_add n=207 row=contract/event-data/wheel area=standard-contracts rec=fct-7nqw7hm2@1
boundary: event data
item: `wheel`
promise: `{holder, cause}`, with `holder` `human` or `driver` and `cause` `start`, `human-input`, `manual-pause` or `release`.
@row_check n=207 row=contract/event-data/wheel sha=82ad2275c98a v=holds judge=sonnet head=8834195b0683
at: src/run/wheel.rs:289
quote: json!({"holder": step.to.as_str(), "cause": step.cause.as_str()}),
at: crates/viola-core/src/lib.rs:310-313
quote: Self::Start => "start", Self::HumanInput => "human-input", Self::ManualPause => "manual-pause", Self::Release => "release",
@row_add n=208 row=contract/event-data/budget-gate area=standard-contracts rec=fct-yvpb6dvx@1
boundary: event data
item: `budget-gate`
promise: `{paused, window?, override_until?}` for the instance's own gate, with `window` `five-hour` or `seven-day`.
@row_check n=208 row=contract/event-data/budget-gate sha=7d9fe3d5ce3c v=holds judge=sonnet head=8834195b0683
at: src/run/budget.rs:125-130
quote: let mut data = json!({"paused": self.paused.is_some()}); if let Some(crossed) = &self.paused { data["window"] = json!(crossed.window.as_str()); } if let Some(end) = &self.override_until { data["override_until"] = json!(end);
at: crates/viola-core/src/lib.rs:259-260
quote: Self::FiveHour => "five-hour", Self::SevenDay => "seven-day",
@row_add n=209 row=contract/event-data/send-issued area=standard-contracts rec=fct-3rjugric@1
boundary: event data
item: `send-issued`
promise: `{cursor, from?}`, where `cursor` is the offset this line starts at: the send's cursor.
@row_check n=209 row=contract/event-data/send-issued sha=853d889080be v=holds judge=sonnet head=8834195b0683
at: src/run/send.rs:179-183
quote: let cursor = append_event_at(self.instance_dir, |cursor| { let mut data = json!({"cursor": cursor}); if let Some(from) = from { data["from"] = json!(from); }
@row_add n=210 row=contract/event-data/send-confirmed area=standard-contracts rec=fct-fpzowl4z@1
boundary: event data
item: `send-confirmed`
promise: `{cursor, confirmed?}`: a confirmed send writes `{cursor}`, an unconfirmable send writes `{cursor, confirmed:false}`, the one record of that outcome (no `send-refused`); the field is additive, no `v` bump, and no product code reads the record.
@row_check n=210 row=contract/event-data/send-confirmed sha=580d8fb9da7d v=holds judge=sonnet head=8834195b0683
at: src/run/send.rs:213
quote: self.record(EventKind::SendConfirmed, json!({"cursor": cursor}))?;
at: src/run/send.rs:221
quote: let data = json!({"cursor": cursor, "confirmed": false});
@row_add n=211 row=contract/event-data/send-refused area=standard-contracts rec=fct-u7zh3gf3@1
boundary: event data
item: `send-refused`
promise: `{refusal, detail, cursor?}`; `cursor` is absent for a refusal before `send-issued`.
@row_check n=211 row=contract/event-data/send-refused sha=a0e80b92fb90 v=holds judge=sonnet head=8834195b0683
at: src/run/send.rs:271-274
quote: let mut data = json!({"refusal": refusal.as_str(), "detail": detail}); if let Some(cursor) = cursor { data["cursor"] = json!(cursor); }
@row_add n=212 row=contract/event-data/dialog-expired area=standard-contracts rec=fct-xpezbo7u@1,fct-575nxi3k@1
boundary: event data
item: `dialog-expired`
promise: `{dialog_id, dialog_kind}` with `source: wrapper`, appended once when a held dialog's hold ends; carries no question, plan or tool text.
@row_check n=212 row=contract/event-data/dialog-expired sha=f0c88429a652 v=holds judge=sonnet head=8834195b0683
at: src/run/dialog.rs:376-382
quote: let line = EventLine::new( &self.name, EventKind::DialogExpired, Source::Wrapper, json!({"dialog_id": pending.dialog_id, "dialog_kind": pending.kind.as_str()}), Utc::now(), );
@row_add n=213 row=decision/wrapper-events-log-only area=standard-contracts rec=fct-575nxi3k@1
name: Wrapper-appended events are log-only
rule: Event kinds the wrapper appends (`wheel`, `budget-gate`, `send-issued`, `send-confirmed`, `send-refused`, `link`, `unlink`) are log-only and never wake `wait`.
why: They are how the audit trail and a replay learn the wheel holder, the budget gate, each send's outcome and the link set.
except: `dialog-expired`, the one wrapper-appended kind that wakes `wait`.
@row_check n=213 row=decision/wrapper-events-log-only sha=fca251a8811b v=holds judge=sonnet head=8834195b0683
at: crates/viola-core/src/lib.rs:99-106
quote: pub const WAIT_WAKE: [Self; 6] = [         Self::TurnEnded,         Self::Question,         Self::Permission,         Self::Plan,         Self::SessionEnd,         Self::DialogExpired,     ];
@row_add n=214 row=contract/wrapper-event-appends/wheel area=standard-contracts rec=fct-575nxi3k@1
boundary: wrapper event appends
item: `wheel`
promise: The wrapper appends `wheel` with `source: wrapper` once at start and on every change.
@row_check n=214 row=contract/wrapper-event-appends/wheel sha=b08d09ba303e v=holds judge=sonnet head=8834195b0683
at: src/cmd/run.rs:619
quote: EventKind::Wheel,
at: src/run/wheel.rs:287
quote: EventKind::Wheel,
@row_add n=215 row=contract/wrapper-event-appends/budget-gate area=standard-contracts rec=fct-575nxi3k@1
boundary: wrapper event appends
item: `budget-gate`
promise: The wrapper appends `budget-gate` with `source: wrapper` once at start and on every change.
@row_check n=215 row=contract/wrapper-event-appends/budget-gate sha=aafd974adf16 v=holds judge=sonnet head=8834195b0683
at: src/cmd/run.rs:622
quote: (EventKind::BudgetGate, budget.data()),
at: src/run/budget.rs:327
quote: EventKind::BudgetGate,
@row_add n=216 row=contract/wrapper-event-appends/send-records area=standard-contracts rec=fct-575nxi3k@1
boundary: wrapper event appends
item: `send-issued` / `send-confirmed` / `send-refused`
promise: The wrapper appends the send records with `source: wrapper` for each send.
@row_check n=216 row=contract/wrapper-event-appends/send-records sha=754d7790d838 v=holds judge=sonnet head=8834195b0683
at: src/run/send.rs:186
quote: EventKind::SendIssued,
at: src/run/send.rs:213
quote: self.record(EventKind::SendConfirmed, json!({"cursor": cursor}))?;
at: src/run/send.rs:279
quote: self.record(EventKind::SendRefused, data)?;
@row_add n=217 row=contract/wrapper-event-appends/link area=standard-contracts rec=fct-575nxi3k@1
boundary: wrapper event appends
item: `link`
promise: The driven instance's wrapper appends `link` (`source: wrapper`, `{driver, driven}`) once per new `driver → driven` pair, on an implicit link (an issued `send` or a taken `answer` from another instance) or a served `link` call.
@row_check n=217 row=contract/wrapper-event-appends/link sha=9d1316126533 v=holds judge=sonnet head=8834195b0683
at: src/run/links.rs:65
quote: let line = self.line(EventKind::Link, driver);
at: src/run/links.rs:124
quote: EventLine::new(&self.name, kind, Source::Wrapper, data, Utc::now())
at: src/run/send.rs:145
quote: links.record(from).map_err(|_| ProtocolError::Internal)?;
at: src/run/dialog.rs:455
quote: links.record(from).map_err(|_| ProtocolError::Internal)?;
at: src/cmd/run.rs:147
quote: "link" => self.links.link(call.params),
@row_add n=218 row=contract/wrapper-event-appends/unlink area=standard-contracts rec=fct-575nxi3k@1
boundary: wrapper event appends
item: `unlink`
promise: The driven instance's wrapper appends `unlink` (`source: wrapper`, `{driver, driven}`) once per held pair a served `unlink` call removes.
@row_check n=218 row=contract/wrapper-event-appends/unlink sha=3b198eda717d v=holds judge=sonnet head=8834195b0683
at: src/run/links.rs:108
quote: let line = self.line(EventKind::Unlink, driver);
at: src/cmd/run.rs:148
quote: "unlink" => self.links.unlink(call.params),
@row_add n=219 row=contract/hook-dialog/data area=standard-contracts rec=fct-zis5czrj@1
boundary: `hook.dialog`
item: `data`
promise: `hook.dialog`'s `data` is the same object as its dialog event's `data`, without `dialog_id`.
@row_check n=219 row=contract/hook-dialog/data sha=e21655a07f70 v=holds judge=sonnet head=8834195b0683
at: src/cmd/hook.rs:323
quote: params.insert("data".to_owned(), dialog.data.clone());
at: src/run/dialog.rs:287-288
quote: let mut data = raise.data.clone(); data.insert("dialog_id".to_owned(), json!(dialog_id));
@row_add n=220 row=contract/hook-dialog/dialog-hooks area=standard-contracts rec=fct-zis5czrj@1
boundary: `hook.dialog`
item: dialog hooks
promise: Dialog hooks send only `hook.dialog`, never a `hook.event` for the same dialog.
@row_check n=220 row=contract/hook-dialog/dialog-hooks sha=088780cf169e v=holds judge=sonnet head=8834195b0683
at: src/cmd/hook.rs:328
quote: match client.request_until("hook.dialog", params, reply_by) {
@row_add n=221 row=decision/dialog-logged-exactly-once area=standard-contracts rec=fct-zis5czrj@1
name: Every dialog logged exactly once
rule: Every dialog appears exactly once in the event log: on receipt the wrapper assigns the `dialog_id` and appends the dialog event before it can wake `wait` or reply, even when it replies `null` at once.
except: A matched continuation appends nothing, being the same dialog.
@row_check n=221 row=decision/dialog-logged-exactly-once sha=8d2a2a9785f9 v=holds judge=sonnet head=8834195b0683
at: src/run/dialog.rs:255
quote: let dialog_id = self.register(&mut state, &raise, hold)?;
at: src/run/dialog.rs:272
quote: return Ok(reply(dialog_id, None));
at: src/run/dialog.rs:243-247
quote: if raise.continuation             && let Some(answer) = continued(&mut state, &raise)         {             return Ok(answer);         }
@row_add n=222 row=contract/answer-refusals/unknown-dialog area=standard-contracts rec=fct-zis5czrj@1
boundary: answer refusals
item: `unknown-dialog`
promise: A late `answer` to a `question` first raised by PermissionRequest with no matching armed dialog is refused `unknown-dialog`.
@row_check n=222 row=contract/answer-refusals/unknown-dialog sha=9eb0c1707e71 v=holds judge=sonnet head=8834195b0683
at: src/run/dialog.rs:250
quote: let unanswerable = raise.continuation && raise.kind == DialogKind::Question;
at: src/run/dialog.rs:446
quote: Some(NotDelivered::UnknownDialog.as_str()),
@row_add n=223 row=contract/cli-json-output/result area=standard-contracts rec=fct-sgn7a3bf@1
boundary: CLI `--json` output
item: channel result
promise: Mirrors the channel `result`: `{"v":1,"ok":{…}}` or `{"v":1,"refusal":"human-typing","detail":null}`, with the matching exit code.
@row_check n=223 row=contract/cli-json-output/result sha=967d804f1df1 v=holds judge=sonnet head=8834195b0683
at: src/cmd/send.rs:110
quote: return client::document(&json!({"v": 1, "ok": ok}));
at: src/cmd/send.rs:133
quote: return client::document(&json!({"v": 1, "refusal": reason, "detail": detail}));
@row_add n=224 row=contract/cli-json-output/error area=standard-contracts rec=fct-sgn7a3bf@1
boundary: CLI `--json` output
item: `error`
promise: Outcomes that have no channel `result` print an `error` document.
@row_check n=224 row=contract/cli-json-output/error sha=ab70a93af8d7 v=holds judge=sonnet head=8834195b0683
at: src/cmd/client.rs:227
quote: document(&json!({"v": 1, "error": code, "detail": null}));
@row_add n=225 row=contract/cli-json-output/instance-unreachable area=standard-contracts rec=fct-ksmkl4jd@1
boundary: CLI `--json` output
item: `instance-unreachable`
promise: `{"v":1,"error":"instance-unreachable","detail":"<cause>"}` with exit 21, `<cause>` one of `strict-modes-failed` · `instance-dead` · `unwrapped-name` · `endpoint-lost` · `server-verify-failed` · `socket-dir-refused`.
@row_check n=225 row=contract/cli-json-output/instance-unreachable sha=503c4afc59e3 v=holds judge=sonnet head=8834195b0683
at: src/cmd/client.rs:233
quote: document(&json!({"v": 1, "error": "instance-unreachable", "detail": cause.as_str()}));
at: crates/viola-core/src/lib.rs:196-201
quote: Self::StrictModes => "strict-modes-failed", Self::InstanceDead => "instance-dead", Self::UnwrappedName => "unwrapped-name", Self::EndpointLost => "endpoint-lost", Self::ServerVerify => "server-verify-failed", Self::SocketDir => "socket-dir-refused",
at: src/cmd/client.rs:265
quote: 21
@row_add n=226 row=contract/mcp-refusal-result/instance-unreachable area=standard-contracts rec=fct-ksmkl4jd@1
boundary: MCP refusal result
item: `instance-unreachable`
promise: The MCP `isError` result for an unreachable instance is the CLI `instance-unreachable` document less `v`, its text `refused: instance-unreachable`.
@row_check n=226 row=contract/mcp-refusal-result/instance-unreachable sha=265002848088 v=holds judge=sonnet head=8834195b0683
at: crates/viola-mcp/src/result.rs:31
quote: error("instance-unreachable", Value::from(cause.as_str()))
at: crates/viola-mcp/src/result.rs:41
quote: result.content = vec![ContentBlock::text(format!("refused: {code}"))];
@row_add n=227 row=contract/cli-json-output/wrapper-fault area=standard-contracts rec=fct-cuelorpk@1
boundary: CLI `--json` output
item: `wrapper-fault`
promise: `{"v":1,"error":"wrapper-fault","detail":{"code":<int>,"message":"…","data":<object|null>}}` with exit 20; `message` is a fixed text and `data` a closed object or null.
@row_check n=227 row=contract/cli-json-output/wrapper-fault sha=42382a13d741 v=holds judge=sonnet head=8834195b0683
at: src/cmd/client.rs:91
quote: json!({"v": 1, "error": "wrapper-fault", "detail": detail})
at: crates/viola-channel/src/lib.rs:141
quote: Some(json!({"code": code, "message": message, "data": data}))
at: src/cmd/client.rs:245
quote: 20
@row_add n=228 row=contract/cli-json-output/internal-error area=standard-contracts rec=fct-cp7zw4em@1
boundary: CLI `--json` output
item: `internal-error`
promise: `{"v":1,"error":"internal-error","detail":null}` with exit 1 on stdout, stderr empty, when `--json` comes before any `--`; without `--json` the human line `error: internal error` stands, and `run`, `revive`'s start, `mcp` and `hook` stay silent.
@row_check n=228 row=contract/cli-json-output/internal-error sha=66b08734803f v=holds judge=sonnet head=8834195b0683
at: src/main.rs:67-68
quote: if json_asked(std::env::args_os()) { cmd::internal_error_document();
at: src/main.rs:145
quote: role == Role::Cli && matches!(outcome, Outcome::Failed | Outcome::Panicked)
@row_add n=229 row=contract/cli-json-output/revive-refusal area=standard-contracts rec=fct-54jxzo44@2
boundary: CLI `--json` output
item: `viola revive --json` refusal
promise: A refused strict-modes check, preflight or start of `viola revive --json` prints `{"v":1,"error":"<code>","detail":null}` with exit 1, codes only (no path, pid or id), stderr empty.
@row_check n=229 row=contract/cli-json-output/revive-refusal sha=4c8d4863569c v=holds judge=sonnet head=8834195b0683
at: src/cmd/revive.rs:107
quote: client::refused_document(self.detail());
at: src/cmd/client.rs:227
quote: document(&json!({"v": 1, "error": code, "detail": null}));
@row_add n=230 row=contract/cli-json-output/revive-refusal-codes area=standard-contracts rec=fct-54jxzo44@2
boundary: CLI `--json` output
item: `viola revive --json` refusal codes
promise: `<code>` is one of `strict-modes-failed` · `already-live` · `no-session` · `session-live` · `cwd-missing` · `stale-heartbeat` · `pinned-hash-mismatch` · `squatted-name` · `socket-dir-refused` · `internal-error`.
@row_check n=230 row=contract/cli-json-output/revive-refusal-codes sha=a2345e2c69c5 v=holds judge=sonnet head=8834195b0683
at: src/cmd/revive.rs:70-73
quote: Self::StrictModes => "strict-modes-failed", Self::NoSession | Self::UnknownId => "no-session", Self::SessionLive => "session-live", Self::CwdMissing => "cwd-missing",
at: src/cmd/run.rs:454-455
quote: Liveness::Live => Some(refused_with(json, "already-live", || refuse_live(name))), Liveness::Stale => Some(refused_with(json, "stale-heartbeat", || {
at: src/cmd/run.rs:295
quote: return Ok(refused_with(launch.json, "squatted-name", || {
at: src/cmd/run.rs:300
quote: return Ok(refused_with(launch.json, "socket-dir-refused", || {
at: src/cmd/run.rs:284
quote: "pinned-hash-mismatch",
at: src/cmd/revive.rs:124
quote: client::refused_document("internal-error");
@row_add n=231 row=contract/cli-json-output/revive-list area=standard-contracts rec=fct-54jxzo44@2
boundary: CLI `--json` output
item: `viola revive <name> --list --json`
promise: Prints `{"v":1,"ok":{"sessions":[{"ts":"…","cause":"…","id":"…"},…]}}` in log order with exit 0, or the `no-session` document with exit 1 on an empty chain.
@row_check n=231 row=contract/cli-json-output/revive-list sha=1bde78cf5a26 v=holds judge=sonnet head=8834195b0683
at: src/cmd/revive.rs:292
quote: client::document(&json!({"v": 1, "ok": {"sessions": sessions}}));
at: src/cmd/revive.rs:283
quote: Refusal::NoSession.write(name, json);
@row_add n=232 row=decision/no-document-after-child-held-terminal area=standard-contracts rec=fct-54jxzo44@2
name: No JSON document once the child held the terminal
rule: An error after the child has held the terminal prints no JSON document.
why: The child has held the terminal.
@row_check n=232 row=decision/no-document-after-child-held-terminal sha=887aaa280c03 v=holds judge=sonnet head=8834195b0683
at: src/cmd/revive.rs:122-126
quote: let started = started(home, args).inspect_err(|_| {         if json {             client::refused_document("internal-error");         }     })?;
@row_add n=233 row=contract/cli-json-output/plugin-install area=standard-contracts rec=fct-2xyi4o3s@1
boundary: CLI `--json` output
item: `viola plugin install --json`
promise: Prints `{"v":1,"ok":{"marketplace":"viola","plugin":"viola@viola"}}` with exit 0, no path.
@row_check n=233 row=contract/cli-json-output/plugin-install sha=5b924725db26 v=holds judge=sonnet head=8834195b0683
at: src/cmd/plugin.rs:67-70
quote: client::document(&json!({ "v": 1, "ok": {"marketplace": marketplace::NAME, "plugin": PLUGIN_REF}, }));
at: crates/viola-agent-claude/src/marketplace.rs:14
quote: pub const PLUGIN_REF: &str = "viola@viola";
@row_add n=234 row=contract/cli-json-output/plugin-install-strict-modes-refused area=standard-contracts rec=fct-2xyi4o3s@1
boundary: CLI `--json` output
item: `viola plugin install --json` strict-modes refusal
promise: A refused strict-modes check prints the `instance-unreachable` document with `detail` `strict-modes-failed` and exit 21.
@row_check n=234 row=contract/cli-json-output/plugin-install-strict-modes-refused sha=32f171e5f090 v=holds judge=sonnet head=8834195b0683
at: src/cmd/plugin.rs:55
quote: client::unreachable_document(UnreachableCause::StrictModes);
at: src/cmd/plugin.rs:60
quote: return Ok(ExitCode::from(21));
@row_add n=235 row=decision/hook-output-one-crate area=standard-contracts rec=fct-scvnkltz@1
name: Hook output produced by one crate
rule: The Claude Code hook contract is produced only by the `viola-agent-claude` crate.
@row_check n=235 row=decision/hook-output-one-crate sha=6589a9849179 v=holds judge=sonnet head=8834195b0683
at: crates/viola-agent-claude/src/dialog.rs:399
quote: json!({"hookSpecificOutput": output})
searched: hookSpecificOutput|permissionDecision|hookEventName · in src/**/*.rs, crates/viola-core, crates/viola-state, crates/viola-channel, crates/viola-mcp, crates/viola-pty (only test assertions in src/cmd/hook.rs)
@row_add n=236 row=contract/hook-contract/decision area=standard-contracts rec=fct-6rjpz6ju@1
boundary: hook contract
item: a decision
promise: A decision is exit 0 with a `hookSpecificOutput` body.
@row_check n=236 row=contract/hook-contract/decision sha=7e271efabcaf v=holds judge=sonnet head=8834195b0683
at: src/cmd/hook.rs:298
quote: out.write_all(body.as_bytes())?;
at: crates/viola-agent-claude/src/dialog.rs:399
quote: json!({"hookSpecificOutput": output})
@row_add n=237 row=contract/hook-contract/pre-tool-use area=standard-contracts rec=fct-6rjpz6ju@1
boundary: hook contract
item: PreToolUse
promise: PreToolUse returns `{"hookSpecificOutput":{"hookEventName":"PreToolUse","permissionDecision":"allow","updatedInput":{…}}}`.
@row_check n=237 row=contract/hook-contract/pre-tool-use sha=1801600204a1 v=holds judge=sonnet head=8834195b0683
at: crates/viola-agent-claude/src/dialog.rs:395
quote: let mut output = json!({"hookEventName": "PreToolUse", "permissionDecision": "allow"});
@row_add n=238 row=contract/hook-contract/permission-request area=standard-contracts rec=fct-6rjpz6ju@1
boundary: hook contract
item: PermissionRequest
promise: PermissionRequest returns `{"hookSpecificOutput":{"hookEventName":"PermissionRequest","decision":{"behavior":"allow|deny","message":"…"}}}`.
@row_check n=238 row=contract/hook-contract/permission-request sha=e195dd9d7bfe v=holds judge=sonnet head=8834195b0683
at: crates/viola-agent-claude/src/dialog.rs:407
quote: json!({"hookSpecificOutput": {"hookEventName": "PermissionRequest", "decision": decision}})
@row_add n=239 row=contract/hook-contract/no-decision area=standard-contracts rec=fct-2vjklnmm@1
boundary: hook contract
item: no decision
promise: No decision is exit 0 with an empty stdout.
@row_check n=239 row=contract/hook-contract/no-decision sha=18d4d9e27049 v=holds judge=sonnet head=8834195b0683
at: src/cmd/hook.rs:297
quote: if let Some(body) = &body {
@row_add n=240 row=contract/hook-contract/async-tier area=standard-contracts rec=fct-2vjklnmm@1
boundary: hook contract
item: async-tier hooks
promise: Async-tier hooks always print nothing.
@row_check n=240 row=contract/hook-contract/async-tier sha=c44b0d0f3fa7 v=holds judge=sonnet head=8834195b0683
at: src/cmd/hook.rs:146-149
quote: match handle(event, &instance, spine_deadline(started), stdin) { Ok(detail) => { decided(event, detail, started); Ok(ExitCode::SUCCESS)
@row_add n=241 row=decision/statusline-hook-exempt area=standard-contracts rec=fct-horlp4ay@2
name: Statusline wrapper exempt from hook output rules
rule: `viola hook statusline` records the `rate_limits` reading, prints the user's statusline command's stdout unchanged, and always exits 0 with empty stderr.
why: It is the statusline wrapper, not a Claude Code hook, so the hook decision rules do not apply.
@row_check n=241 row=decision/statusline-hook-exempt sha=17697f5b3b3a v=holds judge=sonnet head=8834195b0683
at: src/cmd/hook/statusline.rs:127
quote: reading.is_some_and(|reading| write_budget(&instance.home, &reading, Utc::now()).is_ok());
at: src/cmd/hook/statusline.rs:143
quote: let _ = io.out.write_all(&stdout).and_then(|()| io.out.flush());
at: src/cmd/hook/statusline.rs:62
quote: Ok(()) => Ok(ExitCode::SUCCESS),
@row_add n=242 row=contract/session-liveness/heartbeat area=standard-contracts rec=fct-arftgre4@1
boundary: session liveness
item: heartbeat
promise: The wrapper touches `<instance dir>/heartbeat` every 1 s.
@row_check n=242 row=contract/session-liveness/heartbeat sha=4865478a9894 v=holds judge=sonnet head=8834195b0683
at: crates/viola-state/src/heartbeat.rs:13
quote: pub const BEAT_EVERY: Duration = Duration::from_secs(1);
at: crates/viola-state/src/heartbeat.rs:41-42
quote: while let Err(RecvTimeoutError::Timeout) = stopped.recv_timeout(BEAT_EVERY) { let _ = touch_heartbeat(&instance_dir);
@row_add n=243 row=contract/session-liveness/gone area=standard-contracts rec=fct-arftgre4@1
boundary: session liveness
item: `gone`
promise: A session is `gone` when the snapshot's pid + start time no longer name a running process (pid dead, or alive with another start time), whatever the beat's age.
@row_check n=243 row=contract/session-liveness/gone sha=bfc77a8f1279 v=holds judge=sonnet head=8834195b0683
at: crates/viola-state/src/liveness.rs:28-29
quote: if !same_process { return Liveness::Gone;
@row_add n=244 row=contract/session-liveness/live area=standard-contracts rec=fct-arftgre4@1
boundary: session liveness
item: `live`
promise: A session whose process is running and whose beat is at most 5 s old is `live`.
@row_check n=244 row=contract/session-liveness/live sha=225b155c4c02 v=holds judge=sonnet head=8834195b0683
at: crates/viola-state/src/liveness.rs:32
quote: Some(age) if age <= STALE_AFTER => Liveness::Live,
@row_add n=245 row=contract/session-liveness/stale area=standard-contracts rec=fct-arftgre4@1
boundary: session liveness
item: `stale`
promise: A session whose process is running and whose beat is older than 5 s or absent is `stale`.
@row_check n=245 row=contract/session-liveness/stale sha=aeea622c05bb v=holds judge=sonnet head=8834195b0683
at: crates/viola-state/src/liveness.rs:33
quote: _ => Liveness::Stale,
@row_add n=246 row=contract/agents-listing/status area=standard-contracts rec=fct-l2ty3vtu@1
boundary: agents listing
item: `status`
promise: `claude agents --json` adds `status` (idle/busy) to the listing.
@row_check n=246 row=contract/agents-listing/status sha=94883d03cde4 v=holds judge=sonnet head=8834195b0683
at: crates/viola-state/src/board.rs:334
quote: item.status = observations[at].status;
@row_add n=247 row=contract/agents-listing/unwrapped-sessions area=standard-contracts rec=fct-l2ty3vtu@1
boundary: agents listing
item: unwrapped sessions
promise: Unwrapped sessions are listed as read-only rows; a row matching a wrapped instance's `child_pid` or `agent_session_id` is merged into that instance's row, never listed separately.
@row_check n=247 row=contract/agents-listing/unwrapped-sessions sha=60ea54ca7aa0 v=holds judge=sonnet head=8834195b0683
at: crates/viola-state/src/board.rs:343-344
quote: .filter(|(_, joined)| !joined) .map(|(observation, _)| Item::unwrapped(observation))
@row_add n=248 row=decision/agents-listing-never-errors area=standard-contracts rec=fct-l2ty3vtu@1
name: Agents listing degrades, never errors
rule: When `claude agents --json` is not found, fails, is over the cap or cannot be parsed, every `status` is `unknown`, no unwrapped row is listed, and the outcome is never an error.
@row_check n=248 row=decision/agents-listing-never-errors sha=8f3a16c6c07c v=holds judge=sonnet head=8834195b0683
at: src/run/agents.rs:111
quote: return Err(ReadFailure::Unreadable);
at: src/run/agents.rs:86-89
quote: match failure {             AgentsFailure::Oversize => ReadFailure::Oversize,             AgentsFailure::Malformed => ReadFailure::Malformed,         }
at: src/cmd/list.rs:126
quote: board::join(&mut board, observations(list));
@row_add n=249 row=decision/wrapper-channel-json-rpc area=standard-contracts rec=fct-mxxjcuwn@1
name: JSON-RPC over ndjson for the wrapper channel
rule: The wrapper channel speaks JSON-RPC 2.0 over ndjson on local sockets.
@row_check n=249 row=decision/wrapper-channel-json-rpc sha=e55bd33393e3 v=holds judge=sonnet head=8834195b0683
at: crates/viola-channel/src/frame.rs:83
quote: if request.jsonrpc != "2.0" {
at: crates/viola-channel/src/server.rs:123
quote: let listener = ListenerOptions::new()
@row_add n=250 row=decision/mcp-over-stdio area=standard-contracts rec=fct-uy5hviac@1,fct-kq7qpqyi@1
name: MCP over stdio only
rule: MCP speaks JSON-RPC 2.0 over stdio; the MCP server is stdio only.
@row_check n=250 row=decision/mcp-over-stdio sha=b020756c9720 v=holds judge=sonnet head=8834195b0683
at: crates/viola-mcp/src/lib.rs:76
quote: let (stdin, stdout) = rmcp::transport::stdio();
searched: TcpListener|TcpStream|axum|hyper · in Cargo.toml, crates/*/Cargo.toml, src/**/*.rs, crates/**/*.rs
@row_add n=251 row=absence/websocket area=standard-contracts rec=fct-4yre6p6e@1
capability: WebSocket endpoints (v1)
@row_check n=251 row=absence/websocket sha=c9153029bbcc v=holds judge=sonnet head=8834195b0683
searched: WebSocket|websocket|tungstenite · in Cargo.toml, crates/*/Cargo.toml, src/**/*.rs, crates/**/*.rs
searched: TcpListener|TcpStream|axum · in Cargo.toml, crates/*/Cargo.toml, src/**/*.rs, crates/**/*.rs
@row_add n=252 row=absence/polling-endpoints area=standard-contracts rec=fct-4yre6p6e@1
capability: Polling endpoints (v1)
@row_check n=252 row=absence/polling-endpoints sha=5595ed49340c v=holds judge=sonnet head=8834195b0683
searched: (?i)websocket|tungstenite|axum|hyper|warp::|tiny_http|actix · in Cargo.toml, crates/*/Cargo.toml, src/**/*.rs, crates/*/src/**/*.rs (only hyperfine in the e2e harness matched)
searched: poll · in **/*.rs (only internal waits and a pty size poll; no served polling endpoint or route)
@row_add n=253 row=absence/listening-network-port area=occupied-resources rec=fct-kq7qpqyi@1
capability: A listening network port of any kind: no database port, no broker port, no listening TCP port
@row_check n=253 row=absence/listening-network-port sha=ad9153f8f3d4 v=holds judge=sonnet head=8834195b0683
searched: TcpListener|UdpSocket|\.listen\( · in **/*.rs
searched: UnixListener::bind (only a local socket, in a test cleanup helper) · in **/*.rs
@row_add n=254 row=name/other/viola-test-chan-pid-label area=occupied-resources rec=fct-s2vyyjop@2
class: other
name: viola-test-chan-<pid>-<label>
@row_check n=254 row=name/other/viola-test-chan-pid-label sha=61ca232da3cb v=holds judge=sonnet head=8834195b0683
at: crates/viola-channel/src/lib.rs:253
quote: let base = format!("viola-test-chan-{}-{label}", std::process::id());
@row_add n=255 row=decision/test-endpoints-outside-namespace area=occupied-resources rec=fct-s2vyyjop@2
name: Test channel endpoints outside the product namespace
rule: Test-only channel endpoints are named outside the `viola-<h12>` namespace, unique per test and never bound by `viola`.
@row_check n=255 row=decision/test-endpoints-outside-namespace sha=7cc089a1bb28 v=holds judge=sonnet head=8834195b0683
at: crates/viola-channel/src/lib.rs:251
quote: /// A test-owned endpoint outside the `viola-<h12>` namespace `viola run` binds.
@row_add n=256 row=name/other/viola-h12-sock area=occupied-resources rec=fct-wular7qm@2
class: other
name: viola-<h12>.sock
@row_check n=256 row=name/other/viola-h12-sock sha=9a60991c990a v=holds judge=sonnet head=8834195b0683
at: crates/viola-channel/src/endpoint.rs:52
quote: .join(format!("{base}.sock"))
@row_add n=257 row=decision/socket-dir-owner-only area=occupied-resources rec=fct-wular7qm@2
name: Socket in a per-user owner-only directory
rule: The Unix domain socket lives in a per-user 0700 directory, never the `/tmp` root, checked before the bind and before every client's connect; a refused directory (`socket-dir-refused`) is left as found, never narrowed.
@row_check n=257 row=decision/socket-dir-owner-only sha=7556797cbcdf v=holds judge=sonnet head=8834195b0683 masked=true
at: crates/viola-state/src/strict.rs:137
quote: pub fn check_socket_dir(dir: &Path) -> Result<(), Refused> {
at: crates/viola-channel/src/endpoint.rs:40
quote: None => PathBuf::from(format!("{host path}{uid}")),
at: src/cmd/hook.rs:231
quote: if check_endpoint_dir(endpoint).is_err() {
@row_add n=258 row=decision/socket-file-owner-only area=occupied-resources rec=fct-wular7qm@2
name: Socket file owner-only
rule: The socket is chmod 0600 after the bind.
@row_check n=258 row=decision/socket-file-owner-only sha=572663829f7d v=holds judge=sonnet head=8834195b0683
at: crates/viola-channel/src/server.rs:134
quote: fs::set_permissions(&guard.socket, Permissions::from_mode(0o600))
@row_add n=259 row=decision/same-user-peers-only area=occupied-resources rec=fct-wular7qm@2
name: Same-user peers only
rule: Both sides of a socket connection admit only a peer of the owning user's euid; the wrapper drops another before any frame.
@row_check n=259 row=decision/same-user-peers-only sha=06c5d39849fc v=holds judge=sonnet head=8834195b0683
at: crates/viola-channel/src/server.rs:145
quote: .filter(|stream| admitted(peer_euid_of(stream)));
at: src/cmd/hook.rs:236
quote: if !peer_admitted(client.server_euid()) {
at: src/cmd/client.rs:177
quote: if !peer_admitted(client.server_euid()) {
@row_add n=260 row=decision/probe-capture-arms-inert area=occupied-resources rec=fct-cz2fjgdx@1
name: Probe capture arms are inert
rule: The hidden hook `--capture` arms take an absolute, existing directory only, read no `VIOLA_*` variable, start no observability, open no channel, and always exit 0 with empty stderr.
@row_check n=260 row=decision/probe-capture-arms-inert sha=bc9875f30daf v=holds judge=sonnet head=8834195b0683
at: src/cmd/hook.rs:421
quote: if !dir.is_absolute() || !dir.is_dir() {
at: src/cmd/hook.rs:101-106
quote: if let Some(dir) = &args.capture { let filed = capture(event, dir, &mut std::io::stdin().lock()); if let (Some(filed), Some(answers)) = (filed, &args.answers) { answer(event, dir, answers, &filed, &mut std::io::stdout().lock()); } return Ok(ExitCode::SUCCESS);
@row_add n=261 row=name/other/hook-session-start area=occupied-resources rec=fct-p6w2v2xw@1
class: other
name: hook session-start
@row_check n=261 row=name/other/hook-session-start sha=3d882471b6e1 v=holds judge=sonnet head=8834195b0683
at: crates/viola-agent-claude/src/hook.rs:45
quote: Self::SessionStart => "session-start",
@row_add n=262 row=name/other/hook-user-prompt-submit area=occupied-resources rec=fct-p6w2v2xw@1
class: other
name: hook user-prompt-submit
@row_check n=262 row=name/other/hook-user-prompt-submit sha=9289f8355acd v=holds judge=sonnet head=8834195b0683
at: crates/viola-agent-claude/src/hook.rs:46
quote: Self::UserPromptSubmit => "user-prompt-submit",
@row_add n=263 row=name/other/hook-pre-tool-use area=occupied-resources rec=fct-p6w2v2xw@1
class: other
name: hook pre-tool-use
@row_check n=263 row=name/other/hook-pre-tool-use sha=8bc19ef7478e v=holds judge=sonnet head=8834195b0683
at: crates/viola-agent-claude/src/hook.rs:52
quote: Self::PreToolUse => "pre-tool-use",
@row_add n=264 row=name/other/hook-permission-request area=occupied-resources rec=fct-p6w2v2xw@1
class: other
name: hook permission-request
@row_check n=264 row=name/other/hook-permission-request sha=7a2a4d819b2f v=holds judge=sonnet head=8834195b0683
at: crates/viola-agent-claude/src/hook.rs:53
quote: Self::PermissionRequest => "permission-request",
@row_add n=265 row=name/other/hook-stop area=occupied-resources rec=fct-p6w2v2xw@1
class: other
name: hook stop
@row_check n=265 row=name/other/hook-stop sha=15f582d74254 v=holds judge=sonnet head=8834195b0683
at: crates/viola-agent-claude/src/hook.rs:47
quote: Self::Stop => "stop",
@row_add n=266 row=name/other/hook-session-end area=occupied-resources rec=fct-p6w2v2xw@1
class: other
name: hook session-end
@row_check n=266 row=name/other/hook-session-end sha=c54c1e2e28b2 v=holds judge=sonnet head=8834195b0683
at: crates/viola-agent-claude/src/hook.rs:48
quote: Self::SessionEnd => "session-end",
@row_add n=267 row=name/other/hook-notification area=occupied-resources rec=fct-p6w2v2xw@1
class: other
name: hook notification
@row_check n=267 row=name/other/hook-notification sha=cb2b0389dde2 v=holds judge=sonnet head=8834195b0683
at: crates/viola-agent-claude/src/hook.rs:49
quote: Self::Notification => "notification",
@row_add n=268 row=name/other/hook-post-tool-use area=occupied-resources rec=fct-p6w2v2xw@1
class: other
name: hook post-tool-use
@row_check n=268 row=name/other/hook-post-tool-use sha=1e0a69a9693d v=holds judge=sonnet head=8834195b0683
at: crates/viola-agent-claude/src/hook.rs:50
quote: Self::PostToolUse => "post-tool-use",
@row_add n=269 row=name/other/hook-post-tool-use-failure area=occupied-resources rec=fct-p6w2v2xw@1
class: other
name: hook post-tool-use-failure
@row_check n=269 row=name/other/hook-post-tool-use-failure sha=f051982ecabc v=holds judge=sonnet head=8834195b0683
at: crates/viola-agent-claude/src/hook.rs:51
quote: Self::PostToolUseFailure => "post-tool-use-failure",
@row_add n=270 row=name/other/hook-statusline area=occupied-resources rec=fct-p6w2v2xw@1
class: other
name: hook statusline
@row_check n=270 row=name/other/hook-statusline sha=8c12644075e4 v=holds judge=sonnet head=8834195b0683
at: src/cmd/hook/statusline.rs:27
quote: pub(super) const WORD: &str = "statusline";
@row_add n=271 row=name/other/viola area=occupied-resources rec=fct-frmh2fi2@1,fct-vw4eap6t@1
class: other
name: viola
@row_check n=271 row=name/other/viola sha=6ad7d1cbe925 v=holds judge=sonnet head=8834195b0683
at: crates/viola-agent-claude/src/marketplace.rs:11
quote: pub const NAME: &str = "viola";
@row_add n=272 row=name/server/viola area=occupied-resources rec=fct-frmh2fi2@1,fct-vw4eap6t@1
class: server
name: viola
@row_check n=272 row=name/server/viola sha=8e531174678e v=holds judge=sonnet head=8834195b0683
at: crates/viola-agent-claude/src/marketplace.rs:45
quote: NAME: {"command": exe_fwd, "args": ["--home", home_fwd, "mcp"]},
@row_add n=273 row=name/other/viola-at-viola area=occupied-resources rec=fct-vw4eap6t@1
class: other
name: viola@viola
@row_check n=273 row=name/other/viola-at-viola sha=b4afb9351a54 v=holds judge=sonnet head=8834195b0683
at: crates/viola-agent-claude/src/marketplace.rs:14
quote: pub const PLUGIN_REF: &str = "viola@viola";
@row_add n=274 row=decision/plugin-carries-no-hooks area=occupied-resources rec=fct-vw4eap6t@1
name: Installed plugin carries no hooks
rule: The plugin `viola plugin install` adds registers the MCP server `viola` in its `.mcp.json` and carries no hooks.
@row_check n=274 row=decision/plugin-carries-no-hooks sha=83f79a02b200 v=holds judge=sonnet head=8834195b0683
searched: "hooks" · in crates/viola-agent-claude/src/marketplace.rs (only a doc comment and the test marketplace_files_hold_no_hooks; the three generated files hold none)
searched: mcpServers · in crates/viola-agent-claude/src/marketplace.rs (line 44, the only registered thing)
@row_add n=275 row=contract/child-argv/plugin-dir area=occupied-resources rec=fct-swzuengu@1
boundary: flags passed to the child
item: `--plugin-dir`
promise: `--plugin-dir <viola home>/plugin/<version>-<hash>` comes first on every start.
@row_check n=275 row=contract/child-argv/plugin-dir sha=07cf08a88084 v=holds judge=sonnet head=8834195b0683
at: src/run/mod.rs:128-131
quote: let mut args = vec![ OsString::from(PLUGIN_DIR_FLAG), plugin_dir.as_os_str().to_owned(), ];
@row_add n=276 row=contract/child-argv/settings area=occupied-resources rec=fct-swzuengu@1
boundary: flags passed to the child
item: `--settings`
promise: `--settings <instance dir>/settings.json` follows when the start wrote a settings override.
@row_check n=276 row=contract/child-argv/settings sha=c352e400ba27 v=holds judge=sonnet head=8834195b0683
at: src/run/mod.rs:132-135
quote: if let Some(settings) = settings { args.push(OsString::from(SETTINGS_FLAG)); args.push(settings.as_os_str().to_owned()); }
@row_add n=277 row=contract/child-argv/caller-arguments area=occupied-resources rec=fct-swzuengu@1
boundary: flags passed to the child
item: caller's arguments
promise: The caller's arguments follow the viola flags.
@row_check n=277 row=contract/child-argv/caller-arguments sha=e65067ccfa9b v=holds judge=sonnet head=8834195b0683
at: src/run/mod.rs:136
quote: args.extend_from_slice(user_args);
@row_add n=278 row=contract/child-argv/resume area=occupied-resources rec=fct-swzuengu@1
boundary: flags passed to the child
item: `--resume`
promise: A revived start launches `claude` and adds `--resume <id>`, the id one argv element of a direct spawn.
@row_check n=278 row=contract/child-argv/resume sha=c1ccc139cd01 v=holds judge=sonnet head=8834195b0683
at: crates/viola-agent-claude/src/lib.rs:110
quote: let mut args = vec![OsString::from(RESUME_FLAG), OsString::from(id)];
at: src/cmd/revive.rs:230
quote: let mut child_args = resume_args(&id, args.fork);
@row_add n=279 row=contract/child-argv/fork-session area=occupied-resources rec=fct-swzuengu@1
boundary: flags passed to the child
item: `--fork-session`
promise: A revived start adds `--fork-session` after `--resume <id>` under `--fork`.
@row_check n=279 row=contract/child-argv/fork-session sha=52b36fa48412 v=holds judge=sonnet head=8834195b0683
at: crates/viola-agent-claude/src/lib.rs:111-113
quote: if fork { args.push(OsString::from(FORK_SESSION_FLAG)); }
@row_add n=280 row=name/other/viola-verify-probe area=occupied-resources rec=fct-zlrgexly@1
class: other
name: viola-verify-probe
@row_check n=280 row=name/other/viola-verify-probe sha=8889c366c285 v=holds judge=sonnet head=8834195b0683
at: crates/viola-agent-claude/src/ledger.rs:462
quote: "name": "viola-verify-probe",
@row_add n=281 row=contract/settings-override/status-line area=occupied-resources rec=fct-klpb7auf@2
boundary: per-session settings override
item: `statusLine`
promise: `instances/<ViolaName>/settings.json` holds a `statusLine` command running the pinned path's `hook statusline`, rewritten whole at every start before the spawn.
@row_check n=281 row=contract/settings-override/status-line sha=fe553d928ec3 v=holds judge=sonnet head=8834195b0683
at: crates/viola-agent-claude/src/statusline.rs:98-99
quote: let command = format!("{pinned_bin_fwd} hook statusline"); Some(json!({"statusLine": {"type": "command", "command": command}}).to_string())
at: src/cmd/run.rs:567-568
quote: let path = instance_dir.join(SETTINGS_OVERRIDE); replace_private(&path, document.as_bytes(), FILE_MODE)?;
at: src/cmd/run.rs:380
quote: let settings = write_override(&instance_dir, &pinned)?;
@row_add n=282 row=contract/settings-override/plain-path-only area=occupied-resources rec=fct-klpb7auf@2
boundary: per-session settings override
item: plain-character pinned path
promise: The override is written only for a pinned path of plain characters; otherwise no override is written and no `--settings` is passed.
@row_check n=282 row=contract/settings-override/plain-path-only sha=bc957a39b9c3 v=holds judge=sonnet head=8834195b0683
at: crates/viola-agent-claude/src/statusline.rs:95
quote: if !pinned_bin_fwd.chars().all(is_shell_plain) {
at: src/cmd/run.rs:564-566
quote: let Some(document) = statusline::override_document(&pinned.path_fwd) else { return Ok(None); };
@row_add n=283 row=decision/e2e-noop-fake-agent-feature area=occupied-resources rec=fct-j2leq5fi@1
name: No-op fake-agent feature in the e2e crate
rule: The test-only `viola-e2e` crate declares a no-op `fake-agent` feature.
why: So package-scoped `--features fake-agent` invocations resolve.
@row_check n=283 row=decision/e2e-noop-fake-agent-feature sha=73adf280f226 v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/Cargo.toml:12
quote: fake-agent = []
@row_add n=284 row=decision/viola-channel-tokio-free-by-default area=occupied-resources rec=fct-26g75mkn@1
name: viola-channel tokio-free without its feature
rule: The viola-channel dependency graph without its tokio feature carries no Tokio; only a crate that enables the feature, viola-mcp, gets the Tokio client.
guard: channel-tokio-free.sh
@row_check n=284 row=decision/viola-channel-tokio-free-by-default sha=354cdf11d189 v=holds guard=true judge=sonnet head=8834195b0683
at: crates/viola-channel/Cargo.toml:23
quote: tokio = { workspace = true, optional = true, features = ["io-util"] }
at: crates/viola-mcp/Cargo.toml:16
quote: viola-channel = { path = "../viola-channel", features = ["tokio"] }
guard_at: scripts/channel-tokio-free.sh:2
@row_add n=285 row=decision/test-support-never-in-product area=occupied-resources rec=fct-4whhx2hw@2
name: Test-support feature kept out of product builds
rule: The viola-channel test-support feature is enabled only by the root package's dev-dependencies, so no product build carries it.
guard: release-check.sh refuses a release artifact built with it
@row_check n=285 row=decision/test-support-never-in-product sha=4bfe858b17dd v=holds guard=true judge=sonnet head=8834195b0683
at: Cargo.toml:175
quote: viola-channel = { path = "crates/viola-channel", features = ["test-support"] }
guard_at: scripts/release-check.sh:19
@row_add n=286 row=name/crate/viola-fuzz area=occupied-resources rec=fct-3fwjuai5@1,fct-7f6dkl55@1
class: crate
name: viola-fuzz
@row_check n=286 row=name/crate/viola-fuzz sha=1612dbe06bbd v=holds judge=tool head=8834195b0683
at: fuzz/Cargo.toml:2
quote: name = "viola-fuzz"
@row_add n=287 row=name/other/viola-name area=occupied-resources rec=fct-tvaxd6b5@1
class: other
name: VIOLA_NAME
@row_check n=287 row=name/other/viola-name sha=ab593c989db3 v=holds judge=sonnet head=8834195b0683
at: src/run/mod.rs:123
quote: ("VIOLA_NAME".into(), OsString::from(name.as_ref())),
@row_add n=288 row=contract/child-environment/viola-name area=occupied-resources rec=fct-tvaxd6b5@1
boundary: The wrapped child's environment
item: VIOLA_NAME
promise: Set by viola run to the instance's ViolaName; when it is absent, names no existing instance directory, or the instance fails its strict-modes check, every hook event and dialog path exits 0 silently, reading no config, no stdin and writing nothing.
@row_check n=288 row=contract/child-environment/viola-name sha=eab1b6abe1e2 v=holds judge=sonnet head=8834195b0683
at: src/run/mod.rs:123
quote: ("VIOLA_NAME".into(), OsString::from(name.as_ref())),
at: src/cmd/hook.rs:116-117
quote: let Some(instance) = instance.as_ref().and_then(canonical).filter(trusted) else { return Ok(ExitCode::SUCCESS);
@row_add n=289 row=name/other/viola-dir area=occupied-resources rec=fct-esr7jyhf@1
class: other
name: VIOLA_DIR
@row_check n=289 row=name/other/viola-dir sha=e9affefdb262 v=holds judge=sonnet head=8834195b0683
at: src/run/mod.rs:124
quote: ("VIOLA_DIR".into(), instance_dir.as_os_str().to_owned()),
@row_add n=290 row=name/other/viola-bin area=occupied-resources rec=fct-7qo4h5jo@2
class: other
name: VIOLA_BIN
@row_check n=290 row=name/other/viola-bin sha=296cc23c7459 v=holds judge=sonnet head=8834195b0683
at: src/run/mod.rs:125
quote: ("VIOLA_BIN".into(), OsString::from(&pinned.path_fwd)),
@row_add n=291 row=contract/child-environment/viola-bin area=occupied-resources rec=fct-7qo4h5jo@2
boundary: The wrapped child's environment
item: VIOLA_BIN
promise: Set by viola run to the absolute path of the pinned copy of the binary, so skills and Bash tool calls in the session can call that same copy explicitly; viola itself never reads it.
@row_check n=291 row=contract/child-environment/viola-bin sha=30e925bf0a6b v=holds judge=sonnet head=8834195b0683
at: src/run/mod.rs:125
quote: ("VIOLA_BIN".into(), OsString::from(&pinned.path_fwd)),
searched: env::var_os("VIOLA_BIN") · in src/**/*.rs (no read found)
@row_add n=292 row=contract/child-environment/path area=occupied-resources rec=fct-fz733ta4@1
boundary: The wrapped child's environment
item: PATH
promise: Prefixed by viola run with the pinned copy's folder.
@row_check n=292 row=contract/child-environment/path sha=4968d92a6149 v=holds judge=sonnet head=8834195b0683
at: src/run/mod.rs:117
quote: let dirs = std::iter::once(pinned_dir).chain(std::env::split_paths(&inherited));
@row_add n=293 row=decision/claude-env-stripped-from-child area=occupied-resources rec=fct-jujxlicd@2
name: Inherited CLAUDE names stripped from the child
rule: Every inherited CLAUDE* environment name, matched exactly, is removed from the wrapped child's environment, and the identity-floor names are removed whatever the persistent environment says.
except: A name listed in config.json claude_env_keep is kept, unless it is an identity-floor name.
@row_check n=293 row=decision/claude-env-stripped-from-child sha=38a4a958ac31 v=holds judge=sonnet head=8834195b0683
at: crates/viola-agent-claude/src/lib.rs:69
quote: if !text.starts_with("CLAUDE") {
at: crates/viola-agent-claude/src/lib.rs:73
quote: if !floor && persistent.contains(&text) {
at: crates/viola-agent-claude/src/lib.rs:76
quote: plan.remove.push(name);
@row_add n=294 row=decision/env-strip-reads-names-only area=occupied-resources rec=fct-jujxlicd@2
name: Environment strip reads names only
rule: The child-environment strip and its logs handle variable names only and never read a value.
@row_check n=294 row=decision/env-strip-reads-names-only sha=d54d7685292b v=holds judge=sonnet head=8834195b0683
at: crates/viola-agent-claude/src/lib.rs:62-65
quote: pub fn plan_strip( inherited: impl IntoIterator<Item = OsString>, persistent: &[String], ) -> StripPlan {
@row_add n=295 row=decision/plugin-env-vars-unused area=occupied-resources rec=fct-55fxxi35@1
name: Plugin-provided environment variables unused
rule: viola never uses CLAUDE_PLUGIN_ROOT or CLAUDE_PLUGIN_DATA to locate its binary, and reads none of those or CLAUDE_ENV_FILE and CLAUDE_PROJECT_DIR.
@row_check n=295 row=decision/plugin-env-vars-unused sha=c8477b29bca2 v=holds judge=sonnet head=8834195b0683
searched: CLAUDE_PLUGIN_ROOT · in tracked source (*.rs, tests, scripts; none outside the pipeline records)
searched: CLAUDE_PLUGIN_DATA · in tracked source
searched: CLAUDE_ENV_FILE · in tracked source
searched: CLAUDE_PROJECT_DIR · in tracked source
@row_add n=296 row=name/other/agent-run-chunk-base area=occupied-resources rec=fct-mkgpzodl@1
class: other
name: AGENT_RUN_CHUNK_BASE
@row_check n=296 row=name/other/agent-run-chunk-base sha=dbda9536fa58 v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/bin/viola-harness.rs:194
quote: std::env::var("AGENT_RUN_CHUNK_BASE").ok(),
@row_add n=297 row=name/other/agent-run-keep-homes area=occupied-resources rec=fct-mkgpzodl@1
class: other
name: AGENT_RUN_KEEP_HOMES
@row_check n=297 row=name/other/agent-run-keep-homes sha=9b380b118adf v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/session.rs:125
quote: flag_set(std::env::var("AGENT_RUN_KEEP_HOMES")),
@row_add n=298 row=name/other/agent-run-keep-failed area=occupied-resources rec=fct-mkgpzodl@1
class: other
name: AGENT_RUN_KEEP_FAILED
@row_check n=298 row=name/other/agent-run-keep-failed sha=deeb37b146bc v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/session.rs:126
quote: flag_set(std::env::var("AGENT_RUN_KEEP_FAILED")),
@row_add n=299 row=name/other/fake-agent-pump-delay-ms area=occupied-resources rec=fct-4roqlggd@1
class: other
name: FAKE_AGENT_PUMP_DELAY_MS
@row_check n=299 row=name/other/fake-agent-pump-delay-ms sha=78044c92e852 v=holds judge=sonnet head=8834195b0683
at: src/cmd/run.rs:63
quote: let ms = std::env::var("FAKE_AGENT_PUMP_DELAY_MS")
@row_add n=300 row=name/other/fake-agent-hook-panic area=occupied-resources rec=fct-4roqlggd@1
class: other
name: FAKE_AGENT_HOOK_PANIC
@row_check n=300 row=name/other/fake-agent-hook-panic sha=0575b980fef3 v=holds judge=sonnet head=8834195b0683
at: src/cmd/hook/seam.rs:10
quote: panic_on(std::env::var_os("FAKE_AGENT_HOOK_PANIC"));
@row_add n=301 row=decision/test-seams-not-configuration area=occupied-resources rec=fct-4roqlggd@1
name: Test seams are not a configuration channel
rule: Test-seam environment variables are read only by a viola built with the test-only fake-agent feature, never by a release build, and a seam disables no control and widens no redaction.
@row_check n=301 row=decision/test-seams-not-configuration sha=a8cdb18069eb v=holds judge=sonnet head=8834195b0683
at: src/cmd/run.rs:61-62
quote: #[cfg(feature = "fake-agent")] fn hold_pump_start() {
at: src/cmd/hook/seam.rs:8-10
quote: #[cfg(feature = "fake-agent")] pub(super) fn panic_if_asked() { panic_on(std::env::var_os("FAKE_AGENT_HOOK_PANIC"));
@row_add n=302 row=name/other/mcp-serve-test-child area=occupied-resources rec=fct-rw5zqusn@1
class: other
name: MCP_SERVE_TEST_CHILD
@row_check n=302 row=name/other/mcp-serve-test-child sha=a3d177d8dac7 v=holds judge=sonnet head=8834195b0683
at: crates/viola-mcp/src/lib.rs:112
quote: const CHILD: &str = "MCP_SERVE_TEST_CHILD";
@row_add n=303 row=decision/viola-env-namespace-product-only area=occupied-resources rec=fct-rw5zqusn@1
name: VIOLA_ environment namespace is product plumbing
rule: Test-only environment markers stay outside the VIOLA_* namespace, which is reserved for product plumbing.
@row_check n=303 row=decision/viola-env-namespace-product-only sha=570f2c000a3c v=holds judge=sonnet head=8834195b0683
at: crates/viola-mcp/src/lib.rs:112
quote: const CHILD: &str = "MCP_SERVE_TEST_CHILD";
@row_add n=304 row=contract/human-output-colour-env/no-color area=occupied-resources rec=fct-lak6evbp@1
boundary: Human output colour environment
item: NO_COLOR
promise: Present, an empty value included, switches colour off.
@row_check n=304 row=contract/human-output-colour-env/no-color sha=c08388cf5820 v=holds judge=sonnet head=8834195b0683
at: src/human/style.rs:95
quote: no_color: std::env::var_os("NO_COLOR").is_some(),
@row_add n=305 row=contract/human-output-colour-env/term area=occupied-resources rec=fct-lak6evbp@1
boundary: Human output colour environment
item: TERM
promise: Exactly dumb switches colour off; a value containing 256color picks 256 colours.
@row_check n=305 row=contract/human-output-colour-env/term sha=4bcf17dc3496 v=holds judge=sonnet head=8834195b0683
at: src/human/style.rs:65
quote: || facts.term.as_deref() == Some("dumb")
at: src/human/style.rs:75
quote: .is_some_and(|term| term.contains("256color"))
@row_add n=306 row=contract/human-output-colour-env/colorterm area=occupied-resources rec=fct-lak6evbp@1
boundary: Human output colour environment
item: COLORTERM
promise: truecolor or 24bit picks truecolor.
@row_check n=306 row=contract/human-output-colour-env/colorterm sha=fc3412363aa3 v=holds judge=sonnet head=8834195b0683
at: src/human/style.rs:70
quote: if matches!(facts.colorterm.as_deref(), Some("truecolor" | "24bit")) {
@row_add n=307 row=decision/colour-env-decides-only-sgr area=occupied-resources rec=fct-lak6evbp@1
name: Colour variables decide only SGR output
rule: Every viola build's human output reads NO_COLOR, TERM and COLORTERM, never sets them, only compares them, never echoes them, and they decide only whether and how deep SGR bytes are written; COLUMNS, CLICOLOR, CLICOLOR_FORCE and CI are not read.
why: ratified: the colour variables are to be read
@row_check n=307 row=decision/colour-env-decides-only-sgr sha=f6ceb2ba4c67 v=holds judge=sonnet head=8834195b0683
at: src/human/style.rs:95
quote: no_color: std::env::var_os("NO_COLOR").is_some(),
at: src/human/style.rs:96
quote: term: text("TERM"),
at: src/human/style.rs:97
quote: colorterm: text("COLORTERM"),
searched: COLUMNS|CLICOLOR|"CI" · in src/**/*.rs (only a comment at src/cmd/list.rs:61 saying never COLUMNS; CI is read only in crates/viola-e2e)
@row_add n=308 row=decision/config-falls-back-to-defaults area=occupied-resources rec=fct-i4wacyrp@2
name: Unusable configuration falls back to defaults
rule: config.json is read under the frame cap, and an unusable file or value falls back to the default with a parse-rejected config-json line.
why: so no value switches the budget gate off or makes the dialog hold zero or unbounded
@row_check n=308 row=decision/config-falls-back-to-defaults sha=5faf9690a84b v=holds judge=sonnet head=8834195b0683
at: src/obs.rs:187
quote: Ok(file) => match file.take(MAX_FRAME).read_to_end(&mut bytes) {
at: src/obs.rs:224
quote: parser = "config-json",
at: src/run/dialog.rs:180
quote: Some(None) => (DialogHold::default(), vec![ConfigRejection::Malformed]),
@row_add n=309 row=decision/pinned-binary-copy area=occupied-resources rec=fct-hnee7xdr@2
name: Pinned copy of the binary
rule: viola run places a content-hashed pinned copy of its binary once, never clobbers or replaces it, re-hashes an existing copy on every start, and a hash mismatch refuses the start with exit 1 pinned-hash-mismatch, leaving the file as found.
@row_check n=309 row=decision/pinned-binary-copy sha=825813216f7c v=holds judge=sonnet head=8834195b0683
at: crates/viola-state/src/pin.rs:90
quote: Ok(_) => return Err(PinError::HashMismatch),
at: crates/viola-state/src/pin.rs:91
quote: Err(e) if e.kind() == io::ErrorKind::NotFound => place_absent(&path, bytes, &hash)?,
at: src/cmd/run.rs:284
quote: "pinned-hash-mismatch",
@row_add n=310 row=decision/no-deletion-of-pinned-copies area=occupied-resources rec=fct-hnee7xdr@2
name: Pinned copies and plugin folders never deleted
rule: v1 never deletes pinned copies or plugin folders, and each instance snapshot records the pinned path it uses.
why: so any later cleanup can skip copies that a live or stale instance still calls
@row_check n=310 row=decision/no-deletion-of-pinned-copies sha=77a1fb2033b9 v=holds judge=sonnet head=8834195b0683
at: crates/viola-state/src/snapshot.rs:44
quote: pub pinned_bin: String,
searched: remove_file|remove_dir_all|remove_dir · in src/**/*.rs and crates/viola-state/src (only test code and verify tempdirs, none on bin/ or plugin/)
@row_add n=311 row=decision/strict-modes-check-before-log area=occupied-resources rec=fct-ikjy6gp2@1,fct-cicfj3bq@2
name: Strict-modes check before any log
rule: Every command that uses the viola home or an instance's files checks their modes, a folder before what it holds, before opening any log; a refused check exits 1 with the home-state or instance-state unable and hint pair, writes no line and starts no child.
why: opening a log narrows an existing home
@row_check n=311 row=decision/strict-modes-check-before-log sha=4e3c200f3866 v=holds judge=sonnet head=8834195b0683
at: src/cmd/run.rs:203
quote: Ok(check_home_tree(&home, Some(name), Some(&pin_key(&exe))).map(|()| exe))
at: src/cmd/run.rs:224
quote: let (level, rejection) = obs::read_diagnostics_level(home);
@row_add n=312 row=decision/run-pins-checked-bytes area=occupied-resources rec=fct-ikjy6gp2@1
name: Run pins the bytes it checked
rule: viola run pins the very executable bytes its start check examined.
@row_check n=312 row=decision/run-pins-checked-bytes sha=4eca02322574 v=holds judge=sonnet head=8834195b0683
at: crates/viola-state/src/pin.rs:82
quote: pub fn pin_bytes(home: &Path, bytes: &[u8]) -> Result<Pinned, PinError> {
at: src/cmd/run.rs:201
quote: let exe = std::fs::read(std::env::current_exe()?)?;
@row_add n=313 row=decision/process-logs-hold-codes-only area=occupied-resources rec=fct-5jecrew3@2
name: Home-level process logs hold codes only
rule: Home-level per-role process logs carry codes only, one line per write; content-bearing detail stays in the instance detail files.
@row_check n=313 row=decision/process-logs-hold-codes-only sha=f1c2a9943491 v=holds judge=sonnet head=8834195b0683
at: src/obs.rs:80
quote: pub(crate) fn open_role_file(
at: src/obs.rs:292
quote: .join(format!("detail-{}.ndjson", process.as_str()))
@row_add n=314 row=decision/diagnostics-open-checks-modes area=occupied-resources rec=fct-5jecrew3@2
name: Diagnostics files checked before open
rule: Every open of a process-log or detail file first checks the existing diagnostics folder and file: a symbolic link is refused and both must meet the read rule; a refusal is a failed open, ending the role writer at a sink or dropping the detail line.
@row_check n=314 row=decision/diagnostics-open-checks-modes sha=4ff0bc533e89 v=holds judge=sonnet head=8834195b0683
at: src/obs.rs:71-76
quote: fn check_diagnostics(dir: &Path, file: &Path) -> io::Result<()> { [dir, file] .into_iter() .try_for_each(viola_state::strict::check_diagnostics) .map_err(|_| io::Error::from(io::ErrorKind::PermissionDenied))
at: src/obs.rs:89
quote: check_diagnostics(&dir, &path)?;
@row_add n=315 row=contract/diagnostics-lines/process-log-line area=occupied-resources rec=fct-5jecrew3@2,fct-gn6wx4l2@1
boundary: Diagnostics lines
item: home-level process-log line
promise: Format owned by obs and fixed by the default-deny, self-contained schema diag-line.v1.
@row_check n=315 row=contract/diagnostics-lines/process-log-line sha=36e8494268df v=holds judge=sonnet head=8834195b0683
at: schemas/diag-line.v1.json:3
quote: "$id": "https://viola.local/schemas/diag-line.v1.json",
at: schemas/diag-line.v1.json:106
quote: "unevaluatedProperties": false,
@row_add n=316 row=contract/diagnostics-lines/detail-line area=occupied-resources rec=fct-5jecrew3@2,fct-gn6wx4l2@1
boundary: Diagnostics lines
item: instance detail line
promise: Format owned by obs and fixed by the default-deny, self-contained schema diag-detail.v1.
@row_check n=316 row=contract/diagnostics-lines/detail-line sha=59aff30835bf v=holds judge=sonnet head=8834195b0683
at: schemas/diag-detail.v1.json:3
quote: "$id": "https://viola.local/schemas/diag-detail.v1.json",
at: schemas/diag-detail.v1.json:29
quote: "unevaluatedProperties": false
@row_add n=317 row=decision/events-log-never-rotated area=occupied-resources rec=fct-cicfj3bq@2
name: Events log never rotated
rule: v1 never rotates or truncates an instance's events.ndjson; any future size bound must keep existing offsets valid.
why: wait's after and send's cursor are byte offsets into it
@row_check n=317 row=decision/events-log-never-rotated sha=4b0e0306d7af v=holds judge=sonnet head=8834195b0683
searched: rotate · in crates/viola-state/src/*.rs
searched: set_len · in crates/viola-state/src/*.rs
searched: truncat · in crates/viola-state/src/*.rs (only events.rs:2 "never truncated or rewritten", pin.rs hash, fs.rs:67 `.truncate(false)`)
@row_add n=318 row=contract/events-ndjson/byte-offsets area=occupied-resources rec=fct-cicfj3bq@2
boundary: events.ndjson
item: byte offsets
promise: The after of wait and the cursor of send are byte offsets into the instance's events.ndjson.
@row_check n=318 row=contract/events-ndjson/byte-offsets sha=7e642c040dc6 v=holds judge=sonnet head=8834195b0683
at: src/cmd/wait.rs:24-26
quote: /// Return the first event whose line starts at or after this cursor (a byte offset)     #[arg(long, value_name = "CURSOR")]     after: Option<u64>,
at: crates/viola-state/src/events.rs:89-90
quote: let tail = tail(instance_dir)?;     let at = tail.line_start();
at: crates/viola-state/src/events.rs:98
quote: Ok(meta) => Ok(meta.len()),
@row_add n=319 row=decision/shared-budget-reading area=occupied-resources rec=fct-ljlz2wdv@1
name: One shared budget reading
rule: The newest rate_limits reading across wrapped sessions is kept timestamped, last writer wins, replaced whole under a lock, written only by hook statusline; every failed read of it counts as no reading.
@row_check n=319 row=decision/shared-budget-reading sha=3c2c9b089f9c v=holds judge=sonnet head=8834195b0683
at: src/cmd/hook/statusline.rs:127
quote: reading.is_some_and(|reading| write_budget(&instance.home, &reading, Utc::now()).is_ok());
at: crates/viola-state/src/budget.rs:59-61
quote: let lock = open_private_lock(&home.join(BUDGET_LOCK))?; lock.lock()?; replace_private(&home.join(BUDGET), &bytes, FILE_MODE)
at: crates/viola-state/src/budget.rs:87
quote: pub fn read_budget(home: &Path) -> Option<BudgetReading> {
@row_add n=320 row=decision/statusline-source-precedence area=occupied-resources rec=fct-tbyc7us6@1,fct-t3ics2st@1
name: Source of the user's statusline command
rule: When the home holds statusline-source.json it is the source of the user's statusline command; otherwise, only in the default home, the statusLine command of the user's Claude settings is taken; viola writes neither and reads both under the frame bound.
@row_check n=320 row=decision/statusline-source-precedence sha=4f8f9a0541e4 v=holds judge=sonnet head=8834195b0683
at: src/cmd/run.rs:533-535
quote: let own = home.join(STATUSLINE_SOURCE); if own.exists() { return Some(own);
at: src/cmd/run.rs:539-540
quote: (default == home).then(|| { statusline::USER_SETTINGS
at: src/cmd/run.rs:552-554
quote: File::open(source) .ok()? .take(MAX_FRAME)
@row_add n=321 row=decision/verify-sole-stamps-writer area=occupied-resources rec=fct-vevvxa42@1
name: verify is the one stamps writer
rule: viola verify is the one writer of the ledger stamps; run's version gate reads them behind the strict-modes check, and a refusal records cli_verified false.
@row_check n=321 row=decision/verify-sole-stamps-writer sha=109aa6b12dd2 v=holds judge=sonnet head=8834195b0683
at: src/cmd/verify.rs:273
quote: let written = update_stamps(home, |existing| {
at: src/run/version_gate.rs:172
quote: let (verified, detail) = stamps_verdict(read_stamps_strict(home), version);
@row_add n=322 row=decision/verify-probe-dirs-drop-guarded area=occupied-resources rec=fct-ns4etpk3@1,fct-dut3tvmp@1
name: verify probe dirs removed by drop guard
rule: Every probe dir viola verify creates is private and removed whole when verify passes, fails or is stopped by Ctrl-C or TERM, only the dirs that run created, never by a pattern or a listing.
why: its captures are transient, content-bearing raw hook payloads
@row_check n=322 row=decision/verify-probe-dirs-drop-guarded sha=1499d2682306 v=holds judge=sonnet head=8834195b0683
at: src/cmd/verify.rs:389-391
quote: impl Drop for ProbeDir { fn drop(&mut self) { let _ = std::fs::remove_dir_all(&self.dir);
at: src/cmd/verify/typed/runs.rs:224-226
quote: impl Drop for TrustedDir { fn drop(&mut self) { let _ = std::fs::remove_dir_all(&self.0);
@row_add n=323 row=decision/test-evidence-survives-kill area=occupied-resources rec=fct-ut3pz3ml@1,fct-w4pd7id3@1
name: Test evidence survives a killed test
rule: Real-process test rigs stream step evidence to report files outside the viola home, kept when the test panics or is killed and removed when it passes; viola never reads or writes them.
why: so a test the runner kills keeps its evidence
@row_check n=323 row=decision/test-evidence-survives-kill sha=fee5bf1732c4 v=holds judge=sonnet head=8834195b0683
at: tests/support/watch.rs:62-63
quote: if !std::thread::panicking() { let _ = fs::remove_file(&self.report);
at: crates/viola-pty/src/lib.rs:496-501
quote: if std::thread::panicking() { if !self.dsr_reported { self.report_dsr(); } } else { let _ = std::fs::remove_file(&self.report);
@row_add n=324 row=decision/chaos-home-outside-gates area=occupied-resources rec=fct-swegdoif@2
name: Chaos panic home outside the obs gates
rule: The forced vt100 feed-panic test uses a home outside the e2e home tree, so its deliberate panic line falls outside the zero-panic gate, schema conformance and the secret scan; the test asserts the panic and parse-rejected lines itself.
why: the one named carve-out to the test-data rule
guard: chaos_feed_panic
@row_check n=324 row=decision/chaos-home-outside-gates sha=81275a51aa20 v=holds guard=true judge=sonnet head=8834195b0683
at: tests/support/home.rs:214
quote: pub fn outside_scan() -> Self {
guard_at: tests/chaos_feed_panic.rs:44
@row_add n=325 row=decision/config-keys-match-schema area=occupied-resources rec=fct-6e4hf2ai@1
name: Config keys equal the config schema
rule: The set of configuration keys the product knows equals the set of properties of the tolerant config.json schema, which admits additional properties.
guard: config_keys_are_the_config_schema_properties
@row_check n=325 row=decision/config-keys-match-schema sha=6400ba1cd958 v=holds guard=true judge=sonnet head=8834195b0683
at: src/obs.rs:362
quote: assert_eq!(known, properties);
at: schemas/config.v1.json:18
quote: "additionalProperties": true
guard_at: src/obs.rs:352
@row_add n=326 row=decision/secret-scan-skips-only-chunk-diff area=occupied-resources rec=fct-6edq3mfh@1
name: Secret scan skips only the chunk diff
rule: secret-scan covers every file except the harness's chunk.diff, skipped by exact path, and the harness upload excludes that file.
@row_check n=326 row=decision/secret-scan-skips-only-chunk-diff sha=d4e29d8c69b2 v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/secret_scan.rs:79-80
quote: let mutation_diff = roots.agent_run.join("chunk.diff"); capture.retain(|path| *path != mutation_diff);
at: .github/workflows/ci.yml:94
quote: !target/agent-run/chunk.diff
@row_add n=327 row=decision/test-homes-cleaned-by-owner area=occupied-resources rec=fct-e4sufzw6@1
name: Test homes cleaned by owner process
rule: Harness and test homes are kept in CI for the obs gates, and a new test home removes only the dirs whose recorded owner process is gone, never by age or name.
@row_check n=327 row=decision/test-homes-cleaned-by-owner sha=c01ad5d5f22e v=holds judge=sonnet head=8834195b0683
at: tests/support/home.rs:53
quote: pub fn sweep_gone_owners(base: &Path) -> usize {
at: tests/support/home.rs:195-196
quote: if !flag("AGENT_RUN_KEEP_HOMES") && !flag("AGENT_RUN_KEEP_FAILED") { sweep_gone_owners(&base);
@row_add n=328 row=decision/mutants-own-target-dir area=occupied-resources rec=fct-4jytw3f7@1
name: Mutation runs use their own relative target dir
rule: Mutation testing builds in its own target dir, passed to cargo-mutants as a relative path, so each copied tree builds every test binary in its own copy.
why: a copied default target keeps the original tree's binary paths, so root integration tests would drive the unmutated binary
@row_check n=328 row=decision/mutants-own-target-dir sha=5ddbe299818a v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/run/mutants.rs:31
quote: const MUTANTS_TARGET: &str = "target/mutants";
at: crates/viola-e2e/src/harness/run/mutants.rs:248
quote: .env("CARGO_TARGET_DIR", MUTANTS_TARGET)
@row_add n=329 row=decision/run-archive-never-uploaded area=occupied-resources rec=fct-q5vc67hz@1
name: Per-run archive never uploaded
rule: The per-run archive of JUnit files and mutation outcomes stays local, outside the agent-run artifacts, and is never uploaded.
why: The mutation outcomes file carries absolute argv paths.
@row_check n=329 row=decision/run-archive-never-uploaded sha=74c74234fc27 v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/run.rs:184
quote: const ARCHIVE: &str = "target/run-archive";
at: .github/workflows/ci.yml:93-94
quote: target/agent-run/ !target/agent-run/chunk.diff
@row_add n=330 row=decision/agent-driven-development area=cross-cutting-patterns rec=fct-iki7xwfk@1,fct-eiwmnbsy@1
name: Agent-driven development style
rule: The project's development style is agent-driven.
@row_check n=330 row=decision/agent-driven-development sha=94c98e9407b2 v=unverifiable judge=sonnet head=8834195b0683
why: not a code claim
@row_add n=331 row=decision/hook-fails-open area=cross-cutting-patterns rec=fct-7h5nww77@1,fct-vf75c7sp@2
name: Hook fails open toward the human
rule: Any internal failure in viola hook produces exit 0 with no body, so the dialog renders for the human; viola hook always exits 0.
@row_check n=331 row=decision/hook-fails-open sha=c65229ef11b7 v=holds judge=sonnet head=8834195b0683
at: src/main.rs:133
quote: (Role::Hook, _) => ExitCode::SUCCESS,
@row_add n=332 row=decision/unconfirmed-delivery-not-delivered area=cross-cutting-patterns rec=fct-7h5nww77@1
name: Unconfirmed delivery reported as not-delivered
rule: Any failure to confirm delivery is reported as not-delivered and never retried blindly.
except: A ledger local command whose post-condition is none or not yet measured on this CLI version is reported as ok with confirmed false and detail unconfirmable.
@row_check n=332 row=decision/unconfirmed-delivery-not-delivered sha=5f7f74f19396 v=holds judge=sonnet head=8834195b0683
at: src/cmd/send.rs:193
quote: out.unable(RefusalReason::NotDelivered.as_str(), Some(detail.as_str()));
at: src/cmd/send.rs:214
quote: out.unconfirmable(&ok);
@row_add n=333 row=decision/mixed-version-tolerance area=cross-cutting-patterns rec=fct-74jt2s34@1,fct-tkfup5lp@1
name: Mixed-version tolerance
rule: Every channel params, event, snapshot and config carries v; readers of own formats skip unknown kinds and fields and surface the counts.
@row_check n=333 row=decision/mixed-version-tolerance sha=69a74a46c75a v=holds judge=sonnet head=8834195b0683
at: crates/viola-state/src/events.rs:32
quote: pub v: u32,
at: crates/viola-state/src/snapshot.rs:84
quote: v: u32,
@row_add n=334 row=contract/channel-params/v area=cross-cutting-patterns rec=fct-74jt2s34@1,fct-jtewz5y6@1
boundary: Channel params
item: v
promise: Every channel params object carries the format version v.
@row_check n=334 row=contract/channel-params/v sha=937a16ae200c v=holds judge=sonnet head=8834195b0683
at: crates/viola-channel/src/client.rs:114
quote: params.insert("v".to_owned(), PROTOCOL_V.into());
at: crates/viola-channel/src/client.rs:174
quote: params.insert("v".to_owned(), PROTOCOL_V.into());
@row_add n=335 row=contract/channel-params/sender-version area=cross-cutting-patterns rec=fct-74jt2s34@1,fct-jtewz5y6@1
boundary: Channel params
item: sender version
promise: Every channel params object carries the sender's CARGO_PKG_VERSION.
@row_check n=335 row=contract/channel-params/sender-version sha=73cf0ab67616 v=holds judge=sonnet head=8834195b0683
at: crates/viola-channel/src/client.rs:115
quote: params.insert("sender".to_owned(), VERSION.into());
at: crates/viola-channel/src/client.rs:175
quote: params.insert("sender".to_owned(), VERSION.into());
@row_add n=336 row=contract/channel-errors/32602-newer-request area=cross-cutting-patterns rec=fct-74jt2s34@1
boundary: Channel errors
item: -32602 for a newer request
promise: A request newer than the wrapper supports gets a -32602 error naming the wrapper's version.
@row_check n=336 row=contract/channel-errors/32602-newer-request sha=65a88c4f8d71 v=holds judge=sonnet head=8834195b0683
at: crates/viola-channel/src/server.rs:238
quote: if params.v > PROTOCOL_V {
at: crates/viola-channel/src/server.rs:239
quote: return Err(ProtocolError::UnsupportedVersion);
at: crates/viola-channel/src/lib.rs:95
quote: Self::UnsupportedVersion => json!({"supported": PROTOCOL_V, "wrapper": VERSION}),
@row_add n=337 row=decision/tokio-containment area=cross-cutting-patterns rec=fct-me2myqzf@1,fct-xclilru3@1
name: Tokio containment
rule: Code on the run, hook, send and channel-server paths uses std threads and blocking I/O; only viola-mcp may build a Tokio runtime, and CLI wait is a blocking read on the channel's sync client.
@row_check n=337 row=decision/tokio-containment sha=5f5afbd743b4 v=holds judge=sonnet head=8834195b0683
at: crates/viola-mcp/src/lib.rs:70
quote: let runtime = tokio::runtime::Builder::new_current_thread()
@row_add n=338 row=decision/crash-safe-disk-writes area=cross-cutting-patterns rec=fct-4xccofxh@1
name: Crash-safe disk writes
rule: Append-only logs use one write per line, mutable files are replaced atomically, every guarded file has its own .lock sibling, and readers always tolerate a torn last line.
@row_check n=338 row=decision/crash-safe-disk-writes sha=f3539b1dbe09 v=holds judge=sonnet head=8834195b0683
at: crates/viola-state/src/events.rs:335
quote: file.write_all(&[b"\n", bytes].concat())?;
at: crates/viola-state/src/fs.rs:83
quote: tmp.persist(path).map(|_| ()).map_err(|e| e.error.into())
at: crates/viola-state/src/stamps.rs:17
quote: const STAMPS_LOCK: &str = "stamps.json.lock";
@row_add n=339 row=contract/hook-output/stdout area=cross-cutting-patterns rec=fct-aqohicxz@1
boundary: Hook output
item: stdout and stderr
promise: viola hook writes only its decision body to stdout and nothing to stderr.
@row_check n=339 row=contract/hook-output/stdout sha=cd487ae123c2 v=holds judge=sonnet head=8834195b0683
at: src/cmd/hook.rs:133
quote: let out = &mut std::io::stdout().lock();
at: Cargo.toml:257
quote: print_stderr = "deny"
@row_add n=340 row=contract/hook-statusline-output/stdout area=cross-cutting-patterns rec=fct-aqohicxz@1
boundary: Hook statusline output
item: stdout
promise: The hook statusline pass-through carries the user's status line output on stdout and nothing else; its --capture probe arm prints nothing.
@row_check n=340 row=contract/hook-statusline-output/stdout sha=ca0094985db4 v=holds judge=sonnet head=8834195b0683
at: src/cmd/hook/statusline.rs:143
quote: let _ = io.out.write_all(&stdout).and_then(|()| io.out.flush());
at: src/cmd/hook.rs:94-96
quote: let shell = args.shell.as_deref(); capture_statusline(dir, shell, &mut std::io::stdin().lock()); return Ok(ExitCode::SUCCESS);
@row_add n=341 row=decision/run-terminal-child-only area=cross-cutting-patterns rec=fct-aqohicxz@1
name: Run writes only the child's output to the terminal
rule: While the child runs, viola run writes nothing to the terminal except the child's own output.
why: The terminal carries the child's screen.
@row_check n=341 row=decision/run-terminal-child-only sha=72f2490a7e05 v=holds judge=sonnet head=8834195b0683
searched: println!|eprintln!|print!( · in src/run
@row_add n=342 row=decision/codes-only-process-logs area=cross-cutting-patterns rec=fct-aqohicxz@1
name: Codes-only process logs per role
rule: Every role's codes-only process log goes to its home-level role file under diagnostics (run, hook, mcp or cli ndjson).
@row_check n=342 row=decision/codes-only-process-logs sha=5adcd82ce53e v=holds judge=sonnet head=8834195b0683
at: src/obs.rs:62
quote: Some(format!("{stem}.ndjson"))
@row_add n=343 row=decision/content-detail-per-instance area=cross-cutting-patterns rec=fct-aqohicxz@1
name: Content-bearing detail only in the instance
rule: Content-bearing diagnostic detail belongs only in the instance's per-role detail ndjson file.
@row_check n=343 row=decision/content-detail-per-instance sha=71df26187998 v=holds judge=sonnet head=8834195b0683
at: src/obs.rs:292
quote: .join(format!("detail-{}.ndjson", process.as_str()))
@row_add n=344 row=contract/mcp-output/stdout area=cross-cutting-patterns rec=fct-aqohicxz@1
boundary: MCP server output
item: stdout and stderr
promise: viola mcp writes only MCP frames to stdout; when its role file cannot be opened it exits 1 and prints nothing.
@row_check n=344 row=contract/mcp-output/stdout sha=03988b1350e8 v=stale judge=sonnet head=8834195b0683
at: src/obs.rs:141
quote: let file = open_role_file(home, process, instance.as_ref())?;
at: src/main.rs:80
quote: /// `viola mcp`: its failure prints nothing, stdout being the MCP frames.
at: src/main.rs:138
quote: ) => ExitCode::from(1),
shows: A role file that cannot be opened makes the init fail and the mcp role then prints nothing at all and exits with code 1; no code writes JSON to stderr.
cell: promise
value: viola mcp writes only MCP frames to stdout; when its role file cannot be opened it exits 1 and prints nothing.
@row_add n=345 row=decision/cli-verbs-human-stderr area=cross-cutting-patterns rec=fct-aqohicxz@1
name: Short-lived CLI verbs may use stderr
rule: Short-lived CLI verbs may use stderr for their human output.
@row_check n=345 row=decision/cli-verbs-human-stderr sha=39bdcbd2948e v=holds judge=sonnet head=8834195b0683
at: src/cmd/wait.rs:70
quote: let _ = human::write_waiting(&mut io::stderr().lock(), name);
@row_add n=346 row=decision/plugin-paths-forward-slashes area=cross-cutting-patterns rec=fct-fa2klbw5@3
name: Forward slashes in plugin paths
rule: Every path written into plugin files uses forward slashes.
@row_check n=346 row=decision/plugin-paths-forward-slashes sha=edc9d2138cf8 v=holds judge=sonnet head=8834195b0683
at: src/cmd/plugin.rs:122
quote: path.to_string_lossy().replace('\\', "/")
@row_add n=347 row=decision/hashed-short-endpoint-names area=cross-cutting-patterns rec=fct-fa2klbw5@3
name: Hashed short endpoint names
rule: Endpoint names are hashed and short.
@row_check n=347 row=decision/hashed-short-endpoint-names sha=6b2265c6c856 v=holds judge=sonnet head=8834195b0683
at: crates/viola-channel/src/endpoint.rs:30
quote: let hash = fnv1a(name.bytes().chain([0]).chain(home.bytes()));
@row_add n=348 row=decision/os-gated-tests-name-reason area=cross-cutting-patterns rec=fct-fa2klbw5@3
name: OS-gated tests name their reason
rule: Every OS-gated test names its reason in the census, and a gate whose predicate names windows or macos reads as a fault even when a row names it.
guard: contract_os_gated_tests
@row_check n=348 row=decision/os-gated-tests-name-reason sha=a95db8060c92 v=holds guard=true judge=sonnet head=8834195b0683
at: tests/contract_os_gated_tests.rs:533
quote: .map(|(file, test, gate)| format!("another OS: {file} {test} cfg({gate})")),
guard_at: tests/contract_os_gated_tests.rs:33
@row_add n=349 row=decision/ci-linux-fake-agent area=cross-cutting-patterns rec=fct-fa2klbw5@3,fct-f53f3qjo@2
name: CI on Linux against the fake agent
rule: CI runs on GitHub Actions ubuntu-latest against the fake agent.
@row_check n=349 row=decision/ci-linux-fake-agent sha=287fc5f7f1c1 v=holds judge=sonnet head=8834195b0683
at: .github/workflows/ci.yml:18
quote: os: [ubuntu-latest]
at: .github/workflows/ci.yml:281
quote: run: cargo clippy --workspace --all-targets --features fake-agent -- -D warnings
@row_add n=350 row=decision/untrusted-upstream-text area=cross-cutting-patterns rec=fct-3bclffxa@1
name: Upstream text is untrusted
rule: A driven session's Stop text and dialog payloads are content, never commands to viola; viola forwards them as escaped strings and never interprets them.
@row_check n=350 row=decision/untrusted-upstream-text sha=9eab2708e7a2 v=holds judge=sonnet head=8834195b0683
at: src/human/escape.rs:5
quote: pub(crate) fn escape_message(text: &str) -> String {
@row_add n=351 row=decision/same-user-endpoint-trust area=cross-cutting-patterns rec=fct-ao5mmzh3@2
name: Local endpoint trusts the same OS user only
rule: v1 trusts every process of the same OS user and no other to send prompts and answer dialogs; each endpoint socket file must not be connectable by another OS user.
@row_check n=351 row=decision/same-user-endpoint-trust sha=9cffe1257b58 v=holds judge=sonnet head=8834195b0683
at: crates/viola-state/src/strict.rs:229
quote: if mode & 0o077 != 0 {
at: crates/viola-channel/src/client.rs:144-145
quote: pub fn peer_admitted(observed: Option<u32>) -> bool { peer_euid_admitted(observed, own_euid())
@row_add n=352 row=absence/web-or-graphical-frontend area=project-intent rec=fct-iigxhka7@3
capability: A web or graphical frontend
@row_check n=352 row=absence/web-or-graphical-frontend sha=6fce34e0d025 v=holds judge=sonnet head=8834195b0683
searched: ratatui|crossterm|axum|hyper|actix|warp|tauri|egui|eframe|iced|tiny_http · in Cargo.toml, crates/*/Cargo.toml, src/**/*.rs, crates/*/src/**/*.rs
searched: TcpListener|tokio::net::Tcp|<html|WebSocket · in src/**/*.rs, crates/*/src/**/*.rs
@row_add n=353 row=decision/wrap-unmodified-claude area=project-intent rec=fct-hr7nlfjh@1
name: Wrap the unmodified claude CLI
rule: viola drives the unmodified claude CLI wrapped in a PTY, typing at turn boundaries and answering dialogs through hooks, with a one-driver wheel the human can take at any moment.
@row_check n=353 row=decision/wrap-unmodified-claude sha=339bfeddd716 v=holds judge=sonnet head=8834195b0683
at: crates/viola-pty/src/lib.rs:131-132
quote: let pair = native_pty_system() .openpty(pty_size(spec.size))
at: crates/viola-agent-claude/src/dialog.rs:407
quote: json!({"hookSpecificOutput": {"hookEventName": "PermissionRequest", "decision": decision}})
at: src/run/wheel.rs:42
quote: (Input::Human, Wheel::Driver, _) => Some((Wheel::Human, WheelCause::HumanInput)),
@row_add n=354 row=decision/modular-monolith-no-daemon area=project-intent rec=fct-4dkeuwnr@1
name: Modular monolith, no daemon
rule: The product is one binary with compiler-enforced internal crates (pty, channel, state, agent-claude, mcp around core) and no daemon.
@row_check n=354 row=decision/modular-monolith-no-daemon sha=8e11e30c078a v=holds judge=sonnet head=8834195b0683
at: Cargo.toml:147-152
quote: viola-core = { path = "crates/viola-core" } viola-pty = { path = "crates/viola-pty" } viola-agent-claude = { path = "crates/viola-agent-claude" } viola-state = { path = "crates/viola-state" } viola-channel = { path = "crates/viola-channel" } viola-mcp = { path = "crates/viola-mcp" }
searched: daemon|setsid|daemonize · in src/**/*.rs, crates/*/src/**/*.rs, Cargo.toml, crates/*/Cargo.toml
@row_add n=355 row=decision/new-capabilities-as-verbs area=project-intent rec=fct-jkmwg2b6@1
name: New capabilities as subcommands or channel verbs
rule: New capabilities arrive as subcommands or channel verbs exposed consistently on the CLI with --json and, where a driver needs them, as MCP tools.
@row_check n=355 row=decision/new-capabilities-as-verbs sha=3cfa053e1504 v=unverifiable judge=sonnet head=8834195b0683
why: not a code claim
@row_add n=356 row=decision/second-agent-sibling-crate area=project-intent rec=fct-jkmwg2b6@1
name: A second driven agent as a sibling crate
rule: A second driven agent arrives as a sibling viola-agent crate, at which point the adapter trait is extracted into viola-core.
@row_check n=356 row=decision/second-agent-sibling-crate sha=f767403bfa05 v=unverifiable judge=sonnet head=8834195b0683
why: not a code claim
@row_add n=357 row=decision/rust-toolchain-pinned area=inherited-defaults rec=fct-xclilru3@1
name: Pinned Rust toolchain
rule: The Rust toolchain is pinned by rust-toolchain.toml, with a declared MSRV and edition 2024.
@row_check n=357 row=decision/rust-toolchain-pinned sha=eade6845cc9e v=holds judge=sonnet head=8834195b0683
at: rust-toolchain.toml:2
quote: channel = "1.98.1"
at: Cargo.toml:187
quote: edition = "2024"
at: Cargo.toml:188
quote: rust-version = "1.96"
@row_add n=358 row=decision/rmcp-for-mcp area=inherited-defaults rec=fct-xclilru3@1
name: rmcp serves MCP
rule: rmcp is what serves MCP.
@row_check n=358 row=decision/rmcp-for-mcp sha=569e60d14ab7 v=holds judge=sonnet head=8834195b0683
at: crates/viola-mcp/src/tools.rs:19
quote: use rmcp::{RoleServer, ServerHandler};
@row_add n=359 row=decision/clap-for-cli area=inherited-defaults rec=fct-xclilru3@1
name: clap parses the CLI
rule: clap is what parses the CLI.
@row_check n=359 row=decision/clap-for-cli sha=66ac4b1b4386 v=holds judge=sonnet head=8834195b0683
at: src/cmd/mod.rs:43
quote: #[derive(Parser)]
@row_add n=360 row=decision/ndjson-logs-and-json-snapshots area=inherited-defaults rec=fct-gfhsqoso@1
name: State in ndjson logs and JSON snapshots
rule: State is kept in ndjson append logs and atomic JSON snapshots guarded by std file locks, with serde_json.
@row_check n=360 row=decision/ndjson-logs-and-json-snapshots sha=de549a74c603 v=holds judge=sonnet head=8834195b0683
at: crates/viola-state/src/events.rs:68
quote: lock.lock()?;
at: crates/viola-state/src/fs.rs:83
quote: tmp.persist(path).map(|_| ()).map_err(|e| e.error.into())
at: Cargo.toml:228
quote: serde_json = { version = "=1.0.151", features = ["preserve_order"] }
@row_add n=361 row=decision/one-shared-persist-helper area=inherited-defaults rec=fct-gfhsqoso@1
name: Snapshots persisted through one shared helper
rule: Atomic JSON snapshots are written with tempfile persist through one shared helper.
@row_check n=361 row=decision/one-shared-persist-helper sha=86708c1b4706 v=holds judge=sonnet head=8834195b0683
at: crates/viola-state/src/fs.rs:79-83
quote: let mut tmp = tempfile::NamedTempFile::new_in(parent)?; restrict(tmp.path(), mode)?; tmp.write_all(bytes)?; tmp.as_file().sync_all()?; tmp.persist(path).map(|_| ()).map_err(|e| e.error.into())
@row_add n=362 row=decision/interprocess-local-sockets area=inherited-defaults rec=fct-jtewz5y6@1
name: interprocess local sockets for IPC
rule: IPC uses interprocess local sockets, one endpoint per viola run.
@row_check n=362 row=decision/interprocess-local-sockets sha=5f8cd85d9fd2 v=holds judge=sonnet head=8834195b0683
at: crates/viola-channel/src/server.rs:13
quote: GenericFilePath, Listener, ListenerOptions, Stream, ToFsName as _,
at: crates/viola-channel/src/endpoint.rs:49
quote: pub fn endpoint_path(name: &ViolaName, home: &Path) -> Result<String, ChannelError> {
@row_add n=363 row=contract/channel/framing area=inherited-defaults rec=fct-jtewz5y6@1
boundary: Channel
item: framing
promise: The channel speaks JSON-RPC 2.0 over ndjson.
@row_check n=363 row=contract/channel/framing sha=1376325bf121 v=holds judge=sonnet head=8834195b0683
at: crates/viola-channel/src/frame.rs:95
quote: json!({"jsonrpc": "2.0", "id": id, "result": result})
at: crates/viola-channel/src/frame.rs:45
quote: line.push('\n');
@row_add n=364 row=contract/channel-result/refusal area=inherited-defaults rec=fct-v6b65upn@1
boundary: Channel result
item: refusal
promise: A refusal is a typed RefusalReason in result.refusal.
@row_check n=364 row=contract/channel-result/refusal sha=971060547a63 v=holds judge=sonnet head=8834195b0683
at: src/run/dialog.rs:146
quote: json!({"refusal": reason.as_str(), "detail": detail})
@row_add n=365 row=decision/anyhow-only-root-bin area=inherited-defaults rec=fct-vf75c7sp@2
name: anyhow only in the root binary
rule: anyhow is used only in the root binary; CLI exit codes are typed.
@row_check n=365 row=decision/anyhow-only-root-bin sha=fb8d4d5214b9 v=holds judge=sonnet head=8834195b0683
at: src/cmd/mod.rs:22
quote: use anyhow::Context as _;
searched: anyhow · in crates/**/*.rs and crates/**/Cargo.toml (no match)
@row_add n=366 row=decision/tolerant-serde-external area=inherited-defaults rec=fct-tkfup5lp@1
name: Tolerant serde for external payloads
rule: External payloads are deserialised tolerantly.
@row_check n=366 row=decision/tolerant-serde-external sha=6e7e15c5e301 v=holds judge=sonnet head=8834195b0683
at: crates/viola-state/src/snapshot.rs:40
quote: #[serde(default, skip_serializing_if = "Option::is_none")]
at: crates/viola-core/src/lib.rs:165
quote: #[serde(other)]
@row_add n=367 row=contract/timestamps/format area=inherited-defaults rec=fct-2d7pt7rb@1
boundary: Timestamps
item: format
promise: Timestamps are RFC 3339 UTC with milliseconds and Z, in fields named ts or ending in _at, produced with chrono.
@row_check n=367 row=contract/timestamps/format sha=c01f608a3b94 v=holds judge=sonnet head=8834195b0683
at: crates/viola-state/src/lib.rs:31
quote: at.to_rfc3339_opts(SecondsFormat::Millis, true)
@row_add n=368 row=decision/claude-shapes-isolated area=inherited-defaults rec=fct-jo2qqab5@1
name: Claude-specific shapes isolated
rule: Claude-specific shapes live only in viola-agent-claude.
@row_check n=368 row=decision/claude-shapes-isolated sha=d964373c0b25 v=holds judge=sonnet head=8834195b0683
at: crates/viola-agent-claude/src/hook.rs:102
quote: last_assistant_message: Option<String>,
@row_add n=369 row=decision/local-cargo-install area=inherited-defaults rec=fct-ll43vwdo@1
name: Local install with embedded plugin
rule: viola is deployed by a local cargo install, with the plugin embedded in the binary and loaded with --plugin-dir.
@row_check n=369 row=decision/local-cargo-install sha=20dbf3f7a2f1 v=holds judge=sonnet head=8834195b0683
at: crates/viola-agent-claude/src/lib.rs:83
quote: pub const PLUGIN_DIR_FLAG: &str = "--plugin-dir";
at: crates/viola-agent-claude/src/lib.rs:126
quote: include_str!(concat!(
at: src/run/mod.rs:129
quote: OsString::from(PLUGIN_DIR_FLAG),
@row_add n=370 row=absence/containers-and-hosting area=inherited-defaults rec=fct-ll43vwdo@1
capability: Containers or hosting of any kind
@row_check n=370 row=absence/containers-and-hosting sha=220c1c97640d v=holds judge=sonnet head=8834195b0683
searched: Dockerfile* · in repository root glob {Dockerfile*,docker-compose*,Containerfile*,.dockerignore}
searched: Containerfile* · in repository root glob
@row_add n=371 row=decision/no-crates-io-publish area=inherited-defaults rec=fct-khy4oaxt@1
name: Nothing published to crates.io
rule: Every workspace crate, the root bin included, sets publish false; nothing is published to crates.io in v1.
@row_check n=371 row=decision/no-crates-io-publish sha=ec5f66b6f7bb v=holds judge=sonnet head=8834195b0683
at: Cargo.toml:190
quote: publish = false
at: crates/viola-mcp/Cargo.toml:7
quote: publish.workspace = true
@row_add n=372 row=decision/dual-mit-apache-license area=inherited-defaults rec=fct-khy4oaxt@1
name: MIT OR Apache-2.0 license
rule: Every workspace crate inherits the license MIT OR Apache-2.0 from the workspace package.
@row_check n=372 row=decision/dual-mit-apache-license sha=19f272430009 v=holds judge=sonnet head=8834195b0683
at: Cargo.toml:189
quote: license = "MIT OR Apache-2.0"
at: crates/viola-mcp/Cargo.toml:6
quote: license.workspace = true
@row_add n=373 row=contract/refusal-reason/unknown-catch-all area=conventions rec=fct-5uakjii2@1
boundary: RefusalReason
item: unknown catch-all
promise: The serde catch-all, so an older client can still read a refusal from a newer wrapper; its detail is a code word or null.
@row_check n=373 row=contract/refusal-reason/unknown-catch-all sha=a2a225627970 v=holds judge=sonnet head=8834195b0683
at: crates/viola-core/src/lib.rs:165
quote: #[serde(other)]
at: src/cmd/client.rs:80
quote: RefusalReason::Unknown.as_str().to_owned()
@row_add n=374 row=contract/refusal-reason/detail area=conventions rec=fct-5uakjii2@1
boundary: RefusalReason
item: detail
promise: A kebab-case string from a closed set per reason, extended only in viola-core; readers treat an unrecognised code-word detail as opaque text, never as an error.
@row_check n=374 row=contract/refusal-reason/detail sha=53228c595202 v=holds judge=sonnet head=8834195b0683
at: crates/viola-core/src/lib.rs:208
quote: #[serde(rename_all = "kebab-case")]
at: src/cmd/client.rs:84
quote: Some(Value::String(detail)) => is_code_word(detail).then(|| detail.clone()),
@row_add n=375 row=contract/refusal-order/first-only area=conventions rec=fct-5uakjii2@1
boundary: Refusal order
item: several refusals
promise: When several refusals apply, the wrapper returns only the first in the fixed order for the verb.
@row_check n=375 row=contract/refusal-order/first-only sha=36419d5124c4 v=holds judge=sonnet head=8834195b0683
at: src/run/send.rs:88
quote: if let Err(detail) = viola_core::validate_paste_text(text) {
at: src/run/send.rs:96
quote: if let Some(detail) = slot.wheel.human_typing() {
@row_add n=376 row=contract/refusal-order/send area=conventions rec=fct-5uakjii2@1
boundary: Refusal order
item: send
promise: control-character, empty-text, human-typing, budget-paused, turn-running, the readiness gate (input-not-ready), human-typing and turn-running read again after the gate, then no-prompt-submitted.
@row_check n=376 row=contract/refusal-order/send sha=59472d14ff03 v=holds judge=sonnet head=8834195b0683
at: src/run/send.rs:93-94
quote: if text.is_empty() { return ctx.not_delivered(None, NotDelivered::EmptyText);
at: src/run/send.rs:108
quote: return ctx.budget_paused(&crossed);
at: src/run/send.rs:116
quote: return ctx.not_delivered(None, NotDelivered::TurnRunning);
at: src/run/send.rs:130
quote: return ctx.not_delivered(None, NotDelivered::InputNotReady);
at: src/run/send.rs:132
quote: if let Some(detail) = slot.wheel.human_typing() {
at: src/run/send.rs:140
quote: return ctx.not_delivered(None, NotDelivered::TurnRunning);
@row_add n=377 row=contract/refusal-order/answer area=conventions rec=fct-5uakjii2@1
boundary: Refusal order
item: answer
promise: control-character on the response's free text, then human-typing, unverified-cli, then unknown-dialog.
@row_check n=377 row=contract/refusal-order/answer sha=2665394a9978 v=holds judge=sonnet head=8834195b0683
at: src/run/dialog.rs:425
quote: .find_map(|t| viola_core::validate_paste_text(t).err())
at: src/run/dialog.rs:427
quote: return Ok(refusal(RefusalReason::NotDelivered, Some(detail.as_str())));
at: src/run/dialog.rs:429
quote: if let Some(detail) = self.wheel.human_typing() {
at: src/run/dialog.rs:435-436
quote: if !self.cli_verified { return Ok(refusal(RefusalReason::UnverifiedCli, None));
at: src/run/dialog.rs:446
quote: Some(NotDelivered::UnknownDialog.as_str()),
@row_add n=378 row=contract/paste-text/control-characters area=conventions rec=fct-5uakjii2@1
boundary: Paste text
item: control characters
promise: LF, CR and TAB pass; every other C0, DEL and C1 is refused as not-delivered control-character, never stripped; the CLI checks before any frame and the wrapper checks again.
@row_check n=378 row=contract/paste-text/control-characters sha=21dbfc4e338c v=holds judge=sonnet head=8834195b0683
at: crates/viola-core/src/lib.rs:366-367
quote: '\n' | '\r' | '\t' => false, '\u{0}'..='\u{1f}' | '\u{7f}'..='\u{9f}' => true,
at: src/cmd/send.rs:182
quote: if let Err(detail) = viola_core::validate_paste_text(text) {
at: src/run/send.rs:88
quote: if let Err(detail) = viola_core::validate_paste_text(text) {
@row_add n=379 row=contract/paste-text/empty area=conventions rec=fct-5uakjii2@1
boundary: Paste text
item: empty text
promise: A text empty after CR LF and lone CR become LF and trailing CR and LF are dropped is refused as not-delivered empty-text, by the CLI and again by the wrapper; one also holding a refused character is control-character.
@row_check n=379 row=contract/paste-text/empty sha=29653b64af69 v=holds judge=sonnet head=8834195b0683
at: crates/viola-agent-claude/src/hook.rs:232-234
quote: let text = text.trim_end_matches(['\r', '\n']); if text.contains('\r') { Cow::Owned(text.replace("\r\n", "\n").replace('\r', "\n"))
at: src/cmd/send.rs:185-187
quote: typed_text(text) .is_empty() .then_some(NotDelivered::EmptyText)
at: src/run/send.rs:93-94
quote: if text.is_empty() { return ctx.not_delivered(None, NotDelivered::EmptyText);
@row_add n=380 row=contract/send/paste-failure-after-issue area=conventions rec=fct-5uakjii2@1
boundary: send refusals
item: paste failing after send-issued
promise: A paste that fails after send-issued is refused as input-not-ready with that send's cursor.
@row_check n=380 row=contract/send/paste-failure-after-issue sha=133d46b7ee05 v=holds judge=sonnet head=8834195b0683
at: src/run/send.rs:147-149
quote: let cursor = ctx.issue(text.len())?; if paste(text).is_err() { return ctx.not_delivered(Some(cursor), NotDelivered::InputNotReady);
@row_add n=381 row=contract/ok-payload/unconfirmable area=conventions rec=fct-5uakjii2@1
boundary: ok payload
item: unconfirmable
promise: unconfirmable is no refusal detail; it appears only as confirmed false with detail unconfirmable, for a ledger-listed local command whose post-condition is none or not yet measured.
@row_check n=381 row=contract/ok-payload/unconfirmable sha=314139f19390 v=holds judge=sonnet head=8834195b0683
at: src/run/send.rs:224
quote: Ok(json!({"ok": {"confirmed": false, "detail": "unconfirmable", "cursor": cursor}}))
at: src/run/send.rs:151-152
quote: if matches!(confirms, Confirms::Nothing) { return ctx.unconfirmable(cursor, slot.clock.now());
@row_add n=382 row=contract/code-word/format area=conventions rec=fct-7qxhyabf@1
boundary: Code word
item: format
promise: A code word is 1 to 64 bytes of lower-case letters, digits and hyphens, starting with a lower-case letter.
@row_check n=382 row=contract/code-word/format sha=efad6ec3f804 v=holds judge=sonnet head=8834195b0683
at: crates/viola-core/src/lib.rs:377-381
quote: bytes.len() <= 64 && bytes.first().is_some_and(u8::is_ascii_lowercase) && bytes .iter() .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || *b == b'-')
@row_add n=383 row=contract/peer-reply/refusal area=conventions rec=fct-6tvyxpxd@1
boundary: A peer's reply
item: refusal
promise: A refusal that is no code word is read as unknown.
@row_check n=383 row=contract/peer-reply/refusal sha=51f49b8bc721 v=holds judge=sonnet head=8834195b0683
at: src/cmd/client.rs:77-81
quote: let reason = if is_code_word(reason) { reason.to_owned() } else { RefusalReason::Unknown.as_str().to_owned() };
at: crates/viola-mcp/src/result.rs:73-77
quote: let reason = if is_code_word(reason) { reason } else { RefusalReason::Unknown.as_str() };
@row_add n=384 row=contract/peer-reply/detail area=conventions rec=fct-6tvyxpxd@1
boundary: A peer's reply
item: detail
promise: A string detail that is no code word is read as null; a non-string detail makes the reply no known reply.
@row_check n=384 row=contract/peer-reply/detail sha=bc8899cfcb29 v=holds judge=sonnet head=8834195b0683
at: src/cmd/client.rs:84-85
quote: Some(Value::String(detail)) => is_code_word(detail).then(|| detail.clone()), Some(_) => return None,
at: crates/viola-mcp/src/result.rs:80-81
quote: Some(Value::String(detail)) => Some(detail.as_str()).filter(|d| is_code_word(d)), Some(_) => return Err(McpError::Internal),
@row_add n=385 row=contract/unreachable-cause/cause-set area=conventions rec=fct-jpo3rq7m@1
boundary: Unreachable cause
item: cause set
promise: Exit 21 has a closed cause set (StrictModes, InstanceDead, UnwrappedName, EndpointLost, ServerVerify, SocketDir as socket-dir-refused) written by both clients, with no serde path; it is no RefusalReason.
@row_check n=385 row=contract/unreachable-cause/cause-set sha=d2b7e7641676 v=holds judge=sonnet head=8834195b0683
at: crates/viola-core/src/lib.rs:183-191
quote: #[derive(Debug, Clone, Copy, PartialEq, Eq)] pub enum UnreachableCause { StrictModes, InstanceDead, UnwrappedName, EndpointLost, ServerVerify, SocketDir, }
at: crates/viola-core/src/lib.rs:201
quote: Self::SocketDir => "socket-dir-refused",
at: src/cmd/client.rs:123
quote: Self::SocketDir => UnreachableCause::SocketDir,
at: crates/viola-mcp/src/result.rs:55
quote: return Ok(unreachable(UnreachableCause::EndpointLost));
@row_add n=386 row=contract/cli-json-error-document/shape area=conventions rec=fct-vuikun3q@2
boundary: CLI --json error document
item: shape
promise: One stdout document of v 1, error code and detail, with stderr empty; codes include strict-modes-failed, already-live, no-session, session-live, cwd-missing, stale-heartbeat, pinned-hash-mismatch, squatted-name, socket-dir-refused, internal-error.
@row_check n=386 row=contract/cli-json-error-document/shape sha=b08bdd1a0e92 v=holds judge=sonnet head=8834195b0683
at: src/cmd/client.rs:227
quote: document(&json!({"v": 1, "error": code, "detail": null}));
at: src/cmd/revive.rs:70
quote: Self::StrictModes => "strict-modes-failed",
at: src/cmd/run.rs:454
quote: Liveness::Live => Some(refused_with(json, "already-live", || refuse_live(name))),
at: src/cmd/run.rs:455
quote: Liveness::Stale => Some(refused_with(json, "stale-heartbeat", || {
at: src/cmd/run.rs:284
quote: "pinned-hash-mismatch",
at: src/cmd/run.rs:295
quote: return Ok(refused_with(launch.json, "squatted-name", || {
at: src/cmd/run.rs:300
quote: return Ok(refused_with(launch.json, "socket-dir-refused", || {
at: src/cmd/mod.rs:101
quote: client::refused_document("internal-error");
at: src/cmd/revive.rs:71
quote: Self::NoSession | Self::UnknownId => "no-session",
at: src/cmd/revive.rs:72
quote: Self::SessionLive => "session-live",
at: src/cmd/revive.rs:73
quote: Self::CwdMissing => "cwd-missing",
@row_add n=387 row=contract/cli-internal-error/output area=conventions rec=fct-vuikun3q@2
boundary: CLI internal error output
item: internal error line
promise: A dispatch error or caught panic in a cli-role verb prints exactly error: internal error on stderr once, with no hint; under --json before a -- the internal-error document is printed on stdout instead.
@row_check n=387 row=contract/cli-internal-error/output sha=71e033e45812 v=holds judge=sonnet head=8834195b0683
at: src/main.rs:66-70
quote: if prints_internal_error(role, outcome) { if json_asked(std::env::args_os()) { cmd::internal_error_document(); } else { human::internal_error();
at: src/human.rs:49
quote: out.write_all(b"error: internal error\n")
at: src/main.rs:145
quote: role == Role::Cli && matches!(outcome, Outcome::Failed | Outcome::Panicked)
@row_add n=388 row=contract/run-start-refusals/socket-dir-refused area=conventions rec=fct-2y3oelpa@1
boundary: viola run and revive start refusals
item: socket-dir-refused
promise: A refused or failed socket directory exits 1 with an unable and hint pair, or the socket-dir-refused document under revive --json, and one process-exit line; no path, pid or uid; a bind failure is not this code.
@row_check n=388 row=contract/run-start-refusals/socket-dir-refused sha=d2ac86e131b3 v=holds judge=sonnet head=8834195b0683
at: src/cmd/run.rs:497-498
quote: if !socket_dir_ready(&endpoint) { return Ok(Bound::SocketDirRefused);
at: src/cmd/run.rs:300
quote: return Ok(refused_with(launch.json, "socket-dir-refused", || {
at: src/cmd/run.rs:412-413
quote: run::log_self_exit(1, Some(detail)); Started::Refused(ExitCode::from(1))
@row_add n=389 row=contract/run-start-refusals/home-tree area=conventions rec=fct-czy74f6t@1
boundary: viola run and revive start refusals
item: home tree
promise: A refused home tree exits 1 with the home-state or instance-state pair, opens no log, writes no line and starts no child; under revive --json it is the strict-modes-failed document.
@row_check n=389 row=contract/run-start-refusals/home-tree sha=cd002b94ffa0 v=holds judge=sonnet head=8834195b0683
at: src/cmd/revive.rs:143
quote: Err(_) if args.json => {
at: src/cmd/revive.rs:144
quote: Refusal::StrictModes.write(&args.name, true);
at: src/cmd/revive.rs:148
quote: refuse_tree(&args.name, refused);
at: src/cmd/run.rs:210
quote: TreeRefused::HomeLevel(_) => {
at: src/cmd/run.rs:214
quote: TreeRefused::InstanceLevel(_) => {
at: src/cmd/revive.rs:152
quote: let persistent = open_wrapper_log(home, &args.name)?;
@row_add n=390 row=contract/verify-refusals/home-tree area=conventions rec=fct-zvpgaziz@1
boundary: viola verify refusals
item: home tree
promise: A refused home tree exits 1 with the home-state unable and hint pair on stderr and no step line, summary or log line; a strict-modes refusal while measuring exits 1 with the same pair after the step lines, and with an instance set its process-exit carries strict-modes-failed.
@row_check n=390 row=contract/verify-refusals/home-tree sha=a7ae2e8877a5 v=stale judge=sonnet head=8834195b0683
at: src/cmd/verify.rs:270
quote: let results = check_rows(&probes);
at: src/cmd/verify.rs:286-288
quote: Err(StampsError::StrictModes(_)) => { refuse_home(); return Ok(Ended::Refused("strict-modes-failed"));
at: src/cmd/verify.rs:90-91
quote: if instance.is_some() { run::log_self_exit(code, detail);
shows: A strict-modes refusal from update_stamps comes after check_rows has printed the step lines, and with an instance set it still writes a process-exit line with detail strict-modes-failed; only the refused home tree (verify.rs:67-69) prints the pair with no step line and no log line.
cell: promise
value: A refused home tree exits 1 with the home-state unable and hint pair on stderr and no step line, summary or log line; a strict-modes refusal while measuring exits 1 with the same pair after the step lines, and with an instance set its process-exit carries strict-modes-failed.
@row_add n=391 row=contract/verify-process-exit/program-not-found area=conventions rec=fct-tg3zks37@2
boundary: viola verify process-exit detail
item: program-not-found
promise: The code verify's process-exit carries when the claude CLI is not found.
@row_check n=391 row=contract/verify-process-exit/program-not-found sha=f2e2e1a87f95 v=holds judge=sonnet head=8834195b0683
at: src/cmd/verify.rs:164
quote: return Ok(Ended::Refused("program-not-found"));
@row_add n=392 row=contract/verify-process-exit/pinned-hash-mismatch area=conventions rec=fct-tg3zks37@2
boundary: viola verify process-exit detail
item: pinned-hash-mismatch
promise: The code verify's process-exit carries when the pinned copy fails its re-hash.
@row_check n=392 row=contract/verify-process-exit/pinned-hash-mismatch sha=d32138ecead1 v=holds judge=sonnet head=8834195b0683
at: src/cmd/verify.rs:171
quote: return Ok(Ended::Refused("pinned-hash-mismatch"));
@row_add n=393 row=contract/verify-process-exit/version-unreadable area=conventions rec=fct-tg3zks37@2
boundary: viola verify process-exit detail
item: version-unreadable
promise: The code verify's process-exit carries when the CLI version cannot be read.
@row_check n=393 row=contract/verify-process-exit/version-unreadable sha=16484ddb4345 v=holds judge=sonnet head=8834195b0683
at: src/cmd/verify.rs:199
quote: return Ok(Ended::Refused("version-unreadable"));
@row_add n=394 row=contract/verify-process-exit/strict-modes-failed area=conventions rec=fct-tg3zks37@2
boundary: viola verify process-exit detail
item: strict-modes-failed
promise: The code verify's process-exit carries on a strict-modes refusal.
@row_check n=394 row=contract/verify-process-exit/strict-modes-failed sha=1182e548fce0 v=holds judge=sonnet head=8834195b0683
at: src/cmd/verify.rs:288
quote: return Ok(Ended::Refused("strict-modes-failed"));
@row_add n=395 row=contract/verify-process-exit/unclean-fixture area=conventions rec=fct-tg3zks37@2
boundary: viola verify process-exit detail
item: unclean-fixture
promise: The code verify's process-exit carries for an unclean recorded fixture; the fixture's file enters no code.
@row_check n=395 row=contract/verify-process-exit/unclean-fixture sha=c81b2054595e v=holds judge=sonnet head=8834195b0683
at: src/cmd/verify.rs:629
quote: Ended::Refused("unclean-fixture")
@row_add n=396 row=contract/verify-process-exit/interrupted area=conventions rec=fct-tg3zks37@2
boundary: viola verify process-exit detail
item: interrupted
promise: The code verify's process-exit carries when stopped before its rows are checked; a failing row is no refusal and carries no code.
@row_check n=396 row=contract/verify-process-exit/interrupted sha=eed7ae4e648f v=holds judge=sonnet head=8834195b0683
at: src/cmd/verify.rs:87
quote: (1, Some("interrupted"))
@row_add n=397 row=contract/verify/no-json area=conventions rec=fct-tg3zks37@2
boundary: viola verify
item: --json
promise: viola verify takes no --json.
@row_check n=397 row=contract/verify/no-json sha=25e452e2d2b3 v=holds judge=sonnet head=8834195b0683
at: src/cmd/verify.rs:46
quote: pub(crate) struct VerifyArgs {
at: src/cmd/verify.rs:49
quote: record: Option<PathBuf>,
searched: json · in src/cmd/verify.rs struct VerifyArgs (lines 46-53: only record and program args)
searched: global = true · in src/cmd/mod.rs (only the --home arg, line 53)
@row_add n=398 row=contract/client-server-verify/server-verify-failed area=conventions rec=fct-7s455w3m@2
boundary: Client server verification
item: server-verify-failed
promise: A connected server that is not the recorded wrapper gets no frame; the verb exits 21 with the instance-unreachable document of detail server-verify-failed under --json, or the unable line and server-verify hint on stderr.
@row_check n=398 row=contract/client-server-verify/server-verify-failed sha=5ce5ff8208a9 v=holds judge=sonnet head=8834195b0683
at: src/cmd/client.rs:178
quote: return Err(CallError::ServerUnverified);
at: src/cmd/client.rs:183
quote: ServerVerdict::Mismatch | ServerVerdict::Unverifiable => Err(CallError::ServerUnverified),
at: src/cmd/client.rs:122
quote: Self::ServerUnverified => UnreachableCause::ServerVerify,
at: crates/viola-core/src/lib.rs:200
quote: Self::ServerVerify => "server-verify-failed",
@row_add n=399 row=contract/client-server-verify/check-order area=conventions rec=fct-7s455w3m@2
boundary: Client server verification
item: check order
promise: The client's own refusals, the strict-modes instance check and the socket-directory check come before any connect; the cause has no exit code or RefusalReason of its own.
@row_check n=399 row=contract/client-server-verify/check-order sha=bc6341d5cd84 v=holds judge=sonnet head=8834195b0683
at: src/cmd/send.rs:167
quote: if let Some(code) = client::refuse_untrusted(home, &args.name, args.json)? {
at: src/cmd/client.rs:173
quote: if check_endpoint_dir(&live.endpoint).is_err() {
at: src/cmd/client.rs:176
quote: let client = Client::connect(&live.endpoint, "cli")?;
@row_add n=400 row=contract/exit-21-human-hints/placement area=conventions rec=fct-jatimdcm@1
boundary: Exit 21 human hints
item: placement
promise: Each exit 21 human hint (unwrapped name, endpoint lost, socket directory) is the last line on stderr and is plain ASCII.
@row_check n=400 row=contract/exit-21-human-hints/placement sha=aa2fd2033f97 v=holds judge=sonnet head=8834195b0683
at: src/human/hints.rs:50
quote: "unwrapped-name" => format!(
at: src/human/hints.rs:53
quote: "endpoint-lost" => format!(
at: src/human/hints.rs:59
quote: "socket-dir" => "viola's socket directory must be a directory you own, readable only by \
at: src/human/mirror.rs:66
quote: text.push_str(&format!("hint: {hint}\n"));
@row_add n=401 row=contract/cli-exit-21/strict-modes-failed area=conventions rec=fct-u4uyh6oz@1
boundary: CLI exit 21
item: strict-modes-failed
promise: A refused strict-modes check of send, wait, last, answer, pause or release exits 21: human mode prints the unable: / hint: pair, --json exactly {"v":1,"error":"instance-unreachable","detail":"strict-modes-failed"} with stderr empty; nothing under the home is opened and no line is written.
@row_check n=401 row=contract/cli-exit-21/strict-modes-failed sha=54f8d3829139 v=holds judge=sonnet head=8834195b0683
at: src/cmd/client.rs:31
quote: if check_instance(&absolute, &absolute.join("instances").join(name)).is_ok() {
at: src/cmd/client.rs:35
quote: unreachable_document(UnreachableCause::StrictModes);
at: src/cmd/client.rs:233
quote: document(&json!({"v": 1, "error": "instance-unreachable", "detail": cause.as_str()}));
at: src/cmd/client.rs:40
quote: Ok(Some(21))
@row_add n=402 row=contract/cli-exit-21/socket-dir-refused area=conventions rec=fct-wm75dacy@1
boundary: CLI exit 21
item: socket-dir-refused
promise: On send, wait, last, answer, pause, release, link and unlink a refused socket directory exits 21 and writes one process-exit line at WARN with exit_code 21, detail socket-dir-refused and during connect, and no channel-request.
@row_check n=402 row=contract/cli-exit-21/socket-dir-refused sha=2db1c42f203d v=holds judge=sonnet head=8834195b0683
at: src/cmd/client.rs:173-174
quote: if check_endpoint_dir(&live.endpoint).is_err() { return Err(CallError::SocketDirRefused);
at: src/cmd/client.rs:195
quote: Err(CallError::SocketDirRefused) => return Ok(Err(Unreachable::SocketDir)),
at: src/cmd/client.rs:256-264
quote: obs_event!( WARN, ObsEvent::ProcessExit, subject = "self", exit_code = 21u8, detail = cause.detail(), during = cause.during(), duration_ms = obs::duration_ms(), );
at: src/cmd/unlink.rs:21
quote: Ok(client::connect(live)?.request("unlink", params)?)
@row_add n=403 row=contract/mcp-tool-results/refusal area=conventions rec=fct-4bh4742d@1
boundary: MCP tool results
item: refusal
promise: A refusal is a tool result with isError true and the channel's refusal and detail in structuredContent.
@row_check n=403 row=contract/mcp-tool-results/refusal sha=0ba60ea2ed22 v=holds judge=sonnet head=8834195b0683
at: crates/viola-mcp/src/result.rs:21
quote: failed(reason, json!({"refusal": reason, "detail": detail}))
at: crates/viola-mcp/src/result.rs:40
quote: let mut result = CallToolResult::structured_error(body);
@row_add n=404 row=contract/mcp-tool-results/instance-unreachable area=conventions rec=fct-4bh4742d@1
boundary: MCP tool results
item: instance-unreachable
promise: An unreachable instance is an isError true result whose structuredContent is {"error":"instance-unreachable","detail":cause}, as CLI --json, the cause one of strict-modes-failed, instance-dead, unwrapped-name, endpoint-lost, server-verify-failed, socket-dir-refused.
@row_check n=404 row=contract/mcp-tool-results/instance-unreachable sha=4717e114c470 v=holds judge=sonnet head=8834195b0683
at: crates/viola-mcp/src/result.rs:31
quote: error("instance-unreachable", Value::from(cause.as_str()))
at: crates/viola-core/src/lib.rs:196
quote: Self::StrictModes => "strict-modes-failed",
at: crates/viola-core/src/lib.rs:197
quote: Self::InstanceDead => "instance-dead",
at: crates/viola-core/src/lib.rs:198
quote: Self::UnwrappedName => "unwrapped-name",
at: crates/viola-core/src/lib.rs:199
quote: Self::EndpointLost => "endpoint-lost",
at: crates/viola-core/src/lib.rs:200
quote: Self::ServerVerify => "server-verify-failed",
at: crates/viola-core/src/lib.rs:201
quote: Self::SocketDir => "socket-dir-refused",
@row_add n=405 row=contract/mcp-tool-results/wrapper-fault area=conventions rec=fct-4bh4742d@1
boundary: MCP tool results
item: wrapper-fault
promise: A wrapper protocol fault is an isError true result {"error":"wrapper-fault","detail":…} carrying the sanitised JSON-RPC fault {code,message,data}: a fixed message, data a closed object or null.
@row_check n=405 row=contract/mcp-tool-results/wrapper-fault sha=bfe0e8f87a11 v=holds judge=sonnet head=8834195b0683
at: crates/viola-mcp/src/result.rs:59-61
quote: if let Some(fault) = frame.get("error") { return sanitised_fault(fault) .map(|detail| error("wrapper-fault", detail))
at: crates/viola-channel/src/lib.rs:141
quote: Some(json!({"code": code, "message": message, "data": data}))
@row_add n=406 row=contract/mcp-tool-results/invalid-params area=conventions rec=fct-4bh4742d@1
boundary: MCP tool results
item: invalid-params
promise: A tool argument failing re-validation is an isError true result {"error":"invalid-params","detail":"<field>"}, the field one of target, text, after, timeout_ms, dialog_id, response, with no frame written.
@row_check n=406 row=contract/mcp-tool-results/invalid-params sha=bba18bccb417 v=holds judge=sonnet head=8834195b0683
at: crates/viola-mcp/src/result.rs:36
quote: error("invalid-params", Value::from(field))
at: crates/viola-mcp/src/tools.rs:266
quote: .ok_or("target")
at: crates/viola-mcp/src/tools.rs:270
quote: args.get("text").and_then(Value::as_str).ok_or("text")
at: crates/viola-mcp/src/tools.rs:277
quote: Some(value) => value.as_u64().map(Some).ok_or(field),
at: crates/viola-mcp/src/tools.rs:284
quote: .ok_or("dialog_id")
at: crates/viola-mcp/src/tools.rs:291
quote: .ok_or("response")
@row_add n=407 row=contract/mcp-tool-results/is-error-text area=conventions rec=fct-4bh4742d@1
boundary: MCP tool results
item: isError text
promise: Every isError result's text is refused: <code>.
@row_check n=407 row=contract/mcp-tool-results/is-error-text sha=2b5685a53b75 v=holds judge=sonnet head=8834195b0683
at: crates/viola-mcp/src/result.rs:41
quote: result.content = vec![ContentBlock::text(format!("refused: {code}"))];
@row_add n=408 row=decision/mcp-tools-list-held-to-recorded-exchange area=conventions rec=fct-4bh4742d@1
name: MCP tools/list held to a recorded claude exchange
rule: The served tools/list result is held to a recorded claude exchange's result key and type shape, its tool names in order.
guard: mcp_recorded_exchange (viola-e2e)
@row_check n=408 row=decision/mcp-tools-list-held-to-recorded-exchange sha=0c4a5df06968 v=holds guard=true judge=sonnet head=8834195b0683
at: crates/viola-e2e/tests/mcp_recorded_exchange.rs:90-94
quote: fn shape_gaps(recorded: &Value, served: &Value) -> Vec<String> { let mut out = Vec::new(); missing(recorded, served, "", &mut out); out }
at: crates/viola-e2e/tests/mcp_recorded_exchange.rs:96
quote: fn tool_names(result: &Value) -> Vec<String> {
guard_at: crates/viola-e2e/tests/mcp_recorded_exchange.rs:180
@row_add n=409 row=contract/mcp-frame-tools/server-verify area=conventions rec=fct-gvna3tj2@2
boundary: MCP frame tools
item: server-verify
promise: A send, wait, last or answer whose connected server is not the recorded wrapper (peer euid not this user's or unreadable, pid or OS start time not the snapshot's, or uncheckable) returns the instance-unreachable isError result, text refused: instance-unreachable, no frame and no log line.
@row_check n=409 row=contract/mcp-frame-tools/server-verify sha=570c1c65c8a8 v=holds judge=sonnet head=8834195b0683
at: crates/viola-mcp/src/tools.rs:352
quote: if !peer_admitted(client.server_euid()) || !server_verified(live, client.server_pid()).await? {
at: crates/viola-mcp/src/tools.rs:353
quote: return Ok(result::unreachable(UnreachableCause::ServerVerify));
at: crates/viola-mcp/src/result.rs:41
quote: result.content = vec![ContentBlock::text(format!("refused: {code}"))];
@row_add n=410 row=contract/mcp-frame-tools/socket-dir-refused area=conventions rec=fct-qfrfkv27@1
boundary: MCP frame tools
item: socket-dir-refused
promise: A send, wait, last or answer meeting a refused socket directory returns the instance-unreachable isError result with detail socket-dir-refused, no frame written and no log line.
@row_check n=410 row=contract/mcp-frame-tools/socket-dir-refused sha=e88b4275db3b v=holds judge=sonnet head=8834195b0683
at: crates/viola-mcp/src/tools.rs:345-346
quote: if check_endpoint_dir(&live.endpoint).is_err() { return Ok(result::unreachable(UnreachableCause::SocketDir));
@row_add n=411 row=contract/mcp-frame-tools/strict-modes-failed area=conventions rec=fct-pfqpxroc@1
boundary: MCP frame tools
item: strict-modes-failed
promise: A send, wait, last or answer first checks strict modes on the home and the target instance; a refusal returns the instance-unreachable isError result with detail strict-modes-failed, text refused: instance-unreachable, no frame and no line.
@row_check n=411 row=contract/mcp-frame-tools/strict-modes-failed sha=dea97a3224b4 v=holds judge=sonnet head=8834195b0683
at: crates/viola-mcp/src/tools.rs:339-340
quote: if check_instance(&ctx.home, &instance_dir).is_err() { return Ok(result::unreachable(UnreachableCause::StrictModes));
@row_add n=412 row=decision/statusline-source-follows-home area=cross-cutting-patterns rec=fct-k6htnrfj@3
name: Statusline source follows the home
rule: The statusline source is the home's statusline-source.json whenever it exists; only the default home falls back to the user's Claude settings, so a home named by --home or reached through VIOLA_DIR never reads the user's settings.
@row_check n=412 row=decision/statusline-source-follows-home sha=d09f6b0398bb v=holds judge=sonnet head=8834195b0683
at: src/cmd/run.rs:533-535
quote: let own = home.join(STATUSLINE_SOURCE); if own.exists() { return Some(own);
at: src/cmd/run.rs:538-539
quote: let default = super::canonical_home(&super::default_home(user_home)); (default == home).then(|| {
@row_add n=413 row=decision/hook-exits-on-untrusted-instance area=cross-cutting-patterns rec=fct-k6htnrfj@3
name: Hook exits at once on an untrusted instance
rule: viola hook takes its instance only when the VIOLA_DIR instance directory exists and passes the strict-modes check; otherwise it exits 0 at once with no config read, no stdin read, no line, no frame and no SessionEnd append.
except: The two --capture arms, which read no VIOLA_* variable, run ahead of the check.
@row_check n=413 row=decision/hook-exits-on-untrusted-instance sha=78a705c93824 v=holds judge=sonnet head=8834195b0683
at: src/cmd/hook.rs:116-118
quote: let Some(instance) = instance.as_ref().and_then(canonical).filter(trusted) else { return Ok(ExitCode::SUCCESS); };
at: src/cmd/hook.rs:101
quote: if let Some(dir) = &args.capture {
@row_add n=414 row=contract/config-json/unknown-keys area=cross-cutting-patterns rec=fct-k6htnrfj@3
boundary: config.json
item: unknown keys
promise: config.json is parsed tolerantly: unknown keys are skipped and counted.
@row_check n=414 row=contract/config-json/unknown-keys sha=6114013d3556 v=holds judge=sonnet head=8834195b0683
at: src/obs.rs:208
quote: .filter(|k| !CONFIG_KEYS.contains(&k.as_str()))
at: src/obs.rs:213
quote: Some(ConfigRejection::UnknownKeys(unknown))
@row_add n=415 row=decision/diagnostics-level-sole-log-level area=cross-cutting-patterns rec=fct-k6htnrfj@3
name: diagnostics_level as the sole log level source
rule: The config key diagnostics_level is the only source of the process-log level; RUST_LOG has no effect.
@row_check n=415 row=decision/diagnostics-level-sole-log-level sha=f94d474a3bfb v=holds judge=sonnet head=8834195b0683
at: src/obs.rs:201
quote: let (level, bad_value) = match map.get("diagnostics_level").map(Value::as_str) {
at: src/obs.rs:202
quote: None | Some(Some("info")) => (Level::INFO, false),
at: src/obs.rs:203
quote: Some(Some("debug")) => (Level::DEBUG, false),
searched: RUST_LOG · in src/**/*.rs and crates/*/src/**/*.rs (only a comment in src/obs.rs:180 saying never RUST_LOG; no read)
@row_add n=416 row=decision/clap-without-color-feature area=cross-cutting-patterns rec=fct-k6htnrfj@3
name: clap built without its color feature
rule: clap is built without its color feature, so viola's own colour layer is the only reader of colour environment variables.
why: The feature's anstream and anstyle-query route read CLICOLOR, CLICOLOR_FORCE, NO_COLOR, TERM, COLORTERM and CI.
@row_check n=416 row=decision/clap-without-color-feature sha=f61ce80e4327 v=holds judge=sonnet head=8834195b0683
at: Cargo.toml:197-204
quote: clap = { version = "=4.6.7", default-features = false, features = [ "std", "derive", "help", "usage", "error-context", "suggestions", ] }
at: src/human/style.rs:95
quote: no_color: std::env::var_os("NO_COLOR").is_some(),
@row_add n=417 row=decision/agents-join-read-ungated area=cross-cutting-patterns rec=fct-wfge3rsz@3
name: Agents join read not gated on a stamp
rule: viola list and viola revive do not gate their claude agents --json read on a ledger stamp.
why: The read only enriches and its failure is never an error.
@row_check n=417 row=decision/agents-join-read-ungated sha=84141d7e6439 v=holds judge=sonnet head=8834195b0683
at: src/cmd/list.rs:124
quote: .and_then(|cwd| run::agents::read_here(home, &cwd).map_err(|_| ()))
at: src/cmd/revive.rs:192-193
quote: crate::run::agents::read_here(home, cwd) .is_ok_and(|list| list.rows.iter().any(|row| row.session_id == id))
@row_add n=418 row=decision/serde-json-unbounded-depth-banned area=cross-cutting-patterns rec=fct-qip7sr3o@1
name: serde_json unbounded depth banned
rule: The serde_json unbounded-depth feature is banned in the dependency policy.
guard: serde_json_unbounded_depth_is_banned_in_deny_toml
@row_check n=418 row=decision/serde-json-unbounded-depth-banned sha=c9a5bb6cec2d v=holds guard=true judge=sonnet head=8834195b0683
at: deny.toml:54-55
quote: crate = "serde_json" deny = ["unbounded_depth"]
guard_at: tests/contract_bounded_readers.rs:764
@row_add n=419 row=decision/sanitised-error-surfaces area=cross-cutting-patterns rec=fct-7ok4646z@1
name: Sanitised error surfaces
rule: CLI --json documents, MCP error results and channel error data carry codes and fixed messages only, no path, pid or upstream text; a peer's message, data, refusal or detail reaches them only as a closed value.
guard: contract_error_bodies: product_error_writers_are_each_named
except: A code word outside the closed list stays visible by design.
@row_check n=419 row=decision/sanitised-error-surfaces sha=d7ca1340a620 v=holds guard=true judge=sonnet head=8834195b0683
at: crates/viola-channel/src/lib.rs:86
quote: Self::Internal => "internal error",
at: crates/viola-channel/src/lib.rs:117
quote: pub fn sanitised_fault(error: &Value) -> Option<Value> {
guard_at: tests/contract_error_bodies.rs:771
@row_add n=420 row=decision/thiserror-messages-interpolate-nothing area=cross-cutting-patterns rec=fct-7ok4646z@1
name: thiserror messages interpolate nothing
rule: A thiserror error message interpolates no value.
guard: thiserror_messages_interpolate_nothing
@row_check n=420 row=decision/thiserror-messages-interpolate-nothing sha=049c7413fa1b v=holds guard=true judge=sonnet head=8834195b0683
at: crates/viola-state/src/lib.rs:23
quote: #[error("state file i/o failed")]
searched: #\[error\(.*\{ · in src/**/*.rs and crates/*/src/**/*.rs (no match)
guard_at: tests/contract_error_bodies.rs:794
@row_add n=421 row=contract/exit-21-human-hints/socket-dir area=cross-cutting-patterns rec=fct-7ok4646z@1
boundary: Exit 21 human hints
item: socket-dir
promise: The unable: / hint: pair for a socket directory names only the parsed instance name, no path, pid or uid, is plain ASCII with hint: last and no SGR.
@row_check n=421 row=contract/exit-21-human-hints/socket-dir sha=b6ae431b742b v=holds judge=sonnet head=8834195b0683
at: src/human.rs:37
quote: out.write_all(format!("unable: {unable}\nhint: {hint}\n").as_bytes())
at: src/cmd/run.rs:738
quote: &format!("the socket directory for {name} cannot be trusted"),
at: src/human/hints.rs:59
quote: "socket-dir" => "viola's socket directory must be a directory you own, readable only by \
@row_add n=422 row=decision/os-gated-tests-named area=cross-cutting-patterns rec=fct-xxlqtlbc@2
name: OS-gated tests named with their reason
rule: Every test held off a runner by an OS gate is named in the gates table with a non-empty reason, and the table names no gate that no longer stands.
guard: contract_os_gated_tests: os_gated_tests_are_each_named_with_their_reason
@row_check n=422 row=decision/os-gated-tests-named sha=291a55835fa7 v=holds guard=true judge=sonnet head=8834195b0683
at: tests/contract_os_gated_tests.rs:539-541
quote: fn os_gated_tests_are_each_named_with_their_reason() { let faults = faults(&census_files(&workspace()), GATES); assert!(faults.is_empty(), "{faults:#?}");
guard_at: tests/contract_os_gated_tests.rs:539
@row_add n=423 row=decision/no-other-os-test-gates area=cross-cutting-patterns rec=fct-xxlqtlbc@2
name: No test gate naming another OS
rule: A test gate whose predicate names windows or macOS is a fault even when the gates table names it.
guard: census_reads_a_gate_naming_another_os_as_a_fault_even_when_named
@row_check n=423 row=decision/no-other-os-test-gates sha=80b4301cef44 v=holds guard=true judge=sonnet head=8834195b0683
at: tests/contract_os_gated_tests.rs:533
quote: .map(|(file, test, gate)| format!("another OS: {file} {test} cfg({gate})")),
guard_at: tests/contract_os_gated_tests.rs:626
@row_add n=424 row=decision/child-cwd-explicit area=established-decisions rec=fct-ad44nn2e@4
name: Child cwd set explicitly
rule: The child's cwd is set explicitly to viola's current directory.
except: viola revive sets the child's cwd to the snapshot's recorded cwd, read after the strict-modes check, which must name an existing directory.
@row_check n=424 row=decision/child-cwd-explicit sha=238d63443aa3 v=holds judge=sonnet head=8834195b0683
at: src/cmd/run.rs:184
quote: spawn_dir: std::env::current_dir()?,
at: crates/viola-pty/src/lib.rs:136
quote: cmd.cwd(&spec.cwd);
at: src/cmd/revive.rs:224
quote: |(_, recovered)| recorded_dir(recovered).ok_or_else(|| refuse(Refusal::CwdMissing)),
@row_add n=425 row=decision/revive-program-resolution area=established-decisions rec=fct-ad44nn2e@4
name: Revive resolves the program from its own PATH
rule: Program resolution and the version gate never read a revive's recorded directory: the program is looked up on the reviving process's own PATH from its own current directory, and resolved to an absolute path by viola.
why: Ratified as a boundary widening.
@row_check n=425 row=decision/revive-program-resolution sha=d24e81b01d8b v=holds judge=sonnet head=8834195b0683
at: src/cmd/run.rs:267-268
quote: let cwd = std::env::current_dir()?; let program = match run::resolve_program(&launch.program, &cwd) {
at: src/run/mod.rs:143
quote: let path = std::env::var_os("PATH");
at: src/cmd/run.rs:290
quote: let gate = version_gate(&home, &program, &cwd, &strip);
@row_add n=426 row=contract/viola-revive-refusals/cwd-missing area=established-decisions rec=fct-ad44nn2e@4
boundary: viola revive refusals
item: cwd-missing
promise: A revive whose recorded cwd names no existing directory is refused cwd-missing.
@row_check n=426 row=contract/viola-revive-refusals/cwd-missing sha=b9d3069462f2 v=holds judge=sonnet head=8834195b0683
at: src/cmd/revive.rs:224
quote: |(_, recovered)| recorded_dir(recovered).ok_or_else(|| refuse(Refusal::CwdMissing)),
at: src/cmd/revive.rs:263
quote: .filter(|dir| dir.is_dir()),
@row_add n=427 row=contract/child-exit-record/exit-source area=established-decisions rec=fct-ad44nn2e@4
boundary: Child exit record
item: exit_source
promise: The child's exit is recorded with exit_source handle-wait or kill-fallback.
@row_check n=427 row=contract/child-exit-record/exit-source sha=893e46583972 v=holds judge=sonnet head=8834195b0683
at: crates/viola-pty/src/pump.rs:176
quote: source: ExitSource::HandleWait,
at: crates/viola-pty/src/pump.rs:221
quote: source: ExitSource::KillFallback,
@row_add n=428 row=decision/no-key-held-behind-resize area=established-decisions rec=fct-ad44nn2e@4
name: No human key held behind a resize
rule: viola never holds, queues or re-orders a human key behind a resize; a test that writes a key after a resize waits for the child to observe the new size first.
@row_check n=428 row=decision/no-key-held-behind-resize sha=46c87b538e08 v=holds judge=sonnet head=8834195b0683
searched: queue|buffer|hold|delay · in crates/viola-pty/src/pump.rs lines 150-194 (the human input is copied by its own worker `spawn_worker(tx, None, move || copy(input, human))`; the resize is a separate periodic `pty.resize(size)?` and gates no input)
searched: resize · in src/cmd/run.rs, src/run/*.rs (the host size only feeds the screen model, src/cmd/run.rs:681-687; nothing holds keys)
@row_add n=429 row=decision/passthrough-never-waits-on-model area=established-decisions rec=fct-gollksbj@1
name: Passthrough never waits on the screen model
rule: The human's bytes are written first and the screen model gets a copy through a bounded queue it may drop from, so the passthrough never waits on the model.
@row_check n=429 row=decision/passthrough-never-waits-on-model sha=95741298fe6a v=holds judge=sonnet head=8834195b0683
at: src/run/gate.rs:134-135
quote: let written = self.output.write(bytes)?; self.feed.offer(Feed::Bytes(bytes[..written].to_vec()));
at: src/run/gate.rs:109
quote: if let Err(TrySendError::Full(_)) = self.tx.try_send(message) {
at: src/run/gate.rs:190
quote: let (tx, rx) = mpsc::sync_channel(FEED_CAPACITY);
@row_add n=430 row=decision/poisoned-screen-model area=established-decisions rec=fct-gollksbj@1
name: Poisoned screen model reads not ready
rule: A caught vt100 panic or a dropped feed message poisons the screen model until the host size changes, and a poisoned model reads input-not-ready.
why: A dropped byte desynchronises the screen.
@row_check n=430 row=decision/poisoned-screen-model sha=b3ce08a2fde8 v=holds judge=sonnet head=8834195b0683
at: src/run/gate.rs:46-48
quote: if catch_unwind(AssertUnwindSafe(|| step(&mut self.screen))).is_err() { self.screen.poison(); self.panicked = true;
at: src/run/gate.rs:69
quote: self.screen.poison();
at: src/run/gate.rs:57-60
quote: self.panicked = false; self.overflowed = false; self.seen = offered_at; self.step(|s| s.resize(size.rows, size.cols, now));
at: crates/viola-agent-claude/src/screen.rs:139-141
quote: let Some(parser) = &self.parser else { return GateStep::Done(Readiness::InputNotReady); };
@row_add n=431 row=contract/parse-rejected-event/vt100-feed area=established-decisions rec=fct-gollksbj@1
boundary: parse-rejected event
item: vt100-feed
promise: One parse-rejected line with parser vt100-feed per poisoning episode, its detail panicked for a caught panic and oversize for a dropped message.
@row_check n=431 row=contract/parse-rejected-event/vt100-feed sha=ed29eb6ca878 v=holds judge=sonnet head=8834195b0683
at: src/run/gate.rs:81
quote: parser = "vt100-feed",
at: src/run/gate.rs:49
quote: parse_rejected("panicked");
at: src/run/gate.rs:72
quote: parse_rejected("oversize");
@row_add n=432 row=decision/gate-timings-compiled area=established-decisions rec=fct-gollksbj@1
name: Readiness gate timings compiled
rule: The quiet period (300 ms) and the maximum wait (8.5 s) are compiled built-ins, validated per CLI version by the stamped quiet-period ledger row.
why: The maximum wait covers the CLI's 8.0 s paste hint.
@row_check n=432 row=decision/gate-timings-compiled sha=e7be16d24041 v=holds judge=sonnet head=8834195b0683
at: crates/viola-agent-claude/src/screen.rs:10
quote: pub const QUIET_PERIOD: Duration = Duration::from_millis(300);
at: crates/viola-agent-claude/src/screen.rs:16
quote: pub const GATE_MAX_WAIT: Duration = Duration::from_millis(8500);
at: crates/viola-agent-claude/src/ledger.rs:999-1000
quote: within(typed.ready_settle_ms, GATE_MAX_WAIT) && within(typed.turn_settle_ms, GATE_MAX_WAIT)
@row_add n=433 row=decision/screen-signatures-compiled area=established-decisions rec=fct-gollksbj@1
name: Screen signatures as compiled literals
rule: Modal and input-box signatures are compiled literals validated by each CLI version's stamp; a screen is ready when a row holds an input-box literal and no row holds a modal literal.
why: The input-box literal (shift+tab to cycle) is mode-independent and held by no other CLI text read.
@row_check n=433 row=decision/screen-signatures-compiled sha=cd55d32c0619 v=holds judge=sonnet head=8834195b0683
at: crates/viola-agent-claude/src/screen.rs:41-42
quote: input_box: &["for agents", "(shift+tab to cycle)"], modals: &["Yes, I trust this folder", "Yes, allow external imports"],
at: crates/viola-agent-claude/src/screen.rs:153-156
quote: if sigs.modals.iter().any(|m| row.contains(m)) { return GateStep::Done(Readiness::InputNotReady); } input_box = input_box || sigs.input_box.iter().any(|s| row.contains(s));
@row_add n=434 row=decision/partial-gate-on-unverified-cli area=established-decisions rec=fct-gollksbj@1
name: Partial readiness gate on an unverified CLI
rule: On a CLI version with no passing stamp the gate still refuses a poisoned model and a screen not quiet within the maximum wait, and a quiet screen is ready with no row read, leaving the rest to delivery confirmation.
@row_check n=434 row=decision/partial-gate-on-unverified-cli sha=c431fe00f742 v=holds judge=sonnet head=8834195b0683
at: crates/viola-agent-claude/src/screen.rs:139-141
quote: let Some(parser) = &self.parser else { return GateStep::Done(Readiness::InputNotReady); };
at: crates/viola-agent-claude/src/screen.rs:148-150
quote: let Some(sigs) = sigs else { return GateStep::Done(Readiness::Ready); };
at: src/cmd/run.rs:675
quote: let sigs = cli_verified.then_some(&viola_agent_claude::screen::SIGNATURES);
@row_add n=435 row=contract/send-results/input-not-ready area=established-decisions rec=fct-gollksbj@1
boundary: send results
item: input-not-ready
promise: A screen not quiet within the maximum wait, a poisoned model, a quiet screen still without an input-box row at the bound, or a modal on a verified CLI makes send type nothing and report not-delivered with detail input-not-ready.
@row_check n=435 row=contract/send-results/input-not-ready sha=d41ecd64b8d1 v=holds judge=sonnet head=8834195b0683
at: crates/viola-agent-claude/src/screen.rs:140
quote: return GateStep::Done(Readiness::InputNotReady);
at: crates/viola-agent-claude/src/screen.rs:154
quote: return GateStep::Done(Readiness::InputNotReady);
at: crates/viola-agent-claude/src/screen.rs:163
quote: GateStep::Done(Readiness::InputNotReady)
at: src/run/send.rs:130
quote: return ctx.not_delivered(None, NotDelivered::InputNotReady);
@row_add n=436 row=decision/screen-rows-read-only-by-verify area=established-decisions rec=fct-gollksbj@1
name: Screen row text read only by verify
rule: The screen model has one row-text exit, and only viola verify reads it.
@row_check n=436 row=decision/screen-rows-read-only-by-verify sha=3c059f145303 v=holds judge=sonnet head=8834195b0683
at: crates/viola-agent-claude/src/screen.rs:123
quote: pub fn rows(&self) -> Option<Vec<String>> {
at: src/cmd/verify/typed/pty.rs:144
quote: let rows = feed.screen.rows();
at: src/cmd/verify/typed/pty.rs:177
quote: let rows = feed.screen.rows()?;
searched: .rows() · in src/**/*.rs, crates/*/src/**/*.rs (non-test callers only under src/cmd/verify/)
@row_add n=437 row=decision/no-typing-into-trust-dialogs area=established-decisions rec=fct-gollksbj@1
name: viola types into no trust dialog
rule: viola types into neither the folder trust dialog nor the external CLAUDE.md imports dialog.
@row_check n=437 row=decision/no-typing-into-trust-dialogs sha=732dd834547d v=holds judge=sonnet head=8834195b0683
searched: I trust this folder · in src/**/*.rs, crates/*/src/**/*.rs (only the modal signature, the refusal verdict and tests; nothing types a reply)
searched: allow external imports · in src/**/*.rs, crates/*/src/**/*.rs (same)
searched: CLAUDE.md · in src/**/*.rs, crates/*/src/**/*.rs (only comments, signature and tests)
@row_add n=438 row=decision/every-send-confirmed area=established-decisions rec=fct-va43dnf2@2
name: Every send is confirmed
rule: A send not matched by a prompt-submitted within the confirmation window is reported not-delivered with detail no-prompt-submitted.
why: A turn boundary per the hooks does not prove the input box is ready; confirmation is the backstop behind the screen gate.
@row_check n=438 row=decision/every-send-confirmed sha=cf160e2a66c9 v=holds judge=sonnet head=8834195b0683
at: src/run/send.rs:154-157
quote: match slot.confirm(slot.clock.now()) { Some(submitted_at) => ctx.confirm(cursor, &submitted_at, slot.clock.now()), None => ctx.not_delivered(Some(cursor), NotDelivered::NoPromptSubmitted), }
at: crates/viola-core/src/lib.rs:222
quote: Self::NoPromptSubmitted => "no-prompt-submitted",
@row_add n=439 row=decision/confirm-window-compiled area=established-decisions rec=fct-va43dnf2@2
name: Confirmation window compiled
rule: The confirmation window is a compiled 10 s, never in config.json, validated per CLI version by the confirm-window ledger row; run reads no window number from the stamp.
except: A /compact send waiting for its post-condition has its own 40 s window.
@row_check n=439 row=decision/confirm-window-compiled sha=65aaf763bda2 v=holds judge=sonnet head=8834195b0683
at: crates/viola-agent-claude/src/screen.rs:20
quote: pub const CONFIRM_WINDOW_FALLBACK: Duration = Duration::from_secs(10);
at: crates/viola-agent-claude/src/ledger.rs:1002
quote: LedgerRow::ConfirmWindow => within(typed.prompt_latency_ms, CONFIRM_WINDOW_FALLBACK),
at: src/run/send/slot.rs:177-178
quote: Some(Confirms::Post(PostCondition::CompactedSession)) => COMPACT_CONFIRM_WINDOW, _ => CONFIRM_WINDOW_FALLBACK,
at: src/obs.rs:32-39
quote: const CONFIG_KEYS: [&str; 6] = [ "v", "diagnostics_level", "claude_env_keep", "five_hour", "seven_day", "dialog_hold_s", ];
@row_add n=440 row=decision/compact-confirm-window area=established-decisions rec=fct-va43dnf2@2
name: /compact confirmation window
rule: A /compact send waits a compiled 40 s window for its post-condition, validated per CLI version by the local-command-compact row against the measured compact latency.
state: provisional
@row_check n=440 row=decision/compact-confirm-window sha=77f4c2c859dd v=holds judge=sonnet head=8834195b0683
at: crates/viola-agent-claude/src/screen.rs:26
quote: pub const COMPACT_CONFIRM_WINDOW: Duration = Duration::from_secs(40);
at: src/run/send/slot.rs:177
quote: Some(Confirms::Post(PostCondition::CompactedSession)) => COMPACT_CONFIRM_WINDOW,
at: crates/viola-agent-claude/src/ledger.rs:848
quote: && within(typed.compact_latency_ms, COMPACT_CONFIRM_WINDOW)
@row_add n=441 row=decision/send-time-bound area=established-decisions rec=fct-va43dnf2@2
name: Send time bound
rule: send blocks at most the readiness gate's maximum wait plus the paste plus the window, and on the MCP surface that total stays below the MCP client's tool-call timeout.
@row_check n=441 row=decision/send-time-bound sha=8e4b54415997 v=holds judge=sonnet head=8834195b0683
at: src/run/send.rs:128
quote: let (readiness, _) = gate.wait_ready(slot.clock.as_ref(), slot.clock.now());
at: src/run/send.rs:154
quote: match slot.confirm(slot.clock.now()) {
at: crates/viola-agent-claude/src/screen.rs:16
quote: pub const GATE_MAX_WAIT: Duration = Duration::from_millis(8500);
at: crates/viola-mcp/src/tools.rs:42
quote: pub const WAIT_CAP: u64 = 50_000;
@row_add n=442 row=decision/exact-prompt-match area=established-decisions rec=fct-va43dnf2@2
name: Exact prompt match after agent normalisation
rule: The typed text is matched exactly against prompt-submitted's text, whatever its origin, after the agent crate removes the long-paste wrapper and its frame and reverses tag escaping, so the wrapper-side matcher never sees a Claude-specific shape.
@row_check n=442 row=decision/exact-prompt-match sha=a57fb0755514 v=holds judge=sonnet head=8834195b0683
at: crates/viola-agent-claude/src/hook.rs:204
quote: unescape_tags(&unwrap_pastes(raw))
at: src/run/send/slot.rs:91
quote: && f.text == text =>
@row_add n=443 row=decision/typed-text-normalisation area=established-decisions rec=fct-va43dnf2@2
name: Typed text normalisation
rule: send types its text with every CR LF pair and every other CR as one LF and without trailing CR and LF characters, nothing else changed, and uses that typed text for classification, matching, text_bytes and the paste.
why: The CLI drops a pasted text's last newline and submits a CR as LF, so a text typed with them failed the exact match.
@row_check n=443 row=decision/typed-text-normalisation sha=5781c7fc7e42 v=holds judge=sonnet head=8834195b0683
at: crates/viola-agent-claude/src/hook.rs:232
quote: let text = text.trim_end_matches(['\r', '\n']);
at: crates/viola-agent-claude/src/hook.rs:234
quote: Cow::Owned(text.replace("\r\n", "\n").replace('\r', "\n"))
at: src/run/send.rs:111
quote: let confirms = classify(text, slot.cli_verified);
at: src/run/send.rs:147
quote: let cursor = ctx.issue(text.len())?;
at: src/run/send.rs:148
quote: if paste(text).is_err() {
@row_add n=444 row=contract/send-refusals/empty-text area=established-decisions rec=fct-va43dnf2@2
boundary: send refusals
item: empty-text
promise: A send whose typed text is empty is refused not-delivered / empty-text (exit 13) and never issued: nothing reserved, typed or logged as a send, and no wheel read.
@row_check n=444 row=contract/send-refusals/empty-text sha=86b33025b0b7 v=holds judge=sonnet head=8834195b0683
at: src/run/send.rs:93-94
quote: if text.is_empty() { return ctx.not_delivered(None, NotDelivered::EmptyText);
at: src/cmd/send.rs:191-192
quote: if let Some(detail) = refusal_of(text) { refused_client_side(detail);
@row_add n=445 row=contract/events-ndjson/send-confirmed-order area=established-decisions rec=fct-va43dnf2@2
boundary: events.ndjson
item: send-confirmed order
promise: The line that confirms a send (the driver prompt-submitted, or the session-start for /clear and /compact) always precedes send-confirmed, whose submitted_at is that line's ts.
@row_check n=445 row=contract/events-ndjson/send-confirmed-order sha=a7883827e79a v=holds judge=sonnet head=8834195b0683
at: src/run/send/slot.rs:228-230
quote: let appended = feed.appending(&line, || append_event(instance_dir, &line));     if claimed {         slot.settle(appended.is_ok().then(|| line.ts.clone()));
@row_add n=446 row=decision/local-commands-by-ledger area=established-decisions rec=fct-va43dnf2@2
name: Local commands classified by the ledger
rule: A send is a local command only by exact equality of its typed text with the ledger's local-command list, never by a leading slash; any other slash text, skills included, is an ordinary send.
why: Local commands fire no UserPromptSubmit and would otherwise be false negatives; /clear matters most as drivers clear between skills.
@row_check n=446 row=decision/local-commands-by-ledger sha=ac4ca649b51f v=holds judge=sonnet head=8834195b0683
at: src/run/send/classify.rs:28
quote: match LOCAL_COMMANDS.iter().find(|(command, _)| *command == text) {
at: src/run/send/classify.rs:29
quote: None => Confirms::Prompt,
@row_add n=447 row=contract/send-results/unconfirmable area=established-decisions rec=fct-va43dnf2@2
boundary: send results
item: unconfirmable
promise: A listed local command whose post-condition is none, or any listed command on an unverified CLI, is typed and answered ok with {"confirmed":false,"detail":"unconfirmable"} at once; nothing else is ever unconfirmable.
@row_check n=447 row=contract/send-results/unconfirmable sha=aa900508594d v=holds judge=sonnet head=8834195b0683
at: src/run/send/classify.rs:30-31
quote: Some((_, Some(post))) if cli_verified => Confirms::Post(*post), Some(_) => Confirms::Nothing,
at: src/run/send.rs:224
quote: Ok(json!({"ok": {"confirmed": false, "detail": "unconfirmable", "cursor": cursor}}))
@row_add n=448 row=decision/clear-confirmed-by-new-session area=established-decisions rec=fct-va43dnf2@2
name: /clear confirmed by a new session
rule: /clear on a verified CLI is confirmed by a session-start of cause clear whose agent session id differs from the remembered one, and by no prompt-submitted.
@row_check n=448 row=decision/clear-confirmed-by-new-session sha=a7948913e655 v=holds judge=sonnet head=8834195b0683
at: src/run/send/slot.rs:111
quote: Some("clear") if prior.as_deref() != Some(id) => Some(PostCondition::NewSession),
at: src/run/send/slot.rs:90
quote: if matches!((f.confirms, &f.state), (Confirms::Prompt, Match::Waiting))
@row_add n=449 row=decision/compact-confirmed-by-same-session area=established-decisions rec=fct-va43dnf2@2
name: /compact confirmed by the same session
rule: /compact on a verified CLI is confirmed by a session-start of cause compact whose agent session id equals the remembered one, and by no prompt-submitted.
@row_check n=449 row=decision/compact-confirmed-by-same-session sha=936c5ce8cd6a v=holds judge=sonnet head=8834195b0683
at: src/run/send/slot.rs:112
quote: Some("compact") if prior.as_deref() == Some(id) => {
at: src/run/send/slot.rs:90
quote: if matches!((f.confirms, &f.state), (Confirms::Prompt, Match::Waiting))
@row_add n=450 row=decision/append-heals-torn-line area=established-decisions rec=fct-ak4xjyyx@3
name: The next append heals a torn line
rule: Each event line is written in one write call; the next append heals a torn last line, and readers count it and never rewrite the log.
@row_check n=450 row=decision/append-heals-torn-line sha=c9a522d00703 v=holds judge=sonnet head=8834195b0683
at: crates/viola-state/src/events.rs:335
quote: file.write_all(&[b"\n", bytes].concat())?;
at: crates/viola-state/src/events.rs:245
quote: self.skipped.torn_lines += 1;
@row_add n=451 row=decision/reader-counts-skipped-lines area=established-decisions rec=fct-ak4xjyyx@3
name: Event reader counts what it skips
rule: The event reader counts unknown kinds, unknown fields and torn lines per read and does not log the counts.
@row_check n=451 row=decision/reader-counts-skipped-lines sha=25bd3ec4b7d6 v=holds judge=sonnet head=8834195b0683
at: crates/viola-state/src/events.rs:262
quote: self.skipped.unknown_kinds += 1;
at: crates/viola-state/src/events.rs:266
quote: self.skipped.unknown_fields += 1;
at: crates/viola-state/src/events.rs:245
quote: self.skipped.torn_lines += 1;
@row_add n=452 row=contract/list-json/skipped area=established-decisions rec=fct-ak4xjyyx@3
boundary: viola list --json
item: skipped
promise: skipped is {unknown_kinds, unknown_fields, torn_lines}, each summed over every scanned instance, gone ones included.
@row_check n=452 row=contract/list-json/skipped sha=0b4b566c0fd6 v=holds judge=sonnet head=8834195b0683
at: crates/viola-state/src/board.rs:283
quote: board.skipped.add(log_pass(dir));
at: crates/viola-state/src/board.rs:278
quote: Liveness::Gone => None,
@row_add n=453 row=decision/wheel-in-wrapper-memory area=established-decisions rec=fct-ak4xjyyx@3
name: Wheel decided in wrapper memory
rule: The wheel is decided in the wrapper's memory and mirrored to a snapshot.
@row_check n=453 row=decision/wheel-in-wrapper-memory sha=8bc4b9c9a7e6 v=holds judge=sonnet head=8834195b0683
at: src/run/wheel.rs:83
quote: held: Mutex<Held>,
at: src/run/wheel.rs:293
quote: let written = self.snapshots.update(|s| s.wheel = step.to).is_ok();
@row_add n=454 row=decision/budget-readings-last-writer-wins area=established-decisions rec=fct-ak4xjyyx@3
name: Budget readings last-writer-wins
rule: Budget readings are timestamped and last-writer-wins under File::lock.
@row_check n=454 row=decision/budget-readings-last-writer-wins sha=4e318803540f v=holds judge=sonnet head=8834195b0683
at: crates/viola-state/src/budget.rs:56
quote: read_at: crate::timestamp(read_at),
at: crates/viola-state/src/budget.rs:60-61
quote: lock.lock()?; replace_private(&home.join(BUDGET), &bytes, FILE_MODE)
@row_add n=455 row=decision/sqlite-index-upgrade-path area=established-decisions rec=fct-ak4xjyyx@3
name: SQLite index as upgrade path
rule: If a real query need appears, a derived, rebuildable SQLite index sits beside the authoritative log, which stays the source of truth.
@row_check n=455 row=decision/sqlite-index-upgrade-path sha=a0d68aba71d2 v=unverifiable judge=sonnet head=8834195b0683
searched: sqlite · in Cargo.toml, crates/*/Cargo.toml, src/**/*.rs, crates/*/src/**/*.rs
searched: rusqlite · in Cargo.toml, crates/*/Cargo.toml, src/**/*.rs, crates/*/src/**/*.rs
why: not a code claim
@row_add n=456 row=absence/daemon area=established-decisions rec=fct-ak4xjyyx@3,fct-wfmyj5kz@2
capability: A daemon
@row_check n=456 row=absence/daemon sha=e69c4c16bfac v=holds judge=sonnet head=8834195b0683
searched: daemon · in Cargo.toml, crates/*/Cargo.toml, src/**/*.rs, crates/*/src/**/*.rs
searched: Daemon · in Cargo.toml, crates/*/Cargo.toml, src/**/*.rs, crates/*/src/**/*.rs
@row_add n=457 row=decision/pinned-copy-no-clobber-placement area=established-decisions rec=fct-kfz3cseb@2,fct-wbroquss@1,fct-fwlvhl64@1
name: Pinned copy placed no-clobber with the handle closed
rule: The pinned copy's absent-copy write is never a replace: a no-clobber placement that closes the write handle before the rename and leaves an existing copy untouched.
why: A rename with the write handle still open makes a concurrent first start's exec of the copy fail ETXTBSY.
@row_check n=457 row=decision/pinned-copy-no-clobber-placement sha=12c4bc3250dc v=holds judge=sonnet head=8834195b0683
at: crates/viola-state/src/fs.rs:117-118
quote: tmp.as_file().sync_all()?; match tmp.into_temp_path().persist_noclobber(path) {
at: crates/viola-state/src/fs.rs:123
quote: Err(e) if e.error.kind() == io::ErrorKind::AlreadyExists => Ok(Placed::Present),
at: crates/viola-state/src/pin.rs:105
quote: match place_private_new(path, bytes, DIR_MODE)? {
@row_add n=458 row=decision/interprocess-local-socket-ipc area=established-decisions rec=fct-wfmyj5kz@2
name: interprocess local sockets behind the channel seam
rule: IPC is interprocess 2.4.4 local_socket, one endpoint per viola run, used only behind viola's own channel seam.
why: One API over Unix sockets, sync and Tokio flavours, and no daemon; the one-maintainer risk is accepted and contained by the seam.
@row_check n=458 row=decision/interprocess-local-socket-ipc sha=6a7cda2df170 v=holds judge=sonnet head=8834195b0683
at: Cargo.toml:207
quote: interprocess = "=2.4.4"
at: crates/viola-channel/src/server.rs:12
quote: use interprocess::local_socket::{
at: crates/viola-channel/src/endpoint.rs:27
quote: pub fn endpoint_name(name: &ViolaName, home: &Path) -> Result<String, ChannelError> {
@row_add n=459 row=decision/wait-blocks-nothing-polls area=established-decisions rec=fct-wfmyj5kz@2
name: wait blocks, nothing polls
rule: wait blocks until the next driver-relevant event and returns it, so nothing polls and a driver parked in wait learns that a dialog is pending.
@row_check n=459 row=decision/wait-blocks-nothing-polls sha=27d2dc000ca2 v=holds judge=sonnet head=8834195b0683
at: src/run/wait.rs:155
quote: return Ok(json!({"ok": {"event": line.value, "cursor": line.end}}));
at: src/run/wait.rs:158
quote: while feed.generation == seen {
at: src/run/wait.rs:201
quote: if EventKind::WAIT_WAKE
@row_add n=460 row=contract/channel-wait/driver-relevant-events area=established-decisions rec=fct-wfmyj5kz@2
boundary: channel wait
item: driver-relevant events
promise: wait returns on turn-ended, question, permission, plan, session-end or dialog-expired.
@row_check n=460 row=contract/channel-wait/driver-relevant-events sha=27b70fd3e4b0 v=holds judge=sonnet head=8834195b0683
at: crates/viola-core/src/lib.rs:99-106
quote: pub const WAIT_WAKE: [Self; 6] = [ Self::TurnEnded, Self::Question, Self::Permission, Self::Plan, Self::SessionEnd, Self::DialogExpired, ];
at: src/run/wait.rs:201-203
quote: if EventKind::WAIT_WAKE .iter() .any(|k| Some(k.as_str()) == kind)
@row_add n=461 row=contract/channel-wait/after area=established-decisions rec=fct-wfmyj5kz@2
boundary: channel wait
item: after
promise: after is a byte offset into events.ndjson; wait returns the first driver-relevant event whose line starts at or after it, at once if already logged; without after, the next such event appended after the call.
@row_check n=461 row=contract/channel-wait/after sha=49ff943d0ee6 v=holds judge=sonnet head=8834195b0683
at: src/run/wait.rs:140
quote: Some(after) => after,
at: src/run/wait.rs:145
quote: None => end_offset(instance_dir).map_err(|_| ProtocolError::Internal)?,
at: src/run/wait.rs:153
quote: if let Some(line) = scan(instance_dir, &mut from)? {
@row_add n=462 row=contract/channel-wait/timeout-ms area=established-decisions rec=fct-wfmyj5kz@2
boundary: channel wait
item: timeout_ms
promise: On timeout wait returns ok with {"timed_out":true}; with no timeout_ms it blocks until an event, and an endpoint vanishing first gives instance-unreachable with detail endpoint-lost.
@row_check n=462 row=contract/channel-wait/timeout-ms sha=ffac1a33f6af v=holds judge=sonnet head=8834195b0683
at: src/run/wait.rs:161
quote: return Ok(json!({"ok": {"timed_out": true}}));
at: src/run/wait.rs:149
quote: let deadline = timeout_ms.and_then(|ms| deadline(self.clock.now(), ms));
at: src/cmd/client.rs:121
quote: Self::Lost => UnreachableCause::EndpointLost,
@row_add n=463 row=contract/channel-wait/pending-dialog area=established-decisions rec=fct-wfmyj5kz@2
boundary: channel wait
item: pending dialog
promise: A dialog pending when wait is called without after, or with after at or before its event line, is returned at once; with after past that line wait blocks for the next event.
@row_check n=463 row=contract/channel-wait/pending-dialog sha=02bd1bb405d6 v=holds judge=sonnet head=8834195b0683
at: src/run/wait.rs:143-145
quote: match feed.pending_start { Some(start) => start, None => end_offset(instance_dir).map_err(|_| ProtocolError::Internal)?,
@row_add n=464 row=contract/channel-results/cursor area=established-decisions rec=fct-wfmyj5kz@2
boundary: channel results
item: cursor
promise: Every send ok payload and every wait event result carries cursor, the offset to pass as the next after.
@row_check n=464 row=contract/channel-results/cursor sha=942b5ce8bdb6 v=holds judge=sonnet head=8834195b0683
at: src/run/send.rs:215
quote: Ok(json!({"ok": {"submitted_at": submitted_at, "cursor": cursor}}))
at: src/run/send.rs:224
quote: Ok(json!({"ok": {"confirmed": false, "detail": "unconfirmable", "cursor": cursor}}))
at: src/run/wait.rs:155
quote: return Ok(json!({"ok": {"event": line.value, "cursor": line.end}}));
@row_add n=465 row=contract/dialog-events/dialog-id area=established-decisions rec=fct-wfmyj5kz@2
boundary: dialog events
item: dialog_id
promise: Each dialog event carries a wrapper-assigned integer dialog_id, monotonic per instance and never repeated across wrapper restarts; answer names its target by it.
@row_check n=465 row=contract/dialog-events/dialog-id sha=e22e7cb2ee7e v=holds judge=sonnet head=8834195b0683
at: src/run/dialog.rs:236
quote: self.state().next_id = highest.map_or(1, |h| h.saturating_add(1));
at: src/run/dialog.rs:286
quote: let dialog_id = state.next_id;
at: src/run/dialog.rs:442
quote: .filter(|p| p.dialog_id == dialog_id && p.answer.is_none())
@row_add n=466 row=decision/one-pending-dialog area=established-decisions rec=fct-wfmyj5kz@2
name: At most one pending dialog per instance
rule: At most one dialog is pending per instance; a dialog hook arriving while another is pending is answered null at once so it renders for the human, and its event is still logged with its own dialog_id.
@row_check n=466 row=decision/one-pending-dialog sha=fb0f3164ef5b v=holds judge=sonnet head=8834195b0683
at: src/run/dialog.rs:252
quote: && state.pending.is_none()
at: src/run/dialog.rs:272
quote: return Ok(reply(dialog_id, None));
at: src/run/dialog.rs:255
quote: let dialog_id = self.register(&mut state, &raise, hold)?;
@row_add n=467 row=contract/channel-answer/unknown-dialog area=established-decisions rec=fct-wfmyj5kz@2
boundary: channel answer
item: unknown-dialog
promise: An answer for a dialog that was not held pending is refused not-delivered with detail unknown-dialog.
@row_check n=467 row=contract/channel-answer/unknown-dialog sha=7a4487a62b0e v=holds judge=sonnet head=8834195b0683
at: src/run/dialog.rs:442-446
quote: .filter(|p| p.dialog_id == dialog_id && p.answer.is_none()) else { return Ok(refusal( RefusalReason::NotDelivered, Some(NotDelivered::UnknownDialog.as_str()),
@row_add n=468 row=contract/channel-last/last-assistant-message area=established-decisions rec=fct-wfmyj5kz@2
boundary: channel last
item: last_assistant_message
promise: last returns the last_assistant_message of the newest turn-ended event, rebuilt from events.ndjson on start.
@row_check n=468 row=contract/channel-last/last-assistant-message sha=89cf7b4a3288 v=holds judge=sonnet head=8834195b0683
at: src/run/wait.rs:177
quote: Ok(json!({"ok": {"last_assistant_message": message, "ts": ts}}))
at: src/run/wait.rs:117
quote: if line.value["kind"] == EventKind::TurnEnded.as_str() {
@row_add n=469 row=decision/hashed-endpoint-names area=established-decisions rec=fct-wfmyj5kz@2
name: Hashed endpoint names
rule: Endpoint names are a short hash of the instance, never the raw instance name and path.
@row_check n=469 row=decision/hashed-endpoint-names sha=4ef502fc19ba v=holds judge=sonnet head=8834195b0683
at: crates/viola-channel/src/endpoint.rs:30
quote: let hash = fnv1a(name.bytes().chain([0]).chain(home.bytes()));
at: crates/viola-channel/src/endpoint.rs:32
quote: Ok(format!("viola-{}", &hex[..12]))
@row_add n=470 row=decision/socket-dir-check-at-channel-callers area=established-decisions rec=fct-lbhmyin2@1
name: Socket-directory check at the channel's callers
rule: The socket-directory check stands at the channel seam's callers, not inside the channel client's connect.
why: The channel crate has no state-crate edge, and a test writes through the client past every check by design.
@row_check n=470 row=decision/socket-dir-check-at-channel-callers sha=43b43e0a97c9 v=holds judge=sonnet head=8834195b0683
at: crates/viola-channel/src/client.rs:33
quote: Self::connect_by(endpoint, process, busy_deadline(Instant::now()))
at: src/cmd/client.rs:173
quote: if check_endpoint_dir(&live.endpoint).is_err() {
at: src/cmd/hook.rs:231
quote: if check_endpoint_dir(endpoint).is_err() {
at: crates/viola-mcp/src/tools.rs:345
quote: if check_endpoint_dir(&live.endpoint).is_err() {
@row_add n=471 row=decision/verify-server-before-first-frame area=established-decisions rec=fct-rrvv5rrr@2,fct-y7p4rnnj@2
name: Server verified before the first frame
rule: Every frame-writing client compares the connected peer's pid and OS start time with the snapshot's before its first frame, and no server verdict reads as matching without that comparison.
why: A squatted or rogue server is never sent a frame.
@row_check n=471 row=decision/verify-server-before-first-frame sha=591a1810b0f5 v=holds judge=sonnet head=8834195b0683
at: src/cmd/client.rs:180
quote: let verdict = verify_server(live.pid, &live.started_at, client.server_pid());
at: src/cmd/hook.rs:239
quote: let verdict = verify_server(snapshot.pid, &snapshot.started_at, client.server_pid());
at: crates/viola-mcp/src/tools.rs:363
quote: tokio::task::spawn_blocking(move || verify_server(live.pid, &live.started_at, observed))
at: crates/viola-state/src/liveness.rs:157
quote: let observed_start = observed_pid.and_then(process_start_time);
at: crates/viola-state/src/liveness.rs:146
quote: Some(start) if start == recorded_start => ServerVerdict::Matches,
@row_add n=472 row=decision/peer-euid-admission area=established-decisions rec=fct-wqjpkbtb@2,fct-ddp4mvl7@1
name: Peer admitted only on a matching euid
rule: A channel peer is admitted only when its euid is read and equals the own user's; an unreadable euid is refused, and the wrapper drops a refused stream before any frame, writing nothing, with no env, flag or feature seam.
@row_check n=472 row=decision/peer-euid-admission sha=d64fda39eacc v=holds judge=sonnet head=8834195b0683
at: crates/viola-channel/src/client.rs:140
quote: observed == Some(own)
at: crates/viola-channel/src/server.rs:145
quote: .filter(|stream| admitted(peer_euid_of(stream)));
at: crates/viola-channel/src/server.rs:69
quote: self.serve_admitting(dispatch, peer_admitted)
@row_add n=473 row=decision/snapshots-read-through-replay area=established-decisions rec=fct-cskfkcjl@1,fct-ldjewu6a@1
name: Snapshots read through the replay
rule: Product readers read a snapshot only through the replay, which meets an unreadable or newer snapshot as absent, so it gives no endpoint and no statusline command, and no frame is written and nothing runs.
@row_check n=473 row=decision/snapshots-read-through-replay sha=04dd85f123f4 v=holds judge=sonnet head=8834195b0683
at: crates/viola-state/src/liveness.rs:93
quote: let Ok(Recovered::Snapshot(snapshot)) = read_snapshot_or_replay(instance_dir) else {
at: src/cmd/hook.rs:227
quote: let Ok(Recovered::Snapshot(snapshot)) = read_snapshot_or_replay(&instance.dir) else {
at: src/cmd/hook/statusline.rs:129
quote: let command = match read_snapshot_or_replay(&instance.dir) {
@row_add n=474 row=decision/mcp-server-on-rmcp area=established-decisions rec=fct-xewwdlh6@1
name: MCP server on rmcp over stdio
rule: The MCP server viola serves to drivers is built on the rmcp SDK over stdio, pinned to one minor version.
why: rmcp is the official SDK and tracks the MCP spec; the pinned minor limits churn from its near-weekly releases.
@row_check n=474 row=decision/mcp-server-on-rmcp sha=b7c70345d1dc v=holds judge=sonnet head=8834195b0683
at: Cargo.toml:223
quote: rmcp = { version = ">=3.4.1, <3.5", default-features = false, features = ["server", "transport-io"] }
at: crates/viola-mcp/src/lib.rs:76
quote: let (stdin, stdout) = rmcp::transport::stdio();
@row_add n=475 row=contract/mcp-tools/refusal area=established-decisions rec=fct-xewwdlh6@1
boundary: MCP tools
item: refusal
promise: A refusal is an `isError: true` tool result mapped from the channel's `result.refusal`.
@row_check n=475 row=contract/mcp-tools/refusal sha=803b12028ba0 v=holds judge=sonnet head=8834195b0683
at: crates/viola-mcp/src/result.rs:21
quote: failed(reason, json!({"refusal": reason, "detail": detail}))
at: crates/viola-mcp/src/result.rs:40
quote: let mut result = CallToolResult::structured_error(body);
at: crates/viola-mcp/src/result.rs:69
quote: .get("refusal")
@row_add n=476 row=contract/mcp-tools/from area=established-decisions rec=fct-xewwdlh6@1
boundary: MCP tools
item: from
promise: The MCP server adds `from` from its own `VIOLA_NAME` only when it has a name of its own.
@row_check n=476 row=contract/mcp-tools/from sha=f8c445bfee81 v=holds judge=sonnet head=8834195b0683
at: crates/viola-mcp/src/tools.rs:318-319
quote: if let Some(from) = &ctx.from { params.insert("from".to_owned(), Value::from(AsRef::<str>::as_ref(from)));
@row_add n=477 row=decision/human-controls-cli-only area=established-decisions rec=fct-xewwdlh6@1
name: Human wheel and link controls are CLI-only
rule: `release`, `pause`, `link` and `unlink` are offered on the CLI only, never as MCP tools on the driver's surface.
why: They are human controls over the wheel and link topology, so a driver is never offered a way to hand the wheel back; driver links are already recorded from `from`.
@row_check n=477 row=decision/human-controls-cli-only sha=15a54cfafd4d v=holds judge=sonnet head=8834195b0683
at: crates/viola-mcp/src/tools.rs:37
quote: pub const TOOL_NAMES: [&str; 5] = ["send", "wait", "last", "answer", "list"];
at: src/cmd/release.rs:34
quote: pub(crate) fn release(home: &Path, args: &ReleaseArgs) -> anyhow::Result<ExitCode> {
@row_add n=478 row=decision/driver-release-deterred-not-enforced area=established-decisions rec=fct-xewwdlh6@1
name: Release from a driver deterred, not enforced
rule: A `release` whose params carry a string `from` is refused, which deters a driver; a driver can still run CLI `viola release` outside a wrapped session, because v1 trusts every process of the same OS user.
why: `from` is self-reported, so the refusal deters rather than enforces.
@row_check n=478 row=decision/driver-release-deterred-not-enforced sha=6ae5d9c5042e v=holds judge=sonnet head=8834195b0683
at: src/run/wheel.rs:204-206
quote: if let Some(Value::String(from)) = params.get("from") { log_release_from_driver(call, from); return Err(ProtocolError::ReleaseFromDriver);
at: src/cmd/release.rs:34
quote: pub(crate) fn release(home: &Path, args: &ReleaseArgs) -> anyhow::Result<ExitCode> {
@row_add n=479 row=contract/channel-release/release-from-driver area=established-decisions rec=fct-xewwdlh6@1
boundary: Wrapper channel release method
item: release-from-driver
promise: A `release` whose `params` carry a string `from` is refused with `-32602` and `data.reason` `release-from-driver`.
@row_check n=479 row=contract/channel-release/release-from-driver sha=38c4e81ec050 v=holds judge=sonnet head=8834195b0683
at: crates/viola-channel/src/lib.rs:96
quote: Self::ReleaseFromDriver => json!({"reason": "release-from-driver"}),
at: src/run/wheel.rs:204-206
quote: if let Some(Value::String(from)) = params.get("from") {             log_release_from_driver(call, from);             return Err(ProtocolError::ReleaseFromDriver);
@row_add n=480 row=decision/mcp-wait-cap area=established-decisions rec=fct-xewwdlh6@1
name: MCP wait capped at 50 s
rule: MCP `wait` sends `timeout_ms` as the smaller of the given value and 50 000 ms, and 50 000 ms when none is given.
why: The cap sits below the Claude CLI's stdio tool-call default, so the tool call never times out on the client first.
@row_check n=480 row=decision/mcp-wait-cap sha=195a5da7bb78 v=holds judge=sonnet head=8834195b0683
at: crates/viola-mcp/src/tools.rs:42
quote: pub const WAIT_CAP: u64 = 50_000;
at: crates/viola-mcp/src/tools.rs:46
quote: given.map_or(WAIT_CAP, |given| given.min(WAIT_CAP))
@row_add n=481 row=contract/mcp-tool-errors/invalid-params area=established-decisions rec=fct-xewwdlh6@1
boundary: MCP tool errors
item: invalid-params
promise: An argument object that breaks the tool's schema gives the `invalid-params` result with a closed field `detail` and no frame.
@row_check n=481 row=contract/mcp-tool-errors/invalid-params sha=812c09afd80b v=holds judge=sonnet head=8834195b0683
at: crates/viola-mcp/src/result.rs:36
quote: error("invalid-params", Value::from(field))
at: crates/viola-mcp/src/tools.rs:372
quote: (Err(field), _) | (_, Err(field)) => return Ok(result::invalid(field)),
@row_add n=482 row=contract/mcp-tool-errors/-32603 area=established-decisions rec=fct-xewwdlh6@1
boundary: MCP tool errors
item: -32603
promise: An internal handler fault is the JSON-RPC error `-32603` `internal error` with `data: null`.
@row_check n=482 row=contract/mcp-tool-errors/-32603 sha=0321798c9d65 v=holds judge=sonnet head=8834195b0683
at: crates/viola-mcp/src/tools.rs:193-197
quote: ErrorData::new(         ErrorCode::INTERNAL_ERROR,         McpError::Internal.to_string(),         Some(Value::Null),     )
@row_add n=483 row=decision/mcp-stdin-oversize-refused-whole area=established-decisions rec=fct-cvcawwtk@1
name: Oversize MCP stdin line refused whole
rule: An MCP stdin line longer than the frame cap reaches rmcp as no byte and ends the session as at a closed stdin; the line is never truncated and never skipped.
why: A truncated line could dispatch a padded valid request, and a skipped one leaves the peer unanswered with no signal; follows the wrapper channel's close-on-oversize rule.
@row_check n=483 row=decision/mcp-stdin-oversize-refused-whole sha=75cf34bed19f v=holds judge=sonnet head=8834195b0683
at: crates/viola-mcp/src/stdin.rs:60-63
quote: if !self.fits(self.held.len() + segment.len()) { self.held.clear(); self.done = true; self.cut.store(true, Ordering::Relaxed);
at: crates/viola-mcp/src/lib.rs:77
quote: let stdin = stdin::CappedLines::new(stdin, MAX_FRAME);
@row_add n=484 row=contract/mcp-stdin-oversize/parse-rejected area=established-decisions rec=fct-dfjcw6cl@1,fct-u62xcdo4@1
boundary: MCP stdin oversize process log
item: parse-rejected
promise: A session ended by an oversize line writes exactly one `parse-rejected{parser:"mcp-stdin", detail:"oversize", count:1}` line at WARN, holding no byte of the line.
@row_check n=484 row=contract/mcp-stdin-oversize/parse-rejected sha=6d39222956a8 v=holds judge=sonnet head=8834195b0683
at: crates/viola-mcp/src/lib.rs:95-101
quote: obs_event!(         WARN,         ObsEvent::ParseRejected,         parser = "mcp-stdin",         detail = "oversize",         count = 1u64,     );
at: crates/viola-mcp/src/lib.rs:86-87
quote: if cut.load(Ordering::Relaxed) {             reject_oversize_line();
@row_add n=485 row=contract/viola-mcp-exit-codes/0-oversize-after-initialize area=established-decisions rec=fct-dfjcw6cl@1
boundary: viola mcp exit codes
item: 0 after an oversize line once initialized
promise: An oversize line after `initialize` completed ends `viola mcp` with exit 0, as at a closed stdin.
@row_check n=485 row=contract/viola-mcp-exit-codes/0-oversize-after-initialize sha=fd32220cc204 v=holds judge=sonnet head=8834195b0683
at: crates/viola-mcp/src/lib.rs:83
quote: Ok(_) => Ok(()),
at: src/cmd/mcp.rs:38
quote: Ok(ExitCode::SUCCESS)
@row_add n=486 row=contract/viola-mcp-exit-codes/1-oversize-before-initialize area=established-decisions rec=fct-dfjcw6cl@1,fct-u62xcdo4@1
boundary: viola mcp exit codes
item: 1 on an oversize line before initialize
promise: An oversize line before `initialize` completes ends `viola mcp` with exit 1 and both streams empty.
@row_check n=486 row=contract/viola-mcp-exit-codes/1-oversize-before-initialize sha=405b73c49901 v=holds judge=sonnet head=8834195b0683
at: crates/viola-mcp/src/lib.rs:80
quote: Err(_) => Err(ServeError::Initialize),
at: src/main.rs:136-138
quote: Role::Mcp | Role::Cli | Role::Other,             Outcome::Unparsed | Outcome::Failed | Outcome::Panicked,         ) => ExitCode::from(1),
@row_add n=487 row=decision/mcp-stdin-bound-in-own-reader area=established-decisions rec=fct-75j5r4u3@1
name: MCP stdin bound in viola's own reader
rule: The MCP stdin line bound sits in the reader viola hands rmcp, never in rmcp's codec, and a cut reads as end of stream, never as an I/O error.
why: rmcp's stdio transport reads each line unbounded and its codec bounds only the writer side; end of stream keeps rmcp from writing anything of its own.
@row_check n=487 row=decision/mcp-stdin-bound-in-own-reader sha=4b8b07e4ef10 v=holds judge=sonnet head=8834195b0683
at: crates/viola-mcp/src/lib.rs:77
quote: let stdin = stdin::CappedLines::new(stdin, MAX_FRAME);
at: crates/viola-mcp/src/stdin.rs:93-95
quote: if this.done { return Poll::Ready(Ok(())); }
@row_add n=488 row=decision/exec-form-command-hooks area=established-decisions rec=fct-xbwprhwr@2
name: Exec-form command hooks
rule: Hooks are exec-form `command` hooks running the pinned viola copy with `hook <event>` as arguments.
why: Exec form skips shell startup; `http` hooks need a fixed URL and so a daemon; `mcp_tool` is unavailable at SessionStart.
@row_check n=488 row=decision/exec-form-command-hooks sha=46f468846da6 v=holds judge=sonnet head=8834195b0683
at: plugin/hooks/hooks.json:4
quote: {"hooks": [{"type": "command", "command": "@@VIOLA_BIN@@", "args": ["hook", "session-start"], "timeout": 5}]}
at: plugin/hooks/hooks.json:19
quote: {"hooks": [{"type": "command", "command": "@@VIOLA_BIN@@", "args": ["hook", "session-end"]}]}
@row_add n=489 row=contract/hook-tiers/ordering-spine area=established-decisions rec=fct-xbwprhwr@2
boundary: Hook tiers
item: sync with no reply body (the ordering spine)
promise: SessionStart, UserPromptSubmit and Stop each write their `hook.event` and exit 0 within viola's spine deadline, below the 5 s `timeout` they get in `hooks.json`.
@row_check n=489 row=contract/hook-tiers/ordering-spine sha=402e069eea75 v=holds judge=sonnet head=8834195b0683
at: plugin/hooks/hooks.json:4
quote: {"hooks": [{"type": "command", "command": "@@VIOLA_BIN@@", "args": ["hook", "session-start"], "timeout": 5}]}
at: plugin/hooks/hooks.json:7
quote: {"hooks": [{"type": "command", "command": "@@VIOLA_BIN@@", "args": ["hook", "user-prompt-submit"], "timeout": 5}]}
at: plugin/hooks/hooks.json:16
quote: {"hooks": [{"type": "command", "command": "@@VIOLA_BIN@@", "args": ["hook", "stop"], "timeout": 5}]}
at: src/cmd/hook.rs:56
quote: const CONNECT_DEADLINE: Duration = Duration::from_millis(750);
at: src/cmd/hook.rs:570
quote: .notify("hook.event", params)
@row_add n=490 row=contract/hook-tiers/session-end area=established-decisions rec=fct-xbwprhwr@2
boundary: Hook tiers
item: SessionEnd
promise: Sync within about 1 s: the channel, else a direct try-lock append of the event, else nothing; `hooks.json` gives it no `timeout` key.
@row_check n=490 row=contract/hook-tiers/session-end sha=75bb63f93715 v=stale judge=sonnet head=8834195b0683
at: src/cmd/hook.rs:194
quote: append_session_end(instance, normalised);
shows: The hook's channel-else-try-lock fallback and the absent `timeout` key exist, but no `run` code writes `session-end` from the process exit (the only `EventKind::SessionEnd` in src/cmd/run.rs is the hook-kind allow-list at line 90).
cell: promise
value: Sync within about 1 s: the channel, else a direct try-lock append of the event, else nothing; `hooks.json` gives it no `timeout` key.
@row_add n=491 row=decision/spine-deadline area=established-decisions rec=fct-xbwprhwr@2
name: Spine deadline of 1 s
rule: Every ordering-spine hook finishes within one built-in spine deadline of 1 s, the same for every event.
guard: the perf gate (test-plan §10)
@row_check n=491 row=decision/spine-deadline sha=a2564cd15f45 v=holds guard=true judge=sonnet head=8834195b0683
at: crates/viola-core/src/lib.rs:14
quote: pub const SPINE_DEADLINE: Duration = Duration::from_secs(1);
at: src/cmd/hook.rs:146
quote: match handle(event, &instance, spine_deadline(started), stdin) {
guard_at: crates/viola-e2e/src/harness/gate.rs:145
@row_add n=492 row=decision/hook-connect-deadline area=established-decisions rec=fct-xbwprhwr@2
name: Hook connect deadline
rule: The connect deadline a hook passes to the channel client is 750 ms, kept below the spine deadline.
guard: a unit test asserting it below the spine deadline
state: provisional
@row_check n=492 row=decision/hook-connect-deadline sha=9dbd36b8c3c4 v=holds guard=true judge=sonnet head=8834195b0683
at: src/cmd/hook.rs:56
quote: const CONNECT_DEADLINE: Duration = Duration::from_millis(750);
guard_at: src/cmd/hook.rs:713-714
@row_add n=493 row=decision/spine-hook-loses-event-not-turn area=established-decisions rec=fct-xbwprhwr@2
name: Spine hook loses the event, never delays the turn
rule: A spine hook that cannot reach or write the endpoint within the deadline records the failure in `diagnostics/` and exits 0, so the event is lost.
why: The event is lost rather than the turn delayed; a driver recovers through `wait`'s `timeout_ms`.
@row_check n=493 row=decision/spine-hook-loses-event-not-turn sha=b48a640174b6 v=holds judge=sonnet head=8834195b0683
at: src/cmd/hook.rs:196
quote: Ok(Some(not_sent.detail()))
at: src/cmd/hook.rs:148-149
quote: decided(event, detail, started); Ok(ExitCode::SUCCESS)
at: src/cmd/hook.rs:590
quote: Some(detail) => obs_event!(
@row_add n=494 row=decision/session-end-budget-never-raised area=established-decisions rec=fct-xbwprhwr@2
name: SessionEnd budget never raised
rule: The 1.5 s SessionEnd budget is never raised.
@row_check n=494 row=decision/session-end-budget-never-raised sha=5ad9755bb433 v=unverifiable judge=sonnet head=8834195b0683
searched: 1500|1\.5 ?s|SESSION_END · in src/**/*.rs and crates/**/*.rs
searched: timeout · in plugin/hooks/hooks.json
why: no evidence found; a process rule, and no 1.5 s SessionEnd budget constant stands in the code (SessionEnd shares the 750 ms CONNECT_DEADLINE and hooks.json gives it no timeout)
@row_add n=495 row=contract/hook-diagnostics/socket-dir-refused area=established-decisions rec=fct-5ktqti5y@1,fct-qoiufq4r@1
boundary: Hook diagnostics
item: socket-dir-refused
promise: A hook meeting a refused socket directory sends no frame, exits 0 with empty streams and records one `hook-decision` at WARN with `detail` `socket-dir-refused`.
@row_check n=495 row=contract/hook-diagnostics/socket-dir-refused sha=d00bb9f93464 v=holds judge=sonnet head=8834195b0683
at: src/cmd/hook.rs:231-232
quote: if check_endpoint_dir(endpoint).is_err() { return Err(NotSent::SocketDir);
at: crates/viola-core/src/lib.rs:201
quote: Self::SocketDir => "socket-dir-refused",
at: src/cmd/hook.rs:390
quote: WARN,
@row_add n=496 row=contract/hook-diagnostics/server-verify-failed area=established-decisions rec=fct-5ktqti5y@1,fct-qoiufq4r@1
boundary: Hook diagnostics
item: server-verify-failed
promise: A hook whose serving process has another euid, is not the snapshot's pid and start time, or cannot be checked sends no frame, exits 0 with no body and records `server-verify-failed`.
@row_check n=496 row=contract/hook-diagnostics/server-verify-failed sha=0dcbd227bd51 v=holds judge=sonnet head=8834195b0683
at: src/cmd/hook.rs:236-237
quote: if !peer_admitted(client.server_euid()) { return Err(NotSent::ServerUnverified);
at: src/cmd/hook.rs:242
quote: ServerVerdict::Mismatch | ServerVerdict::Unverifiable => Err(NotSent::ServerUnverified),
at: src/cmd/hook.rs:214
quote: Self::ServerUnverified => "server-verify-failed",
@row_add n=497 row=contract/hook-exit/0-with-body area=established-decisions rec=fct-wpbp4hte@2
boundary: Hook exit and body
item: exit 0 with a JSON body
promise: A decision.
@row_check n=497 row=contract/hook-exit/0-with-body sha=d4516f5f4e72 v=holds judge=sonnet head=8834195b0683
at: src/cmd/hook.rs:298
quote: out.write_all(body.as_bytes())?;
at: src/cmd/hook.rs:135
quote: Ok(()) => Ok(ExitCode::SUCCESS),
@row_add n=498 row=contract/hook-exit/0-no-body area=established-decisions rec=fct-wpbp4hte@2
boundary: Hook exit and body
item: exit 0 with no body
promise: No decision: the dialog renders for the human.
@row_check n=498 row=contract/hook-exit/0-no-body sha=342fb0f1c7e1 v=holds judge=sonnet head=8834195b0683
at: src/cmd/hook.rs:354-360
quote: let Asked::Replied { response: Some(response), .. } = asked else { return None; };
at: src/cmd/hook.rs:297
quote: if let Some(body) = &body {
@row_add n=499 row=contract/hook-exit/2 area=established-decisions rec=fct-wpbp4hte@2
boundary: Hook exit and body
item: exit 2
promise: Never produced.
@row_check n=499 row=contract/hook-exit/2 sha=39cbeee3abdc v=holds judge=sonnet head=8834195b0683
at: src/cmd/hook.rs:149
quote: Ok(ExitCode::SUCCESS)
searched: ExitCode::from · in src/cmd/hook.rs
searched: process::exit · in src/cmd/hook.rs
searched: exit(2 · in src/cmd/hook.rs
searched: ExitCode::FAILURE · in src/cmd/hook.rs
@row_add n=500 row=contract/probe-capture/payload-file area=established-decisions rec=fct-wpbp4hte@2
boundary: Hook probe capture directory
item: `<PascalEvent>.<k>.json`
promise: Each captured payload is claimed exclusively at 0600 under the first free `k`, which counts across events, so names sort in arrival order and two captures never share a name.
@row_check n=500 row=contract/probe-capture/payload-file sha=dcead6d42ce6 v=holds judge=sonnet head=8834195b0683
at: src/cmd/hook.rs:437
quote: match create_private_new(&path) {
at: src/cmd/hook.rs:439
quote: Err(e) if e.kind() == std::io::ErrorKind::AlreadyExists => k = k.checked_add(1)?,
at: src/cmd/hook.rs:530
quote: (1..=n + 1).find(|k| !taken.contains(k))
@row_add n=501 row=contract/probe-capture/statusline-shell-file area=established-decisions rec=fct-wpbp4hte@2
boundary: Hook probe capture directory
item: `StatusLine.<k>.shell`
promise: Holds exactly `bin-sh` when the `--shell` word equals `/bin/sh` byte for byte, else `other`; the word itself is written nowhere.
@row_check n=501 row=contract/probe-capture/statusline-shell-file sha=ae52f4f18d2e v=holds judge=sonnet head=8834195b0683
at: src/cmd/hook.rs:460
quote: let verdict = if shell == Some(OsStr::new(BIN_SH)) {
at: src/cmd/hook.rs:476
quote: replace_private(&shell_path, verdict.word().as_bytes(), FILE_MODE).ok()?;
@row_add n=502 row=decision/dialog-hold area=established-decisions rec=fct-wpbp4hte@2
name: Dialog hold per instance
rule: A sync dialog hook waits for the driver at most the instance's dialog hold, whole seconds from 60 to 3600, default 60, read once at the wrapper's start.
state: provisional
@row_check n=502 row=decision/dialog-hold sha=ff3c4f6f5346 v=holds judge=sonnet head=8834195b0683
at: crates/viola-core/src/lib.rs:19
quote: pub const DIALOG_HOLD_DEFAULT: Duration = Duration::from_secs(60);
at: crates/viola-core/src/lib.rs:22
quote: pub const DIALOG_HOLD_MIN: Duration = Duration::from_secs(60);
at: crates/viola-core/src/lib.rs:27
quote: pub const DIALOG_HOLD_MAX: Duration = Duration::from_secs(3600);
at: src/cmd/run.rs:311
quote: let (hold, rejections) = run::dialog::read_hold(&home);
@row_add n=503 row=decision/viola-ends-the-dialog-wait area=established-decisions rec=fct-wpbp4hte@2
name: Viola, never Claude Code, ends the dialog wait
rule: The dialog hook bounds its reply read at the hold ceiling plus 5 s, and the dialog hooks' `timeout` in `hooks.json` sits above that by a pinned margin.
why: Viola ends the wait rather than Claude Code, and the rendered plugin stays byte-identical across starts.
@row_check n=503 row=decision/viola-ends-the-dialog-wait sha=84f3a9156f5f v=holds judge=sonnet head=8834195b0683
at: src/cmd/hook.rs:72
quote: started + DIALOG_HOLD_MAX + REPLY_GRACE
at: plugin/hooks/hooks.json:10
quote: {"matcher": "AskUserQuestion|ExitPlanMode", "hooks": [{"type": "command", "command": "@@VIOLA_BIN@@", "args": ["hook", "pre-tool-use"], "timeout": 3615}]}
at: crates/viola-agent-claude/src/lib.rs:444
quote: assert!(margin >= std::time::Duration::from_secs(10), "{event}");
@row_add n=504 row=contract/dialog-hold-end/null-reply area=established-decisions rec=fct-wpbp4hte@2
boundary: Dialog hold end
item: null reply
promise: At the hold's end the wrapper replies `null`, the hook exits 0 with no body, the dialog renders for the human, and a later `answer` for that dialog is refused `not-delivered` with detail `unknown-dialog`.
@row_check n=504 row=contract/dialog-hold-end/null-reply sha=aacb7a07dfd5 v=holds judge=sonnet head=8834195b0683
at: src/run/dialog.rs:369
quote: reply(dialog_id, answer.as_ref())
at: src/run/dialog.rs:444-447
quote: return Ok(refusal( RefusalReason::NotDelivered, Some(NotDelivered::UnknownDialog.as_str()), ));
at: src/cmd/hook.rs:297
quote: if let Some(body) = &body {
@row_add n=505 row=decision/hook-never-writes-stderr area=established-decisions rec=fct-wpbp4hte@2
name: Hook logs only to diagnostics
rule: A hook never writes to stderr; its codes-only log goes to the home-level diagnostics and content-bearing detail only to the instance's diagnostics, and a failure to write either is ignored.
@row_check n=505 row=decision/hook-never-writes-stderr sha=47e31ce49157 v=holds judge=sonnet head=8834195b0683
at: src/obs.rs:332
quote: let _ = file.write_all(line.as_bytes());
searched: eprint · in src/cmd/hook.rs
searched: io::stderr · in src/cmd/hook.rs
@row_add n=506 row=contract/event-line/dialog-expired area=established-decisions rec=fct-ohekhpaf@1
boundary: The event line
item: dialog-expired
promise: Appended once by the wrapper at a dialog hold's end with `{dialog_id, dialog_kind}` and no question, plan or tool text, waking a parked `wait`; a hand-back or an answer appends none.
@row_check n=506 row=contract/event-line/dialog-expired sha=61466c54aaf1 v=holds judge=sonnet head=8834195b0683
at: src/run/dialog.rs:358
quote: let expired = state.pending.take().filter(|_| deadline_hit);
at: src/run/dialog.rs:376-381
quote: let line = EventLine::new( &self.name, EventKind::DialogExpired, Source::Wrapper, json!({"dialog_id": pending.dialog_id, "dialog_kind": pending.kind.as_str()}), Utc::now(),
at: src/run/dialog.rs:385
quote: .appending(&line, || append_event(&self.instance_dir, &line));
@row_add n=507 row=decision/byte-source-wheel-claim area=established-decisions rec=fct-4coxmu76@2
name: Byte-source wheel claim
rule: Any human editing key since the last turn boundary moves the wheel to the human, and automated `send` is then refused `human-typing`.
why: `viola run` owns the real stdin, so it knows byte provenance exactly.
@row_check n=507 row=decision/byte-source-wheel-claim sha=7abc22e0b5b6 v=holds judge=sonnet head=8834195b0683
at: src/run/wheel.rs:322
quote: if self.keys.feed(&buf[..n]) {
at: src/run/wheel.rs:323
quote: self.wheel.human_input();
at: src/run/send.rs:96
quote: if let Some(detail) = slot.wheel.human_typing() {
at: src/run/send.rs:99
quote: RefusalReason::HumanTyping,
@row_add n=508 row=decision/closed-non-editing-byte-list area=established-decisions rec=fct-4coxmu76@2
name: Closed list of non-editing bytes
rule: Only a closed list of focus reports, mouse reports and terminal replies, each by exact grammar, never takes the wheel; every other byte does, and a reply no terminal was measured to write is not added on a guess.
why: Terminal replies read as typing took the wheel from a live driver with no key typed.
@row_check n=508 row=decision/closed-non-editing-byte-list sha=cf49b8a472e5 v=holds judge=sonnet head=8834195b0683
at: src/run/wheel.rs:491
quote: fn is_reply(params: &[u8], fin: u8) -> bool {
at: src/run/wheel.rs:405
quote: true
@row_add n=509 row=decision/wheel-classifier-only-observes area=established-decisions rec=fct-4coxmu76@2
name: Wheel classifier only observes
rule: The stdin classifier returns every byte whole and unchanged, and a read holding an editing key moves the wheel before it returns.
@row_check n=509 row=decision/wheel-classifier-only-observes sha=22963892ab79 v=holds judge=sonnet head=8834195b0683
at: src/run/wheel.rs:321-325
quote: let n = self.inner.read(buf)?; if self.keys.feed(&buf[..n]) { self.wheel.human_input(); } Ok(n)
@row_add n=510 row=decision/atomic-bracketed-send area=established-decisions rec=fct-4coxmu76@2
name: Atomic bracketed-paste send
rule: A bridge send is written as one bracketed-paste write plus CR, and human bytes arriving during it are held and passed on straight after.
@row_check n=510 row=decision/atomic-bracketed-send sha=3210ca0c9d1e v=holds judge=sonnet head=8834195b0683
at: crates/viola-pty/src/pump.rs:79
quote: bytes.extend_from_slice(b"\x1b[200~");
at: crates/viola-pty/src/pump.rs:80
quote: bytes.extend_from_slice(text.as_bytes());
at: crates/viola-pty/src/pump.rs:81
quote: bytes.extend_from_slice(b"\x1b[201~\r");
at: crates/viola-pty/src/pump.rs:85
quote: .write_all(&bytes)
@row_add n=511 row=decision/human-prompt-confirms-wheel-take area=established-decisions rec=fct-4coxmu76@2
name: Human prompt confirms the wheel take
rule: An unsent prompt the hook filed as `human` moves the wheel before its line is appended; a `harness` prompt and the in-flight driver prompt never move it.
@row_check n=511 row=decision/human-prompt-confirms-wheel-take sha=cc1dad19ad47 v=holds judge=sonnet head=8834195b0683
at: src/run/send/slot.rs:219-220
quote: if line.kind == EventKind::PromptSubmitted && line.data["origin"] == "human" { slot.wheel.human_input();
@row_add n=512 row=decision/harness-prompts-not-human area=established-decisions rec=fct-4coxmu76@2
name: Harness prompts are not a human take
rule: A prompt whose raw start is one of the compiled harness prefixes is filed as a harness turn and never moves the wheel.
except: A human who types such a tag at a prompt's start is filed `harness`, an accepted side effect.
@row_check n=512 row=decision/harness-prompts-not-human sha=4d9d9d6ffa4f v=holds judge=sonnet head=8834195b0683
at: crates/viola-agent-claude/src/hook.rs:193
quote: if HARNESS_PREFIXES.iter().any(|p| raw.starts_with(p)) {
at: src/run/send/slot.rs:219
quote: if line.kind == EventKind::PromptSubmitted && line.data["origin"] == "human" {
@row_add n=513 row=decision/human-interrupt-moves-wheel area=established-decisions rec=fct-4coxmu76@2
name: Human interrupt moves the wheel
rule: A human interrupt is human input and moves the wheel to the human; only a `release` that returns a human-held wheel clears the running-turn state.
why: A turn can end with no `turn-ended`, since Claude Code's Stop hook does not fire on a user interrupt.
@row_check n=513 row=decision/human-interrupt-moves-wheel sha=e53eca605b2a v=holds judge=sonnet head=8834195b0683
at: src/run/wheel.rs:42
quote: (Input::Human, Wheel::Driver, _) => Some((Wheel::Human, WheelCause::HumanInput)),
at: src/run/wheel.rs:155-156
quote: if input == Input::Release { held.turn = Turn::Idle;
@row_add n=514 row=decision/wheel-returns-only-through-release area=established-decisions rec=fct-4coxmu76@2
name: Wheel returns only through CLI release
rule: CLI `viola pause` moves the wheel to the human without a keystroke, and the wheel returns to the driver only through CLI `viola release`.
@row_check n=514 row=decision/wheel-returns-only-through-release sha=cb22eae6f988 v=holds judge=sonnet head=8834195b0683
at: src/run/wheel.rs:44
quote: (Input::Pause, _, _) => Some((Wheel::Human, WheelCause::ManualPause)),
at: src/run/wheel.rs:45
quote: (Input::Release, Wheel::Human, _) => Some((Wheel::Driver, WheelCause::Release)),
@row_add n=515 row=contract/send-refusal/manual-pause area=established-decisions rec=fct-4coxmu76@2
boundary: Send refusal details
item: manual-pause
promise: After `viola pause`, automated `send` is refused `human-typing` with detail `manual-pause`.
@row_check n=515 row=contract/send-refusal/manual-pause sha=f8cde5d06d2d v=holds judge=sonnet head=8834195b0683
at: src/run/wheel.rs:114
quote: .then(|| (held.cause == WheelCause::ManualPause).then_some(HumanTyping::ManualPause))
at: src/run/send.rs:96-101
quote: if let Some(detail) = slot.wheel.human_typing() { return ctx.refuse( None, RefusalReason::HumanTyping, detail.map(HumanTyping::as_str), );
@row_add n=516 row=decision/human-wheel-renders-dialogs area=established-decisions rec=fct-4coxmu76@2
name: Human-held wheel renders dialogs at once
rule: While the wheel is `human`, `hook.dialog` is answered `null` at once and `answer` is refused `human-typing`; a dialog pending when the wheel moves to the human is answered `null` at once.
why: A dialog held by the hook is never hidden from the human for the rest of the hold.
@row_check n=516 row=decision/human-wheel-renders-dialogs sha=056147334564 v=holds judge=sonnet head=8834195b0683
at: src/run/dialog.rs:251-254
quote: let hold = self.cli_verified && state.pending.is_none() && !unanswerable && self.wheel.holder() == Wheel::Driver;
at: src/run/dialog.rs:429-431
quote: if let Some(detail) = self.wheel.human_typing() { return Ok(refusal( RefusalReason::HumanTyping,
at: src/run/wheel.rs:294-295
quote: if step.to == Wheel::Human { self.dialogs.hand_back();
@row_add n=517 row=decision/budget-pause-gates-only-send area=established-decisions rec=fct-4coxmu76@2
name: Budget pause gates only send
rule: `budget-paused` gates only `send`, never `hook.dialog` or `answer`.
why: A running turn finishes.
@row_check n=517 row=decision/budget-pause-gates-only-send sha=b3e88b9c36b8 v=holds judge=sonnet head=8834195b0683
at: src/run/send.rs:103
quote: if let Some(budget) = slot.budget.get() {
at: src/run/send.rs:105
quote: .evaluate(Trigger::Send, Utc::now())
at: src/run/send.rs:108
quote: return ctx.budget_paused(&crossed);
searched: budget · in src/run/dialog.rs (no gate; only a snapshot field budget_paused: false in a test)
@row_add n=518 row=decision/links-recorded-as-log-events area=established-decisions rec=fct-anqliuvh@1
name: Links recorded as log events
rule: A link `driver → driven` is recorded when a `send` or `answer` comes from inside a wrapped session to another instance, or by `viola link`, and removed by `viola unlink`; the snapshot's link set is derived by replaying those events.
why: The log stays the single source of truth for the link set as for everything else.
@row_check n=518 row=decision/links-recorded-as-log-events sha=6e3cae73b8be v=holds judge=sonnet head=8834195b0683
at: src/run/links.rs:65
quote: let line = self.line(EventKind::Link, driver);
at: src/run/send.rs:145
quote: links.record(from).map_err(|_| ProtocolError::Internal)?;
at: src/run/links.rs:81
quote: self.remove(&driver).map_err(|_| ProtocolError::Internal)?;
at: src/run/links.rs:46
quote: apply_link_line(&mut restored, &line?.value);
@row_add n=519 row=contract/event-line/link area=established-decisions rec=fct-anqliuvh@1
boundary: The event line
item: link
promise: The driven wrapper appends a `link` line, `source: wrapper`, only when the pair is not already in its link set; a refused `send` or `answer` and a `from` equal to the wrapper's own name link nothing.
@row_check n=519 row=contract/event-line/link sha=bc6bc817fc4f v=holds judge=sonnet head=8834195b0683
at: src/run/links.rs:62
quote: if let Some(since) = self.since(&set, driver) {
at: src/run/links.rs:124
quote: EventLine::new(&self.name, kind, Source::Wrapper, data, Utc::now())
at: src/run/send.rs:142-145
quote: if let (Some(links), Some(from)) = (slot.links.get(), ctx.from.as_ref()) && from != name { links.record(from).map_err(|_| ProtocolError::Internal)?;
at: src/run/dialog.rs:452-455
quote: if let (Some(links), Some(from)) = (self.links.get(), from.as_ref()) && *from != self.name { links.record(from).map_err(|_| ProtocolError::Internal)?;
@row_add n=520 row=contract/event-line/unlink area=established-decisions rec=fct-anqliuvh@1
boundary: The event line
item: unlink
promise: `viola unlink` appends an `unlink` line with `source: wrapper` and `{driver, driven}`.
@row_check n=520 row=contract/event-line/unlink sha=8cac3358e3e1 v=holds judge=sonnet head=8834195b0683
at: src/run/links.rs:108
quote: let line = self.line(EventKind::Unlink, driver);
at: src/run/links.rs:123
quote: let data = json!({"driver": driver, "driven": driven});
at: src/run/links.rs:124
quote: EventLine::new(&self.name, kind, Source::Wrapper, data, Utc::now())
@row_add n=521 row=contract/channel-link/-32603 area=established-decisions rec=fct-anqliuvh@1
boundary: Wrapper channel link recording
item: -32603
promise: A link that cannot be recorded is `-32603`, with nothing issued or the answer not taken.
@row_check n=521 row=contract/channel-link/-32603 sha=48217a7c9c94 v=holds judge=sonnet head=8834195b0683
at: src/run/send.rs:145
quote: links.record(from).map_err(|_| ProtocolError::Internal)?;
at: src/run/dialog.rs:455
quote: links.record(from).map_err(|_| ProtocolError::Internal)?;
at: crates/viola-channel/src/lib.rs:75
quote: Self::Internal => -32603,
@row_add n=522 row=decision/instance-check-before-any-read area=established-decisions rec=fct-2y2sv475@1
name: Instance check before any read
rule: The client verbs check the home and target instance directories before any role file, log or snapshot read.
why: Opening a log narrows an existing home to 0700, so a check made after it would pass a home at 0770.
@row_check n=522 row=decision/instance-check-before-any-read sha=4b622f9b83ec v=holds judge=sonnet head=8834195b0683
at: src/cmd/client.rs:31
quote: if check_instance(&absolute, &absolute.join("instances").join(name)).is_ok() {
at: src/cmd/send.rs:167
quote: if let Some(code) = client::refuse_untrusted(home, &args.name, args.json)? {
@row_add n=523 row=contract/process-log/link-channel-lines area=established-decisions rec=fct-fmdsf7l4@1
boundary: Wrapper process log
item: link and unlink channel lines
promise: The `channel-request` / `channel-response` pair carries `method:"link"|"unlink"` and no driver or driven name.
@row_check n=523 row=contract/process-log/link-channel-lines sha=35dab669dda8 v=holds judge=sonnet head=8834195b0683
at: crates/viola-channel/src/server.rs:371
quote: method = method,
at: crates/viola-channel/src/server.rs:479
quote: method = self.method,
@row_add n=524 row=decision/budget-governor-soft-stop area=established-decisions rec=fct-ikuc6e4y@2
name: Budget governor soft stop
rule: Once a configured threshold is crossed, automated `send` is refused `budget-paused`; human keys are never blocked, a running turn finishes, and the pause lifts when `resets_at` passes or on `viola release --budget`.
why: The per-model weekly window is hidden, so thresholds are set low and the governor stays visible and conservative.
@row_check n=524 row=decision/budget-governor-soft-stop sha=d0f413702783 v=holds judge=sonnet head=8834195b0683
at: src/run/budget.rs:157
quote: (*used >= f64::from(threshold.into_inner()) && later(resets_at, now)).then(|| Crossed {
at: src/run/send.rs:108
quote: return ctx.budget_paused(&crossed);
at: src/run/wheel.rs:214
quote: let lifted = gate.lift().map_err(|_| ProtocolError::Internal)?;
@row_add n=525 row=decision/budget-default-thresholds area=established-decisions rec=fct-ikuc6e4y@2
name: Budget default thresholds
rule: The defaults are `five_hour` at or above 90 % and `seven_day` at or above 85 %, read once at the wrapper's start, and no value switches the gate off.
@row_check n=525 row=decision/budget-default-thresholds sha=0df8442450a7 v=holds judge=sonnet head=8834195b0683
at: src/run/budget.rs:30
quote: const FIVE_HOUR_DEFAULT: u8 = 90;
at: src/run/budget.rs:31
quote: const SEVEN_DAY_DEFAULT: u8 = 85;
at: src/cmd/run.rs:307
quote: let (thresholds, rejections) = budget::read_thresholds(&home);
@row_add n=526 row=decision/release-budget-per-instance area=established-decisions rec=fct-ikuc6e4y@2
name: Budget release per instance
rule: `release --budget` lifts the pause for one instance only, until the crossed window's `resets_at`, and leaves the wheel; a plain `release` never lifts a budget pause, and an override never outlives its wrapper.
@row_check n=526 row=decision/release-budget-per-instance sha=bfb53bc104bf v=holds judge=sonnet head=8834195b0683
at: src/run/wheel.rs:213-214
quote: if budget { let lifted = gate.lift().map_err(|_| ProtocolError::Internal)?;
at: src/run/budget.rs:314
quote: let end = crossed.resets_at.clone();
at: src/run/wheel.rs:222
quote: Ok(json!({"ok": {"wheel": self.holder().as_str(), "budget_paused": gate.paused()}}))
at: src/run/budget.rs:284
quote: *recorded = self.decide_at(Trigger::Start, now, None);
@row_add n=527 row=decision/budget-gate-never-polls area=established-decisions rec=fct-ikuc6e4y@2
name: Budget gate evaluated at three points
rule: The wrapper evaluates the budget gate only at start, when a `send` arrives and at each `turn-ended`, never by polling, so a snapshot can read paused after the reset until the next point.
@row_check n=527 row=decision/budget-gate-never-polls sha=8f4a012ec12b v=holds judge=sonnet head=8834195b0683
at: src/run/budget.rs:203-207
quote: pub(crate) enum Trigger { Start, Send, TurnEnded, }
at: src/run/send.rs:105
quote: .evaluate(Trigger::Send, Utc::now())
at: src/cmd/run.rs:131
quote: .evaluate(Trigger::TurnEnded, Utc::now())
at: src/cmd/run.rs:596
quote: let budget = recorded.budget.start(Utc::now());
@row_add n=528 row=decision/budget-gate-fail-open area=established-decisions rec=fct-ikuc6e4y@2
name: Budget gate fails open
rule: No reading, an unknown figure, a passed or unparseable `resets_at`, an unreadable `budget.json` or a bad threshold leaves the gate open or at its default.
@row_check n=528 row=decision/budget-gate-fail-open sha=8e6c36f537f1 v=holds judge=sonnet head=8834195b0683
at: crates/viola-state/src/budget.rs:94
quote: check_path(home).ok()?;
at: src/run/budget.rs:138
quote: DateTime::parse_from_rfc3339(instant).is_ok_and(|at| at > now)
at: src/run/budget.rs:155
quote: return None;
at: src/run/budget.rs:81
quote: compiled(default)
@row_add n=529 row=contract/send-refusal/order area=established-decisions rec=fct-ikuc6e4y@2
boundary: Send refusal details
item: order
promise: `control-character` → `empty-text` → `human-typing` → `budget-paused` → `turn-running` → the readiness gate's wait.
@row_check n=529 row=contract/send-refusal/order sha=156f99542796 v=holds judge=sonnet head=8834195b0683
at: src/run/send.rs:88
quote: if let Err(detail) = viola_core::validate_paste_text(text) {
at: src/run/send.rs:93
quote: if text.is_empty() {
at: src/run/send.rs:96
quote: if let Some(detail) = slot.wheel.human_typing() {
at: src/run/send.rs:103
quote: if let Some(budget) = slot.budget.get() {
at: src/run/send.rs:114
quote: if flight.send.is_some() || slot.wheel.turn_running() {
at: src/run/send.rs:128
quote: let (readiness, _) = gate.wait_ready(slot.clock.as_ref(), slot.clock.now());
@row_add n=530 row=decision/only-send-reads-budget-gate area=established-decisions rec=fct-ikuc6e4y@2
name: Only send reads the budget gate
rule: `answer`, `wait`, `last`, `pause` and `hook.dialog` do not read the budget gate.
guard: path6_a_closed_gate_leaves_answer_wait_last_and_pause_alone
@row_check n=530 row=decision/only-send-reads-budget-gate sha=b63c040afc11 v=holds guard=true judge=sonnet head=8834195b0683
at: src/cmd/run.rs:141-145
quote: "wait" => self.wait.wait(&self.instance_dir, call.params), "last" => self.wait.last(call.params), "hook.dialog" => self.dialogs.hook_dialog(call.params), "answer" => self.dialogs.answer(call), "pause" => self.wheel.pause(call),
guard_at: tests/cli_budget.rs:521
@row_add n=531 row=decision/statusline-wrapped-per-session area=established-decisions rec=fct-ikuc6e4y@2
name: Statusline wrapped only per viola session
rule: The statusline is wrapped only for sessions started by `viola run`, through a per-session settings override that runs the user's own statusline command unchanged.
why: The user's global statusline config is never touched.
@row_check n=531 row=decision/statusline-wrapped-per-session sha=14f31875d634 v=holds judge=sonnet head=8834195b0683
at: src/cmd/run.rs:380
quote: let settings = write_override(&instance_dir, &pinned)?;
at: crates/viola-agent-claude/src/statusline.rs:99
quote: Some(json!({"statusLine": {"type": "command", "command": command}}).to_string())
@row_add n=532 row=decision/cli-capability-ledger area=established-decisions rec=fct-qwpklckh@2
name: Per-behaviour CLI capability ledger
rule: Every load-bearing Claude Code CLI behaviour viola relies on is a row of a closed capability ledger, probed by viola verify and stamped per CLI version; a CLI version is verified only when its stamp holds every relied row pass.
why: The docs lag the installed build, and several load-bearing behaviours are measured, not documented.
@row_check n=532 row=decision/cli-capability-ledger sha=54c9458bd0b3 v=holds judge=sonnet head=8834195b0683
at: crates/viola-agent-claude/src/ledger.rs:1237-1239
quote: Ok(LedgerRow::relied() .iter() .all(|row| rows[row.id()] == "pass"))
@row_add n=533 row=contract/claude-cli-version-output/first-line area=established-decisions rec=fct-qwpklckh@2
boundary: Claude CLI --version output
item: first line
promise: Must read exactly X.Y.Z (Claude Code); output viola cannot parse counts as an unlisted version.
@row_check n=533 row=contract/claude-cli-version-output/first-line sha=1154f5f95fdb v=holds judge=sonnet head=8834195b0683
at: crates/viola-agent-claude/src/ledger.rs:436
quote: let version = line.strip_suffix(" (Claude Code)")?;
at: src/run/version_gate.rs:170
quote: let cli_version = answer.ok().and_then(|a| parse_version(&a.stdout));
@row_add n=534 row=decision/exit-plan-mode-allow-with-updated-input area=established-decisions rec=fct-qwpklckh@2
name: ExitPlanMode answered only through PreToolUse
rule: An ExitPlanMode plan is approved only through PreToolUse as allow with updatedInput set to the tool's own input unchanged (a non-object input gives {}), and revised through deny with a message.
why: A bare allow left the plan dialog up on the live CLI.
@row_check n=534 row=decision/exit-plan-mode-allow-with-updated-input sha=77bb9ec14720 v=holds judge=sonnet head=8834195b0683
at: crates/viola-agent-claude/src/dialog.rs:372-374
quote: ) => pre_tool_use_allow(Some(Value::Object( input.as_object().cloned().unwrap_or_default(), ))),
at: crates/viola-agent-claude/src/dialog.rs:381
quote: ) => permission_decision("deny", message.as_deref()),
@row_add n=535 row=decision/r8-identity-floor area=established-decisions rec=fct-qwpklckh@2
name: R8 parent-identity floor
rule: The measured CLAUDE* parent-identity environment names are stripped from the child even when the persistent environment defines them; every other CLAUDE* name follows the prefix rule.
@row_check n=535 row=decision/r8-identity-floor sha=020e25409eab v=holds judge=sonnet head=8834195b0683
at: crates/viola-agent-claude/src/lib.rs:73
quote: if !floor && persistent.contains(&text) {
at: crates/viola-agent-claude/src/lib.rs:17
quote: pub const IDENTITY_FLOOR: [&str; 11] = [
@row_add n=536 row=decision/harness-prompt-prefixes area=established-decisions rec=fct-qwpklckh@2
name: Harness prompts classified by raw prefix
rule: A prompt whose raw start matches a compiled harness prefix (<agent-message from=, <task-notification>, <\cross-session-message, <cross-session-message) is filed harness; an escaped tag is typed text and never classifies harness.
why: Ratified; the escaped cross-session prefix rests on a relayed measurement that the CLI's own injection arrives escaped.
except: <\cross-session-message is a harness prefix, so a human who types that tag at a prompt's start is filed harness too.
@row_check n=536 row=decision/harness-prompt-prefixes sha=75e6c8a9a629 v=holds judge=sonnet head=8834195b0683
at: crates/viola-agent-claude/src/hook.rs:111-114
quote: "<agent-message from=", "<task-notification>", "<\\cross-session-message", "<cross-session-message",
at: crates/viola-agent-claude/src/hook.rs:193
quote: if HARNESS_PREFIXES.iter().any(|p| raw.starts_with(p)) {
@row_add n=537 row=decision/prompt-normalise-order area=established-decisions rec=fct-qwpklckh@2
name: Classify before unwrapping and un-escaping
rule: Prompt normalisation classifies harness on the raw prefix first, then removes any long-paste wrapper with its frame, then un-escapes a backslash after < before an ASCII letter or /.
why: So a typed pair is never unwrapped and delivery matching compares plain text.
@row_check n=537 row=decision/prompt-normalise-order sha=bd15d83bc335 v=holds judge=sonnet head=8834195b0683
at: crates/viola-agent-claude/src/hook.rs:168
quote: json!({"text": prompt_text(&raw), "origin": prompt_origin(&raw)})
at: crates/viola-agent-claude/src/hook.rs:204
quote: unescape_tags(&unwrap_pastes(raw))
at: crates/viola-agent-claude/src/hook.rs:293
quote: if !after.starts_with(|c: char| c.is_ascii_alphabetic() || c == '/') {
@row_add n=538 row=decision/no-probe-for-unrelied-shapes area=established-decisions rec=fct-qwpklckh@2
name: No probe for a shape no send relies on
rule: A CLI shape that no send relies on needs no ledger probe; the paste frame beside typed text and between two pairs stays compiled and held by unit cases only.
why: Ratified ruling on the ledger rule; such shapes arise only when a human types beside a paste, so no claim and no wheel decision rests on them.
@row_check n=538 row=decision/no-probe-for-unrelied-shapes sha=21d495c6ee11 v=holds judge=sonnet head=8834195b0683
at: crates/viola-agent-claude/src/hook.rs:242
quote: const NEXT_FRAME: &str = "\n\n<pasted_content id=\"";
at: crates/viola-agent-claude/src/hook.rs:680
quote: fn prompt_text_drops_the_cli_framing_around_a_pair(#[case] raw: &str, #[case] text: &str) {
@row_add n=539 row=decision/send-never-types-trailing-newline area=established-decisions rec=fct-qwpklckh@2
name: Send never types a trailing newline
rule: send never types a text that ends in a newline.
why: A pasted text's own last newline never reaches the hook.
@row_check n=539 row=decision/send-never-types-trailing-newline sha=2ca8bba19a4a v=holds judge=sonnet head=8834195b0683
at: crates/viola-agent-claude/src/hook.rs:232
quote: let text = text.trim_end_matches(['\r', '\n']);
at: src/run/send.rs:91
quote: let typed = typed_text(text);
@row_add n=540 row=decision/local-command-post-conditions area=established-decisions rec=fct-qwpklckh@2
name: Local commands listed with delivery post-conditions
rule: The ledger lists each known built-in local command with its delivery post-condition or none; skills are not local commands.
why: Built-in local commands fire no UserPromptSubmit, while skills expand into a prompt and fire it.
@row_check n=540 row=decision/local-command-post-conditions sha=9ee63a7ad8f1 v=holds judge=sonnet head=8834195b0683
at: crates/viola-agent-claude/src/ledger.rs:191-195
quote: pub const LOCAL_COMMANDS: [(&str, Option<PostCondition>); 3] = [ ("/clear", Some(PostCondition::NewSession)), ("/remote-control", None), ("/compact", Some(PostCondition::CompactedSession)), ];
@row_add n=541 row=decision/send-waits-out-paste-hint area=established-decisions rec=fct-qwpklckh@2
name: Send waits out the paste hint
rule: On a verified CLI a send issued while the paste hint stands waits in the readiness gate for the input box, bounded at 8.5 s; a hint that outlasts the bound ends not-delivered / input-not-ready with nothing typed.
why: Ratified ruling.
@row_check n=541 row=decision/send-waits-out-paste-hint sha=a170160435b3 v=holds judge=sonnet head=8834195b0683
at: crates/viola-agent-claude/src/screen.rs:16
quote: pub const GATE_MAX_WAIT: Duration = Duration::from_millis(8500);
at: crates/viola-agent-claude/src/screen.rs:160-163
quote: } else if now.saturating_duration_since(waiting_since) < GATE_MAX_WAIT { GateStep::Wait } else { GateStep::Done(Readiness::InputNotReady)
@row_add n=542 row=contract/human-hint-lines/input-not-ready area=established-decisions rec=fct-qwpklckh@2
boundary: Human hint lines
item: input-not-ready
promise: Reads "<name> was not ready for input; send again, and if it repeats a human must look at the session" for a modal, a never-quiet screen, a poisoned screen or a hint past the bound; no hint line is printed for the paste-hint cause.
@row_check n=542 row=contract/human-hint-lines/input-not-ready sha=c949b739cd2e v=holds judge=sonnet head=8834195b0683
at: src/human/hints.rs:40-44
quote: "input-not-ready" => { format!( "{name} was not ready for input; send again, and if it repeats a human must look at the session" ) }
@row_add n=543 row=decision/verify-box-wait-for-literal area=established-decisions rec=fct-qwpklckh@2
name: Verify waits for an input-box literal after added turns
rule: After each added paste turn of viola verify, the rows the paste guard reads come only from a quiet screen holding a compiled literal, a poisoned screen, or the probe deadline; a quiet screen with no literal keeps waiting.
why: Measured: after a long paste the CLI footer holds no input-box literal for about 8 s.
@row_check n=543 row=decision/verify-box-wait-for-literal sha=e65987960812 v=holds judge=sonnet head=8834195b0683
at: src/cmd/verify/typed/pty.rs:167
quote: if feed.screen.is_poisoned() {
at: src/cmd/verify/typed/pty.rs:178-183
quote: rows.iter() .any(|r| SIGNATURES.holds_any(r)) .then_some(Settled { at: quiet_at, rows: Some(rows), })
at: src/cmd/verify/typed/run.rs:152
quote: from + PROBE_DEADLINE,
@row_add n=544 row=decision/verify-types-nothing-into-dialogs area=established-decisions rec=fct-qwpklckh@2
name: Verify never types into a CLI dialog
rule: No viola verify run writes a byte into a CLI dialog, and a start showing a modal is killed with no key.
why: Ratified rulings.
@row_check n=544 row=decision/verify-types-nothing-into-dialogs sha=c5c06377486b v=holds judge=sonnet head=8834195b0683
at: src/cmd/verify/typed/runs.rs:36-37
quote: typed.modal = run.settle(run.spawned_at).and_then(|s| s.rows); run.end_by_kill();
at: src/cmd/verify/typed/runs.rs:66-67
quote: if !input_box_up(typed.ready.as_deref()) { run.end_by_kill();
at: src/cmd/verify/typed/runs.rs:163
quote: if run.input_box_settles() && run.paste_text(PLAN_PROMPT) {
@row_add n=545 row=decision/verify-plan-files-in-run-dir area=established-decisions rec=fct-qwpklckh@2
name: Plan-mode probe keeps plan files in its own dir
rule: The plan-mode verify run points plansDirectory at a 0700 plans dir inside its own 0700 run dir.
why: So the plan file lands inside the run's own dir and none under the user's own plans directory.
@row_check n=545 row=decision/verify-plan-files-in-run-dir sha=b360c2ecb9e7 v=holds judge=sonnet head=8834195b0683
at: src/cmd/verify/typed/runs.rs:154-155
quote: let plans = dir.0.join("plans"); create_private_dir(&plans)?;
at: src/cmd/verify/typed/runs.rs:158
quote: args.push(ledger::plan_settings(&plans.to_string_lossy()).into());
at: src/cmd/verify/typed/runs.rs:219
quote: create_private_dir(&guard.0)?;
@row_add n=546 row=contract/viola-verify-output/step-line area=established-decisions rec=fct-qwpklckh@2
boundary: viola verify stdout
item: step line
promise: One line per relied row, [NN/MM] <row id> <row words>  pass|fail, MM being the relied row count.
@row_check n=546 row=contract/viola-verify-output/step-line sha=e867a76b7860 v=holds judge=sonnet head=8834195b0683
at: src/cmd/verify.rs:494
quote: "[{n:02}/{total:02}] {} {}  {verdict}",
at: src/cmd/verify.rs:475
quote: let total = relied.len();
@row_add n=547 row=contract/viola-verify-output/summary-line area=established-decisions rec=fct-qwpklckh@2
boundary: viola verify stdout
item: summary line
promise: stamped <ver>  N pass  M fail.
@row_check n=547 row=contract/viola-verify-output/summary-line sha=df8b4f702960 v=holds judge=sonnet head=8834195b0683
at: src/cmd/verify.rs:294
quote: "stamped {version}  {} pass  {failed} fail",
@row_add n=548 row=decision/verify-record-only-clean area=established-decisions rec=fct-qwpklckh@2
name: Fixtures recorded only from a clean verify
rule: viola verify --record writes its scrubbed fixtures only when 0 rows fail, and an unclean recording is refused whole, naming the file and the check code, never the content.
@row_check n=548 row=decision/verify-record-only-clean sha=ac59567cf841 v=holds judge=sonnet head=8834195b0683
at: src/cmd/verify.rs:297-301
quote: if failed > 0 { return Ok(Ended::Exit(1)); } match &args.record { Some(dir) => record(dir, &version, &probes),
at: src/cmd/verify.rs:624-629
quote: fn refuse_recording(named: &str) -> Ended { human::refuse( &format!("a recorded fixture is not clean: {named}"), "record with a viola home under your user home", ); Ended::Refused("unclean-fixture")
@row_add n=549 row=decision/real-cli-probe-local-only area=established-decisions rec=fct-qwpklckh@2
name: Real CLI probes run only locally
rule: The real claude probe and --record run only locally; CI runs verify against the fake agent's print mode and its interactive modes.
@row_check n=549 row=decision/real-cli-probe-local-only sha=d354630c0078 v=holds judge=sonnet head=8834195b0683
at: tests/support/verify.rs:494
quote: args.extend(["--screens", "--dialogs", "--framing", "--trusted-root"].map(OsString::from));
@row_add n=550 row=contract/viola-verify-refusal-codes/program-not-found area=established-decisions rec=fct-jzpbbnum@2
boundary: viola verify process-exit refusal codes
item: program-not-found
promise: Written on the process-exit verify logs under a parsed VIOLA_NAME when the program is not found.
@row_check n=550 row=contract/viola-verify-refusal-codes/program-not-found sha=f4534111faca v=holds judge=sonnet head=8834195b0683
at: src/cmd/verify.rs:164
quote: return Ok(Ended::Refused("program-not-found"));
at: src/cmd/verify.rs:90-91
quote: if instance.is_some() { run::log_self_exit(code, detail);
@row_add n=551 row=contract/viola-verify-refusal-codes/pinned-hash-mismatch area=established-decisions rec=fct-jzpbbnum@2
boundary: viola verify process-exit refusal codes
item: pinned-hash-mismatch
promise: Written on the process-exit verify logs under a parsed VIOLA_NAME when the pinned copy's hash does not match.
@row_check n=551 row=contract/viola-verify-refusal-codes/pinned-hash-mismatch sha=b83d4b862945 v=holds judge=sonnet head=8834195b0683
at: src/cmd/verify.rs:171
quote: return Ok(Ended::Refused("pinned-hash-mismatch"));
at: src/cmd/verify.rs:90-91
quote: if instance.is_some() { run::log_self_exit(code, detail);
@row_add n=552 row=contract/viola-verify-refusal-codes/version-unreadable area=established-decisions rec=fct-jzpbbnum@2
boundary: viola verify process-exit refusal codes
item: version-unreadable
promise: Written on the process-exit verify logs under a parsed VIOLA_NAME when the CLI version cannot be read.
@row_check n=552 row=contract/viola-verify-refusal-codes/version-unreadable sha=ec1a4e067cee v=holds judge=sonnet head=8834195b0683
at: src/cmd/verify.rs:199
quote: return Ok(Ended::Refused("version-unreadable"));
at: src/cmd/verify.rs:90-91
quote: if instance.is_some() { run::log_self_exit(code, detail);
@row_add n=553 row=contract/viola-verify-refusal-codes/strict-modes-failed area=established-decisions rec=fct-jzpbbnum@2
boundary: viola verify process-exit refusal codes
item: strict-modes-failed
promise: Written on the process-exit verify logs under a parsed VIOLA_NAME when a strict-modes check refuses; a refused home tree writes no line.
@row_check n=553 row=contract/viola-verify-refusal-codes/strict-modes-failed sha=da9676cba29c v=holds judge=sonnet head=8834195b0683
at: src/cmd/verify.rs:288
quote: return Ok(Ended::Refused("strict-modes-failed"));
at: src/cmd/verify.rs:67-70
quote: if check_home_tree(&home, None, None).is_err() { refuse_home(); return Ok(ExitCode::from(1)); }
@row_add n=554 row=contract/viola-verify-refusal-codes/unclean-fixture area=established-decisions rec=fct-jzpbbnum@2
boundary: viola verify process-exit refusal codes
item: unclean-fixture
promise: Written on the process-exit verify logs under a parsed VIOLA_NAME when a recording is unclean; the fixture's file enters no code.
@row_check n=554 row=contract/viola-verify-refusal-codes/unclean-fixture sha=136e0cffca29 v=holds judge=sonnet head=8834195b0683
at: src/cmd/verify.rs:629
quote: Ended::Refused("unclean-fixture")
at: src/cmd/verify.rs:90-91
quote: if instance.is_some() { run::log_self_exit(code, detail);
@row_add n=555 row=contract/viola-verify-refusal-codes/interrupted area=established-decisions rec=fct-jzpbbnum@2
boundary: viola verify process-exit refusal codes
item: interrupted
promise: Written on the process-exit verify logs under a parsed VIOLA_NAME when verify is interrupted.
@row_check n=555 row=contract/viola-verify-refusal-codes/interrupted sha=1968849462c2 v=holds judge=sonnet head=8834195b0683
at: src/cmd/verify.rs:87
quote: (1, Some("interrupted"))
at: src/cmd/verify.rs:90-91
quote: if instance.is_some() { run::log_self_exit(code, detail);
@row_add n=556 row=decision/session-liveness-heartbeat-and-start-time area=established-decisions rec=fct-44n3w6o5@2
name: Heartbeat primary, pid plus start time decide gone
rule: The heartbeat file is the primary liveness signal; a snapshot whose pid is dead, or alive with another start time, is gone whatever the heartbeat says; claude agents --json only enriches with idle/busy status and unwrapped read-only rows.
why: The prototype's heartbeat proved reliable; a bare pid is unsafe because pids get reused, so start time is matched too.
@row_check n=556 row=decision/session-liveness-heartbeat-and-start-time sha=683b42e9730b v=holds judge=sonnet head=8834195b0683
at: crates/viola-state/src/liveness.rs:28-29
quote: if !same_process { return Liveness::Gone;
at: crates/viola-state/src/liveness.rs:32
quote: Some(age) if age <= STALE_AFTER => Liveness::Live,
at: crates/viola-state/src/board.rs:334
quote: item.status = observations[at].status;
@row_add n=557 row=decision/run-refuses-held-name area=established-decisions rec=fct-44n3w6o5@2
name: A start refuses a held name
rule: viola run and viola revive refuse a name that is live or stale, or whose endpoint answers, with exit 1 and a message naming the instance.
@row_check n=557 row=decision/run-refuses-held-name sha=ebb9fca17a8e v=holds judge=sonnet head=8834195b0683
at: src/cmd/run.rs:454-455
quote: Liveness::Live => Some(refused_with(json, "already-live", || refuse_live(name))), Liveness::Stale => Some(refused_with(json, "stale-heartbeat", || {
at: src/cmd/run.rs:295
quote: return Ok(refused_with(launch.json, "squatted-name", || {
at: src/cmd/revive.rs:212
quote: || collision_check(name, &instance_dir, read.as_ref().ok(), json),
@row_add n=558 row=decision/revive-refuses-live-session area=established-decisions rec=fct-44n3w6o5@2
name: Revive refuses a session a live claude holds
rule: viola revive refuses when a claude agents --json row carries the logged session id about to be resumed; --fork skips the reading, and a failed read passes (fail open).
why: A second claude --resume of a held id is neither refused nor copied by the CLI: both sessions run under one id, two writers on one transcript.
@row_check n=558 row=decision/revive-refuses-live-session sha=02821de8fee2 v=holds judge=sonnet head=8834195b0683
at: src/cmd/revive.rs:223
quote: |(id, _)| (!fork && live_elsewhere(&home, &cwd, id)).then(|| refuse(Refusal::SessionLive)),
at: src/cmd/revive.rs:193
quote: .is_ok_and(|list| list.rows.iter().any(|row| row.session_id == id))
@row_add n=559 row=decision/revive-resumes-in-place area=established-decisions rec=fct-44n3w6o5@2
name: Revive resumes a gone instance in place
rule: viola revive appends to the gone instance's events log with the earlier bytes unchanged, replays no launch, and its first record is wheel with holder driver and cause start, as every start's.
why: Ratified: a revived start keeps the wheel at driver.
@row_check n=559 row=decision/revive-resumes-in-place sha=6dedfde2075f v=holds judge=sonnet head=8834195b0683
at: src/cmd/run.rs:619-620
quote: EventKind::Wheel, json!({"holder": Wheel::Driver.as_str(), "cause": WheelCause::Start.as_str()}),
at: src/cmd/run.rs:625
quote: append_event(instance_dir, &line)?;
@row_add n=560 row=decision/run-start-order area=established-decisions rec=fct-44n3w6o5@2
name: Child spawned last in the start order
rule: A start resolves the program, checks collision, pins the copy and plugin, gates the version, binds the endpoint, evaluates the budget gate, writes the first snapshot, heartbeat and start events, and only then spawns the child.
why: So the child's first SessionStart hook finds a listening endpoint and a recorded snapshot.
@row_check n=560 row=decision/run-start-order sha=be2640cbb226 v=holds judge=sonnet head=8834195b0683
at: src/cmd/run.rs:268
quote: let program = match run::resolve_program(&launch.program, &cwd) {
at: src/cmd/run.rs:278
quote: if let Some(refusal) = collision_check(name, &instance_dir, recovered.as_ref(), launch.json) {
at: src/cmd/run.rs:281
quote: let Some((pinned, plugin_dir)) = pin_and_plugin(&home, exe)? else {
at: src/cmd/run.rs:290
quote: let gate = version_gate(&home, &program, &cwd, &strip);
at: src/cmd/run.rs:292
quote: let (endpoint, server) = match bind_endpoint(name, &home)? {
at: src/cmd/run.rs:366
quote: let (beat, snapshot) = start_state(
at: src/cmd/run.rs:390
quote: let spawned = spawn_child(
@row_add n=561 row=decision/exclusive-endpoint-bind area=established-decisions rec=fct-44n3w6o5@2
name: Exclusive endpoint bind
rule: A start binds its endpoint exclusively, and a bind that finds the name taken is an exit-1 refusal, squatted-name.
why: So two concurrent starts cannot both win.
@row_check n=561 row=decision/exclusive-endpoint-bind sha=a1c8d0e70f6d v=holds judge=sonnet head=8834195b0683
at: src/cmd/run.rs:500-502
quote: match Server::bind(&endpoint) { Ok(server) => Ok(Bound::Serving(endpoint, server)), Err(ChannelError::BindTaken) => Ok(Bound::Taken),
at: src/cmd/run.rs:294-295
quote: Bound::Taken => { return Ok(refused_with(launch.json, "squatted-name", || {
@row_add n=562 row=decision/frame-client-verifies-server area=established-decisions rec=fct-uzcv7hop@2,fct-uqyfijk4@2
name: Frame clients verify the serving wrapper
rule: Before its first frame every frame-writing client checks the recorded endpoint's directory, admits the connected peer's euid, and compares the peer's pid and OS start time with the snapshot's; a mismatch or a check that cannot be made sends no frame.
@row_check n=562 row=decision/frame-client-verifies-server sha=75a2b3c0fdff v=holds judge=sonnet head=8834195b0683
at: src/cmd/client.rs:173
quote: if check_endpoint_dir(&live.endpoint).is_err() {
at: src/cmd/client.rs:177
quote: if !peer_admitted(client.server_euid()) {
at: src/cmd/client.rs:180
quote: let verdict = verify_server(live.pid, &live.started_at, client.server_pid());
at: src/cmd/hook.rs:231
quote: if check_endpoint_dir(endpoint).is_err() {
at: src/cmd/hook.rs:236
quote: if !peer_admitted(client.server_euid()) {
at: src/cmd/hook.rs:239
quote: let verdict = verify_server(snapshot.pid, &snapshot.started_at, client.server_pid());
at: crates/viola-mcp/src/tools.rs:345
quote: if check_endpoint_dir(&live.endpoint).is_err() {
at: crates/viola-mcp/src/tools.rs:352
quote: if !peer_admitted(client.server_euid()) || !server_verified(live, client.server_pid()).await? {
@row_add n=563 row=decision/pty-error-hand-written area=established-decisions rec=fct-xqkfyxga@3
name: Hand-written PTY error type
rule: The PTY crate's error has a hand-written Display and Error with fixed messages, keeping the portable-pty anyhow error only as a boxed source.
why: The seam's normal dependencies are exactly portable-pty, tracing and libc; an exception to the thiserror rule.
@row_check n=563 row=decision/pty-error-hand-written sha=57b494205731 v=holds judge=sonnet head=8834195b0683
at: crates/viola-pty/src/lib.rs:81
quote: impl fmt::Display for PtyError {
at: crates/viola-pty/src/lib.rs:96
quote: impl std::error::Error for PtyError {
at: crates/viola-pty/src/lib.rs:70
quote: Open(Box<dyn std::error::Error + Send + Sync>),
@row_add n=564 row=decision/anyhow-only-in-bin area=established-decisions rec=fct-xqkfyxga@3
name: anyhow only in the viola binary
rule: anyhow is used only in the viola bin: its main, dispatch and the catch-site internal-error reporter.
why: anyhow's context chains are only useful where a person reads them.
@row_check n=564 row=decision/anyhow-only-in-bin sha=813668c8e7e4 v=holds judge=sonnet head=8834195b0683
at: Cargo.toml:153
quote: anyhow.workspace = true
at: src/obs.rs:256
quote: pub(crate) fn report_internal_error(error: &anyhow::Error, sink: Option<&DetailSink>) {
@row_add n=565 row=decision/error-chain-to-diagnostics-only area=established-decisions rec=fct-xqkfyxga@3
name: Error chains reach only the post-mortem detail file
rule: An anyhow error chain is written as chain to the instance's diagnostics detail file for the owner's post-mortem, never to a role line, stdout or stderr.
why: The owner's post-mortem is the one place a person reads the chain.
@row_check n=565 row=decision/error-chain-to-diagnostics-only sha=f15a406426ab v=holds judge=sonnet head=8834195b0683
at: src/obs.rs:259-260
quote: let line = chain_detail_line(&timestamp(Utc::now()), sink.process, &sink.instance, error); write_detail(&sink.home, &sink.instance, sink.process, &line);
@row_add n=566 row=decision/cargo-workspace-flat-crates area=established-decisions rec=fct-yjcz4wzl@1
name: Cargo workspace with flat crates
rule: The project is a Cargo workspace with a flat crates layout, the root manifest being both the viola bin package and the workspace; each product crate is created by the first piece of work that consumes it.
why: The compiler enforces that viola-pty cannot see viola-agent-claude; cargo install --path . requires the bin at the root.
@row_check n=566 row=decision/cargo-workspace-flat-crates sha=6eb79fe626a2 v=holds judge=sonnet head=8834195b0683
at: Cargo.toml:180-181
quote: [workspace] members = ["crates/*"]
@row_add n=567 row=decision/fuzz-own-workspace area=established-decisions rec=fct-yjcz4wzl@1
name: Fuzz targets in their own workspace
rule: The cargo-fuzz targets are their own cargo workspace, excluded from the root workspace.
why: So their nightly toolchain and C++-building libfuzzer-sys never enter the root graph.
@row_check n=567 row=decision/fuzz-own-workspace sha=e91f5e074fa9 v=holds judge=sonnet head=8834195b0683
at: Cargo.toml:182
quote: exclude = ["fuzz"]
at: fuzz/Cargo.toml:20
quote: [workspace]
@row_add n=568 row=decision/tokio-only-in-mcp-crate area=established-decisions rec=fct-yjcz4wzl@1
name: tokio only in the MCP crate
rule: Among the product crates only viola-mcp enables tokio; viola-channel lists it optional behind its tokio feature, and no other product member's normal graph reaches tokio.
guard: scripts/channel-tokio-free.sh
@row_check n=568 row=decision/tokio-only-in-mcp-crate sha=da4e2f469539 v=holds guard=true judge=sonnet head=8834195b0683
at: crates/viola-mcp/Cargo.toml:23
quote: tokio = { workspace = true, features = ["rt"] }
at: crates/viola-channel/Cargo.toml:23
quote: tokio = { workspace = true, optional = true, features = ["io-util"] }
guard_at: scripts/channel-tokio-free.sh:1
@row_add n=569 row=decision/no-cargo-pup area=established-decisions rec=fct-yjcz4wzl@1
name: cargo_pup not taken
rule: Module boundaries are enforced by crate separation, not by cargo_pup.
why: cargo_pup needs nightly and was rejected.
@row_check n=569 row=decision/no-cargo-pup sha=bc6254acb18b v=holds judge=sonnet head=8834195b0683
at: Cargo.toml:181
quote: members = ["crates/*"]
searched: cargo_pup|cargo-pup · in **/*.toml, **/*.yml, **/*.sh, **/*.rs
@row_add n=570 row=decision/pinned-copy-rehash area=established-decisions rec=fct-23yj6ah3@2
name: Pinned copy re-hashed before reuse
rule: An existing pinned copy is re-hashed before reuse; a copy that no longer matches refuses the start with exit 1, pinned-hash-mismatch, and is left as found.
@row_check n=570 row=decision/pinned-copy-rehash sha=b3354056a5a0 v=holds judge=sonnet head=8834195b0683
at: crates/viola-state/src/pin.rs:90
quote: Ok(_) => return Err(PinError::HashMismatch),
at: src/cmd/run.rs:284
quote: "pinned-hash-mismatch",
@row_add n=571 row=decision/hooks-and-mcp-never-search-path area=established-decisions rec=fct-23yj6ah3@2
name: Hooks and MCP never search PATH
rule: Hook and MCP commands are the pinned copy's absolute path substituted at start, never a PATH lookup.
why: PATH-found hooks run a different binary from the wrapper after a rebuild.
@row_check n=571 row=decision/hooks-and-mcp-never-search-path sha=a9135c27cf6e v=holds judge=sonnet head=8834195b0683
at: plugin/hooks/hooks.json:4
quote: {"hooks": [{"type": "command", "command": "@@VIOLA_BIN@@", "args": ["hook", "session-start"], "timeout": 5}]}
at: crates/viola-agent-claude/src/lib.rs:151
quote: .replace(BIN_TOKEN, pinned_bin_fwd)
@row_add n=572 row=decision/frame-version-safety-net area=established-decisions rec=fct-23yj6ah3@2
name: Per-frame version as second safety net
rule: Every channel frame carries v and the sender's version, as the second safety net for the user-level install, which is not bound to a wrapper's pinned copy.
@row_check n=572 row=decision/frame-version-safety-net sha=fc723329605d v=holds judge=sonnet head=8834195b0683
at: crates/viola-channel/src/client.rs:115
quote: params.insert("sender".to_owned(), VERSION.into());
at: crates/viola-channel/src/client.rs:174
quote: params.insert("v".to_owned(), PROTOCOL_V.into());
@row_add n=573 row=contract/plugin-mcp-json/viola area=established-decisions rec=fct-23yj6ah3@2
boundary: The plugin's .mcp.json
item: viola
promise: Registers the server viola with command the pinned copy's absolute path and args ["mcp"].
@row_check n=573 row=contract/plugin-mcp-json/viola sha=67b8ca4b71f5 v=holds judge=sonnet head=8834195b0683
at: plugin/.mcp.json:4-5
quote: "command": "@@VIOLA_BIN@@", "args": ["mcp"]
@row_add n=574 row=decision/installed-plugin-mcp-only area=established-decisions rec=fct-4jk25cex@2
name: Installed plugin carries MCP only
rule: viola plugin install is optional, for the MCP verbs in an unwrapped driver; the installed plugin carries only .mcp.json and no hooks, its command the absolute path of the installing exe.
why: So a wrapped session that also has it installed never runs two hook processes for one event.
@row_check n=574 row=decision/installed-plugin-mcp-only sha=9086a87592b2 v=holds judge=sonnet head=8834195b0683
at: crates/viola-agent-claude/src/marketplace.rs:45
quote: NAME: {"command": exe_fwd, "args": ["--home", home_fwd, "mcp"]},
at: crates/viola-agent-claude/src/marketplace.rs:51
quote: ("viola/.mcp.json", document(&mcp)),
@row_add n=575 row=decision/plugin-precedence-no-ledger-row area=established-decisions rec=fct-4jk25cex@2
name: Plugin precedence is no ledger row
rule: Plugin precedence between the installed and the --plugin-dir plugin is not a capability-ledger row.
why: No send relies on it, the ruled limit.
@row_check n=575 row=decision/plugin-precedence-no-ledger-row sha=7f01b03d1701 v=holds judge=sonnet head=8834195b0683
searched: precedence · in crates/viola-agent-claude/src/ledger.rs
searched: LedgerRow enum (24 members, none for plugin precedence) · in crates/viola-agent-claude/src/ledger.rs:20-45
@row_add n=576 row=decision/verify-transcripts-accepted-residual area=established-decisions rec=fct-4jk25cex@2
name: Verify's CLI transcripts are an accepted residual
rule: The CLI session transcripts and empty project dirs viola verify's runs leave under the user's Claude projects directory are an accepted residual.
@row_check n=576 row=decision/verify-transcripts-accepted-residual sha=7421d42bb2b0 v=unverifiable judge=sonnet head=8834195b0683
why: not a code claim
@row_add n=577 row=decision/cli-human-text-json-exit-codes area=established-decisions rec=fct-onw4pywk@3
name: Human text by default, --json for agents
rule: CLI verbs print human text by default and a machine document under --json, with a typed exit code per refusal; without --json the human output is unchanged.
why: Agents need machine-parseable refusals.
@row_check n=577 row=decision/cli-human-text-json-exit-codes sha=0202c87259bb v=holds judge=sonnet head=8834195b0683
at: src/cmd/client.rs:227
quote: document(&json!({"v": 1, "error": code, "detail": null}));
at: src/cmd/run.rs:418-422
quote: fn refused_with(json: bool, detail: &'static str, human: impl FnOnce()) -> Started { if json { super::client::refused_document(detail); } else { human();
@row_add n=578 row=contract/viola-revive-json/list-document area=established-decisions rec=fct-onw4pywk@3
boundary: viola revive --json
item: --list document
promise: <name> --list --json prints {"v":1,"ok":{"sessions":[…]}} in log order.
@row_check n=578 row=contract/viola-revive-json/list-document sha=65011b7bbaa9 v=holds judge=sonnet head=8834195b0683
at: src/cmd/revive.rs:292
quote: client::document(&json!({"v": 1, "ok": {"sessions": sessions}}));
@row_add n=579 row=contract/viola-revive-json-error-codes/strict-modes-failed area=established-decisions rec=fct-onw4pywk@3
boundary: viola revive --json error codes
item: strict-modes-failed
promise: A strict-modes check refused the start.
@row_check n=579 row=contract/viola-revive-json-error-codes/strict-modes-failed sha=3a6cf9122a29 v=holds judge=sonnet head=8834195b0683
at: src/cmd/revive.rs:70
quote: Self::StrictModes => "strict-modes-failed",
@row_add n=580 row=contract/viola-revive-json-error-codes/already-live area=established-decisions rec=fct-onw4pywk@3
boundary: viola revive --json error codes
item: already-live
promise: The name is held by a live instance.
@row_check n=580 row=contract/viola-revive-json-error-codes/already-live sha=6bbadc2f111e v=holds judge=sonnet head=8834195b0683
at: src/cmd/run.rs:454
quote: Liveness::Live => Some(refused_with(json, "already-live", || refuse_live(name))),
@row_add n=581 row=contract/viola-revive-json-error-codes/no-session area=established-decisions rec=fct-onw4pywk@3
boundary: viola revive --json error codes
item: no-session
promise: No logged session to revive.
@row_check n=581 row=contract/viola-revive-json-error-codes/no-session sha=b8e7d450c779 v=holds judge=sonnet head=8834195b0683
at: src/cmd/revive.rs:71
quote: Self::NoSession | Self::UnknownId => "no-session",
@row_add n=582 row=contract/viola-revive-json-error-codes/session-live area=established-decisions rec=fct-onw4pywk@3
boundary: viola revive --json error codes
item: session-live
promise: The logged session is held by a live claude.
@row_check n=582 row=contract/viola-revive-json-error-codes/session-live sha=42d8041dad18 v=holds judge=sonnet head=8834195b0683
at: src/cmd/revive.rs:72
quote: Self::SessionLive => "session-live",
@row_add n=583 row=contract/viola-revive-json-error-codes/cwd-missing area=established-decisions rec=fct-onw4pywk@3
boundary: viola revive --json error codes
item: cwd-missing
promise: The recorded directory is missing.
@row_check n=583 row=contract/viola-revive-json-error-codes/cwd-missing sha=30f32b1e75d2 v=holds judge=sonnet head=8834195b0683
at: src/cmd/revive.rs:73
quote: Self::CwdMissing => "cwd-missing",
@row_add n=584 row=contract/viola-revive-json-error-codes/stale-heartbeat area=established-decisions rec=fct-onw4pywk@3
boundary: viola revive --json error codes
item: stale-heartbeat
promise: The name is held by a stale instance.
@row_check n=584 row=contract/viola-revive-json-error-codes/stale-heartbeat sha=a41b31317f19 v=holds judge=sonnet head=8834195b0683
at: src/cmd/run.rs:455
quote: Liveness::Stale => Some(refused_with(json, "stale-heartbeat", || {
@row_add n=585 row=contract/viola-revive-json-error-codes/pinned-hash-mismatch area=established-decisions rec=fct-onw4pywk@3
boundary: viola revive --json error codes
item: pinned-hash-mismatch
promise: The pinned copy no longer matches its hash.
@row_check n=585 row=contract/viola-revive-json-error-codes/pinned-hash-mismatch sha=d0949ddbb1bc v=holds judge=sonnet head=8834195b0683
at: src/cmd/run.rs:284
quote: "pinned-hash-mismatch",
@row_add n=586 row=contract/viola-revive-json-error-codes/squatted-name area=established-decisions rec=fct-onw4pywk@3
boundary: viola revive --json error codes
item: squatted-name
promise: The endpoint name is already taken at bind.
@row_check n=586 row=contract/viola-revive-json-error-codes/squatted-name sha=aaff1cdc70cf v=holds judge=sonnet head=8834195b0683
at: src/cmd/run.rs:295
quote: return Ok(refused_with(launch.json, "squatted-name", || {
@row_add n=587 row=contract/viola-revive-json-error-codes/socket-dir-refused area=established-decisions rec=fct-onw4pywk@3
boundary: viola revive --json error codes
item: socket-dir-refused
promise: The socket directory was refused or could not be created.
@row_check n=587 row=contract/viola-revive-json-error-codes/socket-dir-refused sha=3e434239f3e0 v=holds judge=sonnet head=8834195b0683
at: src/cmd/run.rs:300
quote: return Ok(refused_with(launch.json, "socket-dir-refused", || {
@row_add n=588 row=contract/viola-revive-json-error-codes/internal-error area=established-decisions rec=fct-onw4pywk@3
boundary: viola revive --json error codes
item: internal-error
promise: A claude not on PATH or an anyhow error of the start.
@row_check n=588 row=contract/viola-revive-json-error-codes/internal-error sha=7735191af914 v=holds judge=sonnet head=8834195b0683
at: src/cmd/run.rs:270
quote: Err(_) => return Ok(refused_with(launch.json, "internal-error", || {})),
at: src/cmd/revive.rs:124
quote: client::refused_document("internal-error");
@row_add n=589 row=absence/windows-and-macos-support area=established-decisions rec=fct-hss5qy6y@1
capability: Support for Windows or macOS
@row_check n=589 row=absence/windows-and-macos-support sha=928a3120d033 v=holds judge=sonnet head=8834195b0683
searched: windows-latest|macos-latest · in .github/workflows
searched: cfg(windows)|windows-sys|winapi · in src, crates, tests, fuzz
@row_add n=590 row=decision/window-features-target-hyprland area=established-decisions rec=fct-hss5qy6y@1
name: Window features target Hyprland
rule: The window features target the Hyprland compositor.
@row_check n=590 row=decision/window-features-target-hyprland sha=1df32f797725 v=unverifiable judge=sonnet head=8834195b0683
searched: (?i)hyprland|hyprctl · in src, crates, tests, .github, scripts, Cargo.toml
why: no evidence found
@row_add n=591 row=decision/socket-in-per-user-dir area=occupied-resources rec=fct-nm6raycf@2
name: Endpoint socket in a per-user 0700 directory
rule: The Linux endpoint is a Unix socket in a per-user 0700 directory, the runtime dir's viola folder with a per-uid /tmp folder only as fallback, never the /tmp root; a symlink, non-directory, foreign owner or group/other bit is refused and left as found.
@row_check n=591 row=decision/socket-in-per-user-dir sha=36b54cbb8c74 v=holds judge=sonnet head=8834195b0683 masked=true
at: crates/viola-channel/src/endpoint.rs:39-40
quote: Some(xdg) => Path::new(xdg).join("viola"), None => PathBuf::from(format!("{host path}{uid}")),
at: crates/viola-state/src/strict.rs:144-150
quote: socket_dir_verdict( meta.file_type().is_symlink(), meta.is_dir(), meta.mode(), meta.uid(), unix::euid(), )
@row_add n=592 row=decision/stale-socket-file-left area=occupied-resources rec=fct-uufx66yt@1
name: Stale socket file left in place
rule: A force-killed wrapper's socket file stays in the per-user directory until a next bind of that name takes it over; nothing removes it.
why: A choice: a harmless file in a 0700 directory.
@row_check n=592 row=decision/stale-socket-file-left sha=79a8006012ea v=holds judge=sonnet head=8834195b0683
at: crates/viola-channel/src/server.rs:117-118
quote: // A leftover that cannot be removed fails the bind below; that failure is the one reported. let _ = fs::remove_file(&socket);
@row_add n=593 row=decision/mcp-subcommand-hidden area=occupied-resources rec=fct-mnor3ks3@2
name: Hidden mcp subcommand
rule: The viola mcp subcommand is hidden from viola --help and takes no argument but the global --home.
why: The plugin runs it, never a person.
@row_check n=593 row=decision/mcp-subcommand-hidden sha=9e8bd88ca77b v=holds judge=sonnet head=8834195b0683
at: src/cmd/mod.rs:94-96
quote: /// Serve the MCP tools to a Claude Code session (run by the plugin, never by a person) #[command(hide = true)] Mcp,
@row_add n=594 row=contract/viola-mcp-exit-codes/0 area=occupied-resources rec=fct-mnor3ks3@2
boundary: viola mcp exit codes
item: 0
promise: The session ended by stdin closing, or by a stdin line past MAX_FRAME, once initialized; both streams empty.
@row_check n=594 row=contract/viola-mcp-exit-codes/0 sha=d4c85ede829e v=holds judge=sonnet head=8834195b0683
at: crates/viola-mcp/src/lib.rs:77
quote: let stdin = stdin::CappedLines::new(stdin, MAX_FRAME);
at: crates/viola-mcp/src/lib.rs:83
quote: Ok(_) => Ok(()),
@row_add n=595 row=contract/viola-mcp-exit-codes/1 area=occupied-resources rec=fct-mnor3ks3@2
boundary: viola mcp exit codes
item: 1
promise: The session ended before initialize, or the home or diagnostics folder failed the strict-modes check, with nothing on stdout or stderr and no role file.
@row_check n=595 row=contract/viola-mcp-exit-codes/1 sha=f3a05fbf8384 v=holds judge=sonnet head=8834195b0683
at: src/cmd/mcp.rs:22-23
quote: if !home_passes(home) {         return Ok(ExitCode::from(1));
at: src/cmd/mcp.rs:46
quote: Ok(_) => viola_state::strict::check_path(path).is_ok(),
@row_add n=596 row=contract/viola-cli-arguments/verify area=occupied-resources rec=fct-mnor3ks3@2
boundary: viola CLI arguments
item: verify
promise: viola verify takes [--record <DIR>] [-- <program> [args…]], the program defaulting to claude.
@row_check n=596 row=contract/viola-cli-arguments/verify sha=4cbc48d95714 v=holds judge=sonnet head=8834195b0683
at: src/cmd/verify.rs:48-49
quote: #[arg(long, value_name = "DIR")] record: Option<PathBuf>,
at: src/cmd/verify.rs:51-52
quote: #[arg(last = true)] program: Vec<OsString>,
at: src/cmd/verify.rs:146
quote: None => (OsString::from(PROGRAM), Vec::new()),
@row_add n=597 row=contract/viola-cli-arguments/list area=occupied-resources rec=fct-mnor3ks3@2
boundary: viola CLI arguments
item: list
promise: viola list takes [--json] and nothing else.
@row_check n=597 row=contract/viola-cli-arguments/list sha=cc151361b327 v=holds judge=sonnet head=8834195b0683
at: src/cmd/list.rs:29-30
quote: #[arg(long)] json: bool,
@row_add n=598 row=contract/viola-cli-arguments/plugin-install area=occupied-resources rec=fct-mnor3ks3@2
boundary: viola CLI arguments
item: plugin install
promise: viola plugin install takes [--json] and nothing else; viola plugin alone is a usage error.
@row_check n=598 row=contract/viola-cli-arguments/plugin-install sha=2a3105ad9656 v=holds judge=sonnet head=8834195b0683
at: src/cmd/plugin.rs:33
quote: Install(InstallArgs),
at: src/cmd/plugin.rs:39-40
quote: #[arg(long)] json: bool,
@row_add n=599 row=contract/viola-cli-arguments/revive area=occupied-resources rec=fct-mnor3ks3@2
boundary: viola CLI arguments
item: revive
promise: viola revive takes <name> [--id <ID>] [--fork] [-- <child args>], or <name> --list, each with [--json].
@row_check n=599 row=contract/viola-cli-arguments/revive sha=5da8aabb58ec v=holds judge=sonnet head=8834195b0683
at: src/cmd/revive.rs:28
quote: pub(super) name: ViolaName,
at: src/cmd/revive.rs:30-31
quote: #[arg(long, value_name = "ID", value_parser = parse_session_id)] id: Option<String>,
at: src/cmd/revive.rs:33-34
quote: #[arg(long)] fork: bool,
at: src/cmd/revive.rs:36-37
quote: #[arg(long, conflicts_with_all = ["id", "fork", "extra"])] pub(super) list: bool,
at: src/cmd/revive.rs:39-40
quote: #[arg(long)] json: bool,
at: src/cmd/revive.rs:42-43
quote: #[arg(last = true)] extra: Vec<OsString>,
@row_add n=600 row=contract/viola-list-exit-codes/0 area=occupied-resources rec=fct-mnor3ks3@2
boundary: viola list exit codes
item: 0
promise: The board was printed in one stdout write.
@row_check n=600 row=contract/viola-list-exit-codes/0 sha=d5f63112e019 v=holds judge=sonnet head=8834195b0683
at: src/cmd/list.rs:55
quote: client::document(&document(&board, budget.as_ref()));
at: src/cmd/client.rs:221
quote: let _ = io::stdout().lock().write_all(format!("{doc}\n").as_bytes());
at: src/cmd/list.rs:75
quote: Ok(ExitCode::SUCCESS)
@row_add n=601 row=contract/viola-list-exit-codes/21 area=occupied-resources rec=fct-mnor3ks3@2
boundary: viola list exit codes
item: 21
promise: The strict-modes check refused the home, instances/ or any one instance, and the whole board is refused.
@row_check n=601 row=contract/viola-list-exit-codes/21 sha=9e9f9081958e v=holds judge=sonnet head=8834195b0683
at: src/cmd/list.rs:43-47
quote: let scan = match board::scan(&home)? { Ok(scan) => scan, Err(refused) => { refuse(args.json, &refused); return Ok(ExitCode::from(21));
at: crates/viola-state/src/board.rs:59-60
quote: if check_if_present(home, check_read).is_err() || check_if_present(&instances_dir, check_path).is_err()
at: crates/viola-state/src/board.rs:97
quote: if check_instance(home, dir).is_err() {
@row_add n=602 row=contract/viola-list-exit-codes/1 area=occupied-resources rec=fct-mnor3ks3@2
boundary: viola list exit codes
item: 1
promise: An internal error, through the cli catch site.
@row_check n=602 row=contract/viola-list-exit-codes/1 sha=c9e6b05e09a8 v=holds judge=sonnet head=8834195b0683
at: src/main.rs:57-58
quote: obs::report_internal_error(&failure.error, failure.sink.as_ref()); Outcome::Failed
at: src/main.rs:134-138
quote: (Role::Mcp | Role::Cli | Role::Other, Outcome::Done(code)) => code, ( Role::Mcp | Role::Cli | Role::Other, Outcome::Unparsed | Outcome::Failed | Outcome::Panicked, ) => ExitCode::from(1),
@row_add n=603 row=contract/viola-list-exit-codes/2 area=occupied-resources rec=fct-mnor3ks3@2
boundary: viola list exit codes
item: 2
promise: A clap usage error.
@row_check n=603 row=contract/viola-list-exit-codes/2 sha=761d51ad2f8e v=holds judge=sonnet head=8834195b0683
at: src/main.rs:52
quote: Err(usage) => usage.exit(),
@row_add n=604 row=contract/viola-plugin-install-exit-codes/0 area=occupied-resources rec=fct-mnor3ks3@2
boundary: viola plugin install exit codes
item: 0
promise: The result was printed in one stdout write.
@row_check n=604 row=contract/viola-plugin-install-exit-codes/0 sha=22c4ab9209e1 v=holds judge=sonnet head=8834195b0683
at: src/cmd/plugin.rs:73
quote: let _ = human::write_plugin_installed(&mut io::stdout().lock(), &word);
at: src/cmd/plugin.rs:76
quote: Ok(ExitCode::SUCCESS)
@row_add n=605 row=contract/viola-plugin-install-exit-codes/21 area=occupied-resources rec=fct-mnor3ks3@2
boundary: viola plugin install exit codes
item: 21
promise: The strict-modes check refused the home or an existing marketplace tree folder; nothing created, no log opened.
@row_check n=605 row=contract/viola-plugin-install-exit-codes/21 sha=930fbff6181c v=holds judge=sonnet head=8834195b0683
at: src/cmd/plugin.rs:53
quote: if !tree_passes(&home) {
at: src/cmd/plugin.rs:60
quote: return Ok(ExitCode::from(21));
at: src/cmd/plugin.rs:65
quote: let dir = write_tree(&home)?;
@row_add n=606 row=contract/viola-plugin-install-exit-codes/1 area=occupied-resources rec=fct-mnor3ks3@2
boundary: viola plugin install exit codes
item: 1
promise: An internal error, through the cli catch site.
@row_check n=606 row=contract/viola-plugin-install-exit-codes/1 sha=00a799ba20aa v=holds judge=sonnet head=8834195b0683
at: src/main.rs:57-58
quote: obs::report_internal_error(&failure.error, failure.sink.as_ref()); Outcome::Failed
at: src/main.rs:138
quote: ) => ExitCode::from(1),
@row_add n=607 row=contract/viola-plugin-install-exit-codes/2 area=occupied-resources rec=fct-mnor3ks3@2
boundary: viola plugin install exit codes
item: 2
promise: A clap usage error.
@row_check n=607 row=contract/viola-plugin-install-exit-codes/2 sha=28e8dc00741c v=holds judge=sonnet head=8834195b0683
at: src/main.rs:52
quote: Err(usage) => usage.exit(),
@row_add n=608 row=contract/viola-revive-exit-codes/0 area=occupied-resources rec=fct-mnor3ks3@2
boundary: viola revive exit codes
item: 0
promise: The revived child's wrapper ended as viola run's does.
@row_check n=608 row=contract/viola-revive-exit-codes/0 sha=6c3c7e4f5d67 v=holds judge=sonnet head=8834195b0683
at: src/cmd/revive.rs:128
quote: Started::Launched(launched) => pump_child(*launched),
at: src/cmd/run.rs:700-704
quote: match end? { PumpEnd::Exited(exit) => { run::log_child_exit(exit); run::log_self_exit(0, None); Ok(ExitCode::SUCCESS)
@row_add n=609 row=contract/viola-revive-exit-codes/1 area=occupied-resources rec=fct-mnor3ks3@2
boundary: viola revive exit codes
item: 1
promise: A refused strict-modes check, a preflight refusal, or a refusal of the start after a passed preflight.
@row_check n=609 row=contract/viola-revive-exit-codes/1 sha=376d1cc8c4b6 v=holds judge=sonnet head=8834195b0683
at: src/cmd/revive.rs:139
quote: return Ok(Started::Refused(ExitCode::from(1)));
at: src/cmd/revive.rs:223
quote: |(id, _)| (!fork && live_elsewhere(&home, &cwd, id)).then(|| refuse(Refusal::SessionLive)),
at: src/cmd/revive.rs:285
quote: return Ok(ExitCode::from(1));
@row_add n=610 row=contract/viola-revive-exit-codes/2 area=occupied-resources rec=fct-mnor3ks3@2
boundary: viola revive exit codes
item: 2
promise: A clap usage error, a malformed --id included.
@row_check n=610 row=contract/viola-revive-exit-codes/2 sha=5247948b22f3 v=holds judge=sonnet head=8834195b0683
at: src/cmd/revive.rs:30-31
quote: #[arg(long, value_name = "ID", value_parser = parse_session_id)] id: Option<String>,
at: src/main.rs:52
quote: Err(usage) => usage.exit(),
@row_add n=611 row=contract/viola-revive-json/list area=occupied-resources rec=fct-mnor3ks3@2
boundary: viola revive --json output
item: --list
promise: viola revive <name> --list --json prints the ok.sessions document, or the no-session document with exit 1.
@row_check n=611 row=contract/viola-revive-json/list sha=a26e4e87aa58 v=holds judge=sonnet head=8834195b0683
at: src/cmd/revive.rs:292
quote: client::document(&json!({"v": 1, "ok": {"sessions": sessions}}));
at: src/cmd/revive.rs:283-285
quote: Refusal::NoSession.write(name, json); crate::run::log_self_exit(1, Some(Refusal::NoSession.detail())); return Ok(ExitCode::from(1));
@row_add n=612 row=contract/viola-cli-arguments/link-unlink area=occupied-resources rec=fct-scvtx5bh@1
boundary: viola CLI arguments
item: link and unlink
promise: viola link and viola unlink each take <driver> <driven> [--json], the verb's instance being the driven one.
@row_check n=612 row=contract/viola-cli-arguments/link-unlink sha=2977510e0514 v=holds judge=sonnet head=8834195b0683
at: src/cmd/mod.rs:80
quote: Link(link::LinkArgs),
at: src/cmd/mod.rs:82
quote: Unlink(link::LinkArgs),
at: src/cmd/link.rs:24
quote: pub(super) driver: ViolaName,
at: src/cmd/link.rs:27
quote: pub(super) driven: ViolaName,
at: src/cmd/link.rs:30
quote: pub(super) json: bool,
at: src/cmd/mod.rs:192
quote: let sink = cli_sink(&home, &args.driven);
@row_add n=613 row=contract/viola-link-exit-codes/0 area=occupied-resources rec=fct-scvtx5bh@1
boundary: viola link and unlink exit codes
item: 0
promise: One result line, or under --json {"v":1,"ok":<ok>}, link's ok holding since and already_linked and unlink's ok empty.
@row_check n=613 row=contract/viola-link-exit-codes/0 sha=ef635426e353 v=holds judge=sonnet head=8834195b0683
at: src/cmd/link.rs:121
quote: client::document(&json!({"v": 1, "ok": ok}));
at: src/cmd/link.rs:48-49
quote: Verb::Link => ok["since"].is_string() && ok["already_linked"].is_boolean(), Verb::Unlink => ok.is_object(),
at: src/cmd/link.rs:126
quote: Ok(0)
@row_add n=614 row=contract/viola-link-exit-codes/21 area=occupied-resources rec=fct-scvtx5bh@1
boundary: viola link and unlink exit codes
item: 21
promise: The instance check refused, no live endpoint is recorded, the recorded endpoint's folder is refused, or the serving process is not the recorded wrapper or runs under another euid.
@row_check n=614 row=contract/viola-link-exit-codes/21 sha=3c17d75b639d v=holds judge=sonnet head=8834195b0683
at: src/cmd/link.rs:94
quote: if let Some(code) = client::refuse_untrusted(home, &args.driven, args.json)? {
at: src/cmd/link.rs:105
quote: return Ok(unreachable(Unreachable::without_endpoint(&instance_dir)));
at: src/cmd/client.rs:173-174
quote: if check_endpoint_dir(&live.endpoint).is_err() { return Err(CallError::SocketDirRefused);
at: src/cmd/client.rs:177-178
quote: if !peer_admitted(client.server_euid()) { return Err(CallError::ServerUnverified);
at: src/cmd/client.rs:183
quote: ServerVerdict::Mismatch | ServerVerdict::Unverifiable => Err(CallError::ServerUnverified),
at: src/cmd/client.rs:265
quote: 21
@row_add n=615 row=contract/viola-link-exit-codes/20 area=occupied-resources rec=fct-scvtx5bh@1
boundary: viola link and unlink exit codes
item: 20
promise: A wrapper error, a -32602 included, such as driver equal to driven.
@row_check n=615 row=contract/viola-link-exit-codes/20 sha=03e7cbfbef0f v=holds judge=sonnet head=8834195b0683
at: src/cmd/client.rs:244-245
quote: run::log_self_exit(20, Some("wrapper-fault")); 20
at: src/run/links.rs:95
quote: if driven != self.name || driver == driven {
at: src/cmd/link.rs:116
quote: Err(detail) => return Ok(client::fault(args.json, &detail)),
@row_add n=616 row=contract/viola-link-exit-codes/1 area=occupied-resources rec=fct-scvtx5bh@1
boundary: viola link and unlink exit codes
item: 1
promise: A refusal reply, or an ok of another shape.
@row_check n=616 row=contract/viola-link-exit-codes/1 sha=3271ce94e740 v=holds judge=sonnet head=8834195b0683
at: src/cmd/link.rs:61
quote: Reply::Refused { .. } => anyhow::bail!("a link request was refused"),
at: src/cmd/link.rs:51
quote: anyhow::ensure!(known, "malformed reply");
at: src/main.rs:138
quote: ) => ExitCode::from(1),
@row_add n=617 row=contract/frame-verb-exit-codes/21 area=occupied-resources rec=fct-l6ysfimx@2
boundary: send, wait, last, answer, pause and release exit codes
item: 21
promise: The instance check refused, the socket folder was refused, or the server runs under another euid; human form puts the unable/hint pair on stderr, --json the instance-unreachable document on stdout.
@row_check n=617 row=contract/frame-verb-exit-codes/21 sha=bb004923884c v=holds judge=sonnet head=8834195b0683
at: src/cmd/client.rs:35
quote: unreachable_document(UnreachableCause::StrictModes);
at: src/cmd/client.rs:173-174
quote: if check_endpoint_dir(&live.endpoint).is_err() { return Err(CallError::SocketDirRefused);
at: src/cmd/client.rs:177-178
quote: if !peer_admitted(client.server_euid()) { return Err(CallError::ServerUnverified);
at: src/cmd/client.rs:252
quote: unreachable_document(cause.cause());
at: src/cmd/client.rs:254
quote: human();
@row_add n=618 row=contract/instance-unreachable-detail/strict-modes-failed area=occupied-resources rec=fct-l6ysfimx@2
boundary: the instance-unreachable --json document
item: strict-modes-failed
promise: The detail when the instance check refused the instance.
@row_check n=618 row=contract/instance-unreachable-detail/strict-modes-failed sha=1945f9bd7a7f v=holds judge=sonnet head=8834195b0683
at: src/cmd/client.rs:35
quote: unreachable_document(UnreachableCause::StrictModes);
at: crates/viola-core/src/lib.rs:196
quote: Self::StrictModes => "strict-modes-failed",
@row_add n=619 row=contract/instance-unreachable-detail/socket-dir-refused area=occupied-resources rec=fct-l6ysfimx@2
boundary: the instance-unreachable --json document
item: socket-dir-refused
promise: The detail when the recorded socket folder was refused, past the instance check.
@row_check n=619 row=contract/instance-unreachable-detail/socket-dir-refused sha=7215f02f84ab v=holds judge=sonnet head=8834195b0683
at: src/cmd/client.rs:174
quote: return Err(CallError::SocketDirRefused);
at: src/cmd/client.rs:195
quote: Err(CallError::SocketDirRefused) => return Ok(Err(Unreachable::SocketDir)),
at: crates/viola-core/src/lib.rs:201
quote: Self::SocketDir => "socket-dir-refused",
@row_add n=620 row=contract/instance-unreachable-detail/server-verify-failed area=occupied-resources rec=fct-l6ysfimx@2
boundary: the instance-unreachable --json document
item: server-verify-failed
promise: The detail when the server runs under another euid.
@row_check n=620 row=contract/instance-unreachable-detail/server-verify-failed sha=35738b06de0c v=holds judge=sonnet head=8834195b0683
at: src/cmd/client.rs:177-178
quote: if !peer_admitted(client.server_euid()) { return Err(CallError::ServerUnverified);
at: src/cmd/client.rs:194
quote: Err(CallError::ServerUnverified) => return Ok(Err(Unreachable::ServerUnverified)),
at: crates/viola-core/src/lib.rs:200
quote: Self::ServerVerify => "server-verify-failed",
@row_add n=621 row=contract/viola-verify-exit-codes/1 area=occupied-resources rec=fct-l6ysfimx@2
boundary: viola verify exit codes
item: 1
promise: The home-tree check refused the home, ledger, bin or plugin folder, before any log or step line, or the stamps update was refused; the home-state unable/hint pair is printed.
@row_check n=621 row=contract/viola-verify-exit-codes/1 sha=779fc245c562 v=holds judge=sonnet head=8834195b0683
at: src/cmd/verify.rs:67-69
quote: if check_home_tree(&home, None, None).is_err() { refuse_home(); return Ok(ExitCode::from(1));
at: src/cmd/verify.rs:286-288
quote: Err(StampsError::StrictModes(_)) => { refuse_home(); return Ok(Ended::Refused("strict-modes-failed"));
@row_add n=622 row=contract/viola-run-start-exit-codes/1 area=occupied-resources rec=fct-l6ysfimx@2
boundary: viola run and revive start exit codes
item: 1
promise: The home-tree check refused, before any log, or the socket folder was refused or could not be created, before any snapshot, child or socket bind.
@row_check n=622 row=contract/viola-run-start-exit-codes/1 sha=bbf6b652b2c6 v=holds judge=sonnet head=8834195b0683
at: src/cmd/run.rs:171-173
quote: Err(refused) => { refuse_tree(&args.name, refused); return Ok(ExitCode::from(1));
at: src/cmd/run.rs:299-300
quote: Bound::SocketDirRefused => { return Ok(refused_with(launch.json, "socket-dir-refused", || {
at: src/cmd/run.rs:412-413
quote: run::log_self_exit(1, Some(detail)); Started::Refused(ExitCode::from(1))
@row_add n=623 row=contract/hook-capture-arm/exit area=occupied-resources rec=fct-yoyjutua@1
boundary: viola hook --capture arm
item: exit status
promise: Always exit 0 with empty stderr; no VIOLA_* variable read, no diagnostics started, no channel opened.
@row_check n=623 row=contract/hook-capture-arm/exit sha=46efcb84503e v=holds judge=sonnet head=8834195b0683
at: src/cmd/hook.rs:95-96
quote: capture_statusline(dir, shell, &mut std::io::stdin().lock()); return Ok(ExitCode::SUCCESS);
at: src/cmd/hook.rs:106
quote: return Ok(ExitCode::SUCCESS);
@row_add n=624 row=contract/hook-capture-arm/payload-file area=occupied-resources rec=fct-yoyjutua@1
boundary: viola hook --capture arm
item: payload file
promise: Stdin, capped at MAX_FRAME, is written raw to an exclusively claimed <PascalEvent>.<k>.json in the given absolute, existing folder.
@row_check n=624 row=contract/hook-capture-arm/payload-file sha=3aa8ce23a8ea v=holds judge=sonnet head=8834195b0683
at: src/cmd/hook.rs:421
quote: if !dir.is_absolute() || !dir.is_dir() {
at: src/cmd/hook.rs:425
quote: stdin.take(MAX_FRAME + 1).read_to_end(&mut bytes).ok()?;
at: src/cmd/hook.rs:437
quote: match create_private_new(&path) {
at: src/cmd/hook.rs:443
quote: replace_private(&path, &bytes, FILE_MODE).ok()?;
@row_add n=625 row=contract/statusline-capture-shell-file/verdict area=occupied-resources rec=fct-yoyjutua@1
boundary: StatusLine.<k>.shell capture file
item: verdict
promise: Holds exactly bin-sh or other; a missing shell word is other, and the word is written nowhere.
@row_check n=625 row=contract/statusline-capture-shell-file/verdict sha=e52a5b62a722 v=holds judge=sonnet head=8834195b0683
at: src/cmd/hook.rs:460-464
quote: let verdict = if shell == Some(OsStr::new(BIN_SH)) { ShellVerdict::BinSh } else { ShellVerdict::Other };
at: src/cmd/hook.rs:476
quote: replace_private(&shell_path, verdict.word().as_bytes(), FILE_MODE).ok()?;
@row_add n=626 row=contract/claude-child-argv/plugin-dir area=occupied-resources rec=fct-p6lni4fy@2
boundary: the claude child's argv
item: --plugin-dir
promise: --plugin-dir <viola home>/plugin/<version>-<hash> comes first on every start.
@row_check n=626 row=contract/claude-child-argv/plugin-dir sha=55a5dfa9fbb4 v=holds judge=sonnet head=8834195b0683
at: src/run/mod.rs:128-131
quote: let mut args = vec![ OsString::from(PLUGIN_DIR_FLAG), plugin_dir.as_os_str().to_owned(), ];
@row_add n=627 row=contract/claude-child-argv/settings area=occupied-resources rec=fct-p6lni4fy@2
boundary: the claude child's argv
item: --settings
promise: --settings <instance dir>/settings.json follows when the start wrote a settings override; the caller's arguments come after.
@row_check n=627 row=contract/claude-child-argv/settings sha=bce3a51418bd v=holds judge=sonnet head=8834195b0683
at: src/run/mod.rs:132-136
quote: if let Some(settings) = settings { args.push(OsString::from(SETTINGS_FLAG)); args.push(settings.as_os_str().to_owned()); } args.extend_from_slice(user_args);
@row_add n=628 row=contract/claude-child-argv/resume area=occupied-resources rec=fct-p6lni4fy@2
boundary: the claude child's argv
item: --resume
promise: A revived start adds --resume <id> after those flags, the id one argv element: the newest logged session id, or --id when the instance's log holds it.
@row_check n=628 row=contract/claude-child-argv/resume sha=d3421abab3e9 v=holds judge=sonnet head=8834195b0683
at: crates/viola-agent-claude/src/lib.rs:110
quote: let mut args = vec![OsString::from(RESUME_FLAG), OsString::from(id)];
at: src/cmd/revive.rs:250
quote: None => ids.next_back().ok_or(Refusal::NoSession),
at: src/cmd/revive.rs:249
quote: Some(asked) => ids.find(|id| *id == asked).ok_or(Refusal::UnknownId),
@row_add n=629 row=contract/claude-child-argv/fork-session area=occupied-resources rec=fct-p6lni4fy@2
boundary: the claude child's argv
item: --fork-session
promise: --fork-session follows --resume when --fork is given; the words after -- come last.
@row_check n=629 row=contract/claude-child-argv/fork-session sha=f76879f08d73 v=holds judge=sonnet head=8834195b0683
at: crates/viola-agent-claude/src/lib.rs:111-112
quote: if fork { args.push(OsString::from(FORK_SESSION_FLAG));
at: src/cmd/revive.rs:230-231
quote: let mut child_args = resume_args(&id, args.fork); child_args.extend(args.extra);
@row_add n=630 row=contract/session-id/shape area=occupied-resources rec=fct-p6lni4fy@2
boundary: Claude session id
item: shape
promise: 36 characters, hyphens at offsets 8, 13, 18 and 23, ASCII hex elsewhere.
@row_check n=630 row=contract/session-id/shape sha=ac2ce84fa41d v=holds judge=sonnet head=8834195b0683
at: crates/viola-agent-claude/src/lib.rs:97-104
quote: id.len() == 36 && id.bytes().enumerate().all(|(at, byte)| { if matches!(at, 8 | 13 | 18 | 23) { byte == b'-' } else { byte.is_ascii_hexdigit() } })
@row_add n=631 row=decision/logged-session-id-on-child-argv area=occupied-resources rec=fct-p6lni4fy@2
name: Logged session id on the child's command line
rule: A session id read from the instance's log may be placed on the claude child's command line, as one argv element of a direct spawn.
why: A ratified crossing, chosen over allowing only an id the human types.
@row_check n=631 row=decision/logged-session-id-on-child-argv sha=8923f0c26415 v=holds judge=sonnet head=8834195b0683
at: src/cmd/revive.rs:230
quote: let mut child_args = resume_args(&id, args.fork);
at: crates/viola-agent-claude/src/lib.rs:110
quote: let mut args = vec![OsString::from(RESUME_FLAG), OsString::from(id)];
@row_add n=632 row=decision/claude-flag-literals-in-agent-crate area=occupied-resources rec=fct-p6lni4fy@2
name: Claude flag literals in the agent crate
rule: The Claude agent crate is the one product home of the --resume and --fork-session flag literals, and holds the program name and the --settings flag that run and verify read.
@row_check n=632 row=decision/claude-flag-literals-in-agent-crate sha=084bc31dafc2 v=holds judge=sonnet head=8834195b0683
at: crates/viola-agent-claude/src/lib.rs:89
quote: pub const RESUME_FLAG: &str = "--resume";
at: crates/viola-agent-claude/src/lib.rs:92
quote: pub const FORK_SESSION_FLAG: &str = "--fork-session";
at: crates/viola-agent-claude/src/lib.rs:86
quote: pub const PROGRAM: &str = "claude";
at: crates/viola-agent-claude/src/statusline.rs:14
quote: pub const SETTINGS_FLAG: &str = "--settings";
at: src/run/mod.rs:133
quote: args.push(OsString::from(SETTINGS_FLAG));
at: src/cmd/verify/typed/runs.rs:96
quote: args.push(SETTINGS_FLAG.into());
@row_add n=633 row=decision/per-session-settings-override area=occupied-resources rec=fct-ehpssyzz@2
name: Per-session settings override for the status line
rule: Every start, a revived one included, rewrites the instance's settings.json whole at 0600 before the spawn, so the child's status line is viola's wrapper.
why: A code-bearing file handed to the CLI at every start is a ratified boundary widening.
@row_check n=633 row=decision/per-session-settings-override sha=c10945cc12e8 v=holds judge=sonnet head=8834195b0683
at: src/cmd/run.rs:568
quote: replace_private(&path, document.as_bytes(), FILE_MODE)?;
@row_add n=634 row=decision/settings-override-safe-path-only area=occupied-resources rec=fct-ehpssyzz@2
name: Settings override only for a safe pinned path
rule: The settings override is written only when every character of the pinned path is an ASCII letter, a digit or one of _ - . / :; for any other path no override is written and no --settings is passed.
@row_check n=634 row=decision/settings-override-safe-path-only sha=d1dbc7c8b0f7 v=holds judge=sonnet head=8834195b0683
at: crates/viola-agent-claude/src/statusline.rs:95
quote: if !pinned_bin_fwd.chars().all(is_shell_plain) {
at: crates/viola-agent-claude/src/statusline.rs:103
quote: c.is_ascii_alphanumeric() || matches!(c, '_' | '-' | '.' | '/' | ':')
@row_add n=635 row=contract/instance-settings-json/statusline area=occupied-resources rec=fct-ehpssyzz@2
boundary: instance settings.json
item: statusLine
promise: {"statusLine":{"type":"command","command":"<pinned path> hook statusline"}}, the pinned path absolute.
@row_check n=635 row=contract/instance-settings-json/statusline sha=f070d311192e v=holds judge=sonnet head=8834195b0683
at: crates/viola-agent-claude/src/statusline.rs:98-99
quote: let command = format!("{pinned_bin_fwd} hook statusline"); Some(json!({"statusLine": {"type": "command", "command": command}}).to_string())
@row_add n=636 row=decision/session-home-from-viola-dir area=occupied-resources rec=fct-7uan73fu@1
name: Session home resolved from VIOLA_DIR
rule: Every verb resolves the home from the VIOLA_DIR grandparent only when VIOLA_NAME and VIOLA_DIR pass the instance shape check; a pair that fails falls through to the default home, never refusing.
why: A session started with --home keeps its hooks, MCP server and nested CLI calls in that home.
@row_check n=636 row=decision/session-home-from-viola-dir sha=a18822f74ad4 v=holds judge=sonnet head=8834195b0683
at: src/cmd/mod.rs:297-298
quote: if let Some(instance) = instance_of(name, dir) { return Ok(instance.home);
at: src/cmd/mod.rs:277-278
quote: std::env::var_os("VIOLA_NAME"), std::env::var_os("VIOLA_DIR"),
@row_add n=637 row=decision/child-env-claude-strip area=occupied-resources rec=fct-lb4pv4em@2
name: CLAUDE names stripped from the child's environment
rule: Every inherited CLAUDE* name, matched exactly, is removed from the claude child's environment unless config.json claude_env_keep names it.
except: The identity floor's names are removed whatever the persistent environment or claude_env_keep says.
@row_check n=637 row=decision/child-env-claude-strip sha=8b94219665a4 v=holds judge=sonnet head=8834195b0683
at: crates/viola-agent-claude/src/lib.rs:72-77
quote: let floor = IDENTITY_FLOOR.contains(&text.as_str()); if !floor && persistent.contains(&text) { plan.kept.push(text); } else { plan.remove.push(name); }
@row_add n=638 row=decision/identity-floor-eleven-names area=occupied-resources rec=fct-lb4pv4em@2
name: Identity floor of eleven names
rule: The identity floor is CLAUDECODE, CLAUDE_CODE_BRIDGE_SESSION_ID, CLAUDE_CODE_CHILD_SESSION, CLAUDE_CODE_ENTRYPOINT, CLAUDE_CODE_EXECPATH, CLAUDE_CODE_MESSAGING_SOCKET, CLAUDE_CODE_MESSAGING_TOKEN, CLAUDE_CODE_SESSION_ATTENDED, CLAUDE_CODE_SESSION_ID, CLAUDE_EFFORT and CLAUDE_PID.
why: The plugin and project names a hook sees are not identity names, and the prefix rule already removes them when inherited.
@row_check n=638 row=decision/identity-floor-eleven-names sha=2b839763fd91 v=holds judge=sonnet head=8834195b0683
at: crates/viola-agent-claude/src/lib.rs:17
quote: pub const IDENTITY_FLOOR: [&str; 11] = [
at: crates/viola-agent-claude/src/lib.rs:28
quote: "CLAUDE_PID",
@row_add n=639 row=decision/env-strip-names-only area=occupied-resources rec=fct-lb4pv4em@2
name: Environment strip reads names only
rule: The environment strip and the logs read variable names only, never a value.
@row_check n=639 row=decision/env-strip-names-only sha=cee43ac9f695 v=holds judge=sonnet head=8834195b0683
searched: plan_strip\( · in src/**/*.rs (callers pass std::env::vars_os().map(|(k, _)| k), the value dropped: src/run/agents.rs:114, src/cmd/run.rs:289, src/cmd/verify.rs:179)
searched: env::var\(.*CLAUDE · in src/**/*.rs and crates/viola-agent-claude/src/lib.rs (no value read; strip plan and logs hold names only, lib.rs:44-51)
@row_add n=640 row=contract/process-start-claude-child/env-stripped-known area=occupied-resources rec=fct-lb4pv4em@2
boundary: process-start line of the claude child
item: env_stripped_known
promise: Names the CLAUDE* variables stripped from the child's environment, names only.
@row_check n=640 row=contract/process-start-claude-child/env-stripped-known sha=1d54578d434d v=holds judge=sonnet head=8834195b0683
at: src/run/mod.rs:58
quote: env_stripped_known = Some(stripped.as_str()).filter(|s| !s.is_empty()),
at: crates/viola-agent-claude/src/lib.rs:45-46
quote: pub fn removed_names(&self) -> String { joined(self.remove.iter().map(|n| n.to_string_lossy().into_owned()))
@row_add n=641 row=contract/process-start-claude-child/env-kept area=occupied-resources rec=fct-lb4pv4em@2
boundary: process-start line of the claude child
item: env_kept
promise: Names the CLAUDE* variables kept through claude_env_keep, names only.
@row_check n=641 row=contract/process-start-claude-child/env-kept sha=f8f36e59e32b v=holds judge=sonnet head=8834195b0683
at: src/run/mod.rs:59
quote: env_kept = Some(kept.as_str()).filter(|s| !s.is_empty()),
at: crates/viola-agent-claude/src/lib.rs:73-74
quote: if !floor && persistent.contains(&text) { plan.kept.push(text);
@row_add n=642 row=decision/harness-env-unread-by-product area=occupied-resources rec=fct-5lqu6sil@2
name: Harness variables never read by viola
rule: AGENT_RUN_CHUNK_BASE, AGENT_RUN_KEEP_HOMES, AGENT_RUN_KEEP_FAILED and CI are test-harness variables that viola itself never reads.
@row_check n=642 row=decision/harness-env-unread-by-product sha=daf72b7bd485 v=holds judge=sonnet head=8834195b0683
searched: AGENT_RUN_CHUNK_BASE|AGENT_RUN_KEEP_HOMES|AGENT_RUN_KEEP_FAILED · in src/**/*.rs (none; only crates/viola-e2e, the harness)
searched: var_os("CI")|var("CI") · in src/**/*.rs (none; only crates/viola-e2e/src/bin/viola-harness.rs:195 and crates/viola-e2e/src/harness/run.rs:101)
@row_add n=643 row=contract/harness-environment/agent-run-chunk-base area=occupied-resources rec=fct-5lqu6sil@2
boundary: test harness environment
item: AGENT_RUN_CHUNK_BASE
promise: Overrides the diff base of viola-harness run --mutants; without it the harness derives the base from git history.
@row_check n=643 row=contract/harness-environment/agent-run-chunk-base sha=53b1259db59d v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/bin/viola-harness.rs:194
quote: std::env::var("AGENT_RUN_CHUNK_BASE").ok(),
at: crates/viola-e2e/src/harness/run/mutants/base.rs:41-47
quote: let base = from_env .map(|b| b.trim().to_owned()) .filter(|b| !b.is_empty()) .or_else(|| chunk_flip(repo)) .or_else(|| { git(repo, &["merge-base", "HEAD", "origin/main"]).map(|b| b.trim().to_owned()) })?;
@row_add n=644 row=contract/harness-environment/agent-run-keep-homes area=occupied-resources rec=fct-5lqu6sil@2
boundary: test harness environment
item: AGENT_RUN_KEEP_HOMES
promise: The value 1 keeps the test and session homes for the CI gates.
@row_check n=644 row=contract/harness-environment/agent-run-keep-homes sha=9d454ece5cb7 v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/bin/viola-harness.rs:201
quote: let keep = std::env::var("AGENT_RUN_KEEP_HOMES").is_ok_and(|v| v == "1");
at: crates/viola-e2e/src/harness/session.rs:125
quote: flag_set(std::env::var("AGENT_RUN_KEEP_HOMES")),
at: tests/support/home.rs:195
quote: if !flag("AGENT_RUN_KEEP_HOMES") && !flag("AGENT_RUN_KEEP_FAILED") {
@row_add n=645 row=contract/harness-environment/agent-run-keep-failed area=occupied-resources rec=fct-5lqu6sil@2
boundary: test harness environment
item: AGENT_RUN_KEEP_FAILED
promise: The value 1 keeps a failing test's home for post-mortem; run --mutants sets it to 0 on the cargo mutants command.
@row_check n=645 row=contract/harness-environment/agent-run-keep-failed sha=9b1a416c7775 v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/session.rs:126
quote: flag_set(std::env::var("AGENT_RUN_KEEP_FAILED")),
at: crates/viola-e2e/src/harness/run/mutants.rs:247
quote: .env("AGENT_RUN_KEEP_FAILED", "0")
@row_add n=646 row=decision/fake-agent-hook-panic-seam area=occupied-resources rec=fct-2hatag6l@1
name: Hook panic test seam
rule: FAKE_AGENT_HOOK_PANIC, read only by a fake-agent build, makes hook panic only on the exact value 1, with a fixed ASCII payload holding no upstream text, path or secret; the seam disables no control and widens no redaction.
why: A ratified test seam, ratified together with the exact-path exemption for its panic lines.
@row_check n=646 row=decision/fake-agent-hook-panic-seam sha=ec688c790d88 v=holds judge=sonnet head=8834195b0683
at: src/cmd/hook/seam.rs:18-19
quote: if value.as_deref() == Some(OsStr::new("1")) { panic!("{}", "forced-hook-panic ".repeat(256));
at: src/cmd/hook/seam.rs:16
quote: #[cfg(feature = "fake-agent")]
@row_add n=647 row=decision/config-bad-value-falls-back area=occupied-resources rec=fct-3jytrhfq@2
name: Bad config values fall back to defaults
rule: An unusable config.json file or value falls back to its default and is reported as a parse-rejected config-json line; no value switches the budget gate off or makes the dialog hold zero or unbounded.
@row_check n=647 row=decision/config-bad-value-falls-back sha=15db1b145d89 v=holds judge=sonnet head=8834195b0683
at: src/run/budget.rs:79-82
quote: Some(None) => { rejections.push(ConfigRejection::Malformed); compiled(default) }
at: src/run/dialog.rs:180
quote: Some(None) => (DialogHold::default(), vec![ConfigRejection::Malformed]),
at: src/obs.rs:224
quote: parser = "config-json",
@row_add n=648 row=contract/viola-run-start-refusals/pinned-hash-mismatch area=occupied-resources rec=fct-6u55z4cp@2
boundary: viola run start refusals
item: pinned-hash-mismatch
promise: A pinned copy whose bytes differ is left as found and the start is refused, exit 1; the pinned copy is placed once and never replaced.
@row_check n=648 row=contract/viola-run-start-refusals/pinned-hash-mismatch sha=f740de4d0d3e v=holds judge=sonnet head=8834195b0683
at: crates/viola-state/src/pin.rs:90
quote: Ok(_) => return Err(PinError::HashMismatch),
at: crates/viola-state/src/pin.rs:105-108
quote: match place_private_new(path, bytes, DIR_MODE)? { Placed::Written => Ok(()), Placed::Present if file_key(path)? == hash => Ok(()), Placed::Present => Err(PinError::HashMismatch),
at: src/cmd/run.rs:467-468
quote: Err(PinError::HashMismatch) => { tracing::Span::current().record("outcome", "pinned-hash-mismatch");
@row_add n=649 row=contract/diagnostics-role-files/run-name area=occupied-resources rec=fct-6oxorhgk@1
boundary: home-level diagnostics role files
item: run-<name>.ndjson
promise: The wrapper's codes-only process lines, viola revive's start arm included.
@row_check n=649 row=contract/diagnostics-role-files/run-name sha=6dfd42357d55 v=holds judge=sonnet head=8834195b0683
at: src/obs.rs:57-58
quote: (ObsProcess::Run | ObsProcess::Hook | ObsProcess::Cli, Some(name)) => { format!("{}-{}", process.as_str(), name_str(name))
at: src/cmd/run.rs:225
quote: obs::viola_obs_init(home, ObsProcess::Run, Some(name.clone()), level)?;
@row_add n=650 row=contract/diagnostics-role-files/hook-name area=occupied-resources rec=fct-6oxorhgk@1
boundary: home-level diagnostics role files
item: hook-<name>.ndjson
promise: The hook's hook-invoked, hook-decision and parse-rejected lines.
@row_check n=650 row=contract/diagnostics-role-files/hook-name sha=463c284cb56f v=holds judge=sonnet head=8834195b0683
at: src/cmd/hook.rs:173
quote: ObsEvent::HookInvoked,
at: src/cmd/hook.rs:391
quote: ObsEvent::HookDecision,
at: src/cmd/hook.rs:536
quote: ObsEvent::ParseRejected,
@row_add n=651 row=contract/diagnostics-role-files/mcp area=occupied-resources rec=fct-6oxorhgk@1
boundary: home-level diagnostics role files
item: mcp.ndjson
promise: viola mcp's codes-only lines, ending with a self process-exit (exit_code 0, no detail) once stdin closes.
@row_check n=651 row=contract/diagnostics-role-files/mcp sha=f6af695478c0 v=stale judge=sonnet head=8834195b0683
at: src/cmd/mcp.rs:36-37
quote: viola_mcp::serve(home.to_path_buf(), own_name, board)?; run::log_self_exit(0, None);
at: src/run/mod.rs:75
quote: pub(crate) fn log_self_exit(exit_code: u8, detail: Option<&'static str>) {
at: src/run/mod.rs:82
quote: exit_code = exit_code,
shows: serve returns when stdin closes and viola mcp then calls log_self_exit(0, None), which writes a self process-exit with exit_code 0 and no detail, so mcp.ndjson does get a self process-exit on stdin EOF.
cell: promise
value: viola mcp's codes-only lines, ending with a self process-exit (exit_code 0, no detail) once stdin closes.
@row_add n=652 row=contract/diagnostics-role-files/cli-name area=occupied-resources rec=fct-6oxorhgk@1
boundary: home-level diagnostics role files
item: cli-<name>.ndjson
promise: The CLI verbs' lines, <name> being the target instance for the frame verbs and the caller's VIOLA_NAME for list, verify, revive --list and plugin install.
@row_check n=652 row=contract/diagnostics-role-files/cli-name sha=c16b36d7a79c v=holds judge=sonnet head=8834195b0683
at: src/cmd/mod.rs:187
quote: let sink = cli_sink(&home, &args.driven);
at: src/cmd/mod.rs:152
quote: let sink = cli_sink(&home, &args.name);
at: src/cmd/mod.rs:168
quote: let sink = client::own_name().map(|instance| DetailSink {
at: src/cmd/mod.rs:217
quote: let instance = std::env::var_os("VIOLA_NAME")
at: src/cmd/mod.rs:198
quote: let sink = client::own_name().map(|instance| DetailSink {
at: src/cmd/revive.rs:277
quote: if let Some(own) = client::own_name() {
@row_add n=653 row=contract/diag-line/format area=occupied-resources rec=fct-6oxorhgk@1
boundary: diagnostics line
item: format
promise: Owned by obs and fixed by schemas/diag-line.v1.json; one line per write; folder 0700, files 0600.
@row_check n=653 row=contract/diag-line/format sha=77ebbb420a46 v=holds judge=sonnet head=8834195b0683
at: src/obs.rs:78-79
quote: /// `<home>/diagnostics/<role file>`, the dirs 0700 and the file 0600, once the folder and the /// file, each that exists, have passed their check.
at: crates/viola-state/src/fs.rs:10-11
quote: pub const DIR_MODE: u32 = 0o700; pub const FILE_MODE: u32 = 0o600;
at: schemas/diag-line.v1.json:5
quote: "description": "One line of <home>/diagnostics/{run-<name>,hook-<name>,mcp,cli-<name>}.ndjson. Required fields per test-plan §3 Log format; additive fields per obs-plan §6 catalog, default-deny (obs-plan §8). A
@row_add n=654 row=decision/codes-only-role-logs area=occupied-resources rec=fct-6oxorhgk@1
name: Codes-only role logs
rule: Home-level role logs hold codes only, with no row text, prompt, id, path or endpoint; content-bearing detail goes only to the instance's owner-only detail files.
@row_check n=654 row=decision/codes-only-role-logs sha=1819c6c1ef17 v=holds judge=sonnet head=8834195b0683
at: src/obs.rs:323
quote: pub(crate) fn write_detail(home: &Path, instance: &ViolaName, process: ObsProcess, line: &str) {
@row_add n=655 row=decision/refused-check-writes-no-line area=occupied-resources rec=fct-6oxorhgk@1,fct-7uywwkoa@2
name: Refused check writes no line
rule: A refused strict-modes or instance check opens no role file, writes no line and leaves the home's mode as found.
@row_check n=655 row=decision/refused-check-writes-no-line sha=7d8b14c1b78a v=holds judge=sonnet head=8834195b0683
at: src/cmd/list.rs:43-47
quote: let scan = match board::scan(&home)? { Ok(scan) => scan, Err(refused) => { refuse(args.json, &refused); return Ok(ExitCode::from(21));
at: src/cmd/run.rs:172
quote: refuse_tree(&args.name, refused);
at: src/cmd/run.rs:176
quote: let persistent = open_wrapper_log(home, &args.name)?;
at: src/cmd/hook.rs:116-118
quote: let Some(instance) = instance.as_ref().and_then(canonical).filter(trusted) else { return Ok(ExitCode::SUCCESS); };
@row_add n=656 row=contract/frame-verb-exit-detail/instance-dead area=occupied-resources rec=fct-6oxorhgk@1
boundary: CLI frame verb process-exit detail
item: instance-dead
promise: Exit 21 with during connect, at WARN.
@row_check n=656 row=contract/frame-verb-exit-detail/instance-dead sha=7913e8600c43 v=holds judge=sonnet head=8834195b0683
at: src/cmd/client.rs:256-262
quote: obs_event!( WARN, ObsEvent::ProcessExit, subject = "self", exit_code = 21u8, detail = cause.detail(), during = cause.during(),
at: src/cmd/client.rs:137
quote: Self::Dead | Self::Unwrapped | Self::ServerUnverified | Self::SocketDir => "connect",
at: crates/viola-core/src/lib.rs:197
quote: Self::InstanceDead => "instance-dead",
@row_add n=657 row=contract/frame-verb-exit-detail/server-verify-failed area=occupied-resources rec=fct-6oxorhgk@1
boundary: CLI frame verb process-exit detail
item: server-verify-failed
promise: Exit 21 with during connect when the serving process is not the recorded wrapper or runs under another euid; no channel-request line.
@row_check n=657 row=contract/frame-verb-exit-detail/server-verify-failed sha=2079a83c7212 v=holds judge=sonnet head=8834195b0683
at: src/cmd/client.rs:177-178
quote: if !peer_admitted(client.server_euid()) { return Err(CallError::ServerUnverified);
at: src/cmd/client.rs:183
quote: ServerVerdict::Mismatch | ServerVerdict::Unverifiable => Err(CallError::ServerUnverified),
at: src/cmd/client.rs:194
quote: Err(CallError::ServerUnverified) => return Ok(Err(Unreachable::ServerUnverified)),
at: src/cmd/client.rs:137
quote: Self::Dead | Self::Unwrapped | Self::ServerUnverified | Self::SocketDir => "connect",
@row_add n=658 row=contract/frame-verb-exit-detail/unwrapped-name area=occupied-resources rec=fct-6oxorhgk@1
boundary: CLI frame verb process-exit detail
item: unwrapped-name
promise: Exit 21 with during connect.
@row_check n=658 row=contract/frame-verb-exit-detail/unwrapped-name sha=79894086d501 v=holds judge=sonnet head=8834195b0683
at: src/cmd/client.rs:137
quote: Self::Dead | Self::Unwrapped | Self::ServerUnverified | Self::SocketDir => "connect",
at: src/cmd/client.rs:261
quote: detail = cause.detail(),
at: crates/viola-core/src/lib.rs:198
quote: Self::UnwrappedName => "unwrapped-name",
@row_add n=659 row=contract/frame-verb-exit-detail/endpoint-lost area=occupied-resources rec=fct-6oxorhgk@1
boundary: CLI frame verb process-exit detail
item: endpoint-lost
promise: Exit 21 with during call, for a wrapper lost with the call open.
@row_check n=659 row=contract/frame-verb-exit-detail/endpoint-lost sha=e46d21f765f0 v=holds judge=sonnet head=8834195b0683
at: src/cmd/client.rs:136
quote: Self::Lost => "call",
at: src/cmd/client.rs:199-200
quote: Err(CallError::Channel(ChannelError::Closed | ChannelError::Io(_))) => { return Ok(Err(Unreachable::Lost));
at: crates/viola-core/src/lib.rs:199
quote: Self::EndpointLost => "endpoint-lost",
@row_add n=660 row=contract/frame-verb-exit-detail/wrapper-fault area=occupied-resources rec=fct-6oxorhgk@1
boundary: CLI frame verb process-exit detail
item: wrapper-fault
promise: Exit 20.
@row_check n=660 row=contract/frame-verb-exit-detail/wrapper-fault sha=61cb91f9c421 v=holds judge=sonnet head=8834195b0683
at: src/cmd/client.rs:244-245
quote: run::log_self_exit(20, Some("wrapper-fault")); 20
@row_add n=661 row=contract/frame-verb-exit-detail/socket-dir-refused area=occupied-resources rec=fct-6oxorhgk@1
boundary: CLI frame verb process-exit detail
item: socket-dir-refused
promise: Exit 21 with during connect, at WARN, no channel-request line; run and revive's start write it with exit_code 1.
@row_check n=661 row=contract/frame-verb-exit-detail/socket-dir-refused sha=9ef369648dc2 v=holds judge=sonnet head=8834195b0683
at: src/cmd/client.rs:173-174
quote: if check_endpoint_dir(&live.endpoint).is_err() { return Err(CallError::SocketDirRefused);
at: src/cmd/client.rs:137
quote: Self::Dead | Self::Unwrapped | Self::ServerUnverified | Self::SocketDir => "connect",
at: src/cmd/client.rs:256-257
quote: obs_event!( WARN,
at: src/cmd/run.rs:300
quote: return Ok(refused_with(launch.json, "socket-dir-refused", || {
at: src/cmd/run.rs:411-412
quote: pub(super) fn refused(detail: &'static str) -> Started { run::log_self_exit(1, Some(detail));
@row_add n=662 row=contract/process-exit-detail/stale-heartbeat area=occupied-resources rec=fct-6oxorhgk@1
boundary: process-exit detail enum
item: stale-heartbeat
promise: A word of the process-exit detail enum in schemas/diag-line.v1.json.
@row_check n=662 row=contract/process-exit-detail/stale-heartbeat sha=8e069a9901c7 v=holds judge=sonnet head=8834195b0683
at: schemas/diag-line.v1.json:77
quote: "detail": { "enum": ["already-live", "squatted-name", "pinned-hash-mismatch", "internal-error", "instance-dead", "strict-modes-failed", "server-verify-failed", "socket-dir-refused", "wrapper-fault", "cwd-missing
@row_add n=663 row=contract/hook-decision-detail/socket-dir-refused area=occupied-resources rec=fct-6oxorhgk@1
boundary: hook-decision detail
item: socket-dir-refused
promise: One hook-decision line at WARN on a refused socket folder, with decision_emitted false.
@row_check n=663 row=contract/hook-decision-detail/socket-dir-refused sha=b12b08441e7a v=holds judge=sonnet head=8834195b0683
at: src/cmd/hook.rs:215
quote: Self::SocketDir => UnreachableCause::SocketDir.as_str(),
at: src/cmd/hook.rs:389-393
quote: Some(detail) => obs_event!( WARN, ObsEvent::HookDecision, hook_event = event.as_str(), decision_emitted = emitted,
at: src/cmd/hook.rs:590-594
quote: Some(detail) => obs_event!( WARN, ObsEvent::HookDecision, hook_event = event.as_str(), decision_emitted = false,
@row_add n=664 row=contract/channel-response-detail/unsupported-version area=occupied-resources rec=fct-gftfjtay@1
boundary: channel-response detail, a closed enum
item: unsupported-version
promise: Written at WARN on both the server's and the client's line; detail is absent on every other channel-response line and holds no param value or name.
@row_check n=664 row=contract/channel-response-detail/unsupported-version sha=586885d43551 v=holds judge=sonnet head=8834195b0683
at: crates/viola-channel/src/lib.rs:105
quote: Self::UnsupportedVersion => Some("unsupported-version"),
at: crates/viola-channel/src/server.rs:492-496
quote: | ProtocolError::UnsupportedVersion | ProtocolError::InvalidParams | ProtocolError::ReleaseFromDriver, ) => { response_line!(WARN)
at: crates/viola-channel/src/client.rs:77
quote: let mut logged = ResponseLine::client(started, id, &self.conn, label);
at: crates/viola-channel/src/lib.rs:108
quote: Self::Parse | Self::InvalidRequest | Self::MethodNotFound | Self::Internal => None,
@row_add n=665 row=contract/channel-response-detail/invalid-params area=occupied-resources rec=fct-gftfjtay@1
boundary: channel-response detail, a closed enum
item: invalid-params
promise: Written at WARN on both the server's and the client's line, the client reading it from a -32602 error's data.
@row_check n=665 row=contract/channel-response-detail/invalid-params sha=dd23503aaa7f v=holds judge=sonnet head=8834195b0683
at: crates/viola-channel/src/lib.rs:106
quote: Self::InvalidParams => Some("invalid-params"),
at: crates/viola-channel/src/server.rs:492-496
quote: | ProtocolError::UnsupportedVersion | ProtocolError::InvalidParams | ProtocolError::ReleaseFromDriver, ) => { response_line!(WARN)
at: crates/viola-channel/src/lib.rs:128
quote: -32602 => match (data["supported"].as_i64(), wrapper) {
@row_add n=666 row=contract/channel-response-detail/release-from-driver area=occupied-resources rec=fct-gftfjtay@1
boundary: channel-response detail, a closed enum
item: release-from-driver
promise: Written at WARN on both the server's and the client's line.
@row_check n=666 row=contract/channel-response-detail/release-from-driver sha=afa7a29beed6 v=holds judge=sonnet head=8834195b0683
at: crates/viola-channel/src/lib.rs:107
quote: Self::ReleaseFromDriver => Some("release-from-driver"),
at: crates/viola-channel/src/server.rs:492-496
quote: | ProtocolError::UnsupportedVersion | ProtocolError::InvalidParams | ProtocolError::ReleaseFromDriver, ) => { response_line!(WARN)
at: crates/viola-channel/src/client.rs:77
quote: let mut logged = ResponseLine::client(started, id, &self.conn, label);
@row_add n=667 row=contract/verify-exit-detail/program-not-found area=occupied-resources rec=fct-if66544y@2
boundary: viola verify process-exit detail
item: program-not-found
promise: A refusal code verify writes under a parsed VIOLA_NAME.
@row_check n=667 row=contract/verify-exit-detail/program-not-found sha=adc1ca5bdaf0 v=holds judge=sonnet head=8834195b0683
at: src/cmd/verify.rs:164
quote: return Ok(Ended::Refused("program-not-found"));
@row_add n=668 row=contract/verify-exit-detail/pinned-hash-mismatch area=occupied-resources rec=fct-if66544y@2
boundary: viola verify process-exit detail
item: pinned-hash-mismatch
promise: A refusal code verify writes under a parsed VIOLA_NAME.
@row_check n=668 row=contract/verify-exit-detail/pinned-hash-mismatch sha=6fedf289be10 v=holds judge=sonnet head=8834195b0683
at: src/cmd/verify.rs:171
quote: return Ok(Ended::Refused("pinned-hash-mismatch"));
@row_add n=669 row=contract/verify-exit-detail/version-unreadable area=occupied-resources rec=fct-if66544y@2
boundary: viola verify process-exit detail
item: version-unreadable
promise: A refusal code verify writes under a parsed VIOLA_NAME.
@row_check n=669 row=contract/verify-exit-detail/version-unreadable sha=133b80d3f3de v=holds judge=sonnet head=8834195b0683
at: src/cmd/verify.rs:199
quote: return Ok(Ended::Refused("version-unreadable"));
@row_add n=670 row=contract/verify-exit-detail/strict-modes-failed area=occupied-resources rec=fct-if66544y@2
boundary: viola verify process-exit detail
item: strict-modes-failed
promise: A refusal code verify writes under a parsed VIOLA_NAME; a refused home tree writes no line.
@row_check n=670 row=contract/verify-exit-detail/strict-modes-failed sha=15d7ed2b2f9a v=holds judge=sonnet head=8834195b0683
at: src/cmd/verify.rs:288
quote: return Ok(Ended::Refused("strict-modes-failed"));
at: src/cmd/verify.rs:67-69
quote: if check_home_tree(&home, None, None).is_err() { refuse_home(); return Ok(ExitCode::from(1));
@row_add n=671 row=contract/verify-exit-detail/unclean-fixture area=occupied-resources rec=fct-if66544y@2
boundary: viola verify process-exit detail
item: unclean-fixture
promise: A refusal code verify writes under a parsed VIOLA_NAME; the fixture's file name enters no code.
@row_check n=671 row=contract/verify-exit-detail/unclean-fixture sha=318cc734f1ad v=holds judge=sonnet head=8834195b0683
at: src/cmd/verify.rs:629
quote: Ended::Refused("unclean-fixture")
@row_add n=672 row=contract/verify-exit-detail/interrupted area=occupied-resources rec=fct-if66544y@2
boundary: viola verify process-exit detail
item: interrupted
promise: The detail of a stopped run; a failing ledger row is no refusal and carries no code.
@row_check n=672 row=contract/verify-exit-detail/interrupted sha=4e89fca42ed4 v=holds judge=sonnet head=8834195b0683
at: src/cmd/verify.rs:87
quote: (1, Some("interrupted"))
@row_add n=673 row=contract/diag-line/state-recovered area=occupied-resources rec=fct-bmahq2ma@1
boundary: diagnostics line
item: state-recovered
promise: A snapshot the replay rebuilds from the log writes one state-recovered line into the role file of the process that read it, one line per replay.
@row_check n=673 row=contract/diag-line/state-recovered sha=8e82445c2b99 v=holds judge=sonnet head=8834195b0683
at: crates/viola-state/src/replay.rs:163-165
quote: obs_event!( WARN, ObsEvent::StateRecovered,
at: crates/viola-state/src/replay.rs:165
quote: ObsEvent::StateRecovered,
@row_add n=674 row=decision/events-log-never-truncated area=occupied-resources rec=fct-hn66ngvt@2
name: Events log never rotated or truncated
rule: The instance's events.ndjson is never rotated or truncated, and any size bound must keep existing offsets valid.
why: wait's after and send's cursor are byte offsets into the file.
@row_check n=674 row=decision/events-log-never-truncated sha=176f3efb9746 v=holds judge=sonnet head=8834195b0683
searched: set_len|truncate\(|rotate · in src/**/*.rs, crates/viola-state/src/**/*.rs, crates/viola-core/src/**/*.rs (none touch events.ndjson; only crates/viola-state/src/fs.rs:67 `options.create(true).truncate(false).write(true);`, src/cmd/hook.rs:773 body cap and the fake agent prompt)
@row_add n=675 row=decision/strict-check-before-log-open area=occupied-resources rec=fct-hn66ngvt@2,fct-7uywwkoa@2
name: Strict-modes check before any log opens
rule: Every checked entry point runs its strict-modes or instance check before any log opens or any file under the home is read.
why: Opening a log sets an existing home to 0700, so a check made after it would pass a home at 0770.
@row_check n=675 row=decision/strict-check-before-log-open sha=3060e972d884 v=holds judge=sonnet head=8834195b0683
at: src/cmd/run.rs:169
quote: let exe = match checked_exe(home, &args.name)? {
at: src/cmd/run.rs:176
quote: let persistent = open_wrapper_log(home, &args.name)?;
at: src/cmd/list.rs:41
quote: // Before the log opens: opening it narrows the home to its owner, which would erase the very
at: src/cmd/list.rs:43
quote: let scan = match board::scan(&home)? {
at: src/cmd/hook.rs:116
quote: let Some(instance) = instance.as_ref().and_then(canonical).filter(trusted) else {
at: src/cmd/hook.rs:119
quote: let (level, _) = obs::read_diagnostics_level(&instance.home);
@row_add n=676 row=decision/board-refused-whole area=occupied-resources rec=fct-hn66ngvt@2
name: Whole board refused on one failed check
rule: viola list checks the home, instances/ and every instance before opening any file, and the first failure refuses the whole board with exit 21, no log and no line.
why: Ratified as the founder's own answer.
@row_check n=676 row=decision/board-refused-whole sha=491f2970fa74 v=holds judge=sonnet head=8834195b0683
at: crates/viola-state/src/board.rs:59
quote: if check_if_present(home, check_read).is_err()
at: crates/viola-state/src/board.rs:62
quote: return Ok(Err(ScanRefused::Home));
at: crates/viola-state/src/board.rs:97
quote: if check_instance(home, dir).is_err() {
at: crates/viola-state/src/board.rs:98
quote: return Ok(Err(ScanRefused::Instance(name.clone())));
at: src/cmd/list.rs:47
quote: return Ok(ExitCode::from(21));
@row_add n=677 row=decision/refused-statusline-check-no-trace area=occupied-resources rec=fct-hn66ngvt@2
name: Refused statusline check leaves no trace
rule: A refused instance check on viola hook statusline exits 0 at once with no config read, no role file, no line, no stdin read, no command run, nothing printed and budget.json untouched.
why: Ratified as the founder's own answer.
@row_check n=677 row=decision/refused-statusline-check-no-trace sha=260b4f015871 v=holds judge=sonnet head=8834195b0683
at: src/cmd/hook/statusline.rs:42-43
quote: let Some(instance) = instance.as_ref().and_then(canonical).filter(trusted) else { return Ok(ExitCode::SUCCESS);
@row_add n=678 row=decision/strict-read-rule area=occupied-resources rec=fct-kekl3p46@2
name: Strict-modes read rule
rule: The strict-modes read rule is the write rule plus a refusal of any group or other permission bit.
@row_check n=678 row=decision/strict-read-rule sha=568dbe50dddb v=holds judge=sonnet head=8834195b0683
at: crates/viola-state/src/strict.rs:227-229
quote: pub fn unix_read_verdict(mode: u32, uid: u32, euid: u32) -> Result<(), Refused> { unix_verdict(mode, uid, euid)?; if mode & 0o077 != 0 {
@row_add n=679 row=decision/list-home-read-rule area=occupied-resources rec=fct-isxdwj6d@2,fct-5oqvk7j6@1
name: viola list holds the home to the read rule
rule: viola list holds the home to the strict-modes read rule with or without an instance, so a 0750 home refuses the whole board.
except: The budget read, mcp's start and plugin install keep the write rule on the home.
@row_check n=679 row=decision/list-home-read-rule sha=5cb970241663 v=holds judge=sonnet head=8834195b0683
at: crates/viola-state/src/board.rs:59
quote: if check_if_present(home, check_read).is_err()
at: crates/viola-state/src/budget.rs:94
quote: check_path(home).ok()?;
at: src/cmd/mcp.rs:46
quote: Ok(_) => viola_state::strict::check_path(path).is_ok(),
at: src/cmd/plugin.rs:85
quote: Ok(_) => viola_state::strict::check_path(path).is_ok(),
@row_add n=680 row=contract/budget-json/v area=occupied-resources rec=fct-hgzk5gxy@1
boundary: budget.json
item: v
promise: Must be 1, or the file gives no reading.
@row_check n=680 row=contract/budget-json/v sha=ca344fbb5f93 v=holds judge=sonnet head=8834195b0683
at: crates/viola-state/src/budget.rs:112
quote: (stored.v == BUDGET_V).then_some(StampedReading {
@row_add n=681 row=contract/budget-json/five-hour area=occupied-resources rec=fct-hgzk5gxy@1
boundary: budget.json
item: five_hour
promise: The word "unknown" or {used_percentage, resets_at}, either field possibly "unknown".
@row_check n=681 row=contract/budget-json/five-hour sha=501e8c8dc788 v=holds judge=sonnet head=8834195b0683
at: crates/viola-core/src/lib.rs:342-343
quote: pub used_percentage: Reading<f64>, pub resets_at: Reading<String>,
at: crates/viola-state/src/budget.rs:68
quote: five_hour: Reading<BudgetWindow>,
@row_add n=682 row=contract/budget-json/seven-day area=occupied-resources rec=fct-hgzk5gxy@1
boundary: budget.json
item: seven_day
promise: The word "unknown" or {used_percentage, resets_at}, either field possibly "unknown".
@row_check n=682 row=contract/budget-json/seven-day sha=a3703873272e v=holds judge=sonnet head=8834195b0683
at: crates/viola-core/src/lib.rs:342-343
quote: pub used_percentage: Reading<f64>, pub resets_at: Reading<String>,
at: crates/viola-state/src/budget.rs:69
quote: seven_day: Reading<BudgetWindow>,
@row_add n=683 row=contract/budget-json/read-at area=occupied-resources rec=fct-hgzk5gxy@1
boundary: budget.json
item: read_at
promise: An RFC 3339 UTC timestamp with milliseconds; read as "unknown" when absent or no string.
@row_check n=683 row=contract/budget-json/read-at sha=8dac6c9517bf v=holds judge=sonnet head=8834195b0683
at: crates/viola-state/src/lib.rs:31
quote: at.to_rfc3339_opts(SecondsFormat::Millis, true)
at: crates/viola-state/src/budget.rs:108-110
quote: let read_at = match stored.read_at { Some(serde_json::Value::String(at)) => Reading::Known(at), _ => Reading::UNKNOWN,
@row_add n=684 row=decision/budget-json-single-writer area=occupied-resources rec=fct-hgzk5gxy@1
name: budget.json written only by the statusline hook
rule: budget.json holds the newest rate_limits reading across wrapped sessions, last writer wins, written whole only by hook statusline and only when the payload holds a rate_limits object.
@row_check n=684 row=decision/budget-json-single-writer sha=14f31523b7ae v=holds judge=sonnet head=8834195b0683
at: src/cmd/hook/statusline.rs:127
quote: reading.is_some_and(|reading| write_budget(&instance.home, &reading, Utc::now()).is_ok());
@row_add n=685 row=decision/stamps-single-writer area=occupied-resources rec=fct-jznqbn54@2
name: Ledger stamps have one writer
rule: ledger/stamps.json has one writer, called only by viola verify, which checks strict modes before and under the lock and replaces the file whole; a refusal reads and writes nothing.
@row_check n=685 row=decision/stamps-single-writer sha=33b88552d949 v=holds judge=sonnet head=8834195b0683
at: crates/viola-state/src/stamps.rs:61
quote: strict::check_stamps(home).map_err(StampsError::StrictModes)?;
at: crates/viola-state/src/stamps.rs:66
quote: strict::check_stamps(home).map_err(StampsError::StrictModes)?;
at: src/cmd/verify.rs:273
quote: let written = update_stamps(home, |existing| {
@row_add n=686 row=decision/version-gate-strict-stamps-read area=occupied-resources rec=fct-jznqbn54@2
name: Version gate reads stamps strictly
rule: run's version gate reads the stamps only after the owner-only strict-modes check; a refusal records cli_verified false and one parse-rejected ledger-stamps strict-modes-failed line.
@row_check n=686 row=decision/version-gate-strict-stamps-read sha=d6409727bcc0 v=holds judge=sonnet head=8834195b0683
at: src/run/version_gate.rs:172
quote: let (verified, detail) = stamps_verdict(read_stamps_strict(home), version);
at: src/run/version_gate.rs:204
quote: Err(StampsReadError::StrictModes(_)) => (false, Some("strict-modes-failed")),
@row_add n=687 row=decision/verify-removes-only-own-dirs area=occupied-resources rec=fct-i7o6tqmg@2,fct-c3nhxk2n@2
name: verify removes only the probe folders it created
rule: viola verify removes its probe folders whole when it passes, fails or is stopped by Ctrl-C or TERM, and only the folders that run created, never by a pattern or a listing.
except: SIGKILL, a crash or a failed handler install leave the folders to be removed by hand.
@row_check n=687 row=decision/verify-removes-only-own-dirs sha=d2965d6b155f v=holds judge=sonnet head=8834195b0683
at: src/cmd/verify.rs:389-391
quote: impl Drop for ProbeDir { fn drop(&mut self) { let _ = std::fs::remove_dir_all(&self.dir);
at: src/cmd/verify/stop.rs:12
quote: for signal in [libc::SIGINT, libc::SIGTERM] {
@row_add n=688 row=decision/root-waits-share-one-bound area=occupied-resources rec=fct-jv5gqhbl@4
name: Root test waits share one bound
rule: Root test waits on a child share the one 7 s WITHIN bound of the watch support.
@row_check n=688 row=decision/root-waits-share-one-bound sha=a994489976b1 v=holds judge=sonnet head=8834195b0683
at: tests/support/watch.rs:14
quote: pub const WITHIN: Duration = Duration::from_secs(7);
@row_add n=689 row=decision/wait-report-kept-on-failure area=occupied-resources rec=fct-jv5gqhbl@4
name: Wait report kept on failure
rule: A test wait writes a report created empty at its start, with a line only when the polled state changes, codes, counts and booleans only; the report is kept when the test panics or is killed and removed when it passes.
why: A test the runner kills keeps its evidence.
@row_check n=689 row=decision/wait-report-kept-on-failure sha=b54608efdfac v=holds judge=sonnet head=8834195b0683
at: tests/support/watch.rs:31
quote: File::create(&report).expect("watch report");
at: tests/support/watch.rs:41
quote: if last.as_deref() == Some(line) {
at: tests/support/watch.rs:62-63
quote: if !std::thread::panicking() { let _ = fs::remove_file(&self.report);
@row_add n=690 row=decision/no-after-every-test-error-body-helper area=occupied-resources rec=fct-mimmga3r@1
name: No after-every-test error-body helper
rule: Error bodies are checked by the surface witnesses, the enumeration and the census of writers reading the shared error-body check, which reports place and class, never bytes; no after-every-test helper is built.
@row_check n=690 row=decision/no-after-every-test-error-body-helper sha=419981585fc2 v=holds judge=sonnet head=8834195b0683
at: tests/support/events.rs:18
quote: pub fn error_body_hit(body: &Value, home: &Path, planted: &[&str]) -> Option<&'static str> {
at: tests/support/events.rs:45
quote: pub fn assert_error_body_clean(place: &str, body: &Value, home: &Path, planted: &[&str]) {
@row_add n=691 row=decision/e2e-home-owner-only-headroom area=occupied-resources rec=fct-5pbk7sp3@1
name: End-to-end home base checked before use
rule: The keepers of the dev host's end-to-end home base refuse a target that is not a real owner-only folder or whose filesystem has less than the 9 GiB headroom floor free.
@row_check n=691 row=decision/e2e-home-owner-only-headroom sha=edb01dfcd34d v=holds judge=sonnet head=8834195b0683
at: tests/support/home.rs:135
quote: meta.permissions().mode() & 0o077 == 0,
at: tests/support/home.rs:138
quote: if let Some(refused) = headroom_refusal(free_bytes(&target), HEADROOM_FLOOR_MIB) {
at: crates/viola-e2e/src/harness/mod.rs:125
quote: if let Some(low) = headroom_refusal(free(&target)?, HEADROOM_FLOOR_MIB) {
@row_add n=692 row=decision/screen-fixtures-signature-rows-only area=occupied-resources rec=fct-3fjm2cf6@2
name: Screen fixtures keep only signature rows
rule: A recorded screen fixture keeps only the rows that hold a compiled signature literal; every other row is an empty string.
@row_check n=692 row=decision/screen-fixtures-signature-rows-only sha=c501ebd3dcdf v=holds judge=sonnet head=8834195b0683
at: crates/viola-agent-claude/src/ledger.rs:1377-1380
quote: if SIGNATURES.holds_any(r) { r.clone() } else { String::new()
@row_add n=693 row=contract/unclean-fixture-refusal/message area=occupied-resources rec=fct-3fjm2cf6@2
boundary: unclean fixture refusal
item: message
promise: An unclean recording is refused whole as `unable: a recorded fixture is not clean: <file> <code>`, or `<file> row <n>[ seam] <code>` for a screen; the line never names the content.
@row_check n=693 row=contract/unclean-fixture-refusal/message sha=bafebd092fc2 v=holds judge=sonnet head=8834195b0683
at: src/cmd/verify.rs:626
quote: &format!("a recorded fixture is not clean: {named}"),
at: src/cmd/verify.rs:603
quote: return Err(format!("{name} row {}{at} {}", fault.row, fault.why.code()));
@row_add n=694 row=contract/unclean-fixture-refusal/home-path area=occupied-resources rec=fct-3fjm2cf6@2
boundary: unclean fixture refusal
item: home-path
promise: The code given when a recording holds a home path.
@row_check n=694 row=contract/unclean-fixture-refusal/home-path sha=b067dcf6f0b9 v=holds judge=sonnet head=8834195b0683
at: crates/viola-agent-claude/src/ledger.rs:1306
quote: Self::HomePath => "home-path",
@row_add n=695 row=contract/unclean-fixture-refusal/absolute-path area=occupied-resources rec=fct-3fjm2cf6@2
boundary: unclean fixture refusal
item: absolute-path
promise: The code given when a recording holds an absolute path.
@row_check n=695 row=contract/unclean-fixture-refusal/absolute-path sha=aad6eb8af0fb v=holds judge=sonnet head=8834195b0683
at: crates/viola-agent-claude/src/ledger.rs:1307
quote: Self::AbsolutePath => "absolute-path",
@row_add n=696 row=contract/unclean-fixture-refusal/username area=occupied-resources rec=fct-3fjm2cf6@2
boundary: unclean fixture refusal
item: username
promise: The code given when a recording holds the username, including one split across a screen row seam.
@row_check n=696 row=contract/unclean-fixture-refusal/username sha=38c2b7f9758d v=holds judge=sonnet head=8834195b0683
at: crates/viola-agent-claude/src/ledger.rs:1308
quote: Self::Username => "username",
at: src/cmd/verify.rs:602
quote: let at = if fault.seam { " seam" } else { "" };
@row_add n=697 row=contract/unclean-fixture-refusal/email area=occupied-resources rec=fct-3fjm2cf6@2
boundary: unclean fixture refusal
item: email
promise: The code given when a recording holds an email-shaped token.
@row_check n=697 row=contract/unclean-fixture-refusal/email sha=e04b1df0fd40 v=holds judge=sonnet head=8834195b0683
at: crates/viola-agent-claude/src/ledger.rs:1309
quote: Self::Email => "email",
@row_add n=698 row=decision/fixture-keeps-own-repository-remote area=occupied-resources rec=fct-3fjm2cf6@2
name: Fixture keeps this repository's own remote
rule: A workspace.repo naming this repository's own remote is kept in a fixture rather than scrubbed.
why: ratified
@row_check n=698 row=decision/fixture-keeps-own-repository-remote sha=eb7fc4de32a9 v=holds judge=sonnet head=8834195b0683
at: fixtures/claude/2.1.287/StatusLine.default-1.json:1
quote: {"session_id":"70ab8296-ceaf-479f-a393-18d354771fe3","transcript_path":"~/.claude/projects/-home-<user>-dev-projects-viola--viola-verify-2261248/70ab8296-ceaf-479f-a393-18d354771fe3.jsonl","cwd":"~/de
@row_add n=699 row=decision/chunk-diff-secret-scan-skip area=occupied-resources rec=fct-fs7kfa27@2
name: chunk.diff is the one secret-scan skip
rule: The mutation run's chunk.diff is the one file secret-scan skips, by exact path, and the harness artifact upload excludes it.
@row_check n=699 row=decision/chunk-diff-secret-scan-skip sha=00c6c386cd9a v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/secret_scan.rs:79-80
quote: let mutation_diff = roots.agent_run.join("chunk.diff"); capture.retain(|path| *path != mutation_diff);
at: .github/workflows/ci.yml:94
quote: !target/agent-run/chunk.diff
@row_add n=700 row=decision/test-home-owner-sweep area=occupied-resources rec=fct-xib5fpty@2
name: Test homes removed by owner record only
rule: A test home sweep removes only dirs whose recorded owner process is gone, never by age or name and never a dir without a record, and every removal deletes the owner record last.
why: a removal that stops part-way keeps the record for a later sweep
@row_check n=700 row=decision/test-home-owner-sweep sha=6097e84d8be5 v=holds judge=sonnet head=8834195b0683
at: tests/support/home.rs:72-75
quote: if process_start(pid) == Some(started_at) { continue; } if remove_owned(&dir) {
at: tests/support/home.rs:100
quote: whole && fs::remove_file(dir.join(OWNER)).is_ok() && fs::remove_dir(dir).is_ok()
@row_add n=701 row=decision/linked-e2e-home-base-strict area=occupied-resources rec=fct-xib5fpty@2
name: Linked e2e-home base must be a private real directory
rule: A linked e2e-home base must have an absolute target, created with mode 0700 when gone and never chmodded after, and is accepted only when it reads as a real directory with no group or other bit; anything else refuses the start with a fixed message naming no path.
@row_check n=701 row=decision/linked-e2e-home-base-strict sha=11a3c4129077 v=holds judge=sonnet head=8834195b0683
at: tests/support/home.rs:128
quote: let _ = fs::DirBuilder::new().mode(0o700).create(&target);
at: tests/support/home.rs:135
quote: meta.permissions().mode() & 0o077 == 0,
at: crates/viola-e2e/src/harness/mod.rs:120
quote: if meta.permissions().mode() & 0o077 != 0 {
@row_add n=702 row=decision/e2e-home-headroom-floor area=occupied-resources rec=fct-xib5fpty@2
name: e2e-home backing headroom floor
rule: A linked e2e-home base whose filesystem has less than 9 GiB free is refused; the floor is one compiled constant, equal in both keepers and set by no variable, flag or config key.
why: the tmpfs's EDQUOT reading of about 8 GiB free plus 1 GiB
@row_check n=702 row=decision/e2e-home-headroom-floor sha=38a6cd61e43e v=holds judge=sonnet head=8834195b0683
at: tests/support/home.rs:152
quote: pub const HEADROOM_FLOOR_MIB: u64 = 9 * 1024;
at: crates/viola-e2e/src/harness/mod.rs:151
quote: pub const HEADROOM_FLOOR_MIB: u64 = 9 * 1024;
@row_add n=703 row=contract/e2e-home-refusal/headroom-message area=occupied-resources rec=fct-xib5fpty@2
boundary: e2e-home base refusal
item: headroom message
promise: A base below the floor is refused with the fixed message `e2e-home backing: {free} MiB free, below the {floor} MiB floor`, the two counts and no path.
@row_check n=703 row=contract/e2e-home-refusal/headroom-message sha=85a1b345ebb6 v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/mod.rs:157
quote: .then(|| format!("e2e-home backing: {free_mib} MiB free, below the {floor_mib} MiB floor"))
at: tests/support/home.rs:158
quote: .then(|| format!("e2e-home backing: {free_mib} MiB free, below the {floor_mib} MiB floor"))
@row_add n=704 row=decision/dev-host-e2e-home-on-tmpfs area=occupied-resources rec=fct-xib5fpty@2
name: Dev-host test homes on a tmpfs link
rule: On the Linux dev host the e2e-home base is a link to a tmpfs, an environment fact named by no variable, flag or config key; the home path strings are unchanged and CI homes stay on the runner's disk.
why: so a start's pinned copy does not wait on the shared volume's dirty-page budget
@row_check n=704 row=decision/dev-host-e2e-home-on-tmpfs sha=6b8ca9c6c826 v=unverifiable judge=sonnet head=8834195b0683
why: not a code claim
@row_add n=705 row=decision/test-side-removal-outside-workdir area=occupied-resources rec=fct-xib5fpty@2
name: Test-side removals outside the working directory
rule: Through the e2e-home link, the owner sweep, a test home's drop and harness cleanup create and delete outside the working directory, each taking only a dir the test side itself made.
why: a boundary widening, ratified
@row_check n=705 row=decision/test-side-removal-outside-workdir sha=86127c3d1b5d v=holds judge=sonnet head=8834195b0683
at: tests/support/home.rs:86
quote: pub fn remove_owned(dir: &Path) -> bool {
at: crates/viola-e2e/src/harness/cleanup.rs:161
quote: .is_some_and(|n| n.starts_with("viola-session-"));
@row_add n=706 row=decision/profraw-census-dev-host-only area=occupied-resources rec=fct-mtiut4fw@1
name: Profraw census is a dev-host witness only
rule: The profraw census script is a dev-host witness for the corrupt-profile mechanism, and no CI job, pre-push stage or harness command runs either of its forms.
@row_check n=706 row=decision/profraw-census-dev-host-only sha=deeb9e61e3c8 v=holds judge=sonnet head=8834195b0683
searched: profraw-census · in .github/**
searched: profraw-census · in crates/**/*.rs and src/**/*.rs
searched: profraw-census · in scripts/** (only scripts/profraw-census.sh itself names it)
@row_add n=707 row=contract/profraw-census-exit-codes/0 area=occupied-resources rec=fct-mtiut4fw@1
boundary: profraw census exit codes
item: 0
promise: Every run passed and, in the default form, the profile count equals the run count with no third or short profile; in the case form, no profile was refused.
@row_check n=707 row=contract/profraw-census-exit-codes/0 sha=69c64e06b6b3 v=holds judge=sonnet head=8834195b0683
at: scripts/profraw-census.sh:178-180
quote: if [ "$n" -eq "$runs" ] && [ "$passed" -eq "$n" ] && [ "$profiles" -eq "$n" ] && [ "$third" -eq 0 ] \ && [ "$short" -eq 0 ]; then exit 0
at: scripts/profraw-census.sh:162-163
quote: if [ "$n" -eq "$runs" ] && [ "$passed" -eq "$n" ] && [ "$refused" -eq 0 ]; then exit 0
@row_add n=708 row=contract/profraw-census-exit-codes/1 area=occupied-resources rec=fct-mtiut4fw@1
boundary: profraw census exit codes
item: 1
promise: Any other count.
@row_check n=708 row=contract/profraw-census-exit-codes/1 sha=ec9beefac9ac v=holds judge=sonnet head=8834195b0683
at: scripts/profraw-census.sh:182
quote: exit 1
at: scripts/profraw-census.sh:165
quote: exit 1
@row_add n=709 row=contract/profraw-census-exit-codes/2 area=occupied-resources rec=fct-mtiut4fw@1
boundary: profraw census exit codes
item: 2
promise: A usage error, no instrumented binary, or (case form) no llvm-profdata.
@row_check n=709 row=contract/profraw-census-exit-codes/2 sha=1f5f8fb3c4ee v=holds judge=sonnet head=8834195b0683
at: scripts/profraw-census.sh:25
quote: exit 2
at: scripts/profraw-census.sh:75
quote: exit 2
at: scripts/profraw-census.sh:82
quote: exit 2
@row_add n=710 row=decision/mutation-scratch-nocow area=occupied-resources rec=fct-xtqblyzi@2
name: Mutation scratch directory is NOCOW
rule: On the Linux dev host every mutation run's TMPDIR is a sibling directory outside the repository marked NOCOW, an environment fact and not harness code.
why: /tmp is a quota tmpfs too small for the copied tree, and a COW btrfs reflink copy drops the fake agent's exec bit; NOCOW makes the copy fall back to a mode-keeping copy
@row_check n=710 row=decision/mutation-scratch-nocow sha=676144612870 v=unverifiable judge=sonnet head=8834195b0683
why: not a code claim
@row_add n=711 row=decision/rust-toolchain-pin area=stack-and-technologies rec=fct-nuruygpl@2
name: Rust toolchain pinned exactly
rule: Rust is the language of one native binary, pinned exactly by rust-toolchain.toml with rustfmt and clippy, edition 2024.
@row_check n=711 row=decision/rust-toolchain-pin sha=8b14dced8a81 v=holds judge=sonnet head=8834195b0683
at: rust-toolchain.toml:2-3
quote: channel = "1.98.1" components = ["rustfmt", "clippy"]
at: Cargo.toml:187
quote: edition = "2024"
@row_add n=712 row=decision/msrv-floor area=stack-and-technologies rec=fct-nuruygpl@2
name: Declared MSRV floor 1.96
rule: The workspace declares rust-version 1.96 and CI builds against that floor without touching the toolchain pin.
why: the security toolchain floor
guard: msrv CI job
@row_check n=712 row=decision/msrv-floor sha=02699568a381 v=holds guard=true judge=sonnet head=8834195b0683
at: Cargo.toml:188
quote: rust-version = "1.96"
at: .github/workflows/ci.yml:203
quote: run: rustup toolchain install 1.96 --profile minimal
at: .github/workflows/ci.yml:214
quote: RUSTUP_TOOLCHAIN: "1.96"
guard_at: .github/workflows/ci.yml:191
@row_add n=713 row=decision/fuzz-nightly-toolchain area=stack-and-technologies rec=fct-nuruygpl@2
name: Fuzz workspace alone on nightly
rule: Only the separate fuzz workspace builds on a pinned nightly toolchain.
@row_check n=713 row=decision/fuzz-nightly-toolchain sha=c95e8bc06fb5 v=holds judge=sonnet head=8834195b0683
at: fuzz/rust-toolchain.toml:2
quote: channel = "nightly-2026-09-20"
searched: nightly · in rust-toolchain.toml, .github/workflows/*, .cargo/*, scripts/*.sh, clippy.toml, rustfmt.toml (only a ci.yml:369 comment naming fuzz/rust-toolchain.toml's nightly; root pin is 1.98.1)
@row_add n=714 row=decision/hybrid-concurrency area=stack-and-technologies rec=fct-7z3oulbd@1
name: Std threads everywhere, Tokio only inside mcp
rule: Every CLI verb runs on std threads; Tokio is built only inside mcp.
why: no async runtime on the hot hook path; the async ecosystem is used only where rmcp requires it
@row_check n=714 row=decision/hybrid-concurrency sha=efaa02b9d768 v=holds judge=sonnet head=8834195b0683
at: crates/viola-mcp/src/lib.rs:70
quote: let runtime = tokio::runtime::Builder::new_current_thread()
at: crates/viola-mcp/Cargo.toml:23
quote: tokio = { workspace = true, features = ["rt"] }
at: Cargo.toml:238-239
quote: # viola-mcp's runtime and viola-channel's `tokio` client only: every sync crate's own graph stays # free of it (deny-sync.toml, scripts/channel-tokio-free.sh). Each user names its features.
@row_add n=715 row=decision/clap-cli-parser area=stack-and-technologies rec=fct-rd72s73l@1
name: clap as CLI parser, without colour
rule: clap (derive) parses the CLI with its default features minus color.
why: help and usage output stays plain and no colour or terminal crate is in the graph
@row_check n=715 row=decision/clap-cli-parser sha=5e15c74df470 v=holds judge=sonnet head=8834195b0683
at: Cargo.toml:197-204
quote: clap = { version = "=4.6.7", default-features = false, features = [ "std", "derive", "help", "usage", "error-context", "suggestions", ] }
@row_add n=716 row=decision/portable-pty-seam area=stack-and-technologies rec=fct-kxiazeyf@3
name: portable-pty behind viola's own pty seam
rule: The unmodified claude is hosted in openpty through a pinned portable-pty behind viola's own pty seam, with libc for the host tty's raw mode.
@row_check n=716 row=decision/portable-pty-seam sha=2a53867dc0a3 v=holds judge=sonnet head=8834195b0683
at: Cargo.toml:217
quote: portable-pty = "=0.9.0"
at: crates/viola-pty/src/lib.rs:132
quote: .openpty(pty_size(spec.size))
at: crates/viola-pty/src/lib.rs:234
quote: libc::cfmakeraw(&mut raw);
@row_add n=717 row=decision/libc-only-os-binding area=stack-and-technologies rec=fct-iw73rjlh@1
name: libc is the one OS-binding crate
rule: libc is the one OS-binding crate of the Linux-only build, a plain dependency; no crate depends on windows-sys.
@row_check n=717 row=decision/libc-only-os-binding sha=058815f25957 v=holds judge=sonnet head=8834195b0683
at: Cargo.toml:213
quote: libc = "=0.2.189"
at: crates/viola-pty/Cargo.toml:15
quote: libc.workspace = true
searched: windows-sys · in Cargo.toml, crates/*/Cargo.toml (none); Cargo.lock lists it only under third-party packages, none of the viola* package entries (lines 1776-1920)
@row_add n=718 row=decision/vt100-screen-model area=stack-and-technologies rec=fct-jucr2aui@1
name: vt100 as screen model
rule: vt100 gives pre-send readiness and modal detection on run's feed thread and the screen of verify's interactive probe runs.
@row_check n=718 row=decision/vt100-screen-model sha=70ab5b74fd95 v=holds judge=sonnet head=8834195b0683
at: src/run/gate.rs:291
quote: screen: Screen::new(size.rows, size.cols, now),
at: src/cmd/verify/typed/pty.rs:144
quote: let rows = feed.screen.rows();
at: crates/viola-agent-claude/Cargo.toml:22
quote: vt100.workspace = true
@row_add n=719 row=decision/run-never-reads-screen-content area=stack-and-technologies rec=fct-jucr2aui@1
name: run never reads screen content
rule: run never reads screen content; the only row-text exit of the screen is read by verify alone, for its signature-only recorded screen fixtures.
@row_check n=719 row=decision/run-never-reads-screen-content sha=3567b05a980a v=holds judge=sonnet head=8834195b0683
searched: \.rows\(\) · in src/**/*.rs and crates/*/src/**/*.rs (only src/cmd/verify/typed/pty.rs:144 and :177 outside Screen's own tests)
searched: pub fn rows · in crates/viola-agent-claude/src/screen.rs (the one definition, line 123)
@row_add n=720 row=decision/wrapper-ipc-local-socket area=stack-and-technologies rec=fct-jsy3xdhu@1
name: interprocess local sockets for wrapper IPC
rule: Each viola run has one endpoint, a Unix domain socket through interprocess's sync local_socket API, chmod 0600 after bind in a per-user directory; the Tokio flavour is used only by the MCP client.
@row_check n=720 row=decision/wrapper-ipc-local-socket sha=abcfc2eeb36d v=holds judge=sonnet head=8834195b0683
at: crates/viola-channel/src/server.rs:126
quote: .create_sync()
at: crates/viola-channel/src/server.rs:134
quote: fs::set_permissions(&guard.socket, Permissions::from_mode(0o600))
at: crates/viola-channel/Cargo.toml:17
quote: tokio = ["dep:tokio", "interprocess/tokio"]
at: crates/viola-mcp/Cargo.toml:16
quote: viola-channel = { path = "../viola-channel", features = ["tokio"] }
@row_add n=721 row=decision/socket-dir-strict-check area=stack-and-technologies rec=fct-jsy3xdhu@1,fct-rvenzu5e@1
name: Socket directory checked before bind and connect
rule: The recorded endpoint's directory is checked before a bind and before each client's connect; a symlink, a non-directory, another owner or a group or other bit is refused and never chmodded, an absent path passes, and a refused directory writes no frame.
@row_check n=721 row=decision/socket-dir-strict-check sha=7aac3677edb8 v=holds judge=sonnet head=8834195b0683
at: crates/viola-state/src/strict.rs:137-143
quote: pub fn check_socket_dir(dir: &Path) -> Result<(), Refused> { use std::os::unix::fs::MetadataExt as _; let meta = match std::fs::symlink_metadata(dir) { Ok(meta) => meta, Err(e) if e.kind() == io::ErrorKind::NotFound => return Ok(()),
at: crates/viola-state/src/strict.rs:168-174
quote: if is_symlink { return Err(Refused::Symlink); } if !is_dir { return Err(Refused::NotADirectory); } unix_read_verdict(mode, uid, euid)
at: src/cmd/run.rs:511
quote: if check_endpoint_dir(endpoint).is_err() {
at: src/cmd/client.rs:173-174
quote: if check_endpoint_dir(&live.endpoint).is_err() { return Err(CallError::SocketDirRefused);
at: src/cmd/hook.rs:231-232
quote: if check_endpoint_dir(endpoint).is_err() { return Err(NotSent::SocketDir);
at: crates/viola-mcp/src/tools.rs:345-346
quote: if check_endpoint_dir(&live.endpoint).is_err() { return Ok(result::unreachable(UnreachableCause::SocketDir));
@row_add n=722 row=decision/no-database area=stack-and-technologies rec=fct-qzebh4lx@1
name: No database: ndjson logs and JSON snapshots
rule: State lives in ndjson append logs and atomically replaced JSON snapshots on the local filesystem; the log is the authoritative audit trail.
@row_check n=722 row=decision/no-database sha=d9aeee7150ce v=holds judge=sonnet head=8834195b0683
at: crates/viola-state/src/fs.rs:83
quote: tmp.persist(path).map(|_| ()).map_err(|e| e.error.into())
searched: sqlite|rusqlite|sled|redb|rocksdb · in Cargo.toml (workspace.dependencies; no matches)
@row_add n=723 row=decision/versioned-records-no-migrations area=stack-and-technologies rec=fct-rzlme45f@1
name: Versioned records instead of migrations
rule: Every record carries v, readers skip unknown kinds and fields, and a snapshot with an unsupported v is rebuilt by replaying the log.
except: process-log lines carry no v; their version lives in the schema filename, because obs's default-deny field list admits no v key
@row_check n=723 row=decision/versioned-records-no-migrations sha=855d410d732e v=holds judge=sonnet head=8834195b0683
at: crates/viola-state/src/snapshot.rs:140-141
quote: if v > u64::from(SNAPSHOT_V) { return SnapshotRead::Unsupported { v_seen: v };
at: crates/viola-state/src/replay.rs:155
quote: SnapshotRead::Unsupported { v_seen } => ReplayCause::Unsupported { v_seen },
at: schemas/diag-line.v1.json:7
quote: "required": ["timestamp", "level", "target", "message", "event", "process"],
@row_add n=724 row=decision/atomic-replaced-state-files area=stack-and-technologies rec=fct-s7lo3z2q@2
name: Replaced files through one atomic helper
rule: Snapshots and every other replaced file go through one shared helper that sets the mode on a temp file before the write, syncs, and persists it in one attempt.
why: crash-safe multi-process writes with no C code
@row_check n=724 row=decision/atomic-replaced-state-files sha=63b5efa3c786 v=holds judge=sonnet head=8834195b0683
at: crates/viola-state/src/fs.rs:77-83
quote: pub fn replace_private(path: &Path, bytes: &[u8], mode: u32) -> Result<(), StateError> { let parent = path.parent().unwrap_or(Path::new(".")); let mut tmp = tempfile::NamedTempFile::new_in(parent)?; restrict(tmp.path(), mode)?; tmp.write_all(bytes)?; tmp.as_file().sync_all()?; tmp.persist(path)
at: crates/viola-state/src/fs.rs:131
quote: match replace_private(path, bytes, mode) {
@row_add n=725 row=decision/pinned-copy-placed-once area=stack-and-technologies rec=fct-s7lo3z2q@2
name: Pinned copy placed once, never replaced
rule: The pinned copy's absent-copy write is a no-clobber persist after the write handle closes; the copy is placed once and never replaced.
@row_check n=725 row=decision/pinned-copy-placed-once sha=91a1d05f2934 v=holds judge=sonnet head=8834195b0683
at: crates/viola-state/src/fs.rs:117-118
quote: tmp.as_file().sync_all()?; match tmp.into_temp_path().persist_noclobber(path) {
at: crates/viola-state/src/fs.rs:123
quote: Err(e) if e.error.kind() == io::ErrorKind::AlreadyExists => Ok(Placed::Present),
@row_add n=726 row=decision/append-one-write-per-line area=stack-and-technologies rec=fct-s7lo3z2q@2
name: Log appends one write per line
rule: Log appends use std append mode with one write per line.
why: crash-safe multi-process writes with no C code
@row_check n=726 row=decision/append-one-write-per-line sha=a0424dd1f4be v=holds judge=sonnet head=8834195b0683
at: crates/viola-state/src/fs.rs:46
quote: options.create(true).append(true);
at: crates/viola-state/src/events.rs:332
quote: file.write_all(bytes)?;
@row_add n=727 row=decision/content-hash-sha256 area=stack-and-technologies rec=fct-itjxddea@2
name: Pure-Rust SHA-256 content hash
rule: The bin/ and plugin/ version-hash keys are the exe's SHA-256 truncated to 8 bytes (16 hex), through sha2 with no cc.
@row_check n=727 row=decision/content-hash-sha256 sha=e478f5ea3294 v=holds judge=sonnet head=8834195b0683
at: crates/viola-state/src/pin.rs:42
quote: digest[..8].iter().map(|b| format!("{b:02x}")).collect()
at: Cargo.toml:232
quote: sha2 = { version = "=0.11.0", default-features = false }
@row_add n=728 row=decision/pinned-copy-rehash-before-reuse area=stack-and-technologies rec=fct-itjxddea@2
name: Pinned copy re-hashed before every reuse
rule: An existing pinned copy is re-hashed before every reuse.
@row_check n=728 row=decision/pinned-copy-rehash-before-reuse sha=9f119ad00f56 v=holds judge=sonnet head=8834195b0683
at: crates/viola-state/src/pin.rs:88-90
quote: match file_key(&path) { Ok(found) if found == hash => {} Ok(_) => return Err(PinError::HashMismatch),
@row_add n=729 row=decision/sha2-optimized-in-dev area=stack-and-technologies rec=fct-itjxddea@2
name: sha2 optimised in dev builds
rule: The dev profile builds sha2 at opt-level 3.
why: dev and test builds hash the debug exe at release speed
@row_check n=729 row=decision/sha2-optimized-in-dev sha=9d890349481b v=holds judge=sonnet head=8834195b0683
at: Cargo.toml:265-266
quote: [profile.dev.package.sha2] opt-level = 3
@row_add n=730 row=decision/serde-serialization area=stack-and-technologies rec=fct-f6zprgcc@1
name: serde for frames, logs and snapshots
rule: serde and serde_json serialize channel frames, log events and snapshots, and parse external payloads tolerantly with path-precise drift reports.
@row_check n=730 row=decision/serde-serialization sha=0d74a390bf04 v=holds judge=sonnet head=8834195b0683
at: Cargo.toml:228
quote: serde_json = { version = "=1.0.151", features = ["preserve_order"] }
at: Cargo.toml:230
quote: serde_path_to_error = "=0.1.20"
at: crates/viola-agent-claude/src/hook.rs:138
quote: let error = match serde_path_to_error::deserialize::<_, Known>(&*payload) {
@row_add n=731 row=decision/preserve-key-order area=stack-and-technologies rec=fct-f6zprgcc@1
name: Printed documents keep declared key order
rule: serde_json's preserve_order feature is on, so a printed document keeps its declared key order.
@row_check n=731 row=decision/preserve-key-order sha=2c25cbcac618 v=holds judge=sonnet head=8834195b0683
at: Cargo.toml:228
quote: serde_json = { version = "=1.0.151", features = ["preserve_order"] }
@row_add n=732 row=decision/no-unbounded-json-depth area=stack-and-technologies rec=fct-f6zprgcc@1
name: serde_json unbounded_depth banned
rule: serde_json's unbounded_depth feature is banned.
why: every parse of external bytes keeps serde_json's default recursion limit
guard: deny.toml
@row_check n=732 row=decision/no-unbounded-json-depth sha=eec99df0ded5 v=holds guard=true judge=sonnet head=8834195b0683
at: deny.toml:55
quote: deny = ["unbounded_depth"]
guard_at: deny.toml:55
@row_add n=733 row=decision/tracing-process-logs area=stack-and-technologies rec=fct-zse22tqm@1
name: tracing for one-line JSON process logs
rule: Process logs are one-line JSON written by tracing into the viola home's diagnostics, with the root bin the only product dependent of tracing-subscriber; the line format and levels are owned by the obs plan.
@row_check n=733 row=decision/tracing-process-logs sha=c0f3778d3685 v=holds judge=sonnet head=8834195b0683
at: src/obs.rs:120-121
quote: let layer = tracing_subscriber::fmt::layer() .json()
at: Cargo.toml:161
quote: tracing-subscriber.workspace = true
@row_add n=734 row=decision/frame-params-redacted area=stack-and-technologies rec=fct-zse22tqm@1
name: Frame params never rendered in Debug
rule: A frame's params never reach a Debug render, through veil's derived redaction with its toggle feature banned.
@row_check n=734 row=decision/frame-params-redacted sha=c269266da708 v=holds judge=sonnet head=8834195b0683
at: crates/viola-channel/src/frame.rs:51
quote: #[derive(Deserialize, Redact)]
at: deny.toml:40
quote: deny = ["toggle"]
@row_add n=735 row=decision/nutype-domain-newtypes area=stack-and-technologies rec=fct-dy7nnp4c@1
name: nutype domain newtypes
rule: Bounded domain values are nutype newtypes: ViolaName (charset and length cap), Percent (0 to 100), DialogHold (whole seconds 60 to 3600).
@row_check n=735 row=decision/nutype-domain-newtypes sha=90851d2be91e v=holds judge=sonnet head=8834195b0683
at: crates/viola-core/src/lib.rs:49
quote: validate(predicate = is_valid_name),
at: crates/viola-core/src/lib.rs:268
quote: validate(less_or_equal = 100),
at: crates/viola-core/src/lib.rs:277
quote: validate(greater_or_equal = 60, less_or_equal = 3600),
@row_add n=736 row=decision/chrono-timestamps area=stack-and-technologies rec=fct-eaidxga3@1
name: chrono RFC 3339 UTC timestamps
rule: Timestamps are RFC 3339 UTC with millisecond precision, through chrono.
why: already in the tree through rmcp
@row_check n=736 row=decision/chrono-timestamps sha=5dcb10de89c8 v=holds judge=sonnet head=8834195b0683
at: crates/viola-state/src/lib.rs:31
quote: at.to_rfc3339_opts(SecondsFormat::Millis, true)
@row_add n=737 row=decision/typed-errors-per-crate area=stack-and-technologies rec=fct-r7pka372@1
name: thiserror per crate, anyhow at the bin edge
rule: Each crate has typed errors callers can branch on through thiserror; anyhow is used at the bin edge only.
except: viola-pty's error type is hand-written
@row_check n=737 row=decision/typed-errors-per-crate sha=d473beb56127 v=holds judge=sonnet head=8834195b0683
at: crates/viola-state/src/lib.rs:21
quote: #[derive(Debug, thiserror::Error)]
at: crates/viola-pty/src/lib.rs:96
quote: impl std::error::Error for PtyError {
at: src/obs.rs:256
quote: pub(crate) fn report_internal_error(error: &anyhow::Error, sink: Option<&DetailSink>) {
@row_add n=738 row=decision/error-context-detail-file-only area=stack-and-technologies rec=fct-r7pka372@1
name: Error context chains only in the detail file
rule: Error context chains are built at dispatch and recorded only in the owner-only instance detail file.
@row_check n=738 row=decision/error-context-detail-file-only sha=caaca0934d08 v=holds judge=sonnet head=8834195b0683
at: src/obs.rs:259-260
quote: let line = chain_detail_line(&timestamp(Utc::now()), sink.process, &sink.instance, error); write_detail(&sink.home, &sink.instance, sink.process, &line);
@row_add n=739 row=decision/no-message-broker area=stack-and-technologies rec=fct-cy7u4li4@1
name: No message broker
rule: Request and response go over per-wrapper local sockets as JSON-RPC 2.0 over ndjson, with no broker or daemon.
@row_check n=739 row=decision/no-message-broker sha=4767a2f19ff7 v=holds judge=sonnet head=8834195b0683
at: crates/viola-channel/src/frame.rs:95
quote: json!({"jsonrpc": "2.0", "id": id, "result": result})
at: crates/viola-channel/src/server.rs:123
quote: let listener = ListenerOptions::new()
@row_add n=740 row=decision/process-liveness-signals area=stack-and-technologies rec=fct-cnnrnbfw@1
name: Heartbeat first, sysinfo and claude agents enrich
rule: The heartbeat file is the primary liveness signal, enriched by sysinfo's pid and process start time and by claude agents --json.
@row_check n=740 row=decision/process-liveness-signals sha=4c203faeb3d7 v=holds judge=sonnet head=8834195b0683
at: crates/viola-state/src/liveness.rs:32
quote: Some(age) if age <= STALE_AFTER => Liveness::Live,
at: crates/viola-state/src/liveness.rs:45
quote: .map(|(pid, p)| (pid.as_u32(), p.start_time()))
at: crates/viola-agent-claude/src/agents.rs:10
quote: pub const ARGS: [&str; 2] = ["agents", "--json"];
@row_add n=741 row=decision/rmcp-release-features area=stack-and-technologies rec=fct-qeiflw5v@1
name: rmcp release features exactly server and transport-io
rule: The release graph's rmcp features are exactly server and transport-io.
guard: scripts/rmcp-features.sh
@row_check n=741 row=decision/rmcp-release-features sha=35cf78975905 v=holds guard=true judge=sonnet head=8834195b0683
at: Cargo.toml:223
quote: rmcp = { version = ">=3.4.1, <3.5", default-features = false, features = ["server", "transport-io"] }
guard_at: scripts/rmcp-features.sh:33
@row_add n=742 row=decision/e2e-no-rmcp-child-transport area=stack-and-technologies rec=fct-qeiflw5v@1
name: E2E owns the viola mcp child
rule: The test-only E2E crate takes rmcp client alone, never transport-child-process, and owns the viola mcp child and awaits its exit.
why: transport-child-process kills the server 3 s after a close
@row_check n=742 row=decision/e2e-no-rmcp-child-transport sha=99d8e394bb6a v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/Cargo.toml:44
quote: rmcp = { workspace = true, features = ["client"] }
at: crates/viola-e2e/tests/mcp_tools.rs:137
quote: let _ = self.runtime.block_on(self.child.wait());
@row_add n=743 row=absence/model-api-calls area=stack-and-technologies rec=fct-24lf33ny@1
capability: Calling any model API; viola drives the unmodified claude CLI on the user's subscription
@row_check n=743 row=absence/model-api-calls sha=16bd4199f668 v=holds judge=sonnet head=8834195b0683
searched: api.anthropic · in Cargo.toml, crates/*/Cargo.toml, src/**/*.rs, crates/*/src/**/*.rs
searched: ANTHROPIC_API_KEY · in Cargo.toml, crates/*/Cargo.toml, src/**/*.rs, crates/*/src/**/*.rs
searched: reqwest · in Cargo.toml, crates/*/Cargo.toml, src/**/*.rs, crates/*/src/**/*.rs
searched: openai · in Cargo.toml, crates/*/Cargo.toml, src/**/*.rs, crates/*/src/**/*.rs
@row_add n=744 row=decision/local-install-no-hosting area=stack-and-technologies rec=fct-5oigicbx@1
name: Local-only install, no hosting
rule: v1 is installed locally with cargo install and has no container, hosting or deployment.
@row_check n=744 row=decision/local-install-no-hosting sha=ec9661c4e1c4 v=holds judge=sonnet head=8834195b0683
searched: Dockerfile · in Dockerfile*, docker-compose*, .github/workflows/*.yml
searched: docker · in README.md, Cargo.toml, .github/workflows/*.yml
@row_add n=745 row=decision/ci-ubuntu-fake-agent area=stack-and-technologies rec=fct-sb4r6c4a@2
name: CI on ubuntu against the fake agent
rule: CI builds, lints and tests on ubuntu-latest native runners against the fake agent, with SHA-pinned actions and toolchains installed by rustup.
@row_check n=745 row=decision/ci-ubuntu-fake-agent sha=d0611181c0be v=holds judge=sonnet head=8834195b0683
at: .github/workflows/ci.yml:18
quote: os: [ubuntu-latest]
at: .github/workflows/ci.yml:22
quote: - uses: actions/checkout@3d3c42e5aac5ba805825da76410c181273ba90b1 # v7.0.1
at: .github/workflows/ci.yml:26
quote: run: rustup toolchain install
at: .github/workflows/ci.yml:281
quote: run: cargo clippy --workspace --all-targets --features fake-agent -- -D warnings
@row_add n=746 row=decision/pre-push-filter-not-verdict area=stack-and-technologies rec=fct-sb4r6c4a@2
name: Local pre-push gate is a filter, not the verdict
rule: The local pre-push gate runs the ubuntu test job's suites natively in the working tree under a cleared environment, refuses off Linux and installs nothing; it is a pre-push filter, never the verdict of record.
@row_check n=746 row=decision/pre-push-filter-not-verdict sha=a26344e1f228 v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/pre_push.rs:54
quote: return doc.finish(Err(Stop::new("pre-push-linux-only", "")), 2);
at: crates/viola-e2e/src/harness/pre_push/linux.rs:39
quote: cmd.arg("-i")
at: crates/viola-e2e/src/harness/pre_push/linux.rs:105
quote: if out(runner, linux.cmd(&["cc", "--version"])).is_none() {
@row_add n=747 row=decision/clippy-print-and-macro-bans area=stack-and-technologies rec=fct-lj7tqt64@2
name: clippy deny warnings with print and dbg bans
rule: clippy runs with warnings denied, the workspace bans on print_stdout, print_stderr and dbg_macro, and disallowed tracing level macros.
@row_check n=747 row=decision/clippy-print-and-macro-bans sha=38eb2ce24245 v=holds judge=sonnet head=8834195b0683
at: .github/workflows/ci.yml:281
quote: run: cargo clippy --workspace --all-targets --features fake-agent -- -D warnings
at: Cargo.toml:256
quote: print_stdout = "deny"
at: Cargo.toml:257
quote: print_stderr = "deny"
at: Cargo.toml:258
quote: dbg_macro = "deny"
at: clippy.toml:5
quote: { path = "tracing::info", reason = "log through viola_core::obs_event! (obs-plan §3)" },
@row_add n=748 row=decision/cargo-deny-dependency-policy area=stack-and-technologies rec=fct-lj7tqt64@2
name: cargo-deny dependency policy
rule: cargo-deny enforces advisories, licences, sources and bans: C crates, telemetry crates, feature bans, and tokio banned per sync crate.
@row_check n=748 row=decision/cargo-deny-dependency-policy sha=85f7fe5232c4 v=holds judge=sonnet head=8834195b0683
at: deny.toml:7
quote: [advisories]
at: deny.toml:13
quote: [licenses]
at: deny.toml:23
quote: [sources]
at: deny.toml:29
quote: { crate = "cc", reason = "no C in the build" },
at: deny.toml:32
quote: { crate = "opentelemetry-otlp", reason = "no telemetry exporter or egress in v1" },
at: deny.toml:38
quote: [[bans.features]]
at: deny-sync.toml:12
quote: { crate = "tokio", reason = "tokio is confined to viola-mcp; the sync crates stay tokio-free" },
@row_add n=749 row=decision/workflow-lint-zizmor area=stack-and-technologies rec=fct-lj7tqt64@2
name: GitHub workflows linted by zizmor
rule: GitHub workflows are linted by zizmor.
@row_check n=749 row=decision/workflow-lint-zizmor sha=0d7b507cdabb v=holds judge=sonnet head=8834195b0683
at: .github/workflows/ci.yml:392
quote: zizmor --format=json .github/workflows/ > target/supply-chain/zizmor.json || { cat target/supply-chain/zizmor.json; exit 1; }
@row_add n=750 row=decision/coverage-floors-gate area=stack-and-technologies rec=fct-lj7tqt64@2
name: Coverage floors enforced by the harness gate
rule: Coverage is measured by cargo-llvm-cov and the harness gate enforces the coverage floors.
@row_check n=750 row=decision/coverage-floors-gate sha=1aa98efb12a4 v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/run/coverage.rs:25
quote: "llvm-cov",
at: crates/viola-e2e/src/harness/gate.rs:111
quote: for (metric, floor) in COVERAGE_FLOORS {
@row_add n=751 row=decision/fuzz-workspace-deny-exemption area=stack-and-technologies rec=fct-lj7tqt64@2
name: Fuzz workspace outside the cargo-deny graph
rule: Fuzzing runs in CI only, in a separate fuzz workspace with its own lockfile outside the root cargo-deny graph.
why: a test-only exemption
@row_check n=751 row=decision/fuzz-workspace-deny-exemption sha=21e6afd9fe37 v=holds judge=sonnet head=8834195b0683
at: Cargo.toml:182
quote: exclude = ["fuzz"]
at: fuzz/Cargo.toml:20-21
quote: [workspace] members = ["."]
at: .github/workflows/ci.yml:246
quote: run: cargo install --locked cargo-fuzz@0.13.2
@row_add n=752 row=decision/syn-test-only area=stack-and-technologies rec=fct-lj7tqt64@2
name: syn and proc-macro2 test-only
rule: syn and proc-macro2 are direct dependencies of the test-only E2E crate alone; no product crate depends on either.
@row_check n=752 row=decision/syn-test-only sha=4e3d60332794 v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/Cargo.toml:36
quote: proc-macro2.workspace = true
at: crates/viola-e2e/Cargo.toml:39
quote: syn.workspace = true
@row_add n=753 row=decision/release-tooling-later area=stack-and-technologies rec=fct-iro7xkpv@1
name: Release tooling deferred past v1
rule: Public signed releases and installers come with dist and cargo-auditable, and self_update later, once distribution is in scope, not in v1.
@row_check n=753 row=decision/release-tooling-later sha=c1e50543b066 v=holds judge=sonnet head=8834195b0683
searched: cargo-dist · in Cargo.toml, Cargo.lock, crates/*/Cargo.toml, .github/workflows/*.yml, dist-workspace.toml
searched: cargo-auditable · in Cargo.toml, Cargo.lock, crates/*/Cargo.toml, .github/workflows/*.yml
searched: self_update · in Cargo.toml, Cargo.lock, crates/*/Cargo.toml, .github/workflows/*.yml
@row_add n=754 row=contract/list-item/budget-paused area=standard-contracts rec=fct-dtnnvvl3@1
boundary: list payload item
item: budget_paused
promise: The instance's own budget gate from its snapshot; false after a release --budget override, absent for an unwrapped session.
@row_check n=754 row=contract/list-item/budget-paused sha=cf3bc6872e54 v=holds judge=sonnet head=8834195b0683
at: crates/viola-state/src/board.rs:238
quote: budget_paused: Some(snapshot.budget_paused),
at: src/run/budget.rs:334
quote: snapshot.budget_paused = standing.paused.is_some();
at: crates/viola-state/src/board.rs:198
quote: budget_paused: None,
@row_add n=755 row=contract/list-item/wrapped area=standard-contracts rec=fct-dtnnvvl3@1
boundary: list payload item
item: wrapped
promise: false marks a read-only unwrapped row from claude agents --json, named by its sessionId, never a valid target, live, dialog_pending false, with no wheel, budget_paused, cli_version or cli_verified.
@row_check n=755 row=contract/list-item/wrapped sha=b84f21f97807 v=holds judge=sonnet head=8834195b0683
at: crates/viola-state/src/board.rs:194
quote: wrapped: false,
@row_add n=756 row=contract/list-item/liveness area=standard-contracts rec=fct-dtnnvvl3@1
boundary: list payload item
item: liveness
promise: live or stale; gone instances are omitted.
@row_check n=756 row=contract/list-item/liveness sha=1cb4724a1e8a v=holds judge=sonnet head=8834195b0683
at: crates/viola-state/src/board.rs:276-278
quote: Liveness::Live => Some(RowLiveness::Live), Liveness::Stale => Some(RowLiveness::Stale), Liveness::Gone => None,
@row_add n=757 row=contract/list-item/status area=standard-contracts rec=fct-dtnnvvl3@1
boundary: list payload item
item: status
promise: idle, busy or unknown.
@row_check n=757 row=contract/list-item/status sha=e7326778215d v=holds judge=sonnet head=8834195b0683
at: crates/viola-state/src/board.rs:134-136
quote: Self::Idle => "idle", Self::Busy => "busy", Self::Unknown => "unknown",
at: crates/viola-state/src/board.rs:334
quote: item.status = observations[at].status;
@row_add n=758 row=contract/list-item/wheel area=standard-contracts rec=fct-dtnnvvl3@1
boundary: list payload item
item: wheel
promise: human or driver; absent when the session is unwrapped.
@row_check n=758 row=contract/list-item/wheel sha=36e632edfede v=holds judge=sonnet head=8834195b0683
at: crates/viola-state/src/board.rs:237
quote: wheel: Some(snapshot.wheel),
at: crates/viola-state/src/board.rs:197
quote: wheel: None,
@row_add n=759 row=contract/snapshot-envelope/v area=standard-contracts rec=fct-uve5rstc@1
boundary: snapshot envelope
item: v
promise: Read first: above 1 is unsupported whatever else the envelope holds, 1 must parse whole, anything else is unreadable; an unsupported or unreadable snapshot is ignored and the state rebuilt by replaying events.ndjson.
@row_check n=759 row=contract/snapshot-envelope/v sha=194a65c43f6b v=holds judge=sonnet head=8834195b0683
at: crates/viola-state/src/snapshot.rs:137
quote: let Ok(EnvelopeV { v }) = serde_json::from_slice(&bytes) else {
at: crates/viola-state/src/snapshot.rs:141
quote: return SnapshotRead::Unsupported { v_seen: v };
at: crates/viola-state/src/snapshot.rs:143-145
quote: match serde_json::from_slice::<Envelope<InstanceSnapshot>>(&bytes) { Ok(envelope) if envelope.v == SNAPSHOT_V => SnapshotRead::Present(Box::new(envelope.data)), _ => SnapshotRead::Unreadable,
at: crates/viola-state/src/replay.rs:157
quote: let state = replay(instance_dir)?;
@row_add n=760 row=decision/snapshot-replay-log-fields-only area=standard-contracts rec=fct-uve5rstc@1
name: Snapshot replay recovers only log-derived fields
rule: A snapshot rebuilt by replay recovers only links, agent_session_id, wheel and the budget gate from the log; a field no line gave is absent, never a default.
@row_check n=760 row=decision/snapshot-replay-log-fields-only sha=f372f0e58fc9 v=holds judge=sonnet head=8834195b0683
at: crates/viola-state/src/replay.rs:19-23
quote: pub wheel: Option<Wheel>, pub budget_paused: Option<bool>, pub budget_override_until: Option<String>, pub agent_session_id: Option<String>, pub links: Vec<Value>,
at: crates/viola-state/src/replay.rs:52
quote: let mut replayed = Replayed::default();
@row_add n=761 row=decision/replayed-snapshot-liveness area=standard-contracts rec=fct-uve5rstc@1
name: Replayed snapshot liveness rests on the heartbeat
rule: For a replayed snapshot liveness rests on the heartbeat alone, so the instance is live or gone and never stale.
@row_check n=761 row=decision/replayed-snapshot-liveness sha=b351786e88dd v=holds judge=sonnet head=8834195b0683
at: crates/viola-state/src/board.rs:286
quote: if beat_age(dir, now).is_some_and(|age| age <= STALE_AFTER) {
at: crates/viola-state/src/board.rs:290
quote: liveness: RowLiveness::Live,
at: src/cmd/run.rs:442-444
quote: Some(Recovered::Replayed { .. }) => age() .is_some_and(|age| age <= STALE_AFTER) .then_some(Liveness::Live),
@row_add n=762 row=decision/replay-writes-no-file area=standard-contracts rec=fct-uve5rstc@1
name: Snapshot replay writes no file
rule: Replay of an unreadable or unsupported snapshot writes no file and logs one state-recovered line; an absent snapshot is neither replayed nor logged.
@row_check n=762 row=decision/replay-writes-no-file sha=b896826a1a94 v=holds judge=sonnet head=8834195b0683
at: crates/viola-state/src/replay.rs:153
quote: SnapshotRead::Absent => return Ok(Recovered::Absent),
at: crates/viola-state/src/replay.rs:157
quote: let state = replay(instance_dir)?;
at: crates/viola-state/src/replay.rs:163-165
quote: obs_event!( WARN, ObsEvent::StateRecovered,
@row_add n=763 row=decision/hook-fails-open-on-replayed-snapshot area=standard-contracts rec=fct-uve5rstc@1
name: Hook fails open on a replayed snapshot
rule: viola hook reads a replayed or absent snapshot as unreachable and fails open.
@row_check n=763 row=decision/hook-fails-open-on-replayed-snapshot sha=3ce0ce8139f9 v=holds judge=sonnet head=8834195b0683
at: src/cmd/hook.rs:227-228
quote: let Ok(Recovered::Snapshot(snapshot)) = read_snapshot_or_replay(&instance.dir) else { return Err(NotSent::Unreachable);
@row_add n=764 row=decision/revive-refused-without-cwd area=standard-contracts rec=fct-uve5rstc@1
name: Revive over a replayed snapshot refused
rule: A replayed snapshot gives no cwd, so a revive over one is refused cwd-missing.
@row_check n=764 row=decision/revive-refused-without-cwd sha=4708d3fcc7d1 v=holds judge=sonnet head=8834195b0683
at: src/cmd/revive.rs:264
quote: Recovered::Absent | Recovered::Replayed { .. } => None,
at: src/cmd/revive.rs:224
quote: |(_, recovered)| recorded_dir(recovered).ok_or_else(|| refuse(Refusal::CwdMissing)),
@row_add n=765 row=contract/ledger-stamps/rows area=standard-contracts rec=fct-62qgonvf@2
boundary: ledger stamps envelope
item: rows
promise: Maps each ledger row id to "pass" or "fail".
@row_check n=765 row=contract/ledger-stamps/rows sha=d97d38539899 v=holds judge=sonnet head=8834195b0683
at: crates/viola-agent-claude/src/ledger.rs:1127-1130
quote: .map(|(row, pass)| { ( row.id().to_owned(), json!(if *pass { "pass" } else { "fail" }),
@row_add n=766 row=contract/ledger-stamps/measured area=standard-contracts rec=fct-62qgonvf@2
boundary: ledger stamps envelope
item: measured
promise: The probe's readings (largest hook payload per spine event, typed_probe timings, dialog_probe); additions need no v bump, and run reads no field of it.
@row_check n=766 row=contract/ledger-stamps/measured sha=826b1bcf3734 v=holds judge=sonnet head=8834195b0683
at: crates/viola-agent-claude/src/ledger.rs:1141-1142
quote: "measured": { "largest_hook_payload": measured,
at: crates/viola-agent-claude/src/ledger.rs:1143-1144
quote: "typed_probe": { "ready_settle_ms": typed.ready_settle_ms,
at: crates/viola-agent-claude/src/ledger.rs:1150-1151
quote: "dialog_probe": { "parallel_both_before_first_post": parallel_both_before_first_post,
at: src/run/version_gate.rs:206
quote: Ok(Some(bytes)) => match verified(&bytes, version) {
@row_add n=767 row=decision/stamps-merge-own-version-only area=standard-contracts rec=fct-62qgonvf@2
name: Stamps merge replaces only its own version
rule: A stamps merge replaces only its own version's entry and keeps unknown versions and unknown fields; bytes of the wrong shape are replaced whole.
@row_check n=767 row=decision/stamps-merge-own-version-only sha=93246e269362 v=holds judge=sonnet head=8834195b0683
at: crates/viola-agent-claude/src/ledger.rs:1121-1124
quote: let mut doc = match existing.and_then(|b| serde_json::from_slice(b).ok()) { Some(Value::Object(doc)) if stamp_shape_ok(&doc) => doc, _ => Map::new(), };
at: crates/viola-agent-claude/src/ledger.rs:1162
quote: versions.insert(version.to_owned(), entry);
@row_add n=768 row=decision/version-verified-all-relied-rows area=standard-contracts rec=fct-62qgonvf@2
name: A CLI version is verified only when every relied row passes
rule: A version is verified only when every relied ledger row reads pass, so a stamp of an older, shorter row set reads unverified.
@row_check n=768 row=decision/version-verified-all-relied-rows sha=cc69e76586a2 v=holds judge=sonnet head=8834195b0683
at: crates/viola-agent-claude/src/ledger.rs:1237-1239
quote: Ok(LedgerRow::relied() .iter() .all(|row| rows[row.id()] == "pass"))
@row_add n=769 row=decision/link-set-follows-log area=standard-contracts rec=fct-vgnixbh3@1
name: Link set follows the log
rule: The wrapper restores its link set from the log at every start without appending or logging; a failed append leaves the set as it was, and a landed line is in the set even when the snapshot write then fails.
@row_check n=769 row=decision/link-set-follows-log sha=57ef4c46d15e v=holds judge=sonnet head=8834195b0683
at: src/run/links.rs:45-46
quote: for line in read_from(&self.instance_dir, 0)? { apply_link_line(&mut restored, &line?.value);
at: src/cmd/run.rs:333
quote: links.restore()?;
at: src/run/links.rs:132-133
quote: append_event(&self.instance_dir, line)?; apply_link_line(set, &logged);
@row_add n=770 row=contract/channel-unlink/result area=standard-contracts rec=fct-rkas3xta@1
boundary: channel unlink method
item: result
promise: Answers {} whether or not the set held the pair; an absent pair appends nothing and leaves the set unchanged.
@row_check n=770 row=contract/channel-unlink/result sha=48ac5b94f263 v=holds judge=sonnet head=8834195b0683
at: src/run/links.rs:81
quote: self.remove(&driver).map_err(|_| ProtocolError::Internal)?;
at: src/run/links.rs:82
quote: Ok(json!({"ok": {}}))
at: src/run/links.rs:105-107
quote: if self.since(&set, driver).is_none() { return Ok(false); }
@row_add n=771 row=contract/channel-link-unlink/error-32602 area=standard-contracts rec=fct-ooli5yny@1
boundary: channel link and unlink methods
item: -32602
promise: A link or unlink answered -32602 writes no line.
@row_check n=771 row=contract/channel-link-unlink/error-32602 sha=171d95b10657 v=holds judge=sonnet head=8834195b0683
at: src/run/links.rs:92
quote: .ok_or(ProtocolError::InvalidParams)
at: src/run/links.rs:95
quote: if driven != self.name || driver == driven {
at: src/run/links.rs:96
quote: return Err(ProtocolError::InvalidParams);
@row_add n=772 row=contract/channel-link-unlink/error-32603 area=standard-contracts rec=fct-ooli5yny@1
boundary: channel link and unlink methods
item: -32603
promise: A failed record or remove of the link answers -32603.
@row_check n=772 row=contract/channel-link-unlink/error-32603 sha=b9b29de628ff v=holds judge=sonnet head=8834195b0683
at: src/run/links.rs:74
quote: let (since, already_linked) = self.record(&driver).map_err(|_| ProtocolError::Internal)?;
at: src/run/links.rs:81
quote: self.remove(&driver).map_err(|_| ProtocolError::Internal)?;
@row_add n=773 row=decision/server-identity-check-before-frame area=standard-contracts rec=fct-2tmeycxu@2
name: Server identity checked before the first frame
rule: Every frame-writing client refuses a peer with another euid or an unreadable one, then compares the peer pid and its OS start time with the snapshot's; a refusal, a mismatch or a check that cannot be made writes no frame.
@row_check n=773 row=decision/server-identity-check-before-frame sha=87933a2201d7 v=holds judge=sonnet head=8834195b0683
at: crates/viola-channel/src/client.rs:140
quote: observed == Some(own)
at: src/cmd/client.rs:177-180
quote: if !peer_admitted(client.server_euid()) { return Err(CallError::ServerUnverified); } let verdict = verify_server(live.pid, &live.started_at, client.server_pid());
at: src/cmd/hook.rs:236-239
quote: if !peer_admitted(client.server_euid()) { return Err(NotSent::ServerUnverified); } let verdict = verify_server(snapshot.pid, &snapshot.started_at, client.server_pid());
at: crates/viola-mcp/src/tools.rs:352
quote: if !peer_admitted(client.server_euid()) || !server_verified(live, client.server_pid()).await? {
@row_add n=774 row=decision/client-strict-modes-before-frame area=standard-contracts rec=fct-rvenzu5e@1
name: Strict-modes check before any client frame
rule: Each channel client runs the instance's strict-modes check before the server comparison, a refusal writing no frame and no line, and only a real snapshot, not a replayed one, goes on to the comparison.
@row_check n=774 row=decision/client-strict-modes-before-frame sha=695aa46954e4 v=holds judge=sonnet head=8834195b0683
at: src/cmd/client.rs:31
quote: if check_instance(&absolute, &absolute.join("instances").join(name)).is_ok() {
at: src/cmd/hook.rs:84-85
quote: fn trusted(instance: &Instance) -> bool { check_instance(&instance.home, &instance.dir).is_ok()
at: crates/viola-mcp/src/tools.rs:339
quote: if check_instance(&ctx.home, &instance_dir).is_err() {
at: crates/viola-state/src/liveness.rs:93
quote: let Ok(Recovered::Snapshot(snapshot)) = read_snapshot_or_replay(instance_dir) else {
at: src/cmd/hook.rs:227
quote: let Ok(Recovered::Snapshot(snapshot)) = read_snapshot_or_replay(&instance.dir) else {
@row_add n=775 row=contract/channel-send/result area=standard-contracts rec=fct-2rdzhblg@1
boundary: channel send method
item: result
promise: {text, from?} answers {submitted_at, cursor} when confirmed, or {confirmed:false, detail:"unconfirmable", cursor}; cursor is events.ndjson's end offset when the send was accepted, before the paste.
@row_check n=775 row=contract/channel-send/result sha=b7c9d10a271d v=holds judge=sonnet head=8834195b0683
at: src/run/send.rs:215
quote: Ok(json!({"ok": {"submitted_at": submitted_at, "cursor": cursor}}))
at: src/run/send.rs:224
quote: Ok(json!({"ok": {"confirmed": false, "detail": "unconfirmable", "cursor": cursor}}))
at: src/run/send.rs:179-180
quote: let cursor = append_event_at(self.instance_dir, |cursor| { let mut data = json!({"cursor": cursor});
@row_add n=776 row=contract/channel-send/budget-paused area=standard-contracts rec=fct-2rdzhblg@1
boundary: channel send method
item: budget-paused
promise: A closed budget gate read at arrival refuses with budget-paused, detail five-hour or seven-day, before send-issued, with no cursor, nothing typed and no flight reserved.
@row_check n=776 row=contract/channel-send/budget-paused sha=14955445a02d v=holds judge=sonnet head=8834195b0683
at: src/run/send.rs:105
quote: .evaluate(Trigger::Send, Utc::now())
at: src/run/send.rs:108
quote: return ctx.budget_paused(&crossed);
at: src/run/send/budget.rs:11
quote: RefusalReason::BudgetPaused,
at: src/run/send.rs:118
quote: flight.send = Some(InFlight {
@row_add n=777 row=contract/channel-send/from area=standard-contracts rec=fct-2rdzhblg@1
boundary: channel send method
item: from
promise: A from naming another instance records the link from that instance to this one directly before send-issued; a link that cannot be recorded answers -32603 with nothing issued or typed; no from, or the wrapper's own name, records nothing.
@row_check n=777 row=contract/channel-send/from sha=a6e050e7a323 v=holds judge=sonnet head=8834195b0683
at: src/run/send.rs:142-146
quote: if let (Some(links), Some(from)) = (slot.links.get(), ctx.from.as_ref()) && from != name { links.record(from).map_err(|_| ProtocolError::Internal)?; }
at: src/run/send.rs:147
quote: let cursor = ctx.issue(text.len())?;
@row_add n=778 row=contract/channel-wait/result area=standard-contracts rec=fct-kzj34g66@1
boundary: channel wait method
item: result
promise: {after?, timeout_ms?, from?} answers {event, cursor}, cursor the end offset of that event's line, or {timed_out:true}.
@row_check n=778 row=contract/channel-wait/result sha=57734f674270 v=holds judge=sonnet head=8834195b0683
at: src/run/wait.rs:155
quote: return Ok(json!({"ok": {"event": line.value, "cursor": line.end}}));
at: src/run/wait.rs:161
quote: return Ok(json!({"ok": {"timed_out": true}}));
@row_add n=779 row=contract/channel-wait/params area=standard-contracts rec=fct-kzj34g66@1
boundary: channel wait method
item: after and timeout_ms
promise: Each absent or a non-negative integer fitting u64; null, a negative, a fraction or a string is -32602; unknown fields are ignored.
@row_check n=779 row=contract/channel-wait/params sha=20dc4c40ca28 v=holds judge=sonnet head=8834195b0683
at: src/run/wait.rs:185
quote: Some(value) => value.as_u64().map(Some).ok_or(ProtocolError::InvalidParams),
@row_add n=780 row=contract/channel-wait/start-offset area=standard-contracts rec=fct-kzj34g66@1
boundary: channel wait method
item: start offset
promise: The start is after, else the log's end at the call; a line counts only when it starts at or after it, and a pending dialog at or after the start is returned at once.
@row_check n=780 row=contract/channel-wait/start-offset sha=be5b7141f6a5 v=holds judge=sonnet head=8834195b0683
at: src/run/wait.rs:139-147
quote: let mut from = match after { Some(after) => after, None => { let feed = self.feed(); match feed.pending_start { Some(start) => start, None => end_offset(instance_dir).map_err(|_| ProtocolError::Internal)?,
@row_add n=781 row=contract/channel-wait/wake-kinds area=standard-contracts rec=fct-kzj34g66@1
boundary: channel wait method
item: wake kinds
promise: Only turn-ended, question, permission, plan, session-end and dialog-expired wake a wait; the wrapper's own send, link, unlink and start records never wake.
@row_check n=781 row=contract/channel-wait/wake-kinds sha=b81a01af7934 v=holds judge=sonnet head=8834195b0683
at: crates/viola-core/src/lib.rs:99-106
quote: pub const WAIT_WAKE: [Self; 6] = [ Self::TurnEnded, Self::Question, Self::Permission, Self::Plan, Self::SessionEnd, Self::DialogExpired, ];
at: src/run/wait.rs:201-204
quote: if EventKind::WAIT_WAKE .iter() .any(|k| Some(k.as_str()) == kind) {
@row_add n=782 row=contract/channel-link-unlink/from area=standard-contracts rec=fct-463usxdx@1
boundary: channel link and unlink methods
item: from
promise: link and unlink read no from; a from of any shape changes nothing.
@row_check n=782 row=contract/channel-link-unlink/from sha=3a5f237f8f6f v=holds judge=sonnet head=8834195b0683
at: src/run/links.rs:87
quote: fn driver_of(&self, params: &Value) -> Result<ViolaName, ProtocolError> {
at: src/run/links.rs:94
quote: let (driver, driven) = (named("driver")?, named("driven")?);
@row_add n=783 row=contract/channel-link-unlink/answer-order area=standard-contracts rec=fct-463usxdx@1
boundary: channel link and unlink methods
item: answer order
promise: Each answers only after its event line and the snapshot write.
@row_check n=783 row=contract/channel-link-unlink/answer-order sha=31f45d0d892d v=holds judge=sonnet head=8834195b0683
at: src/run/links.rs:132-135
quote: append_event(&self.instance_dir, line)?; apply_link_line(set, &logged); self.snapshots .update(|snapshot| snapshot.links.clone_from(set))
@row_add n=784 row=contract/channel-32602-cause/unsupported-version area=standard-contracts rec=fct-svwdswko@1
boundary: channel -32602 causes
item: unsupported-version
promise: One of the three causes a -32602 fault reads.
@row_check n=784 row=contract/channel-32602-cause/unsupported-version sha=b6c46ae0feb1 v=holds judge=sonnet head=8834195b0683
at: crates/viola-channel/src/lib.rs:105
quote: Self::UnsupportedVersion => Some("unsupported-version"),
@row_add n=785 row=contract/channel-32602-cause/invalid-params area=standard-contracts rec=fct-svwdswko@1
boundary: channel -32602 causes
item: invalid-params
promise: One of the three causes a -32602 fault reads.
@row_check n=785 row=contract/channel-32602-cause/invalid-params sha=e23c6fb7b72a v=holds judge=sonnet head=8834195b0683
at: crates/viola-channel/src/lib.rs:106
quote: Self::InvalidParams => Some("invalid-params"),
@row_add n=786 row=contract/channel-32602-cause/release-from-driver area=standard-contracts rec=fct-svwdswko@1
boundary: channel -32602 causes
item: release-from-driver
promise: One of the three causes a -32602 fault reads.
@row_check n=786 row=contract/channel-32602-cause/release-from-driver sha=7d1a2a362773 v=holds judge=sonnet head=8834195b0683
at: crates/viola-channel/src/lib.rs:107
quote: Self::ReleaseFromDriver => Some("release-from-driver"),
@row_add n=787 row=contract/hook-event/notification area=standard-contracts rec=fct-eiypz3p5@1
boundary: channel hook.event
item: notification
promise: Sent id-less with no reply only after the instance, socket-directory and server checks pass; otherwise nothing is sent and the hook fails open. Shape {ts, event:{kind, data}}.
@row_check n=787 row=contract/hook-event/notification sha=a3b4b55947dd v=holds judge=sonnet head=8834195b0683
at: src/cmd/hook.rs:116
quote: let Some(instance) = instance.as_ref().and_then(canonical).filter(trusted) else {
at: src/cmd/hook.rs:231
quote: if check_endpoint_dir(endpoint).is_err() {
at: src/cmd/hook.rs:239
quote: let verdict = verify_server(snapshot.pid, &snapshot.started_at, client.server_pid());
at: src/cmd/hook.rs:570
quote: .notify("hook.event", params)
@row_add n=788 row=contract/hook-event/kind area=standard-contracts rec=fct-eiypz3p5@1
boundary: channel hook.event
item: kind
promise: One of session-start, prompt-submitted, turn-ended, session-end, activity; data is an object; an invalid event is not appended and an append failure is -32603.
@row_check n=788 row=contract/hook-event/kind sha=5e189b1d9bc8 v=holds judge=sonnet head=8834195b0683
at: src/cmd/run.rs:86-92
quote: const HOOK_KINDS: [EventKind; 5] = [ EventKind::SessionStart, EventKind::PromptSubmitted, EventKind::TurnEnded, EventKind::SessionEnd, EventKind::Activity, ];
at: src/cmd/run.rs:102
quote: let data = event.get("data").filter(|d| d.is_object())?;
at: src/cmd/run.rs:126-127
quote: send::append_hook_event(&self.send, &self.wait, &self.instance_dir, line) .map_err(|_| ProtocolError::Internal)?;
@row_add n=789 row=contract/hook-event/appended-line area=standard-contracts rec=fct-eiypz3p5@1
boundary: channel hook.event
item: appended line
promise: A valid event is appended as {v, ts:<append time>, instance, kind, source:"hook", data}.
@row_check n=789 row=contract/hook-event/appended-line sha=7e5d3b72471d v=holds judge=sonnet head=8834195b0683
at: src/cmd/run.rs:108-114
quote: Some(EventLine::new( &self.name, kind, Source::Hook, data.clone(), Utc::now(), ))
at: crates/viola-state/src/events.rs:54-59
quote: v: 1, ts: crate::timestamp(at), instance: instance.as_ref().to_owned(), kind, source, data,
@row_add n=790 row=contract/hook-event/driver-relabel area=standard-contracts rec=fct-eiypz3p5@1
boundary: channel hook.event
item: driver relabel
promise: A prompt-submitted whose text equals the in-flight send's typed text is appended with origin driver and only then settles that send.
@row_check n=790 row=contract/hook-event/driver-relabel sha=c3f6c8a4cca1 v=holds judge=sonnet head=8834195b0683
at: src/run/send/slot.rs:198-204
quote: let prompt = line.kind == EventKind::PromptSubmitted && line.data["text"] .as_str() .is_some_and(|text| slot.claim(text)); if prompt { line.data["origin"] = json!("driver"); }
at: src/run/send/slot.rs:229-230
quote: if claimed { slot.settle(appended.is_ok().then(|| line.ts.clone()));
@row_add n=791 row=decision/list-needs-no-wrapper area=standard-contracts rec=fct-smoe4pw5@1
name: list is not a channel method
rule: list reads the instances' heartbeats and snapshots and claude agents --json in the calling process and contacts no wrapper, so it needs no live wrapper.
@row_check n=791 row=decision/list-needs-no-wrapper sha=c0bb0894dc67 v=holds judge=sonnet head=8834195b0683
at: src/cmd/list.rs:120
quote: let mut board = board::read(scan, at);
at: src/cmd/list.rs:124
quote: .and_then(|cwd| run::agents::read_here(home, &cwd).map_err(|_| ()))
at: crates/viola-state/src/board.rs:273
quote: match read_snapshot_or_replay(dir) {
@row_add n=792 row=contract/list-payload/agents-read-failure area=standard-contracts rec=fct-smoe4pw5@1
boundary: list payload
item: agents read failure
promise: A failed, oversize or malformed claude agents read leaves every status unknown and adds no unwrapped item.
@row_check n=792 row=contract/list-payload/agents-read-failure sha=4fa2f8205b2d v=holds judge=sonnet head=8834195b0683
at: src/cmd/list.rs:124
quote: .and_then(|cwd| run::agents::read_here(home, &cwd).map_err(|_| ()))
at: src/cmd/list.rs:126
quote: board::join(&mut board, observations(list));
@row_add n=793 row=contract/event-prompt-submitted/text area=standard-contracts rec=fct-kylkhqhb@1
boundary: prompt-submitted event data
item: text
promise: The submitted prompt as typed, with any long-paste wrapper and its frame removed and tag escaping reversed; a pasted text that ended in a newline is one byte short of it.
@row_check n=793 row=contract/event-prompt-submitted/text sha=badf7a3a6f90 v=holds judge=sonnet head=8834195b0683
at: crates/viola-agent-claude/src/hook.rs:168
quote: json!({"text": prompt_text(&raw), "origin": prompt_origin(&raw)})
at: crates/viola-agent-claude/src/hook.rs:204
quote: unescape_tags(&unwrap_pastes(raw))
@row_add n=794 row=contract/event-prompt-submitted/origin area=standard-contracts rec=fct-kylkhqhb@1
boundary: prompt-submitted event data
item: origin
promise: driver, human or harness; the hook files only human or harness, and driver is set only by the wrapper's relabel of the in-flight send's own prompt.
@row_check n=794 row=contract/event-prompt-submitted/origin sha=ba152186a445 v=holds judge=sonnet head=8834195b0683
at: crates/viola-agent-claude/src/hook.rs:194
quote: "harness"
at: crates/viola-agent-claude/src/hook.rs:196
quote: "human"
at: src/cmd/run.rs:103-104
quote: let prompt_ok = data["text"].is_string() && matches!(data["origin"].as_str(), Some("harness" | "human"));
at: src/run/send/slot.rs:203
quote: line.data["origin"] = json!("driver");
@row_add n=795 row=contract/cli-link-json/ok area=standard-contracts rec=fct-ubupynfq@1
boundary: viola link and unlink --json output
item: ok
promise: link prints {"v":1,"ok":{"since":…,"already_linked":…}} and unlink prints {"v":1,"ok":{}}, each with exit 0.
@row_check n=795 row=contract/cli-link-json/ok sha=125eddcbd8c6 v=holds judge=sonnet head=8834195b0683
at: src/cmd/link.rs:121
quote: client::document(&json!({"v": 1, "ok": ok}));
at: src/run/links.rs:82
quote: Ok(json!({"ok": {}}))
at: src/cmd/link.rs:126
quote: Ok(0)
@row_add n=796 row=contract/cli-exit-21-document/strict-modes-failed area=standard-contracts rec=fct-dt7d6rx2@1,fct-ubupynfq@1,fct-qybkldvd@1
boundary: exit-21 CLI document
item: strict-modes-failed
promise: A refused strict-modes check of the home and target instance prints exactly {"v":1,"error":"instance-unreachable","detail":"strict-modes-failed"} on stdout under --json (exit 21), stderr empty, no log line.
@row_check n=796 row=contract/cli-exit-21-document/strict-modes-failed sha=5b4da9848e92 v=holds judge=sonnet head=8834195b0683
at: src/cmd/client.rs:31-35
quote: if check_instance(&absolute, &absolute.join("instances").join(name)).is_ok() { return Ok(None); } if json { unreachable_document(UnreachableCause::StrictModes);
at: src/cmd/client.rs:233
quote: document(&json!({"v": 1, "error": "instance-unreachable", "detail": cause.as_str()}));
at: crates/viola-core/src/lib.rs:196
quote: Self::StrictModes => "strict-modes-failed",
@row_add n=797 row=contract/cli-exit-21-document/instance-dead area=standard-contracts rec=fct-dt7d6rx2@1
boundary: exit-21 CLI document
item: instance-dead
promise: One of six closed detail causes; v 1, error instance-unreachable.
@row_check n=797 row=contract/cli-exit-21-document/instance-dead sha=94cc01defbc4 v=holds judge=sonnet head=8834195b0683
at: crates/viola-core/src/lib.rs:197
quote: Self::InstanceDead => "instance-dead",
at: src/cmd/client.rs:197
quote: return Ok(Err(Unreachable::Dead));
@row_add n=798 row=contract/cli-exit-21-document/unwrapped-name area=standard-contracts rec=fct-dt7d6rx2@1
boundary: exit-21 CLI document
item: unwrapped-name
promise: One of six closed detail causes; v 1, error instance-unreachable.
@row_check n=798 row=contract/cli-exit-21-document/unwrapped-name sha=1274504182b0 v=holds judge=sonnet head=8834195b0683
at: crates/viola-core/src/lib.rs:198
quote: Self::UnwrappedName => "unwrapped-name",
at: src/cmd/client.rs:111
quote: UnreachableCause::UnwrappedName => Self::Unwrapped,
@row_add n=799 row=contract/cli-exit-21-document/endpoint-lost area=standard-contracts rec=fct-dt7d6rx2@1
boundary: exit-21 CLI document
item: endpoint-lost
promise: One of six closed detail causes; v 1, error instance-unreachable.
@row_check n=799 row=contract/cli-exit-21-document/endpoint-lost sha=b675b3a61c5d v=holds judge=sonnet head=8834195b0683
at: crates/viola-core/src/lib.rs:199
quote: Self::EndpointLost => "endpoint-lost",
at: src/cmd/client.rs:199-200
quote: Err(CallError::Channel(ChannelError::Closed | ChannelError::Io(_))) => { return Ok(Err(Unreachable::Lost));
@row_add n=800 row=contract/cli-exit-21-document/server-verify-failed area=standard-contracts rec=fct-dt7d6rx2@1
boundary: exit-21 CLI document
item: server-verify-failed
promise: One of six closed detail causes; v 1, error instance-unreachable.
@row_check n=800 row=contract/cli-exit-21-document/server-verify-failed sha=9b2691689ddc v=holds judge=sonnet head=8834195b0683
at: crates/viola-core/src/lib.rs:200
quote: Self::ServerVerify => "server-verify-failed",
at: src/cmd/client.rs:194
quote: Err(CallError::ServerUnverified) => return Ok(Err(Unreachable::ServerUnverified)),
@row_add n=801 row=contract/cli-exit-21-document/socket-dir-refused area=standard-contracts rec=fct-dt7d6rx2@1,fct-ubupynfq@1,fct-qybkldvd@1
boundary: exit-21 CLI document
item: socket-dir-refused
promise: A refused socket directory met at connect prints the instance-unreachable document with this detail (exit 21) and one process-exit line at WARN.
@row_check n=801 row=contract/cli-exit-21-document/socket-dir-refused sha=637352bb807b v=holds judge=sonnet head=8834195b0683
at: crates/viola-core/src/lib.rs:201
quote: Self::SocketDir => "socket-dir-refused",
at: src/cmd/client.rs:173-174
quote: if check_endpoint_dir(&live.endpoint).is_err() { return Err(CallError::SocketDirRefused);
at: src/cmd/client.rs:256-262
quote: obs_event!( WARN, ObsEvent::ProcessExit, subject = "self", exit_code = 21u8, detail = cause.detail(), during = cause.during(),
@row_add n=802 row=contract/fault-document/known-code-message area=standard-contracts rec=fct-nxxqzjun@1
boundary: The fault document on --json and MCP
item: message of a known fault code
promise: For each of the five known fault codes, `message` is that fault's fixed text; a fault without an integer `code` reads as no fault.
@row_check n=802 row=contract/fault-document/known-code-message sha=a21650ec2d14 v=holds judge=sonnet head=8834195b0683
at: crates/viola-channel/src/lib.rs:118
quote: let code = error["code"].as_i64()?;
at: crates/viola-channel/src/lib.rs:124-127
quote: -32700 => (ProtocolError::Parse.message(), Value::Null), -32600 => (ProtocolError::InvalidRequest.message(), Value::Null), -32601 => (ProtocolError::MethodNotFound.message(), Value::Null), -32603 => (ProtocolError::Internal.message(), Value::Null),
at: crates/viola-channel/src/lib.rs:130
quote: ProtocolError::UnsupportedVersion.message(),
@row_add n=803 row=contract/fault-document/unsupported-version area=standard-contracts rec=fct-nxxqzjun@1
boundary: The fault document on --json and MCP
item: -32602 unsupported version
promise: A `-32602` fault with an integer `data.supported` and a version-text `data.wrapper` (1 to 64 ASCII letters, digits, `.`, `-` or `+`) reads the unsupported-version body with those two values.
@row_check n=803 row=contract/fault-document/unsupported-version sha=8603f7adcddb v=holds judge=sonnet head=8834195b0683
at: crates/viola-channel/src/lib.rs:128-132
quote: -32602 => match (data["supported"].as_i64(), wrapper) { (Some(supported), Some(wrapper)) => ( ProtocolError::UnsupportedVersion.message(), json!({"supported": supported, "wrapper": wrapper}), ),
at: crates/viola-channel/src/lib.rs:146-149
quote: (1..=64).contains(&text.len()) && text .bytes() .all(|b| b.is_ascii_alphanumeric() || matches!(b, b'.' | b'-' | b'+'))
@row_add n=804 row=contract/fault-document/release-from-driver area=standard-contracts rec=fct-nxxqzjun@1
boundary: The fault document on --json and MCP
item: -32602 release-from-driver
promise: A `-32602` fault whose `data.reason` is exactly `release-from-driver` reads the release-from-driver body.
@row_check n=804 row=contract/fault-document/release-from-driver sha=bab81b04b346 v=holds judge=sonnet head=8834195b0683
at: crates/viola-channel/src/lib.rs:133-135
quote: _ if data["reason"] == "release-from-driver" => ( ProtocolError::ReleaseFromDriver.message(), json!({"reason": "release-from-driver"}),
@row_add n=805 row=contract/fault-document/invalid-params area=standard-contracts rec=fct-nxxqzjun@1
boundary: The fault document on --json and MCP
item: other -32602
promise: Any other `-32602` fault reads `invalid params` with `data` null; every other known code reads `data` null.
@row_check n=805 row=contract/fault-document/invalid-params sha=68f9f3500cd4 v=holds judge=sonnet head=8834195b0683
at: crates/viola-channel/src/lib.rs:137
quote: _ => (ProtocolError::InvalidParams.message(), Value::Null),
@row_add n=806 row=contract/fault-document/wrapper-fault area=standard-contracts rec=fct-pavvapld@1
boundary: The fault document on --json and MCP
item: wrapper fault
promise: A fault code outside the five known codes reads `{"code": <code>, "message": "wrapper fault", "data": null}`; the literal `wrapper fault` is a fixed output word.
@row_check n=806 row=contract/fault-document/wrapper-fault sha=d14692ef430e v=holds judge=sonnet head=8834195b0683
at: crates/viola-channel/src/lib.rs:139
quote: _ => ("wrapper fault", Value::Null),
at: crates/viola-channel/src/lib.rs:141
quote: Some(json!({"code": code, "message": message, "data": data}))
@row_add n=807 row=contract/human-error-line/wrapper-fault area=standard-contracts rec=fct-pavvapld@1
boundary: The human error line
item: wrapper fault
promise: A fault code outside the five known codes prints the human line `error: wrapper fault  <code>`.
@row_check n=807 row=contract/human-error-line/wrapper-fault sha=1aa9d19935f7 v=holds judge=sonnet head=8834195b0683
at: src/human/mirror.rs:53
quote: out.write_all(format!("error: wrapper fault  {code}\n").as_bytes())
at: src/cmd/client.rs:241-242
quote: let code = detail["code"].as_i64().unwrap_or_default(); let _ = human::write_wrapper_fault(&mut io::stderr().lock(), code);
@row_add n=808 row=contract/revive-json/refusal-document area=standard-contracts rec=fct-zxepdepr@2
boundary: viola revive --json
item: start refusal document
promise: The refusals `pinned-hash-mismatch`, `squatted-name` and `socket-dir-refused` print the refused document in place of the human pair, before any child is spawned.
@row_check n=808 row=contract/revive-json/refusal-document sha=dc167927fe73 v=holds judge=sonnet head=8834195b0683
at: src/cmd/run.rs:284
quote: "pinned-hash-mismatch",
at: src/cmd/run.rs:295
quote: return Ok(refused_with(launch.json, "squatted-name", || {
at: src/cmd/run.rs:300
quote: return Ok(refused_with(launch.json, "socket-dir-refused", || {
at: src/cmd/run.rs:420
quote: super::client::refused_document(detail);
@row_add n=809 row=contract/revive-json/program-not-found area=standard-contracts rec=fct-zxepdepr@2
boundary: viola revive --json
item: program not found
promise: A program not found prints the `internal-error` document, before any child is spawned.
@row_check n=809 row=contract/revive-json/program-not-found sha=9cdbbe16544a v=holds judge=sonnet head=8834195b0683
at: src/cmd/run.rs:270
quote: Err(_) => return Ok(refused_with(launch.json, "internal-error", || {})),
@row_add n=810 row=contract/revive-json/exit-1-document area=standard-contracts rec=fct-cz7o3ttv@1
boundary: viola revive --json
item: exit-1 document
promise: Every refusal of the start prints one exit-1 document; an internal error of the start prints the `internal-error` document and the error chain is still logged.
@row_check n=810 row=contract/revive-json/exit-1-document sha=ca560cb1fea8 v=holds judge=sonnet head=8834195b0683
at: src/cmd/revive.rs:122-124
quote: let started = started(home, args).inspect_err(|_| { if json { client::refused_document("internal-error");
at: src/cmd/revive.rs:126
quote: })?;
@row_add n=811 row=contract/revive-json/pump-error area=standard-contracts rec=fct-cz7o3ttv@1
boundary: viola revive --json
item: error after the child holds the terminal
promise: An error while pumping the child prints no document, because the child has held the terminal.
@row_check n=811 row=contract/revive-json/pump-error sha=ad4a3108e232 v=holds judge=sonnet head=8834195b0683
at: src/cmd/revive.rs:128
quote: Started::Launched(launched) => pump_child(*launched),
@row_add n=812 row=decision/statusline-wrapper-not-a-hook area=standard-contracts rec=fct-p4s6aosk@2
name: Statusline wrapper is not a hook event
rule: `viola hook statusline` is a hidden arm of `hook` taken on the word `statusline` before the event is mapped, so the hook event set keeps its nine events and the arm is exempt from the hook rules.
why: The statusline arm is the statusline wrapper, not a Claude Code hook.
@row_check n=812 row=decision/statusline-wrapper-not-a-hook sha=f5b57ebfebbc v=holds judge=sonnet head=8834195b0683
at: src/cmd/hook.rs:90-92
quote: if args.event == statusline::WORD { let Some(dir) = &args.capture else { return statusline::statusline(started);
at: src/cmd/hook.rs:98
quote: let Some(event) = HookEvent::from_arg(&args.event) else {
at: crates/viola-agent-claude/src/hook.rs:30
quote: pub const ALL: [Self; 9] = [
@row_add n=813 row=contract/hook-statusline/capture area=standard-contracts rec=fct-p4s6aosk@2
boundary: viola hook statusline
item: --capture <DIR>
promise: With `--capture <DIR>` the arm takes `viola verify`'s probe capture: no instance check, no `budget.json` write, no command run, nothing printed; without it the pass-through is unchanged.
@row_check n=813 row=contract/hook-statusline/capture sha=06088fa4ebae v=holds judge=sonnet head=8834195b0683
at: src/cmd/hook.rs:90-96
quote: if args.event == statusline::WORD { let Some(dir) = &args.capture else { return statusline::statusline(started); }; let shell = args.shell.as_deref(); capture_statusline(dir, shell, &mut std::io::stdin().lock()); return Ok(ExitCode::SUCCESS);
@row_add n=814 row=decision/statusline-source-in-viola-home area=standard-contracts rec=fct-p4s6aosk@2
name: Statusline command source
rule: The user's statusline command is read at start from one named source that viola only reads: the statusline source file in the viola home whenever it exists, else only in the default home the user's Claude settings, else none.
why: Two files deciding a command viola later runs is a ratified boundary widening, choosing the file in the viola home.
@row_check n=814 row=decision/statusline-source-in-viola-home sha=0e11b0511091 v=holds judge=sonnet head=8834195b0683
at: src/cmd/run.rs:533-535
quote: let own = home.join(STATUSLINE_SOURCE); if own.exists() { return Some(own);
at: src/cmd/run.rs:538-539
quote: let default = super::canonical_home(&super::default_home(user_home)); (default == home).then(|| {
at: src/cmd/run.rs:552
quote: File::open(source)
@row_add n=815 row=contract/statusline-source/command area=standard-contracts rec=fct-p4s6aosk@2
boundary: The statusline source settings
item: statusLine command
promise: A command is taken only from a `statusLine` of type `command`, non-empty, with no NUL.
@row_check n=815 row=contract/statusline-source/command sha=899e9a549ed8 v=holds judge=sonnet head=8834195b0683
at: crates/viola-agent-claude/src/statusline.rs:83-87
quote: if line.get("type")?.as_str()? != "command" { return None; } let command = line.get("command")?.as_str()?; (!command.is_empty() && !command.contains('\0')).then(|| command.to_owned())
@row_add n=816 row=decision/statusline-check-before-log area=standard-contracts rec=fct-p4s6aosk@2
name: Statusline instance check before the log
rule: The statusline arm runs the instance check before it opens its log or reads anything under the home.
why: Opening the log sets an existing home to 0700, so a check made after it passes a home at 0770.
@row_check n=816 row=decision/statusline-check-before-log sha=4d7ba63225f4 v=holds judge=sonnet head=8834195b0683
at: src/cmd/hook/statusline.rs:42
quote: let Some(instance) = instance.as_ref().and_then(canonical).filter(trusted) else {
at: src/cmd/hook/statusline.rs:46
quote: let _ = obs::viola_obs_init(
@row_add n=817 row=contract/hook-statusline/refused-check area=standard-contracts rec=fct-p4s6aosk@2
boundary: viola hook statusline
item: refused instance check
promise: A refused check ends the arm at once with exit 0: nothing under the home read or opened, no stdin read, no command run, nothing printed, `budget.json` untouched and the home's mode left as found.
@row_check n=817 row=contract/hook-statusline/refused-check sha=ebcf02acdea6 v=holds judge=sonnet head=8834195b0683
at: src/cmd/hook/statusline.rs:42-44
quote: let Some(instance) = instance.as_ref().and_then(canonical).filter(trusted) else { return Ok(ExitCode::SUCCESS); };
@row_add n=818 row=decision/statusline-only-shell-out area=standard-contracts rec=fct-p4s6aosk@2
name: Statusline command is the only shell-out
rule: The user's statusline command is the only shell-out in viola: one argument of `/bin/sh -c`, with the same stdin, environment and working directory unchanged, stderr discarded, bounded by a deadline.
why: The command is a user-written shell string; a ratified boundary widening.
guard: ledger row statusline-shell
@row_check n=818 row=decision/statusline-only-shell-out sha=d030f733cc80 v=holds guard=true judge=sonnet head=8834195b0683
at: crates/viola-agent-claude/src/statusline.rs:124
quote: [BIN_SH, "-c", command]
at: src/cmd/hook/statusline.rs:201
quote: let ran = run_bounded(Command::new(shell).arg(flag).arg(command), stdin, bound);
at: src/cmd/hook/statusline.rs:228
quote: .stderr(Stdio::null())
at: src/cmd/hook.rs:64
quote: const STATUSLINE_DEADLINE: Duration = Duration::from_secs(5);
guard_at: crates/viola-agent-claude/src/ledger.rs:97
@row_add n=819 row=decision/statusline-deadline area=standard-contracts rec=fct-p4s6aosk@2
name: Statusline command deadline
rule: The statusline command is bounded by a 5 s deadline.
state: provisional
@row_check n=819 row=decision/statusline-deadline sha=514c7b32b063 v=holds judge=sonnet head=8834195b0683
at: src/cmd/hook.rs:64
quote: const STATUSLINE_DEADLINE: Duration = Duration::from_secs(5);
at: src/cmd/hook/statusline.rs:59
quote: bound: STATUSLINE_DEADLINE,
@row_add n=820 row=contract/hook-statusline/stdout area=standard-contracts rec=fct-p4s6aosk@2
boundary: viola hook statusline
item: stdout
promise: Prints the command's stdout unchanged, bounded by the frame cap; prints nothing when the command fails or passes the deadline, when no command is recorded, and on oversize or malformed stdin.
@row_check n=820 row=contract/hook-statusline/stdout sha=ff5966029328 v=holds judge=sonnet head=8834195b0683
at: src/cmd/hook/statusline.rs:138-143
quote: Some(Shell::Exited { code: Some(0), stdout, }) => { // The user's bytes as they are, in one write; a reader that has gone is no fault. let _ = io.out.write_all(&stdout).and_then(|()| io.out.flush());
at: src/cmd/hook/statusline.rs:264
quote: let _ = pipe.take(MAX_FRAME).read_to_end(&mut bytes);
at: src/cmd/hook/statusline.rs:146-147
quote: Some(Shell::Deadline) => Some("deadline"), Some(Shell::Exited { .. } | Shell::NotStarted) | None => None,
at: src/cmd/hook/statusline.rs:117-123
quote: if u64::try_from(bytes.len()).unwrap_or(u64::MAX) > MAX_FRAME { reject_stdin("oversize"); return Ok(Outcome::failed("oversize-stdin")); } let Ok(reading) = statusline::reading(&bytes) else { reject_stdin("malformed");
@row_add n=821 row=contract/hook-statusline/exit area=standard-contracts rec=fct-p4s6aosk@2
boundary: viola hook statusline
item: exit status
promise: Always exits 0 with empty stderr.
@row_check n=821 row=contract/hook-statusline/exit sha=d95b2a52422e v=holds judge=sonnet head=8834195b0683
at: src/main.rs:133
quote: (Role::Hook, _) => ExitCode::SUCCESS,
at: src/cmd/hook/statusline.rs:228
quote: .stderr(Stdio::null())
@row_add n=822 row=contract/board/status area=standard-contracts rec=fct-yj2flshm@2
boundary: The board (CLI list and MCP list)
item: status
promise: Each row carries `status` (idle/busy) from `claude agents --json`, and unwrapped sessions are listed as read-only rows.
@row_check n=822 row=contract/board/status sha=38db36cb1921 v=holds judge=sonnet head=8834195b0683
at: crates/viola-state/src/board.rs:334
quote: item.status = observations[at].status;
at: crates/viola-state/src/board.rs:344
quote: .map(|(observation, _)| Item::unwrapped(observation))
at: crates/viola-agent-claude/src/agents.rs:36
quote: _ => Self::Unknown,
@row_add n=823 row=contract/board/agents-read-failure area=standard-contracts rec=fct-yj2flshm@2
boundary: The board (CLI list and MCP list)
item: agents read failure
promise: If `claude agents` is not found, fails, is over the cap or cannot be parsed, every row's `status` is `unknown`, no unwrapped rows are listed, and the outcome is never an error.
@row_check n=823 row=contract/board/agents-read-failure sha=be7ae3ca1414 v=holds judge=sonnet head=8834195b0683
at: src/cmd/list.rs:121-127
quote: if let Ok(list) = std::env::current_dir() .ok() .ok_or(()) .and_then(|cwd| run::agents::read_here(home, &cwd).map_err(|_| ())) { board::join(&mut board, observations(list)); }
at: crates/viola-state/src/board.rs:236
quote: status: SessionStatus::Unknown,
@row_add n=824 row=contract/board/wrapped-merge area=standard-contracts rec=fct-yj2flshm@2
boundary: The board (CLI list and MCP list)
item: wrapped instance join
promise: An agents row matching a wrapped instance's `child_pid` or `agent_session_id` is merged into that instance's row, never listed separately.
@row_check n=824 row=contract/board/wrapped-merge sha=2cc3f26ca667 v=holds judge=sonnet head=8834195b0683
at: crates/viola-state/src/board.rs:160-161
quote: let by_pid = self.pid.is_some() && self.pid == keys.child_pid; by_pid || keys.agent_session_id.as_deref() == Some(self.session_id.as_str())
at: crates/viola-state/src/board.rs:343
quote: .filter(|(_, joined)| !joined)
@row_add n=825 row=decision/mcp-list-reads-agents area=standard-contracts rec=fct-yj2flshm@2
name: MCP list reads claude agents on each call
rule: The MCP server's `list` tool makes the same one `claude agents` read as CLI `list` on each call.
why: A ratified widening.
@row_check n=825 row=decision/mcp-list-reads-agents sha=be6e3634e5eb v=holds judge=sonnet head=8834195b0683
at: src/cmd/mcp.rs:35
quote: Arc::new(move || list::board_document(&board_home).unwrap_or(Value::Null));
at: src/cmd/list.rs:84
quote: let (board, budget, _) = read(home, &scan, true);
at: src/cmd/list.rs:124
quote: .and_then(|cwd| run::agents::read_here(home, &cwd).map_err(|_| ()))
@row_add n=826 row=decision/workspace-dependency-pins area=build-system rec=fct-j6b6zix5@2
name: Third-party versions pinned in one place
rule: Every third-party dependency version is pinned once in the workspace dependency table and shared by the members that take it.
@row_check n=826 row=decision/workspace-dependency-pins sha=1e2349ecf702 v=holds judge=sonnet head=8834195b0683
at: Cargo.toml:192
quote: [workspace.dependencies]
at: crates/viola-channel/Cargo.toml:21
quote: interprocess.workspace = true
@row_add n=827 row=decision/msrv area=build-system rec=fct-j6b6zix5@2
name: Minimum supported Rust version 1.96
rule: The workspace declares rust-version 1.96, and dependency resolution prefers versions compatible with it.
@row_check n=827 row=decision/msrv sha=f035634f8edc v=holds judge=sonnet head=8834195b0683
at: Cargo.toml:188
quote: rust-version = "1.96"
at: Cargo.toml:183
quote: resolver = "3"
@row_add n=828 row=decision/fmt-and-clippy-gate area=build-system rec=fct-jbq7vmcz@2,fct-jykzrth4@2
name: Formatting and clippy with warnings denied
rule: Code passes the cargo fmt check and clippy over the whole workspace, all targets and the fake-agent feature, with every warning denied.
@row_check n=828 row=decision/fmt-and-clippy-gate sha=a1e6598c6c26 v=holds judge=sonnet head=8834195b0683
at: .github/workflows/ci.yml:278
quote: run: cargo fmt --all --check
at: .github/workflows/ci.yml:281
quote: run: cargo clippy --workspace --all-targets --features fake-agent -- -D warnings
@row_add n=829 row=decision/no-print-macros area=build-system rec=fct-jbq7vmcz@2
name: No print macros
rule: The workspace lints deny `print_stdout`, `print_stderr` and `dbg_macro`, and every product member inherits them.
guard: tests/contract_lints.rs
except: viola-e2e denies only `dbg_macro`, because the harness prints its JSON document; the fake agent carries a crate-level print allow.
@row_check n=829 row=decision/no-print-macros sha=92c491e50c3a v=holds guard=true judge=sonnet head=8834195b0683
at: Cargo.toml:256-258
quote: print_stdout = "deny" print_stderr = "deny" dbg_macro = "deny"
at: crates/viola-e2e/Cargo.toml:52
quote: dbg_macro = "deny"
at: src/bin/viola-fake-agent.rs:7
quote: #![allow(clippy::print_stdout, clippy::print_stderr)]
guard_at: tests/contract_lints.rs:45
@row_add n=830 row=decision/single-human-writer area=build-system rec=fct-jbq7vmcz@2
name: One human writer in the root bin
rule: The root bin writes human output only through its human module, whose writers lock the stream and swallow a closed pipe instead of panicking; hook and obs never call it.
@row_check n=830 row=decision/single-human-writer sha=31a1eb454082 v=holds judge=sonnet head=8834195b0683
at: src/human.rs:43
quote: let _ = write_refusal(&mut io::stderr().lock(), unable, hint);
at: src/human.rs:64
quote: let _ = write_result(&mut io::stdout().lock(), line);
@row_add n=831 row=contract/human-stderr/refusal area=build-system rec=fct-jbq7vmcz@2
boundary: Human stderr
item: refusal
promise: A refusal writes the two fixed `unable:` / `hint:` lines as one write.
@row_check n=831 row=contract/human-stderr/refusal sha=eae2d2de55ec v=holds judge=sonnet head=8834195b0683
at: src/human.rs:37
quote: out.write_all(format!("unable: {unable}\nhint: {hint}\n").as_bytes())
@row_add n=832 row=contract/human-stderr/internal-error area=build-system rec=fct-jbq7vmcz@2
boundary: Human stderr
item: internal error
promise: An internal error writes exactly `error: internal error` and a newline, for every cli verb without `--json`.
@row_check n=832 row=contract/human-stderr/internal-error sha=98b1d471a6cd v=holds judge=sonnet head=8834195b0683
at: src/human.rs:49
quote: out.write_all(b"error: internal error\n")
at: src/main.rs:70
quote: human::internal_error();
@row_add n=833 row=contract/cli-json/internal-error area=build-system rec=fct-jbq7vmcz@2
boundary: CLI --json output
item: internal-error
promise: Under `--json` an internal error prints the `internal-error` document on stdout, with stderr empty.
@row_check n=833 row=contract/cli-json/internal-error sha=f19b150113b0 v=holds judge=sonnet head=8834195b0683
at: src/main.rs:68
quote: cmd::internal_error_document();
at: src/cmd/client.rs:227
quote: document(&json!({"v": 1, "error": code, "detail": null}));
@row_add n=834 row=decision/obs-event-only-emitter area=build-system rec=fct-jbq7vmcz@2
name: obs_event! is the only event emitter
rule: The tracing level macros are banned, so `obs_event!` is the only sanctioned emitter; raw `event!` is caught by a fail-closed grep.
why: Clippy cannot exempt a macro's inner expansion, so raw event! needs the grep.
guard: clippy.toml disallowed-macros; scripts/lint-probes.sh
@row_check n=834 row=decision/obs-event-only-emitter sha=01c8dcbf15c9 v=holds guard=true judge=sonnet head=8834195b0683
at: clippy.toml:4-6
quote: disallowed-macros = [ { path = "tracing::info", reason = "log through viola_core::obs_event! (obs-plan §3)" }, { path = "tracing::warn", reason = "log through viola_core::obs_event! (obs-plan §3)" },
at: scripts/lint-probes.sh:127-128
quote: out=$(cd "$1" && grep -rnE --include='*.rs' --exclude-dir=target --exclude-dir=.git \ '\bevent!\s*[({[]' .) || rc=$?
guard_at: scripts/lint-probes.sh:125
@row_add n=835 row=decision/lint-bans-proven area=build-system rec=fct-jbq7vmcz@2
name: Every lint ban proven to fire
rule: Every lint ban is proven to fire and every control proven to pass.
guard: scripts/lint-probes.sh
@row_check n=835 row=decision/lint-bans-proven sha=40275bb5493c v=holds guard=true judge=sonnet head=8834195b0683
at: scripts/lint-probes.sh:159
quote: if [ "$fired" -eq "${#bans[@]}" ] && [ "$clean" -eq "${#controls[@]}" ] && [ "$grep_ok" -eq 1 ]; then
guard_at: scripts/lint-probes.sh:70
@row_add n=836 row=decision/licence-allowlist area=build-system rec=fct-cotc6dxt@2
name: Licence allowlist
rule: Dependency licences allowed are exactly MIT, Apache-2.0, Zlib and Unicode-3.0.
why: The 0BSD exception was ratified.
guard: cargo deny check
except: 0BSD only through the per-crate exception for doctest-file, never through the allowlist.
@row_check n=836 row=decision/licence-allowlist sha=1410f99d7ef2 v=holds guard=true judge=sonnet head=8834195b0683
at: deny.toml:14
quote: allow = ["MIT", "Apache-2.0", "Zlib", "Unicode-3.0"]
at: deny.toml:19-21
quote: [[licenses.exceptions]] crate = "doctest-file" allow = ["0BSD"]
guard_at: .github/workflows/ci.yml:367
@row_add n=837 row=decision/crates-io-only area=build-system rec=fct-cotc6dxt@2
name: Sources from crates.io only
rule: Every dependency comes from crates.io only.
guard: cargo deny check
@row_check n=837 row=decision/crates-io-only sha=e17aff1d8021 v=holds guard=true judge=sonnet head=8834195b0683
at: deny.toml:24-25
quote: unknown-registry = "deny" unknown-git = "deny"
guard_at: .github/workflows/ci.yml:367
@row_add n=838 row=decision/no-c-building-crates area=build-system rec=fct-cotc6dxt@2
name: No C-building crates
rule: C-building crates (cc, libsqlite3-sys, openssl-sys) are banned from the dependency graph.
guard: cargo deny check
except: The fuzz workspace's libfuzzer-sys builds C++ via cc, a ratified test-only exemption: never linked into viola, and fuzz never joins the root workspace.
@row_check n=838 row=decision/no-c-building-crates sha=0ffbdb4fb2e5 v=holds guard=true judge=sonnet head=8834195b0683
at: deny.toml:29
quote: { crate = "cc", reason = "no C in the build" },
at: deny.toml:30
quote: { crate = "libsqlite3-sys", reason = "no C in the build" },
at: deny.toml:31
quote: { crate = "openssl-sys", reason = "no C in the build" },
guard_at: .github/workflows/ci.yml:367
@row_add n=839 row=decision/no-telemetry-crates area=build-system rec=fct-cotc6dxt@2
name: No telemetry crates
rule: Telemetry crates (opentelemetry-otlp, opentelemetry-stdout, sentry, tracing-appender) are banned.
guard: cargo deny check
@row_check n=839 row=decision/no-telemetry-crates sha=163ff5b04f18 v=holds guard=true judge=sonnet head=8834195b0683
at: deny.toml:32
quote: { crate = "opentelemetry-otlp", reason = "no telemetry exporter or egress in v1" },
at: deny.toml:33
quote: { crate = "opentelemetry-stdout", reason = "no telemetry exporter in v1" },
at: deny.toml:34
quote: { crate = "sentry", reason = "no error reporter or egress in v1" },
at: deny.toml:35
quote: { crate = "tracing-appender", reason = "its non_blocking writer drops lines and pulls a vulnerable time range" },
guard_at: .github/workflows/ci.yml:367
@row_add n=840 row=decision/banned-features area=build-system rec=fct-cotc6dxt@2
name: Banned dependency features
rule: The features veil/toggle, rmcp auth and transport-streamable-http-server, and tracing-subscriber/env-filter are banned.
guard: cargo deny check
@row_check n=840 row=decision/banned-features sha=ee208466399e v=holds guard=true judge=sonnet head=8834195b0683
at: deny.toml:40
quote: deny = ["toggle"]
at: deny.toml:45
quote: deny = ["transport-streamable-http-server", "auth"]
at: deny.toml:50
quote: deny = ["env-filter"]
guard_at: .github/workflows/ci.yml:367
@row_add n=841 row=decision/serde-json-recursion-limit area=build-system rec=fct-cotc6dxt@2
name: serde_json keeps its recursion limit
rule: The serde_json unbounded_depth feature is banned.
why: Every parse of external bytes keeps serde_json's default recursion limit.
guard: probe row serde_json-unbounded-depth
@row_check n=841 row=decision/serde-json-recursion-limit sha=bc286128b215 v=holds guard=true judge=sonnet head=8834195b0683
at: deny.toml:55
quote: deny = ["unbounded_depth"]
guard_at: scripts/deny-probes.sh:32
@row_add n=842 row=decision/deny-single-target area=build-system rec=fct-cotc6dxt@2
name: Dependency policy for one target triple
rule: The dependency policy is evaluated for the one target triple x86_64-unknown-linux-gnu.
@row_check n=842 row=decision/deny-single-target sha=7fe13e2ce1c1 v=holds judge=sonnet head=8834195b0683
at: deny.toml:5
quote: targets = ["x86_64-unknown-linux-gnu"]
@row_add n=843 row=decision/sync-crates-tokio-free area=build-system rec=fct-cotc6dxt@2
name: Sync crates are tokio-free
rule: Tokio is banned anywhere, direct or transitive, in the normal dependency graph of viola-core, viola-pty, viola-state and viola-agent-claude, checked once per crate as the sole root.
why: A direct-parent allowlist would let Tokio arrive through a third-party feature; excluding over the workspace false-fails under feature unification.
guard: deny-sync.toml per crate in scripts/sync-crates.txt
@row_check n=843 row=decision/sync-crates-tokio-free sha=d90b6f71a6b6 v=holds guard=true judge=sonnet head=8834195b0683
at: deny-sync.toml:12
quote: { crate = "tokio", reason = "tokio is confined to viola-mcp; the sync crates stay tokio-free" },
at: scripts/sync-crates.txt:1-4
quote: viola-core viola-pty viola-agent-claude viola-state
at: .github/workflows/ci.yml:382
quote: cargo deny --config deny-sync.toml --manifest-path "crates/$crate/Cargo.toml" check bans
guard_at: deny-sync.toml:12
@row_add n=844 row=decision/channel-tokio-only-behind-feature area=build-system rec=fct-cotc6dxt@2,fct-vavh762f@2
name: viola-channel tokio only behind its feature
rule: viola-channel's normal dependency graph without its tokio feature holds no tokio; the Tokio client exists only behind that feature.
why: As its own root viola-channel false-fails the sole-root ban once viola-mcp enables the feature, so a tree check replaces it.
guard: scripts/channel-tokio-free.sh
@row_check n=844 row=decision/channel-tokio-only-behind-feature sha=5d1776544c9f v=holds guard=true judge=sonnet head=8834195b0683
at: crates/viola-channel/Cargo.toml:17
quote: tokio = ["dep:tokio", "interprocess/tokio"]
at: crates/viola-channel/src/lib.rs:12-13
quote: #[cfg(feature = "tokio")] pub mod tokio_client;
guard_at: scripts/channel-tokio-free.sh:15
@row_add n=845 row=decision/deny-bans-proven area=build-system rec=fct-cotc6dxt@2
name: Every dependency ban proven live
rule: Every dependency ban is proven live: a throwaway project per ban must fail with that ban's own diagnostic, and a clean control must pass.
guard: scripts/deny-probes.sh
@row_check n=845 row=decision/deny-bans-proven sha=17a999c1f830 v=holds guard=true judge=sonnet head=8834195b0683
at: scripts/deny-probes.sh:5
quote: # with that ban's own diagnostic. A clean control project must pass both configs.
at: scripts/deny-probes.sh:61
quote: if [ "$rc" -ne 0 ] && grep -qF "$diagnostic" "$dir/deny.log"; then
guard_at: scripts/deny-probes.sh:1
@row_add n=846 row=decision/fuzz-lockfile-audited area=build-system rec=fct-cotc6dxt@2
name: Fuzz lockfile audited separately
rule: The fuzz workspace's own lockfile is audited separately for advisories and sources.
guard: CI supply-chain step Fuzz lockfile audit; nightly advisories job
@row_check n=846 row=decision/fuzz-lockfile-audited sha=db50ce6e0283 v=holds guard=true judge=sonnet head=8834195b0683
at: .github/workflows/ci.yml:374
quote: cargo deny --manifest-path fuzz/Cargo.toml --format json check advisories sources 2> target/supply-chain/deny-fuzz.json || { cat target/supply-chain/deny-fuzz.json; exit 1; }
at: .github/workflows/nightly.yml:30
quote: cargo deny --manifest-path fuzz/Cargo.toml check advisories
guard_at: .github/workflows/ci.yml:370
@row_add n=847 row=decision/no-orphan-modules area=build-system rec=fct-ecerp7e3@2
name: No orphan modules
rule: No lib or bin target holds an orphan module; the check is a CI gate and is proven to fire on planted orphans.
guard: scripts/orphans-check.sh
@row_check n=847 row=decision/no-orphan-modules sha=6c5d7a7715ba v=holds guard=true judge=sonnet head=8834195b0683
at: .github/workflows/ci.yml:314
quote: run: bash scripts/orphans-check.sh --probe && bash scripts/orphans-check.sh
at: scripts/orphans-check.sh:30
quote: log=$(cargo modules orphans "$@" --deny --target "$triple" 2>&1) || rc=$?
guard_at: scripts/orphans-check.sh:1
@row_add n=848 row=decision/module-graph-review-not-gate area=build-system rec=fct-ecerp7e3@2
name: Module dependency graph is a review, not a gate
rule: The acyclic module dependency graph stays an on-demand review, never a gate.
why: cargo-modules reports every type and its own inherent method as a cycle, whatever the filters.
@row_check n=848 row=decision/module-graph-review-not-gate sha=75dfbcd7067f v=holds judge=sonnet head=8834195b0683
searched: dependencies --acyclic · in .github/ scripts/ (only a comment in scripts/orphans-check.sh:6 saying it is deliberately absent)
searched: --acyclic · in .github/workflows/ci.yml
@row_add n=849 row=decision/rmcp-features-exact area=build-system rec=fct-ecerp7e3@2,fct-u4dur7ij@1
name: rmcp with exactly server and transport-io
rule: rmcp is present in viola's release graph with default features off, and the features requested of it are exactly server and transport-io.
guard: scripts/rmcp-features.sh
@row_check n=849 row=decision/rmcp-features-exact sha=90dfea86e9cd v=holds guard=true judge=sonnet head=8834195b0683
at: Cargo.toml:223
quote: rmcp = { version = ">=3.4.1, <3.5", default-features = false, features = ["server", "transport-io"] }
at: scripts/rmcp-features.sh:33
quote: if [ "$requested" != "server transport-io" ]; then
guard_at: scripts/rmcp-features.sh:1
@row_add n=850 row=decision/install-from-source area=build-system rec=fct-lfogagcm@1
name: Install with cargo install
rule: viola is installed with cargo install from the repository root.
@row_check n=850 row=decision/install-from-source sha=71f1595be6bf v=unverifiable judge=sonnet head=8834195b0683
searched: cargo install --path · in README.md, .github/, scripts/, src/, Cargo.toml
why: no evidence found
@row_add n=851 row=decision/no-crates-io-publishing area=build-system rec=fct-p4rq22po@1
name: Nothing published to crates.io
rule: Every workspace member, the root viola bin included, is unpublished; nothing goes to crates.io in v1.
@row_check n=851 row=decision/no-crates-io-publishing sha=129aac29b69c v=holds judge=sonnet head=8834195b0683
at: Cargo.toml:190
quote: publish = false
at: Cargo.toml:7
quote: publish.workspace = true
at: crates/viola-mcp/Cargo.toml:7
quote: publish.workspace = true
@row_add n=852 row=decision/github-actions-ci area=ci-cd-approach rec=fct-vf6cfvel@2
name: GitHub Actions CI
rule: CI runs on GitHub Actions: one push and pull-request pipeline on ubuntu, and a weekly scheduled workflow.
@row_check n=852 row=decision/github-actions-ci sha=6065517d4856 v=holds judge=sonnet head=8834195b0683
at: .github/workflows/ci.yml:18
quote: os: [ubuntu-latest]
at: .github/workflows/ci.yml:5
quote: pull_request:
at: .github/workflows/nightly.yml:5
quote: - cron: "17 4 * * 1"
@row_add n=853 row=decision/weekly-advisories area=ci-cd-approach rec=fct-vf6cfvel@2
name: Weekly advisory audit
rule: A weekly job audits advisories over both the root and the fuzz lockfile.
why: The advisory database moves without code changes.
@row_check n=853 row=decision/weekly-advisories sha=2865e4793448 v=holds judge=sonnet head=8834195b0683
at: .github/workflows/nightly.yml:29
quote: cargo deny check advisories
at: .github/workflows/nightly.yml:30
quote: cargo deny --manifest-path fuzz/Cargo.toml check advisories
@row_add n=854 row=decision/weekly-fuzz area=ci-cd-approach rec=fct-vf6cfvel@2
name: Weekly fuzzing
rule: A weekly job fuzzes every target for a bounded time and uploads the fuzz artifacts on failure.
@row_check n=854 row=decision/weekly-fuzz sha=1b440dd80ef9 v=holds judge=sonnet head=8834195b0683
at: .github/workflows/nightly.yml:55
quote: cargo "+$channel" fuzz run --fuzz-dir fuzz "$t" "fuzz/corpus/$t" -- -max_total_time=120
at: .github/workflows/nightly.yml:59
quote: if: failure()
@row_add n=855 row=decision/no-workflow-concurrency area=ci-cd-approach rec=fct-vf6cfvel@2
name: No workflow concurrency groups
rule: No workflow has a concurrency block.
why: A concurrency group cancels pending runs, which would drop a cancelled run's always() gate and upload chain.
@row_check n=855 row=decision/no-workflow-concurrency sha=aaafe00a711a v=holds judge=sonnet head=8834195b0683
searched: concurrency · in .github/workflows/*.yml
@row_add n=856 row=decision/workflow-permissions area=ci-cd-approach rec=fct-27xbiznz@1
name: Least-privilege workflow permissions
rule: Every workflow sets empty permissions at the top and read-only contents per job.
guard: zizmor
@row_check n=856 row=decision/workflow-permissions sha=1252204a3b30 v=holds guard=true judge=sonnet head=8834195b0683
at: .github/workflows/ci.yml:7
quote: permissions: {}
at: .github/workflows/nightly.yml:8
quote: permissions: {}
at: .github/workflows/ci.yml:14
quote: contents: read
guard_at: .github/workflows/ci.yml:392
@row_add n=857 row=decision/actions-pinned-by-sha area=ci-cd-approach rec=fct-27xbiznz@1
name: Actions pinned by commit SHA
rule: Every action used in a workflow is pinned by full commit SHA with a version comment.
guard: zizmor
@row_check n=857 row=decision/actions-pinned-by-sha sha=a66c3e3bc417 v=holds guard=true judge=sonnet head=8834195b0683
at: .github/workflows/ci.yml:22
quote: - uses: actions/checkout@3d3c42e5aac5ba805825da76410c181273ba90b1 # v7.0.1
at: .github/workflows/nightly.yml:60
quote: uses: actions/upload-artifact@043fb46d1a93c77aae656e7c1c64a875d1fc6a0a # v7.0.1
guard_at: .github/workflows/ci.yml:392
@row_add n=858 row=decision/event-payload-via-env area=ci-cd-approach rec=fct-27xbiznz@1
name: Event payload only through env
rule: Event-payload values reach a workflow step only through environment variables.
guard: zizmor
@row_check n=858 row=decision/event-payload-via-env sha=55abd842e1cd v=holds guard=true judge=sonnet head=8834195b0683
at: .github/workflows/ci.yml:356
quote: run: cargo install --locked zizmor@1.30.1
at: .github/workflows/ci.yml:12
quote: runs-on: ${{ matrix.os }}
searched: github.event · in .github/workflows/*.yml
searched: ${{ github. · in .github/workflows/*.yml
guard_at: .github/workflows/ci.yml:392
@row_add n=859 row=decision/ci-tools-pinned area=ci-cd-approach rec=fct-uy6tohyo@1
name: CI tools at pinned versions
rule: Every tool CI installs is version-pinned and checkout keeps no credentials; a release asset installed by hand is checked against its published sha256.
@row_check n=859 row=decision/ci-tools-pinned sha=7cff4e2cbebb v=holds judge=sonnet head=8834195b0683
at: .github/workflows/ci.yml:24
quote: persist-credentials: false
at: .github/workflows/ci.yml:354
quote: tool: cargo-deny@0.20.2
at: .github/workflows/ci.yml:356
quote: run: cargo install --locked zizmor@1.30.1
at: .github/workflows/ci.yml:246
quote: run: cargo install --locked cargo-fuzz@0.13.2
at: scripts/install-ripgrep.sh:4
quote: # the sha256 its release publishes, so CI and a local gate run the same binary. No Action is used.
@row_add n=860 row=decision/coverage-floors area=ci-cd-approach rec=fct-3nc6guvb@3
name: Coverage floors
rule: The test job runs the instrumented suite with coverage floors 85/95/80, then doctest, and its gate requires coverage and doctest.
guard: gate --require coverage,doctest
@row_check n=860 row=decision/coverage-floors sha=1b1277608dcb v=holds guard=true judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/run/coverage.rs:16
quote: [("lines", 85.0), ("functions", 95.0), ("regions", 80.0)];
at: .github/workflows/ci.yml:38
quote: run: bash scripts/agent-run.sh run --coverage
at: .github/workflows/ci.yml:117
quote: run: bash scripts/agent-run.sh gate --require coverage,doctest
guard_at: .github/workflows/ci.yml:117
@row_add n=861 row=decision/uploads-gated-on-secret-scan area=ci-cd-approach rec=fct-3nc6guvb@3
name: Uploads gated on the secret scan
rule: CI artifacts are uploaded only when the secret scan succeeds; on a scan failure only the secret-scan report is uploaded.
guard: secret-scan step
@row_check n=861 row=decision/uploads-gated-on-secret-scan sha=c062b58308e1 v=holds guard=true judge=sonnet head=8834195b0683
at: .github/workflows/ci.yml:79
quote: if: always() && steps.secret-scan.outcome == 'success'
at: .github/workflows/ci.yml:106
quote: if: always() && steps.secret-scan.outcome == 'failure'
at: .github/workflows/ci.yml:162
quote: if: always() && steps.secret-scan.outcome == 'success'
guard_at: .github/workflows/ci.yml:73
@row_add n=862 row=decision/perf-gate area=ci-cd-approach rec=fct-3nc6guvb@3
name: Perf runs in its own job
rule: Performance runs only in the perf job, gated on its perf verdict; the test job and the pre-push carry no perf step.
guard: gate --require perf
@row_check n=862 row=decision/perf-gate sha=116c10113c70 v=holds guard=true judge=sonnet head=8834195b0683
at: .github/workflows/ci.yml:147
quote: run: bash scripts/agent-run.sh run --perf
at: .github/workflows/ci.yml:189
quote: run: bash scripts/agent-run.sh gate --require perf
guard_at: .github/workflows/ci.yml:189
@row_add n=863 row=decision/mutation-at-epoch-boundary area=ci-cd-approach rec=fct-3nc6guvb@3
name: Mutation testing at the epoch boundary
rule: CI runs no mutation job; mutation testing runs only through the harness mutants run, kept for the epoch-boundary code audit.
@row_check n=863 row=decision/mutation-at-epoch-boundary sha=c637a0de6436 v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/run.rs:56
quote: /// `--mutants` is the epoch-boundary audit's, and `--local-live` drives the real CLI, so none of
searched: run --mutants · in .github/workflows/*.yml
searched: mutants: · in .github/workflows/*.yml
@row_add n=864 row=decision/msrv-ci area=ci-cd-approach rec=fct-3nc6guvb@3
name: MSRV checked in CI
rule: A CI job checks the workspace and runs the unit tests on Rust 1.96.
guard: gate --require nextest-unit
@row_check n=864 row=decision/msrv-ci sha=038adf4903a2 v=holds guard=true judge=sonnet head=8834195b0683
at: .github/workflows/ci.yml:220
quote: run: cargo check --workspace
at: .github/workflows/ci.yml:225
quote: run: bash scripts/agent-run.sh run --unit
guard_at: .github/workflows/ci.yml:229
@row_add n=865 row=decision/fuzz-replay-ci area=ci-cd-approach rec=fct-3nc6guvb@3
name: Fuzz corpus replayed in CI
rule: A CI job replays the fuzz corpus on every push and pull request.
guard: gate --require fuzz-replay
@row_check n=865 row=decision/fuzz-replay-ci sha=92980cf5e527 v=holds guard=true judge=sonnet head=8834195b0683
at: .github/workflows/ci.yml:249
quote: run: bash scripts/agent-run.sh run --fuzz-replay
guard_at: .github/workflows/ci.yml:253
@row_add n=866 row=decision/gate-steps-fail-closed area=ci-cd-approach rec=fct-3nc6guvb@3
name: Gate steps fail closed
rule: Every CI gate step runs fail-closed in bash.
@row_check n=866 row=decision/gate-steps-fail-closed sha=f8449b5cb240 v=holds judge=sonnet head=8834195b0683
at: .github/workflows/ci.yml:117
quote: run: bash scripts/agent-run.sh gate --require coverage,doctest
at: .github/workflows/ci.yml:59
quote: run: bash scripts/g2-zero-panics.sh --probe && bash scripts/g2-zero-panics.sh
at: .github/workflows/ci.yml:76
quote: run: bash scripts/agent-run.sh secret-scan
@row_add n=867 row=decision/sync-crates-compile-without-tokio area=ci-cd-approach rec=fct-jykzrth4@2
name: Sync crates compile without tokio
rule: Each listed sync crate, the hook path included, is checked to compile on its own with no tokio; an empty list or an absent package fails.
@row_check n=867 row=decision/sync-crates-compile-without-tokio sha=1ff92b3379c7 v=holds judge=sonnet head=8834195b0683
at: .github/workflows/ci.yml:274-275
quote: test -s scripts/sync-crates.txt cargo check $(sed 's/^/-p /' scripts/sync-crates.txt)
@row_add n=868 row=decision/tests-against-fake-agent area=ci-cd-approach rec=fct-jykzrth4@2
name: Tests run against the fake agent
rule: The workspace test suite runs against the fake agent, replaying recorded Claude fixtures.
@row_check n=868 row=decision/tests-against-fake-agent sha=6109c4f2f277 v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/run/coverage.rs:28-29
quote: "--features", "viola/fake-agent",
at: tests/cli_verify.rs:1700
quote: &workspace_path("fixtures/claude"),
@row_add n=869 row=decision/release-build-viola-only area=ci-cd-approach rec=fct-jykzrth4@2
name: Release build yields viola only
rule: The release build yields only the viola executable, and no artifact of it carries the test-only features test-support or fake-agent.
why: The check judges the build's own artifacts, because a shared or cached target directory keeps stale test-only executables.
guard: scripts/release-check.sh
@row_check n=869 row=decision/release-build-viola-only sha=da98d603d420 v=holds guard=true judge=sonnet head=8834195b0683
at: scripts/release-check.sh:31-32
quote: if [ "$name" != "viola" ]; then echo "release-check: FAILED — test-only binary $name"
at: scripts/release-check.sh:19
quote: | .target.name as $t | (.features // [])[] | select(. == "test-support" or . == "fake-agent")
guard_at: scripts/release-check.sh:1
@row_add n=870 row=decision/real-claude-local-only area=ci-cd-approach rec=fct-oqzfmod4@2
name: Real claude CLI only locally
rule: The real claude CLI runs only locally, never in CI; in CI viola verify runs only against the fake agent, and its real-CLI probe and record run only locally.
guard: tests/cli_verify.rs
@row_check n=870 row=decision/real-claude-local-only sha=695d6ac640cc v=holds guard=true judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/run/coverage.rs:29
quote: "viola/fake-agent",
searched: claude · in .github/workflows/*.yml
searched: verify · in .github/workflows/*.yml
guard_at: tests/cli_verify.rs:140
@row_add n=871 row=decision/local-pre-push-gate area=ci-cd-approach rec=fct-ni3gbuvj@2
name: Local pre-push gate
rule: A local pre-push gate precedes every operator push and a red pre-push stops the pass; CI's run on the pushed sha stays the verdict of record.
@row_check n=871 row=decision/local-pre-push-gate sha=d681c9f82e53 v=unverifiable judge=sonnet head=8834195b0683
why: not a code claim
@row_add n=872 row=decision/pre-push-clean-environment area=ci-cd-approach rec=fct-ni3gbuvj@2
name: Pre-push children in a clean environment
rule: The pre-push runs natively in the working tree, every child with an emptied environment holding only HOME from the passwd entry and a fixed PATH; the harness's own HOME is never read.
@row_check n=872 row=decision/pre-push-clean-environment sha=19c6cd6c5046 v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/pre_push/linux.rs:39
quote: cmd.arg("-i")
at: crates/viola-e2e/src/harness/pre_push/linux.rs:40-44
quote: .arg(format!("HOME={}", self.home)) .arg(format!( "PATH={home}/.cargo/bin:/usr/local/bin:/usr/bin:/bin", home = self.home ))
at: crates/viola-e2e/src/harness/pre_push/linux.rs:96
quote: .and_then(|line| line.split(':').nth(5))
@row_add n=873 row=contract/pre-push/linux-only area=ci-cd-approach rec=fct-ni3gbuvj@2
boundary: viola-harness pre-push
item: pre-push-linux-only
promise: On a host other than Linux the pre-push refuses `pre-push-linux-only` with exit 2.
@row_check n=873 row=contract/pre-push/linux-only sha=c0732d4d4b0a v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/pre_push.rs:54
quote: return doc.finish(Err(Stop::new("pre-push-linux-only", "")), 2);
@row_add n=874 row=contract/pre-push/passwd-home area=ci-cd-approach rec=fct-ni3gbuvj@2
boundary: viola-harness pre-push
item: tool-missing passwd-home
promise: A failed passwd probe or a non-absolute home field is `tool-missing` `passwd-home` at stage `tools`.
@row_check n=874 row=contract/pre-push/passwd-home sha=678faeebbd94 v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/pre_push/linux.rs:81
quote: let missing = || Stop::new("tool-missing", "passwd-home");
at: crates/viola-e2e/src/harness/pre_push/linux.rs:97
quote: .filter(|home| home.starts_with('/'))
at: crates/viola-e2e/src/harness/pre_push.rs:89-90
quote: doc.stage = "tools"; let linux = Linux::new(&passwd_home(runner)?, &ws.root);
@row_add n=875 row=absence/deploy-stage area=ci-cd-approach rec=fct-32j43dt2@1
capability: A deploy stage
@row_check n=875 row=absence/deploy-stage sha=0cfbaef8f677 v=holds judge=sonnet head=8834195b0683
searched: deploy · in .github/** Cargo.toml deny.toml
searched: cargo-dist · in .github/** Cargo.toml deny.toml dist*.toml
searched: release*.yml · in .github/workflows/
@row_add n=876 row=decision/release-via-dist area=ci-cd-approach rec=fct-32j43dt2@1
name: Release workflow generated by dist
rule: The v1.x release path is a dist-generated release workflow with cargo-auditable.
@row_check n=876 row=decision/release-via-dist sha=e91a5179468b v=unverifiable judge=sonnet head=8834195b0683
searched: cargo-dist · in .github/** Cargo.toml deny.toml dist*.toml
searched: cargo-auditable · in .github/** Cargo.toml deny.toml Cargo.lock
searched: dist-workspace.toml · in repository root
why: no evidence found
@row_add n=877 row=decision/core-depends-on-no-viola-crate area=crate-dependency-direction rec=fct-pqkdzm6m@1
name: viola-core depends on no viola crate
rule: viola-core depends on no other viola crate.
@row_check n=877 row=decision/core-depends-on-no-viola-crate sha=270125d7c4dc v=holds judge=sonnet head=8834195b0683
at: crates/viola-core/Cargo.toml:10-11
quote: nutype.workspace = true serde.workspace = true
@row_add n=878 row=decision/pty-knows-no-agent area=crate-dependency-direction rec=fct-ggzishfk@2
name: viola-pty depends on no viola crate and knows no agent
rule: viola-pty depends on no other viola crate, knows no agent, and takes no thiserror.
@row_check n=878 row=decision/pty-knows-no-agent sha=7ea6ed05e04b v=holds judge=sonnet head=8834195b0683
at: crates/viola-pty/Cargo.toml:15
quote: libc.workspace = true
at: crates/viola-pty/Cargo.toml:16
quote: portable-pty.workspace = true
at: crates/viola-pty/Cargo.toml:18
quote: tracing.workspace = true
@row_add n=879 row=decision/claude-shapes-in-one-crate area=crate-dependency-direction rec=fct-sfkaxxdn@2
name: Claude payload shapes in one crate
rule: viola-agent-claude is the only crate that knows Claude payload shapes, including those of claude agents --json.
@row_check n=879 row=decision/claude-shapes-in-one-crate sha=4d8593a61649 v=holds judge=sonnet head=8834195b0683
at: crates/viola-agent-claude/src/hook.rs:30
quote: pub const ALL: [Self; 9] = [
searched: hook_event_name|PreToolUse|"session_id"|permission_mode · in crates/viola-core crates/viola-state crates/viola-channel crates/viola-pty crates/viola-mcp src (no match)
@row_add n=880 row=decision/agent-claude-pure area=crate-dependency-direction rec=fct-sfkaxxdn@2
name: viola-agent-claude does no file I/O
rule: viola-agent-claude takes no viola-state dependency and writes no file; stamps, budget and settings files are written by viola-state and the root bin.
why: So the crate stays pure.
@row_check n=880 row=decision/agent-claude-pure sha=76e0ce9ed073 v=holds judge=sonnet head=8834195b0683
at: crates/viola-agent-claude/Cargo.toml:15
quote: viola-core = { path = "../viola-core" }
searched: viola_state|std::fs|fs::|File::|OpenOptions · in crates/viola-agent-claude/src (hits only in #[cfg(test)] code: lib.rs:300, dialog.rs:421, screen.rs:509)
@row_add n=881 row=decision/mcp-dependencies area=crate-dependency-direction rec=fct-u4dur7ij@1
name: viola-mcp takes channel with tokio
rule: viola-mcp depends on viola-channel with its tokio feature and on tokio, and takes no serde.
@row_check n=881 row=decision/mcp-dependencies sha=e29c0f21846d v=holds judge=sonnet head=8834195b0683
at: crates/viola-mcp/Cargo.toml:16
quote: viola-channel = { path = "../viola-channel", features = ["tokio"] }
at: crates/viola-mcp/Cargo.toml:23
quote: tokio = { workspace = true, features = ["rt"] }
searched: serde.workspace · in crates/viola-mcp/Cargo.toml (only serde_json)
@row_add n=882 row=decision/e2e-test-only area=crate-dependency-direction rec=fct-tr3aduln@1
name: viola-e2e is test-only
rule: viola-e2e is test-only, and no product crate depends on it.
@row_check n=882 row=decision/e2e-test-only sha=8f6fc429c6a9 v=holds judge=sonnet head=8834195b0683
searched: viola-e2e · in every Cargo.toml of the tree (only its own manifest and a comment in the root)
@row_add n=883 row=decision/root-bin-consumes-normalised-events area=crate-dependency-direction rec=fct-p5s5gpab@2
name: Root bin consumes only normalised events
rule: The root bin depends on all members and holds the subcommand dispatch, the run pump, the wheel and the budget governor, all of which consume only normalised events.
@row_check n=883 row=decision/root-bin-consumes-normalised-events sha=e3fbc9d77209 v=holds judge=sonnet head=8834195b0683
at: Cargo.toml:147-152
quote: viola-core = { path = "crates/viola-core" } viola-pty = { path = "crates/viola-pty" } viola-agent-claude = { path = "crates/viola-agent-claude" } viola-state = { path = "crates/viola-state" } viola-channel = { path = "crates/viola-channel" } viola-mcp = { path = "crates/viola-mcp" }
at: src/cmd/run.rs:13
quote: use viola_agent_claude::{StripPlan, statusline};
@row_add n=884 row=decision/local-only-deployment area=deployment-model rec=fct-hupns7gt@1
name: Local-only deployment
rule: viola is deployed local-only.
@row_check n=884 row=decision/local-only-deployment sha=7c1195fffef0 v=unverifiable judge=sonnet head=8834195b0683
searched: Dockerfile · in the repository tree (Glob)
why: no evidence found; local-only is a deployment intent, only its absences (see the container row) are searchable
@row_add n=885 row=absence/container-or-serverless-deployment area=deployment-model rec=fct-hupns7gt@1
capability: Container, orchestration or serverless deployment (Docker, Compose, Kubernetes, serverless)
@row_check n=885 row=absence/container-or-serverless-deployment sha=491caebd93e0 v=holds judge=sonnet head=8834195b0683
searched: Dockerfile* · in the repository root and all subdirectories (Glob)
searched: docker-compose* · in the repository root and all subdirectories (Glob)
searched: compose.y*ml · in the repository root and all subdirectories (Glob)
searched: **/k8s/** · in the repository tree (Glob)
searched: serverless.y*ml · in the repository tree (Glob)
@row_add n=886 row=decision/single-binary-on-path area=deployment-model rec=fct-4jixev2q@2
name: Single binary on PATH
rule: viola ships as a single binary installed on PATH.
@row_check n=886 row=decision/single-binary-on-path sha=73ee6cd8cbe0 v=holds judge=sonnet head=8834195b0683
at: Cargo.toml:13
quote: name = "viola"
@row_add n=887 row=decision/run-pins-self-copy area=deployment-model rec=fct-4jixev2q@2
name: run pins a copy of itself and its plugin
rule: viola run pins a copy of itself per version and hash, writes the embedded plugin pointing at that copy, and launches the child with that plugin directory.
@row_check n=887 row=decision/run-pins-self-copy sha=75c5a112b517 v=holds judge=sonnet head=8834195b0683
at: crates/viola-state/src/pin.rs:85
quote: let dir = std::path::absolute(home.join("bin").join(&key))?;
at: crates/viola-state/src/pin.rs:71
quote: format!("{VERSION}-{}", content_key(bytes))
at: src/cmd/run.rs:475
quote: for (rel, content) in viola_agent_claude::plugin_files(&pinned.path_fwd) {
at: src/run/mod.rs:129
quote: OsString::from(PLUGIN_DIR_FLAG),
@row_add n=888 row=decision/exact-toolchain-pin area=project-directory-structure rec=fct-irhgihut@5
name: Exact Rust toolchain pin
rule: The project builds on one exactly pinned stable Rust toolchain with rustfmt and clippy, never a floating channel.
except: The separate fuzz workspace uses its own pinned nightly toolchain.
@row_check n=888 row=decision/exact-toolchain-pin sha=ce1475f7441e v=holds judge=sonnet head=8834195b0683
at: rust-toolchain.toml:2
quote: channel = "1.98.1"
at: rust-toolchain.toml:3
quote: components = ["rustfmt", "clippy"]
at: fuzz/rust-toolchain.toml:2
quote: channel = "nightly-2026-09-20"
@row_add n=889 row=decision/no-tracing-level-macros area=project-directory-structure rec=fct-irhgihut@5
name: Tracing level macros banned
rule: The tracing info, warn, error, debug and trace macros are disallowed in the code.
guard: clippy disallowed-macros, checked by the lint probes
@row_check n=889 row=decision/no-tracing-level-macros sha=c575cafe89e6 v=holds guard=true judge=sonnet head=8834195b0683
at: clippy.toml:4
quote: disallowed-macros = [
at: clippy.toml:5
quote: { path = "tracing::info", reason = "log through viola_core::obs_event! (obs-plan §3)" },
at: clippy.toml:6
quote: { path = "tracing::warn", reason = "log through viola_core::obs_event! (obs-plan §3)" },
at: clippy.toml:7
quote: { path = "tracing::error", reason = "log through viola_core::obs_event! (obs-plan §3)" },
at: clippy.toml:8
quote: { path = "tracing::debug", reason = "log through viola_core::obs_event! (obs-plan §3)" },
at: clippy.toml:9
quote: { path = "tracing::trace", reason = "log through viola_core::obs_event! (obs-plan §3)" },
guard_at: scripts/lint-probes.sh:1
@row_add n=890 row=decision/anyhow-at-bin-edge-only area=project-directory-structure rec=fct-irhgihut@5
name: anyhow at the binary edge only
rule: anyhow error handling is confined to the edge of the viola binary.
@row_check n=890 row=decision/anyhow-at-bin-edge-only sha=d6bde9fd74c2 v=holds judge=sonnet head=8834195b0683
at: Cargo.toml:153
quote: anyhow.workspace = true
searched: anyhow · in crates/**/*.rs (no match)
searched: anyhow · in crates/*/Cargo.toml (no match)
@row_add n=891 row=decision/cargo-deny-bans area=project-directory-structure rec=fct-irhgihut@5
name: cargo-deny supply-chain policy
rule: cargo-deny checks advisories, licences and sources, and bans C dependencies, telemetry and listed features.
guard: deny-probes negative probe per ban with a clean control
@row_check n=891 row=decision/cargo-deny-bans sha=8290b082da0b v=holds guard=true judge=sonnet head=8834195b0683
at: deny.toml:7-8
quote: [advisories] unmaintained = "all"
at: deny.toml:14
quote: allow = ["MIT", "Apache-2.0", "Zlib", "Unicode-3.0"]
at: deny.toml:24
quote: unknown-registry = "deny"
at: deny.toml:29
quote: { crate = "cc", reason = "no C in the build" },
at: deny.toml:32
quote: { crate = "opentelemetry-otlp", reason = "no telemetry exporter or egress in v1" },
at: deny.toml:45
quote: deny = ["transport-streamable-http-server", "auth"]
guard_at: scripts/deny-probes.sh:1
@row_add n=892 row=decision/no-tokio-in-sync-crates area=project-directory-structure rec=fct-irhgihut@5
name: No tokio in sync crates
rule: Every crate on the single sync-crate list must have no tokio in its graph, each checked as the sole root.
guard: cargo-deny tokio ban over the sync-crate list
@row_check n=892 row=decision/no-tokio-in-sync-crates sha=0fa2b7cacb2c v=holds guard=true judge=sonnet head=8834195b0683
at: deny-sync.toml:12
quote: { crate = "tokio", reason = "tokio is confined to viola-mcp; the sync crates stay tokio-free" },
at: scripts/sync-crates.txt:1-4
quote: viola-core viola-pty viola-agent-claude viola-state
at: .github/workflows/ci.yml:382
quote: cargo deny --config deny-sync.toml --manifest-path "crates/$crate/Cargo.toml" check bans
guard_at: deny-sync.toml:12
@row_add n=893 row=decision/channel-tokio-behind-feature area=project-directory-structure rec=fct-irhgihut@5
name: Channel Tokio client behind a feature
rule: viola-channel's normal dependency graph carries no tokio; the Tokio client exists only behind its tokio feature.
guard: channel-tokio-free check in CI supply-chain
@row_check n=893 row=decision/channel-tokio-behind-feature sha=452130c34117 v=holds guard=true judge=sonnet head=8834195b0683
at: crates/viola-channel/Cargo.toml:17
quote: tokio = ["dep:tokio", "interprocess/tokio"]
at: crates/viola-channel/Cargo.toml:23
quote: tokio = { workspace = true, optional = true, features = ["io-util"] }
guard_at: scripts/channel-tokio-free.sh:1
@row_add n=894 row=decision/rmcp-exact-features area=project-directory-structure rec=fct-irhgihut@5
name: rmcp features exactly server and transport-io
rule: viola's release graph requests rmcp with exactly the features server and transport-io.
guard: rmcp-features check in the CI lint job
@row_check n=894 row=decision/rmcp-exact-features sha=92e4fefffaa2 v=holds guard=true judge=sonnet head=8834195b0683
at: Cargo.toml:223
quote: rmcp = { version = ">=3.4.1, <3.5", default-features = false, features = ["server", "transport-io"] }
guard_at: scripts/rmcp-features.sh:13
@row_add n=895 row=decision/release-carries-viola-only area=project-directory-structure rec=fct-irhgihut@5
name: Release build carries viola only
rule: The release build carries the viola binary only and no test-only feature.
guard: release-check in the CI release job
@row_check n=895 row=decision/release-carries-viola-only sha=5cf685da0748 v=holds guard=true judge=sonnet head=8834195b0683
at: scripts/release-check.sh:19
quote: | .target.name as $t | (.features // [])[] | select(. == "test-support" or . == "fake-agent")
guard_at: scripts/release-check.sh:1
@row_add n=896 row=decision/fuzz-separate-workspace area=project-directory-structure rec=fct-irhgihut@5
name: Fuzzing in a separate workspace
rule: Fuzz targets live in a separate cargo-fuzz workspace with its own lockfile, excluded from the root workspace, with committed synthetic seeds.
@row_check n=896 row=decision/fuzz-separate-workspace sha=6a6d71191fa9 v=holds judge=sonnet head=8834195b0683
at: Cargo.toml:182
quote: exclude = ["fuzz"]
at: fuzz/Cargo.lock:1
quote: # This file is automatically @generated by Cargo.
at: fuzz/corpus/viola_name/valid-builder:1
quote: builder
@row_add n=897 row=decision/board-read-model-shape area=project-directory-structure rec=fct-irhgihut@5
name: Board read model is sync and Claude-free
rule: The board read model is sync and std only, carries no Claude shape, and none of its types holds endpoint, pinned_bin, cwd, statusline_command, pid, started_at or links.
@row_check n=897 row=decision/board-read-model-shape sha=dfea5e06bd73 v=holds judge=sonnet head=8834195b0683
at: crates/viola-state/src/board.rs:167
quote: pub struct Item {
at: crates/viola-state/src/board.rs:168
quote: pub name: String,
at: crates/viola-state/src/board.rs:169
quote: pub wrapped: bool,
at: crates/viola-state/src/board.rs:170
quote: pub liveness: RowLiveness,
at: crates/viola-state/src/board.rs:171
quote: pub status: SessionStatus,
at: crates/viola-state/src/board.rs:185
quote: pub join: JoinKeys,
@row_add n=898 row=decision/zero-panics-gate area=project-directory-structure rec=fct-irhgihut@5
name: Zero panic lines in end-to-end runs
rule: No panic role line may appear under the end-to-end home, checked fail-closed.
guard: obs G2 zero-panics gate
except: A panic whose location is the hook seam module.
@row_check n=898 row=decision/zero-panics-gate sha=94c4aac44b56 v=holds guard=true judge=sonnet head=8834195b0683
at: scripts/g2-zero-panics.sh:29
quote: [inputs | fromjson? | select(.event == "panic") | select(exempt | not)] | length' \
at: scripts/g2-zero-panics.sh:28
quote: and (.panic_location | sub(":[0-9]+$"; "")) == $seam;
guard_at: scripts/g2-zero-panics.sh:1
@row_add n=899 row=decision/channel-json-rpc-local-sockets area=project-directory-structure rec=fct-irhgihut@5
name: JSON-RPC over local sockets for the channel
rule: The channel speaks JSON-RPC 2.0 as newline-delimited JSON over interprocess local sockets.
@row_check n=899 row=decision/channel-json-rpc-local-sockets sha=dc20f12a7810 v=holds judge=sonnet head=8834195b0683
at: crates/viola-channel/src/client.rs:125
quote: let frame = json!({"jsonrpc": "2.0", "method": method, "params": params});
at: crates/viola-channel/src/server.rs:11
quote: use interprocess::local_socket::traits::ListenerExt as _;
@row_add n=900 row=decision/pty-seam-portable-pty area=project-directory-structure rec=fct-irhgihut@5
name: portable-pty is the PTY seam
rule: The PTY seam is built over portable-pty, with host raw mode through libc termios.
@row_check n=900 row=decision/pty-seam-portable-pty sha=84e7cf855478 v=holds judge=sonnet head=8834195b0683
at: crates/viola-pty/src/lib.rs:12
quote: use portable_pty::{Child, CommandBuilder, MasterPty, PtySize, native_pty_system};
at: crates/viola-pty/src/lib.rs:234
quote: libc::cfmakeraw(&mut raw);
@row_add n=901 row=decision/unverified-cli-transport-only area=established-decisions rec=fct-7vnaqpcq@1,fct-24sxrnlw@1,fct-qwpklckh@2
name: Transport-only degrade on an unverified CLI
rule: On a CLI version that is not verified, viola still types, runs the wheel and emits events, but withholds dialog answers, drivers get `unverified-cli`, and the readiness gate is partial.
why: The docs lag the installed build, and several load-bearing behaviours are measured, not documented.
@row_check n=901 row=decision/unverified-cli-transport-only sha=82ad93b26d6c v=holds judge=sonnet head=8834195b0683
at: src/run/dialog.rs:436
quote: return Ok(refusal(RefusalReason::UnverifiedCli, None));
at: src/run/gate.rs:90
quote: /// `Some` on a verified CLI version: the full gate. `None`: the partial gate.
@row_add n=902 row=decision/linux-only area=established-decisions rec=fct-hcu7z4ke@2,fct-hss5qy6y@1,fct-igbvtfv6@2,fct-fa2klbw5@3,fct-qndsom6a@3
name: Linux only
rule: viola targets Linux only: the core on any current Linux distribution, built and CI-tested on `ubuntu-latest`, the window features on Hyprland; no Windows or macOS code, test, fixture or CI job stands in the tree.
why: The Windows and macOS support is cut, recoverable at the tag `multi-os-final`.
@row_check n=902 row=decision/linux-only sha=f271c634712f v=holds judge=sonnet head=8834195b0683
at: .github/workflows/ci.yml:18
quote: os: [ubuntu-latest]
searched: windows-latest|macos-latest|cfg(windows) · in .github/workflows, src, crates
@row_add n=903 row=decision/sync-core-tokio-only-in-mcp area=established-decisions rec=fct-l5jpezvw@2,fct-brnw2t5e@1,fct-gmzv3mtg@2
name: Hybrid sync core, Tokio only in mcp
rule: The core is synchronous, and Tokio is confined to the `mcp` subcommand; among the product crates only `viola-mcp` enables `tokio`.
why: portable-pty's handles are blocking, so the pump needs threads either way; `hook` must not pay for runtime startup; rmcp and interprocess-async require Tokio.
guard: sole-root `cargo deny` `tokio` ban (`deny-sync.toml`) and a `cargo check` of the sync crates on `ubuntu-latest`
except: `viola-channel`'s optional `tokio` feature, enabled by `viola-mcp`; its graph without the feature is proven tokio-free.
@row_check n=903 row=decision/sync-core-tokio-only-in-mcp sha=ae563eb0a0e7 v=holds guard=true judge=sonnet head=8834195b0683
at: crates/viola-mcp/Cargo.toml:23
quote: tokio = { workspace = true, features = ["rt"] }
at: crates/viola-channel/Cargo.toml:17
quote: tokio = ["dep:tokio", "interprocess/tokio"]
guard_at: deny-sync.toml:12
@row_add n=904 row=decision/portable-pty-behind-pty-seam area=established-decisions rec=fct-omwrdnlc@3,fct-ad44nn2e@4
name: portable-pty pinned behind viola's pty seam
rule: portable-pty, pinned exactly at `=0.9.0`, is used only behind viola's own `pty` seam (spawn, read, write, resize, wait, kill).
why: Exact, as the workspace pins every third-party version; 0.9.0's graph holds no `pin-utils` (RUSTSEC-2026-0441), and `viola-pty` builds on it unchanged.
@row_check n=904 row=decision/portable-pty-behind-pty-seam sha=0a334eb9ea7e v=holds judge=sonnet head=8834195b0683
at: Cargo.toml:217
quote: portable-pty = "=0.9.0"
at: crates/viola-pty/src/lib.rs:12
quote: use portable_pty::{Child, CommandBuilder, MasterPty, PtySize, native_pty_system};
@row_add n=905 row=decision/child-exit-on-process-handle area=established-decisions rec=fct-omwrdnlc@3,fct-ad44nn2e@4
name: Child exit detected on the process handle
rule: Child exit is detected on the process handle, never on EOF.
@row_check n=905 row=decision/child-exit-on-process-handle sha=a6cb592b05e4 v=holds judge=sonnet head=8834195b0683
at: crates/viola-pty/src/lib.rs:193-196
quote: fn try_wait(&mut self) -> Result<Option<u32>, PtyError> { self.child .try_wait() .map(|s| s.map(|s| s.exit_code()))
@row_add n=906 row=decision/vt100-readiness-gate area=established-decisions rec=fct-hsjl3wtp@1,fct-gollksbj@1
name: vt100 as a pre-send readiness gate only
rule: vt100 0.16.2 serves only as a pre-send readiness gate: after `turn-ended` it waits for a quiet screen, requires no known modal signature and waits, up to its maximum wait, for the version-stamped input-box signature; the passthrough never waits on the model.
why: CLI-native modals bypass every hook, and the screen lags the hooks.
@row_check n=906 row=decision/vt100-readiness-gate sha=996c8f2f1265 v=holds judge=sonnet head=8834195b0683
at: crates/viola-agent-claude/src/screen.rs:142
quote: if now.saturating_duration_since(self.last_fed) < QUIET_PERIOD {
at: crates/viola-agent-claude/src/screen.rs:153
quote: if sigs.modals.iter().any(|m| row.contains(m)) {
at: crates/viola-agent-claude/src/screen.rs:160
quote: } else if now.saturating_duration_since(waiting_since) < GATE_MAX_WAIT {
at: src/run/gate.rs:109
quote: if let Err(TrySendError::Full(_)) = self.tx.try_send(message) {
@row_add n=907 row=decision/ndjson-logs-and-atomic-snapshots area=established-decisions rec=fct-73drmylz@2,fct-ak4xjyyx@3
name: ndjson append logs plus atomic JSON snapshots
rule: State is kept in ndjson append logs and atomic JSON snapshots, with std `File::lock` on separate `.lock` files; the log stays the source of truth.
why: The trail stays human-readable and `tail`-able, with no C in the build and no daemon; SQLite and redb were rejected.
@row_check n=907 row=decision/ndjson-logs-and-atomic-snapshots sha=03b1b1e2096a v=holds judge=sonnet head=8834195b0683
at: crates/viola-state/src/events.rs:68
quote: lock.lock()?;
at: crates/viola-state/src/fs.rs:83
quote: tmp.persist(path).map(|_| ()).map_err(|e| e.error.into())
@row_add n=908 row=decision/separate-lock-files area=established-decisions rec=fct-73drmylz@2,fct-ak4xjyyx@3,fct-s7lo3z2q@2
name: Locks on separate .lock files
rule: State locks are std `File::lock` on separate `.lock` files, opened for write, so no file is both appended to and exclusively locked.
why: A layout chosen while Windows, where append plus exclusive lock fails, was a target, and kept.
@row_check n=908 row=decision/separate-lock-files sha=8029d34a602b v=holds judge=sonnet head=8834195b0683
at: crates/viola-state/src/fs.rs:67
quote: options.create(true).truncate(false).write(true);
at: crates/viola-state/src/events.rs:67
quote: let lock = open_private_lock(&instance_dir.join(EVENTS_LOCK))?;
@row_add n=909 row=decision/tempfile-snapshot-writer area=established-decisions rec=fct-2sdvn2pr@1,fct-kfz3cseb@2
name: tempfile as the snapshot writer
rule: Snapshots and every other replaced file are written through the shared helpers built on tempfile `persist` and `persist_noclobber`; no call site carries its own temp + rename code, a failure leaves the target as it was, and the pinned copy is never replaced.
why: atomic-write-file's BSD-3-Clause licence is outside `deny.toml`'s allow list, and tempfile was already locked.
@row_check n=909 row=decision/tempfile-snapshot-writer sha=c1990fce0fe6 v=holds judge=sonnet head=8834195b0683
at: crates/viola-state/src/fs.rs:83
quote: tmp.persist(path).map(|_| ()).map_err(|e| e.error.into())
at: crates/viola-state/src/fs.rs:118
quote: match tmp.into_temp_path().persist_noclobber(path) {
@row_add n=910 row=decision/ipc-interprocess-local-socket area=established-decisions rec=fct-w2igrcse@2
name: interprocess local_socket for IPC
rule: Inter-process messaging uses interprocess `local_socket`, one endpoint per `viola run`, behind viola's own `channel` seam; `hook` and `wait` block on the channel, so nothing polls.
why: One API over Unix sockets, sync and Tokio flavours, and no daemon.
@row_check n=910 row=decision/ipc-interprocess-local-socket sha=93c36adc4f44 v=holds judge=sonnet head=8834195b0683
at: crates/viola-channel/src/server.rs:123
quote: let listener = ListenerOptions::new()
@row_add n=911 row=decision/identity-by-instance area=established-decisions rec=fct-qwajajfk@1,fct-hag5uwzu@1
name: Identity by instance, never by location
rule: Every hook, MCP call and CLI verb finds its instance through `VIOLA_NAME` or an explicit `ViolaName` argument, never through the working directory; instances are identified by `ViolaName`, endpoints by its short hash, and there are no database keys.
why: Two sessions in one folder never share records; identity is by instance, never by location (R5).
@row_check n=911 row=decision/identity-by-instance sha=0ff0842b3a10 v=holds judge=sonnet head=8834195b0683
at: src/cmd/mod.rs:256
quote: let name = ViolaName::try_new(name?.into_string().ok()?).ok()?;
at: src/cmd/client.rs:210
quote: let raw = std::env::var_os("VIOLA_NAME")?.into_string().ok()?;
@row_add n=912 row=contract/mcp-tools/send area=established-decisions rec=fct-tkwxvt3j@1,fct-xewwdlh6@1,fct-qeiflw5v@1
boundary: MCP tools
item: send
promise: One of the five MCP tools; takes `{target, text}`, `target` a `ViolaName`.
@row_check n=912 row=contract/mcp-tools/send sha=22b0013d395e v=holds judge=sonnet head=8834195b0683
at: crates/viola-mcp/src/tools.rs:370
quote: let (target, text) = match (target_of(args), text_of(args)) {
at: crates/viola-mcp/src/tools.rs:265
quote: .and_then(|name| ViolaName::try_new(name.to_owned()).ok())
@row_add n=913 row=contract/mcp-tools/wait area=established-decisions rec=fct-tkwxvt3j@1,fct-xewwdlh6@1,fct-qeiflw5v@1
boundary: MCP tools
item: wait
promise: One of the five MCP tools; takes `{target, after?, timeout_ms?}`.
@row_check n=913 row=contract/mcp-tools/wait sha=9210f3dff860 v=holds judge=sonnet head=8834195b0683
at: crates/viola-mcp/src/tools.rs:385-387
quote: target_of(args)?,             optional_u64(args, "after")?,             optional_u64(args, "timeout_ms")?,
@row_add n=914 row=contract/mcp-tools/last area=established-decisions rec=fct-tkwxvt3j@1,fct-xewwdlh6@1,fct-qeiflw5v@1
boundary: MCP tools
item: last
promise: One of the five MCP tools; takes `{target}`.
@row_check n=914 row=contract/mcp-tools/last sha=1b0b37fe6a72 v=holds judge=sonnet head=8834195b0683
at: crates/viola-mcp/src/tools.rs:406
quote: let target = match target_of(args) {
at: crates/viola-mcp/src/tools.rs:410
quote: frame(ctx, &target, "last", Map::new(), false).await
@row_add n=915 row=contract/mcp-tools/answer area=established-decisions rec=fct-tkwxvt3j@1,fct-xewwdlh6@1,fct-qeiflw5v@1
boundary: MCP tools
item: answer
promise: One of the five MCP tools; takes `{target, dialog_id, response}`, with `response` in the channel `answer` shape.
@row_check n=915 row=contract/mcp-tools/answer sha=c05861857e68 v=holds judge=sonnet head=8834195b0683
at: crates/viola-mcp/src/tools.rs:415
quote: (|| Ok::<_, &'static str>((target_of(args)?, dialog_id_of(args)?, response_of(args)?)))();
at: crates/viola-mcp/src/tools.rs:290
quote: .and_then(|value| Response::parse(value).ok())
@row_add n=916 row=contract/mcp-tools/list area=established-decisions rec=fct-tkwxvt3j@1,fct-xewwdlh6@1,fct-qeiflw5v@1
boundary: MCP tools
item: list
promise: One of the five MCP tools; takes `{}` and shows the driver the sessions.
@row_check n=916 row=contract/mcp-tools/list sha=6daecb98af40 v=holds judge=sonnet head=8834195b0683
at: crates/viola-mcp/src/tools.rs:124
quote: struct ListInput {}
at: crates/viola-mcp/src/tools.rs:442
quote: result::of_board(&document)
@row_add n=917 row=contract/hook-tiers/sync-with-decision area=established-decisions rec=fct-zhr3wx2i@2,fct-xbwprhwr@2
boundary: Hook tiers
item: sync with a decision
promise: PreToolUse with matcher `AskUserQuestion|ExitPlanMode`, and PermissionRequest, run sync and may reply with a decision.
@row_check n=917 row=contract/hook-tiers/sync-with-decision sha=34f65cc55d62 v=holds judge=sonnet head=8834195b0683
at: plugin/hooks/hooks.json:10
quote: {"matcher": "AskUserQuestion|ExitPlanMode", "hooks": [{"type": "command", "command": "@@VIOLA_BIN@@", "args": ["hook", "pre-tool-use"], "timeout": 3615}]}
at: plugin/hooks/hooks.json:13
quote: {"hooks": [{"type": "command", "command": "@@VIOLA_BIN@@", "args": ["hook", "permission-request"], "timeout": 3615}]}
at: src/cmd/hook.rs:298
quote: out.write_all(body.as_bytes())?;
@row_add n=918 row=contract/hook-tiers/async area=established-decisions rec=fct-zhr3wx2i@2,fct-xbwprhwr@2
boundary: Hook tiers
item: async
promise: Notification, PostToolUse and PostToolUseFailure run with `async: true` and print nothing.
@row_check n=918 row=contract/hook-tiers/async sha=d47da5d9cdbd v=holds judge=sonnet head=8834195b0683
at: plugin/hooks/hooks.json:22
quote: {"hooks": [{"type": "command", "command": "@@VIOLA_BIN@@", "args": ["hook", "notification"], "async": true}]}
at: plugin/hooks/hooks.json:25
quote: {"hooks": [{"type": "command", "command": "@@VIOLA_BIN@@", "args": ["hook", "post-tool-use"], "async": true}]}
at: plugin/hooks/hooks.json:28
quote: {"hooks": [{"type": "command", "command": "@@VIOLA_BIN@@", "args": ["hook", "post-tool-use-failure"], "async": true}]}
@row_add n=919 row=decision/hooks-fail-open area=established-decisions rec=fct-4nt6qtyp@3,fct-nppkdemz@1,fct-wpbp4hte@2
name: Hooks fail open
rule: `viola hook` always exits 0, including on parse errors, channel failures and caught panics, and at once when `VIOLA_NAME` is absent; a JSON body is a decision, no body no decision; exit 2 is never produced and `hook` never writes to stderr.
why: Exit 2 blocks Claude Code, and a viola bug must never block the session.
except: The hidden `--capture` arms, called only by `viola verify`'s probes, read no `VIOLA_*` and open no channel, still exiting 0 with empty stderr.
@row_check n=919 row=decision/hooks-fail-open sha=f8281fbc2346 v=holds judge=sonnet head=8834195b0683
at: src/main.rs:133
quote: (Role::Hook, _) => ExitCode::SUCCESS,
at: src/cmd/hook.rs:117
quote: return Ok(ExitCode::SUCCESS);
at: src/cmd/hook.rs:101-106
quote: if let Some(dir) = &args.capture { let filed = capture(event, dir, &mut std::io::stdin().lock()); if let (Some(filed), Some(answers)) = (filed, &args.answers) { answer(event, dir, answers, &filed, &mut std::io::stdout().lock()); } return Ok(ExitCode::SUCCESS);
@row_add n=920 row=decision/wheel-starts-with-driver area=established-decisions rec=fct-dl246x3d@1,fct-4coxmu76@2
name: The wheel starts with the driver
rule: The wheel starts with `driver` at every start of `viola run`, so automation is accepted until the first human editing key.
@row_check n=920 row=decision/wheel-starts-with-driver sha=7c13a2377bf0 v=holds judge=sonnet head=8834195b0683
at: src/run/wheel.rs:91
quote: holder: Wheel::Driver,
@row_add n=921 row=decision/send-never-queued area=established-decisions rec=fct-dl246x3d@1,fct-4coxmu76@2
name: A send mid-turn or mid-send is refused, never queued
rule: A `send` while a turn runs or while another `send` is in flight is refused `not-delivered` with detail `turn-running`, never queued, and nothing is typed for it.
@row_check n=921 row=decision/send-never-queued sha=df2dbb2fad2c v=holds judge=sonnet head=8834195b0683
at: src/run/send.rs:114-116
quote: if flight.send.is_some() || slot.wheel.turn_running() { drop(flight); return ctx.not_delivered(None, NotDelivered::TurnRunning);
@row_add n=922 row=decision/nutype-newtypes area=established-decisions rec=fct-caebuvjq@1,fct-tkfup5lp@1
name: nutype newtypes for validation
rule: Validated values are nutype newtypes (`ViolaName`, `Percent`, `DialogHold`); no garde or validator.
@row_check n=922 row=decision/nutype-newtypes sha=767769877408 v=holds judge=sonnet head=8834195b0683
at: crates/viola-core/src/lib.rs:52
quote: pub struct ViolaName(String);
at: crates/viola-core/src/lib.rs:271
quote: pub struct Percent(u8);
at: crates/viola-core/src/lib.rs:280
quote: pub struct DialogHold(u16);
@row_add n=923 row=decision/plugin-compiled-into-binary area=established-decisions rec=fct-c3wssf3l@2,fct-23yj6ah3@2
name: Plugin compiled into the binary
rule: viola is installed by `cargo install --path .`, with the plugin compiled into the binary and the plugin files rewritten from the binary on every start.
why: The binary and the plugin can no longer drift apart.
@row_check n=923 row=decision/plugin-compiled-into-binary sha=87ca71686ede v=holds judge=sonnet head=8834195b0683
at: crates/viola-agent-claude/src/lib.rs:126
quote: include_str!(concat!(
at: src/cmd/run.rs:478
quote: replace_private_shared(&path, content.as_bytes(), FILE_MODE)?;
@row_add n=924 row=decision/pinned-exe-copy area=established-decisions rec=fct-c3wssf3l@2,fct-23yj6ah3@2
name: Pinned exe copy keyed by version and content hash
rule: At start `viola run` pins its own exe under the viola home keyed by version and content hash, refusing a copy that fails its re-hash, so a rebuild never changes what a running session calls.
why: The version alone collides across rebuilds, so the content hash keys the copy. Trade-off: a hook fix applies only to sessions started after the rebuild.
@row_check n=924 row=decision/pinned-exe-copy sha=c3c3dc3a4937 v=holds judge=sonnet head=8834195b0683
at: crates/viola-state/src/pin.rs:71
quote: format!("{VERSION}-{}", content_key(bytes))
at: crates/viola-state/src/pin.rs:90
quote: Ok(_) => return Err(PinError::HashMismatch),
@row_add n=925 row=decision/plugin-only-in-wrapped-sessions area=established-decisions rec=fct-le5sc2tk@1,fct-4jk25cex@2
name: Plugin loads only into wrapped sessions
rule: The viola plugin with its hooks loads only into wrapped sessions via `--plugin-dir`; both driver and driven run under `viola run`.
why: Unwrapped sessions carry no viola hooks, so there is no per-tool-call spawn cost outside viola.
except: `viola verify`'s transient unwrapped children, all gone when verify ends.
@row_check n=925 row=decision/plugin-only-in-wrapped-sessions sha=d8f380410ed2 v=holds judge=sonnet head=8834195b0683
at: src/run/mod.rs:129
quote: OsString::from(PLUGIN_DIR_FLAG),
@row_add n=926 row=decision/prompt-text-from-stdin-or-file area=established-decisions rec=fct-nvft5tqc@2,fct-onw4pywk@3
name: Prompt text from stdin or --file
rule: Prompt text comes from stdin or `--file`, never from a leading-slash argument.
@row_check n=926 row=decision/prompt-text-from-stdin-or-file sha=287ddb33ce3d v=holds judge=sonnet head=8834195b0683
at: src/cmd/send.rs:48
quote: Err("the text is read from stdin or --file, never from an argument".to_owned())
@row_add n=927 row=absence/mobile-view area=established-decisions rec=fct-6zkbklpd@1,fct-cmvs3cxm@1
capability: A mobile or phone view
@row_check n=927 row=absence/mobile-view sha=f70a02fa425c v=holds judge=sonnet head=8834195b0683
searched: (?i)mobile|phone|responsive|android|ios · in src/**/*.rs and crates/**/*.rs
@row_add n=928 row=contract/channel-methods/send area=standard-contracts rec=fct-ekt3xwun@1,fct-vi22dxmq@1,fct-4ghn4t2o@1
boundary: channel methods
item: send
promise: A request: `send` `{text, from?}` answers `{submitted_at, cursor}` when confirmed, or `{confirmed:false, detail:"unconfirmable", cursor}`; `cursor` is the end offset of `events.ndjson` when the send was accepted, before the paste.
@row_check n=928 row=contract/channel-methods/send sha=a69ba7ce1f12 v=holds judge=sonnet head=8834195b0683
at: src/run/send.rs:215
quote: Ok(json!({"ok": {"submitted_at": submitted_at, "cursor": cursor}}))
at: src/run/send.rs:224
quote: Ok(json!({"ok": {"confirmed": false, "detail": "unconfirmable", "cursor": cursor}}))
at: src/run/send.rs:179
quote: let cursor = append_event_at(self.instance_dir, |cursor| {
@row_add n=929 row=contract/channel-methods/wait area=standard-contracts rec=fct-rgtvcxfi@1,fct-vi22dxmq@1,fct-4ghn4t2o@1
boundary: channel methods
item: wait
promise: A request: `wait` `{after?, timeout_ms?, from?}` answers `{event:<ndjson event line>, cursor}` or `{timed_out:true}`; a line counts only when it starts at or after `after`, else the log's end at the call.
@row_check n=929 row=contract/channel-methods/wait sha=45ac7040b94e v=holds judge=sonnet head=8834195b0683
at: src/run/wait.rs:155
quote: return Ok(json!({"ok": {"event": line.value, "cursor": line.end}}));
at: src/run/wait.rs:161
quote: return Ok(json!({"ok": {"timed_out": true}}));
at: src/run/wait.rs:145
quote: None => end_offset(instance_dir).map_err(|_| ProtocolError::Internal)?,
@row_add n=930 row=contract/channel-methods/last area=standard-contracts rec=fct-5chyftpi@1,fct-vi22dxmq@1,fct-4ghn4t2o@1
boundary: channel methods
item: last
promise: A request: `last` `{from?}` answers `{last_assistant_message:<string|null>, ts:<string|null>}`: both `null` before any turn; a `turn-ended` whose message is not a string gives `null` plus its `ts`.
@row_check n=930 row=contract/channel-methods/last sha=e2c6fe2f844c v=holds judge=sonnet head=8834195b0683
at: src/run/wait.rs:176-177
quote: let (message, ts) = newest.map_or((None, None), |t| (t.message, Some(t.ts))); Ok(json!({"ok": {"last_assistant_message": message, "ts": ts}}))
at: src/run/wait.rs:34
quote: message: data["last_assistant_message"].as_str().map(str::to_owned),
@row_add n=931 row=contract/channel-methods/answer area=standard-contracts rec=fct-er6xfhrr@1,fct-vi22dxmq@1,fct-4ghn4t2o@1
boundary: channel methods
item: answer
promise: A request: `answer` `{dialog_id, from?, response}` answers `{}`, with `response` normalised per dialog kind.
@row_check n=931 row=contract/channel-methods/answer sha=60e69f139b11 v=holds judge=sonnet head=8834195b0683
at: src/run/dialog.rs:470
quote: Ok(json!({"ok": {}}))
at: crates/viola-agent-claude/src/dialog.rs:211
quote: pub fn parse(response: &Value) -> Result<Self, AgentError> {
@row_add n=932 row=contract/channel-methods/pause area=standard-contracts rec=fct-7jswn64g@1,fct-vi22dxmq@1,fct-4ghn4t2o@1
boundary: channel methods
item: pause
promise: A request: `pause` `{from?}` answers `{wheel:"human"}`, the wheel moving to `human` / `manual-pause`.
@row_check n=932 row=contract/channel-methods/pause sha=1eb4e0cdb7eb v=holds judge=sonnet head=8834195b0683
at: src/run/wheel.rs:191
quote: Ok(json!({"ok": {"wheel": Wheel::Human.as_str()}}))
at: src/run/wheel.rs:44
quote: (Input::Pause, _, _) => Some((Wheel::Human, WheelCause::ManualPause)),
@row_add n=933 row=contract/channel-methods/release area=standard-contracts rec=fct-3nvzp4be@1,fct-vi22dxmq@1,fct-4ghn4t2o@1
boundary: channel methods
item: release
promise: A request: `release` `{budget?:bool, from?}` answers `{wheel:<holder>, budget_paused:bool, override_until?}`.
@row_check n=933 row=contract/channel-methods/release sha=fa155c3a9be5 v=holds judge=sonnet head=8834195b0683
at: src/run/wheel.rs:215-217
quote: let mut ok = json!({"wheel": self.holder().as_str(), "budget_paused": false}); if let Some(end) = lifted { ok["override_until"] = json!(end);
at: src/run/wheel.rs:222
quote: Ok(json!({"ok": {"wheel": self.holder().as_str(), "budget_paused": gate.paused()}}))
@row_add n=934 row=contract/channel-methods/link area=standard-contracts rec=fct-ax5zoxh3@1,fct-vi22dxmq@1,fct-4ghn4t2o@1
boundary: channel methods
item: link
promise: A request: `link` `{driver, driven}` answers `{since, already_linked}`, sent to the driven instance's wrapper.
@row_check n=934 row=contract/channel-methods/link sha=9f143cc5690b v=holds judge=sonnet head=8834195b0683
at: src/run/links.rs:75
quote: Ok(json!({"ok": {"since": since, "already_linked": already_linked}}))
at: src/cmd/run.rs:147
quote: "link" => self.links.link(call.params),
@row_add n=935 row=contract/channel-methods/unlink area=conventions rec=fct-4ghn4t2o@1,fct-ax5zoxh3@1,fct-vi22dxmq@1
boundary: channel methods
item: unlink
promise: A request with an integer JSON-RPC id, monotonic per connection; `unlink` `{driver, driven}` answers `{}`, sent to the driven instance's wrapper.
@row_check n=935 row=contract/channel-methods/unlink sha=c657d30a80fe v=holds judge=sonnet head=8834195b0683
at: src/run/links.rs:82
quote: Ok(json!({"ok": {}}))
at: crates/viola-channel/src/client.rs:74-75
quote: let id = self.next_id; self.next_id += 1;
at: src/cmd/run.rs:148
quote: "unlink" => self.links.unlink(call.params),
@row_add n=936 row=contract/channel-methods/hook-dialog area=conventions rec=fct-4ghn4t2o@1,fct-vwuy3ine@1,fct-vi22dxmq@1
boundary: channel methods
item: hook.dialog
promise: A request with an integer JSON-RPC id, monotonic per connection; `{kind:"question"|"permission"|"plan", data, hook_event, tool?, input?, continuation?}` answers `{dialog_id, response:<object|null>}`; `null` means no decision, so the hook prints nothing.
@row_check n=936 row=contract/channel-methods/hook-dialog sha=ca553f1dff95 v=holds judge=sonnet head=8834195b0683
at: src/run/dialog.rs:142
quote: json!({"ok": {"dialog_id": dialog_id, "response": response.map(Response::to_value)}})
at: crates/viola-channel/src/client.rs:74-75
quote: let id = self.next_id; self.next_id += 1;
at: src/cmd/run.rs:143
quote: "hook.dialog" => self.dialogs.hook_dialog(call.params),
@row_add n=937 row=contract/channel-methods/hook-event area=conventions rec=fct-4ghn4t2o@1,fct-nf7gdkwl@1,fct-vi22dxmq@1
boundary: channel methods
item: hook.event
promise: A notification sent id-less by `hook`, no reply, `{ts, event:{kind, data}}`; the wrapper re-validates the event and appends it to `events.ndjson` with `source:"hook"`, an invalid event not appended.
@row_check n=937 row=contract/channel-methods/hook-event sha=5b0cbefc8f67 v=holds judge=sonnet head=8834195b0683
at: src/cmd/run.rs:108-114
quote: Some(EventLine::new( &self.name, kind, Source::Hook, data.clone(), Utc::now(), ))
at: src/cmd/run.rs:124-126
quote: if let Some(line) = self.hook_event_line(params) { let turn_ended = line.kind == EventKind::TurnEnded; send::append_hook_event(&self.send, &self.wait, &self.instance_dir, line)
@row_add n=938 row=contract/refusal-reason/human-typing area=conventions rec=fct-xscdpgpr@1,fct-5uakjii2@1
boundary: RefusalReason
item: human-typing
promise: Serialised kebab-case; its detail is null or manual-pause.
@row_check n=938 row=contract/refusal-reason/human-typing sha=ef8518ba170d v=holds judge=sonnet head=8834195b0683
at: crates/viola-core/src/lib.rs:172
quote: Self::HumanTyping => "human-typing",
at: crates/viola-core/src/lib.rs:242
quote: Self::ManualPause => "manual-pause",
@row_add n=939 row=contract/refusal-reason/budget-paused area=conventions rec=fct-xscdpgpr@1,fct-5uakjii2@1
boundary: RefusalReason
item: budget-paused
promise: Serialised kebab-case; its detail is five-hour or seven-day.
@row_check n=939 row=contract/refusal-reason/budget-paused sha=f0df8ad63951 v=holds judge=sonnet head=8834195b0683
at: crates/viola-core/src/lib.rs:173
quote: Self::BudgetPaused => "budget-paused",
at: crates/viola-core/src/lib.rs:259-260
quote: Self::FiveHour => "five-hour", Self::SevenDay => "seven-day",
@row_add n=940 row=contract/refusal-reason/unverified-cli area=conventions rec=fct-xscdpgpr@1,fct-5uakjii2@1
boundary: RefusalReason
item: unverified-cli
promise: Serialised kebab-case; its detail is null.
@row_check n=940 row=contract/refusal-reason/unverified-cli sha=59ee57f5ea27 v=holds judge=sonnet head=8834195b0683
at: src/run/dialog.rs:436
quote: return Ok(refusal(RefusalReason::UnverifiedCli, None));
@row_add n=941 row=contract/refusal-reason/not-delivered area=conventions rec=fct-xscdpgpr@1,fct-5uakjii2@1
boundary: RefusalReason
item: not-delivered
promise: Serialised kebab-case; its detail is one of input-not-ready, no-prompt-submitted, turn-running, unknown-dialog, control-character, empty-text.
@row_check n=941 row=contract/refusal-reason/not-delivered sha=284d42abcd55 v=holds judge=sonnet head=8834195b0683
at: crates/viola-core/src/lib.rs:221-226
quote: Self::InputNotReady => "input-not-ready", Self::NoPromptSubmitted => "no-prompt-submitted", Self::TurnRunning => "turn-running", Self::UnknownDialog => "unknown-dialog", Self::ControlCharacter => "control-character", Self::EmptyText => "empty-text",
@row_add n=942 row=contract/cli-exit-codes/0 area=conventions rec=fct-jdhp6aqu@1,fct-vuikun3q@2
boundary: CLI exit codes
item: 0
promise: ok.
@row_check n=942 row=contract/cli-exit-codes/0 sha=574f59127cd6 v=holds judge=sonnet head=8834195b0683
at: src/cmd/list.rs:75
quote: Ok(ExitCode::SUCCESS)
@row_add n=943 row=contract/cli-exit-codes/1 area=conventions rec=fct-jdhp6aqu@1,fct-vuikun3q@2,fct-2y3oelpa@1,fct-czy74f6t@1,fct-zvpgaziz@1
boundary: CLI exit codes
item: 1
promise: Internal error, and the start, revive-preflight and verify refusals, a refused home-tree check or socket directory of the start among them, each an unable and hint pair on stderr with no path, or under --json one refusal document.
@row_check n=943 row=contract/cli-exit-codes/1 sha=4a62ff2d86f2 v=holds judge=sonnet head=8834195b0683
at: src/cmd/run.rs:413
quote: Started::Refused(ExitCode::from(1))
at: src/cmd/revive.rs:139
quote: return Ok(Started::Refused(ExitCode::from(1)));
at: src/cmd/verify.rs:69
quote: return Ok(ExitCode::from(1));
@row_add n=944 row=contract/cli-exit-codes/2 area=conventions rec=fct-jdhp6aqu@1,fct-vuikun3q@2
boundary: CLI exit codes
item: 2
promise: Usage error, and viola send text over 16 MiB or not UTF-8; never produced by viola hook.
@row_check n=944 row=contract/cli-exit-codes/2 sha=39025b18b27b v=holds judge=sonnet head=8834195b0683
at: src/cmd/send.rs:165
quote: return Ok(ExitCode::from(2));
at: src/cmd/send.rs:58
quote: if bytes.len() as u64 > MAX_FRAME {
at: src/main.rs:52
quote: Err(usage) => usage.exit(),
at: src/main.rs:133
quote: (Role::Hook, _) => ExitCode::SUCCESS,
@row_add n=945 row=contract/cli-exit-codes/10 area=conventions rec=fct-jdhp6aqu@1,fct-vuikun3q@2
boundary: CLI exit codes
item: 10
promise: The `human-typing` refusal.
@row_check n=945 row=contract/cli-exit-codes/10 sha=e5d10bc3fec2 v=holds judge=sonnet head=8834195b0683
at: src/cmd/send.rs:67
quote: RefusalReason::HumanTyping => 10,
@row_add n=946 row=contract/cli-exit-codes/11 area=conventions rec=fct-jdhp6aqu@1,fct-vuikun3q@2
boundary: CLI exit codes
item: 11
promise: The `budget-paused` refusal.
@row_check n=946 row=contract/cli-exit-codes/11 sha=05ec770d18f3 v=holds judge=sonnet head=8834195b0683
at: src/cmd/send.rs:68
quote: RefusalReason::BudgetPaused => 11,
@row_add n=947 row=contract/cli-exit-codes/12 area=conventions rec=fct-jdhp6aqu@1,fct-vuikun3q@2
boundary: CLI exit codes
item: 12
promise: The `unverified-cli` refusal.
@row_check n=947 row=contract/cli-exit-codes/12 sha=1362eb4efbe3 v=holds judge=sonnet head=8834195b0683
at: src/cmd/send.rs:69
quote: RefusalReason::UnverifiedCli => 12,
@row_add n=948 row=contract/cli-exit-codes/13 area=conventions rec=fct-jdhp6aqu@1,fct-vuikun3q@2
boundary: CLI exit codes
item: 13
promise: The `not-delivered` refusal.
@row_check n=948 row=contract/cli-exit-codes/13 sha=d19db365b575 v=holds judge=sonnet head=8834195b0683
at: src/cmd/send.rs:70
quote: RefusalReason::NotDelivered => 13,
@row_add n=949 row=contract/cli-exit-codes/14 area=conventions rec=fct-jdhp6aqu@1,fct-vuikun3q@2
boundary: CLI exit codes
item: 14
promise: The `unknown` refusal.
@row_check n=949 row=contract/cli-exit-codes/14 sha=214e1d86c495 v=holds judge=sonnet head=8834195b0683
at: src/cmd/send.rs:71
quote: RefusalReason::Unknown => 14,
@row_add n=950 row=contract/cli-exit-codes/20 area=conventions rec=fct-jdhp6aqu@1,fct-vuikun3q@2
boundary: CLI exit codes
item: 20
promise: Protocol fault from the wrapper, a -32602 from link or unlink included; the link self-pair is refused by the wrapper only, never client-side.
@row_check n=950 row=contract/cli-exit-codes/20 sha=ff63b0aaa04e v=holds judge=sonnet head=8834195b0683
at: src/cmd/client.rs:245
quote: 20
at: src/run/links.rs:95
quote: if driven != self.name || driver == driven {
at: src/run/links.rs:92
quote: .ok_or(ProtocolError::InvalidParams)
@row_add n=951 row=contract/cli-exit-codes/21 area=conventions rec=fct-jdhp6aqu@1,fct-vuikun3q@2,fct-7s455w3m@2,fct-2y2sv475@1,fct-uqyfijk4@2
boundary: CLI exit codes
item: 21
promise: Instance unreachable: no live endpoint, a server that is not the recorded wrapper, a refused strict-modes check of the board, the plugin install tree or the target or driven instance, or a refused socket directory; no frame is written.
@row_check n=951 row=contract/cli-exit-codes/21 sha=91da857303f8 v=holds judge=sonnet head=8834195b0683
at: src/cmd/client.rs:265
quote: 21
at: src/cmd/client.rs:40
quote: Ok(Some(21))
at: src/cmd/list.rs:47
quote: return Ok(ExitCode::from(21));
at: src/cmd/plugin.rs:60
quote: return Ok(ExitCode::from(21));
at: src/cmd/client.rs:195
quote: Err(CallError::SocketDirRefused) => return Ok(Err(Unreachable::SocketDir)),
at: src/cmd/client.rs:194
quote: Err(CallError::ServerUnverified) => return Ok(Err(Unreachable::ServerUnverified)),
@row_add n=952 row=contract/mcp-tool-results/success area=conventions rec=fct-5nrjpmo2@1,fct-4bh4742d@1
boundary: MCP tool results
item: success
promise: A success is a tool result with `isError: false` and the channel `ok` payload as `structuredContent`; for `list`, the caller-side `{items, budget, skipped}` payload.
@row_check n=952 row=contract/mcp-tool-results/success sha=415aa4d09f10 v=holds judge=sonnet head=8834195b0683
at: crates/viola-mcp/src/result.rs:16
quote: CallToolResult::structured(payload)
at: crates/viola-mcp/src/result.rs:89
quote: return Ok(ok(payload.clone()));
at: src/cmd/list.rs:178
quote: json!({"v": 1, "ok": {"items": board.items, "budget": budget, "skipped": board.skipped}})
@row_add n=953 row=contract/mcp-json-rpc-errors/32602 area=conventions rec=fct-5nrjpmo2@1,fct-4bh4742d@1
boundary: MCP JSON-RPC errors
item: -32602
promise: An unknown tool name is the JSON-RPC error `-32602`; JSON-RPC errors are only the MCP connection's own faults.
@row_check n=953 row=contract/mcp-json-rpc-errors/32602 sha=e267bd2db2bc v=holds judge=sonnet head=8834195b0683
at: crates/viola-mcp/src/tools.rs:234
quote: return Err(ErrorData::invalid_params(
at: crates/viola-mcp/src/tools.rs:235
quote: Cow::Borrowed("unknown tool"),
@row_add n=954 row=contract/mcp-json-rpc-errors/32603 area=conventions rec=fct-5nrjpmo2@1,fct-4bh4742d@1
boundary: MCP JSON-RPC errors
item: -32603
promise: An internal handler fault (any other channel error, a reply of no known shape, a `wait` / `last` refusal, a panicked call) is `-32603` `internal error` with `data: null`.
@row_check n=954 row=contract/mcp-json-rpc-errors/32603 sha=f457a19175a0 v=holds judge=sonnet head=8834195b0683
at: crates/viola-mcp/src/tools.rs:243
quote: Ok(Err(McpError::Internal)) | Err(_) => Err(internal_fault()),
at: crates/viola-mcp/src/result.rs:57
quote: Err(_) => return Err(McpError::Internal),
at: crates/viola-mcp/src/result.rs:64
quote: let result = frame.get("result").ok_or(McpError::Internal)?;
at: crates/viola-mcp/src/result.rs:72
quote: .ok_or(McpError::Internal)?;
at: crates/viola-mcp/src/tools.rs:194
quote: ErrorCode::INTERNAL_ERROR,
@row_add n=955 row=absence/database area=conventions rec=fct-tnxtyezf@1,fct-gfhsqoso@1
capability: A database
@row_check n=955 row=absence/database sha=3172c29a0c8e v=holds judge=sonnet head=8834195b0683
searched: rusqlite|sqlx|sqlite|diesel|sled|rocksdb|redb|postgres|database (case-insensitive) · in Cargo.toml, crates/*/Cargo.toml, src/**/*.rs, crates/*/src/**/*.rs
@row_add n=956 row=contract/list-payload/budget area=standard-contracts rec=fct-beanu3w4@1,fct-oy7hacpg@1,fct-dtnnvvl3@1
boundary: list payload
item: budget
promise: A top-level `budget`, `{five_hour, seven_day, read_at, paused}` or `"unknown"`; each window is `{used_percentage, resets_at}`, a field `"unknown"` when unparseable and a window `"unknown"` when omitted; `paused` only says the newest reading crosses a threshold.
@row_check n=956 row=contract/list-payload/budget sha=a37436853887 v=holds judge=sonnet head=8834195b0683
at: src/cmd/list.rs:170-176
quote: Some(budget) => json!({ "five_hour": budget.stamped.reading.five_hour, "seven_day": budget.stamped.reading.seven_day, "read_at": budget.stamped.read_at, "paused": budget.paused, }), None => json!("unknown"),
at: crates/viola-core/src/lib.rs:349-351
quote: pub struct BudgetReading { pub five_hour: Reading<BudgetWindow>, pub seven_day: Reading<BudgetWindow>,
@row_add n=957 row=contract/snapshot-envelope/shape area=standard-contracts rec=fct-vufggymy@1,fct-uve5rstc@1
boundary: snapshot envelope
item: shape
promise: A snapshot is atomically replaced JSON of the shape `{v, written_at, writer, data}`.
@row_check n=957 row=contract/snapshot-envelope/shape sha=fc4784c22646 v=holds judge=sonnet head=8834195b0683
at: crates/viola-state/src/snapshot.rs:83-88
quote: struct Envelope<T> { v: u32, written_at: String, writer: String, data: T, }
at: crates/viola-state/src/snapshot.rs:101
quote: replace_private(&instance_dir.join(SNAPSHOT), &bytes, FILE_MODE)
@row_add n=958 row=contract/ledger-stamps/writer area=standard-contracts rec=fct-scthzguh@1,fct-62qgonvf@2
boundary: ledger stamps envelope
item: writer
promise: `ledger/stamps.json` is atomically replaced JSON written only by `viola verify`; its `writer` is the literal `"verify"`, not a binary version.
@row_check n=958 row=contract/ledger-stamps/writer sha=95d70b312a63 v=holds judge=sonnet head=8834195b0683
at: crates/viola-agent-claude/src/ledger.rs:1157
quote: doc.insert("writer".to_owned(), json!("verify"));
at: crates/viola-state/src/stamps.rs:70
quote: Ok(replace_private(&path, &bytes, FILE_MODE)?)
at: src/cmd/verify.rs:273
quote: let written = update_stamps(home, |existing| {
@row_add n=959 row=contract/instance-snapshot/links area=standard-contracts rec=fct-hv3qoyp2@1,fct-q5tbsmkw@1,fct-vgnixbh3@1
boundary: instance snapshot
item: links
promise: `links` is the set derived by replaying `link`/`unlink` events, as `{driver, driven, since}` with `since` the `link` line's `ts`; the wrapper restores it at every start, a revived one included, and a failed append leaves it as it was.
@row_check n=959 row=contract/instance-snapshot/links sha=86c4495d5dac v=holds judge=sonnet head=8834195b0683
at: src/run/links.rs:133
quote: apply_link_line(set, &logged);
at: src/cmd/run.rs:333
quote: links.restore()?;
at: src/run/links.rs:132
quote: append_event(&self.instance_dir, line)?;
@row_add n=960 row=contract/list-payload/ok area=standard-contracts rec=fct-cqamt3xw@1,fct-smoe4pw5@1
boundary: list payload
item: ok
promise: The `list` `ok` payload, CLI `--json` and MCP `list` alike, is `{items, budget, skipped}`; `list` is not a channel method and needs no live wrapper.
@row_check n=960 row=contract/list-payload/ok sha=314c4c489395 v=holds judge=sonnet head=8834195b0683
at: src/cmd/list.rs:178
quote: json!({"v": 1, "ok": {"items": board.items, "budget": budget, "skipped": board.skipped}})
at: src/cmd/mcp.rs:35
quote: Arc::new(move || list::board_document(&board_home).unwrap_or(Value::Null));
@row_add n=961 row=contract/mcp-refusal-result/shape area=standard-contracts rec=fct-2ry3h2lh@1,fct-lcysncyb@1
boundary: MCP refusal result
item: result shape
promise: `{"content":[{"type":"text","text":"refused: budget-paused"}],"isError":true,"structuredContent":{"refusal":"budget-paused","detail":"five-hour"}}`
@row_check n=961 row=contract/mcp-refusal-result/shape sha=e9c264aeda01 v=holds judge=sonnet head=8834195b0683
at: crates/viola-mcp/src/result.rs:21
quote: failed(reason, json!({"refusal": reason, "detail": detail}))
at: crates/viola-mcp/src/result.rs:40
quote: let mut result = CallToolResult::structured_error(body);
@row_add n=962 row=decision/no-shell-out area=standard-contracts rec=fct-horlp4ay@2,fct-fa2klbw5@3
name: No shelling out
rule: Nothing shells out through `sh` or `bash`; child processes are spawned directly.
except: `hook statusline` runs the user's own statusline command, read at start from one named source, through `/bin/sh -c` with the same stdin.
@row_check n=962 row=decision/no-shell-out sha=22ee66747e6c v=holds judge=sonnet head=8834195b0683
searched: Command::new("sh") / Command::new("bash") / "bash" / "sh" · in src/**/*.rs and crates/**/*.rs (only product shell-out is the statusline /bin/sh; one dev harness script call in crates/viola-e2e/src/harness/pre_push/linux.rs is not product code)
searched: BIN_SH / shell_argv · in crates/viola-agent-claude/src/statusline.rs (line 21 `pub const BIN_SH: &str = "/bin/sh";`, line 123 `pub fn shell_argv(command: &str) -> [&str; 3] {`)
@row_add n=963 row=decision/connect-to-recorded-endpoint area=occupied-resources rec=fct-wular7qm@2,fct-nm6raycf@2
name: Clients connect to the recorded endpoint
rule: The wrapper resolves the endpoint path once and records it in the instance snapshot; every other process connects to that recorded `endpoint` and none rebuilds the path.
why: Processes spawned inside the session may run with a different environment from the wrapper.
@row_check n=963 row=decision/connect-to-recorded-endpoint sha=1ea968c4ebef v=holds judge=sonnet head=8834195b0683
at: src/cmd/run.rs:496
quote: let endpoint = viola_channel::endpoint_path(name, home)?;
at: crates/viola-state/src/liveness.rs:101
quote: endpoint: snapshot.endpoint?,
@row_add n=964 row=decision/no-abstract-namespace-sockets area=occupied-resources rec=fct-wular7qm@2,fct-nm6raycf@2
name: No abstract-namespace sockets
rule: Linux abstract-namespace socket names are never used.
why: They carry no filesystem permissions and any local user could connect.
@row_check n=964 row=decision/no-abstract-namespace-sockets sha=4485ce4025c1 v=holds judge=sonnet head=8834195b0683
searched: GenericNamespaced · in src/**/*.rs and crates/**/*.rs
searched: ToNsName · in src/**/*.rs and crates/**/*.rs
searched: to_ns_name · in src/**/*.rs and crates/**/*.rs
searched: NamespacedNameType · in src/**/*.rs and crates/**/*.rs
@row_add n=965 row=name/binary/viola area=occupied-resources rec=fct-lvi3drxp@2,fct-irhgihut@5
class: binary
name: viola
@row_check n=965 row=name/binary/viola sha=12227ba01072 v=holds judge=tool head=8834195b0683
at: Cargo.toml:13
quote: name = "viola"
@row_add n=966 row=name/binary/viola-fake-agent area=occupied-resources rec=fct-v2uqfwgj@1,fct-bzckmn3n@2,fct-irhgihut@5
class: binary
name: viola-fake-agent
@row_check n=966 row=name/binary/viola-fake-agent sha=aa5e3d45a12e v=holds judge=tool head=8834195b0683
at: Cargo.toml:17
quote: name = "viola-fake-agent"
@row_add n=967 row=name/binary/viola-harness area=occupied-resources rec=fct-v2uqfwgj@1,fct-j2leq5fi@1,fct-bzckmn3n@2,fct-irhgihut@5
class: binary
name: viola-harness
@row_check n=967 row=name/binary/viola-harness sha=8c646d9cfbe0 v=holds judge=tool head=8834195b0683
at: crates/viola-e2e/Cargo.toml:15
quote: name = "viola-harness"
@row_add n=968 row=name/binary/viola-mcp-tap area=occupied-resources rec=fct-v2uqfwgj@1,fct-j2leq5fi@1,fct-bzckmn3n@2,fct-irhgihut@5
class: binary
name: viola-mcp-tap
@row_check n=968 row=name/binary/viola-mcp-tap sha=0130a5cb4dc6 v=holds judge=tool head=8834195b0683
at: crates/viola-e2e/Cargo.toml:20
quote: name = "viola-mcp-tap"
@row_add n=969 row=decision/test-binaries-never-released area=occupied-resources rec=fct-v2uqfwgj@1,fct-bzckmn3n@2
name: Test-only binaries kept out of release builds
rule: viola-fake-agent, viola-harness and viola-mcp-tap are test-only binaries and never ship in a release build; the fake agent is built only behind the fake-agent feature.
@row_check n=969 row=decision/test-binaries-never-released sha=f166be4b31bd v=holds judge=sonnet head=8834195b0683
at: Cargo.toml:19
quote: required-features = ["fake-agent"]
at: crates/viola-e2e/Cargo.toml:15
quote: name = "viola-harness"
at: scripts/release-check.sh:88
quote: json=$(cargo build --release --locked --bin viola --message-format=json) || rc=$?
@row_add n=970 row=name/command/run area=occupied-resources rec=fct-bogfaknz@1,fct-mnor3ks3@2
class: command
name: run
@row_check n=970 row=name/command/run sha=f5bee6f7f3ac v=holds judge=sonnet head=8834195b0683
at: src/cmd/mod.rs:62
quote: Run(run::RunArgs),
@row_add n=971 row=name/command/revive area=occupied-resources rec=fct-bogfaknz@1,fct-mnor3ks3@2
class: command
name: revive
@row_check n=971 row=name/command/revive sha=067e88eaa1e7 v=holds judge=sonnet head=8834195b0683
at: src/cmd/mod.rs:64
quote: Revive(revive::ReviveArgs),
@row_add n=972 row=name/command/send area=occupied-resources rec=fct-bogfaknz@1,fct-mnor3ks3@2
class: command
name: send
@row_check n=972 row=name/command/send sha=a2d95d26750d v=holds judge=sonnet head=8834195b0683
at: src/cmd/mod.rs:66
quote: Send(send::SendArgs),
@row_add n=973 row=name/command/wait area=occupied-resources rec=fct-bogfaknz@1,fct-mnor3ks3@2
class: command
name: wait
@row_check n=973 row=name/command/wait sha=d3638f1b4826 v=holds judge=sonnet head=8834195b0683
at: src/cmd/mod.rs:68
quote: Wait(wait::WaitArgs),
@row_add n=974 row=name/command/last area=occupied-resources rec=fct-bogfaknz@1,fct-mnor3ks3@2
class: command
name: last
@row_check n=974 row=name/command/last sha=f56cf3e5e58a v=holds judge=sonnet head=8834195b0683
at: src/cmd/mod.rs:70
quote: Last(last::LastArgs),
@row_add n=975 row=name/command/list area=occupied-resources rec=fct-bogfaknz@1,fct-mnor3ks3@2
class: command
name: list
@row_check n=975 row=name/command/list sha=c98e05048bed v=holds judge=sonnet head=8834195b0683
at: src/cmd/mod.rs:74
quote: List(list::ListArgs),
@row_add n=976 row=name/command/answer area=occupied-resources rec=fct-bogfaknz@1,fct-mnor3ks3@2
class: command
name: answer
@row_check n=976 row=name/command/answer sha=3fa23fa2465a v=holds judge=sonnet head=8834195b0683
at: src/cmd/mod.rs:72
quote: Answer(answer::AnswerArgs),
@row_add n=977 row=name/command/hook area=occupied-resources rec=fct-bogfaknz@1,fct-mnor3ks3@2
class: command
name: hook
@row_check n=977 row=name/command/hook sha=e613d3cb59fb v=holds judge=sonnet head=8834195b0683
at: src/cmd/mod.rs:93
quote: Hook(hook::HookArgs),
@row_add n=978 row=name/command/mcp area=occupied-resources rec=fct-bogfaknz@1,fct-mnor3ks3@2
class: command
name: mcp
@row_check n=978 row=name/command/mcp sha=d4027effb89e v=holds judge=sonnet head=8834195b0683
at: src/cmd/mod.rs:96
quote: Mcp,
@row_add n=979 row=name/command/verify area=occupied-resources rec=fct-bogfaknz@1,fct-mnor3ks3@2
class: command
name: verify
@row_check n=979 row=name/command/verify sha=2a022382631c v=holds judge=sonnet head=8834195b0683
at: src/cmd/mod.rs:88
quote: Verify(verify::VerifyArgs),
@row_add n=980 row=name/command/pause area=occupied-resources rec=fct-bogfaknz@1,fct-mnor3ks3@2
class: command
name: pause
@row_check n=980 row=name/command/pause sha=25dc804a6ef9 v=holds judge=sonnet head=8834195b0683
at: src/cmd/mod.rs:76
quote: Pause(pause::PauseArgs),
@row_add n=981 row=name/command/release area=occupied-resources rec=fct-bogfaknz@1,fct-mnor3ks3@2
class: command
name: release
@row_check n=981 row=name/command/release sha=e5bfafd9b038 v=holds judge=sonnet head=8834195b0683
at: src/cmd/mod.rs:78
quote: Release(release::ReleaseArgs),
@row_add n=982 row=name/command/link area=occupied-resources rec=fct-bogfaknz@1,fct-mnor3ks3@2
class: command
name: link
@row_check n=982 row=name/command/link sha=fdb835b87e21 v=holds judge=sonnet head=8834195b0683
at: src/cmd/mod.rs:80
quote: Link(link::LinkArgs),
@row_add n=983 row=name/command/unlink area=occupied-resources rec=fct-bogfaknz@1,fct-mnor3ks3@2
class: command
name: unlink
@row_check n=983 row=name/command/unlink sha=8a9f578a2c4a v=holds judge=sonnet head=8834195b0683
at: src/cmd/mod.rs:82
quote: Unlink(link::LinkArgs),
@row_add n=984 row=name/command/plugin-install area=occupied-resources rec=fct-bogfaknz@1,fct-mnor3ks3@2
class: command
name: plugin install
@row_check n=984 row=name/command/plugin-install sha=8783b7425762 v=holds judge=sonnet head=8834195b0683
at: src/cmd/mod.rs:90
quote: Plugin(plugin::PluginArgs),
at: src/cmd/plugin.rs:33
quote: Install(InstallArgs),
@row_add n=985 row=name/other/mcp-tool-send area=occupied-resources rec=fct-frmh2fi2@1
class: other
name: send
@row_check n=985 row=name/other/mcp-tool-send sha=67baa5a5b0d1 v=holds judge=sonnet head=8834195b0683
at: crates/viola-mcp/src/tools.rs:61
quote: "send" => Some(Self::Send),
@row_add n=986 row=name/other/mcp-tool-wait area=occupied-resources rec=fct-frmh2fi2@1
class: other
name: wait
@row_check n=986 row=name/other/mcp-tool-wait sha=0ef6d31e3dd5 v=holds judge=sonnet head=8834195b0683
at: crates/viola-mcp/src/tools.rs:62
quote: "wait" => Some(Self::Wait),
@row_add n=987 row=name/other/mcp-tool-last area=occupied-resources rec=fct-frmh2fi2@1
class: other
name: last
@row_check n=987 row=name/other/mcp-tool-last sha=a0b00973baf0 v=holds judge=sonnet head=8834195b0683
at: crates/viola-mcp/src/tools.rs:63
quote: "last" => Some(Self::Last),
@row_add n=988 row=name/other/mcp-tool-answer area=occupied-resources rec=fct-frmh2fi2@1
class: other
name: answer
@row_check n=988 row=name/other/mcp-tool-answer sha=3bc5ac04f640 v=holds judge=sonnet head=8834195b0683
at: crates/viola-mcp/src/tools.rs:64
quote: "answer" => Some(Self::Answer),
@row_add n=989 row=name/other/mcp-tool-list area=occupied-resources rec=fct-frmh2fi2@1
class: other
name: list
@row_check n=989 row=name/other/mcp-tool-list sha=07f2fb7697b0 v=holds judge=sonnet head=8834195b0683
at: crates/viola-mcp/src/tools.rs:65
quote: "list" => Some(Self::List),
@row_add n=990 row=name/crate/viola area=occupied-resources rec=fct-36275cza@1,fct-26g75mkn@1
class: crate
name: viola
@row_check n=990 row=name/crate/viola sha=5fad02b2a0ad v=holds judge=tool head=8834195b0683
at: Cargo.toml:2
quote: name = "viola"
@row_add n=991 row=name/crate/viola-core area=occupied-resources rec=fct-36275cza@1,fct-26g75mkn@1,fct-irhgihut@5
class: crate
name: viola-core
@row_check n=991 row=name/crate/viola-core sha=4fa0add7fb40 v=holds judge=tool head=8834195b0683
at: crates/viola-core/Cargo.toml:2
quote: name = "viola-core"
@row_add n=992 row=name/crate/viola-pty area=occupied-resources rec=fct-36275cza@1,fct-26g75mkn@1,fct-irhgihut@5
class: crate
name: viola-pty
@row_check n=992 row=name/crate/viola-pty sha=e83e263363a6 v=holds judge=tool head=8834195b0683
at: crates/viola-pty/Cargo.toml:2
quote: name = "viola-pty"
@row_add n=993 row=name/crate/viola-channel area=occupied-resources rec=fct-36275cza@1,fct-26g75mkn@1,fct-irhgihut@5
class: crate
name: viola-channel
@row_check n=993 row=name/crate/viola-channel sha=7bcba6a7e1da v=holds judge=tool head=8834195b0683
at: crates/viola-channel/Cargo.toml:2
quote: name = "viola-channel"
@row_add n=994 row=name/crate/viola-state area=occupied-resources rec=fct-36275cza@1,fct-26g75mkn@1,fct-irhgihut@5
class: crate
name: viola-state
@row_check n=994 row=name/crate/viola-state sha=c43865c4c31e v=holds judge=tool head=8834195b0683
at: crates/viola-state/Cargo.toml:2
quote: name = "viola-state"
@row_add n=995 row=name/crate/viola-agent-claude area=occupied-resources rec=fct-36275cza@1,fct-26g75mkn@1,fct-irhgihut@5
class: crate
name: viola-agent-claude
@row_check n=995 row=name/crate/viola-agent-claude sha=2cabe74ceb02 v=holds judge=tool head=8834195b0683
at: crates/viola-agent-claude/Cargo.toml:2
quote: name = "viola-agent-claude"
@row_add n=996 row=name/crate/viola-mcp area=occupied-resources rec=fct-36275cza@1,fct-26g75mkn@1,fct-irhgihut@5
class: crate
name: viola-mcp
@row_check n=996 row=name/crate/viola-mcp sha=218c603968fa v=holds judge=tool head=8834195b0683
at: crates/viola-mcp/Cargo.toml:2
quote: name = "viola-mcp"
@row_add n=997 row=name/crate/viola-e2e area=occupied-resources rec=fct-j2leq5fi@1,fct-26g75mkn@1,fct-irhgihut@5
class: crate
name: viola-e2e
@row_check n=997 row=name/crate/viola-e2e sha=a09501129c14 v=holds judge=tool head=8834195b0683
at: crates/viola-e2e/Cargo.toml:2
quote: name = "viola-e2e"
@row_add n=998 row=contract/child-environment/viola-dir area=occupied-resources rec=fct-esr7jyhf@1,fct-7uan73fu@1
boundary: The wrapped child's environment
item: VIOLA_DIR
promise: Set by viola run in the child to the absolute path of the instance's state directory; hook, mcp and every CLI verb run inside the session derive the viola home as its grandparent, so a session started with --home keeps its hooks, MCP server and nested CLI calls in that home.
@row_check n=998 row=contract/child-environment/viola-dir sha=ba35934d8a51 v=holds judge=sonnet head=8834195b0683
at: src/run/mod.rs:124
quote: ("VIOLA_DIR".into(), instance_dir.as_os_str().to_owned()),
at: src/cmd/mod.rs:262-264
quote: let instances = dir .parent() .filter(|_| dir.file_name() == Some(OsStr::new(own)))?;
@row_add n=999 row=decision/no-live-claude-in-ci area=occupied-resources rec=fct-mkgpzodl@1,fct-5lqu6sil@2
name: No live claude in CI
rule: When CI is present, the test harness's run --local-live refuses before any build, spawn or suite, and viola itself never reads the harness variables.
why: so the real claude never runs in CI
@row_check n=999 row=decision/no-live-claude-in-ci sha=e374f0c81d19 v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/run.rs:131-132
quote: if sel.local_live && ci { let doc = json!({"v": 1, "cmd": "run", "ok": false, "reason": "live-in-ci"});
at: crates/viola-e2e/src/harness/run.rs:101
quote: let ci = std::env::var_os("CI").is_some();
@row_add n=1000 row=contract/config-json/v area=occupied-resources rec=fct-i4wacyrp@2,fct-6e4hf2ai@1,fct-k6htnrfj@3,fct-3jytrhfq@2
boundary: config.json
item: v
promise: config.json carries v, which must be 1; the file is read with a MAX_FRAME cap.
@row_check n=1000 row=contract/config-json/v sha=154061b4727e v=holds judge=sonnet head=8834195b0683
at: src/obs.rs:198
quote: if map.get("v").and_then(Value::as_u64) != Some(1) {
at: src/obs.rs:187
quote: Ok(file) => match file.take(MAX_FRAME).read_to_end(&mut bytes) {
@row_add n=1001 row=contract/config-json/five-hour area=occupied-resources rec=fct-i4wacyrp@2,fct-6e4hf2ai@1,fct-k6htnrfj@3,fct-3jytrhfq@2
boundary: config.json
item: five_hour
promise: The five-hour budget threshold, a JSON whole number from 0 to 100, default 90; any other value takes the default with one parse-rejected config-json line, the other key standing, and no value switches the gate off.
@row_check n=1001 row=contract/config-json/five-hour sha=1f420d137fd6 v=holds judge=sonnet head=8834195b0683
at: src/run/budget.rs:76-82
quote: let mut threshold = |key: &str, default: u8| match map.get(key).map(whole_percent) { None => compiled(default), Some(Some(percent)) => percent, Some(None) => { rejections.push(ConfigRejection::Malformed); compiled(default)
at: src/run/budget.rs:85
quote: five_hour: threshold(FIVE_HOUR_KEY, FIVE_HOUR_DEFAULT),
at: src/run/budget.rs:30
quote: const FIVE_HOUR_DEFAULT: u8 = 90;
@row_add n=1002 row=contract/config-json/seven-day area=occupied-resources rec=fct-i4wacyrp@2,fct-6e4hf2ai@1,fct-k6htnrfj@3,fct-3jytrhfq@2
boundary: config.json
item: seven_day
promise: The seven-day budget threshold, a JSON whole number from 0 to 100, default 85; any other value takes the default with one parse-rejected config-json line, the other key standing, and no value switches the gate off.
@row_check n=1002 row=contract/config-json/seven-day sha=9df3fa4676c8 v=holds judge=sonnet head=8834195b0683
at: src/run/budget.rs:86
quote: seven_day: threshold(SEVEN_DAY_KEY, SEVEN_DAY_DEFAULT),
at: src/run/budget.rs:31
quote: const SEVEN_DAY_DEFAULT: u8 = 85;
at: src/run/budget.rs:80-81
quote: rejections.push(ConfigRejection::Malformed); compiled(default)
@row_add n=1003 row=contract/config-json/diagnostics-level area=occupied-resources rec=fct-i4wacyrp@2,fct-6e4hf2ai@1,fct-k6htnrfj@3,fct-3jytrhfq@2
boundary: config.json
item: diagnostics_level
promise: "info" by default, or "debug"; diagnostics_level is the only source of the process-log level, and RUST_LOG has no effect.
@row_check n=1003 row=contract/config-json/diagnostics-level sha=61512ca52f6e v=holds judge=sonnet head=8834195b0683
at: src/obs.rs:201-204
quote: let (level, bad_value) = match map.get("diagnostics_level").map(Value::as_str) { None | Some(Some("info")) => (Level::INFO, false), Some(Some("debug")) => (Level::DEBUG, false), Some(_) => (Level::INFO, true),
searched: RUST_LOG · in src/**/*.rs and crates/**/*.rs (only a comment at src/obs.rs:180 saying never RUST_LOG)
searched: EnvFilter · in src/**/*.rs and crates/**/*.rs
@row_add n=1004 row=contract/config-json/claude-env-keep area=occupied-resources rec=fct-i4wacyrp@2,fct-6e4hf2ai@1,fct-k6htnrfj@3,fct-3jytrhfq@2
boundary: config.json
item: claude_env_keep
promise: The R8 pass-list of CLAUDE* names, an array of at most 32 names each matching ^CLAUDE[A-Z0-9_]*$, default empty; a bad value is rejected whole, and an identity-floor name in it has no effect.
@row_check n=1004 row=contract/config-json/claude-env-keep sha=f5b8d22f7df8 v=holds judge=sonnet head=8834195b0683
at: src/run/env.rs:44
quote: (items.len() <= KEEP_MAX)
at: src/run/env.rs:48
quote: .map(|v| v.as_str().filter(|s| is_claude_name(s)).map(str::to_owned))
at: src/run/env.rs:55
quote: None => (Vec::new(), Some(ConfigRejection::Malformed)),
at: crates/viola-agent-claude/src/lib.rs:73
quote: if !floor && persistent.contains(&text) {
@row_add n=1005 row=contract/config-json/dialog-hold-s area=occupied-resources rec=fct-i4wacyrp@2,fct-6e4hf2ai@1,fct-k6htnrfj@3,fct-3jytrhfq@2
boundary: config.json
item: dialog_hold_s
promise: The dialog hold, a JSON whole number of seconds from 60 to 3600, default 60; any other value takes 60 with one parse-rejected config-json line, so no value makes the hold zero or unbounded.
@row_check n=1005 row=contract/config-json/dialog-hold-s sha=9cb32d630075 v=holds judge=sonnet head=8834195b0683
at: src/run/dialog.rs:180
quote: Some(None) => (DialogHold::default(), vec![ConfigRejection::Malformed]),
at: src/run/dialog.rs:186
quote: DialogHold::try_new(whole).ok()
at: crates/viola-core/src/lib.rs:19
quote: pub const DIALOG_HOLD_DEFAULT: Duration = Duration::from_secs(60);
@row_add n=1006 row=decision/recorded-fixtures-scrubbed area=occupied-resources rec=fct-kyb4p5xt@1,fct-3fjm2cf6@2
name: Recorded fixtures scrubbed and hygiene-walked
rule: Fixtures recorded by viola verify --record are written only when no row fails, each is scrubbed and hygiene-walked, and an unclean recording is refused whole, naming the file and the check code, never the content.
guard: contract_fixture_hygiene
@row_check n=1006 row=decision/recorded-fixtures-scrubbed sha=9ec27e8a42a3 v=holds guard=true judge=sonnet head=8834195b0683
at: src/cmd/verify.rs:297-301
quote: if failed > 0 { return Ok(Ended::Exit(1)); } match &args.record { Some(dir) => record(dir, &version, &probes),
at: src/cmd/verify.rs:520
quote: Err(named) => return Ok(refuse_recording(&named)),
at: src/cmd/verify.rs:626
quote: &format!("a recorded fixture is not clean: {named}"),
guard_at: tests/contract_fixture_hygiene.rs:21
@row_add n=1007 row=decision/config-precedence area=cross-cutting-patterns rec=fct-dkudywpi@1,fct-k6htnrfj@3
name: Configuration precedence
rule: CLI flags override the viola home's config.json, which overrides built-in defaults; config.json is parsed tolerantly and carries v.
@row_check n=1007 row=decision/config-precedence sha=0fa70e462c63 v=holds judge=sonnet head=8834195b0683
at: src/obs.rs:198
quote: if map.get("v").and_then(Value::as_u64) != Some(1) {
at: src/obs.rs:206-209
quote: let unknown = map .keys() .filter(|k| !CONFIG_KEYS.contains(&k.as_str())) .count();
@row_add n=1008 row=decision/viola-home-resolution area=cross-cutting-patterns rec=fct-dkudywpi@1,fct-k6htnrfj@3
name: Viola home resolution order
rule: The viola home resolves from --home, then the grandparent of VIOLA_DIR, then .viola under the user home; viola run passes the home to its child only through VIOLA_DIR, so every per-home file follows it.
why: The home is the isolation point for tests: each test runs in its own home.
@row_check n=1008 row=decision/viola-home-resolution sha=cab509da8ec5 v=holds judge=sonnet head=8834195b0683
at: src/cmd/mod.rs:294
quote: if let Some(home) = flag {
at: src/cmd/mod.rs:297
quote: if let Some(instance) = instance_of(name, dir) {
at: src/cmd/mod.rs:301
quote: .map(|user_home| default_home(&user_home))
at: src/run/mod.rs:124
quote: ("VIOLA_DIR".into(), instance_dir.as_os_str().to_owned()),
@row_add n=1009 row=absence/secrets area=cross-cutting-patterns rec=fct-dkudywpi@1,fct-k6htnrfj@3
capability: Secrets of any kind in v1 configuration
@row_check n=1009 row=absence/secrets sha=229c53965dd5 v=holds judge=sonnet head=8834195b0683
searched: secret|password|api_key|credential (case-insensitive) · in src/**/*.rs and crates/*/src/**/*.rs (only test strings and a doc comment pointing at the security plan)
searched: CONFIG_KEYS · in src/obs.rs (config keys hold no secret field)
@row_add n=1010 row=decision/capability-ledger-gate area=cross-cutting-patterns rec=fct-unnwkesq@1,fct-jkmwg2b6@1,fct-jo2qqab5@1,fct-wfge3rsz@3
name: Capability ledger as the single gate for CLI behaviour
rule: Any new dependence on an undocumented claude behaviour is added as a capability-ledger row in viola-agent-claude with a viola verify probe that has a post-condition check, never hard-coded as an unconditional assumption.
except: A CLI shape no send relies on needs no probe, such as the long-paste frame beside typed text and between two pairs.
why: A ruled limit sets the exception.
@row_check n=1010 row=decision/capability-ledger-gate sha=e7a32135c507 v=unverifiable judge=sonnet head=8834195b0683
why: not a code claim
@row_add n=1011 row=decision/bounded-readers area=cross-cutting-patterns rec=fct-tpj3dxid@1,fct-qip7sr3o@1
name: Bounded readers at every boundary
rule: Every product reader of external bytes is bounded in its own statement or named as an exception; none reads an external byte source to its end unbounded.
except: The own-executable reads, which are no external input.
guard: contract_bounded_readers: product_readers_are_each_bounded_or_named
@row_check n=1011 row=decision/bounded-readers sha=56d9f559b23e v=holds guard=true judge=sonnet head=8834195b0683
at: tests/contract_bounded_readers.rs:394
quote: const EXCEPTIONS: &[(&str, &str, &str)] = &[
at: tests/contract_bounded_readers.rs:711
quote: "reader calls with no take( in their statement and no exception: {unbounded:#?}"
guard_at: tests/contract_bounded_readers.rs:690
@row_add n=1012 row=contract/channel-link/already-linked area=standard-contracts rec=fct-rma7vozl@1,fct-anqliuvh@1
boundary: channel link method
item: already_linked
promise: A link of a pair the set holds appends nothing and answers that pair's since with already_linked true; a new pair answers its new link line's ts as since with already_linked false.
@row_check n=1012 row=contract/channel-link/already-linked sha=5d400c6c340b v=holds judge=sonnet head=8834195b0683
at: src/run/links.rs:62-63
quote: if let Some(since) = self.since(&set, driver) { return Ok((since, true));
at: src/run/links.rs:65-67
quote: let line = self.line(EventKind::Link, driver); self.change(&mut set, &line)?; Ok((line.ts, false))
at: src/run/links.rs:75
quote: Ok(json!({"ok": {"since": since, "already_linked": already_linked}}))
@row_add n=1013 row=decision/verify-child-set area=established-decisions rec=fct-qwpklckh@2
name: Verify's bounded child set
rule: viola verify makes one print-mode probe, five interactive PTY runs under the R8 strip at a fixed 80×24, and one agents --json read inside Run B, pasting only compiled texts.
why: Each widening of the child set was ratified.
@row_check n=1013 row=decision/verify-child-set sha=27c91dcede65 v=holds judge=sonnet head=8834195b0683
at: src/cmd/verify.rs:205
quote: .extend(["-p", PROBE_PROMPT, "--model", "haiku", PLUGIN_DIR_FLAG].map(OsString::from));
at: src/cmd/verify/typed.rs:74
quote: let runs: [&RunStep<'_>; 5] = [
at: crates/viola-pty/src/lib.rs:30
quote: pub const DEFAULT: Self = Self { cols: 80, rows: 24 };
at: src/cmd/verify/typed/run.rs:93
quote: screen: Screen::new(Size::DEFAULT.rows, Size::DEFAULT.cols, spawned_at),
at: src/cmd/verify/typed/runs.rs:76
quote: .then(|| agents_read(&run, inputs, &trusted.0))
@row_add n=1014 row=contract/viola-verify-exit-codes/1-relied-row-failed area=established-decisions rec=fct-qwpklckh@2
boundary: viola verify exit codes
item: 1
promise: A relied ledger row failed; the failing row's stamp is still written.
@row_check n=1014 row=contract/viola-verify-exit-codes/1-relied-row-failed sha=d5a4430c1cc6 v=holds judge=sonnet head=8834195b0683
at: src/cmd/verify.rs:273
quote: let written = update_stamps(home, |existing| {
at: src/cmd/verify.rs:297-298
quote: if failed > 0 { return Ok(Ended::Exit(1));
@row_add n=1015 row=decision/permission-request-question-stays-null area=established-decisions rec=fct-qwpklckh@2
name: A question first raised by PermissionRequest stays null
rule: A question first raised by PermissionRequest stays null to the human.
why: A PermissionRequest allow with updatedInput would satisfy it, but that body has no ledger row; adding one needs a 25th row, its viola verify probe and a re-stamp of every stamped version.
@row_check n=1015 row=decision/permission-request-question-stays-null sha=8f52c9dd9731 v=holds judge=sonnet head=8834195b0683
at: crates/viola-agent-claude/src/dialog.rs:389
quote: _ => return None,
@row_add n=1016 row=contract/viola-revive-json/error-document area=established-decisions rec=fct-onw4pywk@3,fct-mnor3ks3@2
boundary: viola revive --json
item: error document
promise: Every refusal of the start, a refused strict-modes check and a preflight refusal among them, prints {"v":1,"error":"<code>","detail":null}, the code one of the ten codes, in place of its unable/hint pair, with stderr empty and exit 1; a pump_child error prints no document.
@row_check n=1016 row=contract/viola-revive-json/error-document sha=f67838210369 v=holds judge=sonnet head=8834195b0683
at: src/cmd/client.rs:227
quote: document(&json!({"v": 1, "error": code, "detail": null}));
at: src/cmd/revive.rs:107
quote: client::refused_document(self.detail());
at: src/cmd/run.rs:413
quote: Started::Refused(ExitCode::from(1))
@row_add n=1017 row=decision/fake-agent-pump-delay-seam area=occupied-resources rec=fct-2hatag6l@1
name: FAKE_AGENT_PUMP_DELAY_MS test seam
rule: FAKE_AGENT_PUMP_DELAY_MS is read only by a viola built with the test-only fake-agent feature, parsed as milliseconds capped at 5 000, and holds the pump back after the child starts; it disables no control and widens no redaction.
why: Ratified as a security-plan Security Anti-Patterns carve-out: a test seam, not a configuration channel.
@row_check n=1017 row=decision/fake-agent-pump-delay-seam sha=e3c6da81f4d4 v=holds judge=sonnet head=8834195b0683
at: src/cmd/run.rs:61-63
quote: #[cfg(feature = "fake-agent")] fn hold_pump_start() { let ms = std::env::var("FAKE_AGENT_PUMP_DELAY_MS")
at: src/cmd/run.rs:67
quote: std::thread::sleep(std::time::Duration::from_millis(ms.min(5_000)));
@row_add n=1018 row=decision/end-to-end-homes-kept-after-each-job area=occupied-resources rec=fct-x2qzlbqt@1,fct-xib5fpty@2
name: Homes kept after each job's end-to-end run
rule: Each CI job keeps its harness and test homes after its end-to-end run, whatever the outcome (AGENT_RUN_KEEP_HOMES "1" on the job, if: always()), the viola-chaos-* home out.
why: The homes are kept in CI for the obs gates.
@row_check n=1018 row=decision/end-to-end-homes-kept-after-each-job sha=7d6d70ad028d v=holds judge=sonnet head=8834195b0683
at: .github/workflows/ci.yml:20
quote: AGENT_RUN_KEEP_HOMES: "1"
at: .github/workflows/ci.yml:57
quote: if: always()
at: tests/support/home.rs:216
quote: .prefix("viola-chaos-")
@row_add n=1019 row=decision/mcp-server-rmcp area=stack-and-technologies rec=fct-qeiflw5v@1,fct-irhgihut@5
name: rmcp is the MCP server
rule: The MCP server is an rmcp stdio server with schemars tool input schemas, rmcp minor pinned, a thin adapter over viola-channel.
@row_check n=1019 row=decision/mcp-server-rmcp sha=c251bc6847f3 v=holds judge=sonnet head=8834195b0683
at: Cargo.toml:223
quote: rmcp = { version = ">=3.4.1, <3.5", default-features = false, features = ["server", "transport-io"] }
at: Cargo.toml:226
quote: schemars = "=1.2.2"
at: crates/viola-mcp/Cargo.toml:16
quote: viola-channel = { path = "../viola-channel", features = ["tokio"] }
@row_add n=1020 row=decision/plugin-embedded-in-binary area=stack-and-technologies rec=fct-5oigicbx@1,fct-irhgihut@5
name: Claude Code plugin compiled into the binary
rule: The Claude Code plugin is compiled into the binary (include_str!) and written out by viola run.
@row_check n=1020 row=decision/plugin-embedded-in-binary sha=a8c626fdb1dd v=holds judge=sonnet head=8834195b0683
at: crates/viola-agent-claude/src/lib.rs:126
quote: include_str!(concat!(
at: src/cmd/run.rs:475
quote: for (rel, content) in viola_agent_claude::plugin_files(&pinned.path_fwd) {
@row_add n=1021 row=decision/malformed-stamps-unverified area=standard-contracts rec=fct-62qgonvf@2
name: Malformed or untrusted stamps read unverified
rule: A non-`v:1` or wrong-shaped stamps envelope, or a strict-modes refusal, makes run's gate record cli_verified:false with one parse-rejected line.
@row_check n=1021 row=decision/malformed-stamps-unverified sha=f3a82b34af81 v=holds judge=sonnet head=8834195b0683
at: src/run/version_gate.rs:204
quote: Err(StampsReadError::StrictModes(_)) => (false, Some("strict-modes-failed")),
at: src/run/version_gate.rs:208
quote: Err(_) => (false, Some("malformed")),
at: src/run/version_gate.rs:177
quote: parser = "ledger-stamps",
@row_add n=1022 row=contract/mcp-list/refusal area=standard-contracts rec=fct-smoe4pw5@1,fct-yj2flshm@2
boundary: MCP list
item: refusal
promise: The board's refusal, its only one, is returned as instance-unreachable with detail strict-modes-failed.
@row_check n=1022 row=contract/mcp-list/refusal sha=e2d9fb6ddf87 v=holds judge=sonnet head=8834195b0683
at: crates/viola-mcp/src/result.rs:92
quote: Some("instance-unreachable") => Ok(unreachable(UnreachableCause::StrictModes)),
at: crates/viola-mcp/src/result.rs:93
quote: _ => Err(McpError::Internal),
@row_add n=1023 row=decision/dual-licence area=build-system rec=fct-duju5u6d@1,fct-irhgihut@5
name: MIT OR Apache-2.0 licence
rule: The project is licensed MIT OR Apache-2.0, set once for the workspace and inherited by every member, with both licence texts at the root and the README naming both.
@row_check n=1023 row=decision/dual-licence sha=4afe6d48e0f8 v=holds judge=sonnet head=8834195b0683
at: Cargo.toml:189
quote: license = "MIT OR Apache-2.0"
at: Cargo.toml:6
quote: license.workspace = true
at: README.md:8-9
quote: Licensed under either of the Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE)) or the MIT license ([LICENSE-MIT](LICENSE-MIT)), at your option.
at: LICENSE-MIT:1
quote: MIT License
@row_none n=1 rec=fct-bq65okf4@3 type=9
@row_none n=2 rec=fct-jqic6r22@2 type=9
@row_none n=3 rec=fct-gevx5agt@1 type=S
@row_none n=4 rec=fct-cdd57vln@1 type=S
@row_none n=5 rec=fct-oizocdnn@1 type=S
@row_none n=6 rec=fct-lh3i55lk@1 type=S
@row_none n=7 rec=fct-y6frgi47@1 type=4
relied: A replayed snapshot carries no endpoint, pid, start time or statusline command, so its readers treat the instance as unreachable and the collision check judges liveness from the heartbeat alone.
@row_none n=8 rec=fct-2fgobtbl@1 type=S
@row_none n=9 rec=fct-fxxhpbnl@1 type=S
@row_none n=10 rec=fct-tp7yy7x7@1 type=S
@row_none n=11 rec=fct-4khdjfgb@1 type=S
@row_none n=12 rec=fct-mxi5smga@1 type=S
@row_none n=13 rec=fct-qslc37ld@1 type=S
@row_none n=14 rec=fct-bsiojb2t@1 type=S
@row_none n=15 rec=fct-h57b2ldw@1 type=5
@row_none n=16 rec=fct-7clomlnw@1 type=S
@row_none n=17 rec=fct-a232k7eb@1 type=S
@row_none n=18 rec=fct-rtcscax5@1 type=S
@row_none n=19 rec=fct-t7ncsx4g@1 type=5
@row_none n=20 rec=fct-xcjb7ry2@1 type=4
relied: The plugin marketplace tree is rewritten whole on every plugin install, any other entry is left as found, and viola never reads the tree back.
@row_none n=21 rec=fct-75zp6bfs@1 type=5
@row_none n=22 rec=fct-cny66nid@1 type=5
@row_none n=23 rec=fct-th4sqjku@1 type=S
@row_none n=24 rec=fct-cqhnweqz@1 type=5
@row_none n=25 rec=fct-rhfeygrr@1 type=5
@row_none n=26 rec=fct-2nq4afki@1 type=5
@row_none n=27 rec=fct-42ufdodu@1 type=5
@row_none n=28 rec=fct-5uugfnul@1 type=5
@row_none n=29 rec=fct-uutjz2rg@1 type=5
@row_none n=30 rec=fct-xscn73uj@1 type=5
@row_none n=31 rec=fct-n6dovmau@1 type=5
@row_none n=32 rec=fct-44on672u@1 type=5
@row_none n=33 rec=fct-5q3bjmyd@2 type=5
@row_none n=34 rec=fct-mcr4cbe6@1 type=5
@row_none n=35 rec=fct-4u2vf5lk@2 type=5
@row_none n=36 rec=fct-nu56jv33@1 type=5
@row_none n=37 rec=fct-gwc5vubx@1 type=6
@row_none n=38 rec=fct-eesxehzs@2 type=5
@row_none n=39 rec=fct-fslyjpzg@1 type=5
@row_none n=40 rec=fct-37dicsq3@1 type=5
@row_none n=41 rec=fct-5gglmp64@1 type=5
@row_none n=42 rec=fct-ziwerfk3@1 type=5
@row_none n=43 rec=fct-io6ijhtt@1 type=5
@row_none n=44 rec=fct-lftr3wgo@1 type=5
@row_none n=45 rec=fct-xzimrtmj@1 type=5
@row_none n=46 rec=fct-7wuxnmfc@2 type=5
@row_none n=47 rec=fct-zuwbep4y@1 type=5
@row_none n=48 rec=fct-o2wbgrws@2 type=S
@row_none n=49 rec=fct-kmtpkt4i@1 type=0
what: a statement that the repository is a single product with no template patterns
@row_none n=50 rec=fct-pcwm7hb5@1 type=0
what: a note that the project had no existing scopes
@row_none n=51 rec=fct-q7tym3q2@1 type=4
relied: run, revive's start, mcp and hook stay silent at the internal-error catch site.
@row_none n=52 rec=fct-dhyimyqh@1 type=4
relied: A failed connect reads as an instance-dead cause and a closed or broken stream as an endpoint-lost cause, which the exit 21 detail words rest on.
@row_none n=53 rec=fct-k7elqwqc@2 type=4
@row_none n=54 rec=fct-kbitf2mh@2 type=4
relied: The socket-directory check passes an absent path, refuses a symlink, a non-directory or an unreadable path, and never chmods, so a refused directory stays as found.
@row_none n=55 rec=fct-kan7g7fj@1 type=4
relied: Every MCP frame tool refuses a home or target instance directory failing the strict-mode check with the `instance-unreachable` result and detail `strict-modes-failed`, sending no frame.
@row_none n=56 rec=fct-au3pdba5@1 type=4
relied: Every unreachable cause a frame tool meets surfaces as the `instance-unreachable` result with no frame and no line.
@row_none n=57 rec=fct-l7zodwoq@1 type=8
@row_none n=58 rec=fct-vwfyd52k@1 type=4
relied: A dialog hook whose server check fails sends no `hook.dialog` and exits 0 at once with no body, so the dialog renders for the human.
@row_none n=59 rec=fct-4qhayrrs@1 type=4
relied: A hook whose instance check refuses or finds no instance exits 0 before reading config, stdin or writing any line.
@row_none n=60 rec=fct-vxml3b56@3 type=9
@row_none n=61 rec=fct-b46ywspz@1 type=4
@row_none n=62 rec=fct-y6kczlfs@1 type=9
@row_none n=63 rec=fct-u3cn6cai@1 type=9
@row_none n=64 rec=fct-vag5jbae@1 type=4
relied: A strict-modes refusal of the home tree or of the stamps makes viola verify exit 1 with the home pair, reading and writing nothing.
@row_none n=65 rec=fct-2iyz7ner@1 type=4
relied: A frame verb against a stale instance reads instance-dead, since no live endpoint is given for a stale wrapper.
@row_none n=66 rec=fct-e4yo5lcc@1 type=4
relied: Without a live endpoint a client reads an existing instance directory as instance-dead and anything else as an unwrapped name.
@row_none n=67 rec=fct-zgxijmsq@1 type=4
relied: Over a replayed snapshot the start's collision check decides from the heartbeat alone, refusing already-live within the stale bound.
@row_none n=68 rec=fct-5kgumnrt@1 type=4
relied: A revive makes one snapshot replay and logs one state-recovered line.
@row_none n=69 rec=fct-wwpgfv5r@1 type=4
relied: Every start refuses a strict-modes home-tree failure with exit 1 before any log, line or child.
@row_none n=70 rec=fct-mefdhndu@1 type=4
relied: A failed socket-dir create and a refused check both read socket-dir-refused with exit 1, and a bind failure does not.
@row_none n=71 rec=fct-5gkabtaw@2 type=4
relied: Program resolution has one refusal, ProgramNotFound, with a fixed message.
@row_none n=72 rec=fct-gctodeyz@2 type=5
@row_none n=73 rec=fct-5ijnay4v@1 type=4
relied: A start pins the very bytes it read once and keyed for its home-tree check.
@row_none n=74 rec=fct-wtlwpexy@2 type=0
what: a measured note that one source tree can build two different binaries, recorded as known with no action
@row_none n=75 rec=fct-nbf2wdvs@2 type=4
relied: verify's Run E resumes the session id taken from Run B's first SessionStart, so Run E depends on Run B.
@row_none n=76 rec=fct-2oafusjr@1 type=4
relied: A hook whose VIOLA_DIR names an instance folder that does not exist exits 0 and writes nothing.
@row_none n=77 rec=fct-wdtionxy@2 type=4
relied: A refused diagnostics folder or file silences the log rather than failing the process, and a detail line is dropped.
@row_none n=78 rec=fct-vs5zumck@2 type=4
relied: run's and revive's starts and verify check the home tree, refusing at home or instance level, before using it.
@row_none n=79 rec=fct-sbmskf35@2 type=4
relied: A path under a regular file is a strict-modes refusal: verify exits 1 with the home pair, the frame verbs exit 21 with the instance pair.
@row_none n=80 rec=fct-flit34us@1 type=4
relied: Harness cleanup reads ok after a force-killed wrapper, since a refused or missing socket reads gone and no leftover file is removed.
@row_none n=81 rec=fct-w6vpbrjt@2 type=4
relied: Root tests freeze a wrapper with SIGSTOP through one shared guard so that it reads stale, resuming it on drop.
@row_none n=82 rec=fct-bh2gc2qq@1 type=5
@row_none n=83 rec=fct-5v4ir6ih@2 type=5
@row_none n=84 rec=fct-vp2lxvsd@2 type=4
relied: Both channel clients read the serving process's pid and euid from the connected socket's peer credentials, the identity each frame-writing client checks before its first frame.
@row_none n=85 rec=fct-e6lqqdby@1 type=4
relied: The serving process's start time read through sysinfo is the observed side of the server check each frame-writing client makes before its first frame.
@row_none n=86 rec=fct-ekks4dfz@1 type=4
relied: The stamps writer runs the strict-modes check before creating the ledger directory and again under its lock, and a refusal reads and writes nothing.
@row_none n=87 rec=fct-fr2pme2q@1 type=4
relied: The link set changes only on an implicit link and on a served link or unlink call.
@row_none n=88 rec=fct-ng4l52uh@1 type=4
relied: Every exit-21 CLI document is written by one writer.
@row_none n=89 rec=fct-a6y4blve@2 type=4
relied: A frame-writing client runs the strict-modes check, the liveness pre-check, the socket-directory check, the peer euid decision and then the server identity check, in that order.
@row_none n=90 rec=fct-4yfg5mra@1 type=4
relied: A frame verb against a stale or dead instance reads instance-dead, while a name with no instance directory reads as unwrapped.
@row_none n=91 rec=fct-ikwybol4@2 type=4
relied: A peer whose euid cannot be read is refused.
@row_none n=92 rec=fct-ntix5ygh@1 type=S
@row_none n=93 rec=fct-nhzfyd7i@1 type=6
@row_none n=94 rec=fct-2nyzhkqj@2 type=5
@row_none n=95 rec=fct-v4imvdl4@1 type=S
@row_none n=96 rec=fct-sgev6rlz@1 type=S
@row_none n=97 rec=fct-a4ctqcf6@2 type=5
@row_none n=98 rec=fct-x3nhuj7r@1 type=5
@row_none n=99 rec=fct-panweniz@1 type=S
@row_none n=100 rec=fct-sxdyevwi@1 type=S
@row_none n=101 rec=fct-uzytp74j@1 type=S
@row_none n=102 rec=fct-5iwrwosa@1 type=S
