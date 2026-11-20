@row_set n=1 row=decision/no-screen-content-parsing cell=rule seen=6 detector=none
value: The rendered `claude` TUI screen is parsed only for compiled signatures (footer literals, the input box's rule and prompt rows, modal literals and a dialog-option row's shape), the prompt row only for its opening char, never the text after it; screenshots are never taken as a11y evidence (R7).
basis: fct-xxy4my3t
because: The readiness predicate now reads a modal literal as standing only when its row has the compiled dialog-option shape or no input-box row stands below it, so the signatures read now include that row shape and the row must say so.
