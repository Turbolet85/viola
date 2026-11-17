@task_prose part=session
The session added a 23rd relied ledger row, trust-record: viola verify writes a trust record for its own temp folder into the config file its claude child reads, starts a sixth interactive run (Run F) that must open with no trust modal, and removes that one record after a pass, a fail or a stop.
Verify's step lines now count to 23 (the step counter [NN/23], a last line for trust-record) and its closing stamp line reads 23 pass 0 fail; its interactive runs went five to six (A to F), its spawn pairs eight to nine.
The trust writer reports a closed set of 14 kebab-case code words (written, removed, config-absent, config-unreadable, config-link, config-not-regular, config-foreign, config-oversize, config-malformed, config-dir-unusable, home-unknown, key-present, lock-held, lost); a row's stamp value may now be that word when nothing was written, and no path, key or value reaches any line, stamp or summary.
Verify reads one more environment value, CLAUDE_CONFIG_DIR, only when the configured strip keeps that name, else the user home; the test fake agent gained a trust-config option and a test-only blank hold, and every test-started verify now runs under a test-owned home, held by a new census check.
One live reading (S0, claude 2.1.325) was recorded as read: trust-record fail, 7 of 23 pass, no record left; it does not verify the row, which stands proven on the fake agent and unverified on the real CLI, its re-read owed at route-resolve.
A watch was noted: the verify case with no screens reads 52 s against its 60 s kill, Run F adding one designed 8.5 s wait.
@task_file n=1 path=crates/viola-agent-claude/src/trust.rs
what: the pure trust-record rules in the one crate that holds claude's shapes — the config file's location, the record key, adding and removing the record, and the closed set of trust code words
@task_file n=2 path=src/trust_record.rs
what: the CLI's bounded config-file reader and writer for the trust record — validation of link, regular file and owner, a lock, three attempts, the file's own mode kept
@task_file n=3 path=tests/contract_verify_home.rs
what: a new contract census check that every test-started verify runs under a test-owned home, with a planted-copy case that reads it red
@task_file n=4 path=crates/viola-agent-claude/src/lib.rs
what: the claude crate's module list, now naming the trust module
@task_file n=5 path=crates/viola-agent-claude/src/ledger.rs
what: the relied ledger rows and their checks and stamp merge — the 23rd row trust-record, its check, the row's stamp value and the measured trust probe
@task_file n=6 path=crates/viola-agent-claude/src/ledger/test_cases.rs
what: the ledger's unit tests over the row list
@task_file n=7 path=crates/viola-agent-claude/src/ledger/test_cases_checks.rs
what: the ledger's unit tests over each row's check, now the trust-record check
@task_file n=8 path=crates/viola-agent-claude/src/ledger/test_cases_stamps.rs
what: the ledger's unit tests over the stamp merge, now the writer's code word as a row value
@task_file n=9 path=src/main.rs
what: the viola binary's module list, now naming the trust-record writer
@task_file n=10 path=src/cmd/verify.rs
what: the viola verify command — its inputs, its step-line output and the one CLAUDE_CONFIG_DIR read that finds the config file
@task_file n=11 path=src/cmd/verify/typed.rs
what: verify's typed interactive runs, now carrying the trust frame, write and remove words
@task_file n=12 path=src/cmd/verify/typed/runs.rs
what: verify's interactive PTY runs A to F, Run F the trust-record run with its guard that removes the record on drop
@task_file n=13 path=src/run/version_gate.rs
what: the version gate that reads the stamped ledger rows, a companion edit for the 23-row count
@task_file n=14 path=src/bin/viola-fake-agent.rs
what: the test-only fake agent, now reading a named trust config by argv and a test-only blank hold
@task_file n=15 path=src/bin/viola-fake-agent/test_cases_opts.rs
what: the fake agent's argv option tests, now the trust-config option and the record read
@task_file n=16 path=tests/support/verify.rs
what: the test support that starts viola verify, now under a test-owned user home with the trust config handed to the fake
@task_file n=17 path=tests/cli_verify.rs
what: the integration tests of viola verify's output and stamps, now the trust-record row's pass, fail, config-absent, kept-config-dir and stop cases
@task_file n=18 path=tests/cli_verify/runs.rs
what: the integration tests of verify's interactive runs, now six
@task_file n=19 path=tests/cli_verify/stamps.rs
what: the integration tests of verify's stamp file and its pass and fail counts
@task_file n=20 path=tests/contract_ledger_probes.rs
what: the contract check over the ledger probes and the stamped pass-count literal
@task_file n=21 path=tests/contract_bounded_readers.rs
what: the contract list of bounded readers, now the two trust-record readers
@task_file n=22 path=crates/viola-e2e/src/harness/boot.rs
what: the E2E harness boot, whose stamping step now runs under the session's own user home with the trust config
@task_file n=23 path=crates/viola-e2e/src/harness/run.rs
what: the E2E harness run, holding the live pass literal of verify's step lines and the ledger row count
@task_file n=24 path=crates/viola-e2e/src/rig/mod.rs
what: the live rig's arms and summary, now the verify arm v and its summary keys
@task_file n=25 path=crates/viola-e2e/src/rig/isolate.rs
what: the live rig's isolation, now the child temp root and the read of a verify record left behind
@task_file n=26 path=crates/viola-e2e/src/bin/viola-live-rig.rs
what: the live rig binary's arm words and child-argument substitution
@task_file n=27 path=crates/viola-e2e/tests/rig_rehearsal.rs
what: the live rig rehearsal tests, now the v arm's rehearsal
@task_item n=1 part=touches
contract/viola-verify/step-line: verify's step counter moved from [NN/22] to [NN/23], with a new last step line for trust-record; any count of rows or step lines it holds is stale.
@task_item n=2 part=touches
decision/critical-path-cli-board-output: verify output, one of the critical path's outputs, now prints one more step line and a stamped line of 23 pass 0 fail.
@task_item n=3 part=touches
contract/interrupted-verify/exit, contract/interrupted-verify/stderr and contract/viola-verify/interrupted-pair: a new case stops verify during Run F; it exits 1 and removes its own record — the interrupted output now covers a sixth run.
@task_item n=4 part=touches
decision/fixed-cli-strings-pinned and decision/fixed-cli-strings-pinned-by-literal: the pinned literals of verify's step lines and the stamped pass-count line moved from 22 to 23, and a full live pass literal of the 23 step lines was added to the harness.
@task_item n=5 part=touches
contract/code-word/format, decision/codes-beside-plain-word and decision/unlisted-code-words-stay-visible: the trust writer adds a closed set of 14 kebab-case code words that may stand as a row's stamp value in place of pass or fail.
@task_item n=6 part=touches
decision/paths-not-carried: the new config-file reader and writer carry no path, key or value into any line, stamp or summary; the rule now reaches the trust record.
@task_item n=7 part=touches
contract/start-refusal/home-strict-modes: verify now finds the claude config through the user home or a kept CLAUDE_CONFIG_DIR, and every test-started verify runs under a test-owned home; a home that cannot be read gives the code word home-unknown, not a refusal.
@task_item n=8 part=touches
decision/live-readings-never-gate: one live reading S0 on claude 2.1.325 read trust-record fail, 7 of 23; it was recorded as read, verified nothing and gated nothing.
@task_item n=9 part=touches
decision/no-screen-content-parsing and decision/boundary-check-never-inspects-claude-content: the trust-record check reads Run F's frame for the absence of the trust modal's literal and for the input box signature — a signature read, no further parsing.
@task_item n=10 part=touches
decision/time-bounds-fall-on-the-driver and decision/time-bounds-never-on-human: Run F adds one designed 8.5 s wait to verify; the no-screens verify case now reads 52 s against its 60 s kill — a bound on the driver only, noted as a watch.
@task_item n=11 part=touches
decision/zero-flakiness: the no-screens verify case runs within 8 s of its kill line, noted as a watch to keep it from flaking.
@task_item n=12 part=touches
decision/a11y-runs-in-existing-test-suites and decision/a11y-inside-existing-suites: a new contract binary for the verify home census runs in the E2E layer by its contract_ prefix, not under the integration filter.
@task_item n=13 part=touches
decision/linux-only and decision/ci-test-gate-no-skips: the new stop case carries no unix gate, as the tree is Linux-only, and adds no OS-gated row.
@task_item n=14 part=touches
decision/static-linear-output and decision/linear-static-cli-output: verify still prints its steps as a fixed linear list, now one line longer; no spinner or progress redraw added.
@task_fact n=1 slug=cognitive-accessibility
fact: The trust-record row's stamp value is pass, fail, or one of a closed set of 14 kebab-case code words from the trust writer (written, removed, config-absent, config-unreadable, config-link, config-not-regular, config-foreign, config-oversize, config-malformed, config-dir-unusable, home-unknown, key-present, lock-held, lost) when no record was written.
@task_fact n=2 slug=1-a11y-scope-summary
fact: viola verify's step lines count 23 relied rows, the last trust-record, and its stamped line reads 23 pass 0 fail on a full pass.
@task_item n=1 part=untouched
Colour and SGR rows — amber DIALOG, bold NAME, dim stale rows, colour decision inputs, no SGR off a terminal: verify's new line adds no colour and no SGR rule changed.
@task_item n=2 part=untouched
TUI passthrough rows — entry, human keystrokes, focus, mouse and resize sequences, vendor content, terminal bytes, feed panics: viola run's passthrough is not reached.
@task_item n=3 part=untouched
viola list board, its caption, order, empty board and unwrapped separator: no board output changed.
@task_item n=4 part=untouched
Refusal layout rows — unable and hint lines, one hint per code, exit codes 2, 10 to 21, --json failure documents, revive refusals, link and unlink, plugin install, send refusals, MCP frame and wait tools, hook calls: no refusal word, hint or exit code was added or changed.
@task_item n=5 part=untouched
viola --help rows and the served verbs: no verb or help line was added.
@task_item n=6 part=untouched
Screen-reader targets and manual Orca and keyboard passes, CI workflow rows, structured violation JSON and the absences of a web frontend, touch surface, rendered page and i18n: none moved.
