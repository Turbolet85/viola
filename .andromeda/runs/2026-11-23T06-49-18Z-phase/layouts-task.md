@task_prose part=chunk
Every session viola starts names its model and its effort: a start (viola run, and the start arm of viola revive) hands its claude child a model and an effort on every spawn, the pair the operator gave, else viola's configured default pair.
The pair is recorded per instance as a closed set of values, beside the lineage record (the events log, the snapshot, the replay).
viola revive replays the recorded pair on its resume, so the resumed session runs on the pair its instance recorded.
The CLI channel for the pair, where and in what shape it is recorded, and what a resume does with model and effort today are measured at P3 first; the operator's forks are gathered there.
@task_item n=1 part=needs
The viola run argument surface: a model and an effort the operator may give at start, parsed with clap derive, beside the existing name and -- claude arguments.
@task_item n=2 part=needs
The viola revive argument surface and its start arm: a start by revive carries a pair too, and its resume replays the pair the instance recorded rather than a fresh default.
@task_item n=3 part=needs
The claude child spawn: on every spawn the child's argv or environment names the model and the effort, through the flag literals and child resolution of the claude agent component; the channel is the one P3 measures, never assumed.
@task_item n=4 part=needs
viola's configured default pair: a start given no pair runs on it; the default lives in viola's own configuration (a default pair key in config.json if the fork lands there).
@task_item n=5 part=needs
The per-instance record of the pair as a closed set of values in the events log and the snapshot, written beside the lineage record and read back on replay.
@task_item n=6 part=needs
The passed start output of viola run and of viola revive, and their --json start documents, where a start now names its pair or where a refused pair must be reported.
@task_item n=7 part=needs
A refusal for a model or effort outside the closed set, if the chunk adds one: it owes a CLI exit code, a refusal line in the shared form, a hint naming the next verb as the last stderr line, and a --json error code.
@task_item n=8 part=needs
The viola revive preflight order and refusal shape, where a recorded pair that cannot be replayed must be ordered among the existing revive checks.
@task_item n=9 part=needs
viola list, viola list --json and the MCP list tool: whether the pair is shown is the operator's fork; if it is, the board columns, the --json document and the MCP list result carry it.
@task_item n=10 part=needs
viola --help and per-verb --help usage lines for run and revive, which change when the verbs gain a model and an effort argument.
@task_item n=11 part=needs
The user's own claude settings stay untouched: the chunk writes nothing to them, so switching a running session's pair later can keep them byte-identical.
@task_item n=12 part=needs
The capability ledger: a row with a verify probe for any undocumented claude behaviour the pair's CLI channel rests on.
@task_item n=13 part=needs
Tests prove the acceptance without a live claude: a start with no pair runs on the default pair, a start with a pair runs on it, and a resume runs on the pair its instance recorded.
@task_standing n=1
convention: Human output is a contract: every new or changed output line of a verb is pinned by a test of its exact text, plain modes included.
home: surface-cli
@task_standing n=2
convention: --json on every data verb: a new field or error code joins the one typed --json document of the verb and is pinned by a test.
home: surface-cli
@task_standing n=3
convention: Every refusal cause has a hint naming the next verb, last on stderr, quoting no text and naming no path.
home: surface-cli
@task_item n=1 part=not_needed
The web surface: viola renders no page, and this chunk adds none.
@task_item n=2 part=not_needed
viola send and its outputs (issue line, read back, word column, paste-cut and input-not-ready hints, send refusals): a send to a running session is untouched by the pair chosen at start.
@task_item n=3 part=not_needed
viola wait, viola last and viola answer outputs and their MCP tools: turn and dialog reading do not depend on the model or effort.
@task_item n=4 part=not_needed
The wheel and handoff verbs (pause, release) and their spoken phraseology: switching a running session's pair belongs to a later chunk.
@task_item n=5 part=not_needed
viola link and unlink output: the pair is recorded beside the lineage record, but linking and its printed names do not change.
@task_item n=6 part=not_needed
viola plugin install and its outputs and refusals: the plugin is not reached by a start's model and effort.
@task_item n=7 part=not_needed
The CLI token rendering of colour, spacing, radius and motion tokens and the readback mirror: no new styling is introduced by naming a pair.
@task_item n=8 part=not_needed
Deep links or URL coupling in the CLI: nothing of the pair is addressed by a link.
