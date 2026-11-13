# Materialization plan — viola · structural re-run · 2026-11-13T01-25-38

**Form:** re-run — structural. The operator's words (`additional/viola-overseer/setup-rows-payload.md`, read whole at
invocation): «the masters now stand as rows (U65, landing 2026-11-13T01-15-10): a structural change in the masters —
re-render CLAUDE.md's overview · modules · warnings · pointer-table blocks from the rows: the verbs line as
claude-md-template.md now spells it (get {id}), the pointer table's «one address per key»; the overview from
overview/project-intent and the versions from the manifest (Cargo.toml · rust-toolchain.toml), never from a row; the
modules from the name rows of class crate and §Inherited Defaults' Workspace crates; the warnings from §Anti-Patterns
(Universal) rows.»

**Development Style:** agent-driven (`dec-yiekzudl`, decision/agent-driven-development). **Stack fragment:** rust
(root `Cargo.toml`). **Host:** linux.

**The read (Phase 0 step 1), bounded to what the named change re-renders:** `overview/project-intent` ·
`overview/stack-and-technologies` · the `name` sheet of architecture (9 `crate/` rows) · §Inherited Defaults' render
(no Workspace crates list stands on rows) · the keys `project-directory-structure` and `crate-dependency-direction` ·
the 42 architecture decision rows naming a crate · the §Anti-Patterns (Universal) areas of the five plans (security-plan
53 + 11 twin · design-system 17 · test-plan 47 · obs-plan 68 · a11y-plan 15) and architecture §Cross-cutting Patterns
(40) · the panic rows of obs-plan (16 matched `unwind` / `first statement`) · `rec.py contracts` of the four keyed
masters · design-system's outline. Every other leaf is preserved, so no other section is re-derived.

## Tier 1 — CLAUDE.md

| block | disposition | source |
|---|---|---|
| overview | re-render | `overview/project-intent` + `absence/web-or-graphical-frontend`; Stack from `overview/stack-and-technologies` with the versions from `rust-toolchain.toml` (1.98.1), `Cargo.toml` (edition 2024, rust-version 1.96, portable-pty `=0.9.0`, rmcp `>=3.4.1, <3.5`) and `Cargo.lock` (rmcp 3.4.1); key directories from the tree (`git ls-files`): `src/` · `crates/` · `plugin/` · `fixtures/` · `scripts/` |
| modules | re-render | the 9 `name/crate/*` rows, each purpose from the crate-naming decision rows (`claude-shapes-in-one-crate`, `pty-knows-no-agent`, `mcp-dependencies`, `root-bin-consumes-normalised-events`, `e2e-test-only`, `fuzz-separate-workspace`, …); the old non-crate «fleet console» line leaves (the overview carries the planned console) |
| warnings | re-render | 10 selected below |
| pointer-table | re-render | the template's verbs line byte for byte (`get {id}`, the id the command prints); «one address per key» on architecture and test-plan; the retired `.andromeda/state.yaml`, `drift-base.md`, `playbook.md` and the gone `logging-redaction-wire` anchor replaced |
| workflow · architecture | not re-derived — cascade-maintained (standing byte for byte) |
| imports | the template's (U01 ok) |
| deeper-topics | recomputed from the files that exist — unchanged (10 docs + 7 services + session-learnings; 11 rule files) |
| USER:session-learnings | preserved verbatim (14 bullets) |

