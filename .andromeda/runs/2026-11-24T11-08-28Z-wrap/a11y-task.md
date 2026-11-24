@task_prose part=session
The session added a new setup verb, viola adopt: it takes a plain claude session by its session-shaped id and resumes it under viola with an operator-given name, an optional parent and an optional model/effort pair.
Its start runs run's own path: home strict-modes check, the pinned exe, the wrapper log as process run, the holder collision check, then a live-elsewhere check revive already had, then the child in its own PTY under the TUI passthrough.
A session a live claude holds is refused with code session-live, exit 1, as one fixed unable/hint pair or one --json document; a malformed id or a parent equal to the name is a usage exit 2.
The help's setup group grew from 6 to 7 entries (adopt after revive); the served-verbs list grew 15 to 16; three error-body census rows and one exit-cause row were added; a test-only live-reading script for the real-CLI reading was added and never runs in a gate.
No new event kind, log line, closed word, time bound, env read, schema field or MCP tool; nothing retired; the real-CLI acceptance stays gated and its W1-W4 rulings stand PROVISIONAL on the delegate's word.
@task_file n=1 path=src/cmd/adopt.rs
what: the viola adopt command — its argument parsing, its start preflight order, its two refusals (strict-modes-failed and session-live) with their fixed unable/hint pairs and --json documents, and the lineage it gives the child
@task_file n=2 path=src/cmd/mod.rs
what: the CLI verb dispatch and the viola --help text, whose setup group label row lists the served setup verbs
@task_file n=3 path=src/main.rs
what: the wrapper's role naming for log identity and the silent roles at the internal-error catch site
@task_file n=4 path=src/cmd/revive.rs
what: viola revive — its session-id parser and its live-elsewhere (session-live) check, now shared with adopt, bodies unchanged
@task_file n=5 path=tests/cli_adopt.rs
what: the integration suite for viola adopt — its refusal pairs and --json documents, strict-modes refusal before any write, usage exits, the internal-error document, and the child in its own terminal
@task_file n=6 path=tests/cli_output_plain.rs
what: the literal pins of the help text and the served-verbs-only list of viola --help
@task_file n=7 path=tests/cross_exit_causes.rs
what: the cross-verb exit-cause table pinning each refusal code to its exit code and its hint line
@task_file n=8 path=tests/contract_error_bodies.rs
what: the error-body census giving every error body a writer and a witness
@task_file n=9 path=tests/security_negatives_env_capture.rs
what: the security negative cases on env capture under verify, with their CI kill bound per class
@task_file n=10 path=tests/support/home.rs
what: the test harness's outer-PTY wrapper driver (now able to start an adopted wrapper)
@task_file n=11 path=scripts/r5-adopt.sh
what: a test-only live-reading script over a real claude session, with a rehearsal mode over the fake agent; never run by a gate in live form
@task_item n=1 part=touches
contract/start-refusal/home-strict-modes — the run, revive start and verify home strict-modes refusal now also covers adopt's start (same pair, exit 1)
@task_item n=2 part=touches
contract/start-refusal/socket-dir — the start socket-directory refusal now also covers adopt's start
@task_item n=3 part=touches
decision/strict-modes-refusal-before-any-write — adopt's start refuses a widened home or instance dir before any log is written, mode left as found
@task_item n=4 part=touches
contract/json-fault/internal-error — adopt under --json prints the internal-error document for an error before the child is pumped (e.g. no claude on PATH)
@task_item n=5 part=touches
contract/cli-help/group-labels and contract/cli-help/served-verbs-only — the setup row now reads open, run, revive, adopt, up, verify, plugin install (6 to 7 entries); the served-verbs list holds adopt
@task_item n=6 part=touches
contract/revive-json-refusal/session-live and contract/revive-human-refusal/pair — the session-live code is now written by adopt too, with adopt's own hint; revive's rows stay true for revive
@task_item n=7 part=touches
decision/one-hint-per-code — session-live now has adopt's hint (close it there first, then adopt it again) beside revive's; check whether the rule holds per verb or needs an exception
@task_item n=8 part=touches
contract/start-json-refusal/document — the run/revive start refusal document now has a third writer, adopt's start, with the same shape
@task_item n=9 part=touches
contract/start-holder-refusal/already-live and contract/start-holder-refusal/stale-heartbeat — adopt's start uses run's holder collision check and prints run's pair under human and --json
@task_item n=10 part=touches
contract/cli-usage-error/exit-2 and contract/cli-exit-codes/2 — adopt adds usage exits: a malformed id, words after --, and a --parent equal to --name (run's fixed line)
@task_item n=11 part=touches
decision/claude-agents-read-fails-open — adopt's live-elsewhere check reads claude agents and starts when that read fails
@task_item n=12 part=touches
decision/error-bodies-have-writer-and-witness — three new error bodies written by adopt, each with its witness case
@task_item n=13 part=touches
contract/cli-refusal/format, contract/cli-refusal/unable-line, contract/cli-refusal/hint-line and contract/cli-refusal/stderr-layout — adopt's refusals are fixed unable/hint pairs in that layout
@task_item n=14 part=touches
decision/fixed-cli-strings-pinned-by-literal and decision/refusal-layout-literal-pins — the help literal and the new refusal pairs are pinned by literal tests
@task_item n=15 part=touches
contract/tui-passthrough/entry, contract/tui-passthrough/run-diagnostics and contract/log-identity/process — adopt hosts its child in the same TUI passthrough as run, its own PTY, diagnostics off the terminal, logging as process run
@task_item n=16 part=touches
contract/tui-passthrough/pump-child-error — that row names revive --json; adopt --json shares the pump_child path
@task_item n=17 part=touches
decision/live-readings-never-gate — a new live-reading script for the real claude adopt reading, run only on the operator's word, never by a gate
@task_item n=18 part=touches
decision/printed-names-are-parsed-arguments — adopt's refusal pairs print the parsed --name value
@task_fact n=1 slug=cognitive-accessibility
fact: viola adopt refuses a session that a live claude holds elsewhere with code session-live, exit 1, as the fixed pair unable: that session is open in another claude / hint: close it there first, then adopt it again (W4 planning form, PROVISIONAL on the delegate's word)
@task_fact n=2 slug=cognitive-accessibility
fact: viola adopt --json refusal codes are strict-modes-failed and session-live of its own plus run's shared already-live, stale-heartbeat, pinned-hash-mismatch, squatted-name, socket-dir-refused and internal-error, each one document with no hint
@task_fact n=3 slug=cognitive-accessibility
fact: viola adopt with a --parent equal to --name, a malformed session id, or words after -- is a usage exit 2 before anything is read; the session id takes ASCII hex of either case
@task_fact n=4 slug=a11y-scope-summary
fact: the viola --help setup group lists open, run, revive, adopt, up, verify, plugin install
@task_item n=1 part=untouched
Colour and SGR rows (amber DIALOG, bold NAME, dim stale rows, no SGR off a terminal) — adopt prints no colour and no board
@task_item n=2 part=untouched
The viola list board, its caption, order, UNWRAPPED separator and NAME truncation — not reached
@task_item n=3 part=untouched
viola link and unlink, viola wait, viola verify output and its interrupted pair, viola up output, plugin install output — not reached
@task_item n=4 part=untouched
Frame verb refusals, hook event and dialog calls, MCP server frame tools and the instance-unreachable details — no IPC method, MCP tool or frame refusal added
@task_item n=5 part=untouched
Screen-reader targets and manual Orca or keyboard passes, and the keyboard navigation rows — no interactive element added
@task_item n=6 part=untouched
CI integration and SLO budget rows (GitHub Actions workflow, coverage suite, no own artifact, zero flakiness) — no CI step or status shape changed
@task_item n=7 part=untouched
Absences (web or graphical frontend, touch surface, i18n, rendered page, time limit on the user) — none changes; no time bound added
@task_item n=8 part=untouched
Time-bound rows (dialog hold, open live wait, window wait, compositor read deadline) — no bound added or moved
