# Fan-out results — 2026-10-04-dialog-answers-by-dialog-id (wrap resumed in this run dir)

Seven Explore doc-agents, one batch, prompts per `amendment-flow.md` (detector counts arch 2 · security 3 · design 1 ·
layout 1 · tests 3 · obs 3 · a11y 2 = 15 = the drift-base `doc:` names). Keyed-contract renders for arch · tests · obs ·
a11y (`{doc}-contracts.md` here). Entity probe on every return: `entities=0` (no `&lt;` `&gt;` `&amp;` in any value).
Registry U40 not present in `upgrade.py detect` (16 detectors of 35 entries) → the D-3 (a) walk-class marks do not fire.

## Verdict lines
- architecture — 28 proposals (D-arch-resources 20 · D-arch-decisions 8); nothing stripped.
- security-plan — 13 proposals (D-security-auth 10 · D-security-input 3); stripped: a trailing note block — D-security-deps
  no drift (insta dev-only exact pin, libc/windows-sys already pinned, `cargo deny` green); the R3 creation half needs no
  amendment (:207 · :265 · :403 · :546 already state the protected user + SYSTEM DACL at creation); the `viola answer`
  snapshot-read dependents are inferred from the liveness-only pre-check's definition.
- design-system — 0 proposals; stripping removed a commentary block → raw twin `.raw-fanout-design-system.md`.
- layout-templates — 1 proposal; nothing stripped.
- test-plan — 19 proposals (D-tests-coverage 8 · D-tests-obs-harness 9 · D-tests-framework 2); stripped: a no-drift note
  (insta already named at 1.48.0 in §2/§4/§7, `crates/*/src/snapshots/` already in §2; log format / status / PID unchanged).
- obs-plan — 6 proposals (D-obs-instrumentation 6); nothing stripped.
- a11y-plan — 1 proposal (D-a11y-surface); stripped: D-a11y-obs-schema no drift (the `parse-rejected.detail` enum
  extension is additive; a11y rows validate against `a11y-row.v1.json`).

## Parsed proposals and dispositions
Disposition key: apply (check, rule) · escalate (check, rule) · reject. `dep` = dependent-of.

### architecture
- A1 D-arch-resources · §Standard Contracts › `hook.dialog` (:286) — served; params `{kind, data, hook_event, tool?, input?,
  continuation?}` → `{dialog_id, response}`; continuation = a PermissionRequest repeating the armed dialog's tool with an
  equal `tool_input` (question and plan), no event, no new id, revise only. → **escalate E1** (check 1, "Boundary widening":
  a channel method newly served); text otherwise accurate this-chunk.
- A2 dep A1 · §Standard Contracts dialog-event paragraph (:307) — exactly-once rule names the matched-continuation
  exception. → apply after E1 (check 1, "Accurate this-chunk addition").
- A3 dep A1 · §Conventions Hook → kind map (:169) — PermissionRequest continuation raises no event. → apply after E1.
- A4 dep A1 · [Message Broker / IPC] (:52) — retire "no `hook.dialog` is served yet"; pending-dialog `after`-less return live.
  → apply after E1.
- A5 dep A1 · §Standard Contracts › `wait` (:276) — pending-dialog start; `appending_dialog` / `dialog_settled`. → apply after E1.
- A6 D-arch-resources · §Standard Contracts `from` paragraph (:274) — `answer` joins send/wait/last. → **escalate E1**
  (served method), expected amendment.
- A7 · §Conventions refusal order (:136) — `answer`: control-character first, both sides. → apply after E1.
- A8 · §Occupied Resources registered hook events (:362) — nine; dialog tier in `hooks.json`, `timeout` 75. → **escalate E1**
  (hook stdin admits two new events).
- A9 dep A8 · [Deployment / Distribution] (:99) — nine exec-form entries. → apply after E1.
- A10 dep A8 · [Hook Contract] (:69) — `DIALOG_DEADLINE` 60 s PROVISIONAL, `timeout` 75, read bound +5 s. → apply after E1
  (expected amendment).
