@history_item n=1 marker=2026-09-27-hooks-to-normalised-events
**2026-09-27-hooks-to-normalised-events** (hook stdin row) — hook stdin read through `take(MAX_FRAME + 1)` so an over-limit payload is detected, `<event>` mapped into the closed `HookEvent`, and the fail-open details `oversize-stdin` · `malformed-json` · `channel-unreachable` set. The PermissionRequest question payload enters on this path and keeps these bounds and fail-open words.

@history_item n=2 marker=2026-09-28-capability-ledger-and-viola-verify
**2026-09-28-capability-ledger-and-viola-verify** (stamps read before strict-modes, the interim gap) — run's version gate read of `ledger/stamps.json` and the rule that a bad stamps file only degrades to `cli_verified:false`. It also set the PREREQ that a non-null dialog decision needs the strict stamps read. That PREREQ is the origin of the stamp gate the new `allow` body passes through.

@history_item n=3 marker=2026-09-28-capability-ledger-and-viola-verify
**2026-09-28-capability-ledger-and-viola-verify** (the hidden `hook --capture` arm, probe captures, fixture scrub) — added the capture arm only verify's probe plugin calls, transient 0600 probe captures removed by a drop guard, and the `--record` scrub that refuses a recording whole. Founder-ratified as a boundary widening. The new verify probe and its `fixtures/claude/<version>/` recordings ride on this arm and scrub.

@history_item n=4 marker=2026-09-29-t15-07-57-wrap
**2026-09-29-t15-07-57-wrap** (Decisions Log moved into the body) — recorded the accepted risk that a driver LLM can be prompt-injected into `answer` `allow`. viola's part is only the closed `behavior` enum and the `unverified-cli` gate. An `allow` that now carries question answers widens what that accepted risk reaches, so the chunk rests on `decision/driver-permission-policy-outside-viola` and must keep viola mechanism-only.

@history_item n=5 marker=2026-10-04-dialog-answers-by-dialog-id
**2026-10-04-dialog-answers-by-dialog-id** (the sixth dated gap: answer and hook.dialog frames, and their validation) — set the `answer` params (`dialog_id` u64, closed per-kind `response`, then `validate_paste_text` on free text) and the `hook.dialog` params (closed `kind`, object `data`, closed `hook_event`, closed-or-absent `tool` / `input` / `continuation`). Added `PermissionRequest` to `HookEvent`. Dialog free text is held to the paste set on both sides (exit 13 client-side). It also bounded the reply read and set one stdout write. A question answer carried under `PermissionRequest` must map into these closed per-kind enums, and its answer values must pass the same paste check on both sides.

