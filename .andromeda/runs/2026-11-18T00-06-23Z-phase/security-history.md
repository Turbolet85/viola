@history_item n=1 marker=2026-11-17-trust-record-ledger-row
**2026-11-17-trust-record-ledger-row** — Data Protection and Threat Model Summary (4 rows): gave `decision/no-writes-under-claude-home`, `decision/viola-writes-nothing-in-claude-dir` and `decision/nothing-written-under-claude-home` their one exception. Verify writes a trust record for its own fresh temp folder into the Claude config its child reads, removes only that record afterwards, and a SIGKILL leaves it. It also added `decision/trust-record-writer-guarded`. Matters because this chunk is the owed live re-read of that same row: S0′ exercises this writer on the real 2.1.325 CLI. "Not a rule for run or revive" keeps the product's write out of scope (it belongs to «Session folder trusted by viola»). All of it stands on a delegate's PROVISIONAL confirm.

@history_item n=2 marker=2026-11-17-trust-record-ledger-row
**2026-11-17-trust-record-ledger-row** — Secret Management (4 rows), Auth (1), Input Validation (1), Data Protection cleanup (2): `r8-claude-env-strip`, `credential-strip-by-name-only`, `env-not-config-channel` and `no-stored-secrets` each gained one exception. Verify reads the value of `CLAUDE_CONFIG_DIR`, but only when `claude_env_keep` keeps that name, to choose the Claude config that holds its trust record. `explicit-mode-before-write` gained an exception: the writer keeps Claude's config file's own mode. `trust-record-closed-words` was added: the outcome is one of 14 closed words, and no path, key or value is stamped. `verify-probe-dirs-cleanup` and `probe-artifacts-cleanup` now say a kill leaves the record. Matters because the rig runs verify under a private `CLAUDE_CONFIG_DIR`, and the measured ground says verify's children receive the kept value. The new `--record` witness and the first-screen words in `measured` must stay closed words with no path, as this row's rule requires.

@history_item n=3 marker=2026-11-17-first-readings-on-the-standing-rig
**2026-11-17-first-readings-on-the-standing-rig** — Secret Management `decision/live-rig-isolated-child`: replaced the sha256-and-mtime witness with a mark witness. The rig reads the user's `~/.claude.json` only after a moved stamp, searches it for rig-made needles, drops the bytes and keeps one closed word. The reason was a host writer of the shared user config, which disproved the stamp-only witness. This is the precedent behind the scope's `--record` witness: a «real file untouched» assertion cannot be attributed on a shared file the host's own sessions rewrite.

@history_item n=4 marker=2026-11-17-first-readings-on-the-standing-rig
**2026-11-17-first-readings-on-the-standing-rig** — Auth `decision/live-round-rig-keys-own-pty` and Input Validation `decision/live-rig-prompt-capture-closed`: the founder's word anew per chunk binds only the rig's closed argv grant. The stop (Ctrl-C into its own pty, or a no-key group kill) is standing and needs no word, which is the founder's own answer. Added the rig's capture verb, which keeps closed words only. Matters because S0′'s `keys:` / stop naming follows this rule (keys: none, stop named beside them), and the `V` arm's six-word printout must keep the closed-words-only shape.

@history_item n=5 marker=2026-11-17-standing-live-rig
**2026-11-17-standing-live-rig** — Auth `live-round-rig-keys-own-pty` (one standing committed rig, closed argv grant) and Secret Management `+ decision/live-rig-credentials-copy` / `+ decision/live-rig-isolated-child`. The OAuth credentials copy is owner-only in private scratch, only its expiry is read, a start with under 20 minutes of token is refused, and the copy is removed on every exit. The child runs under a private `CLAUDE_CONFIG_DIR` with a closed list of kept host names. Matters because the R6 cap's "a start refused for the token window is uncounted" rests on this 20-minute refusal, and the `V` arm's git-initialised seeded cwd must stay inside the rig's private scratch and config.

@history_item n=6 marker=2026-11-15-live-round-on-the-trimmed-product
**2026-11-15-live-round-on-the-trimmed-product** — added `decision/live-round-rig-keys-own-pty` (the rig's keys into its own pty only, on the founder's per-chunk word) and `decision/live-session-screens-kept-local`. Screen frames stay under a gitignored local path, and only a closed-word summary per frame lands. Matters because S0′'s first-screen evidence and any screen frames must stay local, with only closed words in the run dir or a commit.

@history_item n=7 marker=2026-10-18-plugin-precedence-live-readings
**2026-10-18-plugin-precedence-live-readings** — the live rig moved outside the work tree into its own git repo, because an in-tree `proj/` meets the external-imports dialog after the trust dialog. The CLI's `~/.claude.json` trust entry per accepted dialog was named as a residual. Rig keys: at most 2 arrows and 1 Enter, only on a read `selected` row. Matters as the measured precedent for this chunk's hypothesis that something other than trust (for example imports) stops Runs B–F. It also bears on the `V` arm's git-initialised cwd seeded at its root's key, since the trust lookup keys on the git root.

