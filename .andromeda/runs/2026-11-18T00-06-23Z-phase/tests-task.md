@task_prose part=chunk
Adds to viola verify one closed first-screen word per interactive run (trust, imports, box, none; null for a run that did not start) kept in the stamp's measured field, taken from the compiled screen signatures with no screen text kept.
Gives the standing live rig's V arm a print of the six words before teardown, a git-initialised scratch root seeded at its root's key (Runs B-E in a trusted repository, Run F in a fresh dir outside any repository), and gives the --record verify a witness of where its record went.
Bounds the verify_window_ kill and the 52 s cli_verify case (verify window without screens fails every interactive row) with the measured walls, or moves that kill.
Re-reads live, once, the 23rd relied ledger row trust-record on the real 2.1.325 CLI: one base live verify S0' plus at most two spares on a named cause, read against a floor of the 21 rows the live home's 2.1.325 stamp passes.
Changes nothing of viola run or revive and claims no capability; the product write and its e2e tests belong to the next entry.
@task_file n=1 path=src/cmd/verify/typed/run.rs mark=edits
what: viola verify's typed interactive runs, which start each interactive claude child under the kept CLAUDE_CONFIG_DIR and read its first screen against the compiled signatures — edits: the per-run first-screen word written into the stamp's measured field
@task_item n=1 part=needs
The verify stamp's measured field gains a closed per-run first-screen word drawn only from compiled screen signatures: no screen text, no rendered-screen parsing, no assertion beyond the compiled literals.
@task_item n=2 part=needs
Verify remains the only stamp writer; the new measured words travel inside the stamp verify already writes, and a stamp still refuses a widened ledger.
@task_item n=3 part=needs
The trust-record relied ledger row and its verify probe: the row's probe and the ledger-probe contract suite, the version gate that needs every relied row, and a failing row read as a failing row, never as a refusal.
@task_item n=4 part=needs
The viola verify stamped line, step line, summary line and last line as the live reading and the fake-agent tests read them back (verify-exit, rows passed of 23, the trust-record verdict).
@task_item n=5 part=needs
The verify_window_ class kill (60 s) and the cli_verify case that reads 52.2-52.3 s against it: the chunk bounds the case or moves the kill using the measured walls, and the kill lines must stay below the mutants and CI timeouts they sit under.
@task_item n=6 part=needs
Per-test kill bounds for verify-driven tests (20 s kill line for stamped-home tests) and the root wait bound below the kills: any test the chunk adds that waits takes a bound under the 20 s lint.
@task_item n=7 part=needs
A change to the fake agent's screens or to verify's runs puts cli_verify in the gate block: the chunk's fake-agent work is green and landed before the first live start.
@task_item n=8 part=needs
The fake agent's trust dialog screen and its trust-config and trust-record-blank options, as the fake-agent tests that drive the first-screen words (trust, imports, box, none) use them; fake-agent screens are replayed from recorded bytes.
@task_item n=9 part=needs
The one standing committed live rig and its V arm: real claude CLI only locally, never in CI, through the local-live runner seam; its git-initialised scratch root seeded at the root key; each start ledgered with the clock's time before it is made; no keys into any dialog or box; the rig's own stop (Ctrl-C into its pty or a no-key group kill); every process it starts ended.
@task_item n=10 part=needs
A live reading is recorded as read, never as verification, and a fake-agent run is no proof of real-CLI behaviour: the row is lifted from unverified only when the floor holds and trust-record reads pass on 2.1.325 in the rig's private home.
@task_item n=11 part=needs
The --record verify witness of where its record went: tests never read or attribute the host's own claude sessions or a shared file those sessions rewrite; the witness reads only test-owned or rig-private paths.
@task_item n=12 part=needs
The claude child's environment strip and the kept CLAUDE_CONFIG_DIR handed to verify's interactive children, read (argv, env after the strip, cwd, pty size, timing of the screen read) to predict each run's first-screen word; nothing of the founder's config is kept.
@task_item n=13 part=needs
Child exit is read on the process handle, never on reader EOF: the closed fact that separates candidates (child exit before the read, count of settled frames, the run's wall) is read that way and kept as closed words and numbers only.
@task_item n=14 part=needs
Every test verify start is tied to a test-owned home and runs under a test-owned user home; stamping verifies give the fake agent a transcript; one verify stamp per build.
@task_item n=15 part=needs
Each control is a bounded remove-the-guard witness fired with its named --timeout, with ids within 16 characters; holds means those cases ran and failed with no TIMEOUT line; each says what the subject does when the guard is gone; the tied-verify kill witness applies to a stamping verify. No mutation run is made.
@task_item n=16 part=needs
A gate entry naming an installed tool's path is checked against the host before the card.
@task_standing n=1
convention: Each new test function is named by the test naming rule, uses rstest case tables for table-driven cases, and is selected into its layer by its binary prefix.
home: 2-test-strategy · test-strategy
@task_standing n=2
convention: Each new test waits on logged events or files with a bound, never with sleeps or retries.
home: 11-test-anti-patterns-never-do-these
@task_standing n=3
convention: Each new test creates its own private home under the e2e home root and seeds its own data; on-disk product state is produced by viola, never hand-written.
home: test-data-bootstrap
@task_standing n=4
convention: Each new test's oracle is a literal of the contract, never a fixture internal or a Debug string.
home: 11-test-anti-patterns-never-do-these
@task_standing n=5
convention: Each new fixture or fake-agent input holds synthetic text only, scrubbed of home paths and usernames.
home: 7-test-data-fixtures · test-data-fixtures
@task_standing n=6
convention: Each new test that spawns viola or a wrapper holds it tied, in a drop guard, with the inherited session environment dropped.
home: integration-test-strategy
@task_item n=1 part=not_needed
The MCP server surfaces: tools/list, tool arguments, tool results, reply screens and stdin line caps; verify takes no MCP path.
@task_item n=2 part=not_needed
The wrapper channel JSON-RPC errors, link and unlink methods, channel wait and endpoint rows; no channel frame is touched.
@task_item n=3 part=not_needed
viola send, wait, last, answer and release contracts, send confirmation windows and wheel rules; nothing of a running wrapper changes.
@task_item n=4 part=not_needed
viola run and revive start refusals, exit codes, refusal codes and snapshot fields; the scope says nothing of run or revive changes.
@task_item n=5 part=not_needed
Hook decision bodies, dialog events and hook exec-form entry points; no hook behaviour is changed.
@task_item n=6 part=not_needed
viola list board and list --json rows, liveness and heartbeat staleness classification.
@task_item n=7 part=not_needed
Performance budgets and hyperfine perf gates over the hooks; no hook path is timed here.
@task_item n=8 part=not_needed
Fuzz targets, fuzz corpora and property generators over the parsers; no parser surface is added.
@task_item n=9 part=not_needed
The mutation run verdict contracts and the epoch-boundary mutation cadence; the scope makes no mutation run.
@task_item n=10 part=not_needed
Harness status, session record, cleanup document and boot readiness contracts of the agent-run harness; the chunk adds no harness verb.
@task_item n=11 part=not_needed
CI artifact names and retention, workflow permissions and Action SHA pinning; no workflow changes.
@task_item n=12 part=not_needed
The browser suite, database and outbound HTTP absences and the Windows and macOS absence; nothing here approaches them.
