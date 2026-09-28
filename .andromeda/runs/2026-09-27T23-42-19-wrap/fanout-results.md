# Fan-out results — 2026-09-27-hooks-to-normalised-events

Report: `viola-0.1.0/chunks/2026-09-27-hooks-to-normalised-events/report.md` (reused as is on resume).

## Run history
- **First fan-out, 2026-09-27T23:50Z** (this run dir, the halted window): 47 proposals, dispositioned, none applied; the wrap
  HALTED at P2 Validate on E1 (S1–S6, the interim `hook.event` gap), the overseer declining to ratify at night. That record
  held the dispositions but not the parsed proposal lists, so the Setup 2a resume could not reuse the fan-out.
- **Re-fan, 2026-09-28T04:25Z** (resume, fresh window): seven Explore doc-agents, one batch, prompt verbatim from
  amendment-flow.md. Every return was clean YAML; stripping removed only `#` comment lines (listed per doc below) and no
  HTML entity arrived, so no raw twin is warranted. The lists and dispositions below are this re-fan's.
- **E1 ratified, 2026-09-28 06:21 (local):** the founder answered LIVE in the Viola overseer session (AskUserQuestion), after
  being shown each widening: E1, the interim `hook.event` gap until the Epoch 6 entries, is RATIFIED — relay: the Viola
  overseer. In the same answer the `FAKE_AGENT_HOOK_PANIC` seam for the "Hook perf gate" tail entry is RATIFIED.

## Verdicts
| doc | proposals | note |
|---|---|---|
| architecture | 14 | 11 primaries + 3 dependents |
| security-plan | 7 | all E1's subject: 1 primary (Decisions Log) + 4 auth dependents; 1 input primary + 1 dependent |
| design-system | 0 | stripped comment: no Coverage entry flagged `hardcoded✗`; `viola hook` hidden, adds no UI |
| layout-templates | 0 | stripped comment: no new surface; layout-templates.md:499 already says `viola hook` has no human surface |
| test-plan | 14 | D-tests-framework no drift (stripped comment) |
| obs-plan | 0 | stripped comment named 4 stale sites outside its detectors (spine 1.0 s ×2, D-28 >4 KiB, §10 perf rows, §4 Scenario 1) — raised by the orchestrator below |
| a11y-plan | 0 | stripped comment: no interactive element; neither schema changed |

## Parsed lists + dispositions

### architecture
- **A1** D-arch-resources · §Occupied Resources → Repository · change: `crates/viola-core/proptest-regressions/` and
  `crates/viola-agent-claude/proptest-regressions/` (the latter holds `hook.txt`) · basis architecture.md:394.
  → APPLY, playbook "Accurate this-chunk addition" (expected amendment 2).
- **A2** D-arch-resources, dependent-of A1 · §Infrastructure Patterns → Project directory structure · the
  `viola-agent-claude/` tree entry gains `(+ proptest-regressions/, committed seeds)` · basis :488. → APPLY (dependent).
- **A3** D-arch-resources · Repository `fuzz/` bullet · targets today `viola_name`, `channel_frame`, `hook_stdin`; corpus
  `hook_stdin` 10 seeds · basis :395. → APPLY, accurate addition (expected amendment 2).
- **A4** D-arch-resources, dependent-of A3 · directory tree `fuzz_targets/{viola_name,channel_frame,hook_stdin}.rs` ·
  basis :517. → APPLY (dependent).
- **A5** D-arch-resources · §Occupied Resources → Workspace crates, Not-a-member bullet · `viola-fuzz` depends on
  `viola-core` and `viola-agent-claude` by path · basis :358. → APPLY, accurate addition.
- **A6** D-arch-resources · §Infrastructure Patterns → Crate dependency direction · `viola-agent-claude` as landed depends
  on `viola-core`, serde, serde_json, serde_path_to_error `=0.1.20` and thiserror; dev-deps proptest, rstest; `viola-state`
  and vt100 arrive later · basis :447. → APPLY, accurate addition (expected amendment 4).
- **A7** D-arch-resources · §Occupied Resources → Filesystem `diagnostics/` · `run` and `hook` have producers today;
  `hook`'s drift reports go only to `detail-hook.ndjson` · basis :380. → APPLY, accurate addition.
