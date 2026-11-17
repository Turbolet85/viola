@history_item n=1 marker=2026-09-25-pty-wrapper-on-windows
**2026-09-25-pty-wrapper-on-windows** — (R8 persistent-environment exemption) introduced `config.json` `claude_env_keep` (≤ 32 names `^CLAUDE[A-Z0-9_]*$`, rejected whole when bad, an identity-floor name ineffective) and the eleven-name identity floor. S0 runs verify with `CLAUDE_CONFIG_DIR` kept through `claude_env_keep`, and the writer picks `<CLAUDE_CONFIG_DIR>/.claude.json` only when the home keeps that variable for the child. This entry is the origin of that keep-list rule (`contract/config-json/claude-env-keep`, `decision/r8-claude-env-strip`).

@history_item n=2 marker=2026-09-27-instance-state-and-start-order
**2026-09-27-instance-state-and-start-order** — (the tempfile `persist` helper) made `viola_state::fs::replace_private` the one shared replace helper: the mode is set on the temp file before any byte, then `sync_all` and `persist`. This is the origin of `decision/one-replace-helper` and `decision/explicit-mode-before-write`. The record writer's "atomic replace, read-back" sits on that side (`viola-state`), and these rows bind it. The plan must say how the helper's owner-only mode squares with a replaced `~/.claude.json`, which is the CLI's own file and not viola's.

@history_item n=3 marker=2026-09-28-capability-ledger-and-viola-verify
**2026-09-28-capability-ledger-and-viola-verify** — (stamps reads; the `hook --capture` arm, probe captures and the fixture scrub; `MAX_FRAME` consumers) set the base of the capability ledger and `viola verify`:
- probe captures are 0600 files in a 0700 `ledger/probes/<pid>/`, removed by a drop guard;
- `verify --record` scrubs every string and refuses a whole recording that still holds a home path or the user word;
- the stamps read is capped at `MAX_FRAME`;
- the real CLI runs only locally, and verify runs in CI only against the fake agent.

A sixth run's captures and fixtures inherit all of this. The fake-agent path is what keeps CI green.

