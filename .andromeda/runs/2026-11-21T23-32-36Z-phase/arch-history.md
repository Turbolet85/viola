@history_item n=1 marker=2026-09-29-h2-conpty-resize-probe
**2026-09-29-h2-conpty-resize-probe** (H2 recorded under [PTY]) — origin of this chunk's claim: on the windows-2025 runner (inbox ConPTY, under llvm-cov, an isolated loop of the viola-pty red test) a key written through the seam right after `resize` returned never reached the Rust test child in 13 of 200, the child's watcher having seen the new size, no CPR seen; cause inside the ConPTY input path not established, and whether real `claude` loses such a key was left unmeasured. It is a Windows/ConPTY measurement on a test child, never a Linux or real-CLI reading. Same entry minted decision/no-key-held-behind-resize and decision/human-key-never-held-behind-resize: viola never holds, queues or re-orders a human key behind a resize, and a test that writes a key after a resize waits for the child to observe the new size first (0 of 200). Any "settle wait" fix option must be priced against these rows.

@history_item n=2 marker=2026-09-29-h2-conpty-resize-probe
**2026-09-29-h2-conpty-resize-probe** (the viola-pty-watch report files) — registered the test-only `<temp dir>/viola-pty-watch/` pair per real-PTY test: the child's `.report` (start, key-free size watcher's `size`, `byte`, `restored`) and the test's `.test.report` (`resize-returned` · `key-written` · `key-flushed` · `dsr-cpr {n}`), kept on panic, removed on pass, codes and counts only. This is the existing recorder that classifies a lost key (resize never applied · applied and key lost · key before the size) and the precedent for a Linux measuring arm.

@history_item n=3 marker=2026-09-29-verify-stamped-test-homes-and-harness
**2026-09-29-verify-stamped-test-homes-and-harness** (H2 owner) — moved the real-`claude` key-after-resize measurement off the real-CLI verify entry to "First live test and self-drive"; the harness's `run --local-live` (a real-claude verify) explicitly does not claim it. First link in the chain of owners this chunk now discharges.

@history_item n=4 marker=2026-09-29-sideloaded-conpty
**2026-09-29-sideloaded-conpty** — H2 with/without the sideloaded ConPTY in one windows-2025 run: sideloaded 0 of 200, inbox 14 of 200 (no rate claimed). Further evidence the recorded loss is a Windows inbox-ConPTY artefact; the sideload and its readings were later retired with the Windows code.

@history_item n=5 marker=2026-10-05-t00-09-13-wrap
**2026-10-05-t00-09-13-wrap** — Kept: H2's real-claude key loss after a resize stays owned by "First live test and self-drive", beside the DA1 stall and the Windows mouse report; shows the claim was carried as an open measurement, not a fact.

