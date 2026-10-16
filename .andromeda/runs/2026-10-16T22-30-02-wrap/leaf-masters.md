group: masters
read: CLAUDE.md · 127 · «<!-- USER:session-learnings end -->»

- leaf: CLAUDE.md:42   class: stale
  leaf-text: «the three statusline rows and the `resume-session` row included: of the twenty-two rows (`LedgerRow::ALL`), the relied set (`LedgerRow::relied()`: all twenty-two on Unix; on Windows, where the status line is not wrapped, nineteen, the first eighteen and `resume-session`) gates `cli_verified`»
  source: .andromeda/architecture.md:88 «so a Unix host relies on all twenty-three rows and a Windows host on twenty, the first eighteen, `resume-session` and `agents-join`, the last two relied on every OS»
  replace-with: «the three statusline rows, the `resume-session` row and the `agents-join` row included: of the twenty-three rows (`LedgerRow::ALL`), the relied set (`LedgerRow::relied()`: all twenty-three on Unix; on Windows, where the status line is not wrapped, twenty, the first eighteen, `resume-session` and `agents-join`) gates `cli_verified`»

- leaf: CLAUDE.md:42   class: omission
  leaf-text: «probed by verify's Run E, stamped on 2.1.287 on the Linux dev host and relied on every OS (its Windows reading owed on "Windows-only live measurements");»
  source: .andromeda/architecture.md:406 «is measured by row `agents-join`, probed by Run B's one agents read, relied on every OS, stamped live on 2.1.287 on the Linux dev host (`stamped 2.1.287  23 pass  0 fail`), its Windows reading owed»
  replace-with: «probed by verify's Run E, stamped on 2.1.287 on the Linux dev host and relied on every OS (its Windows reading owed on "Windows-only live measurements"); `claude agents --json` naming a live session by `pid` and `sessionId` (the join `viola list` and `viola revive` read, never gated on a stamp) is the row `agents-join`, probed by Run B's one agents read, stamped on 2.1.287 on the Linux dev host and relied on every OS (its Windows reading owed on "Windows-only live measurements");»

- leaf: CLAUDE.md:27   class: stale
  leaf-text: «`list` (`src/cmd/list.rs`: wrapped rows only, uncoloured, the strict-modes scan before any log, exit 21 refusing the whole board)»
  source: .andromeda/registries/contracts/architecture/project-directory-structure.md:28 «list.rs: the caller-side board (the strict-modes scan before any log, then one `claude agents --json` read joined into it — STATUS idle/busy, the unwrapped rows — then the `--json` document»
  replace-with: «`list` (`src/cmd/list.rs`: uncoloured, the strict-modes scan before any log, then one `claude agents --json` read joined into the board — STATUS `idle` / `busy` on joined wrapped rows, the read-only unwrapped rows under `-- UNWRAPPED - READ-ONLY --`, a failed read leaving the board as read — exit 21 refusing the whole board)»

- leaf: CLAUDE.md:27   class: omission
  leaf-text: «`revive` (a dead instance resumed in place through `run`'s start)»
  source: .andromeda/architecture.md:129 «one `claude agents --json` read lists the logged id live in another `claude`; skipped under `--fork`; a failed read passes»
  replace-with: «`revive` (a dead instance resumed in place through `run`'s start; its preflight refuses `session-live` when one `claude agents --json` read lists the logged session live in another `claude`, skipped under `--fork`, a failed read passing)»

- leaf: CLAUDE.md:27   class: omission
  leaf-text: «the link set (`src/run/links.rs`:»
  source: .andromeda/registries/contracts/architecture/project-directory-structure.md:35 «agents.rs: the bounded `claude agents --json` read (`read_logged`, `read_here`) shared by `list`, `revive` and verify's Run B;»
  replace-with: «the bounded `claude agents --json` read (`src/run/agents.rs`: a direct spawn under the R8 strip, `take(MAX_FRAME + 1)`, killed at 5 s, shared by `list`, `revive` and verify's Run B), the link set (`src/run/links.rs`:»

- leaf: CLAUDE.md:23   class: omission
  leaf-text: «the board read model (`board.rs`: `scan` / `read`, read by `viola list`)»
  source: .andromeda/registries/contracts/architecture/project-directory-structure.md:68 «`SkippedTotal`, `RowLiveness`, `SessionStatus`, `ScanRefused`, and the agents join: `JoinKeys`, `Observation`, `join`,»
  replace-with: «the board read model (`board.rs`: `scan` / `read` and the agents join `join`, read by `viola list`)»

- leaf: CLAUDE.md:24   class: omission
  leaf-text: «the vt100 screen model and readiness verdict, the resume argument shapes»
  source: .andromeda/registries/contracts/architecture/project-directory-structure.md:73 «the pure `agents` module (the `claude agents --json` parse)»
  replace-with: «the vt100 screen model and readiness verdict, the pure `agents` module (the `claude agents --json` parse into rows or the closed `AgentsFailure`), the resume argument shapes»

- leaf: CLAUDE.md:15   class: omission
  leaf-text: «and `StatusLine.<stem>-<n>.json` status line variants recorded by `viola verify`»
  source: .andromeda/registries/contracts/architecture/project-directory-structure.md:102 «StatusLine.<stem>-<n>.json status line variants and Agents.recorded-<n>.json agents variants recorded by `viola verify`»
  replace-with: «and `StatusLine.<stem>-<n>.json` status line variants and `Agents.recorded-<n>.json` agents variants recorded by `viola verify`»
