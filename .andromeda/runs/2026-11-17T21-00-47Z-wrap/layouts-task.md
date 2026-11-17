@task_prose part=session
The session added a 23rd relied ledger row, trust-record (a trust record written for an untrusted folder opens it with no modal), checked by a sixth interactive verify run, Run F, the last after Run E.
viola verify now writes a trust record for its own temp dir into the config file its child reads, the one CLAUDE_CONFIG_DIR value when the strip keeps that name, else the user home's .claude.json, and removes only that record after a pass, a fail or a stop.
Counts moved: the relied rows 22 to 23, the step counter [NN/22] to [NN/23], the stamped line 22 pass 0 fail to 23 pass 0 fail, verify's interactive runs five to six (A to F), its spawn pairs eight to nine.
The stamps' rows.trust-record value may be the writer's code word when nothing was written (one of 14 closed words such as config-absent, lock-held, lost); measured.trust_probe write and remove added; no log event, config key or verb added.
The test-only fake agent reads the record through --trust-config and gains a --trust-record-blank hold; the live rig gains a v arm; every test-started verify runs under a test-owned HOME.
One live verify S0 on claude 2.1.325 read trust-record fail, 7 of 23 pass; it is recorded as read, does not verify the row, and the row stands built and proven on the fake agent, unverified on the real CLI.
@task_file n=1 path=crates/viola-agent-claude/src/trust.rs
what: the Claude config's trust-record shape - the config file's place, the record key, adding and removing the one record, and the closed set of trust code words
@task_file n=2 path=crates/viola-agent-claude/src/lib.rs
what: the Claude adapter crate's module list
@task_file n=3 path=crates/viola-agent-claude/src/ledger.rs
what: the relied ledger rows of viola verify, each row's id, words and check, and the stamp merge that writes rows and measured values
@task_file n=4 path=crates/viola-agent-claude/src/ledger/test_cases.rs
what: unit cases of the relied ledger rows
@task_file n=5 path=crates/viola-agent-claude/src/ledger/test_cases_checks.rs
what: unit cases of each ledger row's check
@task_file n=6 path=crates/viola-agent-claude/src/ledger/test_cases_stamps.rs
what: unit cases of the ledger stamp merge
@task_file n=7 path=src/main.rs
what: the viola binary's entry and module list
@task_file n=8 path=src/trust_record.rs
what: the CLI's writer and remover of one trust record in the Claude config file, its lock, attempts and bounded read-backs
@task_file n=9 path=src/cmd/verify.rs
what: the viola verify verb - its inputs, its step lines, its summary, its --record form and its env strip reading the kept config dir
@task_file n=10 path=src/cmd/verify/typed.rs
what: viola verify's typed interactive runs and what each run reports to the ledger
@task_file n=11 path=src/cmd/verify/typed/runs.rs
what: viola verify's interactive runs A to F, Run F the trust-record run and its record guard
@task_file n=12 path=src/run/version_gate.rs
what: the version gate that reads the ledger stamps for a CLI version
@task_file n=13 path=src/bin/viola-fake-agent.rs
what: the test-only fake agent that stands in for the claude CLI, its screens and its trust dialog
@task_file n=14 path=src/bin/viola-fake-agent/test_cases_opts.rs
what: unit cases of the fake agent's argv options
@task_file n=15 path=tests/support/verify.rs
what: the test support that starts viola verify against the fake agent, its test-owned home and config
@task_file n=16 path=tests/cli_verify.rs
what: integration tests of viola verify's output, rows and stamps
@task_file n=17 path=tests/cli_verify/runs.rs
what: integration tests of viola verify's interactive runs
@task_file n=18 path=tests/cli_verify/stamps.rs
what: integration tests of viola verify's ledger stamps
@task_file n=19 path=tests/contract_ledger_probes.rs
what: the contract test of the ledger probes and the stamped summary line
@task_file n=20 path=tests/contract_bounded_readers.rs
what: the contract test that every file reader is bounded
@task_file n=21 path=tests/contract_verify_home.rs
what: the census that every test-started verify runs under a test-owned home
@task_file n=22 path=crates/viola-e2e/src/harness/boot.rs
what: the E2E harness boot that stamps verify under the session's own home
@task_file n=23 path=crates/viola-e2e/src/harness/run.rs
what: the E2E harness run steps and its expected live verify step lines
@task_file n=24 path=crates/viola-e2e/src/rig/mod.rs
what: the live rig, its arms and summary keys
@task_file n=25 path=crates/viola-e2e/src/rig/isolate.rs
what: the live rig's isolation of a child's temp root and private config
@task_file n=26 path=crates/viola-e2e/src/bin/viola-live-rig.rs
what: the live rig binary's argument handling
@task_file n=27 path=crates/viola-e2e/tests/rig_rehearsal.rs
what: the live rig's rehearsal tests against the fake agent
@task_item n=1 part=touches
The viola verify step line: its counter moves from [NN/22] to [NN/23], and a 23rd step line, [23/23] trust-record, now prints last.
@task_item n=2 part=touches
The viola verify summary and the stamped line: 22 pass 0 fail becomes 23 pass 0 fail; a failing probe reads 22 pass 1 fail.
@task_item n=3 part=touches
viola verify output exit 0 and exit 1: exit 1 now also follows a failing trust-record row, including when the config file is absent and Run F never starts.
@task_item n=4 part=touches
viola verify failing row: the trust-record row can fail with the writer's code word as its cause when no record was written.
@task_item n=5 part=touches
viola verify stop and the verify-stopped refusal: a stop during Run F removes verify's own record before verify exits 1.
@task_item n=6 part=touches
viola verify --record: a recording verify now runs through a kept CLAUDE_CONFIG_DIR, because its scrub reads HOME.
@task_item n=7 part=touches
viola verify --help paragraph and usage, where they count the ledger rows or the interactive runs (now 23 rows and six runs).
@task_item n=8 part=touches
viola verify output plain ASCII: the new step line and the trust code words are plain ASCII.
@task_item n=9 part=touches
Any CLI-surface row that says viola writes nothing into the Claude config: verify now writes and removes one record for its own temp dir, the bounded exception.
@task_item n=10 part=touches
Any CLI-surface row on what environment viola reads: verify now reads the one CLAUDE_CONFIG_DIR value, only when the strip keeps that name.
@task_fact n=1 slug=surface-cli
fact: viola verify prints 23 step lines; the 23rd and last is trust-record, the row that passes when a trust record verify wrote for an untrusted folder opens it with no modal.
@task_fact n=2 slug=surface-cli
fact: viola verify writes one trust record for its own temp dir into the Claude config its child reads and removes only that record after a pass, a fail or a stop; it never creates an absent config file, and then the trust-record row fails.
@task_fact n=3 slug=surface-cli
fact: the trust-record writer speaks 14 closed code words (written, removed, and for a record never written one of: config-absent, config-unreadable, config-link, config-not-regular, config-foreign, config-oversize, config-malformed, config-dir-unusable, home-unknown, key-present, lock-held, lost); the stamp's trust-record row carries that word when nothing was written.
@task_item n=1 part=untouched
The web-spa surface and its absence of any page: no web surface was added.
@task_item n=2 part=untouched
The viola list board, the BAY line, its columns, JSON document and styling.
@task_item n=3 part=untouched
viola send, wait, answer and last output, the readback mirror and its vocabulary.
@task_item n=4 part=untouched
The viola mcp tools and their results, and the hidden mcp verb.
@task_item n=5 part=untouched
viola link and unlink output, plugin install output and the frame verbs' exit-21 hints and details.
@task_item n=6 part=untouched
The CLI exit-code table, the global flags, the token rendering and the hint conventions: no code, flag, token or hint added.
@task_item n=7 part=untouched
The deep-link absence: the CLI gained no link or URL.
