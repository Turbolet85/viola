@history_item n=1 marker=2026-11-18-the-2-1-325-recorded-fixture-set
**2026-11-18-the-2-1-325-recorded-fixture-set** (Data Protection, MCP arm): + decision/live-rig-mcp-arm-keeps-no-output. The rig's MCP recording arm keeps no CLI output: stdout is dropped, stderr is kept only as a byte count and SHA, and its MCP config is a private 0600 file removed with the scratch. This chunk's direct precedent: the `R4M` re-record on 2.1.328 runs under this row unchanged.

@history_item n=2 marker=2026-11-18-the-2-1-325-recorded-fixture-set
**2026-11-18-the-2-1-325-recorded-fixture-set** (Data Protection, tap): ~ decision/mcp-fixtures-via-test-only-tap. The tap is now started live by the standing rig (`R4M`) under its private CLI config, never the user's real config. The 2.1.328 `Mcp.` re-record must keep exactly this shape.

@history_item n=3 marker=2026-11-18-the-2-1-325-recorded-fixture-set
**2026-11-18-the-2-1-325-recorded-fixture-set** (Authentication & Authorization): ~ decision/unverified-cli-gate except. StopFailure's owner moved off the 2.1.325 set to «A live StopFailure payload» on the founder's word. This chunk sits directly ahead of that entry and must not take StopFailure or its ledger exception.

@history_item n=4 marker=2026-11-18-the-live-home-s-re-stamp-and-the-product-re-pin
**2026-11-18-the-live-home-s-re-stamp-and-the-product-re-pin** (Data Protection): + decision/live-rig-context-arm-keeps-no-transcript. The CX arm reads the transcript through MAX_FRAME as a count only and sends only through viola send under the rig's private config. Carry C3 (`/context`'s stdout-tag half read from a readable transcript) must use this arm's bounds.

@history_item n=5 marker=2026-11-18-the-live-home-s-re-stamp-and-the-product-re-pin
**2026-11-18-the-live-home-s-re-stamp-and-the-product-re-pin** (Secret Management): ~ decision/live-rig-isolated-child and ~ decision/live-rig-credentials-copy each gained the VH-arm exception (real home, no credentials copy, key names and one needle word only). This was confirmed PROVISIONAL by the delegate. The chunk's boundary forbids anything under the real home beyond what the 2.1.325 starts did, so `R4`/`R4M` must not take the VH exception.

@history_item n=6 marker=2026-11-18-the-live-home-s-re-stamp-and-the-product-re-pin
**2026-11-18-the-live-home-s-re-stamp-and-the-product-re-pin** (Threat Model Summary): ~ decision/verify-never-answers-dialogs. A browser-tools prompt standing at start is left for the human to answer, and verify says so before its summary. The recording verify on 2.1.328 must still type no byte into any dialog.

@history_item n=7 marker=2026-11-18-browser-tools-prompt-modal
**2026-11-18-browser-tools-prompt-modal** (Authentication & Authorization / Anti-Patterns): ~ decision/unverified-cli-gate, ~ decision/dialog-answers-need-verify-stamp and ~ contract/cli-exit-codes/12. "Verified" now means every relied row counts: pass, or absent on the browser-tools-prompt row alone. The 2.1.328 recording's stamp is read under this rule.

@history_item n=8 marker=2026-11-18-browser-tools-prompt-modal
**2026-11-18-browser-tools-prompt-modal** (Secret Management): ~ decision/live-rig-seed-from-bundle-logic (the VP arm leaves the browser-tools key out) and + decision/live-rig-cache-key-name-read (key name only, never a value). These bound what the rig may seed and read in its private config for `R4`. The scope reads the 2.1.328 browser-tools key as unchanged from the bundle.

@history_item n=9 marker=2026-11-18-trust-record-row-read-live
**2026-11-18-trust-record-row-read-live** (Data Protection / Threat Model Summary): ~ decision/screen-fixture-signature-rows-only, ~ decision/verify-signature-only and ~ decision/verify-screen-read-against-literals each gained the except clause for the rig-only first-screen file. That file holds whole scrubbed rows, is written only on a failed recording, and is kept local and git-ignored. Trap: it must never land in `fixtures/claude/2.1.328/`.

