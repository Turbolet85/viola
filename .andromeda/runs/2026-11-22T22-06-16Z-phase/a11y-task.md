@task_prose part=chunk
Builds the lineage registry: a wrapped session's declared parent and role are recorded once, when the wrapper starts, as a new `lineage` event in that instance's own event log, and read back the same after a crash and a replay.
A parent is declared at launch through `viola run` (a flag or the launching session's identity); what a starting wrapper can see of its parent is measured first at P3.
`viola list` shows the fleet as trees in the board's six fixed captions, plain when its stdout is not a terminal; what `list --json` gains is an operator fork.
A reused name never inherits an old parent; `revive` keeps the parent the lineage recorded; a parent edge is a drawing label only, never moving the wheel, the links or whether a send is accepted.
Observed links (0.1.0's session links) are drawn apart from parent edges on every view showing both, a link name shown only after it parses as a viola name.
@task_file n=1 path=src/run/ mark=edits
what: the `viola run` start path, where the wrapper starts and the launch-time parent declaration is taken — edits: the start path, where the lineage line is written
@task_file n=2 path=src/cmd/revive* mark=edits
what: the `viola revive` verb and its start — edits: the parent carried across a revive
@task_file n=3 path=src/human/board/ mark=edits
what: the human `viola list` board drawing, its captions, rows and piped form — edits: the tree drawing, the piped form
@task_item n=1 part=needs
The new `lineage` event kind joins the event kinds in the core crate only, never Claude-named, and its line follows the event line contract; it is written once by the wrapper as the one writer of that instance's event log.
@task_item n=2 part=needs
The `viola list` board keeps its six captions, their words and slot order fixed by literal; the fleet trees are drawn inside those fixed columns and the caption order and board structure stay as pinned.
@task_item n=3 part=needs
The tree form of `viola list` is static linear output: no spinner, no cursor movement, a parent-child relation readable from printed text and order, not from layout or colour alone.
@task_item n=4 part=needs
Plain when piped: with stdout not a colour-capable terminal the tree and link views carry no SGR at all; on a terminal any SGR sits only beside its printed word (bold only on NAME values, dim only on stale rows, amber only on the DIALOG word), colour only a second cue.
@task_item n=5 part=needs
The board's empty-board line, the unwrapped separator `-- UNWRAPPED - READ-ONLY --` and NAME truncation rules hold with the trees drawn; unwrapped sessions carry no lineage.
@task_item n=6 part=needs
Observed links shown apart from parent edges: a link name is printed only after it parses as a viola name, printed names being the parsed arguments, and any C0/C1 control bytes in a printed name or role are escaped.
@task_item n=7 part=needs
The 0.1.0 `link` / `unlink` verbs keep their result line, `--json` output and exit 20 / 21 contracts; link pairs stay validated by the wrapper, and a parent edge is never a link.
@task_item n=8 part=needs
A parent edge never changes whether a `send` is accepted: the send refusals (input-not-ready, no-prompt-submitted) and the wheel rules stay unchanged by lineage.
@task_item n=9 part=needs
The launch-time parent declaration on `viola run`: any new flag shows in `viola --help` under its group label with the usage line, and any refusal it adds (an undeclarable or unparsable parent) is an operator fork that, if taken, follows the refusal layout: unable line with a kebab-case code word, hint line, exit code, one clean `--json` document with no hint, no stack trace.
@task_item n=10 part=needs
Writing the lineage line at launch adds zero viola bytes to the TUI passthrough terminal; run diagnostics of the lineage write never reach the terminal, and human keystrokes are never blocked by it.
@task_item n=11 part=needs
`viola revive` keeps the recorded parent: the revive human and `--json` refusals and `revive --list` rows and `--json` keep their contracts; any parent shown on `revive --list` follows the same name-check and escaping.
@task_item n=12 part=needs
`list --json` payload: any lineage field it gains is an operator fork; a `--json` document stays one clean document with no SGR and no hint.
@task_item n=13 part=needs
The log identity tags (process, service_name) on any new log line of the lineage write and read-back.
@task_item n=14 part=needs
Time bounds never fall on the human: any bound on reading a parent at launch falls on the driver, never a harsh timeout on the user.
@task_item n=15 part=needs
Proof without a live start: the measurements and checks come from tests or no-start probes; live readings never gate, and the boundary tests are sequenced on records, not on screen content.
@task_standing n=1
convention: every new a11y assertion runs inside the existing test suites, no separate a11y driver or suite
home: 1-a11y-scope-summary · a11y-scope-summary
@task_standing n=2
convention: every new fixed CLI string (caption, tree label, refusal, hint) is pinned by its literal in a test
home: 8-cognitive-accessibility · cognitive-accessibility
@task_standing n=3
convention: CLI a11y is checked by literal tests, a failure being a nextest failure
home: a11y-testing-tool-pick
@task_standing n=4
convention: a11y failures are nextest failures in the existing suites, no skips
home: bootstrap-phases-derive-for-route-setup-project
@task_standing n=5
convention: new a11y cases run inside the coverage suite in the push/PR workflow, no a11y artifact of its own
home: 9-ci-integration
@task_standing n=6
convention: every new test is deterministic, zero flakiness, no retries, a gate breach fails the build
home: 10-slo-invariants-a11y-budgets
@task_item n=1 part=not_needed
The dialog pending and answer path, its amber-only and dialog hold rows, and the dialog-expired event: lineage adds nothing to a dialog.
@task_item n=2 part=not_needed
`viola plugin install` output and exit rows: the chunk does not touch the plugin.
@task_item n=3 part=not_needed
`viola verify` and `viola wait` outputs (step line, interrupted pair, waiting line, timed_out): neither verb is changed.
@task_item n=4 part=not_needed
MCP server frame tools, hook event and dialog call refusals and the instance-unreachable details: no frame or hook call is added or changed.
@task_item n=5 part=not_needed
Absences of a web or graphical frontend, a touch surface, internationalization and a rendered page, and the N/A contrast, focus and WCAG mapping areas: nothing here is a page.
@task_item n=6 part=not_needed
The manual screen-reader targets and setups for the cli and tui: they are supplemental, gate nothing, and the chunk adds no manual pass.
@task_item n=7 part=not_needed
The structured violation JSON rows and the vt100 feed panic and pump_child error passthrough rows: the chunk changes no violation report and no feed handling.
