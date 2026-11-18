@row_set n=1 row=contract/verify-output/step-line cell=promise seen=1 detector=D-tests-count-literals
value: One [NN/MM] <row id> <row words>  pass|absent|fail line per relied ledger row, MM the relied count; absent is read on the browser-tools-prompt row alone.
basis: fct-xmezyz7c, fct-tlsodzwd
because: The step line's verdict now prints the row word, so the browser-tools-prompt row can end absent, and the contract's closed pass|fail no longer holds.
@row_set n=2 row=contract/verify-output/summary-line cell=promise seen=1 detector=D-tests-count-literals
value: The last stdout line is stamped <ver>  <n> pass  <m> fail, n counting every relied row that counts toward verified, an absent browser-tools-prompt row among them.
basis: fct-tlsodzwd
because: The summary's n now counts the rows that count, an absent row included, which the contract must say.
@row_set n=3 row=decision/version-gate-needs-all-relied-rows cell=rule seen=2 detector=none
value: A CLI version reads verified only when its stamp holds every relied ledger row as counting, pass or, on the browser-tools-prompt row alone, absent; a retired row it also holds changes nothing, and a stamp of only older row ids reads unverified.
basis: fct-xmezyz7c, fct-tlsodzwd
because: The verified rule moved from every relied row pass to every relied row counting, absent counting on the browser-tools-prompt row only.
@row_set n=4 row=contract/local-live-suite/pass cell=promise seen=1 detector=D-tests-obs-harness
value: Passes when verify exits 0, each relied ledger row id names exactly one pass line, or one absent line on the browser-tools-prompt row alone, and the last line reads stamped with zero fail; it claims no live measurement beyond that.
basis: fct-xmezyz7c, fct-spkdhc2a
because: The harness local-live row check now accepts absent on the browser-tools-prompt row, so a pass line on every row is no longer the promise.
@row_set n=5 row=decision/ledger-row-verify-probe cell=rule seen=1 detector=none
value: Every ledger row has a viola verify probe with a post-condition, run locally, reporting an exit code, a pass, absent or fail step line and a stamped line.
basis: fct-xmezyz7c
because: A ledger row's probe can now report absent, so the row's pass/fail reading is incomplete.
@row_add n=1 row=decision/unreached-control-reads-indeterminate area=test-strategy fact=F1 detector=none
name: Unreached control reads indeterminate
rule: A remove-the-guard control whose run stops before its named case reads indeterminate, neither holds nor fails.
why: A fail-fast nextest run stops at the guard's failing neighbours before the named case is run.
basis: fct-dzlyisot
because: The chunk met a control that never reached its case on a fail-fast run, and the reading of such a control's verdict binds every later control.
@row_add n=2 row=decision/rig-reads-cache-key-name-only area=6-e2e-test-strategy fact=F2 detector=none
name: Live rig reads the extension cache key by name only
rule: The live rig reads the extension cache key cachedChromeExtensionInstalled only by its name in its own private scratch config, after verify exits and before teardown, printing cache-a written, absent or unread, and never reads its value.
why: Ratified as a bounded widening of what a test may read of the CLI's config, presence of the key alone.
basis: fct-h74wljzv, fct-xmezyz7c
because: The rig gained a ratified read of the CLI's own config, and its bound is a decision later live rounds must respect.
@row_none n=1 fact=F3 type=8
what: The scope of one live measurement, the prompt literal read on 2.1.325 under one host's conditions; the per-version ledger stamps and the run's evidence hold it.
