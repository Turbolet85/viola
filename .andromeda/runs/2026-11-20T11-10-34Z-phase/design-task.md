@task_prose part=chunk
A question dialog whose first raise reaches viola through the PermissionRequest hook is answered, on the driver's answer carrying the answers, with an allow decision body whose updatedInput carries those answers, where today that question stays null to the human.
A new relied capability-ledger row in the Claude agent crate for that body, with its viola verify probe; the dialog decision stays gated on a verify stamp on which every relied row counts.
The CLI versions are re-stamped against the grown relied set (which versions and which home is said at P3, on the founder's word).
Capability v2-20: a question first raised by a permission request is answered by the driver, never left to the human; the new row has its probe and the re-stamped versions pass it.
@task_file n=1 path=crates/viola-agent-claude/src/dialog.rs mark=edits
what: the Claude agent's dialog answer mapping and the hook decision body it emits — edits: the question answer mapping, the allow decision body with updatedInput
@task_file n=2 path=crates/viola-agent-claude mark=edits
what: the Claude agent component holding the capability ledger — edits: the ledger's relied rows and their count
@task_file n=3 path=src/cmd/verify mark=edits
what: the viola verify verb and its probes, the verify stamp's row set — edits: a new probe, the stamp's row set
@task_file n=4 path=src/cmd/hook mark=edits
what: the viola hook verb and the run wheel's dialog relay — edits: the PermissionRequest path for a question
@task_file n=5 path=fixtures/claude mark=edits
what: the per-version CLI fixtures and the fake agent's PermissionRequest question script — edits: the question script, the per-version fixtures
@task_file n=6 path=crates/viola-mcp
what: the MCP answer tool, reached only if the answer's shape must change
@task_item n=1 part=needs
The viola answer verb and the MCP answer tool: the driver's answer carrying the question's answers, its printed result, and any change to its shape.
@task_item n=2 part=needs
The hook verb's PermissionRequest decision: an allow body whose updatedInput carries the driver's answers, its exit code and its stderr staying as the hook verb's contract sets them.
@task_item n=3 part=needs
The hook verb's colour decision: hook output carries no colour or styling.
@task_item n=4 part=needs
The dialog-pending state of a question raised by a permission request: the session shows as holding a dialog while it waits for the driver's answer, and the DIALOG field of its strip reads it.
@task_item n=5 part=needs
The viola wait dialog form: a question dialog raised through the PermissionRequest hook is reported to the driver as a dialog it can answer.
@task_item n=6 part=needs
The CLI verification state (verified or unverified-cli): the dialog decision stays gated on a verify stamp on which every relied ledger row counts, and the unverified-cli refusal and its hint when the stamp lacks the new row.
@task_item n=7 part=needs
viola verify output for the new probe: its step line, its fail word, the summary line and its count of rows, and the stopped or refused run forms.
@task_item n=8 part=needs
viola verify refusals and the stamps.json update: the re-stamp of the CLI versions writes the grown relied set, and an update refusal keeps its fixed form.
@task_item n=9 part=needs
The not-delivered unknown-dialog refusal and its hint: an answer to a dialog viola cannot map stays refused, never guessed.
@task_item n=10 part=needs
The human always wins: the human at the keyboard can answer the dialog at any moment, a refusal reaches automation only, and an expired dialog hold keeps the conservative fall-back.
@task_item n=11 part=needs
viola stays mechanism: it carries the driver's answers and never chooses them; option and answer text is content, never a command, never quoted in a hint, and its control characters are escaped in any human output.
@task_item n=12 part=needs
Nothing announces success before confirmation: the answer verb confirms only once the decision body is delivered, with no optimistic confirmation.
@task_item n=13 part=needs
Errors and hints on the answer and verify paths name no internals and print no stack traces.
@task_standing n=1
convention: every new human output line is ASCII-only, plain words, with no glyph or emoji prefix
home: surface-cli
@task_standing n=2
convention: data on stdout, messages on stderr; a --json result is one stdout document
home: surface-cli · surface
@task_standing n=3
convention: each new verify step or refusal line follows the fixed refusal and hint forms, a hint in human mode only
home: surface
@task_item n=1 part=not_needed
viola revive output, its refusals and its --json exit-1 codes: the chunk changes no revive path.
@task_item n=2 part=not_needed
viola run start refusals and the exit-21 causes and their --json causes: the chunk adds no start or instance-reach refusal.
@task_item n=3 part=not_needed
viola link and unlink output and their --json results: pairing is not reached.
@task_item n=4 part=not_needed
Send readback, the readback box and the viola send mirror: the chunk answers a dialog, it sends no prompt or slash command.
@task_item n=5 part=not_needed
viola last output and the viola list table and board layout (columns, row order, NAME cut, skip counts, bay line): no column or board layout changes.
@task_item n=6 part=not_needed
Wrapper fault --json detail and peer reply screening: no new wrapper fault code or peer reply is added.
@task_item n=7 part=not_needed
The web-spa surface, the rendered-page absence, typography, spacing, depth and border radius: viola renders no page.
@task_item n=8 part=not_needed
Strict-modes refusals and the home-under-regular-file check: no home or instance-state check changes.
