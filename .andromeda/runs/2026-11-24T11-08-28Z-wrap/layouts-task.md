@task_prose part=session
The session added a new CLI verb, viola adopt <SESSION_ID> --name <NAME> [--parent <NAME>] [--model <MODEL>] [--effort <EFFORT>] [--json]: a plain claude session, given by its session id, is resumed under viola with an operator-given name and parent.
Adopt runs through run's own start: home strict-modes check, pinned exe, the run wrapper log, the live-name holder check, then revive's live-elsewhere check, then a resume launch (--resume <id>) in the current folder, no window.
Adopt's refusals are fixed pairs or one --json document: strict-modes-failed, session-live (unable: that session is open in another claude / hint: close it there first, then adopt it again), plus run's shared already-live, stale-heartbeat, pinned-hash-mismatch, squatted-name, socket-dir-refused and internal-error.
A malformed id or a --parent equal to --name is a usage exit 2; the id parser is the one revive's --id uses and takes hex of either case.
The help's setup: row grew from 6 to 7 entries (adopt after revive); no new IPC method, event, log line, closed word or MCP tool; the adopt start is silent at the internal-error catch like run and revive.
The gated real-CLI acceptance (v2-13) stays GATED; the W1-W4 widenings stand PROVISIONAL on the delegate's word.
@task_file n=1 path=src/cmd/adopt.rs
what: the viola adopt verb: its arguments, its start preflight order, its refusal pairs and --json refusal documents, its lineage and resume launch
@task_file n=2 path=src/cmd/mod.rs
what: the CLI verb dispatch and the viola --help text, its setup: verb group row
@task_file n=3 path=src/main.rs
what: the CLI entry point and its internal-error catch site, the roles silent there
@task_file n=4 path=src/cmd/revive.rs
what: the viola revive verb: its session-id parser and its session-live check, now shared with adopt
@task_file n=5 path=tests/cli_adopt.rs
what: the integration tests of the viola adopt verb, its start, refusals and usage
@task_file n=6 path=tests/cli_output_plain.rs
what: the human output contract tests, the --help text and its served-verb list
@task_file n=7 path=tests/cross_exit_causes.rs
what: the table of every refusal cause with its exit code and hint, checked across verbs
@task_file n=8 path=tests/contract_error_bodies.rs
what: the census of refusal and error bodies each verb writes, the fixed pairs and documents
@task_file n=9 path=tests/security_negatives_env_capture.rs
what: the security negative tests of environment capture, their verify-driven cases and kill bound class
@task_file n=10 path=tests/support/home.rs
what: the test harness for a viola home and its wrapper starts
@task_file n=11 path=scripts/r5-adopt.sh
what: the test-only live-reading script for the gated adopt acceptance over a real claude, with a rehearsal over the fake agent
@task_item n=1 part=touches
The viola --help verb group rows: the setup: row now reads open, run, revive, adopt, up, verify, plugin install (7 entries, was 6); any row listing the setup group or the served verbs is stale.
@task_item n=2 part=touches
The CLI verbs list: a new verb viola adopt <SESSION_ID> --name <NAME> [--parent <NAME>] [--model] [--effort] [--json] joins it, beside run and revive.
@task_item n=3 part=touches
The per-verb usage (viola <verb> --help) now covers adopt; adopt takes no words after --, unlike run.
@task_item n=4 part=touches
The revive --json error codes and the start refusals shared by run and revive (internal-error, pinned-hash-mismatch, socket-dir-refused, squatted-name, already-live, stale-heartbeat, strict-modes-failed, session-live): adopt now writes the same codes, so rows naming run and revive as their only starts are now incomplete.
@task_item n=5 part=touches
The revive session-open-in-another-claude refusal and the revive session-live check: the same live-elsewhere check now guards adopt, with its own hint line naming adopt.
@task_item n=6 part=touches
The revive malformed --id rule: the same session-id parser serves adopt's positional id; a malformed id is a usage exit 2 before anything is read; the shape takes either hex case.
@task_item n=7 part=touches
The run and revive starts' home-tree-refused and socket-dir-refused refusals: adopt's start refuses the same way, before any log, mode left as found.
@task_item n=8 part=touches
The run and revive usage rule on a launch pair after the dashes: adopt takes its pair as --model and --effort flags only, through run's parsers.
@task_item n=9 part=touches
The CLI internal error contract and the catch site row that names CLI verbs outside a revive start: the adopt start is now also silent at the internal-error catch, its --json form prints the internal-error document.
@task_item n=10 part=touches
The revive preflight order: adopt has its own order, strict-modes check, pinned exe, wrapper log, live-name holder, then live-elsewhere, then launch.
@task_item n=11 part=touches
The revive window placement (no window) and passed start output: an adopted start likewise opens no window and starts in the current folder.
@task_item n=12 part=touches
The viola open --parent default: adopt's parent is --parent, else the inherited VIOLA_NAME unless equal to the name; a --parent equal to --name is a usage exit 2 with the line run uses.
@task_item n=13 part=touches
The decisions that every refusal cause has a hint, hints name the next verb, the hint is the last stderr line: the new session-live hint for adopt names adopt again.
@task_item n=14 part=touches
The decision that --json is on every data verb and the one-document --json mode: adopt carries --json with one document per refusal or success.
@task_item n=15 part=touches
The CLI exit codes: adopt uses exit 1 for its refusals and exit 2 for usage; no new code.
@task_item n=16 part=touches
The decision that a CLI target is always a ViolaName: adopt's --name and --parent are ViolaNames, but its positional is a session id, not a target.
@task_fact n=1 slug=surface-cli
fact: viola adopt <SESSION_ID> --name <NAME> [--parent <NAME>] [--model <MODEL>] [--effort <EFFORT>] [--json] resumes a plain claude session by its id under viola with the given name and parent, in the current folder, no window, logging as process run (PROVISIONAL on the delegate's word, W1-W4).
@task_fact n=2 slug=surface-cli
fact: viola adopt refuses a session a live claude holds with session-live, exit 1, the pair unable: that session is open in another claude / hint: close it there first, then adopt it again (W4 planning form, PROVISIONAL).
@task_fact n=3 slug=surface-cli
fact: viola adopt --json refusal codes are strict-modes-failed and session-live, plus run's shared already-live, stale-heartbeat, pinned-hash-mismatch, squatted-name, socket-dir-refused and internal-error.
@task_fact n=4 slug=surface-cli
fact: viola adopt usage: the id is the session-id shape revive's --id takes (hex of either case); a malformed id, words after --, or a --parent equal to --name are a usage exit 2.
@task_fact n=5 slug=surface-cli
fact: viola adopt's start preflight order is strict-modes check, pinned exe, wrapper log, live-name holder, then the live-elsewhere check, then the resume launch; the adopt start is silent at the internal-error catch.
@task_item n=1 part=untouched
The web-spa surface and its absence of a web page: no web surface added.
@task_item n=2 part=untouched
The viola list board and the BAY line, their columns, widths, truncation, stale rows and tree: no board change.
@task_item n=3 part=untouched
The MCP server and its tools and tool results: no MCP tool added or changed.
@task_item n=4 part=untouched
The send, wait and last output, the readback mirror and its vocabulary: no change to sending or waiting.
@task_item n=5 part=untouched
The CLI token rendering, colour, styling, spinner, banner and footer decisions: adopt adds no styled element.
@task_item n=6 part=untouched
The link and unlink output, the wheel verb output (pause, release) and viola answer output: untouched.
@task_item n=7 part=untouched
The plugin install output and the absence of deep links in the CLI: untouched.
