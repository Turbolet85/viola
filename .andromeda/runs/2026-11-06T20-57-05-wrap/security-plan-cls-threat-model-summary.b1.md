@answer n=1 entity=threat-model-summary#1.1 v=no
why: the session added no log field or event kind, so the prompt and reply lines in events.ndjson it names are unchanged
@answer n=2 entity=threat-model-summary#1.2 v=no
why: the permission, plan and question event shapes gained no field and no kind, so the tool-call and dialog data it names are unchanged
@answer n=3 entity=threat-model-summary#1.3 v=yes mode=add-to
why: ledger/stamps.json is what switches on dialog answering, and statusline-rate-limits can read red on an unchanged CLI so a verify may write no stamp (indirect, task item 9)
@answer n=4 entity=threat-model-summary#1.4 v=no
why: the session reads and stores no credential, the R8 CLAUDE* strip is unchanged (untouched item 2), and no measurement is added
@answer n=5 entity=threat-model-summary#1.5 v=no
why: the budget.json and snapshot fields it names are unchanged and the session touched no operational metadata
@answer n=6 entity=threat-model-summary#2.1 v=no
why: the session adds no method or endpoint, and the socket directory 0700 and socket 0600 were re-read unmoved (task item 14)
@answer n=7 entity=threat-model-summary#3.1 v=no
why: validate_paste_text and the paste framing are unchanged (untouched item 3), the send change being only the retired rewritten-path warning
@answer n=8 entity=threat-model-summary#4.1 v=no
why: the terminal stdin classifier is unchanged and viola-pty passes 33 of 33 on =0.9.0 (task item 16)
@answer n=9 entity=threat-model-summary#5.1 v=no
why: hook parsing, fail-open and the hook statusline shell-out are unchanged (untouched item 6), the hook.rs change being a Windows-only removal
@answer n=10 entity=threat-model-summary#6.1 v=no
why: the MCP tool schemas and the strict-modes and socket checks on the MCP path are unchanged, the session removing only Windows-only remnants
@answer n=11 entity=threat-model-summary#7.1 v=no
why: MAX_FRAME and the CappedLines reader are unchanged (untouched item 3)
@answer n=12 entity=threat-model-summary#8.1 v=no
why: --home and VIOLA_DIR still choose the state tree, the canonical_home change dropping only a Windows verbatim-drive prefix
@answer n=13 entity=threat-model-summary#9.1 v=no
why: the pinned bin/<version>-<hash>/viola and the plugin folders are re-read unmoved (task item 14), and the EXE_SUFFIX removal was empty on Linux
@answer n=14 entity=threat-model-summary#10.1 v=no
why: the session removed only Windows-only remnants and the pinned copy is re-read unmoved (task item 14)
@answer n=15 entity=threat-model-summary#11.1 v=no
why: bare-name claude resolution and the run --version probe are unchanged, the removed EXE_SUFFIX join being empty on Linux and no suffix cited
@answer n=16 entity=threat-model-summary#12.1 v=yes mode=stale
why: the deny.toml advisories ignore list is now empty and the graph lost the serial crates with nix at 0.28.0, so the cargo deny reading it names has moved (task item 10)
@answer n=17 entity=threat-model-summary#13.1 v=no
why: no CI workflow, toolchain pin or release gate changed (untouched item 5), and the gate readings are re-read unmoved (task item 16)
