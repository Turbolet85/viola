@proposal n=1 detector=D-retired severity=warning basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tulramlm
section: §9. CI Integration
change: fct-by4mf7d7 — the platform line reads one push/PR workflow `ci.yml` (the scheduled `nightly.yml` carries no a11y step), matrix `[ubuntu-latest]` (Linux only; the `windows-2025` and `macos-latest` legs removed, recoverable at the tag `multi-os-final`).
sidecar: §9 Platform — CI matrix narrowed to `[ubuntu-latest]`; the windows-2025 / macos-latest legs retired with the Linux-only target (founder rulings 2026-11-04, inputs#I1, inputs#I5 item 6).
rationale: report Retired (fct-tulramlm) names the `windows-2025` and `macos-latest` CI legs; Harness / gate surface (fct-sy4jxsfp): `ci.yml` jobs keep `strategy.matrix.os: [ubuntu-latest]`; Counts (fct-tjt6dmfu): CI matrix 3 OS legs → `[ubuntu-latest]` (`win-ci` stale).
@proposal n=2 detector=D-retired severity=warning dependent_of=D-retired basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-sy4jxsfp
section: §9. CI Integration
change: fct-s4vibfo3 — the Unit / integration row's stage reads «Unit / integration (`ubuntu-latest`)» and its consumer «the `test` job's `gate --require coverage,doctest`; no a11y rows, no conformance claim» (no «all three OS legs», no «per-OS»).
sidecar: §9 artifacts table — the stage runs on the one `ubuntu-latest` leg; the three-OS wording retired.
rationale: report Harness / gate surface (fct-sy4jxsfp) — `test` keeps `strategy.matrix.os: [ubuntu-latest]`; Retired (fct-tulramlm) — the `windows-2025` and `macos-latest` legs.
@proposal n=3 detector=D-retired severity=warning dependent_of=D-retired basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-sy4jxsfp
section: §9. CI Integration
change: fct-pkoabs22 — the CLI output-discipline and TUI boundary cases run inside the `test` job's single instrumented `coverage` suite on the `ubuntu-latest` leg, whose `gate --require coverage,doctest` is the verdict (no Windows / macOS legs).
sidecar: §9 Pipeline integration — the a11y cases' CI home is the one `ubuntu-latest` leg; the Windows and macOS legs retired.
rationale: report Retired (fct-tulramlm) — `windows-2025` and `macos-latest` CI legs; Harness / gate surface (fct-sy4jxsfp) — matrix `[ubuntu-latest]`.
@proposal n=4 detector=D-retired severity=warning dependent_of=D-retired basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-sy4jxsfp
section: §10. SLO Invariants & A11y Budgets
change: fct-k6kozbbz — the CLI output-discipline invariants (P4, P6) pass on the `ubuntu-latest` CI leg inside `nextest-integration` / `nextest-e2e` (CI: the `coverage` suite); the mutation-audit clause unchanged.
sidecar: §10 Standard+ invariants — «all three OS legs» → the `ubuntu-latest` leg (Linux-only target).
rationale: report Retired (fct-tulramlm) — the `windows-2025` and `macos-latest` legs; Outcome fct-2k36zioa — tui boundary and P6 cases inside `coverage` on `ubuntu-latest`, none skipped.
@proposal n=5 detector=D-retired severity=warning basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tulramlm
section: §1. A11y Scope Summary
change: fct-ktqfqjb4 — Runner: `scripts/agent-run.sh run` (no `.ps1`); Gate: the `test` job's (matrix `[ubuntu-latest]`) `gate --require coverage,doctest` never allows skips; CLI output discipline runs in CI inside the `test` job's single instrumented `coverage` suite on the `ubuntu-latest` leg (no «per-OS», no «all three OS legs»).
sidecar: §1 CI integration — the `{sh,ps1}` shim pair and the three OS legs retired; `scripts/agent-run.sh` alone, one `ubuntu-latest` leg.
rationale: report Retired (fct-tulramlm) — the `{sh,ps1}` shim pair (`scripts/agent-run.ps1` deleted) and the `windows-2025` / `macos-latest` legs; Harness / gate surface (fct-sy4jxsfp) — the harness is `scripts/agent-run.sh` alone.
@proposal n=6 detector=D-retired severity=warning dependent_of=D-retired basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-sy4jxsfp
section: §3 → ci-integration
change: fct-56elcbkj — Command: `scripts/agent-run.sh run` followed by `gate --require coverage,doctest` (skips never allowed) on `ci.yml`'s `test` job (matrix `[ubuntu-latest]`); the `scripts/agent-run.ps1` alternative and «every leg» dropped.
sidecar: §3 → CI integration Command — `scripts/agent-run.ps1` deleted with the shim pair; one `ubuntu-latest` leg.
rationale: report Harness / gate surface (fct-sy4jxsfp) — `scripts/agent-run.ps1` deleted, the harness is `scripts/agent-run.sh` alone; Retired (fct-tulramlm) — the `{sh,ps1}` shim pair.
@proposal n=7 detector=D-retired severity=warning dependent_of=D-retired basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-sy4jxsfp
section: §3 → ci-integration
change: fct-gcqo3m4v — the cases run in the existing suites (CI: the `test` job's `coverage` suite, matrix `[ubuntu-latest]`); «per-OS» dropped.
sidecar: §3 → CI integration Pipeline integration — «per-OS `test` job» → the `test` job on `ubuntu-latest`.
rationale: report Harness / gate surface (fct-sy4jxsfp) — `test` keeps `strategy.matrix.os: [ubuntu-latest]`; Retired (fct-tulramlm) — the Windows and macOS legs.
@proposal n=8 detector=D-retired severity=warning dependent_of=D-retired basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-sy4jxsfp
section: §3 → a11y-testing-tool-pick
change: fct-dlpkde76 — cli: output discipline is checked with assert_cmd 2.2.2 + predicates 3.1.4 + trycmd 1.2.1 in `nextest-integration` / `nextest-e2e` locally and in the CI `test` job's `coverage` suite on the `ubuntu-latest` leg («all three OS legs» dropped); the tui line unchanged.
sidecar: §3 → A11y testing tool pick — the cli check's CI reach is the one `ubuntu-latest` leg.
rationale: report Retired (fct-tulramlm) — the `windows-2025` and `macos-latest` legs; Harness / gate surface (fct-sy4jxsfp) — matrix `[ubuntu-latest]`.
@proposal n=9 detector=D-retired severity=warning dependent_of=D-retired basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-sy4jxsfp
section: §3 → bootstrap-phases-derive-for-route-setup-project
change: fct-yftct3hq — a11y-ci-gate-wire: none of its own: the cli / tui cases run inside the `test` job's `coverage` suite (matrix `[ubuntu-latest]`) under `gate --require coverage,doctest`; «per-OS» dropped.
sidecar: §3 → Bootstrap phases a11y-ci-gate-wire — «per-OS `test` job» → the `test` job on `ubuntu-latest`.
rationale: report Harness / gate surface (fct-sy4jxsfp) — matrix `[ubuntu-latest]`; Retired (fct-tulramlm) — the Windows and macOS legs.
@proposal n=10 detector=D-retired severity=warning dependent_of=D-retired basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tulramlm
section: §4. ARIA Patterns & Roles
change: fct-yzqlnwsx — the cli row's automated reach ends «portable-pty outer-PTY SGR checks; the `ubuntu-latest` CI leg» and its supplemental manual column reads «Orca + GNOME Terminal» (NVDA + Windows Terminal/conhost and VoiceOver + Terminal.app dropped).
sidecar: §4 tool-reach table, cli row — three OS legs and the Windows / macOS screen-reader pairings retired with the Linux-only target.
rationale: report Retired (fct-tulramlm) — OS target «Windows first, macOS and Linux» → «Linux only», the `windows-2025` / `macos-latest` legs; Expected amendments (fct-pdxuthnr) — a11y-plan carries Retired: three OS legs, NVDA / Windows Terminal rows.
@proposal n=11 detector=D-retired severity=warning dependent_of=D-retired basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tulramlm
section: §4. ARIA Patterns & Roles
change: fct-jkkyxmll — P4 on the tui: outer-PTY nextest cases on `ubuntu-latest` — zero viola-originated bytes, keystrokes delivered unblocked past the current atomic paste, and focus, mouse and resize sequences not moving the wheel; «all three OS legs», «on Linux and macOS» and the `windows-2025` injected-mouse-report platform fact dropped; the cli clause and SC 2.1.1 unchanged.
sidecar: §4 P4 — the tui cases run on the one Linux leg; the windows-2025 F-W3 platform fact retired with the ConPTY sideload.
rationale: report Retired (fct-tulramlm) — the Linux-only target, the ConPTY sideload, the `windows-2025` and `macos-latest` legs; Outcome fct-2k36zioa — tui boundary cases inside `coverage` on `ubuntu-latest`.
@proposal n=12 detector=D-retired severity=warning dependent_of=D-retired basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tulramlm
section: §4. ARIA Patterns & Roles
change: fct-ijnl2u6y — P6: trycmd / assert_cmd assert the non-TTY modes and the portable-pty outer PTY the TTY-only behaviours on the `ubuntu-latest` leg («all three OS legs» dropped); the rest unchanged.
sidecar: §4 P6 — the P6 cases run on the one `ubuntu-latest` leg.
rationale: report Retired (fct-tulramlm) — the `windows-2025` and `macos-latest` legs; Outcome fct-2k36zioa — P6 cases inside `coverage` on `ubuntu-latest`, none skipped.
@proposal n=13 detector=D-retired severity=warning dependent_of=D-retired basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-iol4iyl6
section: §4. ARIA Patterns & Roles
change: fct-lchghj7u — dim only on stale rows is proven at the unit tier and, under the outer PTY, on `ubuntu-latest` (its gate unconditional); the «Unix legs only» label, `macos-latest` and the «does not run on windows-2025» clause dropped.
sidecar: §4 stale-row dim — the outer-PTY witness runs ungated on the one Linux leg; the macOS / Windows leg wording retired.
rationale: report Deviations (fct-iol4iyl6) — `cross_list_rows_dim_a_stale_row_end_to_end_on_a_terminal` made unconditional; Retired (fct-tulramlm) — the `windows-2025` and `macos-latest` legs.
@proposal n=14 detector=D-retired severity=warning basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tulramlm
section: §8. Cognitive Accessibility › Error recovery
change: fct-2jdilj7v — trycmd pins the `unable <reason> <detail>` + final `hint:` layout per refusal and assert_cmd the typed exit codes and a hint-free `--json`, on the `ubuntu-latest` leg (the frame verbs' server-verify peer pid witness and the socket-directory refusal among them); «all three OS legs», the windows-2025 leg and the macOS no-peer-pid / euid-only carve-out dropped.
sidecar: §8 Error recovery verification — one Linux leg; the macOS no-peer-pid carve-out and the Windows leg retired.
rationale: report Retired (fct-tulramlm) — the macOS `$TMPDIR/viola/` socket dir and the macOS no-peer-pid carve-out, the `windows-2025` / `macos-latest` legs; Symbols / APIs (fct-xxutheix) — `liveness::server_verdict` loses `peer_pid_reported` and the macOS `Matches` arm.
@proposal n=15 detector=D-retired severity=warning basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tulramlm
section: §1. A11y Scope Summary
change: fct-orpg5jsb — Manual pass (supplemental): Orca (Linux) on Section 4 P4 and P6, founder-owned per the creator brief excerpt, Appendix A; NVDA (Windows Terminal) and VoiceOver (macOS) dropped.
sidecar: §1 Screen reader test pattern — the NVDA / VoiceOver passes retired with the Windows and macOS targets; Orca on Linux stays.
rationale: report Retired (fct-tulramlm) — OS target → «Linux only», the Windows and macOS code removed; Expected amendments (fct-pdxuthnr) — a11y-plan carries Retired: NVDA / Windows Terminal rows.
@proposal n=16 detector=D-retired severity=warning dependent_of=D-retired basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tulramlm
section: §1. A11y Scope Summary
change: fct-ol6ygl55 — the screen-reader-priority trigger's required assertion reads «a manual Orca pass on Section 4 P4 and P6 as a supplement» (NVDA / VoiceOver dropped).
sidecar: §1 A11y triggers screen-reader-priority — Orca alone; NVDA / VoiceOver retired with the Windows and macOS targets.
rationale: report Retired (fct-tulramlm) — OS target → «Linux only»; Expected amendments (fct-pdxuthnr) — NVDA / Windows Terminal rows retired.
@proposal n=17 detector=D-retired severity=escalate dependent_of=D-retired basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tulramlm
section: §4. ARIA Patterns & Roles
change: fct-4ktfiloi — the tui row's supplemental manual column reads «Orca + GNOME Terminal» (the cli's Linux pairing; NVDA + Windows Terminal dropped); the rest unchanged.
sidecar: §4 tool-reach table, tui row — NVDA + Windows Terminal retired with the Windows target; the Linux screen-reader pairing replaces it.
rationale: report Retired (fct-tulramlm) — OS target → «Linux only»; Expected amendments (fct-pdxuthnr) — NVDA / Windows Terminal rows retired. Escalate: the replacement tui target is a founder-owned choice (the pass is founder-owned).
@proposal n=18 detector=D-retired severity=warning dependent_of=D-retired basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tulramlm
section: §7. Screen Reader Support › Per-surface SR test pattern
change: fct-2kjabyk2 — the cli row reads «cli (Linux) | Orca + GNOME Terminal | P4 / P6 steps: linear output, refusal read as text, no escape noise» (Windows / macOS and NVDA / VoiceOver pairings dropped).
sidecar: §7 SR table, cli row — Linux only; NVDA / VoiceOver pairings retired.
rationale: report Retired (fct-tulramlm) — OS target → «Linux only»; Expected amendments (fct-pdxuthnr) — NVDA / Windows Terminal rows retired.
@proposal n=19 detector=D-retired severity=escalate dependent_of=D-retired basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tulramlm
section: §7. Screen Reader Support › Per-surface SR test pattern
change: fct-wawttggc — the tui row reads «tui (Linux) | Orca + GNOME Terminal | P4 + O1 step: the wrapped `claude` TUI reads the same as unwrapped» (Windows / NVDA + Windows Terminal dropped).
sidecar: §7 SR table, tui row — the Windows NVDA row retired; the Linux pairing replaces it.
rationale: report Retired (fct-tulramlm) — OS target → «Linux only»; Expected amendments (fct-pdxuthnr) — NVDA / Windows Terminal rows retired. Escalate: the replacement tui target is a founder-owned choice.
@proposal n=20 detector=D-retired severity=escalate dependent_of=D-retired basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tulramlm
section: §3 → screen-reader-test-pattern
change: fct-tocuc2rh — cli: Orca + GNOME Terminal; tui: Orca + GNOME Terminal, the wrapped `claude` TUI reading the same as unwrapped (Must-Work O1); NVDA + Windows Terminal/conhost, VoiceOver + Terminal.app and the tui's NVDA + Windows Terminal dropped.
sidecar: §3 → Screen reader test pattern — Windows / macOS screen-reader pairings retired with the Linux-only target.
rationale: report Retired (fct-tulramlm) — OS target → «Linux only»; Expected amendments (fct-pdxuthnr) — NVDA / Windows Terminal rows retired. Escalate: the tui's replacement target is a founder-owned choice.
@proposal n=21 detector=D-retired severity=warning dependent_of=D-retired basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tulramlm
section: §3 → bootstrap-phases-derive-for-route-setup-project
change: fct-t3p7hz2f — screen-reader-test-spec-setup: none in 0.1.0; the manual Orca pass stays founder-owned (Section 1 → Screen reader test pattern); NVDA / VoiceOver dropped.
sidecar: §3 → Bootstrap phases screen-reader-test-spec-setup — NVDA / VoiceOver retired with the Windows and macOS targets.
rationale: report Retired (fct-tulramlm) — OS target → «Linux only»; Expected amendments (fct-pdxuthnr) — NVDA / Windows Terminal rows retired.
@proposal n=22 detector=D-retired severity=warning basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tulramlm
section: §4. ARIA Patterns & Roles
change: fct-hu3qpzu6 — tui focus order: human keystrokes are never blocked or delayed, and focus/mouse/resize sequences and terminal replies are ignored for wheel purposes; the Windows win32-input-mode clause (F-W3) dropped.
sidecar: §4 P4 focus order — the Windows win32-input-mode mouse-report fact (F-W3) retired with the ConPTY sideload.
rationale: report Retired (fct-tulramlm) — the ConPTY sideload and the Windows target; Symbols / APIs (fct-xxutheix) — `viola-pty` `mod sideload` and the `ReadConsoleW` reader removed, `pty_backend()` always `openpty`.
@proposal n=23 detector=D-retired severity=warning dependent_of=D-retired basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tulramlm
section: §11. A11y Anti-Patterns › Keyboard
change: fct-csjr2vg7 — NEVER block, refuse or delay a human keystroke in `viola run`, or in a start a passed `viola revive` made, past the current atomic paste, and never count focus / mouse / resize sequences or terminal replies as editing (P4); the Windows ConPTY win32 key-down clause (F-W3) dropped.
sidecar: §11 Keyboard ban — the Windows ConPTY F-W3 clause retired with the Windows target.
rationale: report Retired (fct-tulramlm) — the ConPTY sideload and the Windows target; Symbols / APIs (fct-xxutheix) — `pty_backend()` always `openpty`.
@proposal n=24 detector=D-retired severity=warning dependent_of=D-retired basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tulramlm
section: §3 → keyboard-test-harness
change: fct-avr5o2ub — the step-wise case asserts no `wheel` record and the bytes reaching the child unchanged (no «On Linux and macOS», the `windows-2025` F-W3 platform fact and its «Windows-only live measurements» pointer dropped); the zero-viola-bytes clause means no viola literal and no SGR or cursor control the child did not emit, compared byte for byte with an unwrapped fake-agent run (no per-leg split); the revive, statusline and budget readings keep their chunk and run but read «green in CI» (the «three CI runners» re-worded as history); the statusline start writes the settings override and passes `--settings` (the «on Windows it writes none» clause dropped); the whole Windows ConPTY-host passage (inbox and sideloaded preambles, the DA1 query, both backends, `conpty_sideload`'s tampered-companion case) dropped; each of the three boundary clauses is its own nextest case on the `ubuntu-latest` leg.
sidecar: §3 → Keyboard test harness — the Windows ConPTY / F-W3 / macOS-leg wording and the three-runner claims retired with the Linux-only target; the Linux assertions unchanged.
rationale: report Retired (fct-tulramlm) — the ConPTY sideload (`conpty_sideload` among the deleted tests, fct-yq5vc7zj), the Windows target, the `windows-2025` / `macos-latest` legs and «Windows-only live measurements» (retired at the 2026-11-04T23-11-43-wrap); Counts (fct-tjt6dmfu) — a dated measurement on `windows-2025` is re-worded or retired, never left as current.
@proposal n=25 detector=D-retired severity=warning basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tulramlm
section: §1. A11y Scope Summary
change: fct-z2htpz2e — under `viola revive --json`, every refusal of the start it shares with `run` prints one exit-1 document, `pinned-hash-mismatch` and `squatted-name` among them and a program not found `internal-error` (`batch-script-child` dropped); the `pump_child` clause unchanged.
sidecar: §1 revive --json entity — `batch-script-child` retired with the `.cmd`/`.bat` refusal (inputs#I2).
rationale: report Retired (fct-tulramlm) — the `.cmd`/`.bat` refusal (`batch-script-child`, `AgentError::BatchScriptChild`); Counts (fct-tjt6dmfu) — revive's closed `--json` refusal codes 11 → 10.
@proposal n=26 detector=D-retired severity=warning basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tulramlm
section: §1. A11y Scope Summary
change: fct-rdegquaa — the boundary still holds after a contained vt100 feed panic (no viola literal on the outer terminal and the child's own bytes reaching it); «on every OS», «on Unix» and the Windows byte reading owed to "Windows-only live measurements" dropped.
sidecar: §1 viola run TUI entity — the Windows byte-reading pointer to the retired "Windows-only live measurements" entry dropped with the Windows target.
rationale: report Retired (fct-tulramlm) — the Linux-only target; «Windows-only live measurements» retired at the 2026-11-04T23-11-43-wrap (`wolm` sites retired with the Windows claim they owe, Expected amendments fct-pdxuthnr); Counts (fct-tjt6dmfu) — an «on every OS» clause is stale wording on a Linux-only product.
@remark
D-a11y-surface: no drift (report Coverage of new surfaces fct-oex26kqr: the chunk adds no surface, a11y n/a). D-a11y-obs-schema: no drift — `schemas/diag-line.v1.json` changed (`os`, `sideload_fallback`, `batch-script-child`) but a11y emits no violation rows (§3 → structured-violation-json-schema), so nothing diverges. §1 facts are marked verbatim from a11y-scope.md: the orchestrator may need to keep that note true. Detail-file hits (a11y-scope-summary, aria-patterns-roles, cognitive-accessibility, visual-design-verification: Windows / macOS / three-leg wording in standing groups) are the cascade's. Proposals 17, 19, 20 escalate: the tui screen-reader target that replaces NVDA + Windows Terminal is a founder-owned choice.
