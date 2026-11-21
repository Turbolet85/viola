@history_item n=1 marker=2026-10-17-mcp-server-for-drivers
**2026-10-17-mcp-server-for-drivers** (the MCP tests as landed) — set the MCP test layers this chunk extends: `crates/viola-mcp/tests/handlers.rs` drives the transport-free `viola_mcp::Tools` against a `viola_channel::Server` test wrapper with the Tokio client (decision/mcp-handlers-use-tokio-client); E2E is the rmcp client over a test-owned `viola mcp` child, stdin closed and its exit awaited, because the child-process transport killed the server 3 s after close and truncated a coverage profile; `mcp_tools.rs` Paths 2–6 compare each MCP result with the CLI `--json` less `v`; the `wait` clamp is held by `handlers_wait_sends_the_clamped_bound_and_maps_a_timeout`. Any new per-tool deadline or arg cap lands in these two suites and must keep CLI/MCP parity.

@history_item n=2 marker=2026-10-17-mcp-server-for-drivers
**2026-10-17-mcp-server-for-drivers** (harness keys and the time mechanism) — `viola-mcp` owns no timer: its `wait` bound is the pure `wait_timeout` (min(given, 50 000)), a unit case, so no `start_paused` test and no tokio `test-util` in any manifest; `mcp_tools` joined the `ci` 20 s kill line (cases 0.01–5.9 s). Trap: a compiled MCP-side deadline would be the first timer in viola-mcp; it must stay a pure function of an injected instant or be measured at a layer that does not need a paused tokio clock (decision/time-faked-through-seams, decision/wrapper-deadline-injected-clock).

@history_item n=3 marker=2026-10-17-mcp-server-for-drivers
**2026-10-17-mcp-server-for-drivers** (the CI gates) — rmcp features are asserted exactly `server` + `transport-io` by `scripts/rmcp-features.sh`, and viola-channel is held tokio-free by `scripts/channel-tokio-free.sh` / deny probe `channel-tokio-edge` (decision/channel-tokio-free-by-default, decision/viola-channel-tokio-free). Any timeout mechanism added to the MCP or channel path must not pull a new rmcp feature or a tokio edge into viola-channel.

@history_item n=4 marker=2026-10-26-tokio-rustsec-2026-0386-lock-bump
**2026-10-26-tokio-rustsec-2026-0386-lock-bump** (the tokio pin in the time-sensitive fixtures row) — restates that no manifest names tokio `test-util` and `viola-mcp` owns no timer (`wait_timeout(given)` = `min(given, 50 000)`, a unit case), so no `start_paused` test exists; the baseline a bounded-`send` deadline in the MCP adapter would change.

@history_item n=5 marker=2026-10-07-send-waits-out-the-paste-hint
**2026-10-07-send-waits-out-the-paste-hint** — the readiness gate's bound became `GATE_MAX_WAIT` 8.5 s (founder ruling), `send` re-reads human-typing and turn-running after the gate, the `verify_window_` and paste-hint kills moved in proportion with planted-hang controls both ways; standing rule: a kill line moves only for a floor that is by design. The 8.5 s gate is one leg of `send`'s longest block this chunk must sum against the client timeout (contract/readiness-gate/gate-max-wait).

