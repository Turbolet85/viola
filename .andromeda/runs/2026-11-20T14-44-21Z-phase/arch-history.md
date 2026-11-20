@history_item n=1 marker=2026-09-28-capability-ledger-and-viola-verify
**2026-09-28-capability-ledger-and-viola-verify** (the landed ledger rows, `viola verify`'s probe, the stamps envelope) — created the closed ledger, `viola verify [--record] [-- <program>]`, the stamps envelope `{"v":1,"writer":"verify",…}` whose merge keeps unknown versions and fields, and its one writer `update_stamps`; a failing row still stamps and exits 1; `--version` is read bounded (5 s kill, first line `X.Y.Z (Claude Code)`). This is the whole machinery an automatic stamp must call or reuse, and the precedent that a red verify still writes a stamp, which bears on how a failed automatic attempt is recorded and not repeated.

@history_item n=2 marker=2026-09-28-capability-ledger-and-viola-verify
**2026-09-28-capability-ledger-and-viola-verify** (the version gate placed and split by crate) — set the start order with `run::version_gate` (a `version-probe` pair, the stamps read, `cli_version` / `cli_verified` into the first snapshot, nothing printed) and split the gate three ways: parsing and verdict in the pure `viola-agent-claude`, the spawn in the root `run::version_gate`, stamps I/O in `viola_state::stamps`. Any automatic stamp hooks into this gate and must keep that split, since `viola-agent-claude` stays pure.

@history_item n=3 marker=2026-09-29-verify-stamped-test-homes-and-harness
**2026-09-29-verify-stamped-test-homes-and-harness** (verify's spawn pairs, the harness CI read, verify-stamped homes) — harness homes are stamped by `viola verify` against the fake agent; stamps come only from `viola verify`; `run --local-live` refuses under CI (`live-in-ci`), and the real CLI never runs in CI. This is the constraint behind decision/no-live-claude-in-ci and decision/real-claude-local-only. An automatic verify started by `run` must not spawn the real CLI in CI, and its tests must stay on the fake agent.

@history_item n=4 marker=2026-10-04-dialog-answers-by-dialog-id
**2026-10-04-dialog-answers-by-dialog-id** (the strict stamps read) — `run`'s gate reads `ledger/stamps.json` through `read_stamps_strict`; a strict-modes refusal gives `cli_verified:false` and one `parse-rejected{parser:"ledger-stamps", detail:"strict-modes-failed"}`. This is the origin of decision/version-gate-strict-stamps-read, and the read path an automatic stamp has to re-read after it lands.

@history_item n=5 marker=2026-10-05-real-cli-verify-probes
**2026-10-05-real-cli-verify-probes** (screen signatures compiled, ten rows, verify's interactive runs) — `run` passes `Some(&SIGNATURES)` exactly when `cli_verified`, so the full gate runs on a verified version and the partial gate otherwise; verify gained the PTY Runs A and B. Each widening of verify's child set was ratified by the founder live. A session that turns verified after it starts changes which gate `run` runs, and the precedent is that any change to verify's runs is a founder-ratified widening.

@history_item n=6 marker=2026-10-05-real-cli-verify-probes
**2026-10-05-real-cli-verify-probes** (registry rows for verify's interactive runs) — verify's transient children run the user's global hooks and status line and leave CLI transcripts, an accepted residual; its probe dirs are 0700 and removed on every exit path (Run B's `<cwd>/.viola-verify-<pid>/`). An automatic verify carries these same residuals and cwd-relative dirs into the user's project folder with no human present, so its form at P3 must state them.

@history_item n=7 marker=2026-10-05-permission-end-to-end
**2026-10-05-permission-end-to-end** — priced one new row as "its verify probe and a re-stamp of both versions (at least 8 live sessions)" and declined it on the live-session cap. This is a precedent for what one verify costs in live sessions, which bears on the scope's spend and consent question.

@history_item n=8 marker=2026-10-06-local-command-send-outcomes
**2026-10-06-local-command-send-outcomes** (send consumes the local-command list) — the verified bit reaches `send` from the version gate's `cli_verified` through `SendSlot::new`, and the send path reads no `ledger/stamps.json`. This is a trap: a stamp that lands while a session runs does not reach that session's send slot unless the chunk re-reads the stamps or rebuilds the slot (the "beside the session" candidate form).

@history_item n=9 marker=2026-10-07-send-waits-out-the-paste-hint
**2026-10-07-send-waits-out-the-paste-hint** — on a verified CLI the gate waits for the input box up to `GATE_MAX_WAIT` 8.5 s; on an unverified one it stays partial. Its trap: a case that reads a verified, literal-less screen through a clock that never advances waits forever. This bears on tests that flip a session to verified (decision/partial-gate-on-unverified-cli, decision/send-waits-out-paste-hint).

@history_item n=10 marker=2026-10-10-viola-revive
**2026-10-10-viola-revive** (the replay's first product reader, revive as the second way a wrapper starts) — a passing `viola revive` goes through the same start as `run`, the collision check running twice. Its program resolution and version gate never read the recorded directory (decision/revive-program-resolution). A gate-time automatic stamp therefore reaches revive's start too, as the scope's "where the version gate is shared" says.

@history_item n=11 marker=2026-10-13-interrupted-verify-cleanup
**2026-10-13-interrupted-verify-cleanup** — verify's Ctrl-C / TERM stop (`cmd::verify::stop`) is installed by `viola verify` alone. A stopped run kills its child with no key, writes no stamp, exits 1 `unable: verify was stopped before it finished` with `process-exit … detail:"interrupted"`, and removes only what that run created. An automatic verify run in-process by `run` would meet a handler `run` never installs and a terminal `run` owns, so the stop behaviour must be re-decided for it.

@history_item n=12 marker=2026-10-13-paste-newline-ledger-row
**2026-10-13-paste-newline-ledger-row** — when a row is added, a stamp of the older ids reads unverified, and "every home stamped before this chunk reads `cli_verified:false` until its next `viola verify`". This is the same unverified state the scope wants handled automatically, now caused by a row change rather than a CLI move; an automatic form keyed on "no stamp for this version" misses a stamp of a shorter row set unless it keys on the relied-row verdict.

@history_item n=13 marker=2026-10-13-owed-ledger-rows
**2026-10-13-owed-ledger-rows** (three statusline rows and the relied set) — introduced `LedgerRow::relied()`: a version is verified only when every relied row reads `pass` (it had been every landed row), and `[NN/MM]` counts the relied rows. This is the origin of decision/version-verified-all-relied-rows, the verdict an automatic stamp must reach.

@history_item n=14 marker=2026-10-14-resume-and-compact-ledger-rows
**2026-10-14-resume-and-compact-ledger-rows** (the resume row, Run E) — "a verify is six `claude` starts", and Run E resumes Run B's session in a guarded `-resume/` dir. This measures the model turns an automatic verify spends, the scope's spend question.

@history_item n=15 marker=2026-10-16-unwrapped-sessions-on-the-board
**2026-10-16-unwrapped-sessions-on-the-board** (the agents-join row and Run B's agents read) — added the `agents-join` row and verify's one `claude agents --json` read inside Run B. `list` and `revive` gate their agents read on no stamp (decision/agents-join-read-ungated). This shows which start-time reads do not depend on a stamp and so are unaffected by an automatic one.

@history_item n=16 marker=2026-10-17-mcp-server-for-drivers
**2026-10-17-mcp-server-for-drivers** (MCP list builds the board and reads agents) — "a new process spawning `claude`" was recorded as a boundary widening and ratified by the founder's own answer. This is the precedent that `viola run` spawning `viola verify` (and so `claude`) is a widening the founder rules at P3.

@history_item n=17 marker=2026-10-17-cli-output-discipline
**2026-10-17-cli-output-discipline** — the colour reads were ratified by the founder, with the standing precedent that "any further environment read … needs its own ratification". This applies if the automatic form's consent or opt-out takes an environment variable rather than a config key (decision/no-environment-configuration).

@history_item n=18 marker=2026-10-17-cli-machine-contract
**2026-10-17-cli-machine-contract** (revive takes --json on both arms) — registered `schemas/config.v1.json` as the tolerant `config.json` schema of the `CONFIG_KEYS` set. Under decision/config-keys-match-schema, a consent key for automatic stamping must land in both `CONFIG_KEYS` and that schema.

@history_item n=19 marker=2026-10-18-compact-post-condition-ledger-row
**2026-10-18-compact-post-condition-ledger-row** (Run B's sixth paste; Run B's transcript files) — added the 24th row (older stamps read unverified, so every home must re-verify) and recorded that a real-CLI verify leaves Run B three transcript files and C and D one each. These are the residuals an unattended verify leaves in the user's Claude projects dir (decision/verify-transcripts-accepted-residual).

@history_item n=20 marker=2026-10-19-dialog-hold-for-the-driver
**2026-10-19-dialog-hold-for-the-driver** — added the `config.json` key `dialog_hold_s` (bounded, a bad value falls back with one parse-rejected line), ratified by the founder as a boundary widening. This is the precedent and the pattern for a consent or on/off key for the automatic verify (decision/config-bad-value-falls-back).

@history_item n=21 marker=2026-10-19-overseer-switch-to-viola
**2026-10-19-overseer-switch-to-viola** (verify's print-mode probe leaves an empty CLI project dir) — 4 new `~/.claude/projects/` entries per verify, accepted by the founder into the residual class. This is another unattended residual an automatic verify leaves on every new version.

@history_item n=22 marker=2026-10-19-newer-claude-cli-rows
**2026-10-19-newer-claude-cli-rows** — on a newer CLI (2.1.302) `for agents` vanished 15 s after the start, so a two-screen row passed while `run`'s gate would refuse sends on a verified 2.1.302; the row was widened and the literal added. Trap: a new version stamped automatically can pass rows that miss a drift appearing later, which is the risk of trusting a stamp with no human looking.

@history_item n=23 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (the home-tree check before the start's and verify's log; the pin key taken once) — every start runs `check_home_tree` with its name and pin key through `checked_exe` before its wrapper log; verify runs it with no name and no key before its `cli` log; `pin_key` / `pin_bytes` / `pin_exe` were named. An automatic verify inside a start has to fit these two check orders and the one pinned copy (decision/start-pins-the-bytes-it-checked).

@history_item n=24 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (the stamps writer's strict-modes check; `StampsError`) — `update_stamps` runs `check_stamps` before creating `ledger/` and again under its lock; a refusal reads and writes nothing, and verify maps it to exit 1 with the home pair. This is the origin of decision/stamps-writer-checks-before-and-under-lock and decision/verify-refuses-untrusted-home-or-stamps, which bind any new caller of the writer.

@history_item n=25 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (the strict-modes refusals' exits) — exit 1 covers `run`'s and `revive`'s start refusing the home tree, and `verify` refusing it or a refused stamps update, each writing no line. These are the exit and no-line shapes an automatic verify's refusals inside a start must keep (contract/cli-exit-codes/1, contract/verify-refusals/home-tree).

@history_item n=26 marker=2026-10-27-pinned-copy-race-and-reproducible-builds
**2026-10-27-pinned-copy-race-and-reproducible-builds** (the pinned copy placed once with no clobber) — two concurrent first starts raced on the pinned copy (ETXTBSY), fixed by `place_private_new`, where a lost placement re-hashes the copy that stands. This is a precedent for concurrent starts meeting the same new state: two `viola run`s on one unstamped version could both start an automatic verify, which the stamps lock and merge (decision/stamps-merge-own-version-only) must survive.

@history_item n=27 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** (`viola revive --json` documents every refusal of its start) — a refusal of the start shared with `run` prints its exit-1 document under `revive --json`, and a `pump_child` error prints none because the child has held the terminal. Any new refusal an automatic verify adds to the start needs its code in this closed set (contract/viola-revive-json-error-codes/*, decision/no-document-after-child-held-terminal).

@history_item n=28 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** (`viola verify`'s refusal codes on its `process-exit`) — verify's `process-exit` under a parsed `VIOLA_NAME` carries its `Ended::Refused` code beside `interrupted`; a refused home tree writes no line; a failing row is no refusal; verify takes no `--json`. These are the contract/viola-verify-refusal-codes/* rows an automatic verify's outcome reporting must reuse; note that verify writes under `VIOLA_NAME`, which a wrapped start sets.

@history_item n=29 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (the relied set, all twenty-four rows on every build) — `relied()` is every row on every build; `[NN/MM]` uses one count; older id sets read unverified. This removes any per-OS branch from the verdict an automatic stamp must reach.

@history_item n=30 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (verify's stop, `SIGINT` / `SIGTERM` alone) — verify's probe dirs are removed on `SIGINT` / `SIGTERM`; SIGKILL, a crash and a failed handler install leave them. These are the leftovers an unattended verify can strand.

@history_item n=31 marker=2026-11-06-version-done-check
**2026-11-06-version-done-check** (the statusline-rate-limits row reads the account's plan state) — `viola verify` failed a row on an unchanged CLI (`23 pass  1 fail`) because of the account, not the CLI. Trap for the RED path: an automatic verify can go red for reasons outside the CLI version, so "a failed attempt is not repeated" needs a clearing rule that is not just a CLI version change.

@history_item n=32 marker=2026-11-13-verify-readiness-on-the-input-box
**2026-11-13-verify-readiness-on-the-input-box** — `typed_probe` gained `box_only_guards` (contract/ledger-stamps/measured: additions need no `v` bump, and `run` reads no field of it). This is the precedent if the chunk records a failed automatic attempt inside the stamps envelope rather than in a file of its own.

@history_item n=33 marker=2026-11-14-budget-gate-removed
**2026-11-14-budget-gate-removed** (Conventions) — added decision/retired-exit-codes-never-reused (exit 11 stays unassigned) and the bump-rule exception. Any new refusal or exit for an automatic verify must take a fresh code, never 11.

@history_item n=34 marker=2026-11-14-budget-gate-removed
**2026-11-14-budget-gate-removed** (Cross-cutting Patterns) — decision/mixed-version-tolerance now says "a stamp holding a retired ledger row verifies on the relied rows alone". This is how an older stamp is read, which bears on whether an existing stamp counts as "stamped" for the automatic trigger.

@history_item n=35 marker=2026-11-14-status-line-interception-removed
**2026-11-14-status-line-interception-removed** (Conventions; Occupied Resources) — removing the stamps' two status-line row ids kept every own-format `v` at 1 (the protocol-bump except), and the `--settings` literal's one reader is verify. Any stamp-envelope field the chunk adds, such as a failed-attempt record, follows this additive, no-bump rule.

@history_item n=36 marker=2026-11-15-api-error-turn-end
**2026-11-15-api-error-turn-end** (Design Philosophy; Established Decisions) — added the StopFailure exception to decision/capability-ledger and decision/cli-capability-ledger (registered with no ledger row and no verify probe). An automatic verify stamps everything except this exception, so StopFailure behaviour on a new version stays unprobed.

@history_item n=37 marker=2026-11-19-a-live-stopfailure-payload
**2026-11-19-a-live-stopfailure-payload** (the StopFailure exception settled on the live reading) — the exception is no longer PROVISIONAL, kept on one live reading on 2.1.328 "one word … on one version; nothing is claimed for … another version". This is the standing caveat on the picked rows' except clause, relevant now that new versions are stamped with no human reading them.

@history_item n=38 marker=2026-11-16-local-command-delivery
**2026-11-16-local-command-delivery** (local-command echo confirmation and its probe) — added the `local-command-echo` relied row (Run B's seventh paste) and the unconfirmable cases, a stdout-echo command with no readable transcript or any listed command on an unverified CLI. This is the send behaviour that changes when a session turns verified (contract/send-results/unconfirmable, decision/local-command-echo-probe).

@history_item n=39 marker=2026-11-17-standing-live-rig
**2026-11-17-standing-live-rig** — landed `viola-live-rig` (test-only, never released) and `AGENT_RUN_RIG`. The scope requires that any live start is a scratch viola home under this rig, never the live home.

@history_item n=40 marker=2026-11-17-trust-record-ledger-row
**2026-11-17-trust-record-ledger-row** (Established Decisions; Occupied Resources) — Run F made verify six interactive runs (decision/verify-child-set, a ruled widening). Verify now writes and removes its own trust record in the Claude config file, reads `CLAUDE_CONFIG_DIR` when kept, and the cleanup decision covers that record. An unattended verify therefore writes into the user's Claude config, part of the consent the founder rules at P3.

@history_item n=41 marker=2026-11-17-trust-record-ledger-row
**2026-11-17-trust-record-ledger-row** (Standard Contracts) — a stamp's trust-record value may be a trust-writer code word instead of `pass` / `fail` (contract/ledger-stamps/rows and content, contract/trust-writer-words/*). These are outcomes an automatic verify can stamp, and a non-`pass` word leaves the version unverified.

@history_item n=42 marker=2026-11-18-trust-record-row-read-live
**2026-11-18-trust-record-row-read-live** — added the hidden rig-only `--record-first-screens` (a failed `--record` verify's whole-row first screens), `measured.first_screen` and `first_settle_ms`. These are the first-screen readings (trust / imports / browser-tools / box / none / exited) that tell why a new version's verify went red (contract/ledger-stamps/first-screen, contract/viola-cli-arguments/verify).

@history_item n=43 marker=2026-11-18-session-folder-trusted-by-viola
**2026-11-18-session-folder-trusted-by-viola** (Established Decisions) — the start order gained "on a verified CLI writes the trust record of an untrusted spawn folder" after the version gate and before the bind. A version that turns verified by an automatic stamp before the session starts also turns this step on, and one stamped beside the session does not (decision/run-start-order).

@history_item n=44 marker=2026-11-18-session-folder-trusted-by-viola
**2026-11-18-session-folder-trusted-by-viola** (Occupied Resources; Standard Contracts) — the `run` and `revive` start now read `CLAUDE_CONFIG_DIR` when kept; `key-present` / `removed` are verify-only words, read by a start as `lost`. This settles which trust-writer words a start and a verify each give, which matters if `run` drives verify (decision/env-strip-names-only, contract/trust-writer-words/key-present).

@history_item n=45 marker=2026-11-18-browser-tools-prompt-modal
**2026-11-18-browser-tools-prompt-modal** — an `absent` browser-tools-prompt counts toward verified: decision/capability-ledger, decision/cli-capability-ledger, decision/version-verified-all-relied-rows and contract/ledger-stamps/verified-version were reworded, and the step line reads `pass|absent|fail` with the summary counting absent. The automatic verdict "goes on verified" must use this counting rule, not "all pass".

@history_item n=46 marker=2026-11-18-the-live-home-s-re-stamp-and-the-product-re-pin
**2026-11-18-the-live-home-s-re-stamp-and-the-product-re-pin** — the precedent chunk for re-stamping the live home by a run held to the rig and to a cap. Per the scope, the live home still holds a 24-row 2.1.328 stamp; this chunk's form must say whether it ever touches the live home, and what it does in its tests.

@history_item n=47 marker=2026-11-19-context-as-a-local-command
**2026-11-19-context-as-a-local-command** — added decision/local-command-context-probe (Run B's eighth paste, `/context`), ratified as a verify widening, and the unconfirmable case for any listed local command on an unverified CLI. One more relied row the automatic verify must pass.

@history_item n=48 marker=2026-11-20-permissionrequest-question-answered
**2026-11-20-permissionrequest-question-answered** — replaced the null rule with decision/permission-request-question-answered and its own relied row and probe (the 27th relied row): an unverified CLI leaves the question to the human. This is the latest row whose stamp gap forces every existing stamp to re-verify, so an automatic form fires on row additions as well as on CLI moves.

@history_item n=49 marker=2026-11-20-a-quoted-modal-never-blocks-a-send
**2026-11-20-a-quoted-modal-never-blocks-a-send** — reworded decision/screen-signatures-compiled and decision/vt100-readiness-gate to "a standing modal", with its quote except. These are the rules of the full gate that a session turned verified starts running.

@history_item n=50 marker=2026-09-25-pty-wrapper-on-windows
**2026-09-25-pty-wrapper-on-windows** — kept "while the child runs, `run` writes nothing except the child's own output" (decision/run-terminal-child-only) and the R8 strip with `claude_env_keep`. A verify run beside the session cannot print to the terminal, and one run before the session prints before the child holds it.
