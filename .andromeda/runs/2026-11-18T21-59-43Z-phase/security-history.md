@history_item n=1 marker=2026-09-27-epoch-2-cleanup
**2026-09-27-epoch-2-cleanup** — release-check refuses `test-support` / `fake-agent` artifacts (probe 5/5). This is the guard the re-pin's rebuild (decision/release-build-without-test-features, decision/release-build-no-test-binaries) must pass: build the pinned product only through `scripts/release-check.sh`, never a fake-agent build.

@history_item n=2 marker=2026-09-28-capability-ledger-and-viola-verify
**2026-09-28-capability-ledger-and-viola-verify** — introduced the stamps file, the `hook --capture` arm, the probe captures and the `verify --record` scrub. Also the founder's ruling that only `viola verify` writes `ledger/stamps.json`. This is where decision/stamps-written-only-by-verify comes from: no hand restore of a stamp, and a failed live verify still replaces the 2.1.325 entry.

@history_item n=3 marker=2026-10-04-dialog-answers-by-dialog-id
**2026-10-04-dialog-answers-by-dialog-id** — `run`'s version gate reads stamps through `read_stamps_strict`; a refusal, unreadable or malformed file gives `cli_verified:false`. The dated stamp gate came with it. This is the origin of decision/version-gate-strict-stamps-read and contract/strict-modes-refusal/version-gate, which the re-pinned product reads against the new 24-row stamp.

@history_item n=4 marker=2026-10-05-real-cli-verify-probes
**2026-10-05-real-cli-verify-probes** — the real-CLI interactive verify. Children are direct spawns under R8 at 80×24 and no byte goes into any dialog; a modal start is killed with no key. Run B's transcripts are an accepted residual. Dev-host prerequisite: a trusted folder whose external import is already answered. Fixture refusal uses closed codes. This is the base for decision/verify-never-answers-dialogs, verify-never-answers-import and verify-real-cli-transcripts-accepted, all of which the live 24-row verify on the founder's real home runs under.

@history_item n=5 marker=2026-10-05-dialog-rows-and-re-probe
**2026-10-05-dialog-rows-and-re-probe** — Runs C and D: the hook answers, one `touch` in its own dir, and `plansDirectory` inside the probe dir (no plan file under `~/.claude/plans`, measured). Basis of decision/verify-probes-confined-to-own-dir. The real-home verify must leave `~/.claude/plans` unchanged.

@history_item n=6 marker=2026-10-06-local-command-and-paste-framing-rows
**2026-10-06-local-command-and-paste-framing-rows** — Run B's compiled pastes, `/clear` among them. Each paste goes only into a settled input box with no modal; the stamp gained rows, and a stamp of the older ids reads `cli_verified:false`. Precedent for the trap that every row added makes all older stamps unverified (C3/C4).

@history_item n=7 marker=2026-10-07-live-rows-and-paste-shapes-on-the-dev-host
**2026-10-07-live-rows-and-paste-shapes-on-the-dev-host** — two scratch measurements across the credential boundary, each ratified once. Standing rule: answers are spent, and another probe needs its own showing. This applies to each live start and to the PREREQ key-name read of the real `~/.claude.json`.

@history_item n=8 marker=2026-10-08-first-live-test-and-self-drive
**2026-10-08-first-live-test-and-self-drive** — the human-terminal stdin vector. The closed non-editing list was widened once with measured reply shapes, and no reply is added unmeasured. Basis of decision/stdin-closed-non-editing-list and non-editing-list-measured-only, which bind any live start under the real terminal.

@history_item n=9 marker=2026-10-13-interrupted-verify-cleanup
**2026-10-13-interrupted-verify-cleanup** — verify's probe dirs are removed on pass, fail, SIGINT or SIGTERM. A stopped run is killed with no key, and SIGKILL or a crash leaves them by hand. Basis of decision/verify-probe-dirs-cleanup. A live verify's run folders under its cwd are removed and its pinned `bin/<key>/` stays.

@history_item n=10 marker=2026-10-14-resume-and-compact-ledger-rows
**2026-10-14-resume-and-compact-ledger-rows** — Run E resumes only Run B's first SessionStart id, with no fork. It adds no transcript, measured. Basis of decision/verify-run-e-resume-target and session-id-argv-closed-shape, which the live verify exercises.

