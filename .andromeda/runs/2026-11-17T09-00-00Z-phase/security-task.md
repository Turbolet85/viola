@task_prose part=chunk
Makes a `viola run` wrapper a test boots, and the fake agent under it, end when that test ends: passed, failed, timed out or killed by SIGKILL.
Its first step makes the shared stamped test home valid, or rebuilt, on any host where its home is absent, whatever a restored `target/` cache carries; a test fails before the fix and passes after it.
Adds a proof of a killed test: a harness the test owns SIGKILLs a test process mid-run on the fake agent, then a census shows nothing of that test's own process tree is still running.
Re-measures from the code every place a test boots a wrapper, what ends it on each exit path, and why a killed test's wrapper spun; and tests the CI red's cause, the stale stamp marker in a restored cache, closing it against a CI run on an exact cache hit.
@task_file n=1 path=tests/support/home.rs mark=edits
what: the shared stamped test home fixture of the product's tests: its stamp marker and its test home under the e2e-home linked base — edits: stamp_dir, stamped_home
@task_file n=2 path=crates/viola-e2e/src/harness/boot.rs mark=edits
what: the e2e harness's boot of a `viola run` wrapper over the fake agent, and the shared stamped home it boots in
@task_item n=1 part=needs
The shared stamped test home: a stamp marker present while its home is absent (a restored `target/` cache whose home under the runner's temp root, reached through the e2e-home link, is gone) is no longer trusted; the home is re-checked and rebuilt.
@task_item n=2 part=needs
The stamps file of that test home: only `viola verify` writes it, so a rebuilt test home gets its stamps by running verify against the fake agent, never by writing the stamps file by hand.
@task_item n=3 part=needs
The version gate's unverified-CLI refusal a booted wrapper gave when the stamped home was absent: the fix restores the stamps the gate reads, and leaves the gate and its strict-modes read of the stamps as they are.
@task_item n=4 part=needs
Rebuilding the test home goes only through the e2e-home linked base guard: a keeper or rebuild follows the linked base only when it is safe, and test-side removal deletes only directories the tests created, never the backing viola itself ignores.
@task_item n=5 part=needs
Test homes stay out of git, and the rebuilt home keeps owner-only modes so the strict-modes check on the home tree still passes.
@task_item n=6 part=needs
The life of a booted wrapper and its fake agent bound to the test that booted them: on pass, fail, timeout and SIGKILL (which runs no destructor) they end, and after a test run none is left running.
@task_item n=7 part=needs
Process ownership: no test sends a kill to a process it does not own; the kill proof's harness kills only its own test process, and the census reads only that test's own process tree, as the standing rig's census does.
@task_item n=8 part=needs
Process identity for the census, through the process-identity library the project uses, read against the test's own process tree.
@task_item n=9 part=needs
A change `viola run` may need to end once its controlling test, its PTY or its parent is gone (its loop, which spun near a full core), in scope only as P3 measures it; no other product behaviour changes.
@task_item n=10 part=needs
The fake agent and its test seams: any seam a fixture or the kill proof adds lives only under the fake-agent test feature and stays compiled out of the release build.
@task_item n=11 part=needs
A change to the fake agent's screens puts `cli_verify` in the gate block.
@task_item n=12 part=needs
A test that waits on a channel or a thread takes a bound under the project's 20 s lint and cannot hang past a failure; a test socket goes under the short temp root, never a socket at the temp root itself.
@task_item n=13 part=needs
Tests use their own home: a booted test's wrapper, its fake agent and its stamps live only in the test's own viola home, never the user's, and nothing is written under the Claude home.
@task_item n=14 part=needs
No live `claude` start and no mutation run in this chunk; the real claude CLI is used only locally, and CI holds no secrets and starts no real claude.
@task_item n=15 part=needs
The CI test job's cache of `target/`: the acceptance evidence is a CI run on an exact cache hit that reads green; a green re-run alone does not close the red.
@task_item n=16 part=needs
New or changed CI uploads, if any, stay gated by the artifact canary scan, and no mutants.out upload is added.
@task_standing n=1
convention: every new test, fixture or harness uses its own viola home, kept out of git, and removes only directories it created
home: secret-management · security-anti-patterns
@task_standing n=2
convention: every new test-only seam or binary stays under the fake-agent test feature, compiled out of the release build, and any new environment read or seam gets its log entry
home: input-validation · security-anti-patterns
@task_standing n=3
convention: every new or changed CI job pins its actions by commit SHA, runs with least workflow permissions and checkout without persisted credentials, and passes the workflow audit gate
home: dependency-security
@task_standing n=4
convention: every new log line or test output honours the never-log floor: no paths, pids, ids or upstream text in external error output
home: error-handling · bootstrap-phases-derive-for-route-setup-project
@task_item n=1 part=not_needed
The wrapper channel methods, their frames, params, errors and refusal words (send, answer, wait, last, link, unlink, pause, release, hook frames): the chunk adds and changes no channel method.
@task_item n=2 part=not_needed
The MCP tools, MCP stdin line cap, MCP structured content and isError codes: the chunk does not touch the MCP server.
@task_item n=3 part=not_needed
The CLI frame verbs' refusals, exit codes, instance-unreachable details and revive --json codes: no CLI surface or its output changes.
@task_item n=4 part=not_needed
Paste and dialog free-text character sets, control-character refusal and typed-text normalisation: the chunk types no new text into the agent.
@task_item n=5 part=not_needed
Peer euid checks, server verification before a frame and the socket-directory check: the chunk opens no new channel client or listener.
@task_item n=6 part=not_needed
Hook payload parsing, hook strict-modes order and hook decision details: no hook behaviour changes.
@task_item n=7 part=not_needed
Pinned binary placement, re-hash and plugin file rewriting at start: the chunk does not change how a pinned copy is placed or checked.
@task_item n=8 part=not_needed
The standing live rig's own isolation, credentials copy and keying into its own pty: the rig is not reworked here.
@task_item n=9 part=not_needed
Recorded fixture scrubbing and screen fixture signature rows: the chunk records no new real-CLI fixture.
@task_item n=10 part=not_needed
Dependency pins, licence allow list, cargo-deny advisories and the fuzz workspace: the chunk adds no dependency.
@task_item n=11 part=not_needed
Release, signing and deploy decisions and release artefact attestations: nothing of the release stage is reached.
