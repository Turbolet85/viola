# Fan-out results — 2026-10-05-dialog-rows-and-re-probe (wrap 2026-10-05T14-27-34)

Seven Explore doc-agents, one batch, prompts substituted verbatim from `amendment-flow.md` (detector counts arch 2 ·
security 3 · design 1 · layout 1 · tests 3 · obs 3 · a11y 2 = 15 = the drift-base's `doc:` names). Keyed-contract
renders for architecture · test-plan · obs-plan · a11y-plan in this run dir. The returns carried no HTML entities
(entities=0 by read); each parsed as YAML. Fields kept below: detector · section · change · basis · dependent-of;
the `sidecar:` / `rationale:` lines are not kept (every applied text is re-derived from the report, Apply step 1).

## Verdicts
- architecture — 17 proposals (46 in all).
- security-plan — 7 proposals (all `escalate`, the detector's severity).
- design-system — `proposals: []` (no stripping).
- layout-templates — 3 proposals; stripped trailing notes: no other new surface (`hook --answers` hidden, no human
  surface; Runs C/D internal), web-spa untouched, the `--help` "never answers" sentence unchanged.
- test-plan — 14 proposals; stripped comment: D-tests-obs-harness no drift (§3 ↔ obs-plan §3 still agree; obs §4
  :735/:737 is obs-plan's).
- obs-plan — 5 proposals; stripped comments: D-obs-stack no drift (no dependency); D-obs-pii no drift (the `--answers`
  arm logs nothing; fixtures scrubbed); no obs keyed contract carries a verify/capture/spawn-count claim.
- a11y-plan — `proposals: []`; stripped comments: no interactive UI added; the `[NN/MM]` step lines are covered
  generically (a11y-plan §1 path 6, §4 P6); no schema change.

## Parsed lists + dispositions

### architecture
- A1 D-arch-resources · §Occupied Resources → Binary (hidden flag) · register hidden `--answers <DIR>` beside
  `hook <event> --capture <DIR>`; exclusive claim; dialog answer body on stdout; retires "always exit 0 with empty
  stdout" · basis architecture.md:357 — **escalate E1** (check 1: boundary widening — the capture arm gains a stdout
  write).
- A2 D-arch-decisions · [Hook Contract] · capture-arm exception: exclusive claim + `--answers` probe body (founder P4)
  · basis :69 — **escalate E1** (same widening).
- A3 D-arch-resources · §Occupied Resources → Claude Code integration names · dialog-kind probe plugin (PreToolUse
  matcher, PermissionRequest, PostToolUse, `--answers`); four interactive children; Run C / Run D flags · basis :365 —
  **escalate E1** (the Run C/D crossing is the same ruling's).
- A4 D-arch-resources · §Occupied Resources → Filesystem (probe dirs) · four 0700 probe dirs incl. `-dialogs/`,
  `-plan/` + `plans/` · basis :404 — **escalate E1** (Run D `plansDirectory`, Run C `touch`: the STOP 7 / P4 rulings).
- A5 D-arch-resources · §Occupied Resources → Filesystem (`ledger/probes/<pid>/`) · `questions/` and `plan/` roots
  with `answers/` · rationale cites `src/cmd/verify.rs:238-291` — **reject** (re-derivation tell: a source location the
  report does not carry); the fact is in the report (Symbols: `ProbeDir` builds `questions/` and `plan/` roots) →
  **raised by the orchestrator as R-A5, routine** (playbook "Accurate this-chunk addition").
- A6 D-arch-resources · §Occupied Resources → Filesystem (diagnostics spawn pairs) · six spawn pairs · basis :399 —
  apply (playbook: Accurate this-chunk addition).
- A7 D-arch-resources · §Standard Contracts → Ledger stamps envelope · `measured.dialog_probe.
  parallel_both_before_first_post` (additive, no `v` bump, `run` ignores) · basis :253 — apply (Accurate addition).
- A8 D-arch-resources (dependent-of D-arch-resources) · Filesystem `ledger/stamps.json` · the dialog_probe measurement
  listed · basis :402 — apply.
- A9 D-arch-resources · §Occupied Resources → Repository · dialog-variant class `<Event>.<stem>-<n>.json`, 14 rows,
  RELAYED.md superseded · basis :412 — apply.
- A10 D-arch-resources (dependent) · §Infrastructure Patterns → Project directory structure · fixtures comment +
  `typed.rs` four runs · basis project-directory-structure.md:25,83 — apply.
- A11 D-arch-resources · §Occupied Resources → Binary (`viola-fake-agent`) · argv options three → five (`--dialogs`,
  `--stop-receipt-hold-ms`) · basis :355 — apply (the registry already enumerates these options, so not over-reach).
- A12 D-arch-decisions · [CLI Version Compatibility] · 14 rows; four PTY runs; hook answers never a key; R2 dated gap
  removed; CI `--dialogs` · basis :91 — apply (expected amendment; the "hook answers" clause rides E1's ruling).
- A13 D-arch-decisions (dependent) · §Cross-cutting → Capability ledger as the single gate · dated exception removed ·
  basis :459 — apply.
- A14 D-arch-decisions (dependent) · [Plugin Scope] · five transient children; C/D transcripts +2 · basis :100 —
  apply (expected amendment).
- A15 D-arch-decisions (dependent) · Stack (Screen model row) · four interactive probe runs · basis :18 — apply.
- A16 D-arch-decisions (dependent) · §Infrastructure Patterns → Crate dependency direction · four runs · basis
  crate-dependency-direction.md:6 — apply.
- A17 D-arch-decisions (dependent) · §Infrastructure Patterns → CI/CD approach · four typed runs, `--dialogs`, 14 rows
  · basis ci-cd-approach.md:20 — apply.

(The return carried 17 list items, A1–A17; the verdict line's 16 was a miscount, corrected here.)

### security-plan (all detector severity `escalate`)
- S1 · §Input Validation → Hook stdin (capture arm) · exclusive claim, `--answers` take(64) closed `ProbeAnswer`,
  `probe_body` → `decision_body`, one stdout write · basis :226 — **escalate E1**.
- S2 (dependent) · §Input Validation → CLI arguments · `--answers` joins the arm's argument checks · basis :231 —
  **E1**.
- S3 (dependent) · §Error Handling → Internal logging (`hook` bullet) · arm prints a body for a mapped answer · basis
  :503 — **E1**.
- S4 · §Input Validation → Constants (`MAX_FRAME`) · four interactive PTY runs; the answer file's `take(64)` · basis
  :244 — **E1**.
- S5 (dependent) · Threat Model Summary → child spawning · four interactive children, Run C/D · basis :126 — **E1**
  (verbatim copy kept current — playbook rule of 2026-10-04 — judged like any body amendment).
- S6 (dependent) · §Data Protection → At rest → probe dirs · four dirs, Run C `touch`, Run D `plans/` · basis :275 —
  **E1**.
- S7 (dependent) · §Data Protection → Accepted risk · transcripts residual covers B, C, D (+2 per verify); plans out of
  `~/.claude/plans` · basis :283 — **E1**.

### layout-templates
- L1 D-layout-surface · §Output structure — `viola verify` (prose) · four row ids after `confirm-window`; `MM` 14 ·
  basis layout-templates.md:484 + `ledger.rs:68-71,90-93` — **reject** (re-derivation tell: the basis cites source
  lines the report does not carry); its group falls with it:
- L2 (dependent) wireframe counters `/14`, last row `[14/14] dialog-concurrency …`, `14 pass` — rejected with L1.
- L3 (dependent) failing example `12 pass  2 fail` — rejected with L1.
  → **raised by the orchestrator as R-L, routine** (check 5: the plan's layout-templates entry; the report carries the
  row ids and words, Symbols `LedgerRow`).

### test-plan
- T1 D-tests-coverage · §5 `contract_ledger_probes` · fourteen ids, `14 pass  0 fail` under `--dialogs` · basis :663 —
  apply.
- T2 (dependent) · §5 `cli_verify` · `[NN/14]`, four runs, the no-replay four-row fail case; "(22 cases)" re-read ·
  basis :653 — apply (the case count: the report does not carry it → the number is dropped, not re-derived).
- T3 (dependent) · §3 → 5-command implementation (boot step 4) · `--dialogs`, four runs, fourteen rows · basis
  5-command-implementation.md:8 — apply.
- T4 (dependent) · §3 → 5-command implementation (`--local-live`) · `LEDGER_ROWS: [&str; 14]` · basis :53 — apply.
- T5 (dependent) · §7 Fake agent → Modes · `--dialogs`, `--stop-receipt-hold-ms`, four runs · basis :1076 — apply.
- T6 · §2 Test pyramid → Contract · dialog tier recorded by verify Run C/D, relayed set superseded · basis :438 —
  apply.
- T7 (dependent) · §1 contract trigger · dialog captures recorded by live verify · basis :408 — apply.
- T8 (dependent) · §7 Fixture library + Recorded hook payloads · dialog payloads recorded by Run C/D · basis
  :1041,:1051 — apply.
- T9 (dependent) · §7 Fixture hygiene · the walk covers the variant class · basis :1084 — apply.
- T10 (dependent) · §6 Path 4 · permission e2e re-owed to "Permission end to end"; recorded permission fixture;
  `v1-15` case · basis :779,:781 — apply (the owner is this wrap's P5 mint, the founder's P4 split ruling).
- T11 · §4 Dialog mapping S3/S7/S8 · S7 approve = `allow` + `updatedInput` echo · basis :558 — apply (disproved
  claim (1)).
- T12 (dependent) · §6 Path 4 verification signal · the plan-approve snapshot · basis :796 — apply.
- T13 D-tests-framework · §3 → Bootstrap phases · nextest `ci` overrides 45 s / 20 s; no test-side bound on verify;
  `WITHIN` 7 s elsewhere · basis bootstrap-phases-…md:10 — apply (the overseer's founder-delegated decision; not a
  boundary).
- T14 (dependent) · §3 → Bootstrap phases (mutants) · rationale "four times"; kills unchanged · basis :11 — apply.

### obs-plan
- O1 D-obs-instrumentation · §4 Edge flows → `verify` · six spawns, four `verify-pty-probe` · basis :735 — apply.
- O2 (dependent) · §4 CI clause · four runs incl. `--dialogs` · basis :737 — apply (the "at the recorded version
  2.1.287" phrase is checked against the body at apply, not taken from the proposal).
- O3 (dependent) · §6 Child / shell spawns · four runs, six spawns · basis :866 — apply.
- O4 (dependent) · §6 diag-line subject note · six pairs · basis :867 — apply.
- O5 D-obs-instrumentation · §4 capture arm · `--answers` prints a product-built body; still uninstrumented;
  exclusive claim · basis :736 — **escalate E1** (the widening's obs restatement); its absence claim (§4 :583, §2
  :512, §10 ex. 5 :1066 need no change) is verified by the cascade sweep (check 4).

## Orchestrator raises (check 5 — expected amendments the detectors did not propose)
- R-SEC: security-plan §Security Anti-Patterns → Universal — R2's dated gap retired (security-plan.md:608, the ten
  ledger rows / `/10` / ten-row stamp) — routine: the report substantiates it (Outcome: R2 closes, MET).
- R-DS: design-system §Surface: cli Component Patterns 5 — `/14`, the four row ids and words, `14 pass` — routine
  (report Symbols `LedgerRow`; Counts).
- R-L: layout-templates §Output structure — `viola verify` — as L1–L3, re-derived from the report — routine.
- R-A5: architecture Filesystem `ledger/probes/<pid>/` — routine.
- Expected entries all covered: arch (A12/A13 · A2 · A14 · A4/A6 · A9) · security (R-SEC · S1 · S6/S7) · tests
  (T1–T9) · obs (O1–O5; §10 ex. 5 checked in the sweep) · design (R-DS) · layout (R-L; the `--help` sentence: no
  change) · CLAUDE.md:43 → cascade.

## Check results
1. Playbook — routine: "Accurate this-chunk addition" for every applied item; E1 = "Boundary widening" (never-routine).
2. Cross-contradiction — none (A1/A2/S1/O5 state one fact; A12/A13 agree).
3. Intent-consistency — scope record: in-intent `tests/hook_fail_open.rs`, companion `.config/nextest.toml`, both
   within intent; deviations carry the founder's / the overseer's recorded words.
4. Absence-needs-evidence — O5's "no restatement elsewhere" and every "all sites" claim → the cascade sweep.
5. Expected amendments — four raises above.
6. Disproved claims — (1) T11/T12; (2) A1/A2/S1 (E1); (3) a hypothesis, never a spec claim — no channel owns it,
   recorded in the report only; (4) no master claims overlap; the row passes as defined — no amendment.

## Escalations
- E1 — the capture arm's `--answers` stdout body + Run C's executed `allow` (`touch` in its own dir) + Run D's
  `plansDirectory` + the C/D transcripts residual: A1–A4, S1–S7, O5 (+ the "hook answers" clause of A12).
  **Resolved — ratify, apply.** The operator's word at this wrap (AskUserQuestion, 2026-10-05): all four are the
  founder's live rulings, each given after the widening was shown — M7 = A at P4 (its option text named the `touch` and
  the +2 transcripts) and the STOP 7 Run D re-run with `plansDirectory` — relayed by the overseer; recorded as the
  founder's, no new decision made here. All twelve → apply.
