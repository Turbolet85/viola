@row_set n=1 row=decision/snapshot-replay-from-log cell=guard seen=1 detector=D-tests-coverage
value: a_start_over_an_unreadable_snapshot_replays_the_log, e5_the_replay_reads_the_link_the_wrapper_wrote
basis: fct-yeqzuggx (F10 replay guard), fct-npnimtjk, fct-43yqacqc (control link-replay holds)
because: e5_the_replay_reads_the_link_the_wrapper_wrote is now proven by control link-replay to be the guard of the replay's link pass, the test the report names in place of the cell's unwitnessed e5 name, so the guard cell must name it.
@row_set n=2 row=decision/one-shared-test-helper cell=rule seen=1 detector=none
value: Root tests share the events reader and wait, the child guard and runner, the freeze guard, the result struct and digit_runs from the test support module; no root test file defines its own copy.
basis: fct-yeqzuggx (F11 digit_runs), fct-npnimtjk
because: digit_runs now lives once in the test support module and its three copies are gone, so the shared set the rule names must include it.
@row_set n=3 row=decision/pty-tested-on-real-openpty cell=rule seen=1 detector=none
value: PTY integration spawns viola-fake-agent through viola-pty on a real openpty, and a resize is read from the child's own size report alone, with no key written after it.
basis: fct-yeqzuggx (F7 resize test), fct-avspoz2u
because: The resize test now writes no key after the resize and waits on the size receipt alone, so the rule's key-after-resize clause no longer holds.
@row_set n=4 row=decision/hygiene-reports-class-only cell=rule seen=1 detector=none
value: The hygiene checker reports a violation class only (absolute-path, username, email, schema), never the offending content.
basis: fct-yeqzuggx (F3 hygiene email), fct-wzng4bg6
because: The shared hygiene checker gained the email violation class, reported as class only, so the closed class list the rule states must name it.
@row_set n=5 row=decision/driver-hints-never-name-release cell=rule seen=1 detector=none
value: A driver-facing refusal hint (human-typing, budget-paused, unverified-cli) never contains release.
basis: fct-yeqzuggx (F1 hints; Tests added: REFUSALS table read by no_hint_names_release)
because: unverified-cli now carries a driver-facing hint checked by no_hint_names_release, so the list of hinted refusals the rule names must include it.
@row_set n=6 row=decision/strict-modes-two-rules cell=rule seen=1 fact=F1 detector=none
value: Strict modes have a write rule and a read rule, the read rule being the write rule plus a refusal of any group or other permission bit; both refuse a symbolic link at the checked path itself before the mode verdict, leaving link and target as found.
basis: fct-yeqzuggx (F5 strict link), fct-avspoz2u
because: The write and read rules now refuse a symbolic link at the checked path before the mode verdict, a security decision the strict-modes row states less.
@row_set n=7 row=decision/diagnostics-files-owner-only cell=rule seen=1 fact=F2 detector=none
value: Diagnostics, heartbeat and event log files are opened for append refusing a symbolic link at the path and are narrowed to mode 0600 through the open handle, never by path.
basis: fct-yeqzuggx (F6 append), fct-avspoz2u
because: The private append open now refuses a link at the path and narrows the mode through the handle, which the owner-only row states only as the 0600 mode.
@row_set n=8 row=decision/hook-payloads-recorded-by-verify cell=rule seen=1 fact=F5 detector=none
value: Hook payload fixtures are recorded locally by live viola verify, scrubbed and unclean-checked before they are committed, a dirty payload, one holding an email-shaped token in any string or key included, refusing the whole recording, and CI replays all of them.
basis: fct-yeqzuggx (F2 record email), fct-wzng4bg6
because: verify --record now refuses an email-shaped token in any string or key of every recorded payload kind through unclean-fixture, which the recording row states less.
@row_add n=1 row=decision/payload-types-derive-redact area=unit-test-strategy fact=F4 detector=none
name: Content-bearing payload types derive Redact
rule: Every hook and MCP payload type that carries content derives veil Redact, so its Debug output reveals no content.
guard: normalised_debug_holds_no_prompt_text, input_debug_holds_no_answer_or_text
except: A payload type with no content field derives none, since veil 0.3.0 refuses a Redact derive with no redacted field.
basis: fct-yeqzuggx (F9 Redact), fct-irqnu3k2, fct-kvnor7kc
because: The hook and MCP payload types now derive veil Redact where they carry content, a decision with a named exception no row of the area holds.
@row_add n=2 row=contract/viola-answer-human-mode/unable area=e2e-test-strategy fact=F3 detector=none
boundary: viola answer human mode
item: refusal lines
promise: A human-mode answer refused not-delivered ends with the hint send holds for that detail, an unknown-dialog refusal with its own hint, and an unknown refusal, an opaque detail or no detail with no hint.
basis: fct-yeqzuggx (F1 hints), fct-avspoz2u
because: answer's human-mode refusal hints are now a promise at the CLI boundary that no row of the area states.
@row_add n=3 row=decision/control-isolates-its-own-case area=test-strategy fact=F6 detector=none
name: A guard's own case isolated by its own control
rule: A remove-the-guard control fires its entry whole, so a guard's own new case is witnessed red only by a control of its own.
why: nextest's default fail-fast ends the removed firing at its first red, so another case may carry the red while the new case goes unrun.
basis: fct-iuljoafr, fct-k2mygx2a
because: The controls showed a guard's own new case can go unrun under fail-fast, a convention for future controls that no row of the area holds.
