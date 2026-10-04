# Fan-out results — 2026-10-04-the-wheel (wrap 2026-10-04T20-44-01)

Seven Explore doc-agents, one parallel batch, prompts verbatim from `amendment-flow.md` (contracts lines for
architecture · test-plan · obs-plan · a11y-plan; `registry.py contracts` printed `n/a` for the other three, line
dropped). Detector counts per prompt: arch 2 · security 3 · design 1 · layout 1 · tests 3 · obs 3 · a11y 2 = 15 =
the drift-base's `doc:` names. Returns carried no HTML entities (`<` `>` `&` raw; entity probe 0).

## Verdicts
- architecture — 10 proposals
- security-plan — 8 proposals
- design-system — 0 (`proposals: []`; stripping removed a comment block: every Coverage row `tokens n/a`) — raw twin `.raw-fanout-design-system.md`
- layout-templates — 0 (`proposals: []`; stripping removed a comment block, incl. the note that layout-templates §Component — Primary content block 2's failed-verb list names `send, wait, last, answer, verify` and not `pause` / `release`) — raw twin `.raw-fanout-layout-templates.md`
- test-plan — 2 proposals
- obs-plan — 9 proposals (comments stripped: D-obs-stack no drift — no dependency changed; D-obs-pii no drift — keystrokes reach no log)
- a11y-plan — 5 proposals

## architecture (10)
- A1 · D-arch-resources · §Standard Contracts → Channel methods · `pause {from?}` → `{wheel:"human"}`; `release {budget?:bool, from?}` → `{wheel:<holder>, budget_paused:false}`; string `from` → `-32602` "invalid params" `data:{"reason":"release-from-driver"}` + one obs line; other `from` type / non-bool `budget` (incl. null) → `-32602` `data:null`; `budget:true` leaves the wheel; driver-held `release` no-op; replies after the `wheel` record lands (`-32603` if not) · basis architecture.md:282-283
  → **apply** (playbook: Accurate this-chunk addition; plan Expected amendment 2)
- A2 · D-arch-resources (dependent-of D-arch-resources) · §Standard Contracts → Channel methods preamble · `from` readers add `pause`; `release` refuses a string `from` · basis :274
  → **apply** (group with A1)
- A3 · D-arch-resources (dependent-of) · Established Decisions → [MCP] · the open "left to the security specialist" question closed: a string `from` on `release` is refused `release-from-driver`; self-reported, deters not enforces · basis :54
  → **apply** (Accurate this-chunk addition; the report's Symbols carry the refusal and `from_trust:"self-reported"`)
- A4 · D-arch-resources · §Occupied Resources → Filesystem `diagnostics/` · `cli-<name>.ndjson` producers add `pause` / `release` · basis :398
  → **apply** (Accurate this-chunk addition; report Coverage "pause.client / release.client spans + process lines")
- A5 · D-arch-resources (dependent-of) · §Conventions → CLI exit codes `1` · the cli-role verb list adds `pause` / `release`; any other reply = internal error exit 1 · basis :139
  → **apply**
- A6 · D-arch-resources · §Infrastructure Patterns → Project directory structure (key file) · `src/run/` adds `wheel.rs` + `snapshot.rs`; viola-pty adds `host_stdin()`; viola-core adds `HumanTyping` / `WheelCause`; `human.rs` adds the pause/release writers and callers · basis key file :21,25,36,39
  → **apply** (the key file enumerates per-file roles under `src/run/` and per-crate types, so this is its own grain, not Registry over-reach)
- A7 · D-arch-decisions · Established Decisions → [Human Takeover / Wheel] · "focus, mouse and resize do not count" → the F-W2 closed non-editing list (+ the classifier's edge rules) + the F-W3 Windows clause; the wheel's home `src/run/wheel.rs`, the one snapshot holder `src/run/snapshot.rs`, the worker thread, the bounded exit flush before raw restore · basis :70
  → **apply** (plan Expected amendment 1; founder live rulings F-W2 / F-W3, relayed by the overseer; report Spec claims disproved 1 + 3)
- A8 · D-arch-decisions · Established Decisions → [Human Takeover / Wheel] · "the `viola release` that returns the wheel also clears the running-turn state" → release clears none (none exists at HEAD); driver-held release a no-op · basis :70
  → **apply, re-derived** (plan Expected amendment 1 says "clears no running-turn state at HEAD (none exists)": the design clause stays as the intent, the as-built status beside it; the proposal's "recovered by the driver's `wait` `timeout_ms`" is not in the report → dropped; the owner question is P5's founder card)
- A9 · D-arch-decisions · Established Decisions → [PTY] · `viola_pty::host_stdin()`: on a Windows console viola's own `ReadConsoleW` reader (UTF-16 → UTF-8, split surrogate carried, every `0x1A` kept, 0-unit read re-read), because std's console read strips a trailing `0x1A` and ends input on a lone `^Z` (source-read at the pinned toolchain, measured on windows-2025, ci#37226294797); elsewhere `std::io::stdin()`; callers `pump_child` + the fake agent · basis :47
  → **apply** (plan Expected amendment 7; Spec claims disproved 2)
- A10 · D-arch-decisions (dependent-of) · §Stack → PTY layer row · windows-sys also backs the Windows console input read behind `host_stdin()` · basis :17
  → **apply, re-derived** (the proposal's feature name `Win32_System_Console` is not in the report → dropped; Dependencies none, so only the role is named)

## security-plan (8)
- S1 · D-security-input (escalate) · §Input Validation → Channel frames · a `pause` / `release` params clause: `pause` `from` per the `send` rule; `release` `budget` absent or bool (else `-32602` `data:null`), string `from` → `release-from-driver`, other `from` type → `data:null` · basis :224
- S2 · D-security-input (dependent-of) · §Authentication & Authorization → Driver-originated `release` · params `{budget?, from?}`; only a string `from` yields `release-from-driver` + obs line; other type `data:null`; CLI `viola release` sends `VIOLA_NAME`; driver-held release a no-op; `pause` with `from` accepted · basis :206
- S3 · D-security-input (dependent-of) · §Bootstrap phases → auth-scaffolding-baseline amendment 4 · `release-from-driver` landed in the bin's channel dispatch (`ProtocolError::ReleaseFromDriver`, −32602) · basis :397
- S4 · D-security-auth · §Authentication & Authorization → IPC client-side server verification · the seventh dated gap (F-W1): CLI `pause` / `release` frames after the liveness-only pre-check until `:109` / `:111`; borrows none of the six · basis :205
- S5 · D-security-auth (dependent-of) · §`~/.viola/` access control (strict-modes interim list) · a seventh: `viola pause` / `viola release` read the snapshot without strict-modes · basis :207
- S6 · D-security-auth (dependent-of) · §Input Validation → CLI arguments / stdin · the seventh gap by verb + `pause` / `release` arguments (`<name>` through `ViolaName::try_new`, `from` from `VIOLA_NAME` only) · basis :230
- S7 · D-security-auth (dependent-of) · §Input Validation → Own state files on read · the integrity-exception list adds `viola pause` / `viola release` · basis :235
- S8 · D-security-auth (dependent-of) · §Security Anti-Patterns → Authentication · the frame ban's dated exceptions add the `pause` / `release` frames (F-W1, the seventh) · basis :524
  → **S1–S8 escalate — E1** (playbook: Boundary widening + Boundary widening — what ratifies it: the IPC boundary gains two client crossings under a liveness-only pre-check, F-W1; S1–S3 carry detector severity escalate). The plan's "with its Decisions Log entry" is not applied: amendment-flow Apply step 1 — a plan's Decisions Log takes no new entry; the sidecar entry is the record (the sixth gap's precedent likewise carries none).

## test-plan (2)
- T1 · D-tests-coverage · §6 Path 5 → Surfaces involved · an as-landed note: `tests/tui_wheel.rs` + `tests/cli_wheel.rs` cover cli · tui · ipc-internal on all three CI OSes (F-W3 Windows clause on the focus case); the rmcp `send` step + MCP refusal checks owed to `:102`, the Playwright WHEEL cell to `:139` · basis test-plan.md:808
  → **apply, re-derived** (plan Expected amendment 5; test names the report does not carry — `path5_harness_turns_never_take_the_wheel`, `path5_pause_refuses_send_and_answer_and_release_returns_the_wheel` — dropped: the re-derivation tell)
- T2 · D-tests-coverage · §5 → `tests/cli_controls_not_disableable.rs` as-landed note · the human-wheel row has joined; "still to join: the 0770 `--home` negative and the completeness case" · basis test-plan.md:660-661 + `tests/cli_controls_not_disableable.rs:3-6`
  → **reject** (validate preamble: its basis and its "still to join" list cite a source location the report does not carry — the re-derivation tell); its fact **raised by the orchestrator** (check 5, plan Expected amendment 5 §5) from the report alone: the human-wheel row (`send` under a human wheel, exit 10) joins the table → **apply** (routine)

## obs-plan (9)
- B1 · D-obs-instrumentation · §4 Scenario "Human takes the wheel…" required span attributes · `run.wheel_transition.cause` `human-key` → `human-input` (`WheelCause::as_str`); point-in-time span, static name, on the wheel's worker thread; never the keys · basis obs-plan.md:684
  → **apply** (plan Expected amendment 4)
- B2 · D-obs-instrumentation · same scenario, must-trace spans + required log fields · `pause.client` / `release.client` › `channel.request(pause|release)` → dispatch, answered after the `wheel` record lands (`-32603` otherwise) · basis :683
  → **apply** (Accurate this-chunk addition; report Symbols + Coverage)
- B3 · D-obs-instrumentation (dependent-of) · §1 Obs Scope Summary Path 5 · `pause` joins `release` on the span chain · basis :331
  → **apply** (playbook: Verbatim upstream copy kept current — routine)
- B4 · D-obs-instrumentation · §4 Instrumentation per surface, `cli` row · dedicated spans add `pause.client`, `release.client` · basis :581
  → **apply**
- B5 · D-obs-instrumentation (dependent-of) · §4 Span kinds CLIENT · add `pause.client`, `release.client` · basis :573
  → **apply**
- B6 · D-obs-instrumentation (dependent-of) · §2 Naming conventions `<area>` list · add `pause`, `release` · basis :516
  → **apply**
- B7 · D-obs-instrumentation · §4 scenario release-from-driver bullet · the trigger is a string `from` (the CLI forwards `VIOLA_NAME`), INFO, `from` only when a valid name, beside the `-32602` reply; other `from` type / non-bool `budget` plain `-32602`; driver-held release appends nothing · basis :688
  → **apply**
- B8 · D-obs-instrumentation (dependent-of) · §3 → Log format JSON schema (key file) · `release-from-driver` corr rule: a `release` carrying a string `from` · basis key file :16
  → **apply**
- B9 · D-obs-instrumentation (dependent-of) · §6 Additive field catalog `release-from-driver` row · `from` present only when a valid `ViolaName` · basis :824
  → **apply**

## a11y-plan (5)
- Y1 · D-a11y-surface · §3 → Keyboard test harness (key file) · the tui wheel clause: the F-W2 closed non-editing list (terminal replies added); the F-W3 Windows clause (focus reports swallowed by the inbox ConPTY; an injected mouse report under win32-input-mode takes the wheel); resize never; real-terminal mouse measured live at `:82` · basis key file :8
  → **apply** (plan Expected amendment 6; F-W2 / F-W3; Spec claims disproved 1)
- Y2 · D-a11y-surface (dependent-of) · §1 → `viola run` TUI passthrough boundary bullet · same · basis a11y-plan.md:100
  → **apply** (Verbatim upstream copy kept current — routine)
- Y3 · D-a11y-surface (dependent-of) · §1 → Critical path 4 tui focus-order line · same · basis :321
  → **apply** (same rule)
- Y4 · D-a11y-surface (dependent-of) · §4 → P4 (tui) case (3) · as landed: step-wise (resize behind its size receipt, then focus reports with a mouse report as their read barrier), the wheel probed by an `answer` to no pending dialog (exit 13 driver / 10 human); Unix the full assertion; windows-2025 per F-W3 · basis :573
  → **apply** (report Deviations + Reverted facts carry the step-wise form and the `answer` probe)
- Y5 · D-a11y-surface (dependent-of) · §11 Anti-Patterns → Keyboard · the ban: never count focus, mouse, terminal-reply or resize sequences as editing; the windows-2025 platform fact (a mouse report ConPTY already turned into win32 key-down records is typing) · basis :990
  → **apply**

## Orchestrator raises
- L1 · layout-templates §Surface: cli → Component — Primary content block 2 · the failed-verb list (`send`, `wait`, `last`, `answer`, `verify`) adds `pause`, `release` (report Symbols: "any other reply = internal error, exit 1") → **apply** (routine, Accurate this-chunk addition; surfaced by the layout detector's stripped note)
- T2′ · test-plan §5 controls row (above)

## Validate — the six checks
1. Playbook — routine: A1–A10, T1, T2′, B1–B9, Y1–Y5, L1 (Accurate this-chunk addition; Verbatim upstream copy kept current for §1 hits; the plan's Expected amendments name each change). Escalate: S1–S8 → E1 (Boundary widening). No two-rule collision.
2. Cross-contradiction — none: A1/A2 (arch Channel methods), S1/S2 (security) and B7–B9 (obs) state one `release` shape; A7 / Y1–Y5 state one F-W2 list and one F-W3 clause.
3. Intent-consistency — the report's Deviations are justified (CI-measured, or the founder's live rulings F-W1–F-W3); the scope record's 4 lines (1 companion, 3 in-intent) each serve their named file. F-W3 narrows the plan's a11y acceptance on Windows — the founder's ruling is its justification → intent amended via Y1–Y5 + the v1-32 refine at P7.3.
4. Absence-needs-evidence — A3 / L1 / A5 rest on single-site reads; their sweeps are the cascade's (step 2) over all seven masters.
5. Expected amendments — 1 A7+A8 ✓ · 2 A1 ✓ · 3 S1–S8 (E1) ✓, Decisions Log entry not written (Apply step 1) · 4 B1 ✓ · 5 T1 + T2′ ✓ · 6 Y1–Y5 ✓ · 7 A9 ✓ · ledger notes v1-31 / v1-40 → P7.3.
6. Disproved claims — 1 → Y1–Y5 + A7 + `matrix.py refine` v1-32 at P7.3 (PREMISE-CORRECTION) · 2 → A9 · 3 → A7. All DISPOSED.

## Escalation resolved
- E1 (S1–S8) — **ratified** by the operator at this wrap (AskUserQuestion, 2026-10-04): "Ratify F-W1". The operator's note: the founder answered F-W1 live at `:80`'s P4 (2026-10-04, through the overseer's AskUserQuestion, the pause-swallow residual shown), so this records an existing live ruling and makes no new decision; recorded as the founder's live ruling F-W1, relayed by the overseer. → S1–S8 **apply**.
