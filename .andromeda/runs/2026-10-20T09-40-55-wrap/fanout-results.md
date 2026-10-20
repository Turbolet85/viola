# Fan-out results — 2026-10-20-epoch-5-cleanup

Batch: one snapshot pair. Before: `tree 9bf5e3d1613ac2ff8bbb0c1169b0a001a1b8af9c index
5dc137becbc23e323e9c3b4154d90938b9aa2cb8 head 5a3fb42ab8fb0aac4c43912407755dd7573cee59`. After: `tree
fc129e419fb77759b2aa9391d10eda0957c88fe6`, index and head unchanged. `git diff-tree` printed exactly the seven `A` lines
`fanout-{doc}.md`, no other path. Every receipt's count equals its file's item count. No fix and no twin.

- architecture: 4 proposals · fanout-architecture.md — note: D-arch-decisions holds (the `viola-e2e` → libc edge
  adds no library or version); D-retired: no architecture site for the removed services doc.
  1. apply — check 1 «Accurate this-chunk addition» (routine): §Occupied Resources `target/profraw-census/`, the
     census's `--case` form (`architecture.md:395`).
  2. apply — check 1 routine, dependent of 1: Project directory structure, the `profraw-census.sh` comment names both
     forms (`project-directory-structure.md:98-100`).
  3. apply — check 1 routine: §Occupied Resources `target/e2e-home` keepers, the headroom floor as their fourth check
     (`architecture.md:388`).
  4. apply — check 1 routine, dependent of 3: `/tmp/viola-e2e-home-<uid>/` entry, the keepers' refusal list gains the
     floor (`architecture.md:371`).
- security-plan: 0 proposals · fanout-security-plan.md
  1. raised by main (cascade step 2 cross-master sweep, check 1 routine): §Security Anti-Patterns `security-plan.md:536`
     — the two keepers' guard names the headroom refusal beside its three checks (report: Counts / qualifiers moved,
     F8-headroom, stale-incomplete).
