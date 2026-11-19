@history_item n=1 marker=2026-11-15-api-error-turn-end
**2026-11-15-api-error-turn-end** (Standard Contracts: 3 row(s)) — added `cause?` to `turn-ended` data, + contract/event-data/turn-ended-cause and + decision/api-error-turn-end-clears-turn. This is where the StopFailure arm this chunk reads live was born. At the time the payload was read only statically from the bundles.

@history_item n=2 marker=2026-11-15-api-error-turn-end
**2026-11-15-api-error-turn-end** (Cross-cutting Patterns: 1 row(s)) — ~ decision/capability-ledger-gate: added the exception "StopFailure is registered with no ledger row and no verify probe". This is the exception the chunk settles. A ledger-row outcome retires this except clause.

@history_item n=3 marker=2026-11-15-api-error-turn-end
**2026-11-15-api-error-turn-end** (Established Decisions: 3 row(s)) — ~ decision/cli-capability-ledger got the StopFailure exception. ~ contract/hook-tiers/ordering-spine and ~ contract/hook-tiers/sync-spine put StopFailure in Stop's sync tier with the 5 s `timeout`. A probe must read the payload inside that tier.

@history_item n=4 marker=2026-11-15-api-error-turn-end
**2026-11-15-api-error-turn-end** (Design Philosophy: 1 row(s)) — ~ decision/capability-ledger got the StopFailure exception. A ledger-row outcome must change this except too.

@history_item n=5 marker=2026-11-15-api-error-turn-end
**2026-11-15-api-error-turn-end** (Conventions: 2 row(s)) — + contract/hook-to-kind-map/stopfailure, + contract/turn-end-cause/cause-set. These are the map and the cause set the live `error` word is read against.

@history_item n=6 marker=2026-11-15-api-error-turn-end
**2026-11-15-api-error-turn-end** (Occupied Resources: 1 row(s)) — + name/other/hook-stop-failure, the tenth hook event name. Any probe plugin registration for StopFailure uses this name.

@history_item n=7 marker=2026-11-16T06-05-33-wrap
**2026-11-16T06-05-33-wrap** (Design Philosophy, Cross-cutting Patterns and Established Decisions row entries) — dropped "provisionally" from the StopFailure exception on decision/capability-ledger, decision/capability-ledger-gate and decision/cli-capability-ledger. The founder ratified "no ledger row" with StopFailure's live reading owed to «The 2.1.325 recorded fixture set», later re-pinned to this entry. Settling the exception now reopens his ratified word.

@history_item n=8 marker=2026-11-16T06-05-33-wrap
**2026-11-16T06-05-33-wrap** (the StopFailure no-ledger-row answer ratified by the founder, six entries: Standard Contracts, Conventions, Cross-cutting Patterns, Design Philosophy, Established Decisions, Occupied Resources of 2026-11-15-api-error-turn-end) — F1 «no ledger row» ratified, the live reading owned elsewhere; F2's one cause word superseded by «Granular API-error cause». This is the standing ratification the settlement outcome either keeps on his word or replaces with a row.

@history_item n=9 marker=2026-11-16-granular-api-error-cause
**2026-11-16-granular-api-error-cause** (Conventions: 2 row(s)) — TurnEndCause went from one word to nine. ~ decision/unparseable-external-values-unknown except: an error word outside the CLI's known words reads `api-error`, not `unknown`. That is the rule this chunk's live `error` word is checked against (the 13 CLI words were read statically, never live).

@history_item n=10 marker=2026-11-16-granular-api-error-cause
**2026-11-16-granular-api-error-cause** (Standard Contracts: 1 row(s)) — ~ contract/event-data/turn-ended-cause: `cause` is one of the nine words, present only on an API-error end. It crosses into `events.ndjson`, CLI `wait --json` and MCP `wait`.

@history_item n=11 marker=2026-11-16-granular-api-error-cause
**2026-11-16-granular-api-error-cause** (Conventions: 1 row(s)) — ~ contract/hook-to-kind-map/stopfailure: the cause is classified from the CLI's error word inside the Claude adapter. The live payload's word goes through that classifier.

