@proposal n=1 detector=D-arch-decisions severity=warning file=stack-and-technologies anchor=fct-kxiazeyf basis_doc=doc:2026-11-06-version-done-check/report basis_fact=fct-xc746owh
section: ## Stack and Technologies
change: PTY layer row: portable-pty `=0.9.0` (pinned exact, as the workspace pins every third-party version) in place of `=0.8.1`; the rest of the row (libc `=0.2.189`, `HostTerminal`, `host_stdin()`, `host_size()`, `pty_backend()` always `openpty`) unchanged.
sidecar: stack PTY row — portable-pty =0.8.1 → =0.9.0 (chunk 2026-11-06-version-done-check, the dependency move).
rationale: Report Dependencies: `portable-pty` `=0.8.1` → `=0.9.0` in `[workspace.dependencies]`, viola-pty compiles unchanged, `pty_backend()` stays `openpty`.
@proposal n=2 detector=D-arch-decisions severity=warning file=body anchor=fct-zegjs5rw dependent_of=D-arch-decisions basis_doc=doc:2026-11-06-version-done-check/report basis_fact=fct-xc746owh
section: ## Stack and Technologies
change: the stack broad line reads portable-pty `=0.9.0` behind viola's own `pty` seam, in place of `=0.8.1`; nothing else moves.
sidecar: body stack line — portable-pty =0.8.1 → =0.9.0.
rationale: Report Dependencies (the pin move); the broad line the stack-and-technologies PTY row details.
@proposal n=3 detector=D-arch-decisions severity=warning file=established-decisions anchor=fct-ad44nn2e basis_doc=doc:2026-11-06-version-done-check/report basis_fact=fct-xc746owh
section: ## Established Decisions
change: [PTY] head reads portable-pty `=0.9.0` pinned behind viola's own `pty` seam; the Rationale becomes «Exact, as the workspace pins every third-party version; 0.9.0's graph drops `pin-utils` (RUSTSEC-2026-0441 closed) and viola-pty compiles unchanged on it» — the 0.8.1 spike-proven / Windows Terminal sentence, the 0.9.0 Windows garbage-read sentence, the move owed to "Version done-check" and the portable-pty-psmux 0.9.7 swap trigger all leave; the rest of the entity (handle-wait exit, host_stdin, HostTerminal, cwd, revive, resize, `pty_backend()` `openpty`, `host_size()`) stands.
sidecar: [PTY] — pin =0.8.1 → =0.9.0, rationale re-measured, the owed move and the psmux swap trigger retired (chunk 2026-11-06-version-done-check).
rationale: Report Dependencies (the `=0.9.0` move, the comment block naming the 0.9.0 Windows reasons replaced by «Exact, as the workspace pins every third-party version.») and Retired (the `=0.8.1` pin and its Windows rationale; the PTY swap trigger, now spent); expected amendment 1.
@proposal n=4 detector=D-arch-decisions severity=warning file=body anchor=fct-omwrdnlc dependent_of=D-arch-decisions basis_doc=doc:2026-11-06-version-done-check/report basis_fact=fct-xc746owh
section: ## Established Decisions
change: [PTY] broad line reads portable-pty `=0.9.0` pinned; its Rationale becomes «exact, as the workspace pins every third-party version; 0.9.0's graph drops `pin-utils`», the owed move to "Version done-check" removed.
sidecar: body [PTY] — =0.8.1 → =0.9.0, the owed move discharged.
rationale: Report Dependencies and Retired; expected amendment 1 (fct-omwrdnlc).
@proposal n=5 detector=D-retired severity=warning file=established-decisions anchor=fct-co2wsjkw basis_doc=doc:2026-11-06-version-done-check/report basis_fact=fct-uufjzm4k
section: ## Established Decisions
change: retire the item: RUSTSEC-2026-0441 is closed — `portable-pty` `=0.9.0` drops `pin-utils` from the graph (`cargo tree -i pin-utils` finds no package), `cargo deny check` reads advisories ok with `ignore = []`, so its CARRY on "Version done-check" is spent.
sidecar: [PTY] — RUSTSEC-2026-0441 item retired, the advisory closed by the =0.9.0 move with no ignore widening.
rationale: Report Retired names this fact's swap trigger spent (0.9.0 drops pin-utils upstream); Dependencies: RUSTSEC-2026-0441 closed, `ignore = []`; expected amendment 1 «fct-co2wsjkw retired».
@proposal n=6 detector=D-retired severity=warning file=established-decisions anchor=fct-xbwprhwr basis_doc=doc:2026-11-06-version-done-check/report basis_fact=fct-uufjzm4k
section: ## Established Decisions
change: [Hook Transport] Rationale opens «exec form skips shell startup.» — the Git Bash path-rewriting reason leaves; the http, mcp_tool, SessionEnd-budget and ledger-row sentences stand.
sidecar: [Hook Transport] — the Git Bash path-rewriting reason retired with Windows (T2).
rationale: Report Retired: the Git Bash rewritten-path warning (T2, `inputs#I2`); T2 «masters follow»; the build is Linux only, so Git Bash rewriting no longer bears on exec form.
@proposal n=7 detector=D-retired severity=warning file=body anchor=fct-zhr3wx2i dependent_of=D-retired basis_doc=doc:2026-11-06-version-done-check/report basis_fact=fct-uufjzm4k
section: ## Established Decisions
change: [Hook Transport] broad line's Rationale reads «exec form skips shell startup.», the Git Bash path-rewriting reason removed.
sidecar: body [Hook Transport] — Git Bash reason retired (T2).
rationale: Report Retired (the Git Bash rewritten-path warning, T2).
@proposal n=8 detector=D-cascade severity=warning file=established-decisions anchor=fct-vxml3b56 basis_doc=doc:2026-11-06-version-done-check/report basis_fact=fct-oth7uyhe
section: ## Established Decisions
change: A hold near the 3600 s ceiling was not read at "Version done-check" (an unverified CLI replies `null` to a dialog at once, so the round's rig could not reach it); the ceiling stays PROVISIONAL, owner: 0.2.0's first entry.
sidecar: [Hook Contract] — dialog-hold ceiling kept PROVISIONAL, owner re-pointed from "Version done-check" to 0.2.0's first entry (`inputs#I7` item 2).
rationale: Report Insufficient fixes: CARRY 2 stays open → 0.2.0's first entry (`inputs#I7` item 2, the card's row 8); Cross-project: the 3600 s hold NOT read; expected amendment 5 «kept».
@proposal n=9 detector=D-cascade severity=warning file=established-decisions anchor=fct-wpbp4hte dependent_of=D-cascade basis_doc=doc:2026-11-06-version-done-check/report basis_fact=fct-ngwjb7xx
section: ## Established Decisions
change: the dialog-hold constants' mark reads «PROVISIONAL, owner: 0.2.0's first entry» (`DIALOG_HOLD_DEFAULT` · `DIALOG_HOLD_MIN` · `DIALOG_HOLD_MAX`); the rest of the entity stands.
sidecar: [Hook Contract] — the hold's PROVISIONAL mark given its owner, 0.2.0's first entry.
rationale: Report expected amendment 5 names fct-wpbp4hte among the ceiling's owner sites; the hold went unread (Insufficient fixes, CARRY 2 → 0.2.0's first entry).
@proposal n=10 detector=D-cascade severity=warning file=body anchor=fct-4nt6qtyp dependent_of=D-cascade basis_doc=doc:2026-11-06-version-done-check/report basis_fact=fct-ngwjb7xx
section: ## Established Decisions
change: [Hook Contract] broad line reads «ceiling PROVISIONAL, owner: 0.2.0's first entry» in place of «owner: "Version done-check"».
sidecar: body [Hook Contract] — ceiling owner re-pointed to 0.2.0's first entry.
rationale: Report expected amendment 5 (fct-4nt6qtyp), `inputs#I7` item 2.
@proposal n=11 detector=D-new-detail severity=warning file=established-decisions anchor=fct-ikuc6e4y fact=F2 basis_doc=doc:2026-11-06-version-done-check/report basis_fact=fct-hqxtn6v3
section: ## Established Decisions
change: add after the [Budget Governor] entity: «OPEN — whether the budget gate stays and what it stands on, given that it rests on the user's status-line `rate_limits` payload (the founder's question, finding F-06i) · owner: 0.2.0's intent.»
sidecar: [Budget Governor] — OPEN F-06i added: the gate's footing on the user's status line, owner 0.2.0's intent.
rationale: Report Decisions: the founder's own word, 2026-11-06 20:53Z (`inputs#I7` item 1), his question why viola leans on the user's status line goes to 0.2.0's intent (F-06i); Spec claims: the `budget-paused` refusal rests on the statusline-rate-limits row.
@proposal n=12 detector=D-new-detail severity=warning file=established-decisions anchor=fct-qwpklckh fact=F1 basis_doc=doc:2026-11-06-version-done-check/report basis_fact=fct-npz56o6m
section: ## Established Decisions
change: add after the [CLI Version Compatibility] ledger entity: «The capability ledger row `statusline-rate-limits` reads the account's plan state (on this account the status-line payload carries `five_hour` `unknown` and only `seven_day`), so `viola verify` can read it red on an unchanged CLI (2.1.287 read `23 pass  1 fail` on the rig home; the dev home's stamps are not moved); owner 0.2.0, the founder's finding F-06i.»
sidecar: [CLI Version Compatibility] — statusline-rate-limits measured as reading the account's plan, not the CLI; owner 0.2.0 (F-06i).
rationale: Report Spec claims disproved by measurement: the row stated as a CLI behaviour (fct-qwpklckh) read red on an unchanged 2.1.287 because the account's payload carries only the seven-day window; Counts: the ledger stays 24 rows, dev-home stamps unmoved.
@proposal n=13 detector=D-cascade severity=warning file=established-decisions anchor=fct-xqkfyxga basis_doc=doc:2026-11-06-version-done-check/report basis_fact=fct-gtooygvh
section: ## Established Decisions
change: [Error Handling] head names six ratified exceptions to one enum per workspace crate: `viola-pty`'s hand-written `PtyError` (its reason unchanged), and, ratified by the founder as named exceptions (T1, 2026-11-06), `viola-mcp`'s `ServeError` beside `McpError` and `viola-state`'s `StampsError` beside `StateError` (each with its fixed messages as now stated), with `PinError`, `StampsReadError` and `Refused` the three the census names; the «open divergence, not an exception», «its fold or ratification is open» and «The owner of all five: a CARRY on the working-route entry "Version done-check"» clauses leave; the AgentError, anyhow, RefusalReason and Rationale text stands.
sidecar: [Error Handling] — the five one-enum divergences ratified as named exceptions (T1), their CARRY retired.
rationale: Report Decisions: the founder's own word 19:34Z (`inputs#I2`) T1 — ratify the five one-enum divergences as named exceptions; expected amendment 2.
@proposal n=14 detector=D-cascade severity=warning file=body anchor=fct-sha7o35z dependent_of=D-cascade basis_doc=doc:2026-11-06-version-done-check/report basis_fact=fct-gtooygvh
section: ## Established Decisions
change: [Error Handling] broad line reads «ratified exceptions: `viola-pty`'s hand-written `PtyError`, and (T1) `viola-mcp`'s `ServeError`, `viola-state`'s `StampsError`, `PinError`, `StampsReadError` and `Refused`», in place of «one ratified exception … two open divergences … the rule standing».
sidecar: body [Error Handling] — divergences ratified as named exceptions (T1).
rationale: Report T1 (`inputs#I2`); expected amendment 2 (fct-sha7o35z).
@proposal n=15 detector=D-cascade severity=warning file=established-decisions anchor=fct-gctodeyz basis_doc=doc:2026-11-06-version-done-check/report basis_fact=fct-u65xlcax
section: ## Established Decisions
change: The product sources (outside the test-only `crates/viola-e2e`) hold 9 `thiserror::Error` derives, the census of [Error Handling]: `AgentError`, `ChannelError`, `StateError`, `PinError`, `StampsError`, `StampsReadError`, `Refused`, `McpError`, `ServeError`; `command grep -rn "thiserror::Error" src crates --include=*.rs` reads 10 rows, the 10th `crates/viola-e2e/src/harness/mod.rs:25`, test-only.
sidecar: [Error Handling] census — rule re-worded to the product sources, count stays 9 (grep 10 rows, 1 test-only).
rationale: Report Counts: the grep reads 10 rows, 9 outside `crates/viola-e2e` (the 10th `crates/viola-e2e/src/harness/mod.rs:25`, test-only); the rule re-worded to the product sources, the count stays 9 (`inputs#I7` item 4).
@proposal n=16 detector=D-cascade severity=warning file=established-decisions anchor=fct-wtlwpexy basis_doc=doc:2026-11-06-version-done-check/report basis_fact=fct-q2lwjk3d
section: ## Established Decisions
change: the closing clause reads «Its cause is not explained; it is recorded as known, with no action in 0.1.0, and the pinned copy's re-hash stays the guard (the founder's own word).» in place of «acting on it is owed to the working-route entry "Version done-check"»; the measurement stands.
sidecar: [Deployment / Distribution] — Measurement B recorded as known, no action in 0.1.0, the re-hash the guard.
rationale: Report Cross-project `inputs#I2` dialog 7/7: Measurement B known, no action in 0.1.0; P5 card row 7 «the pin re-hash stays the guard»; expected amendment 5.
@proposal n=17 detector=D-cascade severity=warning file=body anchor=fct-c3wssf3l dependent_of=D-cascade basis_doc=doc:2026-11-06-version-done-check/report basis_fact=fct-q2lwjk3d
section: ## Established Decisions
change: [Deployment / Distribution] Rationale's last clause reads «its cause not explained, recorded as known with no action in 0.1.0, the re-hash the guard» in place of «acting on it owed to the working-route entry "Version done-check"».
sidecar: body [Deployment / Distribution] — Measurement B known, no action in 0.1.0.
rationale: Report `inputs#I2` dialog 7/7, card row 7; expected amendment 5 (fct-c3wssf3l).
@proposal n=18 detector=D-retired severity=warning file=established-decisions anchor=fct-onw4pywk basis_doc=doc:2026-11-06-version-done-check/report basis_fact=fct-uufjzm4k
section: ## Established Decisions
change: [CLI Conventions] the sentence «viola warns when an argument carries a Git Bash rewritten-path prefix.» leaves and the Rationale reads «agents need machine-parseable refusals.», the Git Bash `/skill` rewriting reason (brief §6) removed; the stdin or `--file` rule, the leading-slash ban and the `viola revive --json` text stand.
sidecar: [CLI Conventions] — the Git Bash rewritten-path warning and its rationale retired (T2).
rationale: Report Symbols: `warn_if_rewritten` / `looks_rewritten` removed, the only stderr line `send` wrote on a rewritten path gone; Retired: the Git Bash rewritten-path warning (T2, `inputs#I2`); expected amendment 4.
@proposal n=19 detector=D-retired severity=warning file=body anchor=fct-nvft5tqc dependent_of=D-retired basis_doc=doc:2026-11-06-version-done-check/report basis_fact=fct-uufjzm4k
section: ## Established Decisions
change: [CLI Conventions] broad line drops «viola warns when an argument carries a Git Bash rewritten-path prefix.» and its Rationale reads «agents need machine-parseable refusals.»; the stdin or `--file` rule and the leading-slash ban stand.
sidecar: body [CLI Conventions] — Git Bash warning retired (T2).
rationale: Report Retired (T2); expected amendment 4 (fct-nvft5tqc).
@remark
Outside this group's categories, for other passes: Conventions fct-nhsg4k7g and Inherited Defaults fct-vf75c7sp follow T1 and the census rule; Occupied Resources fct-i4wacyrp and fct-6e4hf2ai take the ceiling owner 0.2.0's first entry; body [Budget Governor] fct-3hy3yfb4 is reached only indirectly through F2; crosscut-config fct-k6htnrfj loses its verbatim-drive clause; F3 is cross-cutting-patterns.
