@task_prose part=chunk
Adds a viola verb that switches a running viola-wrapped session, a named instance, to a given model and effort.
The verb records the new pair, so a later resume through revive or up replays the new pair and not the one named at launch.
The switch has no side effect on the user's own settings and does not go through the CLI's own model command.
It adds the verb's argument surface, closed words, exit codes, hints, output lines, --json document, events-log record and process-log lines beside the existing tables.
@task_file n=1 path=src/cmd/revive.rs
what: the viola revive verb — the restart path whose --model / --effort override the recorded pair on the resumed child's argv (given_pair), which a restart-based switch could ride
@task_file n=2 path=crates/viola-agent-claude/src/lib.rs
what: the claude agent's closed effort set (low, medium, high, xhigh, max) and the compiled default launch pair DEFAULT_PAIR
@task_item n=1 part=needs
The new verb's row in the CLI verbs table: a flat verb plus a target, the target always a wrapped ViolaName that is never case-transformed, with the argument order and usage of its model and effort arguments.
@task_item n=2 part=needs
The closed words the verb accepts: model words from the model-word set and effort levels from the effort set, with a refusal for a word outside them.
@task_item n=3 part=needs
The recorded launch pair (the launch line and the snapshot's launch) as the record a later revive or up replays. After a switch, a resume must replay the new pair, which interacts with how revive's --model / --effort override the recorded pair and with the run and revive usage that carries a launch pair.
@task_item n=4 part=needs
No side effect on the user's settings: the verb writes nothing to ~/.claude* or to a project's .claude/settings*, and it never uses the CLI's own model command, which writes the user-scope model key, as its channel.
@task_item n=5 part=needs
The verb's exit codes and refusal causes, placed beside the exit-cause catalogue in the CLI exit codes table, including their --json form.
@task_item n=6 part=needs
A hint for every new refusal cause: the hint is the last stderr line, names the next verb, never quotes text or names paths, and never names viola release.
@task_item n=7 part=needs
The human output lines of the switch as a contract: results go on stdout and everything else on stderr, in the shared refusal line form and word column, using the spoken phraseology of the wheel and handoff verbs where it applies, with no spinner, banner, progress glyph, prompt or pager.
@task_item n=8 part=needs
The verb's --json document: one document per call, typed for success and for refusal, following the rule that every data verb takes --json.
@task_item n=9 part=needs
Reaching the instance: the exit-21 instance-unreachable refusal for a target with no instance, an instance that is not running, a lost wrapper or a refused socket directory, together with its --json detail words, its frame-verb hints and the strict-modes refusal.
@task_item n=10 part=needs
A restart-based switch ends and resumes the session, so it passes through revive's start preflight: the preflight order, the session-live check, and the already-live, session-open-elsewhere and directory-missing refusals with their --json codes.
@task_item n=11 part=needs
A switch of a session that is mid-turn or holding a dialog, and whose wheel the switch needs (the wheel verbs and the send gate), stay unmeasured until P3 reads them.
@task_item n=12 part=needs
The MCP surface: on the conservative arm the switch is CLI-only and no MCP tool is added unless a fork settles otherwise. This touches the MCP tools list and the rule that MCP hides wheel and handoff verbs.
@task_item n=13 part=needs
The verb's place in viola --help (its verb group row and the usage line) and its own per-verb --help usage.
@task_item n=14 part=needs
The events-log record of the switch, as a new kind beside events.md's kinds, and the verb's process-log lines, beside obs-plan's closed details.
@task_item n=15 part=needs
The rule that the CLI holds every action: the console's keyboard reach of the switch belongs to Console actions, but the CLI verb is what that reach calls.
@task_standing n=1
convention: every new refusal cause owes an exit code row, a --json detail word, and a hint that is the last stderr line and names the next verb
home: surface-cli
@task_standing n=2
convention: every new verb owes its CLI verbs row, its --help usage, human output on the stdout/stderr split, and a single --json document
home: surface-cli
@task_item n=1 part=not_needed
The viola list board and its BAY line (columns, ordering, truncation, colour depth, the links block, the stale and unknown cells). The chunk adds no board row and changes no board column.
@task_item n=2 part=not_needed
viola verify output and refusals: the chunk does not touch the recorded-fixture verification.
@task_item n=3 part=not_needed
viola plugin install output and refusals: the switch installs nothing.
@task_item n=4 part=not_needed
viola link and unlink output: the switch makes and removes no links.
@task_item n=5 part=not_needed
viola wait, viola last and viola answer output: the chunk does not change how a turn is waited on, how a last message is read back, or how a dialog is answered.
@task_item n=6 part=not_needed
viola open's --parent default and the window placement of open: this phase opens no window.
