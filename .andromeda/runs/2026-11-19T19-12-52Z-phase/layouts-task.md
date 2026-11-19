@task_prose part=chunk
Adds /context to the compiled list of local commands a send confirms by a post-condition: one more local-command stdout tag in the session transcript after the send's own pre-paste end, the same way /mcp is confirmed today.
Adds a capability-ledger row for that post-condition on /context, with its viola verify probe (a compiled-literal paste in an interactive verify run), stamped live on the installed claude 2.1.328 in the implement/operator legs, never in the phase.
A send of /context then confirms by that post-condition, or answers ok with confirmed false and detail unconfirmable where the version has not measured it; the fake agent rehearses the new paste and post-condition so CI proves the path with no live start.
Carries two open readings: the placeholder hint's cell attributes read on every drawn hint cell past the cursor's cell against the typed text's cells, attributes only; and an intermittent rig rehearsal red acted on only at its next red, by reading its printed summary and kept frame before mending the wait.
@task_file n=1 path=crates/viola-agent-claude/src/ledger.rs mark=edits
what: the capability ledger of the claude agent adapter: the compiled local-command list with each command's delivery post-condition, and the ledger rows with their verify probes and stamps — edits: the local-command list (LOCAL_COMMANDS), its /context entry with the CommandStdout post-condition, and a new ledger row for it
@task_item n=1 part=needs
viola send <target> with the text /context: a local command whose delivery is confirmed by a post-condition read from the session transcript, not by a turn.
@task_item n=2 part=needs
The send outcome for /context: ok with confirmed true when the stdout-tag post-condition is seen, or ok with confirmed false and detail unconfirmable where the post-condition is not measured on the running CLI version (the delivery-confirmation rule).
@task_item n=3 part=needs
The viola send human output for a local command: the read back and issue lines as already given for /clear, /compact and /mcp, and what the read back for /context is.
@task_item n=4 part=needs
The viola send --json typed document for a local-command send: the ok document, its confirmed field and its unconfirmable detail.
@task_item n=5 part=needs
The readback mirror line and readback vocabulary state words a local-command send prints, including the unconfirmable word.
@task_item n=6 part=needs
The send text refusals before any request (empty text, control character) still apply to /context, and the wrapper repeats them.
@task_item n=7 part=needs
The viola verify probe of the new row: a compiled-literal /context paste in an interactive verify run, read on the step line, failing row, summary, exit 0 / exit 1, --record and stop outputs of viola verify.
@task_item n=8 part=needs
The viola verify refusals that the new interactive probe passes through: claude CLI not found, CLI version unreadable, viola home not private, recorded fixture not clean, verify stopped.
@task_item n=9 part=needs
The stamp of the row: stamped live on claude 2.1.328; whether the row is relied on (every later binary reading every stamped version unverified until re-stamped) is closed at P3.
@task_item n=10 part=needs
The mcp send tool and its tool results (success, refusal) carry the same send outcome for /context as the CLI send.
@task_item n=11 part=needs
Indirect: the post-condition mechanism (exactly one stdout tag for /context) rests on one unverified reading and must be proven by the verify probe, never assumed in code or docs.
@task_item n=12 part=needs
Indirect: no live start and no read of the operator's real claude config in the phase; the fake agent's rehearsal is how CI proves the /context path.
@task_item n=13 part=needs
Indirect: no pinned-product rebuild, replace or re-pin, and no documentation figure (counts, member lists, versions) added for the new command or row.
@task_item n=14 part=needs
Carry: the placeholder hint's cell attributes are read cell by cell past the cursor's cell against the typed text's cells, attributes only and never the text; its keys are the founder's word.
@task_item n=15 part=needs
Carry: the intermittent rig rehearsal red that reads the box empty after x and del gets no retry rule and no loosened entry; at its next red the printed rig summary and the kept after-del frame name the cause before the wait is mended.
@task_item n=16 part=needs
Exit codes and the --json form a send of /context returns, including instance unreachable (exit 21) and its detail when the wrapper is absent.
@task_standing n=1
convention: every new or changed human output line keeps the CLI's output contract: results on stdout and the rest on stderr, plain ASCII in non-TTY modes, words carrying every state, no spinner, banner or progress glyphs, the hint last on stderr.
home: surface-cli
@task_standing n=2
convention: every new --json result is one document on stdout, with the shared exit-code --json form.
home: surface-cli
@task_item n=1 part=not_needed
The web surface: viola renders no page.
@task_item n=2 part=not_needed
The viola list board, its BAY line, columns, colour depth, truncation and --json document: a send of /context adds no board column, row or state.
@task_item n=3 part=not_needed
viola run and viola revive starts, their preflight order, refusals and --json error codes: the chunk starts no instance and changes no start.
@task_item n=4 part=not_needed
viola plugin install and viola link / unlink output: the chunk installs and links nothing.
@task_item n=5 part=not_needed
viola wait output, turn-ended and dialog words, and viola answer output: a local command is confirmed without a turn or a dialog.
@task_item n=6 part=not_needed
Wheel and handoff verb output (pause, release) and viola last output: the chunk changes no wheel, handoff or message read.
@task_item n=7 part=not_needed
CLI token rendering (colour, spacing, radius, motion, roles) and the --help usage and verb groups: no new verb, flag or styled token.
@task_item n=8 part=not_needed
CLI deep links and URL coupling: absent and untouched.
