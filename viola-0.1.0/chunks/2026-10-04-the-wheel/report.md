# Report — 2026-10-04-the-wheel

**Chunk:** The wheel — a human editing key takes it, focus/mouse/resize never do, human bytes held only during the paste, harness turns ignored, viola pause / viola release, human-typing / manual-pause refusals ahead of turn-running, null to a pending dialog on a wheel move, release-from-driver
**Date:** 2026-10-04T20:44Z
**Commits:** (since `last_wrap` 2026-10-04T17:20Z; basis `git log --format='%h %s' eb53a58..HEAD`)
- `7f42d6f` chore(2026-10-04-the-wheel): operator pre-CI commit, for the run this chunk's verdict reads
- `5c6101c` fix(…): operator fix after CI run 37225394452 — the human prompt's wheel move ahead of its line, the fake agent's console read without std's ^Z strip, the focus wait naming its keys, the exit flush while the terminal is raw
- `126e921` fix(…): operator fix after CI run 37226294797 — the focus case asks the wheel after each step, focus reports swallowed on Windows
- `849588b` fix(…): operator fix after CI run 37226763168 — a real-PTY measurement of how the platform terminal hands the wrapper a mouse report, plain and under win32-input-mode
- `79ec57c` fix(…): operator fix after CI run 37227518624 — the founder's live ruling F-W3 pinned
- `ba36659` fix(…): operator fix after the wrap's light gate, entry 16 — the test child's read modes ride argv, not a new env read (added at the wrap's P7.1 halt; see Outcome)

## Changes (structured — detectors read this)
- **Files:** (basis `git diff --name-only eb53a58` + `gate.py scope`: changed 26 outside `.andromeda/` and the version dir · listed 22 · recorded 4)
  - new: `src/run/wheel.rs`, `src/run/snapshot.rs`, `src/cmd/pause.rs`, `src/cmd/release.rs`, `tests/cli_wheel.rs`, `tests/tui_wheel.rs`
  - modified (listed): `src/run/mod.rs`, `src/cmd/mod.rs`, `src/cmd/run.rs`, `src/run/send.rs`, `src/run/dialog.rs`, `src/cmd/send.rs`, `src/human.rs`, `crates/viola-core/src/lib.rs`, `crates/viola-channel/src/lib.rs`, `crates/viola-channel/src/server.rs`, `crates/viola-pty/src/lib.rs`, `tests/channel_endpoint.rs`, `tests/cli_controls_not_disableable.rs`, `tests/contract_fixture_hygiene.rs`, `tests/contract_ledger_probes.rs`, `tests/contract_fake_agent_drift.rs`
  - listed, not changed: `src/cmd/answer.rs` (its hint comes from `human::answer_hint`, which now returns the `human-typing` hint)
  - recorded (scope-record.md): `crates/viola-state/src/snapshot.rs`, `tests/support/home.rs`, `tests/hook_events.rs`, `src/bin/viola-fake-agent.rs`
  - chunk folder: `evidence/red-before-green.md`, `evidence/operator-pass.md`, `scope-record.md`, `report.md`; `viola-0.1.0/verification-matrix.json` (v1-32 → implemented)
- **Symbols / APIs:**
  - Channel methods served (were `-32601`): `pause` `{from?}` → `{ok:{wheel:"human"}}`; `release` `{budget?, from?}` → `{ok:{wheel:<holder>, budget_paused:false}}`. A string `from` on `release` → `-32602` `"invalid params"` `data:{"reason":"release-from-driver"}` + one `release-from-driver` obs line `{corr:<rpc id>, conn, from (only when a valid name), from_trust:"self-reported"}` at INFO; `from` of another type, a non-bool `budget` (incl. `null`) → `-32602` `data:null`; `budget:true` leaves the wheel; `release` on a driver-held wheel changes nothing and appends nothing. Both are answered only after the move's record landed (`-32603` if it did not).
  - `viola_channel::ProtocolError::ReleaseFromDriver` (code −32602, message "invalid params", `body().data` `{"reason":"release-from-driver"}`), in the server's −32602 WARN classification. Remaining callers of `ProtocolError::body`: unchanged (frame.rs, lib.rs).
  - `viola_core::HumanTyping { ManualPause }` and `viola_core::WheelCause { Start, HumanInput, ManualPause, Release }` (kebab serde + `as_str`); `viola_state::snapshot::Wheel::as_str`.
  - Root bin: `run::wheel::{WheelSlot, Recorder, Observed, Classifier}` — WheelSlot holds `(holder, cause)` under one lock; each change appends one `wheel` event `{holder, cause}` `source:"wrapper"` (log-only, never through the wait feed), updates the snapshot, opens one point-in-time `run.wheel_transition` span (`wheel_from`, `wheel_to`, `cause`; static name, never the keys), and on a move to the human hands the pending dialog back — all on the wheel's own worker thread; the in-memory move is synchronous. `WheelSlot::flush()` (bounded 2 s) is called at `viola run`'s exit BEFORE the terminal leaves raw mode.
  - `run::snapshot::Snapshots` — the wrapper's one in-process snapshot holder (`init` / `update` under one lock); `start_state`, `spawn_child`'s child pid, `DialogSlot::write_pending` and every wheel change go through it.
  - The stdin observer `Observed` wraps the host stdin passed to `viola_pty::pump_with_paste` (no viola-pty pump change); bytes returned whole and unchanged; a read holding an editing key moves the wheel before it returns.
  - Classifier (F-W2): ground bytes and every completed sequence are editing except `CSI I`/`CSI O`, X10 `CSI M`+3, SGR `CSI < n;n;n M|m`, urxvt `CSI n;n;n M`, DA1 `CSI ? … c`, DA2 `CSI > … c`, CPR `CSI n;n R`, DECRPM `CSI ? n;n $ y`, kitty flags `CSI ? n u`, OSC replies ending BEL or ST, DCS replies ending ST; bound 64 bytes of parameters/payload (past it: editing); a read ending on a lone ESC is the Esc key at once; sequences past `ESC [` / `ESC ]` / `ESC P` / `CSI M` carry across reads; a C0 inside a sequence aborts it as editing.
  - `send` refusal order: `control-character` → `human-typing` (detail `null` | `manual-pause`) → `turn-running` → `input-not-ready` → `no-prompt-submitted`; every wrapper `send-refused` obs line now carries `wheel = <holder>`; `send-refused` event data `{refusal, detail, cursor?}` (detail may be null). `answer` order: params → `control-character` → `human-typing` → `unverified-cli` → `unknown-dialog`.
  - `hook.dialog` under a human holder: registered + logged with its own `dialog_id`, replied `null` at once; a held dialog is handed back on a wheel move (null at once, `pending_dialog` cleared, `dialog_settled`, the PreToolUse arm unanswered; `deadline_hit:false`).
  - An unsent `prompt-submitted` the hook filed `origin:"human"` moves the wheel BEFORE its line is appended (a `harness` prompt and the relabelled in-flight `driver` prompt never move it).
  - CLI: `viola pause <name> [--json]`, `viola release <name> [--json]` (liveness-only pre-check → one request with `from` from `VIOLA_NAME`; exit 0 / 20 / 21; any other reply = internal error, exit 1); stdout `<name>  wheel human  manual-pause  I have control` / `<name>  wheel <holder>  you have control`; `release`'s `about` names it the human's verb. Writers `human::write_paused` / `write_released`; the `human-typing` hint `the human has the wheel; send again after the human hands it back` from `send_hint(…, "human-typing")` and `answer_hint("human-typing", _)`; no hint names `release` (unit-asserted).
  - `viola_pty::host_stdin()` — on Windows, when stdin is a console, a reader calling `ReadConsoleW` itself (no Ctrl-Z wakeup control): UTF-16 → UTF-8, a split surrogate carried, every `0x1A` kept, a 0-unit read read again; elsewhere (and a redirected Windows stdin) `std::io::stdin()`. Callers: `src/cmd/run.rs` `pump_child` (sole product caller) and the fake agent (`src/bin/viola-fake-agent.rs`, test binary).
  - Test support: `Wrapper::resize` (tests/support/home.rs).
- **Crates / modules:** root bin modules added `run::wheel`, `run::snapshot`, `cmd::pause`, `cmd::release`; no crate added or removed.
- **Dependencies:** none added or bumped (`git diff eb53a58 -- Cargo.toml Cargo.lock` empty — gate entry `git diff … | grep -cE '^\+.*(skip|ignore|exceptions|allow *=)'` read 0).
- **Schema / config:** none changed — `schemas/diag-line.v1.json` already admitted `wheel` on `send-refused` and the `release-from-driver` event (G4 schema check green on all three OSes, ci#37232840791). No config key, no env var (`VIOLA_NAME` only, read as before). Snapshot `wheel` field unchanged in shape (`driver`|`human`).
- **Spec-master edits:** none (no `.andromeda/` master touched; `git status --short -- .andromeda/*.md` clean of masters).
- **Counts / qualifiers moved:** the wrapper channel's served methods 5 → 7 (`pause`, `release` added; `link`/`unlink` still −32601); the dated liveness-only gaps 6 → 7 (F-W1); `cli_controls_not_disableable.rs` rows +1 (the human wheel; its header's "still to join" drops the wheel); the `walk-class` marks 0 → 3 (`git grep -l -F -e 'andromeda:walks-tree' -- '*.rs' | wc -l` = 3: `tests/contract_fake_agent_drift.rs`, `tests/contract_fixture_hygiene.rs`, `tests/contract_ledger_probes.rs`); viola-pty real-PTY tests +3 (`console_read_of_a_mouse_report_is_one_whole_read`, `console_reads_under_win32_input_mode_are_the_platform_encoding`, `typed_reads_win32_key_downs_as_their_characters`) + 4 console-reader unit cases.
- **Dev-tool versions:** none — no host tool installed or changed (the `x86_64-pc-windows-msvc` rustup target, already installed, was used for a local Windows-target clippy into `target/wincheck`).
- **Harness / gate surface:** none changed in the harness or CI (`scripts/`, `crates/viola-e2e`, `.github/` untouched). The fake agent's stdin now reads through `viola_pty::host_stdin()` (identical on Unix; on a Windows console it no longer turns a lone `^Z` into end of input).
- **Cross-project / external claims:** CI on the pushed branch `build/viola-0.1.0` (Turbolet85/viola), read by `ci.py conclusion`:
  - ci#37225394452 on `7f42d6f` — red (macOS `path5_human_takes_the_wheel_and_release_returns_it`; windows-2025 `tui_ctrl_z…`, `tui_focus…`)
  - ci#37226294797 on `5c6101c` — red (windows-2025 `tui_focus…` only)
  - ci#37226763168 on `126e921` — red (windows-2025 `tui_focus…`: "a mouse report took the wheel")
  - ci#37227518624 on `849588b` — red (windows-2025 `tui_focus…` + the win32-input-mode measurement case)
  - ci#37232840791 on `79ec57c` — **green 15/15** (the final HEAD; this wrap's own commit adds bookkeeping on top of it)
  - The inbox ConPTY / win32-input-mode behaviour is Windows' (Microsoft conhost on `windows-2025`), measured there (Spec claims disproved).
- **Reverted / negative API facts:**
  - The prompt's wheel move first ran AFTER the line's append; reverted to before it (CI macOS race: the test saw the line, `release` returned the wheel, the late move took it back).
  - `wheel.flush()` first ran after the terminal restore; moved before it.
  - The focus case's first driver probe was a `send` (could never be read back: the reports sit in the fake agent's prompt line); replaced by an `answer` to no pending dialog (exit 13 = driver, 10 = human).
