@task_prose part=session
A live test round on claude 2.1.325, run by evidence rigs outside the product: no IPC method, event kind, verb, flag, output line, refusal, hint, colour or glyph was added, changed or retired.
The one product edit is a doc comment: the dialog hold ceiling (DIALOG_HOLD_MAX) keeps its value, 3600 s, and loses its PROVISIONAL mark (measured live only to about 660 s); it is now measured at the ceiling itself, a wrapper expiry after 3 600 013 ms with the hook still waiting.
The founder's word on the ceiling is hold: stands; the 3605 s backstop stands unchanged.
Also measured live: claude 2.1.325 stamps 21 of 21 relied rows on a test home only (the live home is not stamped); the real CLI erases its input box on DEL, and a send after viola release read send-issued, prompt-submitted, send-confirmed with an empty box.
Measured too: the question dialog stays drawn on the agent's screen after the wrapper's expiry; the user's status row sits at the same place, with the same hash, wrapped and unwrapped.
Matrix v2-03 (the wrapped start's first wheel lines) is not verified this round and is to be un-claimed; that is a ledger fact, not a design one.
@task_file n=1 path=crates/viola-core/src/lib.rs
what: the core library's shared constants, among them the dialog hold ceiling, how long a pending dialog is held for the human before the wrapper expires it (3600 s)
@task_item n=1 part=touches
Any row whose cell holds the dialog hold ceiling (3600 s) or marks it provisional, measured only to about 660 s: the value stands, and the provisional qualifier is now stale, since it was measured at 3600 s on claude 2.1.325.
@task_item n=2 part=touches
Indirect, through the dialog hold ceiling: rows over the dialog-pending display (the cocked strip, the DIALOG field of the flight-progress strip, arrival amber for dialog-pending only, the viola wait dialog form). Each is touched only where its cell names the hold bound or its provisional mark; the display itself did not change.
@task_item n=3 part=touches
The CLI verification state rows (verified, unverified-cli) and the unverified-cli refusal hint: touched only where a cell names a stamped or newest verified CLI version. claude 2.1.325 stamped 21 of 21 relied rows on a test home only, and the live home is not stamped for it.
@task_item n=4 part=touches
The send readback rows (prompt send, the send readback mirror read back, no optimistic send confirmation): touched only where a cell marks the release-then-send path, or the DEL erase of the input box, as unmeasured on the real CLI. Both were measured live this round and confirmed: send-confirmed with an empty box after viola release.
@task_item n=5 part=touches
The refusal hint for not-delivered input-not-ready: touched only where a cell treats input-not-ready as a residue the human must clear. The round read it as a plain refusal, with no box stood and nothing left behind.
@task_item n=6 part=untouched
Colour palette, typography, spacing, depth, border radius and iconography: no colour token, glyph, separator, column or width moved.
@task_item n=7 part=untouched
The viola list board and table, the bay line, NAME truncation, unwrapped rows and the wheel column: the unwrapped-name rows share words with the round's wrapped and unwrapped status-row reading, but that reading is the agent's own status row, a different fact.
@task_item n=8 part=untouched
Exit codes, exit 21 causes, revive, run and link outputs and refusals, strict-modes refusals, catch sites and wrapper fault details: no code, cause, message or order changed.
@task_item n=9 part=untouched
The viola verify output and refusal rows, the stamps.json update and the unclean-fixture refusal: a live recording was refused whole on the standing absolute-path code because a rig's test home was named home. The rows already state it, and nothing in them changed.
@task_item n=10 part=untouched
The command names, the help verb groups, the absences (web surface, rendered page, tape), brand identity, anti-patterns and the self-validation checks: no name was taken or freed and no surface was added.
@task_item n=11 part=untouched
Folder-trust dialog handling: the rig typed the trust keys into its own pty. viola itself types into no trust dialog, so no viola output or rule moved.
