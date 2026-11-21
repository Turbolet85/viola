@task_prose part=chunk
Adds two relied capability-ledger rows to the claude agent component, each with its own viola verify probe: the harness prompt prefixes and the eleven-name identity floor.
The harness-prefix probe has a second session send into a verify child and reads the raw start of the prompt to tell the harness-origin prefix.
The identity-floor probe captures environment variable NAMES only, never their values, into whatever the probe writes.
The new rows are counted in a viola verify stamp from a live run in the rig's scratch home. The live home's stamp and the pinned product stay as they are.
It re-reads the facts the tree can show: the relied-row count, the stamped fixture sets and the default-version constants.
@task_file n=1 path=crates/viola-agent-claude/src/ledger.rs mark=edits
what: the capability ledger of the claude agent: its rows, the relied-row list and the stamp's relied-id set — edits: the relied-row list, the stamp's relied-id set, two new ledger rows and their verify probes
@task_file n=2 path=crates/viola-agent-claude/src/ledger mark=edits
what: the capability ledger's own test cases: checks, framing, runs and stamps
@task_file n=3 path=crates/viola-agent-claude/src/hook.rs mark=edits
what: the hook handling of the claude agent, which holds the compiled harness prompt-prefix literals and the hidden hook capture arm with its closed ids — edits: the harness-prefix literals, the capture arm's closed ids
@task_file n=4 path=crates/viola-agent-claude/src/lib.rs mark=edits
what: the claude agent component's root, which holds the identity floor of parent-identity environment names stripped from the child's environment — edits: IDENTITY_FLOOR
@task_file n=5 path=fixtures/claude
what: the recorded fixture sets of viola verify, one per CLI version, written by verify --record through its scrub and replayed by the fake agent in CI
@task_item n=1 part=needs
viola verify gains two steps, one for each new relied row. Each step prints the verify step line in its fixed form, and a failure prints the failing-row form.
@task_item n=2 part=needs
The viola verify summary and summary line count the two new relied rows, and so do the exit-0 and exit-1 outcomes they report.
@task_item n=3 part=needs
viola verify --record writes the new probes' captures into the recorded fixture set through its scrub. The --record output reports what it recorded.
@task_item n=4 part=needs
viola verify refuses a recorded fixture that is not clean. A captured environment value, or a name that carries private text, must never get past the scrub into a fixture.
@task_item n=5 part=needs
viola verify output stays plain ASCII. No environment value and no compiled screen literal from a probe ever reaches stdout or stderr.
@task_item n=6 part=needs
Hints never quote text or name paths. Any new verify hint or refusal for the new rows names the next verb only and never quotes an environment name's value or a prompt prefix.
@task_item n=7 part=needs
Human output is a contract. The wording of the new verify step lines is fixed and is held by the existing verify output checks.
@task_item n=8 part=needs
Results go to stdout and everything else goes to stderr. The new verify steps follow the same split as the existing ones.
@task_item n=9 part=needs
The viola verify refusals stay reachable with the new probes in place: CLI not found, CLI version unreadable, viola home not private and verify stopped. The stop output keeps its form when a stop lands during a new probe.
@task_item n=10 part=needs
The viola verify --help paragraph and usage still describe what verify does once the two new probes exist.
@task_standing n=1
convention: every new line of human output a verb prints is a contract, fixed in wording and held by a check
home: surface-cli
@task_item n=1 part=not_needed
The viola list board: its BAY line, columns, colour depth, truncation, stale rows and JSON document. The chunk adds verify probes and capability rows and changes no board row.
@task_item n=2 part=not_needed
The viola revive and viola run start contracts: preflight order, refusals, the --list rows and the JSON error codes. The chunk changes no start path.
@task_item n=3 part=not_needed
The viola link and unlink output and the viola plugin install output. The chunk does not touch linking or plugin installation.
@task_item n=4 part=not_needed
The viola mcp tools and their results, and the hidden mcp verb. The chunk adds no MCP tool.
@task_item n=5 part=not_needed
The wheel verb output for pause and release, and the viola wait output words. The chunk adds no wheel or wait wording.
@task_item n=6 part=not_needed
The CLI token rendering rows (roles, colours, spacing, radius, motion) and the board styling decisions. Verify output is plain ASCII with no styling to add.
@task_item n=7 part=not_needed
The web surface absence and CLI deep links. viola renders no page and the chunk adds no link.
