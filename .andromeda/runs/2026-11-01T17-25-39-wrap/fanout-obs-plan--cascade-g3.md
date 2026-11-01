proposals:
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/span-trace-coverage.md
    near: 21
    section: '## 4. Span / Trace Coverage'
    change: 'Add this row after it: | ipc-internal (viola-channel), the wrapper''s accept | none | No span and no line: `accept_loop` filters each accepted stream through `peer_admitted`, dropping a refused peer on the accept thread, before its worker thread and before any frame. The peer reads end-of-file, no line is written because no frame exists to key one on, nothing reaches the outer terminal and the wheel does not move |'
    sidecar: 'span-trace-coverage: the wrapper''s accept-loop drop of a refused peer recorded, no span and no line'
    rationale: 'Report **Symbols / APIs**, `crates/viola-channel/src/server.rs`, and **Coverage of new surfaces**, the wrapper''s drop: instrumentation n/a, no frame to key a line on.'
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:75
    fact: F6
  - detector: D-cascade
    severity: warning
    file: .andromeda/obs-plan-detail/span-trace-coverage.md
    near: 28
    section: '## 4. Span / Trace Coverage'
    change: 'State the order before any frame as the socket-directory check `check_endpoint_dir`, the connect, the euid decision `peer_admitted(client.server_euid())`, then `verify_server`, a refused euid ending in the same `server-verify-failed` line; replace «Linux and Windows; macOS reports no peer pid and stays on the recorded gap.» with «The peer pid and start-time comparison runs on Linux and Windows. `server-verify-failed` reaches Linux, Windows and macOS, where on macOS it means an euid mismatch alone: macOS reports no peer pid, its named carve-out.»'
    sidecar: 'span-trace-coverage: the CLI server check''s order and `server-verify-failed`''s reach brought to Linux, Windows and macOS'
    rationale: 'Report **Symbols / APIs**, `src/cmd/client.rs` `connect`, and **Counts / qualifiers moved**, `verify-word`: the word''s reach moves, each row changes; a row stating macOS compares no peer pid stays true.'
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:15
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/span-trace-coverage.md
    near: 28
    section: '## 4. Span / Trace Coverage'
    change: 'Add this item after it: - **The client euid check:** each client seam (`src/cmd/client.rs` `connect`, `src/cmd/hook.rs` `connect`, `crates/viola-mcp/src/tools.rs` `frame`) reads the server''s euid after the connect and before `verify_server`, through `Client::server_euid` / `TokioClient::server_euid`, which write and log nothing; a peer it cannot read is refused where euids are reported (`PEER_EUID_REPORTED`, Unix). On Windows no euid is read, and the DACL and the pid comparison stay the controls.'
    sidecar: 'span-trace-coverage: the client euid check''s place at the three client seams recorded'
    rationale: 'Report **Symbols / APIs**, `viola_channel::{peer_euid_admitted, peer_admitted, PEER_EUID_REPORTED}`, and the three seams'' entries.'
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:12
    fact: F7
  - detector: D-cascade
    severity: warning
    file: .andromeda/obs-plan-detail/span-trace-coverage.md
    near: 29
    section: '## 4. Span / Trace Coverage'
    change: 'Add this item after it: - The socket-directory refusal''s place is asserted by two captured-span cases: `connect_refuses_a_loosened_socket_directory_inside_its_dispatch_span`, which opens the span by hand (`wait.block`) around `connect`, since the check sits in `connect`, and `hook_refuses_a_loosened_socket_directory_inside_hook_handle`; the span recorder `Seen` stands in the test module `refused_seen`, out of the macOS-gated `refused_inside`.'
    sidecar: 'span-trace-coverage: the socket-directory refusal''s captured-span cases and the recorder''s module recorded'
    rationale: 'Report **Deviations from intent** 4 and 5, and **New text, by line** for both cases.'
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:83
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/span-trace-coverage.md
    near: 30
    section: '## 4. Span / Trace Coverage'
    change: 'Add this item after it: - **cli (short-lived verbs), the socket-directory check:** a CLI frame verb (`send`, `wait`, `last`, `answer`, `pause`, `release`, `link`, `unlink`) meeting a refused socket directory writes one `process-exit{exit_code:21, detail:"socket-dir-refused", during:"connect"}` at WARN inside its dispatch span, no `channel-request` line and no frame. The check lives in `cmd::client::connect`, on the recorded endpoint, and never changes a mode: a refused directory stays as found.'
    sidecar: 'span-trace-coverage: the frame verbs'' socket-directory refusal and its line recorded'
    rationale: 'Report **Coverage of new surfaces**, the eight CLI frame verbs, and **Symbols / APIs**, `src/cmd/client.rs` `connect`.'
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:71
    fact: F3
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/span-trace-coverage.md
    near: 30
    section: '## 4. Span / Trace Coverage'
    change: 'Add this item after it: - The socket-directory refusal''s human pair, `unable: the socket directory for <name> cannot be trusted` / `hint: viola''s socket directory must be a directory you own, readable only by you; viola does not change its permissions`, is printed on start and frame-verb refusals in plain ASCII with `hint:` last and no SGR, and names only the parsed `ViolaName`. Its wording is a planning form, the founder''s own wording owed at Epoch 7''s boundary batch.'
    sidecar: 'span-trace-coverage: the socket-directory refusal''s human pair recorded, its wording a planning form'
    rationale: 'Report **Coverage of new surfaces**, the new pair, and **Decisions & corrections**: the wording owed at Epoch 7''s boundary batch.'
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:76
    fact: F9
  - detector: D-cascade
    severity: warning
    file: .andromeda/obs-plan-detail/span-trace-coverage.md
    near: 31
    section: '## 4. Span / Trace Coverage'
    change: 'State the chain as `read_snapshot_or_replay`, the socket-directory check `check_endpoint_dir`, `connect_by`, the euid decision `peer_admitted(client.server_euid())`, then `verify_server`; add that a refused euid gets no frame and fails open with the same `hook-decision{detail:"server-verify-failed"}`; replace «Linux and Windows; macOS on the recorded gap.» with «The peer pid and start-time comparison runs on Linux and Windows; `server-verify-failed` reaches Linux, Windows and macOS, where on macOS it means an euid mismatch alone (macOS compares no peer pid, its named carve-out).»'
    sidecar: 'span-trace-coverage: the hook server check''s order and `server-verify-failed`''s reach brought to Linux, Windows and macOS'
    rationale: 'Report **Symbols / APIs**, `src/cmd/hook.rs` `connect`: the same three checks in the same order, a refused euid `NotSent::ServerUnverified`; **Counts / qualifiers moved**, `verify-word`.'
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:16
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/span-trace-coverage.md
    near: 31
    section: '## 4. Span / Trace Coverage'
    change: 'Add this item after it: - **cli (`viola hook`), the socket-directory check:** a hook meeting a refused socket directory writes one `hook-decision{detail:"socket-dir-refused"}` at WARN inside `hook.handle`, on the dialog path after `hook-invoked{corr:null}` and with `decision_emitted:false`; exit 0, empty streams, no frame.'
    sidecar: 'span-trace-coverage: the hook''s socket-directory refusal and its line recorded'
    rationale: 'Report **Coverage of new surfaces**, the check at `viola hook`, and **Symbols / APIs**, `src/cmd/hook.rs` `connect`.'
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:72
    fact: F4
  - detector: D-cascade
    severity: warning
    file: .andromeda/obs-plan-detail/span-trace-coverage.md
    near: 33
    section: '## 4. Span / Trace Coverage'
    change: 'State the frame tools'' order after `check_instance` as `live_endpoint`, the socket-directory check `check_endpoint_dir` (a refusal returns `instance-unreachable` with `detail` `socket-dir-refused`, no frame and no line), `TokioClient::connect`, then the euid decision `peer_admitted(client.server_euid())` beside `server_verified`, either refusal reading `server-verify-failed`; replace «Linux and Windows; macOS on the recorded gap.» with «The peer pid and start-time comparison runs on Linux and Windows; `server-verify-failed` reaches Linux, Windows and macOS, where on macOS it means an euid mismatch alone (macOS compares no peer pid, its named carve-out).»'
    sidecar: 'span-trace-coverage: MCP''s frame order and `server-verify-failed`''s reach brought to Linux, Windows and macOS'
    rationale: 'Report **Symbols / APIs**, `crates/viola-mcp/src/tools.rs` `frame`; **Counts / qualifiers moved**, `verify-word`.'
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:17
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/span-trace-coverage.md
    near: 42
    section: '## 4. Span / Trace Coverage › #### Scenario: `viola run` start sequence to child spawn'
    change: 'Add at the end of the item: A further start step carries no span and writes a line only when it refuses (chunk 2026-11-01-unix-endpoint-and-home-hardening): `viola run` and `viola revive`''s start check the endpoint''s parent, create it only when absent, then check again, so a failed create, a lost race and a refused check read one code; a refusal is one `process-exit{subject:"self", exit_code:1, detail:"socket-dir-refused"}`, before any snapshot, child or `Server::bind`. A `Server::bind` failure stays what it was.'
    sidecar: 'span-trace-coverage: the start''s socket-directory step recorded, no span, one line on a refusal'
    rationale: 'Report **Symbols / APIs**, `src/cmd/run.rs`, and **Coverage of new surfaces**: a step with no span, one line.'
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:14
    fact: F2
  - detector: D-cascade
    severity: warning
    file: .andromeda/obs-plan-detail/span-trace-coverage.md
    near: 57
    section: '## 4. Span / Trace Coverage › #### Scenario: `viola run` start sequence to child spawn'
    change: 'Extend the failure `detail` list to `already-live|stale-heartbeat|squatted-name|pinned-hash-mismatch|batch-script-child|socket-dir-refused`, and gloss `socket-dir-refused` as a socket directory the start refuses, on `viola revive`''s start too.'
    sidecar: 'span-trace-coverage: the start''s exit-1 `detail` list gains `socket-dir-refused`'
    rationale: 'Report **Symbols / APIs**, `src/cmd/run.rs`: `start` calls `refused_with` with `socket-dir-refused`, and `revive`''s start shares it; **Schema / config**: the word joins `process-exit`''s `detail`.'
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:14
  - detector: D-cascade
    severity: warning
    file: .andromeda/obs-plan-detail/span-trace-coverage.md
    near: 78
    section: '## 4. Span / Trace Coverage › #### Scenario: Confirmed `send` (CL-1) from driver to readback'
    change: 'Add that before the server check come the socket-directory check on the recorded endpoint, the connect and the euid decision, a refused directory or euid opening no `channel.request` either; replace «(Linux and Windows; macOS on the recorded gap)» with «(the peer pid and its start time on Linux and Windows; `server-verify-failed` on Linux, Windows and macOS, on macOS an euid mismatch alone, macOS comparing no peer pid, its named carve-out)».'
    sidecar: 'span-trace-coverage: `send`''s checks before `channel.request` and the word''s reach brought current'
    rationale: 'Report **Symbols / APIs**, `src/cmd/client.rs` `connect` and `crates/viola-mcp/src/tools.rs` `frame`; **Counts / qualifiers moved**, `verify-word`.'
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:31
  - detector: D-cascade
    severity: warning
    file: .andromeda/obs-plan-detail/span-trace-coverage.md
    near: 93
    section: '## 4. Span / Trace Coverage › #### Scenario: Confirmed `send` (CL-1) from driver to readback'
    change: 'Replace «(Linux and Windows)» with «(Linux, Windows and macOS, on macOS an euid mismatch alone)», and add this item after it: client-side socket-directory refusal, made after the client''s own refusals and before the connect: no `channel-request` / `channel-response` line on either side; CLI `send` ends `process-exit{exit_code:21, detail:"socket-dir-refused", during:"connect"}` at WARN, and MCP `send` writes no line;'
    sidecar: 'span-trace-coverage: `send`''s client-side refusal items, the word''s reach and the socket-directory refusal'
    rationale: 'Report **Counts / qualifiers moved**, `verify-word`; **Coverage of new surfaces**, the check at the frame verbs and at MCP''s `frame`.'
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:71
  - detector: D-cascade
    severity: warning
    file: .andromeda/obs-plan-detail/span-trace-coverage.md
    near: 96
    section: '## 4. Span / Trace Coverage › #### Scenario: Confirmed `send` (CL-1) from driver to readback'
    change: 'Add to exit 21''s words `detail:"socket-dir-refused"` with `during:"connect"` (the recorded endpoint''s socket directory refused; no frame sent); extend `server-verify-failed`''s gloss with a server of another euid; replace «Linux and Windows, macOS on the recorded gap» with «Linux, Windows and macOS, on macOS an euid mismatch alone».'
    sidecar: 'span-trace-coverage: `send.client`''s exit 21 words gain `socket-dir-refused`, the word''s reach moved'
    rationale: 'Report **Counts / qualifiers moved**: exit 21''s closed causes five to six, and `verify-word`.'
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:28
  - detector: D-cascade
    severity: warning
    file: .andromeda/obs-plan-detail/span-trace-coverage.md
    near: 98
    section: '## 4. Span / Trace Coverage › #### Scenario: Confirmed `send` (CL-1) from driver to readback'
    change: 'Replace «`instance-dead`, `unwrapped-name` and `server-verify-failed` are `during` `connect`» with «`instance-dead`, `unwrapped-name`, `server-verify-failed` and `socket-dir-refused` are `during` `connect`».'
    sidecar: 'span-trace-coverage: `socket-dir-refused` joins the `during` `connect` words'
    rationale: 'Report **Coverage of new surfaces**, the frame verbs: `process-exit{exit_code:21, detail:"socket-dir-refused", during:"connect"}`.'
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:71
  - detector: D-cascade
    severity: warning
    file: .andromeda/obs-plan-detail/span-trace-coverage.md
    near: 107
    section: '## 4. Span / Trace Coverage › #### Scenario: `wait` / `last` event-driven readback'
    change: 'Add that the socket-directory check, the connect and the euid decision come before the server check, a refused directory or euid opening no `channel.request` either; replace «(Linux and Windows; macOS on the recorded gap)» with «(the peer pid and its start time on Linux and Windows; `server-verify-failed` on Linux, Windows and macOS, on macOS an euid mismatch alone, macOS comparing no peer pid, its named carve-out)».'
    sidecar: 'span-trace-coverage: `wait` / `last`''s checks before `channel.request` and the word''s reach brought current'
    rationale: 'Report **Symbols / APIs**, `src/cmd/client.rs` `connect`; **Counts / qualifiers moved**, `verify-word`.'
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:15
  - detector: D-cascade
    severity: warning
    file: .andromeda/obs-plan.md
    near: 361
    section: '## 4. Span / Trace Coverage › #### Scenario: `wait` / `last` event-driven readback'
    change: 'Replace «on Linux and Windows a serving process that is not the recorded wrapper ends» with «a serving process that is not the recorded wrapper or runs under another euid ends», and «(macOS compares no peer pid, the recorded gap)» with «(on macOS an euid mismatch alone, since macOS compares no peer pid, its named carve-out)»; add that a refused socket directory ends a CLI `wait` / `last` before any frame with one `process-exit{exit_code:21, detail:"socket-dir-refused", during:"connect"}` at WARN and no `channel-request` line, and MCP''s returns `instance-unreachable` with `detail` `socket-dir-refused` and no line.'
    sidecar: 'obs-plan §4: the `wait` / `last` broad line takes the moved reach and the socket-directory refusal'
    rationale: 'Report **Counts / qualifiers moved**, `verify-word`, each row changes; **Coverage of new surfaces**, the frame verbs and MCP''s `frame`; **Expected amendments** carries obs-plan §4.'
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:31
  - detector: D-cascade
    severity: warning
    file: .andromeda/obs-plan-detail/span-trace-coverage.md
    near: 114
    section: '## 4. Span / Trace Coverage › #### Scenario: `wait` / `last` event-driven readback'
    change: 'Add to exit 21''s words `detail:"socket-dir-refused"` with `during:"connect"`; replace «(Linux and Windows; macOS on the recorded gap)» with «(Linux, Windows and macOS, on macOS an euid mismatch alone)»; replace «then the liveness-only pre-check and then the server check» with «then the liveness-only pre-check, the socket-directory check, the connect, the euid decision and then the server check».'
    sidecar: 'span-trace-coverage: `wait` / `last`''s client `process-exit` gains `socket-dir-refused`, its order and reach moved'
    rationale: 'Report **Symbols / APIs**, `src/cmd/client.rs` `connect`; **Counts / qualifiers moved**, the causes and `verify-word`.'
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:15
  - detector: D-cascade
    severity: warning
    file: .andromeda/obs-plan-detail/span-trace-coverage.md
    near: 116
    section: '## 4. Span / Trace Coverage › #### Scenario: `wait` / `last` event-driven readback'
    change: 'Replace «or on a server-verification refusal made before any frame» with «or on a socket-directory or server-verification refusal made before any frame».'
    sidecar: 'span-trace-coverage: `wait.block` also closes on a socket-directory refusal'
    rationale: 'Report **Coverage of new surfaces**, the frame verbs: no `channel-request`, the `process-exit` the record; `wait` is one of `connect`''s callers.'
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:71
  - detector: D-cascade
    severity: warning
    file: .andromeda/obs-plan-detail/span-trace-coverage.md
    near: 125
    section: '## 4. Span / Trace Coverage › #### Scenario: Dialog → `answer` (question / permission / plan), including the unverified-CLI branch'
    change: 'Add that the socket-directory check, the connect and the euid decision come before each server check of this path, a refused directory or euid opening neither `channel.request`; replace «(Linux and Windows; macOS on the recorded gap)» with «(the peer pid and its start time on Linux and Windows; `server-verify-failed` on Linux, Windows and macOS, on macOS an euid mismatch alone, macOS comparing no peer pid, its named carve-out)».'
    sidecar: 'span-trace-coverage: the dialog path''s checks before each frame and the word''s reach brought current'
    rationale: 'Report **Symbols / APIs**, `src/cmd/client.rs` and `src/cmd/hook.rs` `connect`; **Counts / qualifiers moved**, `verify-word`.'
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:16
  - detector: D-cascade
    severity: warning
    file: .andromeda/obs-plan-detail/span-trace-coverage.md
    near: 134
    section: '## 4. Span / Trace Coverage › #### Scenario: Dialog → `answer` (question / permission / plan), including the unverified-CLI branch'
    change: 'Add this item after it: A refused socket directory is another of those paths: `hook-invoked{corr:null}` then `hook-decision{decision_emitted:false, detail:"socket-dir-refused"}` at WARN, with no `hook.dialog` frame and so no `dialog-raised`.'
    sidecar: 'span-trace-coverage: a refused socket directory joins the dialog hook''s fail-open paths before `dialog_id`'
    rationale: 'Report **Coverage of new surfaces**, the check at `viola hook`: on the dialog path `hook-invoked{corr:null}` first and `decision_emitted:false`.'
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:72
  - detector: D-cascade
    severity: warning
    file: .andromeda/obs-plan-detail/span-trace-coverage.md
    near: 137
    section: '## 4. Span / Trace Coverage › #### Scenario: Dialog → `answer` (question / permission / plan), including the unverified-CLI branch'
    change: 'Add this item after it: a socket-directory refusal on CLI `answer`, made before any frame: no `channel-request` / `channel-response` line, `process-exit{exit_code:21, detail:"socket-dir-refused", during:"connect"}` at WARN; MCP `answer` writes no line.'
    sidecar: 'span-trace-coverage: CLI `answer`''s socket-directory refusal line recorded'
    rationale: 'Report **Coverage of new surfaces**, the frame verbs and MCP''s `frame`; `answer` is one of `connect`''s callers.'
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:71
  - detector: D-cascade
    severity: warning
    file: .andromeda/obs-plan-detail/span-trace-coverage.md
    near: 149
    section: '## 4. Span / Trace Coverage › #### Scenario: Human takes the wheel, automation is refused, `release` returns it'
    change: 'Add that the socket-directory check, the connect and the euid decision come before the server check, a refused directory or euid opening no `channel.request` and sending no frame; replace «(Linux and Windows; macOS on the recorded gap)» with «(the peer pid and its start time on Linux and Windows; `server-verify-failed` on Linux, Windows and macOS, on macOS an euid mismatch alone, macOS comparing no peer pid, its named carve-out)».'
    sidecar: 'span-trace-coverage: `pause` / `release`''s checks before `channel.request` and the word''s reach brought current'
    rationale: 'Report **Symbols / APIs**, `src/cmd/client.rs` `connect`; **Counts / qualifiers moved**, `verify-word`.'
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:15
  - detector: D-cascade
    severity: warning
    file: .andromeda/obs-plan-detail/span-trace-coverage.md
    near: 154
    section: '## 4. Span / Trace Coverage › #### Scenario: Human takes the wheel, automation is refused, `release` returns it'
    change: 'Add this item after it: a socket-directory refusal on `pause` / `release`, made before any frame: no `channel-request` / `channel-response` line, the client''s `process-exit{exit_code:21, detail:"socket-dir-refused", during:"connect"}` at WARN;'
    sidecar: 'span-trace-coverage: `pause` / `release`''s socket-directory refusal line recorded'
    rationale: 'Report **Coverage of new surfaces**, the frame verbs; `pause` and `release` are among `connect`''s callers.'
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:71
note: 'D-obs-stack and D-obs-pii read no drift; D-obs-instrumentation reads none either, the start step''s missing span and the accept drop''s missing line being the report''s stated instrumentation; F1, F5, F8 and F10 are other categories'' facts for the new-detail pass, and `human::refuse(`''s census in obs-plan §3 lies outside this group.'
