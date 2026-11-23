@task_prose part=chunk
Every session viola starts names its model and its effort: a start (viola run, and the start arm of viola revive) hands its claude child the pair the operator gave, else viola's configured default pair, on every spawn.
The pair is recorded per instance as a closed set of values, beside the lineage record in the events log, the snapshot and its replay.
viola revive replays the recorded pair on its resume, so the resumed session runs on the pair its instance recorded.
The CLI channel that carries model and effort to claude, the record's place and shape, and what a resume does with the pair today are measured first at P3; open, up and adopt are not built here.
@task_item n=1 part=needs
The viola run and viola revive argument surface: new arguments naming a model and an effort, tested as a subprocess, with negative cases for bad or out-of-set values on the CLI and env vectors.
@task_item n=2 part=needs
The claude child spawn: the argv or env viola hands the child gains the model and effort pair on every start and every revive, with the env strip of CLAUDE* names still applied and the child given nothing of viola's beyond what the rules allow.
@task_item n=3 part=needs
viola run start ordering and settings: the pair is resolved and checked before the child is spawned, the start's events precede the child spawn, and the settings handed to the child stay as the start contract pins them.
@task_item n=4 part=needs
The process-start record for the claude child: what it logs of the child's spawn (env kept, env stripped) where the pair is passed through the environment.
@task_item n=5 part=needs
viola's configured default pair: a config key for the default model and effort, where a missing or bad value falls back to its default rather than failing the start.
@task_item n=6 part=needs
The per-instance record of the pair as a closed set of values: a closed enum changes only by a logged decision, and an unknown value read back is refused or skipped, never guessed.
@task_item n=7 part=needs
The instance events log and snapshot: a new event or event data keys carrying the pair, the snapshot fields holding it, and snapshot replay from the log recovering the pair in its fixed field set; every snapshot reader goes through read-or-replay.
@task_item n=8 part=needs
Forward compatibility: an events log or snapshot written before this chunk, with no pair recorded, still reads and replays; unknown lines and fields are tolerated and counted, never failed.
@task_item n=9 part=needs
viola revive: the resume replays the recorded pair, its preflight order and refusal codes and exit codes stay as they are, and a revived child runs on the recorded pair rather than the default.
@task_item n=10 part=needs
viola list, list --json and the MCP list tool: if the pair is shown, CLI and MCP return identical payloads, the list --json items and human board columns carry it, and human-mode output escapes control characters.
@task_item n=11 part=needs
The capability ledger: an undocumented claude behaviour the pair relies on (the flag, env name or settings key) is a ledger row with its own verify probe, and the CLI version gate needs every relied row.
@task_item n=12 part=needs
The fake agent stands in for claude: its argv-only command line, its unknown-argument handling and its receipt (argv and env) must accept and record the model and effort the spawn hands it, so tests read the pair from the receipt and never from the child's screen.
@task_item n=13 part=needs
Recorded claude fixtures and env names: a new env name or flag the pair uses is recorded per CLI version and the fake agent is held to the recorded fixtures.
@task_item n=14 part=needs
The user's own claude settings stay byte-identical: the chunk writes nothing to them, and tests never read the host's real claude sessions or home.
@task_item n=15 part=needs
Error and log hygiene for the new values: refusal and error bodies for a bad pair are literal and sanitized, and structured logs carry no secrets.
@task_standing n=1
convention: Every new integration or E2E test creates its own fresh private home and never reads the host's claude state.
home: 5-integration-test-strategy · integration-test-strategy
@task_standing n=2
convention: Every new test runs against the fake agent, never the real claude CLI in CI, and asserts on logged events and receipts, never on screen content.
home: 2-test-strategy · test-strategy
@task_standing n=3
convention: Every new test oracle is a literal, with table cases in rstest and no sleep or retry synchronisation.
home: 11-test-anti-patterns-never-do-these
@task_standing n=4
convention: Every new test function follows the test naming rule, unit tests stay inline, and root integration binaries carry their surface prefix.
home: 2-test-strategy · test-strategy
@task_standing n=5
convention: Every new fixture or synthetic input is scrubbed of real data and carries the canary; on-disk product state is produced by viola, never hand-written.
home: 7-test-data-fixtures · test-data-fixtures
@task_standing n=6
convention: Every new log line is structured JSON with its event, level and corr fields as the log format owns them.
home: log-format
@task_item n=1 part=not_needed
Send, paste and send-confirmation rows: the chunk sets the session's model and effort at spawn and does not change how text is typed into or confirmed by the child.
@task_item n=2 part=not_needed
Dialog hook decision bodies, viola answer and the dialog hold: no dialog is raised or answered differently by naming a pair.
@task_item n=3 part=not_needed
viola wait, viola last and turn-ended wake rows: waking and reading the newest turn do not touch the pair.
@task_item n=4 part=not_needed
Wheel and viola release rows: who holds the wheel is untouched by model and effort.
@task_item n=5 part=not_needed
Wrapper channel link and unlink methods and their JSON-RPC error codes: the pair is recorded at start, not over the channel's link methods.
@task_item n=6 part=not_needed
Hook performance budgets and the hyperfine gate: the chunk adds no work to any hook path.
@task_item n=7 part=not_needed
Switching a running session's pair: that is a later entry, and nothing of it is built or tested here.
