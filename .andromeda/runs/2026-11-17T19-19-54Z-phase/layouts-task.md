@task_prose part=chunk
Adds the 23rd relied capability-ledger row in viola-agent-claude: the claude CLI honours a per-project trust record (hasTrustDialogAccepted under the CLI's own project key) that another process wrote into the config file its child reads.
Its viola verify probe is a sixth interactive verify run over a fresh OS-temp dir: verify writes the trust record for that dir, expects the child's first settled frame to show no trust dialog, and removes only its own record after the run, on every exit path.
The record writer (lock when free, atomic replace, read-back, at most 3 attempts) is built once, where the later product write can reuse it. The fake agent learns to read the record, so the probe and its withheld-record control can fail.
It re-runs the step-count sweep over every spelling of verify's count, and makes one live viola verify (S0, plus a spare S0-2) on the 2.1.325 install to stamp the row. It also re-reads the owed rootOnly trigger from the installed CLI.
@task_item n=1 part=needs
viola verify's step lines and step count: a sixth interactive run joins the ratified child set of five, so every printed count of steps and runs moves from the five-run set to six.
@task_item n=2 part=needs
viola verify's summary output: the summary must print what proves verify's own trust record is gone after the run.
@task_item n=3 part=needs
viola verify's failing-row output and exit 1: when the CLI does not honour the record, the new row reads fail, and verify exits as it does for any failing row.
@task_item n=4 part=needs
viola verify's row for a record it could not write (file absent, unreadable, a link, not the JSON expected, or oversize): nothing is written, and the row names what it could not do in a closed word, never pass.
@task_item n=5 part=needs
viola verify's --record output and the stamp it writes: once the 23rd row is relied, every 22-row stamp reads unverified, the 2.1.325 install's included, so the recorded stamp must carry the new row.
@task_item n=6 part=needs
viola verify's stop and the verify-stopped refusal: a stop (Ctrl-C) mid-run must still remove verify's own record, and the plan names what a SIGKILLed verify leaves behind.
@task_item n=7 part=needs
viola verify's exit 0 and plain ASCII output: the new row's line and its closed words keep verify's output plain ASCII, and a full pass still exits 0.
@task_item n=8 part=needs
viola verify's --help paragraph and usage, where they describe the interactive runs or the rows verify measures.
@task_item n=9 part=needs
viola verify's refusals before any run (CLI not found, version unreadable, viola home not private, recorded fixture not clean) still come first; the sixth run adds no new refusal ahead of them.
@task_item n=10 part=needs
The bounded exception to the rule that viola writes nothing into the user's claude config: verify's own trust record for its own temp dir is the only write, and the product's write is not amended in here.
@task_item n=11 part=needs
The record's form: the CLI's own key (the git root inside a repository, else the cwd), hasTrustDialogAccepted alone, written into CLAUDE_CONFIG_DIR's .claude.json when the child keeps that variable, else the user's ~/.claude.json.
@task_item n=12 part=needs
The writer's race rule: the CLI's lock when free, atomic replace, read-back, at most 3 attempts; the JSON shape belongs to the Claude side and the file work to a side both verify and the later viola run can reach.
@task_item n=13 part=needs
The probe must be able to fail: the post-condition (record written, no trust dialog) and its control (record withheld, the dialog shows) are both proved on the fake agent, which reads the record for exactly the config file and key the writer used.
@task_item n=14 part=needs
The capability-ledger gate: a relied row must have its viola verify probe, and the shared test stamp and CI must stay green with 23 rows.
@task_item n=15 part=needs
Each control's --timeout is named and fired with that flag; control and sweep ids are at most 16 characters.
@task_item n=16 part=needs
A change to the fake agent's screens or trust reading puts the cli_verify job in the gate block; there is no mutation run.
@task_item n=17 part=needs
The live start: one viola verify on the 2.1.325 install under the rig's private config, ledgered with the clock's time before it is made, naming keys: none and stop: for each start; a start refused for the token window is uncounted, and no timer is set.
@task_item n=18 part=needs
The founder's real ~/.claude.json is read only by the mark witness, and every process the chunk starts is ended.
@task_standing n=1
convention: each new test that waits takes a bound under the 20 s lint, and every process a booted test starts ends with that test
home: surface-cli
@task_standing n=2
convention: each new verify output line follows the verify output contract: plain ASCII, results on stdout and the rest on stderr, closed words for each row's state
home: surface-cli
@task_item n=1 part=not_needed
The web surface: viola renders no page, and this chunk adds none.
@task_item n=2 part=not_needed
viola run and viola revive starts, their refusals, their --json documents and their start output: nothing of run or revive changes here; the product's trust write belongs to a later entry.
@task_item n=3 part=not_needed
The viola list board, BAY line and list --json: the chunk does not change how instances are listed or shown.
@task_item n=4 part=not_needed
viola send, its readback mirror, its refusals and its output: the chunk sends no prompt through the product verbs.
@task_item n=5 part=not_needed
viola wait, viola last and viola answer output, and the frame-verb exit-21 hints and details: the chunk does not touch reaching a running instance.
@task_item n=6 part=not_needed
The viola mcp server, its tools and its tool results: the chunk adds no MCP tool or result.
@task_item n=7 part=not_needed
viola link and unlink output, the wheel verbs pause and release, and viola plugin install: none is reached by a verify probe.
@task_item n=8 part=not_needed
CLI token rendering (colour, spacing, radius, motion, roles): verify's output is plain ASCII with no styling.
