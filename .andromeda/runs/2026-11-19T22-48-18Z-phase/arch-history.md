@history_item n=1 marker=2026-09-27-hooks-to-normalised-events
**2026-09-27-hooks-to-normalised-events** — `hook.event` became a served method that re-validates `prompt-submitted` `text`/`origin`, and the normalisation order was fixed: classify `harness` on the raw prefix first, then unwrap, then un-escape. Any wake-vs-typed classification has to fit into this hook-side order and the 5-kind `hook.event` validation (decision/prompt-normalise-order, contract/hook-event/kind).

@history_item n=2 marker=2026-09-29-fake-agent-drift-contract
**2026-09-29-fake-agent-drift-contract** — The closest precedent. Another session's message was filed `human` and flipped the wheel, so the classifier's `harness` class was widened to four compiled `HARNESS_PREFIXES` matched on the raw start. The founder ratified this live as a boundary widening, and it carries an accepted side effect: a human who types the tag is filed `harness`. A new non-human origin for a fired wake is the same kind of widening, needs the same founder word, and has the same false-positive question (decision/harness-prompt-prefixes, decision/harness-prompts-not-human).

@history_item n=3 marker=2026-10-04-confirmed-send-with-cl-1-records
**2026-10-04-confirmed-send-with-cl-1-records** — Established that `origin: driver` is set only by the wrapper's relabel of the in-flight send's prompt, and that the hook files only `human` or `harness`. A wake told apart must take a hook-filed value or a wrapper-side decision without breaking this rule (contract/event-data/prompt-submitted-origin).

@history_item n=4 marker=2026-10-04-the-wheel
**2026-10-04-the-wheel** (the closed non-editing list) — Created the closed non-editing byte list (F-W2): every byte outside it takes the wheel, and the stdin observer only observes. It also fixed that a human-filed `prompt-submitted` moves the wheel before its line is appended (`src/run/wheel.rs`). These are the exact mechanism a fired wake trips today, and the byte-source hypothesis (no human byte since the boundary) rests on this observer (decision/byte-source-wheel-claim, decision/human-prompt-confirms-wheel-take, decision/wheel-classifier-only-observes).

@history_item n=5 marker=2026-10-04-the-wheel
**2026-10-04-the-wheel** (pause and release served) — `pause`/`release` were served, and `release` from a driver (a string `from`) is refused. The wheel returns to the driver only through CLI `viola release`, so after a wake is wrongly filed `human` the only recovery today is a human `release` (decision/wheel-returns-only-through-release, contract/send-refusal/manual-pause).

@history_item n=6 marker=2026-10-04-running-turn-refusal
**2026-10-04-running-turn-refusal** — A running-turn state is held in `WheelSlot`. A `prompt-submitted` of any origin starts it, and an unsent human prompt takes the wheel first, so the next `send` reads `human-typing`, not `turn-running`. A wake told apart still starts a turn, so the send path after a told wake will read `turn-running` until the turn ends (decision/send-never-queued, contract/refusal-order/send).

@history_item n=7 marker=2026-10-04-dialog-answers-by-dialog-id
**2026-10-04-dialog-answers-by-dialog-id** — Landed the dialog tier: `hook.dialog`, `answer`, the pending dialog, at most one dialog pending, and a dialog logged exactly once. A held dialog is answered `null` at once while the wheel is `human`, which is why a wake filed `human` leaves the next question dialog unheld (decision/human-wheel-renders-dialogs, decision/one-pending-dialog).

@history_item n=8 marker=2026-10-07-live-rows-and-paste-shapes-on-the-dev-host
**2026-10-07-live-rows-and-paste-shapes-on-the-dev-host** (cross-session form and typed task-notification) — Measured on 2.1.287 that a cross-session message arrives unescaped and the plain prefix files it `harness`, and that a typed `<task-notification>` at the prompt's start is filed `harness`. These readings came from a scratch probe, not a verify probe, so the harness-prefix ledger row stays owed. Precedent for measuring a prompt's origin shape live, and a warning that a scratch reading lands no row.

@history_item n=9 marker=2026-10-07-live-rows-and-paste-shapes-on-the-dev-host
**2026-10-07-live-rows-and-paste-shapes-on-the-dev-host** (the two rows still owed) — The harness-prefix and R8 identity-floor rows were measured by scratch probe and stay owed with no route entry. If this chunk's means relies on a CLI prompt shape, the ledger rule asks for a relied row with a probe, as the harness-prefix case still does (decision/capability-ledger-gate).

