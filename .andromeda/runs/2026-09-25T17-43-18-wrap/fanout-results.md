# Fan-out results — 2026-09-25-pty-wrapper-on-windows (wrap 2026-09-25T17-43-18)

Seven Explore doc-agents, one parallel batch, prompt from `amendment-flow.md` substituted (no `{…}` left), report
`viola-0.1.0/chunks/2026-09-25-pty-wrapper-on-windows/report.md`. Returns carried no HTML entities (`<dir>`, `<name>`,
`->` arrived raw; entities=0). Raw twins are consolidated here: each proposal is listed with its detector, section, basis
and the orchestrator's validation disposition (V = validate verdict).

| doc | verdict | proposals |
|---|---|---|
| architecture | drift | 24 |
| security-plan | drift | 6 |
| design-system | clean — `proposals: []` (D-design-tokens: both new surfaces `tokens n/a`) | 0 |
| layout-templates | clean — `proposals: []` (refusal lines already in §Output structure `viola run` + design-system:772) | 0 |
| test-plan | drift | 13 |
| obs-plan | drift | 11 |
| a11y-plan | drift | 4 |

## architecture (24)
| # | detector | section | basis | V |
|---|---|---|---|---|
| A1 | D-arch-resources | Occupied Resources → Environment variables (R8 bullet) | arch:360 | routine (this-chunk) · ruling-1 group (E1) |
| A2 | ↳ dep | [CLI Version Compatibility] ledger row "R8 strip list" → identity floor | arch:76 | routine |
| A3 | ↳ dep | Cross-cutting → Config management: env vars not a config channel — qualify for registry names | arch:544 | ESCALATE (E1: qualifies a Critical-Warning claim) |
| A4 | D-arch-resources | Occupied Resources → Filesystem config.json: `claude_env_keep` | arch:368 | routine (report fact only: whole-reject via ConfigRejection; the proposal's `parse-rejected{parser:"config-json"}` is NOT a report fact — dropped) |
| A5 | ↳ dep | Config management Settings list + `claude_env_keep` | arch:543 | routine |
| A6 | D-arch-resources | Conventions → CLI exit codes: exit 1 + batch-script-child | arch:138 | routine |
| A7 | D-arch-resources | Occupied Resources → Workspace crates: viola-pty, viola-agent-claude landed, no-op fake-agent feature | arch:351-352 | routine |
| A8 | ↳ dep | Build system → code-graph rust plane member list | arch:417 | routine |
| A9 | ↳ dep | Build system → Licence inheritance list | arch:421 | routine |
| A10 | D-arch-resources | Repository: `target/tools/ripgrep/probe-*` | arch:397 | routine |
| A11 | D-arch-resources | Repository: orphans-probes set as built (2/2 + control) | arch:391 | routine |
| A12 | D-arch-decisions | Stack → PTY layer: libc cfg(unix), windows-sys Console, HostTerminal | arch:17 | routine |
| A13 | ↳ dep | Crate dependency direction → viola-pty deps | arch:429 | routine, applied as "as landed" (not "exactly"): obs-plan §3 lists tracing for viola-pty; spans are sequenced to "Wrapper channel" (CARRY) |
| A14 | ↳ dep | Project tree comment viola-pty | arch:469 | routine |
| A15 | ↳ dep | Crate dependency direction → viola-agent-claude as landed (thiserror only) | arch:432 | routine |
| A16 | ↳ dep | Crate dependency direction → root bin + windows-sys cfg(windows) | arch:436 | routine |
| A17 | D-arch-decisions | [Error Handling] PtyError hand-written exception | arch:96 | ESCALATE (E2: exception to a locked decision) |
| A18 | ↳ dep | Conventions → Error handling schema | arch:154 | E2 |
| A19 | ↳ dep | Crate dependency direction → shared deps sentence excludes viola-pty | arch:435 | E2 |
| A20 | ↳ dep | Stack → Error types row | arch:28 | E2 |
| A21 | ↳ dep | Inherited Defaults → Errors | arch:588 | E2 |
| A22 | D-arch-decisions | [PTY] as-built decisions (HostTerminal, cwd, resolution, kept writer) | arch:46 | routine (expected amendment 2) |
| A23 | ↳ dep | [CI/CD] "child program is spawned as given" → viola-side resolution | arch:101 | routine |
| A24 | D-arch-decisions | Build system → Lint: root bin local print_stderr allow | arch:407 | routine (plan step 5 intended) |

Count: D-arch-resources 11 (A1–A11) + D-arch-decisions 13 (A12–A24) = 24 (a receipt-time tally of 23 was a miscount).

## security-plan (6)
| # | detector | section | basis | V |
|---|---|---|---|---|
| S1 | D-security-input (escalate) | Input Validation → config.json row + `claude_env_keep` | sec:230 | ESCALATE by detector severity (E1) |
| S2 | D-security-input (escalate) | Input Validation → new row: registry persistent-environment names | sec:229-231 | ESCALATE by detector severity (E1) |
| S3 | D-security-input (escalate) | Input Validation → Child executable resolution as built | sec:235 | ESCALATE by detector severity — recommended apply (plan-intended) |
| S4 | ↳ dep | Threat Model Summary → attack surface "spawns the program as given" | sec:109 | ESCALATE (E4: lands in the VERBATIM copy of threat-assessment.md, sec:18) |
| S5 | D-security-auth | Secret Management → Storage: R8 rule as built | sec:421 | routine (expected amendment 4) · ruling-1 group (E1) |
| S6 | ↳ dep | Threat Model Summary → credential classification | sec:42 | ESCALATE (E4: verbatim section) |
Detector notes (not proposals): D-security-deps clean; out-of-detector hits sec:465 (Error Handling) and sec:389 (Bootstrap `error-sanitization-wire`) name thiserror for `PtyError` → the cascade step-2 sweep of the E2 amendment.

## test-plan (13)
| # | detector | section | basis | V |
|---|---|---|---|---|
| T1 | D-tests-coverage | §4 viola-agent-claude: R8 unit oracle = 11-name floor literal + prefix rule | tp:859 | routine (disproved claim 1) |
| T2 | ↳ dep | §1 inherited credentials strip entity | tp:86 | routine |
| T3 | ↳ dep | §6 E2 steps: 11 floor + unknown canaries | tp:1191 | routine |
| T4 | ↳ dep | §11 Unit literal example | tp:1554 | routine |
| T5 | D-tests-coverage | §6 E2 verification: persistent-set survival, env_kept; VIOLA_* and Unix-fds halves re-pinned to their CARRY entries | tp:1194 | routine (sequencing; CARRYs pinned at P5) |
| T6 | D-tests-coverage | §6 Chaos: no-EOF mode as measured (Linux own group; macOS ends at leader exit) | tp:1304 | routine (disproved claim 2; overseer direction 4) |
| T7 | ↳ dep | §7 fake agent `--exit-no-eof` holder | tp:1366 | routine (disproved claim 3) |
| T8 | D-tests-coverage | §7 fake agent receipt kinds + size/cwd/hold, HostTerminal guard | tp:1357 | routine |
| T9 | D-tests-coverage | §6 Chaos `.cmd` child → cli_program_resolution | tp:1313 | routine (accurate placement; no route entry names a chaos suite) |
| T10 | D-tests-obs-harness | §3 boot step 5: outer PTY, interim retired | tp:519 | routine |
| T11 | ↳ dep | §3 cleanup step 1: Ctrl-C re-press 500 ms | tp:592 | routine |
| T12 | ↳ dep | §3 supervise subcommand | tp:615 | routine |
| T13 | D-tests-obs-harness | §3 run step 2 booted_wrapper over OuterPty + start receipt | tp:542 | routine |

## obs-plan (11)
| # | detector | section | basis | V |
|---|---|---|---|---|
| O1 | D-obs-instrumentation | §6 additive field catalog `process-start` + `env_kept` | obs:1057 | routine (expected amendment 5) |
| O2 | ↳ dep | §4 Scenario 1 required log fields | obs:868 | routine |
| O3 | ↳ dep | §4 Edge flows E2 | obs:968 | routine |
| O4 | D-obs-instrumentation | §12 Decisions Log: entry superseding D-14 | obs:1556-1559 | routine (ruling 1, expected amendment 5) |
| O5 | ↳ dep | §8 data classification R8 row | obs:1176 | routine |
| O6 | D-obs-instrumentation | §4 Scenario 1 span attrs: batch-script-child from resolution, not run.pin_copy | obs:857 | routine |
| O7 | D-obs-stack | §3 SDK packages: viola-pty / viola-agent-claude tracing-free | obs:570,573 | REJECT — sequencing (playbook rule 1): obs §3/§4 require seam spans; zero `#[instrument]` exists workspace-wide (pre-chunk too); "Wrapper channel" carries "the first spans (obs-plan §4)" → CARRY there names `pty.spawn` + the viola-pty seam-span placement choice |
| O8 | ↳ dep | §3 logger-stack-install tracing list | obs:777 | REJECT with O7 |
| O9 | D-obs-stack | §7 PtyError not thiserror | obs:1119 | E2 |
| O10 | D-obs-stack | §3 preamble: `run` pre-spawn refusal stderr exception | obs:556 | routine |
| O11 | ↳ dep | §11 print-macro ban allow-list + refusal fn | obs:1389 | routine |

## a11y-plan (4)
| # | detector | section | basis | V |
|---|---|---|---|---|
| Y1 | D-a11y-surface | §3 Keyboard test harness → Tooling: scope "no SGR/cursor control" to Linux/macOS | a11y:624 | routine (expected amendment 7, disproved claim 4) |
| Y2 | ↳ dep | §1 tui passthrough Reason | a11y:98 | E4 (§1 verbatim per D-A11Y-15; not a deferral clause) |
| Y3 | ↳ dep | §1 cli Notes | a11y:179 | E4 |
| Y4 | ↳ dep | §6 CLI equivalent "zero SGR under viola run" | a11y:947 | routine |
Detector notes: D-a11y-obs-schema clean. Disproved claim 5 ("ConPTY swallows focus reports") not proposed — no basis → E3.

## Orchestrator-raised (check 5 / check 6)
- Expected amendment 7, test-plan half: NOT carried — test-plan states no zero-own-bytes oracle (`grep -n -E 'zero[- ]viola|viola-originated|own bytes|zero own|byte for byte|byte-for-byte'` → 1 hit, tp:1139, a statusline marker — no change; the 10 `ConPTY` hits read in full: none states byte identity).
- Disproved claim 6 (plan-internal) → arch A13/A15/A17 group; the plan-form `run --mutants` → route CARRY (union rule, next chunk) + curation.
- `pty.spawn` / seam spans absent (obs:307, :853, :861, :170, :501) → sequencing CARRY on "Wrapper channel" (P5).

## Escalations
- E1 — R8 boundary widening (operator ruling 1): A1, A3, S1, S2, S5.
- E2 — PtyError exception to "thiserror per crate": A17-A21, O9 (+ cascade sec:465, sec:389).
- E3 — "ConPTY swallows focus reports": no measured basis.
- E4 — verbatim sections: S4, S6 (security Threat Model Summary), Y2, Y3 (a11y §1) + playbook rule proposal.
