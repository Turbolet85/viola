@task_prose part=chunk
Builds the fleet console's scaffold: its own Surface section beside Surface: cli, replacing the 0.1.0 stub that deferred the console to 0.2.0, with the minimal code behind it.
Fixes the console's quarter-screen layout and the smallest size its frame stays whole at, and a named degrade rule below that size.
Reconciles the CLI's redraw bans with an interactive console through an exception named for the console's surface only.
Adds a test-only keystroke driver that asserts on the console's state, never on screen bytes, and gives the console its own diagnostics role.
Folds two carries: re-reads the chrono advisory ignore at the epoch boundary, and the code audit's M1 (one step function for the per-paste probes) and M3 (the relied-row count from one source, the stamped-set table held once).
@task_file n=1 path=deny.toml mark=edits
what: the dependency audit policy, which holds the ignored advisories and their reasons — edits: the RUSTSEC-2026-0543 chrono ignore entry and its reason line
@task_item n=1 part=needs
The console's own Surface section, written beside Surface: cli where the 0.1.0 stub said the console was deferred to 0.2.0 and its technology not chosen. The section names the rows it leaves to Fleet console view (columns, lineage, state words) and to Console actions (verbs, keys) as owed, never as built.
@task_item n=2 part=needs
The quarter-screen layout: the console frame stays whole at a quarter of the screen and at a named smallest size, and below that size it degrades by a named rule. The tiled window and the fleet rows inside the frame are not part of this chunk.
@task_item n=3 part=needs
The redraw bans on the CLI's surfaces (no spinner, no progress glyphs, a single progress pattern, send lines appended and never redrawn, no persistent footer). Which of them reach the console, and how, is settled by an exception named for the console's own surface only. The rule is never generalised, and every other CLI surface keeps the ban (operator fork, conservative arm provisional).
@task_item n=4 part=needs
The other presentation rules of the CLI surface that an interactive console meets: no interactive prompt or pager, no banner, words carry every state, amber means dialog only. The console's section states whether each one reaches the console. Only the redraw ban receives a named exception.
@task_item n=5 part=needs
The console's start verb or flag and its name. This is the operator's fork, taken provisionally on the conservative arm. A new verb or flag joins the CLI verbs boundary and the viola --help verb groups, and it takes the --home global flag like every other verb.
@task_item n=6 part=needs
How the console exits and fails. Quitting from the scaffold's own key, and an internal error at start or while drawing, map onto the CLI exit codes and the CLI internal-error human and --json forms. Results go on stdout and the rest on stderr.
@task_item n=7 part=needs
How the console detects and styles the terminal. TTY detection goes through std IsTerminal, and colour comes from the hand-written SGR styling module and the CLI token rendering, unless a terminal-drawing dependency is taken. Any such new dependency is the operator's fork, taken provisionally on the conservative arm.
@task_item n=8 part=needs
A structured state read the console exposes for tests (or its log lines). The keystroke driver asserts on that state, never on screen bytes.
@task_item n=9 part=needs
The keystroke test driver. It feeds keys through its own pseudo-terminal under a test-owned home, driving only the scaffold's keys (quit and what the frame needs) against the fake agent and fixtures as the contract. It drives no fleet view and no verb.
@task_item n=10 part=needs
The console's own diagnostics role, beside run, hook, mcp and ui. Its structured logs are reachable through the logs command, and its rows join the obs event catalog. The list of roles and its count change wherever a row or a test pins them.
@task_item n=11 part=needs
The phase's bounds. There is no live start, no window opened or moved, nothing dispatched to the compositor, no key sent to a real terminal, no real claude run, and nothing read from the operator's real home. Every console test runs under a test-owned home and its own pseudo-terminal, bounded under the 20 s lint.
@task_item n=12 part=needs
Infra before features and headless agent verification first. No new crate is added without a real consumer and a test, and every layer can be run by an agent over the fake agent and fixtures.
@task_item n=13 part=needs
Carry M1, a restructure of the code audit's own code. The per-paste probes of the turn framing are moved into one step function, and the send path's guarded section is looked at. The probes and the send path behave as before, held by their existing tests.
@task_item n=14 part=needs
Carry M3. The relied-row count is taken from one source instead of literals moved by hand at every new row, and the stamped-set table is held once.
@task_item n=15 part=needs
Carry on the chrono advisory (unmaintained). It is re-read at the epoch boundary. If a safe upgrade exists, the ignore leaves with it. If none exists, the ignore stands one more epoch on the operator's word, and its reason line keeps saying it is re-read at each boundary.
@task_standing n=1
convention: each new verb or flag owes its per-verb --help usage, its row in the viola --help verb groups, and the --home global flag
home: surface-cli
@task_standing n=2
convention: each new refusal owes the shared refusal line form and word column, with a hint for its cause as the last stderr line, naming the next verb and never quoting text or naming paths
home: surface-cli
@task_standing n=3
convention: each new line of human output is a contract, pinned by a test in both TTY and plain output modes
home: surface-cli
@task_item n=1 part=not_needed
The web surface's absence. The console is a terminal surface, so viola still renders no page.
@task_item n=2 part=not_needed
The viola list board and its --json document: columns, tree, stale rows, name truncation, links block, unknown-cell words. The fleet rows inside the console belong to the Fleet console view, not the scaffold.
@task_item n=3 part=not_needed
The output, refusal and --json contracts of viola send, wait, last, answer and switch, the readback mirror, and the wheel verb output. Every verb in the console belongs to Console actions.
@task_item n=4 part=not_needed
The start and refusal contracts of viola run and viola revive, their --list rows, and their --json error codes. The scaffold starts no session.
@task_item n=5 part=not_needed
The output and refusal contracts of viola verify. The chunk runs no live verify and changes none.
@task_item n=6 part=not_needed
The viola mcp tools, their results, and the rule that MCP hides verbs. The console is not an MCP surface.
@task_item n=7 part=not_needed
The output of viola link and unlink, viola plugin install, viola adopt, viola up, and viola open's parent default. The scaffold reaches none of these verbs.
@task_item n=8 part=not_needed
The exit-21 instance-unreachable details and the frame verbs' hints. The scaffold reaches no instance socket.
