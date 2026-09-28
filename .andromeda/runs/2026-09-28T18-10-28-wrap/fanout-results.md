# Fan-out results — 2026-09-28-capability-ledger-and-viola-verify (P2, resumed run)

Seven Explore doc-agents, one parallel batch, prompt from `amendment-flow.md` §Fan-out sent verbatim. Returns carried no
HTML entities (probe below). Parsed lists keep `detector · severity · section · change · basis · dependent-of`; each
return's `sidecar` and `rationale` lines are paraphrased into the disposition, and the proposals are numbered here.

## Verdicts
- architecture — 23 proposals (D-arch-resources 14 · D-arch-decisions 9)
- security-plan — 8 proposals (D-security-input 8, all severity escalate)
- design-system — 0 (`proposals: []`; commentary stripped → `.raw-fanout-design-system.md`)
- layout-templates — 2 proposals (D-layout-surface 2)
- test-plan — 9 proposals (D-tests-framework 4 · D-tests-coverage 3 · D-tests-obs-harness 2)
- obs-plan — 8 proposals (D-obs-instrumentation 7 · D-obs-stack 1)
- a11y-plan — 0 (`proposals: []`; commentary stripped → `.raw-fanout-a11y-plan.md`)

## architecture (23)
- A1 · D-arch-resources · warning · Occupied Resources → Filesystem · register `ledger/probes/<pid>/{plugin/,captures/}` (0700, verify only, drop-guard removal) and `captures/<PascalEvent>.<k>.json` (0600, transient, content-bearing, k across events) · arch:385 — **apply** (playbook: accurate this-chunk addition; a filesystem resource the registry enumerates; plan expected #3)
- A2 · D-arch-resources · warning · Filesystem → `ledger/stamps.json` entry · modes, sole writer `update_stamps` (verify; lock, MAX_FRAME, replace_private), lock-free `read_stamps` readers · arch:385 — **apply, re-derived**: the readers today are `run`'s version gate only (the proposal's "and `ui`" is not in the report)
- A3 · D-arch-resources · warning · Standard Contracts · add the Ledger stamps envelope contract · arch:241-245 — **apply** (a new on-disk contract shape; report Symbols → Stamp envelope)
- A4 · D-arch-resources · warning · Occupied Resources → Binary · `verify [--record <DIR>] [-- <program> [args…]]` + hidden `hook <event> --capture <DIR>` · arch:344-345 — **apply** the verify usage (plan expected #3; the Binary registry enumerates subcommand arguments, so registry over-reach does not govern); the `--capture` half **escalate** with E1 (boundary widening)
- A5 · D-arch-resources · warning · Binary → Test-only binaries · fake agent `-p/--print` · arch:343 — **apply** (plan expected #3)
- A6 · D-arch-resources · warning · Claude Code integration names · probe plugin `viola-verify-probe` + verify's child flags · arch:349-351 — **apply** (a second plugin name, a resource that registry enumerates)
- A7 · D-arch-resources · warning · [Plugin Scope] · verify's probe is the one unwrapped session carrying viola capture hooks · arch:100 · dependent-of D-arch-resources — **escalate** with E1 (it states the capture arm)
- A8 · D-arch-resources · warning · [Hook Contract] · the `--capture` arm is the exception to the VIOLA_NAME-absent immediate exit · arch:69 · dependent-of D-arch-resources — **escalate** with E1
- A9 · D-arch-resources · warning · Filesystem → diagnostics/ · `cli-<name>.ndjson` now has a producer (verify, with a valid VIOLA_NAME) · arch:382 — **apply**
- A10 · D-arch-resources · warning · Occupied Resources → Repository · `schemas/claude-fixture.v1.json` + the recorded fixture file shape · arch:392-395 — **apply**
- A11 · D-arch-resources · warning · Project directory structure · schemas/ comment adds `claude-fixture.v1.json` · arch:486 · dependent-of — **apply**
- A12 · D-arch-resources · warning · Conventions → CLI exit codes · exit 1 gains verify's failing row, its refusals, the cli internal-error line · arch:139 — **apply** (plan expected #4)
- A13 · D-arch-resources · warning · Build system (Lint) · `src/human.rs` writers + callers now incl. verify · arch:426 · dependent-of — **apply**
- A14 · D-arch-resources · warning · Project directory structure · human.rs comment names verify · arch:477 · dependent-of — **apply**
- A15 · D-arch-decisions · warning · [Session Liveness] · the landed start order with the version gate between pin+plugin and the bind · arch:93 — **apply** (plan expected #2)
- A16 · D-arch-decisions · warning · [CLI Version Compatibility] · the six landed rows, the print-mode probe + capture arm, `--record`, all-pass verified · arch:73-91 — **apply** (plan expected #1; the typed-input rows' owners folded in by the orchestrator, check 5); its capture-arm clause lands with E1
- A17 · D-arch-decisions · warning · CI/CD approach · "verify runs only locally" narrowed to the real CLI; verify runs in CI against the fake agent · arch:577 — **apply** (plan expected #1)
- A18 · D-arch-decisions · warning · [Agent Coverage] · the version gate split: Claude parsing in viola-agent-claude, spawn in root `run::version_gate`, I/O in viola-state · arch:45 — **apply**
- A19 · D-arch-decisions · warning · Crate dependency direction · viola-agent-claude as-landed gains `ledger`; still no viola-state dependency · arch:451 · dependent-of — **apply**
- A20 · D-arch-decisions · escalate · [Error Handling] · `StampError` beside `AgentError` vs the one-enum-per-crate decision · arch:97 — **escalate** E3
- A21 · D-arch-decisions · escalate · Conventions → Rust error types · follows E3 · arch:155 · dependent-of — **escalate** E3
- A22 · D-arch-decisions · escalate · Inherited Defaults → Errors · follows E3 · arch:631 · dependent-of — **escalate** E3
- A23 · D-arch-decisions · warning · Crate dependency direction · root bin gains libc (unix) + two windows-sys features for raw panic frames · arch:456 — **apply** (report Dependencies; both already in §Stack)

## security-plan (8)
- S1 · D-security-input · escalate · Input Validation → Own state files on read · second interim exception: run's gate reads `ledger/stamps.json` without strict-modes until :95 · sp:234 — **escalate-resolved**: the founder ratified this widening live at 2026-09-28 12:39:40 (AskUserQuestion, relay the Viola overseer; plan Implementation notes), after it was shown → apply, sidecar records it as the founder's. The detector's note (verify's `update_stamps` read) → **E2**
- S2 · D-security-input · escalate · Auth → `~/.viola/` access control · run's stamps read as a ratified interim gap · sp:207 · dependent-of — **escalate-resolved** with S1 (plan expected #8)
- S3 · D-security-input · escalate · Input Validation → CLI arguments / stdin (Home path) · same gap · sp:229 · dependent-of — **escalate-resolved** with S1
- S4 · D-security-input · escalate · Security Decisions Log · the `2026-09-28` ratification entry · sp:720-728 — **escalate-resolved** with S1 (plan expected #5)
- S5 · D-security-input · escalate · Input Validation → Hook stdin · the hidden `--capture <DIR>` arm · sp:224 — **escalate** E1
- S6 · D-security-input · escalate · Input Validation → CLI arguments (hook sentence) · the capture arm reads no VIOLA_* · sp:229 · dependent-of — **escalate** E1
- S7 · D-security-input · escalate · Error Handling → Internal logging (hook bullet) · the capture arm skips the VIOLA_* checks and the parse · sp:480 · dependent-of — **escalate** E1
- S8 · D-security-input · escalate · Input Validation → Constants · MAX_FRAME consumers + the stamps read, the `--version` reads, the capture stdin · sp:241 — **apply** (plan expected #7; the new readers ARE capped, so the invariant holds and only the list is stale → accurate this-chunk addition); its capture-stdin clause lands with E1
- (raised, check 5) security-plan §Data Protection — probe captures as transient 0600 content-bearing files under the 0700 probe dir, removed at verify's end (plan expected #6; no detector proposed it) — **escalate** with E1

## layout-templates (2)
- L1 · D-layout-surface · warning · Output structure — `viola verify` · the six ledger rows `[NN/06]`, `stamped <ver>  <n> pass  <m> fail`, the synopsis, exit 0/1 · lt:455-466 — **apply** (accurate this-chunk addition), re-derived: the row count grows as owning chunks land their rows
- L2 · D-layout-surface · warning · Component — refusal lines · verify's exit-1 `unable:`/`hint:` pairs join run's as the fixed-message exceptions · lt:532-533 — **apply**

## test-plan (9)
- T1 · D-tests-framework · warning · §5 CLI · verify's human lines pinned by literal asserts in `tests/cli_verify.rs`; trycmd later · tp:957 — **apply** (plan expected #10)
- T2 · D-tests-framework · warning · §7 Fixture hygiene · a run-time directory walk, not rstest `#[files]` (0.27 refuses an empty glob) · tp:1383 — **apply** (disproved claim 2)
- T3 · D-tests-framework · warning · §7 Fake agent · the `-p/--print` mode · tp:1379 — **apply** (plan expected #11)
- T4 · D-tests-framework · warning · §7 Fake agent → `start` receipt · raw-mode guard only in interactive mode · tp:1368 · dependent-of — **apply**
- T5 · D-tests-coverage · warning · §7 Fixture hygiene · the `fixtures/claude` walk + `schemas/claude-fixture.v1.json` joined with 2.1.283 · tp:1387 — **apply**
- T6 · D-tests-coverage · warning · §5 CLI (`contract_ledger_probes.rs`) · record as not landed · tp:967 — **reject** (playbook: sequencing deferral — the tail entry "Verify-stamped test homes and harness" P5 inserts names `contract_ledger_probes.rs` over committed sets; the spec is right)
- T7 · D-tests-coverage · warning · §6 Contract suite · insta hook-sequence pin for 2.1.283 not built · tp:1335 — **reject** (sequencing deferral — owned by the markerless "Fake-agent drift contract" entry, working-route :55, whose scope is exactly that pin)
- T8 · D-tests-obs-harness · warning · §3 `boot` readiness · the line-3 check no longer joins with this chunk; its owner is the tail entry · tp:526 — **apply**, re-derived to name the tail entry; the obs-plan §4 twin (~:870) is raised with it (cross-master bind, check 2)
- T9 · D-tests-obs-harness · warning · §1 Test Scope Summary (readiness) · the same pointer · tp:174 · dependent-of — **apply** (test-plan §1 carries no verbatim-copy label; no playbook rule reserves it)
- (raised, check 5) test-plan `/runs only locally/` sites (2) — A17's narrowing cited in test-plan — **apply** via the cascade sweep

## obs-plan (8)
- O1 · D-obs-instrumentation · warning · §4 Edge flows → `verify` · step lines + `stamped` summary on stdout; refusals the stderr pair · op:979 — **apply** (disproved claim 3; plan expected #9)
- O2 · D-obs-instrumentation · warning · §4 Edge flows · the hidden `hook --capture` arm as a deliberately uninstrumented path · op:967-980 — **escalate** with E1
- O3 · D-obs-instrumentation · warning · §4 instrumentation table, `cli (viola hook)` row · except the capture arm · op:831 · dependent-of — **escalate** with E1
- O4 · D-obs-instrumentation · warning · §10 Zero unlogged panics → bounded exemptions · exemption 5, the capture arm · op:1304-1308 · dependent-of — **escalate** with E1
- O5 · D-obs-instrumentation · warning · §2 Telemetry Strategy · the restated exemption list gains the capture arm · op:512 · dependent-of — **escalate** with E1
- O6 · D-obs-instrumentation · warning · §7 Panic hooks → Per-role behaviour · cli writes the `internal-error` process-exit line like run, then the fixed stderr line · op:1147 — **apply**
- O7 · D-obs-instrumentation · warning · §4 Edge flows → `verify` · how verify's `--version` read and print-mode probe child are logged · op:977-979,1109 — **escalate** E4: the orchestrator read `src/cmd/verify.rs:112,127` — both spawns go through `run_bounded`, which logs nothing; only `run`'s gate writes the `version-probe` pair (`src/run/version_gate.rs:115-139`). An uninstrumented child spawn against §6 with no owner on the route
- O8 · D-obs-stack · warning · §7 Panic hooks · raw never-symbolised frames, reason 351 of 403 ms · op:1140 — **apply** (disproved claim 1; overseer-directed)

## design-system (raised, check 5)
- D1 · design-system §Surface: cli → Exit-code phraseology — the exit-1 rows for the unreadable CLI version, CLI not found, a dirty recording (plan expected #13; the tokens detector does not reach it) — **apply**

## Validate — the six checks
1. Playbook — above, per proposal.
2. Cross-contradiction — none: no two proposals edit one section in opposite directions. The T8 ↔ obs-plan §4 readiness pointer is one bound pair, applied together.
3. Intent-consistency — the scope record's seven lines hold (`serves` true; the two in-intent lines carry the overseer's word). The :53 CARRY "make `stamped_home` stamp through `viola verify` here" and CARRY 2 (harness `boot` line 3, `supervise --fixtures`) moved to the tail entry by the plan (P4 review) — justified, carried by P5's tail entry verbatim.
4. Absence needs evidence — O1 ("stderr summary: 1 hit") and O8 ("force_capture: 1 hit") are re-run by the cascade sweep before any sidecar entry.
5. Expected amendments — 13 entries + the overseer's: all matched or raised above; test-plan §3 `boot` step 4 is the tail's (not carried, pinned by P5); the matrix line is P7.3's (claimed 0).
6. Disproved claims — 1 → O8 · 2 → T2 · 3 → O1: all disposed.

## Escalations (resolved WITH the operator before apply)
- E1 — the hidden `hook <event> --capture <DIR>` arm (A4 half, A7, A8, A16 clause, S5–S8 clause, the Data Protection raise, O2–O5): boundary widening.
- E2 — `viola verify`'s `update_stamps` read-modify-write of `ledger/stamps.json` without strict-modes (S1's note).
- E3 — `StampError` beside `AgentError` (A20–A22).
- E4 — verify's two child spawns unlogged (O7).

### Resolutions (AskUserQuestion, this wrap)
- E1 → **ratified** — FOUNDER RATIFIED LIVE at 2026-09-28 20:24:32 in the Viola overseer session (AskUserQuestion), after
  being shown the capture arm as described in the dialog; relay: the Viola overseer. A4 half, A7, A8, A16 clause, S5–S7,
  S8 clause, the Data Protection raise and O2–O5 → apply; the sidecars record it as the founder's.
- E2 → **ratified under :95** — FOUNDER RATIFIED LIVE at 2026-09-28 20:24:32, same session: the 12:39:40 interim exception
  extends to verify's `update_stamps` read; :95 names both readers. Relay: the Viola overseer. S1–S4 apply naming both
  readers; P5 extends the :95 CARRY.
- E3 → **fold later via CARRY** (overseer: "agreed; the locked one-enum rule stands"). A20–A22 apply as a named interim
  divergence in [Error Handling] only (Conventions / Inherited Defaults keep the rule, citing it); P5 pins the fold on the
  "Verify-stamped test homes and harness" entry.
- E4 → **CARRY to the tail** (overseer: "agreed; the spec stays right"). O7 → no body amendment; P5 pins it on the
  "Verify-stamped test homes and harness" entry.
- Playbook: no new rule proposed — E1/E2 are the never-routine widening class (resolved by ratification); E3/E4 are
  one-off code-vs-locked-spec gaps whose owners ride the route.

## Entity probe
`entities=0` over `fanout-results.md` and both raw twins (`&(lt|gt|amp|quot|#N);`); no return arrived entity-escaped.
