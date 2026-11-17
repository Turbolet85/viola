@history_item n=1 marker=2026-11-17-standing-live-rig
**2026-11-17-standing-live-rig** (Occupied Resources: 4 rows) — added name/binary/viola-live-rig and name/other/agent-run-rig, and widened decision/test-binaries-never-released and decision/harness-env-unread-by-product to name them. This chunk reads on that committed rig, so any arm it adds (wake, latency, imports-modal, verify-record) stays test-only, never ships in a release build, and keeps AGENT_RUN_RIG unread by viola.

@history_item n=2 marker=2026-11-17-standing-live-rig
**2026-11-17-standing-live-rig** (Established Decisions: 2 rows) — rewrote decision/submitted-form-prompt-match's why from R2's live trim reading (a trailing run trimmed, a leading run kept), and added the except to decision/vt100-readiness-gate that lets the rig use vt100 for its child's trust dialog, selector and input box. R2's unexplained `box-text: holds-text` comes from this same rig screen model. Reading it may touch the readiness rule's wording.

@history_item n=3 marker=2026-11-17-standing-live-rig
**2026-11-17-standing-live-rig** (Stack and Technologies: 1 row) — widened decision/vt100-screen-model to cover the screen the test-only live rig reads of its child. Any new rig arm that reads the screen (imports modal, box text) falls under this row.

@history_item n=4 marker=2026-11-17-booted-wrapper-does-not-outlive-its-test
**2026-11-17-booted-wrapper-does-not-outlive-its-test** (Occupied Resources) — registered the test-only markers ROOT_TIED_TEST_CHILD and HARNESS_TIED_TEST_CHILD. The nextest kill-line fix joins `harness_tied` to the 20 s line, so it rests on these tied-child markers and their ending.

@history_item n=5 marker=2026-11-16-scheduled-wake-count
**2026-11-16-scheduled-wake-count** (Standard Contracts: 3 rows) — added contract/event-data/turn-ended-scheduled-wakes (the length of `session_crons`, null when absent or on an API-error end). It also widened the turn-ended data and wait-wake-kinds rows: a scheduled wake is counted but never wakes `wait`. This is the row R3b reads live as 1; that chunk claimed only the measured 0 and synthetic counts.

@history_item n=6 marker=2026-11-16-scheduled-wake-count
**2026-11-16-scheduled-wake-count** (Conventions: 1 row) — contract/hook-to-kind-map/stopfailure gained `scheduled_wakes` null. This bounds what R3b may read on an API-error end.

@history_item n=7 marker=2026-11-17T05-21-35Z-wrap
**2026-11-17T05-21-35Z-wrap** (scheduled-wake-count ratifications, Standard Contracts and Conventions) — the founder ratified the delegate's confirm of the scheduled_wakes rows. R3b's live reading confirms a ratified contract and does not reopen it.

@history_item n=8 marker=2026-11-16-wait-wake-condition
**2026-11-16-wait-wake-condition** (Standard Contracts: 3 rows) — pinned that `wait` wakes by event kind alone, and that a harness-origin prompt-submitted and an activity record never wake it. A fired wake's prompt-submit (R3a) arrives as a prompt the hook classifies. How it is filed (harness vs human) bears on «A fired scheduled wake keeps the wheel», which this chunk must not build.

@history_item n=9 marker=2026-11-16-send-confirmation
**2026-11-16-send-confirmation** (Established Decisions: 3 rows) — added decision/send-flight-lock (the post-gate reads, link, send-issued and paste under one wrapper flight lock that the claim and the turn start take) and replaced exact-prompt-match with decision/submitted-form-prompt-match. R3c measures the residual this lock leaves open: a turn the CLI starts whose UserPromptSubmit has not yet reached the wrapper.

