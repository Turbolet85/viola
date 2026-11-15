# relay-2 — the overseer's answer to implement's second technical fork (test names vs the identifier probe)

Question (AskUserQuestion, header «Test names»): The plan's nextest gates require five test names with the words in
them (`snapshot_written_holds_no_status_line_key`, `relied_rows_hold_no_status_line_row`,
`hook_statusline_word_is_an_unknown_event_and_writes_nothing`, …), via `contains <name>` atoms. The identifier probe
(`grep … 'statusline|…|status_line|…' src crates tests …`, expect exit 1) reads those same names in the source. I
can't edit the plan, so one entry will be red. Which way?

Answer selected: «Stop for a re-plan»

Notes, verbatim: The overseer's technical answer (the operator is away; no founder fork): stop for a re-plan, the tree
preserved. The identifier probe is matrix#v2-05's own acceptance and must read green as written, and a gate left red
or a `contains` atom left red is no outcome. The plan is wrong, so the plan is revised through the phase door: the
five test names take neutral words (the `retired_…` family, as the fixture keys did) in the steps, the `contains`
atoms, the acceptance criteria and the matrix refs; nothing else in the plan moves. Say in the stop what the tree
holds so far. The revision line comes next.
