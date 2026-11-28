@task_prose part=chunk
Builds the fleet console's scaffold: its own surface section in the masters (beside Surface: cli, with the cli / tui landmarks), replacing the 0.1.0 stub deferred to 0.2.0, and the minimal code behind it.
Fixes the console's quarter-screen layout and smallest whole size, and a frame that stays whole there and degrades by a named rule below it.
Reconciles the redraw bans with an interactive console: an exception named for the console's own surface only, never generalised (provisional, the operator's fork).
Adds a test-only keystroke driver feeding the console keys through its own pseudo-terminal under a test-owned home, asserting on the console's state or log lines, never screen bytes; and the console's own diagnostics role reachable through the logs command.
Folds two carries: re-reads the chrono advisory RUSTSEC-2026-0543 and its ignore; restructures the per-paste probe step and the relied-row count / stamped-set table to one source.
@task_file n=1 path=deny.toml mark=edits
what: the dependency advisory policy, its ignore list and the reason line that says the ignore is re-read at each epoch boundary — edits: the RUSTSEC-2026-0543 ignore entry and its reason line
@task_item n=1 part=needs
The console's surface section: a new interactive terminal surface written beside the cli and tui surfaces, with its own structural landmark equivalents; rows owed to the fleet view (columns) and console actions (keys) named as owed, not built.
@task_item n=2 part=needs
Redraw-ban reconciliation: the rows banning spinners, in-place rewrites and cursor-moving status and requiring linear static output reach the console only through an exception named for the console's own surface; the ban stands unchanged for every CLI verb.
@task_item n=3 part=needs
The console as keyboard-interactive: the rules that the CLI takes no interactive input and is not keyboard-interactive get a carve-out for the console surface alone; the scaffold's own keys are quit and whatever the frame needs, no verb.
@task_item n=4 part=needs
The keystroke test driver asserts on the console's state (a structured read exposed for tests) or its log lines, never on screen bytes; the ban on screen-content parsing binds its assertions.
@task_item n=5 part=needs
The driver owns its pseudo-terminal under a test-owned home, over the fake agent and fixtures as the contract; no real terminal, no real claude, nothing of the operator's real home; each console test bounded under the 20 s lint.
@task_item n=6 part=needs
Which pseudo-terminal library drives the console in tests, against the pinned outer-PTY boundary driver; any new dependency that draws or reads a terminal is the operator's fork.
@task_item n=7 part=needs
Quarter-screen layout: the frame stays whole at the smallest size the layout fixes and degrades below it by a named rule, the way NAME truncation on a narrow terminal is ruled.
@task_item n=8 part=needs
The frame's own bytes: control characters escaped in anything it prints, no SGR without a colour-capable terminal, any colour only a second cue beside its word, failure words never coloured.
@task_item n=9 part=needs
The console's diagnostics role: structured logs under its own role beside run, hook, mcp and ui, reachable through the logs command, never written to the console's terminal; log identity tags (process, service_name) carry it; the obs event catalog's rows for it.
@task_item n=10 part=needs
The console's start verb or flag and its name (the operator's fork): if a verb or flag is added, the help listing of served verbs, its usage line and group labels reach it.
@task_item n=11 part=needs
Any fixed string the scaffold prints (frame caption, quit hint) comes from design copy and is pinned by literal; a refusal it raises follows the unable line, reason code and hint layout.
@task_item n=12 part=needs
Time bounds never fall on the human at the console: the scaffold imposes no time limit on the user; bounds fall on the test driver only.
@task_item n=13 part=needs
The console's a11y checks run as literal / state assertions inside the existing suites, failing as nextest failures, with no skips and no retries, under the GitHub Actions gate.
@task_item n=14 part=needs
Carry cry-h6vzhlce: the chrono unmaintained advisory re-read; an upgrade found, the ignore leaves; none, the ignore stands one more epoch on the operator's word, its reason line still saying it is re-read at each boundary.
@task_item n=15 part=needs
Restructure invariant for carry cry-boujwuyr: framing_turns' per-paste probes moved into one step function and send_through's guarded section looked at, the relied-row count taken from one source and the stamped-set table held once; what each does stays as it is, held by the existing tests over them.
@task_item n=16 part=needs
No crate without a real consumer and a test; every console layer agent-runnable headless over the fake agent and fixtures.
@task_standing n=1
convention: each new test fails as a nextest failure inside the existing suites and coverage run, no skips, no retries, zero flakiness
home: 9-ci-integration
@task_standing n=2
convention: each new log line goes to the diagnostics log under its role with its identity tags, never to a terminal
home: a11y-scope-summary
@task_standing n=3
convention: each new fixed CLI string is taken from design copy and pinned by a literal test
home: 8-cognitive-accessibility · cognitive-accessibility
@task_item n=1 part=not_needed
The output contracts of individual verbs (the list board and its caption, revive, link and unlink, wait, verify, switch, up, plugin install): the scaffold prints none of them and changes none.
@task_item n=2 part=not_needed
The socket and IPC refusal contracts (frame-verb refusals, hook calls, MCP frame tools, instance-unreachable details, start-against-holder refusals): the scaffold opens no instance channel.
@task_item n=3 part=not_needed
The critical paths for send readback, dialog answer and taking the wheel, and the send-refused-while-typing rule: these belong to the console actions entry.
@task_item n=4 part=not_needed
The board colour rows (amber on DIALOG, bold NAME, dim stale rows): the fleet rows inside the frame are the fleet console view's.
@task_item n=5 part=not_needed
The compositor time bounds (each compositor read, window step wait, live open wait, respawn reap): the chunk opens or moves no window and dispatches nothing to the compositor.
@task_item n=6 part=not_needed
The manual Orca and manual keyboard passes and their screen-reader targets: the SC checks over the finished console are the fleet a11y verification entry's.
@task_item n=7 part=not_needed
The absences of a web or graphical frontend, a touch surface, a rendered page and internationalisation: the console is a terminal surface and touches none of them.
@task_item n=8 part=not_needed
The viola run TUI passthrough contracts (human keystrokes, focus/mouse/resize sequences, vendor content, vt100 feed panic): the console is its own process, not the wrapped claude terminal.
