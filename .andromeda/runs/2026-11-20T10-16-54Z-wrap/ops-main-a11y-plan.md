@row_set n=1 row=decision/no-screen-content-parsing cell=rule seen=6 detector=none
value: The rendered `claude` TUI screen is parsed only for compiled signatures (footer literals, the input box's rule and prompt rows, modal literals, the compiled shapes over them), the prompt row only for its opening char, never the text after it; screenshots are never taken as a11y evidence (R7).
basis: fct-xxy4my3t, fct-pn22p663
because: The readiness predicate now reads the compiled shapes over the compiled literals (a modal label row's option shape; the box's rows below it), so the signatures the screen is parsed for include them.
