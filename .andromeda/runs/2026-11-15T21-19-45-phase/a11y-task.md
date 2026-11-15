@task_prose part=chunk
A turn the claude CLI ends on an API error is written to the event log as a turn-ended event carrying a cause code from a closed viola set; the error text reaches no log line, event or snapshot.
After that turn end the instance reads idle on viola list and the board, the turn-running state clears as a Stop clears it, a wait on the turn end returns, and the next send is accepted with no human pause and release.
It registers the CLI hook that ends such a turn, reads its payload in the Claude agent crate, maps the CLI error string into the neutral cause set with a fixed fallback member, and lets the fake agent emit that end.
It adds a test driving the API-error end through the fake agent and recorded or synthetic payloads that fails on the tree before the fix; P3 re-reads the defect from the code (which state reads turn-running while the board reads idle).
@task_file n=1 path=plugin/hooks/hooks.json mark=edits
what: the plugin's hook registration list, the hook event and dialog calls the claude CLI makes into viola — edits: the registration for the CLI signal that ends a turn on an API error
@task_file n=2 path=src/run/send/slot.rs mark=edits
what: the run pump's send slot, the wheel's turn-running state that refuses a send while a turn runs — edits: the turn-end path
@task_file n=3 path=src/run/wait.rs mark=edits
what: the viola wait path that returns a waiter on a turn end — edits: the turn-end path
@task_item n=1 part=needs
The turn-ended event line gains a cause key whose value is a code word from a closed set, with a fixed fallback member for any CLI value outside it; the cause value follows the code-word format.
@task_item n=2 part=needs
The CLI's error text (the last assistant message and error details of an API-error end) is vendor content: it never reaches an event line, a snapshot or a log line; only the mapped code word does.
@task_item n=3 part=needs
Vendor strings are screened to code words at the single vendor-content boundary: the CLI error string maps to a viola code word in the Claude agent crate and is never printed or logged as-is.
@task_item n=4 part=needs
No error message the chunk adds interpolates the vendor's text; any new error type's message interpolates nothing and no bare status or vendor string stands as error text.
@task_item n=5 part=needs
viola list and the board read the instance's state by its printed word idle after an API-error turn end, the same word a Stop-ended turn shows.
@task_item n=6 part=needs
viola send after an API-error turn end is accepted; the exit-13 refusal and its turn-running reason code no longer fall on a send after such an end, while the refusal itself, its unable line, reason code and hint stay as they are for a turn still running.
@task_item n=7 part=needs
viola wait on a turn end returns when the turn ends on an API error, so its waiting line is followed by its result rather than hanging until timed_out; time bounds stay on the driver, never on the human.
@task_item n=8 part=needs
The human never needs pause and release to recover the wheel after an API-error end; human keystrokes stay never blocked and the human taking the wheel behaves as before.
@task_item n=9 part=needs
The turn end is read from the hook payload, a record, never by parsing the claude screen content or inspecting claude's content beyond signatures.
@task_item n=10 part=needs
The new guard test runs on the fake agent and recorded or synthetic payloads; a live claude reading never gates the build, and none is started without the operator's cap.
@task_item n=11 part=needs
An unknown or unlisted cause stays visible as the fixed fallback code word rather than being dropped or replaced by the CLI's string.
@task_item n=12 part=needs
A test socket the new test opens lives under the short tmp root, and readers of event lines skip and count unknown data keys so mixed binary versions stay readable.
@task_standing n=1
convention: every new a11y assertion runs inside the existing test suites and its failure is a nextest failure, with no separate a11y driver
home: bootstrap-phases-derive-for-route-setup-project
@task_standing n=2
convention: the CI test gate allows no skipped test
home: 1-a11y-scope-summary
@task_standing n=3
convention: every new test is deterministic: zero flakiness, no retries
home: 10-slo-invariants-a11y-budgets
@task_standing n=4
convention: boundary tests are sequenced on records (hook events), never on timing or screen reads
home: keyboard-test-harness
@task_standing n=5
convention: a fixed CLI or event string a new test reads is pinned by its literal
home: 8-cognitive-accessibility · cognitive-accessibility
@task_item n=1 part=not_needed
Colour and SGR rows (amber, bold, dim stale rows, NAME truncation, colour decision inputs): the chunk adds no styling; the state word it changes is printed as before.
@task_item n=2 part=not_needed
The TUI passthrough's focus, mouse and resize sequences, the vt100 feed panic and the stdin non-editing list: the chunk changes no terminal byte path.
@task_item n=3 part=not_needed
viola revive refusals, revive --list and revive --json documents: the chunk touches no revive path.
@task_item n=4 part=not_needed
viola link and unlink, plugin install output and viola --help rows: no output of those verbs changes.
@task_item n=5 part=not_needed
Manual Orca and keyboard passes and screen-reader targets: supplemental, and the chunk adds no human-read surface.
@task_item n=6 part=not_needed
Absences of i18n, a touch surface or a web or graphical frontend, and the structured violation JSON rows: nothing here reaches them.
@task_item n=7 part=not_needed
Socket-directory, strict-modes and server-verify refusals on frame verbs and MCP frame tools: the chunk adds no refusal of that kind.
