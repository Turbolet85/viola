@task_prose part=chunk
The first live round on the product with the budget gate and the status-line interception removed: test-only rigs drive the real claude headless on a plain pty the rig opens itself, each rehearsed first under the fake agent, then named operator steps fire live only on the operator's word.
It proves one live wrapped start of the unmodified claude on Linux (child screen with no viola byte on the pty, human keys passed unchanged, start order with no budget step, a live or stale name refused, the child's end detected, ledger green).
It reads a live dialog hold near its 3600 s ceiling so the PROVISIONAL mark is settled by the founder's word; the hold value is read, not changed, unless that word moves it.
It re-records the MCP list tool fixture without the usage-budget wording, records a box-only screen fixture by a clean verify --record for the fake agent to replay in CI, and reads a wrapped session's status line beside an unwrapped one's, writing nothing to the user's settings.
@task_file n=1 path=fixtures/claude/2.1.302/Mcp.tools-list-1.json mark=edits
what: a recorded CLI fixture of the MCP list tool's description, replayed by the fake agent in CI — edits: the list tool's description, re-recorded live through the MCP tap
@task_file n=2 path=scripts/agent-run.sh
what: the live-round rig script that drives a session on a plain pty for the live and fake-agent runs
@task_item n=1 part=needs
The TUI passthrough boundary under viola run: zero viola bytes on the terminal while the child runs; the live wrapped start must read the child's screen with no byte of viola's own on the rig's pty.
@task_item n=2 part=needs
Human keystrokes under viola run pass to the child unchanged and are never blocked; the live start must show the human's keys reaching the child as typed.
@task_item n=3 part=needs
The outer-pty boundary driver: the rig opens a plain pty itself and drives the session headless, with no terminal window, compositor or desktop step; every spawn has a kill deadline.
@task_item n=4 part=needs
Live readings never gate: the live wrapped start, the dialog-hold reading and the status-line reading are evidence records per operator step, rehearsed under the fake agent; a gated claim rests on the operator's own word, never a CI gate on a live start.
@task_item n=5 part=needs
The dialog hold and its bound: config.json's dialog_hold_s and the DialogHold ceiling (60 to 3600 s) are read live near the ceiling; the hold bounds the driver and its time bound falls on the driver, never on the human.
@task_item n=6 part=needs
The wrapped TUI reads as unwrapped: the status-line reading compares a wrapped session's status line beside an unwrapped one's on a real claude, as live evidence for that claim, reading the user's own setting and writing nothing.
@task_item n=7 part=needs
Boundary checks never inspect claude content and parse the screen only by signatures: the box-only screen fixture holds the input box's rule and prompt rows with no footer literal, recorded, never hand-written, and its replay matches on those signatures alone.
@task_item n=8 part=needs
Start refusals of a live or stale name: a wrapped start against a holder is refused already-live or stale-heartbeat, read live in the round.
@task_item n=9 part=needs
viola verify output on a test home: its step lines and the capability ledger stamps for the CLI the round runs, reading green for the account; verify --record and its scrub produce the new fixtures.
@task_item n=10 part=needs
Any new rehearsal test the rigs add must pass in CI with no skip and no retry; a live step is never a skipped CI test.
@task_item n=11 part=needs
A red or defect a live reading exposes on a boundary or output contract folds into this chunk as its own item with its own acceptance; otherwise no product behaviour changes.
@task_standing n=1
convention: each new rehearsal or boundary test runs inside the existing test suites, with no separate a11y driver, suite or artifact of its own
home: 1-a11y-scope-summary · 9-ci-integration
@task_standing n=2
convention: each new test's failure is a nextest failure that fails the build, with zero flakiness and no retries
home: bootstrap-phases-derive-for-route-setup-project · 10-slo-invariants-a11y-budgets
@task_item n=1 part=not_needed
Visual design rows on the list board — colour, bold on names, amber on the dialog word, dim on stale rows, name truncation, captions and order: the round prints no board and changes no colour.
@task_item n=2 part=not_needed
The link and unlink contracts, the plugin install output and the revive --list and revive --json refusal documents: the round neither links, installs by the CLI verb nor revives.
@task_item n=3 part=not_needed
The help output's usage line, group labels and served verbs, and the fixed CLI copy and refusal layout literals: the round adds no CLI string and changes no help.
@task_item n=4 part=not_needed
Hook call, frame verb and MCP frame tool refusals over socket-directory, strict-modes and server-verify failures, and the instance-unreachable details: no live step exercises those refusal paths.
@task_item n=5 part=not_needed
The absences of a web or graphical frontend, a rendered page, a touch surface and internationalization, and the browser a11y leg: nothing of the round reaches them.
@task_item n=6 part=not_needed
The send refusals, the wait output and the link line's time: the round sends no prompt through viola send and waits on no instance by the CLI.
