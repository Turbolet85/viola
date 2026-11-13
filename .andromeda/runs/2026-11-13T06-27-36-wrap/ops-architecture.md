@row_set n=1 row=decision/screen-signatures-compiled cell=rule seen=1 fact=F1 detector=D-arch-decisions
value: Modal and input-box signatures are compiled literals validated by each CLI version's stamp; a screen is ready when no row holds a modal literal and a footer literal stands or the box's own rows do: a rule row of the compiled rule char alone, at least 40, above a row opening the compiled prompt char.
basis: fct-xdv7p2qy
because: The readiness predicate now reads the input box up on its own rule and prompt rows beside the footer literals, a rule shorter than 40 chars leaving the gate waiting, so the row's readiness rule must say so.
@row_set n=2 row=decision/verify-box-wait-for-literal cell=rule seen=1 detector=none
value: After each added paste turn of viola verify, the rows the paste guard reads come only from a quiet screen holding the input box (a footer literal or the box's own rows) or a modal literal, a poisoned screen, or the probe deadline; any other quiet screen keeps waiting.
basis: fct-o3sexi33
because: Verify's settle and box wait now end on the input box read by its footer literal or its own rows, or on a modal literal, so the wait's rule must name the box's rows beside the literals.
@row_set n=3 row=contract/ledger-stamps/measured cell=promise seen=1 fact=F2 detector=none
value: The probe's readings (largest hook payload per spine event, typed_probe timings and box_only_guards, the count of typed-run guards that passed on the input box's own rows with no footer literal on any row, dialog_probe); additions need no v bump, and run reads no field of it.
basis: fct-w5at72sz
because: The stamps envelope's measured typed_probe gains box_only_guards, the count verify stamps of box-only guard passes, so the contract's list of readings must carry it.
@row_set n=4 row=decision/screen-fixtures-signature-rows-only cell=rule seen=1 fact=F3 detector=none
value: A recorded screen fixture keeps a row holding a footer or modal literal whole, the input box's rule row as its rule chars alone and its prompt row as the prompt char alone, every other row empty; a box whose live prompt row holds a home path, the username or an email refuses the recording.
basis: fct-w5at72sz
because: A recorded box row is now kept as its literal alone and its seam is the live prompt row's whole text, so the fixture rule must say how box rows are kept and what refuses them.
@row_add n=1 row=decision/advisory-ignore-disabled-feature-only area=build-system fact=F4 detector=none
name: Advisory ignores only for disabled features
rule: A dependency advisory is ignored only where it sits in a feature the dependency policy bans or keeps off the release graph; each ignore carries a reason naming that feature and stands until the dependency leaves its ratified range.
why: Ratified.
guard: cargo deny check
basis: fct-w5at72sz
because: The dependency policy's advisories ignore list went from empty to reasoned ignores for advisories in disabled rmcp features, a ratified rule no build-system row states.