**Universal warnings (top 10), severity security > a11y > obs > tests > design, each holding for every file:**
1. human always wins — `refusals-never-block-human`, `hook-silent-and-exit-zero`, `hook-fails-open`, obs `panic-hook-first` · `panic-unwind-in-all-profiles`
2. upstream text is content — `upstream-text-is-never-a-command`, `untrusted-upstream-text`, `compiled-classification-sources`
3. NEVER-log floor — `no-messaging-secrets-in-outputs`, `content-detail-per-instance`, obs `sensitive-content-detail-file-only`
4. sanitised error surfaces — `sanitised-error-surfaces`, `thiserror-messages-interpolate-nothing`, `no-parser-error-chains-outward`
5. no switch, no env configuration — `controls-not-switchable`, `no-environment-configuration`, obs `allow-list-not-widenable`
6. bound and validate every input — `instance-path-from-validated-name`, `reads-capped-at-max-frame`, `bounded-readers`, `refuse-control-characters`, `server-side-revalidation`
7. disk state — `owner-only-file-modes`, `strict-check-before-narrowing`, `snapshot-written-only-by-wrapper`, `stamps-written-only-by-verify`, `crash-safe-disk-writes`
8. local trust boundary — `verify-server-before-frame`, `same-user-endpoint-trust`, `no-socket-in-tmp-root`, `no-abstract-namespace-sockets`
9. stdout and logging (obs) — `no-print-macros`, `obs-event-macro-only`, `no-bare-instrument`, contract `stdout-by-role/*`
10. capability ledger and containment (arch §Cross-cutting) — `capability-ledger-gate`, `claude-shapes-in-one-crate`, `tokio-containment`

**Rejected (audit):** a11y `refusal-ends-with-hint` · `no-information-by-colour-alone` (CLI human output — path-scoped,
`.claude/rules/frontend.md` / `api.md`); design `plain-output-modes` · `stdout-data-stderr-messages` (CLI surface);
test-plan `no-sleep-synchronization` · `literal-test-oracles` · `no-real-cli-in-ci` (tests — `.claude/rules/testing.md`);
obs `upload-only-after-secret-scan` · `pinned-actions` (CI — `.claude/rules/ci.md`); security `e2e-home-*` and
`test-removal-own-dirs-only` (test harness — `verification-harness.md`); security `new-listener-needs-log-entry` ·
`new-env-read-needs-log-entry` (process rules, carried by `security.md`).

## Tier 2 — .claude/rules/ (all present → preserve)
`security.md` · `host-linux.md` (U04 noted: 9 template lines missing above `## Session Additions` — regenerate only on
the operator's word) · `testing.md` · `testing-src.md` · `verification-harness.md` · `observability.md` · `api.md` ·
`events.md` · `frontend.md` · `a11y.md` · `ci.md`.

## Tier 3 — .claude/docs/ (all present → preserve)
Core 5 (`stack` · `conventions` · `commands` · `gotchas` · `workflow`) · summaries 5 (`security-summary` (U36 noted:
lacks `cascade re-derives it`) · `design-summary` · `tests-summary` · `obs-summary` · `a11y-summary`) · services 7
(`viola` · `viola-core` · `viola-pty` · `viola-channel` · `viola-state` · `viola-agent-claude` · `viola-mcp`) ·
`session-learnings.md`.

## Agent harness
`scripts/agent-run.sh` present → preserve; `scripts/agent-run.ps1` absent (the Windows code left the tree, recoverable
at `multi-os-final`; Linux-only) — not rendered, named on the card. `verification-harness.md` present → preserve.

## Hooks · Code reviewer · Gitignore · Gitattributes
`.claude/settings.json` hooks — U02 ok (write guard, Bash guard and the PostToolUse prologue current) → no write.
`.claude/agents/code-reviewer.md` present → preserve. `.gitignore` — U07 · U56 ok → nothing appended.
`.gitattributes` — U06 ok → unchanged. `rustfmt.toml` — U05 ok.

## Code-graph pipeline
rust plane (root `Cargo.toml`); U03 ok, 0 lines behind on each of the triple → nothing seeded or proposed.

## Seeds
`rec.py ingest --bases` once (a re-run reads `unchanged`).

## Upgrade

HEAD `56aad6ef249fd092f38991d9b78c8e80f15d0e33`; the 5b path set: empty (the full porcelain read nothing).

```
upgrade v1.25 · befde8c9
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
U14 · ok · hand · working-route markerless i… · 25 markerless entries · 0 introducers behind markup or after the em-dash
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

**Consistency (step 9):** every planned write is CLAUDE.md alone (and its backup); every other artifact is preserved or
current by its §Upgrade row.
