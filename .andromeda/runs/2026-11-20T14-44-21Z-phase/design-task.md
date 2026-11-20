@task_prose part=chunk
A viola run (and the start of viola revive where it shares the version gate) that meets a claude CLI version with no verify stamp gets that version stamped automatically, with no human running viola verify by hand, then goes on verified.
The form of the automatic verify (before the session starts, beside it, a separate verb or flag, a config key turning it on or off) is the founder's at P3; P4 plans only the form he answers.
A red automatic verify does not loop: it says which rows failed, leaves the session unverified to the human as today, records the failed attempt so it is not repeated, and names what clears it.
What the product tells the user before the first automatic verify, which spends the user's model turns and writes a stamp in the real viola home, and what a user who never wants it does, is also the founder's at P3.
@task_file n=1 path=src/run/version_gate.rs mark=edits
what: viola run's version gate, the check that reads the claude CLI version against the verify stamps and holds the session verified or unverified-cli — edits: the gate's handling of a version with no stamp
@task_file n=2 path=src/cmd/run.rs mark=edits
what: the viola run verb, the start of a wrapped session — edits: its start, where the automatic verify is reached
@task_file n=3 path=src/cmd/revive.rs mark=edits
what: the viola revive verb, the restart of a logged session — edits: its start, where it shares the version gate
@task_file n=4 path=src/cmd/verify.rs
what: the viola verify verb, its runs, probes, step lines and summary, as the process the automatic form starts
@task_file n=5 path=crates/viola-state/src/stamps.rs
what: the stamps.json store, its stamp writer update_stamps and the stamps lock — stamps written only by verify
@task_item n=1 part=needs
The CLI verification state: a version with no stamp is unverified-cli; once the automatic verify's stamp lands the session turns verified and the driver's sends and answers are no longer refused unverified-cli for that version.
@task_item n=2 part=needs
The unverified-cli refusal, its exit code 12 and its refusal hint, which today tell the driver to run viola verify by hand; what they say once verify can run automatically, and what they still say after a red automatic verify.
@task_item n=3 part=needs
The rule that stamps are written only by verify: the automatic form must still reach the stamp through viola verify's own writer and the stamps lock, never a second writer.
@task_item n=4 part=needs
viola verify's own output: its step lines, fail word, summary line, stopped-or-refused run and stamps.json update refused line, as they read when verify is started by run rather than by a human, and which of its rows a red verify names.
@task_item n=5 part=needs
viola verify's refusals (CLI not found, version unreadable, unclean fixture, interrupted run, home-tree check, stamps.json update) as outcomes the automatic form meets and must not turn into a loop or a blocked start.
@task_item n=6 part=needs
The record of a failed automatic attempt for a version: kept so the next start of that version does not verify again, and the named act that clears it.
@task_item n=7 part=needs
viola run's start and its start refusals (home-tree check, socket directory, already-live, squatted-name, stale-heartbeat, pinned-hash-mismatch) and their order, with the automatic verify placed among them.
@task_item n=8 part=needs
viola revive's start, its refusals and their order, where it shares the version gate with run.
@task_item n=9 part=needs
viola run prints nothing while wrapped and no document once the child held the terminal: any line the automatic verify prints, or its result, must fall before the child starts or stay off the terminal the child holds.
@task_item n=10 part=needs
The child-started output of viola run and revive, and whether the automatic verify adds a line before it.
@task_item n=11 part=needs
No interactive prompt or pager: what the product asks or tells the user before the first automatic verify cannot be an interactive question on the terminal.
@task_item n=12 part=needs
The one progress pattern, the step counter, and no spinner or elapsed-time counter: how an automatic verify shows its progress if it shows any while run waits on it.
@task_item n=13 part=needs
Data on stdout, messages on stderr, and the stderr forms (refusal line, hint line, context lines, error line): where the automatic verify's messages and a red verify's failed rows go.
@task_item n=14 part=needs
No internals in errors or hints, and start refusals show no path or pid: what a red automatic verify says about where its record lives or which rows failed.
@task_item n=15 part=needs
Nothing announces success before confirmation: the session is not shown verified until the stamp has landed.
@task_item n=16 part=needs
The closed key set of config.json, if a consent key turning the automatic form on or off is the founder's form.
@task_item n=17 part=needs
The CLI exit codes viola run and revive return when the automatic verify is red or refused, and whether a run with a red verify still starts its session.
@task_item n=18 part=needs
The verify verb's name and its place in the flat CLI verbs and the viola --help verb groups, and any new verb or flag a launcher calls, if that is the founder's form.
@task_item n=19 part=needs
The --json documents of run and revive (one stdout document, clean failure documents, the --json flag scope at the catch site) as they carry an automatic verify's outcome.
@task_standing n=1
convention: every new human line is ASCII-only, never wrapped, with no glyph or emoji prefix and no decorative icon
home: surface-cli
@task_standing n=2
convention: every new refusal takes the fixed unable form with at most one hint line, in human mode only, and no hint names viola release
home: surface
@task_standing n=3
convention: every new message colours only by the CLI colour decision order and tokens, colour only as a second cue
home: surface-cli
@task_item n=1 part=not_needed
The flight-progress strip, the readback box, the send readback and the handoff transfer marker: the chunk changes no strip, mirror or readback a send produces.
@task_item n=2 part=not_needed
The viola list board and table rows (columns, NAME cut, skip counts, row order, bay line, empty board): the chunk adds nothing to the board.
@task_item n=3 part=not_needed
viola link and unlink output and their --json documents, and viola wait and viola last output: none of these verbs is touched.
@task_item n=4 part=not_needed
The wrapper fault --json detail, peer reply screening and the wrapper socket refused peer: the chunk changes nothing on the wrapper socket.
@task_item n=5 part=not_needed
The exit 21 causes and their --json causes and log detail codes, and the frame verb refusals: these are instance-unreachable outcomes of verbs aimed at a running instance, not of starting one.
@task_item n=6 part=not_needed
The hook and mcp verbs, their stdout, stderr and exit code: the chunk reaches neither.
@task_item n=7 part=not_needed
The rendered page, web-page surface and the N/A token areas (depth, radius, spacing tokens): viola renders no page.
@task_item n=8 part=not_needed
The self-validation protocol's design tests (squint, swap, readback box signature check, domain design): they judge the board's expression, which this chunk does not change.
@task_item n=9 part=not_needed
The brand expression rows for strips (stable session slots, cocked strip, stale strip dimmed, density): the chunk draws no strip.
