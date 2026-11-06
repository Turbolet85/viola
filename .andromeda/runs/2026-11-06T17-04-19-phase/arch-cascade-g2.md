@carried n=1 label=architecture/established-decisions fact=fct-gmzv3mtg
why: The `portable-pty` move must leave the sole-root `tokio` ban, the `cargo check` of the sync crates (`viola-pty` among them) and the tokio-free channel check green, each re-read at the wrap.
quote: CI enforces this with a sole-root `cargo deny` `tokio` ban (`deny-sync.toml`, run once per crate listed in `scripts/sync-crates.txt`), evaluated for the one target triple `x86_64-unknown-linux-gnu`, plus a `cargo check` of the listed sync crates on `ubuntu-latest` (CI job 3). `viola-channel`, whose optional `tokio` feature `viola-mcp` enables, is not on that list (since chunk 2026-10-17-mcp-server-for-drivers): its normal graph without the feature is proven tokio-free by `scripts/channel-tokio-free.sh` in the same CI supply-chain step.
@carried n=1 run=2 label=architecture fact=fct-l5jpezvw
quote: CI enforces it with a sole-root `cargo deny` `tokio` ban and a `cargo check` of the sync crates on `ubuntu-latest`. Rationale: portable-pty's handles are blocking, so the pump needs threads either way;
@carried n=2 label=architecture/established-decisions fact=fct-ad44nn2e
why: The chunk moves `portable-pty` off `=0.8.1`; under the new release the seam must still detect child exit on the process handle, read `openpty`, never hold a key behind a resize and never answer DA1, and the decision's pin text must follow the move.
quote: **[PTY] portable-pty `=0.8.1` pinned behind viola's own `pty` seam (spawn · read · write · resize · wait · kill).** Child exit is detected on the process handle, never on EOF.
@carried n=2 run=2 label=architecture fact=fct-omwrdnlc
quote: Child exit is detected on the process handle, never on EOF; `pty_backend()` is `openpty`; viola never holds, queues or re-orders a human key behind a resize and never answers DA1 itself. Rationale: 0.8.1 is spike-proven; moving `portable-pty` off 0.8.1 to a release whose graph drops `pin-utils` (RUSTSEC-2026-0441) is owed, owner: "Version done-check".
@carried n=3 label=architecture/established-decisions fact=fct-co2wsjkw
why: The chunk owns this remedy: the new `portable-pty` release's graph must drop `pin-utils` so advisories read green, with no ignore-list widening, and this record is then retired.
quote: RUSTSEC-2026-0441 (`pin-utils` unmaintained, via `portable-pty =0.8.1` → `nix 0.25.1`) reads red in supply-chain's advisories family, as measured at ci#42030025050; its owner is a CARRY on the working-route entry "Version done-check": moving `portable-pty` off `=0.8.1` to a release whose graph drops `pin-utils`, with no ignore-list widening.
@carried n=4 label=architecture/established-decisions fact=fct-va43dnf2
why: The live drive session's sends must be read by their confirmed outcome, and an MCP-driven send must stay inside the client's tool-call timeout.
quote: **[Delivery Confirmation] Every `send` is confirmed.** If no matching `prompt-submitted` arrives within the confirmation window, the send is reported as `not-delivered` with detail `no-prompt-submitted`.
@carried n=4 run=2 label=architecture/established-decisions fact=fct-va43dnf2
quote: On the MCP surface that total must stay below the MCP client's tool-call timeout, which on claude 2.1.287 exceeds 50 s (as measured live at chunk 2026-10-17-mcp-server-for-drivers), above the 48.5 s plus paste of a `/compact` send.
@carried n=4 run=3 label=architecture fact=fct-i4ogppls
quote: **[Delivery Confirmation] Every `send` is confirmed.** No matching `prompt-submitted` within the compiled confirmation window (never in `config.json`) is `not-delivered` / `no-prompt-submitted`;
@carried n=5 label=architecture/established-decisions fact=fct-wfmyj5kz
why: The live drive and the near-ceiling dialog hold are read through `wait` waking on a pending dialog or `dialog-expired` and `answer` naming its `dialog_id`.
quote: `hook` blocks on the channel for a dialog answer. `wait` blocks until the next driver-relevant event (`turn-ended`, `question`, `permission`, `plan`, `session-end` or `dialog-expired`) and returns that event, so nothing polls and a driver parked in `wait` learns that a dialog is pending.
@carried n=5 run=2 label=architecture fact=fct-w2igrcse
quote: `hook` blocks on the channel for a dialog answer and `wait` until the next driver-relevant event, so nothing polls;
@carried n=6 label=architecture/established-decisions fact=fct-wqjpkbtb
why: The always-admit `peer_admitted` mutation survivor stays a dated note at matrix#v1-44 and test-plan §1's untestable zone, and no seam may be added to kill it.
quote: `viola_channel` exports `peer_euid_admitted` and `peer_admitted`: `peer_euid_admitted(observed, own)` is `observed == Some(own)`, so a peer whose euid cannot be read is refused.
@carried n=6 run=2 label=architecture/established-decisions fact=fct-ddp4mvl7
quote: The wrapper's `accept_loop` filters `incoming()` by `admitted(peer_euid_of(stream))`, so a refused stream is dropped on the accept thread, before its worker thread and any frame, reading end-of-file, writing no line, nothing on the terminal and moving no wheel; `Server::serve` injects the decision, `peer_admitted`, as a `type Admission` through the private `serve_admitting`, with no env, flag or feature seam.
@carried n=7 label=architecture/established-decisions fact=fct-kbitf2mh
why: The socket-directory pair's wording, owed the founder's word, must state this check's refusals truthfully: a refused directory left as found, never chmodded.
quote: `viola_state::strict::check_socket_dir` takes one `symlink_metadata`: an absent path passes, an unstattable one is `Refused::Unreadable`, then `socket_dir_verdict` reads a symlink as `Refused::Symlink`, a non-directory as the fixed-message `Refused::NotADirectory` and anything else by the read rule; it never chmods, a refused directory stays as found.
@carried n=8 label=architecture/established-decisions fact=fct-xewwdlh6
why: `v1-13`'s MCP clause for `link` and the budget release is disposed of against `release`, `link` and `unlink` staying CLI-only, and an MCP-driven live wait is capped at 50 000 ms per call.
quote: **[MCP] rmcp 3.4.1, stdio, tools `send · wait · last · answer · list`. `release` is CLI-only.**
@carried n=8 run=2 label=architecture/established-decisions fact=fct-xewwdlh6
quote: `pause`, `link` and `unlink` are CLI-only for the same reason: they are human controls over the wheel and the link topology, and links made by drivers are already recorded from `from` on `send` and `answer`.
@carried n=8 run=3 label=architecture/established-decisions fact=fct-xewwdlh6
quote: MCP `wait` sends `timeout_ms` = `min(given, 50 000)` (`viola_mcp::WAIT_CAP`, `wait_timeout`), 50 000 ms when none is given, and `after` as given.
@carried n=8 run=4 label=architecture fact=fct-tkwxvt3j
quote: **[MCP] rmcp 3.4.1, stdio, tools `send · wait · last · answer · list`. `release` is CLI-only.** Refusals are `isError: true` tool results mapped from `result.refusal`; `pause`, `link` and `unlink` are CLI-only too, as human controls over the wheel and the link topology.
@carried n=9 label=architecture/established-decisions fact=fct-wpbp4hte
why: The near-ceiling live hold exercises this range, reply bound and hook timeout, and the ceiling's PROVISIONAL wording is re-worded from that reading.
quote: A sync dialog hook (`viola hook pre-tool-use|permission-request`) waits on `hook.dialog` for at most the instance's dialog hold, `config.json`'s `dialog_hold_s`: whole seconds from 60 to 3600, default 60 (`viola_core::DIALOG_HOLD_DEFAULT` · `DIALOG_HOLD_MIN` · `DIALOG_HOLD_MAX`, PROVISIONAL, beside `SPINE_DEADLINE`), a `DialogHold` read once at the wrapper's start; the hook bounds its reply read at the ceiling, `DIALOG_HOLD_MAX + 5 s` (3605 s), never at each start's hold, and writes stdout once. The dialog hooks' `timeout` in the embedded `hooks.json` is 3615 s, above the ceiling by a margin a test pins, so viola, never Claude Code, ends the wait, and the rendered plugin stays byte-identical across starts. At the hold's end the wrapper replies `null`: the hook exits 0 with no body and the dialog renders for the human.
@carried n=9 run=2 label=architecture/established-decisions fact=fct-ohekhpaf
quote: At the hold's end the wrapper also appends one `dialog-expired` line (`source: wrapper`, `{dialog_id, dialog_kind}`, no question, plan or tool text) through `WaitFeed::appending`, so a parked `wait` wakes and the driver learns the dialog is gone; a hand-back or an answer appends none, and a failed append changes nothing else
@carried n=9 run=3 label=architecture fact=fct-4nt6qtyp
quote: A sync dialog hook waits at most the dialog hold (`dialog_hold_s`, 60–3600 s, default 60; ceiling PROVISIONAL, owner: "Version done-check"), the dialog then rendering for the human.
@carried n=10 label=architecture/established-decisions fact=fct-vxml3b56
why: This chunk owns the first live read of a hold near the 3600 s ceiling, which settles the ceiling's PROVISIONAL standing.
quote: A hold near the 3600 s ceiling is first read live at "Version done-check"; the ceiling is PROVISIONAL.
@carried n=10 run=2 label=architecture/established-decisions fact=fct-l7zodwoq
quote: A dialog hook held 671 s returned the driver's `viola answer` decision into claude 2.1.287 on the Linux dev host, as measured at `viola-0.1.0/chunks/2026-10-19-dialog-hold-for-the-driver/evidence/live-round.md`.
@carried n=11 label=architecture/established-decisions fact=fct-4coxmu76
why: The CARRY on the closed non-editing list measured on foot only may add a shape only when it is measured by its exact grammar, and the live drive starts with the wheel at `driver`.
quote: Only a closed non-editing list never counts (the founder's live ruling F-W2, 2026-10-04, relayed by the overseer): focus reports (`CSI I` / `CSI O`), mouse reports (X10 `CSI M` + 3 bytes, SGR `CSI < n;n;n M|m`, urxvt `CSI n;n;n M`) and terminal replies (DA1 `CSI ? … c`, DA2 `CSI > … c`, CPR `CSI n;n R`, DECRPM `CSI ? n;n $ y`, kitty flags `CSI ? n u`, OSC replies ending BEL or ST, DCS replies ending ST, and seven further reply shapes, each after `CSI` by exact grammar, the fields compared as bytes, no intermediate byte and the field count exact: DSR status `CSI 0 n`, colour scheme `CSI ? 997;1 n` and `CSI ? 997;2 n`, window size `CSI 4;n;n t`, cell size `CSI 6;n;n t`, text area `CSI 8;n;n t`, in-band resize `CSI 48;n;n;n;n t`, modifyOtherKeys reply `CSI > 4;n m`); a host resize is no stdin byte at all.
@carried n=11 run=2 label=architecture/established-decisions fact=fct-4coxmu76
quote: A reply no terminal has been measured to write is not added on a guess.
@carried n=11 run=3 label=architecture/established-decisions fact=fct-4coxmu76
quote: The wheel starts with `driver` when `viola run` starts, so automation is accepted until the first human editing key.
@carried n=11 run=4 label=architecture fact=fct-dl246x3d
quote: The wheel starts with `driver` at every start; any human editing key since the last turn boundary moves it to the human and `send` is refused `human-typing`; only a closed non-editing list never counts;
@binding n=1
**[PTY] ↔ [Concurrency / Backend Framework] ↔ Stack**: the `portable-pty` release the chunk moves to must drop `pin-utils` and keep `viola-pty` tokio-free under the sole-root ban and the sync-crate check, and the `=0.8.1` text moves together in the PTY entity, its body line and the Stack line (architecture/established-decisions fct-ad44nn2e) (architecture fct-omwrdnlc) (architecture/established-decisions fct-gmzv3mtg) (architecture fct-zegjs5rw)
@binding n=2
**RUSTSEC-2026-0441 ↔ Occupied Resources (`target/supply-chain/`)**: the advisory reads green in `deny.json` and `deny-fuzz.json` of CI's `supply-chain` artifact once the move lands, with no ignore-list widening, and the CARRY record retires with it (architecture/established-decisions fct-co2wsjkw) (architecture fct-gwc5vubx)
@binding n=3
**[Hook Contract] dialog hold ↔ Occupied Resources (`config.json`, `schemas/config.v1.json`, registered hook `timeout`)**: whatever the near-ceiling live read settles for the 3600 s ceiling, the hold's range, its PROVISIONAL wording and the 3615 s hook timeout move together in the decision, the `config.json` row, the config schema row and the registered-hooks row (architecture/established-decisions fct-wpbp4hte) (architecture/established-decisions fct-vxml3b56) (architecture fct-i4wacyrp) (architecture fct-6e4hf2ai) (architecture fct-h57b2ldw)
@binding n=4
**[Message Broker / IPC] peer admission ↔ Occupied Resources (Linux IPC endpoint)**: the always-admit `peer_admitted` survivor stays a dated note with no seam added, and the channel-handle no-leak measurement under `/proc/<child pid>/fd` is read against the per-user 0700 socket directory and the 0600 socket this row names (architecture/established-decisions fct-wqjpkbtb) (architecture/established-decisions fct-ddp4mvl7) (architecture fct-wular7qm)
@binding n=5
**[MCP] CLI-only verbs ↔ Standard Contracts (`list` payload) ↔ [GUI Control Scope]**: `v1-13`'s MCP clause for `link` and the budget release has no MCP result while `release` and `link` stay CLI-only, and `v1-36`'s link set exists in no `list` payload but only in the deferred 0.2.0 page's intent, so each clause is deferred with its reason or re-worded on the founder's word (architecture/established-decisions fct-xewwdlh6) (architecture fct-beanu3w4) (architecture fct-7wwoxbip)
@binding n=6
**[Human Takeover / Wheel] closed list ↔ [PTY]**: the wheel's list only grows by a measured exact grammar on the founder's word, and viola never answers DA1 itself, so a terminal reply joins the list without viola writing any reply (architecture/established-decisions fct-4coxmu76) (architecture fct-omwrdnlc)
@binding n=7
**[Error Handling] ↔ Conventions (error types) ↔ Inherited Defaults (Errors)**: each open divergence's fold or ratification and the re-measured `thiserror::Error` derive count are stated alike in all three lines (architecture fct-sha7o35z) (architecture fct-nhsg4k7g) (architecture fct-vf75c7sp)
@binding n=8
**[CLI Conventions] ↔ Cross-cutting (Process and path discipline)**: removing `plain_drive` removes the Git Bash rewritten-path warning it served, so the CLI Conventions line follows the removal, and no gate left by the remnant removal names `windows` or `target_os = "macos"` (architecture fct-nvft5tqc) (architecture fct-fa2klbw5)
@binding n=9
**[Deployment / Distribution] ↔ Occupied Resources (`bin/<version>-<hash>/`)**: Measurement B (one tree keying two pinned copies) is the founder's item at this entry, and the decision's owed clause and the pinned-copy row move together with his word (architecture fct-c3wssf3l) (architecture fct-hnee7xdr)
