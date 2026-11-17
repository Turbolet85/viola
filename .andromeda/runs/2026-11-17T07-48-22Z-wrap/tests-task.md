@task_prose part=session
The session added one standing, committed, test-only live rig, the binary viola-live-rig, a new rig module of the test-only E2E crate that every later live round reuses.
Its rehearsals run against the fake agent in CI as a new integration binary, rig_rehearsal; its live starts run by hand against the real claude 2.1.325 under a cap of 2 starts plus 1 spare, each ledgered before it is made, and the live arm refuses a ci label under CI.
The rig reads the child's screen through a vt100 screen model (45 rows by 210 columns), reads and passes the trust dialog under a narrow key grant, isolates the child under a private config dir and scratch, and owns process hygiene (census by pid and start time, a run marker env name, a watchdog, a repeated Ctrl-C stop before the group's SIGKILL).
The fake agent under its screens mode with an untrusted cwd now draws claude 2.1.325's unnumbered trust form over the set's recorded modal, moves its selector on arrows, and erases the prompt's last character on DEL, receipted as a key; no option, env seam or recorded fixture was added.
Live readings measured: the 2.1.325 trust dialog is unnumbered with the selector first on No, exit; the CLI's DEL erases the typed character; at submit the CLI keeps a leading run of spaces, removes a trailing run of spaces and TAB, and keeps an inner run and an inner LF; the user's claude config read unchanged across both starts.
The product binary is unchanged; nothing was retired; the rehearsal waits sit at WITHIN 18 s with settle 6 s and deadline 15 s, below the 20 s floor and the 30 s kill of the E2E package.
@task_file n=1 path=crates/viola-e2e/src/bin/viola-live-rig.rs
what: new; the live rig binary of the test-only E2E crate, its verbs rehearse, live and census, its options and its exit codes 0, 1, 2, 3
@task_file n=2 path=crates/viola-e2e/src/rig/mod.rs
what: new; the rig's start sequence, its refusals before a start, the R1 trust-and-box arm, the R2 viola-wrapped send arm and its closed summary words
@task_file n=3 path=crates/viola-e2e/src/rig/screen.rs
what: new; the rig's vt100 screen model of the child and its reading of the trust dialog, the input box and the external-imports modal by compiled literals
@task_file n=4 path=crates/viola-e2e/src/rig/isolate.rs
what: new; the rig's isolation of the child: scratch dirs, private claude config dir, the child's environment, the credentials copy and the token-window check, the user config before-and-after stamp
@task_file n=5 path=crates/viola-e2e/src/rig/procs.rs
what: new; the rig's process hygiene: census and sweep by pid and start time, the run marker, the watchdog, the stop handlers and the process-group kill
@task_file n=6 path=crates/viola-e2e/src/rig/keys.rs
what: new; the rig's key grant and its only typing site into the child's PTY, the trust-moves reading and the pass of the trust dialog
@task_file n=7 path=crates/viola-e2e/src/rig/ledger.rs
what: new; the rig's start ledger and its cap and spare admission, the kept screen frames and the printed summary
@task_file n=8 path=crates/viola-e2e/tests/rig_rehearsal.rs
what: new; the rig's rehearsal integration binary, nine cases over the fake agent: trust pass, box after x and DEL, grant refusal, credentials and processes on every exit path, token window, no host path in records, user config, live refused under CI
@task_file n=9 path=crates/viola-e2e/Cargo.toml
what: edited; the test-only E2E crate's manifest: the vt100 dependency and the viola-live-rig binary added
@task_file n=10 path=crates/viola-e2e/src/lib.rs
what: edited; the test-only E2E crate's library root, its module list, the rig module added
@task_file n=11 path=src/bin/viola-fake-agent.rs
what: edited; the fake agent standing in for claude: its screens mode now draws the trust form over the recorded modal, moves the selector, fires SessionStart only after Enter on the trust row, and draws the box rule and prompt row on typed keys
@task_file n=12 path=src/bin/viola-fake-agent/input.rs
what: edited; the fake agent's input handling: the typed prompt buffer and the DEL erase of its last character, receipted as a key
@task_file n=13 path=Cargo.lock
what: edited; the workspace lockfile, one line for vt100 in the test-only E2E crate
@task_item n=1 part=touches
decision/live-readings-not-a-test-layer is stale: its rule says live readings are not a test layer, not harness surface and never in CI; the ratified widening makes one standing committed test-only rig whose rehearsals run in CI over the fake agent while its live starts stay by hand under a cap, and the live arm refuses a ci label under CI.
@task_item n=2 part=touches
The rows that keep the real claude CLI out of CI and local-only (real-claude-local-only, real-claude-never-in-ci, ci-never-real-claude, no-real-cli-in-ci, live-verify-never-in-ci) are reached: a committed binary now drives the real CLI by hand, and its CI part is the rehearsal over the fake agent alone.
@task_item n=3 part=touches
decision/send-confirmation-trailing-trim-match: its why cell words the trim set as an assumption, not a live reading; R2 measured it on 2.1.325 (leading run of spaces kept, trailing spaces and TAB removed, inner run and inner LF kept), so the why is stale while the trailing-only rule stands.
@task_item n=4 part=touches
The rows that the rendered child screen is never parsed or asserted on (no-child-screen-assertions, no-screen-scraping-in-e2e, never-parse-rendered-child-screen, no-screen-parsing, tui-passthrough-never-parses-screen, e2e-asserts-only-cross-process) meet an exception: the rig and its rehearsal read the child's screen through a vt100 model and assert on closed words read from it (trust, selected, options, box-text, modal).
@task_item n=5 part=touches
decision/screen-signatures-only and decision/no-assertion-beyond-compiled-literals: the rig reads the trust row, the No, exit option, the selector glyph, the box prompt and the external-imports modal only by compiled literals; a rule over screen reading by compiled signatures now covers a second, test-only reader.
@task_item n=6 part=touches
The vt100 panic rows (vt100-panic-contained, vt100-feed-panic-safe) reach the rig's own screen feed, contained by catch_unwind into a poisoned reading.
@task_item n=7 part=touches
The test-only crate rows (single-test-only-crate, test-only-e2e-crate, tokio-only-in-e2e-crate) reach a new rig module and a new binary in the test-only E2E crate, which takes vt100 from the workspace pin 0.16.2.
@task_item n=8 part=touches
The release rows (release-build-viola-only, no-test-binary-in-release) reach the new test-only binary: the release check still reads viola only.
@task_item n=9 part=touches
The wait-bound and kill-line rows hold the rehearsal's values: decision/root-wait-bound, decision/within-7s-wait-bound, decision/verify-driven-tests-20s-kill, decision/ci-stamped-home-kill-line (20 s) and decision/viola-e2e-30s-mutants-kill (30 s); the rehearsal sits at WITHIN 18 s with settle 6 s and deadline 15 s, and the contract lint that deadlines sit below the nextest kill line read it red at 60 s first.
@task_item n=10 part=touches
The fake-agent screen and recording rows (fake-agent-matches-recordings, fake-agent-replays-recorded-bytes, fake-agent-and-verify-fixtures-are-contract, fake-agent-matches-verify-fixtures, fixtures-recorded-by-live-verify, screen-fixture-box-rows-as-literals) are reached: the fake's trust form is drawn over the recorded modal so a verify recording over the fake keeps the recorded trust row, and the fake now draws a box rule and prompt row per typed key.
@task_item n=11 part=touches
decision/wrapper-boots-pass-trusted-screens stands true (the boots pass screens and a trusted root) but now sits beside a fake that draws a trust form when its cwd is untrusted.
@task_item n=12 part=touches
The fake agent's contract rows are reached: contract/fake-agent-events/session-start (under screens with an untrusted cwd, SessionStart fires only after Enter on the trust row, no hook before it), contract/fake-agent-input/keystroke and contract/fake-agent-receipt/key (DEL erases the prompt's last character, UTF-8 aware, receipted as a key), contract/fake-agent-exit/exit-triggers (Enter on No exits 1).
@task_item n=13 part=touches
decision/fake-agent-argv-only and contract/fake-agent-cli/argv-only-options stand: the fake's new trust form and DEL erase came with no option, env seam or recorded fixture.
@task_item n=14 part=touches
decision/fake-agent-not-proof-of-real-cli and decision/compat-tests-for-cli-versions: the fake's trust form was aligned to the live 2.1.325 reading (unnumbered, selector first on No, exit, trust row below), the live R1 reading being the authority.
@task_item n=15 part=touches
The process-kill rows (pid-start-time-verified-kill, cleanup-ctrl-c-repeated-until-exit, fake-agent-own-process-group, fake-agent-children-own-process-group) reach the rig's census and sweep by pid and start time, its run marker, and its stop that presses Ctrl-C every 500 ms up to 10 s before the group's SIGKILL.
@task_item n=16 part=touches
The child environment strip rows (claude-child-env-strip, child-env-strip, r8-claude-env-strip, claude-env-strip, test-started-viola-drops-env) reach the rig's child environment: only named host names kept, CLAUDE_CONFIG_DIR set to the scratch config, the run marker AGENT_RUN_RIG added, and R2's viola home keeping CLAUDE_CONFIG_DIR through claude_env_keep.
@task_item n=17 part=touches
decision/tests-never-read-host-sessions and the own-home rows (tests-create-own-home, per-test-home, per-test-fresh-home): the rig runs its child under a private config dir and scratch, and reads the user's claude config only as a sha256 prefix and mtime before and after, never its bytes.
@task_item n=18 part=touches
The secrets rows (logs-carry-no-secrets, no-secrets-in-logs, secrets-never-in-logs, no-committed-test-secrets, synthetic-inputs-with-canary) reach the rig's credentials copy: placed 0600, parsed only for its expiry, removed on every exit path, and no path, credential or config value in any record.
@task_item n=19 part=touches
The private placement rows (private-placement-mode-first, private-filesystem-permissions) reach the rig's credentials copy and seeded config, written through the private replace.
@task_item n=20 part=touches
The bounded reader rows (bounded-readers, bounded-readers-keep-the-cap, complete-lines-reader) reach the rig's credentials read and its read of R2's events by complete lines, each through take(MAX_FRAME).
@task_item n=21 part=touches
The test layer selection rows (integration-binary-surface-prefix, e2e-layer-by-binary-prefix, test-layer-filtersets) reach the new rig_rehearsal binary selected under the integration layer by binary(rig_rehearsal).
@task_item n=22 part=touches
The no-human-in-loop rows (no-human-in-loop-verification, no-manual-smoke-step) are reached: the rig's live starts are by-hand rounds under a cap, while its rehearsals are the automated part.
@task_item n=23 part=touches
contract/run-reason/live-in-ci and decision/local-live-runner-seam sit beside the rig's own refusal of a live ci label under CI, a separate path from the harness's local-live suite.
@task_item n=24 part=touches
decision/utc-now-read-directly reaches the rig's ledger, which reads the wall clock from the system date at each start.
@task_item n=25 part=touches
The cli_verify recording case for screens (verify_record_screens_keep_only_signature_rows) read red when the fake's trust form replaced the recorded modal; a change to the fake's screen reaches the verify recording cases, which select through the stamped verify run, not through the files touched.
@task_fact n=1 slug=6-e2e-test-strategy
fact: One standing committed test-only live rig, viola-live-rig in the test-only E2E crate, is reused by every later live round; its rehearsals over the fake agent run in CI as the rig_rehearsal integration binary, its live starts against the real claude run by hand under a per-chunk cap (2 starts plus 1 spare, the spare only to re-fire a start lost to its shape), each start ledgered with its clock time before it is made, and the live arm refuses a ci label under CI.
@task_fact n=2 slug=6-e2e-test-strategy
fact: The rendered-screen rule has one exception: the live rig, test-only, reads the child's screen through a vt100 model of 45 rows by 210 columns by compiled literals only, into closed words (trust, selected, options, box-text, modal), and its rehearsal asserts on those words; kept screens stay in the gitignored target/live-rig.
@task_fact n=3 slug=6-e2e-test-strategy
fact: The rig is the one site that types into a live child's PTY, under a grant from argv (--grant trust,box): at most 2 down-arrows and Enter on a read trust row, one x, one DEL; Ctrl-C belongs to the stop alone; a key outside the grant or not ready writes nothing and the start reads a refusal word.
@task_fact n=4 slug=6-e2e-test-strategy
fact: The rig isolates each start: the child runs under a private CLAUDE_CONFIG_DIR in a scratch removed at teardown, a credentials copy placed 0600 and removed on every exit path, a start refused below 20 minutes of token window, and the user's claude config read only as a sha256 prefix and mtime that must read unchanged.
@task_fact n=5 slug=pid-file
fact: The rig marks its child with AGENT_RUN_RIG set to its pid and start time; its census reaches only processes carrying its marker or under its scratch cwd, verified by pid and start time; census --kill reaches only a dead rig's leftovers; its stop presses Ctrl-C every 500 ms up to 10 s before the process group's SIGKILL, and a watchdog deadline raises the same stop.
@task_fact n=6 slug=6-e2e-test-strategy
fact: viola-live-rig exits 0 for a whole start, 1 for a start read red, 2 for usage and 3 for a refusal before the ledger line; it prints closed key: word summary lines and one summary line in its ledger, its refusal words a closed list (ci, spent, cap, version, token-window, leftover, ledger, scratch, credentials, spawn, viola, shape, no-session-start, texts, key-not-granted, key-not-ready, key-write-failed, trust-unpassed), trust-moves reading a count, needs-up, needs-third-move or unread.
@task_fact n=7 slug=7-test-data-fixtures
fact: The fake agent under its screens mode with an untrusted cwd draws claude 2.1.325's unnumbered trust form over the set's recorded modal, the selector first on No, exit directly above the recorded trust row (row 16 with no recorded modal); arrows move the selector, Enter on the trust row draws the ready screen and fires the launch SessionStart, Enter on No exits 1, and no hook fires before it.
@task_fact n=8 slug=7-test-data-fixtures
fact: The fake agent draws a box rule and a prompt row on each typed key, and a DEL erases the prompt's last character, UTF-8 aware, receipted as a key.
@task_fact n=9 slug=unit-test-strategy
fact: The submit trim of claude 2.1.325 is measured live: a leading run of spaces is kept, a trailing run of spaces and TAB is removed, an inner run and an inner LF are kept; the trailing-trim tolerance rests on that reading, no longer on an assumption.
@task_item n=1 part=untouched
The product's own contracts: CLI exit codes, JSON and human outputs, MCP tools and results, the wrapper channel and its JSON-RPC errors, hook decision bodies, the event log and its data keys, send, wait, last, release, revive and list; the product binary is unchanged.
@task_item n=2 part=untouched
The harness commands' documents, exit codes and reasons (boot, cleanup, status, run, gate, pre-push, secret-scan, schema-check, supervise) and the agent-run logs stream; the rig is no harness command and changed none of them.
@task_item n=3 part=untouched
CI workflow rows: jobs, artifacts and retention, SHA-pinned actions, permissions, caches, the toolchain pin and MSRV; no workflow changed.
@task_item n=4 part=untouched
Coverage floors and thresholds, mutation gates and their thresholds and cadence, performance budgets and the hyperfine gate; no threshold or budget moved.
@task_item n=5 part=untouched
Fuzzing and property-test rows: corpora, nightly fuzz, replay gate, proptest seeds; no parser or fuzz target changed.
@task_item n=6 part=untouched
Supply-chain bans and the deny gate: vt100 was already in the graph through the agent crate at the same workspace pin, and cargo deny read green.
@task_item n=7 part=untouched
The process log format, log line fields and corr rows, and the diagnostics schema rows; the rig writes no product log.
@task_item n=8 part=untouched
Socket, peer, strict-modes and home-tree permission rows of the product; the session touched no product boundary check.
@task_item n=9 part=untouched
Windows and macOS, browser suite, database and outbound HTTP absences; nothing in the session reaches them.
