# Materialization plan — viola · setup-project re-run (structural)

Run: 2026-11-13T20-05-09-setup-project · Development Style: agent-driven · stack fragment: rust (root `Cargo.toml`) · host: linux

**Structural change named at invocation** (the operator's payload `setup-rows-payload-2.md`, read whole): the masters
now stand as rows (U65, landing 2026-11-13T01-15-10) — re-render CLAUDE.md's overview · modules · warnings ·
pointer-table blocks from the rows: the verbs line as `claude-md-template.md` spells it; the pointer table's «one
address per key» only where `rec.py contracts` lists keys; the overview from `overview/project-intent` and the
versions from the manifest (`Cargo.toml` · `rust-toolchain.toml`), never a row; the module map from the workspace's
packages joined with the `name` rows of class `crate`, every package listed; the warnings from the plans'
§Anti-Patterns (Universal) rows, and for a plan whose anchor holds no row from its guard sections and architecture's
§Cross-cutting · §Conventions · §Design Philosophy · §Established Decisions.

## Tier 1 — CLAUDE.md

- **overview** — `doc:architecture` `overview/project-intent` (+ `absence/web-or-graphical-frontend` of the same
  section); the Stack line from `overview/stack-and-technologies` with every version from the manifest:
  `rust-toolchain.toml` channel 1.98.1 · `[workspace.package]` edition 2024, rust-version 1.96 · `[workspace.dependencies]`
  clap `=4.6.7`, interprocess `=2.4.4`, portable-pty `=0.9.0`, rmcp `>=3.4.1, <3.5` (Cargo.lock 3.4.1), tokio `=1.53.3`,
  vt100 `=0.16.2`; deployment from `doc:architecture/contracts/deployment-model`. Key directories (≤5) from the tree
  (`git ls-files`, the directory-structure key holds rows, no tree): `src/` · `crates/` · `plugin/` · `fixtures/` · `scripts/`.
- **modules** — `cargo metadata --no-deps --offline` workspace_members (8): viola-agent-claude · viola-core ·
  viola-channel · viola-e2e · viola-pty · viola-state · viola-mcp · viola (root) — joined with the `name` rows of class
  `crate` (`rec.py rows --master architecture --sheet name`, 9 `name/crate/*`): the 8 + `viola-fuzz` (a crate row, not
  a workspace member: `fuzz/` is excluded). 9 listed, libraries first (core · pty · channel · state · agent-claude ·
  mcp · e2e), then binaries (viola), then viola-fuzz. Purposes from `doc:architecture/contracts/crate-dependency-direction`
  (7 rows) and the crate rows that name each.
- **warnings** — NO plan holds a §Anti-Patterns (Universal) anchor: `rec.py find --doc all --regex "(?i)universal"`
  → 0 hits, and no `tree` lists such a section or key. Each plan's anchor therefore holds 0 rows → the draw is the
  guard sections + the plans' NEVER-do sections + arch §Cross-cutting · §Design Philosophy. 10 selected (severity
  security > a11y > obs > tests > design), each with its source rows:
  1. human always wins / hook silent, exit 0 / panic hook first, unwind — security `decision/refusals-never-block-human`,
     `decision/hook-silent-and-exit-zero`; arch `decision/human-always-wins-wheel`, `decision/hook-fails-open`;
     obs `decision/panic-hook-first`, `decision/panic-unwind-in-all-profiles`
  2. upstream text is content / compiled classification — security `decision/upstream-text-is-never-a-command`,
     `decision/compiled-classification-sources`; arch `decision/untrusted-upstream-text`
  3. NEVER-log floor — security `decision/no-messaging-secrets-in-outputs`; obs `decision/claude-env-and-cookie-never-logged`,
     `decision/sensitive-content-detail-file-only`; arch `decision/content-detail-per-instance`
  4. sanitised external errors — arch `decision/sanitised-error-surfaces`, `decision/thiserror-messages-interpolate-nothing`;
     security `decision/no-parser-error-chains-outward`, `decision/no-absolute-paths-in-external-errors`
  5. no switch-off / env no config — security `decision/controls-not-switchable`, `decision/no-control-switch-off`;
     obs `decision/allow-list-not-widenable`; arch `decision/no-environment-configuration`
  6. bound and validate inputs — security `decision/instance-path-from-validated-name`, `decision/reads-capped-at-max-frame`,
     `decision/refuse-control-characters`, `decision/server-side-revalidation`; arch `decision/bounded-readers`
  7. disk state — security `decision/owner-only-file-modes`, `decision/strict-check-before-narrowing`,
     `decision/snapshot-written-only-by-wrapper`, `decision/stamps-written-only-by-verify`; arch `decision/crash-safe-disk-writes`
  8. local trust boundary — security `decision/verify-server-before-frame`, `decision/server-verified-before-any-frame`,
     `decision/no-socket-in-tmp-root`, `decision/no-abstract-namespace-sockets`; arch `decision/same-user-endpoint-trust`
  9. stdout per role / obs_event! — obs `contract/stdout-by-role/{cli,hook,mcp,run}`, `decision/no-print-macros`,
     `decision/obs-event-macro-only`, `decision/no-bare-instrument`
  10. capability ledger / Claude shapes / Tokio — arch `decision/capability-ledger-gate`, `decision/capability-ledger`,
      `decision/tokio-containment`; arch key crate-dependency-direction `decision/claude-shapes-in-one-crate`
  Rejected (audit): a11y `decision/refusal-ends-with-hint`, `decision/codes-beside-plain-word`, `decision/no-spinners`
  (human-output scoped → Tier 2 `frontend.md` / `a11y.md`); design `decision/plain-output-modes`,
  `decision/stdout-data-stderr-messages` (cli-surface scoped → `frontend.md`); tests `decision/no-sleep-synchronization`,
  `decision/no-real-cli-in-ci`, `decision/no-process-env-mutation-in-tests` (test files only → `testing.md`); obs
  `decision/no-hot-path-instrumentation` (run pump only → `observability.md`); arch `decision/mixed-version-tolerance`
  (a Tier-1 session learning already carries it).
