@task_prose part=chunk
Lands a new recorded fixture set for claude 2.1.328, made by a clean live viola verify --record under the rig's private config plus the viola-mcp-tap re-record, copied whole from the recorders' scrubbed output with nothing hand-edited.
Points the ledger probes' STAMPED list at the new set, so the drift suites and the recorded-exchange suites replay over it against the fake agent.
Re-reads the /context post-condition's unread stdout-tag half from a readable session transcript next to the new recording, or says it cannot and where it goes.
Settles each of the entry's three carries by name. Whether the fake agent or the rig labels need a version-keyed code change (for example the trust dialog's new decline text) is decided at P3.
@task_file n=1 path=fixtures/claude/2.1.328/
what: the recorded fixture set for claude 2.1.328: the recorded exchanges and screens that the drift suites and recorded-exchange suites replay against the fake agent, each file passed through the recorder's scrub
@task_item n=1 part=needs
The viola verify recording run, whose step lines are the output of the verify that records the set; an interrupted verify is the stop and exit pair it must report.
@task_item n=2 part=needs
The vendor-content boundary: recorded claude screens and exchanges are claude's content. A boundary check or replay asserts signatures only and never inspects or parses claude's screen content beyond them.
@task_item n=3 part=needs
The TUI passthrough over a recorded or live session: vendor content passes through, viola writes zero bytes of its own to the terminal, and its run diagnostics stay off the terminal.
@task_item n=4 part=needs
The dialog pending-then-answer critical path, as replayed from the new set. On 2.1.328 the trust dialog has a decline text and only shows a trust prompt with --add-dir; the browser-tools prompt key and its confirm label are unchanged.
@task_item n=5 part=needs
The confirmed send and readback critical path: the /context send whose readback was not-delivered:no-prompt-submitted, which is the viola send refusal for no-prompt-submitted, and its post-condition read from a transcript.
@task_item n=6 part=needs
Fixed strings pinned by literal: if a 2.1.328 literal the fake agent or suites key on has moved, it is re-pinned as a literal, never loosened.
@task_item n=7 part=needs
Paths not carried: recorded files carry no host or home paths, and any payload the recorder's hygiene refuses counts as a red and is never scrubbed by hand.
@task_item n=8 part=needs
Live readings never gate: the live recording starts are operator rounds under the founder's cap word, and the CI suites replay only recorded fixtures.
@task_item n=9 part=needs
Time bounds fall on the driver only: each test that waits takes a bound under the 20 s lint, and the dialog hold bounds the driver, never the human.
@task_item n=10 part=needs
The CI test gate: no skips and zero flakiness with no retries. A drift suite that goes red on the landed set fails the build and stops implement.
@task_item n=11 part=needs
Human keystrokes are never blocked and the human key takes the wheel during a live recorded session. Boundary tests are sequenced on records.
@task_standing n=1
convention: every replay or check over a new fixture set runs inside the existing test suites as nextest failures, with no separate a11y driver, suite or artifact
home: bootstrap-phases-derive-for-route-setup-project · 9-ci-integration
@task_standing n=2
convention: a11y assertions live inside the existing suites, and the CI gate allows no skips
home: 1-a11y-scope-summary
@task_standing n=3
convention: every new or re-pointed test is deterministic, with zero flakiness and no retries, and a gate breach fails the build
home: 10-slo-invariants-a11y-budgets
@task_item n=1 part=not_needed
The viola list board and caption, colour, SGR, the dim on stale rows and NAME truncation: the chunk records and replays fixtures and changes no board rendering.
@task_item n=2 part=not_needed
The viola revive --json refusals and revive --list output: the chunk does not touch revive.
@task_item n=3 part=not_needed
viola link and unlink, viola plugin install, and the viola --help group labels, usage line and handoff row: none of these is in scope.
@task_item n=4 part=not_needed
The frame verb, hook call and MCP frame tool refusals for socket-dir, strict-modes and server-verify, plus the instance-unreachable details: the chunk changes no client or socket path.
@task_item n=5 part=not_needed
The screen-reader targets and the manual Orca and keyboard passes: they gate nothing, and the chunk makes no screen-reader change.
@task_item n=6 part=not_needed
The absences of a web or graphical frontend, a touch surface and internationalization, and the N/A harness areas for contrast, focus and the WCAG mapping.
@task_item n=7 part=not_needed
The structured violation JSON rows and the reading-level and cognitive escalation decisions: the chunk adds no a11y report format and no new copy.
