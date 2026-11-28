@history_item n=1 marker=amd-j3e3et7l
**amd-j3e3et7l** (2026-11-28-console-actions — Input Validation) — reworded decision/console-key-reader-bounded to "keys of closed classes, an escape sequence bounded" and added decision/console-runs-verbs-as-children, decision/console-typed-text-as-data and decision/console-editing-line-bounded, all PROVISIONAL on a delegate word. F-K rewrites CONSOLE_KEYS and its pinned literal on top of this key reader, and any new key must keep these provisional rows true.

@history_item n=2 marker=amd-qegtos6q
**amd-qegtos6q** (2026-11-28-console-actions — Threat Model Summary) — gave decision/child-path-trust its except "viola console's action children are viola's own executable, never a PATH lookup". Any console or fleet action this chunk adds must spawn viola's own exe and not a PATH lookup.

@history_item n=3 marker=amd-pnhxubzy
**amd-pnhxubzy** (2026-11-28-console-actions — Threat Model Summary) — set decision/child-path-trust provisional on the delegate's word, with the operator away. His word is still owed at the epoch batch.

@history_item n=4 marker=amd-swe46xco
**amd-swe46xco** (2026-11-28-console-actions — Data Protection) — added decision/console-action-child-outlives-quit, decision/console-answer-reads-kind-and-id and decision/console-diagnostics-closed-words (verb word and exit code only, key class and count only), PROVISIONAL. Diagnostics for a reworked key line must stay closed words and counts.

@history_item n=5 marker=amd-2mcteyzf
**amd-2mcteyzf** (2026-11-28-fleet-console-view — Authentication & Authorization) — split contract/strict-modes-refusal/console into two cases: "no line at start" and "one process-exit strict-modes-failed mid-run". The F-R exception for refusals shown to the human sits beside this console refusal shape.

@history_item n=6 marker=amd-6hysgkrk
**amd-6hysgkrk** (2026-11-28-fleet-console-view — Input Validation) — added viola console to decision/claude-agents-resolved-from-path, which resolves claude from the caller's PATH. That makes the console a claude spawner, which is exactly the reach cry-jeokvkdk's guard must cut: a live step reading the board with no claude on PATH or under a private config.

@history_item n=7 marker=amd-hxkafaqw
**amd-hxkafaqw** (2026-11-28-fleet-console-view — Data Protection) — added decision/console-agents-child-outlives-quit, a residual accepted PROVISIONAL on the delegate's word. It is a precedent for children that outlive a fleet-driving console.

@history_item n=8 marker=amd-udjrbu2t
**amd-udjrbu2t** (2026-11-28-console-scaffold — Input Validation) — first landed decision/console-key-reader-bounded (no key byte reaches any log line), PROVISIONAL. This is the origin of the key-reader row the key-line change rests on.

@history_item n=9 marker=amd-ko2kdtfx
**amd-ko2kdtfx** (2026-11-28-console-scaffold — Authentication & Authorization) — added decision/console-terminal-and-home-check and contract/strict-modes-refusal/console. These are the console's own start checks (the home read rule) that any fleet-driving console path must keep first.

@history_item n=10 marker=amd-dsrgnukp
**amd-dsrgnukp** (2026-11-27T19-07-11Z-wrap — Authentication & Authorization) — the operator's own word ratified as standing decision/up-checks-before-log-and-spawn, decision/window-step-moves-own-terminal-window, decision/r7-place-script-closes-own-terminal, decision/switch-ends-only-its-own-child, decision/switch-refusal-order, decision/launch-pair-authenticates-nothing, decision/window-origin-authenticates-nothing and decision/hyprland-recorder-moves-own-window. These are the start, switch and window rows that H1, H2, R6 and R7 rest on, and they are no longer provisional.

@history_item n=11 marker=amd-ijjdulys
**amd-ijjdulys** (2026-11-27T19-07-11Z-wrap — Input Validation 1 of 2) — ratified as standing the board re-parse rows (launch pair, lineage and links, window and origin), decision/open-closed-argument-allowlist, decision/open-terminal-direct-spawn, decision/launch-flags-refused-after-dashes, decision/run-lineage-flags-checked and decision/effort-word-check-in-agent-crate. These are the closed argv and board read-back that "an instance came up" and "a terminal per instance" rely on.

@history_item n=12 marker=amd-nuhx4dle
**amd-nuhx4dle** (2026-11-27T19-07-11Z-wrap — Input Validation 2 of 2) — ratified decision/run-window-flags-checked as standing, including revive's hidden --origin, which asks a window only at a real snapshot's workspace. This bounds "each window on its recorded workspace".

@history_item n=13 marker=amd-tui4x43l
**amd-tui4x43l** (2026-11-27T19-07-11Z-wrap — Data Protection) — ratified decision/launch-pair-kept-to-snapshot-and-event-log, decision/open-ancestor-walk-in-memory and decision/window-record-kept-to-snapshot-and-event-log as standing. The model and effort handed again on restore, and the window record, may stand only there.

@history_item n=14 marker=amd-y3ds7qwn
**amd-y3ds7qwn** (2026-11-27T19-07-11Z-wrap — Threat Model Summary) — ratified decision/switch-method-behind-ipc-controls as standing. This is the switch method under the IPC controls that the R6a/R6b readings exercise.

@history_item n=15 marker=amd-z2hyfd6b
**amd-z2hyfd6b** (2026-11-25-placing-without-moving-the-view — Authentication & Authorization) — widened decision/window-step-moves-own-terminal-window: it switches back once to the workspace active before it, and a failed read or restore changes no placement. R7's two unmeasured readings (focus after the switch back, a failed read or restore) are this widening's live half.

@history_item n=16 marker=amd-3743pzrs
**amd-3743pzrs** (2026-11-25-placing-without-moving-the-view — Authentication & Authorization) — added decision/r7-place-script-closes-own-terminal: the test-only R7 script opens and closes only its own terminal, starts no claude and refuses on a locked desktop. Any live R7 step must keep these bounds.

@history_item n=17 marker=amd-b4oqdioi
**amd-b4oqdioi** (2026-11-24-viola-up — Input Validation) — extended decision/open-terminal-direct-spawn to viola up, which starts only the pinned "viola revive <name> --origin" in the recorded cwd. It also gave revive a hidden --origin under decision/run-window-flags-checked, and made contract/revive-arguments/id conflict --list with --origin. These are the restore path H2's reboot reading (R4b) drives.

