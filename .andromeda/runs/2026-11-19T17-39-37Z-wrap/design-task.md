@task_prose part=session
The session recorded one contained live StopFailure payload on claude 2.1.328 through the test-only live rig, a new SF arm run inside a network-closed bwrap sandbox, and read it GREEN: turn-end stop-failure, error word server_error, cause server-error, cause matched.
On that reading the StopFailure exception is settled as outcome B on the founder's word: StopFailure stays registered with no ledger row and no verify probe, no longer provisional; outcome A (a ledger row with a verify probe) is not opened.
The rig's StopFailure capture keeps key names, the error word only from the CLI's closed set of 13 (else other) and the byte lengths of two text fields, never a text.
The rig read the readback box row's cell attributes after the turn end, never its text: hint-attr inverse, typed-attr plain, told apart yes; a reading of one cell that may be the cursor's, so that carry stays open.
The rig's kept record dir is now keyed by CLI version first, and the fake agent gained two argv options; no product crate, CLI verb, output, exit code, colour or glyph changed.
@task_file n=1 path=crates/viola-e2e/src/rig/isolate.rs
what: the live rig's scratch isolation of its child: the kept record dir keyed by CLI version, the network-closed sandbox argv and its runtime-dir precondition, the seeding of the StopFailure capture hook
@task_file n=2 path=crates/viola-e2e/src/rig/mod.rs
what: the standing live rig's arms and plan: the new SF arm (labels SF and SF-2), the contained run, the sandbox probe and its refusals, and the readings of turn end, cause, unknown values and the box's cell attributes
@task_file n=3 path=crates/viola-e2e/src/rig/capture.rs
what: the live rig's closed hook capture: its StopFailure line keeping key names, the error word of the CLI's 13 else other, and two text lengths, never a text
@task_file n=4 path=crates/viola-e2e/src/rig/screen.rs
what: the live rig's terminal screen model: a cell's attributes and the readback box row's attributes read as closed words, never the cell's text
@task_file n=5 path=crates/viola-e2e/src/bin/viola-live-rig.rs
what: the live rig's command line: rehearse sf with its contain option and the live SF and SF-2 starts
@task_file n=6 path=crates/viola-e2e/tests/rig_rehearsal.rs
what: the live rig's rehearsals over the fake agent run in CI: four sf rehearsals (turn end and cause, an unknown error word, the contained run, the sandbox refusal)
@task_file n=7 path=src/bin/viola-fake-agent.rs
what: the fake agent the tests drive in place of Claude: a turn that ends in a recorded StopFailure variant and extra settings hooks run beside the plugin's
@task_item n=1 part=touches
decision/api-error-turn-end-shown-as-turn-end: the API-error turn end it rules over now stands on a live StopFailure reading (claude 2.1.328, 2026-11-19, start SF, word server_error mapped to cause server-error); a cell citing a synthetic StopFailure payload or a provisional exception is stale.
@task_item n=2 part=touches
The StopFailure exception is settled: StopFailure stays registered with no capability-ledger row and no verify probe, kept on the founder's word, no longer provisional. Any row on CLI verification state, viola verify output or the unverified-cli refusal hint whose cell names that exception as provisional, or as owed to a live payload entry, is now stale.
@task_item n=3 part=touches
Only one live StopFailure was seen, of the connection-lost class, on one version; the other twelve error words of the CLI's list were not seen live. A row that holds the CLI's closed error-word set or its count of 13 now has a test-side copy of it in the rig's capture, which must stay in step with the adapter's cause map.
@task_item n=4 part=touches
decision/readback-box and the readback-box contract rows: the test rig now types one x and one DEL into the box after the turn end and reads the box row's cell attributes, never its text; the live reading told the placeholder hint (inverse) from typed text (plain), an open reading of one cell, not a rule.
@task_item n=5 part=touches
decision/turn-ended-never-fills-readback: indirectly, through the rig's after-turn-end keystrokes into the box; the rig types into the child's box only after the turn ends and keeps no text, so the rule stands, but a picker should check its cell names no rig behaviour now changed.
@task_item n=6 part=touches
decision/hint-never-quotes-text and decision/no-internals-in-errors: indirectly, through the rig's own readings that keep attribute words and lengths only, never a screen or payload text; no viola hint line changed.
@task_item n=1 part=untouched
CLI exit codes and exit-21 causes: the rig's refused: sandbox exit 3 and the instance-dead seen in a red test firing are the test rig's own words and a test fault since fixed, not a viola verb's exit code or cause.
@task_item n=2 part=untouched
Colour palette, CLI colour tokens, typography and SGR rows (dim, bold, amber, handoff blue): the attribute words the rig reads describe Claude's own screen cells, not anything viola draws; no viola colour or style changed.
@task_item n=3 part=untouched
Command names and verb output rows: no viola verb, flag or name was added, taken or freed; sf, SF-2 and the contain option belong to the test rig alone.
@task_item n=4 part=untouched
viola list board, flight-progress strip columns, revive, link, run start refusals, send readback mirror and wait output contracts: none of their outputs changed.
@task_item n=5 part=untouched
Brand identity, iconography, glyphs, spacing, the rendered-page and web-surface absences, the anti-patterns and the self-validation checks: the session brought no visual or interface change.
