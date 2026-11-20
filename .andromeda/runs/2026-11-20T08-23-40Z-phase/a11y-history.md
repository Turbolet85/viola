@history_item n=1 marker=2026-10-07-send-waits-out-the-paste-hint
**2026-10-07-send-waits-out-the-paste-hint** — set the readiness gate: a verified `viola send` waits for the input box at most `GATE_MAX_WAIT` (8.5 s), then is refused `not-delivered` / `input-not-ready` with nothing typed. The bound falls on the driver only, and a human key during the wait takes the wheel, so the waiting send is refused `human-typing`. The guard `send_under_the_paste_hint_a_human_key_during_the_gate_wait_wins` is sequenced on the `channel-request` line and the `wheel` record, never a timer, and the unit tier proves the gate's two wheel reads. This chunk changes the predicate behind this exact refusal (contract/send-refusal/input-not-ready, decision/human-key-takes-the-wheel). The fix must keep the refusal's form, keep the bound on the driver and keep the key-during-wait guard green.

@history_item n=2 marker=2026-11-13-verify-readiness-on-the-input-box
**2026-11-13-verify-readiness-on-the-input-box** — ~ decision/no-screen-content-parsing: the vendor screen may be parsed only for compiled signatures (footer literals, the input box's rule and prompt rows, modal literals), and the prompt row only for its opening char, never the text after it. This was ratified as a boundary widening (founder F1/F2; card confirm PROVISIONAL). Any discriminator that tells a quoted modal literal in a transcript row from a standing modal must stay inside this ban. If it reads a row's text, position or context beyond the compiled signatures, that is a new widening of the row, owed to the founder.

@history_item n=3 marker=2026-11-17-standing-live-rig
**2026-11-17-standing-live-rig** — ~ decision/no-screen-content-parsing except: the first test-only exception, letting the live rig read the trust selector's place, the input box state and modal dialogs as closed words. The rig's modal reading is test-only and does not license the product gate to read more of the screen.

@history_item n=4 marker=2026-11-19-a-live-stopfailure-payload
**2026-11-19-a-live-stopfailure-payload** — ~ decision/no-screen-content-parsing except: the rig may also read the box row's cell attributes after a turn end, as closed words and never text. This is a precedent that a new screen reading takes a row amendment through the founder's or delegate's answer. If this chunk's discriminator reads cell attributes or rows in the product (not the rig), it needs the same path.

@history_item n=5 marker=2026-11-19-context-as-a-local-command
**2026-11-19-context-as-a-local-command** — ~ decision/no-screen-content-parsing except: re-worded to "the test-only rig reads as closed words, never text … after a turn end or send the box row's cell attributes" (PROVISIONAL, delegate overseer). This continues the run of amendments to the ban this chunk's discriminator must respect.

@history_item n=6 marker=2026-11-19-a-fired-scheduled-wake-keeps-the-wheel
**2026-11-19-a-fired-scheduled-wake-keeps-the-wheel** — ~ decision/no-screen-content-parsing except: the current wording (adds box-row cell attributes before any send; PROVISIONAL, delegate overseer 2026-11-20). This is the row as picked for this chunk, its except limited to the test-only rig. It is the last change to the area right before this chunk.

@history_item n=7 marker=2026-10-27-bounded-inputs-at-every-boundary
**2026-10-27-bounded-inputs-at-every-boundary** — added contract/tui-passthrough/after-feed-panic: after a contained vt100 feed panic the driver's `send` is refused `not-delivered` / `input-not-ready` (exit 13) while a human key still reaches the child, and no viola literal reaches the outer terminal (`chaos_feed_panic_refuses_send_and_keeps_passthrough`). This is a second, deliberate `input-not-ready` path from the same gate. Changing the predicate must not lift this refusal.

@history_item n=8 marker=2026-10-04-the-wheel
**2026-10-04-the-wheel** — fixed the tui wheel boundary: a closed non-editing list (focus, mouse and terminal replies), with every other byte editing and taking the wheel, and the wheel probed by an `answer` to no pending dialog (exit 13 driver, 10 human). This is the base of decision/human-keystrokes-never-blocked and the takeover clause. A changed pre-send gate must leave human keys and the wheel untouched.

@history_item n=9 marker=2026-10-08-first-live-test-and-self-drive
**2026-10-08-first-live-test-and-self-drive** — recorded the first live reading of the takeover clause (a `wtype` key wrote `wheel{human}`, and the next `send` exited 10), as supplemental and never gating. This is the origin of decision/live-readings-never-gate: any live confirmation of the quoted-modal fix stays supplemental.

@history_item n=10 marker=2026-10-19-overseer-switch-to-viola
**2026-10-19-overseer-switch-to-viola** — recorded the takeover clause's second live reading (the founder's own key, the next `send` exit 10 `human-typing`, `release` returning the wheel), still never gating. It bears on decision/live-readings-never-gate and decision/critical-path-human-takes-wheel. The defect here was measured the same way (live `send`, then `pause` / `release`).