@history_item n=10 marker=2026-11-18-trust-record-row-read-live
**2026-11-18-trust-record-row-read-live** (Input Validation): + decision/verify-stamp-first-screen-closed-words. The stamp records first screens only as closed words, whole-number settles and literal-id facts. A new 2.1.328 literal, such as the trust dialog's decline text, can enter only as a compiled literal.

@history_item n=11 marker=2026-11-18-trust-record-row-read-live
**2026-11-18-trust-record-row-read-live** (Secret Management): + decision/live-rig-seed-from-bundle-logic. Nothing in the rig child's seeded config is read from the user's config. This is why the scope reads 2.1.328 bundle facts statically before any start.

@history_item n=12 marker=2026-11-18-session-folder-trusted-by-viola
**2026-11-18-session-folder-trusted-by-viola** (Data Protection / Threat Model Summary / Secret Management): these amend the except clauses of decision/no-writes-under-claude-home, decision/viola-writes-nothing-in-claude-dir and decision/nothing-written-under-claude-home, plus decision/trust-record-writer-guarded, decision/cwd-only-in-snapshot, decision/spawn-cwd-snapshot-only and the CLAUDE_CONFIG_DIR exceptions (decision/r8-claude-env-strip, decision/credential-strip-by-name-only, decision/env-not-config-channel, decision/no-stored-secrets). Verify's own trust record, and the start's flag, are the only writes into the Claude config; all PROVISIONAL. The recording verify writes only its own record.

@history_item n=13 marker=2026-11-18-session-folder-trusted-by-viola
**2026-11-18-session-folder-trusted-by-viola** (Input Validation): ~ decision/trust-record-closed-words: 14 verify words and 16 start words. Only verify's word is stamped as the trust-record row's value, never a path, key or value.

