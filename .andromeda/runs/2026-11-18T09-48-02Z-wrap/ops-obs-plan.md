@row_set n=1 row=decision/fixtures-not-telemetry cell=rule seen=1 fact=F1 detector=none
value: Recorded fixtures, the rig-only hidden first-screen record of a failed verify among them, are scrub-required data kept local, never a telemetry source and never a log line, and are excluded from the observability plan.
basis: fct-xaokr7kk, fct-htygamjo
because: The chunk adds a hidden, rig-only first-screen record of whole scrubbed rows written only on a failed record verify and refused whole when unclean, so the fixture row must name it as fixture-side data and not telemetry.
@row_set n=2 row=decision/ledger-contents-never-logged cell=rule seen=1 fact=F2 detector=none
value: The capability ledger's path and its stamp contents never appear in a log line; the stamp's measured first-screen words, settle walls and row facts are closed words, whole numbers and literal ids only, holding no screen text.
basis: fct-vynq47jv, fct-htygamjo
because: The stamp's measured block gains first_screen, first_settle_ms and first_facts as closed words, whole numbers and literal ids with no screen text, and the never-logged ledger row must say what these new stamp contents may hold.
