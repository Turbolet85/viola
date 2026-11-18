@row_set n=1 row=contract/ledger-stamps/measured cell=promise seen=2 fact=F1 detector=D-arch-resources
value: The probe's readings (largest hook payload per spine event, typed_probe timings, box_only_guards counting typed-run guards passed on the box's own rows alone, dialog_probe, each typed run's first_screen, first_settle_ms and first_facts); additions need no v bump, and run reads no field of it.
basis: fct-vynq47jv
because: The stamp's measured block gains first_screen, first_settle_ms and first_facts, so the row's list of readings must name them.
@row_set n=2 row=decision/verify-record-only-clean cell=except seen=1 fact=F2 detector=none
value: The hidden rig-only --record-first-screens writes, on a failed --record verify, a scrubbed whole-row first screen per run short of input box, Run A's trust excepted.
basis: fct-xaokr7kk
because: The hidden first-screen record writes files from a failed verify, the one exception to fixtures recorded only from a clean verify.
@row_set n=3 row=decision/recorded-fixtures-scrubbed cell=except seen=1 fact=F2 detector=none
value: The hidden rig-only --record-first-screens writes, on a failed --record verify, a scrubbed whole-row first screen per run short of input box, Run A's trust excepted.
basis: fct-xaokr7kk
because: The row says fixtures are written only when no row fails, and the hidden first-screen record now writes scrubbed, whole-set-refused files from a failed verify.
@row_set n=4 row=decision/screen-fixtures-signature-rows-only cell=except seen=2 fact=F2 detector=none
value: The hidden rig-only --record-first-screens keeps a failed verify's first screens as whole scrubbed rows, kept locally only.
basis: fct-xaokr7kk
because: The first-screen record keeps whole scrubbed rows, the one exception to signature-rows-only screen fixtures.
@row_set n=5 row=contract/viola-cli-arguments/verify cell=promise seen=1 fact=F2 detector=D-arch-resources
value: viola verify takes [--record <DIR>] [-- <program> [args…]], the program defaulting to claude; a hidden --record-first-screens, absent from --help, is passed only by the live rig.
basis: fct-xaokr7kk
because: verify gains one hidden option passed only by the live rig, which the arguments contract must register.
@row_set n=6 row=decision/run-never-reads-screen-content cell=rule seen=1 detector=none
value: run never reads screen content; the only row-text exit of the screen is read by verify alone, for its signature-only recorded screen fixtures and the rig-only first-screen record of whole scrubbed rows.
basis: fct-xaokr7kk
because: verify now also takes whole scrubbed rows out of the screen for the rig-only first-screen record, so the row's one stated use is incomplete.
@row_set n=7 row=decision/claude-first-no-adapter-trait cell=except seen=3 detector=none
value: Test-only: the rig reads its captured UserPromptSubmit payload's key names and source word, never a value, and seeds its private Claude config; the fake agent and the rig read a config's trust record.
basis: fct-vynq47jv
because: The live rig now seeds its private Claude config with bundle keys, a Claude-specific shape outside viola-agent-claude that the test-only exception must name.
@row_set n=8 row=decision/claude-shapes-in-one-crate cell=except seen=3 detector=none
value: Test-only: the rig reads its captured UserPromptSubmit payload's key names and source word, never a value, and seeds its private Claude config; the fake agent and the rig read a config's trust record.
basis: fct-vynq47jv
because: The live rig now seeds its private Claude config with bundle keys, a Claude-specific shape outside viola-agent-claude that the test-only exception must name.
@row_set n=9 row=decision/claude-shapes-isolated cell=except seen=2 detector=none
value: Test-only: the rig reads its captured UserPromptSubmit payload's key names and source word, never a value, and seeds its private Claude config; the fake agent and the rig read a config's trust record.
basis: fct-vynq47jv
because: The live rig now seeds its private Claude config with bundle keys, a Claude-specific shape outside viola-agent-claude that the test-only exception must name.
@row_add n=1 row=contract/ledger-stamps/first-screen area=standard-contracts fact=F1 detector=D-arch-resources
boundary: ledger stamps envelope
item: measured.first_screen
promise: For each of the six interactive verify runs, one closed word naming its first settled screen: trust, imports, box, none or exited; no screen text.
basis: fct-vynq47jv
because: The stamp's measured block gains first_screen as a closed word per run, a format others read.
@row_add n=2 row=contract/ledger-stamps/first-settle-ms area=standard-contracts fact=F1 detector=D-arch-resources
boundary: ledger stamps envelope
item: measured.first_settle_ms
promise: For each of the six interactive verify runs, the time to its first settled screen in milliseconds, a whole number.
basis: fct-vynq47jv
because: The stamp's measured block gains first_settle_ms as a whole number per run, a format others read.
@row_add n=3 row=contract/ledger-stamps/first-facts area=standard-contracts fact=F1 detector=D-arch-resources
boundary: ledger stamps envelope
item: measured.first_facts
promise: For each of the six interactive verify runs, the first screen's row count, box_any, literals keyed by box literal id each with its first row, rule_above_prompt, first_row as rule, banner or text, and last_col; no screen text.
basis: fct-xaokr7kk
because: The stamp's measured block gains first_facts with its closed fields per run, a format others read.
@row_none n=1 fact=F3 type=4
relied: the stamp's measured first_screen word, read by the live rig's summary, comes from this classification; the code holds how it is built and the report states no reason for it.
