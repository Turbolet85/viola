@task_prose part=chunk
Every session viola starts names its model and its effort: viola run and the start arm of viola revive hand the claude child a model and an effort on every spawn, the pair the operator gave or else viola's configured default pair.
The pair is recorded per instance as a closed set of values, beside the lineage record (events log, snapshot, replay), and viola revive replays the recorded pair on its resume.
Re-measures at P3, before any design: the claude CLI channel for model and effort (flag, environment name or settings key), what a resume does with model and effort today, and where and in what shape the pair is recorded.
Writes nothing to the user's own claude settings; open, up and adopt do not exist yet, so their inheritance of the pair is not built here.
@task_item n=1 part=needs
The viola run argument surface (clap): an optional model and an optional effort given at start, under the flat verb, never through an interactive prompt.
@task_item n=2 part=needs
The viola revive argument surface and its start arm: the same model and effort inputs at a fresh start, and on a resume the pair the instance recorded.
@task_item n=3 part=needs
The child spawn of the claude CLI (argv and environment, the flag literals of the agent adapter and child resolution): every spawn carries a model and an effort, never left to claude's own default.
@task_item n=4 part=needs
viola's configured default pair: the value a start uses when the operator gives no model or effort, held in viola's own configuration (a config.json key if the operator's fork lands there).
@task_item n=5 part=needs
The per-instance record of the pair: a closed set of model and effort values written into the events log and the instance snapshot beside the lineage record, and read back on replay.
@task_item n=6 part=needs
Resume replay: viola revive resumes the session on the pair its instance recorded, so the resumed session runs on that pair and not on the default.
@task_item n=7 part=needs
A model or effort outside the closed set given at start: refused before any child is spawned, through the run and revive start refusals, the CLI refusal line forms and the CLI exit codes, with no path or pid in the refusal.
@task_item n=8 part=needs
The revive --json refusal codes and the run start refusals order, wherever a new refusal of the pair joins them.
@task_item n=9 part=needs
viola run stays silent while the child is wrapped: naming the pair adds no output once the child holds the terminal, and no --json document after it.
@task_item n=10 part=needs
The run and revive output when the child starts, and the run and revive --json result: one stdout document, data on stdout and messages on stderr, wherever the pair is reported.
@task_item n=11 part=needs
The CLI verification state (unverified-cli, verified) and the capability ledger: an undocumented claude behaviour the chunk relies on (the model and effort channel, resume behaviour) is a ledger row with a verify probe.
@task_item n=12 part=needs
The user's own claude settings stay byte-identical: the chunk writes nothing to them, so switching a running session's pair stays reachable for a later chunk.
@task_item n=13 part=needs
Whether the pair is shown on viola list, its --json and the MCP list is an operator fork; the board's columns, its --json document and the list rows are the surfaces such a fork reaches.
@task_item n=14 part=needs
Human-mode output of the new inputs and refusals: ASCII-only, control characters escaped, no internals in errors or hints, no hint under --json.
@task_standing n=1
convention: each new human-mode line or refusal a start or revive prints uses the fixed unable and hint forms, ASCII-only, on stderr, with no colour beyond the CLI's amber and dim
home: surface-cli · surface
@task_standing n=2
convention: each new field a --json result carries is part of one clean stdout document, never a second document
home: surface-cli · surface
@task_item n=1 part=not_needed
The send readback, the readback box and its CLI mirror: the chunk sends no prompt and confirms no send.
@task_item n=2 part=not_needed
viola wait output and its dialog and event forms: the chunk adds no wait.
@task_item n=3 part=not_needed
viola link and unlink output and their --json results: the chunk adds no link.
@task_item n=4 part=not_needed
viola answer, pause and release output and the pause/release control-transfer and handoff display: the chunk transfers no control.
@task_item n=5 part=not_needed
Dialog-pending shown as a cocked strip and the arrival amber reserved for it: the chunk adds no dialog state.
@task_item n=6 part=not_needed
viola last output: the chunk reads no last message.
@task_item n=7 part=not_needed
Peer reply screening and wrapper fault --json detail: the chunk adds no peer reply or wrapper fault code.
@task_item n=8 part=not_needed
A web page or web surface and a rendered page's tokens: viola renders no page.
@task_item n=9 part=not_needed
The hook verb's exit code and stderr: the chunk changes no hook.
@task_item n=10 part=not_needed
The self-validation protocol's squint, swap and readback-box signature tests: they judge the visual design, which the chunk does not change.