@history_item n=8 marker=2026-10-17-optional-plugin-install
**2026-10-17-optional-plugin-install** — the live round's residual under `~/.claude`. The rig dir was its own git repo because the CLI takes the local scope's project from the git work tree root. The CLI's own writes were accepted, and viola itself writes nothing there. Matters as the earliest record that the CLI keys project state on the git root, which is the rule Runs B–E (in a trusted repository) and Run F (outside any repository) are now set to read apart.

@history_item n=9 marker=2026-10-05-real-cli-verify-probes
**2026-10-05-real-cli-verify-probes** — verify's interactive PTY children, ratified by the founder. They are direct spawns under the R8 strip at 80×24, with output capped at `MAX_FRAME` into a Screen under `catch_unwind` and read only against compiled literals. A modal start is killed with no key, and no byte goes into a dialog. Screen fixtures keep signature rows only, and refusals carry closed codes and no content. The trusted-runs transcripts residual was accepted. The prerequisite: verify must run from a trusted folder whose external import is already answered (keyed on the git root). This is the source of `verify-children-direct-spawn`, `verify-never-answers-dialogs`, `verify-never-answers-import(-dialog)`, `verify-signature-only` and `screen-fixture-signature-rows-only`. The first-screen word (`trust` · `imports` · `box` · `none`) must be decided from these compiled signatures alone, and a modal start must still be killed with no key.

@history_item n=10 marker=2026-10-05-dialog-rows-and-re-probe
**2026-10-05-dialog-rows-and-re-probe** — Runs C and D joined verify's child set (four interactive children). The capture arm answers only through the probe hook's output, and no byte is typed into any dialog. Matters because `verify-writes-no-dialog-byte` ("probe dialogs are answered only by the probe hook's output") comes from here. The per-run first-screen word covers these runs too.

@history_item n=11 marker=2026-10-06-local-command-and-paste-framing-rows
**2026-10-06-local-command-and-paste-framing-rows** — Run B's compiled pastes go only into rows that hold the input-box literal and no modal literal. The wait runs up to `PROBE_DEADLINE` (120 s) from that turn's Stop, and a modal, a poisoned screen or the deadline ends the pastes with no key. Founder-ratified. Matters for the `verify_window_` kill and the 52 s case: the walls a whole verify implies, and whether a child exited or was cut, rest on these waits.

@history_item n=12 marker=2026-10-19-newer-claude-cli-rows
**2026-10-19-newer-claude-cli-rows** — Run B's pastes go into rows holding *an* input-box literal (the compiled set has two). Also accepted one scratch footer reading of a newer CLI as a one-off crossing: "another reading needs its own answer". Matters because the first-screen `box` word reads the same compiled input-box literals. It also stands as precedent that each live start of the user's CLI needs its own ruled cap, which R6 supplies here.

@history_item n=13 marker=2026-11-13-verify-readiness-on-the-input-box
**2026-11-13-verify-readiness-on-the-input-box** — changed `~ decision/screen-fixture-signature-rows-only` (the box rule row as rule chars alone, the prompt row as the prompt char alone; any other box row refused) and added `+ decision/input-box-readiness-from-literals`. One predicate (a footer literal, or a rule row of at least 40 chars directly above a prompt-char row, never while a modal literal stands) decides readiness for verify and for run's gate. Matters because the `box` first-screen word must reuse this single predicate and not a second reading. That keeps `viola run` unchanged, as the boundaries require.

@history_item n=14 marker=2026-11-16-local-command-delivery
**2026-11-16-local-command-delivery** — added `decision/verify-echo-probe-residuals` (Run B's seventh paste, `/mcp disable viola-probe-absent`; whether it writes to the user's real Claude config stays unmeasured) and `decision/transcript-read-bounded`. Changed `decision/compiled-classification-sources`. Ratified 2026-11-17. Matters because S0′ runs Run B's echo probe live under the rig's private config, so its residual lands in the private config and not the user's. Whatever S0′ reads must stay within these count-only, compiled-source rules.

@history_item n=15 marker=2026-10-13-interrupted-verify-cleanup
**2026-10-13-interrupted-verify-cleanup** — verify's probe and interactive dirs are removed on pass, fail, or a SIGINT/SIGTERM stop. A stopped run kills its child with no key, and guards remove only what their own run created, never by pattern. Matters because the rig's Ctrl-C stop of S0′ relies on these drop guards (which now also remove the trust record). A group kill counts as a SIGKILL-class exit, which leaves the record to be removed by hand.

