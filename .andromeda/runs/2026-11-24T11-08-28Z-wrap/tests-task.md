@task_prose part=session
The session added the CLI verb viola adopt: a plain claude session, given by its session-shaped id, resumed under viola with an operator-given name, an optional parent and an optional model/effort pair, started through run's own start path in the current folder.
Adopt refuses a session a live claude holds (session-live, exit 1, fixed unable/hint pair, no child) and a widened home or instance dir (strict-modes-failed) before any log; a failed agents read still starts; run's shared refusals (already-live, stale-heartbeat, pinned-hash-mismatch, squatted-name, socket-dir-refused, internal-error) apply; under --json an error before the child pump prints the internal-error document.
A malformed id, a --parent equal to --name or words after -- are usage exits 2; the id shape takes ASCII hex of either case (an upper-case id parses).
Tests added: a new integration binary for adopt (12 cases), unit cases (21), a session-live row in the cross exit-cause table, three error-body census rows, the help setup row now listing adopt (6 to 7 entries, served list 15 to 16), and the two verify-driven cases of the env-capture security binary moved into the verify_window_ class (0 to 2).
A test-only live-reading script for R5 (R5a adopt of a real ended session, R5b refusal of a real live one) was added with a fake-agent rehearsal; R5 stays unfired, matrix v2-13 stays claimed GATED, and the W1 to W4 widenings stand PROVISIONAL on the delegate's word.
@task_file n=1 path=src/cmd/adopt.rs
what: the viola adopt verb: its arguments, preflight order, refusals (session-live, strict-modes-failed) and the resumed start through run's start
@task_file n=2 path=src/cmd/revive.rs
what: viola revive: its session-id parser and its live-elsewhere (session-live) check, now shared with adopt
@task_file n=3 path=src/cmd/mod.rs
what: the CLI verb dispatch and the viola --help grouped verb table (the setup row)
@task_file n=4 path=src/main.rs
what: the role of each verb at the internal-error catch site (silent roles)
@task_file n=5 path=tests/cli_adopt.rs
what: the root integration binary for viola adopt over the fake agent
@task_file n=6 path=tests/cli_output_plain.rs
what: the plain CLI output check: the literal help text and the served-verbs list
@task_file n=7 path=tests/cross_exit_causes.rs
what: the cross-surface exit-cause table (CAUSES), each cause with its own hint
@task_file n=8 path=tests/contract_error_bodies.rs
what: the census of named product error writers and their error bodies
@task_file n=9 path=tests/security_negatives_env_capture.rs
what: the env-capture security negatives, its verify-driven cases in the verify_window_ class
@task_file n=10 path=tests/support/home.rs
what: the shared test-home and test-wrapper support (stamped homes, Wrapper starts in an outer PTY, held in a drop guard)
@task_file n=11 path=scripts/r5-adopt.sh
what: the gated live-reading script R5 for adopt (R5a, R5b) with its fake-agent rehearsal, test-only
@task_item n=1 part=touches
Root integration binaries prefixed by surface: a new cli_ binary for adopt joins the integration binaries.
@task_item n=2 part=touches
CLI human output internal error line (cli-human/internal-error): its silent list of roles gains adopt's start.
@task_item n=3 part=touches
viola --help grouped verb table: the setup row now reads open, run, revive, adopt, up, verify, plugin install (6 to 7 entries); help carries no bold or underline still holds.
@task_item n=4 part=touches
Strict-modes refusal rows naming run, mcp, revive and verify, and strict-modes check before any read or open, widened home tree refused at start: adopt's start joins them (refused before any log, mode left as found).
@task_item n=5 part=touches
Run and revive exit 1 causes and run and revive start refusals (already-live, stale-heartbeat, socket-dir-refused, squatted-name, pinned-hash-mismatch, strict-modes-failed): adopt's start shares them through run's start.
@task_item n=6 part=touches
revive exit 1 cause session-live and revive refusal code session-live: the same code, pair form and exit now also refuse an adopt of a session open in another claude.
@task_item n=7 part=touches
session-live reading fails open: adopt also starts when the agents read fails (W1, PROVISIONAL).
@task_item n=8 part=touches
revive preflight order: adopt has its own order (instance check, pinned exe, wrapper log, holder, live-elsewhere); a refused holder takes no agents read.
@task_item n=9 part=touches
Each exit cause has its own hint: the cross exit-cause table gains a session-live row with adopt's hint.
@task_item n=10 part=touches
Every product error writer is named and error bodies held by surface witnesses: three new error-body rows for adopt, the internal-error one witnessed by a --json case with no claude on PATH.
@task_item n=11 part=touches
80 s kill for the verify_window_ class: the class gains the two verify-driven cases of the env-capture security binary (CI under coverage 11.520 s and 14.588 s of 80 s); the binary's other three cases keep the 20 s kill line for verify-driven and stamped-home tests.
@task_item n=12 part=touches
Real claude CLI tested local-only and fake-agent run is not proof of real-CLI behaviour: adopt's real-CLI clause rests on the gated live reading R5, unfired; the fake-agent rehearsal is no proof.
@task_item n=13 part=touches
Live reading recorded as read, never as verification, and refusal rehearsal reads the refusal's own effects: the R5 script's rehearsal prints a rehearsed line, not a live reading; R5b reads exit 1, session-live, no child.
@task_item n=14 part=touches
Tests never read the host's claude sessions and agents list answered by a stub on PATH: adopt's cases read the agents listing from the test stub, never the host.
@task_item n=15 part=touches
Test runs and revives under a test-owned user home, and test wrappers held in a drop guard: adopt's test starts run the same way through the shared support.
@task_item n=16 part=touches
Child environment strip and claude child holds nothing of viola's: adopt's child is spawned through run's start, with the same strip, in its own PTY.
@task_item n=17 part=touches
Fake-agent receipt start row: the start receipt holds no argv; the adopted child's --resume and id are read from the live child's /proc cmdline instead.
@task_item n=18 part=touches
Contract tables are literal and CLI fixed strings pinned by literals: adopt's refusal pairs and help line are pinned as literals.
@task_fact n=1 slug=e2e-test-strategy
fact: Real adopt of a plain claude session (resume of a real ended session, refusal of a real live one) is proven only by the gated live reading R5; the fake-agent rehearsal of its script is not a live reading.
@task_fact n=2 slug=e2e-test-strategy
fact: viola adopt preflight order is instance check, pinned exe, wrapper log, holder, then the live-elsewhere read; the first refusal wins and a refused holder takes no agents read.
@task_fact n=3 slug=e2e-test-strategy
fact: viola adopt refuses with exit 1 session-live when the session id is open in another claude or held by a live adopted instance, with the pair unable that session is open in another claude and hint close it there first, then adopt it again, and starts no child (W4 wording a planning form, PROVISIONAL).
@task_fact n=4 slug=e2e-test-strategy
fact: viola adopt's --json refusal codes are strict-modes-failed and session-live beside run's shared already-live, stale-heartbeat, pinned-hash-mismatch, squatted-name, socket-dir-refused and internal-error; an error before the child pump prints the internal-error document.
@task_fact n=5 slug=e2e-test-strategy
fact: viola adopt's id is accepted only in the session-id shape (ASCII hex of either case) as one argv element after --resume of a direct spawn; a malformed id, a --parent equal to --name, or words after -- are a usage exit 2 before anything is read (W2, PROVISIONAL).
@task_fact n=6 slug=test-data-fixtures
fact: The fake agent's start receipt records no argv; a test claim about the child's argv is read from the live child's /proc cmdline.
@task_fact n=7 slug=e2e-test-strategy
fact: viola adopt's lineage parent is --parent, else the inherited VIOLA_NAME unless equal to the given name; its snapshot cwd is the folder adopt was run in.
@task_item n=1 part=untouched
The harness and agent-run contracts (boot, cleanup, status document, gate, pre-push, run document, session record, exit codes): no agent-run verb, CI step or status shape changed.
@task_item n=2 part=untouched
The MCP surface (tools list, tool results, invalid-params fields, stdin line cap): no MCP tool added or changed.
@task_item n=3 part=untouched
The hook surface (viola hook entry points, dialogs, decision bodies, perf budgets for hooks): untouched.
@task_item n=4 part=untouched
send, wait, last, answer, release and the wrapper channel and its JSON-RPC errors: no IPC method, event kind or wake changed.
@task_item n=5 part=untouched
Log format and log event corr rows and the diagnostics schema: no new log line, span, field or closed word; adopt logs through run's existing lines as process run.
@task_item n=6 part=untouched
CI workflows, artifacts, supply-chain, fuzz, mutation and coverage floors: no workflow, tool, threshold or kill line changed.
@task_item n=7 part=untouched
The list board and list --json, focus, open, up and window placement and raise rows: not reached.
@task_item n=8 part=untouched
Fixture recording, scrub and hygiene rows, and the fake agent's command line options: no fake-agent mode or fixture added.
