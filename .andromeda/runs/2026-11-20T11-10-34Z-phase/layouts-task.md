@task_prose part=chunk
A question dialog first raised through the PermissionRequest hook is answered, on the driver's answer carrying the answers, with an allow decision body whose updatedInput carries those answers, where today that question stays null to the human.
A new relied capability-ledger row for that allow body is added, with its viola verify probe; the dialog decision stays gated on a verify stamp on which every relied row counts.
The CLI versions are re-stamped against the grown relied set (which versions and which home are said at P3).
Masters rows whose words the change reaches are searched and reworded: the answer path, the hook's decision words, allow, the dialog relay, the ledger's row list and count wording, verify's probes.
@task_file n=1 path=crates/viola-agent-claude/src/dialog.rs mark=edits
what: the agent adapter's dialog answer mapping — how a driver's answer becomes the hook decision body — edits: the dialog answer mapping, the PermissionRequest decision body for a question
@task_file n=2 path=crates/viola-agent-claude mark=edits
what: the agent adapter for the claude CLI, holding the capability ledger and its relied rows — edits: the capability ledger's relied rows and their count, the hook decision body
@task_file n=3 path=src/cmd/verify mark=edits
what: the viola verify command and its probes, which stamp a CLI version against the relied ledger rows — edits: a new probe, the verify stamp's row set
@task_file n=4 path=src/cmd/hook mark=edits
what: the hook command and the run wheel's dialog relay — the PermissionRequest path that holds a dialog for the driver — edits: the PermissionRequest path for a question
@task_file n=5 path=fixtures/claude mark=edits
what: recorded CLI fixtures per CLI version and the fake agent's PermissionRequest question script — edits: the fake agent's PermissionRequest question script
@task_item n=1 part=needs
The PermissionRequest path for a question dialog: a question first raised there is now answered with an allow decision whose updatedInput carries the driver's answers, instead of staying null to the human.
@task_item n=2 part=needs
The driver's answer to a dialog, through viola answer and the answer MCP tool: the answers it carries are relayed as given; viola never chooses them, and option and answer text is content, never a command.
@task_item n=3 part=needs
The output of viola answer when a question dialog is answered: the answered line and the not-delivered hints, and the unknown-dialog refusal, as they apply to a question raised through the permission request.
@task_item n=4 part=needs
The answer MCP tool's parameters and its success, refusal and invalid-params results, only so far as the answer's shape must change to carry a question's answers.
@task_item n=5 part=needs
viola wait output for a dialog and for an expired dialog: the human always wins, the human at the keyboard may answer the dialog himself at any moment, and an expired dialog hold keeps today's conservative fall-back.
@task_item n=6 part=needs
viola verify output with one more relied ledger row: the new probe's step line, the failing-row line, the summary and its row count, plain ASCII, the record form, and the exit 0 / exit 1 outcomes.
@task_item n=7 part=needs
viola verify refusals that still guard a re-stamp of a CLI version: claude CLI not found, CLI version unreadable, recorded fixture not clean, viola home not private, verify stopped.
@task_item n=8 part=needs
The dialog decision stays gated on a verify stamp on which every relied ledger row counts; a stamp short of the grown row set does not let the dialog be answered automatically.
@task_item n=9 part=needs
The DIALOG attention word and the amber colour that means a dialog only, on the board and the wait output, as a question dialog now leaves the dialog state through the driver's answer.
@task_item n=10 part=needs
Spoken phraseology for the dialog verbs and the readback vocabulary's state words, where a question answered through the permission request is reported.
@task_item n=11 part=needs
Human output is a contract: every line the chunk adds or rewords (verify step line, answer output) is fixed wording, with results on stdout and the rest on stderr, and a hint for every refusal cause that names the next verb and never quotes text or names paths.
@task_item n=12 part=needs
The --json form of viola answer and viola verify: one document, typed, on every data verb the chunk touches.
@task_item n=13 part=needs
The run verb's wheel: the dialog relay lives in the run wheel and the driver's answer reaches the wrapped claude through it.
@task_standing n=1
convention: every new viola verify probe prints one plain-ASCII step line in the fixed form, counts in the summary, and names its failing row when it fails
home: surface-cli
@task_standing n=2
convention: every new or reworded human output line is fixed wording with a --json twin in one typed document, results on stdout and the rest on stderr
home: surface-cli
@task_item n=1 part=not_needed
The web surface: viola renders no page, and this chunk adds none.
@task_item n=2 part=not_needed
The board's layout rows: column widths, name truncation, row ordering, rack separator, stale-row dimming, colour depth and the BAY line's cell order; the chunk changes no board column.
@task_item n=3 part=not_needed
viola revive and viola run start refusals and their --json error codes, the start preflight order and the passed start output; the chunk changes no start.
@task_item n=4 part=not_needed
viola link and unlink output, and viola plugin install output and refusals; the chunk touches neither verb.
@task_item n=5 part=not_needed
viola send output: the issue line, the readback mirror, the slash-command read backs, unable and unconfirmable, and the send refusals before any request; the chunk changes no send.
@task_item n=6 part=not_needed
The frame verb exit-21 hints and the exit-21 --json detail for an unreachable instance; the chunk adds no unreachable cause.
@task_item n=7 part=not_needed
The CLI's token rendering rows (spacing, radius, motion tokens, surface colours) and the styling module choices (SGR, TTY detection, clap derive); the chunk adds no styled element.
@task_item n=8 part=not_needed
viola last output and the cursor byte offset, and the wheel verb output for pause and release; the chunk changes neither.