@history_item n=12 marker=2026-11-17T05-21-35Z-wrap
**2026-11-17T05-21-35Z-wrap** (the delegate's confirm ratified by the founder, on 2026-11-16-granular-api-error-cause and 2026-11-16-scheduled-wake-count) — the founder's own word now stands behind the nine-word cause set, the fallback to `api-error`, the classified stopfailure map, the turn-ended `cause` promise, the four-key turn-ended data and the wake rule. The chunk checks a live word against this standing set.

@history_item n=13 marker=2026-11-16-scheduled-wake-count
**2026-11-16-scheduled-wake-count** (Standard Contracts: 3 row(s); Conventions: 1 row(s)) — `turn-ended` data became `{last_assistant_message, background_tasks, scheduled_wakes, cause?}`, + contract/event-data/turn-ended-scheduled-wakes, and the stopfailure map's data became exactly those keys null plus `cause`. A live StopFailure line is checked against this exact set.

@history_item n=14 marker=2026-11-16-wait-wake-condition
**2026-11-16-wait-wake-condition** (Standard Contracts: 3 row(s)) — every `turn-ended` wakes `wait` by kind alone. A StopFailure turn end therefore wakes a parked driver `wait` just as a Stop does.

@history_item n=15 marker=2026-10-19-overseer-switch-to-viola
**2026-10-19-overseer-switch-to-viola** (`question` gains `option_details`, `turn-ended` gains `background_tasks`) — added the count-only `background_tasks` rule (null when absent or not an array). It is null on the StopFailure arm, and it set the count-only, no-element-carried precedent for a payload field.

@history_item n=16 marker=2026-10-19-overseer-switch-to-viola
**2026-10-19-overseer-switch-to-viola** (verify's print-mode probe leaves an empty CLI project dir) — every verify child leaves `~/.claude/projects/` residue that the founder accepted by class. A contained live start's footprint is counted against this.

@history_item n=17 marker=2026-10-04-running-turn-refusal
**2026-10-04-running-turn-refusal** (the running-turn state as built) — `turn-ended` ends the running-turn state. decision/api-error-turn-end-clears-turn and decision/human-interrupt-moves-wheel rest on this: a live StopFailure must clear the turn with no pause or release.

@history_item n=18 marker=2026-10-04-wait-and-last
**2026-10-04-wait-and-last** (wait and last as built) — `last` is updated under the append's lock and rebuilt at start (contract/channel-methods/last, last-after-seen-line). A StopFailure end with a null `last_assistant_message` gives `null` plus its `ts`.

@history_item n=19 marker=2026-09-27-hooks-to-normalised-events
**2026-09-27-hooks-to-normalised-events** (seven exec-form hooks, `hook.event` served) — set up the exec-form `hooks.json` spine entries at `"timeout": 5` and `hook.event` re-validation with `source:"hook"`. StopFailure's registration extends this tier.

@history_item n=20 marker=2026-10-04-readiness-gate-and-timing-constants
**2026-10-04-readiness-gate-and-timing-constants** (the readiness gate as landed) — the spine deadline is `viola_core::SPINE_DEADLINE` = 1 s, below the 5 s hook timeout. This is the deadline the ordering-spine rows hold StopFailure to.

@history_item n=21 marker=2026-09-28-capability-ledger-and-viola-verify
**2026-09-28-capability-ledger-and-viola-verify** (the six landed ledger rows, `viola verify`'s probe, the stamps envelope) — origin of the closed ledger, the `viola verify` print-mode probe, the stamps envelope, and `--record` writing scrubbed `<Event>.default.json` fixtures only at 0 fail. A StopFailure row joins this mechanism, and its fixture is recorded under these rules.

@history_item n=22 marker=2026-09-28-capability-ledger-and-viola-verify
**2026-09-28-capability-ledger-and-viola-verify** (the hidden `hook --capture` arm, founder-ratified) — the hidden `--capture` arm writes the raw `<PascalEvent>.<k>.json` and reads no `VIOLA_*`. Only verify's probe plugin may register it. A StopFailure probe captures through this arm, and the arm is a founder-ratified boundary.

@history_item n=23 marker=2026-10-05-t00-09-13-wrap
**2026-10-05-t00-09-13-wrap** (the typed-input probe ratified) — the founder ratified verify driving the installed `claude` locally only, never with a credential on a CI runner. This bounds where any StopFailure probe may run (decision/real-cli-probe-local-only).

@history_item n=24 marker=2026-10-05-real-cli-verify-probes
**2026-10-05-real-cli-verify-probes** (verify's interactive runs) — added the first interactive PTY runs (A untrusted, B trusted), under the founder's "two runs, never accept" and no-key-into-dialogs rulings. Precedent: every widening of verify's run set is the founder's own word.

@history_item n=25 marker=2026-10-05-dialog-rows-and-re-probe
**2026-10-05-dialog-rows-and-re-probe** (fourteen ledger rows, Runs C and D) — added Runs C and D. A run that raised no event fails its rows. This is the shape a row needing a run that provokes StopFailure inherits, and a new run is a ratified child-set widening.

@history_item n=26 marker=2026-10-05-permission-end-to-end
**2026-10-05-permission-end-to-end** (the question first raised by PermissionRequest stays null) — a statically read CLI body was left with no row because a new row costs a probe plus a re-stamp of every stamped version (decision/permission-request-question-stays-null). This is the price precedent for the row-or-exception choice.

@history_item n=27 marker=2026-10-06-local-command-and-paste-framing-rows
**2026-10-06-local-command-and-paste-framing-rows** (seventeen ledger rows; registry rows for the framing variants) — a stamp of the older id set reads unverified. New `--record` variant stems went through the same scrub and hygiene walk, and the fake agent replays them by argv options with no env. The trap: adding a StopFailure row unverifies every stamped home until it is re-verified.

@history_item n=28 marker=2026-10-07-live-rows-and-paste-shapes-on-the-dev-host
**2026-10-07-live-rows-and-paste-shapes-on-the-dev-host** (the long-paste frame; the identity floor's Linux reading) — the founder's ruled limit, "a shape no send relies on needs no probe" (decision/no-probe-for-unrelied-shapes). This is a lead for keeping the exception. Also, scratch-probe readings landed no row and left the rows owed: a contained live reading by itself does not settle a row.

@history_item n=29 marker=2026-10-10-viola-revive
**2026-10-10-viola-revive** (a second relied-on shape with no row) — `--resume` landed as a relied shape with no row, its row owed. It was later rowed as `resume-session`. This is the route that one no-row exception took to a row with its probe.

@history_item n=30 marker=2026-10-10-statusline-pass-through
**2026-10-10-statusline-pass-through** (the capability ledger: three relied-on dependences with no row) — named exceptions to the ledger rule each carry an owner, and the rule itself does not change. This is the form a kept StopFailure exception takes.

@history_item n=31 marker=2026-10-13-paste-newline-ledger-row
**2026-10-13-paste-newline-ledger-row** (eighteen ledger rows) — an owed relied shape was rowed by widening a run, with `[NN/MM]` MM = relied count and older stamps unverified. The widening was the founder's own P4 answer. This is a template for the row outcome.

@history_item n=32 marker=2026-10-14-resume-and-compact-ledger-rows
**2026-10-14-resume-and-compact-ledger-rows** (the resume row, Run E; Run E's resources) — a new interactive run (Run E) was a ratified child-set widening and brought its probe dir, spawn pair, `--record` stem and fixture count. A StopFailure run would follow both: his word, and that resource checklist.

@history_item n=33 marker=2026-10-16-unwrapped-sessions-on-the-board
**2026-10-16-unwrapped-sessions-on-the-board** (the agents-join row; the Agents fixture class) — a row was added by a read inside an existing run with no new start, and a new recorded fixture class is replayed by the fake agent from files beside the exe with no new env seam (the founder's word). These are precedents for a StopFailure reading and fixture under CI's fake agent.

@history_item n=34 marker=2026-10-18-compact-post-condition-ledger-row
**2026-10-18-compact-post-condition-ledger-row** (the twenty-fourth ledger row) — the most recent row added by a paste into Run B: stamped live, fixture counted, older stamps unverified, every home re-verified.

@history_item n=35 marker=2026-10-17-optional-plugin-install
**2026-10-17-optional-plugin-install** (plugin install as built; precedence not measured) — plugin precedence was ruled "no ledger row" under the ruled limit, its reading owed to a live entry. Precedent for a measured-but-unrowed outcome.

@history_item n=36 marker=2026-10-20-epoch-5-cleanup-ii
**2026-10-20-epoch-5-cleanup-ii** (the recorded live MCP exchange; the test-only child marker) — a live CLI payload was recorded by a test-only tool outside `--record`, through `ledger::scrub` and `screen_fault`, refusing with the file name and a closed code. It took the founder's word at P4. This is the nearest precedent for recording a contained live payload through the rig's scrub. Test-only markers carry no `VIOLA_` prefix.

@history_item n=37 marker=2026-11-17-trust-record-ledger-row
**2026-11-17-trust-record-ledger-row** (Established Decisions: 4 row(s); 1 row(s) ×2) — the latest row-plus-probe addition: Run F made six interactive runs (decision/verify-child-set, a ruled widening), + decision/verify-own-trust-record, and decision/trust-record-ledger-row asks per CLI version rather than stating fact, because the one live reading read fail. decision/permission-request-question-stays-null now says "a further relied row". This is the closest template for a StopFailure row with its probe.

@history_item n=38 marker=2026-11-17-trust-record-ledger-row
**2026-11-17-trust-record-ledger-row** (Occupied Resources: 4 row(s) and 1 row(s)) — runs A to F in order; verify removes its own record and dirs on pass, fail, Ctrl-C or TERM; keeping `CLAUDE_CONFIG_DIR` makes verify read its value (contract/config-json/claude-env-keep). These rules bind any new probe and the contained start under the rig's private config.

@history_item n=39 marker=2026-11-17-trust-record-ledger-row
**2026-11-17-trust-record-ledger-row** (Infrastructure Patterns, Inherited Defaults, Cross-cutting Patterns, Conventions: 1 row(s) each) — the test-only Claude-shape excepts (claude-shapes-in-one-crate, claude-shapes-isolated) gained the fake agent and rig reading trust records. no-environment-configuration and env-var-viola-prefix gained `CLAUDE_CONFIG_DIR`. Any new rig read of a StopFailure payload, or a containment env such as `ANTHROPIC_BASE_URL` on the child, must be placed against these rows.

@history_item n=40 marker=2026-11-17-trust-record-ledger-row
**2026-11-17-trust-record-ledger-row** (Standard Contracts: 9 row(s) and 7 row(s)) — stamp rows and content may hold the trust writer's word, and the closed trust-writer code words landed. These picked rows bound what a stamp holds and what a contained start's trust write can answer.

@history_item n=41 marker=2026-11-18-trust-record-row-read-live
**2026-11-18-trust-record-row-read-live** (Established Decisions, Occupied Resources, Stack, Infrastructure, Inherited Defaults) — the rig-only `--record-first-screens` writes from a failed verify, the one exception on verify-record-only-clean and recorded-fixtures-scrubbed, as a hidden flag passed only by the rig. The rig seeds its private Claude config. These are the precedents for a rig-only recording path and for the private config the contained start runs under (C1, C2).

@history_item n=42 marker=2026-11-18-trust-record-row-read-live
**2026-11-18-trust-record-row-read-live** (Standard Contracts: 2 row(s)) — + `measured.first_screen` and `measured.first_settle_ms`. A new probe's readings are additive `measured` fields with no `v` bump.

@history_item n=43 marker=2026-11-18-session-folder-trusted-by-viola
**2026-11-18-session-folder-trusted-by-viola** (all five entries) — "run and revive write no trust record" is retired, and the start writes the trust record on a verified CLI. `key-present` and `removed` are verify's words only, and a start reads them as lost. The run and revive start read a kept `CLAUDE_CONFIG_DIR`. A contained wrapped start under a private config passes through all of this.

@history_item n=44 marker=2026-11-18-browser-tools-prompt-modal
**2026-11-18-browser-tools-prompt-modal** (Standard Contracts, Design Philosophy, Established Decisions) — verified now means every relied row "counts": pass, or `absent` on browser-tools-prompt alone. The step line prints pass|absent|fail, the summary counts the rows that count, and + decision/browser-tools-prompt-ledger-row is the 24th relied row. This is the precedent for a row that may not occur in a run, likely the shape of a StopFailure row. A second absent-countable row widens decision/capability-ledger itself, and a new row is the 25th (MM moves).

@history_item n=45 marker=2026-11-18-browser-tools-prompt-modal
**2026-11-18-browser-tools-prompt-modal** (the Claude-shape excepts, 1 row(s) ×3; the verified-version rename) — the rig's reading of the `cachedChromeExtensionInstalled` key name stood on the founder's own word naming that key alone. A rig reading of StopFailure keys needs its own named word.

@history_item n=46 marker=2026-11-18-the-live-home-s-re-stamp-and-the-product-re-pin
**2026-11-18-the-live-home-s-re-stamp-and-the-product-re-pin** (Established Decisions, Infrastructure Patterns, Inherited Defaults) — the current compressed text of the three test-only Claude-shape excepts (rig, VH, fake agent). It is PROVISIONAL, and a new test-only read must extend it without dropping a name.

@history_item n=47 marker=2026-11-17-standing-live-rig
**2026-11-17-standing-live-rig** (Occupied Resources, Established Decisions, Stack) — + `viola-live-rig` (a test-only binary, never released), + `AGENT_RUN_RIG` (never read by viola), and vt100 as the rig's screen model. The C5 record-dir mend and the contained start live in this binary.

@history_item n=48 marker=2026-11-17-first-readings-on-the-standing-rig
**2026-11-17-first-readings-on-the-standing-rig** (Infrastructure Patterns; Established Decisions) — first test-only exception: the rig captures a hook payload and reads key names and the source word, never a value. The founder ratified "Разрешить захват в риге". This is the precedent for the rig capturing a StopFailure payload.

@history_item n=49 marker=2026-11-17-booted-wrapper-does-not-outlive-its-test
**2026-11-17-booted-wrapper-does-not-outlive-its-test** (Occupied Resources: 2 row(s)) — test-only self-spawn markers are registered outside `VIOLA_*` (decision/harness-env-var-prefix). Any containment env the rig sets is placed by this rule.

@history_item n=50 marker=2026-09-29-verify-stamped-test-homes-and-harness
**2026-09-29-verify-stamped-test-homes-and-harness** (verify's spawn pairs, the harness CI read; StampError folded) — `run --local-live` under CI refuses before any spawn (decision/no-live-claude-in-ci). Each probe logs one spawn pair. A malformed stamps envelope makes run read unverified (decision/malformed-stamps-unverified).

@history_item n=51 marker=2026-10-04-dialog-answers-by-dialog-id
**2026-10-04-dialog-answers-by-dialog-id** (decisions on the six-row stamp, the strict stamps read) — run's gate reads stamps through `read_stamps_strict` (decision/version-gate-strict-stamps-read). Relayed, statically read fixtures were later superseded by a live re-probe. This is the precedent for retiring a statically read shape with a live reading.

@history_item n=52 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (the stamps writer's strict-modes check; the home-tree check before verify's log) — `update_stamps` checks strict modes before and under its lock, and verify refuses an untrusted home tree before any probe (stamps-writer-checks-before-and-under-lock, verify-refuses-untrusted-home-or-stamps). A re-stamp after a new row goes through both.

@history_item n=53 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** (`viola verify`'s refusal codes on its `process-exit`) — gave verify's `process-exit` closed refusal codes, `version-unreadable` and `unclean-fixture` among them; the fixture's file enters no code. An unclean StopFailure recording is refused under these.

@history_item n=54 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (the relied set on every build; `batch-script-child` retired) — `relied()` is every row of `ALL` on every build, and an older id set reads unverified. Verify's refusal codes went to five plus `interrupted`. A new row is relied on every build, and a new probe must not widen the closed code set silently.

@history_item n=55 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (the R8 strip: exact names; Linux only; CI on `ubuntu-latest`) — the strip uses exact `CLAUDE*` names with `claude_env_keep` as the one pass-list, so a containment env such as `ANTHROPIC_BASE_URL` passes through untouched. Linux only, no shell or tmux dependence: a no-network-namespace means must not shell out. CI runs the fake agent only, so a StopFailure row must be stampable through a fake-agent replay.

@history_item n=56 marker=2026-09-25-pty-wrapper-on-windows
**2026-09-25-pty-wrapper-on-windows** (R8 prefix rule + identity floor) — origin of the R8 `CLAUDE*` strip and the eleven-name identity floor (decision/r8-identity-floor, child-env-claude-strip). A contained start's child env passes through it.

@history_item n=57 marker=2026-11-06-version-done-check
**2026-11-06-version-done-check** (the statusline-rate-limits row reads the account's plan state) — a ledger row whose reading depended on the account, not the CLI, read red on an unchanged CLI and was later retired. The trap: a StopFailure row provoked through the network or auth may read the environment, not the CLI.

@history_item n=58 marker=2026-11-14-budget-gate-removed
**2026-11-14-budget-gate-removed** (Cross-cutting Patterns: 1 row(s); Established Decisions 1 of 2; Conventions 1 of 2) — decision/mixed-version-tolerance: a stamp holding a retired row verifies on the relied rows alone, the converse of the row outcome's cost. The relied count cells moved with a row. decision/unparseable-external-values-unknown lost its budget fields, leaving the rule that later took the StopFailure exception.

@history_item n=59 marker=2026-11-14-hyprland-seam
**2026-11-14-hyprland-seam** (Conventions: 1 row(s)) — the first exception on decision/unparseable-external-values-unknown (the hyprctl reply). The StopFailure clause was appended beside it.

@history_item n=60 marker=2026-11-14-status-line-interception-removed
**2026-11-14-status-line-interception-removed** (Established Decisions ×2, Occupied Resources 1 of 2, Cross-cutting Patterns) — one hidden `--capture` arm remains (hooks-fail-open, probe-capture-arms-inert, hook-exits-on-untrusted-instance). Hook JSON is parsed tolerantly with drift reports (tolerant-external-payloads). A StopFailure probe uses that one arm and that parse.

@history_item n=61 marker=2026-10-27-bounded-inputs-at-every-boundary
**2026-10-27-bounded-inputs-at-every-boundary** (the serde_json unbounded_depth ban; bounded readers) — `serde_json/unbounded_depth` is banned in `deny.toml` (no-unbounded-json-depth, serde-json-recursion-limit). Every reader of external bytes is bounded or named, held by `tests/contract_bounded_readers.rs`. Any new reader of a captured payload must pass it.

@history_item n=62 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** (the sanitised error surfaces pattern) — no upstream text reaches an error surface. The CLI's `error` word may cross only as the closed TurnEndCause word (decision/untrusted-upstream-text, sanitised-error-surfaces).

@history_item n=63 marker=2026-10-10-self-healing-state
**2026-10-10-self-healing-state** (the reader counts three names) — the event reader counts unknown kinds, fields and torn lines and does not log them (decision/reader-counts-skipped-lines).

@history_item n=64 marker=2026-09-24-diagnostics-plane
**2026-09-24-diagnostics-plane** (v exception) — the `v` rule names its exception (process-log lines carry no `v`, decision/versioned-records-no-migrations). Additive `measured` fields and event keys keep `v` at 1.

@history_item n=65 marker=2026-10-13-interrupted-verify-cleanup
**2026-10-13-interrupted-verify-cleanup** (verify's Ctrl-C / TERM stop) — verify's probe dirs are removed by drop guards on pass, fail or stop, and a stopped run kills its child with no key (decision/verify-probe-dirs-drop-guarded, verify-removes-only-own-dirs).

@history_item n=66 marker=2026-09-28-hook-perf-gate
**2026-09-28-hook-perf-gate** (the second fake-agent test seam `FAKE_AGENT_HOOK_PANIC`) — env test seams are founder-ratified one by one (decision/fake-agent-hook-panic-seam). A fake-agent StopFailure mode should be an argv option or a file, not a third env seam.

@history_item n=67 marker=2026-09-26-local-linux-pre-push-gate
**2026-09-26-local-linux-pre-push-gate** (`FAKE_AGENT_PUMP_DELAY_MS` test seam) — the first ratified test-seam carve-out (decision/fake-agent-pump-delay-seam, test-seams-not-configuration).

@history_item n=68 marker=2026-10-17-cli-output-discipline
**2026-10-17-cli-output-discipline** (the colour reads) — precedent: any further environment read needs its own ratification (decision/env-var-viola-prefix, no-environment-configuration).
