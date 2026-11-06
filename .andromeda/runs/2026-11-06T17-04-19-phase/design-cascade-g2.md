@carried n=1 label=design-system/color-palette fact=fct-dxnsp32p
why: the `portable-pty` move and the remnant removal must leave the CLI's one attention span exactly so, amber on the `DIALOG` word of `viola list` alone, every gate re-asserting it green.
quote: The only attention colour: the 4px holder band and the `DIALOG` word of a cocked strip, and the CLI `DIALOG` word in `viola list` (the kind word after it stays uncoloured).
@carried n=1 run=2 label=design-system fact=fct-w6mrbusn
quote: Accent #D97706 arrival amber, the only attention colour, never for errors, warnings, budget or hover
@carried n=1 run=3 label=design-system fact=fct-sjonlmz3
quote: **Arrival-holder amber #D97706:** `dialog_pending` only.
@carried n=2 label=design-system/color-palette fact=fct-mbty7rip
why: the live session's send counts only once read back, by a matching `prompt-submitted` or a met ledger post-condition, and its evidence must read that state, never presume it.
quote: Readback::read (matching `prompt-submitted`, or ledger post-condition met) | #E6D8AE | #E6D8AE | #3B3A36 in the box / #E6D8AE word | Box filled, `RB` inside, word `read back`. Instant, no transition.
@carried n=2 run=2 label=design-system/color-palette fact=fct-7zwwnytz
quote: Readback::open (from CL-1 `send-issued`) | #1E2124 | #E6D8AE (1px) | #E6D8AE | Empty square, then the word `open`
@carried n=2 run=3 label=design-system/color-palette fact=fct-ynfdj6qo
quote: Readback::unconfirmable (`ok`, `confirmed:false`) | #1E2124 | #E6D8AE | #E6D8AE | The open drawing, never filled, with the word `unconfirmable`
@carried n=2 run=4 label=design-system/color-palette fact=fct-jyi6cq2d
quote: Success, confirmed after the fact (read back)
@carried n=2 run=5 label=design-system/color-palette fact=fct-7jomlrim
quote: Warning, degraded but not refused (`unverified-cli`, `unconfirmable`, expired budget window)
@carried n=2 run=6 label=design-system fact=fct-juf7qttk
quote: a refused send folds into its send line, never a separate line; `turn-ended` never fills a readback box
@carried n=3 label=design-system/color-palette fact=fct-34xpzqyz
why: a live send that is not read back must be evidenced as `unable` plus its typed reason and detail, never as a silent miss.
quote: Readback::refused (from CL-1 `send-refused`) | #1E2124 + 1px #E6D8AE `/` strike | #E6D8AE | #E6D8AE | Strike, then `unable`, then `reason · detail`
@carried n=3 run=2 label=design-system/color-palette fact=fct-lib6c2wc
quote: Error, "unable" plus a reason (refusal, HTTP problem)
@carried n=4 label=design-system/color-palette fact=fct-aglfnzk3
why: the founder's word on the board's read rule (item 7) rules over these status words, and the master text must agree with whichever he rules: `idle` or `busy` from a joined row, else `unknown`.
quote: SessionStatus::unknown | #2A2E33 | #9C9278 | #E6D8AE | `unknown` printed in the box (`claude agents --json` missing, failed or not joined)
@carried n=4 run=2 label=design-system/color-palette fact=fct-e4xmrwl5
quote: SessionStatus::idle | #2A2E33 | #9C9278 | #E6D8AE | `idle`
@carried n=4 run=3 label=design-system/color-palette fact=fct-ypt4sxmn
quote: SessionStatus::busy | #2A2E33 | #9C9278 | #E6D8AE | `busy`. No spinner, pulse or animation.
@carried n=4 run=4 label=design-system/color-palette fact=fct-zx6x66nl
quote: Wrapped::false | #1E2124 | #4A5057 (1px dashed outline, no band slot) | #E6D8AE (name, live, status) / #9C9278 (`n/a`) | Bare strip, another facility's traffic, in the UNWRAPPED · READ-ONLY rack. The name is shown as text, never as a target.
@binding n=1
**color-palette Accent ↔ cli tokens and toolkit (body)**: the CLI's one amber span, the `DIALOG` word with its kind word uncoloured (design-system/color-palette fct-dxnsp32p), stays the SGR module's own, emitted only on a terminal by the colour decision order (design-system fct-dufqiv2l), and the `portable-pty` move adds no dependency reading a colour or terminal variable, clap staying without its `color` feature (design-system fct-3n5f23up).
@binding n=2
**color-palette Readback rows ↔ cli-send mirror and the Readback anchor (body)**: the live session's evidence reads the CLI twin of the readback states, `[RB] read back` with its cursor, `[  ]` `open` / `unconfirmable`, or `[/ ] unable` plus reason and detail (design-system fct-ahmeujab), a send counting only once read back (design-system fct-l76aktpo), its states and their words those of the Readback rows (design-system/color-palette fct-mbty7rip).
@binding n=3
**color-palette SessionStatus and Wrapped rows ↔ cli-list as-built rule (body)**: the founder's word on the board's read rule rules over the as-built `viola list` rule, a joined wrapped row `idle` or `busy`, else `unknown`, unwrapped rows after the read-only separator (design-system fct-mfvo2or6); the SessionStatus and Wrapped::false rows (design-system/color-palette fct-aglfnzk3) must state the same rule once he rules.
