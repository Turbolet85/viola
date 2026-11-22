@history_item n=1 marker=2026-09-26-local-linux-pre-push-gate
**2026-09-26-local-linux-pre-push-gate** (the `FAKE_AGENT_PUMP_DELAY_MS` carve-out) — added the fake-agent-only u64 ms seam capped at 5 000 that holds the run pump back after the child starts, compiled out of release, configuring nothing; a NEVER for any further seam without its own Decisions Log entry. Matters: this is the seam built to land a host resize in the spawn-to-pump window (decision/fake-agent-pump-delay-seam); a rehearsal of a key after a resize may reuse it, but any new fake-agent seam or env read (e.g. a resize-to-key delay) is a boundary widening needing its own entry and the founder's word (decision/new-env-read-needs-log-entry, decision/test-seams-fake-agent-only).

@history_item n=2 marker=2026-09-27-wrapper-channel
**2026-09-27-wrapper-channel** (founder ratification of three standing boundary widenings) — the founder ratified the pump-delay seam exactly as written and set the standing rule that a widening halts the wrap for a live answer; an earlier delegate direction never ratifies one. Matters: any settle-wait, re-type or rig widening this chunk prices is held to that rule.

@history_item n=3 marker=2026-09-28-hook-perf-gate
**2026-09-28-hook-perf-gate** (test seam `FAKE_AGENT_HOOK_PANIC`) — a second fake-agent seam, ratified live; the seam rows now name two seams and require another seam to carry its own entry. Matters: decision/no-control-switch-off and decision/test-seams-fake-agent-only count exactly these two seams; a third added for a resize rehearsal moves those rows.

