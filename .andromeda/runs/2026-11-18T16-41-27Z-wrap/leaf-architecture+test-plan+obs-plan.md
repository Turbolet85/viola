@leaf_group group=architecture+test-plan+obs-plan
@leaf_read leaf=.claude/docs/commands.md lines=89
last: - `scripts/agent-run.sh logs --instance <name> | jq -c 'sele
@leaf_edit n=1 leaf=.claude/docs/commands.md line=23 class=omission
leaf-text: print `[NN/MM]` step lines (`MM` the relied count
source: ctr-w52tbxph «contract/viola-verify-output/step-line · viola verify stdout · step line — One line per relied row, [NN/MM] <row id> <row words>  pass|absent|fail, MM being the relied row count.»
replace-with: print `[NN/MM]` step lines ending `pass`, `absent` or `fail` (`absent` on the browser-tools-prompt row alone, when no run met the CLI's browser-tools prompt; `MM` the relied count
@leaf_edit n=2 leaf=.claude/docs/commands.md line=23 class=omission
leaf-text: + the `stamped <ver>  <n> pass  <m> fail` summary on stdout
source: ctr-w3mgefkp «contract/viola-verify-output/summary-line · viola verify stdout · summary line — stamped <ver>  N pass  M fail, N counting every relied row that counts toward verified, an absent browser-tools-prompt among them.»
replace-with: + the `stamped <ver>  <n> pass  <m> fail` summary on stdout (`<n>` counting every relied row that counts toward verified, an absent browser-tools-prompt among them)
@leaf_edit n=3 leaf=.claude/docs/commands.md line=24 class=stale
leaf-text: (`LedgerRow::relied()`: all twenty-three, on every build) `pass`:
source: ctr-z3nfk3bf «contract/ledger-stamps/verified-version · ledger stamps envelope · verified version — A CLI version is verified only when every relied ledger row reads `pass`, or `absent` on `browser-tools-prompt` alone.»
replace-with: (`LedgerRow::relied()`: all twenty-three, on every build) `pass`, or `absent` on `browser-tools-prompt` alone:
@leaf_edit n=4 leaf=.claude/docs/commands.md line=24 class=stale
leaf-text: a stamp's `rows` maps each row id to `pass` or `fail`, but trust-record holds
source: ctr-ufkcrgxb «contract/ledger-stamps/rows · ledger stamps envelope · rows — Maps each ledger row id to "pass" or "fail"; browser-tools-prompt may also read "absent", and trust-record holds the trust writer's code»
replace-with: a stamp's `rows` maps each row id to `pass` or `fail`, browser-tools-prompt may also read `absent`, and trust-record holds
