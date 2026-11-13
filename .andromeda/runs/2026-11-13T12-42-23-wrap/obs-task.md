@task_prose part=session
A cleanup chunk with no product API, wire, schema, config or dependency change: test support, test splits and dead code.
It retired the secret scan's cookie class: a Cookie header line now reads as no hit, and the remaining classes are claude-stripped, content-canary, mode and unreadable, each with a planted case (unreadable newly planted).
It added one shared masking source scanner that the three test censuses use (bounded readers, error bodies, OS-gated tests), and one shared private-directory helper for test homes.
It split the largest files (the fake agent binary, the capability-ledger test cases, the verify integration tests) into module files with bodies unchanged, and removed a dead strict-modes function plus the harness's non-unix ledger-row twins.
It closed the named mutation survivors with new tests (25 gate controls hold, no cargo-mutants run), including a WARN parse-rejected line for oversize MCP stdin and a pinned-copy write over an unreadable name that places nothing.
@task_file n=1 path=tests/support/source_scan.rs
what: the one shared masking scanner (comments, or comments and literals) that the test-source censuses read code through
@task_file n=2 path=tests/contract_os_gated_tests.rs
what: the OS-gated test census; it now masks comments through the shared scanner and reads block comments as comment lines
@task_file n=3 path=tests/contract_bounded_readers.rs
what: the bounded-readers census over product source; its filter now leaves out the fake agent's whole module directory
@task_file n=4 path=tests/contract_error_bodies.rs
what: the clean-error-bodies census over product source; its filter now leaves out the fake agent's whole module directory
@task_file n=5 path=src/bin/viola-fake-agent.rs
what: the fake agent used by CI verify instead of the real claude CLI (bin root, now split into modules)
@task_file n=6 path=src/bin/viola-fake-agent/agents.rs
what: the fake agent's claude agents output and live-session registry
@task_file n=7 path=src/bin/viola-fake-agent/input.rs
what: the fake agent's typed-key and paste input reader
@task_file n=8 path=src/bin/viola-fake-agent/statusline.rs
what: the fake agent's statusline command shell-out and echo
@task_file n=9 path=src/bin/viola-fake-agent/test_cases.rs
what: the fake agent's unit tests
@task_file n=10 path=src/bin/viola-fake-agent/test_cases_opts.rs
what: the fake agent's unit tests of options, input and hook scripts
@task_file n=11 path=src/bin/viola-fake-agent/test_cases_modes.rs
what: the fake agent's unit tests of statusline, dialog and resume replay modes
@task_file n=12 path=crates/viola-agent-claude/src/ledger/test_cases.rs
what: the capability ledger's unit tests (root of the split)
@task_file n=13 path=crates/viola-agent-claude/src/ledger/test_cases_stamps.rs
what: the capability ledger's unit tests of the ledger rows, the stamps merge and the verified reading (an unreadable or short stamp reads as unverified)
@task_file n=14 path=crates/viola-agent-claude/src/ledger/test_cases_checks.rs
what: the capability ledger's unit tests of the verify record scrub and the per-row checks
@task_file n=15 path=crates/viola-agent-claude/src/ledger/test_cases_runs.rs
what: the capability ledger's unit tests of the resume, paste, local-command and statusline row checks
@task_file n=16 path=crates/viola-agent-claude/src/ledger/test_cases_framing.rs
what: the capability ledger's unit tests of the framing and dialog probes
@task_file n=17 path=tests/cli_verify.rs
what: the viola verify integration tests (root file; keeps the unix-gated cases the OS-gated census names)
@task_file n=18 path=tests/cli_verify/stamps.rs
what: the viola verify integration tests of stamps, refusals, the identity strip and the verify log lines
@task_file n=19 path=tests/cli_verify/runs.rs
what: the viola verify integration tests of recorded screens, run dirs, dialog and framing runs and hook capture
@task_file n=20 path=crates/viola-e2e/tests/mcp_tap.rs
what: the e2e tests of the test-only MCP tap: its usage refusal, its fixture scrub refusing a dirty capture by file name and code alone, and its record of a viola mcp exchange
@task_file n=21 path=crates/viola-e2e/src/bin/viola-mcp-tap.rs
what: the test-only MCP tap that records a viola mcp exchange and writes a scrubbed fixture
@task_file n=22 path=crates/viola-e2e/src/harness/secret_scan.rs
what: the secret scan run before artifact upload, its pattern classes and its hit report
@task_file n=23 path=crates/viola-e2e/src/harness/mod.rs
what: the E2E harness: test homes under the e2e home and their free-space check
@task_file n=24 path=crates/viola-e2e/src/harness/run.rs
what: the E2E harness's ledger-row run expectations (the non-unix twins removed)
@task_file n=25 path=crates/viola-state/src/strict.rs
what: the strict-modes check of the home tree (a dead function removed)
@task_file n=26 path=crates/viola-state/src/fs.rs
what: the owner-only private file and directory placement under the home
@task_file n=27 path=crates/viola-state/src/pin.rs
what: the pinned copy of the claude binary, placed never replaced
@task_file n=28 path=crates/viola-mcp/src/lib.rs
what: the viola mcp stdio server; a test now observes its WARN parse-rejected line through a scoped test-module subscriber
@task_file n=29 path=crates/viola-mcp/src/stdin.rs
what: the viola mcp stdin reader bounded by MAX_FRAME
@task_file n=30 path=src/cmd/hook.rs
what: the viola hook command (a test helper added)
@task_file n=31 path=src/run/mod.rs
what: the viola run wrapper (a test-only snapshot builder added)
@task_file n=32 path=src/run/links.rs
what: the viola run link handling (tests only)
@task_file n=33 path=src/run/snapshot.rs
what: the viola run snapshot write (tests only)
@task_file n=34 path=tests/support/home.rs
what: the shared test-home helper; it now holds the one helper that makes a private directory for test homes
@task_file n=35 path=.claude/docs/services/viola-state.md
what: the viola-state service doc
@task_item n=1 part=touches
decision/no-secrets-in-log-lines: its Cookie clause goes, since the secret scan's cookie class is retired.
@task_item n=2 part=touches
decision/secret-scan-before-upload: its Cookie clause goes; the scan's classes are now claude-stripped, content-canary, mode and unreadable.
@task_item n=3 part=touches
decision/secrets-never-logged: its Cookie clause goes with the retired cookie class.
@task_item n=4 part=touches
decision/cookie-header-never-logged: the scan no longer checks for Cookie headers; a Cookie header line reads as no hit. The row stands as written or is retired.
@task_item n=5 part=touches
decision/claude-env-and-cookie-never-logged: its Cookie half goes; the CLAUDE values half stands.
@task_item n=6 part=touches
The section 11 anti-pattern row whose text names user content, token, cookie or CLAUDE* value: the cookie word goes with the retired class.
@task_item n=7 part=touches
contract/secret-scan-hit-report/pattern-class: the closed class list drops cookie and keeps claude-stripped, content-canary, mode and unreadable.
@task_item n=8 part=touches
decision/secret-scan-scope and decision/scan-failure-blocks-upload: any class list or Cookie mention in them drops cookie; unreadable now has a planted case that hits.
@task_item n=9 part=touches
decision/os-gated-test-census: the census masks comments through the one shared scanner, reads a blanked line as a comment line, and now skips block comments. It walks only files directly under tests, so gated cases stay in a split binary's root file.
@task_item n=10 part=touches
decision/error-bodies-clean and decision/fixed-error-messages: the error-bodies census, a check over them, now leaves the fake agent's whole module directory out of its filter.
@task_item n=11 part=touches
decision/ci-verify-fake-agent-only and decision/real-cli-verify-never-in-ci (indirect): the fake agent behind CI verify was split into modules with unchanged behaviour.
@task_item n=12 part=touches
decision/subscriber-only-in-root-bin: kept. The viola mcp WARN test carries a minimal tracing Subscriber scoped to its own test module, with no global install and no tracing-subscriber dev-dependency.
@task_item n=13 part=touches
contract/parse-rejected/mcp-stdin-oversize and contract/mcp-stdin/oversize-line: now asserted by a test that writes exactly one WARN parse-rejected line for a line past MAX_FRAME.
@task_item n=14 part=touches
contract/mcp-stdin/max-frame and contract/mcp-stdin/line-within-max-frame: a new test reads capped lines across short reads in order.
@task_item n=15 part=touches
decision/pinned-copy-never-replaced: a new test shows a pin over an unreadable name fails and places nothing, with the parent directory's mtime unchanged.
@task_item n=16 part=touches
decision/private-diagnostics-files and decision/diagnostics-owner-only-modes (indirect): the private-directory placement gained a test for a too-long name that fails and is not present.
@task_item n=17 part=touches
decision/test-homes-under-e2e-home and decision/test-homes-under-scan-root: test homes now make their private directories through one shared helper, still under the same root.
@task_item n=18 part=touches
decision/obs-code-mutation-tested and decision/surviving-obs-mutant-fails: the named mutation survivors were closed by tests and 25 gate controls hold, with no cargo-mutants run in this chunk.
@task_item n=19 part=touches
decision/claude-fixtures-scrub-required and decision/fixtures-not-telemetry: the MCP tap's fixture scrub is now e2e-tested. It refuses a dirty capture by file name and code alone, and the class document of a clean capture is written.
@task_item n=20 part=touches
decision/unreadable-ledger-is-unverified and decision/unreadable-ledger-means-unverified (indirect): the ledger tests that read a short or malformed stamp as unverified moved into split files unchanged.
@task_item n=21 part=touches
decision/pre-log-strict-refusals-write-no-line and contract/viola-verify-refusal/home-tree-refused (indirect): the strict-modes check lost a dead, uncalled presence function. No refusal behaviour changed.
@task_fact n=1 slug=8-pii-scrubbing-compliance
fact: The artifact secret scan has no cookie class. A Cookie header line in an upload member is no hit. Its classes are claude-stripped, content-canary, mode and unreadable, each proven by a planted case.
@task_fact n=2 slug=ci-integration
fact: The test-source censuses (bounded readers, error bodies, OS-gated tests) read code through one shared masking scanner, with comments or comments and literals blanked. The OS-gated census reads a blanked line as a comment line, block comments included.
@task_fact n=3 slug=ci-integration
fact: The bounded-readers and error-bodies censuses exempt the fake agent's whole module directory (its bin root and every file under it), not only its single bin file.
@task_fact n=4 slug=ci-integration
fact: The OS-gated test census walks only files directly under a tests directory. A test binary split into submodules keeps every census-named gated case in its root file.
@task_fact n=5 slug=bootstrap-phases-derive-for-route-setup-project
fact: A library crate's unit test may carry its own minimal tracing Subscriber scoped to that test, never installed globally and with no tracing-subscriber dependency, to observe a line it emits. This is no exception to tracing-subscriber only in the root binary.
@task_item n=1 part=untouched
Metric coverage rows: no metric, grouping enum or jq derivation changed.
@task_item n=2 part=untouched
SLO, error-budget and liveness rows: no threshold, staleness bound, budget or gate exit rule moved.
@task_item n=3 part=untouched
Error capture and panic rows: no panic hook, catch site, backtrace handling or panic line changed.
@task_item n=4 part=untouched
Trace context propagation and correlation rows (corr, conn, rpc_id): no correlation field or join changed.
@task_item n=5 part=untouched
Process and detail log line schemas, event enum, detail codes and exit codes: no field, word or code added, renamed or removed.
@task_item n=6 part=untouched
Logging dependency rows (tracing-subscriber floor and features, banned telemetry deps, cargo deny, no OTel): no manifest or lockfile changed.
@task_item n=7 part=untouched
config.json keys, home resolution order and service identity rows: no schema, config or version source changed.
@task_item n=8 part=untouched
Absences (OTel export, Sentry, network egress, hosted deployment, database, gRPC, frontends): none gained or lost.
@task_item n=9 part=untouched
Channel JSON-RPC, link, send, dialog and wheel contracts: no method, refusal or log field changed.
