@history_item n=1 marker=2026-10-19-overseer-switch-to-viola
**2026-10-19-overseer-switch-to-viola** — (`question` gains `option_details`, `turn-ended` gains `background_tasks`) made `turn-ended` data `{last_assistant_message, background_tasks}`, with `background_tasks` the Stop payload's array length only, no element read or carried, and `null` when the key is absent, `null` or not an array. The founder answered the count-only form himself (P4 fork Q4). This is the direct precedent the scope tells the new `session_crons` count to copy: the same count-only parse, the same `null` rule, and the same hops (events.ndjson, CLI `wait --json`, MCP `wait`).

@history_item n=2 marker=2026-11-16-wait-wake-condition
**2026-11-16-wait-wake-condition** — rewrote `contract/channel-methods/wait-wake-kinds` so that `wait` wakes by event kind alone and every `turn-ended` wakes whatever its `background_tasks`. It also changed `contract/event-data/turn-ended-last-assistant-message` to say `turn-ended` "carries no scheduled-wake count", leaving the count as a founder candidate. This chunk must rewrite that absence clause, and it must keep the kind-only wake rule unchanged (the scope boundary "wait's wake rule unchanged").

@history_item n=3 marker=2026-11-15-api-error-turn-end
**2026-11-15-api-error-turn-end** — (Standard Contracts) made `turn-ended` data `{last_assistant_message, background_tasks, cause?}`, adding `contract/event-data/turn-ended-cause` as an optional key that crosses into `events.ndjson`, CLI `wait --json` and MCP `wait`. It also added `contract/hook-to-kind-map/stopfailure` and `name/other/hook-stop-failure`. This is the most recent additive change to the same data row and the shape that row now holds. The chunk extends it, and must decide what a StopFailure-produced `turn-ended` carries for the new count.

@history_item n=4 marker=2026-11-16-granular-api-error-cause
**2026-11-16-granular-api-error-cause** — rewrote `contract/hook-to-kind-map/stopfailure`: the StopFailure arm's `turn-ended` data is "exactly `last_assistant_message` null, `background_tasks` null and `cause`" (one of nine TurnEndCause words). It also changed `contract/event-data/turn-ended-cause` and the `decision/unparseable-external-values-unknown` exception. Trap: this row enumerates the StopFailure data exactly, so adding a scheduled-wake count to `turn-ended` means amending this row too (for example, null on that arm), or it goes false.

@history_item n=5 marker=2026-11-16T06-05-33-wrap
**2026-11-16T06-05-33-wrap** — ratified, on the founder's own word, the StopFailure exception in `decision/capability-ledger`, `decision/capability-ledger-gate` and `decision/cli-capability-ledger`: registered with no ledger row and no verify probe. Its live reading is owned by «The 2.1.325 recorded fixture set». It also ratified `decision/submitted-form-prompt-match`. This is the standing precedent for the scope's open premise of whether an undocumented Stop field needs a ledger row, and it names who owns the 2.1.325 recorded reading that the scope's premise about `session_crons` on 2.1.325 rests on.

@history_item n=6 marker=2026-10-07-live-rows-and-paste-shapes-on-the-dev-host
**2026-10-07-live-rows-and-paste-shapes-on-the-dev-host** — (the long-paste frame) recorded the founder's ruled limit, now carried by `decision/no-probe-for-unrelied-shapes` and the `decision/capability-ledger-gate` exception: "a CLI shape no send relies on needs no probe". This is the rule P3 weighs when it decides whether a read-only telemetry count that falls to `null` needs a ledger row.

@history_item n=7 marker=2026-10-10-statusline-pass-through
**2026-10-10-statusline-pass-through** — (the capability ledger: three relied-on dependences with no row) kept the rule that any new dependence on an undocumented `claude` behaviour is a ledger row with a probe, and it named each unrowed reliance as an exception with an owner, never as a silent gap. This is the precedent the chunk follows if it relies on `session_crons` without a row.

@history_item n=8 marker=2026-10-10-self-healing-state
**2026-10-10-self-healing-state** — made `events::read_from` count `unknown_kinds`, `unknown_fields` and `torn_lines`. A known-kind line with a key outside its kind's `data` list is returned and counted as an unknown field (row `decision/reader-counts-skipped-lines`). Trap: the new `turn-ended` data key must be added to the kind's known `data` list, or every new turn end is counted as an unknown field. `viola list` surfaces those counts (2026-10-11-the-board-viola-list).

