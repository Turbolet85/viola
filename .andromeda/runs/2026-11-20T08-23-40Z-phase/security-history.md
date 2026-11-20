@history_item n=1 marker=2026-11-13-verify-readiness-on-the-input-box
**2026-11-13-verify-readiness-on-the-input-box** (Input Validation) — added `+ decision/input-box-readiness-from-literals`: the readiness predicate gained the input box's own rows (rule row of ≥40 rule chars above a prompt-char row) beside the footer literals, "never while a modal literal stands", ONE predicate for verify and `viola run`'s pre-send gate, a ratified boundary widening. This is the exact row the chunk's fix touches: any change to how a "standing modal" is read moves verify's Run B framing pastes too, and changing the predicate's widening needs its own showing to the founder.

@history_item n=2 marker=2026-11-13-verify-readiness-on-the-input-box
**2026-11-13-verify-readiness-on-the-input-box** (Data Protection) — `~ decision/screen-fixture-signature-rows-only`: screen fixtures keep only signature rows (a row holding a compiled footer or MODAL literal whole, a box rule row as rule chars, a prompt row as the prompt char), another box row refused. Bears on the chunk's test inputs: a recorded fixture holds a modal row only as the literal whole, so a "quoting transcript row" screen is not in any recorded set and must be a named synthetic screen, never a fixture that widens this rule.

@history_item n=3 marker=2026-10-04-readiness-gate-and-timing-constants
**2026-10-04-readiness-gate-and-timing-constants** — landed the vt100 feed under `catch_unwind` (tee → feed thread), a caught panic poisoning the screen model so confirmed `send` reads `not-delivered`/`input-not-ready`. The same refusal word the quoted-modal defect produces: the fix must keep the poisoned-model path refusing (guard `chaos_feed_panic_refuses_send_and_keeps_passthrough`) and not conflate it with the modal reading.

@history_item n=4 marker=2026-10-04-confirmed-send-with-cl-1-records
**2026-10-04-confirmed-send-with-cl-1-records** — send's input validation as landed (`validate_paste_text` client-side and first in the wrapper, one bracketed paste) and the feed queue bounded (`FEED_CAPACITY` 256, a dropped copy poisons the model to input-not-ready; the model behind a mutex shared with the gate). Defines the gate's place in the wrapper's refusal order and the model the gate reads; the rows `decision/vt100-feed-bounded-queue`, `decision/vt100-feed-isolated`, `decision/paste-validation-on-both-sides` rest on it.

@history_item n=5 marker=2026-10-05-real-cli-verify-probes
**2026-10-05-real-cli-verify-probes** — verify's interactive children: read only against compiled literals, never a byte into a CLI-native dialog, a modal start killed with no key; screens kept as signature rows only, with the closed unclean-fixture refusal codes (`home-path` · `absolute-path` · `username` · `email`). Origin of the no-key-into-dialog control the chunk's false-negative side protects, and of the fixture refusal contract any new screen input must pass.

@history_item n=6 marker=2026-10-06-local-command-and-paste-framing-rows
**2026-10-06-local-command-and-paste-framing-rows** — Run B's added pastes go "only into rows that hold the input-box literal and no modal literal"; a modal, a poisoned screen and the deadline end the added pastes with no key. Precedent that the modal-literal veto also gates verify's own typing: loosening the modal reading for `viola run`'s gate moves this verify rule through the shared predicate, a widening of the ratified verify child set unless kept apart.

@history_item n=7 marker=2026-10-13-paste-newline-ledger-row
**2026-10-13-paste-newline-ledger-row** — the fifth Run B paste added under the same rule (each paste only into rows holding the input-box literal and no modal literal, none after the stop), a widening ratified by the founder. Restates the modal veto per paste; any change to it is a change to this ratified rule.

@history_item n=8 marker=2026-10-18-compact-post-condition-ledger-row
**2026-10-18-compact-post-condition-ledger-row** — the sixth Run B paste (`/compact`), each added paste only into the settled input box. Another consumer of the readiness/modal reading whose behaviour the chunk must keep.

@history_item n=9 marker=2026-10-19-newer-claude-cli-rows
**2026-10-19-newer-claude-cli-rows** — Run B's pastes now go into rows holding *an* input-box literal and no modal literal (the compiled set gained a second input-box literal); also one scratch footer reading of a newer CLI accepted as no standing permission ("another reading needs its own answer"). Precedent for how the literal set grows by compiled literals only, and a trap: no scratch start or live screen read is licensed for this no-live-start chunk.

@history_item n=10 marker=2026-10-27-bounded-inputs-at-every-boundary
**2026-10-27-bounded-inputs-at-every-boundary** — the vt100 passthrough after a caught feed panic witnessed on the outer terminal by `chaos_feed_panic_refuses_send_and_keeps_passthrough` (the guard of picked `decision/vt100-feed-under-panic-catch`). A test the chunk's gate change must keep green: a poisoned model still refuses the send.

@history_item n=11 marker=2026-11-18-trust-record-row-read-live
**2026-11-18-trust-record-row-read-live** — excepts `decision/screen-fixture-signature-rows-only`, `decision/verify-signature-only` and `decision/verify-screen-read-against-literals` with the hidden rig-only first-screen record of whole scrubbed rows (local, git-ignored), and adds `decision/verify-stamp-first-screen-closed-words` (first screen stamped only as closed words and literal-id facts). Bounds what screen text may leave the process: the chunk's synthetic or quoting screens must not become committed whole-row text.

