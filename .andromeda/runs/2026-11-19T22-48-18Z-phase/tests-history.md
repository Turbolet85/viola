@history_item n=1 marker=2026-09-29-fake-agent-drift-contract
**2026-09-29-fake-agent-drift-contract** (Harness prefixes: four) — widened the harness class of `prompt-submitted` origin to four raw-start prefixes (`<agent-message from=`, `<task-notification>`, escaped and plain `<cross-session-message`), no trim; tag mid-prompt or after a leading space stays `human`; witnessed red-first by `prompt_origin_files_the_cross_session_tag_as_harness`. Direct precedent for this chunk: a new origin class (or a wake told apart) extends `contract/prompt-submitted/origin-harness` and `decision/harness-turn-keeps-the-wheel`, needs a founder-ratified widening and a red-first unit witness.

@history_item n=2 marker=2026-10-07-live-rows-and-paste-shapes-on-the-dev-host
**2026-10-07-live-rows-and-paste-shapes-on-the-dev-host** — added the live-measured origin cases `prompt_origin_live_shape_files_as_harness` (a `<task-notification>` typed at a prompt's very start; the unescaped cross-session raw start) as literal oracles beside the normalisation cases. The origin classifier's case tables are where a wake-vs-typed case lands; the literal-oracle rule (measured shape pinned, not the product's list) binds it.

@history_item n=3 marker=2026-10-04-the-wheel
**2026-10-04-the-wheel** (Path 5 as landed) — Path 5's wheel tests: `tests/tui_wheel.rs` (outer-PTY steps, the harness-turn case, focus/mouse/resize, `^Z`) and `tests/cli_wheel.rs` (pause/release, `human-typing` refusals with detail null | manual-pause, hints without `release`, release-from-driver exit 20); the human-wheel row joined the controls table. The wake case is a new Path 5 case beside the harness-turn one, through the outer PTY, verdict from records only.

@history_item n=4 marker=2026-10-04-running-turn-refusal
**2026-10-04-running-turn-refusal** (Path 5 and Path 2 span the running turn) — a harness-injected turn logs `prompt-submitted{origin:"harness"}` and no `wheel`, and `send` during it exits 13 `turn-running` until `turn-ended`; every test that sends twice ends the first turn with a scripted Stop (the fake agent's interactive submit fires none). Trap and precedent for a wake-turn: a wake told apart keeps the wheel but its turn still runs, so the driver's next send is refused `turn-running` until its `turn-ended` (`decision/injected-turn-keeps-wheel`).

@history_item n=5 marker=2026-10-08-first-live-test-and-self-drive
**2026-10-08-first-live-test-and-self-drive** (Path 5 records the classifier's tables) — the terminal-reply classifier's closed list (`classifier_terminal_reply_is_not_editing`, 23 `REPLIES`, negative controls) is unit-pinned and its real-terminal reading is a local live measurement, not a suite. Bears directly on a pty-byte-source means: rows `decision/terminal-replies-not-editing` and `decision/terminal-reply-reading-live-measured`; the defect was invisible at every CI tier because no tier writes a terminal reply.

@history_item n=6 marker=2026-10-08-first-live-test-and-self-drive
**2026-10-08-first-live-test-and-self-drive** (`--local-live` alone; Path 2 live readings) — a text ending in one CR was filed `human`, moved the wheel and ended exit 13, with no test at any tier. Precedent of a non-typed prompt mis-filed `human` taking the wheel, read live and left unpinned; a wake told apart needs its pin at a tier.

@history_item n=7 marker=2026-10-07-send-waits-out-the-paste-hint
**2026-10-07-send-waits-out-the-paste-hint** — `send` reads human-typing and turn-running a second time after the gate returns ready (four clock-driven units); `send_under_the_paste_hint_a_human_key_during_the_gate_wait_wins` (exit 10, nothing typed, key reaches the child) cannot tell the first wheel read from the second, so its green is a floor. Constraint for the send path after a wake: `decision/send-refusal-order` and `contract/viola-send/human-key-during-gate`.

@history_item n=8 marker=2026-10-09-epoch-3-cleanup
**2026-10-09-epoch-3-cleanup** (the typed text, the `empty-text` refusal) — the send order became control-character → empty-text → human-typing → …, with five unit cases for the new rung. The refusal order a wake fix must keep (`decision/empty-text-refused-before-the-wheel`, `decision/send-refusal-order`).

@history_item n=9 marker=2026-11-14-budget-gate-removed
**2026-11-14-budget-gate-removed** (4. Unit Test Strategy: 5 rows) — `decision/send-refusal-order` lost the budget-paused rung (now control-character, empty-text, human-typing, turn-running, input-not-ready, no-prompt-submitted, re-reading human-typing and turn-running after the gate). The current order the wake's send-path change rests on.

@history_item n=10 marker=2026-11-14-budget-gate-removed
**2026-11-14-budget-gate-removed** (6. E2E Test Strategy: 9 rows) — `contract/event-log/start-wheel-record`: every start appends `wheel{driver,start}` first and session-start as its second and last start record (budget-gate record gone). The start's wheel baseline a wake test boots from.

@history_item n=11 marker=2026-11-15-live-round-on-the-trimmed-product
**2026-11-15-live-round-on-the-trimmed-product** (6. E2E: start-wheel-record) — live start measured folder-trust keys passing through the wrapper writing `wheel{human,human-input}` before session-start, a later human key adding none. Trap: a live wake reading starts with the wheel already human after trust keys; any rig start reading the wheel must account for it.

@history_item n=12 marker=2026-11-15-live-round-on-the-trimmed-product
**2026-11-15-live-round-on-the-trimmed-product** (+ decision/start-lines-read-from-start-offset) — a reader of a reused instance name's log must read a start's lines from that start's own offset, never line 1. Binds any rig or e2e reader of the wake start's wheel and prompt-submitted lines.

@history_item n=13 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (E5 built: steps 1–3) — every start appends `wheel{driver,start}`, so after a restart the wheel is the driver's; the replay recovers the wheel with no file written. Rests `decision/start-hands-wheel-to-driver`, the wheel baseline a wake must not disturb.

@history_item n=14 marker=2026-11-16-wait-wake-condition
**2026-11-16-wait-wake-condition** (6. E2E: viola-wait/non-waking-records) — a harness-origin `prompt-submitted` never wakes a parked wait, beside activity and wheel records. If a wake gets its own origin (or files as harness), the wait wake set and this row must be read for it.

@history_item n=15 marker=2026-11-16-wait-wake-condition
**2026-11-16-wait-wake-condition** (7. Test Data: fake-agent-cli/script) — the fake agent's turn script gained Notification and UserPromptSubmit steps. A scripted UserPromptSubmit step (no typed bytes) is the fake-agent mechanism a wake rehearsal or CI case would use.

@history_item n=16 marker=2026-11-16-wait-wake-condition
**2026-11-16-wait-wake-condition** (3. Test Harness: + decision/one-verify-stamp-per-build; 5. Integration: decision/fresh-home-per-test; 3. decision/verify-driven-tests-20s-kill) — stamped homes copy one verify stamp per build under a lock, keeping each stamped-home test under the 20 s kill. Any added ledger row or probe changes every stamp and runs under these rules.

@history_item n=17 marker=2026-11-16-scheduled-wake-count
**2026-11-16-scheduled-wake-count** (5. Integration: contract/event-data/turn-ended) — turn-ended data gained `scheduled_wakes` (four keys). The count this chunk's scope names as the prior chunk's surface; the "while a wake was scheduled" hypothesis reads it.

@history_item n=18 marker=2026-11-16-scheduled-wake-count
**2026-11-16-scheduled-wake-count** (4. Unit: + contract/dialog-data/scheduled-wakes) — `scheduled_wakes` is only the length of the Stop payload's `session_crons`, no element read or kept, null when absent or not an array, a non-array one drift entry. Constraint: a means that uses the cron state may not read its elements without widening this row.

@history_item n=19 marker=2026-11-16-scheduled-wake-count
**2026-11-16-scheduled-wake-count** (6. E2E: + contract/turn-ended/scheduled-wakes) — the count reaches `events.ndjson`, CLI `wait --json` and MCP `wait` only, never `last`, `list`, the snapshot or the replay. A means keeping wake state in the wrapper must not leak it to those surfaces without amending this reach.

@history_item n=20 marker=2026-11-17T05-21-35Z-wrap
**2026-11-17T05-21-35Z-wrap** (the delegate's confirm ratified — scheduled-wake-count, 4. Unit / 5. Integration / 6. E2E) — the founder ratified the three scheduled-wake-count rows; the same wrap placed this entry and recorded the founder's word that its means is its phase's to find. The rows stand on his word, not provisional.

@history_item n=21 marker=2026-10-19-overseer-switch-to-viola
**2026-10-19-overseer-switch-to-viola** — named the `background_tasks` count cases (`turn_ended_background_tasks_counts_the_array`, `…_is_null_when_absent_or_not_an_array`) and recorded live readings as a local measurement, not a suite. The sibling pattern `scheduled_wakes` copied; precedent for stating this chunk's live evidence.

@history_item n=22 marker=2026-11-15-api-error-turn-end
**2026-11-15-api-error-turn-end** (6. E2E: contract/viola-send/turn-running; 5. Integration: event-data/turn-ended) — an API-error turn end clears the running turn with no pause or release; turn-ended gained `cause`. A wake's turn ends by Stop or StopFailure; after either a send is accepted.

@history_item n=23 marker=2026-11-15-api-error-turn-end
**2026-11-15-api-error-turn-end** (7. Test Data: fake-script/format ten events; + decision/send-cases-copy-recorded-fixtures; hook-payloads-recorded-by-verify except) — a send-after-turn case must start from a copy of the recorded fixture set (synthetic payloads alone draw no input box, send reads input-not-ready); StopFailure registered with no recorded payload/ledger row/probe. Trap for any wake send case; precedent for a CLI field with no recorded fixture.

@history_item n=24 marker=2026-11-19-a-live-stopfailure-payload
**2026-11-19-a-live-stopfailure-payload** (the StopFailure exception settled on the live reading) — one contained live StopFailure reading on 2.1.328 settled the exception on the founder's word, claiming nothing beyond the one word and version, no fixture made. Precedent for how a single live reading of an undocumented payload is scoped (the wake's absent `source` key).

@history_item n=25 marker=2026-11-19-a-live-stopfailure-payload
**2026-11-19-a-live-stopfailure-payload** (6. E2E: decision/standing-live-rig except; 7. Test Data: + --turn-fail, --settings-hooks) — the rig's SF arm rehearses unwrapped in CI and its contained rehearsal is a local entry skipping on its sandbox probe; the fake agent gained `--turn-fail` and `--settings-hooks`. Where a wake arm would sit and what fake-agent options already exist.

@history_item n=26 marker=2026-11-19-a-live-stopfailure-payload
**2026-11-19-a-live-stopfailure-payload** (1. no-child-screen-assertions, never-parse-rendered-child-screen; 11. no-screen-parsing; 2. discipline-at-every-layer) — the rig's screen read widened to the box row's cell attributes, closed words only, never text. The current except on every screen-parse row; a wake means must not read child screen content outside it.

@history_item n=27 marker=2026-11-17-standing-live-rig
**2026-11-17-standing-live-rig** (6. E2E: − live-readings-not-a-test-layer → + decision/standing-live-rig) — live rounds reuse one committed rig, `viola-live-rig` in viola-e2e, rehearsals over the fake agent in CI, live starts by hand under the founder's cap, refusing under CI. Any live wake reading (the scope's carry for a third hint reading included) goes through this rig.

@history_item n=28 marker=2026-11-17-standing-live-rig
**2026-11-17-standing-live-rig** (1. decision/real-claude-local-only; 11. no-blanket-approval-tests except) — the real CLI is reached only through `viola verify` and the rig's live starts, never in CI; the rig passes a read trust dialog under its key grant. Bounds where wake behaviour on the real CLI can be read.

@history_item n=29 marker=2026-11-17-standing-live-rig
**2026-11-17-standing-live-rig** (7. Test Data: trust dialog, session-start, exit triggers, keystroke) — the fake agent draws a box on typed keys, erases on DEL, holds its launch SessionStart behind the trust form. The fake agent's key model a "no byte from the human" means is rehearsed against.

@history_item n=30 marker=2026-11-17-standing-live-rig
**2026-11-17-standing-live-rig** (4. Unit: send-confirmation-trailing-trim-match why) — the CLI's prompt trim was read live (trailing spaces/TAB removed, leading run kept). Picked row `decision/send-confirmation-trailing-trim-match`, which a driver send confirmed after a wake still relies on.

@history_item n=31 marker=2026-11-16-send-confirmation
**2026-11-16-send-confirmation** (4. Unit: send-confirmation-trailing-trim-match; 6. E2E: viola-send/records; 7. Test Data: bracketed-paste `--trim-prompt-end`) — the claim takes a trailing-trimmed prompt; send records hold the submitted text; the fake agent gained `--trim-prompt-end`. The confirmation path ("the driver's next send is confirmed") the acceptance rests on.

@history_item n=32 marker=2026-11-16T06-05-33-wrap
**2026-11-16T06-05-33-wrap** (send trailing-only tolerance ratified; StopFailure no-ledger-row ratified) — both rows stand on the founder's word; a leading trim waits for a live reading. Neither may be moved incidentally by this chunk.

@history_item n=33 marker=2026-11-19-context-as-a-local-command
**2026-11-19-context-as-a-local-command** (6. E2E: + decision/live-rig-hint-cell-reading) — the rig's hint-cell reading records attribute words past the cursor after the second send, as a reading never verification; `CC` read no cell. PROVISIONAL until the founder's word; the scope's carried third reading (fresh box, keys before any send) extends this row.

@history_item n=34 marker=2026-11-17-trust-record-ledger-row
**2026-11-17-trust-record-ledger-row** (6. E2E: + decision/live-failing-reading-not-verification) — a failing live reading is recorded as read, never as verification; a confounded one leaves the row unverified and its re-read owed. Binds how this chunk records any wake reading.

@history_item n=35 marker=2026-11-17-trust-record-ledger-row
**2026-11-17-trust-record-ledger-row** (3. + decision/verify-under-test-owned-user-home; 5. + decision/no-verify-start-outside-test-home) — every test-started verify runs under a test-owned user home, held by a census. Binds any new verify probe for a wake ledger row.

@history_item n=36 marker=2026-11-18-session-folder-trusted-by-viola
**2026-11-18-session-folder-trusted-by-viola** (3. + decision/run-under-test-owned-user-home, harness-wrappers-under-session-user-home) — run and revive tests run under a test-owned user home, so a start's trust step never writes the real config. Binds every new wake e2e case.

@history_item n=37 marker=2026-10-05-real-cli-verify-probes
**2026-10-05-real-cli-verify-probes** — the first ledger expansion: each new row moves `[NN/MM]`, `LEDGER_ROWS`, `cli_verify` and `contract_ledger_probes` literals and boot step 4's verify flags; screens fixture class added. Precedent for the cascade a wake ledger row (if the means takes one) triggers.

@history_item n=38 marker=2026-10-18-compact-post-condition-ledger-row
**2026-10-18-compact-post-condition-ledger-row** (twenty-four rows) — adding `local-command-compact` moved relied counts, `STEPS_PASS`/`ROW_IDS`/`LEDGER_ROWS`, no-framing and tag-turn summaries, the older-stamp oracle, Run B's pastes and fixture counts. The most recent full worked cascade for a new relied row.

@history_item n=39 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (the relied set, all twenty-four rows on every build) — `LedgerRow::relied()` is every row on every build, literals ungated; the older-stamp oracle reads older sets unverified. Current shape of `decision/version-gate-needs-all-relied-rows` and the count literals a new row moves.

@history_item n=40 marker=2026-11-18-browser-tools-prompt-modal
**2026-11-18-browser-tools-prompt-modal** (4. version-gate-needs-all-relied-rows; 5. verify-output step/summary line; 3. local-live-suite/pass; 1. ledger-row-verify-probe) — a probe may report `absent` (browser-tools-prompt row only) counting toward verified. PROVISIONAL (delegate). Current verdict vocabulary any new wake probe would use.

@history_item n=41 marker=2026-11-16-local-command-delivery
**2026-11-16-local-command-delivery** (3. + decision/stamping-verify-passes-transcript) — every stamping verify passes `--transcript` beside `--framing` or the echo row fails. Convention a new stamping helper or probe for a wake row must keep.

@history_item n=42 marker=2026-10-19-dialog-hold-for-the-driver
**2026-10-19-dialog-hold-for-the-driver** — the dialog hold's unit cases on the injected clock (expiry's one `dialog-expired` + `dialog-answered{deadline_hit:true}`, no record on hand-back or answer, `pending_dialog` before the line), `dialog_hold_s` controls row. The acceptance's "next question dialog is held for the driver" rests on these rows (`decision/dialog-hold-expiry`, `dialog-hold-read-once`, `pending-dialog-before-its-line`).

@history_item n=43 marker=2026-10-04-dialog-answers-by-dialog-id
**2026-10-04-dialog-answers-by-dialog-id** — Path 4 over `path4.json` (question with annotations, plan, second concurrent dialog empty, `unknown-dialog`). The dialog-to-driver e2e a wake case extends to show the next question held for the driver after a fired wake.

@history_item n=44 marker=2026-10-17-mcp-server-for-drivers
**2026-10-17-mcp-server-for-drivers** (the MCP tests as landed) — Path 5's MCP step `mcp_tools_path4_path5_answer_by_id_and_a_paused_send` compares MCP refusals with the CLI `--json`. Rows `contract/mcp-send/refused-human-typing` and cli-mcp parity: a wake fix changes what both surfaces return and must hold parity.

@history_item n=45 marker=2026-10-27-bounded-inputs-at-every-boundary
**2026-10-27-bounded-inputs-at-every-boundary** (the feed-panic passthrough witnessed on the outer terminal) — `chaos_feed_panic_refuses_send_and_keeps_passthrough` byte-searches the outer terminal, the one sanctioned exception to no-screen assertions. A means in the pty pump's byte path must keep this passthrough and its rows (`decision/vt100-panic-contained`, `vt100-panic-keeps-passthrough`).

@history_item n=46 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (the hook's instance check on its event and dialog paths) — the hook runs the instance check before any config read; line-expecting hook cases run in a viola-made instance dir (`Env::Made`). Binds any hook-side test of a UserPromptSubmit wake payload (`decision/hook-instance-check-first`, `hook-line-cases-in-made-instance-dir`).

@history_item n=47 marker=2026-11-14-status-line-interception-removed
**2026-11-14-status-line-interception-removed** (1. viola-hook exit-code, stdout; 6. hook-instance-check-before-config except) — `viola hook <event>` alone, always exit 0, stdout empty or a decision body; one `--capture` arm stays ahead of the instance check. Current hook contract rows a UserPromptSubmit parse change sits under.

@history_item n=48 marker=2026-11-17-booted-wrapper-does-not-outlive-its-test
**2026-11-17-booted-wrapper-does-not-outlive-its-test** (5. wrapper-drop-guard, + no-untied-viola-spawn, + tied-child-spawn-thread) — every long-lived viola child of a root test is tied to its test thread by parent-death kill, held by a census. New wake test files must spawn tied.

@history_item n=49 marker=2026-11-17-supply-chain-green-and-booted-tests-end
**2026-11-17-supply-chain-green-and-booted-tests-end** (5. + decision/no-untied-harness-boot) — no viola-e2e test boots a harness session untied, held by a census read red on a planted boot. Binds a rig or harness-session wake rehearsal.

@history_item n=50 marker=2026-11-15-mutation-and-coverage-baselines-green
**2026-11-15-mutation-and-coverage-baselines-green** (3. root-wait-bound, within-7s-wait-bound, mutants kill) — WITHIN is 12 s and the mutants default kill 15 s, below cargo-mutants' 20 s floor. The wait bounds a new wake test uses; a kill moves only for a by-design floor.

@history_item n=51 marker=2026-11-13-rows-proposal-rule-work
**2026-11-13-rows-proposal-rule-work** (5. decision/pty-tested-on-real-openpty) — the resize test writes no key after a resize, reading the child's size report alone. Bears on a pump-side means that reasons about bytes around a resize (`decision/key-after-resize-waits-for-size`, `resize-baseline-is-spawn-size`).

@history_item n=52 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (the OS scope: Linux only, one CI leg) — Linux only, CI on `ubuntu-latest`, Windows/macOS code out of the tree. Any wake case is a Linux case; no OS gate may name another OS (census).
