
## 2026-10-13-owed-ledger-rows — three statusline ledger rows and the per-OS relied set
**Section:** §Established Decisions → [CLI Version Compatibility]; §Standard Contracts → Ledger stamps envelope, hook statusline; §Occupied Resources → Claude Code integration names (settings override); §Cross-cutting Patterns → Capability ledger; registries `ci-cd-approach`, `crate-dependency-direction`
**Change:**
- Was eighteen rows; now twenty-one, the members of `LedgerRow::ALL`, ending `paste-newline` · `statusline-override` · `statusline-shell` · `statusline-rate-limits`.
- The relied set is `LedgerRow::relied()`: `ALL` filtered by `relied_on(row, statusline::WRAPPED_HERE)`. All twenty-one are relied on Unix, the first eighteen on Windows.
- A version is verified only when every relied row reads `pass` (was every landed row). Unix reads an all-`pass` stamp of the eighteen pre-chunk ids as unverified, Windows as verified.
- `MM` in the `[NN/MM]` step lines is `LedgerRow::relied().len()`: 21 on Unix, 18 on Windows.
- `Probes` gains `statusline`. Its three checks:
  - override: at least one capture;
  - shell: at least one capture, every verdict `bin-sh`;
  - rate-limits: a capture whose `rate_limits.five_hour` and `.seven_day` each hold an integer `used_percentage` 0–100 and an integer `resets_at` (`whole_rate_limits`).
- `--record` also writes `StatusLine.default-1.json` and `StatusLine.rate-limits-1.json` (`ledger::statusline_variants`).
- The statusline override, `/bin/sh` and `rate_limits` are now rows. They were "relied on with no row", and `budget-paused` now rests on a stamped row. They are stamped on live 2.1.287 on the Linux dev host (`21 pass  0 fail`). Their Windows legs are owed to "Windows-only live measurements".
- A single relied dependence still has no row: `viola revive`'s `--resume`.
- `cli_verified` is read over the relied set. The hypothetical PermissionRequest body row is now the 22nd.
- CI/CD: verify covers the relied rows. Crate dependency direction: `ledger` holds the twenty-one rows, `relied()`, `ShellVerdict`, `StatusLineCapture` and `statusline_variants`; `statusline` adds `WRAPPED_HERE`, `BIN_SH`, `SHELL_FLAG` and `probe_document`.
**Why:** The three shapes are relied on only where the status line is wrapped. Shipping them on Windows would leave every Windows home unverified, and passing them there would claim an unmeasured shape. The founder chose the cut (P4 fork 1, `inputs#I1`, relayed verbatim by the operator). Counts carry their rule (`LedgerRow::ALL`, `relied()`).
**Kept:** the paste-newline chunk's dated `stamped 2.1.287  18 pass  0 fail` readings stand as history.
**Ref:** .andromeda/runs/2026-10-14T06-37-54-wrap/
