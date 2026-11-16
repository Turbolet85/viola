@history_item n=1 marker=2026-10-04-wait-and-last
**2026-10-04-wait-and-last** (wait and last as built) — set `wait` to `{after?, timeout_ms?, from?}`, its start offset at `after` or else at the log's end, and the line-starts rule. It added `EventKind::WAIT_WAKE` and the `WaitFeed` wake: one Mutex + Condvar, the hook line appended inside the lock, only hook lines signal, and 20 ms clock re-reads that never re-scan. An `after`-less wait starts at the log end. Why it matters: this is the mechanism this chunk documents and pins. A turn end followed by a notification is decided here, by which kinds are in `WAIT_WAKE` and by the fact that a no-cursor wait sees only later lines.

@history_item n=2 marker=2026-10-04-dialog-answers-by-dialog-id
**2026-10-04-dialog-answers-by-dialog-id** (hook.dialog, answer, the deadline) — made a pending dialog come first in `wait`: an `after`-less wait, or one at or before the dialog's line, returns it at once, through `WaitFeed::appending_dialog` / `dialog_settled`. Why it matters: the turn-end question this chunk must wake on rides this pending-dialog path (contract/channel-wait/pending-dialog, contract/channel-methods/wait-pending-dialog).

@history_item n=3 marker=2026-10-19-dialog-hold-for-the-driver
**2026-10-19-dialog-hold-for-the-driver** (the configurable dialog hold and the dialog-expired record) — added `dialog-expired` as the one wrapper-appended kind that wakes `wait`, which made the wake set six (`turn-ended` · `question` · `permission` · `plan` · `session-end` · `dialog-expired`), with "only hook lines signal" now excepting it. It also moved the `pending_dialog` write ahead of the event line. Why it matters: this is the wake set the chunk restates and pins, and the one precedent for a wrapper line that wakes.

@history_item n=4 marker=2026-10-19-overseer-switch-to-viola
**2026-10-19-overseer-switch-to-viola** (`question` gains `option_details`, `turn-ended` gains `background_tasks`) — made `turn-ended` data `{last_assistant_message, background_tasks}`. `background_tasks` is a count only of the Stop payload's array, or `null` when the key is absent, `null` or not an array. The founder answered the count-only form himself. Why it matters: the chunk's case of "a turn end held by a background task" is read through this field. Any wake rule keyed on it is bounded by the count-only, no-element-read form.

@history_item n=5 marker=2026-11-15-api-error-turn-end
**2026-11-15-api-error-turn-end** (Standard Contracts: 3 rows) — gave `turn-ended` data an optional `cause` (`api-error` only, PROVISIONAL), so it is now `{last_assistant_message, background_tasks, cause?}`. It added decision/api-error-turn-end-clears-turn. Why it matters: a StopFailure turn end is a second source of `turn-ended` that wakes `wait`. The pinned wake cases must cover it, and they must not move the provisional `cause` contract, whose founder ratification is still owed.

@history_item n=6 marker=2026-11-15-api-error-turn-end
**2026-11-15-api-error-turn-end** (Conventions: 2 rows; Established Decisions: 3 rows; Cross-cutting Patterns: 1 row; Design Philosophy: 1 row) — added contract/hook-to-kind-map/stopfailure (StopFailure → `turn-ended`) and contract/turn-end-cause/cause-set. It put StopFailure in the sync spine tiers and registered the StopFailure exception ("provisionally, no ledger row and no verify probe") on decision/capability-ledger, decision/cli-capability-ledger and decision/capability-ledger-gate. Why it matters: the hook-to-kind map is where this chunk states which hooks wake. The StopFailure live reading and its ledger exception are out of scope for this chunk, so the plan must leave them as they stand.

@history_item n=7 marker=2026-09-27-hooks-to-normalised-events
**2026-09-27-hooks-to-normalised-events** (seven exec-form hooks, `hook.event` served) — registered Notification, PostToolUse and PostToolUseFailure as `async: true` hooks, and set `hook.event`'s five hook kinds with `activity` among them. Why it matters: a notification reaches the log as `activity`, which is log-only (contract/hook-to-kind-map/notification, contract/hook-tiers/async). That kind is the "no false wake" line this chunk must pin.

