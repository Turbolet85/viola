@row_set n=1 row=contract/viola-send-output/read-back cell=promise seen=1 fact=F1 detector=none
value: `[RB] read back …` prints on stdout with exit 0 when the send is confirmed by a prompt-submitted whose prompt equals the typed text, or equals it with a trailing run of space, TAB, CR and LF removed (that trailing tolerance provisional); any leading or inner difference leaves the send unconfirmed.
basis: fct-ynhvzkwk
because: The confirmation match widened from exact equality to also take the typed text less a trailing run of space, TAB, CR and LF (PROVISIONAL), so the row's «matching prompt-submitted» must say what now matches.
