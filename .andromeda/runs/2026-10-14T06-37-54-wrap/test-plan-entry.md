
## 2026-10-13-owed-ledger-rows — the relied set, the statusline arms, the StatusLine fixtures and the kept workspace.repo
**Section:** §2 (contract trigger, Contract row); §3 → 5-command implementation (boot step 4, `run --local-live`); §4 viola-agent-claude; §5 `cli_verify`, `contract_ledger_probes`; §7 Recorded hook payloads, Fake agent, Fixture hygiene; §11 Test Data
**Change:**
- Step lines: one `[NN/MM]` per relied row, `MM` = `LedgerRow::relied()`'s length (21 on Unix, 18 on Windows). Was eighteen.
- `cli_verify`'s per-OS literals:
  - no framing replay: `17 pass  4 fail` on Unix, `17 pass  1 fail` on Windows;
  - tag-turn modal: `19 pass  2 fail` on Unix, `16 pass  2 fail` on Windows.
- `contract_ledger_probes` lists each relied id once (`ROW_IDS` 21) and ends at the relied count.
- Boot stamps every relied row; on Unix, Run B carries its one probe `--settings`.
- `run --local-live`'s `LEDGER_ROWS` is `[&str; 21]` under `cfg(unix)` and the 18-id form under `cfg(not(unix))`, keeping it out of the Windows mutation scope. The 21-row live firing is recorded (`passed 1`, census empty).
- §4: three statusline arms with nine `check_statusline_*` cases, beside the shell-verdict, variant, file-name and relied-set cases.
- Fixtures:
  - §2: the drift contract replays the `StatusLine` variants byte for byte under `--framing`, and jsonschema checks them against `schemas/claude-statusline-fixture.v1.json`.
  - §7: two variants under `2.1.287`, scrubbed, refused if unclean and hygiene-walked.
- The fake agent's `--framing` replays them through an absolute `--settings` command. `"$0"` is handed over as `/bin/sh`, with no shell. The first-Stop replay comes after the turn screen, and no option is added. The resume payload is owed to "Resume and compact ledger rows" (was "Owed ledger rows").
- §7 Fixture hygiene and §11: a `workspace.repo` naming this repository's own remote is kept in a fixture. The scrub does not rewrite it and the walk does not refuse it. Was: fixtures scrubbed of usernames, no exception.
**Why:**
- Run B's probe `--settings` is the founder's own answer to P4 fork 3, relayed verbatim by the operator (`inputs#I1`). The wrap's card confirm is PROVISIONAL (delegate viola-overseer).
- The kept `workspace.repo` is his own word (`inputs#I3`, relayed verbatim by the operator, given after it was shown to him), worded by `inputs#I4`. Nothing else in the hygiene rule widens.
**Kept:** `:753`'s `stamped 2.1.287  18 pass  0 fail` is a dated reading; "eighteen verify-driven binaries" counts nextest binaries.
**Ref:** .andromeda/runs/2026-10-14T06-37-54-wrap/
