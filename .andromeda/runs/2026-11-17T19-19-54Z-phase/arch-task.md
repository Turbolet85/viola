@task_prose part=chunk
Adds the 23rd relied capability-ledger row in viola-agent-claude: the claude CLI honours a per-project trust record (projects[key].hasTrustDialogAccepted) that another process wrote into the config file its child reads.
Adds its viola verify probe as a sixth interactive verify run that writes the trust record for its own OS-temp dir, expects the child's first settled frame to be no trust dialog, and removes that one record of its own after the run.
Builds the trust-record writer once, where the later product write of the next entry can reuse it, and makes the fake agent decide trust by reading that record, so the probe and its control can fail on the fake.
Re-runs the step-count sweep the sixth run moves, re-measures only what its own code touches (and the owed rootOnly reading), and stamps the row with one live viola verify S0 on the 2.1.325 install under the rig's private config.
Claims no capability; viola run, revive and the product's own trust write stay unchanged.
@task_item n=1 part=needs
The capability ledger and its gate: a new relied row with its own verify probe, read by the rule that a CLI version is verified only when every relied row passes; the row reads fail when the CLI does not honour the record.
@task_item n=2 part=needs
The ledger stamps: once the 23rd row is relied every 22-row stamp reads unverified (2.1.325's included); verify stays the one stamps writer, a stamp merge replaces only its own version, malformed or untrusted stamps read unverified, and the tests' shared stamp and CI must stay green.
@task_item n=3 part=needs
Verify's bounded child set grows from five interactive runs to six (a Run A twin over a fresh OS-temp dir with its record pre-written); verify's probe dirs stay drop-guarded and verify removes only the probe folders it created.
@task_item n=4 part=needs
The trust record's form: the CLI's own key (the cwd's git root inside a repository, else the cwd) with hasTrustDialogAccepted alone, written into the config file the child reads (the .claude.json under CLAUDE_CONFIG_DIR when the home keeps that variable for the child through claude_env_keep, else the user's .claude.json).
@task_item n=5 part=needs
The record writer's race rule: take the CLI's lock when free, atomic replace, read-back, at most three attempts; through viola's crash-safe replaced-file discipline, mode set before bytes and the strict-modes checks over the files it opens.
@task_item n=6 part=needs
The writer's refusals: a config file absent, unreadable, a symlink, not the JSON expected or oversize leaves nothing written, read through a bounded reader that tolerates unknown keys, and the probe's row reports a closed word for what it could not do, never pass.
@task_item n=7 part=needs
Verify removes only its own trust record, never another, on every exit path the run reaches (interrupted included); what a SIGKILLed verify leaves and how the rig's stop removes it is stated; the summary prints what proves the record is gone.
@task_item n=8 part=needs
The writer's placement under the crate dependency direction: viola-agent-claude does no file I/O and viola-state is Claude-free, so the record's JSON shape lives on the Claude side and the lock, atomic replace and read-back on a side both verify and the later run reach; the exact home is a technical fork decided at P3.
@task_item n=9 part=needs
The probe's post-condition and control: with the record written the child's first settled frame shows no trust dialog; with it withheld the same run reads the trust dialog; detected through compiled screen signatures and screen rows read only by verify, and verify types nothing into any dialog.
@task_item n=10 part=needs
The fake agent's trust decision: it reads the record from the same config file and under the same key the writer used, not argv --trusted-root alone, through its test seams; a change to its screens or trust reading puts cli_verify in the gate block.
@task_item n=11 part=needs
Verify's stdout step lines and summary line: the sixth run moves verify's step counts, and the step-count sweep runs over every spelling of the count (four files at the cancelled P3's count).
@task_item n=12 part=needs
The bounded exception: the rows saying viola writes nothing into the user's .claude.json take verify's own record as their exception, and viola still types into no trust dialog; the product's own write is not amended in by this chunk.
@task_item n=13 part=needs
The live start: real CLI probes run only locally, never in CI; S0 is one live viola verify on the 2.1.325 install by path under the rig's private config (seed_config), seven claude starts, plus one spare S0-2; keys none, stop the rig's standing one; a start refused for the token window is uncounted, with no timer.
@task_item n=14 part=needs
The child environment verify spawns: inherited CLAUDE names stripped, CLAUDE_CONFIG_DIR reaching the child only where claude_env_keep keeps it, so the writer targets the config file the child actually reads.
@task_item n=15 part=needs
Verify's exit codes and refusals: a failed relied row exits 1; the new row's closed-word outcomes join verify's existing exit detail without reusing a retired code.
@task_item n=16 part=needs
The owed rootOnly reading (only the git root's record counts): what triggers it on 2.1.325 and 2.1.326, read from the installed CLI's strings without a start where reachable, else named unmeasured.
@task_item n=17 part=needs
Test waits and controls: each wait takes a bound under the 20 s lint and the shared root bound; each control fires with its named --timeout; control and sweep ids at most 16 characters; no mutation run.
@task_item n=18 part=needs
The session ends every process it starts and sets no scheduled wake or timer; booted test processes end with their test.
@task_standing n=1
convention: each new test runs in its own isolated viola home, cleaned by its owner process, with its evidence surviving a kill
home: occupied-resources
@task_standing n=2
convention: each new test runs against the fake agent; no live claude in CI
home: ci-cd-approach
@task_standing n=3
convention: each new error variant joins its crate's one thiserror enum with messages that interpolate nothing
home: conventions
@task_standing n=4
convention: each new log line is a codes-only process-log line, content only in the instance detail file
home: cross-cutting-patterns
@task_standing n=5
convention: each new OS-gated test names its reason
home: cross-cutting-patterns
@task_standing n=6
convention: each new screen fixture keeps only signature rows and is recorded only from a clean verify, scrubbed and hygiene-walked
home: occupied-resources
@task_item n=1 part=not_needed
The MCP server, its tools, results and stdin caps: the chunk adds no MCP surface.
@task_item n=2 part=not_needed
The wrapper channel's frames, methods, fault codes and refusal order: verify's sixth run sends no channel frame.
@task_item n=3 part=not_needed
The wheel, human take and release rules: no wheel state changes.
@task_item n=4 part=not_needed
Send, paste and confirmation rows (typed-text normalisation, flight lock, send results): the probe types no prompt of its own beyond verify's ratified literals.
@task_item n=5 part=not_needed
Hook tiers, the hook-to-kind map, hook exit bodies and dialog hold and answer contracts: no hook behaviour changes.
@task_item n=6 part=not_needed
The board and list payloads, agents listing and session liveness: nothing listed or read for the board changes.
@task_item n=7 part=not_needed
viola run and revive start order, refusals and JSON documents, and the session-folder trust write: those belong to the next entry.
@task_item n=8 part=not_needed
Normalised event kinds, event data and the events log: the probe appends no event.
@task_item n=9 part=not_needed
The Hyprland seam and window features: no compositor call.
@task_item n=10 part=not_needed
Plugin install, the plugin's MCP config and pinned-copy placement: untouched.
@task_item n=11 part=not_needed
Fuzzing, mutation runs, release tooling and deploy rows: none run or change here.
