@history_item n=1 marker=2026-09-27-hooks-to-normalised-events
**2026-09-27-hooks-to-normalised-events** — (`hook.event` served; hook stdin row) the wrapper re-validates `hook.event` (closed hook kinds, object `data`; an invalid event is not appended), hook stdin goes through `take(MAX_FRAME + 1)`, `<event>` maps into the closed `HookEvent`, and the fail-open details are `oversize-stdin` · `malformed-json` · `channel-unreachable`. A StopFailure arm and a `turn-ended` cause must fit this re-validation (contract/channel-frames/hook-event-revalidation) and the closed event mapping.

@history_item n=2 marker=2026-10-04-dialog-answers-by-dialog-id
**2026-10-04-dialog-answers-by-dialog-id** — (the sixth dated gap and its validation) `HookEvent` grew from 7 to 9 (PreToolUse / PermissionRequest), and the security fields map into closed enums with fail-open on a malformed payload. This is the precedent for adding a tenth hook event: the closed set, the fail-open shape and the rows that count the events (decision/hook-unknown-event-silent-exit, decision/hook-security-fields-closed) all move with it.

@history_item n=3 marker=2026-10-19-overseer-switch-to-viola
**2026-10-19-overseer-switch-to-viola** — (question option text; `background_tasks` a count) Stop's `background_tasks` is carried only as a length (null when absent or not an array, plus a drift entry), and more upstream text crossing into `events.ndjson` / `wait` payloads was ruled a boundary widening needing the founder's own answer. This governs `turn-ended`'s data on the API-error path: putting `error_details` or `last_assistant_message` into the event would be the same widening class. The chunk's rule (closed cause, no error text) must keep to decision/stop-background-tasks-count-only and decision/upstream-text-is-content.

@history_item n=4 marker=2026-10-10-self-healing-state
**2026-10-10-self-healing-state** — (own state files on read: three counts) the reader counts `unknown_kinds`, `unknown_fields` and `torn_lines` per read without logging them, and a known-kind line with a key outside its contract is returned and counted once. This is the basis for the scope's premise that adding a `cause` key to `turn-ended` keeps mixed binary versions readable (decision/events-reader-bounded-lines, decision/readers-tolerate-torn-lines).

@history_item n=5 marker=2026-10-04-wait-and-last
**2026-10-04-wait-and-last** — (CLI wait / last; the events reader) the `wait` / `last` params clause and the events reader `read_from` through `take(MAX_FRAME + 1)`. The chunk's "a `wait` on the turn end returns" goes through this reader and the `wait` frames (contract/wrapper-channel-methods/wait, contract/mcp-tools/wait).

@history_item n=6 marker=2026-10-04-the-wheel
**2026-10-04-the-wheel** — (pause and release frames, and their validation) the wheel methods and the driver-originated `release` refusal. The chunk clears the wheel's turn-running state on an API-error end "exactly as a Stop clears it"; the wheel's existing controls (decision/driver-release-refused, the from-trust rule) stay unchanged, and the human pause + release is not to become the remedy.

