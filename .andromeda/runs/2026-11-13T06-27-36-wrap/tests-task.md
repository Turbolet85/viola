@task_prose part=session
The screen readiness check now reads the input box as up on a footer literal OR on the box's own rows (a rule row of at least 40 box-rule chars directly above a row opening the prompt char), never while a modal literal stands; one predicate decides for verify's typed-run guards, the input-box-signature ledger row and viola run's pre-send readiness gate.
Recorded screen fixtures keep a box rule row as its run of rule chars alone and a prompt row as the prompt char alone; the fixture hygiene walk accepts exactly those two forms beside the four held literals (its literal set grows 4 to 6) and refuses any other text on such a row.
A recording now also refuses as unclean-fixture where the input box's prompt row holds a home path, the user word or an email.
Verify counts the guards that passed on the box alone and writes the count beside the typed-probe timings in the stamps; the relied ledger rows stay 24.
cargo-deny gains three reasoned advisory ignores (RUSTSEC-2026-0457, -0461, -0462), each in an rmcp feature that is banned or off, standing until rmcp leaves its <3.5 range; rustix moves 1.1.5 to 1.1.6 in the lockfile alone; the supply-chain CI job is green again; the founder ratified both the ignores and the box signature (the latter as a boundary widening).
Measured: the live verify ran on claude 2.1.323 (23 pass, 1 fail on statusline-rate-limits, 3 box-only guards); every stamped fixture set replays at 0 fail on the new predicate.
@task_file n=1 path=crates/viola-agent-claude/src/screen.rs
what: the screen signatures and the readiness verdict - the compiled footer, modal and now input-box rule and prompt literals, the input-box-up predicate, the box-rule minimum, and the verdict viola run's readiness gate reads
@task_file n=2 path=crates/viola-agent-claude/src/ledger.rs
what: the CLI-version ledger - the input-box-signature row's check, the signature rows a recorded screen fixture keeps, the screen-fixture fault check, and the stamps' measured typed-probe record
@task_file n=3 path=crates/viola-agent-claude/src/ledger/test_cases.rs
what: the ledger's unit tests - the input-box-signature row over box-only screens, signature-row keeping, the screen-fixture fault refusals and the stamps merge
@task_file n=4 path=src/cmd/verify/typed/turns.rs
what: viola verify's typed-run turn guards - the input-box readiness guard over each framing turn (long, tag-like and newline pastes, /compact, the local command) and the box-only guard count
@task_file n=5 path=src/cmd/verify/typed/pty.rs
what: viola verify's PTY settle and box waits - the quiet-screen wait that ends on the input box up or a modal literal
@task_file n=6 path=src/cmd/verify/typed/runs.rs
what: viola verify's typed runs - the trusted run's ready guard and the box-only count it hands to the stamps
@task_file n=7 path=src/run/gate.rs
what: viola run's pre-send readiness gate (wait_ready, GATE_MAX_WAIT) - one case added for a verified CLI ready on the box alone
@task_file n=8 path=tests/contract_fixture_hygiene.rs
what: the fixture hygiene run-time walk over recorded fixtures - which screen rows pass as signature rows, the held literal list, and the planted-violation cases
@task_file n=9 path=deny.toml
what: the cargo-deny supply-chain gate configuration - advisories ignore list, feature bans, licenses and sources
@task_file n=10 path=Cargo.lock
what: the workspace lockfile - the resolved dependency graph the supply-chain gate checks
@task_file n=11 path=fuzz/fuzz_targets/vt100_feed.rs
what: the vt100 screen-feed fuzz target in the separate fuzz workspace - builds its own screen signatures
@task_item n=1 part=touches
decision/no-advisory-ignored (No dependency advisory ignored) is broken: the deny config now ignores three advisories, each with a reason naming the disabled rmcp feature it sits in, ratified by the founder.
@task_item n=2 part=touches
decision/no-ignored-advisories (No advisory ignored) under CI integration is broken the same way: three reasoned ignores stand until rmcp leaves <3.5.
@task_item n=3 part=touches
The cargo-deny gate rows (decision/cargo-deny-gate, decision/cargo-deny-supply-chain-gate, decision/supply-chain-fail-closed, decision/supply-chain-ci-gates) - the gate now passes with an ignore list; any row saying the gate is clean with no ignores is stale; supply-chain is green again on the pushed HEAD.
@task_item n=4 part=touches
decision/weekly-advisories-run and decision/weekly-advisory-scan - the advisory scan now runs over a non-empty ignore list; check whether a row says it reports every advisory.
@task_item n=5 part=touches
Indirect: decision/rmcp-feature-limit, decision/rmcp-features-server-transport-io and decision/deny-bans-probed - the three ignores rest on the rmcp auth and Streamable HTTP features staying banned or off; the feature limit and its ban probes are now load-bearing for the ignores (unchanged in themselves).
@task_item n=6 part=touches
decision/screen-signatures-only (Screen read only for compiled signatures) - the compiled input-box literals now include the box's rule and prompt chars; readiness reads a footer literal or the box's rows, never with a modal; check its wording.
@task_item n=7 part=touches
contract/readiness-gate/gate-max-wait - a quiet screen with no input-box literal waits to GATE_MAX_WAIT; a screen showing only the box's rule and prompt rows is now ready, and a rule shorter than 40 chars still waits.
@task_item n=8 part=touches
decision/readiness-gate-tested-by-refusals and decision/readiness-stall-report - viola run's readiness gate now reads ready on the box alone and input-not-ready on the box beside a modal; new gate cases pin both.
@task_item n=9 part=touches
decision/no-assertion-beyond-compiled-literals - the compiled literals screen assertions may use grew by the box rule and prompt chars and a box-rule minimum of 40.
@task_item n=10 part=touches
Rows that the rendered child screen is never parsed (decision/no-screen-parsing, decision/never-parse-rendered-child-screen, decision/no-child-screen-assertions, decision/tui-passthrough-never-parses-screen) - readiness now reads row shape (a rule row above a prompt row) for compiled chars; check they still hold as written.
@task_item n=11 part=touches
The fixture hygiene walk (decision/fixture-hygiene-walk, decision/fixtures-walked-for-hygiene, decision/planted-hygiene-violations) - a screen row now passes as a signature row when it holds one of four literals, is exactly the prompt char, or is rule chars alone; anything else on such a row is refused NotASignatureRow; the walk's literal list went 4 to 6.
@task_item n=12 part=touches
Screen fixture scrub and recording rows (decision/screen-fixture-scrub-as-detector, decision/recorder-scrub, decision/dirty-payload-refuses-recording, decision/scrubbed-fixtures) - a recorded box row is kept as its literal alone, and the recording refuses where the live prompt row next to a kept rule row holds a home path, user word or email.
@task_item n=13 part=touches
contract/verify-exit-detail/unclean-fixture and contract/process-exit-detail/unclean-fixture - a new cause reaches unclean-fixture: an input-box prompt row holding identifying text.
@task_item n=14 part=touches
Fake-agent and fixture replay rows (decision/fake-agent-matches-verify-fixtures, decision/fixtures-per-cli-version, decision/replay-every-cli-version-fixture, decision/fake-agent-replays-recorded-bytes) - the recorded screen form changed; every stamped set replays at 0 fail on the new predicate; no box-only screen fixture is replayed in CI yet.
@task_item n=15 part=touches
decision/record-only-at-zero-failing-rows - it is why no box-only fixture could be recorded: verify on this account fails statusline-rate-limits.
@task_item n=16 part=touches
Ledger rows (decision/ledger-row-verify-probe, decision/version-gate-needs-all-relied-rows, decision/ledger-probe-contract-suite) - the input-box-signature row's check now reads the box predicate; its id and the relied count 24 are unchanged.
@task_item n=17 part=touches
Stamps from verify (decision/stamps-written-only-by-verify, decision/stamps-only-from-verify, decision/verify-sole-stamp-writer) - verify now writes box_only_guards into the stamps' measured typed-probe record; still verify alone.
@task_item n=18 part=touches
Real-CLI and compat rows (decision/compat-tests-for-cli-versions, decision/real-claude-local-only, decision/live-verify-never-in-ci) - the live verify ran locally on claude 2.1.323, a version newer than the plan named.
@task_item n=19 part=touches
Indirect: fuzz rows (decision/fuzz-separate-workspace, decision/fuzzing-with-cargo-fuzz, decision/fuzz-toolchain-single-source, decision/vt100-feed-panic-safe) - the vt100 fuzz target builds screen signatures and changed with them; no gate builds the fuzz workspace, so it was compile-checked by hand on the pinned nightly.
@task_fact n=1 slug=unit-test-strategy
fact: The input box reads up when no row holds a modal literal and either a footer literal stands or a box-rule row (at least 40 rule chars alone) stands directly above a row opening the prompt char; this one predicate decides verify's guards, the input-box-signature row and viola run's readiness gate.
@task_fact n=2 slug=test-data-fixtures
fact: A recorded screen fixture keeps a box rule row as its rule chars alone and a prompt row below it as the prompt char alone; the hygiene walk refuses a prompt row with text after the prompt char or a rule row with other text.
@task_fact n=3 slug=7-test-data-fixtures
fact: A screen recording refuses as unclean-fixture where the input box's prompt row holds a home path, the user word or an email, since a kept rule row's seam is that whole live row.
@task_fact n=4 slug=9-ci-integration
fact: Exception to no ignored advisories: an advisory that sits only in an rmcp feature banned or off in the release graph is ignored with a reason naming that feature, standing until rmcp leaves <3.5 (RUSTSEC-2026-0457, -0461, -0462).
@task_fact n=5 slug=5-command-implementation
fact: No gate entry builds the fuzz workspace; a change to a type its targets construct needs a compile check on the pinned fuzz nightly, offline and with no toolchain install, under a kill deadline.
@task_item n=1 part=untouched
MCP, channel, JSON-RPC and wrapper-channel contracts and their error codes: no surface, refusal word or code changed.
@task_item n=2 part=untouched
viola send, wait, last, release, list and answer contracts and their exit codes: send confirmation, refusal order and wheel rules are unchanged; only the readiness verdict before a send reads a new arm.
@task_item n=3 part=untouched
Hook, dialog, decision-body, statusline and budget rows: no hook path, deadline, budget threshold or statusline behaviour changed.
@task_item n=4 part=untouched
Harness contract rows (boot, cleanup, status, run, gate breach, pre-push, session record, exit codes, local-live failure codes): no harness verb, status shape or gate step changed.
@task_item n=5 part=untouched
Coverage floors, mutation thresholds, perf budgets and kill lines: no threshold, floor or bound moved; no mutation run.
@task_item n=6 part=untouched
Log format, corr fields, event-line and process-log contracts: no log line or field changed.
@task_item n=7 part=untouched
CI workflow rows (artifacts, SHA-pinned actions, permissions, zizmor, toolchain pin, MSRV, caches): no workflow or job changed.
@task_item n=8 part=untouched
Filesystem strict modes, socket directory, peer admission, home read rule and run-start refusal rows: unchanged.
