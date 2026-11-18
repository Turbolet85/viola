@task_prose part=chunk
Builds the product write of the CLI's per-project trust record at a wrapped start: at viola run and viola revive (their one shared start), after the version gate and before the spawn, on a verified CLI, when no record already trusts the child's folder read the CLI's way.
viola writes only projects[<key>].hasTrustDialogAccepted (the CLI's own key) into the one Claude config file the child will read, under the CLI's lock when free, by atomic replace, read back, at most 3 attempts.
A refused file form or a lost record writes nothing and leaves the dialog to the human, viola typing nothing; one closed word per start reaches the run log, nothing of the file anywhere.
Every refusal and skip gets a case proving nothing was written; the live proof S1 (viola run in a fresh untrusted folder under the rig's private config) and its control S2 (a plain claude) are asked at the end of P3.
Reuses the writer the trust-record ledger row built rather than adding a second; the measurement that no flag or setting trusts a folder is read, not re-derived.
@task_file n=1 path=crates/viola-agent-claude/src/trust.rs mark=edits
what: the trust record's shape and the key rule (the git root inside a repository, else the cwd) — edits: the trust reading the CLI's way, the exact key then the parent walk
@task_file n=2 path=crates/viola-agent-claude/src/trust_record.rs mark=edits
what: the trust-record file write: lock, atomic replace keeping the file's mode, the refusals and their closed code words — edits: the read-back and the retry of at most 3 attempts, the refusals for an absent, unreadable, linked, unexpected or oversize file
@task_item n=1 part=needs
The viola run start and the viola revive start, their one shared start: the trust write sits after the version gate and before the spawn, so the start's preflight order gains a step that refuses nothing.
@task_item n=2 part=needs
The passed start output of viola run and viola revive stays as it is: the write adds no line to stdout or stderr, the file and its path reach no output, only one closed word per start reaches the run log.
@task_item n=3 part=needs
The write never refuses a start: a refused file form, a lost record or a skip adds no exit code, no refusal line and no hint; the start proceeds and the dialog is the human's.
@task_item n=4 part=needs
The bound R2: viola writes only projects[<key>].hasTrustDialogAccepted for one folder, changes no other key, removes no entry, writes nothing for the home folder, and never creates the file.
@task_item n=5 part=needs
The file the child reads: <CLAUDE_CONFIG_DIR>/.claude.json when viola's home keeps that variable for the child (claude_env_keep), else the user's ~/.claude.json — viola reading that one environment value for that one purpose.
@task_item n=6 part=needs
The race R3: take the CLI's own lock when free, replace atomically, read the record back before the spawn, at most 3 attempts; a record still lost leaves the dialog to the human; the residual of a concurrent write lost in the window is named.
@task_item n=7 part=needs
Trust read the CLI's way: the exact key, then the parent walk (up to the git root inside a repository, to / outside one), so no record is written for a folder a parent already trusts.
@task_item n=8 part=needs
On a verified CLI only: a home whose stamp predates the 23rd row reads unverified, so run on it is transport-only and writes no trust record until its next live verify; the version gate's verified reading is what the write acts on.
@task_item n=9 part=needs
The refusal cases: file absent, unreadable, a link, not the JSON expected (not an object, projects not an object, the entry not an object), oversize past MAX_FRAME, a lost race; and the skips: unverified CLI, a folder already trusted by exact key or parent, the home folder — each proving the file's bytes unchanged, no temp sibling, no lock left.
@task_item n=10 part=needs
The fake agent's trust decision read from the Claude config file, so an e2e case of the write can fail when viola's write is gone; a change to its screens or trust reading puts cli_verify in the gate block.
@task_item n=11 part=needs
The run log's closed word per start: a fixed word from a closed vocabulary, carrying no file content, no path and no quoted text.
@task_item n=12 part=needs
Live starts S1 and S2 by shape at the end of P3: the folder, the rig's private config with one other project's entry as sentinel, the 23-row stamp, the browser-tools key seeded, the count per CLI version, every byte the rig typed including the stop (Ctrl-C into the rig's pty, never while a dialog stands); S1 red on a trust first screen, a sentinel differing by a byte or a written record not exactly the one key; S2 red when the first screen does not read trust; both red on a missing teardown word.
@task_item n=13 part=needs
Each control's --timeout named in the plan's prose and fired with that flag, ids within 16 characters, holds meaning those cases ran and failed with no TIMEOUT line; each control says what the subject does when the guard is gone.
@task_item n=14 part=needs
A test that waits takes a bound under the 20 s lint; a test-support helper spawning an instrumented binary under a cleared environment runs it in a scratch directory.
@task_standing n=1
convention: each new log line is a closed word from a fixed vocabulary, quoting no text and naming no path
home: surface-cli
@task_standing n=2
convention: each new start case keeps results on stdout and the rest on stderr, and keeps the human output of the verbs it drives as their contract states
home: surface-cli
@task_item n=1 part=not_needed
The web surface: viola renders no page and this chunk adds none.
@task_item n=2 part=not_needed
The viola list board, the BAY line, its columns, colour depth, truncation and its --json document: the write adds no field or state to the board.
@task_item n=3 part=not_needed
viola send, its readback mirror, its output lines and its refusals before any request: the chunk sends no prompt text.
@task_item n=4 part=not_needed
The viola mcp tools and their results: no tool is added or changed.
@task_item n=5 part=not_needed
viola wait, viola answer and viola last output: the chunk changes no dialog reading or turn ending as these verbs print them.
@task_item n=6 part=not_needed
viola link / unlink output and the wheel verbs pause and release: untouched by a start's trust write.
@task_item n=7 part=not_needed
CLI token rendering and SGR styling (colour roles, spacing, radius, motion tokens): the write prints nothing.
@task_item n=8 part=not_needed
viola plugin install and its output: the trust write is not reached by the plugin install.
@task_item n=9 part=not_needed
viola --help and per-verb usage: no verb, flag or usage line is added.
