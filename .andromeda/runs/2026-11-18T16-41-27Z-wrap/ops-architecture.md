@row_set n=1 row=decision/version-verified-all-relied-rows cell=rule seen=1 fact=F2 detector=none
value: A version is verified only when every relied ledger row counts, reading pass or, on browser-tools-prompt alone, absent, so a stamp of an older, shorter row set reads unverified.
basis: fct-xmezyz7c
because: The verified rule now reads every relied id through counts, which accepts absent on the browser-tools-prompt row alone, so a rule of every row pass is false.
@row_set n=2 row=contract/ledger-stamps/verified-version cell=promise seen=1 detector=none
value: A CLI version is verified only when every relied ledger row reads `pass`, or `absent` on `browser-tools-prompt` alone.
basis: fct-tlsodzwd
because: The verified rule moved from every relied row pass to every relied row counting, absent on browser-tools-prompt among them.
@row_set n=3 row=decision/capability-ledger cell=rule seen=3 detector=none
value: Any behaviour of the `claude` CLI that viola relies on is a row in a version-stamped capability ledger; a CLI version is verified only when every relied ledger row of its stamp reads `pass`, or `absent` on browser-tools-prompt alone, and `viola verify` re-stamps it.
basis: fct-tlsodzwd
because: An absent browser-tools-prompt row now counts toward verified, so the row's every-row-pass clause is false.
@row_set n=4 row=decision/cli-capability-ledger cell=rule seen=3 detector=none
value: Every load-bearing Claude Code CLI behaviour viola relies on is a row of a closed capability ledger, probed by viola verify and stamped per CLI version; a CLI version is verified only when every relied row of its stamp reads pass, or absent on browser-tools-prompt alone.
basis: fct-tlsodzwd
because: An absent browser-tools-prompt row now counts toward verified, so the row's every-row-pass clause is false.
@row_set n=5 row=contract/ledger-stamps/rows cell=promise seen=2 detector=none
value: Maps each ledger row id to "pass" or "fail"; browser-tools-prompt may also read "absent", and trust-record holds the trust writer's code word instead when no record was written.
basis: fct-vh4fhtvm
because: The stamp gains the browser-tools-prompt row key with the words pass, absent and fail, so a map of pass or fail alone is incomplete.
@row_set n=6 row=contract/ledger-stamps/content cell=promise seen=2 detector=none
value: Per CLI version, the envelope holds each ledger row's `pass` or `fail`, `absent` also for browser-tools-prompt, or for trust-record the trust writer's word when no record was written, and the probe's measured values, of which `run` reads none.
basis: fct-vh4fhtvm
because: The browser-tools-prompt row may be stamped absent, a third word the envelope now holds.
@row_set n=7 row=contract/ledger-stamps/first-screen cell=promise seen=1 detector=none
value: For each of the six interactive verify runs, one closed word naming its first settled screen: trust, imports, browser-tools, box, none or exited; no screen text.
basis: fct-vh4fhtvm
because: The measured.first_screen closed words gain browser-tools, 5 to 6.
@row_set n=8 row=contract/viola-verify-output/step-line cell=promise seen=1 detector=none
value: One line per relied row, [NN/MM] <row id> <row words>  pass|absent|fail, MM being the relied row count.
basis: fct-xmezyz7c
because: The step line now prints the row's word through step_word, pass, absent or fail, so a pass|fail verdict is incomplete.
@row_set n=9 row=contract/viola-verify-output/summary-line cell=promise seen=1 detector=none
value: stamped <ver>  N pass  M fail, N counting every relied row that counts toward verified, an absent browser-tools-prompt among them.
basis: fct-tlsodzwd
because: The summary's n now counts the rows that count, an absent row in n.
@row_set n=10 row=decision/claude-first-no-adapter-trait cell=except seen=4 detector=none
value: Test-only: the rig seeds its private Claude config and reads key names only of it and of its captured UserPromptSubmit payload, plus the source word; the fake agent and rig read a trust record.
basis: fct-h74wljzv
because: The rig now reads whether a key name stands in its own private Claude config, never a value, a test-only exception the row must name.
@row_set n=11 row=decision/claude-shapes-in-one-crate cell=except seen=4 detector=none
value: Test-only: the rig seeds its private Claude config and reads key names only of it and of its captured UserPromptSubmit payload, plus the source word; the fake agent and rig read a trust record.
basis: fct-h74wljzv
because: The rig now reads whether a key name stands in its own private Claude config, never a value, a test-only exception the row must name.
@row_set n=12 row=decision/claude-shapes-isolated cell=except seen=3 detector=none
value: Test-only: the rig seeds its private Claude config and reads key names only of it and of its captured UserPromptSubmit payload, plus the source word; the fake agent and rig read a trust record.
basis: fct-h74wljzv
because: The rig now reads whether a key name stands in its own private Claude config, never a value, a test-only exception the row must name.
@row_add n=1 row=decision/browser-tools-prompt-ledger-row area=established-decisions fact=F2 detector=none
name: Browser-tools-prompt ledger row and its readings
rule: The browser-tools-prompt relied ledger row reads pass when some verify run's first screen is the CLI's browser-tools prompt, absent when no run met it, and fail when any first screen reads none; it is the one relied row whose absent counts toward verified.
basis: fct-xmezyz7c
because: A 24th relied ledger row lands with three readings and its own counting rule, a contract no standing row says.
@row_add n=2 row=decision/browser-tools-prompt-never-answered area=established-decisions fact=F1 detector=none
name: Browser-tools prompt never answered or seeded
rule: On a host where the CLI's browser-tools prompt stands unanswered, viola neither answers nor seeds it: verify types nothing, the paste rows and trust-record fail, and the CLI version stays unverified until a person answers the prompt once.
basis: fct-qlgfl5fh
because: The chunk leaves the browser-tools prompt to the human, which keeps a version unverified on such a host, a decision no standing row says.
@row_add n=3 row=decision/modal-signature-check-untrusted-start-modals area=established-decisions fact=F3 detector=none
name: Modal-signature check reads the untrusted-start modals
rule: The modal-signature ledger check at an untrusted start reads only the untrusted-start modals, trust and imports; the browser-tools literal is a modal for verify's settle, the pre-send gate and Run F, never for that check.
basis: fct-xmezyz7c
because: ModalSignature and Run A's trust-record clause now read the untrusted-start slice while the whole modal set serves the settle, the gate and Run F, a split no standing row says.
