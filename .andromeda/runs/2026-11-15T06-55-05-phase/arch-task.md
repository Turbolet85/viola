@task_prose part=chunk
The first live round on the product after the budget gate and the status-line interception left: rigs that drive the real claude headless on a plain pty the rig opens itself, each rehearsed first under the fake agent, then named operator steps, each with its live-start count, fired only on the operator's word.
It proves the wrapped start live (matrix v2-03): the unmodified claude wrapped on Linux, no viola byte on the pty, keys passed unchanged, the pinned copy run with a fresh plugin by absolute path, the start order with no budget step, a live or stale name refused, the child's end read on the process, on a ledger green for the CLI it runs.
It reads a live dialog hold near the 3600 s DIALOG_HOLD_MAX ceiling (matrix v2-02) so the founder can settle its PROVISIONAL mark; the ceiling itself is not changed unless that word moves it.
It re-records the MCP tools/list fixture so the list tool's description drops the usage budget, records a box-only screen fixture through a clean viola verify --record for the fake agent to replay in CI, and reads a wrapped session's status line beside an unwrapped one's, writing nothing to the user's settings.
@task_file n=1 path=fixtures/claude/2.1.302/Mcp.tools-list-1.json
what: the recorded claude MCP tools/list exchange the MCP server's tool list is held to, re-recorded live through viola-mcp-tap
@task_file n=2 path=crates/viola-e2e/src/bin/viola-harness.rs mark=edits
what: the test-only viola-harness binary of the viola-e2e crate, the live-round rig that drives real claude headless on its own pty and rehearses under the fake agent
@task_file n=3 path=crates/viola-e2e/src/bin/viola-mcp-tap.rs mark=edits
what: the test-only viola-mcp-tap binary of the viola-e2e crate, the tap that records the MCP exchange of a live claude start
@task_file n=4 path=scripts/agent-run.sh mark=edits
what: the agent-run test harness script that runs the end-to-end rigs under AGENT_RUN_ knobs on test homes
@task_item n=1 part=needs
The wrapped start of the unmodified claude CLI: run's start order with the child spawned last and no budget step, the pinned copy of the binary and its plugin passed by --plugin-dir by absolute path, the child's argv and environment, and run writing only the child's output to the terminal.
@task_item n=2 part=needs
A start over a live or stale name is refused, and the child's exit is detected on the process handle; the live rig asserts both.
@task_item n=3 part=needs
The human's keys pass unchanged through the passthrough to the child, never held behind a resize or the screen model.
@task_item n=4 part=needs
The dialog hold and its bound: DialogHold, DIALOG_HOLD_MAX 60 to 3600 s, config.json's dialog_hold_s, the hold's end with a null reply and the dialog-expired event; read live near the ceiling, its value left as it is unless the founder's word moves it.
@task_item n=5 part=needs
The capability ledger and its stamps for the CLI the round runs: viola verify as the one stamps writer on a test home, a version verified only when every relied row passes, the instance snapshot's cli_verified and cli_version reading green.
@task_item n=6 part=needs
viola verify --record: fixtures recorded only from a clean verify, scrubbed and hygiene-walked, refused as an unclean fixture on a home path, username, email or absolute path; screen fixtures keep only signature rows; verify types nothing into a CLI dialog.
@task_item n=7 part=needs
The fixture class fixtures/claude/<cli-version>/ with a new box-only Screen fixture (the input box's rule and prompt rows, no footer literal), never hand-written, replayed by the fake agent in CI; and the MCP list tool's description, held to the recorded tools/list exchange, carrying no usage-budget wording.
@task_item n=8 part=needs
The status line reading: the global statusline config untouched, no settings layer and no --settings at start; the live reading reads the user's own status-line setting and writes nothing to the user's settings file, as evidence for matrix v2-05 only.
@task_item n=9 part=needs
Live sessions run headless on a plain pty the rig opens itself, no compositor, window or desktop step; homes are test homes on tmpfs under the e2e-home base, its headroom checked first; every spawn carries a kill deadline.
@task_item n=10 part=needs
Nothing is written under the user's viola home or the user's settings; the pinned product binary and the pinned copies are untouched and never deleted; test homes are cleaned by their owner only.
@task_item n=11 part=needs
Real claude runs only locally, never in CI; CI stays on the fake agent, and each live rig has a fake-agent rehearsal that runs there.
@task_item n=12 part=needs
viola types into no trust dialog; a rig typing arrow and Enter into its own pty's trust dialog does so only on the founder's per-chunk word, and each rig says what it does without that word.
@task_item n=13 part=needs
The CLI version the round runs is the founder's choice among the shim's pinned one, the PATH claude and the fixture directory's version; the plan leaves it one line to fill, with the claude --version first line read to name it.
@task_item n=14 part=needs
The hour-long hold step journals its state in the work dir before the wait and names what a resumed window re-reads.
@task_item n=15 part=needs
Each live step leaves a live-round record in the chunk's evidence dir; the operator's live-start counts per step and the total stay one line each for the founder to fill.
@task_item n=16 part=needs
A guard test's remove-the-guard control runs through the gate's control with its case the test's name alone; no mutation run.
@task_item n=17 part=needs
A defect a live reading exposes folds into this chunk as its own item with its own acceptance; no other product behaviour changes.
@task_standing n=1
convention: each new test runs in its own viola home on the tmpfs-linked e2e-home base, its homes kept after the run and removed by owner record, its evidence surviving a kill
home: occupied-resources
@task_standing n=2
convention: each new test seam or harness knob is a test seam, never configuration, carrying the AGENT_RUN_ or FAKE_AGENT_ prefix, never read by the product
home: conventions · occupied-resources
@task_standing n=3
convention: each new recorded fixture is recorded from a clean verify, scrubbed, hygiene-walked, a screen fixture keeping only signature rows
home: occupied-resources
@task_standing n=4
convention: each new test-only binary or rig stays in the test-only crate, out of release builds, and every end-to-end run shows zero panic lines
home: crate-dependency-direction · project-directory-structure
@task_item n=1 part=not_needed
The wheel, link and unlink, pause and release behaviour: the round wraps a start and reads a hold, it changes no wheel or link rule.
@task_item n=2 part=not_needed
Send delivery and confirmation, the readiness gate's send path, paste normalisation and refusal orders: no send behaviour is changed.
@task_item n=3 part=not_needed
Revive, the board and list payloads, session liveness for listing: not exercised by the round.
@task_item n=4 part=not_needed
The channel protocol's frames, fault codes and versioning: no frame or method is added or changed.
@task_item n=5 part=not_needed
The Hyprland compositor seam and window features: live sessions run headless with no compositor.
@task_item n=6 part=not_needed
Fuzzing, mutation testing, coverage floors and the perf job: no mutation run, no fuzz target, no perf change.
@task_item n=7 part=not_needed
Dependency policy, toolchain pins, licence and release tooling: the round adds no dependency and releases nothing.