@history_item n=6 marker=2026-10-18-compact-post-condition-ledger-row
**2026-10-18-compact-post-condition-ledger-row** (/compact's same-session confirmation and its own window) — `/compact` on a verified CLI waits inside `COMPACT_CONFIRM_WINDOW` (40 s, PROVISIONAL), pinned by `send_compact_confirm_window_is_its_own_bound`, beside `CONFIRM_WINDOW_FALLBACK` 10 s for every other send (decision/compact-confirm-window). This is `send`'s longest single block and the likeliest to exceed an MCP client's tool-call timeout; the window is provisional and clock-driven at the unit tier.

@history_item n=7 marker=2026-10-18-compact-post-condition-ledger-row
**2026-10-18-compact-post-condition-ledger-row** (twenty-four rows, the compact variant) — precedent for adding a relied ledger row: relied count bumped in `cli_verify` `[NN/MM]`, `STEPS_PASS` / `ROW_IDS`, `contract_ledger_probes` `ROW_IDS`, `LEDGER_ROWS` / `LIVE_PASS`, the older-stamp oracle and the no-framing / tag-turn summaries, no kill line moved. The scope's fork (a 30th row with a verify probe) would move each of these literal sites (decision/ledger-probe-contract-suite, decision/version-gate-needs-all-relied-rows).

@history_item n=8 marker=2026-10-13-paste-newline-ledger-row
**2026-10-13-paste-newline-ledger-row** — another ledger-row precedent: `LedgerRow::ALL` grew, `LEDGER_ROWS` unit-proven by `LIVE_PASS`, a live home stamped at the older count reads `row-missing` until re-stamped by `run --local-live`, and counts carry their rule (`LedgerRow::ALL`). Trap for a 30th row: every standing stamp, including the local-live home, reads unverified until re-stamped.

@history_item n=9 marker=2026-11-21-cross-session-prefix-ledger-row
**2026-11-21-cross-session-prefix-ledger-row** (stamping verifies get a test-owned runtime dir) — the latest ledger row (29) made verify read `XDG_RUNTIME_DIR`, so every stamping verify gets a test-owned owner-only runtime dir (decision/stamping-verify-test-owned-runtime-dir). A 30th-row probe that newly reads env, a config or spawns must give the same test-owned treatment and be said whole (the scope's widening boundary).

@history_item n=10 marker=2026-11-18-browser-tools-prompt-modal
**2026-11-18-browser-tools-prompt-modal** — verify's step line gained `absent` (browser-tools-prompt row alone), the summary counts counting rows, the version gate needs every relied row counting, and `run --local-live` passes on one pass or absent line per relied id (contract/verify-output/step-line, contract/verify-output/summary-line, decision/version-gate-needs-all-relied-rows, contract/local-live-suite/pass, decision/ledger-row-verify-probe). A new bound row's probe must fit this pass|absent|fail grammar.

@history_item n=11 marker=2026-11-17-trust-record-ledger-row
**2026-11-17-trust-record-ledger-row** — added the census holding every test verify start tied to a test-owned home (decision/no-verify-start-outside-test-home), the rule that test verifies run under a test-owned user home (decision/verify-under-test-owned-user-home), and that a failing or confounded live reading is recorded as read, never as verification (decision/live-failing-reading-not-verification). Binds how the client-timeout reading (strings only, no live start this phase) is recorded and any new verify probe is tested.

@history_item n=12 marker=2026-11-17-standing-live-rig
**2026-11-17-standing-live-rig** — live readings of the real CLI go only through the one committed rig `viola-live-rig` (rehearsals in CI, live starts by hand under the start cap), and the real claude is reached only locally (decision/standing-live-rig, decision/real-claude-local-only, decision/real-claude-never-in-ci). Any later live measurement of the client's tool-call timeout reuses that rig; this phase makes none.

@history_item n=13 marker=2026-11-17-supply-chain-green-and-booted-tests-end
**2026-11-17-supply-chain-green-and-booted-tests-end** (kill witness for a tied stamping verify) — measured that processes and dirs alone cannot tell a tied verify from an untied one; the witness asserts an owned run dir at the kill and reads the stamp's absence (decision/tied-verify-kill-witness). Precedent for any kill or bound witness the chunk adds around verify.

@history_item n=14 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (remove-the-guard witnesses bounded by `WITHIN`) — a remove-the-guard witness hung 27 minutes under nextest's default profile (no kill line); a control whose neutralised guard lets a product process start needs a bounded witness (decision/bounded-remove-the-guard-witness). Directly applies to the "test that can fail" for a compiled MCP bound: its red control must itself be bounded.

@history_item n=15 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (the six CLI frame verbs and the MCP frame tools under the instance check) — MCP `send`, `wait`, `last`, `answer` run `check_instance` before the liveness pre-check, refused with no frame and no line (`handlers_frame_tools_refuse_a_widened_instance_before_any_frame`; decision/mcp-frame-tools-check-instance-first). These client-side checks are the first leg of `send`'s whole path the scope bounds.

@history_item n=16 marker=2026-10-26-server-verification-before-any-frame
**2026-10-26-server-verification-before-any-frame** (the Wrapper channel's server-mismatch cases and the MCP impostor case) — `handlers_frame_tools_write_no_frame_to_a_server_that_is_not_the_recorded_wrapper` drives MCP frame tools against an impostor (a held live process as the recorded pid) through the `Tools` handle; the server check is another pre-frame leg of the MCP path (decision/server-identity-verified, decision/server-checked-before-any-frame).

@history_item n=17 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** (the IPC boundary's directory check and MCP's refusal; a test's endpoint directory is made private) — MCP frame tools refuse a loosened socket directory as `instance-unreachable` / `socket-dir-refused` with no frame (`handlers_frame_tools_refuse_a_loosened_socket_directory`); a bare `tempfile::tempdir()` endpoint dir (0755) is refused, which broke nine `handlers` cases (decision/test-endpoint-dirs-made-private, contract/mcp-frame-tools/socket-dir-refused). Any new handlers case must make its endpoint dir private.

@history_item n=18 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** (MCP's `instance-unreachable` body names its cause; lost-cause witness; a wrapper lost under an open call) — MCP's `isError` body carries the closed cause word, the CLI exit-21 document less `v`; `mcp_tools_a_wrapper_killed_under_a_parked_wait_reads_endpoint_lost` and `chaos_wait_vanish.rs` hold `endpoint-lost` (`during:"call"`) by a real kill (contract/mcp-cli-parity/exit-21, contract/mcp-unreachable-detail/*, contract/wait/instance-unreachable). A client-side timeout firing mid-wait must keep this cause mapping and parity.

@history_item n=19 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** (the error bodies' surface witnesses; the reply screen and the fault screen; `channel-response`'s closed `detail`) — `mcp_tools_error_results_hold_no_path_or_upstream_text`, `sanitised_fault`, `of_reply` / `reply_of` screening and the `-32603` `internal error` / `null` rule bind every MCP result (decision/error-payloads-sanitized, contract/mcp-reply-screen/*, contract/mcp-server/internal-fault, decision/error-writers-each-named via `tests/contract_error_bodies.rs`). A new refusal or timeout result for an over-long call must be a closed word, named in the enumeration, and carry no upstream text.

@history_item n=20 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** (`viola mcp`'s exit at a line past the cap before `initialize`) — `viola mcp` exits 1 with both streams empty and one `parse-rejected{mcp-stdin, oversize}` line, held by `serve_at_a_line_past_max_frame_before_initialize_is_the_initialize_error` and `mcp_tools_a_line_past_max_frame_before_initialize_exits_1_with_one_line` (contract/viola-mcp-exit/oversize-before-initialize).

@history_item n=21 marker=2026-10-27-bounded-inputs-at-every-boundary
**2026-10-27-bounded-inputs-at-every-boundary** (the `viola mcp` stdin line cap's tests; the bounded-reader enumeration) — `CappedLines` at `MAX_FRAME` with `capped_lines_*` and `serve_*` unit cases (each killed at 5 s) and the E2E case under an 8 s kill deadline; `tests/contract_bounded_readers.rs` census requires every external reader bounded or named (contract/mcp-stdin/line-cap, decision/mcp-stdin-line-cap, decision/bounded-readers). Precedent for bounding an MCP-side input in code with a census-backed witness; a new arg cap is the same pattern.

@history_item n=22 marker=2026-10-20-epoch-5-cleanup-ii
**2026-10-20-epoch-5-cleanup-ii** (the `Mcp.` exchange: its walk, its seed row and its replay) — recorded MCP exchanges `Mcp.<variant>.json` come only from the test-only `viola-mcp-tap` (one live `claude -p` start, captured 0600, scrubbed), walked by `mcp_fixtures` against `schemas/claude-mcp-fixture.v1.json`, replayed by `tests/mcp_recorded_exchange.rs` (decision/fixtures-recorded-by-live-verify). The only standing path for a recorded client-side MCP reading; a timeout reading taken from the installed bundles' strings is not a fixture.

@history_item n=23 marker=2026-11-15-mutation-and-coverage-baselines-green
**2026-11-15-mutation-and-coverage-baselines-green** — `WITHIN` and root waits moved 7 s → 12 s, the mutants default kill 10 s → 15 s, below cargo-mutants' 20 s floor (decision/root-wait-bound, decision/within-7s-wait-bound, decision/mutants-kill-below-cargo-mutants-timeout, decision/send-window-paste-hint-30s-kill). Any test that waits out a compiled MCP bound must sit under these kill lines or join a named class with a planted-hang control.

@history_item n=24 marker=2026-10-20-epoch-5-cleanup
**2026-10-20-epoch-5-cleanup** (the mutants profile's paste-hint class and the verify_window_ kill) — the `send_under_the_paste_hint_` class takes a 30 s mutants kill because it waits out the 8.5 s gate by design, pinned by `mutants_paste_hint_override_holds_the_class_above_the_default_kill`; each kill moved only with a planted-hang pair (decision/send-window-paste-hint-30s-kill). Trap: a test that blocks to `send`'s real bound (40 s compact window) cannot fit the default kill and must be clock-driven at unit tier instead (decision/no-test-owned-real-time, decision/e2e-asserts-only-cross-process).

@history_item n=25 marker=2026-11-18-trust-record-row-read-live
**2026-11-18-trust-record-row-read-live** (the verify_window_ kill) — the `verify_window_` class kill moved 60 s → 80 s in `ci` and `mutants`; a 30th-row probe that adds verify runs or waits moves the verify-driven timings this kill sizes.

@history_item n=26 marker=2026-11-16-wait-wake-condition
**2026-11-16-wait-wake-condition** — a harness-origin prompt-submitted never wakes a parked wait; one verify stamp per build keeps each stamped-home test under the verify-driven 20 s kill (decision/verify-driven-tests-20s-kill, decision/guarded-paths-start-stamped). Bears on `wait`'s block semantics and on cost of any new stamped-home MCP case.

@history_item n=27 marker=2026-11-16-scheduled-wake-count
**2026-11-16-scheduled-wake-count** (6. E2E Test Strategy) — added contract/turn-ended/scheduled-wakes, which reaches events.ndjson, CLI `wait --json` and MCP `wait` only; a change to MCP `wait`'s result or bound must keep that field's reach.

@history_item n=28 marker=2026-11-16-local-command-delivery
**2026-11-16-local-command-delivery** — `/mcp` confirmed only by a new stdout tag past the transcript's pre-paste end; an echo-confirmed command with no readable transcript reads unconfirmable at once with no window (decision/local-command-post-condition, contract/send-result/unconfirmable, decision/local-command-unconfirmable). These set which sends block on a window and which return at once.

@history_item n=29 marker=2026-11-19-context-as-a-local-command
**2026-11-19-context-as-a-local-command** — `/context` joined `/mcp` as stdout-tag confirmed (decision/local-command-post-condition); another windowed local-command path in `send`'s longest-block inventory.

@history_item n=30 marker=2026-11-16-send-confirmation
**2026-11-16-send-confirmation** — the claim tolerates a trailing trim (decision/send-confirmation-trailing-trim-match, contract/viola-send/records); a send whose prompt goes unclaimed waits its whole window, so this rule decides when `send` blocks to the bound.

@history_item n=31 marker=2026-11-15-api-error-turn-end
**2026-11-15-api-error-turn-end** (6. E2E Test Strategy) — after turn-ended, an API-error end included, a send is accepted with no pause or release (contract/viola-send/turn-running); part of `send`'s refusal path before any window.

@history_item n=32 marker=2026-11-14-budget-gate-removed
**2026-11-14-budget-gate-removed** (4. Unit Test Strategy; 6. E2E Test Strategy) — the budget rung left the send refusal order (decision/send-refusal-order), exit 11 is retired never reused (contract/viola-send/exit-11), and `contract/mcp-tool-result/budget-paused` was removed. Any new refusal for an over-long or capped call takes a new closed word and exit, never 11.

@history_item n=33 marker=2026-11-13-verify-readiness-on-the-input-box
**2026-11-13-verify-readiness-on-the-input-box** (4. Unit Test Strategy) — contract/readiness-gate/gate-max-wait: a quiet screen whose input box does not read up waits up to 8 500 ms then reads input-not-ready; the gate leg of `send`'s block.

@history_item n=34 marker=2026-11-20-a-quoted-modal-never-blocks-a-send
**2026-11-20-a-quoted-modal-never-blocks-a-send** (4. Unit Test Strategy) — decision/screen-signatures-only now reads compiled shapes over the compiled literals as the gate's verdict; the gate's wait/ready decision feeding `send`'s bound.

@history_item n=35 marker=2026-10-19-dialog-hold-for-the-driver
**2026-10-19-dialog-hold-for-the-driver** — the dialog hold is read once from `dialog_hold_s` (bounded, falls back to sixty), its expiry writes one `dialog-expired` that wakes `wait`, and the controls table gained the `dialog_hold_s` row (decision/dialog-hold-read-once, decision/dialog-hold-expiry, decision/dialog-timeouts-outlast-the-hold, decision/controls-not-disableable). Precedent: a configurable bound gets a controls-table row proving no setting unbounds it.

@history_item n=36 marker=2026-10-27-pinned-copy-race-and-reproducible-builds
**2026-10-27-pinned-copy-race-and-reproducible-builds** (Path 7's MCP leg on the default pair, unstamped) — `mcp_tools_path7_unverified_cli_answer` boots the default pair unstamped via `SessionOptions { unstamped: true, .. }` (contract/mcp-answer/unverified-cli, contract/mcp-tool-result/unverified-cli); the harness form any new MCP E2E case on an unstamped session follows.

@history_item n=37 marker=2026-11-20-a-new-cli-version-stamped-automatically
**2026-11-20-a-new-cli-version-stamped-automatically** — only verify writes stamps.json, a keyed start's own verify included (decision/single-writer-state-files); a 30th relied row changes what a keyed start's automatic stamp must pass.

@history_item n=38 marker=2026-10-09-epoch-3-cleanup
**2026-10-09-epoch-3-cleanup** (the typed text, the `empty-text` refusal) — `send` refuses control-character → empty-text → human-typing and on, empty-text right after control-character before anything is issued, held at client and wrapper (decision/empty-text-refused-before-the-wheel, contract/cli-send-json/empty-text, decision/paste-validated-at-both-ends); the cheap refusals ahead of the windowed path.

@history_item n=39 marker=2026-09-28-mutation-testing-to-the-epoch-boundary
**2026-09-28-mutation-testing-to-the-epoch-boundary** — no per-chunk, pre-push or CI mutation gate; mutation runs at the epoch boundary through `/andromeda-code-audit`, verdict `missed == 0`, `timeout == 0`, `unviable <= caught` (decision/mutation-testing). A compiled bound's tests must kill its mutants by the owning package's own tests (decision/mutants-tested-by-owning-package).
