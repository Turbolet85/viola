@task_prose part=chunk
Adds the 23rd relied capability-ledger row in the Claude adapter: the claude CLI honours a per-project trust record (hasTrustDialogAccepted under the CLI's own project key) that another process wrote into the config file its child reads.
Gives the row its viola verify probe: a sixth interactive verify run over its own fresh OS-temp dir that writes that trust record, expects the child's first settled frame to be no trust dialog, and removes only its own record after the run; a control with the record withheld must read the dialog.
Builds the trust-record writer once (lock when free, atomic replace, read-back, at most 3 attempts), placed where the later product write in viola run can reach it, and makes the fake agent read the record from the config file rather than decide trust from argv alone.
Re-runs the step-count sweep the sixth run moves over every spelling of the count, and stamps the row with one live viola verify (S0, plus one spare S0-2) on the 2.1.325 install under the live rig's private config; no capability is claimed.
@task_item n=1 part=needs
The capability ledger and the unverified-CLI gate: a 23rd relied row with its own verify probe; once it is relied every 22-row stamp reads unverified, so the stamps verify writes and the version gate's read of them are reached, while the tests' shared stamp and CI must stay green.
@task_item n=2 part=needs
The ratified verify child set grows from five to six: the sixth interactive run is spawned directly like the others, over its own OS-temp probe directory, and its effects stay in that directory except the one bounded record.
@task_item n=3 part=needs
Verify never types into or answers a dialog: the sixth run sends no dialog byte and pastes only its own ratified literals; it decides the trust dialog's absence only by reading the screen against compiled literals and signatures.
@task_item n=4 part=needs
The rule that viola writes nothing under the Claude home or into Claude's config (~/.claude.json): verify's own trust record becomes its one bounded exception; the product's write is not amended in by this chunk.
@task_item n=5 part=needs
The record's form and target: key is the cwd's git root inside a repository, else the cwd; hasTrustDialogAccepted alone; the file is <CLAUDE_CONFIG_DIR>/.claude.json when the child's environment keeps that variable, else ~/.claude.json.
@task_item n=6 part=needs
The writer's race rule and replace path: take the CLI's lock when free, replace atomically, read back, at most 3 attempts; it meets the rule that one shared helper replaces files and the rule that state readers never write.
@task_item n=7 part=needs
The writer's refusals: a config file absent, unreadable, a symlink, not the JSON expected or oversize leaves nothing written, and the probe's row reads a closed word for what it could not do, never pass; its read of the config is bounded and keeps the default JSON depth limit.
@task_item n=8 part=needs
Removal of verify's record: only its own record, never another project's, on every exit path the run reaches; the plan states what a SIGKILLed verify leaves and that the stop removes it; the probe directory is removed by its drop guard and probe artefacts stay owner-only and transient.
@task_item n=9 part=needs
Writer placement across the crate boundary: the Claude adapter does no file I/O and the state crate is Claude-free, so the record's JSON shape sits on the Claude side and the lock, atomic replace and read-back on a side both verify and the later run reach; the sync crates stay tokio-free.
@task_item n=10 part=needs
The probe must be able to fail: when the CLI does not honour the record the row reads fail; the product then never writes the record and the trust dialog stays the human's to answer.
@task_item n=11 part=needs
The fake agent's test seam changes: it decides trust by reading the record from the same config file and key the writer used, not argv --trusted-root alone, so both the post-condition and the control can fail on it; a change to its screens or trust reading puts the cli_verify job in the gate block.
@task_item n=12 part=needs
The child's environment: CLAUDE_CONFIG_DIR reaches the verify child only through the claude_env_keep config value, while the inherited CLAUDE credentials and parent identity variables stay stripped and the process-start line's env_kept field reports it.
@task_item n=13 part=needs
The live round: S0 is one live verify by path on the 2.1.325 install (7 claude starts: 1 print probe, 6 interactive) under the live rig's private config seeded by seed_config, child isolated from the user's config, credentials copy transient; keys: none into any dialog or box, the rig keying only its own pty; stop: Ctrl-C into its own pty or a no-key group kill; the real CLI runs only locally, never in CI.
@task_item n=14 part=needs
The founder's real ~/.claude.json is read only by the mark witness, and verify's summary prints what proves its record is gone after the run, holding no paths, pids or identity values.
@task_item n=15 part=needs
The owed rootOnly reading: what makes the CLI count only the git root's record is read from the installed CLI's strings where reachable without a start, else named unmeasured.
@task_item n=16 part=needs
Verify's accepted residuals widen by one run: the trusted-run risk, real-CLI transcripts and echo-probe residuals now also cover the sixth run and its pre-written record.
@task_item n=17 part=needs
Tests and controls: no mutation run; each control fired with a named --timeout; control and sweep ids at most 16 characters; a waiting test bounded under the 20 s lint; the session sets no timer and ends every process it starts.
@task_standing n=1
convention: each new test runs under its own viola home and removes only the directories it created
home: secret-management · security-anti-patterns
@task_standing n=2
convention: each new fake-agent behaviour is a test seam only under the fake-agent feature, compiled out of release builds
home: security-anti-patterns · input-validation
@task_standing n=3
convention: each new seam or environment read needs its log entry
home: security-anti-patterns-never-do-these
@task_standing n=4
convention: each new external reader is bounded by the read cap or named in the bounded-readers census
home: input-validation
@task_standing n=5
convention: each new error or refusal message is fixed text carrying no path, upstream text or identity value
home: error-handling
@task_standing n=6
convention: each new recorded screen fixture is scrubbed to signature rows and passes the fixture hygiene walk
home: data-protection
@task_item n=1 part=not_needed
The wrapper channel methods, channel frames and JSON-RPC error codes: the chunk adds no channel method and changes no frame.
@task_item n=2 part=not_needed
The MCP tools, MCP stdin cap and MCP isError contracts: the MCP server is untouched.
@task_item n=3 part=not_needed
Hook payload parsing, hook-decision details and hook capture: no hook behaviour changes.
@task_item n=4 part=not_needed
Send, answer and paste text validation and their refusal words: verify's sixth run pastes only its existing ratified literals and no typed-text rule changes.
@task_item n=5 part=not_needed
The revive and revive --json contracts, the start collision check and the run start's socket-directory refusal: nothing of viola run or revive changes here, and the product's own trust write belongs to a later entry.
@task_item n=6 part=not_needed
Peer euid checks, server verification before a frame and socket-directory placement: the chunk touches no IPC endpoint.
@task_item n=7 part=not_needed
Link and unlink, pause and release, the human-typing wheel and their event lines: untouched.
@task_item n=8 part=not_needed
The Hyprland recorder, terminal colour facts and viola list's board scan: untouched.
@task_item n=9 part=not_needed
The absences of network listeners, hosting, accounts, databases, webhooks and remote access: the chunk adds no such surface.
