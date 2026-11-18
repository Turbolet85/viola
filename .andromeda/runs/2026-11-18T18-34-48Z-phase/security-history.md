@history_item n=1 marker=2026-09-28-capability-ledger-and-viola-verify
**2026-09-28-capability-ledger-and-viola-verify** (the hook --capture arm, probe captures and fixture scrub; MAX_FRAME consumers and verify's CI split) — added the hidden `hook <event> --capture` arm, verify's transient 0600 probe captures under `ledger/probes/<pid>/`, and the `verify --record` scrub: home → `~`, user word → `<user>`, and the whole recording refused when `/home/`, `/Users/`, a drive path or the user word survives. It also ruled that the real CLI and `--record` run only locally, with CI running verify only against the fake agent. This is the origin of the C6 record-home trap: a test home with a path component named `home` gets the whole set refused, and the scrub must never be patched by hand. It is also why the 2.1.325 set is recorded live on the dev host and replayed by the fake agent in CI (decision/real-cli-local-only, decision/fixture-scrub-and-refusal, decision/hook-capture-arm).

@history_item n=2 marker=2026-10-05-real-cli-verify-probes
**2026-10-05-real-cli-verify-probes** — added verify's interactive PTY children: direct spawns under the R8 strip at 80×24, output capped at MAX_FRAME, read only against compiled literals. No byte goes into any dialog, and a start showing a modal is killed with no key. Screen fixtures keep signature rows only. The unclean-fixture refusal became `a recorded fixture is not clean: <file> <code>` with the closed codes home-path · absolute-path · username · email. Two items were accepted: a transcript residual under `~/.claude/projects/`, and the precondition of a pre-trusted folder whose import has already been answered. These are the founder's rulings behind decision/verify-never-answers-dialogs, decision/verify-children-direct-spawn, contract/fixture-refusal-line/form and the four refusal-code contracts that the clean 2.1.325 recording must pass.

@history_item n=3 marker=2026-10-05-dialog-rows-and-re-probe
**2026-10-05-dialog-rows-and-re-probe** — the capture arm answers dialogs through the hidden `--answers` option (closed ProbeAnswer ids read through `take(64)`, body built only by `dialog::decision_body`). This added Run C (an `ask` rule, one `touch` in its own dir) and Run D (plan mode, `plansDirectory` inside its own dir). Verify's dialog probes answer only by hook output, never by a key, and their effects stay in their own dir (decision/capture-answers-closed-probe-ids, decision/verify-probes-confined-to-own-dir, decision/verify-writes-no-dialog-byte). Under these rows the new recording's Runs C/D captures are taken.

@history_item n=4 marker=2026-10-06-local-command-and-paste-framing-rows
**2026-10-06-local-command-and-paste-framing-rows** — Run B's added compiled pastes, each typed only into a settled input box with no modal literal, with a wait up to `PROBE_DEADLINE`. A stamp of the older id set reads `cli_verified:false`. This is the precedent that each new relied row makes every earlier stamp unverified until verify runs again. C3 and C8 rest on it: the 22- and 23-row stamps read unverified, and the 24-row live re-stamp is still owed.

@history_item n=5 marker=2026-10-13-interrupted-verify-cleanup
**2026-10-13-interrupted-verify-cleanup** — verify's probe dirs are removed on pass, fail, SIGINT or SIGTERM, each guard removing only what its own run created, never by pattern. SIGKILL or a crash leaves them to be removed by hand (decision/verify-probe-dirs-cleanup, decision/probe-artifacts-cleanup). A live `--record` that is stopped or killed during this chunk's starts falls under this rule. Its exception now also covers verify's own trust record (see 2026-11-17-trust-record-ledger-row).

@history_item n=6 marker=2026-10-14-resume-and-compact-ledger-rows
**2026-10-14-resume-and-compact-ledger-rows** — added Run E, which resumes only the session id of Run B's first SessionStart capture when that id has the session-id shape. With no such capture Run E spawns nothing and `resume-session` fails (decision/verify-run-e-resume-target, decision/session-id-argv-closed-shape). This is the mechanism behind the C10 hypothesis: on the browser-tools-prompt host Run B was killed on the modal and left no capture, so Run E could not start. A clean `--record` here shows whether Run E starts.

@history_item n=7 marker=2026-10-16-unwrapped-sessions-on-the-board
**2026-10-16-unwrapped-sessions-on-the-board** — added Run B's one bounded `claude agents --json` read (direct spawn under the R8 strip, MAX_FRAME-capped, killed at 5 s, parsed tolerantly into rows or a closed failure). `--record` keeps only Run B's own row in `Agents.recorded-1.json`. That file is part of the set this chunk records whole (decision/claude-agents-read-fail-open, decision/child-output-bounded).

@history_item n=8 marker=2026-10-18-compact-post-condition-ledger-row
**2026-10-18-compact-post-condition-ledger-row** — added Run B's `/compact` paste and accepted its three-file transcript residual per round. The stamp gate moved to the 24-row relied set. The recording's Run B sequence and its accepted CLI-side residual come from here.

@history_item n=9 marker=2026-10-19-overseer-switch-to-viola
**2026-10-19-overseer-switch-to-viola** (verify's print-mode probe residual accepted) — the print-mode probe leaves the CLI's empty project dir under `~/.claude/projects/`, accepted into the same residual class as the transcripts. This is a residual the live 2.1.325 verify will reproduce. It is the CLI's write, not viola's.

@history_item n=10 marker=2026-10-19-newer-claude-cli-rows
**2026-10-19-newer-claude-cli-rows** — accepted one scratch footer reading of a newer CLI: one start, no key, only scrubbed footer rows kept. That answer is no standing permission, and any further reading needs its own answer (decision/scratch-footer-reading-residual). Run B's pastes go into rows holding *an* input-box literal. This is the precedent that each live start on 2.1.325 needs the founder's cap word (C1, Boundaries).

@history_item n=11 marker=2026-10-20-epoch-5-cleanup-ii
**2026-10-20-epoch-5-cleanup-ii** (the `Mcp.` exchange recorded by the test-only tap) — `Mcp.` exchange fixtures are recorded by the test-only `viola-mcp-tap`. The tap sits between one live CLI start and a direct spawn of `viola mcp`, caps each line, and keeps its raw capture 0600 in a private dir that is never committed. It applies the same scrub and closed refusal codes, validated by `schemas/claude-mcp-fixture.v1.json`. Founder-ratified, with a cap of one start. This governs C1's live re-record of `Mcp.tools-list-1.json` on 2.1.325 (decision/mcp-fixtures-via-test-only-tap).

@history_item n=12 marker=2026-11-14-budget-gate-removed
**2026-11-14-budget-gate-removed** — retired the budget gate, budget.json and the release `budget` param. decision/config-cannot-disable-controls lost its budget clause, and decision/viola-format-fixtures-hygiene was added (viola-format fixtures under the hygiene walk). This explains why C1's re-recorded `list` tool description must read without «and the usage budget». The committed `Mcp.tools-list-1.json` predates this removal.

@history_item n=13 marker=2026-10-17-mcp-server-for-drivers
**2026-10-17-mcp-server-for-drivers** (MCP tool input validation; the mcp process spawns claude agents --json) — the MCP `list` tool (contract/mcp-tools/list) reads the board through CLI list's path with one bounded agents read. Handlers validate raw arguments themselves because rmcp does not enforce its schemas (decision/mcp-handlers-check-raw-args, decision/mcp-handlers-validate-raw-arguments). The `tools/list` exchange the tap re-records publishes these advisory schemas and descriptions.

@history_item n=14 marker=2026-10-27-bounded-inputs-at-every-boundary
**2026-10-27-bounded-inputs-at-every-boundary** — `viola mcp`'s stdin lines are capped at MAX_FRAME by `CappedLines`, and an over-long line ends the session whole (decision/mcp-stdin-line-cap, decision/mcp-stdin-line-cap-on-reader). The bounded-reader census was added (decision/bounded-readers-census). The tap's live exchange runs through this reader, and any new product reader added for recording must be bounded or named in the census.

@history_item n=15 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (`ledger/stamps.json` checked before `ledger/` is made; `verify`'s home tree before its log) — `update_stamps` runs the strict-modes check before creating `ledger/` and again under its lock. Verify checks the home, ledger, bin and plugin dirs before its cli log, and a refusal is exit 1 with no step line. The new ban: never run the check after a create, a mode set or a log open that narrows the path (decision/verify-home-tree-check, decision/stamps-strict-check-first, decision/strict-check-before-narrowing). The live home's 24-row re-stamp (C3, C8) writes through this path.

@history_item n=16 marker=2026-10-04-dialog-answers-by-dialog-id
**2026-10-04-dialog-answers-by-dialog-id** (run's strict stamps read; decisions on the six-row stamp) — run's version gate reads stamps through the strict check, and a refusal or malformed file gives `cli_verified:false`. A non-null decision flows only on a verify stamp. This is the origin of decision/version-gate-strict-stamps-read and contract/strict-modes-refusal/version-gate: an unverified 2.1.325 stamp keeps every answer null (C8: a binary built from the tree runs transport-only).

@history_item n=17 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** (verify's refusal codes) — verify writes a closed refusal code on its process-exit (pinned-hash-mismatch, strict-modes-failed, unclean-fixture, …). The fixture's `<file>` enters no code, and a failing row is no refusal. These are the contracts contract/verify-refusal-codes/* that an unclean recording on 2.1.325 would read.

@history_item n=18 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (the relied set on every build; the fixture scrub matches names exactly; batch-script-child retired) — `LedgerRow::relied()` is the whole set on every build. The fixture scrub matches the home prefix exactly, with no case fold. Verify's refusal codes lost `batch-script-child`. The relied-row count and the scrub the new set passes are these Linux-only forms.

@history_item n=19 marker=2026-11-14-status-line-interception-removed
**2026-11-14-status-line-interception-removed** — removed the statusline capture arm (capture arms 2 → 1), Run B's `--settings` and the StatusLine fixtures. It retired the `workspace.repo` keep exception from decision/fixture-scrub-and-refusal and decision/fixture-hygiene, and narrowed decision/capture-arms-isolated and decision/hook-silent-exit-zero to one arm. A new recording must hold no StatusLine file, and a `workspace.repo` value no longer has a kept exception.

@history_item n=20 marker=2026-11-13-verify-readiness-on-the-input-box
**2026-11-13-verify-readiness-on-the-input-box** — decision/screen-fixture-signature-rows-only now keeps box rule rows as rule chars and prompt rows as the prompt char. Another box row is refused. decision/input-box-readiness-from-literals was added: readiness comes only from footer or rule-row literals, never while a modal stands. The new set's screen fixtures, and C7's `box-text: holds-text` placeholder-hint question, rest on this predicate and form.

@history_item n=21 marker=2026-11-14-hyprland-seam
**2026-11-14-hyprland-seam** (Threat Model Summary) — decision/child-output-tolerant and decision/child-output-tolerant-unknown changed so that a child-output failure becomes unknown or a closed failure word. This is the current form of the tolerant child-output rows the recording's agents and print reads follow.

@history_item n=22 marker=2026-11-15-live-round-on-the-trimmed-product
**2026-11-15-live-round-on-the-trimmed-product** — added decision/live-round-rig-keys-own-pty (rig keys only into its own pty, the founder's word per chunk) and decision/live-session-screens-kept-local (a live session's screens stay under a gitignored local path, never in a work dir, run dir or commit). Every live start of this chunk and its evidence fall under both. Its C1 sources come from this chunk's inputs.

@history_item n=23 marker=2026-10-18-plugin-precedence-live-readings
**2026-10-18-plugin-precedence-live-readings** (the live round's rig outside the work tree, its trust keys and residual) — this is the precedent for a live rig placed outside the work tree, after an in-tree start met the external-imports modal. It passes folder trust only by keys into its own pty, at most two arrows and one Enter on a read selected row. The residual is the CLI's own writes under `~/.claude`. Live `--record` starts face the same placement and import-dialog trap.

@history_item n=24 marker=2026-11-15-api-error-turn-end
**2026-11-15-api-error-turn-end** — registered StopFailure with no ledger row and no verify probe. It added the exception to decision/unverified-cli-gate «owned by the 2.1.325 recorded fixture set», which is C2's PROVISIONAL exception this chunk must settle. It also added decision/api-error-turn-end-text-dropped and decision/api-error-classes-adapter-only, which drop the StopFailure error text at the parse. A recorded live StopFailure payload must cross only as the cause word.

@history_item n=25 marker=2026-11-16T06-05-33-wrap
**2026-11-16T06-05-33-wrap** — the founder ratified «no ledger row» for StopFailure. The unverified-cli-gate exception is no longer provisional, and its live reading is owned by «The 2.1.325 recorded fixture set». The one-word `api-error` cause was superseded by the granular cause. Settling C2 now means either a ledger row with its probe or the exception kept on the founder's word. Dropping the exception silently is not an option.

@history_item n=26 marker=2026-11-16-granular-api-error-cause
**2026-11-16-granular-api-error-cause** — the turn-end cause set grew to nine closed words (contract/channel-frames/hook-event-revalidation, decision/api-error-classes-adapter-only, decision/api-error-turn-end-text-dropped). Only the cause word leaves the adapter, never the error word. A live StopFailure's `error` is read against this nine-word map (C2: the CLI's 13 words must each map, with no `parse-rejected` `unknown-value`). Ratified 2026-11-17.

@history_item n=27 marker=2026-11-16-scheduled-wake-count
**2026-11-16-scheduled-wake-count** — added decision/stop-session-crons-count-only: Stop's `session_crons` is carried as a count only. An API-error end writes `scheduled_wakes` null. A recorded live Stop or StopFailure payload in the new set crosses under these count-only rules. Ratified 2026-11-17.

@history_item n=28 marker=2026-11-16-local-command-delivery
**2026-11-16-local-command-delivery** — added decision/verify-echo-probe-residuals: Run B's seventh paste, `/mcp disable viola-probe-absent`, is accepted, but whether it writes to the user's real Claude config was left unmeasured. That is C4's carry, measured live beside this recording. It also added decision/transcript-path-shape-check, decision/transcript-read-bounded and decision/transcript-path-redacted. The SessionStart transcript path appears in no verify record. It also changed decision/compiled-classification-sources. Ratified 2026-11-17.

@history_item n=29 marker=2026-11-17-standing-live-rig
**2026-11-17-standing-live-rig** — the standing rig's closed argv key grant, decision/live-rig-credentials-copy (OAuth copy transient, under 20 minutes of token refused, nothing recorded) and decision/live-rig-isolated-child (private `CLAUDE_CONFIG_DIR`, a closed kept env list). The C5 `viola verify --record` on the standing rig runs inside these rows.

@history_item n=30 marker=2026-11-17-first-readings-on-the-standing-rig
**2026-11-17-first-readings-on-the-standing-rig** — the rig's stop (Ctrl-C or a no-key group kill) needs no word, while its argv grant still needs the founder's word per chunk. The user's `~/.claude.json` witness changed to a needle search after a moved stamp. decision/live-rig-prompt-capture-closed was added. This chunk's R4 / C5 / C7 carries come from this chunk's readings.

@history_item n=31 marker=2026-11-17-trust-record-ledger-row
**2026-11-17-trust-record-ledger-row** — verify writes a trust record for its own temp folder and removes only that record. A SIGKILL leaves it (decision/no-writes-under-claude-home, decision/viola-writes-nothing-in-claude-dir and decision/nothing-written-under-claude-home exceptions). It added decision/trust-record-writer-guarded and decision/trust-record-closed-words. It added the exception that `CLAUDE_CONFIG_DIR`'s value is read only when kept (decision/r8-claude-env-strip, decision/credential-strip-by-name-only, decision/env-not-config-channel, decision/no-stored-secrets). It also extended the probe-cleanup exceptions to the trust record. The live `--record` writes and removes this record in the rig's private config.

@history_item n=32 marker=2026-11-18-trust-record-row-read-live
**2026-11-18-trust-record-row-read-live** — added the hidden rig-only first-screen record of whole scrubbed rows on a failed recording verify, kept local and git-ignored, an unclean set refused whole. That is the exception on decision/screen-fixture-signature-rows-only, decision/verify-signature-only and decision/verify-screen-read-against-literals. It added decision/verify-stamp-first-screen-closed-words and decision/live-rig-seed-from-bundle-logic. A clean `--record` must not commit those first-screen files. The trust-record row was read live on 2.1.325 under the rig's private config only (C8).

@history_item n=33 marker=2026-11-18-session-folder-trusted-by-viola
**2026-11-18-session-folder-trusted-by-viola** — added decision/start-writes-trust-record-flag-only: run and revive's start set `hasTrustDialogAccepted` for an untrusted spawn folder, and the record stays. The guarded writer is shared with verify. The cwd rows now name the trust-record key (decision/cwd-only-in-snapshot, decision/spawn-cwd-snapshot-only). The `CLAUDE_CONFIG_DIR` exceptions extend to the start. The rig seed rule was narrowed. A live start on the user's real home writes into his real Claude config, which is his to allow (Boundaries).

@history_item n=34 marker=2026-11-18-browser-tools-prompt-modal
**2026-11-18-browser-tools-prompt-modal** — decision/unverified-cli-gate, decision/dialog-answers-need-verify-stamp and contract/cli-exit-codes/12 now count a relied row as pass, or absent on the browser-tools-prompt row alone. The rig's VP arm leaves the browser-tools key out, and decision/live-rig-cache-key-name-read was added (key name only, never a value). This is the 24th relied row behind C8's re-stamp and C9's exposure: an unanswered prompt reads 8/24, and viola never answers or seeds it. Its `bt-absent` remove-the-guard control is C11's indeterminate reading. All its rows are PROVISIONAL on the delegate's confirm.