@history_item n=11 marker=2026-10-16-unwrapped-sessions-on-the-board
**2026-10-16-unwrapped-sessions-on-the-board** — Run B's bounded `claude agents --json` read, keeping only its own row. Each new ledger row (`agents-join`) left the older stamps unverified. Same trap as C4: the live home's 2.1.296–.325 stamps read unverified under a tree-built binary.

@history_item n=12 marker=2026-10-18-compact-post-condition-ledger-row
**2026-10-18-compact-post-condition-ledger-row** — Run B's `/compact` paste and its measured backup residual, ratified as a widening of the verify child set. Precedent for C6: measuring a local command's post-condition live (here `/context`) is a widening whose residual must be named before the start.

@history_item n=13 marker=2026-10-18-plugin-precedence-live-readings
**2026-10-18-plugin-precedence-live-readings** — the live round's rig sits outside the work tree and keys only into its own pty. The residual is the CLI's own writes to the real `~/.claude.json` and `~/.claude/` (trust entry, transcripts); viola writes nothing there. Precedent for naming, before each live start, the files written under the real `~/.claude*`.

@history_item n=14 marker=2026-10-19-overseer-switch-to-viola
**2026-10-19-overseer-switch-to-viola** — the print-mode probe residual (an empty `~/.claude/projects/` dir) was measured at the overseer's switch to viola's verify and accepted into the class. This is the direct precedent for the product re-pin and the operator's switch step: residuals of a real-home verify are measured and accepted by the founder's word.

@history_item n=15 marker=2026-10-19-newer-claude-cli-rows
**2026-10-19-newer-claude-cli-rows** — a one-start scratch reading of a newer CLI was a widening needing its own answer, and earlier scratch answers stay spent. Applies to every live start here, all under the founder's cap word.

@history_item n=16 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** — `update_stamps` runs `check_stamps` before `ledger/` is made and again under its lock. `verify` runs `check_home_tree` (home, `ledger/`, `bin/`, `plugin/`) before its cli log, and `run`/`revive` check the home tree before the wrapper log. Basis of decision/verify-home-tree-check, stamps-checked-under-lock, strict-check-before-narrowing and home-tree-strict-modes-before-log. The real `~/.viola` must pass these before the live verify writes its stamp.

@history_item n=17 marker=2026-10-27-pinned-copy-race-and-reproducible-builds
**2026-10-27-pinned-copy-race-and-reproducible-builds** — the pinned `viola` is placed by `place_private_new` (mode before any byte, handle closed, no-clobber) and never replaced. A lost placement is re-hashed, and different bytes give `HashMismatch` with the file left as found. One tree may key two folders. Basis of decision/pinned-binary-placed-once, lost-placement-rehash, pinned-copy-no-clobber-placement and two-keys-per-tree-accepted. The re-pin's rebuild gets a new `bin/<key>/`, and old pinned folders are never deleted.

@history_item n=18 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** — verify's closed refusal codes on its `process-exit`. A failing row is no refusal and no code, and a refused home tree is lineless. Source of contract/verify-refusal-codes/*, the words a RED live verify can end in.

@history_item n=19 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** — Linux-only. The relied set is all ledger rows on every build, verify's refusal codes went from 6 to 5 (`batch-script-child` gone), the R8 strip matches names exactly, and child resolution is a given path or a bare name on PATH. Basis of r8-claude-env-strip, run-resolves-absolute-program and contract/verify-refusal-codes/program-not-found.

@history_item n=20 marker=2026-11-06-version-done-check
**2026-11-06-version-done-check** — Measurement B (one tree keys two pinned folders) is recorded as known with no action, the re-hash staying the guard. A verify read a row red on an unchanged CLI because it read the account, not the CLI. Trap: a live real-home verify can read RED for reasons outside the CLI version.

@history_item n=21 marker=2026-11-13-verify-readiness-on-the-input-box
**2026-11-13-verify-readiness-on-the-input-box** — input-box readiness comes from compiled literals (footer or rule row plus prompt row, never under a modal), one predicate for verify and run's pre-send gate. The screen fixture keeps only signature rows. Basis of decision/input-box-readiness-from-literals and screen-fixture-signature-rows-only.

@history_item n=22 marker=2026-11-14-status-line-interception-removed
**2026-11-14-status-line-interception-removed** — Run B's probe `--settings` and the statusline capture arm are gone, and viola runs no shell. The settings override is no longer written, and code-bearing files are the plugin files alone. Fixes what Run B's argv and the start's code-bearing writes are for the live verify and the re-pinned product.

