# Cascade dispositions — 2026-10-07-test-homes-off-the-contended-volume

Written from the listing `cascade.py sweep` printed after every body of this pass was applied (trail
`cascade-2026-10-07-test-homes-off-the-contended-volume.json`; baseline `b329083e`, the pre-CI commit's parent). Seven
patterns, each with a control that fired on the pre-pass masters (`cascade-patterns.toml`). Two more were swept by hand
because no pre-pass master or registry line holds them, so the tool would refuse them for want of a control:
`create_dir_all` and `cargo clean`. Line numbers are post-pass: this pass inserted one line in `architecture.md` (after
`:408`), one in `security-plan.md` (after `:599`) and one in `test-data-bootstrap.md` (after `:12`).

## What was searched
- `g2-find` (`find target/e2e-home -path`, the retired start point) · `e2e-home` (the path token: every statement
  of where homes live) · `remove-owned` (`remove_owned`: the removals that now reach through the link) ·
  `keep-failed` (`AGENT_RUN_KEEP_FAILED`: a kept home's lifetime) · `home-lifetime` (`lifetime of the session home`:
  the retention claim in its own words) · `tempdir-in` (`tempdir_in`: the base the homes are made in) · `outside-ws`
  (`outside the (workspace|working directory|repository)`: the boundary the widening crosses).
- Not searched: a claim that homes are physically on the repository's volume worded with none of these tokens (the
  detectors read architecture, test-plan and obs-plan for it by meaning and named none); line-number citations of the
  three files that gained a line were searched separately (`git grep -E '(architecture|security-plan)\.md:[0-9]+'` and
  the three key-file names, outside `runs/`, `chunks/`, the sidecars and the friction log): 0 citations at or past an
  insertion point.

## Per pattern
### g2-find — 0 rows (control fired at pre-pass `obs-plan.md:1033`)
The retired start point stands nowhere after the pass: masters, registries, curation homes, bases and leaves.

