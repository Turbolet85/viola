@history_item n=1 marker=2026-11-21-mcp-calls-bounded
**2026-11-21-mcp-calls-bounded** (8. Mocking: decision/time-faked-through-seams) — time is faked only through injected seams, the MCP adapter's deadline provider fired by a signal and never by waiting out the 55 s `MCP_CALL_DEADLINE`, no paused Tokio clock; the paste bound's tests must take the same route (an injected or signalled bound, never a real wait to a product deadline), and the bound's arithmetic is read against that 55 s deadline.

@history_item n=2 marker=2026-11-21-mcp-calls-bounded
**2026-11-21-mcp-calls-bounded** (1. Scope · 5. Integration · 6. E2E: + contract/instance-unreachable-cause/call-deadline, + contract/mcp-unreachable-detail/call-deadline, ~ contract/mcp-tool-result/instance-unreachable, ~ contract/mcp-tools/result-body, ~ contract/mcp-tools/instance-unreachable, ~ decision/cli-mcp-identical-payloads except) — `call-deadline` was added as an MCP-only cause word with a per-tool integration row and named exceptions to CLI/MCP parity; it is the outcome-unknown state this chunk's bound exists to pre-empt, and the precedent for how a new closed word lands across the parity, result-body and cause rows.

@history_item n=3 marker=2026-11-21-key-after-a-resize
**2026-11-21-key-after-a-resize** (5. Integration: ~ decision/key-after-resize-waits-for-size except; 6. E2E: + decision/kr-after-key-settle, + decision/kr-late-redraw-not-dirty-box, + decision/refusal-rehearsal-reads-own-effects, + decision/kr-seams-rehearse-only) — the previous chunk made no viola-pty edit, so its carry (the `CHILD_WITHIN` comment in `crates/viola-pty/src/lib.rs` naming a 10 s mutants kill, now 15 s) falls to this chunk if it edits viola-pty; its rehearse-only seams and the refusal-rehearsal rule (a refusal is rehearsed by reading its own effects, not the shared teardown check) bear on rehearsing a refused paste.

@history_item n=4 marker=2026-11-15-mutation-and-coverage-baselines-green
**2026-11-15-mutation-and-coverage-baselines-green** (3. Harness: ~ decision/root-wait-bound, ~ decision/within-7s-wait-bound, ~ decision/mutants-kill-below-cargo-mutants-timeout, ~ decision/send-window-paste-hint-30s-kill why) — `WITHIN` and the root wait bound moved 7 s → 12 s and the mutants default kill 10 s → 15 s, below cargo-mutants' 20 s floor, a stated departure from «a slow case is fixed by removing slow work, never by raising the default»; any wait this chunk's tests add must sit under these lines, and the `CHILD_WITHIN` comment carry stems from this move.

@history_item n=5 marker=2026-10-07-send-waits-out-the-paste-hint
**2026-10-07-send-waits-out-the-paste-hint** — the readiness gate's `GATE_MAX_WAIT` became 8.5 s; `send` reads human-typing then turn-running a second time after the gate returns ready; `send_under_the_paste_hint_a_human_key_during_the_gate_wait_wins` (exit 10, nothing typed, key reaches the child) is a floor that cannot tell the two wheel reads apart, the refusal at the bound unit-tier only; standing rule: a kill line moves only for a by-design floor with a planted-hang control both ways. The gate bound is one term of the paste bound's arithmetic, and the human-key-during-a-wait case is the model for a human key while the paste is blocked.