@history_item n=18 marker=amd-4g4366rn
**amd-4g4366rn** (2026-11-24-viola-up — Threat Model Summary) — added viola up's board-scan refusal to contract/cli-exit-codes/21. It also widened decision/cwd-only-in-snapshot and decision/spawn-cwd-snapshot-only so the recorded cwd stands on up's terminal argv. Restore must not carry the cwd anywhere further.

@history_item n=19 marker=amd-xkcla76g
**amd-xkcla76g** (2026-11-24-viola-up — Authentication & Authorization) — added contract/strict-modes-refusal/up: exit 21, no line, nothing spawned.

@history_item n=20 marker=amd-4ldud76d
**amd-4ldud76d** (2026-11-24-viola-up — Authentication & Authorization) — added decision/up-checks-before-log-and-spawn: the board scan runs before the log, and the home tree check and pinned re-hash run before the first spawn. Any restore or start of a fleet through up keeps this order.

@history_item n=21 marker=amd-jsdtpxb2
**amd-jsdtpxb2** (2026-11-24-viola-up — Threat Model Summary) — marked the two cwd rows provisional, because the cwd on up's argv was a widening of its own outside U2 and was asked as a fork. This is a precedent: a crossing outside the operator's answer goes to founder-forks and is not built on a delegate's word.

@history_item n=22 marker=amd-kp64tfg2
**amd-kp64tfg2** (2026-11-23-viola-open-2 — Threat Model Summary) — set decision/cwd-only-in-snapshot and decision/spawn-cwd-snapshot-only standing on the operator's relayed W1 answer (the cwd on viola up's terminal argv, as built).

@history_item n=23 marker=amd-ob2y7jhw
**amd-ob2y7jhw** (2026-11-23-viola-open-2 — Input Validation) — resolved hyprctl on PATH under decision/hyprctl-read-deadline (with the R8 names stripped). It also added decision/open-closed-argument-allowlist, decision/open-terminal-direct-spawn, decision/run-window-flags-checked and decision/board-window-and-origin-re-parsed. These cover a terminal per instance on a named workspace and its read-back on the board.

@history_item n=24 marker=amd-45c6f26a
**amd-45c6f26a** (2026-11-23-viola-open-2 — Authentication & Authorization) — added decision/window-origin-authenticates-nothing and decision/window-step-moves-own-terminal-window. The product moves only its own terminal's window, which bounds any placement step of a fleet start or restore.

@history_item n=25 marker=amd-u7rbht75
**amd-u7rbht75** (2026-11-23-viola-open-2 — Data Protection) — added decision/window-record-kept-to-snapshot-and-event-log and decision/open-ancestor-walk-in-memory (at most 8 ancestor pids, never recorded).

@history_item n=26 marker=amd-vj42vyj4
**amd-vj42vyj4** (2026-11-23-viola-open-2 — Threat Model Summary) — added open to decision/state-tree-strict-modes-before-use. A new start verb joins this list, so any verb this chunk adds to start a fleet must be named there too.

@history_item n=27 marker=amd-cmh23teh
**amd-cmh23teh** (2026-11-24-viola-adopt — Authentication & Authorization) — added decision/adopt-signals-no-foreign-process (never end or signal a process viola did not start) and added adopt's start to decision/start-checks-home-tree-before-log-opens and decision/start-checks-socket-dir-before-bind. This applies directly to moving his live fleet, some on an older prototype binary, onto this binary: viola must never take over or signal those processes.

@history_item n=28 marker=amd-xigshwwf
**amd-xigshwwf** (2026-11-24-viola-adopt — Authentication & Authorization) — added contract/strict-modes-refusal/adopt and decision/r5-adopt-script-ends-own-processes: the test-only live script ends only the wrapper it recorded (TERM, then 10 s, then KILL only on an unchanged start time). This is a precedent for the R4b reboot script's own process bounds.

@history_item n=29 marker=amd-6foqdnvf
**amd-6foqdnvf** (2026-11-24-viola-adopt — Input Validation) — gave decision/session-id-argv-closed-shape its except (an operator-typed id, in the same closed shape), later ratified as W2. "A session resumed by its own id" must go through the one resume-args builder.

@history_item n=30 marker=amd-rjqgl3a3
**amd-rjqgl3a3** (2026-11-24-viola-adopt — Input Validation) — added adopt's live-elsewhere check to decision/claude-agents-resolved-from-path and adopt to decision/launch-flags-checked.

@history_item n=31 marker=amd-3mh5uf3b
**amd-3mh5uf3b** (2026-11-24-viola-adopt — Error Handling) — added adopt's start to decision/home-tree-strict-modes-before-log, decision/run-refuses-socket-dir-before-start and contract/cli-json-error/internal-error. This is the pattern each new start verb follows.

@history_item n=32 marker=amd-moaoiixf
**amd-moaoiixf** (2026-11-24-viola-adopt — Data Protection) — added adopt to decision/start-writes-trust-record-flag-only, decision/no-writes-under-claude-home and decision/viola-writes-nothing-in-claude-dir, later ratified W5 "like any start". A fleet start through any start path writes the trust flag of an untrusted spawn folder.

@history_item n=33 marker=amd-4mwl66bv
**amd-4mwl66bv** (2026-11-24-viola-adopt — Threat Model Summary) — added adopt's start to decision/state-tree-strict-modes-before-use.

@history_item n=34 marker=amd-3oykhjjv
**amd-3oykhjjv** (2026-11-24-viola-adopt — Threat Model Summary) — added adopt's start to decision/nothing-written-under-claude-home's except.

@history_item n=35 marker=amd-5sxvogei
**amd-5sxvogei** (2026-11-24-viola-adopt — Secret Management 1 of 2) — added adopt's start to the CLAUDE_CONFIG_DIR value-read excepts of decision/credential-strip-by-name-only, decision/env-not-config-channel and decision/no-stored-secrets.

@history_item n=36 marker=amd-lr66lc35
**amd-lr66lc35** (2026-11-24-viola-adopt — Secret Management 2 of 2) — added adopt's start to decision/r8-claude-env-strip's except. The R8 strip binds every child of a fleet start.

