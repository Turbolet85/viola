@amendment marker=2026-11-06-version-done-check
title: the cli output-discipline checks on std::process::Command and literal pins
section: body §1 A11y Scope Summary, §2 A11y Strategy (Understandable and Cognitive rows), §4 ARIA Patterns & Roles (cli reach row, P2 cli, P4, P6), §9 CI Integration (unit / integration stage row), Strategy (the NEVER rule); a11y-scope-summary.md (cli automated tool reach); aria-patterns-roles.md (P4 cli, P6 head, cli mirror, C0/C1); visual-design-verification.md (§6 the check's tools)
change:
  - The cli checks are named `std::process::Command` with literal pins in the §1 cli / tui line, the §4 cli reach row, the §9 unit / integration stage row and Strategy's NEVER rule (was assert_cmd 2.2.2 + predicates 3.1.4 + trycmd 1.2.1, or `assert_cmd` / `trycmd`).
  - The §2 Understandable and Cognitive rows and §4 P2 cli read the `hint:` line pinned by literal (was via / pinned by trycmd).
  - P4's cli `human-typing` refusal with its typed exit code is checked through `std::process::Command` (was assert_cmd), in the body and aria-patterns-roles.
  - P6: literal pins over `std::process::Command` assert the non-TTY modes (was trycmd / assert_cmd), in the body and aria-patterns-roles; the cli mirror's «not a trycmd case» clause removed; C0/C1 escaping checked with `std::process::Command` (was assert_cmd).
  - a11y-scope-summary's cli fallback reads `std::process::Command` on the `viola` binary and the fixed strings pinned by literal in root and unit tests (was `assert_cmd` 2.2.2 with `predicates` 3.1.4, `trycmd` 1.2.1).
  - §6: checked with `std::process::Command` + portable-pty on ubuntu-latest (was assert_cmd + portable-pty).
why: CARRY 1, carried by the chunk's report: the trycmd / assert_cmd text re-worded to the literal pins the CLI tests carry; the tools' version numbers leave with them.
kept: the results stay output-discipline stand-ins for SC 1.3.2 / 1.4.1 / 3.3.1, with no WCAG conformance claim for the cli or the TUI.
ref: .andromeda/runs/2026-11-06T20-57-05-wrap/
@amendment marker=2026-11-06-version-done-check
title: the outer-PTY driver on portable-pty =0.9.0
section: body §4 ARIA Patterns & Roles (tui reach row); a11y-scope-summary.md (tui automated tool reach); contracts/a11y-testing-tool-pick (tui bullet)
change:
  - The tui boundary driver reads portable-pty `=0.9.0` (was the inherited `=0.8.1`) in the §4 row and the scope summary, and in the tool pick «on the workspace's exact `=0.9.0` pin» (Decisions Log D-A11Y-13).
  - The tool pick's inherited-risk note (the `=0.8.1` pin of 2023-03-13 behind the maintained 0.9.0 line) removed.
why: the report's Dependencies: the workspace's portable-pty moved `=0.8.1` → `=0.9.0`, the `=0.8.1` pin and its Windows rationale retired; viola-pty 33/33 and the outer-PTY e2e binaries green on `=0.9.0`, `pty_backend()` still `openpty`.
kept: a11y inherits that driver as-is and never re-pins it.
ref: .andromeda/runs/2026-11-06T20-57-05-wrap/
@amendment marker=2026-11-06-version-done-check
title: the §8 verification lines pinned by literal
section: body §8 Error recovery (Verification), Plain language (Verification), Consistency and predictability (Verification sources); cognitive-accessibility.md (Error recovery cli item, Plain language first verification item)
change:
  - Error recovery: literal pins in root and unit tests hold the `unable <reason> <detail>` + final `hint:` layout per refusal, and literal asserts over `std::process::Command` the typed exit codes (`10`–`14`, `20`, `21`) and a hint-free `--json` (was trycmd pins / trycmd cases, and assert_cmd); the named witnesses unchanged.
  - Plain language: every fixed CLI string is pinned by literal, `viola list`, `viola --help`, `viola plugin install`, the server-verify, socket-directory, `unwrapped-name` and `endpoint-lost` hints and the `viola link` / `viola unlink` result lines among them (was pinned by trycmd, those the literal exceptions); a copy change still fails CI until the pinned string is updated on purpose.
  - Consistency and predictability: the verification source is the `CAPTION` literal (was trycmd).
why: CARRY 1, carried by the chunk's report: the trycmd / assert_cmd text re-worded to the literal pins the CLI tests carry.
ref: .andromeda/runs/2026-11-06T20-57-05-wrap/
@amendment marker=2026-11-06-version-done-check
title: the dialog hold bounds owned by 0.2.0's first entry
section: cognitive-accessibility.md (§8 Timeout extensions, CLI item)
change:
  - The dialog hold's bounds `DIALOG_HOLD_MIN` / `DIALOG_HOLD_MAX` read each PROVISIONAL, owner: 0.2.0's first entry (was each PROVISIONAL, no owner named).
why: the founder's own conditional of 2026-11-06 19:34Z — a red near-ceiling hold leaves the ceiling PROVISIONAL, owned by 0.2.0's first entry — relayed by the overseer in the wrap directive, item 2; the done-check's live round did not read the hold, as an unverified CLI replies `null` to a dialog at once.
kept: the PROVISIONAL mark on both bounds, and the rest of the hold's rule (60–3600 s, default 60, the `hooks.json` timeout 3615 s, the hook's bound 3605 s, the hold bounding the driver, never the human).
ref: .andromeda/runs/2026-11-06T20-57-05-wrap/
@amendment marker=2026-11-06-version-done-check
title: the cli tooling in the a11y testing contracts
section: contracts/a11y-testing-tool-pick (cli bullet); contracts/bootstrap-phases-derive-for-route-setup-project (a11y-tooling-install); contracts/keyboard-test-harness (Tooling)
change:
  - Tool pick, cli: output discipline is checked by tests that drive the `viola` binary through `std::process::Command` and pin each fixed string by a literal in its test file, in the `nextest-integration` / `nextest-e2e` suites and the CI `test` job's `coverage` suite on ubuntu-latest (was assert_cmd 2.2.2 + predicates 3.1.4 + trycmd 1.2.1).
  - Bootstrap, a11y-tooling-install: still none in 0.1.0; the cli cases drive the `viola` binary through `std::process::Command` and the tui cases use the tests' own portable-pty pin (was the tests' own assert_cmd, predicates, trycmd and portable-pty pins).
  - Keyboard test harness: for cli, the tests that drive the `viola` binary through `std::process::Command` cover stdin / `--file` input only (was trycmd/assert_cmd); the tui clauses unchanged.
why: CARRY 1, carried by the chunk's report: the trycmd / assert_cmd text re-worded to the literal pins the CLI tests carry; the tools' version numbers leave with them.
ref: .andromeda/runs/2026-11-06T20-57-05-wrap/