@history_item n=16 marker=2026-10-14-resume-and-compact-ledger-rows
**2026-10-14-resume-and-compact-ledger-rows** — Run E joined the child set (`<cwd>/.viola-verify-<pid>-resume/`, `--resume` of Run B's first SessionStart id, empty input, no key). With no id, Run E spawns nothing and `resume-session` fails. Matters because Run E's first-screen word and its `null` (a run that did not start) are among the six words this chunk records, and `verify-run-e-resume-target` is a picked row.

@history_item n=17 marker=2026-10-18-compact-post-condition-ledger-row
**2026-10-18-compact-post-condition-ledger-row** — Run B's sixth paste (`/compact`) and the twenty-four-row stamp. Run B's residual is three transcript files. A `20 pass  4 fail` home reads `cli_verified:false`. Matters for the predicate (the 21 rows the live home's 2.1.325 stamp passes) and for reading S0′'s `7/23`-style counts against the relied-set history.

@history_item n=18 marker=2026-10-16-unwrapped-sessions-on-the-board
**2026-10-16-unwrapped-sessions-on-the-board** — Run B makes one bounded `claude agents --json` read after its first turn settles. verify writes eight spawn pairs, and the ledger gains `agents-join`. Matters because the scope's "7 claude starts by the product's count" and the per-run walls depend on this child set.

@history_item n=19 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** — the R8 strip matches names exactly, and the identity floor is the eleven names. verify's stop is SIGINT/SIGTERM only. `LedgerRow::relied()` is all twenty-four rows on every build. Matters because the scope's argv/environment comparison (what verify's strip leaves the child, `HOME`, `TERM`) and the stop semantics are read against these Linux-only rules.

@history_item n=20 marker=2026-11-14-status-line-interception-removed
**2026-11-14-status-line-interception-removed** — removed the settings override and Run B's probe `--settings` (`decision/statusline-override-plain-path-only` is gone, so no `--settings` is passed to the child). Retired the `workspace.repo` exception to `fixture-scrub-and-refusal` / `fixture-hygiene`. Narrowed `home-strict-modes-before-read`, `strict-modes-check` and `max-frame-shared-cap`. Matters for the P3 list of how verify's child argv differs from the rig's own wrapped start (no `--settings` now). The `--record` scrub runs with no `workspace.repo` carve-out.

@history_item n=21 marker=2026-09-28-capability-ledger-and-viola-verify
**2026-09-28-capability-ledger-and-viola-verify** — `MAX_FRAME` consumers extended, verify's CI split: the real CLI runs only locally, verify runs in CI only against the fake agent, and its real-CLI probe and `--record` only locally. Also: probe captures are transient 0600 files in a 0700 per-pid dir, and `verify --record` scrubs then refuses the whole recording. These are the source of `real-cli-local-only`, `ci-no-secrets-no-real-claude`, `verify-probe-captures-transient` and `fixture-recording-scrubbed`. Matters because the fake-agent work must be green in CI before the one local live start, and the `--record` witness must keep the scrub-or-refuse-whole shape.

@history_item n=22 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** — `ledger/stamps.json` is checked before `ledger/` is made and again under its lock. `viola verify` runs `check_home_tree` on the home, `ledger/`, `bin/` and `plugin/` before its cli log, and a refusal is exit 1 with no step line or summary. New ban: never run the check after a create, a mode set or a log open that narrows the path. This is the source of `stamps-checked-under-lock`, `verify-home-tree-check`, `stamps-strict-check-first` and `strict-check-before-narrowing`. Matters because writing first-screen words into the stamp's `measured` goes through `update_stamps` and must keep this order.

@history_item n=23 marker=2026-10-10-statusline-pass-through
**2026-10-10-statusline-pass-through** (the arm's check before its log) — measured that obs init sets an existing home to 0700, so a check made after it passes a 0770 home. Hence the check runs first. Trap behind `strict-refusal-leaves-no-trace` and `home-tree-strict-modes-before-log`: any new verify or rig step that opens a log or creates a dir must come after the strict-modes check.

@history_item n=24 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** — verify's refusal codes: one closed code per refusal on its `process-exit`, a refused home tree lineless, a failing row no refusal and no code. `StampsError::StrictModes` reads `strict-modes-failed`. This is the source of `contract/verify-refusal-codes/strict-modes-failed` and `contract/strict-modes-refusal/verify`. Matters because any new verify outcome (the `--record` witness, the `verify_window_` kill) must not invent a refusal code outside this closed set without its own ruling.

@history_item n=25 marker=2026-10-04-dialog-answers-by-dialog-id
**2026-10-04-dialog-answers-by-dialog-id** — run's strict stamps read, and the standing ban: never answer non-`null` unless `run`'s gate read the stamps through the strict check. This is the origin of `dialog-answers-need-verify-stamp` and `stamps-written-only-by-verify`. Matters because the live home is not re-stamped here: S0′ writes only the rig's private home's stamp, which switches on dialog answering only there.

@history_item n=26 marker=2026-11-06-version-done-check
**2026-11-06-version-done-check** — `statusline-rate-limits` reads the account's plan, not the CLI: a verify on an unchanged 2.1.287 read it red (`23 pass  1 fail`). Matters for the predicate's floor (the rows the live home's stamp passes). A row that reads the account and not the CLI must not be read as a CLI regression in S0′.

@history_item n=27 marker=2026-11-15-api-error-turn-end
**2026-11-15-api-error-turn-end** — Auth `decision/unverified-cli-gate` excepts StopFailure (no ledger row, no verify probe), its live reading owned by «The 2.1.325 recorded fixture set». Ratified at 2026-11-16T06-05-33-wrap. Matters because the live home's 2.1.325 re-stamp belongs to that entry, not this chunk, as the boundaries state.

@history_item n=28 marker=2026-10-07-live-rows-and-paste-shapes-on-the-dev-host
**2026-10-07-live-rows-and-paste-shapes-on-the-dev-host** — two scratch measurements across the credential boundary, each ratified once. Standing rule: "the answers are spent; another peer message or another names probe needs its own showing". Matters as the precedent that a live start is capped by its own ruling. S0′ and its two spares are bounded by R6, and a fourth start goes back to the founder.

@history_item n=29 marker=2026-10-19-overseer-switch-to-viola
**2026-10-19-overseer-switch-to-viola** — verify's print-mode probe residual (an empty project dir with `memory/` under `~/.claude/projects/`) accepted into the trusted-runs class. Matters because S0′'s print probe counts as one of the 7 starts, and its residual lands in the rig's private config.

@history_item n=30 marker=2026-10-04-readiness-gate-and-timing-constants
**2026-10-04-readiness-gate-and-timing-constants** — the vt100 degrade: every feed is under `catch_unwind`, and a caught panic poisons the screen model until a size change. A poisoned model reads `input-not-ready`. Matters because the first-screen word must handle a poisoned screen (a likely `none`) and the count of settled frames, which are the closed facts the plan uses to tell candidates apart.

@history_item n=31 marker=2026-10-27-bounded-inputs-at-every-boundary
**2026-10-27-bounded-inputs-at-every-boundary** — added the bounded-reader census `product_readers_are_each_bounded_or_named` (source of `decision/bounded-readers-census`). Matters because any new read in verify or the rig (the record witness, a re-read of the Claude config) must be bounded in its statement or named in `EXCEPTIONS`.

@history_item n=32 marker=2026-10-03-mutation-scoring-completion
**2026-10-03-mutation-scoring-completion** — the native pre-push crossing: every child runs under `/usr/bin/env -i` with exactly HOME (from passwd) and PATH, and a host-valued assignment is a boundary widening. This is the source of `pre-push-empty-environment-launch` and `hermetic-child-environment`. Matters because a gate entry for the new `cli_verify` cases must not carry a host value such as `CLAUDE_CONFIG_DIR` into a gated child.

@history_item n=33 marker=2026-09-28-mutation-testing-to-the-epoch-boundary
**2026-09-28-mutation-testing-to-the-epoch-boundary** — removed the CI and pre-push mutation legs (the founder's ruling), so mutation belongs to the epoch-boundary audit (`decision/mutation-testing-at-audit`). Matters because the scope's "No mutation run" matches it, and controls are remove-the-guard runs with `--timeout`.

@history_item n=34 marker=2026-11-16-wait-wake-condition
**2026-11-16-wait-wake-condition** — `decision/test-removal-own-dirs-only` gained a fourth way: another build's stamp dir, whose lock is taken with `try_lock`. Ratified at 2026-11-16T06-05-33-wrap. Matters because the bounded 52 s case and any new e2e fixtures under the e2e home must remove only dirs they created by these four ways.

@history_item n=35 marker=2026-09-25-pty-wrapper-on-windows
**2026-09-25-pty-wrapper-on-windows** — `config.json` `claude_env_keep`: at most 32 names matching `^CLAUDE[A-Z0-9_]*$`, rejected whole when bad, and a floor name has no effect. This is the source of `contract/config-json/claude-env-keep`. Matters because verify's `CLAUDE_CONFIG_DIR` read is gated on this key keeping that name, and the rig's config must keep it for verify's children to receive the private config.

@history_item n=36 marker=2026-10-20-t06-58-31-wrap
**2026-10-20-t06-58-31-wrap** — the founder's Epoch 5 batch made his own the delegate confirms on Run B's probe `--settings`, agents read, newline and `/compact` pastes, and Run E. Matters as the ratification standing behind today's verify child set, which this chunk instruments but must not widen.
