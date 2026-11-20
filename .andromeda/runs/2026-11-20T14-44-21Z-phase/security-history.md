@history_item n=1 marker=2026-09-27-wrapper-channel
**2026-09-27-wrapper-channel** (founder ratification of three standing boundary widenings) — set the standing rule that a boundary widening needs the founder's live answer and an earlier direction never ratifies one; any automatic verify form (a spawn by `viola run`, a stamp written with no human, a consent config key) is a widening that must be shown to him whole at P3, not ratified by the overseer's relay alone.

@history_item n=2 marker=2026-09-28-capability-ledger-and-viola-verify
**2026-09-28-capability-ledger-and-viola-verify** (`run`'s and `verify`'s stamps reads before strict-modes) — introduced `read_stamps` for `viola run`'s version gate (unreadable/malformed reads `cli_verified:false` with one `parse-rejected{parser:"ledger-stamps"}`) and `update_stamps` as verify's locked read-modify-write; the gate and stamp writer this chunk rewires, and the origin of "a bad stamps file only degrades `run` to unverified".

@history_item n=3 marker=2026-09-28-capability-ledger-and-viola-verify
**2026-09-28-capability-ledger-and-viola-verify** (`MAX_FRAME` consumers extended, verify's CI split) — the gate's and verify's `--version` reads are capped and killed at 5 s; the real CLI runs only locally and verify runs in CI only against the fake agent (decision/real-cli-local-only, decision/ci-no-secrets-no-real-claude): an automatic verify started by `run` must never reach the real CLI in CI and must be provable over the fake agent.

@history_item n=4 marker=2026-09-28-capability-ledger-and-viola-verify
**2026-09-28-capability-ledger-and-viola-verify** (the hidden `hook --capture` arm, probe captures, fixture scrub) — verify's probe captures are transient 0600 files in a 0700 per-pid probe dir removed on exit, only a scrubbed `--record` copy leaving it (decision/verify-probe-captures-transient); an automatic verify inherits these artefacts under the user's real home.

@history_item n=5 marker=2026-09-29-t15-07-57-wrap
**2026-09-29-t15-07-57-wrap** (registry migration) — moved into the body the accepted risk that a driver LLM can be prompt-injected into `answer` `allow`, viola's part being only the closed `behavior` enum and the `unverified-cli` gate (decision/driver-permission-policy-outside-viola), and the ban on any non-null decision while `run`'s stamps read skips strict-modes; turning a version verified automatically lifts exactly that gate, so this risk rests on the auto-stamp being honest.

@history_item n=6 marker=2026-10-04-dialog-answers-by-dialog-id
**2026-10-04-dialog-answers-by-dialog-id** (run's strict stamps read; decisions on the six-row stamp) — `run`'s gate reads stamps only through `read_stamps_strict` (refusal → `parse-rejected{ledger-stamps, strict-modes-failed}`, `cli_verified:false`), now standing as decision/version-gate-strict-stamps-read and contract/strict-modes-refusal/version-gate; a gate that re-reads stamps after an automatic verify must keep this strict, unlocked, capped read.

@history_item n=7 marker=2026-10-04-the-wheel
**2026-10-04-the-wheel** (pause and release frames) — set the driver-originated `release` refusal (string `from` refused, CLI `viola release` forwarding `VIOLA_NAME` so a driver session exits 20), the source of decision/driver-release-refused, decision/release-from-driver-guard and contract/cli-release/from; an automatic verify must not give a driver a new way to move the wheel or verify state.

@history_item n=8 marker=2026-10-05-real-cli-verify-probes
**2026-10-05-real-cli-verify-probes** (verify's typed-input PTY probe) — verify's interactive children are direct spawns under the R8 strip at 80×24, no byte into any CLI dialog, a modal start killed with no key, nothing written under `~/.claude`; accepted residual: Run B leaves the CLI's transcript and runs the user's global hooks and status line; real-CLI verify needs a trusted folder with its import answered (decision/verify-never-answers-import*). These are the spends and residuals an automatic verify would incur with no human present.

@history_item n=9 marker=2026-10-05-dialog-rows-and-re-probe
**2026-10-05-dialog-rows-and-re-probe** (Runs C and D, R2 closed) — added Runs C and D (the one `touch` in its own dir, `plansDirectory` inside the probe dir, decision/verify-probes-confined-to-own-dir) and retired the dated six-row gap: a stamp short of the relied rows records `cli_verified:false` and `answer` exits 12; precedent for what a partial/RED automatic verify must leave: unverified.

@history_item n=10 marker=2026-10-06-local-command-and-paste-framing-rows
**2026-10-06-local-command-and-paste-framing-rows** (Run B's four compiled pastes; seventeen-row stamp) — Run B's added pastes go only into a settled box with no modal; the transcript residual grows per verify; a stamp of only older ids reads `cli_verified:false`; each widening of the verify child set was the founder's own answer, so changing who or when verify runs falls in that ratified class.

@history_item n=11 marker=2026-10-07-live-rows-and-paste-shapes-on-the-dev-host
**2026-10-07-live-rows-and-paste-shapes-on-the-dev-host** (two scratch measurements across the credential boundary) — kept the identity floor at eleven names (decision/identity-floor-eleven-names) and set the standing rule that a live answer is spent once used; another live probe needs its own showing — a per-version standing permission for live verify (the scope's carried founder word) must be recorded as exactly that grant.

@history_item n=12 marker=2026-10-08-first-live-test-and-self-drive
**2026-10-08-first-live-test-and-self-drive** (the human's terminal on `viola run`'s stdin) — named the stdin vector and the stdin classifier's closed non-editing list (decision/human-typing-takes-wheel, contract/wheel-event/human-input, contract/send-refusal/human-typing); a verify run by `viola run` before or beside the session must not consume or misread the human's stdin.

@history_item n=13 marker=2026-10-10-viola-revive
**2026-10-10-viola-revive** (the revived child: recorded cwd ratified, session id on argv) — revive spawns in the recorded cwd read only after strict-modes, used only as spawn directory (decision/revive-recorded-cwd-spawn-dir, decision/revive-cwd-spawn-only), and puts a closed-shape session id on argv of a direct spawn (decision/session-id-argv-*); revive's start shares the gate this chunk changes, so an auto-verify there must not read the recorded cwd for anything new.

@history_item n=14 marker=2026-10-10-viola-revive
**2026-10-10-viola-revive** (revive reads its own state) — revive's two arms run `check_instance` before any snapshot or log value is used (decision/revive-strict-modes-before-use); an automatic verify at revive's start must sit after that check.

@history_item n=15 marker=2026-10-10-statusline-pass-through
**2026-10-10-statusline-pass-through** (revive: the session id ratified) — the logged session id on `--resume` became a founder-ratified crossing after a recorded-but-unratified interval; precedent that a crossing landed on a delegate's word is written NOT ratified until his own word.

@history_item n=16 marker=2026-10-10-budget-governor
**2026-10-10-budget-governor** (the two instance checks ahead of any log) — measured that a log opened before or after a refused check narrows the home to 0700 and erases the refused state, so the check must run before any log (decision/strict-refusal-leaves-no-trace, decision/home-tree-strict-modes-before-log); trap for an automatic verify started from `run`: its own checks and logs must not reorder or pre-narrow `run`'s.

@history_item n=17 marker=2026-10-13-interrupted-verify-cleanup
**2026-10-13-interrupted-verify-cleanup** (verify's dirs removed on Ctrl-C/TERM) — verify's stop handler is installed by `viola verify` alone, guards remove only what their own run created, SIGKILL/crash leave dirs by hand (decision/verify-probe-dirs-cleanup, decision/probe-artifacts-cleanup); if `viola run` drives verify in-process or as a child, the handler ownership and cleanup guarantees must be re-established.

@history_item n=18 marker=2026-10-13-paste-newline-ledger-row
**2026-10-13-paste-newline-ledger-row** (eighteen stamp rows) — a stamp holding exactly an older id set, all pass, records `cli_verified:false`; a new ledger row invalidates older stamps, so an automatic verify also re-arms whenever the relied set grows, not only when the CLI version moves.

@history_item n=19 marker=2026-10-13-owed-ledger-rows
**2026-10-13-owed-ledger-rows** (Run B's probe --settings, the relied set) — the stamp is read over the relied set; an all-pass shorter stamp reads unverified; the relied set's counting is what the gate checks after any automatic stamp.

@history_item n=20 marker=2026-10-14-resume-and-compact-ledger-rows
**2026-10-14-resume-and-compact-ledger-rows** (Run E) — verify spawns Run E resuming only Run B's captured session id (decision/verify-run-e-resume-target); an automatic verify brings every one of these children, and their transcript residual, into an ordinary `viola run` start.

@history_item n=21 marker=2026-10-16-unwrapped-sessions-on-the-board
**2026-10-16-unwrapped-sessions-on-the-board** (claude agents --json child) — verify's Run B and revive's preflight read `claude agents --json` under the cap with tolerant parse (decision/child-output-bounded, decision/child-output-tolerant*); verify spawn pairs counted (eight); a verify launched from `run` adds these children to its start.

@history_item n=22 marker=2026-10-18-compact-post-condition-ledger-row
**2026-10-18-compact-post-condition-ledger-row** (Run B's sixth paste, twenty-four rows) — Run B's accepted residual is three transcript files per real verify and the stamp gate reads `/24`; the measured per-verify residual an automatic verify would leave in the user's `~/.claude/projects/` each new version.

@history_item n=23 marker=2026-10-19-overseer-switch-to-viola
**2026-10-19-overseer-switch-to-viola** (verify's print-mode probe residual accepted) — the print probe leaves an empty project dir under `~/.claude/projects/`, accepted on the founder's word into the class of verify's trusted-run residuals (decision/verify-real-cli-transcripts-accepted, decision/verify-trusted-runs-risk).

@history_item n=24 marker=2026-10-19-dialog-hold-for-the-driver
**2026-10-19-dialog-hold-for-the-driver** (`dialog_hold_s` validated into `DialogHold`) — precedent for a new `config.json` key: a validated surface admitting new input is a boundary widening the founder answers, read once at start from config only, no env var or flag (decision/dialog-hold-config-only, contract/config-json/dialog-hold-s); a consent/off key for automatic verify would follow this form.

@history_item n=25 marker=2026-10-19-newer-claude-cli-rows
**2026-10-19-newer-claude-cli-rows** (the scratch footer reading of a newer CLI accepted) — one tmpfs-home start of a newer CLI was no product path and no standing permission; another reading needs its own answer (decision/scratch-footer-reading-residual); contrast for this chunk's carried founder word granting one live verify per new version.

@history_item n=26 marker=2026-10-17-cli-machine-contract
**2026-10-17-cli-machine-contract** (revive's --json documents; CLI home resolution) — every verb resolves `--home`, else the `VIOLA_NAME`/`VIOLA_DIR` instance home, else `~/.viola`, canonicalised, nothing created (decision/cli-home-resolution-order, contract/cli-state-root/*); revive's closed `--json` codes; an automatic verify must stamp the home `run` resolved, not another.

@history_item n=27 marker=2026-10-18-plugin-precedence-live-readings
**2026-10-18-plugin-precedence-live-readings** (the VIOLA_DIR home step ratified) — the founder ratified the `VIOLA_DIR` step and canonicalisation of the CLI home; the home a child verify inherits through `VIOLA_DIR` is this ratified resolution.

@history_item n=28 marker=2026-10-17-cli-output-discipline
**2026-10-17-cli-output-discipline** (terminal colour facts join the admitted env set) — added the Universal ban: no further seam or environment read without its own Decisions Log entry (decision/new-env-read-needs-log-entry, decision/controls-not-switchable); an env switch for automatic verify is barred unless ruled.

@history_item n=29 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (`ledger/stamps.json` checked before `ledger/` is made, verify's home tree before its log) — `update_stamps` runs `check_stamps` before creating `ledger/` and again under its lock; verify runs `check_home_tree` before its log; never check after a create/mode set/log open that narrows the path (decision/stamps-checked-under-lock, decision/stamps-strict-check-first, decision/strict-check-before-narrowing, decision/verify-home-tree-check, contract/strict-modes-refusal/verify).

@history_item n=30 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (the home tree checked at `run` and `revive`'s start) — `check_home_tree` runs through `checked_exe` before the wrapper log; refusal is exit 1, no log, no line, no child (decision/start-checks-home-tree-before-log-opens, contract/strict-modes-refusal/run-revive); where an automatic verify slots into `run`'s start must keep this order.

@history_item n=31 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (`check_instance`'s set and the read rule) — the read rule on home, instance and `diagnostics/`, fixed `Readable`/`Symlink` refusal messages (decision/strict-modes-read-rule, decision/strict-modes-before-read, contract/strict-refusal-message/*).

@history_item n=32 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (the ConPTY companions' owner/DACL check) — `run`'s start pins the bytes it already checked (`pin_bytes`) while `pin_exe`, which verify calls, delegates to it; a verify started by `run` touches the same pinned copy.

@history_item n=33 marker=2026-10-27-pinned-copy-race-and-reproducible-builds
**2026-10-27-pinned-copy-race-and-reproducible-builds** (the first-start race closed, a lost placement re-hashed) — concurrent first starts raced the pinned copy (`ETXTBSY`); now placed no-clobber with the write handle closed and a lost placement re-hashed (decision/pinned-copy-content-hash, decision/pinned-binary-rehashed); `run` plus a verify it launches are two concurrent pinners of one key.

@history_item n=34 marker=2026-10-27-bounded-inputs-at-every-boundary
**2026-10-27-bounded-inputs-at-every-boundary** (the bounded-reader census) — every product reader must be bounded or named in `tests/contract_bounded_readers.rs` (decision/bounded-readers-census); any new read the automatic form adds (a failed-attempt record, a consent key) must join the census.

@history_item n=35 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** (the clients' screen of a peer's reply; the error bodies' census) — clients pass on only closed values (decision/client-reads-closed-values-only) and every product error writer must be named in the census (decision/every-error-writer-named, decision/error-messages-interpolate-nothing); a new "verify failed for this version" message or document must be a closed, uninterpolated, censused writer.

@history_item n=36 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** (verify's refusal codes) — verify writes a closed refusal code on its `process-exit`, a refused home tree lineless, a failing row no refusal and no code, no `--json` (contract/verify-refusal-codes/*); how `run` reads an automatic verify's outcome and names failed rows must use these closed words.

@history_item n=37 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** (revive --json documents every refusal of its start) — revive's `--json` closed exit-1 codes cover any refusal of the start it shares with `run`, an anyhow error printing `internal-error` (contract/revive-json/*); a new refusal or outcome of the automatic verify at revive's start must land in this closed set.

@history_item n=38 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** (a refused socket directory at a hook and at a start) — `socket_dir_ready` runs before any snapshot, child or bind at `run` and revive's start (decision/start-checks-socket-dir-before-bind, decision/run-refuses-socket-dir-before-start, contract/socket-dir-refusal/run-and-revive-start); an automatic verify placed before the session must not start a child ahead of this check.

@history_item n=39 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (child program resolution is a given path or a bare name on PATH) — `viola run` resolves `claude` on PATH to an absolute path, `ProgramNotFound` otherwise (decision/run-resolves-absolute-program, decision/claude-resolved-from-path, contract/verify-refusal-codes/program-not-found); verify and run must probe the same resolved `claude` or the stamp keys the wrong version.

@history_item n=40 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (the relied set holds on every build) — `LedgerRow::relied()` became all rows on every build, a `20 pass  4 fail` stamp recording `cli_verified:false`; the relied set the gate demands before the auto-stamp turns the session verified.

@history_item n=41 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (the R8 strip and fixture scrub match names exactly) — every inherited `CLAUDE*` name stripped exactly except `claude_env_keep`, the identity floor always (decision/r8-claude-env-strip, contract/config-json/claude-env-keep); a verify child started by `run` must get the same strip.

@history_item n=42 marker=2026-11-06-version-done-check
**2026-11-06-version-done-check** (statusline-rate-limits reads the account's plan) — on an unchanged CLI a verify read a row red because it measured the account, not the CLI; trap for the RED-verify design: a failed automatic verify is not always a CLI change, so the failed-attempt record and what clears it must allow for non-CLI causes.

@history_item n=43 marker=2026-11-13-verify-readiness-on-the-input-box
**2026-11-13-verify-readiness-on-the-input-box** (Input Validation) — added decision/input-box-readiness-from-literals: one compiled-literal predicate decides readiness for verify and `viola run`'s pre-send gate, a ratified widening.

@history_item n=44 marker=2026-11-14-budget-gate-removed
**2026-11-14-budget-gate-removed** (Input Validation 1 of 2; Authentication & Authorization) — `config.json` lost `five_hour`/`seven_day` and decision/config-cannot-disable-controls dropped the budget clause; the release rows (decision/driver-release-refused, decision/release-from-driver-guard) reworded; precedent for changing the config key set and for config never switching a control off.

@history_item n=45 marker=2026-11-14-status-line-interception-removed
**2026-11-14-status-line-interception-removed** (Input Validation 1 of 2) — reworded decision/max-frame-shared-cap and decision/home-strict-modes-before-read after removing a reader; the current text of the caps and pre-read checks a new reader must join.

@history_item n=46 marker=2026-11-15-api-error-turn-end
**2026-11-15-api-error-turn-end** (Authentication & Authorization) — gave decision/unverified-cli-gate its first named exception (StopFailure registered with no ledger row and no verify probe), then provisional; the gate row this chunk rests on.

@history_item n=47 marker=2026-11-16T06-05-33-wrap
**2026-11-16T06-05-33-wrap** (Authentication & Authorization; the StopFailure no-ledger-row answer ratified) — the unverified-cli-gate exception ratified by the founder, no longer provisional.

@history_item n=48 marker=2026-11-15-live-round-on-the-trimmed-product
**2026-11-15-live-round-on-the-trimmed-product** (Authentication & Authorization; Data Protection) — added decision/live-round-rig-keys-own-pty (founder's word anew per chunk for the rig's keys) and decision/live-session-screens-kept-local (live screens only under a gitignored local path); bind any live start this chunk makes under the rig.

@history_item n=49 marker=2026-11-16-local-command-delivery
**2026-11-16-local-command-delivery** (Security Anti-Patterns; Input Validation; Data Protection) — set decision/compiled-classification-sources, decision/transcript-read-bounded, decision/transcript-path-redacted and decision/verify-echo-probe-residuals (Run B's seventh paste may touch the user's real Claude config, unmeasured); another residual an automatic verify brings into an ordinary start.

@history_item n=50 marker=2026-11-17-standing-live-rig
**2026-11-17-standing-live-rig** (Authentication & Authorization; Secret Management) — the standing rig's closed argv key grant, decision/live-rig-credentials-copy and decision/live-rig-isolated-child; the scope's "any live start is a scratch viola home under the rig" lands under these rows.

@history_item n=51 marker=2026-11-17-first-readings-on-the-standing-rig
**2026-11-17-first-readings-on-the-standing-rig** (Secret Management; Authentication & Authorization) — the rig reads the user's config only by a rig-made-needle mark witness, and the rig's stop needs no per-chunk word while its argv grant does (current text of decision/live-rig-isolated-child, decision/live-round-rig-keys-own-pty).

@history_item n=52 marker=2026-11-17-trust-record-ledger-row
**2026-11-17-trust-record-ledger-row** (Data Protection 3 rows; Threat Model Summary) — verify writes, then removes, a trust record for its own temp folder in the Claude config, a SIGKILL leaving it (except of decision/no-writes-under-claude-home, decision/viola-writes-nothing-in-claude-dir, decision/nothing-written-under-claude-home); an automatic verify writes into the user's Claude config with no human present.

@history_item n=53 marker=2026-11-17-trust-record-ledger-row
**2026-11-17-trust-record-ledger-row** (Authentication & Authorization) — decision/explicit-mode-before-write gained its except: the trust record writer keeps Claude's config file's own mode.

@history_item n=54 marker=2026-11-17-trust-record-ledger-row
**2026-11-17-trust-record-ledger-row** (Secret Management 4 rows) — verify reads `CLAUDE_CONFIG_DIR`'s value only when kept, the one bounded value read (excepts of decision/r8-claude-env-strip, decision/credential-strip-by-name-only, decision/env-not-config-channel, decision/no-stored-secrets).

@history_item n=55 marker=2026-11-17-trust-record-ledger-row
**2026-11-17-trust-record-ledger-row** (Input Validation; Data Protection 2 rows) — decision/trust-record-closed-words (outcome stamped only as a closed word) and the cleanup excepts naming verify's trust record left by a kill (decision/verify-probe-dirs-cleanup, decision/probe-artifacts-cleanup).

@history_item n=56 marker=2026-11-18-trust-record-row-read-live
**2026-11-18-trust-record-row-read-live** (Data Protection; Threat Model Summary; Input Validation; Secret Management) — the rig-only first-screen record except on decision/verify-signature-only and decision/verify-screen-read-against-literals, decision/verify-stamp-first-screen-closed-words (the stamp's first screen as closed words only), and decision/live-rig-seed-from-bundle-logic; what an automatic stamp may hold.

@history_item n=57 marker=2026-11-18-session-folder-trusted-by-viola
**2026-11-18-session-folder-trusted-by-viola** (Data Protection 4 rows; Data Protection 1 row) — on a verified CLI only, `run` and revive's start write the trust flag for an untrusted spawn folder, which stays, its price (a human's earlier no overridden) ratified (decision/start-writes-trust-record-flag-only, decision/trust-record-writer-guarded); an automatic verify that flips a session verified mid-start also switches on this start write.

@history_item n=58 marker=2026-11-18-session-folder-trusted-by-viola
**2026-11-18-session-folder-trusted-by-viola** (Input Validation; Threat Model Summary) — the recorded/spawn cwd also keys the start's trust record (decision/revive-recorded-cwd-spawn-dir, decision/revive-cwd-spawn-only, decision/cwd-only-in-snapshot, decision/spawn-cwd-snapshot-only), never on a log line, error body or stdout.

@history_item n=59 marker=2026-11-18-session-folder-trusted-by-viola
**2026-11-18-session-folder-trusted-by-viola** (Secret Management, 1 of 2 and 2 of 2; Input Validation) — run and revive's start read `CLAUDE_CONFIG_DIR`'s value too, through verify's shared resolution; the start's trust step ends in one of 16 closed start words (decision/trust-record-closed-words).

@history_item n=60 marker=2026-11-18-browser-tools-prompt-modal
**2026-11-18-browser-tools-prompt-modal** (Authentication & Authorization; Security Anti-Patterns ×2) — the verified rule became every relied row counts: pass, or absent on the browser-tools-prompt row alone (decision/unverified-cli-gate, decision/dialog-answers-need-verify-stamp, contract/cli-exit-codes/12); the exact predicate an automatic stamp must meet before the session goes on verified.

@history_item n=61 marker=2026-11-18-the-2-1-325-recorded-fixture-set
**2026-11-18-the-2-1-325-recorded-fixture-set** (Authentication & Authorization) — moved the unverified-cli-gate exception's owner to its own entry.

@history_item n=62 marker=2026-11-18-the-live-home-s-re-stamp-and-the-product-re-pin
**2026-11-18-the-live-home-s-re-stamp-and-the-product-re-pin** (Threat Model Summary) — decision/verify-never-answers-dialogs: a browser-tools prompt standing at start is left for the human, verify saying so before its summary; an automatic verify with no human watching must still surface that line somewhere the human reads.

@history_item n=63 marker=2026-11-18-the-live-home-s-re-stamp-and-the-product-re-pin
**2026-11-18-the-live-home-s-re-stamp-and-the-product-re-pin** (Secret Management ×2) — the rig's VH arm runs under the real home and config, the one exception of decision/live-rig-isolated-child and decision/live-rig-credentials-copy; the precedent for any stamp written into the live home, which this chunk's scope forbids at the phase.

@history_item n=64 marker=2026-11-19-a-live-stopfailure-payload
**2026-11-19-a-live-stopfailure-payload** (Authentication & Authorization ×2) — the unverified-cli-gate except reads «StopFailure is registered with no capability ledger row and no verify probe.», settled on the founder's word; current text of the gate row.

@history_item n=65 marker=2026-11-19-context-as-a-local-command
**2026-11-19-context-as-a-local-command** (Security Anti-Patterns) — decision/stamps-written-only-by-verify gained its first except (the test-only rig seeds a kept stamp into its own scratch home); the row this chunk extends if `run` triggers the stamp write, and the precedent that any other stamp writer is a named exception.

@history_item n=66 marker=2026-11-19-context-as-a-local-command
**2026-11-19-context-as-a-local-command** (Data Protection; Threat Model Summary) — decision/verify-context-probe-residual (Run B's /context, residual accepted only as measured) and decision/live-rig-kept-stamp-owner-only; contract/wrapper-channel-methods/send now answers a local command on an unverified CLI as unconfirmable — behaviour that changes once a version is auto-verified.

@history_item n=67 marker=2026-11-19-a-fired-scheduled-wake-keeps-the-wheel
**2026-11-19-a-fired-scheduled-wake-keeps-the-wheel** (Security Anti-Patterns; Data Protection; Authentication & Authorization) — widened decision/stamps-written-only-by-verify's except to CC or R3W, decision/live-rig-kept-stamp-owner-only, and added decision/live-rig-hb-arm-keeps-attributes-only.

@history_item n=68 marker=2026-11-20-a-quoted-modal-never-blocks-a-send
**2026-11-20-a-quoted-modal-never-blocks-a-send** (Input Validation ×3; Threat Model Summary) — decision/input-box-readiness-from-literals reworded to «never while a standing modal is up» with its quote except, and verify's readings now «compiled signatures: the literals, and the compiled shapes over them» (decision/verify-reads-only-signatures, decision/verify-pty-output-caught, decision/verify-screen-read-against-literals, decision/verify-signature-only); still PROVISIONAL until the founder's word.

@history_item n=69 marker=2026-11-20-permissionrequest-question-answered
**2026-11-20-permissionrequest-question-answered** (Data Protection) — decision/live-rig-kept-stamp-owner-only now names PQ; the last change to the kept-stamp rule (the 27-row 2.1.328 stamp the scope cites stands only in the rig's kept stamp), PROVISIONAL.