@history_item n=8 marker=2026-10-04-confirmed-send-with-cl-1-records
**2026-10-04-confirmed-send-with-cl-1-records** (confirmed send: kinds, refusals, exits) — added the wrapper-appended, log-only kinds `send-issued` · `send-confirmed` · `send-refused`. Why it matters: these are among the kinds the chunk's test must show re-park a wait (decision/wrapper-events-log-only, contract/channel-wait/wake-kinds).

@history_item n=9 marker=2026-10-10-session-links
**2026-10-10-session-links** (the implicit link as landed, the explicit verbs held) — added `link` to the wrapper-appended kinds, log-only, and to the `wait` row's list of wrapper lines that never wake. Why it matters: it is a precedent for how each new log-only kind was added to the never-wake list. A further log-only kind follows the same pattern.

@history_item n=10 marker=2026-10-26-explicit-session-links
**2026-10-26-explicit-session-links** (a product path appends the unlink event) — made a product path append `unlink` and added it to the `wait` records that never wake (the `send-*`, `link`, `unlink` and start records). Why it matters: this is the current never-wake list the chunk's documentation restates (contract/channel-wait/wake-kinds).

@history_item n=11 marker=2026-11-14-budget-gate-removed
**2026-11-14-budget-gate-removed** (Standard Contracts: wrapper-events-log-only, both rows; Conventions: budget-gate kind removed) — removed `budget-gate` from the wrapper-appended log-only kinds and from the normalised event kinds. Every own-format `v` stayed at 1. Why it matters: the log-only list the chunk pins no longer holds `budget-gate`. An older log's `budget-gate` line is now an unknown kind that the reader skips, and it must not wake.

@history_item n=12 marker=2026-11-15-live-round-on-the-trimmed-product
**2026-11-15-live-round-on-the-trimmed-product** (Established Decisions: 4 rows) — moved the dialog hold and its bound to standing. It also said that at the hold's end the expiry ends only the hook's wait while the CLI's dialog stays drawn (contract/dialog-hold-end/null-reply). Why it matters: a woken `wait` on `dialog-expired` does not mean the dialog is gone. This is relevant to how the chunk documents what each wake means.

@history_item n=13 marker=2026-10-07-live-rows-and-paste-shapes-on-the-dev-host
**2026-10-07-live-rows-and-paste-shapes-on-the-dev-host** (the paste hint measured on the live CLI) — measured that a no-cursor `viola wait` issued in the hint window woke on nothing and ran to its deadline, while `wait --after` the earlier cursor returned at once. Why it matters: this is a measured precedent for the no-cursor vs cursor behaviour. The overseer's waiter re-arm (the scope's `[inferred]` item) turns on the same start-offset rule (contract/channel-wait/after, contract/channel-wait/start-offset).

@history_item n=14 marker=2026-10-04-running-turn-refusal
**2026-10-04-running-turn-refusal** (the running-turn state as built) — keyed the running-turn state on kind alone: `prompt-submitted` starts it, `turn-ended` / `session-start` / `session-end` end it, and `activity` leaves it. Why it matters: it is the existing kind-keyed rule beside `WAIT_WAKE` that treats `activity` as inert. A wake rule that tells a held turn end apart must not contradict it.