- design-system: 13 proposals · fanout-design-system.md — note: D-design-tokens holds; the «A 0.2.0 frontend design
  input» sections were read and left.
  1. apply — check 1 (D-retired, the item named in Retired; the P5-approved Expected amendment names line 16):
     §Brand Identity ATIS anchor drops `<viola-atis>`.
  2. apply — the same, line 21: Signature element drops `<viola-event-feed>` and the web marker placement.
  3. apply — the same, line 41: the per-surface table drops the `web-spa` 0.3 row.
  4. apply — the same, line 45: the single-source paragraph drops the web page's 0.3 sentence.
  5. apply — check 1 routine (a sweep dependent of the retired web motion row; the plan names the paragraph as the
     sweep's): Supporting attention element, the translate and band are a 0.2.0 design input.
  6. apply — check 1 routine (the same; the plan names the design-direction paragraph): «The page is …» is the
     direction a 0.2.0 frontend inherits.
  7. apply — check 1 routine (Retired names `:136`): Semantic Colors DialogPending::true drops «see web-spa component
     1».
  8. apply — check 1 routine (Retired names `:208`): Problem::unauthorized drops «see web-spa component 6».
  9. apply — check 1 routine (a sweep dependent of the retired `web-spa (viola ui)` row): TapeConnection::closed drops
     the cut verb name `viola ui`.
  10. apply — check 1 routine (Retired names `:247`): Typography Display row drops `<viola-atis>` and the page's `<h1>`.
  11. apply — check 1 routine (Retired names `:335`): §Motion heading drops «`0.3` on web-spa».
  12. **reject** — check 1, no match and the change is structural: `## Surface: web-spa` (`:370`) is the setup grammar's
      surface slot (`## Surface: {type}`, the `{surface}-{topic}` subjects, layout-templates' `## Surface: web-spa` at
      `:15`). It names the deferred 0.2.0 surface and already holds only the deferral stub, so it describes no
      standing component. Renaming it would leave layout-templates' slot alone, and neither the report nor the plan
      names the heading.
  13. apply — check 1 routine (Retired names `:584`): Signature Test points to the one 0.1.0 place, the `[RB]` column
      of `viola send`.
- layout-templates: 0 proposals · fanout-layout-templates.md
- test-plan: 11 proposals · fanout-test-plan.md — note: D-tests-obs-harness holds.
  1. apply — check 1, the P5-approved Expected amendment (test-plan §3 Bootstrap phases, `[profile.mutants]`) names
     the change: four overrides, `verify_window_` 60 s first, the paste-hint class 30 s last, each floor beside
     (`bootstrap-phases…md:11`).
  2. apply — the same, dependent of 1: the fail-fast paragraph's kill list (`:12`).
  3. apply — the same, dependent of 1: the viola-e2e sub-bullet's «45 s kill» → 60 s (`:20`).
  4. apply — the same, dependent of 1: Run E's «the `mutants` profile's 45 s» → its 60 s kill (`:10 @c3888`).
  5. apply — check 1 «Accurate this-chunk addition» (report Dependencies): the Dev-deps bullet in `viola-e2e` lists the
     cfg(unix) libc (`:24`).
  6. apply — check 1, the Expected amendment (§3 Test data bootstrap (Mechanism)): `prepare_home_base`'s headroom floor
     (`test-data-bootstrap.md:13`).
  7. apply — check 1 routine, dependent of 6: `boot` step 2's `ensure_e2e_home` holds the same floor and reads
     `build-failed` (`5-command-implementation.md:6`).
  8. apply — check 1 routine (report F11, stale): the scratch case's sibling is granted `BUILTIN\Users` write
     explicitly (`5-command-implementation.md:35 @c3573`).
  9. apply — check 1 routine (report F7, stale; Spec claims disproved's second entry, disposed here): the fake agent's
     hooks run in their own process group on Unix (`test-plan.md:963`).
  10. apply — check 1, the Expected amendment (test-plan §10 Stack adjustments, item (8)'s finding): the hangup
      mechanism named and the census's `--case` form (`test-plan.md:1092`).
  11. apply — check 1 routine (D-retired, re-derived): §6 E4's heading «SSE `Last-Event-ID` resume» over its deferral
      stub is the same retired web scenario as obs-plan §4's GUI heading (the Retired bullet). Re-derived as a heading
      that names no standing scenario, the E-numbering kept: obs-plan's «Edge flows (tests E1–E3 and E5)» already
      skips E4.
- obs-plan: 1 proposal · fanout-obs-plan.md — note: the three detectors read clean.
  1. apply — check 1 (D-retired, the item named in Retired and an Expected amendment): §4 the GUI view scenario heading
     goes. Re-derived: the heading and its stub become ONE gap marker at their place (classification writing rule 6),
     `N/A — …`.
- a11y-plan: 0 proposals · fanout-a11y-plan.md
  1. Expected amendment «a11y-plan §1 — the `Standard (1)` tier's basis stated on the terminal surfaces alone» —
     disposed, no change: at HEAD `a11y-plan.md:19` already reads «cli and tui get output-discipline and boundary
     assertions plus manual screen-reader passes, with no WCAG conformance claim», and §1's frontend line reads «The
     frontend is deferred to 0.2.0; its technology is not chosen.» §1 is a verbatim upstream copy (playbook
     «Verbatim upstream copy», routine) and the detector found nothing standing on the web surface.

## Validate checks
- Check 1 (playbook): every apply above routine, by «Accurate this-chunk addition», a D-retired item the Retired
  bullet names, or a P5-approved Expected amendment naming the change; one structural no-match rejected. No boundary
  widening among them: the keepers' headroom read is a test-side refusal added beside three standing ones, on the
  backing they already reach.
- Check 2 (cross-contradiction): none.
- Check 3 (intent-consistency): the deviations carry justifications, and the scope record is empty.
- Check 4 (absence needs evidence): the report's «no master lists the services docs» rests on its grep, 0 lines.
- Check 5 (expected amendments): every entry matched or disposed: test-plan §3 ← tp 1–4; §10 ← tp 10; §3 Test data
  bootstrap ← tp 6; design-system Brand Identity ← ds 1–6; a11y-plan §1 ← disposed above; obs-plan §4 ← obs 1;
  architecture Project directory structure ← no master site, the leaf `CLAUDE.md:100` is the cascade's step 3.
- Check 6 (disproved claims):
  - The plan's step 7 premise (no master site) is disposed. It is routed by the recorded direction inputs#I4 §3 (the
    Viola overseer, the founder's delegate: PROVISIONAL) to the next epoch boundary's Windows `mutants (viola-pty)`
    reading, `unviable` ≤ `caught`, as the P5 card's row 2 already carries (owed to that audit, no route entry). The
    unmet acceptance criterion is resolved by that direction, with no halt.
  - test-plan `:963` is disposed by tp 9.
- Escalations: 0. Rejected for a source the report does not carry: 0.
