@history_item n=1 marker=2026-09-24-fake-agent-and-test-data-fixtures
**2026-09-24-fake-agent-and-test-data-fixtures** (fixture naming and the `--fixtures` root) — fixtures named `<Event>.<variant>.json` by the CLI's own PascalCase hook name with no mapping table, the fake agent joining `<cli-version>` under the parent `--fixtures fixtures/claude`; a payload replayed as recorded bytes, consumer-first, no shape invented before a recorded fixture. The new `fixtures/claude/2.1.325/` set must land in this exact layout (decision/fixture-named-by-hook-event, contract/claude-fixtures/payload-path).

@history_item n=2 marker=2026-09-28-capability-ledger-and-viola-verify
**2026-09-28-capability-ledger-and-viola-verify** — the fixture hygiene walk is a run-time directory walk over `fixtures/claude/*/*.json`, not rstest `#[files]`, because rstest 0.27 refuses an empty glob at compile time. A new version dir joins the walk with no code edit, and every file in it must pass schema and the unclean checks (decision/fixture-hygiene-walk).

@history_item n=3 marker=2026-09-29-verify-stamped-test-homes-and-harness
**2026-09-29-verify-stamped-test-homes-and-harness** — `stamped_home` and `Wrapper::boot` pin `--cli-version` to the recorded set's version; the fake agent's `DEFAULT_CLI_VERSION` equals that version and the harness `boot --cli-version` default matches (decision/harness-cli-version-matches-fake). It also specified `run --local-live` (suite `local-live`, `row-missing` / `verify-exit-<n>` codes, `live-in-ci` refusal under `CI`). If this chunk moves the default to 2.1.325, every one of these sites moves together.

@history_item n=4 marker=2026-09-29-fake-agent-drift-contract
**2026-09-29-fake-agent-drift-contract** — `contract_fake_agent_drift` compares the fake agent's hook stdin byte for byte with each recorded `fixtures/claude/<ver>/` set, walked at run time, an empty walk failing; never insta (insta is kept for decision bodies only). The 2.1.325 set is replayed by this walk automatically, so every variant it records must replay byte-exact (decision/fake-agent-contract-suite, decision/fixture-drift-byte-compare, decision/fake-transcripts-compared-to-recordings).

@history_item n=5 marker=2026-10-04-dialog-answers-by-dialog-id
**2026-10-04-dialog-answers-by-dialog-id** — the precedent for moving the default CLI version (2.1.283 → 2.1.287 across `DEFAULT_CLI_VERSION`, `RECORDED_CLI_VERSION`, `stamped_home`, `Wrapper::boot`) and for the hygiene walk naming the sets it covers. The relayed dialog tier it describes was later superseded by recordings. This is the checklist if 2.1.325 becomes the default.

