@task_prose part=session
Added the CLI verb viola adopt: a plain claude session, named by its session id, resumed under viola with an operator-given name, optional parent and optional model and effort pair, with --json.
adopt refuses a session that a live claude elsewhere holds (closed detail session-live, the fixed pair unable: that session is open in another claude / hint: close it there first, then adopt it again, exit 1) and a home or instance dir another user could write (strict-modes-failed, the instance-state pair); it shares run's already-live, stale-heartbeat, pinned-hash-mismatch, squatted-name, socket-dir-refused and internal-error refusals.
A --parent equal to --name is a usage exit 2 with the fixed line run uses; a malformed id is a usage exit 2 before anything is read; the session-id shape takes hex of either case.
The help's setup: row grows from 6 to 7 verbs (adopt after revive); adopt is a silent role at the internal-error catch site and, under --json, an error before the child holds the terminal prints the internal-error document.
No new log line, event kind, record field, closed word, colour, column or glyph; adopt logs through run's own lines; the session-live hint wording is a planning form and W1 to W4 stand PROVISIONAL on the delegate's word; matrix v2-13 stays GATED on R5.
@task_file n=1 path=src/cmd/adopt.rs
what: the viola adopt verb: its arguments, its start refusals and their fixed pairs and --json codes, its refusal order and the lineage of the adopted start
@task_file n=2 path=src/cmd/mod.rs
what: the CLI verb set and its dispatch, and the viola --help verb groups (the setup: row)
@task_file n=3 path=src/main.rs
what: the CLI catch site: which verbs stay silent at the internal-error catch and which print the --json internal-error document
@task_file n=4 path=src/cmd/revive.rs
what: the viola revive verb: its session-id parser and its live-elsewhere (session-live) check, now shared with adopt, behaviour unchanged
@task_file n=5 path=tests/cli_adopt.rs
what: the integration checks of viola adopt: resume under name and parent, its own child in its own terminal, the session-live, strict-modes, collision and internal-error refusals and the usage exits
@task_file n=6 path=tests/cli_output_plain.rs
what: the plain CLI output check: the --help text and the list of served verbs
@task_file n=7 path=tests/cross_exit_causes.rs
what: the cross-verb check of exit causes and their hints, one row per refusal cause
@task_file n=8 path=tests/contract_error_bodies.rs
what: the census check of every fixed refusal and error body each verb writes
@task_file n=9 path=tests/security_negatives_env_capture.rs
what: the security negatives over captured environment, including the verify-window cases and their kill bound
@task_file n=10 path=tests/support/home.rs
what: the test harness home and wrapper support, now able to start an adopted wrapper
@task_file n=11 path=scripts/r5-adopt.sh
what: the test-only live-reading script for the gated R5 reading of adopt over a real claude, with a rehearsal over the fake agent
@task_item n=1 part=touches
The viola --help verb groups contracts (help-output verb-groups and viola-help verb-groups): the setup: row now reads open, run, revive, adopt, up, verify, plugin install, 7 entries where it held 6; any row listing the setup verbs or their count is stale.
@task_item n=2 part=touches
The command names (name/command/*): the name adopt is now taken by a served CLI verb and no name row holds it yet.
@task_item n=3 part=touches
Flat CLI verbs: adopt is one more flat top-level verb beside run and revive, no subcommand.
@task_item n=4 part=touches
The start refusal rows scoped to run and revive (run and revive start refusals fixed message, viola run start refusals already-live, stale-heartbeat, pinned-hash-mismatch, squatted-name, socket directory, home-tree check): adopt's start now shares each of them; rows that name only run and revive as the starts are incomplete.
@task_item n=5 part=touches
The revive --json refusal code rows and viola revive --json exit-1 codes: they stay true for revive; adopt brings its own list beside them (strict-modes-failed, session-live and run's shared already-live, stale-heartbeat, pinned-hash-mismatch, squatted-name, socket-dir-refused, internal-error), with no cwd-missing or no-session.
@task_item n=6 part=touches
viola revive refusals session-live: the live-elsewhere check it describes is now shared with adopt, which refuses with the same closed detail but its own hint line telling to adopt it again.
@task_item n=7 part=touches
The silent catch sites decision and the CLI catch site silent-verbs and --json internal-error contracts: adopt joins the silent roles, and under --json an error before the child holds the terminal prints the internal-error document.
@task_item n=8 part=touches
No document once the child held the terminal: adopt follows the same rule, no --json document after its child started.
@task_item n=9 part=touches
The CLI strict-modes refusal instance-state pair, the fixed refusal pairs instance-state and home-state, and the instance strict-modes refusal rows: adopt's start now uses them, refused before any log, mode left as found.
@task_item n=10 part=touches
The CLI exit codes 1 and 2: adopt's refusals exit 1; a malformed id, a --parent equal to --name or words after -- are a usage exit 2.
@task_item n=11 part=touches
viola run and revive output child started, and the run and revive automatic-verify line rows: these name run and revive as the starts; adopt's start goes through run's own start path and may belong beside them.
@task_item n=12 part=touches
Start refusals show no path or pid, and no internals in errors or hints: adopt's refusal pairs hold no id, path or pid.
@task_item n=13 part=touches
viola run silent while wrapped: adopt hosts its child like run and prints through run's lines.
@task_item n=14 part=touches
The CLI refusal line fixed unable form, its hint, and the stderr refusal and hint line contracts: adopt's session-live pair is a new fixed unable and hint pair, its wording a planning form (W4 PROVISIONAL).
@task_fact n=1 slug=surface-cli
fact: adopt is a served CLI command name, viola adopt SESSION_ID --name NAME with optional --parent, --model, --effort and --json, listed in the help's setup: row after revive.
@task_fact n=2 slug=surface-cli
fact: viola adopt --json exit-1 codes are strict-modes-failed, session-live, already-live, stale-heartbeat, pinned-hash-mismatch, squatted-name, socket-dir-refused and internal-error.
@task_fact n=3 slug=surface-cli
fact: viola adopt refuses a session open in another claude with session-live, exit 1, the fixed pair unable: that session is open in another claude / hint: close it there first, then adopt it again, and starts no child (wording a planning form, PROVISIONAL on the delegate's word, W4).
@task_fact n=4 slug=surface-cli
fact: viola adopt refusal order is the home-tree strict-modes check, the pinned executable, the wrapper log, the name holder, then the live-elsewhere check; the first refusal wins and a refused holder takes no agents read; a failed agents read still starts.
@task_fact n=5 slug=surface-cli
fact: viola adopt takes the session id only in the closed session-id shape, ASCII hex of either case; a malformed id, or a --parent equal to --name with the fixed line run uses, is a usage exit 2 before anything is read.
@task_fact n=6 slug=surface-cli
fact: an adopted start's parent is --parent, else the inherited VIOLA_NAME unless it equals the name; it spawns in the current folder and logs as process run through run's own lines.
@task_fact n=7 slug=surface
fact: adopt is a silent role at the internal-error catch site; under --json an error before its child holds the terminal prints the internal-error document.
@task_item n=1 part=untouched
Brand identity: strips, readback box, cocked dialog-pending strip, expression level and phraseology; adopt adds no display element.
@task_item n=2 part=untouched
Colour palette, colour tokens, colour depth and colour decision order: no colour or SGR change.
@task_item n=3 part=untouched
Typography, spacing, iconography and glyphs: no new column, separator, arrow or glyph.
@task_item n=4 part=untouched
The viola list board and table rows: columns, widths, skip counts, row escaping and the unwrapped row are unchanged.
@task_item n=5 part=untouched
Send readback, the viola send mirror, viola wait and viola last outputs: unchanged.
@task_item n=6 part=untouched
viola link and unlink, viola focus, viola up and viola verify outputs and refusals: unchanged.
@task_item n=7 part=untouched
Wrapper fault --json detail, peer reply screening, the wrapper socket and placement words: no new IPC method.
@task_item n=8 part=untouched
The web-spa surface absence, the rendered page absence and the self-validation protocol: no page.
@task_item n=9 part=untouched
Anti-patterns: no spinner, prompt, pager or status dot added; adopt asks no interactive question.
