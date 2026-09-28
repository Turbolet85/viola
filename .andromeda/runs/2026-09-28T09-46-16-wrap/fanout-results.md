# Fan-out results — wrap 2026-09-28T09-46-16 · chunk 2026-09-28-cli-output-tokens

Seven Explore doc-agents, one parallel batch, the amendment-flow prompt verbatim. No return carried HTML entities
(`entities=0` by read: no `&lt;` / `&gt;` / `&amp;` in any return); no raw twin warranted (every `proposals: []` return
arrived as bare YAML plus comment lines, and stripping removed only the `#` commentary summarised on each verdict line).

## Verdicts
- **architecture** — 4 proposals (D-arch-decisions ×4; D-arch-resources no drift: no new resource, `human` is a module).
- **security-plan** — `proposals: []` (input/auth/deps: no new external input, no secret, no dependency added; notes
  :427 and :598 accurate after the chunk).
- **design-system** — `proposals: []` (D-design-tokens: both surfaces `tokens n/a`; notes the Toolkit expected
  amendment is outside its detector).
- **layout-templates** — `proposals: []` (no new surface; `--help` at :468-481 and the run refusal at :483-492 already
  documented).
- **test-plan** — 1 proposal (D-tests-coverage; framework and harness no drift).
- **obs-plan** — 2 proposals (D-obs-stack ×2; instrumentation and PII no drift).
- **a11y-plan** — `proposals: []` (no interactive element; schema unchanged).

## Proposals and dispositions
### architecture
1. D-arch-decisions · Infrastructure Patterns → Build system (lint bullet, architecture.md:426) — the root bin's only
   human-stderr writer is `src/human.rs` (`write_refusal` one `write_all`, `refuse` locked stderr, result dropped),
   called only by `run`'s five start refusals (squatted added). **apply** — check 1 playbook "Accurate this-chunk
   addition" (report Symbols bullet); body re-derived from the report.
2. D-arch-decisions (dependent-of) · Project directory structure (:475-480) — `src/human.rs` in the `src/` tree.
   **apply** — check 1 same rule; expected amendment 3.
3. D-arch-decisions · Stack and Technologies CLI parser row (:16) — clap 4.6.7 `default-features = false`, its default
   set minus `color`. **apply** — check 1 same rule; expected amendment 1.
4. D-arch-decisions (dependent-of) · Cross-cutting Patterns → Config management (:586) — the exhaustive env sentence
   holds because clap is built without `color`; the anstream → anstyle-query route named closed. **apply** — check 1
   same rule; expected amendment 2; disposes report Spec claims disproved 1.
### test-plan
5. D-tests-coverage · §5 Integration Test Strategy → CLI (after :959) — `tests/cli_output_plain.rs` listed. **apply** —
   check 1 same rule; expected amendment 5.
### obs-plan
6. D-obs-stack · §3 harness intro (obs-plan.md:556) — run's pre-spawn stderr exception names all five start refusals
   through `human::refuse`. **apply** — check 1 same rule (the report names this writer and its five callers; the site
   described the re-sited writer); check 4: :556 read whole (367 chars).
7. D-obs-stack (dependent-of) · §11 Obs Anti-Patterns → Logs (:1392) — run's refusals carry no print macro and no
   `#[allow]`. **apply** — check 1 same rule; check 4: :1392 read whole (789 chars).
### orchestrator-raised (check 5)
8. design-system §Surface: cli → Toolkit (:686) — clap 4.6.7 (derive) without its `color` feature: every styled byte of
   human output is viola's own SGR module. **apply** — routine, report Dependencies substantiates (expected amendment 4).

## Validate checks
- 1 playbook: all 8 routine (Accurate this-chunk addition); no Boundary-widening match — the chunk narrows the env
  surface (six reads removed), it widens nothing.
- 2 cross-contradiction: none (arch :426 and obs :556/:1392 state the same writer).
- 3 intent-consistency: the report's deviations are justified; scope record none; **one UNMET acceptance clause** —
  plan (tests) "scoped mutants over `src/cmd/run.rs` … 0 missed", red on the Windows host by construction →
  **ESCALATION E1** (unlinked to the matrix).
- 4 absence-needs-evidence: the arch agent's sweep (`writeln`, `colou?r`, `anstyle`, `CLICOLOR`, `NO_COLOR`,
  `human stderr`, `src/human`: one hit, :426) and obs's reads cite their lines; the cascade sweep below re-checks all
  seven masters.
- 5 expected amendments: 1 ← #3 · 2 ← #4 · 3 ← #1/#2 · 4 ← #8 (raised) · 5 ← #5. All five covered.
- 6 disproved claims: 1 (arch:586 env sentence) ← #4; 2 (plan entry 17) → E1.

## Escalations
- **E1 — resolved.** Operator (overseer) answer: "Run it, accept by the word" — note: "overseer: yes. Running beats
  skipping: the red is recorded, and the union verdict is cited. Mint the working-route pin so the next plan authors
  host-killable scoped entries." Treatment: the P7 light gate runs the block unskipped; entry
  `run --mutants --file src/cmd/run.rs` red is recorded and accepted on this word, the mutation verdict the union
  (pre-push ×2, CI ci#36404931982 `mutants-verdict` breaches []); P5 pins the authoring rule on the working route.