@history_item n=6 marker=2026-10-05-real-cli-verify-probes
**2026-10-05-real-cli-verify-probes** — `contract_ledger_probes` keeps literal lists: stamped sets and drift-only sets, every fixture dir on exactly one list; screens are a separate fixture class checked against `schemas/claude-screen.v1.json`, signature rows only, scrub-as-detector, refused whole. A new `2.1.325` dir must be put on one list (the scope's «`STAMPED`»), or the census fails (decision/ledger-probe-contract-suite, decision/screen-fixture-scrub-as-detector).

@history_item n=7 marker=2026-10-05-dialog-rows-and-re-probe
**2026-10-05-dialog-rows-and-re-probe** — the dialog tier is recorded by verify's Runs C / D as `<Event>.<stem>-<n>.json` (12 per version), scrubbed and unclean-checked, and the drift contract replays every variant. A clean `--record` on 2.1.325 must produce this whole dialog set.

@history_item n=8 marker=2026-10-06-local-command-and-paste-framing-rows
**2026-10-06-local-command-and-paste-framing-rows** — framing variants (`UserPromptSubmit.paste-<n>`, `SessionEnd/SessionStart.clear-<n>`) are recorded by Run B. A set without them is run without `--framing` and must exit 1, and only the stamped set(s) end `N pass  0 fail`, the others staying drift-only. A partial 2.1.325 set cannot be listed `STAMPED`.

@history_item n=9 marker=2026-10-08-first-live-test-and-self-drive
**2026-10-08-first-live-test-and-self-drive** — `run --local-live`, because `verify` names no program, stamps the first `claude` on `PATH`. This is a trap given the scope's take-up fact: a bare `claude` now reads 2.1.327, so a live re-stamp or `--record` for 2.1.325 must name the install path explicitly. A route entry's live readings are a local measurement, not a suite.

@history_item n=10 marker=2026-10-13-paste-newline-ledger-row
**2026-10-13-paste-newline-ledger-row** — the precedent for a widened relied set: the tmpfs live home stamped at the older count reads `row-missing` until its next `run --local-live` re-stamps it. The same mechanism explains C3/C8 (every 22- or 23-row stamp reads unverified until a 24-row re-stamp).

@history_item n=11 marker=2026-10-14-resume-and-compact-ledger-rows
**2026-10-14-resume-and-compact-ledger-rows** — Run E records `SessionStart.resume-1.json` and the drift contract replays it. The fake agent's `--resume` builds on the resume variant where the set holds it, else `SessionStart.default`. This bears on C10: whether Run E starts on 2.1.325, and so whether the set carries the resume variant.

@history_item n=12 marker=2026-10-16-unwrapped-sessions-on-the-board
**2026-10-16-unwrapped-sessions-on-the-board** — Run B's agents read is recorded as `Agents.recorded-1.json`, schema-checked against `schemas/claude-agents-fixture.v1.json`, hygiene-walked and shape-compared by `fake_agent_agents_replay_matches_every_recorded_agents_set`. The new set must carry it, or that every-set walk will read the gap.

@history_item n=13 marker=2026-10-18-compact-post-condition-ledger-row
**2026-10-18-compact-post-condition-ledger-row** — Run B's added `/compact` paste records `SessionStart.compact-<n>`, replayed by `fake_agent_compact_replay_matches_every_recorded_compact_set`. The fake agent's compact replay sets only `session_id`, the one built payload beside `--resume`'s (contract/fake-agent-input/compact). The 2.1.325 set needs this variant to stamp the `local-command-compact` row.

@history_item n=14 marker=2026-10-19-newer-claude-cli-rows
**2026-10-19-newer-claude-cli-rows** — the direct precedent for landing a new version's recorded set (2.1.302): `contract_ledger_probes`' stamped sets grew from `2.1.287` to `2.1.287` + `2.1.302`. The hygiene walk and the Recorded hook payloads row named the new set's dialog, framing, resume, agents and screen variants, and the Contract row counted the variants per set. The 2.1.325 landing follows the same pattern.

@history_item n=15 marker=2026-10-20-epoch-5-cleanup-ii
**2026-10-20-epoch-5-cleanup-ii** (the `Mcp.` exchange) — `Mcp.tools-list-1.json` is recorded by the test-only `viola-mcp-tap` between one live `claude -p` start and a direct `viola mcp` spawn, through the `--record` scrub and its four refusal codes. `mcp_fixtures` walks it against `schemas/claude-mcp-fixture.v1.json`, and `tests/mcp_recorded_exchange.rs` replays it against the served `tools/list`, its tool names in order, `ttlMs` and `cacheScope` included. This is C1's re-record target (decision/mcp-exchange-recorded-by-tap, decision/mcp-tap-refuses-dirty-exchange, contract/mcp-server/tools-list).

@history_item n=16 marker=2026-10-26-server-verification-before-any-frame
**2026-10-26-server-verification-before-any-frame** (fake-agent copies planted one after another) — a test that plants more than one fake-agent copy, one per recorded set's bin, plants them sequentially before any thread spawns a child, else ETXTBSY on tmpfs (6 of 40 red in `fake_agent_agents_replay_matches_every_recorded_agents_set`). A fifth recorded set adds one more planted copy to every set-walking test.

@history_item n=17 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** — `verify` refuses a widened `ledger/` before any step, and `update_stamps` refuses a widened ledger before its create and again under the lock, reading and writing nothing (decision/stamps-refuse-widened-ledger). `only_the_wrapper_writes_the_snapshot_and_only_verify_the_stamps` holds the single writers. These bind the live home's re-stamp (C3/C8).

@history_item n=18 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** (`viola verify`'s refusal codes) — verify writes its refusal code on `process-exit` under a parsed `VIOLA_NAME`. A recording refused as unclean is `Ended::Refused` with `detail` `unclean-fixture`, and the fixture's file name enters no code. A failing row is no refusal and carries no code, and verify takes no `--json`. A dirty 2.1.325 recording surfaces this way (contract/verify-exit-detail/unclean-fixture, decision/verify-failing-row-no-refusal, decision/verify-takes-no-json).

@history_item n=19 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (the relied set on every build) — `LedgerRow::relied()` is all rows on every build (`STEPS_PASS`, `ROW_IDS`, `LEDGER_ROWS` as `[&str; 24]`), and the older-stamp oracle reads earlier id sets unverified. Today's relied count, with the trust-record and browser-tools-prompt rows since added, is what a clean 2.1.325 `--record` must stamp.

@history_item n=20 marker=2026-11-06-version-done-check
**2026-11-06-version-done-check** (the planted annotation drift witness; the statusline arm's live red) — `dialog_drift_names_a_changed_annotation` plants an `annotations` byte change and reads `dialog_drift` red, the synthetic witness of the drift suite this chunk runs against the new set. The same wrap recorded a live verify reading a row red on account state, not CLI behaviour (`statusline-rate-limits`, since retired): a precedent for a live failure that is not a CLI fact.

@history_item n=21 marker=2026-11-13-verify-readiness-on-the-input-box
**2026-11-13-verify-readiness-on-the-input-box** — added decision/screen-fixture-box-rows-as-literals: a recorded screen keeps a box rule row as its rule chars alone and the prompt row as the prompt char alone, and the walk refuses anything more. It also moved contract/readiness-gate/gate-max-wait to read the box up by its rule and prompt rows. This constrains the box-only screen C1's CARRY 2 reads, and relates to C7's holds-text placeholder hint.

@history_item n=22 marker=2026-11-14-hyprland-seam
**2026-11-14-hyprland-seam** — reworded the fixture rows this chunk relies on: decision/fixtures-recorded-by-live-verify (a third recorder, `viola-hypr-tap`), decision/recorder-scrub (both taps apply the verify scrub), decision/dirty-payload-refuses-recording (a dirty payload, MCP exchange or Hyprland set refuses the whole recording), decision/fixture-hygiene-walk and decision/fixtures-walked-for-hygiene, and decision/fakes-for-external-services / decision/ci-never-real-claude (no real `hyprctl` in CI either).

@history_item n=23 marker=2026-11-14-hyprland-exact-frame-bound-pinned
**2026-11-14-hyprland-exact-frame-bound-pinned** — decision/fakes-for-external-services gained its except: `viola-hyprland`'s own unit tests serve `hyprctl` by a planted shell stub, because cargo-mutants runs only the mutated package's tests.

@history_item n=24 marker=2026-11-14-budget-gate-removed
**2026-11-14-budget-gate-removed** — decision/version-gate-needs-all-relied-rows gained «a retired row it also holds changes nothing» (`statusline-rate-limits` retired). contract/boot-readiness/events and contract/run-start-events/order dropped to two start records (wheel, then session-start), and decision/state-files-never-hand-written and decision/self-bootstrapping-test-data lost `budget.json`. A 2.1.325 stamp must satisfy the current relied set, not the retired row.

@history_item n=25 marker=2026-11-14-status-line-interception-removed
**2026-11-14-status-line-interception-removed** — the StatusLine fixtures and verify's `--record` status-line variants were removed. decision/fake-agent-contract-suite and decision/fixtures-recorded-by-live-verify no longer list a status line family, and the `workspace.repo` exceptions of decision/recorder-scrub, decision/fixture-hygiene-walk and decision/no-production-data-in-tests are gone. So a 2.1.325 recording writes no StatusLine file, and any `workspace.repo` it captured would be refused, not kept.

@history_item n=26 marker=2026-11-15-live-round-on-the-trimmed-product
**2026-11-15-live-round-on-the-trimmed-product** — contract/event-log/start-wheel-record: where folder-trust keys pass through the wrapper, a `wheel{holder:"human", cause:"human-input"}` precedes session-start. Added decision/start-lines-read-from-start-offset: a reused instance name's log is read from the start's own offset, never line 1. Both bind any live start reading in this chunk.

@history_item n=27 marker=2026-11-15-mutation-and-coverage-baselines-green
**2026-11-15-mutation-and-coverage-baselines-green** — added decision/record-home-names-no-home: a home given to `viola verify --record` comes from the shared `record_home()` helper and fixture, with no path component named `home`, because the recorder refuses a payload holding `/home/`. This is the tests' half of C6, whose rig half this chunk owns. WITHIN moved to 12 s (decision/within-7s-wait-bound).

@history_item n=28 marker=2026-11-15-api-error-turn-end
**2026-11-15-api-error-turn-end** — decision/hook-payloads-recorded-by-verify gained its except: StopFailure is registered with no recorded payload, no ledger row and no verify probe, and its tests use a synthetic payload. contract/event-data/turn-ended gained `cause` on an API-error end only. C2's live StopFailure recording settles or keeps exactly this exception.

@history_item n=29 marker=2026-11-16T06-05-33-wrap
**2026-11-16T06-05-33-wrap** (StopFailure no-ledger-row ratified) — the founder ratified StopFailure standing with no ledger row and no verify probe, «its live reading owned by «The 2.1.325 recorded fixture set»». This chunk is that named owner (C2): a live StopFailure payload read here either earns a row and probe, or the exception stays on the founder's word.

@history_item n=30 marker=2026-11-16-granular-api-error-cause
**2026-11-16-granular-api-error-cause** — contract/event-data/turn-ended's `cause` became one of nine closed words (rate-limited … api-error). C2 reads a live StopFailure `error` word against this nine-word map, with no `parse-rejected` `unknown-value` line for any of the CLI's 13 words.

@history_item n=31 marker=2026-11-16-scheduled-wake-count
**2026-11-16-scheduled-wake-count** — added contract/dialog-data/scheduled-wakes and contract/turn-ended/scheduled-wakes (length of the Stop payload's `session_crons`, or null), and turn-ended's key set became four. The 2.1.325 recorded `Stop` payload feeds these through replay.

@history_item n=32 marker=2026-11-16-local-command-delivery
**2026-11-16-local-command-delivery** — `/mcp` is confirmed by a new stdout tag in the transcript (decision/local-command-post-condition, contract/send-result/unconfirmable, decision/local-command-unconfirmable). The fake agent gained `--transcript` and the echo command (contract/fake-agent-cli/transcript, contract/fake-agent-input/echo-command), and decision/stamping-verify-passes-transcript says without a transcript Run B's echo probe is skipped and `local-command-echo` fails. This bears on C3 (the 22nd row) and C4 (`/context` unmeasured).

@history_item n=33 marker=2026-11-16-wait-wake-condition
**2026-11-16-wait-wake-condition** — added decision/one-verify-stamp-per-build (stamped homes copy one verify stamp per pair of built binaries under `target/e2e-home`, keyed by the binaries' hashes, under a lock). decision/verify-driven-tests-20s-kill now rests on that one stamp. Moving the default set or the stamped list changes what that one stamp holds.

@history_item n=34 marker=2026-11-17-standing-live-rig
**2026-11-17-standing-live-rig** — decision/standing-live-rig replaced the old live-readings row: live rounds reuse one committed `viola-live-rig`, its rehearsals run in CI, and its live starts run by hand under the founder's start cap, each ledgered with its clock time and refused under CI. The child screen may be read by compiled literals into closed words (except cells on decision/never-parse-rendered-child-screen, decision/no-child-screen-assertions, decision/discipline-at-every-layer, decision/no-blanket-approval-tests). decision/real-claude-local-only now names the rig. Every live start of this chunk (C1, C5) goes through it.

@history_item n=35 marker=2026-11-17-booted-wrapper-does-not-outlive-its-test
**2026-11-17-booted-wrapper-does-not-outlive-its-test** — decision/wrapper-drop-guard became a parent-death tie, and decision/no-untied-viola-spawn (a census over every test source) and decision/tied-child-spawn-thread were added. Any new test that spawns `viola verify` against the new set must use the tied builder.

@history_item n=36 marker=2026-11-17-supply-chain-green-and-booted-tests-end
**2026-11-17-supply-chain-green-and-booted-tests-end** — added decision/no-untied-harness-boot (a viola-e2e census, proven red on a planted untied boot). decision/rstest-case-tables gained the shared-helper except (viola-e2e and viola-core use labelled tables), which binds new table-driven cases over the sets.

@history_item n=37 marker=2026-11-17-trust-record-ledger-row
**2026-11-17-trust-record-ledger-row** — added decision/verify-under-test-owned-user-home: every test verify runs under a test-owned user home, and a `--record` verify keeps the real HOME, because the scrub reads HOME, pointing a kept `CLAUDE_CONFIG_DIR` at the test-owned home. Also added decision/no-verify-start-outside-test-home (census), decision/live-failing-reading-not-verification (a failing or confounded live reading is recorded as read, never as verification) and the fake agent's `--trust-config`. All bind this chunk's `--record` runs.

@history_item n=38 marker=2026-11-18-trust-record-row-read-live
**2026-11-18-trust-record-row-read-live** — decision/record-only-at-zero-failing-rows gained its except: a failed recording verify by the live rig alone writes a first-screen file per short run, scrubbed, refused whole when unclean, never committed. The `verify_window_` kill moved 60 s → 80 s. Only a clean 0-fail `--record` lands fixtures, so the 2.1.325 set needs every relied row passing (or absent where allowed).

@history_item n=39 marker=2026-11-18-session-folder-trusted-by-viola
**2026-11-18-session-folder-trusted-by-viola** — contract/fake-agent-cli/trust-config now models the CLI's walk (the cwd's exact key, then parents up to the git root). Added decision/run-under-test-owned-user-home, decision/harness-wrappers-under-session-user-home and decision/trust-start-folders-outside-repository: a start writes the Claude config, so test starts never touch the real one. This is relevant to recording under the rig's private config.

@history_item n=40 marker=2026-11-18-browser-tools-prompt-modal
**2026-11-18-browser-tools-prompt-modal** — added the `absent` verdict on the browser-tools-prompt row alone: contract/verify-output/step-line and summary-line (absent counts toward n), decision/version-gate-needs-all-relied-rows, contract/local-live-suite/pass and decision/ledger-row-verify-probe. It also added contract/fake-agent-cli/start-screen. A 2.1.325 recording on a host whose browser-tools prompt was never answered reads 8/24 (C9), and the stamped literal for the new set must count an absent row; all of it is PROVISIONAL on the delegate's confirm.