@history_item n=10 marker=2026-11-16T06-05-33-wrap
**2026-11-16T06-05-33-wrap** (send trailing-only tolerance ratified, Established Decisions and Standard Contracts) — the founder ratified the trailing-only tolerance and made a leading trim wait for the live reading carried on «Standing live rig with a screen model». R2's reading has since landed. Any further trim reading here stays within that ruling.

@history_item n=11 marker=2026-10-04-running-turn-refusal
**2026-10-04-running-turn-refusal** — built the in-memory running-turn state, marked on any prompt-submitted, and recorded the residual that a turn starting during the readiness gate's wait is not refused. This is the earliest form of the race R3c measures (a turn viola has not yet learned of).

@history_item n=12 marker=2026-10-07-send-waits-out-the-paste-hint
**2026-10-07-send-waits-out-the-paste-hint** — set GATE_MAX_WAIT to 8.5 s, made send read the wheel and the running turn again after the gate (retiring the 2026-10-04 residual), and set the send bound. Trap: a case that reads a verified literal-less screen through a clock that never advances waits forever. R3c's window sits beside this second read.

@history_item n=13 marker=2026-11-13-verify-readiness-on-the-input-box
**2026-11-13-verify-readiness-on-the-input-box** (Established Decisions: 2 rows) — changed decision/screen-signatures-compiled to accept the box's own rows (a rule row of at least 40 rule chars above a prompt-char row) beside the footer literals, and changed decision/verify-box-wait-for-literal to match. R2's `holds-text` reading of an empty-looking box, and R4's live record, both rest on this predicate.

@history_item n=14 marker=2026-11-13-verify-readiness-on-the-input-box
**2026-11-13-verify-readiness-on-the-input-box** (Standard Contracts: 1 row) — contract/ledger-stamps/measured gained `box_only_guards`. A live `viola verify --record` (R4) stamps this reading, and run reads none of it.

@history_item n=15 marker=2026-11-13-verify-readiness-on-the-input-box
**2026-11-13-verify-readiness-on-the-input-box** (Occupied Resources: 1 row) — decision/screen-fixtures-signature-rows-only now keeps the box's rule and prompt rows as their chars alone. A box whose live prompt row holds a home path, the username or an email refuses the recording. This is a direct trap for R4: the live record home and the prompt row must hold no such text.

@history_item n=16 marker=2026-10-19-newer-claude-cli-rows
**2026-10-19-newer-claude-cli-rows** — added the second input-box literal `(shift+tab to cycle)` after 2.1.302 hid `for agents`, widened the input-box-signature row to three screens, and recorded the 2.1.302 set whole in one `--record` round. This is the precedent for a new CLI version (2.1.326, unstamped) changing its footer: a stamp of a new version is its own reading, counted apart.

@history_item n=17 marker=2026-10-19-newer-claude-cli-rows
**2026-10-19-newer-claude-cli-rows** (the 2.1.302 fixture set) — committed 2.1.302 at 29 files under the rule `ls fixtures/claude/<v> | wc -l`, stamped through `contract_ledger_probes`'s STAMPED list. The scope bars re-recording a fixture set unless R4's own record home is the reading, so this is the shape any recorded output would take.

@history_item n=18 marker=2026-10-05-real-cli-verify-probes
**2026-10-05-real-cli-verify-probes** (screen signatures compiled) — compiled the modals list `Yes, I trust this folder` and `Yes, allow external imports`, built Runs A and B with no key into any CLI dialog, and measured on 2.1.288 that the external-imports approval is keyed on the git root and that trust is inherited from a trusted parent. These are the direct precedents for the external-imports modal reading. The rig's cwd and its git root decide whether the modal shows, and no key may be typed into it.

@history_item n=19 marker=2026-10-05-real-cli-verify-probes
**2026-10-05-real-cli-verify-probes** (registry rows) — registered verify's interactive probe dirs (0700, removed on every exit path), signature-only `--record` screens, and the named `--record` refusal (file plus closed code, never the content). This binds how R4's live `--record` writes and refuses.

