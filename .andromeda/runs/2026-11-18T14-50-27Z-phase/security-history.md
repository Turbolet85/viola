@history_item n=1 marker=2026-09-28-capability-ledger-and-viola-verify
**2026-09-28-capability-ledger-and-viola-verify** — (the capture arm, probe captures and fixture scrub; MAX_FRAME consumers, verify's CI split) set up the hidden capture arm, verify's transient 0600 probe captures under a per-pid 0700 dir, the `--record` scrub that refuses a whole recording, and the rule that the real CLI runs only locally while CI runs verify only against the fake agent. Matters here because the new browser-tools row's probe and any recorded screen follow this scrub, capture and CI split, and the fake agent is what proves the literal in CI.

@history_item n=2 marker=2026-10-04-readiness-gate-and-timing-constants
**2026-10-04-readiness-gate-and-timing-constants** — landed the vt100 feed under `catch_unwind` on a tee after the human's write, with a caught panic poisoning the model to `input-not-ready`. This is the screen model the pre-send gate reads. A new modal literal is read through this same feed (rows decision/vt100-feed-under-panic-catch and decision/vt100-inside-catch-unwind).

@history_item n=3 marker=2026-10-04-confirmed-send-with-cl-1-records
**2026-10-04-confirmed-send-with-cl-1-records** — (send's input validation; the feed bound closed) bounded the tee-to-feed queue at `FEED_CAPACITY` 256 with `try_send`, so a dropped copy poisons the model and never blocks passthrough. It also put `validate_paste_text` first on both sides. The gate's refusal on a standing modal rests on this bounded, poisonable feed (decision/vt100-feed-bounded-queue, decision/vt100-feed-isolated).

@history_item n=4 marker=2026-10-04-dialog-answers-by-dialog-id
**2026-10-04-dialog-answers-by-dialog-id** — (run's strict stamps read; decisions on the six-row stamp) moved `run`'s version gate onto `read_stamps_strict`, where a refusal, an unreadable file or a malformed file reads `cli_verified:false`. It also set the rule that no non-null decision flows unless the gate read the stamps through the strict check. Adding a 24th relied row changes what a stamp must hold for `cli_verified`, so this gate is the place the 23-to-24 step bites (decision/version-gate-strict-stamps-read, decision/dialog-answers-need-verify-stamp).

@history_item n=5 marker=2026-10-05-real-cli-verify-probes
**2026-10-05-real-cli-verify-probes** — set verify's interactive PTY children as direct spawns under the R8 strip at 80×24, output capped at MAX_FRAME into a Screen under `catch_unwind`. Neither run sends a byte into a CLI-native dialog, a modal start is killed with no key, and viola writes nothing under `~/.claude`. It also set screen fixtures to signature rows only, with closed refusal codes. This is the founder-ratified no-key rule the browser-tools prompt must follow: it is read as a modal and killed, never answered (decision/verify-never-answers-dialogs, decision/verify-children-direct-spawn).

@history_item n=6 marker=2026-10-05-dialog-rows-and-re-probe
**2026-10-05-dialog-rows-and-re-probe** — added Runs C and D, with no byte typed into any dialog and probe dialogs answered only by the hook's output. It retired the R2 dated gap: the stamp now holds fourteen rows, and a partial stamp records `cli_verified:false`. This is the precedent for growing the relied set with a re-probe, and for answering only through the probe hook and never by keys (decision/verify-writes-no-dialog-byte).

@history_item n=7 marker=2026-10-06-local-command-and-paste-framing-rows
**2026-10-06-local-command-and-paste-framing-rows** — Run B's added pastes go only into rows holding the input-box literal and no modal literal. A modal, a poisoned screen or the deadline ends the pastes with no key. A stamp of only the older ids, all pass, reads `cli_verified:false`. A new modal literal is exactly what stops these pastes, and the older-stamp rule says what a 23-row home reads once the relied set is 24.

@history_item n=8 marker=2026-10-13-interrupted-verify-cleanup
**2026-10-13-interrupted-verify-cleanup** — a stopped run kills and reaps its child with no key, and guards remove only what that run created, never by pattern. SIGKILL and crashes leave the dirs by hand. A verify run killed at a standing browser-tools modal ends the same way (decision/verify-probe-dirs-cleanup, decision/probe-artifacts-cleanup).

@history_item n=9 marker=2026-10-13-paste-newline-ledger-row
**2026-10-13-paste-newline-ledger-row** — a new ledger row with its verify probe widened the ratified verify child set and raised the stamp counter (17 to 18). A stamp of exactly the older ids, all pass, reads `cli_verified:false`. This is the precedent shape for this chunk's 24th relied row, and the founder treats a change to the verify child set as a widening.

@history_item n=10 marker=2026-10-14-resume-and-compact-ledger-rows
**2026-10-14-resume-and-compact-ledger-rows** — Run E was added to the verify child set with no key typed, and the ledger and counters were raised. An older all-pass stamp reads unverified on every OS. If the browser-tools probe needs a new run or start, it widens the verify child set the same way and needs the founder's own word.

@history_item n=11 marker=2026-10-16-unwrapped-sessions-on-the-board
**2026-10-16-unwrapped-sessions-on-the-board** — the ledger went to twenty-three rows (`agents-join` last), and a stamp of exactly the pre-chunk ids reads unverified. Run B's added bounded read was ruled a widening of the verify child set. This is the 22-to-23 step whose shape the scope tells P3 to follow for 23 to 24.

@history_item n=12 marker=2026-10-18-compact-post-condition-ledger-row
**2026-10-18-compact-post-condition-ledger-row** — Run B's sixth paste and a new relied row raised the stamp. Each added paste goes only into the settled input box, and an older-id stamp reads `cli_verified:false`. This is another row-count step, and it repeats that pastes stop at any modal literal.

@history_item n=13 marker=2026-10-17-optional-plugin-install
**2026-10-17-optional-plugin-install** — (the live round's residual under ~/.claude) a live round runs the CLI's own plugin commands at local scope from a rig repo. The CLI's writes under `~/.claude` are its own, and viola writes nothing there. A live start here stays inside this rig and residual class (decision/live-plugin-install-round, decision/plugin-install-live-round-rig).

@history_item n=14 marker=2026-10-18-plugin-precedence-live-readings
**2026-10-18-plugin-precedence-live-readings** — (the rig outside the work tree, its trust keys and residual) the rig passes folder trust only by keys into its own pty: at most 2 arrows and 1 Enter on a read selected row, and no key on any other screen. It runs from a project outside the work tree. This is the root of the rig's closed key grant: a browser-tools modal is "any other screen", so it gets no key.

@history_item n=15 marker=2026-10-19-overseer-switch-to-viola
**2026-10-19-overseer-switch-to-viola** — (verify's print-mode probe residual accepted) the print-mode probe leaves the CLI's empty project dir under `~/.claude/projects/`, accepted as a CLI write. The scope's unmeasured CARRY asks whether the print-mode probe or Run A writes the extension-detected cache (`cachedChromeExtensionInstalled`) that arms the prompt. That cache would be another CLI-side residual of verify's earlier steps.

@history_item n=16 marker=2026-10-19-newer-claude-cli-rows
**2026-10-19-newer-claude-cli-rows** — (the scratch footer reading accepted; Run B's pastes into rows holding an input-box literal) one scratch `viola run` start of a newer CLI with no key typed was accepted as a one-off; another reading needs its own answer. The compiled set grew to two input-box literals, and pastes go only into rows holding one and no modal literal. This is the precedent for adding a compiled screen literal, and any new live reading needs the founder's word.

@history_item n=17 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** — (`ledger/stamps.json` checked before `ledger/` is made, and verify's home tree before its log; the home tree at run's start) `update_stamps` checks before the create and again under the lock. Verify runs `check_home_tree` before its cli log, and `run` checks the home tree before the wrapper log. A check made after a narrowing create passes a widened tree, which is a trap. The new row's stamp write goes through this ordering (decision/stamps-checked-under-lock, decision/verify-home-tree-check, decision/strict-check-before-narrowing).

@history_item n=18 marker=2026-10-10-budget-governor
**2026-10-10-budget-governor** — (the two instance checks ahead of any log) found that a log opened before or after a refused check narrows the home to 0700 and erases what the check refuses. So a refusal writes no line, and this is the reason behind decision/strict-refusal-leaves-no-trace and decision/home-tree-strict-modes-before-log, which bind verify and run here.

@history_item n=19 marker=2026-10-27-bounded-inputs-at-every-boundary
**2026-10-27-bounded-inputs-at-every-boundary** — (the bounded-reader census; the vt100 passthrough witness) `product_readers_are_each_bounded_or_named` requires every product reader to be bounded or named, and a witness shows passthrough surviving a caught feed panic. Any new probe read this chunk adds must pass the census (decision/bounded-readers-census).

@history_item n=20 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** — (verify's refusal codes; the catch site's document under --json) verify writes a closed refusal code on its `process-exit`, `strict-modes-failed` and `version-unreadable` among them, and the catch site prints the `internal-error` document under `--json`. These are the picked contracts contract/verify-refusal-codes/* and contract/internal-error-output/json. A new probe adds no refusal code unless the plan names one.

@history_item n=21 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** — (the relied set on every build; the home check owner and mode alone, an unstattable home refused) `LedgerRow::relied()` became one relied set on every build, and an older stamp records `cli_verified:false`. Strict-modes is Linux owner and mode, and an unstattable home is a refusal. The 24th row joins one relied set with no per-OS split (decision/unstattable-home-refused).

@history_item n=22 marker=2026-11-13-verify-readiness-on-the-input-box
**2026-11-13-verify-readiness-on-the-input-box** — (Input Validation; Data Protection) added decision/input-box-readiness-from-literals: one predicate for verify and run's pre-send gate reads ready only on compiled literals and never while a modal literal stands. It also reworded decision/screen-fixture-signature-rows-only to keep footer and modal literal rows whole, box rule and prompt rows as literals alone. This chunk adds a modal literal to exactly this shared predicate, and its fixture keeps the modal row whole under this rule.

@history_item n=23 marker=2026-11-14-hyprland-seam
**2026-11-14-hyprland-seam** — (Threat Model Summary) reworded decision/child-output-tolerant and decision/child-output-tolerant-unknown: a failure becomes unknown or a closed failure word. Any new probe reading of CLI output follows this wording.

@history_item n=24 marker=2026-11-14-status-line-interception-removed
**2026-11-14-status-line-interception-removed** — (Threat Model Summary) dropped `statusline_command` from contract/cli-state-root/home-flag and contract/cli-state-root/viola-dir. It also removed the statusline rows and the settings override from the relied surface, which explains why the stamp stands at 23 rows before this chunk's 24th.

@history_item n=25 marker=2026-10-18-plugin-precedence-live-readings
**2026-10-18-plugin-precedence-live-readings** — (the VIOLA_DIR home step ratified) the CLI verbs' `VIOLA_DIR` step and canonicalisation, which choose the state root, were ratified by the founder. These are the picked contracts contract/cli-state-root/*.

@history_item n=26 marker=2026-11-15-live-round-on-the-trimmed-product
**2026-11-15-live-round-on-the-trimmed-product** — added decision/live-round-rig-keys-own-pty (the rig's keys only into its own pty, viola typing into no dialog) and decision/live-session-screens-kept-local (live screen frames kept local; only closed-word summaries land). Any live start in this chunk that shows the browser-tools modal keeps its frames local and types nothing into the modal.

@history_item n=27 marker=2026-11-15-api-error-turn-end
**2026-11-15-api-error-turn-end** — (Authentication & Authorization) put the except on decision/unverified-cli-gate: StopFailure is registered with no ledger row and no verify probe, owned by the 2.1.325 recorded fixture set. This is the contrasting precedent: this chunk takes the ledger-row-plus-probe route instead, so it must not widen that except.

@history_item n=28 marker=2026-11-16T06-05-33-wrap
**2026-11-16T06-05-33-wrap** — (the StopFailure no-ledger-row answer ratified) the founder ratified the decision/unverified-cli-gate except. Its owner, «The 2.1.325 recorded fixture set», is the same entry that keeps this chunk's live 24-row verify as CARRY `cry-bb5cy7aq`.

@history_item n=29 marker=2026-11-16-local-command-delivery
**2026-11-16-local-command-delivery** — (Security Anti-Patterns; Data Protection) decision/compiled-classification-sources now says screen signatures come only from compiled viola-agent-claude ledger rows, never runtime or upstream text. The seventh Run B paste's residuals were accepted (decision/verify-echo-probe-residuals). The browser-tools literal must be a compiled ledger-row literal, and any probe residual needs the same acceptance.

@history_item n=30 marker=2026-11-17-standing-live-rig
**2026-11-17-standing-live-rig** — made the rig's key grant a closed argv grant (two down-arrows and Enter only on a read trust row, one x and one DEL in a ready box, nothing else), and added decision/live-rig-credentials-copy and decision/live-rig-isolated-child. The scope directive recommends no key into the browser-tools modal; this grant has no key for it.

@history_item n=31 marker=2026-11-17-first-readings-on-the-standing-rig
**2026-11-17-first-readings-on-the-standing-rig** — the founder's word per chunk binds only the argv grant, and the stop (Ctrl-C or a no-key group kill) needs no word. The founder's real config is read only by the mark witness, keeping one closed word. decision/live-rig-prompt-capture-closed was added. These bind the directive's live-start shape: the stop is the no-key group kill, and the founder's config is read by mark witness only.

@history_item n=32 marker=2026-11-17-trust-record-ledger-row
**2026-11-17-trust-record-ledger-row** — added verify's trust-record exception to decision/no-writes-under-claude-home and its twins, and to decision/explicit-mode-before-write. It made a CLAUDE_CONFIG_DIR value read an exception to decision/r8-claude-env-strip, decision/credential-strip-by-name-only and decision/env-not-config-channel, and extended the cleanup excepts. This is the most recent ledger-row-plus-probe precedent (the 23rd row). It also shows that any write into Claude's config, such as seeding `claudeInChromeDefaultEnabled`, is a bounded exception the scope puts out of this chunk.

@history_item n=33 marker=2026-11-18-trust-record-row-read-live
**2026-11-18-trust-record-row-read-live** — added the rig-only first-screen except to decision/screen-fixture-signature-rows-only, decision/verify-signature-only and decision/verify-screen-read-against-literals, plus decision/verify-stamp-first-screen-closed-words and decision/live-rig-seed-from-bundle-logic (the browser-tools prompt turned off in the rig seed). This is the chunk that measured the `browser-tools-prompt` modal (the CARRY `cry-mfxnxzub` evidence). The stamp's first screen must name the new modal only as a closed word or literal id.

@history_item n=34 marker=2026-11-18-session-folder-trusted-by-viola
**2026-11-18-session-folder-trusted-by-viola** — widened the decision/no-writes-under-claude-home, decision/viola-writes-nothing-in-claude-dir and decision/nothing-written-under-claude-home excepts to the start's trust flag, and moved the CLAUDE_CONFIG_DIR value read to run and revive's start. It reworded decision/live-rig-seed-from-bundle-logic so nothing seeded is read from the user's config. This is the last chunk, PROVISIONAL until the founder's word. The excepts list every viola write into Claude's config, so turning the browser-tools prompt off in the product would need a new except, which the scope rules out.

@history_item n=35 marker=2026-10-03-mutation-scoring-completion
**2026-10-03-mutation-scoring-completion** — the pre-push gate launches every child natively as `env -i` with exactly HOME (from passwd) and PATH, with no other launcher and no uid-0 launch. A gate or fake-agent test this chunk adds runs under it (decision/hermetic-child-environment, decision/pre-push-empty-environment-launch, decision/no-uid-0-launch).

@history_item n=36 marker=2026-09-28-hook-perf-gate
**2026-09-28-hook-perf-gate** — added the second fake-agent test seam and the rule that another seam needs its own entry. Teaching the fake agent a browser-tools screen must add no environment seam (decision/new-env-read-needs-log-entry, decision/test-seams-fake-agent-only).

@history_item n=37 marker=2026-09-27-epoch-2-cleanup
**2026-09-27-epoch-2-cleanup** — `release-check` refuses `test-support` and `fake-agent` artifacts (probe 5/5). The fake agent's new screen stays inside the test-only feature (decision/release-build-without-test-features, decision/release-build-no-test-binaries).

@history_item n=38 marker=2026-09-27-instance-state-and-start-order
**2026-09-27-instance-state-and-start-order** — set every replaced file through the shared `replace_private` helper, mode set before any byte. Any stamp or fixture write the probe adds goes through it (decision/one-replace-helper, decision/explicit-mode-before-write).
