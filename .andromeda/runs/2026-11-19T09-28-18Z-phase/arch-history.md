@history_item n=1 marker=2026-09-28-capability-ledger-and-viola-verify
**2026-09-28-capability-ledger-and-viola-verify** (the six landed ledger rows, `viola verify`'s probe, the stamps envelope) — introduced `viola verify --record`, which writes scrubbed `<Event>.default.json` fixtures only at 0 fail, the `schemas/claude-fixture.v1.json` shape and the first recorded set (2.1.283), and the stamps envelope with its merge rule. This is the root of the 2.1.328 recording: a recording that is not clean is refused whole, and the stamps merge replaces only its own version's entry.

@history_item n=2 marker=2026-09-29-verify-stamped-test-homes-and-harness
**2026-09-29-verify-stamped-test-homes-and-harness** (verify's spawn pairs, the harness CI read, verify-stamped homes) — root rstest homes are stamped by `stamped_home` at a recorded version, harness homes by `viola verify` against the fake agent, and `run --local-live` is refused under `CI` (`live-in-ci`). The real CLI never runs in CI, so the 2.1.328 set reaches CI only through the fake agent's replay. Any `stamped_home` pin that this chunk moves follows this rule.

@history_item n=3 marker=2026-10-04-dialog-answers-by-dialog-id
**2026-10-04-dialog-answers-by-dialog-id** (decisions on the six-row stamp, the strict stamps read, the 2.1.287 fixtures) — `run`'s version gate reads stamps through `read_stamps_strict`. The 2.1.287 set mixed recorded spine fixtures with relayed dialog fixtures (`RELAYED.md`), later superseded. Precedent: a set is landed as recorded or explicitly marked otherwise. The scope's "plain copy, none hand-edited" rule continues this.

@history_item n=4 marker=2026-10-05-real-cli-verify-probes
**2026-10-05-real-cli-verify-probes** (registry rows for verify's interactive runs and screen fixtures) — added `schemas/claude-screen.v1.json` and the `Screen.<phase>.json` signature-only screen fixtures. The `--record` refusal was changed to name the file and the closed code, never the content. 2.1.283 became drift-only. This is the screen-fixture class that the 2.1.328 recording writes and that the recorded-screen suites replay.

@history_item n=5 marker=2026-10-05-dialog-rows-and-re-probe
**2026-10-05-dialog-rows-and-re-probe** (registry rows for Runs C and D, the dialog variants and the answers flag) — added the `<Event>.<stem>-<n>.json` dialog-variant class (12 per version), recorded by Runs C/D, and the fake agent's `--dialogs` mode. The 2.1.328 set must carry this class whole for the fake agent's dialog replay.

@history_item n=6 marker=2026-10-06-local-command-and-paste-framing-rows
**2026-10-06-local-command-and-paste-framing-rows** (registry rows for the framing variants, the fake agent's two options and the committed sets) — `--record` writes Run B's framing captures (`UserPromptSubmit.paste-<n>`, `SessionEnd/SessionStart.clear-<n>`). A set without them (2.1.288, never re-recorded) dropped to drift-only. Trap: a set that is not re-recorded whole can no longer stamp every row. This is why the 2.1.328 set must land whole.

@history_item n=7 marker=2026-10-06-local-command-send-outcomes
**2026-10-06-local-command-send-outcomes** (the paste-hint reading, the fake agent's eighth option) — added `--tag-turn-screen <phase>`, which draws `Screen.<phase>` of the set after the tag-like turn, a missing file drawing nothing. This bears on the scope's P3 question of which fake-agent behaviour is keyed to a set's files, and therefore to a version.

@history_item n=8 marker=2026-10-13-paste-newline-ledger-row
**2026-10-13-paste-newline-ledger-row** (eighteen ledger rows, `paste-newline`, Run B's fifth paste) — `PROBE_NEWLINE_PASTE` became Run B's fifth paste. The 2.1.287 set gained a fifth framing variant (`paste-3`), replayed by `--framing` through `framing_stem`. A stamp of older ids reads unverified until re-verified. The 2.1.328 set's framing variants must match the current paste list.

@history_item n=9 marker=2026-10-14-resume-and-compact-ledger-rows
**2026-10-14-resume-and-compact-ledger-rows** (Run E's resources and a killed PTY child's exit status) — `--record` writes `SessionStart.resume-1.json` (`resume_variants`). The fake agent's `--resume` builds its launch SessionStart on the set's resume variant when the set holds it (`launch_variant`). This is a fake-agent behaviour keyed to what a version's set holds.

@history_item n=10 marker=2026-10-16-unwrapped-sessions-on-the-board
**2026-10-16-unwrapped-sessions-on-the-board** (the fake agent's agents mode, the Agents fixture class) — `Agents.recorded-1.json` keeps Run B's own row only, validated by `schemas/claude-agents-fixture.v1.json` and replayed by the fake agent's `agents` mode. The row replays recorded values only because the host's real cwd leaked a CI checkout path into `--record`. Hygiene trap for the new set.

@history_item n=11 marker=2026-10-17-mcp-server-for-drivers
**2026-10-17-mcp-server-for-drivers** (the MCP tools' contract as landed) — `tools/list` carries `ttlMs` 0 and `cacheScope` `private`, without which claude drops every tool. These contract rows (`mcp-list-results/*`) are what the `Mcp.` recorded-exchange replay holds the served result to on each set, 2.1.328's included.

@history_item n=12 marker=2026-10-18-compact-post-condition-ledger-row
**2026-10-18-compact-post-condition-ledger-row** (the twenty-fourth ledger row, local-command-compact, and Run B's sixth paste) — Run B gained a `/compact` paste. `--record` writes `SessionStart.compact-<n>.json`, and `FRAMING_TURNS` gained `compact-1`, which the fake agent's `--framing` replays. The 2.1.328 set must hold this variant for the framing replay.

@history_item n=13 marker=2026-10-18-compact-post-condition-ledger-row
**2026-10-18-compact-post-condition-ledger-row** (CARRY 2's readings to their own entry; Run B's three transcript files) — Run B leaves three transcript files per real-CLI verify, and C and D one each. This is the accepted transcript residual (`verify-transcripts-accepted-residual`). It also bears on C3: whether Run B's own readable transcript can carry `/context`'s stdout-tag reading.

@history_item n=14 marker=2026-10-19-overseer-switch-to-viola
**2026-10-19-overseer-switch-to-viola** (verify's print-mode probe leaves an empty CLI project dir) — the print probe's empty `~/.claude/projects/` dir joined the accepted residual. The scope keeps the live starts under the rig's private config, with nothing under the real home, so this residual class is what a by-path recording may leave.

@history_item n=15 marker=2026-10-19-newer-claude-cli-rows
**2026-10-19-newer-claude-cli-rows** (the second input-box literal and the three-screen input-box-signature row) — a newer CLI (2.1.302) hid `for agents`, which forced the `(shift+tab to cycle)` literal and a three-screen check. Direct precedent: a new CLI version can move a compiled literal, the risk the scope's P3 must read from the 2.1.328 bundle before a start.

@history_item n=16 marker=2026-10-19-newer-claude-cli-rows
**2026-10-19-newer-claude-cli-rows** (the 2.1.302 fixture set and the long-turn screen switch) — the template for a new version's set: recorded whole by one `--record` round, counted by `ls fixtures/claude/<v> | wc -l`, added to `contract_ledger_probes`' `STAMPED` list, and given a fake-agent screen switch (`--long-turn-screen`) where the version's screens needed one.

@history_item n=17 marker=2026-10-20-epoch-5-cleanup-ii
**2026-10-20-epoch-5-cleanup-ii** (the recorded live MCP exchange: the tap, the `Mcp.` class and its schema) — introduced `viola-mcp-tap` (`record` / `fixture`), the `Mcp.<variant>.json` class recorded by the tap rather than `--record`, `schemas/claude-mcp-fixture.v1.json`, `mcp_fixtures_pass_hygiene` and `tests/mcp_recorded_exchange.rs`. This is the `R4M` re-record this chunk repeats for 2.1.328.

@history_item n=18 marker=2026-10-20-epoch-5-cleanup-ii
**2026-10-20-epoch-5-cleanup-ii** (the measured 2.1.302 MCP opening and its recorded replay) — 2.1.302 opens with `server/discover` and then `tools/list`, with no `initialize`, which disproved the planned handshake. The replay takes the `tools/list` response by its request id. Trap: a new CLI version may change the MCP opening, so the 2.1.328 exchange's shape must be read, not assumed.

@history_item n=19 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (CI on the one `ubuntu-latest` leg) — CI runs on `ubuntu-latest` against the fake agent alone (`ci-ubuntu-fake-agent`). The drift and recorded-exchange suites over the new set run there.

@history_item n=20 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (the relied set: all rows on every build) — `LedgerRow::relied()` became every row of `ALL` on every build, so a stamped set is stamped at the whole relied set with no OS split.

@history_item n=21 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (the `.cmd`/`.bat` refusal `batch-script-child` retired) — verify's `Ended::Refused` codes became five, `unclean-fixture` among them, pinned in `tests/cross_exit_causes.rs`. This is the refusal an unclean 2.1.328 recording reads.

@history_item n=22 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** (`viola verify`'s refusal codes on its `process-exit`) — verify writes its refusal code (`unclean-fixture`, `version-unreadable`, `strict-modes-failed`, …) on `process-exit` under a parsed `VIOLA_NAME`. The fixture's file enters no code, a failing row is no refusal, and verify takes no `--json`. These are the `viola-verify-refusal-codes/*` rows this chunk's recording rests on.

@history_item n=23 marker=2026-11-13-verify-readiness-on-the-input-box
**2026-11-13-verify-readiness-on-the-input-box** (Established Decisions: 2 rows) — `screen-signatures-compiled` reads the box ready on its own rule and prompt rows beside the footer literals, and `verify-box-wait-for-literal` waits for that box. A 2.1.328 screen that moved its box or footer is read through these rules.

@history_item n=24 marker=2026-11-13-verify-readiness-on-the-input-box
**2026-11-13-verify-readiness-on-the-input-box** (Occupied Resources: 1 row; Standard Contracts: 1 row) — `screen-fixtures-signature-rows-only` keeps the box's rule row as its rule chars and its prompt row as the prompt char. A box whose live prompt row holds a home path, the username or an email refuses the recording. `measured` gained `box_only_guards`. This is the recorded-screen hygiene the new set passes through.

@history_item n=25 marker=2026-11-14-budget-gate-removed
**2026-11-14-budget-gate-removed** (Cross-cutting Patterns: 1 row) — `mixed-version-tolerance` states that a stamp holding a retired ledger row verifies on the relied rows alone. This bears on older sets' stamps beside the new one and on what `STAMPED` asserts.

@history_item n=26 marker=2026-11-14-status-line-interception-removed
**2026-11-14-status-line-interception-removed** (Occupied Resources: 10 rows, 1 of 2) — the StatusLine fixtures were deleted, `fixture-keeps-own-repository-remote` was retired (no fixture may hold `workspace.repo`), and the stamps lost two row ids with `v` kept at 1. The 2.1.328 recording must write no StatusLine variant and no `workspace.repo`.

@history_item n=27 marker=2026-11-15-api-error-turn-end
**2026-11-15-api-error-turn-end** (Cross-cutting Patterns: 1 row) — StopFailure was registered with no ledger row and no verify probe (the `capability-ledger*` exceptions in the rows file). C1 places this chunk directly ahead of «A live StopFailure payload», so StopFailure stays out of the recorded set's probe rows.

@history_item n=28 marker=2026-11-16T06-05-33-wrap
**2026-11-16T06-05-33-wrap** (the StopFailure no-ledger-row answer ratified by the founder — Cross-cutting Patterns) — ratified that StopFailure stands with no ledger row and named its live reading as owned by «The 2.1.325 recorded fixture set». This is the sibling chunk this one copies, and the owner chain C3 moved from.

@history_item n=29 marker=2026-11-16-local-command-delivery
**2026-11-16-local-command-delivery** (Established Decisions: 2 rows, 2 of 2) — added `local-command-echo-confirmation` (count of `<local-command-stdout>` tags in the session transcript) and `local-command-echo-probe` (Run B's seventh paste, `/mcp disable viola-probe-absent`; with no readable transcript the paste is skipped and the row fails). This is the mechanism C3's `/context` stdout-tag half would be read by.

@history_item n=30 marker=2026-11-16-local-command-delivery
**2026-11-16-local-command-delivery** (Established Decisions: 4 rows, 1 of 2) — `local-commands-by-ledger` classifies by name plus rest. `every-send-confirmed` and `send-results/unconfirmable` make a stdout-echo command with no readable transcript `unconfirmable`. This is the reading behind C3's L2 `not-delivered` with the transcript absent.

@history_item n=31 marker=2026-11-17T05-21-35Z-wrap
**2026-11-17T05-21-35Z-wrap** (the delegate's confirm ratified by the founder — 2026-11-16-local-command-delivery, Established Decisions, the echo confirmation and probe) — the echo-confirmation and echo-probe rows stand on the founder's own word. Any `/context` disposition under C3 extends ratified rows, not provisional ones.

@history_item n=32 marker=2026-11-17-standing-live-rig
**2026-11-17-standing-live-rig** (Occupied Resources: 4 rows; Stack and Technologies: 1 row) — registered `viola-live-rig` and `AGENT_RUN_RIG` (never read by viola), and vt100 as the rig's screen model. This is the rig whose labels `R4` / `R4M` drive this chunk's live starts, and the scope's P3 asks whether they need any change for 2.1.328.

@history_item n=33 marker=2026-11-17-first-readings-on-the-standing-rig
**2026-11-17-first-readings-on-the-standing-rig** (Infrastructure Patterns: 1 row) — first test-only exception to `claude-shapes-in-one-crate`: the rig reads its captured UserPromptSubmit key names and source word, never a value. This is the base of the exception the rig rows still carry.

@history_item n=34 marker=2026-11-17-trust-record-ledger-row
**2026-11-17-trust-record-ledger-row** (Established Decisions: 4 rows) — verify's child set became six runs (Run F). Added `verify-own-trust-record` (verify writes and removes its own record, never creating an absent config) and `trust-record-ledger-row`. 2.1.328 adds a decline text to the trust dialog, so Run A/F's trust screens are where it could surface.

@history_item n=35 marker=2026-11-17-trust-record-ledger-row
**2026-11-17-trust-record-ledger-row** (Occupied Resources: 4 rows, 1 of 2) — Runs A to F execute in order (`verify-run-e-resumes-run-b-first-session`). Verify removes its own trust record with its probe dirs. Verify reads `CLAUDE_CONFIG_DIR`'s value only when `claude_env_keep` keeps it. The rig's private-config recording (R4) depends on that exception.

@history_item n=36 marker=2026-11-17-trust-record-ledger-row
**2026-11-17-trust-record-ledger-row** (Standard Contracts: 9 rows, 1 of 2) — a stamp's `trust-record` value may be the trust writer's code word, and the closed `trust-writer-words/*` set was registered. A 2.1.328 stamp's trust-record cell reads within this set.

@history_item n=37 marker=2026-11-18-trust-record-row-read-live
**2026-11-18-trust-record-row-read-live** (Established Decisions: 2 rows; Occupied Resources: 3 rows) — added the hidden rig-only `--record-first-screens` (whole scrubbed first screens on a failed `--record` verify, kept locally) as the one exception to `verify-record-only-clean`, `recorded-fixtures-scrubbed` and `screen-fixtures-signature-rows-only`. Also the rig's private-config seeding exception. This is the fallback if R4 fails to record clean.

@history_item n=38 marker=2026-11-18-trust-record-row-read-live
**2026-11-18-trust-record-row-read-live** (Standard Contracts: 2 rows) — `measured.first_screen` (a closed word per run) and `measured.first_settle_ms` were added to the stamp. The 2.1.328 stamp written by R4 carries them.

@history_item n=39 marker=2026-11-18-session-folder-trusted-by-viola
**2026-11-18-session-folder-trusted-by-viola** (Established Decisions: 2 rows; Occupied Resources: 3 rows) — a wrapped start on a verified CLI writes the trust record of an untrusted spawn folder, and run/revive read `CLAUDE_CONFIG_DIR` when kept. `key-present` and `removed` are verify's words only. The trust lookup the scope reads as unchanged on 2.1.328 is what this rests on.

@history_item n=40 marker=2026-11-18-browser-tools-prompt-modal
**2026-11-18-browser-tools-prompt-modal** (Established Decisions: 4 rows) — added the 24th relied row `browser-tools-prompt` (pass / absent / fail, absent counting toward verified) and the `pass|absent|fail` step line and summary count. The scope reads the browser-tools prompt's key and confirm label as unchanged on 2.1.328, and the new set's stamp is read by these rules.

@history_item n=41 marker=2026-11-18-browser-tools-prompt-modal
**2026-11-18-browser-tools-prompt-modal** (Standard Contracts: 4 rows, 1 of 2; Design Philosophy: 1 row) — `version-verified-all-relied-rows`, `ledger-stamps/verified-version`, `ledger-stamps/rows`, `ledger-stamps/content` and `capability-ledger` now accept `absent` on browser-tools-prompt alone. This is what `STAMPED`'s verified check over 2.1.328 must hold.

@history_item n=42 marker=2026-11-18-browser-tools-prompt-modal
**2026-11-18-browser-tools-prompt-modal** (Standard Contracts: 1 row, 2 of 2) — `measured.first_screen` gained the word `browser-tools` (six words), so a 2.1.328 first screen showing that prompt reads within the closed set.

@history_item n=43 marker=2026-11-18-the-live-home-s-re-stamp-and-the-product-re-pin
**2026-11-18-the-live-home-s-re-stamp-and-the-product-re-pin** (Established Decisions / Infrastructure Patterns / Inherited Defaults: 1 row each) — the test-only exception on `claude-first-no-adapter-trait`, `claude-shapes-in-one-crate` and `claude-shapes-isolated` now names the VH arm's real-config key names and needle, PROVISIONAL (delegate). This is the immediately preceding chunk, which stamped 2.1.328 24/24 on the live home. Any rig read this chunk adds must stay inside, or extend, this exception.
