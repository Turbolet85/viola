@row_set n=1 row=contract/run-terminal-output/child-output cell=promise seen=1 detector=none
value: run writes nothing to the terminal except the child's own output and, before the child, a keyed start's automatic-verify lines and the verify child's own output, none under revive --json; while the child runs, neither run nor a revived start writes anything of its own.
basis: adaptation:4
because: A start with verify_new_cli on writes its automatic-verify lines and lets the verify child's output reach the terminal before the child starts, so the only-the-child promise must name them.
@row_add n=1 row=decision/standing-modal-reading-uninstrumented area=4-span-trace-coverage detector=none
name: Standing-modal reading uninstrumented
rule: The readiness gate's standing-modal reading adds no log line, span field, event or detail code of its own: a modal label quoted above a ready input box lets the send proceed with nothing written, and a standing modal refuses with the existing detail input-not-ready.
basis: adaptation:5
because: The built readiness gate reads a standing modal with no new line, span field or detail, a standing modal keeping input-not-ready, and no row yet says this reading stays uninstrumented.
