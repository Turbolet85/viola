@carried n=1 label=architecture/standard-contracts fact=fct-dt7d6rx2
why: the one-schema rule requires every exit-21 `--json` document to carry these same six closed causes, `v` and `error` on Linux, macOS and Windows, never an OS-only word.
quote: The exit-21 document's `detail` is one of six closed causes: `strict-modes-failed`, `instance-dead`, `unwrapped-name`, `endpoint-lost`, `server-verify-failed` and `socket-dir-refused`; its `v` is 1 and its `error` `instance-unreachable` on all six.
@carried n=1 run=2 label=architecture/standard-contracts fact=fct-ng4l52uh
quote: `client::unreachable_document` is the one writer of the exit-21 CLI document
@carried n=1 run=3 label=architecture fact=fct-ksmkl4jd
quote: `{"v":1,"error":"instance-unreachable","detail":"<cause>"}` (exit 21), `<cause>` one of `strict-modes-failed` · `instance-dead` · `unwrapped-name` · `endpoint-lost` · `server-verify-failed` · `socket-dir-refused` (`viola_core::UnreachableCause`), written by `client::unreachable_document`
@carried n=2 label=architecture/standard-contracts fact=fct-qybkldvd
why: the frame verbs' refused socket directory is reached only on the Unix socket path, and its document must read the same closed word and exit code the Windows build would print.
quote: A refused socket directory at any of the six, met in `connect` after that check, prints the same document with `detail` `socket-dir-refused` (exit 21), with one `process-exit` at WARN and no `channel-request`.
@carried n=3 label=architecture/standard-contracts fact=fct-ubupynfq
why: `viola link` / `viola unlink` reach the same Unix socket-directory refusal, whose document must keep the one schema.
quote: A refused socket directory at either verb, met in `connect` after that check, prints the same document with `detail` `socket-dir-refused` (exit 21), with one `process-exit` at WARN.
@carried n=4 label=architecture/standard-contracts fact=fct-zxepdepr
why: the openpty `run` / `revive` start must print the same refusal documents as the ConPTY start, each returning before the spawn.
quote: With it true, `refused_with` (`src/cmd/run.rs`) prints `refused_document` in place of the human pair for `batch-script-child`, `pinned-hash-mismatch`, `squatted-name` and `socket-dir-refused`, and a program not found, which has no human pair, prints `internal-error`; each returns before `spawn_child`.
@carried n=5 label=architecture/standard-contracts fact=fct-cz7o3ttv
why: `viola revive --json`'s refusal codes are among the closed words the one-schema rule names, and revive runs in the Unix kill-and-revive case.
quote: `viola revive --json` prints one exit-1 document for every refusal of its start.
@carried n=5 run=2 label=architecture fact=fct-54jxzo44
quote: `{"v":1,"error":"<code>","detail":null}` (exit 1): `viola revive --json` refusing its strict-modes check, its preflight or the start it shares with `run`, `<code>` one of `strict-modes-failed` · `already-live` · `no-session` · `session-live` · `cwd-missing` · `stale-heartbeat` · `batch-script-child` · `pinned-hash-mismatch` · `squatted-name` · `socket-dir-refused` · `internal-error`, written by `client::refused_document` (codes only: no path, pid or id), stderr empty.
@carried n=6 label=architecture/standard-contracts fact=fct-nxxqzjun
why: the wrapper-fault `--json` document's `detail` must be the same closed object on every OS.
quote: `viola_channel::sanitised_fault(&Value) -> Option<Value>` is the one fault reader, of `reply_of` and `of_reply`: `None` without an integer `code`; for each of the five codes `message` is the fault's fixed text;
@carried n=6 run=2 label=architecture/standard-contracts fact=fct-pavvapld
quote: A fault code outside the five reads `{"code": <code>, "message": "wrapper fault", "data": null}`: the literal `wrapper fault` is a fixed output word on `--json` and MCP
@carried n=6 run=3 label=architecture fact=fct-cuelorpk
quote: `{"v":1,"error":"wrapper-fault","detail":{"code":<int>,"message":"…","data":<object|null>}}` (exit 20): `message` a fixed text and `data` a closed object or null, read through `viola_channel::sanitised_fault`
@carried n=7 label=architecture/standard-contracts fact=fct-2tmeycxu
why: the Unix suite must read the peer-euid check on both Unix OSes and the pid comparison on Linux only, macOS holding the euid check alone as the recorded carve-out.
quote: where it is reported (`PEER_EUID_REPORTED`, Unix), refuses another euid or one it cannot read (`peer_admitted`); it then compares the peer pid and that process's OS start time with the snapshot's `pid` and `started_at` from its one snapshot read (Session liveness, below); a refusal, a mismatch, or a check that cannot be made, writes no frame. macOS reports no peer pid and holds the euid check alone, the named carve-out; Windows reads no euid, its DACL and the pid comparison the controls.
@carried n=7 run=2 label=architecture fact=fct-ld365mzq
quote: on Linux and Windows, for the server check every frame-writing client makes before its first frame: the connected peer's pid and its OS start time against these two fields, a mismatch writing no frame (macOS reports no peer pid, the recorded gap).
@carried n=8 label=architecture/standard-contracts fact=fct-a6y4blve
why: the macOS gap stays recorded as `Matches` with no comparison, so no test may read macOS's server check as a pid or start-time comparison.
quote: The closed decision enum `ServerVerdict` (no crate error enum) is `Matches` · `Mismatch` · `Unverifiable`. The pure `server_verdict(recorded_pid, started_at, observed_pid, observed_start, peer_pid_reported)` compares the snapshot's `pid` and `started_at` (whole seconds, read through its RFC 3339 form) with the observed peer pid and that process's OS start time; with `peer_pid_reported` false, as on macOS, which reports no peer pid, it returns `Matches` with no comparison: the recorded gap.
@carried n=9 label=architecture/standard-contracts fact=fct-ikwybol4
why: the peer-euid decision is fixed by `cfg!(unix)`, and no Unix test seam the suite adds may force it to refuse.
quote: `peer_euid_admitted(observed, own, reported)` is `!reported || observed == Some(own)`, so a peer it cannot read is refused where reported; `peer_admitted(observed)` is that decision over this process's `own_euid()` and `PEER_EUID_REPORTED` (`cfg!(unix)`).
@carried n=10 label=architecture/standard-contracts fact=fct-rvenzu5e
why: the socket-directory check between the snapshot read and the connect is the Unix endpoint path the parity suite reads, each surface's refusal its own closed word.
quote: Between that read and the connect each client checks the recorded endpoint's directory (`viola_state::strict::check_endpoint_dir`), and a refused directory writes no frame: `src/cmd/client.rs` `connect` (`CallError::SocketDirRefused`, one `process-exit`), the hook's `connect` (`NotSent::SocketDir`, one `hook-decision`) and MCP's `frame` (`UnreachableCause::SocketDir`, no line).
@carried n=11 label=architecture/standard-contracts fact=fct-4yfg5mra
why: beside the stale wrapper the v1-40 witness boots, a frame verb must read `instance-dead` on every OS.
quote: Where `live_endpoint` gives none, `viola_state::liveness::without_endpoint(instance_dir) -> UnreachableCause` reads the cause through `symlink_metadata`: a directory reads `InstanceDead`, anything else (absent, a regular file, a symlink) `UnwrappedName`. It opens nothing and writes no line, and the strict-modes check comes first at every caller: `client::Unreachable::without_endpoint` and `frame` in `viola-mcp`'s `tools.rs`. `live_endpoint` gives none for a stale wrapper, so a frame verb against a stale instance reads `instance-dead`.
@carried n=12 label=architecture/standard-contracts fact=fct-dtnnvvl3
why: the v1-40 witness reads a row whose `liveness` is `stale`, its beat older than 5 s, and the list item's fields hold on every OS.
quote: `liveness` is `live` or `stale`. Gone instances are omitted.
@carried n=12 run=2 label=architecture fact=fct-beanu3w4
quote: `items` entry: `{name, wrapped, liveness, status, wheel?, budget_paused?, dialog_pending, cli_version?, cli_verified?}`; `liveness` `live` or `stale`, gone instances omitted;
@carried n=12 run=3 label=architecture fact=fct-arftgre4
quote: The wrapper touches `<instance dir>/heartbeat` every 1 s. When the snapshot's pid + start time no longer name a running process (the pid is dead, or alive with another start time) the session is `gone`, whatever the beat's age. Otherwise a beat at most 5 s old is `live`, and an older or absent beat is `stale`.
@carried n=13 label=architecture/standard-contracts fact=fct-smoe4pw5
why: the v1-40 witness runs CLI `list`, which reads heartbeat and snapshot from disk and needs no live wrapper, and its `{items, budget, skipped}` document is one the one-schema rule covers.
quote: `list` is not a channel method. The calling process reads `instances/*/` (heartbeat and snapshot) and `claude agents --json` itself, so `list` needs no live wrapper. Its `ok` payload (CLI `--json` and MCP `list`) is `{items, budget, skipped}`, with the item, `budget` and `skipped` shapes of the `list` payload above. As landed (chunk 2026-10-11-the-board-viola-list), CLI `list` reads `<home>/instances/` (`viola_state::board::scan`, then `board::read`) and `budget.json` (through `read_budget_stamped`) and contacts no wrapper.
@carried n=13 run=2 label=architecture fact=fct-cqamt3xw
quote: `list` is not a channel method: the calling process reads `instances/*/` and `budget.json` itself and makes one `claude agents --json` read joined into the board, so `list` needs no live wrapper; its `ok` payload (CLI `--json` and MCP `list`) is `{items, budget, skipped}`, and a failed agents read leaves every `status` `unknown`.
@carried n=14 label=architecture/standard-contracts fact=fct-yj2flshm
why: the witness's `viola list` makes the `claude agents --json` read on the Unix runners, and a failed read must leave every `status` `unknown` and never be an error.
quote: If it is not found, fails, is over the cap or cannot be parsed, every row's `status` is `unknown`, no unwrapped rows are listed, and the outcome is never an error. A row that matches a wrapped instance's `child_pid` or `agent_session_id` in its snapshot is merged into that instance's row, never listed separately. Both fields are exposed (`pid`, `sessionId`; as measured on 2.1.287), and the join is ledger row `agents-join`, relied on every OS.
@carried n=14 run=2 label=architecture fact=fct-l2ty3vtu
quote: If it is not found, fails, is over the cap or cannot be parsed, every `status` is `unknown`, no unwrapped row is listed, and the outcome is never an error.
@carried n=15 label=architecture/standard-contracts fact=fct-uve5rstc
why: the snapshots must carry one envelope on every OS, and since a replayed snapshot is never `stale`, the v1-40 witness must stand on a real snapshot with an old beat.
quote: A snapshot whose `v` is higher than the reader supports, or that fails to parse, is ignored, and the state is rebuilt by replaying `events.ndjson`. Replay recovers only log-derived fields: `links`, `agent_session_id`, `wheel`, `budget_paused` and `budget_override_until`. Fields no event carries (`endpoint`, `pid`, `started_at`, `pinned_bin`, `statusline_command`, `cli_version`, `cli_verified`, `child_pid`, `pending_dialog`, `cwd`) are treated as absent. Liveness then rests on the heartbeat alone, so the instance is `live` or gone and never `stale`.
@carried n=15 run=2 label=architecture fact=fct-vufggymy
quote: (atomically replaced JSON): `{v, written_at, writer, data}`.
@carried n=16 label=architecture/standard-contracts fact=fct-vgnixbh3
why: the instance snapshot's fields, `links` among them, must read the same shape on every OS, a field one OS cannot fill omitted as its documented absence.
quote: `links` is the set derived by replaying `link`/`unlink` events, as `{driver, driven, since}`. `since` is the `link` line's `ts`.
@carried n=16 run=2 label=architecture fact=fct-q5tbsmkw
quote: `data` is `{endpoint?, pid, started_at, pinned_bin, statusline_command?, cli_version?, cli_verified, wheel, budget_paused, budget_override_until?, pending_dialog?, links, child_pid?, agent_session_id?, cwd?}`. `started_at` is the wrapper's own OS start time (sysinfo, whole seconds) rendered RFC 3339, so a reader can compare it with the same field of the recorded `pid`.
@carried n=17 label=architecture/standard-contracts fact=fct-2rdzhblg
why: the `send` ok payload is a `--json` document whose fields must read the same on every OS.
quote: `send` `{text, from?}` → `{submitted_at, cursor}` when confirmed, or `{confirmed:false, detail:"unconfirmable", cursor}`. `cursor` is the end offset of `events.ndjson` when the send was accepted, before the paste.
@carried n=18 label=architecture/standard-contracts fact=fct-kzj34g66
why: the `wait` ok payload is a `--json` document whose fields must read the same on every OS.
quote: `wait` `{after?, timeout_ms?, from?}` → `{event:<ndjson event line>, cursor}`, where `cursor` is the end offset of that event's line, or `{timed_out:true}`.
@carried n=19 label=architecture/standard-contracts fact=fct-eiypz3p5
why: on Unix the hook's notification passes the peer-euid server check before its frame, and the line it appends must read the same `events.ndjson` shape on every OS.
quote: `hook.event`, sent id-less by `hook` once its instance check (`trusted`), its socket-directory check and its server check, the peer euid's included, have passed (a notification, no reply; a refused or absent instance, a replayed or absent snapshot, a refused socket directory or a failed server check sends nothing and the hook fails open): `{ts, event:{kind, data}}`. The wrapper re-validates the event (`kind` is one of the five hook kinds `session-start` · `prompt-submitted` · `turn-ended` · `session-end` · `activity`; `data` is an object; for `prompt-submitted`, `text` is a string and `origin` is `harness` · `human`) and appends `{v, ts:<append time>, instance, kind, source:"hook", data}` to `events.ndjson`.
@carried n=19 run=2 label=architecture fact=fct-lzgtihuf
quote: `source` ∈ `hook` · `wrapper` · `cli`. `data` is kind-specific, uses only viola's normalised fields, and never embeds a raw Claude payload.
@carried n=20 label=architecture/standard-contracts fact=fct-kylkhqhb
why: `prompt-submitted`'s `origin` words are closed event data that must read the same on every OS.
quote: `origin` is `driver` · `human` · `harness`: the hook files only `human` · `harness`, and `driver` is set only by the wrapper's relabel of the in-flight send's own prompt.
@carried n=21 label=architecture/standard-contracts fact=fct-p4s6aosk
why: the statusline shell-out is relied on Unix only and runs nothing on Windows, an OS-differing behaviour a parity test must read per OS, never moving the Windows side.
quote: The shell is the ledger row `statusline-shell` (probed through Run B's `--shell "$0"` word, relied on Unix only, since chunk 2026-10-13-owed-ledger-rows; §Cross-cutting Patterns, Capability ledger). On Windows `shell_argv` returns `None` for every input and no command runs
@carried n=21 run=2 label=architecture fact=fct-horlp4ay
quote: runs the user's own statusline command, read at start from one named source, through `/bin/sh -c` with the same stdin (the only shell-out in viola; none on Windows), printing its stdout unchanged; it always exits 0 with empty stderr.
@binding n=1
**session-liveness server check ↔ channel-methods ↔ instance snapshot (macOS carve-out)**: the server check every frame-writing client makes compares the peer pid and OS start time with the snapshot's `pid` and `started_at` on Linux and Windows only; on macOS `server_verdict` returns `Matches` with no comparison and the euid check stands alone, so the parity suite must read the comparison on Linux and must not read macOS as a comparison (architecture/standard-contracts fct-2tmeycxu) (architecture/standard-contracts fct-a6y4blve) (architecture fct-ld365mzq).
@binding n=2
**session-liveness ↔ list payload ↔ snapshot envelope (v1-40 witness)**: the witness's `viola list` must read a wrapper with a real snapshot whose beat is older than 5 s as `stale`; a replayed snapshot is `live` or gone and never `stale`, so the witness cannot stand on one (architecture fct-arftgre4) (architecture/standard-contracts fct-dtnnvvl3) (architecture/standard-contracts fct-uve5rstc) (architecture/standard-contracts fct-smoe4pw5).
@binding n=3
**session-liveness ↔ cli-json-output (stale wrapper)**: a stale wrapper gives no live endpoint, so a frame verb against it must print the exit-21 document with `detail` `instance-dead`, the same closed word on every OS (architecture/standard-contracts fct-4yfg5mra) (architecture/standard-contracts fct-dt7d6rx2).
@binding n=4
**channel-methods socket-directory check ↔ cli-json-output (one schema)**: the socket-directory refusal is reached only on the Unix endpoint path, and every surface it reaches (frame verbs, `link` / `unlink`, `run` / `revive`) must print the same closed `socket-dir-refused` word through the one writer, never an OS-only word (architecture/standard-contracts fct-rvenzu5e) (architecture/standard-contracts fct-qybkldvd) (architecture/standard-contracts fct-ubupynfq) (architecture/standard-contracts fct-zxepdepr) (architecture/standard-contracts fct-ng4l52uh).
@binding n=5
**peer-euid decision ↔ scope boundary**: the peer-euid decision is fixed by `PEER_EUID_REPORTED` (`cfg!(unix)`) and admits only the owning euid; a Unix-only test seam the suite adds must not force a refusing decision and needs a Decisions Log entry (architecture/standard-contracts fct-ikwybol4) (architecture/standard-contracts fct-eiypz3p5).
@binding n=6
**list ↔ claude agents read (fake agent in CI)**: the witness's `viola list` makes one `claude agents --json` read; where it is not found or fails on the runner, every `status` must read `unknown` with no unwrapped row and no error, the same on every OS (architecture/standard-contracts fct-yj2flshm) (architecture fct-cqamt3xw).
@binding n=7
**hook-statusline ↔ OS-differing behaviour**: the statusline shell-out through `/bin/sh -c` is relied on Unix only and Windows runs no command; a parity test must read each OS's own leg and move nothing on Windows (architecture/standard-contracts fct-p4s6aosk) (architecture fct-horlp4ay).
