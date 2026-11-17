@history_item n=1 marker=2026-09-25-pty-wrapper-on-windows
**2026-09-25-pty-wrapper-on-windows** (R8 prefix rule + identity floor, explicit child cwd, viola-side program resolution) — made R8 a `CLAUDE*` prefix strip with an 11-name identity floor and the `config.json` `claude_env_keep` pass-list, set the child cwd explicitly and had viola resolve the child program itself. Why it matters: S0 runs verify under the rig's private config through `CLAUDE_CONFIG_DIR` kept by `claude_env_keep` (the file the record goes into depends on whether that name survives the strip), and the sixth run sets its child's cwd to its own OS-temp dir.

@history_item n=2 marker=2026-09-26-local-linux-pre-push-gate
**2026-09-26-local-linux-pre-push-gate** (`FAKE_AGENT_PUMP_DELAY_MS` test seam) — the first ratified fake-agent env seam, `fake-agent` feature only, a boundary widening. Precedent for this chunk: the fake agent must read the trust record from the config file, so any new fake-agent input needs to fit the seam rules (`decision/fake-agent-test-seams`, `decision/test-seams-not-configuration`) rather than add a new env name.

@history_item n=3 marker=2026-09-27-instance-state-and-start-order
**2026-09-27-instance-state-and-start-order** (tempfile `persist` snapshot writer) — replaced atomic-write-file (BSD-3-Clause, outside `deny.toml`) with tempfile `persist` through ONE shared helper in `viola_state::fs` (mode set before any byte). This is the reason behind `decision/tempfile-snapshot-writer` and `decision/atomic-replaced-state-files`: the record writer's atomic replace of `.claude.json` must go through those helpers, not carry its own temp + rename.

@history_item n=4 marker=2026-09-28-hook-perf-gate
**2026-09-28-hook-perf-gate** (the second fake-agent test seam `FAKE_AGENT_HOOK_PANIC`) — two ratified seams now, each a founder-ratified boundary widening. A third env seam would need ratification. The founder later refused one for the fake agent (see 2026-10-16), so the fake agent's record read should take the config path it already gets (`CLAUDE_CONFIG_DIR` / `~/.claude.json`), not a new seam.

