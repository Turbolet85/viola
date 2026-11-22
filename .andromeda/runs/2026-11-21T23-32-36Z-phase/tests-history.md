@history_item n=1 marker=2026-09-25-pty-wrapper-on-windows
**2026-09-25-pty-wrapper-on-windows** (11-name identity floor, no-EOF measured per OS, harness on outer PTYs, fake-agent receipts) — the fake agent's receipt kinds gained `size`, `cwd`, `hold`, and the harness `supervise` took one outer PTY per instance; this is where the `size` receipt that every resize test (and the "key after a resize waits for the size" rule) reads was born, and where `booted_wrapper` first waited on the `start` receipt before any key.

@history_item n=2 marker=2026-09-26-local-linux-pre-push-gate
**2026-09-26-local-linux-pre-push-gate** (pump resize baseline) — Module ↔ PTY: a resize landing between the spawn sizing and the pump's first look is propagated because the pump's baseline is the spawn size, witnessed by a forced-window test through the `fake-agent`-only `FAKE_AGENT_PUMP_DELAY_MS` seam (6/6 red before, 6/6 green after). The origin of `decision/resize-baseline-is-spawn-size`; any change to the resize path or a settle wait must keep this witness green and reuse the seam rather than invent a timer.

@history_item n=3 marker=2026-09-29-h2-conpty-resize-probe
**2026-09-29-h2-conpty-resize-probe** (Module ↔ PTY: a key after a resize waits for the observed size, and the H2 limit) — THE source of this chunk's claim: a key written right after a resize returned was lost in 13 of 200 windows-2025 runner iterations (ci#36527891850), resize seen, key flushed, lost between the ConPTY input-pipe write and the Rust test child's console read, cause not established; the red test `spawn_runs_a_raw_child_that_sees_its_size_a_resize_and_its_own_exit_code` was reshaped to wait for its `size 120x40` line (never a timer, retry or skip, §10 zero-flakiness) and `spawn_reports_a_resize_to_a_child_that_reads_no_key` added. Whether real `claude` loses such a key was recorded as unmeasured — a Windows ConPTY hypothesis, never a Linux reading; it is the guard of `decision/key-after-resize-waits-for-size`.

@history_item n=4 marker=2026-09-29-verify-stamped-test-homes-and-harness
**2026-09-29-verify-stamped-test-homes-and-harness** (fake-agent default 2.1.283; perf session stamped; H2 owner re-named) — the Module ↔ PTY row's H2 owner moved to the route entry "First live test and self-drive", and `run --local-live` was stated not to claim it: the first hop of the owner chain that ends at this chunk.