@history_item n=10 marker=2026-10-07-a-send-ending-in-a-newline-is-confirmed
**2026-10-07-a-send-ending-in-a-newline-is-confirmed** — A defect of the same class, earlier and since fixed. An in-flight send whose prompt was not relabelled was filed `human` and moved the wheel. The fix changed the typed text rather than the claim's tolerance, and "a prompt that is not the in-flight send's typed text claims nothing and is appended as the hook filed it" stands, so a wake prompt is appended as the hook files it.

@history_item n=11 marker=2026-10-07-send-waits-out-the-paste-hint
**2026-10-07-send-waits-out-the-paste-hint** — `send` reads the wheel and the running turn twice, at arrival and again after the gate, so a wheel move during the gate's 8.5 s wait refuses the send `human-typing`. A wake told apart must not move the wheel at either read (contract/refusal-order/send, decision/send-time-bound).

@history_item n=12 marker=2026-10-08-first-live-test-and-self-drive
**2026-10-08-first-live-test-and-self-drive** (seven terminal reply shapes) — Seven reply shapes joined the closed non-editing list, each by exact grammar with negative controls, on the founder's ratification. Standing rule: the list grows only by measured shapes, never on a guess. Any byte-source means that adds to or reads the non-editing list is bound by this rule (decision/closed-non-editing-byte-list).

@history_item n=13 marker=2026-10-10-viola-revive
**2026-10-10-viola-revive** (the wheel at a revived start) — On the founder's answer, a revived start's first record is `wheel{holder:"driver", cause:"start"}`, so the start-time wheel is `driver` whether `run` or `revive` started it. The state a wake must leave unchanged (decision/wheel-starts-with-driver).

@history_item n=14 marker=2026-10-19-dialog-hold-for-the-driver
**2026-10-19-dialog-hold-for-the-driver** — Set the configurable `dialog_hold_s` (60 to 3600 s) and the `dialog-expired` wake, and handed a pending dialog back `null` when the wheel moves to the human. A wake's wrong wheel move ends a held dialog's hold early. "Held for the driver" in v2-36's acceptance is this hold (decision/dialog-hold, contract/dialog-hold-end/null-reply, contract/event-line/dialog-expired).

@history_item n=15 marker=2026-10-19-overseer-switch-to-viola
**2026-10-19-overseer-switch-to-viola** — `turn-ended` gained `background_tasks` as a count only, with no element read. This is the precedent for `scheduled_wakes`'s count-only form and for the wrapper knowing there is pending CLI work at a turn end.

