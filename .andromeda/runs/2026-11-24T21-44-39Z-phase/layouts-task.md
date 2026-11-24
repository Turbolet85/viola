@task_prose part=chunk
Changes the folder where viola adopt starts its resumed claude child, so the CLI finds the adopted session (today the caller's current directory).
Adds what viola adopt says and records when the resumed child ends at once after the start: a closed word, an exit code, a hint and an output line beside the existing adopt refusals, an event and a log line, and whether the instance directory and the name are released after it.
If the fix moves where the R5 live-reading script types viola adopt, that script changes with it, and its rehearse form reaches the changed path with no claude.
Re-reads, with no live start: what revive and up use as the resume folder and where they take it from, what the wrapper kept of the two red children, and what the capability ledger holds about a resume from another folder.
@task_file n=1 path=src/cmd/adopt.rs mark=edits
what: the viola adopt verb, adopting a session by id under a name and starting its resumed claude child — edits: the spawn folder of the adopt start, and the handling of a child that ends at once
@task_file n=2 path=scripts/r5-adopt.sh mark=edits
what: the R5 live-reading script for viola adopt, with its live arm and its rehearse form — edits: the folder the command is typed in, and the rehearse path
@task_item n=1 part=needs
The viola adopt verb's contract (viola adopt SESSION_ID --name NAME): its passed start, its refusals, and the new early-exit outcome added beside them.
@task_item n=2 part=needs
The CLI exit codes table: the early end of the resumed child must take a closed exit code from it, and its --json form must carry that code.
@task_item n=3 part=needs
The CLI refusal line form and the shared refusal word column: the new early-exit word is a closed word written in that line form and aligned in that column.
@task_item n=4 part=needs
A hint for every refusal cause: the new early-exit outcome carries its own hint, as the last stderr line, naming the next verb to run.
@task_item n=5 part=needs
Hints never quote text or name paths: the early-exit hint must not quote the child's output nor name the spawn folder or any session file path.
@task_item n=6 part=needs
Results on stdout, the rest on stderr: the early-exit word, output line and hint go to stderr; nothing of a failed adopt reaches stdout as a result.
@task_item n=7 part=needs
CLI --json mode as one document: under --json the early-exit outcome is a single refusal document with its code, never mixed with human lines.
@task_item n=8 part=needs
Human output is a contract and words carry every state: the early-exit line is fixed wording pinned by tests, its state carried by a word and never by colour alone.
@task_item n=9 part=needs
The resume folder as revive and up use it: the recorded working directory of the snapshot, and the refusal when that recorded directory is missing; adopt has no such record, and any folder it chooses must be stated the same way.
@task_item n=10 part=needs
Release after an early exit: whether the instance directory and the ViolaName are freed when the child ends at once, so a later adopt or run under that name is not refused as already live or squatted.
@task_item n=11 part=needs
The recorded trace of an early exit: the event and the log line written when the resumed child ends at once, holding its exit code and time to exit.
@task_item n=12 part=needs
A CLI target is always a ViolaName, never case-transformed: the --name given to adopt is kept as given across the early-exit path and its output line.
@task_item n=13 part=needs
No interactive prompt, no spinner, no progress glyph: an early exit is reported once, after the child ended, with no waiting animation and no question asked.
@task_item n=14 part=needs
The rehearse form of the R5 script reaches the changed adopt start with no real claude: proofs of this chunk run with a stand-in child and never read the operator's real Claude config.
@task_standing n=1
convention: every new refusal or outcome word gets its closed exit code, its hint as the last stderr line, and its --json document form
home: surface-cli
@task_standing n=2
convention: every new human output line is fixed wording, uncoloured in plain modes, and pinned by a test of its exact text
home: surface-cli
@task_item n=1 part=not_needed
The web surface: viola renders no page, and this chunk adds none.
@task_item n=2 part=not_needed
The viola list board and its BAY line, columns, truncation, colour depth and links block: the chunk changes no board row nor list output.
@task_item n=3 part=not_needed
viola send, its readback mirror, its read-back lines and its refusals before a request: adopt sends no prompt text.
@task_item n=4 part=not_needed
viola wait, viola last and viola answer output: the chunk adds no wait outcome, message read or dialog answer.
@task_item n=5 part=not_needed
viola verify output and refusals: the chunk adds no verify step nor fixture check.
@task_item n=6 part=not_needed
The viola mcp tools and their results: adopt is not an MCP tool and the chunk adds none.
@task_item n=7 part=not_needed
viola link and unlink output, the wheel verbs pause and release, and viola switch output: none of these verbs is touched.
@task_item n=8 part=not_needed
viola plugin install and viola open outputs: the chunk does not reach plugin install nor window opening.
@task_item n=9 part=not_needed
CLI token rendering of colour, spacing, radius and motion tokens: the early-exit line adds no styled token.
