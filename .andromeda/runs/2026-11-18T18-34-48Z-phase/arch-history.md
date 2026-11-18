@history_item n=1 marker=2026-09-28-capability-ledger-and-viola-verify
**2026-09-28-capability-ledger-and-viola-verify** (the landed ledger rows, `viola verify`'s probe, the stamps envelope) — founded the closed ledger, the stamps envelope (one writer, merge keeps unknown versions) and `--record`, which writes scrubbed fixtures only at 0 fail; registered `schemas/claude-fixture.v1.json` and the first recorded set 2.1.283. The base this chunk's 2.1.325 set extends: a whole set comes only from a clean `--record`.

@history_item n=2 marker=2026-09-28-capability-ledger-and-viola-verify
**2026-09-28-capability-ledger-and-viola-verify** (the hidden `hook --capture` arm) — set up the probe capture arm (`<PascalEvent>.<k>.json`, no `VIOLA_*` read, exit 0, empty streams). Every hook payload this set records goes through it, the StopFailure capture (C2) included.

@history_item n=3 marker=2026-09-29-verify-stamped-test-homes-and-harness
**2026-09-29-verify-stamped-test-homes-and-harness** (verify-stamped homes and the harness CI read) — test homes are stamped by `viola verify` against the fake agent, root homes by `stamped_home` at a recorded version, and `run --local-live` is the one real-`claude` verify home; `CI` refuses `live-in-ci`. This is how the drift suites and `STAMPED` reach a new set, and it keeps the live record local.

@history_item n=4 marker=2026-10-04-dialog-answers-by-dialog-id
**2026-10-04-dialog-answers-by-dialog-id** (decisions on the six-row stamp, the strict stamps read, the 2.1.287 fixtures) — `run`'s gate reads stamps through `read_stamps_strict`; the 2.1.287 set began as four spine fixtures plus relayed dialog fixtures, later superseded. Precedent: a set that is not wholly recorded is labelled as such, while this chunk's set must be landed whole.

@history_item n=5 marker=2026-10-05-real-cli-verify-probes
**2026-10-05-real-cli-verify-probes** (screen signatures compiled, ten rows, verify's interactive runs) — compiled `SIGNATURES` and added Runs A and B under "never type into a CLI dialog". Signature-only `--record` screens: the `Screen.*` files this chunk records.

@history_item n=6 marker=2026-10-05-real-cli-verify-probes
**2026-10-05-real-cli-verify-probes** (registry rows for the interactive runs and screen fixtures) — added `schemas/claude-screen.v1.json` and `Screen.<phase>.json`. The `--record` refusal names the file and the closed code, never the content (the unclean-fixture rows). Runs B–D leave transcripts as an accepted residual.

@history_item n=7 marker=2026-10-05-dialog-rows-and-re-probe
**2026-10-05-dialog-rows-and-re-probe** (fourteen rows, Runs C and D) — added Runs C and D and the `<Event>.<stem>-<n>.json` dialog variants that `--record` writes. These are part of the set a clean 2.1.325 recording must hold whole.

@history_item n=8 marker=2026-10-05-dialog-rows-and-re-probe
**2026-10-05-dialog-rows-and-re-probe** (registry rows for Runs C and D and the dialog variants) — the variant class (12 per version), the stamped sets and the `measured.dialog_probe` field. Their counts are what `ls fixtures/claude/<v> | wc -l` rules read for a new set.

@history_item n=9 marker=2026-10-06-local-command-and-paste-framing-rows
**2026-10-06-local-command-and-paste-framing-rows** (seventeen rows, the paste-frame unwrap, Run B's pastes) — Run B's compiled pastes and the framing rows. A stamp of older ids reads unverified. Precedent for every row-count bump this set's re-stamp meets.

@history_item n=10 marker=2026-10-06-local-command-and-paste-framing-rows
**2026-10-06-local-command-and-paste-framing-rows** (registry rows for the framing variants and the committed sets) — `--record` writes the `paste-<n>` / `clear-<n>` framing variants. 2.1.288 went drift-only because it was not re-recorded. Trap: a set that cannot stamp every relied row leaves `STAMPED`.

@history_item n=11 marker=2026-10-06-local-command-send-outcomes
**2026-10-06-local-command-send-outcomes** (send consumes the local-command list) — local commands with post-condition "none" are `unconfirmable`, and `/clear` is confirmed by its post-condition. This is the precedent for C4's live `/context` post-condition reading.

@history_item n=12 marker=2026-10-07-live-rows-and-paste-shapes-on-the-dev-host
**2026-10-07-live-rows-and-paste-shapes-on-the-dev-host** (the long-paste frame beside typed text) — the founder ruled `decision/no-probe-for-unrelied-shapes`: a shape no send relies on needs no probe. That limit applies when C2 settles StopFailure as a row or keeps the exception.

@history_item n=13 marker=2026-10-07-send-waits-out-the-paste-hint
**2026-10-07-send-waits-out-the-paste-hint** — `GATE_MAX_WAIT` became 8.5 s for the 8.0 s paste hint (`decision/gate-timings-compiled`, `decision/send-waits-out-paste-hint`), validated per version by the quiet-period row. A new set re-validates these per version.

@history_item n=14 marker=2026-10-13-interrupted-verify-cleanup
**2026-10-13-interrupted-verify-cleanup** — verify's probe dirs are removed on pass, fail or Ctrl-C/TERM, and only what that run created (`decision/verify-probe-dirs-drop-guarded`). Bounds what a live `--record` round leaves behind.

@history_item n=15 marker=2026-10-13-paste-newline-ledger-row
**2026-10-13-paste-newline-ledger-row** — the 18th row added Run B's fifth paste, and every home stamped earlier read `cli_verified:false` until re-verified. This is the same mechanism behind C3/C8: a row added since a stamp makes the live home re-stamp owed.

@history_item n=16 marker=2026-10-14-resume-and-compact-ledger-rows
**2026-10-14-resume-and-compact-ledger-rows** (the resume row, Run E and /compact listed) — Run E takes Run B's first SessionStart id through `ledger::resume_target` and spawns nothing without it (`decision/verify-run-e-resumes-run-b-first-session`). This is the mechanism behind C10's hypothesis that Run E started nothing.

@history_item n=17 marker=2026-10-14-resume-and-compact-ledger-rows
**2026-10-14-resume-and-compact-ledger-rows** (Run E's resources) — `--record` writes `SessionStart.resume-1.json`, which the fake agent's `--resume` replays. It is one file of the whole set; a killed PTY child at a stop logs `child_exit_status`.

@history_item n=18 marker=2026-10-16-unwrapped-sessions-on-the-board
**2026-10-16-unwrapped-sessions-on-the-board** (the agents-join row and Run B's agents read) — added Run B's one `agents --json` read and `Agents.recorded-1.json`. A set gains the agents variant; older stamps read unverified.

@history_item n=19 marker=2026-10-16-unwrapped-sessions-on-the-board
**2026-10-16-unwrapped-sessions-on-the-board** (the fake agent's agents mode and the Agents fixture class) — added `schemas/claude-agents-fixture.v1.json` and replay from files beside the exe. Trap: the host's real cwd leaked a CI checkout path into `--record`, so the row replays recorded values only (recorder hygiene).

@history_item n=20 marker=2026-10-17-mcp-server-for-drivers
**2026-10-17-mcp-server-for-drivers** (the MCP tools' contract as landed) — `tools/list` carries `ttlMs` 0 and `cacheScope` private, without which claude drops every tool (`contract/mcp-list-results/*`); `viola-e2e` takes rmcp `client` alone. The `Mcp.tools-list-1.json` re-record (C1) is held to these.

@history_item n=21 marker=2026-10-18-compact-post-condition-ledger-row
**2026-10-18-compact-post-condition-ledger-row** (the 24th row, `local-command-compact`, Run B's sixth paste) — added `SessionStart.compact-<n>.json` and `measured.typed_probe.compact_latency_ms`, which validates the 40 s window (`decision/compact-confirm-window`, provisional). A new set re-reads the latency.

@history_item n=22 marker=2026-10-18-compact-post-condition-ledger-row
**2026-10-18-compact-post-condition-ledger-row** (Run B's three transcript files) — measured how many transcripts each run leaves (`decision/verify-transcripts-accepted-residual`). These are what a live `--record` leaves in the user's projects dir.

@history_item n=23 marker=2026-10-19-overseer-switch-to-viola
**2026-10-19-overseer-switch-to-viola** (the print-mode probe's empty project dir) — the founder accepted the print probe's empty `~/.claude/projects/` dir into the same residual class. This bounds a live recording round's leftovers.

@history_item n=24 marker=2026-10-19-newer-claude-cli-rows
**2026-10-19-newer-claude-cli-rows** (the second input-box literal) — 2.1.302 hid `for agents`, so `(shift+tab to cycle)` was added and the input-box-signature row reads three screens. Precedent: a newer CLI can break a signature that a stamp still passes.

@history_item n=25 marker=2026-10-19-newer-claude-cli-rows
**2026-10-19-newer-claude-cli-rows** (the 2.1.302 fixture set and the long-turn screen switch) — the last whole set: recorded by one `viola verify --record` round, the count rule `ls fixtures/claude/2.1.302 | wc -l`, added to `contract_ledger_probes`' `STAMPED`, with older sets drift-only. This is the direct template for the 2.1.325 set.

@history_item n=26 marker=2026-10-20-epoch-5-cleanup-ii
**2026-10-20-epoch-5-cleanup-ii** (the recorded live MCP exchange: the tap, the `Mcp.` class and its schema) — added `viola-mcp-tap` record/fixture, `schemas/claude-mcp-fixture.v1.json` and `mcp_fixtures_pass_hygiene`. `Mcp.*` is recorded by the tap, not by `--record`. This is C1's re-record path; a crossing the founder ratified.

@history_item n=27 marker=2026-10-20-epoch-5-cleanup-ii
**2026-10-20-epoch-5-cleanup-ii** (the measured 2.1.302 MCP opening and its recorded replay) — 2.1.302 opens with `server/discover` and no `initialize`; `tests/mcp_recorded_exchange.rs` holds the served result to the recorded key/type shape and tool names (`decision/mcp-tools-list-held-to-recorded-exchange`). The 2.1.325 re-record must satisfy this replay.

@history_item n=28 marker=2026-10-07-test-homes-off-the-contended-volume
**2026-10-07-test-homes-off-the-contended-volume** — on the dev host `target/e2e-home` links to a tmpfs (`decision/dev-host-e2e-home-on-tmpfs`), and only the keepers create or remove under it. The live rig's homes and C6's record-home path live there.

@history_item n=29 marker=2026-10-20-epoch-5-cleanup
**2026-10-20-epoch-5-cleanup** (the keepers' headroom floor) — a linked base is refused below 9 GiB free (`decision/e2e-home-headroom-floor`) after EDQUOT struck stamped-home gates. A live recording home is subject to it.

@history_item n=30 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (the home-tree check before the start's and verify's log) — verify refuses a widened home tree before any log (`decision/verify-refuses-untrusted-home-or-stamps`, `contract/verify-refusals/home-tree`). A live `--record` home must pass it.

@history_item n=31 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (the stamps writer's strict-modes check) — `update_stamps` checks strict modes before it creates `ledger/` and again under its lock (`decision/stamps-writer-checks-before-and-under-lock`). This governs C3/C8's live re-stamp.

@history_item n=32 marker=2026-10-27-pinned-copy-race-and-reproducible-builds
**2026-10-27-pinned-copy-race-and-reproducible-builds** (the pinned copy placed once, with no clobber) — the copy is placed once and re-hashed before reuse, and a mismatch refuses with `pinned-hash-mismatch`. Bears on C3/C8: the pinned product binary is not re-pinned by this chunk.

@history_item n=33 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** (`viola verify`'s refusal codes on its `process-exit`) — `unclean-fixture`, `version-unreadable` and the rest go on verify's `process-exit`, and the fixture's file enters no code (`contract/viola-verify-refusal-codes/*`). This is the refusal surface of an unclean `--record`.

@history_item n=34 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (the relied set: every row on every build) — `LedgerRow::relied()` became every row, `[NN/24]`, with no per-OS stamps. This chunk's set stamps against the one relied set.

@history_item n=35 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (the R8 strip: exact names, `claude_env_keep` the one pass-list) — verify's children are stripped of every inherited `CLAUDE*` name not kept (`decision/r8-identity-floor`, `decision/child-env-claude-strip`). The rig's private config and `CLAUDE_CONFIG_DIR` keep rest on it.

@history_item n=36 marker=2026-11-06-version-done-check
**2026-11-06-version-done-check** (the statusline-rate-limits row read the account's plan state) — a verify on an unchanged CLI failed a row because of the account, not the CLI. Precedent for C9: a host-state reading, the browser-tools prompt, decides a stamp.

@history_item n=37 marker=2026-11-06-version-done-check
**2026-11-06-version-done-check** (Measurement B known, no action in 0.1.0) — one tree can build two files and key two pinned copies, and the re-hash is the guard. Bears on C3/C8's caveat about not rebuilding or re-pinning the product.

@history_item n=38 marker=2026-11-13-verify-readiness-on-the-input-box
**2026-11-13-verify-readiness-on-the-input-box** (Established Decisions) — `decision/screen-signatures-compiled` reads the box's own rule/prompt rows beside footer literals, and `decision/verify-box-wait-for-literal` waits for the box. These are the readiness rules C7's `holds-text` box reading tests.

@history_item n=39 marker=2026-11-13-verify-readiness-on-the-input-box
**2026-11-13-verify-readiness-on-the-input-box** (Standard Contracts) — `measured.typed_probe` gained `box_only_guards` (`contract/ledger-stamps/measured`). A box-only screen read in this set's record (C1 CARRY 2) is counted there.

@history_item n=40 marker=2026-11-13-verify-readiness-on-the-input-box
**2026-11-13-verify-readiness-on-the-input-box** (Occupied Resources) — `decision/screen-fixtures-signature-rows-only`: box rows are kept as their rule and prompt chars alone, and a live prompt row holding a home path, username or email refuses the recording. Recorder-hygiene trap for C1/C7.

@history_item n=41 marker=2026-11-14-budget-gate-removed
**2026-11-14-budget-gate-removed** (Cross-cutting Patterns) — `decision/mixed-version-tolerance`: a stamp holding a retired ledger row verifies on the relied rows alone. Older committed sets keep reading under this rule beside the new set.

@history_item n=42 marker=2026-11-14-budget-gate-removed
**2026-11-14-budget-gate-removed** (Conventions, 1 of 2) — the MCP `list` payload lost its `budget` member (`contract/mcp-tool-results/success`). Explains C1's reading that the `list` description no longer says «and the usage budget», which the re-recorded `Mcp.tools-list-1.json` must reflect.

@history_item n=43 marker=2026-11-14-status-line-interception-removed
**2026-11-14-status-line-interception-removed** (Established Decisions, 1 of 2) — the statusline rows and the second capture arm left (`decision/hooks-fail-open` names one `--capture` arm). The relied rows fell to 21, so a new set records no `StatusLine.*` variants.

@history_item n=44 marker=2026-11-14-status-line-interception-removed
**2026-11-14-status-line-interception-removed** (Occupied Resources, 1 of 2) — removed `decision/fixture-keeps-own-repository-remote` with the StatusLine fixtures, and left `--settings` as a flag verify alone reads (`decision/claude-flag-literals-in-agent-crate`). Hygiene: no `workspace.repo` exception remains for a recorded fixture.

@history_item n=45 marker=2026-11-15-api-error-turn-end
**2026-11-15-api-error-turn-end** (Established Decisions) — StopFailure joined the sync spine (`contract/hook-tiers/*`) and became the named exception to `decision/cli-capability-ledger`: no ledger row, no verify probe. C2 settles this exception.

@history_item n=46 marker=2026-11-15-api-error-turn-end
**2026-11-15-api-error-turn-end** (Cross-cutting Patterns) — `decision/capability-ledger-gate` names the StopFailure exception beside the "no probe for an unrelied shape" limit. A new row with its probe, or the kept exception, edits this row.

@history_item n=47 marker=2026-11-15-api-error-turn-end
**2026-11-15-api-error-turn-end** (Design Philosophy) — `decision/capability-ledger` gained the StopFailure exception. Its third site, which a settlement under C2 must move in step.

@history_item n=48 marker=2026-11-15-api-error-turn-end
**2026-11-15-api-error-turn-end** (Conventions) — added `contract/hook-to-kind-map/stopfailure` and `contract/turn-end-cause/cause-set`. These are the map a live StopFailure payload's `error` word is read against (C2).

@history_item n=49 marker=2026-11-16T06-05-33-wrap
**2026-11-16T06-05-33-wrap** (Design Philosophy, Cross-cutting Patterns and Established Decisions edits) — dropped "provisionally" from the StopFailure exception on the three ledger rows. The founder ratified the no-row registration.

@history_item n=50 marker=2026-11-16T06-05-33-wrap
**2026-11-16T06-05-33-wrap** (the StopFailure no-ledger-row answer ratified — Standard Contracts, Conventions, Cross-cutting, Design Philosophy, Established Decisions, Occupied Resources) — ratified the no-row answer and names StopFailure's live reading as owned by «The 2.1.325 recorded fixture set». This is the direct mandate for C2.

@history_item n=51 marker=2026-11-16-granular-api-error-cause
**2026-11-16-granular-api-error-cause** (Conventions, 2 rows) — `TurnEndCause` holds nine words, and an unknown StopFailure error word reads `api-error` (`decision/unparseable-external-values-unknown` except). C2 reads the live word against this map and against the "no `parse-rejected` unknown-value line" claim.

@history_item n=52 marker=2026-11-16-granular-api-error-cause
**2026-11-16-granular-api-error-cause** (Conventions, 1 row; Standard Contracts) — `contract/hook-to-kind-map/stopfailure` classifies the CLI's error word in the Claude adapter, and `contract/event-data/turn-ended-cause` carries one of the nine words. These are the live StopFailure payload's expected outputs.

@history_item n=53 marker=2026-11-16-scheduled-wake-count
**2026-11-16-scheduled-wake-count** (Conventions) — the StopFailure arm writes `scheduled_wakes` null. Completes the exact data a live StopFailure must produce (C2).

@history_item n=54 marker=2026-11-16-local-command-delivery
**2026-11-16-local-command-delivery** (Established Decisions, 2 of 2) — added `decision/local-command-echo-confirmation` and `decision/local-command-echo-probe`: Run B's seventh paste `/mcp disable viola-probe-absent`, which fails with no readable transcript. This is the 22nd row whose stamps read unverified (C3); C4 asks whether that paste writes to the user's real config.

@history_item n=55 marker=2026-11-16-local-command-delivery
**2026-11-16-local-command-delivery** (Established Decisions, 1 of 2) — `decision/local-commands-by-ledger` matches by name plus argument, and `decision/every-send-confirmed` / `contract/send-results/unconfirmable` cover stdout-echo commands. These frame C4's unmeasured `/context` post-condition.

@history_item n=56 marker=2026-11-16-local-command-delivery
**2026-11-16-local-command-delivery** (Design Philosophy; Cross-cutting Patterns; Conventions; Standard Contracts) — the unconfirmable exception was widened on `decision/send-confirmed-after-the-fact`, `decision/unconfirmed-delivery-not-delivered` and `contract/ok-payload/unconfirmable`, and `hook.event` gained `transcript?` (`contract/hook-event/transcript`). These rows move if C4 measures `/context`'s post-condition.

@history_item n=57 marker=2026-11-17T05-21-35Z-wrap
**2026-11-17T05-21-35Z-wrap** (the delegate's confirm ratified — local-command-delivery, granular-api-error-cause, scheduled-wake-count) — the founder ratified the echo row, the nine causes and the scheduled-wake rows. Every row C2–C4 read against now stands on his word.

@history_item n=58 marker=2026-11-17-standing-live-rig
**2026-11-17-standing-live-rig** (Occupied Resources) — registered the test-only `viola-live-rig` and `AGENT_RUN_RIG`, which viola never reads. This chunk's live `--record` (C5) runs on the standing rig.

@history_item n=59 marker=2026-11-17-standing-live-rig
**2026-11-17-standing-live-rig** (Established Decisions; Stack and Technologies) — `decision/submitted-form-prompt-match`'s trim was read live, and vt100 serves the rig's screen model (`decision/vt100-readiness-gate` except, `decision/vt100-screen-model`). This is the rig's screen reading that C7 relies on.

@history_item n=60 marker=2026-11-17-first-readings-on-the-standing-rig
**2026-11-17-first-readings-on-the-standing-rig** (Infrastructure Patterns; Established Decisions) — the rig's capture reads a UserPromptSubmit payload's key names and source outside `viola-agent-claude`, as a test-only exception (`decision/claude-shapes-in-one-crate`, `decision/claude-first-no-adapter-trait`). C5's R4 and C7's `holds-text` reading come from this chunk.

@history_item n=61 marker=2026-11-17-trust-record-ledger-row
**2026-11-17-trust-record-ledger-row** (Established Decisions, 4 rows) — Run F made six interactive runs (`decision/verify-child-set`); added `decision/verify-own-trust-record` and `decision/trust-record-ledger-row` (the 23rd row). A clean 2.1.325 `--record` drives all six runs and verify's trust write.

@history_item n=62 marker=2026-11-17-trust-record-ledger-row
**2026-11-17-trust-record-ledger-row** (Occupied Resources; Cross-cutting Patterns; Conventions) — runs execute A to F, verify removes its own trust record, and verify reads a kept `CLAUDE_CONFIG_DIR`'s value (`decision/env-strip-names-only`, `contract/config-json/claude-env-keep`, `decision/no-environment-configuration`). The rig's private config is located this way.

@history_item n=63 marker=2026-11-17-trust-record-ledger-row
**2026-11-17-trust-record-ledger-row** (Standard Contracts, both parts; Infrastructure Patterns; Inherited Defaults) — trust-record's stamp holds a trust-writer word when nothing was written (`contract/ledger-stamps/rows|content`, `contract/trust-writer-words/*`); the fake agent and rig read trust records. A recorded set's stamp and the fake agent's replay must carry these.

@history_item n=64 marker=2026-11-17-trust-record-ledger-row
**2026-11-17-trust-record-ledger-row** (Established Decisions, the trust-record rule reworded; the PermissionRequest row count) — the row asks per version whether a record opens the folder, and its one live reading on 2.1.325 read fail. A 2.1.325 stamp must now read it again.

@history_item n=65 marker=2026-11-18-trust-record-row-read-live
**2026-11-18-trust-record-row-read-live** (Established Decisions; Occupied Resources) — added the hidden rig-only `--record-first-screens`, which writes whole scrubbed first screens on a failed `--record`, kept local (`decision/verify-record-only-clean`, `decision/recorded-fixtures-scrubbed`, `decision/screen-fixtures-signature-rows-only` excepts, `contract/viola-cli-arguments/verify`). A failed attempt at this chunk's recording falls under this exception; such screens are never committed.

@history_item n=66 marker=2026-11-18-trust-record-row-read-live
**2026-11-18-trust-record-row-read-live** (Stack and Technologies; Infrastructure Patterns; Inherited Defaults; Standard Contracts) — `decision/run-never-reads-screen-content` names the first-screen record; the rig seeds its private Claude config; stamps gained `measured.first_screen` / `first_settle_ms`. A clean 2.1.325 stamp carries these readings; C10 reads Run E's first screen from them.

@history_item n=67 marker=2026-11-18-session-folder-trusted-by-viola
**2026-11-18-session-folder-trusted-by-viola** (Established Decisions; Standard Contracts; Occupied Resources; Cross-cutting Patterns; Conventions) — on a verified CLI a start writes the trust record of an untrusted spawn folder (`decision/run-start-order`); `key-present` / `removed` are verify's words only; run and revive read a kept `CLAUDE_CONFIG_DIR`. A verified 2.1.325 stamp turns this start trust write on for that version.

@history_item n=68 marker=2026-11-18-browser-tools-prompt-modal
**2026-11-18-browser-tools-prompt-modal** (Established Decisions, 4 rows; Design Philosophy) — added `decision/browser-tools-prompt-ledger-row` (the 24th relied row: pass, absent or fail, absent counting toward verified); `pass|absent|fail` step lines and the summary count. C8/C9: the live home's 24-row verify and the 8/24 exposure when the prompt was never answered.

@history_item n=69 marker=2026-11-18-browser-tools-prompt-modal
**2026-11-18-browser-tools-prompt-modal** (Standard Contracts, both parts) — `decision/version-verified-all-relied-rows` and `contract/ledger-stamps/verified-version|rows|content` admit `absent` on browser-tools-prompt; `first_screen` gains `browser-tools`. The new set's stamp and `STAMPED` read through this counting rule. C11's `bt-absent` control is the guard on it.

@history_item n=70 marker=2026-11-18-browser-tools-prompt-modal
**2026-11-18-browser-tools-prompt-modal** (Established Decisions, Infrastructure Patterns and Inherited Defaults excepts) — the rig reads the `cachedChromeExtensionInstalled` key name, never a value, on the founder's word naming that key alone. Bounds C9: viola never answers or seeds the prompt, and the rig reads no more than that key.
