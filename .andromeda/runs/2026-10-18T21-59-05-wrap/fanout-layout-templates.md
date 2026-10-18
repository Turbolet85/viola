proposals:
  - detector: D-layout-surface
    severity: warning
    section: "§Surface: cli → Output structure — `viola verify`"
    change: "The Unix wireframe reads `(a Unix host: 24 relied rows)`, every step line `[NN/24]`, a new last step line `[24/24] local-command-compact /compact keeps the session id and submits no prompt  pass` after `[23/24] agents-join …`, and the summary `stamped 2.1.287  24 pass  0 fail`."
    sidecar: "Output structure — `viola verify`: the wireframe gains the twenty-fourth step line `local-command-compact` (`LedgerRow::ALL` 23 → 24, relied 24 on Unix), `[NN/23]` → `[NN/24]`, summary `24 pass  0 fail` (chunk 2026-10-18-compact-post-condition-ledger-row)."
    rationale: "The report's Symbols / APIs add `LedgerRow::LocalCommandCompact`, the twenty-fourth and last member of `LedgerRow::ALL`, relied on every OS (24 on Unix / 21 on Windows); Counts `rows-count` names layout-templates 173-191 (the 23 step lines and the stamped line) as changed; Outcome: `viola verify` prints exactly twenty-four `[NN/24]` lines, the last `[24/24] local-command-compact …`, then `stamped <ver>  24 pass  0 fail`. The verify output gains a row the wireframe does not show."
    basis: ".andromeda/layout-templates.md:173-191"
  - detector: D-layout-surface
    severity: warning
    section: "§Surface: cli → Output structure — `viola verify`"
    change: "The failing-row example summary reads `stamped 2.1.287  22 pass  2 fail` (the counts summing to the twenty-four Unix relied rows), not `21 pass  2 fail`."
    sidecar: "Output structure — `viola verify`: the failing-row example summary `21 pass  2 fail` → `22 pass  2 fail`, its sum the twenty-four relied rows (chunk 2026-10-18-compact-post-condition-ledger-row)."
    rationale: "The same wireframe block's second example summary sums to the retired 23-row count; with `LedgerRow::relied()` at 24 on Unix (report Symbols / APIs, Counts `rows-count`) a two-fail run reads 22 pass. The report's sweep did not list this line, so it is a duplicate occurrence of the count the primary retires."
    basis: ".andromeda/layout-templates.md"
    dependent-of: D-layout-surface
  - detector: D-layout-surface
    severity: warning
    section: "§Surface: cli → Output structure — `viola verify`"
    change: "The Usage paragraph's ledger-order list ends `… · resume-session · agents-join · local-command-compact`; `MM` is 24 on Unix and 21 on Windows, where the relied set skips the three statusline rows and ends at `local-command-compact`."
    sidecar: "Output structure — `viola verify` Usage: `local-command-compact` appended to the ledger-order list; `MM` 23 / 20 → 24 / 21; the Windows relied set ends at `local-command-compact` (chunk 2026-10-18-compact-post-condition-ledger-row)."
    rationale: "Report Symbols / APIs: `LocalCommandCompact` is the last member of `LedgerRow::ALL: [Self; 24]`, relied on every OS (no arm in `relied_on`), so `LedgerRow::relied()` is 24 on Unix and 21 on Windows (was 23 / 20); Counts `rows-count` names layout-templates 202 as changed, and expected amendment 7 names the row list and `MM` with the `LedgerRow::relied()` rule."
    basis: ".andromeda/layout-templates.md:202"
    dependent-of: D-layout-surface
  - detector: D-retired
    severity: warning
    section: "§Surface: cli → Signature placement"
    change: "Item 2 reads: `[RB] read back …` with exit 0 when the send is confirmed, by the matching `prompt-submitted` or, on a verified CLI version, for `/clear` by its new-session post-condition (a `session-start` with cause `clear` and a new session id) and for `/compact` by its compacted-session post-condition (a `session-start` with cause `compact` and the session's own id, inside its own 40 s PROVISIONAL window, else `not-delivered` / `no-prompt-submitted`); `[  ] unconfirmable …` with exit 0 for a local command with no measured post-condition (`/remote-control`, and `/compact` on an unverified CLI version)."
    sidecar: "Signature placement item 2: `/compact` on a verified CLI is now confirmed by its same-session `session-start` of cause `compact` (`COMPACT_CONFIRM_WINDOW` 40 s PROVISIONAL), no longer among the unmeasured local commands (chunk 2026-10-18-compact-post-condition-ledger-row)."
    rationale: "Report Retired: the '`/compact` reads `unconfirmable` at once on a verified CLI' behaviour — now confirmed by its same-session SessionStart or expiring `not-delivered` / `no-prompt-submitted` at 40 s; Symbols / APIs: `PostCondition::CompactedSession`, `LOCAL_COMMANDS` `/compact` → `Some(CompactedSession)`, `/remote-control` keeps `None`, unverified `/compact` stays `Confirms::Nothing`; Outcome: a human-mode `viola send` of `/compact` on a stamped home prints `[RB] read back` with its cursor. Item 2 names `/clear` as the only post-condition-confirmed local command, so `/compact` still reads as an unmeasured local command; expected amendment 7 names this site (`readback` layout-templates 44-53)."
    basis: ".andromeda/layout-templates.md:44-53"
note: "The `viola send` wireframe's `remote.txt` unconfirmable example (layout-templates 118) and the hero component (303-307) stay true (the report's `readback` class: still printed for `/remote-control` and for `/compact` on an unverified CLI); no new surface beyond the verify step line was added."
