@row_set n=1 row=decision/browser-tools-prompt-reading-uninstrumented cell=rule seen=1 fact=F1 detector=D-obs-instrumentation
value: The reading of the CLI's browser-tools prompt as a modal adds no log line, span, event or detail code of its own; verify's reading of it reaches only the ledger stamp and verify's stdout, whose one fixed line on a met prompt is the only record that the prompt stood at start.
basis: fct-jzh734gm
because: Verify's stdout gains a browser-tools line written with no log line, span or event, so the row that already bans its instrumentation must name that line as the prompt's only record.
@row_set n=2 row=decision/test-homes-under-e2e-home cell=except seen=1 fact=F3 detector=none
value: the one chaos_feed_panic test home; the live rig's vh arm, run by hand under the real home and Claude config and never in CI, reading names and one needle only and keeping no value
basis: fct-xiw3vi2f
because: The test-only live rig gains the vh arm that runs under the real home outside the one test-home root, so the rule's named exceptions must carry it.
@row_add n=1 row=contract/verify-stdout/browser-tools-line area=4-span-trace-coverage fact=F2 detector=none
boundary: verify stdout
item: browser-tools line
promise: On a run that met the browser-tools prompt, verify prints one fixed line before the summary: the browser-tools prompt stood at start; viola never answers it: answer it once in claude, then run viola verify again; the summary stays the last line and the prompt-absent stdout is unchanged.
basis: fct-sfqjrkq4
because: Verify's stdout gains a fixed line on a met browser-tools prompt, a new promise at the verify stdout boundary that no standing row states.
