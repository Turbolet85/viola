@task_prose part=session
A live round on claude 2.1.325 read the dialog hold at its 3600 s ceiling: the wrapper expired the dialog after 3600013 ms with the hook still waiting and no human key during the hold.
On the founder's word the hold stands at 3600 s; its provisional mark (measured live only to about 660 s) is gone, and the ceiling now rests on one reading on one CLI version.
The only product change is the text of the hold ceiling's doc comment; no value, IPC method, event kind, flag, config key or CLI output changed.
The round also measured that the real CLI erases its input box on DEL, that 2.1.325 shows a folder-trust dialog with unnumbered options before any hook in a fresh cwd, and that the question dialog stays on the screen after the wrapper's expiry.
Matrix v2-03 (a wrapped start's first wheel lines) was not verified and is to be un-claimed; the rigs that drove the round are evidence scripts outside the product.
@task_file n=1 path=crates/viola-core/src/lib.rs
what: the core crate's shared constants, among them the dialog hold's ceiling DIALOG_HOLD_MAX (3600 s) that bounds how long the driver's pending dialog is held before the wrapper expires it
@task_item n=1 part=touches
decision/dialog-hold-bounds-the-driver: its state is provisional; the 3600 s ceiling is now measured live at the ceiling itself (expiry after 3600013 ms on claude 2.1.325) and stands on the founder's word, so the provisional mark leaves it.
@task_item n=2 part=touches
Any row whose cell holds the dialog hold's ceiling or its qualifier (3600 s, provisional, measured only to about 660 s) — the value is unchanged at 3600 s, but the qualifier moved from provisional to measured.
@task_item n=3 part=touches
contract/config-json/dialog-hold-s: the round ran dialog_hold_s 3600 live; the key and its bound are unchanged, but a cell naming the ceiling as unmeasured or provisional is now stale.
@task_item n=4 part=touches
contract/event-line/dialog-expired: the wrapper's dialog-expired line was read live at the ceiling (source wrapper, one hour after the hook's question); a cell calling that expiry unmeasured is stale, the line's form is unchanged.
@task_item n=5 part=touches
decision/time-bounds-fall-on-the-driver and decision/time-bounds-never-on-human: the live reading confirms the hold bounds only the driver — no human key was needed during the hold and the question dialog stayed on the screen after the expiry.
@task_item n=6 part=touches
absence/user-time-limit and decision/no-unwarned-timeouts: indirectly, through the hold ceiling; the expiry ends the driver's wait, not the human's, which the live reading bears out.
@task_item n=7 part=touches
decision/live-readings-never-gate: this round's live readings were recorded as fired and gate no build; the hold reading moved a qualifier, not a gate.
@task_fact n=1 slug=cognitive-accessibility
fact: After the wrapper expires a pending dialog at the hold's ceiling, claude's question dialog stays on the screen for the human to answer; the expiry ends only the driver's wait.
@task_item n=1 part=untouched
CLI output rows (refusal lines, hint lines, exit codes, --json documents, the list board, its captions, colour and SGR rules): no CLI string, code word or exit code changed.
@task_item n=2 part=untouched
TUI passthrough contracts (viola terminal bytes, human keystrokes, focus, mouse and resize sequences, vt100 feed panic): the passthrough was not changed; the keys typed into the trust dialog and input box were a test rig's, not viola's.
@task_item n=3 part=untouched
The send and readback path and its refusals (input-not-ready, no-prompt-submitted): measured live, not changed; the DEL erase reading is a vendor-CLI fact, not a viola contract.
@task_item n=4 part=untouched
CI integration, SLO and a11y gate rows: no gate, suite, threshold or workflow changed; CI was green on the pushed commit.
@task_item n=5 part=untouched
Screen-reader, manual-pass, keyboard-navigation and visual-design rows, and the absences of a web, touch or i18n surface: nothing in the session reaches them.
