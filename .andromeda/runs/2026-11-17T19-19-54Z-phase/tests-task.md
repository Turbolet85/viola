@task_prose part=chunk
Adds the 23rd relied capability-ledger row on the claude side: the claude CLI honours a per-project trust record (hasTrustDialogAccepted) that another process wrote into the config file its child reads.
Adds that row's viola verify probe, a sixth interactive verify run that writes the trust record for its own OS-temp dir, expects the child's first settled frame to be no trust dialog, and removes that one record of its own after the run.
Builds the trust-record writer once (lock when free, atomic replace, read-back, at most 3 attempts), where the next entry's product write can reach it, and makes the fake agent decide trust by reading the record as well as argv.
Re-runs the step-count sweep that the sixth run moves, and fires one live viola verify (S0) on the 2.1.325 install under the rig's private config to stamp the row. No capability is claimed.
@task_item n=1 part=needs
The capability ledger gains a 23rd relied row with its own viola verify probe; every ledger row has a verify probe, and the ledger probe contract suite holds the new probe.
@task_item n=2 part=needs
viola verify's ratified child set grows from five interactive runs to six: the sixth is a twin of the first over a fresh OS-temp dir with its trust record written before the child starts.
@task_item n=3 part=needs
The record's form: the CLI's own key (the cwd's git root inside a repository, else the cwd) with hasTrustDialogAccepted alone, in the file the child reads (the .claude.json under CLAUDE_CONFIG_DIR when the child keeps that variable, else the user's .claude.json).
@task_item n=4 part=needs
The writer's race rule: take the CLI's lock when free, replace the file atomically, read it back, at most 3 attempts; the JSON shape lives on the claude side and the file work on a side both verify and the later run reach.
@task_item n=5 part=needs
A config file that is absent, unreadable, a link, not the JSON expected, or oversize gets nothing written, and the probe's row reads what it could not do in a closed word, never pass; that word carries no path and no file content.
@task_item n=6 part=needs
verify removes only its own record, never another, on every exit path the run reaches, interrupt by SIGINT or SIGTERM included; a SIGKILLed verify leaves its record and the stop removes it.
@task_item n=7 part=needs
The probe can fail: with the record written the fake agent's first settled frame is no trust dialog; with the record withheld the same run shows the trust dialog screen. Both proved on the fake agent.
@task_item n=8 part=needs
The fake agent decides trust by reading the record from exactly the config file and key the writer used, not from argv --trusted-root alone; this change to its trust reading and screens puts the cli_verify suite in the gate block.
@task_item n=9 part=needs
When the CLI does not honour the record the row reads fail; a failing ledger row is no refusal, the product never writes the record, and the trust dialog stays the human's.
@task_item n=10 part=needs
The sixth run moves verify's step counts: the step line and summary line counts change, and the step-count sweep re-runs over every spelling of the count.
@task_item n=11 part=needs
Stamps go stale: once the 23rd row is relied, every 22-row stamp (2.1.325's included) reads unverified under the version gate that needs every relied row; the tests' shared stamp from the fake agent's verify and CI stay green.
@task_item n=12 part=needs
The bounded exception: rows saying viola writes nothing into the user's .claude.json take verify's own record as their exception; the product's write is not amended in.
@task_item n=13 part=needs
One live start S0 (plus one spare S0-2 only to re-fire a start lost to its shape): one live viola verify on the 2.1.325 install by path, 7 claude starts (1 print probe, 6 interactive), under the standing rig's private seeded config through a kept CLAUDE_CONFIG_DIR; ledgered with the clock's time before it is made, never in CI.
@task_item n=14 part=needs
The live start's keys and stop: no key typed into any dialog or box by the rig (verify pastes only its own ratified literals); the stop is the rig's standing one, Ctrl-C into its own pty or a no-key group kill; a start refused for the token window is uncounted.
@task_item n=15 part=needs
The founder's real .claude.json is read only by the mark witness, and verify's summary prints what proves its record is gone after the run.
@task_item n=16 part=needs
The owed reading: what triggers the CLI's rootOnly branch (only the git root's record counts) is read from the installed CLI's strings if reachable without a start, else named unmeasured.
@task_item n=17 part=needs
No mutation run for this chunk.
@task_item n=18 part=needs
Each control's --timeout is named in prose and fired with that flag; control and sweep ids are at most 16 characters.
@task_item n=19 part=needs
Every test that waits takes a bound under the 20 s kill line for verify-driven tests, and every process the chunk's tests or session start ends with them; no scheduled wake or timer.
@task_standing n=1
convention: each new test function is named by the test naming rule and its cases go in rstest case tables
home: 2-test-strategy
@task_standing n=2
convention: each new test creates its own private home and directories through tempfile and seeds its own data
home: 7-test-data-fixtures · test-data-fixtures
@task_standing n=3
convention: each new test waits on logged events or file state, never sleeps or retries, and mutates no process-wide environment
home: 11-test-anti-patterns-never-do-these
@task_standing n=4
convention: each new fixture or synthetic input is free of real data and carries the canary
home: 6-e2e-test-strategy · e2e-test-strategy
@task_standing n=5
convention: each new test that spawns viola or a child holds it tied in a drop guard so it ends with the test
home: integration-test-strategy
@task_standing n=6
convention: each new log line is a structured JSON line carrying codes only, its detail kept apart
home: log-format
@task_item n=1 part=not_needed
MCP server, MCP tools and MCP tool results: verify has no MCP surface and the chunk adds none.
@task_item n=2 part=not_needed
viola send, answer, wait, last and release contracts, and the wheel: nothing of the driver's frame verbs changes.
@task_item n=3 part=not_needed
viola run start and revive refusals, exit codes and start events: the boundary says nothing of run or revive changes here.
@task_item n=4 part=not_needed
The product's own trust-record write for session folders, its e2e tests and its live starts: they belong to the next entry.
@task_item n=5 part=not_needed
Wrapper channel JSON-RPC errors, link and unlink methods and the event-data keys of those records: the chunk touches no wrapper channel.
@task_item n=6 part=not_needed
Hook dialog decision bodies, hook exec entry points and hook performance budgets: no hook behaviour changes.
@task_item n=7 part=not_needed
viola list board and list --json, and the agent-run status document: no listing or status surface changes.
@task_item n=8 part=not_needed
Harness pre-push stages, details and reasons: the chunk adds no pre-push stage.
@task_item n=9 part=not_needed
CI artifact names and retention: the chunk adds no CI job and no artifact.
