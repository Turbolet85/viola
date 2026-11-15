@task_prose part=chunk
A test-only corrective chunk: it changes test code and test config so the next boundary's audit can reach a green coverage suite and a green mutation baseline; it runs neither.
It gives the hook fail-open test's channel a socket name that does not grow with the pid and the event, and proves the path's length bound under the mutation scratch with a 7-digit pid.
It resizes the nextest mutants profile's slow-case kill and the receipt wait together, re-measuring the slow cases' wall times with the host's load beside them.
It kills the ten surviving mutants of the hypr-tap recorder test binary with tests that fail on each mutated site, or states in the test-plan's tier why the recorder's main is outside the tier.
It makes the rig's home-naming helper refuse a test home for viola verify --record with a path component named home.
@task_file n=1 path=tests/hook_fail_open.rs mark=edits
what: the hook fail-open integration test and its test channel endpoint socket — edits: the test socket name and its join and bind
@task_file n=2 path=.config/nextest.toml mark=edits
what: the nextest test-runner config, its mutants profile — edits: the mutants profile's slow-timeout kill
@task_file n=3 path=tests/support/watch.rs mark=edits
what: the test support receipt watch, the bound a test waits for a receipt — edits: the WITHIN constant and its doc comment
@task_file n=4 path=tests/support/home.rs mark=edits
what: the test rig's home helper, the test home every viola verify --record test caller takes — edits: TestHome::new, the home() fixture, scratch()
@task_file n=5 path=crates/viola-e2e/src/bin/viola-hypr-tap.rs mark=edits
what: the hypr-tap recorder, a test-only end-to-end binary that records desktop events — edits: its tests over main's argument match and its record and record-dispatch arms, version_of's Version arm, texts' array arm
@task_item n=1 part=needs
The test channel's socket path stays within the Unix socket path bound under the mutation scratch directory with a 7-digit pid and the longest event label, proved by a computed or asserted bound in the test.
@task_item n=2 part=needs
The mutants profile's slow-case kill margin stays below cargo-mutants' 20 s auto-timeout floor and covers the ~5 s slow class (instance-state run, revive after a killed wrapper, start over an unreadable snapshot) with measured wall times.
@task_item n=3 part=needs
The receipt wait bound stays strictly below the mutants profile's kill, so the two move together; whether the bound is per call or scaled is closed at P3 from the wait_for callers.
@task_item n=4 part=needs
Each hypr-tap survivor is killed by a test proved by a remove-the-guard control under a kill deadline, the control's case the test's name alone, holding only when that case ran and failed.
@task_item n=5 part=needs
A test home handed to viola verify --record never holds a path component named home; the rig's home-naming helper enforces it so no test plan can name it wrong.
@task_item n=6 part=needs
The viola verify --record path is used by the verify test callers through the test home helper; its output and refusals stay as they are, the chunk changes only the home the tests give it.
@task_item n=7 part=needs
No product line changes unless a test proves one wrong; a red or defect found now folds into this chunk as an inferred item with its own acceptance.
@task_item n=8 part=needs
No mutation run and no coverage run as acceptance, no live claude start and no live desktop step inside the chunk.
@task_item n=9 part=needs
Tool behaviour the chunk relies on (nextest override inheritance and the slow-timeout form, the temp directory placed under TMPDIR) is measured at P3 and cited, not assumed.
@task_item n=10 part=needs
If the recorder's main is ruled outside the mutation tier, the test-plan's tier text states it with its reason as a card item.
@task_item n=11 part=needs
New tests are named so an absence probe checks against the plan's own test names and fixture keys before the card.
@task_item n=1 part=not_needed
The board and the BAY line of viola list, its columns, truncation and styling: the chunk renders no output and changes no board.
@task_item n=2 part=not_needed
The CLI token rendering, colour, motion and spacing rows and the SGR styling decisions: test-only, no styled output is changed.
@task_item n=3 part=not_needed
viola send output, the readback mirror and send refusals: no send path or wording changes.
@task_item n=4 part=not_needed
The viola mcp tools and tool results: no MCP surface is touched.
@task_item n=5 part=not_needed
viola revive and viola run start outputs, refusals and --json error codes: no start path changes.
@task_item n=6 part=not_needed
viola link and unlink output, viola answer, viola last and viola wait output: none is touched.
@task_item n=7 part=not_needed
CLI exit codes, the refusal line form, hints and the frame-verb exit-21 detail: no refusal or exit path changes.
@task_item n=8 part=not_needed
The web surface absence and the deep-link absence: the chunk adds no surface.
