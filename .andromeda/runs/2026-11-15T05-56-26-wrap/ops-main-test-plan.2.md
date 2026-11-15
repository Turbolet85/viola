@row_set n=1 row=decision/plugin-rewritten-on-start cell=rule seen=2 detector=none
value: The plugin directory is rewritten on every start.
basis: fct-vey5bl3d
because: A start no longer writes an instance settings.json (Retired (1)), so the rule keeps the plugin rewrite alone.
@row_set n=2 row=decision/recorder-scrub cell=except seen=3 detector=none
value:
basis: fct-tls6mbvr
because: No fixture holds a workspace.repo once the StatusLine variants left, so the except loses its subject (the P5 card's approved row, directive inputs#I10 item 3).