- A11 · §Occupied Resources `target/perf/` (:428) — five rows, unstamped perf session. → apply (check 1, accurate addition).
- A12 dep A11 · `target/agent-run/` (:418) — five perf exports. → apply.
- A13 · §Occupied Resources `diagnostics/` (:398) — `answer` a `cli-<name>.ndjson` producer. → apply (expected amendment).
- A14 dep A13 · §Conventions exit 1 (:139) — `role_of` verb list gains `answer`. → apply (the list already enumerates verbs).
- A15 dep A13 · Build system key (`build-system.md:4`) — `human.rs` callers gain `answer`. → apply (key already enumerates callers).
- A16 dep A13 · Project directory structure key (:21) — `human.rs` callers + `run/dialog.rs`. → apply (tree already enumerates).
- A17 · §Occupied Resources `fixtures/claude/<cli-version>/` (:410) — sets 2.1.283 + 2.1.287, relayed dialog fixtures,
  `RELAYED.md`, hygiene-walked. → apply (expected amendment, CLI Version Compatibility family).
- A18 dep A17 · `target/e2e-home/` (:420) — `stamped_home` at 2.1.287. → apply.
- A19 dep A17 · Project directory structure key (:76) — fixtures dir also holds relayed fixtures. → apply.
- A20 · §Occupied Resources `ledger/stamps.json` (:401) — run's gate reads through `read_stamps_strict`. → apply (accurate
  addition; a tightening).
- A21 dep A20 · §Standard Contracts ledger stamps envelope (:253) — strict-modes refusal the second unverified path. → apply.
- A22 D-arch-decisions · §Stack test tooling (:38) — insta `=1.48.0` dev-only. → apply (Dependencies bullet).
- A23 dep A22 · Crate dependency direction key (:7) — viola-agent-claude dev-deps + `dialog` module. → apply.
- A24 dep A22 · Project directory structure key (:31) — snapshots at `crates/viola-agent-claude/src/snapshots/`. → apply.
- A25 D-arch-decisions · Crate dependency direction key (:6) — viola-state takes libc (unix) / windows-sys (windows,
  +`Win32_UI_Shell`). → apply (Dependencies bullet).
- A26 dep A25 · §Stack state-file primitives (:22) — libc/windows-sys uses in viola-state. → apply.
- A27 D-arch-decisions · [CLI Version Compatibility] (:91) — S3/S7/S8 + concurrency rows owed to `:82` (R1); non-null
  decisions flow on the six-row stamp, dated gap (R2). → **escalate E2** (check 1, "Boundary widening").
- A28 dep A27 · §Cross-cutting capability-ledger gate (:456) — the dated S3/S7/S8 exception named. → **escalate E2**.

### security-plan
- S1 D-security-auth · §AuthN&AuthZ IPC server verification (:205) — sixth dated gap: `answer` after the liveness-only
  pre-check, `hook.dialog` after the hook's shape check, until `:109` / `:111`. → **escalate E1** (check 1, "Boundary widening").
- S2 dep S1 · §Anti-Patterns › Authentication (:524) — the closed exception list gains both frames. → **escalate E1**.
- S3 dep S1 · `~/.viola/` access control strict-modes interim list (:207) — `viola answer`'s snapshot read. → **escalate E1**.
- S4 dep S1 · §Input Validation CLI arguments (:230) — `answer`'s liveness-only read; `<name>` via `ViolaName::try_new`,
  `<dialog_id>` a `u64`. → **escalate E1**.
- S5 dep S1 · §Input Validation Own state files (:235) — integrity exceptions name `viola answer`. → **escalate E1**.
- S6 D-security-auth · `~/.viola/` access control (:207) — second gap narrows to `verify`'s `update_stamps`. → apply
  (check 1, accurate addition; a tightening; expected amendment).
- S7 dep S6 · CLI arguments (:230). → apply.
- S8 dep S6 · Own state files (:235). → apply.
- S9 dep S6 · §Anti-Patterns › Universal (:605) — interim-worded ban restated as standing. → apply.
- S10 D-security-auth · §Anti-Patterns › Universal (:604) — non-null decisions on the 6-row stamp until `:82` (R2), the
  residual named. → **escalate E2**.