@history_item n=6 marker=2026-10-07-t05-47-07-wrap
**2026-10-07-t05-47-07-wrap** — re-owned the real-`claude` key-after-resize question to "Windows-only live measurements", blocked on an interactive Windows host (the founder's split R-L1..R-L4; first live test on the Linux dev host). Confirms the claim was always framed as Windows-only and never read on Linux.

@history_item n=7 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** ([PTY]: openpty alone) — retired the Windows ConPTY host and the H2 resize key-loss readings on `windows-2025`; `pty_backend()` is always `openpty`, `host_size()` reads `TIOCGWINSZ`, `host_stdin()` is `std::io::stdin()`, `HostTerminal` sets termios raw mode, the pump keeps the input writer until the child exits; "the resize rule (no human key held behind a resize) stands". The OPEN kept only the resize key, owner the open residuals of `2026-11-04T23-11-43-wrap` (target `another-os`). Defines today's Linux resize path this chunk must describe.

@history_item n=8 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (windows-sys out, libc plain) — `viola-pty` now depends on portable-pty, tracing and libc only (no thiserror, `PtyError` the hand-written exception); any fix inside viola-pty must keep that edge set (decision/pty-knows-no-agent).

@history_item n=9 marker=2026-11-08T20-32-52-wrap
**2026-11-08T20-32-52-wrap** (0.2.0 version census, C-005) — the key-after-resize OPEN item's owner moved from the `another-os` residuals to "the measurement of «Key after a resize», on Linux" — the route entry this chunk is; the chunk discharges that OPEN.

@history_item n=10 marker=2026-11-06-version-done-check
**2026-11-06-version-done-check** (portable-pty pinned at =0.9.0) — every pin site reads portable-pty `=0.9.0` behind viola's own `pty` seam (decision/portable-pty-behind-pty-seam); `viola-pty` compiles unchanged and passes its tests on it, `pty_backend()` `openpty`. The resize call this chunk inspects runs on this version; the 0.9.0 Windows garbage-read rationale was retired.

@history_item n=11 marker=2026-09-25-pty-wrapper-on-windows
**2026-09-25-pty-wrapper-on-windows** — [PTY] as built: host-terminal raw mode (`HostTerminal`), explicit child cwd, viola-side program resolution; viola-pty takes no thiserror and `PtyError` is the ratified exception. Background for the seam (spawn, read, write, resize, wait, kill) this chunk's resize path sits in.

@history_item n=12 marker=2026-10-13-epoch-4-survivors
**2026-10-13-epoch-4-survivors** — the real-PTY watch reports became `<test name>.<pid>.report` / `.<pid>.test.report`, because the `resize` mutant read missed by the audit was red when its tests ran alone (two test processes shared one report file; that cause recorded, not established). A trap for any new resize test or recorder.

@history_item n=13 marker=2026-10-04-the-wheel
**2026-10-04-the-wheel** — the closed non-editing list as landed: only focus reports, mouse reports and terminal replies never take the wheel; "a resize is no stdin byte"; every other byte takes the wheel; the classifier only observes and moves the wheel before a read holding an editing key returns. Binds how a key typed after a resize is classified and confirms a resize itself never touches the wheel.

@history_item n=14 marker=2026-10-08-first-live-test-and-self-drive
**2026-10-08-first-live-test-and-self-drive** (seven terminal reply shapes) — added `4;n;n t`, `6;n;n t`, `8;n;n t`, `48;n;n;n;n t` (window/cell-size reports) and others by exact grammar, measured on foot; standing rule: the list grows only by measured shapes, each with its grammar and negative controls, a reply no terminal was measured to write not added. A resize-adjacent reply a fix might meet must follow this rule.

@history_item n=15 marker=2026-11-16-survey-key-keeps-the-wheel
**2026-11-16-survey-key-keeps-the-wheel** — decision/closed-non-editing-byte-list now records, on the founder's «Выкинуть фичу», that a key answering the CLI's session-feedback survey takes the wheel like any other byte: no CLI-state-specific exemption is built into the byte list.

@history_item n=16 marker=2026-10-04-readiness-gate-and-timing-constants
**2026-10-04-readiness-gate-and-timing-constants** — the screen model on `run`'s feed thread through a tee that writes the human's bytes first; each feed and resize under `catch_unwind`; a caught panic poisons the model until the host size changes and a poisoned model reads `input-not-ready`. Source of decision/poisoned-screen-model and decision/passthrough-never-waits-on-model — the resize is already the screen model's un-poison event.

@history_item n=17 marker=2026-10-04-confirmed-send-with-cl-1-records
**2026-10-04-confirmed-send-with-cl-1-records** — the tee → feed queue is `sync_channel(256)` + `try_send`, a dropped copy poisoning like a panic (one line per episode, a size message carrying the drop count); a paste failing after `send-issued` is `input-not-ready` with its cursor. Constrains any resize settle or read-back on the send path.

@history_item n=18 marker=2026-10-27-bounded-inputs-at-every-boundary
**2026-10-27-bounded-inputs-at-every-boundary** ([Screen Model] poisoning detail) — the poisoning episode's `parse-rejected{parser:"vt100-feed"}` detail is `panicked` for a caught panic and `oversize` for a dropped message; the same chunk set the bounded-readers census (decision/bounded-readers) every new reader must pass.

@history_item n=19 marker=2026-10-07-send-waits-out-the-paste-hint
**2026-10-07-send-waits-out-the-paste-hint** — precedent for a "settle wait" fix: `GATE_MAX_WAIT` raised to 8.5 s on the founder's priced live ruling, `QUIET_PERIOD` 300 ms, the gate re-reading wheel and running turn after its wait, `send` total 18.5 s, fake agent hold cap raised; trap: a case reading a verified literal-less screen through a clock that never advances waits forever. A resize settle would move decision/gate-timings-compiled and decision/send-time-bound the same way.

@history_item n=20 marker=2026-10-07-live-rows-and-paste-shapes-on-the-dev-host
**2026-10-07-live-rows-and-paste-shapes-on-the-dev-host** (the paste hint measured, the founder's decision) — precedent for "measured, then decided on a priced card": the hint timed live (nine timings), the founder chose the remedy only after the numbers with its price (live starts, longest block, cap) shown. Mirrors this chunk's "choice made only after the reading".

@history_item n=21 marker=2026-10-07-live-rows-and-paste-shapes-on-the-dev-host
**2026-10-07-live-rows-and-paste-shapes-on-the-dev-host** (the long-paste frame) — minted decision/no-probe-for-unrelied-shapes: a CLI shape no send relies on needs no ledger probe (founder-ruled limit on the ledger rule). Decides whether a key-after-resize reading must become a relied ledger row or can stay a recorded reading.

@history_item n=22 marker=2026-11-16-send-confirmation
**2026-11-16-send-confirmation** — minted decision/send-flight-lock: the send's post-gate section runs under one wrapper flight lock that turn and wheel moves also take, and a human key never takes the lock. A resize-related wait or read-back must not put the human's keys behind that lock.

@history_item n=23 marker=2026-11-21-mcp-calls-bounded
**2026-11-21-mcp-calls-bounded** — decision/send-time-bound now reads that on the MCP surface the 55 s call deadline bounds the driver's wait, never the send itself, because the paste write (`PasteHandle::paste`'s blocking `write_all`) has no time bound — the route CARRY `cry-7mgt7ybs` this chunk's scope names, taken only if the fix opens that write path.

@history_item n=24 marker=2026-09-26-local-linux-pre-push-gate
**2026-09-26-local-linux-pre-push-gate** — registered the `FAKE_AGENT_PUMP_DELAY_MS` seam (fake-agent builds only, capped 5 s, configures nothing) and, under Kept, left the test temp trail `viola-resize-<pid>.ndjson` unregistered as registry over-reach. Any fake-agent resize behaviour must follow the test-seam rows and not add an env seam.

@history_item n=25 marker=2026-10-19-newer-claude-cli-rows
**2026-10-19-newer-claude-cli-rows** (the fake agent's options) — the fake agent's argv options are counted by architecture's own list (thirteen, `--long-turn-screen` added), options not env; a fake-agent resize mode for a fix or rehearsal adds to that counted list.

@history_item n=26 marker=2026-11-15-mutation-and-coverage-baselines-green
**2026-11-15-mutation-and-coverage-baselines-green** — decision/root-waits-share-one-bound moved the shared root `WITHIN` bound from 7 s to 12 s; a new waiting test (e.g. waiting for the child to observe the new size) takes this bound, under the 20 s lint.

@history_item n=27 marker=2026-11-17-standing-live-rig
**2026-11-17-standing-live-rig** — landed the test-only `viola-live-rig` (AGENT_RUN_RIG unread by viola) and gave vt100 its third role, the screen the rig reads of its child (decision/vt100-screen-model, the vt100-readiness-gate except). A measuring arm in `crates/viola-e2e` would build on this rig and its screen model.

@history_item n=28 marker=2026-11-19-context-as-a-local-command
**2026-11-19-context-as-a-local-command** — the test-only except of decision/claude-first-no-adapter-trait, decision/claude-shapes-in-one-crate and decision/claude-shapes-isolated sits at its 200-character bound (spaces already given up); a rig arm that reads any new Claude shape must widen that list within the bound, with the founder's word on the widening.

@history_item n=29 marker=2026-11-18-trust-record-row-read-live
**2026-11-18-trust-record-row-read-live** — decision/run-never-reads-screen-content: the only row-text exit of the screen is read by verify alone (signature-only fixtures and the rig-only first-screen record); a read-back fix must not make `run` read screen content.

@history_item n=30 marker=2026-11-20-a-quoted-modal-never-blocks-a-send
**2026-11-20-a-quoted-modal-never-blocks-a-send** — decision/vt100-readiness-gate and decision/screen-signatures-compiled now refuse only a standing modal; contract/send-results/input-not-ready and the input-not-ready hint line name a standing modal. The latest form of the gate a resize settle or refusal would extend.

@history_item n=31 marker=2026-11-13-verify-readiness-on-the-input-box
**2026-11-13-verify-readiness-on-the-input-box** — the readiness rule reads the input box by a footer literal or its own rows (rule row ≥ 40 above the prompt row), and verify's box wait (decision/verify-box-wait-for-literal) ends only on the box, a modal literal, a poisoned screen or the deadline. The "empty input box" state the measurement must type into is defined here.

@history_item n=32 marker=2026-11-15-live-round-on-the-trimmed-product
**2026-11-15-live-round-on-the-trimmed-product** — decision/wheel-starts-with-driver now names a key the human types into the folder-trust dialog before session-start as taking the wheel; a live start that types keys must reach the empty box, never a dialog, and account for the wheel.
