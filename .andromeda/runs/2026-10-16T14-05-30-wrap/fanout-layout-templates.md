proposals:
  - detector: D-layout-surface
    severity: warning
    section: "§Surface: cli → Output structure — `viola verify` (the example block)"
    change: "The Unix example block reads 22 relied rows: header `(a Unix host: 22 relied rows)`, every counter `[NN/22]`, a new last step line `[22/22] resume-session --resume <id> reopens the session under that id  pass` after `statusline-rate-limits`, and the summary `stamped 2.1.287  22 pass  0 fail`."
    sidecar: "layout-templates §Output structure — `viola verify`: the example block gains the `resume-session` step line and moves 21 → 22 relied rows on Unix (chunk 2026-10-14-resume-and-compact-ledger-rows)."
    rationale: "The report adds a new verify output line: Coverage of new surfaces names `The resume-session step line (CLI output)`; Symbols make `LedgerRow::ResumeSession` the 22nd and last member of `LedgerRow::ALL`, id `resume-session`, words `--resume <id> reopens the session under that id`, relied on every OS; Counts move the relied set 21 → 22 on Unix and the stamped line to `stamped <ver>  22 pass  0 fail`, with site `rows21` layout-templates.md:476–493 standing. The example block shows no such line, so the new region is undocumented."
    basis: "crates/viola-agent-claude/src/ledger.rs:43, :96, :128; layout-templates.md:476–493"
  - detector: D-layout-surface
    severity: warning
    section: "§Surface: cli → Output structure — `viola verify` (the Usage paragraph)"
    change: "The ledger-order list ends `… · statusline-shell · statusline-rate-limits · resume-session`; `MM` is 22 on Unix and 19 on Windows, where the relied set skips the three statusline rows (the status line is not wrapped there) and ends at `resume-session` — retiring `18 on Windows, which stops at paste-newline`."
    sidecar: "layout-templates §Output structure — `viola verify` Usage: `resume-session` appended to the ledger order; MM 22 Unix / 19 Windows, the Windows set skipping the statusline rows and ending at `resume-session` (chunk 2026-10-14-resume-and-compact-ledger-rows)."
    rationale: "Same new step line; the report's Counts bullet `Ledger rows` moves the relied set 21 → 22 on Unix and 18 → 19 on Windows, MM = 22 / 19, and lists layout-templates.md:504 under both `rows21` and `rows18`; `relied_on` leaves `ResumeSession` under `_ => true`, so on Windows it follows `paste-newline` past the three Unix-only statusline rows and the claim `stops at paste-newline` no longer holds."
    basis: "crates/viola-agent-claude/src/ledger.rs:43, :47, :69; layout-templates.md:504"
    dependent-of: D-layout-surface
  - detector: D-layout-surface
    severity: warning
    section: "§Surface: cli → Output structure — `viola verify` (the failing-row example)"
    change: "The failing-row example's summary reads `stamped 2.1.287  20 pass  2 fail` (summing to the 22 Unix relied rows), not `19 pass  2 fail`."
    sidecar: "layout-templates §Output structure — `viola verify`: the failing-row example's summary re-summed to 22 rows (chunk 2026-10-14-resume-and-compact-ledger-rows)."
    rationale: "A count reader of the same claim: `19 pass  2 fail` sums to the retired 21-row Unix relied set; the report's Counts bullet `Ledger rows` moves it to 22 on Unix (19 on Windows), which `19 + 2` matches on neither OS. The report states no line for this example, so none is cited."
    dependent-of: D-layout-surface
note: "`/compact` (Signature placement, report's expected amendment) needs no edit — the outcome-line wording `[  ] unconfirmable … for an unmeasured local command` already covers it generically, and the report's `local-cmd` pattern reads 0 rows in layout-templates.md."
