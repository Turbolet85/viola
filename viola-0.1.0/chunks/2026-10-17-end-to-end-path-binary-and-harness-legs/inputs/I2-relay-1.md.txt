Question put at /implement P2 (2026-10-17, AskUserQuestion, header "E2E naming"): the plan names its two new binaries
`path1_run_start` and `path6_budget_harness`, but test-plan §3's E2E regex `^(path|tui|mcp|cross|chaos|contract)_`
(and testing.md's file form `{path,mcp,cross}_<topic>.rs`) matches only `path_…`, so both land in the integration
layer and entries 11 and 12 go red — widen the regex, rename the files, or stop?

Answer selected: "Widen the regex" — change the harness constants to `^(path[0-9]*|tui|mcp|cross|chaos|contract)_`;
the plan's names and entries 10–12 stay as written; test-plan §3's regex (and the a11y/testing copies) surfaced for
the wrap to amend.

Notes given with the answer, verbatim:

Overseer, technical (founder-delegated): numbered path binaries name the test plan's numbered Paths; the widened pattern matches no other binary today. The wrap amends test-plan §3's regex and the testing.md / a11y copies to the same form..
