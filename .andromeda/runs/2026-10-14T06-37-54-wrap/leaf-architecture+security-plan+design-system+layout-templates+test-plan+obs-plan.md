group: architecture+security-plan+design-system+layout-templates+test-plan+obs-plan
read: CLAUDE.md · 128 · «<!-- USER:session-learnings end -->»

- leaf: CLAUDE.md:15   class: omission
  leaf-text: «`<Event>.<stem>-<n>.json` dialog and framing variants recorded by `viola verify`»
  source: .andromeda/registries/contracts/architecture/project-directory-structure.md:108 «<Event>.<stem>-<n>.json dialog and framing variants and StatusLine.<stem>-<n>.json status line variants recorded by `viola verify`»
  replace-with: «`<Event>.<stem>-<n>.json` dialog and framing variants and `StatusLine.<stem>-<n>.json` status line variants recorded by `viola verify`»

- leaf: CLAUDE.md:25   class: omission
  leaf-text: «the `--settings` override document, the `/bin/sh -c` argv)»
  source: .andromeda/registries/contracts/architecture/crate-dependency-direction.md:6 «`statusline` module: `reading`, `user_command`, `override_document`, `shell_argv`, `SETTINGS_FLAG`, `USER_SETTINGS`, `WRAPPED_HERE`, `BIN_SH`, `SHELL_FLAG`, `probe_document`»
  replace-with: «the `--settings` override document and `viola verify`'s probe document, `WRAPPED_HERE` (Unix), the `/bin/sh -c` argv)»

- leaf: CLAUDE.md:28   class: omission
  leaf-text: «the `hook statusline` arm (the instance check before anything under the home is opened; Unix: then the user's command through `/bin/sh -c`)»
  source: .andromeda/architecture.md:326 «With `--capture <DIR>` it takes `viola verify`'s probe capture instead, before the pass-through ([Hook Contract]): no instance check, no `budget.json` write, no command run, nothing printed»
  replace-with: «the `hook statusline` arm (the instance check before anything under the home is opened; Unix: then the user's command through `/bin/sh -c`; its hidden `--capture <DIR> [--shell <WORD>]` form is `viola verify`'s probe capture: no instance check, no `budget.json` write, no command run, nothing printed)»

- leaf: CLAUDE.md:43   class: stale
  leaf-text: «the paste-framing and `/clear` rows and the `paste-newline` row included: eighteen rows (`LedgerRow::ALL`) gate `cli_verified`»
  source: .andromeda/architecture.md:91 «the three statusline rows are relied only where the status line is wrapped (Unix), so a Unix host relies on all twenty-one rows and a Windows host on the first eighteen»
  replace-with: «the paste-framing and `/clear` rows, the `paste-newline` row and the three statusline rows included: of the twenty-one rows (`LedgerRow::ALL`), the relied set (`LedgerRow::relied()`: all twenty-one on Unix, the first eighteen on Windows, where the status line is not wrapped) gates `cli_verified`»

- leaf: CLAUDE.md:43   class: stale
  leaf-text: «two relied-on dependences have no row yet: `claude --resume <id>` reopening a session, and the statusline's `rate_limits` payload (which the `budget-paused` send refusal now rests on), `--settings` override and `/bin/sh` shell, both owed on the route entry "Owed ledger rows" and, for Windows, on "Windows-only live measurements")»
  source: .andromeda/architecture.md:473 «One relied-on dependence has no row yet and is not under that limit: `viola revive`'s. The statusline pass-through's three shapes have their rows since chunk 2026-10-13-owed-ledger-rows (below).»
  replace-with: «the statusline's `--settings` override, `/bin/sh` shell and `rate_limits` payload (which the `budget-paused` send refusal rests on) are the rows `statusline-override` · `statusline-shell` · `statusline-rate-limits`, stamped on 2.1.287 on the Linux dev host and relied on Unix only (their Windows legs owed on "Windows-only live measurements"); one relied-on dependence has no row yet: `claude --resume <id>` reopening a session, owed with `/compact`'s on the route entry "Resume and compact ledger rows")»