@history_item n=37 marker=amd-cdk7rqca
**amd-cdk7rqca** (2026-11-24-viola-adopt — Data Protection) — added adopt to decision/exec-commands-absolute-pinned-path. Every exec-form command a start writes is the absolute pinned path, which matters when "an instance is driven by another binary".

@history_item n=38 marker=amd-5gt6hsrh
**amd-5gt6hsrh** (2026-11-24-switching-a-session-s-model-and-effort — Authentication & Authorization) — added decision/switch-ends-only-its-own-child and decision/switch-refusal-order. These are the subject of R6a (an idle switch read on its next turn) and R6b (a switch refused mid-turn).

@history_item n=39 marker=amd-mhueqyr5
**amd-mhueqyr5** (2026-11-24-switching-a-session-s-model-and-effort — Threat Model Summary) — added contract/wrapper-channel-methods/switch (its refusal words include human-typing and unverified-cli) and decision/switch-method-behind-ipc-controls.

@history_item n=40 marker=amd-upyi5vid
**amd-upyi5vid** (2026-11-24-switching-a-session-s-model-and-effort — Input Validation) — added contract/channel-frames/switch-params and added switch to decision/launch-flags-checked.

@history_item n=41 marker=amd-ukincsso
**amd-ukincsso** (2026-11-24-switching-a-session-s-model-and-effort — Data Protection) — changed decision/launch-pair-kept-to-snapshot-and-event-log to "one launch line at start and one per switch, never in the user's Claude settings". This is the "his settings byte-equal" half of R6a.

@history_item n=42 marker=amd-mt7eqn5h
**amd-mt7eqn5h** (2026-11-24-switching-a-session-s-model-and-effort — Error Handling) — added switch to decision/cli-frame-verbs-strict-modes-first.

@history_item n=43 marker=amd-76oabfbm
**amd-76oabfbm** (2026-11-24-switching-a-session-s-model-and-effort — Security Anti-Patterns) — made switch exit 12 on unverified-cli (contract/cli-exit-codes/12). A real-session switch reading needs a verify stamp covering that CLI.

@history_item n=44 marker=amd-rwvfu3ft
**amd-rwvfu3ft** (2026-11-24-viola-focus — Threat Model Summary) — added viola focus to contract/cli-exit-codes/21 ("A frame verb, or viola focus, refused …"). The console's f key drives focus.

@history_item n=45 marker=amd-3w3vmtkj
**amd-3w3vmtkj** (2026-11-24-viola-focus — Threat Model Summary) — corrected decision/state-tree-strict-modes-before-use to drop focus again, because focus runs the instance check and not check_home_tree. This is a trap: list a verb under the check it actually runs.

@history_item n=46 marker=amd-aqrz2f4i
**amd-aqrz2f4i** (2026-11-24-viola-focus — Error Handling) — added focus to decision/cli-frame-verbs-strict-modes-first ("before any frame or compositor read").

@history_item n=47 marker=amd-gj7rkr67
**amd-gj7rkr67** (2026-11-24-viola-focus — Authentication & Authorization) — added decision/focus-raises-only-ancestry-windows. A fleet-driving console focuses windows only within that bound.

