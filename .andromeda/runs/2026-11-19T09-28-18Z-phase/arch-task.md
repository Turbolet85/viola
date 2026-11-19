@task_prose part=chunk
Lands a new recorded fixture set for claude 2.1.328, recorded by a clean live `viola verify --record` under the rig's private config (R4) plus the `viola-mcp-tap` re-record (R4M), copied whole from the recorders' output with every file through the recorder's scrub and none hand-edited.
Names the new set in the ledger probes' stamped list, so the drift suites and the recorded-exchange suites read over it; a drift suite red on the landed set stops the work.
Changes rig code for the R4 / R4M labels only if 2.1.328 needs it; otherwise the chunk is the two live starts, the landing and the gates.
Disposes the three carries by name: the placement ahead of the live StopFailure payload, the founder's cap word for the live starts, and the `/context` post-condition's unread stdout-tag half, read from a readable transcript beside the recording or named as leaving with its next reader.
@task_file n=1 path=fixtures/claude/2.1.328
what: the recorded fixture set for one claude version (hook payloads, screen signature rows, agents listing, the MCP tools/list exchange), recorded by a clean verify and the MCP tap, scrubbed and hygiene-walked, replayed by the fake agent in CI
@task_file n=2 path=tests/contract_ledger_probes.rs mark=edits
what: the ledger probes and drift suites over the stamped recorded fixture sets, each set read at once — edits: the STAMPED list of stamped versions
@task_item n=1 part=needs
Fixtures are recorded only from a clean `viola verify --record`: an unclean run records nothing, and an unclean-fixture refusal (absolute path, home path, username, email) is a red to report, never a file scrubbed by hand.
@task_item n=2 part=needs
The recorder's scrub and hygiene walk bind every recorded file of the new set; screen fixtures keep only signature rows; the set lands as a plain copy of the recorders' output.
@task_item n=3 part=needs
The MCP tools/list answer is held to a recorded claude exchange: the `viola-mcp-tap` re-record gives the 2.1.328 exchange the recorded-exchange suites read.
@task_item n=4 part=needs
The ledger stamps for 2.1.328: verify is the one stamps writer, a version counts as verified only when every relied row counts, and a stamps merge replaces only its own version; the live home's 24-row stamp is the product's ground.
@task_item n=5 part=needs
The drift suites and recorded-screen suites replay a recorded set against the fake agent; whether they run over every stamped set or the newest, and what in the fake agent is keyed to a version, decide whether the new set changes the fake agent or its test seams.
@task_item n=6 part=needs
The capability ledger rows read on 2.1.328: the browser-tools prompt row, the trust-record row and its probe, the local-command echo row; a moved screen literal or the trust dialog's new decline text reaches the compiled screen signatures and literals.
@task_item n=7 part=needs
Verify never types into a CLI dialog or a trust dialog, writes and removes its own trust record, keeps plan-mode probe files in its own run dir and removes only the probe folders it created.
@task_item n=8 part=needs
Real claude runs only locally, never in CI: the live starts run through the live rig (labels R4 and R4M) under its private config, by path, nothing under the operator's real home beyond the credentials file read once and the read-only stamped config; each start needs the founder's cap word.
@task_item n=9 part=needs
Verify refuses an untrusted home tree or stamps, and checks strict modes before any log opens; the rig's private home must pass those checks for the recording to run.
@task_item n=10 part=needs
The `/context` local command's delivery post-condition: confirmation by its stdout echo read from the session transcript; on 2.1.328 the send read not-delivered with no-prompt-submitted and the transcript absent, so the stdout-tag half stays unread until read beside this recording or carried on.
@task_item n=11 part=needs
Verify's CLI transcripts are an accepted residual of a live run; the rig keeps its frames and the kept record locally, never uploaded.
@task_item n=12 part=needs
Every test start under a test-owned viola home; every test wait takes a bound under the shared root wait bound (the 20 s lint); every process started is ended; no scheduled wake or timer.
@task_item n=13 part=needs
No mutation run in this chunk; CI against the fake agent stays the verdict and must pass with the new set in place.
@task_item n=14 part=needs
The fixture set's version directory name is the claude version the CLI's `--version` first line reports.
@task_standing n=1
convention: each new recorded fixture file passes the recorder's scrub and hygiene walk, comes only from a clean verify, and screen fixtures keep only signature rows
home: occupied-resources
@task_standing n=2
convention: each new or changed test runs against the fake agent in CI, never a live claude
home: ci-cd-approach
@task_standing n=3
convention: each new test runs in its own viola home and every wait it takes stays inside the one shared root wait bound
home: established-decisions · occupied-resources
@task_standing n=4
convention: each live start of the real claude is a local-only probe, never a CI job
home: ci-cd-approach
@task_item n=1 part=not_needed
The wheel rows (byte-source claim, human take, release, wheel events): the chunk changes no wheel behaviour.
@task_item n=2 part=not_needed
The wrapper channel's frames, methods, fault codes and the exit-21 documents of the frame verbs: no channel method or frame changes.
@task_item n=3 part=not_needed
Revive, snapshot replay and session liveness rows: the chunk revives nothing and changes no snapshot.
@task_item n=4 part=not_needed
The Hyprland seam, window features and the terminal fleet console: nothing of the compositor is touched.
@task_item n=5 part=not_needed
config.json keys, the colour environment and diagnostics-level rows: the chunk adds no configuration.
@task_item n=6 part=not_needed
Release, deployment, cargo install and plugin install rows: nothing is released or installed by the chunk.
@task_item n=7 part=not_needed
The list board, agents listing join and link / unlink rows: no surface of the board or the link set changes.
@task_item n=8 part=not_needed
The fuzz workspace and its nightly toolchain: no fuzz target or corpus changes.
