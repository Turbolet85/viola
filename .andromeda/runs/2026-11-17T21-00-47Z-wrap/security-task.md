@task_prose part=session
The session added the 23rd relied ledger row, trust-record: viola verify writes a trust record for its own fresh temp folder into the claude config its child reads, runs a sixth interactive run (Run F) there, checks the folder opens with no trust modal, then removes that one record.
The record goes to the .claude.json in the CLAUDE_CONFIG_DIR value when the claude env strip keeps that name, else to the user HOME's .claude.json; verify reads that one value and nothing else new from the environment.
The writer refuses a link, a non-regular file, a file of another owner, an oversize or malformed document and an already present key, never creates an absent config, takes a lock dir beside the file, tries three times, keeps the file's mode, and reports a closed set of 14 code words; no path, key or value enters any line, stamp or summary.
A drop guard removes the record after a pass, a fail or a SIGTERM stop; a SIGKILL leaves it. The product's own write for run and revive is not built here.
Counts moved: relied rows 22 to 23, verify's interactive runs five to six (A-F), spawn pairs eight to nine; the stamps' row value for trust-record may be a writer's word, and measured.trust_probe was added.
The one live reading S0 on claude 2.1.325 read trust-record fail (7/23), recorded as read: the row stands proven on the fake agent and unverified on the real CLI, cause unmeasured; every test-started verify now runs under a test-owned HOME, with recording runs keeping the real HOME and a kept CLAUDE_CONFIG_DIR.
@task_file n=1 path=crates/viola-agent-claude/src/trust.rs
what: new, the Claude adapter's pure trust-record shapes: which config file holds the record, the record key, adding and removing one entry, and the closed trust code words
@task_file n=2 path=src/trust_record.rs
what: new, the CLI's trust-record writer and remover for the claude config file: owner, link and regular-file checks, bounded read, lock, three attempts, mode-keeping replace
@task_file n=3 path=tests/contract_verify_home.rs
what: new, census test that every test-started verify runs under a test-owned home
@task_file n=4 path=crates/viola-agent-claude/src/lib.rs
what: edited, the Claude adapter crate's module list (adds the trust module)
@task_file n=5 path=crates/viola-agent-claude/src/ledger.rs
what: edited, the relied ledger rows, verify's typed run results, the row checks and the stamps merge
@task_file n=6 path=crates/viola-agent-claude/src/ledger/test_cases.rs
what: edited, ledger row unit tests
@task_file n=7 path=crates/viola-agent-claude/src/ledger/test_cases_checks.rs
what: edited, ledger row check unit tests
@task_file n=8 path=crates/viola-agent-claude/src/ledger/test_cases_stamps.rs
what: edited, stamps merge unit tests
@task_file n=9 path=src/main.rs
what: edited, the CLI binary's module list
@task_file n=10 path=src/cmd/verify.rs
what: edited, viola verify: its inputs, the claude env strip it applies and which trust config file it uses
@task_file n=11 path=src/cmd/verify/typed.rs
what: edited, verify's interactive PTY probe runs and their order
@task_file n=12 path=src/cmd/verify/typed/runs.rs
what: edited, verify's interactive runs, now with Run F and its record drop guard
@task_file n=13 path=src/run/version_gate.rs
what: edited, viola run's version gate reading the verify stamps
@task_file n=14 path=src/bin/viola-fake-agent.rs
what: edited, the test-only fake agent: new argv options to read a trust config and to hold a blank start
@task_file n=15 path=src/bin/viola-fake-agent/test_cases_opts.rs
what: edited, the fake agent's option unit tests
@task_file n=16 path=tests/support/verify.rs
what: edited, test support that starts verify: test-owned user home, trust config, kept config dir
@task_file n=17 path=tests/cli_verify.rs
what: edited, viola verify integration tests
@task_file n=18 path=tests/cli_verify/runs.rs
what: edited, verify interactive run integration tests
@task_file n=19 path=tests/cli_verify/stamps.rs
what: edited, verify stamps integration tests
@task_file n=20 path=tests/contract_ledger_probes.rs
what: edited, the ledger probes contract test
@task_file n=21 path=tests/contract_bounded_readers.rs
what: edited, the bounded readers census (two new boundaries for the trust config reads)
@task_file n=22 path=crates/viola-e2e/src/harness/boot.rs
what: edited, the E2E harness boot: stamps under a session-owned HOME with a seeded config
@task_file n=23 path=crates/viola-e2e/src/harness/run.rs
what: edited, the E2E harness runner and its ledger row count
@task_file n=24 path=crates/viola-e2e/src/rig/mod.rs
what: edited, the live-round rig: new verify arm and its summary keys
@task_file n=25 path=crates/viola-e2e/src/rig/isolate.rs
what: edited, the live rig's isolation of the child from the user's config, and its check for a record left behind
@task_file n=26 path=crates/viola-e2e/src/bin/viola-live-rig.rs
what: edited, the live rig binary
@task_file n=27 path=crates/viola-e2e/tests/rig_rehearsal.rs
what: edited, the live rig rehearsal tests
@task_item n=1 part=touches
decision/no-writes-under-claude-home is stale: verify now writes one trust record for its own temp folder into .claude.json and removes it after, a bounded exception.
@task_item n=2 part=touches
decision/nothing-written-under-claude-home and decision/viola-writes-nothing-in-claude-dir need the same bounded exception for verify's own record.
@task_item n=3 part=touches
decision/verify-children-direct-spawn: verify's child set grows from five to six interactive runs (Run F); spawn pairs eight to nine.
@task_item n=4 part=touches
decision/verify-probe-dirs-cleanup and decision/probe-artifacts-cleanup: a drop guard now also removes verify's own trust record after a pass, a fail or a SIGTERM stop; a SIGKILL leaves it.
@task_item n=5 part=touches
decision/verify-probes-confined-to-own-dir: Run F's effect reaches the claude config file, for a key of its own temp folder only.
@task_item n=6 part=touches
decision/r8-claude-env-strip, decision/credential-strip-by-name-only and decision/env-not-config-channel: verify now reads the one value of CLAUDE_CONFIG_DIR when claude_env_keep keeps that name.
@task_item n=7 part=touches
decision/new-env-read-needs-log-entry: the new CLAUDE_CONFIG_DIR value read is owed its Decisions Log entry.
@task_item n=8 part=touches
decision/verify-never-answers-import, decision/verify-never-answers-import-dialog and decision/verify-trusted-runs-risk: Run F trusts its folder by writing a record rather than by a prepared trusted folder or an answer.
@task_item n=9 part=touches
decision/verify-never-answers-dialogs, decision/verify-writes-no-dialog-byte and decision/verify-screen-read-against-literals: Run F reads that its frame holds no trust modal literal and the input box is up, typing nothing.
@task_item n=10 part=touches
decision/bounded-readers-census, decision/bounded-external-readers and decision/reads-capped-at-max-frame: new readers of the claude config (verify's writer, the fake agent, the rig) each read through MAX_FRAME; the census gained two boundaries.
@task_item n=11 part=touches
decision/json-default-depth-limit and decision/serde-json-bounded-depth: the config document is parsed at default depth.
@task_item n=12 part=touches
decision/one-replace-helper: the trust record is written through replace_private keeping the file's own mode.
@task_item n=13 part=touches
decision/same-user-trust-boundary and decision/os-user-ambient-trust: the writer refuses a config owned by another uid (config-foreign) and a link or non-regular file.
@task_item n=14 part=touches
decision/stamps-written-only-by-verify, decision/version-gate-strict-stamps-read and decision/dialog-answers-need-verify-stamp: the stamps carry a 23rd row whose value may be a writer's code word, plus measured.trust_probe; the version gate reads them.
@task_item n=15 part=touches
Any row holding the relied ledger row count (22) or verify's five runs: now 23 rows and six runs.
@task_item n=16 part=touches
decision/tests-use-own-home and decision/test-homes-gitignored: every test-started verify runs under a test-owned HOME, enforced by a census; recording runs keep the real HOME with CLAUDE_CONFIG_DIR set to the test user home.
@task_item n=17 part=touches
decision/fixture-recording-scrubbed and decision/fixture-scrub-and-refusal: the recording scrub reads HOME, which is why recording verify starts keep the real HOME.
@task_item n=18 part=touches
decision/test-seams-fake-agent-only and decision/test-seams-compiled-out: the fake agent gains test-only --trust-config and --trust-record-blank.
@task_item n=19 part=touches
decision/live-rig-isolated-child and decision/plugin-install-live-round-rig: the rig gains a verify arm reading trust-record and whether a record was left in its private config.
@task_item n=20 part=touches
decision/refusals-hold-no-paths-pids-ids, decision/never-log-claude-values and decision/closed-types-for-protocol-fields: trust outcomes are 14 closed code words, with no path, key or value in any line, stamp or summary.
@task_fact n=1 slug=data-protection
fact: viola verify writes a trust record for its own fresh temp folder into the claude config its child reads and removes only that record after, through a drop guard (a SIGKILL leaves it); this is the one bounded exception to viola writing nothing in Claude's config, and the product's write for run and revive is not ruled by it.
@task_fact n=2 slug=data-protection
fact: The trust record writer never creates an absent config, refuses a link, a non-regular file, a file of another owner, an oversize or malformed document and a present key, takes a lock dir beside the file only when free, tries three times, keeps the file's mode, and touches no other key.
@task_fact n=3 slug=secret-management
fact: viola verify reads the value of CLAUDE_CONFIG_DIR, and only when claude_env_keep keeps that name, to choose the config holding its trust record; else it uses the user HOME's .claude.json.
@task_fact n=4 slug=input-validation
fact: Trust record outcomes are a closed set of 14 code words (written, removed, config-absent, config-unreadable, config-link, config-not-regular, config-foreign, config-oversize, config-malformed, config-dir-unusable, home-unknown, key-present, lock-held, lost), stamped as the trust-record row's value when nothing was written.
@task_fact n=5 slug=data-protection
fact: The trust-record row stands proven on the fake agent and unverified on the real claude CLI 2.1.325 (the live S0 read fail, confounded); the product writes Session folder trusted by viola only when that row passes.
@task_fact n=6 slug=secret-management
fact: Every test-started verify runs under a test-owned HOME, enforced by a census test; a recording verify keeps the real HOME and points CLAUDE_CONFIG_DIR at the test user home, so its record never reaches the user's config.
@task_item n=1 part=untouched
Channel socket, peer euid checks and server verification in authentication-authorization: no socket, listener or frame changed.
@task_item n=2 part=untouched
MCP tools, MCP stdin and the rmcp feature rows: no MCP surface changed.
@task_item n=3 part=untouched
Wrapper channel methods, channel frames and channel errors: no method, frame or code added.
@task_item n=4 part=untouched
Hook stdin, hook decisions and hook contracts: the hook was not reached.
@task_item n=5 part=untouched
Dependency-security rows (pins, cargo-deny, advisory floors, licence list, CI workflows, actions): no dependency or workflow added.
@task_item n=6 part=untouched
Send, answer and paste text character sets and their refusals: no typed-text rule moved.
@task_item n=7 part=untouched
Link and unlink, events log lines and event log rotation: untouched.
@task_item n=8 part=untouched
Pinned binary placement, re-hash and plugin file rewrite rows: unchanged.
@task_item n=9 part=untouched
Absences (network, hosting, accounts, secrets, database, webhooks): no capability added there.