### e2e-home — new 5 · standing 32 · leaf 7 · curation 0 · base 0
- **new** (this pass's own text, 5): `architecture.md:409` (A2) · `security-plan.md:600` (S1) · `obs-plan.md:1032`
  (O1's comment) · `log-file-location.md:8` (O3) · `test-data-bootstrap.md:13` (T5).
- **standing edited** (amended lines, the swept token still on them by design, since the path string is unchanged):
  `architecture.md:424` (A1, read by offset: the six pre-pass matches name the three home classes, the new ones the
  link) · `test-plan.md:626` (T7) · `obs-plan.md:1033`, `:1034` (O1, O2: now `target/e2e-home/`) ·
  `5-command-implementation.md:6` (T1), `:53` (T2), `:90` (T3), `:102` (T4) · `test-data-bootstrap.md:17` (T6).
- **standing, no change** (each read whole or by a bounded window; each states the path string or a CI fact, and the
  path string is unchanged while no CI runner has a link):
  - `architecture.md:384` (the keep variable, "keeps `target/e2e-home/` homes for the CI gates") · `:408` (the chaos
    home sits outside `target/e2e-home`);
  - `test-plan.md:970`, `:971` (the canary and schema scans over `target/e2e-home/**`) · `:990` (a Windows DACL test's
    home where the CI homes live) · `:1096` (CI: homes under the workspace's `target/e2e-home/`, CI-scoped) · `:1165`,
    `:1173` (upload globs) · `:1218` (the perf session's home, by path);
  - `obs-plan.md:915` (the chaos home outside `target/e2e-home`) · `:964`, `:999`, `:1046` (upload and scan globs) ·
    `:1011` ("homes must live under `target/e2e-home/`": holds by path, and the scans reach them through the link, as
    measured at this chunk's smoke) · `:1044` (CI step order) · `:1084` (the perf hook's home, by path);
  - `ci-cd-approach.md:5` (the CI `diag-perf` glob, window at c1999) · `project-directory-structure.md:69` (G2 "under
    target/e2e-home", a tree comment) · `5-command-implementation.md:33` (window at c564: "homes under
    `target/e2e-home/viola-test-*`", by path) · `:116`, `:118`, `:121` (the schema-check and secret-scan walks; both use
    `read_dir`, which follows a link at the root, and neither changed) ·
    `bootstrap-phases-derive-for-route-setup-project.md:50` (G2 "counts … under `target/e2e-home`", no start point).
- **leaf** (7): `testing.md:34` re-derived (the Test data bullet now says the statement holds by path) ·
  `verification-harness.md:28`, `:43` re-derived (the `boot` step and the root chain name the keepers and the dev-host
  link) · `commands.md:75`, `:91` re-derived (clear the entries, never the link; re-make the link after `cargo
  clean`) · `commands.md:70` no change (G2 "over `target/e2e-home/**/diagnostics/*.ndjson`", by path) ·
  `tests-summary.md:20` re-derived (the Tempdir convention names the keepers and the link).

### remove-owned — new 0 · standing 2 · leaf 2
- `architecture.md:424` and `test-data-bootstrap.md:17`: both amended lines (A1, T6); the `remove_owned` sentences
  themselves stand (the removal order did not change), and the through-the-link fact was added beside them.
- leaves `verification-harness.md:43` (re-derived, above) and `tests-summary.md:20` (re-derived, above).

### keep-failed — new 3 · standing 5 · leaf 4
- new: `architecture.md:409` (A2) · `log-file-location.md:8` (O3) · `5-command-implementation.md:102` (T4).
- standing, no change: `architecture.md:385` (what the variable does and who reads it) · `ci-cd-approach.md:21` and
  `5-command-implementation.md:45`, `:49` (`run --mutants` sets both keep variables to `0`).
- standing edited: `test-data-bootstrap.md:17` (T6: the kept-home lifetime added after "so that `logs` can inspect
  the home").
- leaves: `verification-harness.md:43` (re-derived) · `:44` (the mutation run's `=0`, no change) ·
  `conventions.md:13` (the variable's naming convention, no change) · `tests-summary.md:20` (re-derived).

### home-lifetime — new 0 · standing 4 · leaf 0
- `obs-plan.md:288`, `:451` (§1, the verbatim scope copy): no change. "Retention is the lifetime of the session home"
  stays true; what this chunk adds is what ends a kept home's life on one host, stated in §3 (O3) and in tests' `logs`
  (T4). The playbook's "verbatim upstream copy kept current" rule was read: nothing in §1 became false.
- `log-file-location.md:8` (O3) and `5-command-implementation.md:102` (T4): amended, the claim kept and bounded.

### tempdir-in — new 2 · standing 2 · leaf 1
- new: `security-plan.md:600` (S1: ownership rides the `tempdir_in` that follows) · `test-data-bootstrap.md:13` (T5).
- standing edited: `test-plan.md:626` (T7) · `5-command-implementation.md:6` (T1).
- leaf `testing.md:34`: re-derived (above).

### outside-ws — new 6 · standing 3 · leaf 2
- new: `architecture.md:409` (A2) · `:424` ×2 (A1) · `security-plan.md:600` ×2 (S1) · `test-plan.md:626` (T7) ·
  `5-command-implementation.md:90` (T3) · `test-data-bootstrap.md:17` (T6).
- standing, no change: `architecture.md:410` (test-side install sites outside the repository) · `:441` (the mutation
  scratch, outside the repository) · `obs-plan.md:969` (the same scratch, never printed). Each is another site, and
  none says it is the only one.
- leaves `obs-summary.md:52` and `security-summary.md:69`: the mutation-scratch lines, no change; the security summary
  gained its own bullet for this widening (below).

### By hand: `create_dir_all`, `cargo clean`
- `create_dir_all`: 0 hits in the masters, the registries and the leaves, before and after (`grep -rn -F`). The
  keepers' amended text says "created as a directory" and names no call.
- `cargo clean`: `commands.md:91` (re-derived, above) and this pass's own text at `architecture.md:424` and
  `test-data-bootstrap.md:17`; nothing else.

## Leaves recomputed (cascade step 3)
- From architecture (§Occupied Resources): CLAUDE.md's `GENERATED:setup:*` blocks were re-read against the amended
  section: no line of the overview, modules, warnings, pointer table or architecture blocks derives from the
  Repository or Filesystem registries, so none changed. `gotchas.md` (extracted from architecture and the plans'
  measured facts) gained "`cargo clean` removes the dev host's test-home link". `commands.md` as above.
- From test-plan: `tests-summary.md`, `testing.md` (body), `verification-harness.md` (body), as above.
- From obs-plan: `obs-summary.md` and `observability.md` carry neither G2's command form nor the retention bound, so
  neither changed; `commands.md:70` states G2's scope by path.
- From security-plan: `security-summary.md` gained one Key-decisions bullet for the widening. `rules/security.md`
  (always loaded) was left as it is: it distils none of §Security Anti-Patterns → Code Patterns' test-side bullets
  (the mutation-scratch bullet is not in it either).
- Binds: tests §3 ↔ obs §3: the kept-home lifetime now stands on both sides (T4 and O3). The a11y ↔ obs schema bind
  is untouched.
- Curation homes and judgment bases: 0 rows in every pattern.
