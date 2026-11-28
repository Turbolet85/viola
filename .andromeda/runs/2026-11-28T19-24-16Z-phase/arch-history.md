@history_item n=1 marker=2026-10-08-first-live-test-and-self-drive
**2026-10-08-first-live-test-and-self-drive** (first live on the Linux dev host) — moved the first live run to the Linux dev host and recorded the live readings on 2.1.287; the precedent for how a live reading is taken and recorded (as measured, on one version) that every List B reading of this chunk follows.

@history_item n=2 marker=2026-10-10-viola-revive
**2026-10-10-viola-revive** — landed `viola revive` (resume in place, events log appended, first record `wheel{driver,start}`), the snapshot's `cwd` as the revived child's second cwd source (`cwd-missing`), the replay's first product reader and `--resume <logged id>` on the child argv; H2 «restored after a reboot» rests on decision/revive-in-place, decision/revive-resumes-in-place, decision/child-cwd-explicit and contract/instance-snapshot/cwd as built here.

@history_item n=3 marker=2026-10-10-statusline-pass-through
**2026-10-10-statusline-pass-through** (four crossings ratified) — ratified the logged session id as `--resume`'s value on a revived child (decision/logged-session-id-on-child-argv); any new path that hands a session id to a child (e.g. «the folder's newest transcript») is a new crossing, not covered by this ratification.