@history_item n=48 marker=amd-kimkrw4z
**amd-kimkrw4z** (2026-11-24-viola-focus — the operator's answer to F-A ratified) — the operator's F-A arm 1 ("a command for the human only") ratified exit 21 for focus and the frame-verbs-first row.

@history_item n=49 marker=amd-cyffjwmn
**amd-cyffjwmn** (2026-11-23-model-and-effort-at-launch — Input Validation 1 of 2) — added decision/launch-flags-checked, decision/launch-pair-argv-crossing (revive hands the recorded pair onto the resumed child's argv), decision/launch-flags-refused-after-dashes and decision/board-launch-pair-re-parsed. These are "the model and effort handed again" on restore, and the v2-30 runs-on reading.

@history_item n=50 marker=amd-lwle2z4t
**amd-lwle2z4t** (2026-11-23-model-and-effort-at-launch — Threat Model Summary) — added the closed model-word and effort-word checks to decision/cli-input-trust-boundary.

@history_item n=51 marker=amd-h7kkyskc
**amd-h7kkyskc** (2026-11-23-model-and-effort-at-launch — Data Protection) — first landed decision/launch-pair-kept-to-snapshot-and-event-log, where the pair leaves only through list --json and the MCP list item.

@history_item n=52 marker=amd-nsmu6byb
**amd-nsmu6byb** (2026-11-23-model-and-effort-at-launch — Authentication & Authorization) — added decision/launch-pair-authenticates-nothing.

@history_item n=53 marker=amd-rsny4v7o
**amd-rsny4v7o** (2026-11-23-model-and-effort-at-launch — Input Validation 2 of 2) — added decision/effort-word-check-in-agent-crate, an exception to shared validators living in core.

@history_item n=54 marker=amd-oiti77hg
**amd-oiti77hg** (2026-11-22-lineage-registry — Input Validation) — added decision/run-lineage-flags-checked and decision/board-lineage-and-links-name-checked. "Overseers wrapped with their parents declared" (v2-28, cry-saswcxef) rests on these checks.

@history_item n=55 marker=amd-ypoq3sdq
**amd-ypoq3sdq** (2026-11-22-lineage-registry — Authentication & Authorization) — added decision/lineage-parent-authenticates-nothing (the parent comes from --parent or the inherited VIOLA_NAME and is never an identity). A declared parent in the fleet grants no authority.

@history_item n=56 marker=amd-4yavc3aj
**amd-4yavc3aj** (2026-11-22-lineage-registry — Data Protection) — added decision/lineage-kept-to-snapshot-and-event-log.

@history_item n=57 marker=amd-6vsxv2tl
**amd-6vsxv2tl** (2026-11-23-seam-requests-for-viola-open — Authentication & Authorization) — added decision/hyprland-recorder-moves-own-window (the test-only recorder moves only a terminal it started). This is a precedent for any live compositor step. The scope allows no window move on his desktop.

@history_item n=58 marker=amd-uc6w5fo7
**amd-uc6w5fo7** (2026-11-14-hyprland-seam — Input Validation) — added decision/hyprctl-read-deadline (5 s, killed and reaped) and the seam's Address validator to decision/shared-validators-in-core's except. Read-only compositor queries named in the scope stay under this deadline.

@history_item n=59 marker=amd-5m7c4zbi
**amd-5m7c4zbi** (2026-11-14-hyprland-seam — Data Protection) — added decision/hyprland-reply-text-dropped: window titles never reach a line or an error. Any read-only compositor query on his host keeps no title text.

@history_item n=60 marker=amd-kls4yh5k
**amd-kls4yh5k** (2026-11-14-hyprland-seam — Threat Model Summary) — changed decision/child-output-tolerant and decision/child-output-tolerant-unknown so a failure becomes unknown or a closed failure word.

@history_item n=61 marker=amd-yqn3v56o
**amd-yqn3v56o** (2026-11-22-paste-write-bounded — Threat Model Summary) — gave decision/bridge-send-one-bracketed-paste and decision/send-text-is-one-bracketed-paste the paste-cut except. "An opening message into a resumed session" is a send under these rows.

@history_item n=62 marker=amd-q3meownq
**amd-q3meownq** (2026-11-22-paste-write-bounded — Error Handling) — added contract/channel-refusal/paste-cut.

@history_item n=63 marker=amd-xsehduha
**amd-xsehduha** (2026-11-19-context-as-a-local-command — Threat Model Summary) — changed contract/wrapper-channel-methods/send so a listed local command sent before its transcript exists answers confirmed false (unconfirmable). This is a trap for an opening message sent at once into a freshly resumed session.

@history_item n=64 marker=amd-npnbpxay
**amd-npnbpxay** (2026-11-16-send-confirmation — Threat Model Summary) — replaced the exact-match row with decision/delivery-confirmation-trailing-tolerance.

@history_item n=65 marker=amd-gii3fh35
**amd-gii3fh35** (2026-11-16T06-05-33-wrap — trailing-only tolerance ratified, Threat Model Summary) — the founder ratified the trailing-only tolerance. A leading trim waits for a live reading, so no other tolerance is built.

@history_item n=66 marker=amd-l2ajoemi
**amd-l2ajoemi** (2026-11-16-local-command-delivery — Input Validation) — added decision/transcript-path-shape-check and decision/transcript-read-bounded, and the transcript check to decision/shared-validators-in-core's except. A resumed session's transcript ("the folder's newest transcript") may be read only within these bounds.

@history_item n=67 marker=amd-ow5es6l4
**amd-ow5es6l4** (2026-11-16-local-command-delivery — Data Protection) — added decision/transcript-path-redacted (the path is in no event, line, reply or record). A "newest transcript" pick must not print the path.

@history_item n=68 marker=amd-p6q452zp
**amd-p6q452zp** (2026-11-16-scheduled-wake-count — Data Protection) — last set decision/api-error-turn-end-text-dropped: no error text, and one nine-word cause crossing into wait --json and MCP wait. It bounds "a wait that prints the last message".

@history_item n=69 marker=amd-bvogo5l6
**amd-bvogo5l6** (2026-10-19-overseer-switch-to-viola — question option text crosses into the event and wait payloads) — the founder's own widening let question option description and preview text cross into events, hook.dialog, wait --json and MCP wait (decision/question-option-text-crosses-boundary). "A wait that prints a pending dialog's text" rests on this crossing and no wider.

@history_item n=70 marker=amd-sx3c5orj
**amd-sx3c5orj** (2026-11-15-live-round-on-the-trimmed-product — Authentication & Authorization) — added decision/live-round-rig-keys-own-pty, which needs his word per chunk. Any live rig step of this chunk needs his word anew.

@history_item n=71 marker=amd-fxcdo265
**amd-fxcdo265** (2026-11-15-live-round-on-the-trimmed-product — Data Protection) — added decision/live-session-screens-kept-local (screen frames only under a gitignored local path, only closed-word summaries landing). This binds any live reading's evidence.

@history_item n=72 marker=amd-qkep5vnr
**amd-qkep5vnr** (2026-11-17-standing-live-rig — Authentication & Authorization) — turned decision/live-round-rig-keys-own-pty into the standing rig's closed argv grant.

@history_item n=73 marker=amd-sdgerwnu
**amd-sdgerwnu** (2026-11-17-standing-live-rig — Secret Management) — added decision/live-rig-credentials-copy and decision/live-rig-isolated-child. U6 arm 3 ("no credential copy, no code that copies one") contradicts the copy row for R4b. A no-copy live arm must take the row's VH-style except, or be his fork, and is not built on a delegate's word.

@history_item n=74 marker=amd-bqtrjts2
**amd-bqtrjts2** (2026-11-17-first-readings-on-the-standing-rig — Authentication & Authorization) — settled that the rig's stop (Ctrl-C into its own pty, or a no-key group kill) needs no word. Only the argv grant needs his word per chunk.

@history_item n=75 marker=amd-e5wmqgif
**amd-e5wmqgif** (2026-11-17-first-readings-on-the-standing-rig — Secret Management) — reworded decision/live-rig-isolated-child: the user's ~/.claude.json is read only after a moved stamp, searched for rig-made needles. This is the "under a private config" half of cry-jeokvkdk's guard.

@history_item n=76 marker=amd-evepzqrw
**amd-evepzqrw** (2026-11-17-first-readings-on-the-standing-rig — Input Validation) — added decision/live-rig-prompt-capture-closed.

@history_item n=77 marker=amd-vzuvoiac
**amd-vzuvoiac** (2026-11-19-a-live-stopfailure-payload — Input Validation) — last widened decision/live-rig-prompt-capture-closed to StopFailure error words and text lengths, never a text.

@history_item n=78 marker=amd-r342utoe
**amd-r342utoe** (2026-11-19-a-live-stopfailure-payload — Secret Management) — added decision/live-rig-sf-arm-network-closed (the sandbox refusal precedes any credentials read). This is a precedent for containing a live arm before it touches credentials.

@history_item n=79 marker=amd-77k63hef
**amd-77k63hef** (2026-11-18-the-live-home-s-re-stamp-and-the-product-re-pin — Secret Management) — gave decision/live-rig-isolated-child the VH-arm except: it runs under the real home and config, reads no credentials and prints no value.

@history_item n=80 marker=amd-qb7muvik
**amd-qb7muvik** (2026-11-18-the-live-home-s-re-stamp-and-the-product-re-pin — Secret Management) — gave decision/live-rig-credentials-copy the VH-arm except: no copy, and no credentials file read. This is the one standing form of a live arm on the real home with no credential copy, and the nearest precedent for R4b under U6 arm 3.

@history_item n=81 marker=amd-fmbo3x4m
**amd-fmbo3x4m** (2026-11-18-the-live-home-s-re-stamp-and-the-product-re-pin — Data Protection) — added decision/live-rig-context-arm-keeps-no-transcript.

@history_item n=82 marker=amd-yy5a677v
**amd-yy5a677v** (2026-11-22T17-29-41Z-wrap — ratified, 2026-11-17-standing-live-rig Secret Management) — the founder ratified the rig's credentials-copy and isolation rows. His later U6 arm 3 answer for R4b narrows what a reboot arm may do beside them.

@history_item n=83 marker=amd-5mxpt3aw
**amd-5mxpt3aw** (2026-11-18-browser-tools-prompt-modal — Secret Management) — last set decision/live-rig-seed-from-bundle-logic: no value read from the user's config.

@history_item n=84 marker=amd-yytfovag
**amd-yytfovag** (2026-11-19-a-fired-scheduled-wake-keeps-the-wheel — Authentication & Authorization) — added decision/live-rig-hb-arm-keeps-attributes-only.

@history_item n=85 marker=amd-iqlhd4rk
**amd-iqlhd4rk** (2026-11-19-a-fired-scheduled-wake-keeps-the-wheel — Data Protection) — added decision/live-rig-wake-arm-reads-own-wheel-lines and widened decision/live-rig-kept-stamp-owner-only.

@history_item n=86 marker=amd-z4sujral
**amd-z4sujral** (2026-11-21-key-after-a-resize — Authentication & Authorization) — added decision/live-rig-kr-arm-keeps-attributes-only.

@history_item n=87 marker=amd-zg5yoeah
**amd-zg5yoeah** (2026-11-21-key-after-a-resize — Data Protection) — generalised decision/live-rig-kept-stamp-owner-only (a stamp is kept owner-only from a scratch home of its own and seeded only into its own), with no arm labels.

@history_item n=88 marker=amd-qwrscm3p
**amd-qwrscm3p** (2026-11-21-mcp-calls-bounded — Data Protection) — added decision/live-rig-mb-arm-keeps-no-output.

@history_item n=89 marker=amd-f2ikqoqo
**amd-f2ikqoqo** (2026-11-18-the-2-1-325-recorded-fixture-set — Data Protection) — added decision/live-rig-mcp-arm-keeps-no-output.

@history_item n=90 marker=amd-i4r2kggb
**amd-i4r2kggb** (2026-11-19-context-as-a-local-command — Data Protection) — added decision/live-rig-config-top-names-only and decision/live-rig-kept-stamp-owner-only.

@history_item n=91 marker=amd-mgjnx7tx
**amd-mgjnx7tx** (2026-11-18-session-folder-trusted-by-viola — Data Protection) — added decision/start-trust-skips-trusted-folders and extended decision/trust-record-writer-guarded, decision/no-writes-under-claude-home and decision/viola-writes-nothing-in-claude-dir to the start's persisting trust flag. A fleet start or restore in each folder goes through this writer.

@history_item n=92 marker=amd-cu4fmc7c
**amd-cu4fmc7c** (2026-11-18-session-folder-trusted-by-viola — Input Validation) — made the recorded cwd of a revive also the key of the start's trust record (decision/revive-recorded-cwd-spawn-dir, decision/revive-cwd-spawn-only).

@history_item n=93 marker=amd-wcbqjrb7
**amd-wcbqjrb7** (2026-11-18-session-folder-trusted-by-viola — Threat Model Summary) — widened decision/cwd-only-in-snapshot, decision/spawn-cwd-snapshot-only and decision/nothing-written-under-claude-home to the Claude config.

@history_item n=94 marker=amd-5t3eupw2
**amd-5t3eupw2** (2026-11-18-session-folder-trusted-by-viola — Data Protection) — added decision/start-writes-trust-record-flag-only, ratified with its price: a human's earlier "no" for the folder is overridden.

@history_item n=95 marker=amd-q7uiwun7
**amd-q7uiwun7** (2026-11-18-session-folder-trusted-by-viola — Input Validation) — added the start's 16 closed trust words to decision/trust-record-closed-words.

@history_item n=96 marker=amd-4sakddpi
**amd-4sakddpi** (2026-11-18-session-folder-trusted-by-viola — Secret Management 1 of 2) — extended the CLAUDE_CONFIG_DIR value-read excepts to run's and revive's start.

@history_item n=97 marker=amd-4sxz5z6g
**amd-4sxz5z6g** (2026-11-17-trust-record-ledger-row — Authentication & Authorization) — gave decision/explicit-mode-before-write its except: the trust writer keeps Claude's config file's own mode.

@history_item n=98 marker=amd-pikr4h2t
**amd-pikr4h2t** (2026-11-14-status-line-interception-removed — Threat Model Summary) — reworded decision/replayed-snapshot-yields-nothing and contract/cli-state-root/home-flag / viola-dir with no statusline_command. A replayed snapshot (for example after a crash) yields no frame.

@history_item n=99 marker=amd-vzhwfuz2
**amd-vzhwfuz2** (2026-11-14-status-line-interception-removed — Input Validation 2 of 2) — reworded decision/replayed-snapshot-fallbacks and decision/replayed-snapshot-inert (no endpoint, pid or start time; the collision check by the heartbeat alone). A post-reboot restore over a torn or newer snapshot meets these fallbacks.

@history_item n=100 marker=amd-p7254a3f
**amd-p7254a3f** (2026-11-14-status-line-interception-removed — Input Validation 1 of 2) — trimmed decision/max-frame-shared-cap and decision/home-strict-modes-before-read of the statusline reads.

@history_item n=101 marker=amd-uyh4basd
**amd-uyh4basd** (2026-11-14-status-line-interception-removed — Authentication & Authorization) — limited decision/strict-modes-check to readers of snapshot.json and ledger/stamps.json.

@history_item n=102 marker=amd-jkaqfv65
**amd-jkaqfv65** (2026-11-14-status-line-interception-removed — Data Protection) — left the plugin files as the only code-bearing files rewritten at every start (decision/code-bearing-files-rewritten-each-start).

@history_item n=103 marker=amd-5665sqdu
**amd-5665sqdu** (2026-11-14-status-line-interception-removed — Security Anti-Patterns) — reworded the why of decision/snapshot-written-only-by-wrapper: the endpoint in the snapshot decides where clients connect. Nothing but the instance's own wrapper may write it, including any restore helper.

@history_item n=104 marker=amd-qqrepp7q
**amd-qqrepp7q** (2026-11-14-status-line-interception-removed — Bootstrap phases) — set decision/user-content-only-to-diagnostics's except: events.ndjson, wait, last and hook.dialog carry content by design.

@history_item n=105 marker=amd-y27ejyen
**amd-y27ejyen** (2026-11-14-status-line-interception-removed — Bootstrap phases) — reworded decision/user-content-only-to-diagnostics's rule. User content goes only to diagnostics outside those payloads, which bounds what a wait print may show.

@history_item n=106 marker=amd-g3td2om6
**amd-g3td2om6** (2026-11-22T17-29-41Z-wrap — Input Validation) — added "or lifts the unverified CLI gate" to decision/config-cannot-disable-controls.

@history_item n=107 marker=amd-4qbr2oe3
**amd-4qbr2oe3** (2026-11-14-budget-gate-removed — Input Validation 1 of 2) — dropped the budget gate from decision/config-cannot-disable-controls.

@history_item n=108 marker=amd-e4fxmjmx
**amd-e4fxmjmx** (2026-11-13-rows-proposal-rule-work — Data Protection) — extended decision/logs-owner-only to the heartbeat, with a no-symlink owner-only append open.

@history_item n=109 marker=amd-a3mziedv
**amd-a3mziedv** (2026-11-13-verify-readiness-on-the-input-box — Input Validation) — added decision/input-box-readiness-from-literals, the gate an opening message into a resumed session must pass.

@history_item n=110 marker=amd-zhyjbrd4
**amd-zhyjbrd4** (2026-11-20-a-quoted-modal-never-blocks-a-send — Input Validation) — changed decision/input-box-readiness-from-literals to "never while a standing modal is up".

@history_item n=111 marker=amd-vjnqackf
**amd-vjnqackf** (2026-11-20-a-quoted-modal-never-blocks-a-send — Input Validation) — gave decision/input-box-readiness-from-literals its quote except; a stand-alone quote line still refuses. This is the pre-send gate an opening message meets.

@history_item n=112 marker=amd-ijqhczgw
**amd-ijqhczgw** (2026-11-06-version-done-check — the board's read rule confirmed) — the founder's word confirmed decision/list-home-read-rule / decision/board-home-read-rule: a readable home is refused with or without an instance. A board read-back ("an instance came up") follows it.

@history_item n=113 marker=amd-x2oocdat
**amd-x2oocdat** (2026-11-06-version-done-check — handle inheritance measured on portable-pty) — decision/no-handle-inheritance now rests on a live fd-table reading.

@history_item n=114 marker=amd-hkfcxgt5
**amd-hkfcxgt5** (2026-11-06-version-done-check — Measurement B recorded as known) — one tree's release builds can key two pinned folders, recorded as known with the re-hash as the guard. This bears on "an instance driven by another binary" and on the fleet's older prototype binary keying its own bin/ folder.

@history_item n=115 marker=amd-zuhqs6fi
**amd-zuhqs6fi** (2026-11-05-linux-only-code-and-ci — the IPC endpoint is a Unix socket alone) — the endpoint is a per-user 0700 directory under $XDG_RUNTIME_DIR/viola/ or /tmp/viola-<uid>/. After a reboot the runtime dir is fresh, and restore re-binds there.

@history_item n=116 marker=amd-zoeaxb4o
**amd-zoeaxb4o** (2026-11-05-linux-only-code-and-ci — the home strict-modes check is owner and mode alone) — the strict-modes rows (decision/strict-modes-check, decision/strict-modes-read-rule, decision/unstattable-home-refused) now read owner and mode alone, and an unstattable home is refused.

@history_item n=117 marker=amd-pgq5ocey
**amd-pgq5ocey** (2026-11-05-linux-only-code-and-ci — client server verification is one Linux flow) — every client checks the socket dir, the peer euid, then the peer pid and start time, with no carve-out (decision/server-verified-before-any-frame, decision/client-peer-euid-check).

@history_item n=118 marker=amd-l2btr5qg
**amd-l2btr5qg** (2026-11-05-linux-only-code-and-ci — child program resolution) — set decision/run-resolves-absolute-program and claude as a bare name on PATH for list and mcp. A stand-in claude on PATH is what cry-jeokvkdk's no-claude guard must keep away.

@history_item n=119 marker=amd-6dirgg7r
**amd-6dirgg7r** (2026-11-05-linux-only-code-and-ci — the R8 strip matches names exactly) — the R8 strip matches names exactly and claude_env_keep is the pass-list (decision/r8-claude-env-strip, decision/claude-credentials-stripped-from-child).

@history_item n=120 marker=amd-5vfzlnsc
**amd-5vfzlnsc** (2026-11-05-linux-only-code-and-ci — verify's stop and the colour decision) — reworded the colour-facts row (decision/no-width-or-colour-override-env). The console's key line width comes from the host terminal size, with no env read.

@history_item n=121 marker=amd-cf34yque
**amd-cf34yque** (2026-11-01-unix-endpoint-and-home-hardening — the socket-directory check) — set decision/checks-at-seams-not-channel-client and decision/socket-directory-check. Checks sit at the seams, and the channel client connects past them by design.

@history_item n=122 marker=amd-anlwj4zr
**amd-anlwj4zr** (2026-11-01-unix-endpoint-and-home-hardening — socket-dir-refused) — added socket-dir-refused to exit 21's closed causes (contract/cli-instance-unreachable/json-document) and to revive --json (contract/revive-json/socket-dir-refused).

@history_item n=123 marker=amd-hubagvb3
**amd-hubagvb3** (2026-11-01-unix-endpoint-and-home-hardening — a refused socket directory at a hook and at a start) — added decision/start-checks-socket-dir-before-bind and contract/socket-dir-refusal/run-and-revive-start. The first start after a reboot creates the socket dir, and a race's loser reads the same verdict.

@history_item n=124 marker=amd-eogaeelp
**amd-eogaeelp** (2026-11-01-unix-endpoint-and-home-hardening — the client's peer euid check and the order at a seam) — set decision/client-seam-check-order and decision/client-peer-euid-check (a refused euid reads server-verify-failed). A console or fleet client keeps this order.

@history_item n=125 marker=amd-ho4nohh4
**amd-ho4nohh4** (2026-11-01-unix-endpoint-and-home-hardening — the board's one rule on the home) — the board scan holds the home to the read rule (decision/list-home-read-rule).

@history_item n=126 marker=amd-iznbdfht
**amd-iznbdfht** (2026-11-01-unix-endpoint-and-home-hardening — a killed wrapper's leftover socket file) — a killed wrapper's socket file stays until the next bind of that name takes it over, and nothing removes it. A restore after a crash or reboot rebinds over it.

@history_item n=127 marker=amd-tsvxhgce
**amd-tsvxhgce** (2026-10-31-exit-cause-code-catalogue — exit 21 names its cause in detail) — exit 21 carries a closed cause in detail (contract/cli-instance-unreachable/json-document, contract/cli-json-error/strict-modes-failed). A fleet read-back reads only these words.

@history_item n=128 marker=amd-ugrtlmgv
**amd-ugrtlmgv** (2026-10-31-exit-cause-code-catalogue — the no-endpoint reading) — set instance-dead vs unwrapped-name. After a reboot every instance reads instance-dead until restored.

@history_item n=129 marker=amd-a37wdtds
**amd-a37wdtds** (2026-10-31-exit-cause-code-catalogue — a stale holder's code at the collision check) — added contract/collision-check/stale-heartbeat and already-live. After a reboot a restore meets stale holders whose pids are gone.

@history_item n=130 marker=amd-g3esiqyy
**amd-g3esiqyy** (2026-10-31-exit-cause-code-catalogue — revive --json documents every refusal of its start) — revive --json documents every start refusal with closed codes (contract/revive-json/* rows, contract/revive-json/error-document, contract/revive-json/internal-error). Restore's read-back of each instance reads these codes.

@history_item n=131 marker=amd-sdvrxace
**amd-sdvrxace** (2026-10-31-exit-cause-code-catalogue — the catch site's document under --json) — added contract/cli-json-error/internal-error (run and revive's start stay silent).

@history_item n=132 marker=amd-bt2fw3dh
**amd-bt2fw3dh** (2026-10-31-sanitised-error-surfaces — the clients' screen of a peer's reply) — set decision/client-reads-closed-values-only, decision/reply-code-words-validated and contract/channel-fault/unknown-code. Console and fleet clients show only closed words from a peer.

@history_item n=133 marker=amd-6wkya775
**amd-6wkya775** (2026-10-31-sanitised-error-surfaces — the error bodies' census) — added decision/every-error-writer-named: a new product error writer must join the census.

@history_item n=134 marker=amd-57ydhv4e
**amd-57ydhv4e** (2026-10-27-pinned-copy-race-and-reproducible-builds — the pinned viola placed, never replaced) — set decision/pinned-viola-never-replaced and decision/pinned-copy-no-clobber-placement.

@history_item n=135 marker=amd-m4px326a
**amd-m4px326a** (2026-10-27-pinned-copy-race-and-reproducible-builds — the first-start race closed) — set decision/pinned-binary-placed-once and decision/lost-placement-rehash (ETXTBSY on concurrent first starts). A fleet started or restored at once runs into this race.

@history_item n=136 marker=amd-ws2bsod6
**amd-ws2bsod6** (2026-10-27-pinned-copy-race-and-reproducible-builds — one tree's release builds can key two pinned folders) — the measurement behind the later "known" record. Binaries coexist per pinned key.

@history_item n=137 marker=amd-o677no2k
**amd-o677no2k** (2026-10-27-bounded-inputs-at-every-boundary — the bounded-reader census) — set decision/bounded-readers-census: any new reader (console, restore, reboot script helper in product code) must be bounded or named.

@history_item n=138 marker=amd-ap3enen5
**amd-ap3enen5** (2026-10-26-home-and-code-bearing-file-integrity — the dated strict-modes gaps retired) — set decision/strict-modes-before-snapshot-read and decision/client-strict-modes-first.

@history_item n=139 marker=amd-cus6gsms
**amd-cus6gsms** (2026-10-26-home-and-code-bearing-file-integrity — the collision check read through the replay) — set decision/collision-check-needs-no-server-verify over a replayed read (a fresh replayed beat is already-live).

@history_item n=140 marker=amd-mk6igznz
**amd-mk6igznz** (2026-10-26-home-and-code-bearing-file-integrity — check_instance's set and the read rule) — set decision/strict-modes-read-rule and the Readable refusal (contract/strict-refusal-message/readable).

@history_item n=141 marker=amd-xuzwgrgf
**amd-xuzwgrgf** (2026-10-26-home-and-code-bearing-file-integrity — the home tree checked at run and revive's start) — set decision/start-checks-home-tree-before-log-opens and decision/strict-modes-at-start.

@history_item n=142 marker=amd-24x45dat
**amd-24x45dat** (2026-10-26-home-and-code-bearing-file-integrity — the snapshot readers move onto read_snapshot_or_replay) — set decision/classified-snapshot-read-only (no product read_snapshot) and decision/replayed-snapshot-fallbacks.

@history_item n=143 marker=amd-yexrjpag
**amd-yexrjpag** (2026-10-26-server-verification-before-any-frame — run's collision check: the server check retired as moot) — the origin of decision/collision-check-needs-no-server-verify.

@history_item n=144 marker=amd-zqh3rwyd
**amd-zqh3rwyd** (2026-10-26-explicit-session-links — the served link / unlink methods) — the founder's widening behind contract/wrapper-channel-methods/link / unlink (from is not read).

@history_item n=145 marker=amd-akjbvlc5
**amd-akjbvlc5** (2026-10-26-explicit-session-links — link / unlink's instance check before any log) — the trap: opening a role file narrows an existing home to 0700, so a check placed after start passes a 0770 home. Any new verb this chunk adds checks first.

@history_item n=146 marker=amd-e54gcl5j
**amd-e54gcl5j** (2026-10-26-explicit-session-links — the link lines) — link and unlink write only the two names to events.ndjson.

@history_item n=147 marker=amd-xhxx3ana
**amd-xhxx3ana** (2026-11-15T18-01-44-wrap — the served link and unlink methods' halt confirm ratified) — the link and unlink methods stand ratified.

@history_item n=148 marker=amd-ambqnrib
**amd-ambqnrib** (2026-10-20-t06-58-31-wrap — the founder's Epoch 5 batch) — revive's refused check writing no line is his (decision/strict-refusal-leaves-no-trace, contract/strict-modes-refusal/run-revive). This applies to restore through revive.

@history_item n=149 marker=amd-mwc7q2vw
**amd-mwc7q2vw** (2026-10-19-newer-claude-cli-rows — the scratch footer reading accepted) — one scratch start of the real CLI was accepted as "no product path and no standing permission: another reading needs its own answer". This is the precedent for List B's live readings, each needing his word.

@history_item n=150 marker=amd-fvz7juca
**amd-fvz7juca** (2026-10-18-plugin-precedence-live-readings — the live round's rig outside the work tree) — the rig passes the trust dialog only by keys into its own pty, never a desktop key. Live steps never key his real terminal.

@history_item n=151 marker=amd-3oiym264
**amd-3oiym264** (2026-10-18-plugin-precedence-live-readings — the VIOLA_DIR home step ratified) — the founder ratified decision/cli-home-resolution-order (--home, then the VIOLA_NAME/VIOLA_DIR pair, then the default). Fleet children and console actions strip VIOLA_NAME/VIOLA_DIR for this reason.

@history_item n=152 marker=amd-okzja6o7
**amd-okzja6o7** (2026-10-17-cli-output-discipline — the terminal colour facts) — the origin of decision/no-width-or-colour-override-env: COLUMNS is not read, and the 80-column floor reading of the key line uses the host size.

@history_item n=153 marker=amd-wrds3kue
**amd-wrds3kue** (2026-10-17-mcp-server-for-drivers — the mcp process spawns claude agents --json) — the widening behind decision/claude-agents-resolved-from-path for MCP list.

@history_item n=154 marker=amd-d74wzrrq
**amd-d74wzrrq** (2026-10-16-unwrapped-sessions-on-the-board — the claude agents --json child) — set decision/claude-agents-read-fail-open and decision/unwrapped-name-never-target, and revive refusing session-live. Board reads in live steps spawn claude unless it is guarded off PATH.

@history_item n=155 marker=amd-otl23zzy
**amd-otl23zzy** (2026-10-14-resume-and-compact-ledger-rows — a third session-id source) — widened decision/session-id-argv-closed-shape with another id source. A new source ("the folder's newest transcript") is a new crossing needing its own ratification.

@history_item n=156 marker=amd-zh2jq57t
**amd-zh2jq57t** (2026-10-13-t06-38-35-wrap — the board's refusal his) — decision/whole-board-refusal and its no-line shape are the founder's.

@history_item n=157 marker=amd-cccya7ze
**amd-cccya7ze** (2026-10-11-the-board-viola-list — viola list joins the strict-modes entry points) — set decision/list-scan-checks-before-open and decision/whole-board-refusal, and list rows escaping every control (decision/human-output-escapes-controls).

@history_item n=158 marker=amd-hje724bm
**amd-hje724bm** (2026-10-10-budget-governor — the two instance checks ahead of any log) — the measured trap behind decision/strict-refusal-leaves-no-trace: a log opened before or after a refused check narrows the home.

@history_item n=159 marker=amd-vi2zwy7z
**amd-vi2zwy7z** (2026-10-10-viola-revive — the revived child: the recorded cwd ratified) — the founder ratified the recorded cwd as a spawn dir (decision/revive-recorded-cwd-spawn-dir). The logged id was recorded unratified, a precedent that a crossing not shown to him lands with his word owed.

@history_item n=160 marker=amd-b7itse67
**amd-b7itse67** (2026-10-10-viola-revive — revive reads its own state) — set decision/revive-strict-modes-before-use and decision/revive-trailing-words-as-argv.

@history_item n=161 marker=amd-4pl4d6hl
**amd-4pl4d6hl** (2026-10-10-statusline-pass-through — revive: the session id ratified) — the founder ratified the logged session id on --resume (decision/session-id-argv-no-shell). Note the rejected option "allow only an id the human types".

@history_item n=162 marker=amd-psva565i
**amd-psva565i** (2026-10-10-session-links — the link-set restore reads the log) — a start restores its links from events.ndjson, so a revived fleet keeps its driver links.

@history_item n=163 marker=amd-6vkwo27n
**amd-6vkwo27n** (2026-10-09-inner-cr-and-crlf-in-a-sent-text) — set decision/typed-text-line-ending-normalisation. Any other rewrite needs its own ruling.

@history_item n=164 marker=amd-nldbkmqu
**amd-nldbkmqu** (2026-10-09-epoch-3-cleanup — trailing CR and LF removed; an empty typed text refused) — set decision/empty-typed-text-refused and contract/send-refusal/empty-text.

@history_item n=165 marker=amd-esb55l5a
**amd-esb55l5a** (2026-10-08-first-live-test-and-self-drive — the human's terminal on viola run's stdin) — a key reaching viola run's stdin is the human's whoever typed it, and contract/send-refusal/human-typing rests on that stdin classifier. An agent once typed keys only on a compositor it started itself, nested and with the desktop lock untouched. This is a precedent for "no key to a real terminal" in this chunk.

@history_item n=166 marker=amd-ntq53yr2
**amd-ntq53yr2** (2026-10-07-live-rows-and-paste-shapes-on-the-dev-host — two scratch measurements across the credential boundary) — standing rule: those answers are spent, and another measurement across the credential boundary needs its own showing.

@history_item n=167 marker=amd-slat2ang
**amd-slat2ang** (2026-10-04-confirmed-send-with-cl-1-records — send's input validation as landed) — set decision/paste-validation-on-both-sides and contract/send-refusal/control-character.

@history_item n=168 marker=amd-5ndzdd2g
**amd-5ndzdd2g** (2026-09-28-capability-ledger-and-viola-verify — verify's CI split) — the origin of decision/real-cli-local-only (verify in CI only against the fake agent). The chunk's CI proofs stay over the fakes.

@history_item n=169 marker=amd-vrudj5qx
**amd-vrudj5qx** (2026-09-27-wrapper-channel — founder ratification of three standing boundary widenings) — standing rule: a boundary widening halts for his live answer, and an earlier delegate direction never ratifies one. The scope's "a widening is never built on a delegate's word" is this rule.