@history_item n=6 marker=2026-10-20-epoch-5-cleanup
**2026-10-20-epoch-5-cleanup** (the mutants profile's paste-hint class) — `send_under_the_paste_hint_` cases got their own 30 s mutants override beside `send_window_`, each pinned with planted-hang TIMEOUT/PASS controls and `mutants_paste_hint_override_holds_the_class_above_the_default_kill`; a test that waits out a paste bound by design lands in, or beside, these classes and must carry the same two-sided control, never a raised default.

@history_item n=7 marker=2026-10-18-compact-post-condition-ledger-row
**2026-10-18-compact-post-condition-ledger-row** (/compact's own window) — `COMPACT_CONFIRM_WINDOW` (40 s, own bound, `send_compact_confirm_window_is_its_own_bound`) beside the 10 s fallback window; the largest confirm window is a term of `send`'s compiled total that the paste bound plus the rest must keep under `MCP_CALL_DEADLINE`.

@history_item n=8 marker=2026-10-06-local-command-send-outcomes
**2026-10-06-local-command-send-outcomes** — a local command is never presumed delivered (`send_window_local_command_is_not_presumed_delivered`), and the forced-window pair `send_under_the_paste_hint_on_a_verified_cli` drives the fake agent's `--paste-hint-ms` hold; the fake agent's test-only hold options are the precedent for the measuring child that stops reading its PTY, and a cut paste likewise must never read delivered.

@history_item n=9 marker=2026-11-16-send-confirmation
**2026-11-16-send-confirmation** (4. Unit: − decision/send-confirmation-exact-match → + decision/send-confirmation-trailing-trim-match; 6. E2E: ~ contract/viola-send/records; 7. Fixtures: ~ contract/fake-agent-input/bracketed-paste) — confirmation tolerates only a trailing trim (ratified 2026-11-16T06-05-33-wrap; the why read live at 2026-11-17-standing-live-rig); a cut paste's partial text left in the box (no Enter) must be shown to claim no later send under this match.

@history_item n=10 marker=2026-11-16-local-command-delivery
**2026-11-16-local-command-delivery** (1. Scope: ~ contract/driver-send/local-command, ~ decision/local-command-post-condition, ~ contract/send-result/unconfirmable; 4. Unit: ~ decision/local-command-unconfirmable) — local commands classified by name and confirmed only by post-condition (transcript stdout tag for /mcp, later /context), unconfirmable otherwise; the rows a cut paste of a local command must still respect (ratified 2026-11-17T05-21-35Z-wrap).

@history_item n=11 marker=2026-11-14-budget-gate-removed
**2026-11-14-budget-gate-removed** (4. Unit: ~ decision/send-refusal-order; 6. E2E: ~ contract/viola-send/exit-11) — the refusal order lost its budget rung and exit 11 is retired, never renumbered or reused; a new closed word at the paste bound must take its place in `decision/send-refusal-order` and an exit in 10–14 without reviving 11.

@history_item n=12 marker=2026-10-09-epoch-3-cleanup
**2026-10-09-epoch-3-cleanup** (the typed text and the `empty-text` refusal) — `empty-text` was inserted into the send order (control-character → empty-text → human-typing …) with five unit cases and two cross-process cases; the precedent for slotting a new refusal rung and naming its cases at each tier.

@history_item n=13 marker=2026-10-09-inner-cr-and-crlf-in-a-sent-text
**2026-10-09-inner-cr-and-crlf-in-a-sent-text** — the typed text (every CR LF and CR as one LF, trailing CR/LF dropped) is what is pasted and matched (`decision/sent-text-typed-form`); the bytes a bounded write cuts are the typed text's bracketed-paste frame, and what a cut leaves is read against that form.

@history_item n=14 marker=2026-10-04-the-wheel
**2026-10-04-the-wheel** — Path 5 landed (`tests/tui_wheel.rs`, `tests/cli_wheel.rs`): human-typing refusals, hints without `release`, focus/mouse/resize never taking the wheel; the chunk's «wheel unmoved» at the bound and «what the human's keys do while a paste is blocked» are asserted on these suites' records.

@history_item n=15 marker=2026-10-04-running-turn-refusal
**2026-10-04-running-turn-refusal** — every test that sends twice must end the first turn with a scripted Stop; `turn-running` became a control negative in `cli_controls_not_disableable.rs`; the fake agent quiesces its hooks before exit — constraints on a test where a send after a cut paste meets the next send.

@history_item n=16 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** — the closed causes (`viola_core::UnreachableCause`), `tests/cross_exit_causes.rs` with a last `hint:` per cause, `no_two_codes_share_a_hint`, `no_hint_names_release`, `every_code_is_a_code_word`, the binary on the `ci` 20 s kill line, and `endpoint-lost` `during:"call"`; a new closed word at the paste bound joins this table, its hint and its code-word checks.

@history_item n=17 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** (exit 21's sixth cause) — `socket-dir-refused` was added across `viola-core`'s closed set, `src/human/hints.rs`, `cross_exit_causes.rs` (human and `--json`), revive's codes and the MCP cause list; the worked precedent for landing one new closed word on every surface.

@history_item n=18 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** — the reply screen (`reply_of` / `of_reply`: a refusal that is no code word reads `unknown`, a free-text detail `null`), the surface witnesses and `tests/contract_error_bodies.rs`'s census of writers; the paste bound's word must be a code word the screen passes and a named writer in the enumeration, never upstream text.

@history_item n=19 marker=2026-10-27-bounded-inputs-at-every-boundary
**2026-10-27-bounded-inputs-at-every-boundary** — `tests/contract_bounded_readers.rs` (`BOUNDARIES`, `EXCEPTIONS`, a census proven on a planted hit) holds every external read to a cap; the precedent for an enumeration that no unbounded call slips past, if the bounded write is to be held the same way.

@history_item n=20 marker=2026-09-26-local-linux-pre-push-gate
**2026-09-26-local-linux-pre-push-gate** (Module ↔ PTY) — the pump's resize baseline fix was witnessed by a forced-window test through the `fake-agent`-only `FAKE_AGENT_PUMP_DELAY_MS` seam, 6/6 red before and green after; the precedent for driving `crates/viola-pty/src/pump.rs` timing from the fake agent.

@history_item n=21 marker=2026-10-27-pinned-copy-race-and-reproducible-builds
**2026-10-27-pinned-copy-race-and-reproducible-builds** (8. Mocking: forced-window test parameters) — a forced-window hook passed to a private function is a test parameter, not a seam: no env read, no feature, no mockall double, nothing added to the seam list (`decision/forced-window-hook-is-test-parameter`); binds how a paste-write bound may be forced in a unit test.

@history_item n=22 marker=2026-11-13-rows-proposal-rule-work
**2026-11-13-rows-proposal-rule-work** (5. Integration: ~ decision/pty-tested-on-real-openpty) — PTY integration spawns the fake agent on a real openpty and reads a resize from the child's size report alone with no key after it; viola-pty tests of the bounded write run on the same real openpty.

@history_item n=23 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (the PTY seam on openpty alone; OS scope; the OS-gate census reads another OS as a fault) — viola-pty is openpty only (the ConPTY and `TerminateProcess` paths gone), the product Linux only, and a gate naming windows or macos is a census fault; the bounded write is a Linux openpty behaviour with no per-OS arm.

@history_item n=24 marker=2026-11-06-version-done-check
**2026-11-06-version-done-check** (the portable-pty pin at =0.9.0) — viola-pty's seam sits over portable-pty `=0.9.0`, whose writer is the blocking `write_all` the carry names at `pump.rs:77`; the write contract this chunk bounds is that crate's.

@history_item n=25 marker=2026-10-13-epoch-4-survivors
**2026-10-13-epoch-4-survivors** — viola-pty's `PortablePty::resize` mutant read missed only because two mutation jobs shared one report file; report files now carry the test process's id. A trap for any new viola-pty test that writes a shared report file.

@history_item n=26 marker=2026-09-28-mutation-testing-to-the-epoch-boundary
**2026-09-28-mutation-testing-to-the-epoch-boundary** — no per-chunk, pre-push or CI mutation gate; mutation runs at the epoch boundary and on demand only; consistent with the scope's «no mutation run», yet every new wait must still sit under the mutants kill lines.

@history_item n=27 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (remove-the-guard witnesses bounded by `WITHIN`) — a control whose neutralised guard lets a product process start needs a bounded witness (an unbounded one hung 27 minutes under nextest's default profile); a remove-the-bound control for the paste write is exactly such a witness.

@history_item n=28 marker=2026-11-17-booted-wrapper-does-not-outlive-its-test
**2026-11-17-booted-wrapper-does-not-outlive-its-test** (5. Integration: ~ decision/wrapper-drop-guard, + decision/no-untied-viola-spawn, + decision/tied-child-spawn-thread) — every long-lived viola child of a root test is tied to its test thread by a parent-death kill, held by a census; a wrapper whose paste blocks on a non-reading child must be spawned tied, on a thread that outlives it.

@history_item n=29 marker=2026-11-18-session-folder-trusted-by-viola
**2026-11-18-session-folder-trusted-by-viola** (3. Harness: + decision/run-under-test-owned-user-home) — every test-started `viola run` and `revive` runs under a test-owned user home, since a start writes the Claude config; the scope's «every test start runs under a test-owned user home».

@history_item n=30 marker=2026-11-16-wait-wake-condition
**2026-11-16-wait-wake-condition** (5. Integration: ~ decision/fresh-home-per-test; 3. Harness: + decision/one-verify-stamp-per-build) — a stamped home is a copy of its build's one verify stamp, never a link; the stamped-home setup any verified-CLI send case here starts from.

@history_item n=31 marker=2026-11-21-cross-session-prefix-ledger-row
**2026-11-21-cross-session-prefix-ledger-row** (3. Harness: + decision/stamping-verify-test-owned-runtime-dir) — each stamping verify gets a test-owned runtime dir; a new stamped-home helper or case must keep it.

@history_item n=32 marker=2026-10-04-confirmed-send-with-cl-1-records
**2026-10-04-confirmed-send-with-cl-1-records** — the `paste_text` fuzz target and the 512-case `validate_paste_text` property landed, and the chaos home carve-out (F4): `tests/chaos_feed_panic.rs` alone boots outside G2, G4 and the secret scan; a chaos-style case with a child that stops reading its PTY must not join that carve-out.

@history_item n=33 marker=2026-10-07-test-homes-off-the-contended-volume
**2026-10-07-test-homes-off-the-contended-volume** — standing rule: a stalled-start red is a finding about the backing, stopped on and reported, never re-run for green, no kill, `WITHIN` or retry moved; matches the directive's «a gate red with no failed assertion is read against `hostwatch.py` first and never re-run for green».

@history_item n=34 marker=2026-11-17-standing-live-rig
**2026-11-17-standing-live-rig** (6. E2E: − decision/live-readings-not-a-test-layer → + decision/standing-live-rig; 7. Fixtures: fake-agent trust dialog, session-start, exit-triggers, keystroke rows; 1. Scope: ~ decision/real-claude-local-only) — live readings run only through the one committed rig under the founder's start cap; the fake agent exits at stdin EOF and draws its box on keys. This chunk makes no live or rig start (a real start is a fork), and its non-reading fake behaviour sits beside these fake-agent input/exit rows.

@history_item n=35 marker=2026-11-19-a-live-stopfailure-payload
**2026-11-19-a-live-stopfailure-payload** (7. Fixtures: + contract/fake-agent-cli/turn-fail, + contract/fake-agent-cli/settings-hooks) — each new fake-agent argv option lands as its own `contract/fake-agent-cli/*` row (argv only, no env); the option that makes the fake agent stop reading its PTY owes the same row.

@history_item n=36 marker=2026-10-17-mcp-server-for-drivers
**2026-10-17-mcp-server-for-drivers** — MCP handlers are tested through the transport-free `viola_mcp::Tools` against a `viola_channel::Server` test wrapper, MCP stdio tests own their `viola mcp` child, and each MCP step is compared with the CLI's `--json` less `v`; where the MCP `send` tool's result at the bound is held.

@history_item n=37 marker=2026-11-13-verify-readiness-on-the-input-box
**2026-11-13-verify-readiness-on-the-input-box** (4. Unit: ~ contract/readiness-gate/gate-max-wait) — a quiet screen whose input box does not read up waits up to 8 500 ms and reads `input-not-ready`; the gate term of the bound's arithmetic, and what the next send meets over a box holding cut paste text.

@history_item n=38 marker=2026-11-20-a-quoted-modal-never-blocks-a-send
**2026-11-20-a-quoted-modal-never-blocks-a-send** (4. Unit: ~ decision/screen-signatures-only) — the gate reads only compiled signatures (literals and their compiled shapes); a box left holding cut paste text is read through these signatures alone, never by screen text.

@history_item n=39 marker=2026-11-19-a-fired-scheduled-wake-keeps-the-wheel
**2026-11-19-a-fired-scheduled-wake-keeps-the-wheel** (6. E2E: + decision/fired-wake-keeps-wheel) — a keyless human-filed prompt is relabelled harness only when no stdin key set the typed mark; a human key typed while a paste is blocked sets that mark, so the chunk's human-key disposition reads against this rule.

@history_item n=40 marker=2026-11-15-live-round-on-the-trimmed-product
**2026-11-15-live-round-on-the-trimmed-product** (6. E2E: ~ contract/event-log/start-wheel-record, + decision/start-lines-read-from-start-offset) — a start may log a human-input wheel record before session-start, and a start's lines are read from its own offset; wheel assertions in this chunk's cases must read from the case's own cursor.

@history_item n=41 marker=2026-10-25-windows-endpoint-admission
**2026-10-25-windows-endpoint-admission** (scoped log captures through `with_scoped_default`) — every scoped log capture in the root bin's unit tests goes through the pinned-dispatch helper (`decision/scoped-capture-pinned-dispatch`); a unit case asserting the bound's log line must use it.