@history_item n=20 marker=2026-10-05-t00-09-13-wrap
**2026-10-05-t00-09-13-wrap** — the founder ratified the typed-input verify probe against the live installed claude, local only, with no Claude credential on a CI runner. This is the basis of decision/real-cli-probe-local-only, which R4's live verify sits under.

@history_item n=21 marker=2026-10-05-dialog-rows-and-re-probe
**2026-10-05-dialog-rows-and-re-probe** — added Runs C and D, answered only by the probe hook and ended by a kill. Its `--record` writes the dialog variants, and it records the transcripts verify leaves. These are part of the bounded child set a live `viola verify --record` (R4) drives.

@history_item n=22 marker=2026-10-14-resume-and-compact-ledger-rows
**2026-10-14-resume-and-compact-ledger-rows** — added Run E and the `resume-session` row, and made a verify six claude starts. This counts toward the founder's live start cap for R4.

@history_item n=23 marker=2026-10-18-compact-post-condition-ledger-row
**2026-10-18-compact-post-condition-ledger-row** (the 24th row and Run B's sixth paste, and CARRY 2's transcript count) — added `/compact` to Run B with its 40 s window, and measured three transcript files per Run B. This is the cost and residual of a live verify on the rig.

@history_item n=24 marker=2026-10-06-local-command-and-paste-framing-rows
**2026-10-06-local-command-and-paste-framing-rows** — measured that 2.1.287's footer holds no input-box literal for 8.0 s after a long paste, and bounded Run B's waits by PROBE_DEADLINE. This is where decision/verify-box-wait-for-literal comes from, and it bears on R2's box reading after pastes.