- **pointer-table** — verbs line byte for byte from the template. `rec.py contracts`: architecture 5 keys · test-plan
  6 · obs-plan 10 · a11y-plan 9 → keyed clause on those four rows («one address per key» on architecture and
  test-plan, as the template spells them); security-plan · design-system · layout-templates `n/a` → section alone.
  Architecture's directory row gains the template's rows note (the tree is `git ls-files`'s). Every section anchor
  checked against `rec.py tree` (all stand).
- **workflow** · **architecture** — re-derived; stand as they read (arch §Design Philosophy rows
  `decision/mechanism-not-policy`, `decision/send-confirmed-after-the-fact`, `decision/no-daemon-disk-is-shared-truth`).
- **imports** — the template's (`@.claude/session-handoff.md`; U01 ok).
- **deeper-topics** — from the files that exist: 10 docs (5 core + 5 summaries) · services ×7 · session-learnings ·
  11 rule files (unchanged).
- The template's harness maintainer comment (the `rec.py harness land` note) is added below the imports note.

## Tier 2 — .claude/rules/ (11, all preserve)
security · host-linux · testing · testing-src · verification-harness · observability · api · events · frontend · a11y · ci

## Tier 3 — .claude/docs/ (preserve)
core 5 · summaries 5 · services 7 (viola, viola-core, viola-pty, viola-channel, viola-state, viola-agent-claude,
viola-mcp) · session-learnings. Not planned: `services/viola-e2e.md`, `services/viola-fuzz.md` (test-only members;
the previous run's choice stands — rendered only on the operator's `regenerate {leaf}`).

## Agent harness
`scripts/agent-run.sh` preserve (present; drift compare not made — see card) · `verification-harness.md` preserve.

## Hooks · Code reviewer · Gitignore · Gitattributes
Hooks: U02 ok — no re-render needed (entries current). Reviewer: `.claude/agents/code-reviewer.md` preserve (rust).
`.gitignore` U07/U56 ok · `.gitattributes` U06 ok · `rustfmt.toml` U05 ok.

## Code-graph pipeline
U03 ok — 0 lines behind on each of the triple; vendored files present.

## Upgrade
HEAD 4c7d75974ba94a217c3fe9d911b0815cd6b3e41b · 5b path set: `.andromeda/db/log/2026-11-13T07-21-57-2026-11-13-epoch-8-cleanup.ndjson`
· `.andromeda/runs/2026-11-13T12-42-23-wrap/evolve-2026-11-13-epoch-8-cleanup.json` · `.claude/session-handoff.md`

```
upgrade v1.25 · ec33f523
U01 · ok · setup · CLAUDE.md @imports block · 1 import line(s) = the template's
U02 · ok · setup · .claude/settings.json hooks · write current · bash current · PostToolUse on the stdin prologue
U03 · ok · setup · scripts/code-gra… · lines behind: code-graph.py 0 · code-graph-views.sql 0 · code-graph-cookbook.md 0
U04 · noted · setup · .claude/rul… · 9 template line(s) missing above `## Session Additions` — regenerate on the operator's word
U05 · ok · setup · rustfmt.toml · rustfmt.toml present
U06 · ok · setup · .gitattributes · .gitattributes carries `* text=auto eol=lf`
U07 · ok · setup · .gitignore base ignores · every base ignore decided by the root .gitignore (depth 2 included)
U08 · ok · hand · .andromeda/playbook.md seed rules · all 6 seed rules present by name or `seed:` tag (rows)
U09 · ok · noted · .claude/docs/workflow.md · .claude/docs/workflow.md carries `it never commits`
U10 · ok · noted · scripts/agent-run.sh ensure_fresh_artifacts … · scripts/agent-run.sh carries `ensure_fresh_artifacts`
U11 · n/a · hand · code-metrics.ndjson … · past U61 the audit's records are `metric` rows: no ledger file, no terminator
U12 · n/a · hand · .andromeda/residuals.md header · past U61 the residuals are `residual` rows: no file, no header
U13 · ok · hand · .andromeda/{doc}-amendme… · masters in the store — every sidecar entry on the form held at write (U60)
U14 · ok · hand · working-route markerless i… · 27 markerless entries · 0 introducers behind markup or after the em-dash
U35 · ok · hand · masters' lo… · masters in the store — the logs and the keyed contracts out of the body, a key regrown refused held at write (U60)
U36 · noted · noted · .claude/docs/{security,design,te… · .claude/docs/security-summary.md lacks `cascade re-derives it`
U48 · ok · hand · CLAUDE.md p… · masters in the store — CLAUDE.md's pointer rows naming `rec.py contracts {plan}` held at write (U60)
U56 · ok · setup · .gitignore base ignor… · a run's prompts/ ignored by the root .gitignore (.andromeda/runs/*/prompts/)
U57 · ok · hand · the friction family in the store — `.andromeda/db/{moved, log/*.ndjson, reco… · friction: in the store
U58 · ok · hand · the spine in the store — `version` · `epoch` · `chunk` · `carry` · `capability`… · spine: in the store
U59 · ok · hand · a chunk's and a version's documents and files in the store — `doc` · `section` ·… · docs: in the store
U60 · ok · hand · the masters in the store — the seven `.andromeda/{doc}.md` bodies, their cate… · masters: in the store
U61 · ok · hand · the bases in the store — `.andromeda/residuals.md`, `playbook.md`, `drift-base.… · bases: in the store
U62 · ok · hand · the run dirs sealed — every `.andromeda/runs/{run}/` but the sealer's own as `.a… · runs: in the store
U63 · ok · hand · the harness files as the store's render — CLAUDE.md, `.claude/rules/**`, `.cl… · harness: in the store
U65 · ok · hand · the masters as rows — every `section` and `fact` of the seven ma… · masters: 7 master(s) stand as rows
upgrade: for setup 0 · blocks phase 0 · awaiting a door 0 · noted 2 (U04, U36) · INDETERMINATE 0 · 26 detectors of 60 registry entries
```

Dispositions: every existing Tier-2 / Tier-3 leaf and the reviewer `preserve`; none `regenerate` (no operator word).
Phase 0 consistency: the plan writes CLAUDE.md alone (plus this run dir); no leaf, script, hook or ignore changes.