@history_item n=6 marker=2026-10-04-dialog-answers-by-dialog-id
**2026-10-04-dialog-answers-by-dialog-id** (run's strict stamps read, decisions on the stamp) — `run`'s gate reads stamps through `read_stamps_strict`. Standing ban: never answer non-null unless the gate read the stamps through the strict check. The six-row-stamp gap shows the residual of a relied row missing from the stamp: a body-shape change in a new CLI goes uncaught until its row and re-probe land. This chunk's new relied row closes exactly that kind of gap for the question body.

@history_item n=7 marker=2026-10-05-real-cli-verify-probes
**2026-10-05-real-cli-verify-probes** — verify's interactive PTY children: direct spawns under the R8 strip at 80×24, read only against compiled literals, never a byte into a CLI-native dialog, and a modal start killed with no key. It also recorded the accepted transcript residual and the named fixture refusal with closed codes. Each new verify child or paste was a founder-ratified widening of the verify child set. A new probe for the question body must stay inside it or take its own ruling.

@history_item n=8 marker=2026-10-05-dialog-rows-and-re-probe
**2026-10-05-dialog-rows-and-re-probe** — the direct precedent. The capture arm answers dialogs through the hidden `--answers <DIR>`, read through `take(64)` into a closed `ProbeAnswer` id. `ledger::probe_body` builds the body only through `dialog::decision_body`. Run C (dialogs, an `ask` rule, its one `allow` running `touch` in its own dir) and the `questions/` root were added, with no byte typed into any dialog (founder rulings M7 = A, "the hook answers, never a key"). The stamp grew to fourteen rows, so a home with failing dialog rows reads `cli_verified:false` and `answer` exits 12. A question-answers probe must answer the same way: a new closed `ProbeAnswer` id through `decision_body`, never a key.

@history_item n=9 marker=2026-10-06-local-command-and-paste-framing-rows
**2026-10-06-local-command-and-paste-framing-rows** — the stamp grew from fourteen rows to seventeen. A stamp holding exactly the older ids, all `pass`, records `cli_verified:false`. Trap for this chunk: adding a relied row turns every existing stamp unverified until re-stamped. That is why the scope names a re-stamp of 2.1.328 (and maybe 2.1.329).

@history_item n=10 marker=2026-10-13-paste-newline-ledger-row
**2026-10-13-paste-newline-ledger-row** — a new ledger row (`paste-newline`) and its Run B paste landed as a ratified widening of the verify child set, the stamp grew from `/17` to `/18`, and older stamps read unverified. Precedent for a ledger row plus its probe: founder answer to a P4 fork, the card confirm PROVISIONAL until his word.

@history_item n=11 marker=2026-10-13-owed-ledger-rows
**2026-10-13-owed-ledger-rows** — rows relied on without a ledger row became rows with a probe and fixtures, and the stamp is read over the relied set (`LedgerRow::relied()`). The probe's new `--settings` and capture arm were a ratified widening. Precedent for turning a relied behaviour into a probed row, and for how the relied set and its counter move together.

@history_item n=12 marker=2026-10-14-resume-and-compact-ledger-rows
**2026-10-14-resume-and-compact-ledger-rows** — Run E joined the verify child set (its own 0700 dir, no key typed), the ledger grew to twenty-two rows, and older stamps read unverified on every OS. It was ratified as a widening of the verify child set. Precedent if the question probe needs a new run rather than a step in Run C.

@history_item n=13 marker=2026-10-16-unwrapped-sessions-on-the-board
**2026-10-16-unwrapped-sessions-on-the-board** — the ledger grew to twenty-three rows (`agents-join` last) and a stamp of exactly the twenty-two pre-chunk ids reads unverified. It shows the row-count and failing-example wording a new row must update across the gate rows.

@history_item n=14 marker=2026-10-18-compact-post-condition-ledger-row
**2026-10-18-compact-post-condition-ledger-row** — the twenty-four-row stamp: `local-command-compact` became relied, and a twenty-three-id older stamp reads `cli_verified:false`. It is the last body-text record of the relied count and its counter. The scope's directive fact (26 relied rows at `1e9f1ea3`) grows by one here.

@history_item n=15 marker=2026-10-07-a-send-ending-in-a-newline-is-confirmed
**2026-10-07-a-send-ending-in-a-newline-is-confirmed** — answer free text is typed as validated. Standing rule: any removal from answer free text needs its own ruling. Question answers carried in `updatedInput` are dialog free text, so no trim or normalisation may be applied to them without a ruling.

@history_item n=16 marker=2026-10-09-inner-cr-and-crlf-in-a-sent-text
**2026-10-09-inner-cr-and-crlf-in-a-sent-text** — kept: `answer` does not call `typed_text`, and answer free text is typed as validated. Reject, don't strip, holds. The answers in the `allow` body must cross exactly as validated, with no CR/LF rewrite.

@history_item n=17 marker=2026-10-17-mcp-server-for-drivers
**2026-10-17-mcp-server-for-drivers** (MCP tool input validation and error bodies as landed) — the MCP `answer` tool checks raw arguments itself: `dialog_id` u64, `response` through `Response::parse`, then `validate_paste_text` on every free-text answer value. A failure is `invalid-params` with a closed field word (`response`) and no frame. If the answer shape must change to carry question answers, `Response::parse` and the field words are what must extend.

@history_item n=18 marker=2026-10-19-dialog-hold-for-the-driver
**2026-10-19-dialog-hold-for-the-driver** — `dialog_hold_s` is validated into `DialogHold` (60–3600, default 60, config only), and the hook's dialog reply read is bounded at `DIALOG_HOLD_MAX + 5 s` with the wrapper replying `null` at its hold's end. This is the "expired hold keeps today's conservative fall-back" the scope relies on: a PermissionRequest question not answered in time stays null to the human.

@history_item n=19 marker=2026-10-19-overseer-switch-to-viola
**2026-10-19-overseer-switch-to-viola** — question option text (`description`, `preview`, taken only as strings else null) crosses into `events.ndjson`, `hook.dialog`, CLI `wait --json` and MCP `wait`, as the founder's own widening. R1 lists question and option text as upstream text, never a command. The driver reads these options to choose answers. The new path must keep option and answer text as content (`decision/question-option-text-crosses-boundary`, `decision/upstream-text-is-never-a-command`).

@history_item n=20 marker=2026-10-26-server-verification-before-any-frame
**2026-10-26-server-verification-before-any-frame** (a failed server check: no frame, and what each client reports) — on the `hook.dialog` path a failed check fails open: exit 0, no body, `hook-decision` `server-verify-failed` after `hook-invoked{corr:null}` with `decision_emitted:false`. The CLI and MCP `answer` report `instance-unreachable`. The PermissionRequest question relay inherits these fail-open shapes unchanged.

@history_item n=21 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (the dated strict-modes gaps retired) — every client, `answer` and `hook.dialog` included, runs `check_instance` before reading the snapshot, then the liveness and server checks. A refusal writes no frame and no line. This is the frame order the new answer path must not bypass.

@history_item n=22 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (`viola hook`'s event and dialog paths check the instance before obs init) — the dialog path canonicalises `VIOLA_DIR` and runs `check_instance` before diagnostics level and obs init. A refused or absent instance exits 0 with no config, stdin, line or frame, and the capture arm stays ahead of the check. Any change to the PermissionRequest path in `src/cmd/hook` keeps this order.

@history_item n=23 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (`ledger/stamps.json` checked before `ledger/` is made) — `update_stamps` runs `check_stamps` before creating `ledger/` and again under its lock, and verify checks its home tree before its log. New ban: never touch the stamps before the check, nor check after a narrowing create. The re-stamp this chunk needs goes through this path.

@history_item n=24 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** (the clients' screen of a peer's reply) — CLI and MCP clients take refusals and details only as code words and faults through `sanitised_fault`, so nothing of the peer's but closed values reaches a document or result. A new answer outcome or refusal word must be a code word on this screen.

@history_item n=25 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** (verify's refusal codes) — verify writes closed refusal codes on its `process-exit`, and a failing row is no refusal and no code. A new probe's failure reads as a row result, never a new refusal code.

@history_item n=26 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** (a refused socket directory at a hook and at a start) — on a refused socket directory the dialog path fails open with one `hook-decision` `socket-dir-refused` after `hook-invoked{corr:null}`, `decision_emitted:false`, and no frame. This is part of the dialog relay's fail-open set the new path inherits.

@history_item n=27 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** (the client's peer euid check and the order at a seam) — fixes the order on every client seam: socket-directory check, connect, peer euid decision, then server verification. A refused euid reads `server-verify-failed`. The `answer` CLI and MCP tools and the hook's `hook.dialog` frame keep this order (`decision/client-seam-check-order`).

@history_item n=28 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (the relied set holds on every build) — `LedgerRow::relied()` became all rows on every build, with one counter and one failing example. The new relied row counts on every build, with no per-OS split.

@history_item n=29 marker=2026-11-13-verify-readiness-on-the-input-box
**2026-11-13-verify-readiness-on-the-input-box** — `decision/screen-fixture-signature-rows-only` was reworded: screen fixtures keep only signature rows, and another box row, a rewritten row or a seam-split username is refused. New probe fixtures under `fixtures/claude/<version>/` must pass this walk.

@history_item n=30 marker=2026-11-15-api-error-turn-end
**2026-11-15-api-error-turn-end** (Authentication & Authorization) — `decision/unverified-cli-gate` gained its one named exception: StopFailure registered with no ledger row and no verify probe. The contrasting precedent: this chunk takes the other route, a relied row with its probe, so the gate's exception list stays StopFailure alone.

@history_item n=31 marker=2026-11-19-a-live-stopfailure-payload
**2026-11-19-a-live-stopfailure-payload** (the StopFailure exception settled on the live reading) — the no-ledger-row exception was settled on the founder's word on one contained live reading, with nothing claimed for other words or versions. This is the bar a live reading sets for any per-version claim the re-stamp makes about 2.1.328 versus 2.1.329.

@history_item n=32 marker=2026-11-17-trust-record-ledger-row
**2026-11-17-trust-record-ledger-row** (Data Protection) — the most recent new-ledger-row-with-probe precedent. Run F's probe writes and removes its own trust record, and the exceptions to "viola writes nothing under the Claude home" were amended with a guarded writer, landed PROVISIONAL on the delegate's confirm. A question probe that writes anything outside its own probe dir would need the same treatment.

@history_item n=33 marker=2026-11-17-trust-record-ledger-row
**2026-11-17-trust-record-ledger-row** (Input Validation) — `decision/trust-record-closed-words`: the probe's outcome crosses into the stamp only as a closed word, never a path, key or value. The pattern for what the new row may stamp.

@history_item n=34 marker=2026-11-18-trust-record-row-read-live
**2026-11-18-trust-record-row-read-live** (Input Validation) — `decision/verify-stamp-first-screen-closed-words`: the stamp records first screens only as closed words, whole-number settles and literal-id facts, never screen text. Any measured part of the new row is held to the same closed shape.

@history_item n=35 marker=2026-11-18-browser-tools-prompt-modal
**2026-11-18-browser-tools-prompt-modal** (Authentication & Authorization) — `decision/unverified-cli-gate` now reads "every relied ledger row counts: pass, or absent on the browser-tools-prompt row alone". The new relied row must read `pass` to count, and absent on it leaves the version unverified unless a ruling says otherwise.

@history_item n=36 marker=2026-11-18-browser-tools-prompt-modal
**2026-11-18-browser-tools-prompt-modal** (Security Anti-Patterns, `decision/dialog-answers-need-verify-stamp`) — a hook.dialog gets a non-null decision, PermissionRequest `allow` included, only on a stamp on which every relied row counts. The new `allow` with `updatedInput` is exactly such a decision and stays behind this gate (the scope's "the dialog decision stays gated").

@history_item n=37 marker=2026-11-18-browser-tools-prompt-modal
**2026-11-18-browser-tools-prompt-modal** (Security Anti-Patterns, `contract/cli-exit-codes/12`) — `answer` refused `unverified-cli` (exit 12) when no stamp counts every relied row. After the new row lands, a home holding only the old 26-row stamp hits this refusal until re-stamped.

@history_item n=38 marker=2026-11-18-the-live-home-s-re-stamp-and-the-product-re-pin
**2026-11-18-the-live-home-s-re-stamp-and-the-product-re-pin** (Threat Model Summary) — `decision/verify-never-answers-dialogs`: verify writes no byte into any dialog, and a standing browser-tools prompt is left for the human. A question probe must answer through the hook body, never by typing into the question modal.

@history_item n=39 marker=2026-11-18-the-live-home-s-re-stamp-and-the-product-re-pin
**2026-11-18-the-live-home-s-re-stamp-and-the-product-re-pin** (Secret Management, the VH arm) — the rig's VH arm runs under the real home and config (a named exception to `decision/live-rig-isolated-child`), PROVISIONAL. The live home's re-stamp belongs to the product re-pin step, and this chunk does no live start and reads no real `~/.claude*`.

@history_item n=40 marker=2026-11-19-context-as-a-local-command
**2026-11-19-context-as-a-local-command** (Security Anti-Patterns) — `decision/stamps-written-only-by-verify` gained the test-only exception that the rig seeds a kept stamp into its own scratch home only. The kept 26-row 2.1.328 stamp in `target/live-rig/wake-stamp.json` is this path's product. Re-stamping in the rig's scratch home stays within it.

@history_item n=41 marker=2026-11-19-context-as-a-local-command
**2026-11-19-context-as-a-local-command** (Data Protection, `decision/live-rig-kept-stamp-owner-only`) — the kept stamp is owner-only and no-clobber, seeded only into the rig's own scratch home, and refused before its ledger line when absent. This governs where a re-stamped 2.1.328/2.1.329 stamp may be kept and seeded.

@history_item n=42 marker=2026-11-19-a-fired-scheduled-wake-keeps-the-wheel
**2026-11-19-a-fired-scheduled-wake-keeps-the-wheel** (Security Anti-Patterns, Data Protection) — widened the stamp-seed exception and the kept-stamp row to CS/WS → CC/R3W, landed PROVISIONAL (delegate overseer, 2026-11-20). A new re-stamp label or arm would need the same row amendments.

@history_item n=43 marker=2026-11-17-first-readings-on-the-standing-rig
**2026-11-17-first-readings-on-the-standing-rig** (Authentication & Authorization) — `decision/live-round-rig-keys-own-pty`: the rig's closed argv key grant needs the founder's word anew per chunk, and viola types into no dialog. Any live start this chunk shapes at P3 for its probe needs that word. No key may answer the question modal.

@history_item n=44 marker=2026-11-20-a-quoted-modal-never-blocks-a-send
**2026-11-20-a-quoted-modal-never-blocks-a-send** (Input Validation, `decision/input-box-readiness-from-literals` rule and except) — readiness now refuses only while a standing modal is up, and a quoted modal literal is no modal. Landed PROVISIONAL. The question dialog is a standing modal while it waits: the probe and the relay must not read it as a ready box.