- **A8** D-arch-resources · Repository · new bullet `target/mutants/`, the `run --mutants` cargo target dir
  (`MUTANTS_TARGET`), root pre-build absolute, cargo-mutants relative, still `--copy-target=true` · basis :399. → APPLY,
  accurate addition (disproved claim (c)).
- **A9** D-arch-resources · §Standard Contracts → Channel methods · new bullet `hook.event` id-less `{ts, event:{kind,
  data}}`, no reply; the wrapper re-validates (5 hook kinds, object `data`, `prompt-submitted` string `text` + `origin` ∈
  harness|human) and appends `{v, ts:<append time>, instance, kind, source:"hook", data}`; invalid → not appended; append
  failure `-32603` · basis :266. → APPLY, accurate addition.
- **A10** D-arch-decisions · [Deployment / Distribution] · the `{"hooks": {}}` placeholder retired; seven exec-form
  entries with tiers; PreToolUse/PermissionRequest arrive with the dialog chunk; only `.mcp.json` stays `{"mcpServers":
  {}}` · basis :99. → APPLY, accurate addition (expected amendment 1).
- **A11** D-arch-decisions · [Hook Transport] spine tier · "the spine deadline is no longer an open item; it is the
  hook-local provisional `SPINE_DEADLINE` of 750 ms" · basis :64. → APPLY, RE-DERIVED: the value is provisional and
  crate-private (report Symbols), so naming it as the product constant the perf gate reads stays open, with the
  "Readiness gate and timing constants" entry ("named confirmation window and deadlines"); the proposal's "no longer an
  open item" is not applied (expected amendment 5).
- **A12** D-arch-decisions, dependent-of A11 · [Hook Transport] SessionEnd · replace `~1 s` with the 750 ms deadline; the
  fallback is `try_append_event` · basis :65. → APPLY, RE-DERIVED: the connect deadline is the same provisional 750 ms and
  the fallback `try_append_event`; the `~1 s` budget stays (the report does not retire it, and test-plan :122/:385 rest on
  it).
- **A13** D-arch-decisions · [CLI Version Compatibility] paste-wrapper row · the CLI's unescaped pair
  `<pasted_content id="X">\n…\n</pasted_content id="X">` (same id), unwrapped only as that exact pair, ends kept byte for
  byte · basis :81. → APPLY, accurate addition (expected amendment 3).
