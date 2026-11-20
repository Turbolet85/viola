@task_prose part=chunk
A viola run (and revive's start where it shares the version gate) that meets a claude CLI version with no verify stamp gets that version stamped automatically by verify, with no human running verify by hand, then goes on verified.
The driver's sends and answers are then no longer refused unverified-cli for that version.
A red automatic verify does not loop: it says which rows failed, leaves the session to the human as today, records the failed attempt so it is not repeated, and names what clears it.
The automatic form (before the session, beside it, a separate verb or flag, a config key) and what the user is told or asked before the first automatic verify are the founder's at P3.
Verify's own runs, probes, typed bytes and dialog answers stay as they are unless the founder's form needs it; proved over the fake agent, never the live home.
@task_file n=1 path=src/run/version_gate.rs mark=edits
what: viola run's version gate, the check that sets a session unverified-cli when the claude version has no stamp — edits: the version gate
@task_file n=2 path=src/cmd/run.rs mark=edits
what: viola run's start, its refusals and its --json start output — edits: the start, where the automatic verify is launched
@task_file n=3 path=src/cmd/revive.rs mark=edits
what: viola revive's start, its human and --json refusals — edits: the start, where it shares the version gate
@task_file n=4 path=src/cmd/verify.rs
what: viola verify, the process the automatic form starts; its step lines, failed rows and interrupted pair, and its call of the stamp writer
@task_file n=5 path=crates/viola-state/src/stamps.rs
what: the stamps store, its stamp writer update_stamps and the stamps lock
@task_item n=1 part=needs
The unverified-cli state and its exit 12: after an automatic stamp lands the session turns verified and send and answer are no longer refused unverified-cli for that version; with no stamp they still are.
@task_item n=2 part=needs
New fixed CLI copy the chunk adds (the notice or question before the first automatic verify, the line naming failed rows, the line naming what clears a recorded failed attempt) comes from design and is pinned by literal.
@task_item n=3 part=needs
A red automatic verify's output: the verify step line form, failure and unable words never coloured, a word beside every SGR span, no SGR off a terminal, static linear output with no spinner.
@task_item n=4 part=needs
Indirect, through viola run's TUI passthrough: an automatic verify run before or beside the session puts zero viola bytes on the child's terminal; its diagnostics never reach the terminal and go to the run diagnostics.
@task_item n=5 part=needs
Indirect, through the wheel: while an automatic verify runs beside a session, human keystrokes are never blocked and a human key still takes the wheel; after a red verify the session is the human's as today.
@task_item n=6 part=needs
Time bounds of the automatic verify fall on the driver only, never on the human; no harsh timeout without warning.
@task_item n=7 part=needs
Any new refusal of run or revive start (a recorded failed attempt, a refused automatic verify): unable line with its kebab-case reason code beside a plain word, one hint per code, the hint ending the refusal and naming what clears it, never naming release; stderr layout as the refusal contract holds it.
@task_item n=8 part=needs
Under --json, run and revive start keep one clean refusal or failure document with no hint; any new code word joins the document's code-word format.
@task_item n=9 part=needs
An automatic verify interrupted keeps verify's interrupted pair and its exit 1 stderr lines.
@task_item n=10 part=needs
Printed claude version strings and failed-row names from the CLI are printed with C0 and C1 control characters escaped.
@task_item n=11 part=needs
New error bodies have a writer and a witness, read as word, detail and hint, carry no stack trace or bare status, and their thiserror messages interpolate nothing; faults carry no hint.
@task_item n=12 part=needs
If the founder's form is a consent config key, it joins config.json's closed key set as a contract row beside dialog_hold_s.
@task_item n=13 part=needs
Proof over the fake agent: any live start is a scratch viola home under the rig, and live readings never gate; manual screen-reader or keyboard passes stay supplemental and gate nothing.
@task_standing n=1
convention: each new test runs inside the existing suites as a nextest case, no skips, zero flakiness, no retries; an a11y failure is a nextest failure and fails the build
home: 1-a11y-scope-summary
@task_standing n=2
convention: each new fixed CLI string is pinned by its literal in a test
home: 8-cognitive-accessibility · cognitive-accessibility
@task_standing n=3
convention: each new refusal ends with a hint and carries its code beside a plain word, never coloured
home: 11-a11y-anti-patterns-never-do-these
@task_standing n=4
convention: each new log line carries the log identity tags and stays off the terminal
home: a11y-scope-summary
@task_item n=1 part=not_needed
viola link and unlink result lines, their --json output and exits 20 and 21: the chunk does not touch linking.
@task_item n=2 part=not_needed
viola plugin install output and its exit 21: the plugin install is not reached.
@task_item n=3 part=not_needed
viola revive --list human rows and --json output: the listing of revivable sessions is unchanged.
@task_item n=4 part=not_needed
MCP server frame tools, frame verb refusals and hook event and dialog call refusals (socket directory, strict modes, server verify): the channel and hook paths are not changed.
@task_item n=5 part=not_needed
viola wait output, its waiting line and timed_out, and the MCP wait tool's timeout_ms: waiting is not reached.
@task_item n=6 part=not_needed
viola list board visual rows: NAME bold and truncation, DIALOG amber, stale-row dim and caption order are unchanged.
@task_item n=7 part=not_needed
instance-unreachable details and exit 21 under --json: reaching a running instance is not changed.
@task_item n=8 part=not_needed
Absences of a web or graphical frontend, a touch surface, internationalization and a rendered page: nothing the chunk adds is one.
@task_item n=9 part=not_needed
Screen-reader target and test setup rows (Orca with GNOME Terminal or foot): no screen-reader setup changes.
