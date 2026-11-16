@history_item n=1 marker=2026-11-15-api-error-turn-end
**2026-11-15-api-error-turn-end** — (Standard Contracts) `~ contract/event-data/turn-ended-last-assistant-message` gains `cause?`; `+ contract/event-data/turn-ended-cause` (`cause` only on an API-error end, as `api-error`, crossing into `events.ndjson`, `wait --json` and MCP `wait`); `+ decision/api-error-turn-end-clears-turn`. These are the rows this chunk changes: one word becomes nine. F2 (the one word) was a delegate's PROVISIONAL answer, now superseded by this chunk.

@history_item n=2 marker=2026-11-15-api-error-turn-end
**2026-11-15-api-error-turn-end** — (Conventions) `+ contract/hook-to-kind-map/stopfailure` (data exactly `last_assistant_message` null, `background_tasks` null, `cause` `api-error`) and `+ contract/turn-end-cause/cause-set` (closed `TurnEndCause` in viola-core beside `UnreachableCause`, one member, no Claude-specific name). These are the two rows the nine words rewrite. The set stays in viola-core with agent-neutral words, and the StopFailure map row must name the classified word in place of the fixed `api-error`.

@history_item n=3 marker=2026-11-15-api-error-turn-end
**2026-11-15-api-error-turn-end** — (Cross-cutting Patterns) `~ decision/capability-ledger-gate` except: StopFailure registered with no ledger row and no verify probe. This is the standing exception the chunk rests on. Its classifier has no ledger row to re-validate the CLI's `error` values, so the mapping's honesty rests on recorded payloads or a static read (the P3 premise).

@history_item n=4 marker=2026-11-15-api-error-turn-end
**2026-11-15-api-error-turn-end** — (Established Decisions) `~ decision/cli-capability-ledger` except (StopFailure, no row); `~ contract/hook-tiers/ordering-spine` and `~ contract/hook-tiers/sync-spine` add StopFailure to Stop's sync tier, 5 s timeout. The hook path that carries the classified word is already in place. The chunk must not move the tier or the spine deadline.

@history_item n=5 marker=2026-11-15-api-error-turn-end
**2026-11-15-api-error-turn-end** — (Design Philosophy) `~ decision/capability-ledger` except: StopFailure registered with no ledger row. The same exception on the design-philosophy row, which is picked here.

@history_item n=6 marker=2026-11-15-api-error-turn-end
**2026-11-15-api-error-turn-end** — (Occupied Resources) `+ name/other/hook-stop-failure`. The hook word stays as it is. No new hook name or EventKind is minted (scope boundary).

@history_item n=7 marker=2026-11-16T06-05-33-wrap
**2026-11-16T06-05-33-wrap** — (Standard Contracts, ratifying 2026-11-15-api-error-turn-end) The founder ratified F1 (no ledger row; the live reading is owned by «The 2.1.325 recorded fixture set»). F2's one word `api-error` is superseded, not ratified, by «Granular API-error cause», which owns its mark, and the rows keep the one word until this entry lands. This chunk must discharge that mark on every row that names the one word.

@history_item n=8 marker=2026-11-16T06-05-33-wrap
**2026-11-16T06-05-33-wrap** — (Conventions, ratifying 2026-11-15-api-error-turn-end) The same supersession on the Conventions rows (`turn-end-cause/cause-set`, `hook-to-kind-map/stopfailure`). Those rows' one word is this chunk's to replace.

@history_item n=9 marker=2026-11-16T06-05-33-wrap
**2026-11-16T06-05-33-wrap** — (Design Philosophy) `~ decision/capability-ledger` except drops «provisionally». StopFailure with no ledger row now stands ratified, so the nine words go through the same no-row path.

@history_item n=10 marker=2026-11-16T06-05-33-wrap
**2026-11-16T06-05-33-wrap** — (Cross-cutting Patterns) `~ decision/capability-ledger-gate` except drops «provisionally». The picked row's current text. Adding a ledger row instead would be a new widening.

