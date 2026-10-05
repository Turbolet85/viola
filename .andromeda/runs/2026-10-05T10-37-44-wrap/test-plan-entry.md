
## 2026-10-05-real-cli-verify-probes — ten ledger rows, verify's interactive runs and the screen fixture class
**Section:** §2 Test pyramid (Contract row) · §3 → 5-command implementation (`boot` steps 4 and 5, `--local-live`) · §3 → Bootstrap phases (nextest) · §4 viola-agent-claude · §5 CLI (`cli_verify`, `contract_ledger_probes`) · §7 Fake agent · Fixture hygiene · Path owner sites (`send` local command, dialog captures, `permission` e2e)
**Change:**
- §3: boot step 4 runs verify against the fake agent with `--screens --turn-stop --trusted-root <workspace root>` (ten rows), and supervise passes `--screens --trusted-root <cwd>`. `--local-live` checks ten literal row ids (`LEDGER_ROWS`), unit-proven; its live firing at ten rows is owed to "First live test and self-drive".
- nextest `profile.mutants` overrides: was the `viola-e2e` override alone; now it also lists `test(/send_window_/)` and `test(/verify_window_/)` at 15 s × 2 (the ~11 s verify window cases carry no test-side deadline).
- §5: `cli_verify` went from 12 to 22 cases, `[NN/06]` → `[NN/10]`, and covers both interactive runs, the named refusal and no run dir left. `contract_ledger_probes` was "every set"; now 2.1.287 and 2.1.288 are stamped `10 pass  0 fail` and 2.1.283 is drift-only, with literal lists and every dir on exactly one list.
- §7 Fake agent: `--trusted-root`, `--screens` and `--turn-stop` (argv, no env); wrapper boots pass `--screens --trusted-root <cwd>`, and `boot_untrusted` passes no root. Print mode is the print probe's mode, no longer "the single probe".
- §7 Fixture hygiene: the walk adds the 2.1.288 set and walks `Screen.*.json` apart, against `schemas/claude-screen.v1.json`. It also refuses an email-shaped token and a seam-split username. Screens are signature-rows-only and checked scrub-as-detector, refused whole with a named code.
- §2 Contract row: screens are checked against `claude-screen.v1.json`. §4: signatures (`SIGNATURES`) feed the full gate on a verified CLI and verify's settle and record helpers.
- Owner sites: was `:84`; now local-command `unconfirmable` is owed to "Local-command and paste-framing rows", and the relayed dialog captures, the `permission` e2e case and its wake witness are owed to "Dialog rows and re-probe".
**Why:** the chunk built W1 + W5 of the founder's live three-way split (2026-10-05 05:58Z); the remaining work moved to the two new route entries.
**Ref:** .andromeda/runs/2026-10-05T10-37-44-wrap/