@history_item n=23 marker=2026-11-15-live-round-on-the-trimmed-product
**2026-11-15-live-round-on-the-trimmed-product** — added decision/live-round-rig-keys-own-pty (rig keys only into its own pty, the founder's word anew per chunk) and decision/live-session-screens-kept-local (live screens only under a gitignored local path, a closed-word summary landing). Binds every live start of this chunk.

@history_item n=24 marker=2026-11-15-api-error-turn-end
**2026-11-15-api-error-turn-end** — decision/unverified-cli-gate gained its StopFailure exception: no ledger row and no verify probe. StopFailure stays outside the 24-row stamp's coverage.

@history_item n=25 marker=2026-11-16T06-05-33-wrap
**2026-11-16T06-05-33-wrap** — the founder ratified the StopFailure no-ledger-row exception on decision/unverified-cli-gate, and the trailing-only send tolerance.

@history_item n=26 marker=2026-11-16-local-command-delivery
**2026-11-16-local-command-delivery** — added decision/verify-echo-probe-residuals: Run B's seventh paste `/mcp disable viola-probe-absent`. Whether it writes to the user's real Claude config stays unmeasured, and it adds one input-history line. Also added decision/transcript-read-bounded and compiled-classification-sources (local command by compiled name). This is the residual C6 measures by key names before and after.

@history_item n=27 marker=2026-11-17T05-21-35Z-wrap
**2026-11-17T05-21-35Z-wrap** — the founder ratified local-command-delivery's rows, verify-echo-probe-residuals and transcript-read-bounded among them. The residual the live measurement resolves stands on his word.

@history_item n=28 marker=2026-11-17-standing-live-rig
**2026-11-17-standing-live-rig** — the rig's closed argv key grant, its transient credentials copy (no read of the operator's credentials outside the rig's own check) and its isolated child under a private `CLAUDE_CONFIG_DIR`. Contrasts with this chunk's starts under the founder's real home and config.

@history_item n=29 marker=2026-11-17-first-readings-on-the-standing-rig
**2026-11-17-first-readings-on-the-standing-rig** — the rig reads the user's `~/.claude.json` only after a moved stamp, searching for rig-made needles, dropping the bytes and keeping one closed word. The stop (Ctrl-C or a no-key group kill) needs no word. Precedent for the PREREQ's names-only read of the real `~/.claude.json`: never a value.

@history_item n=30 marker=2026-11-17-trust-record-ledger-row
**2026-11-17-trust-record-ledger-row** — verify writes and removes its own trust record in the Claude config its child reads, and a SIGKILL leaves it. Added decision/trust-record-writer-guarded, decision/trust-record-closed-words and the explicit-mode-before-write exception, plus the `CLAUDE_CONFIG_DIR` value read. The live verify on the real home writes into the real `~/.claude.json` through this writer.

@history_item n=31 marker=2026-11-18-trust-record-row-read-live
**2026-11-18-trust-record-row-read-live** — a hidden rig-only first-screen record of scrubbed rows on a failed recording verify, kept local and git-ignored. The stamp's first screen is held to closed words (decision/verify-stamp-first-screen-closed-words). Also decision/live-rig-seed-from-bundle-logic. Fixes what a 24-row stamp records and what a RED live verify may leave.

@history_item n=32 marker=2026-11-18-session-folder-trusted-by-viola
**2026-11-18-session-folder-trusted-by-viola** — run and revive's start write the trust flag for an untrusted spawn folder; the record persists and skips trusted folders. Adds the `CLAUDE_CONFIG_DIR` value read at start, and the cwd is held in the Claude config as the trust key. The re-pinned product gains this write into the real `~/.claude.json`; its rows are PROVISIONAL until the founder's word.

@history_item n=33 marker=2026-11-18-browser-tools-prompt-modal
**2026-11-18-browser-tools-prompt-modal** — decision/unverified-cli-gate, dialog-answers-need-verify-stamp and contract/cli-exit-codes/12 now count `absent` on the browser-tools-prompt row alone as verified. Added decision/live-rig-cache-key-name-read (key name only, never a value). This is the verified rule the 24-row stamp is read by, and the names-only precedent for the PREREQ key read. PROVISIONAL.

@history_item n=34 marker=2026-11-18-the-2-1-325-recorded-fixture-set
**2026-11-18-the-2-1-325-recorded-fixture-set** — StopFailure's exception owner moved to «A live StopFailure payload». The MCP tap is now started live by the standing rig under its private config, never the user's real config. This chunk takes its carries (C1–C6) from that set.
