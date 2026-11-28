# layouts · standing conventions

- S1 · every changed human output line (the console key line, a start or refusal line) is pinned as a literal in a test, and the literal changes in the same change as the line · surface-console · surface-cli
- S2 · every new refusal carries a hint that names the next verb, never quotes text or names a path, and is the last stderr line · surface-cli
  contract/viola-send-output/paste-cut-hint · viola send output · paste-cut hint — An unable not-delivered paste-cut outcome, on its own struck mirror line, ends stderr with `hint: <name> did not take the whole text in time; a human must look at the session before the next send`; the answer hints are unchanged.
  contract/viola-verify-refusals/stopped · viola verify refusals · verify stopped — A run stopped by Ctrl-C or TERM is exit 1, stdout empty, no summary, `unable: verify was stopped before it finished` / `hint: run viola verify again to stamp this CLI`, the hint last on stderr.
  decision/hints-never-quote-or-locate · Hints never quote text or name paths — A hint never quotes the sent text or any upstream text, and never names a path or a pid.
