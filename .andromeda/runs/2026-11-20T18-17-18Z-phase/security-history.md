@history_item n=1 marker=2026-09-25-pty-wrapper-on-windows
**2026-09-25-pty-wrapper-on-windows** — (R8 persistent-environment exemption) set R8 as the prefix rule plus persistent exemption plus the 11-name identity floor (both messaging variables on it), names only; `claude_env_keep` capped at 32 `^CLAUDE[A-Z0-9_]*$` names, a floor name ineffective. Origin of `IDENTITY_FLOOR` and of decision/r8-claude-env-strip, contract/config-json/claude-env-keep and decision/config-cannot-disable-controls, which the identity-floor ledger row will now pin.

@history_item n=2 marker=2026-09-27-hooks-to-normalised-events
**2026-09-27-hooks-to-normalised-events** — (`hook.event` served) the wrapper re-validates `prompt-submitted` `text` as a string and `origin` ∈ harness|human, an invalid event not appended. This is contract/channel-frames/hook-event-revalidation, the channel side of the harness classification this chunk's probe exercises.

@history_item n=3 marker=2026-09-28-capability-ledger-and-viola-verify
**2026-09-28-capability-ledger-and-viola-verify** — (the hidden `hook --capture` arm, verify's probe captures and the fixture scrub) created the capture arm (absolute existing DIR, `take(MAX_FRAME + 1)`, raw 0600 write, no `VIOLA_*`, no channel, exit 0), transient 0600 captures in a 0700 per-pid probe dir, and `--record`'s scrub and whole-recording refusal; a founder-ratified widening that no other caller may register. The env-names capture and its `--record` handling of a name carrying private text extend exactly this surface. A sibling entry of the same chunk put the real CLI and `--record` local-only and verify in CI against the fake agent only (decision/real-cli-local-only).

@history_item n=4 marker=2026-09-29-fake-agent-drift-contract
**2026-09-29-fake-agent-drift-contract** — (The cross-session tag filed as harness origin) fixed the harness classifier as four compiled literals on the raw start, no trim (`<agent-message from=`, `<task-notification>`, `<\cross-session-message`, `<cross-session-message`), with the ratified side effect that a human typing the escaped form is filed harness; condition: the first live test measures it and owns the ledger row. This is decision/harness-prefix-literals and the ledger row this chunk delivers.

@history_item n=5 marker=2026-10-04-confirmed-send-with-cl-1-records
**2026-10-04-confirmed-send-with-cl-1-records** — (send's input validation as landed) `validate_paste_text` on both sides, first in the wrapper; one bracketed `write_all`; `send` params `-32602` before the paste check; send text capped at `MAX_FRAME`; vt100 feed queue bounded at 256. Any paste a harness-prefix probe types passes these rows (paste-validation-on-both-sides, channel-frames/send-params, vt100-feed-bounded-queue).

@history_item n=6 marker=2026-10-04-dialog-answers-by-dialog-id
**2026-10-04-dialog-answers-by-dialog-id** — (run's strict stamps read; decisions on the six-row stamp until :82) `run`'s gate reads stamps through the strict check (`cli_verified:false` with `parse-rejected{ledger-stamps, strict-modes-failed}` on refusal), and the precedent that a stamp missing relied rows leaves a body-shape change uncaught until the rows and their re-probe land. Rests under version-gate-strict-stamps-read and contract/strict-modes-refusal/version-gate; adding relied rows re-opens the same question for older stamps.

@history_item n=7 marker=2026-10-05-real-cli-verify-probes
**2026-10-05-real-cli-verify-probes** — (verify's typed-input PTY probe) verify's interactive children are direct spawns under the R8 strip at 80×24, output capped into a `Screen` under `catch_unwind`, no byte into any dialog, a modal start killed with no key; fixture refusal became `a recorded fixture is not clean: <file> <code>` with codes `home-path` · `absolute-path` · `username` · `email`; accepted transcript residual. Each new probe in the child set is a boundary widening needing the founder's live word.

@history_item n=8 marker=2026-10-05-dialog-rows-and-re-probe
**2026-10-05-dialog-rows-and-re-probe** — (the capture arm answers, Runs C and D) the capture arm claims files exclusively, `--answers` read through `take(64)` into closed `ProbeAnswer` ids, body only via `dialog::decision_body`; Runs C/D added. Source of decision/capture-answers-closed-probe-ids and decision/hook-capture-arm; a probe answering by hook output, never a key, is the ratified form.

@history_item n=9 marker=2026-10-06-local-command-and-paste-framing-rows
**2026-10-06-local-command-and-paste-framing-rows** — (Run B's four compiled pastes) added `PROBE_TAG_PASTE` among compiled literals pasted only into a ready box, waits up to `PROBE_DEADLINE`, one more transcript; stamp `/17`, an all-pass stamp of the older ids reading `cli_verified:false`. Precedent for a tag-like paste probe and for how a stamp grows when relied rows are added.

@history_item n=10 marker=2026-10-07-live-rows-and-paste-shapes-on-the-dev-host
**2026-10-07-live-rows-and-paste-shapes-on-the-dev-host** — (two scratch measurements across the credential boundary) one synthetic peer message sent through the CLI's own messaging into a scratch probe session, and a scratch hook's names-only `CLAUDE*` list read against the floor; 2.1.287 hands a hook 12 names, 4 outside the eleven, 3 of the eleven absent; the floor stays eleven. Standing rule: those answers are spent; another peer message or another names probe needs its own showing. Directly governs both of this chunk's probes (a second session sending into a verify child; env names captured) and decision/identity-floor-eleven-names.

@history_item n=11 marker=2026-10-07-a-send-ending-in-a-newline-is-confirmed
**2026-10-07-a-send-ending-in-a-newline-is-confirmed** — (trailing LF removed after validation) "reject, don't strip" stands for refused characters; the typed text alone is used for classification, the match, byte count and paste; any other removal needs its own ruling. Rests under decision/reject-dont-strip-paste-text and refused-text-never-stripped.

@history_item n=12 marker=2026-10-09-epoch-3-cleanup
**2026-10-09-epoch-3-cleanup** — (trailing CR and LF removed; an empty typed text refused) the `empty-text` refusal on both sides before any frame or typing (contract/send-refusal/empty-text, decision/empty-typed-text-refused).

@history_item n=13 marker=2026-10-09-inner-cr-and-crlf-in-a-sent-text
**2026-10-09-inner-cr-and-crlf-in-a-sent-text** — typed text normalises every CR LF and lone CR to LF and drops trailing CR/LF after validation; typing a character the received text did not hold, other than this LF, is a new class needing its own ruling. Binds any probe paste (decision/typed-text-line-ending-normalisation).

@history_item n=14 marker=2026-10-13-interrupted-verify-cleanup
**2026-10-13-interrupted-verify-cleanup** — verify's probe dirs removed on pass, fail, SIGINT/SIGTERM by guards removing only what that run created, never by pattern; a stopped Run B killed with no key and no later run started. Any new probe dir or session this chunk adds falls under decision/verify-probe-dirs-cleanup and probe-artifacts-cleanup.

@history_item n=15 marker=2026-10-13-paste-newline-ledger-row
**2026-10-13-paste-newline-ledger-row** — (Run B's fifth compiled paste, eighteen stamp rows) precedent for a new relied row with its probe: a widening of the ratified verify child set, ratified by the founder's own P4 answer; stamp counter moved and older stamps read unverified.

@history_item n=16 marker=2026-10-13-owed-ledger-rows
**2026-10-13-owed-ledger-rows** — (Run B's probe --settings, the statusline capture arm, the relied set) precedent for owed relied rows landing with a new hidden capture arm (closed verdict word, the observed word written nowhere) as a widening of the verify child set; the closest model for an env-names capture that keeps a closed verdict, not the value.

@history_item n=17 marker=2026-10-14-resume-and-compact-ledger-rows
**2026-10-14-resume-and-compact-ledger-rows** — (Run E in the verify child set) a new interactive child and its id source widen the ratified child set and the session-id crossing, ratified by the founder's P4 answer; measured transcript residual. Precedent if the harness-prefix probe adds a second session or start.

@history_item n=18 marker=2026-10-16T14-58-10-wrap
**2026-10-16T14-58-10-wrap** — (the loopback GUI leaves 0.1.0) the only secrets are the inherited `CLAUDE_CODE_MESSAGING_TOKEN` / `_SOCKET` and the R8-stripped `CLAUDE*` values; the NEVER-log floor names them. Bears on decision/messaging-credentials-are-secret and no-messaging-secrets-in-outputs, which the env-names probe must keep.

@history_item n=19 marker=2026-10-16-remove-the-browser-leg
**2026-10-16-remove-the-browser-leg** — the artifact canary scan's classes are the twelve stripped `CLAUDE*` `claude-stripped` needles, the content canary and the diagnostics mode; matched bytes never printed. contract/artifact-canary-scan-classes/claude-stripped is the CI guard any captured env value would trip.

@history_item n=20 marker=2026-10-16-unwrapped-sessions-on-the-board
**2026-10-16-unwrapped-sessions-on-the-board** — Run B's `claude agents --json` read: direct spawn under the R8 strip, 5 s kill, tolerant parse into closed `oversize`/`malformed`; spawn pairs counted; stamp `/23`. Source of decision/claude-agents-read-fail-open and child-output-bounded; a second session found through agents would read through it.

@history_item n=21 marker=2026-10-17-cli-output-discipline
**2026-10-17-cli-output-discipline** — (terminal colour facts join the admitted env set) the standing ban: NEVER another seam or environment read without its own Decisions Log entry (decision/new-env-read-needs-log-entry, test-seams-fake-agent-only). An env-names read by a hook or probe is such a read and needs its entry.

@history_item n=22 marker=2026-10-18-compact-post-condition-ledger-row
**2026-10-18-compact-post-condition-ledger-row** — (Run B's sixth paste, its residual, and the twenty-four-row stamp) latest model of a relied row added through a Run B paste, ratified as a child-set widening, its measured transcript residual accepted only as named.

@history_item n=23 marker=2026-10-18-plugin-precedence-live-readings
**2026-10-18-plugin-precedence-live-readings** — (the live round's rig outside the work tree, its trust keys and residual) live rounds run from a `proj/` outside the work tree; rig keys into its own pty only; the CLI's own writes under `~/.claude` are the residual, viola writing nothing there. Precedent for what a live stamping round leaves in the user's Claude config.

@history_item n=24 marker=2026-10-19-overseer-switch-to-viola
**2026-10-19-overseer-switch-to-viola** — (verify's print-mode probe residual accepted) the CLI's empty project dir under `~/.claude/projects/` accepted into the trusted-runs class, as measured. A new probe's residual is accepted only as measured (decision/verify-real-cli-transcripts-accepted, verify-trusted-runs-risk).

@history_item n=25 marker=2026-10-19-newer-claude-cli-rows
**2026-10-19-newer-claude-cli-rows** — (the scratch footer reading of a newer CLI accepted) one tmpfs-home start, no key, scrubbed footer rows only; no standing permission, another reading needs its own answer (decision/scratch-footer-reading-residual). Relevant if 2.1.329 is read before being stamped.

@history_item n=26 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** — (`ledger/stamps.json` checked before `ledger/` is made, and `verify`'s home tree before its log; `viola hook`'s event and dialog paths check the instance before obs init) `update_stamps` checks before create and under lock, refusal reads and writes nothing; verify's home-tree check before its log; hook event/dialog run `check_instance` before obs init, the capture arms staying ahead of it. Rows stamps-checked-under-lock, verify-home-tree-check, hook-instance-check-first, strict-check-before-narrowing.

@history_item n=27 marker=2026-10-27-bounded-inputs-at-every-boundary
**2026-10-27-bounded-inputs-at-every-boundary** — (the bounded-reader census) `product_readers_are_each_bounded_or_named` requires every product reader bounded or named with a witness; a new capture or probe read-back must be bounded or the census reads red (decision/bounded-readers-census).

@history_item n=28 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** — (verify's refusal codes; the catch site's document under --json) verify writes closed refusal codes (`program-not-found`, `pinned-hash-mismatch`, `version-unreadable`, `strict-modes-failed`, `unclean-fixture`), the fixture's file name entering no code; the catch site's `internal-error` document. Picked contracts verify-refusal-codes/* and internal-error-output/json.

@history_item n=29 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** — (the R8 strip and the fixture scrub match names exactly; the statusline shell-out, the settings override and the relied set hold on every build) R8 strips exact names, `claude_env_keep` the pass-list, the identity floor stays the eleven (Windows-host measurement and the Linux reading kept beside it); the scrub matches the home prefix exactly; `LedgerRow::relied()` is one set on every build. The identity-floor row now pins that eleven-name set on Linux.

@history_item n=30 marker=2026-11-13-verify-readiness-on-the-input-box
**2026-11-13-verify-readiness-on-the-input-box** — ~ decision/screen-fixture-signature-rows-only (signature rows only, box rows as their literals); + decision/input-box-readiness-from-literals. Any new recorded screen and any paste into a verify child are held to these.

@history_item n=31 marker=2026-11-14-status-line-interception-removed
**2026-11-14-status-line-interception-removed** — ~ decision/hook-unknown-event-silent-exit, ~ decision/capture-arms-isolated (one capture arm), ~ decision/hook-instance-check-first, ~ decision/max-frame-shared-cap, ~ decision/hook-silent-exit-zero, ~ decision/hook-strict-modes-first (the capture arm runs ahead of the check), ~ contract/cli-state-root/home-flag and viola-dir, and the `except` of decision/fixture-scrub-and-refusal and decision/fixture-hygiene emptied (no `workspace.repo` kept). Any second capture arm or env-names arm changes these counts and clauses.

@history_item n=32 marker=2026-11-15-live-round-on-the-trimmed-product
**2026-11-15-live-round-on-the-trimmed-product** — + decision/live-round-rig-keys-own-pty, + decision/live-session-screens-kept-local: a live round's keys bounded and granted per chunk; live screens kept only under a gitignored local path. Binds the «stamped live» round.

@history_item n=33 marker=2026-11-15-api-error-turn-end
**2026-11-15-api-error-turn-end** — ~ decision/hook-unknown-event-silent-exit (ten events), ~ contract/channel-frames/hook-event-revalidation (turn-end cause), ~ decision/unverified-cli-gate except: StopFailure registered with no ledger row and no verify probe. The one named exception to «every relied behaviour is a ledger row with a probe», the invariant this chunk discharges for two more behaviours.

@history_item n=34 marker=2026-11-16-send-confirmation
**2026-11-16-send-confirmation** — − decision/delivery-confirmation-exact-match → + decision/delivery-confirmation-trailing-tolerance; ~ decision/typed-text-line-ending-normalisation why. How a sent probe text is confirmed as the driver's.

@history_item n=35 marker=2026-11-16-local-command-delivery
**2026-11-16-local-command-delivery** — ~ decision/compiled-classification-sources: screen signatures, harness prefixes and local-command names come only from compiled `viola-agent-claude` ledger rows, never runtime or upstream text; + decision/transcript-path-redacted; + decision/verify-echo-probe-residuals. The harness-prefix ledger row must stay the compiled source this rule names.

@history_item n=36 marker=2026-11-16-granular-api-error-cause
**2026-11-16-granular-api-error-cause** — ~ contract/channel-frames/hook-event-revalidation: turn-ended cause in the closed nine-word set. The row's current promise, beside the prompt-submitted origin harness|human check.

@history_item n=37 marker=2026-11-17-standing-live-rig
**2026-11-17-standing-live-rig** — ~ decision/live-round-rig-keys-own-pty (closed argv grant), + decision/live-rig-credentials-copy, + decision/live-rig-isolated-child (private `CLAUDE_CONFIG_DIR`, closed kept env list). The rig that will stamp the new rows live, and its env kept-list bears on what the identity-floor probe sees.

@history_item n=38 marker=2026-11-17-first-readings-on-the-standing-rig
**2026-11-17-first-readings-on-the-standing-rig** — ~ decision/live-rig-isolated-child (user config read only after a moved stamp, by rig-made needles, one closed word); ~ decision/live-round-rig-keys-own-pty (stop needs no word, the argv grant does); + decision/live-rig-prompt-capture-closed (key names and closed words, never a value). Model for an env-names capture that keeps names, never values.

@history_item n=39 marker=2026-11-17-trust-record-ledger-row
**2026-11-17-trust-record-ledger-row** — ~ decision/r8-claude-env-strip, credential-strip-by-name-only, env-not-config-channel, no-stored-secrets except: verify reads `CLAUDE_CONFIG_DIR`'s value only when kept; + decision/trust-record-writer-guarded, + decision/trust-record-closed-words; probe cleanup excepts the trust record. The latest precedent of a relied row whose probe widened the env-value read and the Claude-config write, each as a named exception.

@history_item n=40 marker=2026-11-18-trust-record-row-read-live
**2026-11-18-trust-record-row-read-live** — ~ decision/screen-fixture-signature-rows-only, ~ decision/verify-signature-only, ~ decision/verify-screen-read-against-literals except: the rig-only first-screen file of whole scrubbed rows, local and git-ignored; + decision/verify-stamp-first-screen-closed-words; + decision/live-rig-seed-from-bundle-logic. What a re-recorded stamp holds of first screens and how the rig's live stamping round may write.

@history_item n=41 marker=2026-11-18-session-folder-trusted-by-viola
**2026-11-18-session-folder-trusted-by-viola** — ~ decision/no-writes-under-claude-home, viola-writes-nothing-in-claude-dir, nothing-written-under-claude-home except (verify's trust record; the start's trust flag); ~ the `CLAUDE_CONFIG_DIR` value-read excepts to run and revive's start; ~ decision/trust-record-writer-guarded, ~ decision/trust-record-closed-words. The current form of every Claude-config write and env-value read the new probes must stay within.

@history_item n=42 marker=2026-11-18-browser-tools-prompt-modal
**2026-11-18-browser-tools-prompt-modal** — ~ decision/unverified-cli-gate, ~ decision/dialog-answers-need-verify-stamp, ~ contract/cli-exit-codes/12: verified means every relied row counts, pass or absent on the browser-tools-prompt row alone; ~ live-rig-seed-from-bundle-logic. Adding two relied rows changes what «verified» rests on for every stamp, so older stamps (the live home's 24-row stamp) read unverified.

@history_item n=43 marker=2026-11-18-the-live-home-s-re-stamp-and-the-product-re-pin
**2026-11-18-the-live-home-s-re-stamp-and-the-product-re-pin** — ~ decision/live-rig-isolated-child and live-rig-credentials-copy except (the VH arm under the real home, no credentials read); ~ decision/verify-never-answers-dialogs (browser-tools prompt left for the human). The live home's re-stamp stays that re-pin step's, as the scope says.

@history_item n=44 marker=2026-11-19-a-live-stopfailure-payload
**2026-11-19-a-live-stopfailure-payload** — + decision/live-rig-sf-arm-network-closed; ~ decision/live-rig-prompt-capture-closed (StopFailure error word of 13 else other, lengths, never a text); ~ decision/unverified-cli-gate except kept on the founder's word on one live reading, nothing claimed for other versions. A live reading settles one behaviour on one version only.

@history_item n=45 marker=2026-11-19-context-as-a-local-command
**2026-11-19-context-as-a-local-command** — + decision/verify-context-probe-residual (Run B's eighth paste, residual accepted only as measured), + decision/live-rig-config-top-names-only, + decision/live-rig-kept-stamp-owner-only; ~ contract/wrapper-channel-methods/send; ~ decision/stamps-written-only-by-verify except (rig seeds a kept stamp into its own scratch home). The most recent relied-row probe added to Run B, and the kept-stamp mechanism a live stamp of 2.1.328/2.1.329 runs through.

@history_item n=46 marker=2026-11-19-a-fired-scheduled-wake-keeps-the-wheel
**2026-11-19-a-fired-scheduled-wake-keeps-the-wheel** — ~ decision/harness-prefix-literals except: the wrapper relabels a keyless human-filed prompt told as a CLI-scheduled wake origin harness; ~ decision/stamps-written-only-by-verify except (CC or R3W); ~ decision/live-rig-kept-stamp-owner-only; + decision/live-rig-wake-arm-reads-own-wheel-lines (key names, never values). The harness classification's only non-literal path; the harness-prefix ledger row must leave it intact.

@history_item n=47 marker=2026-11-20-a-quoted-modal-never-blocks-a-send
**2026-11-20-a-quoted-modal-never-blocks-a-send** — ~ decision/input-box-readiness-from-literals (standing modal; quote except), ~ decision/verify-reads-only-signatures, ~ decision/verify-pty-output-caught, ~ decision/verify-screen-read-against-literals, ~ decision/verify-signature-only (literals and compiled shapes over them). A probe that reads what a verify child shows reads only through these.

@history_item n=48 marker=2026-11-20-permissionrequest-question-answered
**2026-11-20-permissionrequest-question-answered** — ~ decision/live-rig-kept-stamp-owner-only (CS, WS or PQ kept stamps): the 27th relied row's PQ stamp, the 27-row 2.1.328 stamp the scope names as standing in the rig's kept `target/live-rig/question-stamp.json`.