- S11 D-security-input (severity escalate) · Hook stdin (:225) — `HookEvent` 7→9, `classify` into closed dialog enums,
  `DialogMalformed` fails open, read bound `DIALOG_DEADLINE + 5 s`. → **escalate E1** (detector severity + widening).
- S12 D-security-input (escalate) · Channel frames (:224) — `hook.dialog` and `answer` param validation. → **escalate E1**.
- S13 D-security-input (escalate) · Dialog free text (:223) — `validate_paste_text` both sides. → **escalate E1**.

### layout-templates
- L1 D-layout-surface · §Surface: cli › Primary content block 2 (:547) — catch-site list gains `answer`. → apply (expected
  amendment).

### test-plan
- T1 D-tests-coverage · §6 Path 4 Surfaces (:779) — as-landed note; `permission` e2e owed `:82`. → apply (expected amendment).
- T2 · §6 Path 3 (:757) — question/plan wake witness landed; `permission` e2e owed `:82`. → apply (expected amendment).
- T3 · §2 Contract row (:438) — relayed dialog captures as a fixture source. → apply (expected amendment).
- T4 dep T3 · §7 Fixture library (:1041). → apply.
- T5 dep T3 · §7 Recorded hook payloads row (:1051). → apply.
- T6 dep T3 · §7 Fixture hygiene (:1084) — sets 2.1.283 + 2.1.287, relayed walked. → apply (the same line's fake-script
  list re-derived too: `gated-turn.json`, `path3.json`, `path4.json`).
- T7 · §7 Fake agent hook commands (:1060) — matchers evaluated against `tool_name`. → apply (report Symbols, fake agent).
- T8 · §6 Security sweep Windows `--home` (:989) — creation half landed (R3), unit both OS + integ; the RUNNER_TEMP
  `security_negatives_*` case stays owed. → apply (accurate addition).
