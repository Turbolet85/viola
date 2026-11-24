@task_prose part=chunk
Adds a new CLI verb `viola adopt <session-id> --name <n> [--parent <p>]` that takes a plain claude session by its session-shaped id and starts a viola wrapper whose claude child resumes it, so it stands as a named instance with its lineage and is driven like any wrapped session.
It is built on the revive start path: the resume argv, the claude agents --json preflight with its closed session-live refusal, and the start shared with run.
A malformed id is refused with a closed word, and a session live in another claude is refused before any child is spawned. adopt never takes over a running process's terminal and never ends or signals a process it did not start (the conservative arm, provisional).
Folded CI item: it measures what two slow env-capture security-negative test cases wait on and what they cost locally and under coverage, against their kill line. It gives them a margin through their class or reduced work, never by raising the kill line for every test.
@task_file n=1 path=src/cmd/revive.rs
what: the viola revive start: its preflight order, the session-live check against claude agents --json, the resume argv and the start shared with run, which adopt builds on
@task_file n=2 path=.config/nextest.toml mark=edits
what: the test runner configuration holding the per-test kill line and test classes — edits: the class or override for the two env-capture security-negative cases
@task_item n=1 part=needs
A new entry in the CLI verb table for viola adopt <session-id> --name <n> [--parent <p>], with its usage line and per-verb --help usage.
@task_item n=2 part=needs
A new row in the viola --help verb group rows and usage line for the adopt verb.
@task_item n=3 part=needs
Refusal of a session listed live in another claude (the session-open-elsewhere / session-live refusal), with its refusal line, its hint and its --json error code, shared in form with viola revive.
@task_item n=4 part=needs
Refusal of a malformed session id with a closed word before any home read beyond the strict ones, in the same form as viola revive's malformed --id refusal.
@task_item n=5 part=needs
The preflight order of the start adopt reuses: strict-modes and home-tree checks, the session-live check, then the start; refusals come before any child is spawned.
@task_item n=6 part=needs
Start refusals shared with run and revive, human and --json: squatted name, socket directory refused, home tree refused, pinned hash mismatch, internal error, already live.
@task_item n=7 part=needs
The passed start output of adopt, modelled on the revive and run start output, and its --json document (one document on stdout in --json mode).
@task_item n=8 part=needs
Closed outcomes for a second adopt of the same id and for an adopt onto a name already held, each with its refusal word, hint and exit code.
@task_item n=9 part=needs
The --parent value: how a given parent is taken and what the default is when none is given, in line with viola open's --parent default.
@task_item n=10 part=needs
The CLI exit code table and its --json form: adopt's refusals map onto the existing exit codes, and any new cause gets a row.
@task_item n=11 part=needs
Every refusal cause gets a hint that is the last stderr line, names the next verb (for a live session, end it by hand and then adopt), never quotes text or names paths, and never names viola release.
@task_item n=12 part=needs
The CLI target and --name are a ViolaName that is never case transformed; --parent is a wrapped name.
@task_item n=13 part=needs
The CLI conventions the new verb's code must follow: flat verb plus target, clap derive parsing, results on stdout and the rest on stderr, no prompt, pager, spinner, banner or terminator word, plain output modes, and TTY detection through std IsTerminal.
@task_item n=14 part=needs
Window placement: adopt either opens no window and runs in the terminal it is typed in (as revive) or opens one (as open and up); P3 fork, and the adopt rows follow whichever arm is taken.
@task_item n=15 part=needs
The question of whether adopt is exposed as an MCP tool (conservative arm: CLI only, so no MCP tool row); P3 fork.
@task_item n=16 part=needs
The CLI catch site for an internal error in the adopt verb, human and --json.
@task_item n=17 part=needs
Human output is a contract: adopt's human lines are fixed wording, with the shared refusal word column and captions.
@task_standing n=1
convention: every new verb's human output and --json document are asserted as contracts by its tests: one document in --json mode, results on stdout, refusals and hints on stderr
home: surface-cli
@task_standing n=2
convention: every new refusal cause added by the chunk carries a hint row, an exit code row and a --json error code
home: surface-cli
@task_item n=1 part=not_needed
viola send output, the readback mirror and readback vocabulary, and send refusals: adopt sends no prompt text.
@task_item n=2 part=not_needed
viola wait output and viola answer output (dialog, turn-ended, timed out, answered): adopt waits on no turn and answers no dialog.
@task_item n=3 part=not_needed
Wheel verb output (pause, release) and viola link / unlink output: adopt changes no wheel or link.
@task_item n=4 part=not_needed
viola verify output and refusals, and viola plugin install output: adopt does not touch verification or plugin install.
@task_item n=5 part=not_needed
viola last output and the cursor: adopt reads no message from a session.
@task_item n=6 part=not_needed
The web-spa surface: viola renders no page.
@task_item n=7 part=not_needed
CLI token rendering of colour, radius, spacing and motion tokens: adopt adds no styled board element.
