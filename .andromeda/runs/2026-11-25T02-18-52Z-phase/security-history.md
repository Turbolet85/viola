@history_item n=1 marker=2026-11-14-hyprland-seam
**2026-11-14-hyprland-seam** (Threat Model Summary: 2 row(s)) — ~ decision/child-output-tolerant and ~ decision/child-output-tolerant-unknown: a failure becomes "unknown or a closed failure word", because the hyprctl reply read maps failures to the seam's closed words (oversize, malformed, unreachable). A new move form's reply must parse into the same closed set. It cannot add free text.

@history_item n=2 marker=2026-11-14-hyprland-seam
**2026-11-14-hyprland-seam** (Input Validation: 3 row(s)) — Three changes. ~ decision/shared-validators-in-core gains its except: the Address validator lives in the seam crate. ~ decision/parser-fuzz-scope adds the hyprctl reply parsers to the fuzz and property scope. + decision/hyprctl-read-deadline adds the 5 s deadline with kill and reap. A changed MoveToWorkspace request or reply parser stays under the fuzz and property coverage and under this deadline. It must stay inside the closed request set, never a second seam.

@history_item n=3 marker=2026-11-14-hyprland-seam
**2026-11-14-hyprland-seam** (Data Protection: 1 row(s), amd-5m7c4zbi) — + decision/hyprland-reply-text-dropped: reply types hold no text field. Titles, classes and every other string are dropped at the parse and reach no log or error. This binds any reply that a new no-focus move dispatch returns.

@history_item n=4 marker=2026-11-14-hyprland-seam
**2026-11-14-hyprland-seam** (Data Protection: 1 row(s), amd-bqrnqhz3) — + decision/hyprland-recorder-string-allowlist: viola-hypr-tap keeps a string only under a closed per-request key allowlist, inside the founder's answer to the recorder fork. It governs the move-fixture re-record this chunk plans.

@history_item n=5 marker=2026-11-23-seam-requests-for-viola-open
**2026-11-23-seam-requests-for-viola-open** (Input Validation: 1 row(s)) — ~ decision/hyprctl-read-deadline state: provisional to standing at an unchanged 5 s, on the operator's own word. Moving off 5 s for a new dispatch form would reopen a settled value.

@history_item n=6 marker=2026-11-23-seam-requests-for-viola-open
**2026-11-23-seam-requests-for-viola-open** (Data Protection: 1 row(s), amd-lhxjm56s, rule) — ~ decision/hyprland-recorder-string-allowlist rule: monitors keeps no string, and a non-JSON reply, which is the move's, is kept only as `ok`. This is why the move fixture holds only `ok` (the scope's carry). A re-recorded move fixture of a new form must hold to the same rule, or this row changes.

@history_item n=7 marker=2026-11-23-seam-requests-for-viola-open
**2026-11-23-seam-requests-for-viola-open** (Data Protection: 1 row(s), amd-tmetbr6e, guard) — ~ decision/hyprland-recorder-string-allowlist guard: now `record_replaces_every_string_off_the_allowlist; record_keeps_no_monitors_string`. The cell holds at most two guard names. A new move-recording test has no room in it without displacing one.

@history_item n=8 marker=2026-11-23-seam-requests-for-viola-open
**2026-11-23-seam-requests-for-viola-open** (Authentication & Authorization: 1 row(s)) — + decision/hyprland-recorder-moves-own-window, provisional on the delegate's host rule: the test-only recorder moves only the window of a terminal it started, then ends it and waits for the window to go. This bounds the operator-granted live re-record ("one terminal window, no claude").

@history_item n=9 marker=2026-11-23-viola-open-2
**2026-11-23-viola-open-2** (Input Validation: 5 row(s)) — Rows changed or added:
- ~ decision/hyprctl-read-deadline: hyprctl is resolved on PATH, with the R8 strip's names on the wrapper's read.
- + decision/open-terminal-direct-spawn.
- + decision/run-window-flags-checked: `--origin` cli|agent and an integer `--place`, a ratified widening.
- + decision/board-window-and-origin-re-parsed.

