@row_set n=1 row=decision/no-screen-content-parsing cell=rule seen=1 fact=F1 detector=none
value: The rendered `claude` TUI screen is parsed only for compiled signatures (footer literals, the input box's rule and prompt rows, modal literals), the prompt row only for its opening char, never the text after it; screenshots are never taken as a11y evidence (R7).
basis: fct-xdv7p2qy, fct-rss6xvgu
because: The readiness signatures now include the input box's rule and prompt rows, ratified as a boundary widening, so the row bounding what the vendor screen is parsed for must name them and the prompt row's opening-char-only read.
