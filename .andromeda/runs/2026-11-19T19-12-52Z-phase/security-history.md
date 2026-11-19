@history_item n=1 marker=2026-09-26-local-linux-pre-push-gate
**2026-09-26-local-linux-pre-push-gate** — (the `FAKE_AGENT_PUMP_DELAY_MS` carve-out) added the fake-agent-only test seam row and the ban on another seam or env read without its own Decisions Log entry, a boundary widening. Matters because the fake agent's rehearsal of the `/context` paste and post-condition must add no new seam or env read (decision/test-seams-fake-agent-only, decision/new-env-read-needs-log-entry, decision/test-seams-compiled-out).

@history_item n=2 marker=2026-09-27-epoch-2-cleanup
**2026-09-27-epoch-2-cleanup** — (test-only feature proven absent) recorded release-check refusing `test-support` / `fake-agent` artifacts (probe 5/5). Matters because any fake-agent rehearsal code for `/context` must stay behind the test-only feature that scripts/release-check.sh refuses (decision/release-build-no-test-binaries, decision/release-build-without-test-features).

@history_item n=3 marker=2026-09-28-capability-ledger-and-viola-verify
**2026-09-28-capability-ledger-and-viola-verify** — (the hidden `hook --capture` arm, verify's probe captures and the fixture scrub) set verify's 0600 probe captures in a 0700 per-pid dir removed by a drop guard, and `--record`'s scrub that refuses a recording holding a path or user word. Matters because the `/context` probe's captures and any recorded fixture follow this transient-capture and scrub rule (decision/verify-probe-captures-transient).

@history_item n=4 marker=2026-09-28-capability-ledger-and-viola-verify
**2026-09-28-capability-ledger-and-viola-verify** — (MAX_FRAME consumers extended, verify's CI split) set that the real CLI runs only locally and verify runs in CI only against the fake agent, its real-CLI probe and `--record` only locally. Matters because the `/context` row's live stamp is a local leg; CI proves the path only through the fake agent (decision/real-cli-local-only, decision/ci-no-secrets-no-real-claude).

@history_item n=5 marker=2026-09-29-fake-agent-drift-contract
**2026-09-29-fake-agent-drift-contract** — (the cross-session tag filed as harness origin) named the compiled-prefix ban: classification sources are compiled literals only, never runtime or upstream text. Matters because `/context` must enter the compiled list only, never be derived from upstream text (decision/compiled-classification-sources).

@history_item n=6 marker=2026-10-04-readiness-gate-and-timing-constants
**2026-10-04-readiness-gate-and-timing-constants** — set the vt100 feed under `catch_unwind`, a caught panic poisoning the model so confirmed send reads `input-not-ready`. Matters because a `/context` send goes through the same pre-send readiness gate (decision/vt100-feed-under-panic-catch, decision/vt100-inside-catch-unwind).

@history_item n=7 marker=2026-10-04-confirmed-send-with-cl-1-records
**2026-10-04-confirmed-send-with-cl-1-records** — (send's input validation as landed) set `validate_paste_text` in `viola send` (exit 13) and first in the wrapper's `send`, the paste one bracketed `write_all`, send params `-32602` before the paste check, and send stdin capped at `MAX_FRAME`. Matters because a `/context` send rides this send path unchanged (decision/paste-validation-on-both-sides, contract/channel-frames/send-params, contract/send-refusal/control-character).

@history_item n=8 marker=2026-10-04-dialog-answers-by-dialog-id
**2026-10-04-dialog-answers-by-dialog-id** — (run's strict stamps read) made `run`'s gate read `ledger/stamps.json` through `read_stamps_strict`, a refusal `cli_verified:false` with `strict-modes-failed`, and the standing ban on a non-null decision without that strict read. Matters because a new relied `/context` row is read through this gate (decision/version-gate-strict-stamps-read, contract/strict-modes-refusal/version-gate).

@history_item n=9 marker=2026-10-05-real-cli-verify-probes
**2026-10-05-real-cli-verify-probes** — (verify's typed-input PTY probe) set verify's interactive children as direct spawns under the R8 strip at 80×24, read only against compiled literals, Run B getting one compiled paste, no byte into any dialog, a modal start killed with no key, and the accepted transcript residual. Matters because the `/context` probe is a compiled-literal paste in that child set, a widening of it needing the founder's word (decision/verify-children-direct-spawn, decision/verify-never-answers-dialogs, decision/verify-trusted-runs-risk).

@history_item n=10 marker=2026-10-05-dialog-rows-and-re-probe
**2026-10-05-dialog-rows-and-re-probe** — (the capture arm answers, Runs C and D) kept every probe effect inside the probe's own dir and no plan file under `~/.claude/plans`. Matters as the precedent that a new probe's effects stay confined to its own directory (decision/verify-probes-confined-to-own-dir).

@history_item n=11 marker=2026-10-06-local-command-and-paste-framing-rows
**2026-10-06-local-command-and-paste-framing-rows** — Run B went from one paste to four compiled literals, `PROBE_LOCAL_COMMAND` = `/clear` among them, each only into rows holding the input-box literal and no modal literal, one more transcript accepted, and the stamp grew to seventeen rows with older all-pass stamps reading unverified. The direct precedent for adding a local-command paste and its row: a boundary widening of the ratified verify child set, ratified by the founder's own live answer.

@history_item n=12 marker=2026-10-07-a-send-ending-in-a-newline-is-confirmed
**2026-10-07-a-send-ending-in-a-newline-is-confirmed** — made the typed text (trailing LF removed after validation) the only text `send` uses, for local-command classification, confirmation, byte count and paste; any other removal needs its own ruling. Matters because `/context` is classified on the typed text (decision/refused-text-never-stripped, decision/reject-dont-strip-paste-text).

@history_item n=13 marker=2026-10-09-epoch-3-cleanup
**2026-10-09-epoch-3-cleanup** — (trailing CR and LF removed; an empty typed text refused) made a listed local command followed by LF, CR or CRLF that command, and added `empty-text` on both sides. Matters because `/context` with a line ending must classify as the local command (decision/refuse-control-characters, decision/never-refuse-line-breaks).

@history_item n=14 marker=2026-10-09-inner-cr-and-crlf-in-a-sent-text
**2026-10-09-inner-cr-and-crlf-in-a-sent-text** — set the typed text as every CR LF and lone CR as one LF and trailing CR/LF removed, validation unchanged and first; a change typing a character the received text did not hold needs its own ruling. Matters because the `/context` classification and confirmation read this typed text (decision/typed-text-line-ending-normalisation, decision/bridge-send-one-bracketed-paste).

@history_item n=15 marker=2026-10-13-interrupted-verify-cleanup
**2026-10-13-interrupted-verify-cleanup** — verify's probe dirs removed on pass, fail or SIGINT/SIGTERM by guards that remove only what their run created; a stopped Run B pastes nothing more. Matters because the `/context` probe extends Run B under these guards (decision/verify-probe-dirs-cleanup, decision/probe-artifacts-cleanup).

@history_item n=16 marker=2026-10-13-paste-newline-ledger-row
**2026-10-13-paste-newline-ledger-row** — Run B's fifth compiled paste and the eighteen-row stamp, older stamps of exactly the earlier ids reading unverified; ratified by the founder's own answer to a P4 fork, card confirm provisional. Precedent: each added Run B paste widens the ratified verify child set and moves the relied-row counter.

@history_item n=17 marker=2026-10-13-owed-ledger-rows
**2026-10-13-owed-ledger-rows** — the relied set and stamp read over it (`/21` then), an all-pass older stamp reading `cli_verified:false`. Matters because a relied `/context` row makes every earlier stamp read unverified until re-stamped (decision/unverified-cli-gate, decision/dialog-answers-need-verify-stamp).

@history_item n=18 marker=2026-10-14-resume-and-compact-ledger-rows
**2026-10-14-resume-and-compact-ledger-rows** — Run E and the twenty-two-row stamp; an all-pass 18- or 21-id older stamp reads unverified on every OS. Precedent for the stamp counter moving with a new relied row, ratified by the founder's P4 fork answer.

@history_item n=19 marker=2026-10-16-unwrapped-sessions-on-the-board
**2026-10-16-unwrapped-sessions-on-the-board** — Run B's bounded `claude agents --json` read and the twenty-three-row stamp; a stamp of exactly the pre-chunk ids reads unverified. Matters as the child-output bound and tolerant parse (decision/child-output-bounded, decision/claude-resolved-from-path) and the stamp-counter precedent.

@history_item n=20 marker=2026-10-17-mcp-server-for-drivers
**2026-10-17-mcp-server-for-drivers** — (MCP tool input validation and error bodies) set each MCP handler to validate raw arguments, `validate_paste_text` then the empty-typed-text check client-side on the MCP `send` tool, and closed `invalid-params` field words. Matters because a `/context` send through MCP takes this path (decision/mcp-handlers-validate-raw-arguments, contract/mcp-tool-inputs/invalid-params, contract/mcp-errors/invalid-params, contract/mcp-tools/send).

@history_item n=21 marker=2026-10-17-cli-output-discipline
**2026-10-17-cli-output-discipline** — admitted the terminal colour facts and restated the ban: NEVER another seam or environment read without its own Decisions Log entry. Matters because the chunk's rig and fake-agent work may not add an env read silently (decision/new-env-read-needs-log-entry, decision/controls-not-switchable).

@history_item n=22 marker=2026-10-18-compact-post-condition-ledger-row
**2026-10-18-compact-post-condition-ledger-row** — Run B's sixth paste `PROBE_COMPACT_COMMAND` (`/compact`, `LOCAL_COMMANDS[2].0`) only into the settled input box, its answer waited up to `PROBE_DEADLINE`, its measured residual (a backup transcript) accepted, and the stamp moved to twenty-four rows with `local-command-compact` relied; a widening ratified by the founder's own P4 fork answer. The closest precedent for `/context`: a local command added to `LOCAL_COMMANDS` with a relied post-condition row and a Run B paste, its transcript residual measured.

@history_item n=23 marker=2026-10-19-overseer-switch-to-viola
**2026-10-19-overseer-switch-to-viola** — (question option text) extended R1: upstream text, question and option text included, is never a viola command or config. Matters because `/context`'s stdout is upstream content viola never interprets (decision/upstream-text-is-never-a-command, decision/upstream-text-is-content).

@history_item n=24 marker=2026-10-19-overseer-switch-to-viola
**2026-10-19-overseer-switch-to-viola** — (verify's print-mode probe residual accepted) accepted the CLI's empty project dir under `~/.claude/projects/` into the trusted-runs residual class. Matters because any new transcript or dir residual of a `/context` probe must be measured and accepted into this class (decision/verify-real-cli-transcripts-accepted).

@history_item n=25 marker=2026-10-19-newer-claude-cli-rows
**2026-10-19-newer-claude-cli-rows** — (a scratch footer reading) recorded that a live start is no standing permission and another reading needs its own answer; Run B's pastes go into rows holding an input-box literal. Matters because the chunk's live stamp and rig start need the founder's word anew.

@history_item n=26 marker=2026-10-20-t06-58-31-wrap
**2026-10-20-t06-58-31-wrap** — (the founder's Epoch 5 batch) made the Run B newline and `/compact` pastes, their residual and the stamp changes his own word. Matters as the ratification route for a new Run B paste: a delegate's card confirm is provisional until his own word.

@history_item n=27 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** — (`ledger/stamps.json` checked before `ledger/` is made, and `verify`'s home tree before its log) set `check_stamps` before `create_private_dir(ledger/)` and again under the lock, `check_home_tree` before verify's log, and the ban on a check after a narrowing create, mode set or log open. Matters because re-stamping `/context` live goes through `update_stamps` (decision/stamps-checked-under-lock, decision/verify-home-tree-check, decision/strict-check-before-narrowing, decision/stamps-strict-check-first, contract/strict-modes-refusal/verify).

@history_item n=28 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** — (`check_instance`'s set and the read rule as built) added the read rule on home, instance and `diagnostics/`, and the fixed `Readable` / `Symlink` refusal messages. Matters as the strict-modes rows any new reader follows (decision/strict-modes-before-read, contract/strict-refusal-message/readable, contract/strict-refusal-message/symlink).

@history_item n=29 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** — (the home tree checked at `run` and `revive`'s start, before the wrapper log) set `check_home_tree` before any log at start. Matters because the wrapper's send path rests on this start order (decision/strict-modes-at-start, decision/state-tree-strict-modes-before-use, decision/strict-refusal-leaves-no-trace).

@history_item n=30 marker=2026-10-27-bounded-inputs-at-every-boundary
**2026-10-27-bounded-inputs-at-every-boundary** — (the bounded-reader census) added `product_readers_are_each_bounded_or_named`: every product reader bounded in its statement or named. Matters because any new transcript-count or probe read for `/context` must pass the census (decision/bounded-readers-census, decision/max-frame-cap).

@history_item n=31 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** — (channel-response's closed detail word) closed `channel-response`'s `detail` to three words. Matters because any send-path error stays in this closed detail (contract/channel-response-log/detail).

@history_item n=32 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** — (the clients' screen of a peer's reply) added `sanitised_fault` and `is_code_word`; a client takes refusals and details only as code words. Matters because a `/context` send's `confirmed:false` / `unconfirmable` reply crosses CLI and MCP only through this screen (decision/client-reads-closed-values-only, decision/client-reply-code-words-only, decision/reply-code-words-validated).

@history_item n=33 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** — (verify's refusal codes) gave verify closed refusal codes on its `process-exit`, `strict-modes-failed` and `version-unreadable` among them. Matters because the `/context` stamp run reports refusals only through these (contract/verify-refusal-codes/strict-modes-failed, contract/verify-refusal-codes/version-unreadable).

@history_item n=34 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** — (child program resolution) dropped `batch-script-child` from verify's refusal codes (6 → 5), `claude` a bare name on PATH. Matters for the closed verify code set the chunk relies on (decision/claude-resolved-from-path).

@history_item n=35 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** — (the statusline shell-out, the settings override and the relied set) made `LedgerRow::relied()` all rows on every build (`/24`). Matters because a new `/context` row, if relied, joins this one set on every build and moves the counter.

@history_item n=36 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** — (the R8 strip and the fixture scrub match names exactly) set the exact-name `CLAUDE*` strip with the identity floor always stripped. Matters because verify's `/context` child and the rig run under this strip (decision/parent-identity-env-stripped, decision/claude-credentials-stripped-from-child, decision/inherited-credentials-never-serialised).

@history_item n=37 marker=2026-11-13-verify-readiness-on-the-input-box
**2026-11-13-verify-readiness-on-the-input-box** — (Data Protection) rewrote decision/screen-fixture-signature-rows-only: a screen fixture keeps only signature rows, box rule and prompt rows as their literals alone. Matters for any screen fixture the `/context` probe records.

@history_item n=38 marker=2026-11-13-verify-readiness-on-the-input-box
**2026-11-13-verify-readiness-on-the-input-box** — (Input Validation) added decision/input-box-readiness-from-literals: one predicate for verify and `viola run`'s pre-send gate, never while a modal literal stands, a ratified widening. Matters because the `/context` paste goes only into a box this predicate reads ready.

@history_item n=39 marker=2026-11-14-hyprland-seam
**2026-11-14-hyprland-seam** — (Threat Model Summary) changed decision/child-output-tolerant and decision/child-output-tolerant-unknown to "unknown or a closed failure word". Matters because CLI output read for `/context` fails to unknown or a closed word.

@history_item n=40 marker=2026-11-14-status-line-interception-removed
**2026-11-14-status-line-interception-removed** — (Input Validation 1 of 2) narrowed decision/max-frame-shared-cap's consumer list and the capture arms to one (`hook event --capture`). Matters because the `/context` probe's reads stay inside the MAX_FRAME list and the one capture arm.

@history_item n=41 marker=2026-11-14-status-line-interception-removed
**2026-11-14-status-line-interception-removed** — (Threat Model Summary) set contract/cli-state-root/home-flag and contract/cli-state-root/viola-dir to choose the state tree, snapshot, endpoint and ledger stamps. Matters because the live re-stamp must target the right home's stamps.

@history_item n=42 marker=2026-11-15-live-round-on-the-trimmed-product
**2026-11-15-live-round-on-the-trimmed-product** — (Authentication & Authorization) added decision/live-round-rig-keys-own-pty: the rig keys only into its own pty, its keys the founder's word per chunk. Matters because the carry cry-kutofxml's hint reading and any live `/context` start need the founder's word, asked at this phase.

@history_item n=43 marker=2026-11-15-live-round-on-the-trimmed-product
**2026-11-15-live-round-on-the-trimmed-product** — (Data Protection) added decision/live-session-screens-kept-local: live screen frames only under a gitignored local path, a closed-word summary landing. Matters for any live `/context` reading and the hint-cell reading.

@history_item n=44 marker=2026-11-15-api-error-turn-end
**2026-11-15-api-error-turn-end** — (Authentication & Authorization) gave decision/unverified-cli-gate its StopFailure except: registered with no ledger row and no verify probe, first provisional. Matters as the only standing exception to the relied-row gate; `/context` is planned with a ledger row and probe, not that route.

@history_item n=45 marker=2026-11-16-send-confirmation
**2026-11-16-send-confirmation** — (Threat Model Summary; Input Validation) replaced the exact-match row with decision/delivery-confirmation-trailing-tolerance and dropped "exact equality" from decision/typed-text-line-ending-normalisation's why. Matters because send confirmation now rests on post-condition readings and the local-command classification of the typed text.

@history_item n=46 marker=2026-11-16T06-05-33-wrap
**2026-11-16T06-05-33-wrap** — (the send trailing-only tolerance ratified by the founder) ratified the trailing-only tolerance; a leading trim waits for a live reading. Matters as a trap: `/context` classification must not grow a leading trim without that reading.

@history_item n=47 marker=2026-11-16-local-command-delivery
**2026-11-16-local-command-delivery** — (Security Anti-Patterns) rewrote decision/compiled-classification-sources: a typed text is a local command only by its compiled name, alone or followed by one space and arguments. Core rule `/context` enters by: the compiled `LOCAL_COMMANDS` list only.

@history_item n=48 marker=2026-11-16-local-command-delivery
**2026-11-16-local-command-delivery** — (Input Validation) added decision/transcript-path-shape-check and decision/transcript-read-bounded (no symlink, owner's regular file, from the pre-send offset through MAX_FRAME, a count of local-command-stdout tags alone), and the shared-validators except. This is the `CommandStdout` post-condition `/context` reuses as `/mcp` does.

@history_item n=49 marker=2026-11-16-local-command-delivery
**2026-11-16-local-command-delivery** — (Threat Model Summary) set contract/wrapper-channel-methods/hook-event: SessionStart's optional transcript param re-checked by the wrapper. Matters because the `/context` post-condition reads the transcript that param names.

@history_item n=50 marker=2026-11-16-local-command-delivery
**2026-11-16-local-command-delivery** — (Data Protection) added decision/transcript-path-redacted and decision/verify-echo-probe-residuals (Run B's seventh paste `/mcp disable viola-probe-absent`, residuals unmeasured and accepted). The direct precedent for a new local-command verify probe paste and its accepted residuals; a `/context` probe adds an eighth paste whose residuals need the same naming.

@history_item n=51 marker=2026-11-17T05-21-35Z-wrap
**2026-11-17T05-21-35Z-wrap** — (the delegate's confirm ratified by the founder, local-command-delivery rows) ratified the four local-command-delivery entries' rows on the founder's own word. Matters because the rows `/context` extends stand ratified, not provisional.

@history_item n=52 marker=2026-11-17-standing-live-rig
**2026-11-17-standing-live-rig** — (Authentication & Authorization; Secret Management) made the rig's key grant a closed argv grant on the founder's word anew per chunk, and added decision/live-rig-credentials-copy and decision/live-rig-isolated-child. Matters because a live start in the implement/operator leg runs under these rows.

@history_item n=53 marker=2026-11-17-first-readings-on-the-standing-rig
**2026-11-17-first-readings-on-the-standing-rig** — (Secret Management; Authentication & Authorization; Input Validation) reworded decision/live-rig-isolated-child (the needle witness), made the rig's stop need no word while the argv grant does, and added decision/live-rig-prompt-capture-closed. Matters for which keys of a `/context` live start need the founder's word.

@history_item n=54 marker=2026-11-17-trust-record-ledger-row
**2026-11-17-trust-record-ledger-row** — (Data Protection; Threat Model Summary) gave decision/no-writes-under-claude-home, decision/viola-writes-nothing-in-claude-dir and decision/nothing-written-under-claude-home the verify trust-record except, and widened the probe cleanup excepts. Matters because the `/context` verify run writes and removes only its own trust record.

@history_item n=55 marker=2026-11-17-trust-record-ledger-row
**2026-11-17-trust-record-ledger-row** — (Authentication & Authorization) gave decision/explicit-mode-before-write its except: the trust record writer keeps Claude's config file's own mode. Matters as the only exception to mode-before-byte any probe write may lean on.

@history_item n=56 marker=2026-11-17-trust-record-ledger-row
**2026-11-17-trust-record-ledger-row** — (Secret Management) gave decision/env-not-config-channel and decision/no-stored-secrets the `CLAUDE_CONFIG_DIR` except. Matters because no further env read may be added for `/context` without its own entry.

@history_item n=57 marker=2026-11-18-trust-record-row-read-live
**2026-11-18-trust-record-row-read-live** — (Data Protection; Threat Model Summary) gave decision/screen-fixture-signature-rows-only, decision/verify-signature-only and decision/verify-screen-read-against-literals the rig-only first-screen except. Matters because verify's screen reads for the `/context` probe stay signature-only.

@history_item n=58 marker=2026-11-18-trust-record-row-read-live
**2026-11-18-trust-record-row-read-live** — (Input Validation; Secret Management) added decision/verify-stamp-first-screen-closed-words and decision/live-rig-seed-from-bundle-logic. Matters because the `/context` re-stamp writes the stamp's first-screen part only as closed words, and the rig's seeded config takes no user value.

@history_item n=59 marker=2026-11-18-session-folder-trusted-by-viola
**2026-11-18-session-folder-trusted-by-viola** — (Data Protection; Threat Model Summary) set the excepts that run and revive's start set an untrusted spawn folder's trust flag. Matters as the current wording of the never-under-`~/.claude` rows the chunk rests on.

@history_item n=60 marker=2026-11-18-session-folder-trusted-by-viola
**2026-11-18-session-folder-trusted-by-viola** — (Secret Management 1 of 2; Secret Management) extended the `CLAUDE_CONFIG_DIR` excepts to run and revive's start and reworded decision/live-rig-seed-from-bundle-logic. Matters for decision/env-not-config-channel, decision/no-stored-secrets and the rig rows.

@history_item n=61 marker=2026-11-18-browser-tools-prompt-modal
**2026-11-18-browser-tools-prompt-modal** — (Authentication & Authorization; Security Anti-Patterns) moved decision/unverified-cli-gate, decision/dialog-answers-need-verify-stamp and contract/cli-exit-codes/12 to "every relied row counts: pass, or absent on the browser-tools-prompt row alone". Matters because a relied `/context` row must read pass, never absent, for a version to read verified.

@history_item n=62 marker=2026-11-18-browser-tools-prompt-modal
**2026-11-18-browser-tools-prompt-modal** — (Secret Management) gave decision/live-rig-seed-from-bundle-logic the VP arm clause. Matters for the rig's seeded config on a live `/context` start.

@history_item n=63 marker=2026-11-18-the-2-1-325-recorded-fixture-set
**2026-11-18-the-2-1-325-recorded-fixture-set** — (Data Protection) added decision/live-rig-mcp-arm-keeps-no-output. Matters because the `/context` reading in the carry came from a live rig arm whose output-keeping rules bind.

@history_item n=64 marker=2026-11-18-the-live-home-s-re-stamp-and-the-product-re-pin
**2026-11-18-the-live-home-s-re-stamp-and-the-product-re-pin** — (Data Protection) added decision/live-rig-context-arm-keeps-no-transcript: the CX arm reads the transcript through MAX_FRAME, counting tags past the kept length, keeping no byte, sending only through `viola send` under the rig's private config. This is the arm that took the `/context` reading (cry-qklc2wb6) and the rule any further `/context` live reading keeps.

@history_item n=65 marker=2026-11-18-the-live-home-s-re-stamp-and-the-product-re-pin
**2026-11-18-the-live-home-s-re-stamp-and-the-product-re-pin** — (Threat Model Summary) extended decision/verify-never-answers-dialogs: a browser-tools prompt at start is left for the human to answer. Matters because the `/context` live stamp on 2.1.328 runs verify under this rule.

@history_item n=66 marker=2026-11-18-the-live-home-s-re-stamp-and-the-product-re-pin
**2026-11-18-the-live-home-s-re-stamp-and-the-product-re-pin** — (Secret Management) gave decision/live-rig-isolated-child and decision/live-rig-credentials-copy the VH arm except (real home, no credentials read, explicit home), provisional until the founder's word. Matters because a live re-stamp of the `/context` row on the real home would take the VH arm and its residual (re-pin is out of scope, its own operator step).

@history_item n=67 marker=2026-11-19-a-live-stopfailure-payload
**2026-11-19-a-live-stopfailure-payload** — (Secret Management; Input Validation) added decision/live-rig-sf-arm-network-closed and widened decision/live-rig-prompt-capture-closed to StopFailure error words. Matters because the carried hint-cell reading (cry-kutofxml) rides this chunk's rig start under these rig rows.

@history_item n=68 marker=2026-11-19-a-live-stopfailure-payload
**2026-11-19-a-live-stopfailure-payload** — (Authentication & Authorization; the StopFailure exception settled) dropped decision/unverified-cli-gate's spent owner, the exception kept on the founder's word on one contained reading on 2.1.328. Matters because that except is the gate's only standing exception; the `/context` row takes the ordinary relied-row route.