@history_item n=4 marker=2026-10-16-unwrapped-sessions-on-the-board
**2026-10-16-unwrapped-sessions-on-the-board** — `list` joins one `claude agents --json` read (unwrapped rows, idle/busy), and `revive` refuses `session-live` when a live claude holds the logged id (measured: two writers on one transcript); bears on «a read-back that an instance came up» and on resuming a fleet whose sessions may still be open (decision/revive-refuses-live-session, contract/board/*).

@history_item n=5 marker=2026-10-17-cli-machine-contract
**2026-10-17-cli-machine-contract** — gave `revive` its `--json` documents on both arms and the CLI verbs the `VIOLA_DIR` home step; the machine forms a script-replacing caller (up/revive/open) reads, and the start's home resolution (decision/viola-home-resolution).

@history_item n=6 marker=2026-10-04-wait-and-last
**2026-10-04-wait-and-last** — built `wait {after?, timeout_ms?, from?}` and `last`, the blocking wake and the last-assistant-message rebuild; «a wait that prints the last message and a pending dialog's text» (the `pwait.sh` fact) is measured against contract/channel-wait/*, contract/channel-last/last-assistant-message and decision/wait-blocks-nothing-polls as built here.

@history_item n=7 marker=2026-10-19-dialog-hold-for-the-driver
**2026-10-19-dialog-hold-for-the-driver** — configurable dialog hold and the `dialog-expired` record, the one wrapper kind that wakes `wait`, and `pending_dialog` written before the event line; part of what a `pwait.sh`-style wait sees (contract/channel-wait/driver-relevant-events, contract/event-line/dialog-expired).

@history_item n=8 marker=2026-10-19-overseer-switch-to-viola
**2026-10-19-overseer-switch-to-viola** — the overseer's switch to driving through viola: `question` gained `option_details`, `turn-ended` `background_tasks`; the prior step of moving the fleet onto viola, and the precedent that option text stays upstream content, never a human line (decision/option-text-never-in-human-line).

@history_item n=9 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** — the start's `check_home_tree` before any log (exit 1, no line), the frame verbs' instance check first, and every snapshot reader moved onto the replay (a replayed snapshot gives no endpoint; one read per revive); constraints on any start or read-back this chunk adds (decision/instance-check-before-any-read, decision/snapshots-read-through-replay, decision/refused-check-writes-no-line, decision/start-pins-the-bytes-it-checked).

@history_item n=10 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** — `stale-heartbeat` split from `already-live`, `revive --json` documenting every refusal of its start (the closed code list), exit 21's closed causes in `detail`; the code sets a fleet restorer branches on (contract/viola-revive-json-error-codes/*, contract/cli-json-error-document/shape).

@history_item n=11 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** — the start's `socket_dir_ready` refusal (`socket-dir-refused`, exit 1) and the socket-dir/euid checks at every client; part of the start order and refusal set any new start path inherits (decision/socket-dir-strict-check, decision/socket-dir-refused-covers-create-and-check).

@history_item n=12 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** — viola is Linux only, CI on `ubuntu-latest` against the fake agent, the window features on Hyprland, no shell dependence; bounds where this chunk's rehearsals run and that real claude never runs in CI (decision/linux-only, decision/ci-against-fake-agent, decision/window-features-target-hyprland, decision/no-tmux-or-shell-dependency).

@history_item n=13 marker=2026-11-08T20-32-52-wrap
**2026-11-08T20-32-52-wrap** (version census 0.2.0) — retired the web/graphical frontend: the one view beyond `viola list` is the keyboard-driven terminal fleet console tiled on Hyprland, owed by the Epoch 6 console entries; the origin of decision/terminal-fleet-console that «driven from one console» (H3) rests on.

@history_item n=14 marker=2026-11-14-hyprland-seam
**2026-11-14-hyprland-seam** — `viola-hyprland` as the one compositor seam, hyprctl spawned directly with a closed argv, its five closed errors and the malformed-reply exception; any read-only compositor query this chunk names goes through this seam (decision/hyprctl-direct-spawn, contract/hyprland-seam-errors/*, decision/hyprland-seam-crate).

@history_item n=15 marker=2026-11-14-status-line-interception-removed
**2026-11-14-status-line-interception-removed** — no settings layer, no `--settings` at start: the child's argv is `--plugin-dir` then the caller's args; a script fact that leans on settings or a status line has no product home (absence/status-line-interception, contract/child-argv/*).

@history_item n=16 marker=2026-11-14-budget-gate-removed
**2026-11-14-budget-gate-removed** (Cross-cutting Patterns) — decision/mixed-version-tolerance now says an older writer's state reads whole and a stamp holding a retired row verifies on the relied rows; bears on moving a fleet partly on an older prototype binary onto this binary.

@history_item n=17 marker=2026-11-15-live-round-on-the-trimmed-product
**2026-11-15-live-round-on-the-trimmed-product** — a live round's readings landed as row changes: the wheel starts with `driver` but a key typed into the folder-trust dialog takes it; a trap for any live start of a fleet whose folders show the trust dialog (decision/wheel-starts-with-driver).

@history_item n=18 marker=2026-11-15-mutation-and-coverage-baselines-green
**2026-11-15-mutation-and-coverage-baselines-green** — the shared root wait bound `WITHIN` moved 7 s → 12 s and test channel sockets bind under a short `/tmp` root; the bound and socket location the rehearsal tests over the fakes inherit (decision/root-waits-share-one-bound).

@history_item n=19 marker=2026-11-17-standing-live-rig
**2026-11-17-standing-live-rig** — the test-only `viola-live-rig` binary and the `AGENT_RUN_RIG` census marker, never read by viola, plus vt100 as the rig's screen model; the instrument the List A live-step guard (no `claude` reachable, or a private config) and the rehearsal forms extend (decision/test-binaries-never-released, decision/vt100-readiness-gate except).

@history_item n=20 marker=2026-11-18-trust-record-row-read-live
**2026-11-18-trust-record-row-read-live** — the rig seeds its own private Claude config (a named test-only exception to the one-crate rule) and verify gained a rig-only hidden `--record-first-screens`; precedent for running a live step under a private config, the arm cry-jeokvkdk names (decision/claude-first-no-adapter-trait except).

@history_item n=21 marker=2026-11-18-session-folder-trusted-by-viola
**2026-11-18-session-folder-trusted-by-viola** — a run/revive start on a verified CLI writes the CLI's trust record for an untrusted spawn folder (keyed by the recorded `cwd` on revive), and reads a kept `CLAUDE_CONFIG_DIR`'s value; a restored fleet's folders and a private-config live step both pass through this (decision/run-start-order, decision/spawn-directory-confined-to-snapshot, contract/config-json/claude-env-keep).

@history_item n=22 marker=2026-11-19-a-live-stopfailure-payload
**2026-11-19-a-live-stopfailure-payload** — one contained live reading (the rig's start inside a network-closed sandbox, on the founder's word, one word on one version) settled a ledger exception; the precedent for a narrow, named live reading with nothing claimed beyond it, as List B's gated readings need.

@history_item n=23 marker=2026-11-20-a-new-cli-version-stamped-automatically
**2026-11-20-a-new-cli-version-stamped-automatically** — a start runs an automatic `viola verify` after the bind when `verify_new_cli` is on, under `ledger/verify.lock`, and records the final verdict; the start order every fleet start or restore goes through (decision/start-verifies-new-cli-version, decision/run-start-order).

@history_item n=24 marker=2026-11-22T17-29-41Z-wrap
**2026-11-22T17-29-41Z-wrap** (Cross-cutting Patterns) — decision/viola-home-resolution: `viola run` hands the home to its claude child only through `VIOLA_DIR` and to its automatic verify child only through `--home`; a constraint on how a fleet start hands each instance its home.

@history_item n=25 marker=2026-11-22-lineage-registry
**2026-11-22-lineage-registry** — `run --parent/--role`, the `lineage` event and snapshot field, `list`'s `parent`/`role`/`links`, revive keeping the recorded lineage, and decision/lineage-label-only; «the overseers wrapped with their parents declared» (v2-28, cry-saswcxef) rests on these rows.

@history_item n=26 marker=2026-11-23-model-and-effort-at-launch
**2026-11-23-model-and-effort-at-launch** — run/revive `--model/--effort`, the `launch` event, `default_model`/`default_effort`, revive handing the recorded pair, and decision/launch-pair-handed-not-verified; «the model and effort handed again» is built here, and v2-30's «runs on» half is the unmeasured gap this row names.

@history_item n=27 marker=2026-11-23-seam-requests-for-viola-open
**2026-11-23-seam-requests-for-viola-open** — the seam's dispatch set gained `hl.dsp.window.move` beside `hl.dsp.focus`, measured live on Hyprland 0.56.2; the only dispatch forms the seam admits, which this chunk must not fire on the founder's desktop (decision/hyprctl-direct-spawn).

@history_item n=28 marker=2026-11-23-viola-open-2
**2026-11-23-viola-open-2** — `viola open` (a terminal per instance on a named workspace), the `window` event/snapshot field, `run --origin/--place`, open's exit codes and `terminal-not-found`/`not-live`; and the later ratification of up opening each terminal in the recorded `cwd`; «a terminal per instance on a named workspace» is built here (contract/cli-verbs/open, contract/viola-open-exit-codes/*, contract/instance-snapshot/window).

@history_item n=29 marker=2026-11-24-viola-focus
**2026-11-24-viola-focus** — `viola focus <name> [--json]`, CLI only, with no driver-facing form (decision/focus-has-no-driver-form, ratified standing); R7's readings of where focus lands after the window step are readings of this verb, not a re-claim of v2-07.

@history_item n=30 marker=2026-11-24-viola-up
**2026-11-24-viola-up** — `viola up` (re-opens instances gone before the boot, parents first, writes no state of its own), revive's hidden `--origin`, up's exit codes and `boot-time-unread`; R4/R4b, v2-12 and the `up.sh` retirement measure against decision/up-selection-rule, decision/up-parents-first, decision/up-writes-no-state and contract/cli-verbs/up, all ratified at 2026-11-27T19-07-11Z-wrap.

@history_item n=31 marker=2026-11-24-viola-adopt
**2026-11-24-viola-adopt** — `viola adopt` resumes a plain claude session under viola by its session id, refusing `session-live`, through run's start and collision check; the built answer to «an instance driven by another binary» or a session started outside viola joining the fleet (decision/adopt-refuses-live-session, ratified W1).

@history_item n=32 marker=2026-11-24-adopt-s-resume-folder
**2026-11-24-adopt-s-resume-folder** — adopt's `not-resumed` code and its document printed after the child released the terminal, the one named exception to decision/no-document-after-child-held-terminal; a trap for adopting a session whose transcript is in another folder.

@history_item n=33 marker=2026-11-24-switching-a-session-s-model-and-effort
**2026-11-24-switching-a-session-s-model-and-effort** — `viola switch` in place (old child reaped within `RESPAWN_REAP_BOUND`, `launch` appended, resumed under the same wrapper), a standing switch refusing `send` `turn-running`, CLI-only, no user settings written; R6a/R6b (v2-31) read this build live (decision/switch-in-place, decision/send-never-queued).

@history_item n=34 marker=2026-11-25-placing-without-moving-the-view
**2026-11-25-placing-without-moving-the-view** — `viola verify` refuses `untrusted-folder` (no stamp) when a probe run's first screen reads the trust or imports form; a trap for any verify or automatic verify a live fleet step triggers in an untrusted folder.

@history_item n=35 marker=2026-11-27T19-07-11Z-wrap
**2026-11-27T19-07-11Z-wrap** — the operator's own word ratified switch-in-place, switch-cli-only, up-parents-first, up-writes-no-state, revive-pair-from-record and the Epoch 5 delegate marks (lineage, launch pair, open, focus, up); the rows this chunk rests on stand standing, not provisional.

@history_item n=36 marker=2026-11-28-console-scaffold
**2026-11-28-console-scaffold** — `viola console` registered (command name, the `console` role log, exit 1 refusals) and named the exception to `--json` (decision/cli-human-text-json-exit-codes, decision/cli-output-conventions, decision/new-capabilities-as-verbs), PROVISIONAL on the delegate; the surface F-K's key line lives on.

@history_item n=37 marker=2026-11-28-fleet-console-view
**2026-11-28-fleet-console-view** — the console's agents read ungated (decision/agents-join-read-ungated) and its mid-run strict-modes refusal writing one `process-exit` line, the named exception to decision/refused-check-writes-no-line; the view H3 «driven from one console» builds on, its v2-14 reading not re-claimed.

@history_item n=38 marker=2026-11-28-console-actions
**2026-11-28-console-actions** (Stack and Technologies) — added decision/unicode-width-column-widths (provisional): unicode-width measures the console's terminal column widths; F-K's 79-column key line against the 80-column floor is measured in these widths, and the v2-15 actions' gated reading is not re-claimed.
