# Validation — 2026-09-27-instance-state-and-start-order

48 proposals (fan-out) + 1 orchestrator-raised. No proposal cites a source the report does not carry (the
re-derivation tell): none rejected on that ground. Escalations: 0 open — two classes that would escalate are settled
by RECORDED operator directions (below), so no HALT.

## Check 1 — playbook
| id | doc:site | disposition | rule / reason |
|---|---|---|---|
| A1 | arch:50 [Snapshot writer] | APPLY | reversal of a locked decision — settled by the recorded operator ruling at P4 and the overseer wrap directive item (3) ("the arch [Snapshot writer] amendment to tempfile persist"); no playbook rule matches a directed reversal → apply, propose the rule at the card |
| A2 | arch:22 Stack row | APPLY (re-derived) | Accurate this-chunk addition; + `.lock` siblings opened write-mode (report Symbols, measured os error 5) |
| A3 | arch:594 Database | APPLY | dependent of A1 |
| A4 | arch:435 viola-state deps | APPLY (re-derived) | Accurate this-chunk addition; `notify (tailing)` kept as the future edge |
| A5 | arch:23 Content hash row | APPLY | Accurate this-chunk addition (sha2 product dep, dev-profile opt-level 3) |
| A6 | arch:310 Session liveness | APPLY | Accurate this-chunk addition (expected amendment 5) |
| A7+A9 | arch:92 [Session Liveness] | APPLY (one re-derived edit) | start order as landed, gone by pid + start time, endpoint/version-gate slots named as later steps |
| A8 | arch:246 Instance snapshot | APPLY | expected amendment 4 |
| A10 | arch:138 exit 1 | APPLY | Accurate this-chunk addition |
| A11 | arch:411 lint / stderr site | APPLY RE-DERIVED FROM CODE — proposal text REJECTED | the proposal says a local `#[allow(clippy::print_stderr)]` sits on `refuse`; `grep allow\(clippy::print_std src/` finds none in the root bin (only the fake agent's crate-level allow): the refusals write with `writeln!` on the locked stderr, which `print_stderr` does not fire on. The old text's "local allow" was already false; the body now states the code |
| A12 | arch:372 bin/ | APPLY | Accurate this-chunk addition |
| A13 | arch:98 [Deployment] | APPLY + A17's fact folded in | re-hash refusal; plugin ships empty `hooks`/`mcpServers` until the `hook`/`mcp` verbs (operator ruling P4) |
| A14 | arch:353 Landed so far | APPLY | expected amendment 3 |
| A15 | arch:421 rust plane | APPLY | expected amendment 3 |
| A16 | arch:425 licence list | APPLY | expected amendment 3 |
| A17 | arch:345 integration names | REJECT (registry) — fact moved to A13 | the names registry (plugin `viola`, MCP server `viola`, tools) stays true; the empty-content fact is realization, stated where the plugin content is described ([Deployment] :98) |
| A18 | arch:436 agent-claude as-landed | APPLY | the line already enumerates its as-landed symbols; + `PLUGIN_DIR_FLAG`, `plugin_files` |
| A19 | arch:390 e2e-home | APPLY (re-derived) | + owner record / gone-owner sweep; "kept in CI" narrowed: not under `run --mutants` |
| A20 | arch:544 pre-push | APPLY | Accurate this-chunk addition (Harness surface) |
| S1–S5 | security-plan:207, 197, 265, 266, 380 | APPLY | expected amendment 6 (the `.url` and `settings.json` writers are not built yet: the named primitive is the project's one atomic helper, so the rename is current truth for them too) |
| S6 (raised) | security-plan:686 (+ :426) `env -i` | APPLY — overseer RATIFICATION | `pre-push` now adds one assignment, `TMPDIR=<distro home>/viola-pre-push-scratch`, to the Linux mutation leg's `env -i`; `:686` says "HOME + PATH only". Boundary-widening class (never routine) — ratified by the overseer's recorded word before the fan-out: recorded as the exact carve-out (a named, constant, distro-derived assignment; no host value), so a future assignment carrying a host value still reads as a widening |
| O1 | obs-plan:853 span status line | REJECT — Sequencing deferral | spans are "Wrapper channel"'s (working-route:40 CARRY lands the first spans); the Scenario-1 span set rides that entry as a CARRY at P5 (plan's route freight) — a status line in the body is not current truth of the design |
| O2 | obs-plan:1073 detail catalog | APPLY | expected amendment 7 |
| O3 | obs-plan:856 collision outcome | APPLY | dependent of O2 |
| O4 | obs-plan:620 run-file collision append | APPLY | dependent of O2 |
| — | obs-plan:72, :185 (§1) atomic-write-file | REJECT — Verbatim scope copy | §1 keeps its wording; §3/§6/§12 win |
| T1–T5 | test-plan:523-526, 174, 228-229, 1027-1031, 542 | APPLY | readiness staged; endpoint / session-start later (expected amendment 8, Spec claims 2, 5); test-plan §1 carries no verbatim label (read: `test-plan.md:20` heading, no VERBATIM marker in :1-220) |
| T6–T9 | test-plan:555, 597, 746 ×2 | APPLY | mutation no-keep override; fixture owner record + sweep |
| T10–T11 | test-plan:664, 662-663 | APPLY | pre-push cache fields + TMPDIR scratch |
| T12 | test-plan:509 | APPLY | `logs --kind` landed |
| T13–T16 | test-plan:859, 435, 1526, 210 | APPLY | classify as a pure fn over an injected age; gone = process check |
| T17–T18 | test-plan:944-945, 432 | REJECT — Sequencing deferral | the crate-level `crates/viola-state/tests/` round-trip suite is owed by the chunk that lands the reader: CARRY on "Self-healing state" (working-route:65) at P5 |
| T19–T20 | test-plan:1016-1032, 542 (fixture choice) | REJECT — Sequencing deferral | the `path_` E2E binary for Path 1 lands when its later steps exist: CARRY on "The board: viola list" (working-route:73) at P5 |

## Check 2 — cross-contradiction
A13 and A17 touch the same fact (plugin content) in different sections — resolved by moving A17's fact into A13.
No opposing edits.

## Check 3 — intent-consistency
Every deviation in the report carries its justification (report §Deviations); none contradicts the working-route
entry or the plan's acceptance criteria. The plan's step-3/4 wording (lock via `open_private_append`) and step-5
wording (the drop joins the thread) were plan text disproved by measurement — plan-only, dispositioned in check 6.

## Check 4 — absence needs evidence
The only absence claims used: "no allow in the root bin" (A11) — `grep -n "allow(clippy::print_std" src/` read:
one hit, `src/bin/viola-fake-agent.rs:6` (the fake agent's crate-level allow), none in `src/cmd` / `src/run` /
`src/obs.rs` / `src/main.rs`; "test-plan §1 is not verbatim" — `test-plan.md:1-220` headings read, no VERBATIM marker.
Line profiles and every sweep hit: `cascade-sweep.md` (step 2).

## Check 5 — expected amendments
All 8 carried: 1 → A1/A2/A3 · 2 → A4 · 3 → A14/A15/A16 · 4 → A8 · 5 → A6 · 6 → S1–S5 · 7 → O2 · 8 → T4.

## Check 6 — disproved claims
1 → A1–A3, S1–S5 (obs §1 kept verbatim) · 2 → A8, T1–T3 · 3 → plan text only; `.claude/rules/events.md` already
states "append + exclusive lock fails on Windows"; the arch Stack row now names write-mode lock files (A2) ·
4 → plan text only (no master states that the heartbeat guard joins: see `cascade-sweep.md`) · 5 → T4.
