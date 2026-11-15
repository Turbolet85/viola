@task_prose part=chunk
Makes a turn the claude CLI ends on an API error end as a turn: a turn-ended event carrying a cause code from a closed viola set, never the error text.
After that turn end the instance reads idle on viola list and the board, the wheel's turn-running state clears as a Stop clears it, a wait on the turn end returns, and the next viola send is accepted with no pause and release.
Adds the hook registration for the CLI signal that ends such a turn, its payload read in the Claude agent crate, the neutral cause set in the core crate, and the fake agent's ability to emit it.
Adds a test driving the API-error end through the fake agent and recorded or synthetic payloads, failing on the tree before the fix.
Re-reads at P3 from the code which state reads turn-running while the board reads idle, and re-measures the ending mechanism from the code and the recorded fixtures.
@task_file n=1 path=plugin/hooks/hooks.json mark=edits
what: the plugin's hook registrations for the claude CLI's hook events — edits: the registration for the CLI signal that ends a turn on an API error
@task_file n=2 path=src/run/send/slot.rs mark=edits
what: the run pump's send slot, the wheel's turn-running state that gates the next send — edits: the turn-end path
@task_file n=3 path=src/run/wait.rs mark=edits
what: the run pump's wait on a turn end, what wakes a waiter — edits: the turn-end path
@task_item n=1 part=needs
The turn-ended event and its data: a new cause key holding a member of a closed set, the CLI's error string mapped into it and an unknown value read as a fixed fallback member.
@task_item n=2 part=needs
The instance state after an API-error turn end: viola list and the board's STATUS read idle, consistent with the wheel's turn-running state now cleared.
@task_item n=3 part=needs
viola wait on a turn end: a waiter returns on an API-error turn end, its turn-ended output as for a Stop-ended turn.
@task_item n=4 part=needs
viola send after an API-error turn end: accepted, no longer refused not-delivered turn-running with exit 13.
@task_item n=5 part=needs
The wheel verbs pause and release are not needed to recover from an API-error turn end; no hint or output may route the operator to them for it.
@task_item n=6 part=needs
turn-ended stays a wait outcome, never a send outcome, the API-error end included.
@task_item n=7 part=needs
The error text (the last assistant message and error details on this path) never reaches an event, a snapshot, a log line, a hint or any CLI or --json output; hints never quote text.
@task_item n=8 part=needs
The cause code as a word: any place the cause shows in human or --json output uses the closed set's fixed tokens, words carrying the state, never the CLI's own string.
@task_item n=9 part=needs
The --json documents of list and wait stay one typed document each; an added cause field rides inside the existing turn-ended shape, unknown keys skipped by readers.
@task_item n=10 part=needs
A possible capability-ledger probe through viola verify for whether the CLI fires the API-error signal live, if P3 or P4 decides one, and its verify output rows.
@task_item n=11 part=needs
Test sockets go under the short /tmp root, and any home given to --record goes through the record home helper.
@task_item n=12 part=needs
Each new guard test carries a one-off remove-the-guard control named by the test's name alone; no mutation run.
@task_item n=1 part=not_needed
The web surface: viola renders no page and the chunk adds none.
@task_item n=2 part=not_needed
Board and list colour, token rendering, column widths, truncation and styling: the chunk changes a state value, not how the board draws it.
@task_item n=3 part=not_needed
viola revive and viola run start outputs and refusals, and their --json error codes: the chunk does not touch starting an instance.
@task_item n=4 part=not_needed
viola link and unlink output, viola plugin install output and refusals, and viola --help and per-verb usage text.
@task_item n=5 part=not_needed
viola answer output and the dialog-related wait outputs and DIALOG board cells: the chunk handles turn ends, not dialogs.
@task_item n=6 part=not_needed
The send readback mirror, read-back lines and send-confirmation wording, and the send text refusals before any request: send confirmation is out of scope.
@task_item n=7 part=not_needed
The frame verb exit-21 hints and instance-unreachable --json details: an API-error turn end leaves the instance reachable.