@history_item n=11 marker=2026-11-16T06-05-33-wrap
**2026-11-16T06-05-33-wrap** — (Established Decisions) `~ decision/cli-capability-ledger` except drops «provisionally». The picked row's current text.

@history_item n=12 marker=2026-11-16T06-05-33-wrap
**2026-11-16T06-05-33-wrap** — (the ratification records for 2026-11-15-api-error-turn-end's Cross-cutting, Design Philosophy, Established Decisions and Occupied Resources entries) F1 ratified, with the StopFailure live reading owned by «The 2.1.325 recorded fixture set». This chunk takes no live start. A mapping value that needs a live reading belongs to that entry and must be named, not measured, here.

@history_item n=13 marker=2026-11-16-wait-wake-condition
**2026-11-16-wait-wake-condition** — `~ contract/channel-methods/wait-wake-kinds` (a wake is decided by event kind alone, so every `turn-ended` wakes), `~ contract/channel-wait/wake-kinds`, and `~ contract/event-data/turn-ended-last-assistant-message` (no scheduled-wake count). The wake must not start branching on `cause`. The turn-ended data row was last amended here, so the nine-word edit must keep the no-wake-count clause.

@history_item n=14 marker=2026-10-19-overseer-switch-to-viola
**2026-10-19-overseer-switch-to-viola** — `turn-ended` gained `background_tasks` as a count only (no element read or carried, `null` when absent), and option text crossing outward was a founder-answered widening. This is the precedent for what may leave the adapter: a closed or derived value only, never upstream text. It matches the boundary that the error text crosses nothing but the word.

@history_item n=15 marker=2026-11-14-budget-gate-removed
**2026-11-14-budget-gate-removed** — (Conventions) `~ decision/protocol-version-bump-rule` gains its except (removing the budget fields and kind kept every own-format `v` at 1). The rule this chunk relies on: a closed value set that grows on an existing field is additive with no `v` bump. Older readers meet a new word as an unknown value and must tolerate it.

@history_item n=16 marker=2026-11-14-status-line-interception-removed
**2026-11-14-status-line-interception-removed** — (Conventions) `~ decision/protocol-version-bump-rule` except widened to the statusline removals. This is the row's current except text. The chunk edits nothing in it.

@history_item n=17 marker=2026-11-15T18-01-44-wrap
**2026-11-15T18-01-44-wrap** — The founder ratified the widened protocol-version-bump except. The row stands on his word, and the chunk must leave it as it reads.

@history_item n=18 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** — Created `viola_core::UnreachableCause`, exit 21's closed cause set: no serde path, words through `as_str`, not a `RefusalReason`. This is the direct precedent for `TurnEndCause`, which sits beside it. Crossing a cause word to a same-user driver was the founder's own word.

@history_item n=19 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** — The exit-21 document carries its cause in `detail`, five closed words; `v` stays 1; MCP's result shape is unchanged. This is the precedent for widening a closed word set outward (CLI `--json` and MCP) with no `v` bump. It was ratified as a widening by the founder.

@history_item n=20 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** — (code-word rule) Every exit cause passes `is_code_word` and is a row of `tests/contract_error_bodies.rs`, with no path, pid or upstream text. The nine cause words should meet the same code-word rule and contract-table pattern.

@history_item n=21 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** — The `process-exit` `detail` enum in `schemas/diag-line.v1.json` gained six words. This is the precedent for noticing a new value through a closed log word, never its text. The schema's closed enum has to grow with any word the chunk logs.

@history_item n=22 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** — `unknown`'s detail is a code word or null. A peer's `refusal` that is not a code word reads `unknown`, and a non-code-word `detail` reads `null`. This is the precedent for «a value outside the map reads `api-error`»: unknown input folds to a closed fallback word, never to text and never to an error.

@history_item n=23 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** — `sanitised_fault`: a fault code outside the five reads the fixed `wrapper fault`. This is a second fallback-word precedent for an unmapped external value.

@history_item n=24 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** — The sanitised-error-surfaces pattern (`decision/sanitised-error-surfaces`, guard `contract_error_bodies`) and its three surface witnesses. These are the guard and census the nine words must pass. Upstream error text must reach no document.

@history_item n=25 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** — `channel-response`'s `detail` closed to three words in the diag schema, with no param value. This is the precedent for closing a schema field to an enum when adding a log word for an unmapped `error` value.

@history_item n=26 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** — No after-every-test error-body helper (`decision/no-after-every-test-error-body-helper`). The surface witnesses, the enumeration and the census are the check. The chunk proves «text crosses nothing» through them and builds no new per-test helper.

@history_item n=27 marker=2026-10-09-epoch-3-cleanup
**2026-10-09-epoch-3-cleanup** — `empty-text` added as the sixth closed `not-delivered` detail: no exit code, channel method or `v` moved. This is the precedent for growing a closed word set additively.

@history_item n=28 marker=2026-10-10-self-healing-state
**2026-10-10-self-healing-state** — `events::read_from` counts per read `unknown_kinds`, `unknown_fields` and `torn_lines`. A known-kind line whose key is outside its kind's `data` list is returned and counted. This is the event-log reader that must accept the nine words (`decision/reader-counts-skipped-lines`). The counts are not logged, so they cannot carry a new-value notice.

@history_item n=29 marker=2026-10-11-the-board-viola-list
**2026-10-11-the-board-viola-list** — `viola list` is the first surface showing the three skip counts, summed over every scanned instance (`contract/list-json/skipped`). The board is a named consumer, and an unknown cause word must not become a skip or a refusal there.

@history_item n=30 marker=2026-09-27-hooks-to-normalised-events
**2026-09-27-hooks-to-normalised-events** — `hook.event` is served and re-validated (hook kinds, object `data`), appended with `source:"hook"`, and an invalid event is not appended. This is the origin of the wrapper's appended-line check in `src/cmd/run.rs`, which must admit exactly the nine words.

@history_item n=31 marker=2026-11-16-local-command-delivery
**2026-11-16-local-command-delivery** — (Conventions) `~ contract/channel-methods/hook-event` gains `transcript?`. This is the picked row's current shape. The chunk changes `data`'s `cause` values, not the params.

@history_item n=32 marker=2026-11-16-local-command-delivery
**2026-11-16-local-command-delivery** — (Standard Contracts) `~ contract/hook-event/notification` gains `transcript?`, and `+ contract/hook-event/transcript`. This is the picked row's current text, and it shows the latest addition to the hook.event frame was optional with no `v` bump.

@history_item n=33 marker=2026-10-04-wait-and-last
**2026-10-04-wait-and-last** — `wait` (`WAIT_WAKE`, line-start rule, cursor) and `last` (rebuilt from the newest `turn-ended`) as built. These are the consumers `wait --json` and `last` that read `turn-ended`, and the cause words must pass through them unchanged.

@history_item n=34 marker=2026-10-19-dialog-hold-for-the-driver
**2026-10-19-dialog-hold-for-the-driver** — `dialog-expired` was added to the normalised kinds and the wake set became six. This is the last change to the wake-kind rows and a precedent for a viola-core addition. This chunk adds no kind.

@history_item n=35 marker=2026-10-17-mcp-server-for-drivers
**2026-10-17-mcp-server-for-drivers** — MCP `wait` sends `min(given, 50 000)` ms (`decision/mcp-wait-cap`). Each handler re-validates its raw arguments, and `invalid-params` carries a closed field word. MCP `wait` is a named consumer, and its success payload passes the event line through.

@history_item n=36 marker=2026-10-04-running-turn-refusal
**2026-10-04-running-turn-refusal** — The running-turn state: `turn-ended` ends it, keyed on kind alone. Every cause word must still end the turn the way `api-error` does (`decision/api-error-turn-end-clears-turn`).

@history_item n=37 marker=2026-11-16-send-confirmation
**2026-11-16-send-confirmation** — `+ decision/send-flight-lock`: the hook.event claim, turn start and end, and wheel moves take one wrapper flight lock. An API-error turn end goes through that lock, and the chunk must not move it.

@history_item n=38 marker=2026-10-07-live-rows-and-paste-shapes-on-the-dev-host
**2026-10-07-live-rows-and-paste-shapes-on-the-dev-host** — The founder's ruled limit that a CLI shape no send relies on needs no probe (`decision/no-probe-for-unrelied-shapes`). This bears on whether the nine-word map needs a probe, and on stating that no row is added.

@history_item n=39 marker=2026-10-10-statusline-pass-through
**2026-10-10-statusline-pass-through** — Relied-on dependences with no row are recorded as named exceptions, each with an owner, and the ledger rule stands. This is the precedent form for stating the StopFailure `error`-value reading and its owner without a row.

@history_item n=40 marker=2026-10-05-permission-end-to-end
**2026-10-05-permission-end-to-end** — A body shape was read statically from both installed bundles, never run, and was left unused because it had no ledger row. This is the precedent for the classifier's values «read statically from the 2.1.287–2.1.325 bundles»: P3 must mark each value as measured or read from source.

@history_item n=41 marker=2026-10-13-owed-ledger-rows
**2026-10-13-owed-ledger-rows** — A version is verified only when every relied row reads `pass`, and adding a row makes every older stamp read unverified (`decision/version-verified-all-relied-rows`). This is the cost if the chunk added a ledger row instead of the ruled no-row path.

@history_item n=42 marker=2026-11-14-budget-gate-removed
**2026-11-14-budget-gate-removed** — (Cross-cutting Patterns) `~ decision/mixed-version-tolerance`: an older writer's state reads whole. Older readers in the field still accept only `api-error`. Mixed-version tolerance means they must skip or count the new words, never fail.

@history_item n=43 marker=2026-09-24-diagnostics-plane
**2026-09-24-diagnostics-plane** — Codes-only home-level role logs, with content-bearing detail only in instance detail files. Process-log lines carry no `v`; their version is in the schema filename. A log word for an unmapped `error` value must be a closed code in the diag-line schema, and the error text reaches no line of either kind.

@history_item n=44 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** — The instance detail files are named `instances/<ViolaName>/diagnostics/detail-<role>.ndjson` (format `schemas/diag-detail.v1.json`). The founder's boundary forbids the CLI's error text here too, although detail files are content-bearing by design.

@history_item n=45 marker=2026-09-28-capability-ledger-and-viola-verify
**2026-09-28-capability-ledger-and-viola-verify** — The hidden `hook --capture` arm and recorded fixtures, scrubbed, written only from a clean verify (`decision/verify-record-only-clean`). These recorded payloads are the evidence P3 reads for each `error` value with no live start.

@history_item n=46 marker=2026-10-05-dialog-rows-and-re-probe
**2026-10-05-dialog-rows-and-re-probe** — The capture arm claims `<PascalEvent>.<k>.json` exclusively, `k` counted across events (`contract/probe-capture/payload-file`). This is how a StopFailure payload capture is named if one is recorded.

@history_item n=47 marker=2026-11-14-status-line-interception-removed
**2026-11-14-status-line-interception-removed** — (Established Decisions) `~ decision/hooks-fail-open` except names the one `--capture` arm. This is the picked row's current text, and the StopFailure hook stays fail-open whatever its `error` value.

@history_item n=48 marker=2026-11-15-mutation-and-coverage-baselines-green
**2026-11-15-mutation-and-coverage-baselines-green** — `~ decision/root-waits-share-one-bound`: `WITHIN` is 12 s. This is the bound the chunk's bounded tests share (directive: bounded tests).

@history_item n=49 marker=2026-09-28-mutation-testing-to-the-epoch-boundary
**2026-09-28-mutation-testing-to-the-epoch-boundary** — CI runs no mutation job, and mutation runs only at the epoch-boundary audit (`decision/mutation-at-epoch-boundary`). This matches the directive's «no mutation run».

@history_item n=50 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** — CI runs on the one `ubuntu-latest` leg against the fake agent (`decision/ci-ubuntu-fake-agent`, `decision/ci-against-fake-agent`). The chunk's proof rests on that leg and on recorded payloads.