- **Insufficient fixes (written, kept, not the remedy):** the flush-before-raw-restore reorder was written for a local pre-push red (a truncated `.profraw`); it is kept, but the red's cause is not proven (see Outcome / Spec claims), so the reorder is a hypothesis-driven remedy, not a proven one — owner: the wrap (P7 ASSERT basis) / operator.
- **Spec claims disproved by measurement:**
  1. a11y-plan case (3) / v1-32 acceptance / the plan's acceptance "focus reports, a mouse report and a host resize append no wheel record" and "reach the child unchanged" — FALSE on `windows-2025`: the inbox ConPTY outer terminal swallows `ESC[I` / `ESC[O` (child keys held the mouse report alone, ci#37226294797), and under the sideloaded ConPTY's win32-input-mode request (`ESC[?9001h`) it hands the wrapper an injected mouse report as one win32 key-down record per character, `ESC[0;0;<Uc>;1;0;1_` (ci#37227518624, `console_reads_under_win32_input_mode…`), which the F-W2 classifier rightly reads as typing → the wheel moves. Plain (no 9001h) the report arrives as one whole read. Disposition: the founder's live ruling F-W3 (below) — a Windows clause, test-side pin; the amendment is owed here.
  2. The plan's/research's prediction that CARRY §8 is viola's alone — measured: viola's own reader carries `^Z` (ci#37226294797 green on `tui_ctrl_z…`); the first Windows red was the FAKE AGENT's std console read (kept home `viola-test-wvRdGs`: child exit 0 at +52 ms after the `^Z`, no Ctrl-C, no channel call).
  3. architecture [Human Takeover / Wheel] lists terminal replies nowhere (focus/mouse/resize only) — superseded by F-W2 (closed non-editing list incl. terminal replies).
- **Expected amendments (from plan):** (site search: per-master `grep -c` over `.andromeda/{architecture,security-plan,design-system,layout-templates,test-plan,obs-plan,a11y-plan}.md`)
  - architecture [Human Takeover / Wheel] (terminal replies non-editing per F-W2; `release` clears no running-turn state; the wheel in `src/run/wheel.rs` + one snapshot holder `src/run/snapshot.rs`) — carried (Symbols / APIs: classifier, WheelSlot, Snapshots). Sites: `Human Takeover / Wheel` architecture 2 · security-plan 1 · others 0.
  - architecture §Standard Contracts → Channel methods (`release-from-driver` data + message; `pause`/`release` served) — carried (Symbols / APIs). Sites: `Channel methods` architecture 2; `release-from-driver` architecture 0 · security-plan 2 · design 1 · test-plan 4 · obs-plan 6.
  - security-plan §Security Anti-Patterns → Authentication, §Authentication & Authorization "IPC client-side server verification", §Input Validation "CLI arguments / stdin" + "Channel frames" — the seventh dated gap (F-W1) + the `pause {from?}` / `release {budget?, from?}` params clause; §Bootstrap auth-scaffolding-baseline amendment 4 (`release-from-driver`) folded — carried (Symbols / APIs; Counts: 6 → 7 gaps). Sites: `liveness-only` security-plan 3 · obs-plan 1; `dated gap` architecture 1 · security-plan 2 · obs-plan 1.
  - obs-plan §4 Scenario "Human takes the wheel…" — `run.wheel_transition.cause` `human-key` → `human-input` — carried (Symbols: the span's `cause` takes `WheelCause::as_str`). Sites: `human-key` obs-plan 1, others 0.
  - test-plan §6 Path 5 as landed (`tests/tui_wheel.rs`, `tests/cli_wheel.rs`), MCP `send` owed to `:102`, Playwright WHEEL cell owed to `:139`; §5 CLI the controls table's human-wheel row — carried (Files; Counts). Sites: `Path 5` test-plan 1 · obs-plan 1 · a11y-plan 1.
  - a11y-plan §1 tui + §3 Keyboard test harness — terminal replies non-editing (F-W2); case (3) as landed with the `windows-2025` reading (CARRY §9) — carried, AND widened by F-W3 (Spec claims disproved 1). Sites: `Keyboard test harness` a11y-plan 3; focus mentions a11y-plan 62 (`ESC\[I\|focus\b\|focus/`).
  - architecture [PTY] — std's console `^Z` strip, source-read and measured on `windows-2025`, and viola's own console reader (CARRY §8) — carried (Symbols: `host_stdin`; Spec claims disproved 2). Sites: `PTY]` architecture 2; `^Z`/`0x1A` 0 in every master.
  - Matrix notes owed at P5: `v1-31` (the "human-held wheel leaves the dialog to the human" clause lands here) and `v1-40` (the "focus, mouse and resize never move the wheel" clause lands here — on Windows per F-W3) — ledger-note — owner P7.3.
- **Coverage of new surfaces:**
  - channel `pause` → validation `parse_from`✓ · instrumentation channel-request/response + `run.wheel_transition`✓ · PII n/a (no user content) · tests unit (cmd::run, wheel) + integ (cli_wheel) · a11y n/a · tokens n/a
  - channel `release` (+ `release-from-driver`) → validation string-`from` refusal, `parse_from`, bool `budget`✓ · instrumentation `release-from-driver` line + channel lines✓ · PII `from` only when a valid name✓ · tests unit + integ · a11y n/a · tokens n/a
  - CLI `viola pause` / `viola release` → validation `parse_name` (`ViolaName::try_new`)✓ · instrumentation `pause.client` / `release.client` spans + process lines✓ · PII n/a · tests integ (cli_wheel, tui_wheel) · a11y CLI lines ASCII, no SGR under non-TTY✓ · tokens n/a
  - stdin observer / classifier (hot path) → validation bounded state machine (64-byte bound)✓ · instrumentation no per-byte line (one span per transition)✓ · PII keystrokes reach no log/span/event/detail (secret scan green on 3 OSes)✓ · tests unit (classifier table whole + split) + e2e (tui_wheel) · a11y focus/mouse/resize case (3)✓ (Windows per F-W3) · tokens n/a
  - `viola_pty::host_stdin` (Windows console reader) → validation n/a · instrumentation n/a (seam read, never logged) · PII n/a · tests unit (stand-in source: ^Z kept, empty read re-read, surrogate split, long read) + Windows CI (tui_ctrl_z, console_read cases) · a11y n/a · tokens n/a
  - wheel worker records (`wheel` event, snapshot) → validation closed types✓ · instrumentation `run.wheel_transition`✓ · PII n/a · tests unit (order, flush, concurrency on the snapshot holder) · a11y n/a · tokens n/a

## Deviations from intent
- `release` on a wheel the driver already holds changes nothing (no record). The plan's transition list ("release → driver/release") left the case open; a no-op avoids a cause-only record.
- `pause` / `release` reply only after their `wheel` record lands (worker ack), so the CLI's exit follows the disk; `-32603` if it did not land.
- Exit flush: `viola run` waits (≤ 2 s) for the wheel's queued records before exit, with the terminal still raw — not in the plan; added so the stop's Ctrl-C take is never lost (measured: the `hook_events` count went nondeterministic without it).
- The human-prompt wheel move happens before the line's append (the plan said "reports to the wheel" without order) — CI-measured race (macOS).
- The fake agent reads through `viola_pty::host_stdin()` — CI-measured (CARRY §8 fold); test-side.
- The focus case is step-wise (resize first with its size-receipt barrier; focus reports with a mouse report as their read barrier) and Windows-split per F-W3 (founder live ruling) — the plan's single-sequence form could not tell which input moved the wheel.
- `tests/hook_events.rs` count expectations updated for the legitimate extra `wheel` records (a human-filed prompt; the stop's Ctrl-C).
- Scope record (`gate.py scope`: clean — changed 26 · listed 22 · recorded 4 · widening 0):
  - companion: `tests/hook_events.rs` · serves `src/run/wheel.rs` · self
  - in-intent: `crates/viola-state/src/snapshot.rs` · serves step 2 · self; `tests/support/home.rs` · serves `tests/tui_wheel.rs` · self; `src/bin/viola-fake-agent.rs` · serves `tests/tui_wheel.rs` · self

## Decisions & corrections
- **F-W1 — the founder's live ruling, 2026-10-04, relayed by the overseer through the operator's P4 answer (the pause-swallow residual shown):** CLI `viola pause` / `viola release` write their frames after the liveness-only pre-check — the seventh dated gap, until Epoch 6 `:109` / `:111`; borrows none of the six earlier gaps.
- **F-W2 — the founder's live ruling, 2026-10-04, same exchange (the CPR / Shift+F3 collision shown):** the closed non-editing list (focus, mouse, terminal replies); every other byte takes the wheel.
- **F-W3 — the founder's live ruling, 2026-10-04 (answered 20:34Z through the overseer's AskUserQuestion, the measured win32-input-mode mechanism shown, "the error only ever favours the human"), relayed by the overseer:** "Pin the platform fact." — on Windows an injected mouse report takes the wheel (test-side), Unix keeps the full assertion, a11y-plan case (3) and the v1-32 acceptance get a Windows clause at this wrap; route pin: a mouse report from a real Windows terminal is measured at `:82` live.
- Operator word (this session): "Run the operator pass with the ci.py conclusion read (leg=operator) as usual. A windows-2025 red folds into this chunk, measure first: the ^Z reader and the ConPTY focus reports are measured only there." — followed: every Windows red measured before its fold (kept-home role lines, a step-wise case, a viola-pty read-boundary measurement).
- Operator direction (P5, carried in the plan): the `prompt-submitted` → `turn-ended` running-turn state has no route owner — bring it to the wrap's route-resolve as a recommended-first card for the founder.
- Correction for curation: `testing.md`'s "one Rust test" example names `scripts/agent-run.sh run --e2e --filter …` — the harness has no `--e2e` selector (exit 2 usage; P5 corrected this chunk's own gate to `--integration`).
- Sweep hazards: (a) an outer-PTY test that writes reports into a fake agent's line and then `send`s can never be read back (the paste joins the junk line) — probe the wheel with `answer` to an unknown id instead; (b) on Windows, bytes written as TEXT into a ConPTY input pipe are typed keys to the console beneath it once win32-input-mode is on — a test cannot inject a "mouse report" there; (c) a count of `events.ndjson` lines after a `Wrapper::stop` now includes the Ctrl-C's `wheel` record.
- Host: the Bash guard refused a `cat` heredoc to a scratchpad file (recurrence of host-win32.md 2026-09-29) — scripts went through the Write tool.
- Host reboot at 19:09Z mid-pass: background CI waits died; the CI read was re-done with `ci.py conclusion --sha 126e921` on the operator's word.

## Outcome
- Acceptance (re-asserted against the diff):
  - (tests) The wheel / v1-32 — MET on Linux and macOS as written; on `windows-2025` MET under F-W3's Windows clause (injected mouse report takes the wheel; focus reports swallowed); the acceptance TEXT still says otherwise → amendment owed (Spec claims disproved 1); ci#37232840791 green on all three OSes.
  - (arch) classifier table — MET (`run::wheel::tests::classifier_*`, whole and split).
  - (arch) refusal orders — MET (`send_refusal_order`, `answer_refusal_order_under_a_human_wheel`).
  - (arch) concurrent snapshot changes + `wheel` never wakes `wait` — MET (`snapshot_concurrent_wheel_and_dialog_changes_both_reach_the_disk`; `wheel` appended via `append_event`, never the feed).
  - (security) `release-from-driver` — MET (unit + `cli_wheel`); `pause` with `from` accepted — MET.
  - (security) seventh dated gap F-W1 — MET in code (liveness-only `live_endpoint`); spec amendment owed.
  - (security) no `#[ignore]`/retry/skip/env read/seam/G2 exemption, no deny change — MET (the three `git diff eb53a582c8dc` probe entries green).
  - (obs) `send-refused` carries `wheel`, `side`, `corr`, `conn`, `rpc_id`; `release-from-driver` fields; G4 — MET (`cli_wheel`; CI G4 green ×3).
  - (obs) no keystroke byte in logs — MET (CI secret scan green ×3).
  - (a11y) case (3) — MET per F-W3 (Windows clause); text amendment owed.
  - (a11y) human-mode refusal lines — MET (`cli_wheel` exact stderr; `--json` no hint).
  - (design) `pause` / `release` stdout lines — MET (`cli_wheel`).
  - (tests) CARRY §8 — MET (windows-2025 green on `tui_ctrl_z…` since ci#37226294797).
  - (tests) controls human-wheel row — MET.
  - (tests) U40 — MET: the probe entry `git grep -l -F -e 'andromeda:walks-tree' -- '*.rs' | wc -l` reads `3`; every gate run's header printed `walk-class rust 3 (tests/contract_fake_agent_drift.rs, tests/contract_fixture_hygiene.rs, tests/contract_ledger_probes.rs) · uncommitted {u}`.
  - (tests) pre-push `linux-tests` ok + CI `verdict: green` on the final HEAD — MET (`79ec57c`, ci#37232840791).
- Gates (implement's final block on the tree before the operator pass; the operator pass re-ran pre-push before every push; outcome basis below):
  - `cargo fmt --all --check` — green
  - `cargo clippy --workspace --all-targets --features fake-agent -- -D warnings` — green (and the windows-msvc target clippy clean, locally)
  - `bash scripts/agent-run.sh run --unit --filter 'test(/run::wheel::tests::|…|cmd::answer::tests::/)'` — green, 199 passed (baseline 137)
  - `bash scripts/agent-run.sh run --unit --filter 'package(viola-core) | package(viola-channel) | package(viola-pty)'` — green, 186 passed (baseline 178), more after the viola-pty measurement cases
  - `bash scripts/agent-run.sh run --unit` — green
  - `bash scripts/agent-run.sh run --integration --filter 'binary(cli_wheel) | …'` — green
  - `bash scripts/agent-run.sh run --integration --filter 'binary(tui_wheel) | …'` — green, 74 passed (first run red on the focus case's own probe design; fixed)
  - `bash scripts/agent-run.sh run` — green (first red on 3 `hook_events` count cases; folded)
  - `git grep -l -F -e 'andromeda:walks-tree' -- '*.rs' | wc -l` — green, last line 3
  - `bash scripts/agent-run.sh cleanup/boot/status/cleanup --session p-wheel-smoke` (smoke) — green ×4 (`processes_gone`, `endpoint_gone` true)
  - `git diff eb53a582c8dc -- Cargo.toml Cargo.lock deny.toml deny-sync.toml | grep -cE …` — green (exit 1, last line 0)
  - `git diff --quiet eb53a582c8dc -- src/cmd/hook/seam.rs scripts/g2-zero-panics.sh` — green
  - `! (git diff eb53a582c8dc -- '*.rs' .config/nextest.toml | grep … | grep -v 'VIOLA_NAME')` — green
  - `bash scripts/agent-run.sh pre-push` — green on the final tree (one red during the fold: coverage `llvm-profdata merge` refused one truncated `viola-2660688-…_16.profraw` (77 496 B; common size 80 848 B) with 1450/1450 tests passed; not reproduced in 5 later pre-push runs nor CI; cause NOT proven — hypothesis: a harness `cleanup` Ctrl-C reached a wrapper whose exit-time flush ran after the terminal left raw mode → SIGINT mid profile write; the flush now precedes the restore. Open for the wrap's disposition)
  - `gate.py hygiene` (leg operator) — clean before each push (evidence/operator-pass.md)
  - `git diff --quiet && … && git push origin HEAD` (leg operator) — pushed `7f42d6f`, `5c6101c`, `126e921`, `849588b`, `79ec57c`
  - `ci.py conclusion --sha HEAD --wait 1800` (leg operator) — final: `79ec57c verdict: green · checks 15/15`, ci#37232840791 (the earlier four runs red, each measured and folded — evidence/operator-pass.md)
  - Red-before-green controls: 7 guards neutralised, each red on its own rows, restores hash-verified (evidence/red-before-green.md)
- Watches: none folded.
- Wrap light gate (P7.1, run 2026-10-04T20-44-01): the first run read entry 16 (the no-new-env-read guard) RED on
  `849588b`'s `std::env::var(CHILD_MODE)` line in viola-pty's cfg(test) child. The wrap halted on the overseer's
  direction (founder-delegated). The operator pass's `ba36659` moved the `reads` / `reads-win32` modes to argv,
  leaving the guard unchanged; it is locally green (lint, viola-pty 38/0, entry 16, pre-push), and its CI read is
  ci#37235841342 on `ba36659`, `verdict: green · checks 15/15`, now the final HEAD's run
  (`evidence/operator-pass.md`).
- Outcome basis: the operator pass ran (pre-CI `7f42d6f`, parent `eb53a58`); its commits `7f42d6f..79ec57c` (above) and the final HEAD's CI run ci#37232840791 recorded in `chunks/2026-10-04-the-wheel/evidence/operator-pass.md`; implement's P4 report (this session's conversation) for the rest.
- Process hygiene: none of this chunk's processes left running — every test wrapper, fake agent, harness session (`p-wheel-smoke`) and background gate/CI read terminated (census at implement P4 and again at the operator pass's end). Foreign processes seen and left: the operator's `viola-lab` prototype `viola run` sessions and their `viola wait`s, and a `ci.py --wait 2400` from another session.