@history_item n=25 marker=2026-11-16-local-command-delivery
**2026-11-16-local-command-delivery** (Established Decisions: 2 of 2) — added decision/local-command-echo-confirmation and decision/local-command-echo-probe (Run B's seventh paste). With no readable transcript the paste is skipped and the row fails. A live verify on the rig home (R4) must have a readable transcript for this row to pass.

@history_item n=26 marker=2026-10-07-live-rows-and-paste-shapes-on-the-dev-host
**2026-10-07-live-rows-and-paste-shapes-on-the-dev-host** (the long-paste frame) — the founder ruled decision/no-probe-for-unrelied-shapes: a shape no send relies on needs no probe. This is the precedent for this chunk's boundary that a reading here builds no ledger row or probe.

@history_item n=27 marker=2026-10-04-readiness-gate-and-timing-constants
**2026-10-04-readiness-gate-and-timing-constants** — built the pure vt100 screen on run's feed thread, made a caught panic poison the model until resize, and made a poisoned model read input-not-ready. This is the base of decision/poisoned-screen-model and decision/vt100-readiness-gate, which any screen reading here must respect.

@history_item n=28 marker=2026-09-28-capability-ledger-and-viola-verify
**2026-09-28-capability-ledger-and-viola-verify** (the landed ledger rows, verify's probe, the stamps envelope) — set up verify's stamping, merge-own-version-only, and `--record` writing scrubbed fixtures only at 0 fail. It also set the `--version` first-line rule `X.Y.Z (Claude Code)`. These are the base of every R4 reading, and of the 2.1.326 version read.

@history_item n=29 marker=2026-10-04-dialog-answers-by-dialog-id
**2026-10-04-dialog-answers-by-dialog-id** (decisions on the six-row stamp, the strict stamps read) — made run's version gate read stamps through `read_stamps_strict`, a refusal giving cli_verified false and one parse-rejected line. A start on PATH 2.1.326, which no stamp covers, reads unverified through this gate.

@history_item n=30 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (home-tree check before the start's and verify's log) — made verify run `check_home_tree` before its log: the home pair, exit 1, no line. This is decision/verify-refuses-untrusted-home-or-stamps. The record home R4 picks must pass it.

@history_item n=31 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (the stamps writer's strict-modes check) — made `update_stamps` check strict modes before creating `ledger/` and again under its lock. This is decision/stamps-writer-checks-before-and-under-lock, which R4's stamping goes through.

@history_item n=32 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** (verify's refusal codes on its process-exit) — added the closed codes `program-not-found`, `version-unreadable`, `unclean-fixture` and others under a parsed VIOLA_NAME, with no `--json`. These are the outcomes a live R4 record can end in, and they must be named as read.

@history_item n=33 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (the batch-script-child refusal retired) — cut verify's `Ended::Refused` codes from six to five. This is the current closed set the rows contract/viola-verify-refusal-codes/* hold.

@history_item n=34 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (CI on the one ubuntu-latest leg) — CI is `ubuntu-latest` only, 7 jobs, against the fake agent. This is the frame of decision/ci-ubuntu-fake-agent, and of the nextest `ci` profile the kill-line fix edits.

@history_item n=35 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (the relied set) — made `LedgerRow::relied()` every row of ALL on every build. This is the basis of decision/version-verified-all-relied-rows, which decides whether a 2.1.326 stamp verifies.

@history_item n=36 marker=2026-11-14-status-line-interception-removed
**2026-11-14-status-line-interception-removed** (Established Decisions: 5 rows) — retired the statusline rows (the relied set fell from 23 to 21), the second capture arm, and the snapshot's statusline command. It also made the run child take no `--settings`. A live verify today stamps the reduced row set, and decision/claude-flag-literals-in-agent-crate now names `--settings` for verify alone.

@history_item n=37 marker=2026-11-14-budget-gate-removed
**2026-11-14-budget-gate-removed** (Cross-cutting Patterns) — widened decision/mixed-version-tolerance so that a stamp holding a retired ledger row verifies on the relied rows alone. Older live-home stamps (2.1.296 to 2.1.325) read under this rule.

@history_item n=38 marker=2026-11-06-version-done-check
**2026-11-06-version-done-check** (the statusline-rate-limits row reads the account's plan state) — a verify on the rig home read `23 pass 1 fail` on an unchanged CLI because of the account's payload, ruled a finding. This is the precedent that a live verify on the rig home can fail on account state rather than on the CLI, so R4's reading must name its home and binary.

@history_item n=39 marker=2026-10-19-overseer-switch-to-viola
**2026-10-19-overseer-switch-to-viola** (verify's print-mode probe leaves an empty CLI project dir) — counted 4 new `~/.claude/projects/` entries per verify as the accepted residual. This is decision/verify-transcripts-accepted-residual, which R4's live round leaves.

@history_item n=40 marker=2026-10-13-interrupted-verify-cleanup
**2026-10-13-interrupted-verify-cleanup** — made verify's probe dirs removed on pass, fail, or a stop by Ctrl-C or TERM, and only the dirs that run created, never by a pattern. This is decision/verify-removes-only-own-dirs and verify-probe-dirs-drop-guarded, which bind a live R4 round stopped part-way.

@history_item n=41 marker=2026-10-16-unwrapped-sessions-on-the-board
**2026-10-16-unwrapped-sessions-on-the-board** (the Agents fixture class) — the agents registry row replays recorded values only, because the host's real cwd leaked a CI checkout path into `--record`. This is a hygiene trap for R4's live record home and cwd.

@history_item n=42 marker=2026-10-20-epoch-5-cleanup-ii
**2026-10-20-epoch-5-cleanup-ii** (the recorded live MCP exchange) — the `Mcp.` fixture class is recorded by `viola-mcp-tap`, not by `--record`, and is part of the 2.1.302 set's file count. A `--record` round does not write it, so a recorded set from R4 would not be whole by `--record` alone.

@history_item n=43 marker=2026-11-16T06-05-33-wrap
**2026-11-16T06-05-33-wrap** (StopFailure no-ledger-row ratified, Standard Contracts) — gave StopFailure's live reading to «The 2.1.325 recorded fixture set». That is the entry the scope's boundary keeps separate: no fixture set is re-recorded here.

@history_item n=44 marker=2026-09-29-verify-stamped-test-homes-and-harness
**2026-09-29-verify-stamped-test-homes-and-harness** (verify's spawn pairs, the harness CI read, verify-stamped homes) — made `run --local-live` refuse under CI, and has harness session homes stamped at `boot` step 4 by `viola verify` against the fake agent. These are decision/no-live-claude-in-ci and the stamping `boot` the nextest kill-line carry names (`harness_lifecycle`, `cli`, `harness_tied` each drive a stamping verify through `boot`).

@history_item n=45 marker=2026-10-17-end-to-end-path-binary-and-harness-legs
**2026-10-17-end-to-end-path-binary-and-harness-legs** — gave the harness session guard's Drop the keep variables, and added `boot --agents-mode` files. This is part of the harness `boot` path whose readiness deadline the harness_lifecycle case may shorten.

@history_item n=46 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** (the sanitised error surfaces pattern and its contract binary) — put two root binaries in the nextest `ci` profile's kill-line filter. This is the precedent for adding binaries to that filter, which the kill-line carry repeats for viola-e2e's three.

@history_item n=47 marker=2026-11-15-mutation-and-coverage-baselines-green
**2026-11-15-mutation-and-coverage-baselines-green** — raised the shared root wait bound WITHIN from 7 s to 12 s, and added decision/test-sockets-under-short-tmp-root. Test timing bounds moved recently, which bears on the measured walls behind the 20 s kill line.

@history_item n=48 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** (a force-killed wrapper's socket file) — made harness `cleanup` read a refused or missing socket as gone, so cleanup reads ok after a force-kill. This is decision/cleanup-reads-refused-or-missing-socket-gone, which bears on harness_lifecycle cases a kill line would end.

@history_item n=49 marker=2026-10-07-test-homes-off-the-contended-volume
**2026-10-07-test-homes-off-the-contended-volume** — linked the dev-host e2e-home to a tmpfs, kept by two keepers. Standing rule: a stalled-start red is a finding about the backing, reported and never re-run for green. The measured walls this chunk takes for the kill line fall under that rule.

@history_item n=50 marker=2026-10-20-epoch-5-cleanup
**2026-10-20-epoch-5-cleanup** (the keepers' headroom floor) — made the keepers refuse a linked base with less than 9 GiB free. This is decision/e2e-home-headroom-floor, which a live or booted round on the dev host meets first.

@history_item n=51 marker=2026-10-02-epoch-2b-cleanup
**2026-10-02-epoch-2b-cleanup** — made test homes removed through `remove_owned`, with the owner record last. This is decision/test-home-owner-sweep for homes a killed or shortened test leaves.

@history_item n=52 marker=2026-10-27-pinned-copy-race-and-reproducible-builds
**2026-10-27-pinned-copy-race-and-reproducible-builds** (the pinned copy on disk) — places the pinned copy once with no clobber, and a lost placement re-hashes the copy that stands. This fixed the ETXTBSY failures of concurrent first starts in booted tests, and is the source of decision/pinned-binary-copy and run-pins-checked-bytes.

@history_item n=53 marker=2026-11-15-live-round-on-the-trimmed-product
**2026-11-15-live-round-on-the-trimmed-product** (Established Decisions: 4 rows) — decision/wheel-starts-with-driver now counts a key typed into the folder-trust dialog before session-start as taking the wheel. This governs the rig's trust pass, and any start that shows the external-imports modal.

@history_item n=54 marker=2026-10-17-optional-plugin-install
**2026-10-17-optional-plugin-install** (plugin install as built) — that live round took none of its readings: inside the work tree the local scope landed at its root, and with `proj/` its own git repo the folder-trust dialog held both wrapped starts. This trap bears on the imports-modal arm, whose cwd must import a file outside the project.
