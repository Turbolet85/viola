@history_item n=1 marker=2026-11-24-viola-adopt
**2026-11-24-viola-adopt** (Authentication & Authorization, 3 rows) — added decision/adopt-signals-no-foreign-process (PROVISIONAL on the delegate's word: adopt resumes only a session no live claude holds, spawns its own child in its own terminal, never signals a process it did not start) and widened decision/start-checks-home-tree-before-log-opens and decision/start-checks-socket-dir-before-bind to adopt's start. This is the row set the chunk extends: any early-exit handling or a changed spawn folder must keep adopt off foreign processes and keep both checks ahead of the log, bind and child.

@history_item n=2 marker=2026-11-24-viola-adopt
**2026-11-24-viola-adopt** (Authentication & Authorization, 2 rows) — added contract/strict-modes-refusal/adopt and decision/r5-adopt-script-ends-own-processes (the R5 script ends only the wrapper it recorded, TERM then 10 s then KILL if the start time is unchanged, verdicts in words only). If this chunk changes `scripts/r5-adopt.sh` (the typed-in folder at line 155, its `run_env`), that row binds the change and its `--rehearse` form.

@history_item n=3 marker=2026-11-24-viola-adopt
**2026-11-24-viola-adopt** (Data Protection and Threat Model Summary, the trust-record sets) — decision/start-writes-trust-record-flag-only, decision/no-writes-under-claude-home, decision/viola-writes-nothing-in-claude-dir and decision/nothing-written-under-claude-home now name adopt's start, which writes a persisting trust record "for the caller's current folder" (PROVISIONAL, delegate overseer Q2). Moving adopt's spawn folder away from the caller's cwd moves the key of that persisting record in the user's Claude config. This is a boundary question the plan must surface, not take as routine.

@history_item n=4 marker=2026-11-24-viola-adopt
**2026-11-24-viola-adopt** (Secret Management, 4 rows) — decision/credential-strip-by-name-only, decision/env-not-config-channel, decision/no-stored-secrets and decision/r8-claude-env-strip carve out adopt's start reading CLAUDE_CONFIG_DIR's value (only when kept) to choose the trust record's config (PROVISIONAL). Any new read this chunk adds to find the session's folder, such as the Claude projects store, has no such carve-out. It would need its own row and ruling.

@history_item n=5 marker=2026-11-24-viola-adopt
**2026-11-24-viola-adopt** (Input Validation: session-id-argv-closed-shape, claude-agents-resolved-from-path, launch-flags-checked) — the operator-typed id crosses onto the child's argv in the closed shape through the one resume-args builder (PROVISIONAL, fork W2). Adopt's live-elsewhere check spawns `claude agents --json` from the caller's PATH and cwd. Adopt takes --model/--effort through run's parsers. A spawn-folder change must not route the id or pair around these checks. The agents read's cwd is part of the same resolution.

@history_item n=6 marker=2026-11-24-viola-adopt
**2026-11-24-viola-adopt** (Threat Model Summary, Error Handling, Data Protection: state-tree-strict-modes-before-use, home-tree-strict-modes-before-log, run-refuses-socket-dir-before-start, cli-json-error/internal-error, exec-commands-absolute-pinned-path) — adopt's start joined run's and revive's ordered checks, its socket-dir-refused code and its silent internal-error catch site, and it runs run's own start. A new early-exit word, exit and line must sit beside these: adopt's start stays silent at the catch site, and a refusal opens no log.

@history_item n=7 marker=2026-10-10-viola-revive
**2026-10-10-viola-revive** (the revived child: the recorded cwd ratified) — the founder ratified, live, a recorded directory as a spawn directory as a boundary widening. revive spawns in the snapshot's `cwd`, read only after the strict-modes check, used as the spawn directory alone, and refused `cwd-missing` when absent. This is the direct precedent for choosing adopt's resume folder: a folder taken from any recorded source is a widening that needs its own showing. Adopt has no snapshot record, which the scope asks the phase to measure.

@history_item n=8 marker=2026-10-10-viola-revive
**2026-10-10-viola-revive** (revive reads its own state) — revive became a strict-modes entry point (`check_instance` before any value is used, refusal `strict-modes-failed`, no child). The classified read and replay became its preflight. This is the pattern any recorded-folder read for adopt must follow: check first, then read.

@history_item n=9 marker=2026-11-18-session-folder-trusted-by-viola
**2026-11-18-session-folder-trusted-by-viola** (Input Validation, Threat Model Summary, Data Protection) — revive-recorded-cwd-spawn-dir and revive-cwd-spawn-only now let the recorded cwd key the start's trust record too. cwd-only-in-snapshot and spawn-cwd-snapshot-only allow the cwd into the Claude config as that key. start-trust-skips-trusted-folders and start-writes-trust-record-flag-only were added, all ratified by the founder at 2026-11-22. A new adopt spawn folder flows into that same trust step. The no-log, no-error-body, no-stdout rule binds any folder the new early-exit hint or line might name.

@history_item n=10 marker=2026-11-24-viola-up
**2026-11-24-viola-up** (Threat Model Summary, Input Validation) — cwd-only-in-snapshot and spawn-cwd-snapshot-only widened so the recorded cwd may also stand as the working directory on viola up's terminal argv. That landed PROVISIONAL because it was "a boundary widening of its own" and was asked as a fork of its own. open-terminal-direct-spawn now has up start `revive` "in the recorded cwd". This is the latest precedent: each new place a session folder reaches was ratified separately, and adopt's terminal or child cwd would be one more.

@history_item n=11 marker=2026-11-23-viola-open-2
**2026-11-23-viola-open-2** (Threat Model Summary, the cwd rows discharged; Input Validation: open-terminal-direct-spawn) — the operator's W1 answer discharged the provisional marks on cwd-only-in-snapshot and spawn-cwd-snapshot-only. open-terminal-direct-spawn (provisional) fixes that a terminal spawn is direct, with a closed argv of one element per value. Adopt "spawns its own child in its own terminal". A changed folder must ride that closed argv, never a shell `cd`.

@history_item n=12 marker=2026-10-17-cli-machine-contract
**2026-10-17-cli-machine-contract** (security: revive's --json documents) — revive's refusals print exactly `{"v":1,"error":"<code>","detail":null}` with closed codes, `cwd-missing` among them, never a path or pid. This is the shape and closedness an adopt early-exit word under --json must match. contract/revive-json/cwd-missing is the model for a "no folder / no session here" refusal.

@history_item n=13 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** (revive --json documents every refusal of its start; the exit-cause table; a stale holder's code) — closed exit-1 codes grew by named carve-outs of the founder's word. `tests/cross_exit_causes.rs` holds the table with `every_code_is_a_code_word` and `no_two_codes_share_a_hint`. collision_check reads `stale-heartbeat` against `already-live`. A new early-exit closed word and hint must join that table with a unique hint. Releasing or keeping the name after an early exit is judged by the collision check's pid, start-time and heartbeat rule.

@history_item n=14 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** (the error bodies' census) — `tests/contract_error_bodies.rs` names every error body with its writer and witness. `product_error_writers_are_each_named` fails on an unnamed writer, and `thiserror_messages_interpolate_nothing` fails on interpolation. A new early-exit refusal or hint is a new writer the census must name, with no path or upstream text (decision/every-error-writer-named, decision/error-messages-interpolate-nothing).

@history_item n=15 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (the server check's reference values and the collision check read through the replay; the home tree checked at run and revive's start) — revive's preflight reads the snapshot once and hands that read to both its collision check and its recorded-directory reading. `check_home_tree` runs before the wrapper log, and a refusal leaves no log, line or child. This is the order a folder lookup for adopt must sit in, and the trap that a log or create before the check narrows the home.

@history_item n=16 marker=2026-10-10-budget-governor
**2026-10-10-budget-governor** (the two instance checks ahead of any log) — revive's check was moved before the wrapper log because a log opened before or after a refused check narrowed the home to 0700 and erased the refused state. This is the root of decision/strict-refusal-leaves-no-trace and decision/strict-check-before-narrowing, which bind where any new early-exit recording may open a log.

@history_item n=17 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** (a refused socket directory at a start; a killed wrapper's leftover socket file) — `socket_dir_ready` checks, creates only when absent and checks again before any snapshot, child or bind. A killed wrapper's socket file stays until a next bind of that name takes it over, and nothing removes it. Whether the name is released after a child ends at once is bounded by these rows: no new removal of the leftover socket file.

@history_item n=18 marker=2026-10-14-resume-and-compact-ledger-rows
**2026-10-14-resume-and-compact-ledger-rows** (Run E in the verify child set; a third session-id source) — verify's Run E resumes Run B's session with `--resume <id>` from `resume_args` in its own `<cwd>/.viola-verify-<pid>-resume/` folder, a folder other than Run B's, under the `resume-session` ledger row. This is the ledger evidence the scope's measurement on "a resume from another folder" must read before calling it UNMEASURED.

@history_item n=19 marker=2026-11-16-local-command-delivery
**2026-11-16-local-command-delivery** (Input Validation and Data Protection: transcript-path-shape-check, transcript-read-bounded, transcript-path-redacted; ratified 2026-11-17) — the only standing read of a Claude transcript is shape-checked, opened no-follow, owned by euid, capped at MAX_FRAME and count-only. The transcript path is redacted from every log and reply. Telling "no such session here" apart by looking in the Claude projects store would be a new read of that class, held to these rows or widened by ruling.

@history_item n=20 marker=2026-10-16-unwrapped-sessions-on-the-board
**2026-10-16-unwrapped-sessions-on-the-board** — added the `claude agents --json` child of revive's preflight: a live id elsewhere is the closed refusal `session-live`, and a failed read passes. Rows hold no row text, id, cwd or path. Adopt's live-elsewhere check reuses this body unchanged, so any cwd the agents rows carry must not leak into adopt's new refusal or line.

@history_item n=21 marker=2026-11-23-model-and-effort-at-launch
**2026-11-23-model-and-effort-at-launch** (Input Validation: launch-pair-argv-crossing, launch-flags-checked) — the launch pair crosses the claude child's argv only as four direct-spawn elements after the plugin-dir flag. That is a ratified crossing, and revive's recorded pair is a second one. The argv order a resumed adopt child takes, `--resume <id>` included, is fixed by these rows.

@history_item n=22 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (child program resolution is a given path or a bare name on PATH) — `viola run` resolves its program itself to an absolute path, and a program not found is `ProgramNotFound` (`internal-error` under revive --json). decision/run-resolves-absolute-program and claude-resolved-from-path rest on this. A changed spawn cwd must not change program resolution, which never reads the cwd.

@history_item n=23 marker=2026-10-19-newer-claude-cli-rows
**2026-10-19-newer-claude-cli-rows** (the scratch footer reading of a newer CLI accepted) — one scratch `viola run` start of the user's CLI was a boundary widening the founder answered himself, "no product path and no standing permission: another reading needs its own answer". This is the precedent for the scope's owed live measurement telling hypothesis A from B: it is a new crossing to be priced and asked, never fired by this phase.

@history_item n=24 marker=2026-10-07-live-rows-and-paste-shapes-on-the-dev-host
**2026-10-07-live-rows-and-paste-shapes-on-the-dev-host** (two scratch measurements across the credential boundary) — standing rule: "the answers are spent; another peer message or another names probe needs its own showing". This backs the scope's boundary that any live start or resume against the operator's real `~/.claude*` is asked through the overseer, priced, and never fired unasked.

@history_item n=25 marker=2026-10-05-real-cli-verify-probes
**2026-10-05-real-cli-verify-probes** (verify's typed-input PTY probe: residual and dev-host prerequisite) — the accepted risk that real-CLI runs leave transcripts under `~/.claude/projects/` and trust entries, and that the start folder must be trusted. The live-measurement cost the scope asks for (what a start leaves in the operator's real config, and how it is undone) is the same residual class. A cwd-keyed transcript store bears directly on hypothesis A.

@history_item n=26 marker=2026-11-15-live-round-on-the-trimmed-product
**2026-11-15-live-round-on-the-trimmed-product** (Data Protection) — added decision/live-session-screens-kept-local: a live session's screen frames stay under a gitignored local path, never in a run dir or commit, and only closed-word summaries land. This binds any R5a re-fire evidence and any evidence of the two red children this chunk measures.

@history_item n=27 marker=2026-11-17-standing-live-rig
**2026-11-17-standing-live-rig** (Secret Management: live-rig-credentials-copy, live-rig-isolated-child; ratified 2026-11-22) — the live rig's real-claude child runs under a private CLAUDE_CONFIG_DIR with a closed kept environment, and its credentials copy is transient and unrecorded. `scripts/r5-adopt.sh`'s live arm passes only PATH and inherits the caller's real home, which is the axis hypotheses A and B turn on. Any change to that script's environment or folder is measured against these rows.

@history_item n=28 marker=2026-11-18-the-live-home-s-re-stamp-and-the-product-re-pin
**2026-11-18-the-live-home-s-re-stamp-and-the-product-re-pin** (Secret Management) — the VH arm is the one named exception to live-rig-isolated-child and live-rig-credentials-copy. It runs under the real home and config, reads no credentials, prints no value, and needs an explicit home, Claude dir and user config. This is the precedent for any rig arm (R5a) that must run against the operator's real Claude store to resume a real session.

@history_item n=29 marker=2026-11-17-first-readings-on-the-standing-rig
**2026-11-17-first-readings-on-the-standing-rig** (Secret Management, Input Validation) — live-rig-isolated-child's sha256-and-mtime witness was disproved by a host writer of the shared user config and replaced by the needle search. live-rig-prompt-capture-closed was added. This is a trap for any measurement of what a live adopt start leaves in `~/.claude.json`: a stamp moves under other writers.

@history_item n=30 marker=2026-11-19-a-live-stopfailure-payload
**2026-11-19-a-live-stopfailure-payload** (Secret Management, Input Validation) — added live-rig-sf-arm-network-closed and widened live-rig-prompt-capture-closed. One contained live reading settled an exception "for one word on one version, nothing claimed for the others". This is the precedent for how narrowly a single live reading of the early-exit cause may be claimed.

@history_item n=31 marker=2026-11-24-switching-a-session-s-model-and-effort
**2026-11-24-switching-a-session-s-model-and-effort** (Authentication & Authorization) — switch-ends-only-its-own-child: a wrapper that ends its own child and resumes the session in place, PROVISIONAL on fork F1. This is a sibling resume path: it reuses the instance's own folder, and how it reports a resumed child that fails is the nearest built precedent for adopt's early-exit report.