@history_item n=5 marker=2026-09-28-capability-ledger-and-viola-verify
**2026-09-28-capability-ledger-and-viola-verify** (ledger rows, `viola verify`'s probe, the stamps envelope) — created the closed ledger in `viola_agent_claude::ledger`, the "verified only when every row passes" rule, `viola verify` with its `ledger/probes/<pid>/` drop-guarded dirs, the stamps envelope and its one writer `update_stamps`. This is the register the 23rd row extends.

@history_item n=6 marker=2026-09-28-capability-ledger-and-viola-verify
**2026-09-28-capability-ledger-and-viola-verify** (the version gate placed and split by crate) — split the gate three ways: Claude parsing, rows and verdict in the pure `viola-agent-claude` (no `viola-state` edge), the spawn in the root bin, stamps I/O in `viola_state::stamps`. This is the precedent for the scope's inferred split of the record writer: the JSON shape on the Claude side, the lock, atomic replace and read-back on a side both `verify` and the later `run` reach (`decision/agent-claude-pure`).

@history_item n=7 marker=2026-09-28-capability-ledger-and-viola-verify
**2026-09-28-capability-ledger-and-viola-verify** (the hidden `hook --capture` arm, founder-ratified exceptions) — verify's transient unwrapped probe children and the capture arm were ratified as boundary widenings and named the only exception to [Plugin Scope]. The sixth run is another transient unwrapped child, and its record write is a new widening of what verify touches outside the viola home.

@history_item n=8 marker=2026-09-29-verify-stamped-test-homes-and-harness
**2026-09-29-verify-stamped-test-homes-and-harness** — harness homes are stamped by `viola verify` against the fake agent at boot, root homes by `stamped_home`, and `run --local-live` is the one real-`claude` verify home. Why it matters: once the row is relied, the scope's claim that "the tests' shared stamp and CI stay green" rests on these stamp paths and on `contract_ledger_probes`' stamped sets carrying the new row.

@history_item n=9 marker=2026-10-04-confirmed-send-with-cl-1-records
**2026-10-04-confirmed-send-with-cl-1-records** — introduced the partial readiness gate on an unverified CLI (`decision/partial-gate-on-unverified-cli`). This is the behaviour every 22-row-stamped home falls back to once the 23rd row is relied and its stamp reads unverified (the scope's "stamps go stale").

@history_item n=10 marker=2026-10-04-dialog-answers-by-dialog-id
**2026-10-04-dialog-answers-by-dialog-id** (decisions on the six-row stamp, the strict stamps read) — `run`'s version gate reads stamps through `read_stamps_strict`. A refusal is `cli_verified:false` with `parse-rejected{ledger-stamps, strict-modes-failed}` (`decision/version-gate-strict-stamps-read`). `stamped_home` stamps at a recorded version. This is the stale-stamp path the plan must name.

@history_item n=11 marker=2026-10-04-readiness-gate-and-timing-constants
**2026-10-04-readiness-gate-and-timing-constants** — made vt100 the pure `viola_agent_claude::screen::Screen`, a pre-send gate only, with modal signatures as compiled literal lists (`decision/vt100-readiness-gate`, `decision/vt100-screen-model`). The sixth run's post-condition ("first settled frame is no trust dialog") reads the same screen model and the modal literal.

@history_item n=12 marker=2026-10-05-real-cli-verify-probes
**2026-10-05-real-cli-verify-probes** (screen signatures compiled, ten ledger rows, verify's interactive runs) — compiled `SIGNATURES` with the modal `Yes, I trust this folder`. It added Run A (untrusted, OS-temp dir, killed, no key into the dialog) and Run B (trusted). Measured on 2.1.288: trust is inherited from a trusted parent. The sixth run is a Run A twin over a fresh OS-temp dir, and its control is Run A's own reading. The inherited-trust fact is a trap: the temp dir must have no trusted ancestor.

@history_item n=13 marker=2026-10-05-real-cli-verify-probes
**2026-10-05-real-cli-verify-probes** (registry rows for verify's interactive runs and screen fixtures) — gave the fake agent `--trusted-root` (argv, no env), Run A's OS-temp `viola-verify-*` dir via `tempfile`, verify's spawn-pair count and the `Screen.<phase>` fixtures. Per the scope's corrected premise, the fake agent decides trust from `--trusted-root` alone, and that is what this chunk changes. A new run moves the spawn-pair count and the probe-dir registry.

@history_item n=14 marker=2026-10-05-dialog-rows-and-re-probe
**2026-10-05-dialog-rows-and-re-probe** — added Runs C and D (four interactive runs), the probe-dir list, and the spawn pairs going from four to six. It is the pattern for the bookkeeping a new interactive run carries: `diagnostics/` spawn pairs, the probe dir list, `[Plugin Scope]`'s transient-children count and CI's fake-agent modes.

@history_item n=15 marker=2026-10-05-permission-end-to-end
**2026-10-05-permission-end-to-end** — kept a question first raised by PermissionRequest `null` because its body has no ledger row, and adding one costs a new row, its probe and a re-stamp. This is the source of `decision/permission-request-question-stays-null`. Its "Nth row" count moves when this chunk takes the 23rd row (it reads 22nd today).

@history_item n=16 marker=2026-10-06-local-command-and-paste-framing-rows
**2026-10-06-local-command-and-paste-framing-rows** — set the precedent that adding rows makes a stamp of the older id set read unverified, and that a Run B widening needs the founder's own live answer. It also re-recorded only one version, which left another drift-only. Both are relevant to the 23rd row staling every 22-row stamp and to S0 stamping 2.1.325 alone.

@history_item n=17 marker=2026-10-07-live-rows-and-paste-shapes-on-the-dev-host
**2026-10-07-live-rows-and-paste-shapes-on-the-dev-host** (the long-paste frame as measured beside typed text and between two pairs) — the founder ruled the limit that a CLI shape no `send` relies on needs no probe (`decision/no-probe-for-unrelied-shapes`). This chunk's shape is relied on by the later product write, so it takes a row with a probe, not that limit.

@history_item n=18 marker=2026-10-07-live-rows-and-paste-shapes-on-the-dev-host
**2026-10-07-live-rows-and-paste-shapes-on-the-dev-host** (the identity floor's Linux reading) — recorded the CLAUDE* names a hook and a session are handed, names only, and that the floor stays at eleven (`decision/identity-floor-eleven-names`, `decision/env-strip-names-only`). Relevant to keeping `CLAUDE_CONFIG_DIR` through `claude_env_keep` for S0 and to the rule that the strip reads names, never values.

@history_item n=19 marker=2026-10-07-send-waits-out-the-paste-hint
**2026-10-07-send-waits-out-the-paste-hint** — set `GATE_MAX_WAIT` to 8.5 s, validated per version by the quiet-period row (`decision/gate-timings-compiled`), and raised the fake agent's hold cap. A settled-frame read in the sixth run uses these compiled timings. Its standing trap: a case reading a verified literal-less screen through a clock that never advances waits forever.

@history_item n=20 marker=2026-10-10-viola-revive
**2026-10-10-viola-revive** (the snapshot's `cwd` and the child's second cwd source) — gave the child a second cwd source, the revived snapshot's `cwd` (`decision/child-cwd-explicit` except). That except is the shape for naming another cwd source. The sixth run sets its own temp dir, and its record key is the cwd, or the cwd's git root inside a repository.

@history_item n=21 marker=2026-10-10-statusline-pass-through
**2026-10-10-statusline-pass-through** (four crossings ratified by the founder) — writing a code-bearing file the CLI reads (the settings override) was a boundary widening the founder ratified live. Standing rule: a widening he answered before the wrap still halts there as one card. The record writer into the CLI's own config file is the same class of crossing, and its expected amendments ride the wrap's answer card.

@history_item n=22 marker=2026-10-10-budget-governor
**2026-10-10-budget-governor** (the instance check before any log, on both arms) — measured that opening a log narrows an existing home to 0700, so a check after it passes a widened home. This is the source of `decision/strict-check-before-log-open` and `decision/refused-check-writes-no-line`. Any new verify path that reads home files must keep its check before the log.

@history_item n=23 marker=2026-10-13-interrupted-verify-cleanup
**2026-10-13-interrupted-verify-cleanup** — verify's dirs are removed when it passes, fails, or is stopped by Ctrl-C or TERM, through drop guards that remove only what that run created, never by a pattern or a listing. A stopped run kills its child with no key, writes no stamp and exits 1 `interrupted`. SIGKILL and a crash leave the dirs to be removed by hand. This is the model for "verify removes ONLY its own record on every exit path" and for saying what a SIGKILLed verify leaves (`decision/verify-removes-only-own-dirs`).

@history_item n=24 marker=2026-10-13-paste-newline-ledger-row
**2026-10-13-paste-newline-ledger-row** — took the ledger to eighteen rows. The step lines `[NN/MM]` take `MM` from the relied count, older-id stamps read unverified, and "every home stamped before reads `cli_verified:false` until its next verify". The new paste was ratified as a boundary widening of the verify child set. This is the template for the 23rd row's step-count sweep and stale-stamp statement.

@history_item n=25 marker=2026-10-13-owed-ledger-rows
**2026-10-13-owed-ledger-rows** — introduced `LedgerRow::relied()` and "a version is verified only when every relied row reads pass" (`decision/version-verified-all-relied-rows`, `contract/ledger-stamps/verified-version`). It also added Run B's probe `--settings`, the probe document verify writes into its own dir. The new row joins the relied set and moves `MM`.

@history_item n=26 marker=2026-10-14-resume-and-compact-ledger-rows
**2026-10-14-resume-and-compact-ledger-rows** (the resume row, Run E and /compact listed) — Run E made verify FIVE interactive runs. It was a boundary widening ratified by the founder's own answer, it runs last in a guarded 0700 dir, and it depends on Run B (`decision/verify-run-e-resumes-run-b-first-session`). This is the direct precedent for the sixth run: its order relative to A–E, its own dir and its widening card.

@history_item n=27 marker=2026-10-14-resume-and-compact-ledger-rows
**2026-10-14-resume-and-compact-ledger-rows** (Run E's resources and a killed PTY child's exit status) — a new run adds a spawn pair (seven then), a probe-dir entry and a fixture stem, and stamps the committed set via `contract_ledger_probes`. A killed interactive child closes with a `child_exit_status`. These are the registry rows the sixth run moves.

@history_item n=28 marker=2026-10-16-unwrapped-sessions-on-the-board
**2026-10-16-unwrapped-sessions-on-the-board** (CLI Version Compatibility: the agents-join row and Run B's agents read) — Run B's read widened verify's child set on the founder's own answer, with "no live start added" and the next row numbered. The fake-agent entry of the same wrap had the founder rule "files beside the exe, no third fake-agent env seam". That ruling constrains how the fake agent learns which config file to read for the record.

@history_item n=29 marker=2026-10-17-cli-output-discipline
**2026-10-17-cli-output-discipline** — the colour reads were ratified as an env-read widening. Standing precedent: "any further environment read … needs its own ratification" (`decision/no-environment-configuration`). If the record writer or the probe reads `CLAUDE_CONFIG_DIR` or `HOME` to find the file the child reads, that read falls under this precedent.

@history_item n=30 marker=2026-10-17-optional-plugin-install
**2026-10-17-optional-plugin-install** — that chunk's live round took none of its readings because both wrapped starts were held by the folder-trust dialog (`proj/` its own git repo). This is the measured trap behind this chunk, and behind the record key being the cwd's git root inside a repository. viola types no key into it (`decision/no-typing-into-trust-dialogs`).

@history_item n=31 marker=2026-10-18-compact-post-condition-ledger-row
**2026-10-18-compact-post-condition-ledger-row** (the twenty-fourth ledger row) — added a row, its probe and a Run B paste. The paste was ratified by the founder's own answer, and his answer-card confirm was PROVISIONAL until his word. The step lines moved, older stamps read unverified, and the committed set was re-stamped. The same sweep (step lines, `--record`, fixtures, the fake agent replay) applies to the 23rd row.

@history_item n=32 marker=2026-10-19-overseer-switch-to-viola
**2026-10-19-overseer-switch-to-viola** (verify's print-mode probe leaves an empty CLI project dir) — the founder accepted the CLI-side residues verify leaves under `~/.claude/projects/` as an accepted class. The sixth run's residue in the CLI's config differs: it must be removed, never left, and the summary must print the proof it is gone.

@history_item n=33 marker=2026-10-19-newer-claude-cli-rows
**2026-10-19-newer-claude-cli-rows** — a newer CLI version needed a recorded fixture set, stamped via `contract_ledger_probes`' `STAMPED` list, and a new fake-agent option, with the option count kept by its list rule. This is relevant to recording and stamping 2.1.325 with the new row and to counting fake-agent options when the fake agent gains record reading.

@history_item n=34 marker=2026-10-20-epoch-5-cleanup-ii
**2026-10-20-epoch-5-cleanup-ii** (the split modules, `relied_here` and the `viola-mcp` serde edge retired) — split `src/cmd/verify/typed.rs` into `typed/runs.rs` (Runs A–E, `trusted_args`, `TrustedDir`), `turns.rs`, `pty.rs` and `run.rs`. This is where the sixth run and its record write and removal land. It also folded `relied_here` into `relied()`.

@history_item n=35 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (the home-tree check before the start's and verify's log) — verify runs `check_home_tree` before its `cli` log. A refusal is exit 1 with the home pair and no line (`decision/verify-refuses-untrusted-home-or-stamps`, `contract/verify-refusals/home-tree`). The sixth run sits after this check, and any new file verify reads must fit the strict-check-before-log order.

@history_item n=36 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (the stamps writer's strict-modes check) — `update_stamps` runs `check_stamps` before creating `ledger/` and again under its lock, and a refusal reads and writes nothing (`decision/stamps-writer-checks-before-and-under-lock`). This is the pattern of "a check before, and again under the lock" for a writer of a shared file, which the record writer's "CLI's lock when free, atomic replace, read-back" race rule parallels.

@history_item n=37 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (the read rule and check_instance's widened set) — defined the strict-modes read rule (the write rule plus no group or other bit, `decision/strict-read-rule`). The record writer's refusals (absent, unreadable, a link, wrong JSON, oversize leading to nothing written) follow the same closed-word style.

@history_item n=38 marker=2026-10-27-pinned-copy-race-and-reproducible-builds
**2026-10-27-pinned-copy-race-and-reproducible-builds** ([Snapshot writer]: the pinned copy's no-clobber placement beside the replace helpers) — added `place_private_new` (`persist_noclobber`) beside `replace_private`. Every replaced file goes through `viola_state::fs`'s shared helpers, and no call site carries its own temp + rename. The record writer's atomic replace must reuse these helpers (`decision/tempfile-snapshot-writer`).

@history_item n=39 marker=2026-10-27-bounded-inputs-at-every-boundary
**2026-10-27-bounded-inputs-at-every-boundary** — set the bounded-readers pattern held by the `tests/contract_bounded_readers.rs` census (`decision/bounded-readers`) and the serde_json `unbounded_depth` ban (`decision/no-unbounded-json-depth`). Reading `.claude.json`, both in the writer and in the fake agent's record read, is a new external read that must be bounded in its statement, or the census goes red. The "oversize leading to nothing written" rule fits this.

@history_item n=40 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** — set the code-word format (`viola_core::is_code_word`, `contract/code-word/format`) and the sanitised-error-surfaces census (`decision/sanitised-error-surfaces`). The probe's closed words for what the writer could not do must be code words and carry no path.

@history_item n=41 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** (`viola verify`'s refusal codes on its `process-exit`) — verify's `process-exit` carries its `Ended::Refused` code. A failing row is no refusal and carries no code, and verify takes no `--json`. The sixth run's "could not write" outcome is a row reading, not a refusal, so it adds no process-exit code unless the plan decides otherwise (`contract/viola-verify-refusal-codes/*`).

@history_item n=42 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (the R8 strip: exact names, `claude_env_keep` the one pass-list) — the strip matches `CLAUDE*` names exactly, `claude_env_keep` is the only pass-list, and the floor's names are removed whatever. This decides whether `CLAUDE_CONFIG_DIR` reaches the child, and so which file the record goes into.

@history_item n=43 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** ([Snapshot writer]: `replace_private` is one attempt) — the Windows retry loop is gone, and a failed `persist` returns at once with the target as it was. The record writer's "at most 3 attempts" is a new retry, of the whole lock, replace and read-back race, and the plan must place it beside this one-attempt helper.

@history_item n=44 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (the relied set: all twenty-four rows on every build) — `LedgerRow::relied()` is every row of `ALL` on every build, so `MM` is the plain row count. The 23rd row is relied everywhere, and the step-count sweep moves one number on Linux only.

@history_item n=45 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (verify's stop: `SIGINT` / `SIGTERM` alone) — verify's cleanup runs on SIGINT and SIGTERM, and the leftovers are SIGKILL, a crash and a failed handler install. The plan must say what a SIGKILLed verify leaves (its own trust record) and which stop removes it.

@history_item n=46 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (child program resolution: a given path as given, a bare name on `PATH`) — this is the form of `decision/child-program-resolution`. S0 runs verify "on the 2.1.325 install by path", a given path taken as given.

@history_item n=47 marker=2026-11-06-version-done-check
**2026-11-06-version-done-check** (the statusline-rate-limits row reads the account's plan state) — a ledger row failed on an unchanged CLI because it read account or home state, not the CLI (`23 pass  1 fail` on the rig home). This is the trap for a row whose verdict depends on the config file the child reads. The probe must control its own record so the row measures the CLI, and its control must be able to fail.

@history_item n=48 marker=2026-11-13-verify-readiness-on-the-input-box
**2026-11-13-verify-readiness-on-the-input-box** — rewrote `decision/screen-signatures-compiled` (ready: no modal row, and a footer literal or the box's own rows) and the screen-fixture rule (`decision/screen-fixtures-signature-rows-only`). It added `box_only_guards` to `contract/ledger-stamps/measured`. The sixth run's "first settled frame is no trust dialog" reads this modal rule, and any recorded screen of it follows the fixture rule.

@history_item n=49 marker=2026-11-14-hyprland-seam
**2026-11-14-hyprland-seam** (Conventions) — added an except to `decision/unparseable-external-values-unknown` for a seam's closed malformed error. It is the shape for the record writer answering unparseable `.claude.json` with a closed word rather than `unknown`.

@history_item n=50 marker=2026-11-14-budget-gate-removed
**2026-11-14-budget-gate-removed** (Established Decisions; Cross-cutting Patterns) — retired the `statusline-rate-limits` row (24 to 23) and moved `decision/permission-request-question-stays-null`'s count. It extended `decision/mixed-version-tolerance` so that a stamp holding a retired row verifies on the relied rows alone. This is how row-count changes ripple, and the converse of this chunk: adding a row stales every older stamp.

@history_item n=51 marker=2026-11-14-status-line-interception-removed
**2026-11-14-status-line-interception-removed** — removed two statusline rows (23 to 21) and widened `decision/protocol-version-bump-rule`'s except (the stamps' row ids removed with `v` kept at 1), later ratified. It narrowed `decision/claude-flag-literals-in-agent-crate` so that only verify reads `--settings`, and left one `--capture` arm. Adding the 23rd row is additive (no `v` bump), and the permission row's "Nth" count sits at 22nd.

@history_item n=52 marker=2026-11-15T18-01-44-wrap
**2026-11-15T18-01-44-wrap** (the widened protocol-version-bump exception ratified by the founder) — `decision/protocol-version-bump-rule`'s widened except stands on the founder's word. This confirms that ledger-row id changes in the stamps keep `v` at 1.

@history_item n=53 marker=2026-11-15-live-round-on-the-trimmed-product
**2026-11-15-live-round-on-the-trimmed-product** — `decision/wheel-starts-with-driver` now names a key the human types into the folder-trust dialog before the session starts. Measured context for the trust dialog this row lets the product avoid. The rig types no key into any dialog.

@history_item n=54 marker=2026-11-15-mutation-and-coverage-baselines-green
**2026-11-15-mutation-and-coverage-baselines-green** — the shared root wait bound `WITHIN` moved to 12 s (`decision/root-waits-share-one-bound`). Any new root test of the record writer or the sixth run that waits on a child shares that bound, under the 20 s lint.

@history_item n=55 marker=2026-11-15-api-error-turn-end
**2026-11-15-api-error-turn-end** (Design Philosophy; Established Decisions; Cross-cutting Patterns) — added the named exception "StopFailure is registered with no ledger row and no verify probe" to `decision/capability-ledger`, `decision/cli-capability-ledger` and `decision/capability-ledger-gate`. The founder ratified it at 2026-11-16T06-05-33-wrap. This is the only standing carve-out to the single-gate rule this chunk upholds by adding a row with a probe.

@history_item n=56 marker=2026-11-16-local-command-delivery
**2026-11-16-local-command-delivery** (Established Decisions, 2 of 2) — added `decision/local-command-echo-probe`, a new relied ledger row with its own verify probe (the 22nd today), ratified at 2026-11-17T05-21-35Z-wrap. It is the latest precedent for adding a relied row plus probe and for the count this chunk takes to 23.

@history_item n=57 marker=2026-11-17-standing-live-rig
**2026-11-17-standing-live-rig** — registered the test-only `viola-live-rig`, `AGENT_RUN_RIG` and the rig's vt100 screen model of its child's trust dialog (`decision/vt100-readiness-gate` except, `decision/vt100-screen-model`). S0 runs on this rig under its private config (`seed_config`), and the scope asks whether the rig has a verify arm.

@history_item n=58 marker=2026-11-17-booted-wrapper-does-not-outlive-its-test
**2026-11-17-booted-wrapper-does-not-outlive-its-test** — registered the test-only self-spawn markers `ROOT_TIED_TEST_CHILD` and `HARNESS_TIED_TEST_CHILD` outside `VIOLA_*` (`decision/harness-env-var-prefix`, `decision/viola-env-namespace-product-only`). Any new test marker this chunk adds follows that naming and registration.

@history_item n=59 marker=2026-11-17-first-readings-on-the-standing-rig
**2026-11-17-first-readings-on-the-standing-rig** — named the rig's test-only exception to `decision/claude-shapes-in-one-crate` and `decision/claude-first-no-adapter-trait`, recorded PROVISIONAL. If the rig or the fake agent reads `.claude.json`'s `projects[<key>].hasTrustDialogAccepted` outside `viola-agent-claude`, that is another Claude shape outside the one crate, and it needs the same kind of named exception.