@history_item n=9 marker=2026-09-27-hooks-to-normalised-events
**2026-09-27-hooks-to-normalised-events** — served `hook.event`: the wrapper re-validates the event (the hook kinds, object `data`) before it appends it with `source:"hook"`. This is the hop where the new count crosses from the hook process into the wrapper (rows `contract/channel-methods/hook-event` and `contract/hook-event/appended-line`), so re-validation must admit it.

@history_item n=10 marker=2026-10-04-wait-and-last
**2026-10-04-wait-and-last** — built `wait` `{after?, timeout_ms?, from?}` with the `EventKind::WAIT_WAKE` set and the `WaitFeed` wake, in which only hook lines signal. It also built `last`, rebuilt from the newest `turn-ended` under the append's lock. These are the `wait` and `last` hops the count reaches (rows `contract/channel-methods/wait`, `contract/channel-methods/last`, `contract/channel-methods/last-after-seen-line`). The wake set is the thing this chunk must not change.

@history_item n=11 marker=2026-10-19-dialog-hold-for-the-driver
**2026-10-19-dialog-hold-for-the-driver** — extended the wake set to its current six (adding `dialog-expired`) and changed "only hook lines signal". This is the last change to `contract/channel-wait/driver-relevant-events` and `contract/channel-methods/wait-wake-kinds`, which the chunk must leave as they stand.

@history_item n=12 marker=2026-11-14-status-line-interception-removed
**2026-11-14-status-line-interception-removed** — (Conventions) widened the `decision/protocol-version-bump-rule` exception (removals kept every own-format `v` at 1). The rule itself stands: adding a field is additive and needs no `v` bump. The founder ratified the widened exception at 2026-11-15T18-01-44-wrap. The new count is such an additive field, and this is the row's current text.

@history_item n=13 marker=2026-11-14-budget-gate-removed
**2026-11-14-budget-gate-removed** — (Cross-cutting Patterns) changed `decision/mixed-version-tolerance` to say readers of own formats skip unknown kinds and fields and surface the counts, so an older writer's state reads whole. This bears on the scope boundary that an older reader must not break on the added field.

@history_item n=14 marker=2026-09-29-fake-agent-drift-contract
**2026-09-29-fake-agent-drift-contract** — widened the harness prompt set to four compiled prefixes, cross-session forms included, with the founder ratifying the side effect that a human who types such a tag is filed `harness`. This is the precedent for the directive's adjacent fact: a CLI-scheduled wake prompt is logged `origin: human` and moves the wheel. P3 must say whether this chunk's count tells a wake's prompt from a typed one. Rows: `decision/harness-prompt-prefixes`, `decision/harness-prompts-not-human`, `decision/prompt-normalise-order`.

@history_item n=15 marker=2026-10-07-live-rows-and-paste-shapes-on-the-dev-host
**2026-10-07-live-rows-and-paste-shapes-on-the-dev-host** — (the cross-session form and the typed task-notification as measured) measured the plain cross-session prefix filing `harness` live on 2.1.287 from a scratch probe, with the harness-prefix ledger row still owed. This is the measured base for any further step that classifies a wake's prompt as harness, which the scope places but does not build.

@history_item n=16 marker=2026-10-04-the-wheel
**2026-10-04-the-wheel** — (the wheel as landed) made a human-filed `prompt-submitted` move the wheel before its line is appended, and a driver `send` then refused `human-typing`. This is the mechanism behind the adjacent fact that a scheduled wake's prompt takes the wheel (`decision/human-prompt-confirms-wheel-take`, `decision/byte-source-wheel-claim`). The chunk's count does not change it.

@history_item n=17 marker=2026-10-19-newer-claude-cli-rows
**2026-10-19-newer-claude-cli-rows** — (the 2.1.302 fixture set) committed `fixtures/claude/2.1.302/` (recorded whole by `viola verify --record`) beside 2.1.283, 2.1.287 and 2.1.288, and the fake agent replays recorded payloads unchanged. The scope's premise names `Stop.default.json` in those four sets as holding `session_crons`, so CI's fake agent carries the field on replay. Any fixture change goes through the recorder's scrub and hygiene walk.