@history_item n=16 marker=2026-10-26-server-verification-before-any-frame
**2026-10-26-server-verification-before-any-frame** (the hook's one connect) — Gave the hook's event and dialog paths one shared connect with a server check. A failed check sends no frame, records `server-verify-failed` and fails open. Any new hook-side reading of UserPromptSubmit stays behind this connect (contract/hook-diagnostics/server-verify-failed, decision/dialog-hook-fails-open-on-server-check).

@history_item n=17 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (the hook's instance check) — `viola hook` takes its instance only after `trusted`/`canonical`, before any config read, stdin read or obs init, and a refused instance exits 0 silently. A wake-classification read in the hook comes after this check (decision/hook-without-trusted-instance-exits-silently, decision/hook-exits-on-untrusted-instance).

@history_item n=18 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** — Added the hook's `socket-dir-refused` fail-open, with one `hook-decision` at WARN. It is part of the hook path order that a new UserPromptSubmit field read must keep (contract/hook-diagnostics/socket-dir-refused).

@history_item n=19 marker=2026-10-04-readiness-gate-and-timing-constants
**2026-10-04-readiness-gate-and-timing-constants** — Fixed `SPINE_DEADLINE` at 1 s and the hook's 750 ms `CONNECT_DEADLINE` (PROVISIONAL), and made vt100 the pre-send gate on `run`'s feed thread. UserPromptSubmit is a spine hook, so any added classification work must stay inside the spine deadline (decision/spine-deadline, decision/hook-connect-deadline, decision/vt100-readiness-gate).

@history_item n=20 marker=2026-11-13-verify-readiness-on-the-input-box
**2026-11-13-verify-readiness-on-the-input-box** — Made readiness read the input box's own rule and prompt rows beside the footer literals. This bears on the carried third reading of the placeholder hint's cell attribute (cry-iuatjqkx), a screen reading on the same input box.

@history_item n=21 marker=2026-11-15-live-round-on-the-trimmed-product
**2026-11-15-live-round-on-the-trimmed-product** — `wheel-starts-with-driver` now names a key typed into the folder-trust dialog before the session starts as a wheel take. The dialog-hold end promise changed to "ends only the hook's wait; the CLI's dialog stays drawn". Both are the latest word on the wheel's start and the hold's end.

@history_item n=22 marker=2026-11-15-api-error-turn-end
**2026-11-15-api-error-turn-end** — Registered StopFailure with no ledger row and no verify probe as a named exception to the capability-ledger rule, added `turn-ended`'s `cause`, and made an API-error turn end clear the running turn. This is the precedent for relying on a CLI hook behaviour without a row, if the means here leans on a hook shape. The exception is written into decision/capability-ledger, decision/cli-capability-ledger and decision/capability-ledger-gate.

@history_item n=23 marker=2026-11-16T06-05-33-wrap
**2026-11-16T06-05-33-wrap** — The founder ratified StopFailure's no-row exception, with its live reading owned elsewhere, and the trailing-only send tolerance (decision/submitted-form-prompt-match standing). The no-row path stands only on the founder's own word, not on a delegate's.

@history_item n=24 marker=2026-11-16-wait-wake-condition
**2026-11-16-wait-wake-condition** — Pinned that `wait` wakes by event kind alone, and that an activity record and a harness-origin `prompt-submitted` never wake a parked wait. This was the first step on the wake problem: at the time `turn-ended` carried no scheduled-wake count and the count was a founder candidate. A new origin for a fired wake must keep the wake-kind rules (contract/channel-wait/wake-kinds, contract/channel-methods/wait-wake-kinds).

@history_item n=25 marker=2026-11-16-send-confirmation
**2026-11-16-send-confirmation** — Replaced the exact match with the submitted-form match and put the send's post-gate section (the human-typing and turn-running reads, the link, send-issued, the paste) under one wrapper flight lock. The hook.event claim, turn start and end, and wheel moves take the same lock, and a human key never does. A wake-aware wheel decision on `prompt-submitted` runs under this lock (decision/send-flight-lock, decision/submitted-form-prompt-match).

@history_item n=26 marker=2026-11-16-survey-key-keeps-the-wheel
**2026-11-16-survey-key-keeps-the-wheel** — A direct precedent and a trap. A proposed exemption that would have kept the wheel on a CLI-driven input (the survey key) was thrown out by the founder («Выкинуть фичу»), and `closed-non-editing-byte-list` now records that a survey key takes the wheel. A means that keeps the wheel for a non-human event needs the founder's own word, and the survey case is one human act a byte-source means must still tell apart.

@history_item n=27 marker=2026-11-16-scheduled-wake-count
**2026-11-16-scheduled-wake-count** — Added `turn-ended` `scheduled_wakes` (the Stop payload's `session_crons` length, count only, `null` on StopFailure) and stated that a scheduled wake is counted but never a wake condition. This is the register the chunk rests on: whether a wake is pending can be known from the turn end before the fire (contract/event-data/turn-ended-scheduled-wakes, contract/hook-to-kind-map/stopfailure).

@history_item n=28 marker=2026-11-17T05-21-35Z-wrap
**2026-11-17T05-21-35Z-wrap** (scheduled-wake count ratified) — The founder ratified the delegate's confirm of the scheduled-wake-count rows (turn-ended data, wait wake kinds, `scheduled_wakes`, the StopFailure map). This same wrap placed this entry and recorded the measured defect, so the count's contract now stands on his word.

@history_item n=29 marker=2026-11-16-local-command-delivery
**2026-11-16-local-command-delivery** — `hook.event` gained the optional sibling `transcript` param, and stdout-echo commands are confirmed by a transcript tag count. A candidate means could read the session transcript, and this entry is the precedent and the shape (contract/channel-methods/hook-event, contract/hook-event/notification).

@history_item n=30 marker=2026-11-17-standing-live-rig
**2026-11-17-standing-live-rig** — Landed the test-only `viola-live-rig` and `AGENT_RUN_RIG`, gave vt100 its rig-screen exception, and read the CLI's submit trim live. This is where a wake would be rehearsed live, and the rig names it adds are registered (decision/vt100-screen-model, decision/test-binaries-never-released, decision/harness-env-unread-by-product).

@history_item n=31 marker=2026-11-17-first-readings-on-the-standing-rig
**2026-11-17-first-readings-on-the-standing-rig** — The rig's capture verb now reads the key names and the `source` word of its own captured UserPromptSubmit payload, never a value. This was the founder-ratified widening that let a fired wake's payload be read (it showed no `source` key). It opened the test-only exception on decision/claude-first-no-adapter-trait and decision/claude-shapes-in-one-crate. Any further payload reading must extend that exception.

@history_item n=32 marker=2026-11-17-booted-wrapper-does-not-outlive-its-test
**2026-11-17-booted-wrapper-does-not-outlive-its-test** — Registered the test-only markers `ROOT_TIED_TEST_CHILD` and `HARNESS_TIED_TEST_CHILD` outside `VIOLA_*`. A new test seam or marker for rehearsing a wake must follow this naming and registration (decision/viola-env-namespace-product-only).

@history_item n=33 marker=2026-11-17-trust-record-ledger-row
**2026-11-17-trust-record-ledger-row** — The latest widening of verify's child set (Run F, six interactive runs), with a new relied row, its probe and closed code words. This is the template and the price if the means is a ledger row with a verify probe: a founder-ratified widening, a re-stamp, and the claude-shapes exceptions extended (decision/verify-child-set, decision/verify-removes-only-own-dirs).

@history_item n=34 marker=2026-11-18-trust-record-row-read-live
**2026-11-18-trust-record-row-read-live** — Added the hidden rig-only `--record-first-screens` (an exception to verify-record-only-clean) and `measured.first_screen`/`first_settle_ms`, and the rig now seeds its private Claude config. This is the current text of the test-only rig exceptions a wake rehearsal would extend.

@history_item n=35 marker=2026-11-18-browser-tools-prompt-modal
**2026-11-18-browser-tools-prompt-modal** — "Verified" changed to every relied row counting, `absent` allowed on browser-tools-prompt alone, and the step and summary lines changed with it. A new relied row for a wake shape must say how it counts and whether `absent` is possible (decision/capability-ledger, decision/version-verified-all-relied-rows, decision/browser-tools-prompt-ledger-row).

@history_item n=36 marker=2026-11-18-the-live-home-s-re-stamp-and-the-product-re-pin
**2026-11-18-the-live-home-s-re-stamp-and-the-product-re-pin** — The claude-first and claude-shapes test-only exceptions were shortened to name the VH key-name and needle reads. Trap: these cells keep growing with each rig reading and have to be compressed.

@history_item n=37 marker=2026-11-19-a-live-stopfailure-payload
**2026-11-19-a-live-stopfailure-payload** — The StopFailure capture's key names, error word and two lengths were added to the same test-only exception. StopFailure was settled as outcome B (no row) on one contained live reading on 2.1.328, on the founder's word. This is the precedent for a contained live hook-payload reading in the rig's network-closed sandbox, and for the no-row outcome.

@history_item n=38 marker=2026-11-19-context-as-a-local-command
**2026-11-19-context-as-a-local-command** — Added the 25th relied row (`local-command-context`) with Run B's eighth paste, and restored «private» in the claude-shapes exception within its 200-character bound by dropping the separators' spaces. Trap: another rig reading added to that exception has almost no room left. This is also the latest widening of the relied set (decision/local-command-context-probe).

@history_item n=39 marker=2026-10-13-paste-newline-ledger-row
**2026-10-13-paste-newline-ledger-row** — A widening of Run B to rest a relied shape on a row. Every home stamped before reads `cli_verified:false` until re-verified. The cost a new relied row imposes on the live home and the product re-pin (decision/cli-capability-ledger).

@history_item n=40 marker=2026-11-14-status-line-interception-removed
**2026-11-14-status-line-interception-removed** — The hook keeps one hidden `--capture` arm, inert (no `VIOLA_*`, no channel, exit 0). A verify probe capturing a wake's UserPromptSubmit would use this one arm (decision/probe-capture-arms-inert, decision/hooks-fail-open).

@history_item n=41 marker=2026-11-14-budget-gate-removed
**2026-11-14-budget-gate-removed** — The budget rung left the send refusal order, which now reads control-character, empty-text, human-typing, turn-running, gate. This is the current order that a wake told apart must leave the driver's send passing (contract/send-refusal/order, contract/refusal-order/send).

@history_item n=42 marker=2026-10-10-session-links
**2026-10-10-session-links** — The implicit link is recorded on a send after the second wheel and turn reads and before `send-issued`, and a refused send links nothing. Part of the send path that a wrongly moved wheel refuses today (contract/event-line/link).

@history_item n=43 marker=2026-11-15-mutation-and-coverage-baselines-green
**2026-11-15-mutation-and-coverage-baselines-green** — The shared root-test wait bound `WITHIN` moved to 12 s, and test channel sockets moved under a short `/tmp` root. A new root test of a wake rehearsal under the fake agent shares this bound (decision/root-waits-share-one-bound).

@history_item n=44 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** ([Human Takeover / Wheel]: the Windows input passage retired) — The Windows wheel input readings were retired, and the bracketed send and the prompt-origin rules that move the wheel were kept unchanged. The wheel's origin rules are Linux-only and stand as the base this chunk amends.