- **A14** D-arch-decisions, dependent-of A13 · tag-escaping row · a typed close arrives as `<\/pasted_content`;
  un-escape `<\` before an ASCII letter or `/` after unwrapping, so a typed pair is never unwrapped · basis :89. → APPLY
  (expected amendment 3).
- **Orchestrator-raised (check 5 / the sweep):**
  - **A15** dependent-of A9 · §Standard Contracts frame example :261 · the `hook.event` example carries `data`. → APPLY.
  - **A16** dependent-of A10 · §Occupied Resources → Claude Code integration names :349 · seven events registered today;
    PreToolUse and PermissionRequest join with the dialog chunk. → APPLY.
  - **A17** [Snapshot writer] :51 · the Windows access-denied (raw 5) retry, every 10 ms, at most `REPLACE_ATTEMPTS` = 100,
    re-persisting the same temp file. → APPLY, accurate addition (the chunk's CARRY 5; report Symbols `viola-state`).

### security-plan (E1 — RATIFIED by the founder, relay: the Viola overseer)
- **S1** D-security-auth · §Security Decisions Log · new entry: `viola hook` sends `hook.event` to the snapshot endpoint
  with no server verification and no strict-modes until Epoch 6; the wrapper serves `hook.event` with re-validation ·
  basis report.md:29-39,45-48; security-plan.md:577,587. → APPLY as the founder's ratification (playbook "Boundary
  widening" + "what ratifies it": a live answer given after the widening was shown; relay named). Also discharges the
  §Anti-Patterns → Universal rule "no new channel method without a Decisions Log entry".
- **S2** D-security-auth, dependent-of S1 · §Authentication, IPC client-side server verification row, Timing · interim
  `hook.event` exception · basis :205. → APPLY (E1).
- **S3** D-security-auth, dependent-of S1 · §Authentication, `~/.viola/` access control row, strict-modes entry points ·
  `hook <event>` interim gap · basis :207 (offsets @c929/@c954). → APPLY (E1).
- **S4** D-security-auth, dependent-of S1 · §Security Anti-Patterns → Authentication · the frame-verification ban gains
  its dated interim exception · basis :503. → APPLY (E1).
- **S5** D-security-auth, dependent-of S1 · §Error Handling, Internal logging · the hook's fail-open checks as run today;
  server verification and strict-modes join at Epoch 6 · basis :478. → APPLY (E1: a restatement site of the same gap,
  not a new widening).
- **S6** D-security-input · §Input Validation, CLI arguments / stdin row, Home path · `viola hook` reads `VIOLA_DIR`
  without canonicalising or strict-modes; shape-checked only; interim · basis :229. → APPLY (E1).
- **S7** D-security-input, dependent-of S6 · §Input Validation, Own state files row, Integrity · interim exception for
  the hook's `snapshot.json` `endpoint` read · basis :233. → APPLY (E1).
- **Orchestrator-raised (check 5):**
  - **S8** §Input Validation, Hook stdin row :224 · `take(MAX_FRAME + 1)` detects the over-limit; the event argument is
    the closed `HookEvent` (unknown → silent exit 0); fail-open details `oversize-stdin` · `malformed-json` ·
    `channel-unreachable`. → APPLY, accurate addition (expected amendment 7; validation present).
  - **S9** §Input Validation, Channel frames row :223 · the wrapper's `hook.event` re-validation (closed kind set, object
    `data`, `prompt-submitted` `text`/`origin`). → APPLY, accurate addition (validation present; within S1's subject).
  - Threat Model Summary hits (:58, :90, :114) → REJECT for that section, playbook "Verbatim upstream copy".

### test-plan
- **T1** D-tests-obs-harness · §3 `run` step 4 Mutation · `CARGO_TARGET_DIR=<repo>/target/mutants` for the root
  pre-build, relative `target/mutants` for cargo-mutants (was `env_remove`), still `--copy-target=true`; the copied
  default `target/` kept the original tree's `CARGO_BIN_EXE_*` paths · basis :554. → APPLY, accurate addition (claim (c)).
  The obs-plan §3 bind is unaffected (no mutation-target text there).
- **T2** D-tests-obs-harness · §3 `boot` readiness · line-3 `session-start` check joins with "Capability ledger and viola
  verify"; `boot` checks lines 1–2 · basis :526. → APPLY, playbook "Sequencing deferral" (expected amendment 9); owner
  pinned at P5 (CARRY on that entry).
- **T3** dependent-of T2 · §1 `boot` readiness :174. → APPLY.
- **T4** D-tests-coverage · §10 Performance budgets · status note: hook perf rows + `--perf` not built; carried to "Hook
  perf gate"; interim bound = the fail-open matrix `< 1.0 s`; `SPINE_DEADLINE` 750 ms hook-local · basis :1519-1525. →
  APPLY, RE-DERIVED: "UNRATIFIED" is dropped (the tail's seam is ratified; the perf rows were never a widening); sequencing
  (expected amendment 13).
- **T5** dependent-of T4 · §2 Performance row :435. → APPLY.
- **T6** dependent-of T4 · §9 Perf row :1447. → APPLY.
- **T7** D-tests-coverage · §6 Security sweep :1254 · the forced-panic case + trigger → the tail. → APPLY, RE-DERIVED: the
  trigger `FAKE_AGENT_HOOK_PANIC` (fake-agent-only) is RATIFIED by the founder 2026-09-28 (relay: the Viola overseer) and
  lands with "Hook perf gate", not "rejected"/"UNRATIFIED"; the in-process unit coverage stands (expected amendment 11).
- **T8** dependent-of T7 · §6 fail-open case list :1266. → APPLY.
- **T9** D-tests-coverage · §5 Module ↔ DB concurrent-append :924 · landed for the hook files (8 processes, 16 + 8
  lines, 3 OSes) without the >4 KiB line; that half → the tail. → APPLY, accurate + sequencing (expected amendment 12).
- **T10** D-tests-coverage · §6 Property suite :1326 · hook stdin + prompt round-trip properties landed; `resets_at` →
  "Statusline pass-through"; the fuzz paragraph names `hook_stdin`. → APPLY (expected amendment 10).
- **T11** dependent-of T10 · §2 Property row :434 · fuzz list gains `hook_stdin`. → APPLY.
- **T12** dependent-of T10 · §3 `--fuzz-replay` seed list :560 · `hook_stdin`: 10 seeds. → APPLY.
- **T13** D-tests-coverage · §6 Path 1 :1028 · the placeholder clause retired; records 1–3 + M6 asserted by the sibling
  test. → APPLY, accurate addition (expected amendment 1's test-plan hit).
- **T14** D-tests-coverage · §7 Fake agent :1362 · fires SessionStart/default once after `start_receipts`. → APPLY
  (expected amendment 8).
- **Orchestrator-raised (the sweep):**
  - **T15** dependent-of T2 · §1 cli surface :228 ("once Hooks to normalised events lands"). → APPLY.
  - **T16** dependent-of T7 · §1 hook surface Required test type :331 (a forced panic). → APPLY.
  - **T17** dependent-of T10 · §6 budget path step 2 :1138 (the `resets_at` property pointer). → APPLY.

### obs-plan (all orchestrator-raised, check 5)
- **O1** §4 Scenario 1 :870 · `session-start{source:"hook"}` is record three. → APPLY, accurate addition.
- **O2** §3 D-28 :621 · the check landed for the hook files without the >4 KiB line; that half goes with the panic seam to
  "Hook perf gate". → APPLY, accurate + sequencing (expected amendment 12).
- **O3** §10 Performance budgets · status note: the hook rows land with "Hook perf gate". → APPLY, sequencing (expected
  amendment 13).
- **O4** §1 / §2 / §5 / §9 hyperfine mentions (:417, :504, :1004, :1244, :1285) → NO CHANGE: they state the planned gate,
  and §10's status note carries the timing (§1 is also the verbatim scope copy).
- **O5** §10 :1321 · the gate stays provisionally 1.0 s (tests-owned) and the hook's own provisional 750 ms deadline sits
  below it. → APPLY, re-derived (expected amendment 5). The same fact at :419 is in §1 → REJECT for §1, playbook "Verbatim
  scope copy" (first applied, then reverted in this pass).

## Validate checks
1. **Playbook** — above. No proposal re-derives from git/source (every basis is the report or the doc).
2. **Cross-contradiction:** none. A11/A12, T4 and O5 agree: gate provisionally 1.0 s, hook deadline 750 ms, SessionEnd
   budget ~1 s.
3. **Intent-consistency:** the report's deviations are justified (the obs init reuse, the `is_spine` omission, the test-only
   `open`, the sibling Path 1 test, the harness fold, the hygiene rename) and consistent with the working-route entry and
   the plan acceptance.
4. **Absence-needs-evidence:** A8's "not registered" re-read: `grep -c target/mutants .andromeda/architecture.md` = 0. The
   security :207 hits were read by offset (@c929, @c954, @c2023).
5. **Expected amendments:** 1 A10 A16 T13 · 2 A1–A4 · 3 A13 A14 · 4 A6 · 5 A11 O5 · 6 S1 (E1) · 7 S8 · 8 T14 · 9 T2 ·
   10 T10 · 11 T7 · 12 O2 T9 · 13 O3 T4. Every entry covered.
6. **Disproved claims:** (a) and (b) are plan-only, closed by the implement edits, routed to curation. (c) is disposed by T1
   and A8.

## Escalations
- **E1 — RESOLVED (ratified).** Subject S1–S7 (+ S9 inside it): the interim `hook.event` gap. Ratified by the founder live,
  2026-09-28 06:21, relay: the Viola overseer. Recorded in the security-plan sidecar as the founder's.
- **Seam ratification (P5 subject, recorded here for provenance):** `FAKE_AGENT_HOOK_PANIC` for the "Hook perf gate" tail —
  RATIFIED by the founder in the same answer. It lands with that entry; its security-plan Decisions Log entry is written
  when it lands (security.md: "Another seam needs a Decisions Log entry").
