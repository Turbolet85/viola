@proposal n=1 detector=D-cascade severity=warning file=a11y-scope-summary anchor=fct-vtox22ts basis_doc=doc:2026-11-06-version-done-check/report basis_fact=fct-ngwjb7xx
section: ## 1. A11y Scope Summary
change: in the cli Automated tool reach fallback, replace «`assert_cmd` 2.2.2 with `predicates` 3.1.4, `trycmd` 1.2.1, and SGR-only checks through the portable-pty outer PTY» with «`assert_cmd` 2.2.2 with `predicates` 3.1.4, the fixed strings pinned by literal in root and unit tests rather than by a trycmd file, and SGR-only checks through the portable-pty `=0.9.0` outer PTY»; the rest of the row unchanged.
sidecar: a11y-scope-summary cli tool reach re-worded to the literal pins (CARRY 1) and the outer PTY named at portable-pty =0.9.0 (2026-11-06-version-done-check).
rationale: the report's Expected amendments entry 12 (and entry 10's `trycmd` pattern, a11y-plan 26 rows) carries the trycmd/assert_cmd text to the literal pins, and its Dependencies move portable-pty =0.8.1 to =0.9.0.
@proposal n=2 detector=D-cascade severity=warning file=a11y-scope-summary anchor=fct-twqx5x46 basis_doc=doc:2026-11-06-version-done-check/report basis_fact=fct-xc746owh
section: ## 1. A11y Scope Summary
change: in the tui Automated tool reach, «the tests' portable-pty `=0.8.1` outer-PTY driver» reads «the tests' portable-pty `=0.9.0` outer-PTY driver»; the rest unchanged (pty_backend stays openpty, the outer-PTY e2e binaries green on 0.9.0).
sidecar: a11y-scope-summary tui reach — the outer-PTY driver pin moved =0.8.1 to =0.9.0 (2026-11-06-version-done-check).
rationale: the report's Dependencies (portable-pty =0.8.1 to =0.9.0, crates/viola-pty compiles unchanged) and Outcome (pty) met on =0.9.0; Expected amendments entry 12 names fct-twqx5x46.
@proposal n=3 detector=D-cascade severity=warning file=body anchor=fct-4ktfiloi dependent_of=D-cascade basis_doc=doc:2026-11-06-version-done-check/report basis_fact=fct-xc746owh
section: ## 4. ARIA Patterns & Roles
change: the tui reach row's «portable-pty outer-PTY boundary driver (inherited `=0.8.1`)» reads «portable-pty outer-PTY boundary driver (`=0.9.0`)»; the rest of the row unchanged.
sidecar: §4 tui reach row — portable-pty pin =0.8.1 to =0.9.0 (2026-11-06-version-done-check).
rationale: the same pin move as proposal 2, the body's broad line of the tui reach; Expected amendments entry 12 names fct-4ktfiloi.
@proposal n=4 detector=D-cascade severity=warning file=aria-patterns-roles anchor=fct-uok334yy basis_doc=doc:2026-11-06-version-done-check/report basis_fact=fct-ngwjb7xx
section: ## 4. ARIA Patterns & Roles
change: the P6 head «trycmd / assert_cmd assert the non-TTY modes» reads «literal pins and assert_cmd assert the non-TTY modes»; the cli-mirror clause «not a trycmd case» and every other item unchanged.
sidecar: aria-patterns-roles P6 — the non-TTY assertions re-worded from trycmd to the literal pins (CARRY 1, 2026-11-06-version-done-check).
rationale: the report's Expected amendments entry 12 with entry 10 (CARRY 1) carries the §4 P6 trycmd wording to the literal pins; the same entity already says the caption order is a literal, not a trycmd case.
@proposal n=5 detector=D-cascade severity=warning file=body anchor=fct-ijnl2u6y dependent_of=D-cascade basis_doc=doc:2026-11-06-version-done-check/report basis_fact=fct-ngwjb7xx
section: ## 4. ARIA Patterns & Roles
change: «trycmd / assert_cmd assert the non-TTY modes» reads «literal pins and assert_cmd assert the non-TTY modes»; the rest of the P6 line unchanged.
sidecar: §4 P6 broad line — trycmd re-worded to the literal pins (CARRY 1, 2026-11-06-version-done-check).
rationale: the body's broad line of aria-patterns-roles#3.1, the same CARRY 1 re-wording as proposal 4.
@proposal n=6 detector=D-cascade severity=warning file=cognitive-accessibility anchor=fct-6edxk4iz basis_doc=doc:2026-11-06-version-done-check/report basis_fact=fct-hvqxfdzx
section: ## 8. Cognitive Accessibility › ### Timeout extensions
change: after «its bounds `DIALOG_HOLD_MIN` / `DIALOG_HOLD_MAX`, each PROVISIONAL» add «(the 3600 s ceiling not read live at the version done-check, it stays PROVISIONAL, owned by 0.2.0's first entry)»; the rest of the entity unchanged.
sidecar: cognitive-accessibility timeout extensions — the dialog hold ceiling stays PROVISIONAL, its owner re-pointed from the Version done-check to 0.2.0's first entry (inputs#I7 item 2, 2026-11-06-version-done-check).
rationale: the report's Deviations (the hold unread, CARRY 2 PROVISIONAL, owner 0.2.0's first entry), Insufficient fixes and Expected amendments entry 5 (`hold-ceiling`, a11y-plan 1 row).
@proposal n=7 detector=D-cascade severity=warning file=cognitive-accessibility anchor=fct-g3h7n6e2 basis_doc=doc:2026-11-06-version-done-check/report basis_fact=fct-ngwjb7xx
section: ## 8. Cognitive Accessibility › ### Error recovery
change: the cli verification's head «trycmd cases pin the `unable <reason> <detail>` + final `hint:` layout per refusal» reads «literal pins in root and unit tests pin the `unable <reason> <detail>` + final `hint:` layout per refusal»; the parenthesised witnesses and the rest unchanged.
sidecar: cognitive-accessibility error recovery verification — trycmd re-worded to the literal pins (CARRY 1, 2026-11-06-version-done-check).
rationale: the report's Expected amendments entry 12 carries the §8 verification lines to the literal pins (CARRY 1); the entity's own witnesses are literals (`list_refuses_*`, `HINT`).
@proposal n=8 detector=D-cascade severity=warning file=body anchor=fct-2jdilj7v dependent_of=D-cascade basis_doc=doc:2026-11-06-version-done-check/report basis_fact=fct-ngwjb7xx
section: ## 8. Cognitive Accessibility › ### Error recovery
change: «trycmd pins the `unable <reason> <detail>` + final `hint:` layout per refusal» reads «literal pins hold the `unable <reason> <detail>` + final `hint:` layout per refusal»; the rest of the line unchanged.
sidecar: §8 error recovery broad verification line — trycmd re-worded to the literal pins (CARRY 1, 2026-11-06-version-done-check).
rationale: the body's broad line of cognitive-accessibility#22.1, the same CARRY 1 re-wording as proposal 7.
@proposal n=9 detector=D-cascade severity=warning file=cognitive-accessibility anchor=fct-oji2cu7x basis_doc=doc:2026-11-06-version-done-check/report basis_fact=fct-ngwjb7xx
section: ## 8. Cognitive Accessibility › ### Plain language
change: the first verification item opens «Every fixed CLI string is pinned by literal:» in place of «trycmd pins every fixed CLI string, except `viola list`'s, which literals pin», and lists the same pins (`viola list`'s, `viola --help`'s, `viola plugin install`'s, the server-verify hint, the instance-state and home-state pairs) with their tests unchanged; the closing «Any copy change therefore fails CI until the pinned string is updated on purpose.» and the other items unchanged.
sidecar: cognitive-accessibility plain-language verification — every fixed CLI string pinned by literal, trycmd wording retired (CARRY 1, 2026-11-06-version-done-check).
rationale: the report's Expected amendments entry 12 carries the §8 lines to the literal pins (CARRY 1); the HELP literal and help tests are byte-identical (Symbols / APIs), so the named pins stand.
@proposal n=10 detector=D-cascade severity=warning file=body anchor=fct-fhiyvzbh dependent_of=D-cascade basis_doc=doc:2026-11-06-version-done-check/report basis_fact=fct-ngwjb7xx
section: ## 8. Cognitive Accessibility › ### Plain language
change: «every fixed CLI string is pinned, by trycmd or, for `viola list`, …, by literal» reads «every fixed CLI string is pinned by literal (`viola list`, `viola --help`, `viola plugin install`, the server-verify, socket-directory, `unwrapped-name` and `endpoint-lost` hints and the `viola link` / `viola unlink` result lines among them)»; the closing clause unchanged.
sidecar: §8 plain-language broad verification line — trycmd re-worded to the literal pins (CARRY 1, 2026-11-06-version-done-check).
rationale: the body's broad line of cognitive-accessibility#31.1, the same CARRY 1 re-wording as proposal 9.
@proposal n=11 detector=D-cascade severity=warning file=body anchor=fct-yzqlnwsx dependent_of=D-cascade basis_doc=doc:2026-11-06-version-done-check/report basis_fact=fct-ngwjb7xx
section: ## 4. ARIA Patterns & Roles
change: the cli reach row's «assert_cmd 2.2.2 + predicates 3.1.4 + trycmd 1.2.1; portable-pty outer-PTY SGR checks» reads «assert_cmd 2.2.2 + predicates 3.1.4 + literal pins; portable-pty `=0.9.0` outer-PTY SGR checks»; the rest of the row unchanged.
sidecar: §4 cli reach row — trycmd re-worded to the literal pins (CARRY 1) and the outer PTY at =0.9.0 (2026-11-06-version-done-check).
rationale: the body's broad line of the cli reach, the same re-wording as proposal 1 (Expected amendments entries 10 and 12, Dependencies).
@remark
Other body trycmd lines outside the picked groups (fct-xmkdtzxa fct-gglbhu4k §1 harness keys, fct-zj2wxt2x fct-6py5v5ks §2 pyramid, fct-3xiq4dqd §4 P2, fct-cyo3rpos §8 consistency, fct-s4vibfo3 §9 stage row) follow the same CARRY 1 re-wording, and the §3 contract keys a11y-testing-tool-pick keyboard-test-harness bootstrap-phases are key files; the statusline-rate-limits disproof the §8 budget pause rests on is architecture's ledger-row fact fct-qwpklckh (owner 0.2.0 F-06i) and the a11y claim that no key is blocked still holds.
