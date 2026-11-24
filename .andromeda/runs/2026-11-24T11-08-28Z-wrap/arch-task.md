@task_prose part=session
The session added the CLI verb viola adopt: a plain claude session not started under viola, named by its session id, is resumed under viola with an operator-given name, an optional parent and an optional model and effort pair.
Adopt takes the id through the same session-id parser revive uses, and the name, parent and pair through run's own parsers. A parent equal to the name is a usage exit 2.
Adopt starts through run's start path in this order: instance check, checked exe, run's log, the held-name check, the live-elsewhere check revive uses, then a child spawned with --resume and the id. It refuses a session a live claude holds with session-live, exit 1, and a fixed hint pair.
Adopt logs as process run and takes the silent-at-the-catch role. It adds no IPC method, event kind, log line, record field, env read, time bound, closed word or MCP tool. The help's setup row now lists adopt after revive.
A measurement showed the shared session-id parser accepts hex of either case, so an upper-case id parses. The real-CLI acceptance (v2-13) stays GATED and unproven. The widenings W1 to W4 land PROVISIONAL on the delegate's word, his own word still owed.
@task_file n=1 path=src/cmd/adopt.rs
what: new - the adopt CLI verb: its arguments, its start preflight order, its two closed refusals (strict-modes-failed, session-live) and the lineage it hands the start
@task_file n=2 path=src/cmd/mod.rs
what: edited - the CLI subcommand set and its dispatch, and the help text's setup row of verbs
@task_file n=3 path=src/main.rs
what: edited - the root bin's role mapping for the internal-error catch site, which names the roles kept silent there
@task_file n=4 path=src/cmd/revive.rs
what: edited - the revive verb; its session-id parser and its live-elsewhere check (a session a live claude holds) are now shared with adopt
@task_file n=5 path=tests/cli_adopt.rs
what: new - the integration test binary for the adopt verb over the fake agent
@task_file n=6 path=scripts/r5-adopt.sh
what: new - a test-only live-reading script for the real-CLI adopt acceptance, with a rehearsal mode over the fake agent
@task_file n=7 path=tests/cli_output_plain.rs
what: edited - the human output test holding the help literal and the list of served verbs
@task_file n=8 path=tests/cross_exit_causes.rs
what: edited - the cross-verb exit-cause table of refusal details, exit codes and hints
@task_file n=9 path=tests/contract_error_bodies.rs
what: edited - the error-body census check over every writer of a refusal or error body
@task_file n=10 path=tests/security_negatives_env_capture.rs
what: edited - the security negative tests on environment capture, including the verify-window class under its own kill bound
@task_file n=11 path=tests/support/home.rs
what: edited - the test-support wrapper harness that starts a wrapped instance in an outer PTY and waits it ready
@task_item n=1 part=touches
CLI verbs and command names: a new verb adopt joins the CLI verb set and occupies the command name adopt; the help's setup row grows from 6 to 7 entries.
@task_item n=2 part=touches
Revive refuses a session a live claude holds: the same live-elsewhere refusal (session-live) now also guards adopt.
@task_item n=3 part=touches
Run, revive, mcp and hook silent at the catch site: adopt joins the roles silent at the internal-error catch site.
@task_item n=4 part=touches
viola revive --json error codes and the revive refusal-codes output: adopt now has its own --json refusal code list beside revive's (strict-modes-failed, session-live, already-live, stale-heartbeat, pinned-hash-mismatch, squatted-name, socket-dir-refused, internal-error); revive's list stays true for revive.
@task_item n=5 part=touches
Flags passed to the child and the claude child's argv, --resume: an adopted start also passes --resume with the operator-typed session id; any row naming only revive or verify as the starts that pass --resume is now incomplete.
@task_item n=6 part=touches
Logged session id on the child's command line: adopt puts an operator-typed id, not a logged one, onto the child's argv as one element of a direct spawn (W2, PROVISIONAL).
@task_item n=7 part=touches
Claude session id shape: measured that the shared parser accepts hex of either case; a row saying an upper-case id is refused would be wrong.
@task_item n=8 part=touches
Instance snapshot cwd and the spawn directory confined to the snapshot: adopt spawns in the caller's current directory and writes that as the snapshot cwd; there is no earlier snapshot for it to come from.
@task_item n=9 part=touches
Lineage (instance snapshot lineage, lineage event data, lineage is a label only, run --parent): adopt's parent is --parent, otherwise the inherited VIOLA_NAME unless it equals the name, with no role.
@task_item n=10 part=touches
Start order and start refusals (child spawned last, a start refuses a held name, no start over a live or stale instance, viola run and revive start refusals for home tree and socket-dir, run and revive start exit code 1, pinned-hash-mismatch start refusal): adopt's start goes through run's start path and these same checks, so rows that name only run and revive leave adopt out.
@task_item n=11 part=touches
Instance check before any read and strict-modes check before any log: adopt checks the instance before it opens its log and refuses a widened home or instance dir with strict-modes-failed.
@task_item n=12 part=touches
Launch pair (launch pair handed, never verified; recorded launch pair on the child's command line; compiled default launch pair; run --model and run --effort): adopt hands the given pair through run's parsers and records it like any start.
@task_item n=13 part=touches
viola CLI arguments: adopt takes a positional session id, --name, --parent, --model, --effort and --json, and no words after the double dash (a usage exit 2); CLI exit code 2 now also covers a parent equal to the name and a malformed id.
@task_item n=14 part=touches
Home-level diagnostics role file run-<name>.ndjson and the process-exit detail enum: adopt logs as process run into run's role file and reuses existing details (session-live, already-live, stale-heartbeat, internal-error).
@task_item n=15 part=touches
CLI --json internal-error output and no JSON document once the child held the terminal: under --json, an error before the child holds the terminal prints the internal-error document.
@task_item n=16 part=touches
Agents listing degrades and never errors: adopt's live-elsewhere check reads claude agents, and a failed agents read lets the start go ahead.
@task_item n=17 part=touches
No shelling out and child program resolution: adopt resolves the claude program and spawns it directly with a closed argv.
@task_item n=18 part=touches
New capabilities as subcommands: adopt is added as a new subcommand of the one binary.
@task_fact n=1 slug=established-decisions
fact: viola adopt resumes a plain claude session not started under viola, named by its session id, under viola with an operator-given name and optional parent and launch pair, and it refuses (session-live, exit 1) a session a live claude holds, as revive does (W1, PROVISIONAL on the delegate's word).
@task_fact n=2 slug=standard-contracts
fact: viola adopt --json refusal codes are strict-modes-failed, session-live, already-live, stale-heartbeat, pinned-hash-mismatch, squatted-name, socket-dir-refused and internal-error. Its only own closed details are strict-modes-failed and session-live. The session-live human pair reads unable: that session is open in another claude / hint: close it there first, then adopt it again (W4 wording, PROVISIONAL).
@task_fact n=3 slug=conventions
fact: CLI verb adopt takes the arguments <SESSION_ID> --name <NAME> [--parent <NAME>] [--model <MODEL>] [--effort <EFFORT>] [--json]. The id is in the closed session-id shape (hex of either case), and no words are allowed after the double dash. A --parent equal to --name is a usage exit 2.
@task_fact n=4 slug=occupied-resources
fact: The command name adopt is occupied.
@task_fact n=5 slug=occupied-resources
fact: An adopted start puts the operator-typed session id on the claude child's argv after --resume, as one element of a direct spawn, only in the session-id shape (W2, PROVISIONAL on the delegate's word).
@task_fact n=6 slug=established-decisions
fact: Adopt's lineage parent is --parent, otherwise the inherited VIOLA_NAME unless that equals the name. Adopt sets no role and launches with no window.
@task_fact n=7 slug=standard-contracts
fact: An adopted start spawns in the caller's current directory and records that directory as the instance snapshot cwd.
@task_fact n=8 slug=established-decisions
fact: Adopt logs as process run in run-<name>.ndjson and adds no IPC method, event kind, log line, record field, env read, time bound or MCP tool of its own.
@task_item n=1 part=untouched
Hook tiers, hook-to-kind map, hook contract, dialog hold and dialog hooks: adopt adds and changes no hook.
@task_item n=2 part=untouched
Wrapper channel methods, frames, fault codes, protocol versioning and peer admission: no channel method or frame shape changed.
@task_item n=3 part=untouched
MCP server, MCP tools and their results and deadlines: no MCP tool added or changed.
@task_item n=4 part=untouched
The set of normalised event kinds and the wrapper event appends' kinds: no kind added.
@task_item n=5 part=untouched
Wheel, send, paste, confirmation windows, local commands and the answer path: nothing changed.
@task_item n=6 part=untouched
viola verify, ledger stamps, the capability ledger, trust records and screen signatures: nothing changed.
@task_item n=7 part=untouched
config.json keys, environment-variable naming and the harness environment: no key or variable added.
@task_item n=8 part=untouched
Dependencies, crate graph and dependency direction, toolchain and MSRV pins, cargo-deny policy, CI workflow steps, coverage floors, release and deployment: no crate, dependency, CI step or gate threshold changed.
@task_item n=9 part=untouched
Hyprland window seam, focus, open and up: adopt opens no window and none of those verbs changed.
@task_item n=10 part=untouched
The board, list payload and session liveness states: no field or status changed.
@task_item n=11 part=untouched
Design philosophy, stack, project intent and inherited defaults: no principle or stack choice moved.