@history_item n=11 marker=2026-10-04-dialog-answers-by-dialog-id
**2026-10-04-dialog-answers-by-dialog-id** — the dialog deadline bounds the driver: on expiry the dialog renders for the human in the claude TUI, and a later `answer` is refused `not-delivered` / `unknown-dialog` (exit 13). This is the origin of contract/cli-exit-codes/13 and decision/dialog-hold-bounds-the-driver. It applies because a standing modal must keep refusing the driver's send while the dialog is the human's.

@history_item n=12 marker=2026-10-19-dialog-hold-for-the-driver
**2026-10-19-dialog-hold-for-the-driver** — made the dialog hold configurable (`dialog_hold_s` 60–3600 s). At its end the wrapper replies `null` and the dialog renders for the human, and any human key hands a held dialog back at once. This is the rule behind decision/dialog-hold-bounds-the-driver. The scope asks whether the dialog relay reads the same predicate as the gate.

@history_item n=13 marker=2026-11-06-version-done-check
**2026-11-06-version-done-check** — (the dialog hold bounds owned by 0.2.0's first entry) set `DIALOG_HOLD_MIN` / `DIALOG_HOLD_MAX` PROVISIONAL, owned by 0.2.0's first entry. This is the dialog-hold area this chunk's modal reading sits beside.

@history_item n=14 marker=2026-11-15-live-round-on-the-trimmed-product
**2026-11-15-live-round-on-the-trimmed-product** — ~ decision/dialog-hold-bounds-the-driver state provisional → standing: the dialog was read on the screen after expiry, so the row now stands. The standing-modal side this chunk must keep refusing rests on it.

@history_item n=15 marker=2026-11-06-version-done-check
**2026-11-06-version-done-check** — (the cli output-discipline checks on std::process::Command and literal pins) moved the cli checks to `std::process::Command` with literal pins; P4's cli `human-typing` refusal and its typed exit code are checked that way. This is the source of the guard on decision/send-refused-while-human-typing and of decision/refusal-layout-literal-pins. A new send-delivered or send-refused test follows this form.

@history_item n=16 marker=2026-11-06-version-done-check
**2026-11-06-version-done-check** — (the §8 verification lines pinned by literal) literal pins hold each refusal's `unable <reason> <detail>` plus final `hint:` layout, with literal asserts on the typed exit codes (`10`–`14`, `20`, `21`) and a hint-free `--json`. These are the pins (contract/cli-refusal/*, decision/refusal-layout-literal-pins) the unchanged `input-not-ready` refusal must keep green.

@history_item n=17 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** — (the `--json` failure documents' witness) `cli_error_documents_hold_no_path_or_upstream_text`: each `--json` failure is one document, with a literal exit, empty stderr, no ESC and a clean body. This is the source of contract/cli-json-failure/document, which the send refusal still obeys under `--json`.

@history_item n=18 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** — (`--json`'s one document and its exit-2 exception) the one-document rule has an exception: `viola send` / `viola answer` exit-2 paths write a fixed stderr line and no document. This is a send-path carve-out to keep in mind when testing send outcomes under `--json`.

@history_item n=19 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** — (`unknown`'s detail, a code word or nothing) changed the except on decision/refusal-ends-with-hint: `unknown` (exit 14) prints no hint, and its detail is a code word or nothing.

@history_item n=20 marker=2026-10-18-compact-post-condition-ledger-row
**2026-10-18-compact-post-condition-ledger-row** — added the confirm windows (`CONFIRM_WINDOW_FALLBACK` 10 s, `COMPACT_CONFIRM_WINDOW` 40 s) as driver-only bounds on a typed send, with `not-delivered` / `no-prompt-submitted` when unconfirmed. A send this chunk now delivers enters this confirm path, which is part of the time-bounds rows.

@history_item n=21 marker=2026-11-14-budget-gate-removed
**2026-11-14-budget-gate-removed** — (8. Cognitive Accessibility: 6 row(s)) ~ decision/time-bounds-fall-on-the-driver dropped the budget pause, and exit 11 is retired and never reused. The send path has no pre-send gate besides readiness and the wheel, and the list of bounds was narrowed.

@history_item n=22 marker=2026-11-14-budget-gate-removed
**2026-11-14-budget-gate-removed** — (8. Cognitive Accessibility: 1 row(s)) ~ decision/time-bounds-never-on-human dropped the budget pause from viola's time bounds.

@history_item n=23 marker=2026-11-14-status-line-interception-removed
**2026-11-14-status-line-interception-removed** — ~ decision/time-bounds-never-on-human and ~ decision/time-bounds-fall-on-the-driver dropped the status-line deadline, leaving the list of bounds this chunk's readiness gate belongs to.

@history_item n=24 marker=2026-11-18-session-folder-trusted-by-viola
**2026-11-18-session-folder-trusted-by-viola** — ~ decision/time-bounds-never-on-human and ~ decision/time-bounds-fall-on-the-driver now also list the start's Claude config lock wait, which falls on viola's own start. This is the current wording of both picked rows. The send readiness gate stays a driver-only bound.

@history_item n=25 marker=2026-11-18-browser-tools-prompt-modal
**2026-11-18-browser-tools-prompt-modal** — ~ contract/viola-verify/step-line now reads pass|absent|fail, with absent only on the browser-tools-prompt row (PROVISIONAL, delegate overseer). This is the latest modal-literal chunk, and verify's Run B framing pastes use the same input-box predicate this chunk changes. Its modal signature and verify output must not move.

@history_item n=26 marker=2026-10-10-viola-revive
**2026-10-10-viola-revive** — a passed `viola revive` enters the TUI passthrough boundary, and the keystroke ban covers its start. This is the source of decision/single-vendor-content-boundary and the revive clause of decision/human-keystrokes-never-blocked. A revived session's sends pass through the same gate.

@history_item n=27 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** — (the tui boundary on openpty alone, the ConPTY and F-W3 readings retired) the TUI is hosted in openpty alone, each of the three boundary clauses is its own nextest case on ubuntu-latest, and the feed-panic boundary is read without a Windows arm. This is where decision/boundary-tests-sequenced-on-records and decision/tui-boundary-assertions-only now run.

@history_item n=28 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** — (the a11y cases' CI home, the one ubuntu-latest leg) the cli / tui a11y cases run in the `test` job's `coverage` suite on `[ubuntu-latest]` alone, gated by `gate --require coverage,doctest`. This chunk's new cases land there.

@history_item n=29 marker=2026-10-16-remove-the-browser-leg
**2026-10-16-remove-the-browser-leg** — the cli / tui cases became nextest cases in the coverage suite, and the zero-flakiness parallel bans `#[ignore]`. This is the source of decision/zero-flakiness. It bears on this chunk's watch and carry (the `stop_presses_ctrl_c…` and `rig_rehearsal…` reds): no retry and no ignore.

@history_item n=30 marker=2026-11-12T03-23-49-rows
**2026-11-12T03-23-49-rows** — turned the whole master into rows (decision / contract / absence), each verified against the code. It is the baseline from which every picked row's handle descends.

@history_item n=31 marker=2026-11-08T20-32-52-wrap
**2026-11-08T20-32-52-wrap** — retired the frontend stubs: no web or graphical frontend, and §5–§7 and the harness keys read N/A. This is the source of the plan's absences (absence/web-or-graphical-frontend, absence/rendered-page) that hold for this chunk.