@history_item n=7 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** — (`viola hook`'s event and dialog paths check the instance before obs init) every hook event path canonicalises `VIOLA_DIR` and runs `check_instance` before the diagnostics level and obs init; a refused or absent instance exits 0 with no config, stdin, line, frame or SessionEnd append. A new StopFailure arm must take the same order (decision/hook-strict-check-first, decision/hook-instance-check-first, decision/hook-strict-modes-first). Trap: a check placed after a log or role-file open narrows a widened home first and so passes it.

@history_item n=8 marker=2026-10-26-server-verification-before-any-frame
**2026-10-26-server-verification-before-any-frame** — (a failed server check: no frame, and what each client reports) a hook event whose server check fails writes no frame and exits 0 with empty streams, its `hook-decision` carrying `server-verify-failed`; a SessionEnd whose check fails is still appended through `append_session_end`. The StopFailure hook's `hook.event` sits under the same client checks (decision/hook-refusals-write-no-frame, contract/hook-decision-detail/server-verify-failed), and whether an API-error turn end, like SessionEnd, needs a direct append on a refused check is a choice this chunk makes against that precedent.

@history_item n=9 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** — (a refused socket directory at a hook; the client's peer euid check and the order at a seam) a hook on a refused socket directory exits 0 with empty streams, no frame, and one `hook-decision` of `socket-dir-refused` at WARN. The order on every client seam is directory check, connect, peer euid, server verification. Both bind the new hook arm's `hook.event` (contract/socket-dir-refusal/hook, decision/client-seam-check-order).

@history_item n=10 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** — (the snapshot readers move onto `read_snapshot_or_replay`) over a replayed snapshot hook `connect` writes no frame and fails open as `channel-unreachable`, with a SessionEnd appended directly. The API-error turn-end hook inherits this fallback (decision/replayed-snapshot-fallbacks, decision/replayed-snapshot-yields-nothing).

@history_item n=11 marker=2026-11-14-status-line-interception-removed
**2026-11-14-status-line-interception-removed** — (Input Validation 1 of 2; Error Handling; Threat Model Summary; Bootstrap phases) these entries set the current wording of rows this chunk extends. decision/hook-unknown-event-silent-exit is a "closed set of nine events and no other word", which a StopFailure event makes ten. decision/hook-instance-check-first, decision/max-frame-shared-cap, decision/hook-silent-exit-zero and decision/hook-strict-modes-first leave one capture arm. decision/replayed-snapshot-fallbacks / -yields-nothing drop the statusline command. decision/user-content-only-to-diagnostics now reads «events.ndjson, wait and last results and hook.dialog carry content by design», which decides where the API-error text may go (the detail tier question). decision/code-bearing-files-rewritten-each-start and decision/absolute-viola-path-in-commands now cover the plugin files and hooks.json alone, and the new hook registration in `plugin/hooks/hooks.json` is one of those files.

@history_item n=12 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** — (exit 21 names its cause in detail) `viola_core::UnreachableCause` is a closed cause set in viola-core with no serde path. Each writer writes every word, and a cause word holds no path, pid or upstream text. This is the nearest precedent for the chunk's closed turn-end cause set in viola-core with a fixed fallback member. The `cross_exit_causes` table (`every_code_is_a_code_word`) is a precedent for its witness.

@history_item n=13 marker=2026-11-14-hyprland-seam
**2026-11-14-hyprland-seam** — (Threat Model Summary; Input Validation; Data Protection) decision/child-output-tolerant(-unknown) now reads «a failure becomes unknown or a closed failure word». decision/shared-validators-in-core gained its one named exception (a seam-crate validator). decision/parser-fuzz-scope lists the hook stdin parser among the fuzzed surfaces. decision/hyprland-reply-text-dropped is the precedent for dropping upstream text at the parse and keeping none of it. The CLI's `error` string mapping into a closed member, with the error text dropped at the parse in viola-agent-claude, follows these.

@history_item n=14 marker=2026-09-28-hook-perf-gate
**2026-09-28-hook-perf-gate** — (test seam `FAKE_AGENT_HOOK_PANIC`) a second fake-agent-only env seam needed the founder's live ratification and its own Decisions Log entry, and the ban says another seam needs its own entry. If the fake agent's ability to emit an API-error end needs a new env or flag seam, it is a boundary widening (decision/new-env-read-needs-log-entry, decision/test-seams-fake-agent-only, decision/no-control-switch-off).

@history_item n=15 marker=2026-09-26-local-linux-pre-push-gate
**2026-09-26-local-linux-pre-push-gate** — (`FAKE_AGENT_PUMP_DELAY_MS` carve-out) the first fake-agent test seam: `fake-agent`-only, compiled out of release builds, configures nothing, disables no control. It sets the bounds any fake-agent change for the StopFailure path must stay within (decision/test-seams-compiled-out).

@history_item n=16 marker=2026-09-27-epoch-2-cleanup
**2026-09-27-epoch-2-cleanup** — (test-only feature proven absent) release-check refuses `test-support` / `fake-agent` artifacts (probe 5/5). Fake-agent emission code for the API-error end must stay behind the feature so the release build still passes (decision/release-build-no-test-binaries, decision/release-build-without-test-features).

@history_item n=17 marker=2026-09-28-capability-ledger-and-viola-verify
**2026-09-28-capability-ledger-and-viola-verify** — (the hidden `hook --capture` arm, verify's probe captures and the fixture scrub) the capture arm writes raw payloads 0600 to an argv-named dir, probe captures are removed by a drop guard, and `verify --record` scrubs and refuses unclean recordings. A StopFailure ledger row with a verify probe or a recorded fixture would go through this arm and scrub (decision/fixture-hygiene, decision/probe-artifacts-cleanup).

@history_item n=18 marker=2026-09-28-capability-ledger-and-viola-verify
**2026-09-28-capability-ledger-and-viola-verify** — (`MAX_FRAME` consumers extended, verify's CI split) the real CLI runs only locally, and `verify` runs in CI only against the fake agent, its real-CLI probe and `--record` only locally. This binds the chunk's fake-agent-first test and any new ledger probe (decision/real-cli-local-only, decision/ci-no-secrets-no-real-claude).

@history_item n=19 marker=2026-10-05-real-cli-verify-probes
**2026-10-05-real-cli-verify-probes** — (verify's typed-input PTY probe) each widening of verify's interactive child set (runs, pastes, dirs, residual) was the founder's live ruling after it was shown. Verify never types a byte into a dialog and kills a modal start with no key. A probe for a StopFailure ledger row would widen this set, and the scope's "how the probe could be measured without forcing an API error" fork falls under it (decision/verify-children-direct-spawn, decision/verify-writes-no-dialog-byte, decision/verify-signature-only).

@history_item n=20 marker=2026-10-05-dialog-rows-and-re-probe
**2026-10-05-dialog-rows-and-re-probe** — (the capture arm answers, Runs C and D) Runs C and D joined the verify child set with probe effects kept in their own dirs, and the stamp row count rose. Precedent for adding a hook-observed ledger row with its probe captures (decision/verify-probes-confined-to-own-dir, decision/verify-probe-dirs-cleanup).

@history_item n=21 marker=2026-10-18-compact-post-condition-ledger-row
**2026-10-18-compact-post-condition-ledger-row** — (Run B's sixth paste and the twenty-four-row stamp) the latest stamp-count change: a new ledger row moves the relied set and the `cli_verified` counter, and older stamps read unverified. A new StopFailure ledger row makes the same stamp-gate change.

@history_item n=22 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** — (the statusline shell-out, the settings override and the relied set hold on every build) `LedgerRow::relied()` became all rows on every build, counter `/24`. Any ledger row this chunk adds extends that relied set and its counter.

@history_item n=23 marker=2026-10-07-live-rows-and-paste-shapes-on-the-dev-host
**2026-10-07-live-rows-and-paste-shapes-on-the-dev-host** — (two scratch measurements across the credential boundary) the standing rule: a live or scratch measurement answer is spent once used, and another needs its own showing. Any live `claude` start this chunk asks for to read StopFailure needs its own founder answer before the card.

@history_item n=24 marker=2026-10-19-newer-claude-cli-rows
**2026-10-19-newer-claude-cli-rows** — (the scratch footer reading of a newer CLI accepted) a one-start scratch reading was a boundary widening ratified for that reading only, with no standing permission. This is the precedent for naming a live StopFailure reading as a counted start with what it reads.

@history_item n=25 marker=2026-11-15-live-round-on-the-trimmed-product
**2026-11-15-live-round-on-the-trimmed-product** — (Authentication & Authorization; Data Protection) decision/live-round-rig-keys-own-pty (the rig keys only into its own pty, a word per chunk) and decision/live-session-screens-kept-local bind any live start this chunk is granted.

@history_item n=26 marker=2026-11-14-budget-gate-removed
**2026-11-14-budget-gate-removed** — (Data Protection: decision/viola-format-fixtures-hygiene) committed viola-format fixtures fall under the hygiene walk. The chunk's synthetic StopFailure payloads and fixtures, committed for the fake-agent test, must pass it (decision/probe-prompts-synthetic, decision/fixture-hygiene).

@history_item n=27 marker=2026-09-24-observability-gates
**2026-09-24-observability-gates** — (CI runs the obs artifact canary scan before uploads) the test-home canary scan checks against the NEVER-log floor. It is a standing witness that the API-error text the chunk plants in a test reaches no uploaded log, alongside the plan's own absence probe (decision/stderr-never-log-floor).