These are the window step's and `open`'s input bounds that the changed move sits behind.

@history_item n=10 marker=2026-11-23-viola-open-2
**2026-11-23-viola-open-2** (Threat Model Summary: 1 row(s)) — ~ decision/state-tree-strict-modes-before-use: `open` joins run, revive's start and verify in checking the home tree before any read or log. `open` is a caller of the move.

@history_item n=11 marker=2026-11-23-viola-open-2
**2026-11-23-viola-open-2** (Authentication & Authorization: 2 row(s)) — Two rows added:
- + decision/window-origin-authenticates-nothing.
- + decision/window-step-moves-own-terminal-window: the product moves only its own terminal's window, found by parent pid within 5 s, at most once.

This is the core bound on the move this chunk reshapes. A no-focus form must keep "own window, at most once".

@history_item n=12 marker=2026-11-23-viola-open-2
**2026-11-23-viola-open-2** (Data Protection: 2 row(s)) — + decision/window-record-kept-to-snapshot-and-event-log: no address, pid or workspace enters a process-log line. + decision/open-ancestor-walk-in-memory. Any active-workspace reading the new acceptance clause takes must not leak a workspace into a process log.

@history_item n=13 marker=2026-11-24-viola-focus
**2026-11-24-viola-focus** (Threat Model Summary: 2 row(s)) — `focus`, the first product dispatch to the compositor after the move, was added to the state-tree row and to contract/cli-exit-codes/21 (exit 21 strict-modes-failed). This is a precedent for compositor-dispatching verbs and their refusal shape.

@history_item n=14 marker=2026-11-24-viola-focus
**2026-11-24-viola-focus** (Threat Model Summary: 1 row(s)) — ~ decision/state-tree-strict-modes-before-use is corrected back. `focus` runs the instance check, not check_home_tree. This trap shows the verb lists in these rows track which check a verb actually calls.

@history_item n=15 marker=2026-11-24-viola-focus
**2026-11-24-viola-focus** (Authentication & Authorization: 1 row(s)) — + decision/focus-raises-only-ancestry-windows, provisional and later made standing. It is the sibling compositor-dispatch bound and the counterpart to F4's "no focus change". The move must not become a focus.

