@task_prose part=session
A cleanup chunk with no product behaviour change: no product API, channel method, MCP tool, refusal word, exit code, config key, dependency, CI step or status shape changed.
Retired: the CI artifact canary scan's cookie class (a cookie header line is now no hit), and one dead public function of the strict-modes check with no caller.
Added: one shared masking scanner for the source census tests, one shared owner-only test directory helper, and new cases for the canary scan reading a file it cannot read, the MCP stdin over-limit WARN parse-rejected line, MCP stdin line capping across reads, no-clobber placement of the pinned copy, private-file placement, and the test-only MCP tap.
Split, cases and names unchanged: the fake agent test binary into a module directory, the verify ledger unit cases, and the viola verify integration cases; the census filters for readers and error writers now leave out the fake agent's whole module directory.
Measured: tempfile's no-clobber persist refuses an existing name with EEXIST even for a directory, so a test separating the present-name guard needs a parent-mtime witness; 25 named mutant-survivor controls now hold, with no cargo-mutants run.
@task_file n=1 path=crates/viola-e2e/src/harness/secret_scan.rs
what: the CI artifact canary scan and its classes (claude-stripped, content canary, diagnostics file mode, unreadable; cookie header retired) that gate CI uploads
@task_file n=2 path=crates/viola-state/src/strict.rs
what: the strict-modes check on trusted state under the viola home (one dead public function removed)
@task_file n=3 path=crates/viola-state/src/fs.rs
what: the one shared helper that places and replaces files under owner-only modes and creates private directories
@task_file n=4 path=crates/viola-state/src/pin.rs
what: the pinned copy keyed by SHA-256, placed once by no-clobber placement and re-hashed before use
@task_file n=5 path=crates/viola-mcp/src/lib.rs
what: the viola mcp server and its MCP stdin refusal log (parse-rejected WARN line for an over-limit line)
@task_file n=6 path=crates/viola-mcp/src/stdin.rs
what: the MCP stdin line cap on the reader, lines capped at MAX_FRAME
@task_file n=7 path=crates/viola-e2e/src/bin/viola-mcp-tap.rs
what: the test-only MCP tap that records MCP fixtures and scrubs or refuses them
@task_file n=8 path=crates/viola-e2e/tests/mcp_tap.rs
what: the end-to-end cases of the test-only MCP tap: usage refusal, clean fixture written, dirty capture refused by file name and code alone, an exchange recorded both ways into an owner-only capture
@task_file n=9 path=crates/viola-e2e/src/harness/mod.rs
what: the e2e harness and its e2e home (free space read of the e2e home)
@task_file n=10 path=crates/viola-e2e/src/harness/run.rs
what: the e2e harness run layers and their ledger-row count (non-unix twins removed)
@task_file n=11 path=tests/contract_bounded_readers.rs
what: the census that every product reader is bounded or named, now leaving out the fake agent's whole module directory
@task_file n=12 path=tests/contract_error_bodies.rs
what: the census that every product error writer is named, now leaving out the fake agent's whole module directory
@task_file n=13 path=tests/contract_os_gated_tests.rs
what: the census of OS-gated test cases, now masking comments through the shared scanner
@task_file n=14 path=tests/support/source_scan.rs
what: the one shared masking scanner (comments, or comments and literals) the source census tests take
@task_file n=15 path=tests/support/home.rs
what: the test home helper (tests use their own owner-only home) and its one call of the private-directory creation
@task_file n=16 path=src/bin/viola-fake-agent.rs
what: the root of the fake agent test binary, the test seams built only under the fake-agent feature
@task_file n=17 path=src/bin/viola-fake-agent/agents.rs
what: the fake agent's claude agents output and its session registry
@task_file n=18 path=src/bin/viola-fake-agent/input.rs
what: the fake agent's typed-input reader (bracketed paste, keys, submit)
@task_file n=19 path=src/bin/viola-fake-agent/statusline.rs
what: the fake agent's statusline command replay through a shell
@task_file n=20 path=src/bin/viola-fake-agent/test_cases.rs
what: the fake agent's own unit cases (shared helpers)
@task_file n=21 path=src/bin/viola-fake-agent/test_cases_opts.rs
what: the fake agent's unit cases for its options, input and hook commands
@task_file n=22 path=src/bin/viola-fake-agent/test_cases_modes.rs
what: the fake agent's unit cases for statusline replay, dialog replay, agents output and live sessions
@task_file n=23 path=crates/viola-agent-claude/src/ledger/test_cases.rs
what: the root of the verify ledger's unit cases
@task_file n=24 path=crates/viola-agent-claude/src/ledger/test_cases_stamps.rs
what: the verify ledger's unit cases for the ledger rows, the capture plugin and the stamps file written only by verify
@task_file n=25 path=crates/viola-agent-claude/src/ledger/test_cases_checks.rs
what: the verify ledger's unit cases for the fixture scrub and refusal, the screen fixture signature rows and the input box checks
@task_file n=26 path=crates/viola-agent-claude/src/ledger/test_cases_runs.rs
what: the verify ledger's unit cases for the claude agents join, resume target, paste rows and statusline rows
@task_file n=27 path=crates/viola-agent-claude/src/ledger/test_cases_framing.rs
what: the verify ledger's unit cases for framing, dialog probes and the compiled probe answers
@task_file n=28 path=tests/cli_verify.rs
what: the root of the viola verify integration cases, keeping its unix-gated cases
@task_file n=29 path=tests/cli_verify/stamps.rs
what: viola verify integration cases for the stamps, verify's refusal codes, the identity strip and the internal error line
@task_file n=30 path=tests/cli_verify/runs.rs
what: viola verify integration cases for recorded screens and fixtures, the unclean-fixture refusal, probe directory cleanup and the hook capture arm
@task_file n=31 path=src/cmd/hook.rs
what: viola hook, its instance check first and refusals (a test helper added)
@task_file n=32 path=src/run/mod.rs
what: run's start of an instance (a test-only first snapshot builder added)
@task_file n=33 path=src/run/links.rs
what: run's link restore of the names it wrote (test lines only)
@task_file n=34 path=src/run/snapshot.rs
what: the instance snapshot written only by its wrapper (test lines only)
@task_file n=35 path=.claude/docs/services/viola-state.md
what: the viola-state service document (strict-modes check, private files, pinned copy)
@task_item n=1 part=touches
contract/artifact-canary-scan-classes/cookie-header is retired: the canary scan holds no cookie class and reads a cookie header line as no hit; the row is to be removed.
@task_item n=2 part=touches
Any decision listing the artifact canary scan's classes (the CI artifact canary scan, scan-gated CI uploads, perf uploads gated by the scan) loses cookie from its list; the remaining classes are claude-stripped, content canary, diagnostics file mode and unreadable.
@task_item n=3 part=touches
decision/bounded-readers-census: the census filter now leaves out the fake agent's whole module directory, not only its root file — an exception widened with the split.
@task_item n=4 part=touches
decision/every-error-writer-named: the census filter now leaves out the fake agent's whole module directory, not only its root file.
@task_item n=5 part=touches
Rows on the fake agent's test seams (test seams only under the fake-agent feature, the hook panic and pump delay seams, test seams compiled out of release) — the fake agent is now a module directory; seams keep their names and feature gate.
@task_item n=6 part=touches
Strict-modes check rows (strict-modes check on trusted state, strict-modes read rule): one dead public function of the check, with no caller, is removed; no check or refusal changed.
@task_item n=7 part=touches
decision/pinned-copy-no-clobber-placement and decision/pinned-binary-placed-once: a new case pins that a placement over an unreadable name fails and places nothing; measured that the no-clobber persist refuses an existing directory with EEXIST.
@task_item n=8 part=touches
decision/one-replace-helper, decision/private-file-modes and decision/owner-only-file-modes: the shared private-file helper gained a case (a too-long name fails and is not present); tests now create owner-only directories through one shared test helper.
@task_item n=9 part=touches
decision/tests-use-own-home: test homes and private test directories are now made through one shared helper over the product's private-directory creation.
@task_item n=10 part=touches
MCP stdin rows (decision/mcp-stdin-line-cap, decision/mcp-stdin-line-cap-on-reader, decision/over-long-line-ends-session, contract/mcp-stdin/oversize-line, contract/mcp-stdin/parse-rejected): new cases pin the one WARN parse-rejected line on an over-limit line and lines carried whole across short reads; behaviour unchanged.
@task_item n=11 part=touches
decision/mcp-fixtures-via-test-only-tap: the test-only MCP tap now has its own end-to-end cases, including a dirty capture refused by file name and code alone and an owner-only capture file.
@task_item n=12 part=touches
Rows whose guard cell names a test of the viola verify integration cases, the verify ledger unit cases or the fake agent unit cases: those cases moved into split files with their names kept (fixture scrub and refusal, fixture refusal codes, verify refusal codes, verify probe directory cleanup, screen fixture signature rows, stamps written only by verify).
@task_item n=13 part=touches
decision/ci-artifact-canary-scan and decision/artifact-canary-scan acceptance: the unreadable class now has its planted case (a file the scan cannot read is flagged).
@task_item n=14 part=touches
decision/mutation-testing-at-audit: named mutant survivors were closed by 25 controls that hold, with no cargo-mutants run this chunk.
@task_item n=15 part=touches
decision/release-build-no-test-binaries and decision/release-build-without-test-features: the fake agent test binary's root was split into a module directory; the release check still reads viola only.
@task_item n=16 part=touches
decision/real-cli-local-only, or any row capping live starts of the real claude CLI: the founder's word that a print-and-exit read (claude --version, --help) is no live start and counts against no live-start cap.
@task_item n=17 part=touches
E2E harness rows (decision/e2e-home-keepers-linked-base, decision/e2e-home-link-guard, decision/viola-ignores-e2e-home-backing): the e2e home's free-space read was restated through a private product helper; no guard changed.
@task_fact n=1 slug=secret-management
fact: The artifact canary scan has an unreadable class: a file the scan cannot read is a hit, and a planted case holds it.
@task_fact n=2 slug=secret-management
fact: The artifact canary scan holds no cookie class; a cookie header line in an artifact is no hit.
@task_item n=1 part=untouched
Channel methods, MCP tools, wrapper channel faults, CLI exit codes, JSON error documents and refusal words: no product contract changed.
@task_item n=2 part=untouched
Dependency security: no manifest, lockfile, pin, version floor, licence list, advisory ignore or deny rule changed; no dependency was added for the scoped test subscriber.
@task_item n=3 part=untouched
CI workflows, action SHA pins, workflow permissions, zizmor and the scheduled advisory and fuzz runs: no CI step changed.
@task_item n=4 part=untouched
Peer euid checks, server verification before any frame and the socket file mode: not reached.
@task_item n=5 part=untouched
Paste text, dialog free text and control-character rules, and the human-output escaping: not reached.
@task_item n=6 part=untouched
The absences (network listener, outbound calls, accounts, secrets, database, hosting, release stage): no capability added or removed.
@task_item n=7 part=untouched
rmcp feature set and API security rows, and the event log rotation and diagnostics file rows: not reached.
