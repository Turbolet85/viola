@task_prose part=session
The session added the CLI verb viola adopt: it resumes a plain claude session, given by its session id, under viola, with a name, an optional parent and an optional model and effort pair, given by the operator.
Adopt runs these checks in order: instance strict-modes, pinned exe, log open, holder collision check, live-elsewhere check. Then it starts the child through run's own start, with --resume and the id on the argv of a direct spawn, its own child in its own PTY.
A session that a live claude holds is refused as session-live, exit 1, with one fixed pair and no child started. A failed read of the claude agents list lets adopt start anyway. A malformed id is a usage exit 2 before anything is read.
Measured: the shared session-id parser accepts ASCII hex in either case, so an upper-case id parses. The plan's row saying an upper-case id is refused was wrong. Every security-plan row only says the closed session-id shape.
It adds no IPC method, event kind, log line, record field, env read, time bound, closed word, MCP tool, dependency or schema. Adopt logs as process run, through run's own lines. Nothing was retired.
W1 to W4 rest on the delegate's word and land PROVISIONAL. The real-CLI gate R5 is unfired, and matrix v2-13 stays claimed GATED.
@task_file n=1 path=src/cmd/adopt.rs
what: the new viola adopt CLI verb, which is a start path. It holds the argument checks (session id, name, parent, launch pair), the refusal pairs and --json documents (strict-modes-failed, session-live), the preflight check order (instance strict-modes, pinned exe, log open, collision check, live-elsewhere), the lineage choice and the launch with --resume on the child argv
@task_file n=2 path=src/cmd/revive.rs
what: viola revive's start, which holds the session-id parser for --id (the closed session-id shape) and the live-elsewhere check that refuses a session a live claude holds. Both are now shared with adopt, with their bodies unchanged
@task_file n=3 path=src/cmd/mod.rs
what: the CLI verb dispatch and the help text. The adopt command is dispatched with a run-process detail sink, and the help's setup row now lists adopt
@task_file n=4 path=src/main.rs
what: the binary's role mapping at the internal-error catch site, where adopt is a named wrapper role that stays silent at the catch site
@task_file n=5 path=tests/cli_adopt.rs
what: the new integration test binary for viola adopt. It covers resume under name and parent, the own child in its own terminal, the session-live refusals, a failed agents read that still starts, the collision pair, the strict-modes refusal of widened state, the malformed id as a usage error and the internal-error document
@task_file n=6 path=tests/cli_output_plain.rs
what: the plain CLI output test, holding the help literal and the list of served verbs
@task_file n=7 path=tests/cross_exit_causes.rs
what: the cross-cutting exit-cause table test, which pairs each refusal with its exit code and hint. A session-live row for adopt was added
@task_file n=8 path=tests/contract_error_bodies.rs
what: the census of product error writers and their fixed bodies (the error-writer guard). Three rows were added for adopt's writers
@task_file n=9 path=tests/security_negatives_env_capture.rs
what: the security negatives binary for environment capture, whose verify-driven cases moved into the verify_window_ timing class
@task_file n=10 path=tests/support/home.rs
what: the test-home support, which gains a wrapper started by viola adopt on an outer PTY over the fake agent
@task_file n=11 path=scripts/r5-adopt.sh
what: a new test-only live-reading script for gate R5. It hosts adopt under script over a FIFO and ends only the wrapper it recorded (while that process's exe is viola) and its own script host. Its verdicts hold words only, it sends the screen to /dev/null and it has a rehearse mode over the fake agent
@task_item n=1 part=touches
The rows on the session id on the child argv (closed session-id shape, one argv element, no shell) now also cover adopt's operator-typed positional id (W2 PROVISIONAL). The shape takes either hex case, and no row should claim upper case is refused.
@task_item n=2 part=touches
The revive --id argument contract: adopt uses the same session-id parser. A malformed id is a usage exit 2 before anything is read.
@task_item n=3 part=touches
The rows on reading the claude agents list (bounded, fail open, resolved from the caller's PATH) and the agents-probe log rows (content-free, redacted): adopt makes that read for its live-elsewhere check. A failed read lets it start (the W1 no-signal arm), and the probe line holds no id.
@task_item n=4 part=touches
The revive refusal of a session a live claude holds, and contract revive-json session-live: adopt now carries the same refusal, with its own fixed pair and a session-live document under --json. It also refuses the id that a live adopted instance holds.
@task_item n=5 part=touches
The named lists of starts checked for strict modes: the strict-modes refusal contract for run and revive, start checks the home tree before its log opens, home tree checked before any log opens, strict-modes check at start, a strict-modes refusal leaves no trace. Adopt's start joins each of them and is refused before any log, with the mode left as it was found.
@task_item n=6 part=touches
The socket-directory refusal for run and the start of revive, and the rows that check the socket directory before any bind, snapshot or child: adopt's start goes through run's start and carries socket-dir-refused.
@task_item n=7 part=touches
The start collision-check contracts already-live and stale-heartbeat, and the row that run's collision check is not server-verified: adopt runs the same holder check and prints run's collision pair.
@task_item n=8 part=touches
The lineage rows: self-reported parent authenticates nothing, lineage kept to the snapshot and the event log, viola run's lineage flags checked before anything is read. Adopt takes --parent, otherwise the inherited VIOLA_NAME unless it equals the name. A --parent equal to --name is refused as a usage exit 2.
@task_item n=9 part=touches
The launch-pair rows (flags checked before anything is read, refused after the dashes, handed on the claude child's argv, kept to the snapshot and the event log, authenticates nothing): adopt takes --model and --effort through run's own parsers and accepts no words after the dashes.
@task_item n=10 part=touches
The recorded-cwd rows (cwd kept to the snapshot, spawn directory in the snapshot only, recorded cwd used only to spawn): adopt spawns in the current directory and writes that as the snapshot cwd.
@task_item n=11 part=touches
The every-product-error-writer-named census and the fixed error display messages: three error bodies for adopt's writers were added to the census.
@task_item n=12 part=touches
The catch-site internal-error output under --json, and the CLI internal-error document: an anyhow error in adopt before the pump prints the internal-error document, and adopt is silent at the catch site.
@task_item n=13 part=touches
The pinned-binary rows (pinned viola re-hashed before use, pinned folder first on the child's PATH, run resolves its child program itself) and the pinned-hash-mismatch and squatted-name codes: adopt's start goes through the same checked exe and run's start.
@task_item n=14 part=touches
The CLI input rows (CLI input trust boundary, CLI names through the viola name check, names through the validated name type): adopt's --name and --parent go through the name parser.
@task_item n=15 part=touches
Refusals hold no paths, pids or ids, and no identity values in verification output: adopt's session-live pair and its documents hold no id, path or pid.
@task_item n=16 part=touches
The rows that say unwrapped or claude agents names are not targets: adopt brings an unwrapped plain session under viola, under a name the operator gives. It never takes a name from the agents list.
@task_item n=17 part=touches
Child spawn rows (direct spawn, parent identity variables and CLAUDE credentials stripped from the child): adopt's child is launched by run's own start, so the same rules apply to an adopted child. It acts on no other process.
@task_item n=18 part=touches
The live-round rig rows (rig keys only into its own pty, live rig child isolated): the R5 adopt script is a new live-reading rig that ends only its own recorded wrapper and its own script host.
@task_fact n=1 slug=input-validation
fact: viola adopt takes an operator-typed session id only in the closed session-id shape (ASCII hex, either case), passed as one argv element after --resume on a direct spawn. A malformed id is a usage exit 2 before anything is read. PROVISIONAL on the delegate's word (W2).
@task_fact n=2 slug=authentication-authorization
fact: viola adopt refuses a session that a live claude holds, including an id that a live adopted instance holds. The refusal is session-live, exit 1, one fixed pair, no child started. PROVISIONAL on the delegate's word (W1, W4 wording a planning form).
@task_fact n=3 slug=input-validation
fact: if the claude agents read for adopt's live-elsewhere check fails, adopt starts anyway (the no-signal arm fails open). PROVISIONAL on the delegate's word (W1).
@task_fact n=4 slug=authentication-authorization
fact: adopt's start checks in this order: instance strict-modes, pinned exe, wrapper log open, holder collision check, live-elsewhere check, then run's start. A refused holder makes no agents read. A widened home or instance dir is refused before any log, with the mode left as it was found.
@task_fact n=5 slug=input-validation
fact: adopt's parent is --parent, otherwise the inherited VIOLA_NAME unless that equals the name. A --parent equal to --name is a usage exit 2. No words are accepted after the dashes.
@task_fact n=6 slug=error-handling
fact: adopt's --json refusal codes are strict-modes-failed and session-live, plus run's shared already-live, stale-heartbeat, pinned-hash-mismatch, squatted-name, socket-dir-refused and internal-error. Adopt logs as process run and is silent at the internal-error catch site.
@task_fact n=7 slug=authentication-authorization
fact: the R5 adopt live-reading script ends only the wrapper it recorded, while that process's exe is the given viola, and its own script host: TERM, then 10 s, then KILL only if the start time is unchanged. It keeps verdict words only and adds no time bound to R5b's wait.
@task_item n=1 part=untouched
Dependency security: no crate, dependency, feature, pin, toolchain, advisory or workflow changed.
@task_item n=2 part=untouched
API security and the MCP rows: no MCP tool, MCP input, rmcp feature or MCP stdin cap changed.
@task_item n=3 part=untouched
The wrapper channel methods, the channel frames, the channel error codes and the peer euid or server-verify checks: adopt adds no IPC method or frame.
@task_item n=4 part=untouched
The hook contract and hook refusals: no hook payload, hook event or hook decision changed.
@task_item n=5 part=untouched
Paste, send and answer text sets, the dialog free-text rules and the typed-text refusals: adopt types no text.
@task_item n=6 part=untouched
The absences (network, hosting, database, secrets, accounts, remote access, webhooks, release stage): none is touched.
@task_item n=7 part=untouched
Config keys and the rule that config cannot disable controls: no config key or env read was added.
@task_item n=8 part=untouched
The window rows for focus, open, up and the Hyprland recorder: adopt opens no window.
@task_item n=9 part=untouched
The plugin install, marketplace and Claude-home write rows: adopt writes nothing there.
@task_item n=10 part=untouched
CI artifact canary scan, fixture scrub and fixture hygiene rows: no fixture or CI upload changed.
