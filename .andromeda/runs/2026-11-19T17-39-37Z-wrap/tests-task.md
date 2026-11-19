@task_prose part=session
The session added a StopFailure arm (SF, spare SF-2) to the standing live rig and spent one contained live start on claude 2.1.328: GREEN reading, turn-end stop-failure, captured word server_error, cause server-error.
The StopFailure exception (registered with no ledger row and no verify probe) is now SETTLED as outcome B on the founder's word, citing that live reading (2.1.328, 2026-11-19, start SF, word server_error); outcome A (a ledger row with a verify probe) is not opened; one live payload of one class on one version, nothing claimed for the other twelve error words or another version.
The live SF runs inside a network-closed bwrap sandbox (a test-only host dependency of the rig alone); without it the rig refuses (refused: sandbox, exit 3) before any credentials read or ledger line; a sandbox probe also refuses no-runtime-dir unless XDG_RUNTIME_DIR is absolute and outside the temp root.
The rig's StopFailure capture keeps key names, the error word from the CLI's closed 13 (else other) and two byte lengths, never a text; the rig reads the box row's cell attributes, never its text, after the turn end.
The fake agent gained two argv options, --turn-fail <variant> (the turn ends with the recorded set's StopFailure payload) and --settings-hooks <file> (a Claude settings file whose hooks run beside the plugin's, command split on spaces, no shell); four sf rehearsals over the fake agent were added, one of them the contained local entry.
The rig's kept record dir is keyed by CLI version: a version already kept is refused with nothing written, other versions' sets are kept beside it. All changes are test-only; no product crate, schema, config key, dependency or harness command changed.
@task_file n=1 path=crates/viola-e2e/src/rig/isolate.rs
what: edited — the standing live rig's scratch isolation: the scratch home and its kept record dir per CLI version, the seeded capture hook, the network-closed sandbox argv and the runtime-dir check
@task_file n=2 path=crates/viola-e2e/src/rig/mod.rs
what: edited — the standing live rig itself: its arms and labels, the plan, the child spawn, the summary words it prints and the sandbox probe
@task_file n=3 path=crates/viola-e2e/src/rig/capture.rs
what: edited — the rig's closed hook-payload capture: the line it keeps per hook event, now with the StopFailure key names, error word and text lengths
@task_file n=4 path=crates/viola-e2e/src/rig/screen.rs
what: edited — the rig's screen model of the agent's input box, now reading cell attributes of the box row, never text
@task_file n=5 path=crates/viola-e2e/src/bin/viola-live-rig.rs
what: edited — the standing live rig's command line: rehearse and live verbs, the new sf arm and its --contain option
@task_file n=6 path=crates/viola-e2e/tests/rig_rehearsal.rs
what: edited — the rig rehearsals over the fake agent that run in CI, gaining four sf rehearsals
@task_file n=7 path=src/bin/viola-fake-agent.rs
what: edited — the fake agent (viola-fake-agent command line, its turn end and its hook execution), gaining --turn-fail and --settings-hooks
@task_item n=1 part=touches
decision/standing-live-rig: the standing live rig gains the SF arm, its rehearsal unwrapped over the fake agent in CI and a wrapped (contained) local entry; every live SF runs contained.
@task_item n=2 part=touches
decision/hook-payloads-recorded-by-verify: its except naming no recorded StopFailure payload and a synthetic StopFailure payload is stale; the exception is now settled on the founder's word citing the live reading (2.1.328, 2026-11-19, SF, server_error).
@task_item n=3 part=touches
decision/ledger-row-verify-probe and decision/unverified-cli-gate: StopFailure stays registered with no ledger row and no verify probe, now kept by settlement (outcome B), no longer provisional; any except or citation there names the live reading.
@task_item n=4 part=touches
contract/fake-agent-cli/argv-only-options and the other viola-fake-agent command line contract items: the modes and options list gains --turn-fail <variant> and --settings-hooks <file>.
@task_item n=5 part=touches
decision/fake-agent-argv-only: the two new fake agent options are argv-only, read no env and run no shell, which the rule must still hold.
@task_item n=6 part=touches
contract/fake-agent-hooks/command-spawn and contract/fake-agent-hooks/stdin-payload: the fake agent now also runs hooks from a Claude settings file beside the plugin's hooks, a command string with no args split on spaces, no shell.
@task_item n=7 part=touches
decision/fake-agent-replays-recorded-bytes, decision/fake-agent-matches-verify-fixtures and decision/fake-agent-matches-recordings: --turn-fail ends a turn with the recorded set's StopFailure.<variant> payload in place of Stop.default.
@task_item n=8 part=touches
decision/no-screen-parsing, decision/never-parse-rendered-child-screen, decision/no-child-screen-assertions, decision/screen-signatures-only, decision/no-screen-scraping-in-e2e: the rig now reads the box row's cell attributes (dim, italic, bold, inverse, fg colour words), never a cell's text, after the turn end — an exception in that exact width.
@task_item n=9 part=touches
decision/real-claude-local-only, decision/real-claude-never-in-ci, decision/ci-never-real-claude, decision/no-real-cli-in-ci, decision/live-verify-never-in-ci: the live SF start against the real claude is local only; CI runs only the sf rehearsals over the fake agent.
@task_item n=10 part=touches
decision/no-real-network-in-tests and absence/outbound-http: the live SF child runs in a network-closed sandbox with its own network namespace (net-ns: own); its turn fails by connection loss.
@task_item n=11 part=touches
decision/live-failing-reading-not-verification: the SF reading is recorded as read (GREEN on predicates 1-7), never as verification; no capability claimed.
@task_item n=12 part=touches
decision/os-gated-tests-registered, decision/os-gated-cases-in-binary-root and decision/skipped-counts-real-skips-only: the contained sf rehearsal skips by its printed sandbox probe when bwrap is absent or the runtime dir is unset or inside the temp root.
@task_item n=13 part=touches
decision/pre-push-clean-environment: pre-push runs children under env -i, which unsets XDG_RUNTIME_DIR, so the contained rehearsal skips there by its probe (no-runtime-dir).
@task_item n=14 part=touches
decision/hook-payloads-recorded-locally, decision/fixtures-per-cli-version and contract/claude-fixtures/payload-path: the sf rehearsals copy the recorded 2.1.328 set and plant a StopFailure error word into it.
@task_item n=15 part=touches
decision/harness-cli-version-matches-fake and decision/compat-tests-for-cli-versions: the sf rehearsals pass --cli-version 2.1.328 while the fake agent's default stays 2.1.287.
@task_item n=16 part=touches
decision/synthetic-inputs-with-canary, decision/scan-hits-never-echo-bytes and decision/no-production-data-in-tests: a needle rehearsal plants needles and the canary into the StopFailure payload and asserts them absent from stdout, the ledger and the out dir.
@task_item n=17 part=touches
decision/bounded-readers, decision/bounded-readers-keep-the-cap and decision/no-unbounded-json-depth: the rig's StopFailure capture reads through MAX_FRAME + 1 at default JSON depth, an oversize or non-object payload kept as unread.
@task_item n=18 part=touches
decision/unknown-events-counted-skipped and decision/forward-compat-lines-skipped: an error word outside the CLI's 13 reads as cause api-error and is counted as one unknown-value line.
@task_item n=19 part=touches
decision/root-wait-bound, decision/within-7s-wait-bound and decision/waits-key-on-events: the SF arm waits up to 480 s for the turn end and 10 s for the capture word, bounds held where a row names wait bounds.
@task_item n=20 part=touches
decision/ci-stamped-home-kill-line and decision/verify-driven-tests-20s-kill: the unknown-word sf case is its own rehearsal so two rig runs never share one case under the integration profile's kill.
@task_item n=21 part=touches
decision/tests-never-read-host-sessions and decision/per-test-fresh-home: the live start's user-config witness reads only the stamp of the real ~/.claude.json (changed-by-others, no rig needle); nothing is written under the real ~/.claude* or ~/.viola.
@task_item n=22 part=touches
decision/test-only-e2e-crate and decision/single-test-only-crate: all new symbols stay in the test-only E2E crate and the fake-agent binary; no product crate reaches them.
@task_fact n=1 slug=7-test-data-fixtures
fact: The fake agent takes --turn-fail <variant>: beside --turn-stop, the turn ends with the recorded set's StopFailure.<variant> payload in place of Stop.default.
@task_fact n=2 slug=7-test-data-fixtures
fact: The fake agent takes --settings-hooks <file>: a Claude settings file whose hooks run beside the plugin's; a command string with no args is split on spaces, never run through a shell; argv only, no env read.
@task_fact n=3 slug=6-e2e-test-strategy
fact: The live rig's SF arm runs only inside a network-closed bwrap sandbox, a test-only host dependency of the rig alone; without the sandbox the rig refuses (refused: sandbox, exit 3, uncounted) before any credentials read or ledger line.
@task_fact n=4 slug=6-e2e-test-strategy
fact: The rig's sandbox probe refuses no-runtime-dir unless XDG_RUNTIME_DIR is absolute and outside the temp root, so a contained rehearsal skips by its printed probe where the runtime dir cannot cross the sandbox.
@task_fact n=5 slug=6-e2e-test-strategy
fact: The rig's StopFailure capture keeps key names, the error word only from the CLI's closed 13 (else other) and the byte lengths of two text fields, never a text.
@task_fact n=6 slug=6-e2e-test-strategy
fact: The rig's kept record dir is keyed by CLI version first: a version already kept is refused with nothing written, and a later version's set is kept beside earlier ones.
@task_fact n=7 slug=test-data-fixtures
fact: The StopFailure exception (no ledger row, no verify probe) is settled as outcome B on the founder's word on the live reading (claude 2.1.328, 2026-11-19, start SF, word server_error); one connection-lost payload on one version, nothing claimed for the other twelve words or another version.
@task_item n=1 part=untouched
Product crates, their public surfaces and their contracts: CLI exit codes, --json and human outputs, send, wait, answer, release, list, revive and run contracts — the session changed no product code.
@task_item n=2 part=untouched
The wrapper channel, JSON-RPC errors, MCP tools and MCP stdin contracts, and the IPC security vectors.
@task_item n=3 part=untouched
The harness commands (boot, run, status, logs, cleanup), the harness session record, gate breaches, pre-push detail and reason codes, and the status document — none of their shapes changed.
@task_item n=4 part=untouched
CI workflow rows: the single ubuntu leg, artifacts, secret scan, caching, SHA pinning, permissions and the CI job list.
@task_item n=5 part=untouched
Coverage floors and thresholds, mutation gates and kills, performance budgets and hyperfine rows — no threshold, floor or budget moved.
@task_item n=6 part=untouched
Supply-chain rows (cargo-deny bans, advisories, rmcp features, tokio bans, MSRV, pinned toolchain): no dependency was added or bumped.
@task_item n=7 part=untouched
Log format, log event corr and process log line contracts, diagnostics schema and pid-file rows.
@task_item n=8 part=untouched
decision/sandbox-contract-trigger-layer: another sense of the word sandbox (the contract-test trigger layer), true as worded.