- T9 D-tests-obs-harness · §3 → 5-command implementation `run --perf` (key :51) — five rows, unstamped. → apply (the "6
  passed" figure not touched: the report carries none).
- T10 dep T9 · key :149 — `gate --require perf` five rows. → apply (const name re-read at apply).
- T11 dep T9 · §10 Perf session (:1217). → apply.
- T12 dep T9 · §10 Status (:1221). → apply.
- T13 dep T9 · §10 budget table `pre-tool-use` row (:1227). → apply.
- T14 dep T9 · §2 Performance row (:435). → apply.
- T15 D-tests-obs-harness · key :45 `--mutants` invocation — `--build-timeout=400` replaces the multiplier. → apply (Red B,
  report Symbols harness).
- T16 dep T15 · key :45 rationale sentence. → apply.
- T17 dep T15 · key :49 `--package` arm. → apply after the shared-const read.
- T18 D-tests-framework · §7 Fake agent Modes (:1076) — default 2.1.287. → apply.
- T19 dep T18 · key :33 fixture chain literal — 2.1.287 when it is one of the three constants (plan :435 names
  `RECORDED_CLI_VERSION` / `DEFAULT_CLI_VERSION`). → apply after the read.

### obs-plan
- O1 D-obs-instrumentation · §6 `parse-rejected` detail catalog (:835) — `strict-modes-failed` for `ledger-stamps`. → apply
  (disproved-claims entry 3; expected-but-unplanned).
- O2 dep O1 · §6 Filesystem refusals (:837). → apply.
- O3 dep O1 · §4 Capability ledger edge flow (:731). → apply.
- O4 D-obs-instrumentation · §10 Status (:1076) — five timed rows. → apply.
- O5 dep O4 · §10 budget table (:1081). → apply.
- O6 dep O4 · §9 CI artifacts hyperfine row (:1002) — five exports. → apply (expected amendment).

### a11y-plan
- Y1 D-a11y-surface · §8 Timeout extensions CLI bullet (:823) — `answer` is bounded by `DIALOG_DEADLINE`; on expiry the hook
  fails open and the dialog stays with the human in the `claude` TUI. → apply after E1 (accurate addition; text re-derived
  from arch's fail-open contract, no claim beyond it).

### orchestrator raises (check 5 — expected amendments no detector proposed)
- X1 architecture §Standard Contracts › `answer` — `permission` suggestions a deliberate v1 limit (founder F3, live ~11:15Z):
  the permission answer stays `{behavior, message?}`; an offered `permission_suggestions` entry cannot be picked. → apply
  after E1 (the expected entry names the change; report Expected amendments "suggestions a v1 limit: carried").
- X2 test-plan §6 Path 4 step 1 — fixture names to the §2 form (`PreToolUse.ask-user-question.json`,
  `PreToolUse.exit-plan-mode.json`, `PermissionRequest.exit-plan-mode.json`, `PermissionRequest.ask-user-question.json`;
  the `permission` fixture owed `:82`). → apply (expected entry names it).
- X3 obs-plan §4 edge flow (:737) — "at the recorded version 2.1.283" → 2.1.287 (report Counts: the default moved; the
  architecture [CLI Version Compatibility] family). → apply.
- X4 security-plan `~/.viola/` access control — R3 creation half: no body change (the body already states the protected
  user + SYSTEM DACL at creation, :207 · :265 · :403 · :546; the landing matches it, founder's words "exactly as
  security-plan words it"); recorded at T8 (test-plan) and A25/A26 (arch). → no change.

### check 6 — disproved claims
1. print-mode probe → disposed at the phase revision (plan, not a master). 2. scope §2 premise → corrected in scope.
3. `strict-modes` detail → O1/O2/O3. 4. "17 lines" count → plan-only figure; no master carries it.

### superseded first-pass items
- verify `/10` (test-plan §3 · layout wireframe · design-system sample) — the counter stays `/06`: no change (report:
  unchanged truth). The `stamped 2.1.283` samples (layout :474/:478, design-system :801) and arch :251's envelope example
  are illustrative outputs, no change.

## Escalations
- **E1** — boundary widening, the sixth dated gap and the newly served dialog crossings: A1 · A6 · A8 · S1–S5 · S11–S13
  (+ their dependents A2–A5 · A7 · A9 · A10 · X1 · Y1 applying with them).
- **E2** — boundary widening, non-null decisions on the six-row spine stamp until `:82`: A27 · A28 · S10.

## Resolutions (the operator, this wrap, 2026-10-04, AskUserQuestion)
- E1 → ratified: recorded as the founder's live ruling F1 (answered live at P4, 2026-10-04 ~11:15Z, through the overseer's
  AskUserQuestion, the residual on PermissionRequest `allow` shown) — an existing live ruling, no new decision. Every E1
  proposal and its dependents → apply.
- E2 → ratified: recorded as the founder's live ruling R2 (answered live at the revision P4, ~14:08Z, relayed by the
  overseer, the residual shown), `:82` the closer. A27 · A28 · S10 → apply.
- Open escalations after this: 0. No new playbook rule: the widening class is never-routine by the founder's ruling.

## Apply outcome
- A22 → **reject** at apply (check 1, "Registry over-reach"): arch §Stack carries no test-library row; insta lands in
  `crate-dependency-direction` (A23) instead.
- Every other proposal and X1–X3 applied, the text re-derived from the report and the invariant; X4 no change.
- Cascade: one more same-master duplicate found by the sweep and amended (test-plan:408, T3's claim); leaves per
  `cascade-dispositions.md`.
- Sidecar entries: architecture 2 · security-plan 2 · test-plan 2 · obs-plan 1 · layout-templates 1 · a11y-plan 1, each
  `sidecar.py check` on-form, appended by `splice.py append`, read back.
- Totals: 68 proposals → 66 applied (incl. dependents) · 1 rejected (A22) · 1 superseded by apply detail (none) — plus 3
  orchestrator raises applied and 1 cascade amendment; 2 escalation groups (E1, E2) resolved with the operator.
