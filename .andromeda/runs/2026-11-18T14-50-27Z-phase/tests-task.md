@task_prose part=chunk
Adds a compiled screen literal for the CLI's browser-tools offer (2.1.325, «Claude in Chrome extension detected») so the one shared settled-box predicate reads it as a standing modal, never as a settled input box.
Adds its row to the capability ledger (the 24th relied row) and the viola verify probe that measures it on an installed CLI, the ledger count going 23 to 24.
Holds run's pre-send gate to refuse to type at that modal: no byte is typed into the prompt, the stop with the modal standing included.
Teaches the fake agent the browser-tools screen so CI proves the literal, the row and the gate's refusal against recorded screens.
Reads, not re-derives, the measured gap of the earlier live starts; names which verify step arms the prompt as measured, read statically or still unmeasured.
@task_file n=1 path=viola-agent-claude mark=edits
what: the claude agent component: the capability ledger, its screen-signature literals and the modal set the settled-box predicate reads — edits: the capability ledger (a new relied row), the screen-signature literals, the modal set
@task_file n=2 path=src/run/ mark=edits
what: viola run's wrapper and its pre-send gate, the readiness wait before a send is typed — edits: the pre-send gate's reading of a modal
@task_file n=3 path=src/cmd/verify mark=edits
what: the viola verify command and its probe runs, which measure each ledger row and write the stamp — edits: the new row's probe, its stamp, the row count
@task_file n=4 path=viola-fake-agent mark=edits
what: the fake agent that serves the claude boundary in tests, its screens and command line — edits: a browser-tools screen
@task_item n=1 part=needs
A new compiled screen signature: the browser-tools offer is read only by its compiled literal, never by parsing the rendered child screen beyond compiled signatures.
@task_item n=2 part=needs
The shared settled-box predicate used by verify's pastes and run's pre-send gate reads modal literals first, so the browser-tools screen reads as a modal standing and never as a settled box.
@task_item n=3 part=needs
A new relied ledger row: every ledger row has a verify probe, and the version gate needs every relied row to be stamped.
@task_item n=4 part=needs
The relied row count moves 23 to 24: stamps refuse a widened ledger, so a home stamped at 23 rows meets a binary with 24; what it reads there and what run does is the 22 to 23 step's shape.
@task_item n=5 part=needs
The verify probe for the new row: its step line, summary line and last line, a failing row being no refusal, and its stamped line for the installed CLI version.
@task_item n=6 part=needs
The pre-send gate refuses to type while the browser-tools modal stands, as at every modal: the send reads input-not-ready within the readiness gate's bound and no byte reaches the prompt.
@task_item n=7 part=needs
Viola answers nothing in the browser-tools prompt and writes no setting to turn it off; no blanket dialog approval and no key typed into a modal, by run or by verify.
@task_item n=8 part=needs
The fake agent gains a browser-tools screen, chosen by its command line alone, alongside its trust dialog screen; the fake agent is held to the recorded verify fixtures byte for byte.
@task_item n=9 part=needs
Recorded screen fixtures for 2.1.325 change only where the recorded set needs the new screen; screen fixtures keep box rows as their literals and are scrubbed, refused when dirty.
@task_item n=10 part=needs
Stamps in CI come only from verify against the fake agent: the fake agent must replay a pass of the new probe so the harness boot's stamping verify still stamps the home.
@task_item n=11 part=needs
A fake-agent screen change puts cli_verify in the CI gate block for this chunk.
@task_item n=12 part=needs
Live starts on the standing rig run the real claude CLI locally only, never in CI: one arm where the prompt must stand and its control, each with a failing predicate and RED on a missing teardown word, and a live reading recorded as read, never as verification.
@task_item n=13 part=needs
Every live or test verify start is tied to a test-owned user home and killed by its group on stop; the rig types no byte into the modal and the founder's real config is read as a witness only.
@task_item n=14 part=needs
Waits in the new tests key on logged events, stay inside the 20 s kill line for verify-driven tests and the WITHIN bound, and never sleep or retry; controls are named with --timeout.
@task_item n=15 part=needs
E2E cases for the gate's refusal assert on logged events and exit documents, never on the child's screen content.
@task_item n=16 part=needs
The claim is for CLI 2.1.325 only; the unverified CLI gate still governs any other version.
@task_item n=17 part=needs
A verify record word for the new probe, if the obs plan adds one, lands as a structured JSON log line with no secrets.
@task_standing n=1
convention: every new test creates its own private home under target/e2e-home and runs verify, run or revive under a test-owned user home
home: test-data-bootstrap
@task_standing n=2
convention: new test functions follow the naming rule, unit tests inline and table-driven with rstest, root integration binaries prefixed by their surface
home: 2-test-strategy · test-strategy
@task_standing n=3
convention: new tests assert literal oracles on contracts and observable behaviour, with no sleeps, no retries and no shared mutable state
home: 11-test-anti-patterns-never-do-these
@task_standing n=4
convention: every new fixture or fake-agent screen is synthetic, scrubbed of real data and produced by viola or the recorder, never hand-written product state
home: 7-test-data-fixtures · test-data-fixtures
@task_standing n=5
convention: every test that spawns viola or a verify start is tied, held in a drop guard and bounded by its per-test kill line
home: integration-test-strategy
@task_item n=1 part=not_needed
The browser test suite absence: the browser-tools prompt is a CLI screen, not a web frontend; no browser suite is added.
@task_item n=2 part=not_needed
Hook dialog decision bodies, dialog answer mapping and dialog holds: the browser-tools offer is no hook dialog and nothing is answered.
@task_item n=3 part=not_needed
MCP tool schemas, tools/list, MCP replies and MCP frame-tool causes: the chunk adds no MCP tool or reply.
@task_item n=4 part=not_needed
Wrapper channel JSON-RPC errors and the link and unlink methods: the channel's methods and bodies are untouched.
@task_item n=5 part=not_needed
revive refusal codes, list board and list --json rows, release and the wheel verbs: none of these surfaces changes.
@task_item n=6 part=not_needed
Hook performance budgets and the hyperfine perf job: no hook path changes.
@task_item n=7 part=not_needed
Mutation run verdicts and the mutants harness contracts: the directive runs no mutation for this chunk.
@task_item n=8 part=not_needed
Cross-user peer checks, socket directory refusal and server identity checks: no IPC security control is touched.
@task_item n=9 part=not_needed
Windows and macOS support, a database and outbound HTTP: absences the chunk does not approach.