@history_item n=15 marker=2026-10-17-mcp-server-for-drivers
**2026-10-17-mcp-server-for-drivers** (the MCP tools' contract as landed) — made MCP `wait` send `timeout_ms` = `min(given, 50 000)` (`WAIT_CAP`), 50 000 ms by default, with `after` passed as given. Each handler re-validates its raw arguments. Why it matters: the MCP `wait` surface the chunk pins carries this cap (decision/mcp-wait-cap). A parked MCP wait returns `{timed_out:true}` at 50 s regardless of wake kinds.

@history_item n=16 marker=2026-10-17-mcp-server-for-drivers
**2026-10-17-mcp-server-for-drivers** (the Tokio client and viola-channel's tree-form ban) — put MCP `wait` on `viola-channel`'s feature-gated `TokioClient`, while CLI `wait` stays a blocking read on the sync client. Why it matters: the chunk's `crates/viola-mcp/src/tools.rs` surface and decision/tokio-containment / decision/hybrid-concurrency bound where any wait change may live.

@history_item n=17 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** (the exit-21 document carries its cause in `detail`) — made a `wait` whose endpoint vanishes read `instance-unreachable` with `detail` `endpoint-lost` (contract/channel-wait/timeout-ms). Why it matters: a blocked wait that ends with no event is part of the wake contract the chunk documents.

@history_item n=18 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** (a client's refused socket directory; the peer euid on both sides) — fixed the frame-writing clients' order: the strict-modes check, the liveness pre-check, `check_endpoint_dir`, the peer euid, then the server identity check. CLI `wait` and MCP `wait` both run it. Why it matters: decision/frame-client-check-order binds any change to `src/cmd/wait.rs` or the MCP `wait` tool. No check may be reordered.

@history_item n=19 marker=2026-10-26-server-verification-before-any-frame
**2026-10-26-server-verification-before-any-frame** (the snapshot's pid and started_at as the server identity) — made the `wait` client, CLI and MCP, compare the peer pid and OS start time with the snapshot's before its first frame. Why it matters: these are the rows decision/verify-server-before-first-frame and decision/server-identity-check-before-frame. Tests that drive `wait` go through this check.

@history_item n=20 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (the instance check first at every CLI frame verb and MCP frame tool) — made `wait` (CLI through `client::refuse_untrusted`, MCP in `frame`) run `check_instance` before any snapshot read. A refusal is exit 21 with no line. Why it matters: decision/instance-check-before-any-read, decision/client-strict-modes-before-frame and decision/mcp-frame-tools-strict-modes-check all bind the wait surfaces this chunk touches.

@history_item n=21 marker=2026-10-10-self-healing-state
**2026-10-10-self-healing-state** (the next append heals a torn last line; the reader counts three names) — made the next append heal a torn last line. `events::read_from` counts `unknown_kinds`, `unknown_fields` and `torn_lines`, and does not return an unknown kind. Why it matters: the wait feed reads `events.ndjson`. An unknown or torn line must neither wake nor move a cursor wrongly (decision/append-heals-torn-line, decision/reader-counts-skipped-lines, decision/events-log-never-truncated).

@history_item n=22 marker=2026-10-10-session-links
**2026-10-10-session-links** (the link set in the replay, the snapshot and the start) — fixed that a start reads `events.ndjson` twice before the endpoint serves (`WaitFeed::rebuild`, then `LinkSet::restore`). Why it matters: `WaitFeed::rebuild` is the start-time step that seeds `last` (contract/channel-last/last-assistant-message) and the feed. A change to the feed's state must keep that rebuild order.

@history_item n=23 marker=2026-10-05-permission-end-to-end
**2026-10-05-permission-end-to-end** (the question first raised by PermissionRequest stays null) — logged a `question` first raised by PermissionRequest once and answered it `null` at once. A later answer is `unknown-dialog`. Why it matters: such a question still appends a `question` line that wakes `wait` (decision/permission-request-question-stays-null). That is a wake case beside the turn-end question.

@history_item n=24 marker=2026-10-16-unwrapped-sessions-on-the-board
**2026-10-16-unwrapped-sessions-on-the-board** (the fake agent's agents mode) — added the `agents` mode word and files beside the fake agent's executable. This was the founder's own answer that there be no third fake-agent env seam. Why it matters: the chunk extends the fake agent's turn scripts (`src/bin/viola-fake-agent.rs`, `fixtures/fake-scripts/`). A new knob takes the argv, script or mode-word route, never a new env seam (decision/fake-agent-test-seams).

@history_item n=25 marker=2026-09-28-hook-perf-gate
**2026-09-28-hook-perf-gate** (the second fake-agent test seam `FAKE_AGENT_HOOK_PANIC`) — ratified a second fake-agent env seam as a boundary widening. The founder ruled it live. Why it matters: it is a precedent that any further fake-agent env seam is a widening needing the founder's word (decision/fake-agent-test-seams, decision/viola-env-namespace-product-only).

@history_item n=26 marker=2026-10-19-newer-claude-cli-rows
**2026-10-19-newer-claude-cli-rows** (the 2.1.302 fixture set and the long-turn screen switch) — counted the fake agent's argv options at thirteen (the count's rule is architecture's own list) and added `--long-turn-screen`. Why it matters: a turn-script option added for a background-task or notification case moves this registered count. The count must be updated by its rule.

@history_item n=27 marker=2026-09-29-verify-stamped-test-homes-and-harness
**2026-09-29-verify-stamped-test-homes-and-harness** (verify's spawn pairs, the harness CI read, verify-stamped homes) — made root rstest homes stamped by `stamped_home` through `viola verify` against the fake agent. Why it matters: this is the fixture behind the carried CI red. `path1_run_start_one_home_in_order`'s extra time sits in the unbounded `viola verify` at `tests/support/verify.rs`. Option (a), stamping once and sharing, changes this design.

@history_item n=28 marker=2026-10-14-resume-and-compact-ledger-rows
**2026-10-14-resume-and-compact-ledger-rows** (the resume row, Run E and /compact listed) — made verify drive five interactive runs, so a verify is six `claude` starts. Why it matters: it explains why the `stamped_home` fixture's verify costs ~5 s even under the fake agent. That cost grows with load, and it is the time the chunk must take out of the measured window or size the `ci` kill to.

@history_item n=29 marker=2026-10-07-test-homes-off-the-contended-volume
**2026-10-07-test-homes-off-the-contended-volume** (the test homes' base, its keepers and the dev host's tmpfs backing) — made the dev host's `target/e2e-home` a tmpfs link. It also set the standing rule that "a stalled-start red is a finding about the backing, reported and never re-run for green". Why it matters: this is the rule for the carried `path1_run_start_one_home_in_order` timeout. The chunk's measured choice between (a) and (b) must not re-run for green, and any shared stamped home lives under these keepers.

@history_item n=30 marker=2026-10-20-epoch-5-cleanup
**2026-10-20-epoch-5-cleanup** (the keepers' headroom floor and the census's second form) — added the `HEADROOM_FLOOR_MIB` (9 GiB) refusal after the stamped-home gates hit EDQUOT on the dev host's tmpfs. Why it matters: stamped homes are a measured disk and time cost. Sharing or copying one stamped home (option a) interacts with this floor and with the keepers' isolation.

@history_item n=31 marker=2026-10-02-epoch-2b-cleanup
**2026-10-02-epoch-2b-cleanup** (test homes removed with their owner record last) — routed both removals of a test home through `remove_owned`, which deletes `owner.json` last. Why it matters: a home stamped once and shared or copied across tests must keep the owner-record sweep rule (decision/test-home-owner-sweep, decision/test-homes-cleaned-by-owner).

@history_item n=32 marker=2026-11-15-mutation-and-coverage-baselines-green
**2026-11-15-mutation-and-coverage-baselines-green** (Occupied Resources: 2 rows) — raised the shared root `WITHIN` bound from 7 s to 12 s (decision/root-waits-share-one-bound) and moved test channel sockets under a short `/tmp` root. Why it matters: the chunk's new wait tests share this one bound. Sizing the `ci` kill or a wait is the same kind of change, made the chunk before.

@history_item n=33 marker=2026-10-09-epoch-3-cleanup-ii
**2026-10-09-epoch-3-cleanup-ii** (the Watch bound's waits counted by pattern) — made the count's rule the `Instant::now() + WITHIN` pattern rather than a census. It recorded the trap that a wait written through `EXIT_WITHIN` or a qualified path is outside the pattern. Why it matters: new root wait cases this chunk adds move the registered count, which must be re-measured by its rule.

@history_item n=34 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (the root `WITHIN` waits: 31 sites in 23 files; CI on the one `ubuntu-latest` leg) — set the last registered count of root waits on the bound and made CI `ubuntu-latest` only, with `test` running coverage through `scripts/agent-run.sh`. Why it matters: it is the count base the chunk's new waits move, and the one CI leg where the carried timeout reds (decision/ci-linux-fake-agent, decision/coverage-floors-gate).

@history_item n=35 marker=2026-10-16-remove-the-browser-leg
**2026-10-16-remove-the-browser-leg** (Infrastructure Patterns) — made `.config/nextest.toml` hold the `ci` and `mutants` profiles. Why it matters: the `ci` profile's 20 s kill that the carried red hits lives in this file, and option (b) edits it.

@history_item n=36 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** (the sanitised error surfaces pattern and its contract binary) — put two root binaries, which require `fake-agent`, in the `ci` profile's kill-line filter. Why it matters: it is a precedent for sizing or filtering a test class under the `ci` profile's kill, the mechanism option (b) uses.

@history_item n=37 marker=2026-09-28-mutation-testing-to-the-epoch-boundary
**2026-09-28-mutation-testing-to-the-epoch-boundary** (mutation leaves CI and the pre-push) — removed mutation testing from CI and the pre-push, under the founder's ruling. Mutation runs only at the epoch-boundary audit. Why it matters: the scope's "no mutation run" boundary rests on this. The new guard tests carry their controls through `gate.py control` instead (decision/mutation-at-epoch-boundary).