@history_item n=4 marker=2026-09-28-capability-ledger-and-viola-verify
**2026-09-28-capability-ledger-and-viola-verify** (`MAX_FRAME` consumers extended, verify's CI split) — the real `claude` CLI runs only locally; verify runs in CI only against the fake agent, its real-CLI probe and record only locally. Matters: decision/real-cli-local-only and decision/ci-no-secrets-no-real-claude bind the measuring arm — the measurement of a lost key on real claude can never be a CI test; CI holds only the fake-agent rehearsal.

@history_item n=5 marker=2026-09-28-mutation-testing-to-the-epoch-boundary
**2026-09-28-mutation-testing-to-the-epoch-boundary** — the founder removed the CI and pre-push mutation legs; mutation testing belongs to the epoch-boundary audit. Matters: decision/mutation-testing-at-audit, and the scope's «No mutation run».

@history_item n=6 marker=2026-10-04-readiness-gate-and-timing-constants
**2026-10-04-readiness-gate-and-timing-constants** (the vt100 degrade on the PTY output row) — a tee writes the human's bytes first and a feed thread runs every feed AND resize under `catch_unwind`; a caught panic poisons the screen model until the host size changes, a poisoned model reads `input-not-ready`; vt100 0.16.2 panics are reachable at real small sizes (24×1 + wide char, 1×1, 1×2); `viola_pty::host_size` never yields a zero size. Matters: this is the resize path's security-side history — the resize itself is fed under the catch and a size change is what un-poisons the model, so a settle wait or read-back after a resize interacts with the poison/unpoison rule (decision/vt100-feed-under-panic-catch, decision/vt100-inside-catch-unwind).

@history_item n=7 marker=2026-10-04-confirmed-send-with-cl-1-records
**2026-10-04-confirmed-send-with-cl-1-records** (send's input validation as landed; the feed bound closed) — `validate_paste_text` runs client-side and first in the wrapper; the paste is one bracketed `write_all` through `viola-pty`'s `PasteHandle`; the tee→feed queue became `sync_channel(256)`, `try_send` after the human's write, a dropped copy poisoning the model until a size change. Matters: the paste write the scope names (`PasteHandle::paste`, the unbounded `write_all` of carry cry-7mgt7ybs) and the bounded feed queue (decision/vt100-feed-bounded-queue) are the exact seams a post-resize fix touches; a re-type fix must keep one bracketed paste and validation first.

@history_item n=8 marker=2026-10-04-the-wheel
**2026-10-04-the-wheel** — pause / release served and validated; the wheel's frames behind the IPC controls. Matters: the wheel the scope lists among touched surfaces; a fix that re-types or refuses after a resize must not move the wheel outside its contract (contract/wheel-event/human-input, contract/send-refusal/human-typing).

@history_item n=9 marker=2026-10-05-real-cli-verify-probes
**2026-10-05-real-cli-verify-probes** — verify's interactive children: direct spawns under the R8 strip at a fixed 80×24, output capped at `MAX_FRAME` into a screen under `catch_unwind`, no byte into any CLI-native dialog, a modal start killed with no key; the founder's word given only after each widening was shown. Matters: verify's children never resize (fixed 80×24) and never type into a dialog (decision/verify-children-direct-spawn, decision/verify-writes-no-dialog-byte); any verify-side measuring probe of a resize is a widening of the ratified child set.

@history_item n=10 marker=2026-10-06-local-command-and-paste-framing-rows
**2026-10-06-local-command-and-paste-framing-rows** — Run B's added pastes go only into rows holding the input-box literal and no modal literal; waits bounded by `PROBE_DEADLINE`; a modal, a poisoned screen and the deadline end the pastes with no key. Precedent: every added Run B paste was a founder-ratified widening of the verify child set, and the wait's lengthening was shown to him.

@history_item n=11 marker=2026-10-07-live-rows-and-paste-shapes-on-the-dev-host
**2026-10-07-live-rows-and-paste-shapes-on-the-dev-host** — two scratch dev-host measurements crossed the credential boundary once each, on the founder's live answers; standing rule: the answers are spent, another probe needs its own showing. Matters: the scope's «no live start this phase; the start cap is the operator's word» rests on this rule — the lost-key measurement needs its own word.

@history_item n=12 marker=2026-10-07-a-send-ending-in-a-newline-is-confirmed
**2026-10-07-a-send-ending-in-a-newline-is-confirmed** — reject, don't strip stands; the typed text (trailing LF removed after validation) is the only text `send` uses for classification, confirmation, byte count and paste; standing rule: any other removal from a sent text needs its own ruling. Matters: a «read-back and one re-type» fix must re-type the typed text alone and never alter it (decision/refused-text-never-stripped, decision/reject-dont-strip-paste-text).

@history_item n=13 marker=2026-10-08-first-live-test-and-self-drive
**2026-10-08-first-live-test-and-self-drive** — new attack-surface vector: every byte the terminal writes to `viola run`'s stdin; the stdin classifier's closed non-editing list widened once by seven measured terminal reply shapes, among them `CSI 4;n;n t`, `6;n;n t`, `8;n;n t` and `48;n;n;n;n t` (size reports); no reply added unmeasured; the classifier reads bytes, not time; agent-typed keys via `wtype` on a nested compositor ratified once. Matters: a resize provokes terminal size replies on stdin — a reply not on the list takes the wheel as typing (decision/human-typing-takes-wheel, decision/stdin-closed-non-editing-list, decision/non-editing-list-measured-only); a key «lost» after a resize may be a classifier/wheel effect, and any new reply needs measurement, never a guess.

@history_item n=14 marker=2026-10-09-epoch-3-cleanup
**2026-10-09-epoch-3-cleanup** (trailing CR and LF removed; an empty typed text refused) — the one removal is every trailing CR and LF; `empty-text` refused by client before any frame and by the wrapper before the first wheel read. Matters: contract/send-refusal/empty-text and decision/empty-typed-text-refused order a re-type or refusal fix against.

@history_item n=15 marker=2026-10-09-inner-cr-and-crlf-in-a-sent-text
**2026-10-09-inner-cr-and-crlf-in-a-sent-text** — inner CR / CRLF typed as one LF after validation; «a later change that types a character the received text did not hold, other than this LF, would be a new class and needs its own ruling». Matters: decision/typed-text-line-ending-normalisation; a resize fix typing any extra byte (a redraw nudge, a filler key) is that new class.

@history_item n=16 marker=2026-10-13-paste-newline-ledger-row
**2026-10-13-paste-newline-ledger-row** — Run B's fifth compiled paste and a new relied ledger row moved the stamp count; a widening of the ratified verify child set on the founder's answer to a P4 fork. Precedent for «crates/viola-agent-claude — only if the answer becomes a relied row»: a new relied row moves the stamp gate (decision/unverified-cli-gate, decision/dialog-answers-need-verify-stamp, contract/cli-exit-codes/12) and its probe is a widening.

@history_item n=17 marker=2026-10-17-cli-output-discipline
**2026-10-17-cli-output-discipline** — `NO_COLOR` / `TERM` / `COLORTERM` admitted, read in every build for colour only; `COLUMNS` etc. not read; the width is `viola_pty::host_size()`; NEVER another environment read without its own Decisions Log entry. Matters: a resize fix must take size from `host_size`, never a new env read (decision/new-env-read-needs-log-entry, decision/controls-not-switchable).

@history_item n=18 marker=2026-10-17-optional-plugin-install
**2026-10-17-optional-plugin-install** (the live round's residual under ~/.claude) — a live round runs the CLI's own commands from a rig dir; the CLI's writes under `~/.claude` are its own; viola writes nothing there. Precedent of the accepted live-round residual class (decision/nothing-written-under-claude-home).

@history_item n=19 marker=2026-10-18-plugin-precedence-live-readings
**2026-10-18-plugin-precedence-live-readings** (the live round's rig outside the work tree, its trust keys) — the rig passes the folder-trust dialog by keys into its own pty only, never a desktop key: at most 2 arrows and 1 Enter only on a read `selected`, any other screen no key; rig outside the work tree. Matters: origin of decision/live-round-rig-keys-own-pty, the bound on what keys a measuring arm may type.

@history_item n=20 marker=2026-10-19-newer-claude-cli-rows
**2026-10-19-newer-claude-cli-rows** (the scratch footer reading of a newer CLI) — one scratch `viola run` start in a detached 80×24 pane, no key typed, bottom rows scrubbed into evidence, cwd removed; «no product path and no standing permission: another reading needs its own answer». Precedent for pricing the cheapest live start in the P3 block.

@history_item n=21 marker=2026-10-27-pinned-copy-race-and-reproducible-builds
**2026-10-27-pinned-copy-race-and-reproducible-builds** — the pinned `viola` placed once by a no-clobber placement with its write handle closed before the name appears (ETXTBSY measured), never replaced. Matters: decision/pinned-binary-placed-once and the scope's «pinned product untouched; re-pin outside this chunk».

@history_item n=22 marker=2026-10-27-bounded-inputs-at-every-boundary
**2026-10-27-bounded-inputs-at-every-boundary** — the vt100 passthrough after a caught feed panic witnessed on the outer terminal (`chaos_feed_panic_refuses_send_and_keeps_passthrough`); the bounded-reader census `product_readers_are_each_bounded_or_named` over product sources. Matters: decision/bounded-readers-census — any new read on the resize/read-back path must be bounded or named; the feed-panic witness guards the resize-poison path.

@history_item n=23 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** — clients take a refusal or detail only as a code word and a fault only through `sanitised_fault`; `UNUSABLE_RESPONSE` / `UNUSABLE_TEXT` stderr lines named. Matters: decision/client-reads-closed-values-only, decision/client-reply-code-words-only, decision/unusable-response-line-on-stderr — a «refusal» fix must surface as a closed code word.

@history_item n=24 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** — the closed exit causes; each new closed word in `detail` was the founder's own word given after the widening was shown. Precedent: a new refusal word for a key lost after a resize is a widening of a closed set needing his word.

@history_item n=25 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (the ConPTY sideload retired; verify's stop and colour lose Windows arms) — Linux-only: `pty_backend()` always `openpty`, ConPTY and its resize path gone; the feed-panic witness reads the child's bytes on the outer terminal with no OS limit. Matters: the measurement and any fix are Linux/openpty only; the «real claude on Linux» framing of the working entry.

@history_item n=26 marker=2026-11-06-version-done-check
**2026-11-06-version-done-check** (portable-pty pinned =0.9.0) — portable-pty moved to `=0.9.0`, `viola-pty` compiling unchanged, `pty_backend()` stays `openpty`; handle inheritance measured. Matters: the PTY resize call viola makes goes through portable-pty 0.9.0; a fix in `viola-pty` stands on that pin.

@history_item n=27 marker=2026-11-13-verify-readiness-on-the-input-box
**2026-11-13-verify-readiness-on-the-input-box** — added decision/input-box-readiness-from-literals (the box's own rows beside the footer literals, one predicate for verify and run's pre-send gate), a ratified widening; screen fixtures keep only signature rows. Matters: a read-back after a resize would read through this predicate; «empty input box, never a dialog» is defined by it.

@history_item n=28 marker=2026-11-14-hyprland-seam
**2026-11-14-hyprland-seam** — decision/child-output-tolerant and decision/child-output-tolerant-unknown now let a failure become a closed failure word; decision/hyprctl-read-deadline adds a spawned child bounded by a deadline applied ahead of its settlement against a measured need, PROVISIONAL. Precedent for a settle-wait bound set before its measurement.

@history_item n=29 marker=2026-11-15-live-round-on-the-trimmed-product
**2026-11-15-live-round-on-the-trimmed-product** — added decision/live-round-rig-keys-own-pty (rig keys only into its own pty, the founder's word per chunk) and decision/live-session-screens-kept-local (live screens only under a gitignored local path, a closed-word summary per frame landing). Matters: any key typed into real claude by a rig and the frames it reads to tell a lost key from a kept one are bound by both rows.

@history_item n=30 marker=2026-11-15-api-error-turn-end
**2026-11-15-api-error-turn-end** — decision/unverified-cli-gate gained its StopFailure exception (no ledger row, no verify probe). Matters: the gate's exception is the precedent for a capability registered without a relied row.

@history_item n=31 marker=2026-11-16-send-confirmation
**2026-11-16-send-confirmation** — decision/delivery-confirmation-exact-match replaced by decision/delivery-confirmation-trailing-tolerance (trailing ASCII space, TAB, CR, LF tolerated; no leading trim). Matters: a key lost after a resize shows as a confirmation mismatch; this row decides what a read-back counts as kept.

@history_item n=32 marker=2026-11-16T06-05-33-wrap
**2026-11-16T06-05-33-wrap** — the founder ratified the trailing-only tolerance; «a leading trim ("either end") waits for the live reading carried on Standing live rig with a screen model»; StopFailure's no-ledger-row exception ratified. Matters: a lost leading key is exactly the difference a leading trim would hide — the founder held it for a live reading, so no fix may widen the match without it.

@history_item n=33 marker=2026-11-16-local-command-delivery
**2026-11-16-local-command-delivery** — decision/compiled-classification-sources: a typed text is a local command only by its compiled name; transcript read bounded and shape-checked. Matters: a dropped first key turns `/clear` into `clear` — classification uses the typed text only, never a screen read-back.

@history_item n=34 marker=2026-11-17-standing-live-rig
**2026-11-17-standing-live-rig** — decision/live-round-rig-keys-own-pty reworded for the standing rig's closed argv grant (≤2 down-arrows, Enter only on a read trust row, one x and one DEL in a ready box); added decision/live-rig-credentials-copy and decision/live-rig-isolated-child. Matters: a measuring arm typing a key after a resize falls outside the granted keys unless the founder widens the grant; the rig's credential copy and isolated child bound its start.

@history_item n=35 marker=2026-11-17-first-readings-on-the-standing-rig
**2026-11-17-first-readings-on-the-standing-rig** — the rig's stop (Ctrl-C into its own pty or a no-key group kill) needs no word, the argv grant does; decision/live-rig-isolated-child moved to the mark witness on `~/.claude.json`; decision/live-rig-prompt-capture-closed added. Matters: the per-chunk word covers only the key grant; the home/config a measuring start runs under is the isolated-child row's.

@history_item n=36 marker=2026-11-17-trust-record-ledger-row
**2026-11-17-trust-record-ledger-row** — decision/nothing-written-under-claude-home and decision/no-writes-under-claude-home gained verify's trust-record exception; decision/explicit-mode-before-write gained the trust-record writer's except; decision/env-not-config-channel and decision/no-stored-secrets gained CLAUDE_CONFIG_DIR's value read. Matters: the exact excepts of rows picked for this chunk; a measuring start must stay inside them.

@history_item n=37 marker=2026-11-18-trust-record-row-read-live
**2026-11-18-trust-record-row-read-live** — decision/verify-signature-only and decision/verify-screen-read-against-literals gained the rig-only first-screen record except (whole scrubbed rows, local, git-ignored); decision/live-rig-seed-from-bundle-logic added. Matters: the only path by which row text leaves verify; telling a lost key needs a closed word, not a text.

@history_item n=38 marker=2026-11-18-session-folder-trusted-by-viola
**2026-11-18-session-folder-trusted-by-viola** — run and revive's start set the trust flag of an untrusted spawn folder (excepts of decision/nothing-written-under-claude-home, decision/no-writes-under-claude-home, decision/viola-writes-nothing-in-claude-dir); CLAUDE_CONFIG_DIR's value read at the start too; seed rule reworded. Matters: a `viola run` measuring start writes this trust flag — part of the residual the P3 block must price.

@history_item n=39 marker=2026-11-18-browser-tools-prompt-modal
**2026-11-18-browser-tools-prompt-modal** — decision/unverified-cli-gate, decision/dialog-answers-need-verify-stamp and contract/cli-exit-codes/12 now count absent on the browser-tools-prompt row; seed rule gained the VP arm. Matters: if the answer becomes a relied row, these three rows define what a covering stamp is.

@history_item n=40 marker=2026-11-18-the-live-home-s-re-stamp-and-the-product-re-pin
**2026-11-18-the-live-home-s-re-stamp-and-the-product-re-pin** — decision/verify-never-answers-dialogs now says the browser-tools prompt is left for the human; the VH arm (real home, no credentials copy) is the one exception to decision/live-rig-isolated-child and decision/live-rig-credentials-copy. Matters: «the home and config it runs under» for the measurement — the real home is allowed only through the VH arm's bounds.

@history_item n=41 marker=2026-11-19-a-live-stopfailure-payload
**2026-11-19-a-live-stopfailure-payload** — decision/live-rig-sf-arm-network-closed added; the StopFailure exception settled on one contained live reading, «nothing claimed for the other twelve words or another version». Precedent for a mixed or single reading's scope: one reading on one version claims nothing beyond it.

@history_item n=42 marker=2026-11-19-context-as-a-local-command
**2026-11-19-context-as-a-local-command** — contract/wrapper-channel-methods/send: a listed local command sent before its transcript exists, or on an unverified CLI, answers ok with confirmed false (unconfirmable). Matters: the send contract a post-resize read-back or refusal must keep.

@history_item n=43 marker=2026-11-19-a-fired-scheduled-wake-keeps-the-wheel
**2026-11-19-a-fired-scheduled-wake-keeps-the-wheel** — added decision/live-rig-hb-arm-keeps-attributes-only: the HB arm types one x and one DEL into a fresh box in its own unwrapped pty before any send and keeps only cell attributes and counts. Matters: the nearest existing arm that types keys into real claude's box — carry cry-lqe3bfwx (DEL erases) and carry cry-234cbfoa (the intermittent x-and-DEL rehearsal red) sit on it; a resize-then-key arm is its sibling and keeps the same no-text bound.

@history_item n=44 marker=2026-11-20-a-quoted-modal-never-blocks-a-send
**2026-11-20-a-quoted-modal-never-blocks-a-send** — decision/input-box-readiness-from-literals moved to «never while a standing modal is up» with its quote except; decision/verify-reads-only-signatures, decision/verify-pty-output-caught, decision/verify-screen-read-against-literals and decision/verify-signature-only now read «the literals, and the compiled shapes over them». Matters: the current wording of the readiness predicate and signature-read rows any read-back after a resize uses.

@history_item n=45 marker=2026-11-20-harness-prefix-and-identity-floor-ledger-rows
**2026-11-20-harness-prefix-and-identity-floor-ledger-rows** — identity floor grew to thirteen names; capture arm's env-names read on SessionStart. Matters: decision/parent-identity-env-stripped as it stands for any measuring child spawn.

@history_item n=46 marker=2026-11-21-cross-session-prefix-ledger-row
**2026-11-21-cross-session-prefix-ledger-row** — decision/verify-children-direct-spawn gained Run B's compiled accept settings literal and the sender's composed uds address. Matters: the current wording of the verify child-set row a verify-side probe would widen.

@history_item n=47 marker=2026-11-21-mcp-calls-bounded
**2026-11-21-mcp-calls-bounded** — contract/mcp-iserror/instance-unreachable and contract/instance-unreachable-cause/call-deadline: MCP frame tool calls bounded by a compiled deadline, `call-deadline` on MCP alone. Matters: the last chunk's time bound on who waits; a settle wait added to `send` runs inside the MCP call deadline and must not push a send past it.
