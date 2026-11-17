@task_prose part=session
Added the 23rd relied ledger row, trust-record: the CLI honours a trust record another process wrote, so a recorded untrusted folder opens with no trust modal.
viola verify gains a sixth interactive run (Run F, after Run E) that writes a trust record for its own temp dir into the config its child reads, and removes that one record after through a drop guard, also on a stop; a writer that cannot write stamps its closed code word instead of pass or fail.
Counts moved: relied rows 22 to 23, the step counter [NN/23], the stamped line 23 pass 0 fail, interactive runs five to six (A to F), spawn pairs eight to nine, fake-agent starts per verify six to seven.
Every test-started stamping verify and harness boot step 4 now run under a test-owned user home seeded with an empty config, and the fake agent is told that file by --trust-config; a --record verify keeps the real HOME and points a kept CLAUDE_CONFIG_DIR at the test-owned user home.
The fake agent reads the record itself and gains a test-only hold --trust-record-blank; the live rig gains a v arm; a new census binary holds every test-started verify to a test home.
The one live reading S0 on claude 2.1.325 read trust-record fail at 7 of 23 and is recorded as read: it does not verify the row; the row stands proven on the fake agent and unverified on the real CLI, its live re-read owed.
@task_file n=1 path=crates/viola-agent-claude/src/trust.rs
what: the Claude config trust-record shapes (new): config file choice, record key, adding and removing one record, and the closed set of trust code words
@task_file n=2 path=src/trust_record.rs
what: verify's trust-record writer and remover (new): a bounded config-file reader, a lock, owner and link checks, a private replace keeping the file's mode, three attempts
@task_file n=3 path=tests/contract_verify_home.rs
what: a new contract census binary: every test-started verify runs under a test home, and a planted untied verify start reads red
@task_file n=4 path=crates/viola-agent-claude/src/lib.rs
what: the one crate holding Claude shapes, its module list
@task_file n=5 path=crates/viola-agent-claude/src/ledger.rs
what: the relied ledger rows, each row's verify probe check, and the stamp merge writing rows and measured values
@task_file n=6 path=crates/viola-agent-claude/src/ledger/test_cases.rs
what: unit cases of the ledger rows
@task_file n=7 path=crates/viola-agent-claude/src/ledger/test_cases_checks.rs
what: unit cases of each ledger row's probe check
@task_file n=8 path=crates/viola-agent-claude/src/ledger/test_cases_stamps.rs
what: unit cases of the stamp merge
@task_file n=9 path=src/main.rs
what: the viola binary's module list
@task_file n=10 path=src/cmd/verify.rs
what: the viola verify command: its inputs, the env strip plan, the fixture recording and its scrub
@task_file n=11 path=src/cmd/verify/typed.rs
what: verify's typed run results the ledger checks read
@task_file n=12 path=src/cmd/verify/typed/runs.rs
what: verify's interactive PTY runs, now A to F
@task_file n=13 path=src/run/version_gate.rs
what: the unverified CLI version gate over the relied rows and its unit cases
@task_file n=14 path=src/bin/viola-fake-agent.rs
what: the fake agent serving the claude boundary: its argv options and screens
@task_file n=15 path=src/bin/viola-fake-agent/test_cases_opts.rs
what: unit cases of the fake agent's argv parse
@task_file n=16 path=tests/support/verify.rs
what: the shared verify test helper that starts test verifies, ties them and sets their home and env
@task_file n=17 path=tests/cli_verify.rs
what: the viola verify integration binary
@task_file n=18 path=tests/cli_verify/runs.rs
what: viola verify integration cases over the interactive runs and the verify_window_ class
@task_file n=19 path=tests/cli_verify/stamps.rs
what: viola verify integration cases over the stamps and the step and stamped lines
@task_file n=20 path=tests/contract_ledger_probes.rs
what: the ledger probe contract suite
@task_file n=21 path=tests/contract_bounded_readers.rs
what: the census of bounded external readers keeping the frame cap
@task_file n=22 path=crates/viola-e2e/src/harness/boot.rs
what: the harness boot command: makes the session home, stamps it by verify and starts wrappers
@task_file n=23 path=crates/viola-e2e/src/harness/run.rs
what: the harness run command, its local-live suite and its expected live pass literal
@task_file n=24 path=crates/viola-e2e/src/rig/mod.rs
what: the standing live rig: its arms, plans and summary keys
@task_file n=25 path=crates/viola-e2e/src/rig/isolate.rs
what: the live rig's isolation: private config, scratch and child temp root
@task_file n=26 path=crates/viola-e2e/src/bin/viola-live-rig.rs
what: the live rig binary's entry
@task_file n=27 path=crates/viola-e2e/tests/rig_rehearsal.rs
what: the live rig's rehearsal tests against the fake agent
@task_item n=1 part=touches
Every ledger row has a verify probe: the new row trust-record gets its probe, Run F, checked as written, Run A's modal literal, Run F's frame free of that literal, and the input box up.
@task_item n=2 part=touches
Stamping verifies give the fake agent a transcript: every stamping verify now also runs under a test-owned user home and hands the fake agent --trust-config; a --record verify keeps the real HOME with a kept CLAUDE_CONFIG_DIR.
@task_item n=3 part=touches
Fake agent configured by argv alone and the fake-agent command line's modes and options: two new options, --trust-config and the test-only hold --trust-record-blank.
@task_item n=4 part=touches
The fake agent's trust dialog screen: a start whose folder the named config's record trusts draws no trust dialog.
@task_item n=5 part=touches
Harness boot makes the home, stamps it and starts wrappers (section 3 boot step 4): boot now stamps under the session's own user home with a seeded empty config and the fake agent told that file.
@task_item n=6 part=touches
Kill witness for a tied stamping verify: the stamping verify is now also tied to a test-owned home, held by a census.
@task_item n=7 part=touches
Counts a row's cell may hold: relied rows 22 to 23, step lines [NN/23], stamped line 23 pass 0 fail, interactive runs five to six, spawn pairs eight to nine, fake-agent starts per verify six to seven, env receipts six to seven; check the verify step line, summary line, last line, the stamped human line and the stamped line rows.
@task_item n=8 part=touches
Version gate needs every relied row and stamps refuse a widened ledger: the relied set is now 23 rows with trust-record last.
@task_item n=9 part=touches
Stamp shape: a stamp's row value for trust-record may now be the writer's code word, and a measured trust_probe write and remove pair is added, with no version bump.
@task_item n=10 part=touches
Agent identity and R8 CLAUDE environment strip rows: verify alone now reads the one value of CLAUDE_CONFIG_DIR, only when the config keeps that name.
@task_item n=11 part=touches
Every external reader is bounded and bounded readers keep the frame cap: two new config-file readers join the census.
@task_item n=12 part=touches
viola verify interrupted on SIGINT or SIGTERM: a stop during Run F removes its own record and exits 1.
@task_item n=13 part=touches
Failing ledger row is no refusal: a trust-record fail reads 22 pass 1 fail and exit 1.
@task_item n=14 part=touches
One standing committed live rig: a new v arm reads verify-exit, trust-record, pass-count and record-left, and counts a verify exit 1 as a whole start.
@task_item n=15 part=touches
Fake-agent run is not proof of real-CLI behaviour: trust-record is proven on the fake agent only; the live S0 reading is confounded and leaves the row unverified on the real CLI.
@task_item n=16 part=touches
60 s kill for the verify_window_ class: the verify_window_without_screens case now reads about 52 s against that 60 s kill (was about 43 s), Run F adding one designed wait.
@task_item n=17 part=touches
E2E layer selected by binary prefix and root integration binaries prefixed by surface: the new contract_ census runs in the E2E layer, not under the integration filter.
@task_item n=18 part=touches
OS-gated tests registered with a reason: the new stop case carries no unix gate, the tree being Linux-only, so no registry entry.
@task_item n=19 part=touches
Every test creates its own home and no root test spawns viola untied: test-started verifies now also carry a test-owned user home, held by a census.
@task_item n=20 part=touches
The local-live suite's pass: the expected live pass now spans 23 step lines.
@task_item n=21 part=touches
Tests never read the host's claude sessions: verify now writes and removes one record in the config its child reads; in tests that config is always test-owned.
@task_fact n=1 slug=test-data-bootstrap
fact: Every test-started verify and harness boot stamping runs under a test-owned user home whose config is seeded empty and owner-only, and the fake agent is told that file; a --record verify keeps the real HOME and points a kept CLAUDE_CONFIG_DIR at the test-owned user home instead.
@task_fact n=2 slug=integration-test-strategy
fact: A census binary holds every test-started verify start tied to a test-owned home, with a closed exempt list each entry carrying its reason, and is proven red on a planted untied start.
@task_fact n=3 slug=7-test-data-fixtures
fact: The fake agent reads a trust record by its own key rule, the nearest ancestor holding a git entry else the canonical cwd, through a bounded read, any failure reading untrusted.
@task_fact n=4 slug=7-test-data-fixtures
fact: The fake agent's test-only hold --trust-record-blank makes a start trusted by the record alone draw no screen, holding Run F's window open for a stop case.
@task_fact n=5 slug=6-e2e-test-strategy
fact: A live reading that fails a row is recorded as read and never as verification of the row; a confounded live reading leaves the row unverified on that CLI version and its live re-read owed.
@task_item n=1 part=untouched
MCP server, tools and tool results, and MCP and CLI parity rows.
@task_item n=2 part=untouched
Wrapper channel, JSON-RPC error bodies, link and unlink methods.
@task_item n=3 part=untouched
viola send, answer, wait, last, release and the wheel rows.
@task_item n=4 part=untouched
viola revive and run start refusals and exit codes.
@task_item n=5 part=untouched
Hook exec entry points, hook deadlines and performance budgets.
@task_item n=6 part=untouched
CI workflow, supply-chain, coverage and mutation gates, and the pre-push gate.
@task_item n=7 part=untouched
Log format, log event corr and process log line rows; no log event or field was added.
@task_item n=8 part=untouched
viola list board and harness status, pid-file and session record shapes.