@history_item n=12 marker=2026-11-18-browser-tools-prompt-modal
**2026-11-18-browser-tools-prompt-modal** — the browser-tools prompt as a modal: the verified rule moved to every relied row "counts" (absent allowed on the browser-tools-prompt row alone) in `decision/unverified-cli-gate` and `decision/dialog-answers-need-verify-stamp`. Names a compiled modal literal the gate must still read as standing, and the current form of two picked rows.

@history_item n=13 marker=2026-11-18-the-live-home-s-re-stamp-and-the-product-re-pin
**2026-11-18-the-live-home-s-re-stamp-and-the-product-re-pin** — `~ decision/verify-never-answers-dialogs`: a browser-tools prompt standing at start is left for the human to answer once in claude. Confirms a standing modal is never typed into; the chunk's "standing modal still refuses" direction must hold for it.

@history_item n=14 marker=2026-11-16-local-command-delivery
**2026-11-16-local-command-delivery** — `~ decision/compiled-classification-sources`: screen signatures, harness prefixes and local-command names come only from compiled ledger rows, never runtime or upstream text. Constraint on the discriminator: it must be built from compiled literals and screen geometry, never from the quoted upstream text itself (also `decision/upstream-text-is-content`).

@history_item n=15 marker=2026-11-19-context-as-a-local-command
**2026-11-19-context-as-a-local-command** — `~ contract/wrapper-channel-methods/send`: a listed local command sent before its transcript exists, or on an unverified CLI, answers ok with `confirmed:false`. The send contract the chunk's gate sits in front of; a send delivered past the fixed gate still answers by this contract.

@history_item n=16 marker=2026-11-16-send-confirmation
**2026-11-16-send-confirmation** — `− decision/delivery-confirmation-exact-match → + decision/delivery-confirmation-trailing-tolerance` (trailing ASCII whitespace run tolerated, nothing else), ratified at 2026-11-16T06-05-33-wrap. The confirmation step the delivered send reaches once the gate no longer refuses; leading trim stays unruled.

@history_item n=17 marker=2026-10-09-epoch-3-cleanup
**2026-10-09-epoch-3-cleanup** — `empty-text` refusal added on both sides, the wrapper's check after the typed text is taken and before the first wheel read. Fixes the wrapper's refusal order around which the input-not-ready gate sits (picked `contract/send-refusal/empty-text`, `contract/channel-refusal/empty-text`); the fix must not reorder it.

@history_item n=18 marker=2026-10-08-first-live-test-and-self-drive
**2026-10-08-first-live-test-and-self-drive** — the human's terminal on `viola run`'s stdin as an attack-surface vector, its closed non-editing list (the except of picked `decision/human-typing-takes-wheel`). A send refused for human typing is a separate refusal from input-not-ready; the chunk must not blur the two.

@history_item n=19 marker=2026-11-19-a-fired-scheduled-wake-keeps-the-wheel
**2026-11-19-a-fired-scheduled-wake-keeps-the-wheel** — added `decision/live-rig-hb-arm-keeps-attributes-only` (HB arm types x and DEL into a fresh box, keeps cell attributes and counts, never screen text) and the told-wake harness relabel exception. The previous chunk's rows nearest to the input box; the HB arm is the live witness of box readiness, not to be run here (no live start).

@history_item n=20 marker=2026-11-17-first-readings-on-the-standing-rig
**2026-11-17-first-readings-on-the-standing-rig** — `~ decision/live-round-rig-keys-own-pty`: the founder's word anew per chunk binds the live rig's closed argv grant; viola types into no dialog. Constraint: this chunk has no live start, so no rig grant is in play and any screen it needs is synthetic.

@history_item n=21 marker=2026-10-17-mcp-server-for-drivers
**2026-10-17-mcp-server-for-drivers** (MCP tool input validation and error bodies) — the MCP `send` tool runs `validate_paste_text` then the empty-typed-text check client-side before any frame; `isError` bodies codes only. The MCP path (picked `contract/mcp-tools/send`) reaches the same wrapper gate and reports its `not-delivered` refusal as a closed code.

@history_item n=22 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** — `sanitised_fault` and `is_code_word`: clients take a refusal or detail only as a code word (picked `decision/client-reads-closed-values-only`, `decision/client-reply-code-words-only`). The input-not-ready refusal reaches CLI and MCP only as its closed word; no new detail word may be added without passing this screen.

@history_item n=23 marker=2026-11-14-hyprland-seam
**2026-11-14-hyprland-seam** — `~ decision/child-output-tolerant` / `decision/child-output-tolerant-unknown`: a child-output failure becomes unknown or a closed failure word. The current form of the picked tolerant-parse rows the screen reading of the claude child falls under.

@history_item n=24 marker=2026-11-19-a-live-stopfailure-payload
**2026-11-19-a-live-stopfailure-payload** — `decision/unverified-cli-gate`'s except settled to "StopFailure is registered with no capability ledger row and no verify probe", its owner dropped, on the founder's word. The current form of the picked gate row; the scope keeps every relied ledger row and verify probe unchanged.

@history_item n=25 marker=2026-11-18-session-folder-trusted-by-viola
**2026-11-18-session-folder-trusted-by-viola** — the excepts of `decision/nothing-written-under-claude-home`, `decision/no-writes-under-claude-home`, `decision/viola-writes-nothing-in-claude-dir` and the `CLAUDE_CONFIG_DIR` env reads now name run and revive's start trust flag. Current form of picked rows; the chunk reads nothing of the operator's real Claude config and must add no write there.

@history_item n=26 marker=2026-11-14-budget-gate-removed
**2026-11-14-budget-gate-removed** — `~ decision/config-cannot-disable-controls` (no config value switches off a control; the budget gate's clause gone). The fix must add no config key, env var or flag that relaxes the pre-send gate (also `decision/controls-not-switchable`, `decision/no-control-switch-off`).