@history_item n=16 marker=2026-11-24-viola-focus
**2026-11-24-viola-focus** (the operator's answer to F-A ratified) — F-A arm 1 ratified: focus is a command for the human only. contract/cli-exit-codes/21 and decision/cli-frame-verbs-strict-modes-first stand. This is the wrap the scope's second carry came from (R4a read 1 before and 5 after).

@history_item n=17 marker=2026-11-24-viola-up
**2026-11-24-viola-up** (Input Validation: 3 row(s)) — Three rows extended for `up`:
- ~ decision/open-terminal-direct-spawn: `up` spawns pinned `revive <name> --origin`.
- ~ decision/run-window-flags-checked: revive's hidden `--origin` asks a window only at the workspace a real snapshot recorded.
- ~ contract/revive-arguments/id.

`up` and `revive --origin` are the move's other callers, which the scope requires to read the same answer.

@history_item n=18 marker=2026-11-24-viola-up
**2026-11-24-viola-up** (Threat Model Summary: 3 row(s)) — Three rows changed:
- ~ contract/cli-exit-codes/21: up's board-scan refusal.
- ~ decision/cwd-only-in-snapshot and ~ decision/spawn-cwd-snapshot-only: the recorded cwd crosses onto up's terminal argv.

This is the boundary widening behind `up`, the caller whose R4a reading exposed the defect.

@history_item n=19 marker=2026-11-24-viola-up
**2026-11-24-viola-up** (Threat Model Summary: 2 row(s)) — The two cwd rows were set provisional. The cwd crossing onto up's argv was a widening of its own outside U2 and was asked of the operator as a fork.

@history_item n=20 marker=2026-11-23-viola-open-2
**2026-11-23-viola-open-2** (Threat Model Summary: 2 row(s)) — decision/cwd-only-in-snapshot and decision/spawn-cwd-snapshot-only set standing on the operator's W1 answer: the recorded cwd on up's terminal argv, as built.

@history_item n=21 marker=2026-11-24-viola-up
**2026-11-24-viola-up** (Authentication & Authorization: 1 row(s), two entries) — + contract/strict-modes-refusal/up and + decision/up-checks-before-log-and-spawn, both on the delegate's word, provisional. These are up's check order before its log and its first spawn of a placed start.

@history_item n=22 marker=2026-11-24-viola-adopt
**2026-11-24-viola-adopt** (Threat Model Summary: 1 row(s), amd-4mwl66bv and amd-3oykhjjv) — ~ decision/state-tree-strict-modes-before-use gains adopt's start. ~ decision/nothing-written-under-claude-home: the start of run, revive and adopt sets the trust flag of an untrusted spawn folder. These are the start rows of the run-start path that the folded automatic verify runs inside.

@history_item n=23 marker=2026-11-24-viola-adopt
**2026-11-24-viola-adopt** (Error Handling: 3 row(s)) — Three rows gain adopt's start:
- ~ decision/home-tree-strict-modes-before-log.
- ~ decision/run-refuses-socket-dir-before-start.
- ~ contract/cli-json-error/internal-error: adopt is silent at the catch site.

These are the picked rows the start, and the automatic verify run there, must respect.

@history_item n=24 marker=2026-11-24-viola-adopt
**2026-11-24-viola-adopt** (Authentication & Authorization: 3 row(s)) — ~ decision/start-checks-home-tree-before-log-opens and ~ decision/start-checks-socket-dir-before-bind gain adopt's start. + decision/adopt-signals-no-foreign-process. This is the start's check order, which the folded fix must not move.

@history_item n=25 marker=2026-11-24-viola-adopt
**2026-11-24-viola-adopt** (Authentication & Authorization: 2 row(s)) — + contract/strict-modes-refusal/adopt. + decision/r5-adopt-script-ends-own-processes, which bounds the R5 live-reading script that found the folded automatic-verify defect. It sets the precedent for operator-fired live legs.

@history_item n=26 marker=2026-11-24-viola-adopt
**2026-11-24-viola-adopt** (Data Protection: 3 row(s)) — ~ decision/start-writes-trust-record-flag-only: "On a verified CLI only" the start writes the trust record. ~ decision/no-writes-under-claude-home and ~ decision/viola-writes-nothing-in-claude-dir follow. The folded item turns on what an automatic verify stamps in an untrusted folder. Moving it, or trusting its folder first, touches this verified-only trust write.

@history_item n=27 marker=2026-11-24-viola-adopt
**2026-11-24-viola-adopt** (Secret Management: 3 row(s) (1 of 2)) — ~ decision/credential-strip-by-name-only, ~ decision/env-not-config-channel and ~ decision/no-stored-secrets: the start of run, revive and adopt reads CLAUDE_CONFIG_DIR's value when kept. These are the picked env carve-outs any change to where the automatic verify runs rests on.

@history_item n=28 marker=2026-11-24-viola-adopt
**2026-11-24-viola-adopt** (Secret Management: 1 row(s) (2 of 2)) — ~ decision/r8-claude-env-strip except: verify and the start of run, revive and adopt read CLAUDE_CONFIG_DIR's value. This bounds how a start-launched verify chooses the Claude config.

@history_item n=29 marker=2026-11-24-viola-adopt
**2026-11-24-viola-adopt** (Input Validation: 2 row(s)) — ~ decision/claude-agents-resolved-from-path and ~ decision/launch-flags-checked gain adopt. These picked rows bound the start path.

@history_item n=30 marker=2026-11-24-viola-adopt
**2026-11-24-viola-adopt** (Threat Model Summary — decision/nothing-written-under-claude-home ratified (W5), and its W5 siblings) — Adopt's trust-flag write is ratified on the operator's W5 arm a ("as any start"). Sibling entries ratify the same W5 for Secret Management (credential-strip, env-not-config-channel, no-stored-secrets, r8-claude-env-strip) and Data Protection (start-writes-trust-record-flag-only, no-writes, viola-writes-nothing). Changing what a start writes or reads there reopens a ratified answer.

@history_item n=31 marker=2026-11-17-trust-record-ledger-row
**2026-11-17-trust-record-ledger-row** (Data Protection: 3 row(s)) — Verify writes a trust record for its own fresh temp folder and removes only it, through + decision/trust-record-writer-guarded, which refuses a present key for verify. This is the ruled widening that lets verify run trusted. It is the mechanism the folded fix (verify in a trusted place) would rest on.

@history_item n=32 marker=2026-11-17-trust-record-ledger-row
**2026-11-17-trust-record-ledger-row** (Threat Model Summary: 1 row(s)) — ~ decision/nothing-written-under-claude-home except: verify's own trust record is the bounded exception.

@history_item n=33 marker=2026-11-17-trust-record-ledger-row
**2026-11-17-trust-record-ledger-row** (Authentication & Authorization: 1 row(s)) — ~ decision/explicit-mode-before-write except: the trust record writer keeps the Claude config file's own mode. This is a picked row.

@history_item n=34 marker=2026-11-17-trust-record-ledger-row
**2026-11-17-trust-record-ledger-row** (Input Validation: 1 row(s)) — + decision/trust-record-closed-words: the trust record outcome reaches the stamp only as a closed word. A stamp written on verify's own failed precondition must stay in closed words.

@history_item n=35 marker=2026-11-17-trust-record-ledger-row
**2026-11-17-trust-record-ledger-row** (Data Protection: 2 row(s)) — ~ decision/verify-probe-dirs-cleanup and ~ decision/probe-artifacts-cleanup: a SIGKILL leaves the probe dirs and verify's own trust record. This matters for the automatic verify run from a start.

@history_item n=36 marker=2026-11-18-session-folder-trusted-by-viola
**2026-11-18-session-folder-trusted-by-viola** (Data Protection: 4 row(s)) — + decision/start-trust-skips-trusted-folders. The start's trust write goes through the same guarded writer, refusing a present key for verify only. The no-writes excepts name the start's write. This shows how the start's trust step and verify's trust record share one writer, which the folded item's fix would touch.

@history_item n=37 marker=2026-11-18-session-folder-trusted-by-viola
**2026-11-18-session-folder-trusted-by-viola** (Data Protection: 1 row(s)) — + decision/start-writes-trust-record-flag-only, with its ratified price: an earlier no is overridden, and a lost write in the lock window is accepted.

@history_item n=38 marker=2026-11-18-session-folder-trusted-by-viola
**2026-11-18-session-folder-trusted-by-viola** (Input Validation: 1 row(s)) — ~ decision/trust-record-closed-words: a start's trust step ends in one of 16 start words, separate from verify's 14.

@history_item n=39 marker=2026-11-18-session-folder-trusted-by-viola
**2026-11-18-session-folder-trusted-by-viola** (Input Validation: 2 row(s)) — ~ decision/revive-recorded-cwd-spawn-dir and ~ decision/revive-cwd-spawn-only: the recorded cwd is also the key of the start's trust record. Picked rows. `revive --origin` is a move caller.

@history_item n=40 marker=2026-11-18-session-folder-trusted-by-viola
**2026-11-18-session-folder-trusted-by-viola** (Threat Model Summary: 3 row(s)) — ~ decision/cwd-only-in-snapshot and ~ decision/spawn-cwd-snapshot-only gain the Claude config trust key. ~ decision/nothing-written-under-claude-home except.

@history_item n=41 marker=2026-11-18-session-folder-trusted-by-viola
**2026-11-18-session-folder-trusted-by-viola** (Secret Management: 3 row(s) (1 of 2)) — The CLAUDE_CONFIG_DIR value read is extended from verify to run and revive's start in the credential-strip, env-not-config-channel and no-stored-secrets excepts. This is the precedent for widening the verify-only env carve-out.

@history_item n=42 marker=2026-11-18-browser-tools-prompt-modal
**2026-11-18-browser-tools-prompt-modal** (Authentication & Authorization: 1 row(s)) — ~ decision/unverified-cli-gate: every relied row must count, which means pass, or absent on the browser-tools-prompt row alone. The folded failing stamp (9 pass, 19 fail, 1 absent) keeps the version unverified under exactly this rule.

@history_item n=43 marker=2026-11-18-browser-tools-prompt-modal
**2026-11-18-browser-tools-prompt-modal** (Security Anti-Patterns (NEVER do these): 1 row(s), two entries) — ~ decision/dialog-answers-need-verify-stamp and ~ contract/cli-exit-codes/12, both to the counts rule. These are the gate a failing automatic-verify stamp keeps shut.

@history_item n=44 marker=2026-11-19-a-live-stopfailure-payload
**2026-11-19-a-live-stopfailure-payload** (the StopFailure exception settled on the live reading, kept on the founder's word — Authentication & Authorization) — The except of decision/unverified-cli-gate is settled and no longer provisional: StopFailure has no ledger row. The gate row's except is ratified text.

@history_item n=45 marker=2026-11-22T17-29-41Z-wrap
**2026-11-22T17-29-41Z-wrap** (Input Validation: 3 row(s)) — ~ decision/config-cannot-disable-controls: "or lifts the unverified CLI gate". The verify_new_cli key only starts a start's automatic verify, and only a green stamp lifts the gate. This is the direct precedent for the folded item: the automatic verify is config-started and never a gate bypass. ~ decision/verify-reads-only-signatures and ~ decision/verify-pty-output-caught carry the compiled-shapes parenthetical.

@history_item n=46 marker=2026-11-22T17-29-41Z-wrap
**2026-11-22T17-29-41Z-wrap** (Threat Model Summary: 3 row(s)) — ~ decision/verify-screen-read-against-literals and ~ decision/verify-signature-only carry the compiled-shapes parenthetical. A modal or untrusted-folder precondition check on the automatic verify must be read only through these compiled signatures.

@history_item n=47 marker=2026-11-18-the-live-home-s-re-stamp-and-the-product-re-pin
**2026-11-18-the-live-home-s-re-stamp-and-the-product-re-pin** (Threat Model Summary: 1 row(s)) — ~ decision/verify-never-answers-dialogs: a browser-tools prompt at start is left for the human. Verify never types into a dialog. An automatic verify that meets the trust dialog in an untrusted folder can only refuse or skip, never answer it.

@history_item n=48 marker=2026-10-05-real-cli-verify-probes
**2026-10-05-real-cli-verify-probes** (verify's typed-input PTY probe: the widening, its dirs, residual and refusal) — Three changes:
- The founder's rulings: no key into any dialog, and a modal start is killed with no key.
- A dev-host prerequisite: run verify from a trusted folder whose external import is answered. This is the source of decision/verify-never-answers-import(-dialog), and the root of the folded defect: an automatic verify in an untrusted scratch folder violates this prerequisite.
- The named fixture refusal codes: home-path, absolute-path, username, email.

@history_item n=49 marker=2026-10-05-dialog-rows-and-re-probe
**2026-10-05-dialog-rows-and-re-probe** (the capture arm answers, Runs C and D, R2 closed) — Run D's plan file goes in its own 0700 `plans/`, never `~/.claude/plans`, which is the source of decision/verify-probes-confined-to-own-dir. The stamp's relied set closed R2, so an all-but-failing stamp reads `cli_verified:false`.

@history_item n=50 marker=2026-11-18-trust-record-row-read-live
**2026-11-18-trust-record-row-read-live** (Data Protection: 1 row(s)) — ~ decision/screen-fixture-signature-rows-only except: on a failed recording verify, a hidden rig-only option keeps one first-screen file of whole scrubbed rows per run short of the box, kept local and git-ignored. This is the existing route for reading what a failing verify saw.

@history_item n=51 marker=2026-11-18-trust-record-row-read-live
**2026-11-18-trust-record-row-read-live** (Threat Model Summary: 2 row(s)) — decision/verify-signature-only and decision/verify-screen-read-against-literals gain the same first-screen except. These are picked rows.

@history_item n=52 marker=2026-11-18-trust-record-row-read-live
**2026-11-18-trust-record-row-read-live** (Input Validation: 1 row(s)) — + decision/verify-stamp-first-screen-closed-words: the stamp's first screen holds only closed words, whole-number settles and fact ids. Any new "own-precondition failed" stamp field must use the same closed form.

@history_item n=53 marker=2026-11-13-verify-readiness-on-the-input-box
**2026-11-13-verify-readiness-on-the-input-box** (Input Validation: 1 row(s)) — + decision/input-box-readiness-from-literals: one predicate decides box readiness for verify and run's gate.

@history_item n=54 marker=2026-11-20-a-quoted-modal-never-blocks-a-send
**2026-11-20-a-quoted-modal-never-blocks-a-send** (Input Validation: 1 row(s), two entries) — Both change decision/input-box-readiness-from-literals: its rule now refuses only while a standing modal is up, and its except says a quoted label is not a modal. This is the predicate an automatic verify's own modal precondition would read.

@history_item n=55 marker=2026-11-20-a-quoted-modal-never-blocks-a-send
**2026-11-20-a-quoted-modal-never-blocks-a-send** (Input Validation: 2 row(s)) — ~ decision/verify-reads-only-signatures and ~ decision/verify-pty-output-caught: verify reads compiled signatures and the compiled shapes over them.

@history_item n=56 marker=2026-11-20-a-quoted-modal-never-blocks-a-send
**2026-11-20-a-quoted-modal-never-blocks-a-send** (Threat Model Summary: 2 row(s)) — ~ decision/verify-screen-read-against-literals and ~ decision/verify-signature-only get the same compiled-shapes wording.

@history_item n=57 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (`ledger/stamps.json` checked before `ledger/` is made, and `verify`'s home tree before its log) — This closed the second dated gap. update_stamps runs check_stamps before creating ledger/ and again under its lock. Verify runs check_home_tree before its cli log. Trap: a check after a create or log open narrows the path first. Any rework of when an automatic verify stamps must keep this order.

@history_item n=58 marker=2026-10-04-dialog-answers-by-dialog-id
**2026-10-04-dialog-answers-by-dialog-id** (run's strict stamps read; decisions on the six-row stamp until :82) — run's version gate reads stamps through read_stamps_strict. A refusal or a malformed file reads `cli_verified:false`. This is the source of decision/version-gate-strict-stamps-read and contract/strict-modes-refusal/version-gate.

@history_item n=59 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** (verify's refusal codes) — Verify writes a closed code per refusal on its process-exit. A failing row is no refusal and no code. update_stamps' strict-modes refusal carries `strict-modes-failed`. An automatic verify refusing on its own precondition would need a code of this closed set, or a new one, which is a widening.

@history_item n=60 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (child program resolution is a given path or a bare name on PATH; the .cmd/.bat refusal retired) — decision/run-resolves-absolute-program and verify's refusal codes shrank to five, `batch-script-child` gone. These are the picked contract/verify-refusal-codes rows as they now stand.

@history_item n=61 marker=2026-11-21-cross-session-prefix-ledger-row
**2026-11-21-cross-session-prefix-ledger-row** (Data Protection: 3 row(s)) — ~ decision/fixture-recording-scrubbed and ~ decision/fixture-scrub-and-refusal: the refusal covers home, drive-letter or user-folder paths only, as measured. A runtime-dir path passes. This bounds the scrub a re-recorded Hyprland move fixture passes through.

@history_item n=62 marker=2026-11-21-cross-session-prefix-ledger-row
**2026-11-21-cross-session-prefix-ledger-row** (Data Protection: 1 row(s), amd-ssxajcrx) — ~ contract/fixture-refusal-codes/absolute-path: home, drive-letter or user-folder path only.

@history_item n=63 marker=2026-11-21-cross-session-prefix-ledger-row
**2026-11-21-cross-session-prefix-ledger-row** (Secret Management: 1 row(s)) — + decision/verify-runtime-dir-held-in-memory: verify reads XDG_RUNTIME_DIR once and keeps it in memory, never in a stamp or log. This is a picked row.

@history_item n=64 marker=2026-11-20-harness-prefix-and-identity-floor-ledger-rows
**2026-11-20-harness-prefix-and-identity-floor-ledger-rows** (Secret Management: 1 row(s)) — + decision/env-probe-names-and-counts-only: the stamp's env probe holds only names and counts. This is a picked row on what a stamp may hold.

@history_item n=65 marker=2026-11-19-context-as-a-local-command
**2026-11-19-context-as-a-local-command** (Security Anti-Patterns (NEVER do these): 1 row(s)) — ~ decision/stamps-written-only-by-verify except: the test-only rig seeds a kept stamp into its own scratch home. Only verify writes stamps. A fix that discards a failing automatic-verify stamp is a stamps write and must stay verify's.

@history_item n=66 marker=2026-11-19-a-fired-scheduled-wake-keeps-the-wheel
**2026-11-19-a-fired-scheduled-wake-keeps-the-wheel** (Security Anti-Patterns (NEVER do these): 2 row(s)) — ~ decision/stamps-written-only-by-verify except widened to the CC or R3W scratch homes.

@history_item n=67 marker=2026-10-13-interrupted-verify-cleanup
**2026-10-13-interrupted-verify-cleanup** (verify's dirs removed on a Ctrl-C / TERM stop, the `--settings` move done, the matrix count) — Verify's probe dirs are removed on pass, fail or stop. Guards remove only what their own run created, never by pattern. This is the source of decision/verify-probe-dirs-cleanup, which a start-launched verify still owes.

@history_item n=68 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** (a refused socket directory at a hook and at a start) — run and revive's start exit 1 `socket-dir-refused` through socket_dir_ready before any snapshot, child or bind. This is the source of contract/socket-dir-refusal/run-and-revive-start and decision/start-checks-socket-dir-before-bind.

@history_item n=69 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (the home tree checked at `run` and `revive`'s start, before the wrapper log) — check_home_tree runs at run and revive's start before the wrapper log, exit 1 with no log, line or child. This is the source of decision/start-checks-home-tree-before-log-opens and contract/strict-modes-refusal/run-revive.

@history_item n=70 marker=2026-10-10-budget-governor
**2026-10-10-budget-governor** (the two instance checks ahead of any log) — Trap: a log opened before or after a refused check narrows the home to 0700 and erases the refused state. This is the source of decision/strict-refusal-leaves-no-trace and decision/strict-check-before-narrowing.

@history_item n=71 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (`check_instance`'s set and the read rule as built) — The read rule covers the home, instance and diagnostics/, with the Readable and Symlink refusals. This is the source of decision/strict-modes-before-read and decision/strict-modes-read-rule.

@history_item n=72 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (the diagnostics opens refuse a symlink or a widened path) — check_diagnostics runs before narrowing. A refusal sends the writer to a sink. This is the source of decision/diagnostics-open-fails-to-sink and decision/diagnostics-strict-modes-fail-open.

@history_item n=73 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (the home strict-modes check is owner and mode alone; the Windows DACL arm retired) — The strict-modes check is owner and mode alone. An unstattable home is a refusal. This is the source of decision/unstattable-home-refused and the current owner-only rows.

@history_item n=74 marker=2026-11-13-rows-proposal-rule-work
**2026-11-13-rows-proposal-rule-work** (Data Protection: 1 row(s)) — ~ decision/logs-owner-only: the owner-only append open never follows a symlink, and the 0600 mode is set through the handle.

@history_item n=75 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (the snapshot readers move onto `read_snapshot_or_replay`) — No product code calls read_snapshot. Readers go through the classified read or the replay after the strict-modes check. This is the source of decision/classified-snapshot-read-only and the rule the window step's snapshot reads follow.

@history_item n=76 marker=2026-10-27-bounded-inputs-at-every-boundary
**2026-10-27-bounded-inputs-at-every-boundary** (the bounded-reader census, the compare read bounded by its own length, the own-executable reads named) — The census `product_readers_are_each_bounded_or_named` requires every product reader to be bounded or named. A new hyprctl reply read for the no-focus move or the active-workspace query must be bounded or listed.

@history_item n=77 marker=2026-10-27-bounded-inputs-at-every-boundary
**2026-10-27-bounded-inputs-at-every-boundary** (serde_json's `unbounded_depth` banned in `deny.toml`) — The ban is enforced by deny-probes. A hyprctl JSON reply parse keeps the default recursion limit.

@history_item n=78 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** (the error bodies' census and what it does not drive) — The `product_error_writers_are_each_named` census and `thiserror_messages_interpolate_nothing`. A new refusal or error for the move or the automatic verify must be named there, with a fixed message.

@history_item n=79 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** (the clients' screen of a peer's reply) — Clients take a peer's refusal or detail only as a code word, through sanitised_fault and is_code_word. This is the source of decision/client-reads-closed-values-only.

@history_item n=80 marker=2026-09-27-epoch-2-cleanup
**2026-09-27-epoch-2-cleanup** (wsl-exec.sh ratified (overseer, live), test-only feature proven absent, host scratch guard) — release-check refuses test-support and fake-agent artifacts (probe 5/5). This is the source of decision/release-build-without-test-features. The Hyprland seam's fake and tap must stay out of the release build.

@history_item n=81 marker=2026-09-26-local-linux-pre-push-gate
**2026-09-26-local-linux-pre-push-gate** (`FAKE_AGENT_PUMP_DELAY_MS` carve-out; WSL pre-push distro: CI's pins, `env -i`) — The first fake-agent test seam. Another seam or environment read needs its own Decisions Log entry, as a boundary widening. This is the source of decision/test-seams-fake-agent-only and decision/fake-agent-pump-delay-seam. A new env switch for the Hyprland fake would fall under it.

@history_item n=82 marker=2026-09-28-hook-perf-gate
**2026-09-28-hook-perf-gate** (test seam `FAKE_AGENT_HOOK_PANIC` and G2's exact-path exemption, ratified by the founder) — The second test seam, ratified live. The ban names any further seam as needing its own entry. This is the source of decision/fake-agent-hook-panic-seam.

@history_item n=83 marker=2026-10-20-epoch-5-cleanup-ii
**2026-10-20-epoch-5-cleanup-ii** (the `Mcp.` exchange recorded by the test-only tap) — Precedent for a test-only tap re-recording a fixture from one live start. The capture is private and never committed, it is scrubbed and refused through closed codes, and the start is capped on the founder's word. This is the pattern viola-hypr-tap's live move re-record follows.

@history_item n=84 marker=2026-11-15-live-round-on-the-trimmed-product
**2026-11-15-live-round-on-the-trimmed-product** (Authentication & Authorization: 1 row(s)) — + decision/live-round-rig-keys-own-pty: live legs act only into their own pty, and each needs the founder's or operator's word anew. This is the precedent for holding the move re-record and v2-07's live half as gated, operator-fired legs.