@history_item n=14 marker=2026-11-17-trust-record-ledger-row
**2026-11-17-trust-record-ledger-row** (Data Protection / Threat Model / A&A / Secret Management / Input Validation): verify writes and removes its own trust record. This set the first except clauses on the no-writes rows and decision/explicit-mode-before-write (Claude's config keeps its own mode), added + decision/trust-record-writer-guarded and + decision/trust-record-closed-words, and extended decision/verify-probe-dirs-cleanup and decision/probe-artifacts-cleanup (a SIGKILL leaves the record). It is the origin of the residual a recording verify can leave.

@history_item n=15 marker=2026-11-17-standing-live-rig
**2026-11-17-standing-live-rig** (Authentication & Authorization): ~ decision/live-round-rig-keys-own-pty. The rig is one standing committed rig with a closed argv key grant, on the founder's word anew per chunk. That is the source of carry C2: the cap word is needed for `R4`/`R4M`.

@history_item n=16 marker=2026-11-17-standing-live-rig
**2026-11-17-standing-live-rig** (Secret Management): + decision/live-rig-credentials-copy (an owner-only transient copy, expiry read only, a 20-minute floor, removed on every exit) and + decision/live-rig-isolated-child (private CLAUDE_CONFIG_DIR, a closed kept env list). These are the rows every live start of this chunk runs under.

@history_item n=17 marker=2026-11-17-first-readings-on-the-standing-rig
**2026-11-17-first-readings-on-the-standing-rig** (Secret Management): ~ decision/live-rig-isolated-child. A host writer disproved the sha256-and-mtime witness on `~/.claude.json`. It was replaced by the rig-needle mark witness, which keeps one closed word. This is the "stamped read-only" reading of the user config that the scope allows.

@history_item n=18 marker=2026-11-17-first-readings-on-the-standing-rig
**2026-11-17-first-readings-on-the-standing-rig** (Authentication & Authorization): ~ decision/live-round-rig-keys-own-pty. The per-chunk word binds only the argv key grant; the stop (Ctrl-C into its own pty, or a no-key group kill) is standing. The cap word asked under C2 covers the grant, not the stop.

@history_item n=19 marker=2026-11-17-first-readings-on-the-standing-rig
**2026-11-17-first-readings-on-the-standing-rig** (Input Validation): + decision/live-rig-prompt-capture-closed. The rig's UserPromptSubmit capture keeps only key names and one closed source word, never a value. It bounds any capture used to read C3's `/context` turn.

@history_item n=20 marker=2026-11-16-local-command-delivery
**2026-11-16-local-command-delivery** (Input Validation / Data Protection): + decision/transcript-path-shape-check, + decision/transcript-read-bounded, + decision/transcript-path-redacted and + decision/verify-echo-probe-residuals, with the decision/shared-validators-in-core except clause. The transcript is read count-only, never via a symlink, and only from the recorded offset; its path appears in no record. These bind C3's transcript read and what the recorded set may hold. Run B's seventh paste residuals are accepted.

@history_item n=21 marker=2026-11-15-live-round-on-the-trimmed-product
**2026-11-15-live-round-on-the-trimmed-product** (A&A / Data Protection): + decision/live-round-rig-keys-own-pty and + decision/live-session-screens-kept-local. Live screen frames stay under a git-ignored local path, never in a work dir, run dir or commit; only closed-word summaries land. The rig's frames and kept record from `R4` stay local.

@history_item n=22 marker=2026-11-15-api-error-turn-end
**2026-11-15-api-error-turn-end** (Authentication & Authorization): ~ decision/unverified-cli-gate except. StopFailure is registered with no ledger row or verify probe and was first owned by the 2.1.325 fixture set; it was ratified at 2026-11-16T06-05-33-wrap and then moved (see n=3). Do not re-attach it to this set.

@history_item n=23 marker=2026-11-13-verify-readiness-on-the-input-box
**2026-11-13-verify-readiness-on-the-input-box** (Data Protection): ~ decision/screen-fixture-signature-rows-only. Screen fixtures keep footer and modal literals whole, a box rule row as rule chars and a prompt row as the prompt char; any other box row is refused. A 2.1.328 screen that trips this is a red, never a hand scrub.

@history_item n=24 marker=2026-11-13-verify-readiness-on-the-input-box
**2026-11-13-verify-readiness-on-the-input-box** (Input Validation): + decision/input-box-readiness-from-literals. One predicate for verify and run's gate reads the box from compiled literals and never while a modal literal stands. A moved 2.1.328 literal would need a compiled change, not runtime text.

@history_item n=25 marker=2026-11-14-status-line-interception-removed
**2026-11-14-status-line-interception-removed** (Data Protection / Anti-Patterns): the except clauses of decision/fixture-scrub-and-refusal and decision/fixture-hygiene were emptied when the StatusLine fixtures left. No `workspace.repo` keep-exception remains, so a recorded owner handle is now refused like any other user word.

@history_item n=26 marker=2026-11-14-budget-gate-removed
**2026-11-14-budget-gate-removed** (Data Protection): + decision/viola-format-fixtures-hygiene. The hygiene walk was widened to the viola-format fixtures; the walk the new set must pass covers both classes.

@history_item n=27 marker=2026-11-14-hyprland-seam
**2026-11-14-hyprland-seam** (Threat Model Summary): ~ decision/child-output-tolerant and ~ decision/child-output-tolerant-unknown. A child-output failure becomes unknown or a closed failure word. The drift suites read the recorded set under this tolerant-parse rule.

@history_item n=28 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (Secret Management / Data Protection): the R8 strip matches names exactly, and the fixture scrub matches the home prefix exactly (no Windows case-fold). These are the scrub semantics the 2.1.328 recording runs under.

@history_item n=29 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (child program resolution): `batch-script-child` left verify's refusal codes (6 to 5). The closed set holding contract/verify-refusal-codes/unclean-fixture is now five.

@history_item n=30 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** (verify's refusal codes): verify writes `unclean-fixture` among its closed refusal codes on `process-exit`, and the fixture's `<file>` enters no code. This is the refusal an unclean 2.1.328 recording ends in.

@history_item n=31 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (`ledger/stamps.json` and verify's home tree): `ledger/stamps.json` is checked before `ledger/` is created and again under its lock, and verify runs `check_home_tree` before its cli log. These are the origins of decision/stamps-checked-under-lock and decision/verify-home-tree-check, which the recording verify's stamp update runs under.

@history_item n=32 marker=2026-10-27-bounded-inputs-at-every-boundary
**2026-10-27-bounded-inputs-at-every-boundary** (census): `tests/contract_bounded_readers.rs` requires every product reader to be bounded or named (decision/bounded-readers-census). Any reader this chunk adds, such as a recorded-set loader, must satisfy the census.

@history_item n=33 marker=2026-10-20-epoch-5-cleanup-ii
**2026-10-20-epoch-5-cleanup-ii** (Repository fixtures): `Mcp.` exchanges are recorded by the test-only `viola-mcp-tap`. It spawns `viola mcp` directly with no shell, caps each line, keeps a 0600 capture never committed, and refuses with the closed codes, validated by `schemas/claude-mcp-fixture.v1.json`. This is the founding widening for the `R4M` re-record, with its start cap.

@history_item n=34 marker=2026-10-19-newer-claude-cli-rows
**2026-10-19-newer-claude-cli-rows** (Accepted residual): a scratch reading of a newer CLI is no standing permission, and another reading needs its own answer; earlier scratch answers stay spent. This is the precedent for C2's per-chunk cap word on 2.1.328 starts.

@history_item n=35 marker=2026-10-19-overseer-switch-to-viola
**2026-10-19-overseer-switch-to-viola** (Accepted risk): the print-mode probe leaves the CLI's empty project dir under `~/.claude/projects/`, and it is accepted into the verify transcripts class (decision/verify-real-cli-transcripts-accepted). Expect it again on 2.1.328.

@history_item n=36 marker=2026-10-18-plugin-precedence-live-readings
**2026-10-18-plugin-precedence-live-readings** (Data Protection, rig): the live rig sits outside the work tree, passes folder trust only by keys into its own pty (at most two arrows, one Enter on a read row), and its residual is the CLI's own writes. This is the origin of the key grant now in decision/live-round-rig-keys-own-pty.

@history_item n=37 marker=2026-10-18-plugin-precedence-live-readings
**2026-10-18-plugin-precedence-live-readings** (Input Validation): the CLI verbs' `VIOLA_DIR` home step and canonicalisation were ratified. This is the basis of decision/cli-home-resolution-order and contract/cli-state-root/viola-dir, which choose the home a recording verify stamps into.

@history_item n=38 marker=2026-10-18-compact-post-condition-ledger-row
**2026-10-18-compact-post-condition-ledger-row**: Run B's sixth paste (`/compact`) and its accepted three-file transcript residual. This is the `/compact` sibling of carry C3's `/context` post-condition and the accepted-risk class a recording verify extends.

@history_item n=39 marker=2026-10-16-unwrapped-sessions-on-the-board
**2026-10-16-unwrapped-sessions-on-the-board**: Run B's one bounded `claude agents --json` read. `--record` keeps only Run B's own row in `Agents.recorded-1.json`, with no row text, id, cwd or path in the probe pairs. This binds the agents file of the 2.1.328 set.

@history_item n=40 marker=2026-10-16-remove-the-browser-leg
**2026-10-16-remove-the-browser-leg** (Secret scanning): the artifact canary scan's classes are the twelve stripped `CLAUDE*` needles, the content canary and the diagnostics mode. These are the classes (contract/artifact-canary-scan-classes/*) that gate CI uploads over the new set's suites.

@history_item n=41 marker=2026-11-13-epoch-8-cleanup
**2026-11-13-epoch-8-cleanup** (Secret Management): − the canary scan's cookie-header class and + its unreadable class. This gives the current closed class list the drift-suite artifacts are scanned against.

@history_item n=42 marker=2026-10-14-resume-and-compact-ledger-rows
**2026-10-14-resume-and-compact-ledger-rows**: Run E resumes only the session id of Run B's first SessionStart capture, without fork, and spawns nothing without one (decision/verify-run-e-resume-target). Its transcript residual is accepted. A recording verify on 2.1.328 runs Run E under this.

@history_item n=43 marker=2026-10-13-interrupted-verify-cleanup
**2026-10-13-interrupted-verify-cleanup**: verify's probe dirs are removed on pass, fail, SIGINT or SIGTERM, only what the run created and never by pattern; SIGKILL or a crash leaves them. This is the cleanup rule (decision/verify-probe-dirs-cleanup) every live recording start relies on.

@history_item n=44 marker=2026-10-05-dialog-rows-and-re-probe
**2026-10-05-dialog-rows-and-re-probe**: Runs C and D confine the dialog probe's one `touch` and the plan file to their own probe dirs, never `~/.claude/plans` (decision/verify-probes-confined-to-own-dir). The capture arm answers only by its hook output (decision/verify-writes-no-dialog-byte).

@history_item n=45 marker=2026-10-05-real-cli-verify-probes
**2026-10-05-real-cli-verify-probes**: the founding widening of verify's PTY children. They are direct spawns under R8 at 80×24 with no key into any dialog. It made screens keep signature rows only, named the refusal `a recorded fixture is not clean: <file> <code>` with the codes `home-path`, `absolute-path`, `username` and `email`, accepted Run B's transcripts, and set the trusted-folder prerequisite. These are the rows the recorder's hygiene enforces on the 2.1.328 set.

@history_item n=46 marker=2026-10-07-live-rows-and-paste-shapes-on-the-dev-host
**2026-10-07-live-rows-and-paste-shapes-on-the-dev-host**: two scratch measurements across the credential boundary were ratified once each. Standing rule: those answers are spent, and another probe needs its own showing. This backs C2: every live start needs a fresh word.

@history_item n=47 marker=2026-10-07-test-homes-off-the-contended-volume
**2026-10-07-test-homes-off-the-contended-volume**: test-side removals under `target/e2e-home` take only dirs the test side created. The link to the tmpfs backing reaches outside the work tree (decision/e2e-home-removal-own-dirs-only). This applies to the drift suites' test homes.

@history_item n=48 marker=2026-11-16-wait-wake-condition
**2026-11-16-wait-wake-condition** (Anti-Patterns): ~ decision/test-removal-own-dirs-only adds another build's stamp dir, taken only under `try_lock`, later ratified. This is the complete list of test-side removals the suites may make.

@history_item n=49 marker=2026-09-28-capability-ledger-and-viola-verify
**2026-09-28-capability-ledger-and-viola-verify** (capture arm, fixture scrub): + the hidden `hook --capture` arm, transient 0600 probe captures, and the `verify --record` scrub (home to `~`, user word to `<user>`), with the whole recording refused when a path or user word survives. This is the origin of decision/fixture-recording-scrubbed, decision/hook-capture-arm and decision/verify-probe-captures-transient.

@history_item n=50 marker=2026-09-28-capability-ledger-and-viola-verify
**2026-09-28-capability-ledger-and-viola-verify** (MAX_FRAME, CI split): verify runs in CI only against the fake agent, and its real-CLI probe and `--record` run only locally (decision/real-cli-local-only, decision/ci-no-secrets-no-real-claude). The live recording is local, and CI replays the landed set through the fake agent.

@history_item n=51 marker=2026-09-26-local-linux-pre-push-gate
**2026-09-26-local-linux-pre-push-gate**: the first fake-agent test seam was admitted, compiled out of release, configuring nothing. NEVER add another seam or env read without its own Decisions Log entry (decision/test-seams-fake-agent-only, decision/new-env-read-needs-log-entry). Any fake-agent change for 2.1.328 replay must not add an env seam unlogged.

@history_item n=52 marker=2026-09-29-t15-07-57-wrap
**2026-09-29-t15-07-57-wrap**: `scripts/release-check.sh` refuses any executable but `viola` and any `test-support` or `fake-agent` artifact. This is the guard of decision/release-build-no-test-binaries and decision/release-build-without-test-features, which keeps the rig and tap test-only.
