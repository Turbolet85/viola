@row_set n=1 row=contract/confirm-window-close/ordinary-send cell=promise seen=1 fact=F1 detector=none
value: Closes on the prompt-submitted{origin:"driver"} whose prompt equals the typed text, or is non-empty and equals the typed text with a trailing run of ASCII space, TAB, CR or LF removed (a provisional tolerance), or on window expiry.
basis: fct-ynhvzkwk
because: The claim that confirms a send and logs send-confirmed now takes the submitted prompt with a provisional trailing-whitespace tolerance in place of exact equality, so the row's «matching» must say which prompt closes the window.
@row_set n=2 row=contract/send-refused-detail/turn-running cell=promise seen=1 fact=F2 detector=none
value: Covers a running turn the wrapper knows of before the paste, or another send in flight, with no new field; the refusal has no cursor (corr the end offset at refusal), no send-issued, and the client exits 13.
basis: fct-ynhvzkwk
because: The post-gate turn read, the link record, send-issued and the paste now run under one flight lock, so a turn known to the wrapper before the paste always refuses turn-running with no send-issued line, while a turn whose UserPromptSubmit arrives after the paste stays undetected (fct-izz7vw4n), and the row must bound what it covers.
