@task_prose part=session
The session added one standing, committed, test-only live rig (the binary viola-live-rig in the viola-e2e test crate). Every later live round reuses it. Its rehearsals against the fake agent run in CI, and its live starts against the real claude CLI run by hand under a per-chunk cap of 2 plus 1 spare.
The rig owns its child's isolation: a private scratch with its own CLAUDE_CONFIG_DIR, a transient credentials copy, and a child environment built from a kept list. It also owns process hygiene through a census and kill keyed on pid and start time and on its run marker env AGENT_RUN_RIG.
The rig is the one typing site into its own pty under a closed grant read from argv: at most 2 down-arrows and Enter on a read trust row, one x, one DEL. Ctrl-C belongs to the stop alone. The founder ratified this grant.
The fake agent under --screens now draws claude 2.1.325's unnumbered trust form over the recorded modal and erases on DEL. It added no option, env seam or fixture.
Measured live on claude 2.1.325: the trust form is unnumbered with the selector on No, exit. At submit the CLI keeps a leading run of spaces, removes trailing spaces and TAB, and keeps an inner run and an inner LF. The user's ~/.claude.json read unchanged under a private config dir.
No product surface changed: the viola binary, its channel, MCP, hook and CLI contracts are as they were. Nothing was retired.
@task_file n=1 path=crates/viola-e2e/src/bin/viola-live-rig.rs
what: the standing live-round rig binary, test-only: its verbs rehearse, live and census, its argv grant, cap and credentials options, and the live arm's refusal under CI
@task_file n=2 path=crates/viola-e2e/src/rig/mod.rs
what: the live rig's start sequence: refusals before any start (ci, cap, version, token-window, leftover), the R1 and R2 live-round arms against the real claude CLI, the child spawn by full program path, the stop that presses Ctrl-C before the group SIGKILL, and the send reply read as closed code words
@task_file n=3 path=crates/viola-e2e/src/rig/screen.rs
what: the live rig's screen model: the vt100 feed under a panic catch, and the read of the trust dialog, the external-imports modal and the input box against compiled literals
@task_file n=4 path=crates/viola-e2e/src/rig/isolate.rs
what: the live rig's isolation boundary: the private scratch and CLAUDE_CONFIG_DIR, the credentials copy placed owner-only and removed on every exit path, the token-window check, the child environment from a kept list of names, and the user config stamp before and after
@task_file n=5 path=crates/viola-e2e/src/rig/procs.rs
what: the live rig's process hygiene: the census and kill by pid and start time limited to its own run marker or scratch cwd, the watchdog and the stop handlers
@task_file n=6 path=crates/viola-e2e/src/rig/keys.rs
what: the live rig's key grant: the only site that types into the rig's own pty, a closed grant, keys typed only on a read trust row or a ready input box, and the trust pass
@task_file n=7 path=crates/viola-e2e/src/rig/ledger.rs
what: the live start ledger: each start ledgered before it is made, the cap and spare admission, and the kept screen frames
@task_file n=8 path=crates/viola-e2e/tests/rig_rehearsal.rs
what: the rig's CI rehearsal tests against the fake agent: no key outside the grant, no credentials copy left and no process left on any exit path, the token-window refusal, no credential or host path in any record, the user config read before and after, and the live arm refused under CI
@task_file n=9 path=crates/viola-e2e/Cargo.toml
what: the e2e test crate's manifest: the new rig binary and the vt100 dependency on the workspace exact pin
@task_file n=10 path=crates/viola-e2e/src/lib.rs
what: the e2e test crate's root, exposing the rig module
@task_file n=11 path=src/bin/viola-fake-agent.rs
what: the fake agent test binary behind the fake-agent feature: its trust dialog drawn under --screens over the recorded modal, the selector moves, and the launch SessionStart fired only after trust
@task_file n=12 path=src/bin/viola-fake-agent/input.rs
what: the fake agent's typed-input handling: the prompt row and the DEL erase
@task_file n=13 path=Cargo.lock
what: the root dependency lockfile under the cargo-deny gate
@task_item n=1 part=touches
decision/live-round-rig-keys-own-pty goes stale. The ratified grant is at most 2 down-arrows and Enter only on a read trust row, one x, one DEL, all read from argv as --grant trust,box, with any other word refused. Ctrl-C is the stop's alone: pressed every 500 ms up to 10 s before the group SIGKILL. The row's grant names no Ctrl-C and no read trust row.
@task_item n=2 part=touches
The rows on live rounds being isolated (decision/plugin-install-live-round-rig and the live-round rules near it) now have one standing rig that every live round reuses: private scratch, private CLAUDE_CONFIG_DIR, a start cap with one spare, and each start ledgered before it is made.
@task_item n=3 part=touches
decision/live-session-screens-kept-local: the rig keeps frames under the gitignored target/live-rig/<run>/screens/. A kept live trust frame holds the scratch cwd path as the CLI draws it, with no user name or home path.
@task_item n=4 part=touches
The rows that say nothing is written under the Claude home (decision/nothing-written-under-claude-home, decision/no-writes-under-claude-home, decision/viola-writes-nothing-in-claude-dir): the rig's child runs under a private CLAUDE_CONFIG_DIR. The user's ~/.claude.json is read only as a sha256 prefix and mtime before and after, and it read unchanged live.
@task_item n=5 part=touches
The credential rows (decision/claude-credentials-stripped-from-child, decision/credential-strip-by-name-only, decision/hermetic-child-environment, decision/inherited-credentials-never-serialised): the rig builds its child's environment from a kept list of host names plus its own. It copies the OAuth credentials file into scratch owner-only and parses only the expiry from it.
@task_item n=6 part=touches
The no-secrets rows (decision/no-stored-secrets, absence/secrets, absence/stored-secrets, absence/key-management): the rig holds a transient copy of the subscription credentials file in its scratch and removes it on every exit path, and it writes no credential, path or config value into any record. Read whether the absence wording still holds.
@task_item n=7 part=touches
decision/real-cli-local-only and decision/ci-no-secrets-no-real-claude: the rig's live arm refuses the start with the word ci under CI. Only the fake-agent rehearsals run in CI, so the real CLI stays local.
@task_item n=8 part=touches
The rows on verify never answering or typing into a dialog (decision/verify-never-answers-dialogs, decision/verify-writes-no-dialog-byte, decision/verify-never-answers-import, decision/verify-never-answers-import-dialog): the test-only rig now passes the trust dialog live, while product verify still answers none. The rig only reads the external-imports modal and types nothing into it.
@task_item n=9 part=touches
decision/screen-fixture-signature-rows-only: the fake agent's trust form is drawn over the recorded Screen.modal, so verify --record over the fake still keeps the recorded trust row and signature rows only.
@task_item n=10 part=touches
The screen-model rows (decision/vt100-inside-catch-unwind, decision/vt100-feed-under-panic-catch, decision/vt100-feed-isolated, decision/input-box-readiness-from-literals, decision/verify-screen-read-against-literals): the rig's screen model feeds vt100 under catch_unwind with a poisoned reading. It reads the trust, imports and box rows against compiled literals at 45 rows by 210 columns.
@task_item n=11 part=touches
The test-only binary and feature rows (decision/release-build-no-test-binaries, decision/release-build-without-test-features, contract/release-check-output/test-only-feature): a new test-only binary was added, and release-check still reads release-check: viola only.
@task_item n=12 part=touches
The test-seam rows (decision/test-seams-fake-agent-only, decision/test-seams-compiled-out, decision/fake-agent-hook-panic-seam, decision/fake-agent-pump-delay-seam): the fake agent gained a trust form and a DEL erase under the existing --screens, with no new option, env seam or fixture.
@task_item n=13 part=touches
decision/new-env-read-needs-log-entry: the rig sets the env name AGENT_RUN_RIG on its child only, and its census reads that marker from /proc environ. This is test-side, not a product read.
@task_item n=14 part=touches
The child program resolution rows (decision/child-path-trust, decision/run-resolves-absolute-program, decision/claude-resolved-from-path): R2's child is the install root's full path, which replaced the bare name claude resolved on PATH.
@task_item n=15 part=touches
The exact-pin and deny rows (decision/exact-dependency-pins, decision/exact-third-party-pins, decision/cargo-deny-gate, decision/ci-cargo-deny-gate): vt100 enters the viola-e2e test crate on the workspace pin =0.16.2. It was already in the graph, and cargo deny check is green.
@task_item n=16 part=touches
The send trim rows (decision/delivery-confirmation-trailing-tolerance, decision/typed-text-line-ending-normalisation, decision/send-text-is-one-bracketed-paste): R2 measured the CLI's submit trim live. A leading run is kept, trailing spaces and TAB are removed, and an inner run and an inner LF are kept. Any cell that words the trim set as unmeasured, or leading trim as open, is now stale.
@task_item n=17 part=touches
The cleanup rows (decision/probe-artifacts-cleanup, decision/verify-probe-dirs-cleanup, decision/test-homes-gitignored, decision/tests-use-own-home): the rig's scratch is created per run under its own base, and teardown removes it on every exit path. Its frames sit under gitignored target paths.
@task_item n=18 part=touches
decision/data-classes and decision/refusals-hold-no-paths-pids-ids: the rig's summary and refusal output are closed key: word lines and one ndjson summary line, with no path, credential or config value.
@task_fact n=1 slug=authentication-authorization
fact: The live rig is the one site that types into a live claude pty. It holds a closed grant read from argv (--grant trust,box): at most 2 down-arrows and Enter only on a read trust row, one x and one DEL in a ready input box. Any other key writes nothing. Ctrl-C is pressed only by the stop, every 500 ms up to 10 s, before the process group's SIGKILL.
@task_fact n=2 slug=secret-management
fact: The live rig places a copy of the claude OAuth credentials file in its private scratch at mode 0600. It parses only claudeAiOauth.expiresAt, refuses a start with under 20 minutes of token, removes the copy on every exit path, and writes no credential, host path or config value into any ledger, frame or summary.
@task_fact n=3 slug=secret-management
fact: The live rig runs its child under a private CLAUDE_CONFIG_DIR in its scratch, with an environment built from a closed kept list of host names plus its own. It reads the user's ~/.claude.json only as a sha256 prefix and mtime before and after each start.
@task_fact n=4 slug=threat-model-summary
fact: The live rig's live arm refuses with the word ci under CI. Live starts against the real claude CLI run only by hand, under a per-chunk start cap with one spare, and each start is ledgered before it is made. The rig's fake-agent rehearsals run in CI.
@task_fact n=5 slug=threat-model-summary
fact: The live rig reads the external-imports modal and types no key into it. A key into that modal is the founder's to grant, and product verify still answers no dialog.
@task_fact n=6 slug=input-validation
fact: The live rig reads the send reply and the events lines only as closed code words through MAX_FRAME-bounded complete lines. It feeds child screen bytes to vt100 under catch_unwind, reading poisoned on a panic.
@task_fact n=7 slug=data-protection
fact: The live rig's census and kill reach a process only by pid and start time, and only one that carries its own run marker AGENT_RUN_RIG=<rig pid>-<start time> or has its scratch cwd. A dead rig's leftovers are killed only by census --kill.
@task_fact n=8 slug=new
fact: The test-only binary name viola-live-rig and the env name AGENT_RUN_RIG, set on the rig's child only, are occupied.
@task_item n=1 part=untouched
The wrapper channel method and MCP tool contracts, their params, refusals and error codes: no product channel, MCP or hook surface changed.
@task_item n=2 part=untouched
The socket and peer rows: the per-user socket directory, chmod after bind, peer euid checks on client and server, server verification before any frame, and the abstract namespace ban.
@task_item n=3 part=untouched
The strict-modes checks and their refusal contracts across run, revive, list, link, hook, mcp, plugin install and verify: no product reader or checker moved.
@task_item n=4 part=untouched
The CLI exit code, revive --json and instance-unreachable contracts, and the error-handling rows on fixed messages and sanitised channel faults.
@task_item n=5 part=untouched
The pinned binary and plugin rows: the content hash, no-clobber placement, re-hash before reuse, and plugin files rewritten each start.
@task_item n=6 part=untouched
The CI workflow rows: actions SHA-pinned, least permissions, zizmor, the artifact canary scan classes, fuzzing, mutation testing and the rmcp feature bans.
@task_item n=7 part=untouched
The event log rotation, diagnostics files, link names and transcript path redaction rows in data protection.
@task_item n=8 part=untouched
The paste text and dialog free-text character-set rows and the control-character refusal: the product's validators did not change, and the rig types only granted keys.
