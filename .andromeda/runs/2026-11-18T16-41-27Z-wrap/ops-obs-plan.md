@row_set n=1 row=contract/verify-stdout/step-line cell=promise seen=1 detector=none
value: One "[NN/MM] <row id> <row words>  pass|absent|fail" line is printed for each relied ledger row, MM being the relied row count; absent is the browser-tools-prompt row's word when no run met the prompt.
basis: fct-xmezyz7c (verify.rs `step_word` pass · absent · else fail, `check_step` prints `row_word`'s word); fct-p6udr6gw (obs-plan `contract/verify-stdout/step-line`, the verdict `absent`)
because: The step line's verdict is no longer pass or fail alone: the 24th row reads absent when no run met the browser-tools prompt, so the promised line shape must name it.
@row_set n=2 row=contract/verify-stdout/summary-line cell=promise seen=1 detector=none
value: The last stdout line is "stamped <version>  N pass  N fail", the two counts summing to MM; the pass count holds every row that counts, an absent browser-tools-prompt row among them.
basis: fct-tlsodzwd (verify's summary `n pass  m fail`: `n` now counts the rows that count, an `absent` row in `n`); fct-p6udr6gw (obs-plan `summary-line`)
because: The summary's pass count now includes an absent row that counts, so the promise must say what N pass counts.
@row_set n=3 row=contract/verify-exit-codes/0 cell=promise seen=1 detector=none
value: Every relied ledger row counted: each passed, or the browser-tools-prompt row read absent.
basis: fct-tlsodzwd (the verified rule: every relied row `pass` → every relied row counting, `absent` on `browser-tools-prompt` alone); fct-wudigse5 (`verifiedrule`: the layout-templates exit 0 «when every relied row passes» changed)
because: An absent browser-tools-prompt row now counts toward a verified CLI, so exit 0 no longer requires every relied row to pass.
@row_add n=1 row=decision/browser-tools-prompt-reading-uninstrumented area=4-span-trace-coverage fact=F1 detector=D-obs-instrumentation
name: Browser-tools prompt reading uninstrumented
rule: The reading of the CLI's browser-tools prompt as a modal adds no log line, span, event or detail code of its own; verify's reading of it reaches only the ledger stamp and verify's stdout.
basis: fct-orxmgvlj (coverage: `BROWSER_TOOLS_LITERAL` instrumentation n/a, no new line; `browser-tools-prompt` ledger row instrumentation n/a, stamp only); fct-vh4fhtvm (the stamp gains `browser-tools-prompt` and `measured.first_screen` `browser-tools`)
because: The chunk adds a new screen reading with no telemetry by design, and no standing row of the area says that this reading stays uninstrumented.