@history_item n=4 marker=2026-10-04-dialog-answers-by-dialog-id
**2026-10-04-dialog-answers-by-dialog-id** — (run's strict stamps read; decisions on the six-row stamp) made `run`'s gate read `ledger/stamps.json` through `read_stamps_strict`. A refusal or a malformed file is `cli_verified:false`, and a non-null answer requires a strict stamp read (`decision/version-gate-strict-stamps-read`, `decision/dialog-answers-need-verify-stamp`). It is also the first stamp-row-count precedent: decisions flowed on a six-row stamp until new rows landed. This is the mechanism by which adding the 23rd relied row turns every 22-row stamp unverified.

@history_item n=5 marker=2026-10-05-real-cli-verify-probes
**2026-10-05-real-cli-verify-probes** — (verify's typed-input PTY probe) ratified verify's interactive children:
- direct spawns under the R8 strip at 80×24, output capped at `MAX_FRAME` into a `Screen` under `catch_unwind`;
- Run A untrusted in a fresh 0700 OS-temp dir, killed;
- never a byte into a CLI-native dialog, and a modal start killed with no key;
- 0700 probe dirs removed on every exit path;
- viola writes nothing under `~/.claude`.

It also recorded the dev-host prerequisite: a trusted folder with its import answered. The overseer set the repo-root flags in one atomic `~/.claude.json` edit with a backup, which is the only prior write into `~/.claude.json` and was made by hand, not by viola. The sixth run is a Run A twin and widens exactly this ratified set, so it needs the founder's ruling.

@history_item n=6 marker=2026-10-05-dialog-rows-and-re-probe
**2026-10-05-dialog-rows-and-re-probe** — (Runs C and D; R2 closed) took verify from two to four children, and from a ten-row to a fourteen-row stamp (`/14`). A stamp missing the new rows reads `cli_verified:false`, and `answer` exits 12 (`contract/cli-exit-codes/12`). The probe effects stay inside the probe's own dir (`decision/verify-probes-confined-to-own-dir`). This is the precedent for growing the child set, the residual count and the stamp at once, as a founder-ratified widening.

@history_item n=7 marker=2026-10-06-local-command-and-paste-framing-rows
**2026-10-06-local-command-and-paste-framing-rows** — (the seventeen-row stamp) raised the stamp from 14 to 17 rows. A stamp of exactly the fourteen older ids, all `pass`, reads `cli_verified:false`. This is the same "old stamps go stale" rule this chunk must name for the 22 → 23 move.

@history_item n=8 marker=2026-10-13-interrupted-verify-cleanup
**2026-10-13-interrupted-verify-cleanup** — (verify's dirs removed on a Ctrl-C / TERM stop) fixed verify's cleanup contract:
- the dirs are removed when verify passes, fails, or is stopped by SIGINT or SIGTERM through `cmd::verify::stop`, a handler that only sets a flag;
- a guard removes only what its own run created, never by a pattern or a listing;
- no remover of an earlier run's dirs exists;
- SIGKILL, a crash or a failed handler install leave them for removal by hand.

This is the direct precedent for "verify removes ONLY its own record on every exit path; the plan says what a SIGKILLed verify leaves" (`decision/verify-probe-dirs-cleanup`, `decision/probe-artifacts-cleanup`).

@history_item n=9 marker=2026-10-13-paste-newline-ledger-row
**2026-10-13-paste-newline-ledger-row** — (eighteen stamp rows) took the stamp to `/18`; a stamp of an older id set reads `cli_verified:false`. This is the per-row stamp-staleness precedent. The fifth paste was recorded as a widening of the ratified verify child set.

@history_item n=10 marker=2026-10-13-owed-ledger-rows
**2026-10-13-owed-ledger-rows** — (the relied set) first made the stamp read over a relied set that differed per OS (`/21` Unix, `/18` Windows). An all-`pass` older stamp reads unverified. The probe `--settings` and capture arm were a founder-ratified widening of the verify child set. This entry is the origin of "relied" as distinct from "in the ledger", which the 23rd relied row extends.

@history_item n=11 marker=2026-10-14-resume-and-compact-ledger-rows
**2026-10-14-resume-and-compact-ledger-rows** — (Run E in the verify child set) took verify to five interactive children, the "ratified child set of five" the scope widens to six:
- Run E runs last in its own 0700 `-resume/` dir with its guard and no key typed;
- an uninterrupted verify writes seven spawn pairs;
- the ledger reached twenty-two rows (`/22`), and an older stamp reads unverified.

This is the closest precedent for adding one interactive run, its dir, its spawn-pair count and its stamp row.

@history_item n=12 marker=2026-10-16-unwrapped-sessions-on-the-board
**2026-10-16-unwrapped-sessions-on-the-board** — (Run B's agents read; twenty-three rows) added a row (`agents-join`) and changed verify's spawn-pair count (seven to eight). A stamp of exactly the pre-chunk ids reads unverified. `decision/agents-probe-log-content-free` and `decision/agents-probe-log-redaction` came from here: an agents-probe pair holds no row text, id, cwd or path. A sixth run's logging must follow the same no-path rule, and the step-count and spawn-pair sweep has precedent here.

@history_item n=13 marker=2026-10-17-optional-plugin-install
**2026-10-17-optional-plugin-install** — (the live round's residual under ~/.claude) states that viola itself writes nothing under `~/.claude` and spawns no `claude` for plugin install; only the CLI's own commands write there. This is the origin of `decision/no-writes-under-claude-home`, `decision/viola-writes-nothing-in-claude-dir` and `decision/nothing-written-under-claude-home`. These are the rows the chunk must give a bounded exception for verify's own record, and only that.

@history_item n=14 marker=2026-10-18-compact-post-condition-ledger-row
**2026-10-18-compact-post-condition-ledger-row** — (the twenty-four-row stamp) moved the stamp to `/24` Unix and `/21` Windows. A twenty-three-id older stamp reads `cli_verified:false`. The residual count was measured per round. This is the most recent "row added, older stamps unverified" precedent before the Linux-only fold.

@history_item n=15 marker=2026-10-18-plugin-precedence-live-readings
**2026-10-18-plugin-precedence-live-readings** — (the live round's rig outside the work tree, its trust keys and residual) ratified a rig `proj/` that is its own git repo outside the work tree. An in-tree `proj/` meets the external-imports dialog after the trust dialog. The rig passes the folder-trust dialog only by keys into its own pty. The residual includes the CLI's own `~/.claude.json` trust entry per accepted dialog, and viola writes nothing into `~/.claude.json`. This is the precedent for the trust record (the CLI's own write) and for the git-root key. The external-imports trap applies to the sixth run's temp dir.

@history_item n=16 marker=2026-10-19-overseer-switch-to-viola
**2026-10-19-overseer-switch-to-viola** — (verify's print-mode probe residual accepted) accepted the print probe's empty `~/.claude/projects/` dir into the verify trusted-runs residual class (`decision/verify-real-cli-transcripts-accepted`, `decision/verify-trusted-runs-risk`). A sixth run's own residuals must be measured and named in that class.

@history_item n=17 marker=2026-10-19-newer-claude-cli-rows
**2026-10-19-newer-claude-cli-rows** — (the scratch footer reading accepted) states that a live start of the user's CLI is no standing permission: "another reading needs its own answer", and earlier scratch answers stay spent. This is the precedent behind S0 and the spare S0-2 being capped and individually ruled.

@history_item n=18 marker=2026-10-20-t06-58-31-wrap
**2026-10-20-t06-58-31-wrap** — (the founder's Epoch 5 batch) made the founder's own word stand for Run E, Run B's added pastes and the compaction residual. This is the ratification of the five-child verify set that this chunk widens to six. A widening of it again needs his word, not a delegate's confirm.

@history_item n=19 marker=2026-10-20-epoch-5-cleanup-ii
**2026-10-20-epoch-5-cleanup-ii** — (Run E's `resume_run` re-cited at its module) records that Runs A–E live in `src/cmd/verify/typed/runs.rs` (`typed.rs` is a module root). This is where the sixth run, a Run A twin, belongs. The entry also covers the `Mcp.` fixture tap's scrub with closed refusal codes, the fixture-cleanliness rule a sixth run's recordings meet.

@history_item n=20 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** — (`ledger/stamps.json` checked before `ledger/` is made, and `verify`'s home tree before its log):
- `update_stamps` runs `strict::check_stamps` before `create_private_dir(ledger/)` and again under its lock; a refusal reads and writes nothing;
- `verify` runs `check_home_tree` before its `cli` log;
- new ban: never run the check after a create, a mode set or a log open that narrows the path.

This is the source of `decision/stamps-checked-under-lock`, `decision/verify-home-tree-check`, `decision/stamps-strict-check-first`, `contract/strict-modes-refusal/verify` and `decision/strict-refusal-leaves-no-trace`. Trap: a create or log open placed before the check passes a widened tree.

@history_item n=21 marker=2026-10-27-bounded-inputs-at-every-boundary
**2026-10-27-bounded-inputs-at-every-boundary** — (the bounded-reader census; serde_json `unbounded_depth` banned) added `tests/contract_bounded_readers.rs`: `product_readers_are_each_bounded_or_named` requires every product reader to be bounded in its statement or named as an exception. It also banned `unbounded_depth` in `deny.toml` (`decision/bounded-readers-census`, `decision/serde-json-bounded-depth`). The new reads of `.claude.json` (the writer's read-back, and the fake agent's record read) must be capped and censused, and the JSON parse keeps the default depth.

@history_item n=22 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** — (the error bodies' census) added `tests/contract_error_bodies.rs`: `product_error_writers_are_each_named` fails on an unnamed product error writer, and `thiserror_messages_interpolate_nothing` fails on an interpolating message (`decision/every-error-writer-named`, `decision/error-display-no-upstream-text`). Any new writer error or refusal for the record writer must be named there and carry fixed messages, never a path.

@history_item n=23 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** — (verify's refusal codes) gave `verify` closed refusal codes on its `process-exit`. A failing row is no refusal and no code, and the fixture's `<file>` enters no code. This is the precedent for the probe's row reading what it could not do "in a closed word, never `pass`" without making it a verify refusal.

@history_item n=24 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** — (the relied set holds on every build; the R8 strip matches names exactly) covers two things:
- `LedgerRow::relied()` became one set on every build (`/24` then), with no per-OS relied set;
- the R8 strip matches `CLAUDE*` names exactly, keeps only `claude_env_keep` names and always strips the identity floor.

The 23rd relied row is one set on Linux. `CLAUDE_CONFIG_DIR` reaches the child only through `claude_env_keep`, which decides which config file the writer targets.

@history_item n=25 marker=2026-11-06-version-done-check
**2026-11-06-version-done-check** — (statusline-rate-limits reads the account's plan) recorded that a relied row went red on an unchanged CLI (`23 pass  1 fail`) because it read account state, not the CLI. This is a trap for the new row: a probe must measure the CLI's honouring of the record, with a control that reads the dialog when the record is withheld, not an environmental accident.

@history_item n=26 marker=2026-11-13-verify-readiness-on-the-input-box
**2026-11-13-verify-readiness-on-the-input-box** — (Data Protection and Input Validation) covers two rows:
- `decision/screen-fixture-signature-rows-only` was widened so screen fixtures keep only signature rows;
- `decision/input-box-readiness-from-literals` was added: input-box readiness is read only from compiled literals, one predicate for verify and `run`'s gate.

The sixth run's settled-frame read (no trust dialog) goes through these literals and fixture rules (`decision/verify-screen-read-against-literals`).

@history_item n=27 marker=2026-11-13-rows-proposal-rule-work
**2026-11-13-rows-proposal-rule-work** — (Data Protection) `decision/logs-owner-only`: the owner-only append open never follows a symbolic link, and a link at the path is a failed open. This is the precedent for the writer's rule that "a link → nothing written".

@history_item n=28 marker=2026-11-14-status-line-interception-removed
**2026-11-14-status-line-interception-removed** — (Input Validation; Data Protection; Error Handling) removed the statusline arm and Run B's `--settings`. The capture arms went from two to one (`decision/capture-arms-isolated`, `decision/hook-silent-exit-zero`), and the `MAX_FRAME` consumer list shrank (`decision/max-frame-shared-cap`). The fixture-scrub `workspace.repo` exception was retired. These rows hold verify's current argv and capture state, which the sixth run's twin copies.

@history_item n=29 marker=2026-11-15-live-round-on-the-trimmed-product
**2026-11-15-live-round-on-the-trimmed-product** — (Authentication & Authorization; Data Protection) added two rows that bind S0's live start:
- `decision/live-round-rig-keys-own-pty`: rig keys only into its own pty, the founder's word per round;
- `decision/live-session-screens-kept-local`: live screen frames stay under a gitignored local path, and only a closed-word summary lands.

@history_item n=30 marker=2026-11-15-api-error-turn-end
**2026-11-15-api-error-turn-end** — (Authentication & Authorization) gave `decision/unverified-cli-gate` its except: StopFailure is registered with no capability ledger row and no verify probe, owned by the 2.1.325 fixture set. This chunk takes the opposite route for the trust record: a relied ledger row with its own probe. The gate's exception list stays as is.

@history_item n=31 marker=2026-11-16T06-05-33-wrap
**2026-11-16T06-05-33-wrap** — (the StopFailure no-ledger-row answer ratified) removed the provisional mark from the `decision/unverified-cli-gate` except: the founder's own word. The gate row this chunk extends stands ratified as it reads.

@history_item n=32 marker=2026-11-16-local-command-delivery
**2026-11-16-local-command-delivery** — (Data Protection) added `decision/verify-echo-probe-residuals` and `decision/transcript-path-redacted`:
- Run B's seventh paste (`/mcp disable …`) may write to the user's real Claude config, unmeasured, and that residual was accepted;
- the transcript path, a host path carrying the username, appears in no event data, log line or verify record.

This is the closest precedent for a verify probe touching the user's Claude config. It is also the redaction rule the record's key path (an absolute temp or git-root path) must follow.

@history_item n=33 marker=2026-11-17T05-21-35Z-wrap
**2026-11-17T05-21-35Z-wrap** — (the delegate's confirm ratified, local-command-delivery, Data Protection) made `decision/transcript-path-redacted` and `decision/verify-echo-probe-residuals` stand on the founder's own word. The verify-residual precedent this chunk extends is ratified.

@history_item n=34 marker=2026-11-17-standing-live-rig
**2026-11-17-standing-live-rig** — (Authentication & Authorization; Secret Management) made the rig one standing committed rig with a closed argv key grant. It added `decision/live-rig-credentials-copy` (an owner-only OAuth copy that refuses a start with under 20 minutes of token and leaves no credential or host path in any record) and `decision/live-rig-isolated-child` (a private `CLAUDE_CONFIG_DIR` and a closed kept env list). S0 runs under this config. A start refused for the token window is uncounted.

@history_item n=35 marker=2026-11-17-first-readings-on-the-standing-rig
**2026-11-17-first-readings-on-the-standing-rig** — (Secret Management; Authentication & Authorization; Input Validation) made three changes:
- `decision/live-rig-isolated-child`: the stamp-only witness was disproved and replaced by the mark witness, which reads the user's real `~/.claude.json` only after a moved stamp, searches it for rig-made needles, drops the bytes and keeps one closed word;
- `decision/live-round-rig-keys-own-pty`: the stop (Ctrl-C into its own pty, or a no-key group kill) needs no per-chunk word, but the argv grant does;
- `decision/live-rig-prompt-capture-closed` was added.

The scope's "the founder's real `~/.claude.json` is read only by the mark witness" rests on this, along with the `stop:` and `keys:` lines for each start.
