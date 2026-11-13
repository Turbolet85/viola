@row_set n=1 row=decision/no-advisory-ignored cell=except seen=1 detector=none
value: An advisory that sits only in an rmcp feature banned or off in the release graph is ignored with a reason naming that feature, standing until rmcp leaves <3.5.
basis: fct-w5at72sz, fct-gnrmbzq2
because: deny.toml now ignores three rmcp advisories, each reasoned by its disabled feature and ratified by the founder, so the no-ignore rule stands only with that exception.
@row_set n=2 row=decision/no-ignored-advisories cell=except seen=1 fact=F4 detector=none
value: An advisory that sits only in an rmcp feature banned or off in the release graph is ignored with a reason naming that feature, standing until rmcp leaves <3.5.
basis: fct-w5at72sz, fct-gnrmbzq2
because: cargo deny now ignores three advisories that sit only in banned or disabled rmcp features, ratified as an exception, so the row reading no ignore at all is false without it.
@row_set n=3 row=contract/readiness-gate/gate-max-wait cell=promise seen=1 detector=none
value: On a verified CLI a quiet screen on which the input box does not read up (no footer literal and no box rule row directly above a prompt row) waits up to 8 500 ms and reads input-not-ready from that bound on.
basis: fct-xdv7p2qy
because: the gate's verdict now reads the input box up on its own rule and prompt rows with no footer literal, so a quiet screen with no footer literal but the box up no longer waits.
@row_set n=4 row=decision/screen-signatures-only cell=rule seen=1 fact=F1 detector=none
value: The screen is read only for compiled literals: the input box reads up when no row holds a modal literal and a footer literal stands or a rule row of at least 40 rule chars alone stands directly above a row opening the prompt char; that one predicate decides verify's guards and the readiness gate.
basis: fct-xdv7p2qy, fct-fwcqjhid
because: readiness is now one predicate over the footer literals or the box's own rows with no modal, ratified for verify and viola run's gate, so the row naming only literals says it less.
@row_add n=1 row=decision/screen-fixture-box-rows-as-literals area=test-data-fixtures fact=F2 detector=none
name: Box rows recorded as their literals alone
rule: A recorded screen fixture keeps a box rule row as its rule chars alone and the box's prompt row under that rule as the prompt char alone, and the fixture hygiene walk refuses a prompt row with text after the prompt char or a rule row with any other text.
guard: planted_box_row_beyond_its_literal_is_rejected
basis: fct-w5at72sz, fct-fpz37g3r
because: screen fixtures now keep the input box's rows in a fixed literal-only form the walk enforces, a recording contract no standing row of the area states.
@row_add n=2 row=decision/fuzz-workspace-compile-check area=5-command-implementation fact=F5 detector=none
name: Fuzz workspace compile-checked on a type change
rule: A change to a type the fuzz targets construct needs a compile check of the fuzz workspace on its pinned nightly, offline, with no toolchain install and under a kill deadline.
why: No gate entry builds the fuzz workspace.
basis: fct-i7ad7xbh, fct-khgwjgxl
because: the chunk found no gate builds the fuzz workspace and set the compile check as the rule for a change that reaches its targets' types.
@row_none n=1 fact=F3 type=4
relied: A screen recording refuses as unclean-fixture where the input box's prompt row holds a home path, the user word or an email, the kept rule row's seam being that whole row.
