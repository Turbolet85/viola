@row_set n=1 row=contract/viola-verify/step-line cell=promise seen=1 detector=none
value: Each verify step prints a line [NN/NN] … pass|absent|fail; absent is printed only on the browser-tools-prompt row, when no run met the prompt.
basis: fct-xmezyz7c
because: verify's step line now prints the ledger row's own word through pass · absent · fail, so a row promising pass|fail alone is false.
@row_add n=1 row=contract/viola-verify/summary-line area=1-a11y-scope-summary fact=F1 detector=none
boundary: viola verify output
item: summary line
promise: The summary line stamped <ver>  n pass  m fail counts in n every relied row that counts, an absent browser-tools-prompt row included, while that row's step line still prints absent.
basis: fct-tlsodzwd
because: verify's summary n now counts the rows that count, an absent browser-tools-prompt row among them, and no a11y-plan row states the summary line's promise.