@history_item n=5 marker=2026-09-29-sideloaded-conpty
**2026-09-29-sideloaded-conpty** (the H2 pair) — a with/without pair on one windows-2025 run: sideloaded ConPTY 0 of 200 lost, inbox 14 of 200 (ci#36563868040), and the forced-window resize test resizes on the wrapper's `process-start{claude-child}` line. Evidence that the measured loss sat in the inbox ConPTY, not in viola or a Linux PTY — bears on whether a Linux loss is expected at all.

@history_item n=6 marker=2026-09-29-fake-agent-drift-contract
**2026-09-29-fake-agent-drift-contract** (change-driven size) — the fake agent's `size` receipt became watcher-driven (written at once, then polled every 10 ms, written on each change, no key needed, interactive mode only); the earlier key-driven sampling left a resize with no later key unreceipted (the H2 CARRY), witnessed red by `pty_resize_reaches_a_child_that_reads_no_key`. Any fake-agent resize/input change this chunk makes extends `contract/fake-agent-receipt/size` and must keep that witness.

@history_item n=7 marker=2026-10-04-readiness-gate-and-timing-constants
**2026-10-04-readiness-gate-and-timing-constants** — the vt100 feed property `screen::tests::feed_prop_a_caught_panic_poisons_until_resize` landed (a caught feed panic poisons the screen model until a resize) with the `vt100_feed` fuzz target; a resize is part of the screen model's recovery contract, so a resize-path change must not break it (`decision/vt100-feed-panic-safe`).

@history_item n=8 marker=2026-10-04-confirmed-send-with-cl-1-records
**2026-10-04-confirmed-send-with-cl-1-records** (the second test-data carve-out: the chaos home) — `tests/chaos_feed_panic.rs` alone boots in `TestHome::outside_scan()`, outside G2/G4/the secret scan, because its forced vt100 panic writes a counted panic line; the one sanctioned exception if the chunk touches the feed-panic/passthrough path.

@history_item n=9 marker=2026-10-04-the-wheel
**2026-10-04-the-wheel** (Path 5 as landed) — `tests/tui_wheel.rs` holds the focus/mouse/resize case (resize never takes the wheel), the outer-PTY steps and the `^Z` case; a key typed after a resize is human input, so any re-type or refusal option must keep `decision/focus-mouse-resize-keep-wheel` and the human-key-takes-the-wheel rows intact.

@history_item n=10 marker=2026-10-04-running-turn-refusal
**2026-10-04-running-turn-refusal** (Path 5 and Path 2 span the running turn) — a running turn refuses `send` exit 13 `turn-running`, so every test that sends twice must end the first turn with a scripted Stop (the fake agent's interactive submit fires none); a trap for any send-after-resize case this chunk writes.

@history_item n=11 marker=2026-10-07-t05-47-07-wrap
**2026-10-07-t05-47-07-wrap** (H2's real-`claude` question re-pointed) — whether real `claude` loses a key typed right after a resize moved to the route entry "Windows-only live measurements", blocked on an interactive Windows host; the second hop of the owner chain, and the record that the question was always framed for Windows.

@history_item n=12 marker=2026-10-07-send-waits-out-the-paste-hint
**2026-10-07-send-waits-out-the-paste-hint** — the gate's bound became `GATE_MAX_WAIT` 8 500 ms; `send` reads human-typing then turn-running again after the gate returns ready; `send_under_the_paste_hint_a_human_key_during_the_gate_wait_wins` (exit 10, nothing typed, key reaches the child). Standing rule from it: a kill line moves only for a floor that is by design, with a planted-hang control recorded both ways — binding on any settle wait this chunk adds to send's path.

@history_item n=13 marker=2026-10-08-first-live-test-and-self-drive
**2026-10-08-first-live-test-and-self-drive** (Path 5 records the classifier's tables; the real CLI has two local routes) — the classifier's terminal-reply reading on a real terminal is a local live measurement, not a suite (`decision/terminal-reply-reading-live-measured`), and a route entry's live readings of a real session are that chunk's own local measurement, never CI. The precedent for how this chunk's "measured" answer is recorded.

@history_item n=14 marker=2026-10-09-epoch-3-cleanup
**2026-10-09-epoch-3-cleanup** (the typed text, the `empty-text` refusal) — `send` refusal order became control-character → empty-text → human-typing → …; a read-back-and-re-type or a new refusal option would extend `decision/send-refusal-order` and `decision/sent-text-typed-form`, whose cases are named here.

@history_item n=15 marker=2026-10-09-inner-cr-and-crlf-in-a-sent-text
**2026-10-09-inner-cr-and-crlf-in-a-sent-text** — the typed text holds no CR (every CR LF and lone CR one LF, trailing CR/LF dropped), pinned by `typed_text_drops_every_trailing_newline` / `typed_text_every_inner_cr_is_one_lf`; a re-typed key or text must be the same typed form, and a fake-agent green proves an outcome only by reading the typed bytes.

@history_item n=16 marker=2026-10-13-epoch-4-survivors
**2026-10-13-epoch-4-survivors** (the resize miss recorded) — viola-pty's `PortablePty::resize` → `Ok(())` read missed under two cargo-mutants jobs sharing one `TMPDIR`, while its three resize tests run alone under that mutant read red; the report files now carry the test process id. A trap if this chunk adds or reshapes resize tests: they must kill that mutant by an observed size, not a shared file.

@history_item n=17 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (remove-the-guard witnesses bounded by `WITHIN`) — a control whose neutralised guard lets a product process start needs a bounded witness (an unbounded one hung 27 min under nextest's default profile); applies to any remove-the-guard control for a resize fix.

@history_item n=18 marker=2026-10-27-bounded-inputs-at-every-boundary
**2026-10-27-bounded-inputs-at-every-boundary** (the feed-panic passthrough witnessed on the outer terminal) — `chaos_feed_panic_refuses_send_and_keeps_passthrough`: send exits 13 `input-not-ready`, a typed key reaches the child, the outer terminal is byte-searched for the child's own bytes and no viola literal — the one sanctioned outer-terminal read (`decision/vt100-panic-contained`, `decision/no-child-screen-assertions`' except).

@history_item n=19 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** (a test's endpoint directory is made private) — a test placing an endpoint in a temp dir makes it through `viola_state::fs::create_private_dir`, never a bare `tempfile::tempdir()` (0755, refused by `check_endpoint_dir`; nine `handlers` cases failed on it): the origin of `decision/private-dir-for-test-endpoints`.

@history_item n=20 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (the OS scope; the PTY seam on openpty alone) — Linux only, one `ubuntu-latest` leg, the Windows/macOS code out of the tree at `multi-os-final` (`ecbe7ab3`); Module ↔ PTY reads real openpty spawns with "a key after a resize sent once the child reports the size", and the windows-2025 key-loss readings, the H2 limit and the sideload boundary LEFT the plan. So no Linux reading of a lost key has ever been recorded — the claim this chunk measures is carried from a retired Windows hypothesis.

@history_item n=21 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (the relied set, all twenty-four rows on every build) — `run --local-live`: the H2 key-after-resize question was moved to the open residuals of origin `2026-11-04T23-11-43-wrap`, target `another-os` (was "Windows-only live measurements"); the third hop of the owner chain.

@history_item n=22 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (the OS-gate census reads another OS as a fault) — `contract_os_gated_tests` faults a gate naming `windows` or `target_os = "macos"` even when named; any test this chunk gates must be a named `GATES` row with its reason (`decision/os-gated-tests-each-named`, `decision/linux-only`).

@history_item n=23 marker=2026-11-06-version-done-check
**2026-11-06-version-done-check** (the portable-pty pin at =0.9.0) — the `viola-pty` seam (spawn, read, write, resize, wait, kill) moved to portable-pty `=0.9.0`, `pty_backend()` still `openpty`; the resize call this chunk inspects is 0.9.0's, and the outer-PTY driver and supervisor run on the same pin.

@history_item n=24 marker=2026-11-06-version-done-check
**2026-11-06-version-done-check** (the always-admit survivor as 0.1.0's known limit) — the one standing mutation survivor (a seam's peer-admitted read replaced by always-admit) is the version's known limit read at each epoch-boundary audit (`decision/mutation-run-verdict` except); with no mutation run in this chunk, any new resize/settle code is scored only at the Epoch 4 boundary.

@history_item n=25 marker=2026-11-08T20-32-52-wrap
**2026-11-08T20-32-52-wrap** (version census 0.2.0) — `--local-live`: the H2 key-after-resize question moved from the open residuals of target `another-os` to the route entry «Key after a resize» (C-288): the census row that created this chunk's ownership of the question.

@history_item n=26 marker=2026-11-13-verify-readiness-on-the-input-box
**2026-11-13-verify-readiness-on-the-input-box** (4. Unit Test Strategy; 7. Test Data & Fixtures) — `contract/readiness-gate/gate-max-wait` changed: the input box reads up on its own rule and prompt rows with no footer literal; `decision/screen-fixture-box-rows-as-literals` added. A redraw after a resize (the CLI's input-box repaint) is read through this gate rule and these box-row fixtures.

@history_item n=27 marker=2026-11-13-rows-proposal-rule-work
**2026-11-13-rows-proposal-rule-work** (5. Integration Test Strategy) — `decision/pty-tested-on-real-openpty` changed from "a key is written after a resize only once the child reports the new size" to "a resize is read from the child's own size report alone, with no key written after it": the resize test now writes no key after the resize. This chunk's row pair (`decision/key-after-resize-waits-for-size` vs `decision/pty-tested-on-real-openpty`) must be reconciled against it.

@history_item n=28 marker=2026-11-15-live-round-on-the-trimmed-product
**2026-11-15-live-round-on-the-trimmed-product** — `contract/event-log/start-wheel-record` widened: where folder-trust keys pass through the wrapper a `wheel{human, human-input}` record precedes session-start, and `decision/start-lines-read-from-start-offset` added (a reused instance name keeps earlier starts' lines at the log head). A live measuring start typing keys reads its own lines from its own offset and expects the trust-key wheel line.

@history_item n=29 marker=2026-11-15-mutation-and-coverage-baselines-green
**2026-11-15-mutation-and-coverage-baselines-green** (3. Test Harness Contract) — `decision/root-wait-bound` and `decision/within-7s-wait-bound` moved to `WITHIN` = 12 s, the mutants default kill to 15 s (`decision/mutants-kill-below-cargo-mutants-timeout`), all below cargo-mutants' 20 s floor; the `send-window-paste-hint-30s-kill` why lost its 10 s literal. Any waiting test or settle bound this chunk adds sits under these (the scope's 20 s lint), and the `CHILD_WITHIN` comment carry (`cry-m267ghbx`, «10 s kill» now 15 s) stems from this move.

@history_item n=30 marker=2026-11-15-api-error-turn-end
**2026-11-15-api-error-turn-end** (7. Test Data & Fixtures) — `decision/send-cases-copy-recorded-fixtures` added: a purely synthetic fixtures root draws no input box, so any send after the first turn reads `input-not-ready`; a send-after-resize case must start from a copy of the recorded set.

@history_item n=31 marker=2026-11-16-send-confirmation
**2026-11-16-send-confirmation** — `decision/send-confirmation-exact-match` replaced by `decision/send-confirmation-trailing-trim-match` and `contract/viola-send/records` widened; a read-back-and-re-type fix decides confirmation against this match.

@history_item n=32 marker=2026-11-17-standing-live-rig
**2026-11-17-standing-live-rig** (6. E2E Test Strategy; 7. Test Data & Fixtures; 1./2./11. excepts) — `decision/live-readings-not-a-test-layer` replaced by `decision/standing-live-rig`: every later live round reuses the committed `viola-live-rig` (rehearsals in CI, live starts by hand under the founder's start cap, each ledgered); the fake agent's keystroke now draws a box rule and prompt row and DEL erases (`contract/fake-agent-input/keystroke`, which meets carry `cry-lqe3bfwx`); the rig reads the child's screen only into closed words and passes a read trust dialog under its key grant. The «measured» reading this chunk prices is a rig arm under these rows.

@history_item n=33 marker=2026-11-17-standing-live-rig
**2026-11-17-standing-live-rig** (4. Unit Test Strategy) — `decision/send-confirmation-trailing-trim-match`'s why: the CLI's submit trim was read live by the rig (trailing spaces/TAB removed, leading/inner runs and inner LF kept) — the precedent of a rig reading closing an assumption.

@history_item n=34 marker=2026-11-17-booted-wrapper-does-not-outlive-its-test
**2026-11-17-booted-wrapper-does-not-outlive-its-test** — every long-lived viola child a root test spawns is tied to its test thread by a parent-death kill (`decision/wrapper-drop-guard`), with `decision/no-untied-viola-spawn` (census) and `decision/tied-child-spawn-thread`; any new PTY/resize test that starts `viola run` must use the tied builder.

@history_item n=35 marker=2026-11-17-supply-chain-green-and-booted-tests-end
**2026-11-17-supply-chain-green-and-booted-tests-end** (5. Integration Test Strategy) — `decision/no-untied-harness-boot` added: no viola-e2e test boots a harness session untied, a census proven on a planted boot; binds a rig or e2e measuring arm.

@history_item n=36 marker=2026-11-17-trust-record-ledger-row
**2026-11-17-trust-record-ledger-row** — `decision/no-verify-start-outside-test-home` (census of every test verify start tied to a test-owned home), `decision/verify-under-test-owned-user-home`, and `decision/live-failing-reading-not-verification` (a failing or confounded live reading is recorded as read, never as verification, the row left unverified on that version). The last binds how a mixed or failing key-loss reading is recorded.

@history_item n=37 marker=2026-11-18-session-folder-trusted-by-viola
**2026-11-18-session-folder-trusted-by-viola** (3. Test Harness Contract) — `decision/run-under-test-owned-user-home` and `decision/harness-wrappers-under-session-user-home`: a start writes the Claude config, so every test-started `viola run`/`revive` runs under a test-owned user home — the scope's "every test start under a test-owned user home".

@history_item n=38 marker=2026-11-18-browser-tools-prompt-modal
**2026-11-18-browser-tools-prompt-modal** — a ledger row may end `absent` (browser-tools-prompt alone); `decision/version-gate-needs-all-relied-rows`, `contract/local-live-suite/pass` and `decision/ledger-row-verify-probe` read pass/absent/fail. If the answer becomes a relied row, these are the rows its probe and the version gate extend.

@history_item n=39 marker=2026-11-19-a-live-stopfailure-payload
**2026-11-19-a-live-stopfailure-payload** (6. E2E Test Strategy; 1./2./11. excepts) — `decision/standing-live-rig` gained the SF arm's except (rehearsals unwrapped in CI, contained rehearsal local, skipping by its printed sandbox probe); the rig's screen reading widened to the box row's cell attributes, never text (`decision/no-child-screen-assertions`, `decision/never-parse-rendered-child-screen`, `decision/no-screen-parsing`, `decision/discipline-at-every-layer`). The precedent for adding a measuring arm and the closed-word vocabulary that may tell a lost key from a kept one.

@history_item n=40 marker=2026-11-19-context-as-a-local-command
**2026-11-19-context-as-a-local-command** (6. E2E Test Strategy) — `decision/live-rig-hint-cell-reading` added: the rig's hint-cell reading is recorded as a reading, never verification, a widening inside the founder's «2 клавиши: x, DEL» — the x-and-DEL rehearsal behind carry `cry-234cbfoa` (`rig_rehearsal_reads_the_box_empty_after_x_and_del`).

@history_item n=41 marker=2026-11-19-a-fired-scheduled-wake-keeps-the-wheel
**2026-11-19-a-fired-scheduled-wake-keeps-the-wheel** (6. E2E Test Strategy) — `decision/rig-no-key-read-from-keys-word`: a rig rehearsal reads no key typed from the rig's own keys word, never the shared fake-agent receipt (which carries send's submit Enters as key lines); `decision/fired-wake-keeps-wheel`; the hint-cell reading extended to a fresh box before any send. Directly binds how a key-after-resize arm counts keys typed versus kept.

@history_item n=42 marker=2026-11-20-a-quoted-modal-never-blocks-a-send
**2026-11-20-a-quoted-modal-never-blocks-a-send** — `decision/screen-signatures-only` now reads compiled signatures (literals and their compiled shapes) as the gate's verdict, and the rig's screen excepts gained "their compiled shapes" (`decision/no-child-screen-assertions`, `decision/never-parse-rendered-child-screen`, `decision/discipline-at-every-layer`, `decision/no-screen-parsing`); the closed vocabulary a post-resize redraw is read through.

@history_item n=43 marker=2026-11-21-cross-session-prefix-ledger-row
**2026-11-21-cross-session-prefix-ledger-row** — `contract/fake-agent-receipt/start` carries pid and the print prompt; `contract/fake-agent-hooks/stdin-payload` and `decision/fake-agent-replays-recorded-bytes` gained the sender's synthetic hooks; `decision/stamping-verify-test-owned-runtime-dir` added (each stamping verify gets a test-owned runtime dir). The latest state of the fake-agent rows a resize/input change would extend, and the stamping rule any new stamped test inherits.

@history_item n=44 marker=2026-11-21-mcp-calls-bounded
**2026-11-21-mcp-calls-bounded** (8. Mocking & Stubbing Discipline) — `decision/time-faked-through-seams`: time is faked only through injected seams (the `viola_core` Clock, the MCP deadline provider fired by a signal), no mock_instant, no paused Tokio clock, a pure decision taking its instant as an argument. A settle-wait fix must be clock-driven at the unit tier, never a test sleep (`decision/no-test-owned-real-time`, `decision/no-sleep-synchronization`).
