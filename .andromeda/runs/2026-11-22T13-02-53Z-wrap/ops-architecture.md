@row_set n=1 row=decision/send-time-bound cell=rule seen=2 detector=none
value: send blocks at most the readiness gate's maximum wait plus the compiled 5 s paste bound plus the window; on the MCP surface the MCP call deadline bounds the driver's wait, never the send itself.
basis: fct-z6owtgsx
because: The paste is now bounded by the compiled PASTE_BOUND of 5 s, so the send's bound names it instead of an unbounded paste.
@row_set n=2 row=decision/atomic-bracketed-send cell=except seen=1 detector=none
value: A paste not finished within the compiled paste bound is cut: the writer stops after its in-flight chunk and writes the closing paste marker with no CR.
basis: fct-z6owtgsx
because: A cut paste closes the bracket without the CR, an exception the row's rule of one bracketed paste plus CR no longer covers.
@row_set n=3 row=contract/refusal-reason/not-delivered cell=promise seen=1 detector=none
value: Serialised kebab-case; its detail is one of input-not-ready, no-prompt-submitted, turn-running, unknown-dialog, control-character, empty-text, paste-cut.
basis: fct-vl24ioge
because: The closed not-delivered detail set gained paste-cut, so the row's closed list is incomplete.
@row_set n=4 row=contract/refusal-order/send cell=promise seen=2 detector=none
value: control-character, empty-text, human-typing, turn-running, the readiness gate (input-not-ready), human-typing and turn-running read again after the gate, paste-cut of a standing cut mark, paste-cut of a paste cut at its bound, then no-prompt-submitted.
basis: fct-vl24ioge
because: The send refusal order gained two paste-cut rungs, the mark after the second reads and the cut after the paste.
@row_set n=5 row=contract/send-refusal/order cell=promise seen=2 detector=none
value: `control-character` → `empty-text` → `human-typing` → `turn-running` → the readiness gate's wait → `human-typing` and `turn-running` again → `paste-cut` of a standing cut mark → `paste-cut` of a paste cut at its bound.
basis: fct-vl24ioge
because: The send refusal order gained two paste-cut rungs after the gate, which the row's order does not show.
@row_set n=6 row=contract/send/paste-failure-after-issue cell=promise seen=1 detector=none
value: A paste that fails after send-issued is refused as input-not-ready with that send's cursor, except a paste cut at its bound, refused as paste-cut with that send's cursor.
basis: fct-z6owtgsx
because: A cut paste after send-issued is now refused paste-cut rather than input-not-ready, so the row's promise is false for it.
@row_add n=1 row=decision/paste-bound area=established-decisions fact=F1 detector=none
name: Paste cut at a compiled paste bound
rule: A send's paste not finished within the compiled 5 s paste bound, counted from the call, is cut: the writer stops after its in-flight chunk of at most 4096 bytes and writes the closing paste marker with no Enter; a paste still queued at the deadline is cut with nothing written.
why: So a paste never blocks past its bound, even while queued behind human keys.
basis: fct-z6owtgsx
because: The chunk bounds the paste into the child's PTY by a compiled bound and cuts a paste that cannot finish, a decision no row holds.
@row_add n=2 row=decision/paste-cut-mark area=established-decisions fact=F2 detector=none
name: Cut mark refuses sends until the human acts
rule: A cut sets an in-memory mark at the wheel's human-key count; while the count is unchanged the next send is refused not-delivered paste-cut with no cursor; a human key read, a prompt-submitted of any origin, a session-start or a session-end clears it; a release with no key or a turn-ended leaves it.
basis: fct-z6owtgsx
because: The chunk adds an in-memory cut mark with its set and clears, a refusal rule no row of the area states.
@row_add n=3 row=decision/one-pty-writer-thread area=established-decisions fact=F3 detector=none
name: One PTY writer thread with a bounded queue
rule: All writes into the child's PTY go through one writer thread under catch_unwind, taking human key chunks and paste jobs in arrival order from a bounded queue of 8; human keys block on a full queue and are never dropped; a writer panic ends the run with the child killed.
basis: fct-z6owtgsx
because: The shared input-writer lock is replaced by one writer thread with a bounded job queue, a decision on how the human and the driver share the PTY that no row holds.
@row_add n=4 row=contract/not-delivered-details/paste-cut area=established-decisions fact=F4 detector=none
boundary: not-delivered refusal details
item: paste-cut
promise: A send whose paste was cut at its bound, with that send's cursor, or refused by a standing cut mark, with no cursor, is refused `not-delivered` with this detail, exit 13.
basis: fct-z6owtgsx
because: The closed not-delivered detail set gained paste-cut, a new item of the boundary whose other details stand as rows.
@row_add n=5 row=contract/human-hint-lines/paste-cut area=established-decisions detector=none
boundary: Human hint lines
item: paste-cut
promise: Reads "<name> did not take the whole text in time; a human must look at the session before the next send"; the wording is provisional.
basis: fct-z6owtgsx
because: The paste-cut refusal gained its own human hint line, beside the input-not-ready hint row.
