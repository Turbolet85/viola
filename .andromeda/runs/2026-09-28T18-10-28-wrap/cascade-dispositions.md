# Cascade dispositions — 2026-09-28-capability-ledger-and-viola-verify

**The search:** `cascade.py sweep` over `cascade-patterns.toml` (18 patterns, every one derived from this pass's amendments
before the first grep; every control fired on the pre-pass masters, baseline `9b4f6f47`), listing in `cascade-sweep.txt`.
Patterns and the claims they key on — the verbs and phrasings, not only the names:
`stderr-summary` (verify's summary on stderr) · `backtrace-sym` (`force_capture`, `Backtrace::`, symbolising) ·
`readiness-owner` (the `boot` line-3 check "joins with" this chunk) · `only-locally` (verify runs only locally / never in
CI) · `verify-14-steps` (the 14-row verify sketch, the step counter) · `files-glob` (rstest `#[files]` as the hygiene
walk) · `human-caller` (`human.rs`'s only caller is `run`) · `gate-no-step` · `no-viola-hooks` (unwrapped sessions carry
no viola hooks) · `name-absent` (hook silent when `VIOLA_NAME` is absent) · `producers` (only run/hook write role
files) · `probe-suite` (the live probe suite) · `gate-in-agent` (the version gate lives only in viola-agent-claude) ·
`one-enum` (one error enum per crate) · `panic-exempt` (the closed panic-exemption list) · `maxframe-list` (the
`MAX_FRAME` consumer list) · `strict-stamps` (every process reading stamps runs strict-modes first) · `fake-stdout` (the
fake agent's stdout contract).
Sections read beside the sweep: arch [CLI Version Compatibility] whole, [Session Liveness], [Error Handling],
§Occupied Resources (Binary, integration names, Filesystem, Repository), §Crate dependency direction, §Build system;
security §Input Validation table + Constants, §Authentication `~/.viola/` row (offset-read, 5 710 chars), §Data
Protection, §Error Handling, Decisions Log; test-plan §1 readiness, §3 `boot`, §5 CLI, §7 Fake agent + Fixture hygiene;
obs-plan §2, §4 table + Edge flows, §7 Panic hooks, §10 exemptions; layout `viola verify` + refusal block; design cli
patterns + Exit-code phraseology.

## Rows
- `stderr-summary` — arch:139, security:241, design:809: **new** (this pass's own text naming stderr for refusals /
  drained stderr; true). The retired obs:979 claim is gone (0 standing rows).
- `backtrace-sym` — arch:466, obs:1141: **new** (the raw-frames text; "never-symbolised" is the new claim). 0 standing.
- `readiness-owner` — test-plan:526: **edited** (the new text names the chunk as history: "since … landed the recorded
  fixtures with the harness unchanged"; true). test-plan:174, obs:870 amended (T9, T8 twin).
- `only-locally` — arch:91 new, arch:587 edited (the new split; true). security:162 → **amended** this pass (cited the
  retired arch claim). test-plan:60 → **amended** (§1 quotation of arch's CI/CD line). test-plan:93 **no change**
  (`claude agents --json` shape, another subject). test-plan:1670, test-plan:1697, obs:1771 **no change**: Decisions Log
  history (1697 is the very amendment request this pass satisfied). workflow.md:49 **leaf → re-derived**.
- `verify-14-steps` — design:795 → **amended** (the 14-row sketch → six rows, stdout, 2.1.283). layout:354 **no
  change** (the step counter still is the one progress pattern; no count stated). test-plan:958 edited (new; true).
  design-summary.md:55 **leaf, no change** (names the step counter only).
- `files-glob` — test-plan:1384 new (the retirement). test-plan:836, :864 **no change**: §4 Unit's `#[files]` over
  `fixtures/claude/*/*.json` for hook-payload parsing — the glob now matches the 2.1.283 set, so the rstest refusal
  (empty glob) no longer applies; a true plan sharing the token. test-plan:1346 **no change** (the fixture library
  lists `#[files]`; true). services/viola-agent-claude.md:36 **leaf → re-derived** (it named the walk as `#[files]`).
- `human-caller` — arch:487 edited (new comment; true); arch:436 amended (A13). 0 other rows.
- `gate-no-step` — 0 rows (control fired; the sentence is gone from arch:93).
- `no-viola-hooks` — arch:100 edited (the exception now follows the claim in the same rationale; true).
- `name-absent` — arch:69 edited (the capture exception follows). test-plan:119 **no change** (a test input
  variation). obs:512, obs:1307 **no change** (exemption 3, hook without `VIOLA_NAME`, still true; the capture arm is
  exemption 5). obs:1573 **no change** (Decisions Log D-16). arch:368 (`VIOLA_NAME` … "its absence makes every hook a
  silent exit 0", 60+ chars from the token, so outside the pattern's window) was read by hand beside the sweep:
  **amended** — "every hook event path", with the capture arm named as reading no `VIOLA_*`.
- `producers` — arch:390 edited (verify added; true).
- `probe-suite` — arch:91 new ("typed or live probe": the owners' probes; true). commands.md:27 **leaf → re-derived**.
- `gate-in-agent` — 0 rows (arch:45 re-worded).
- `one-enum` — arch:97, arch:155 edited, arch:641 new (the interim divergence named beside the rule; true).
  conventions.md:15 **leaf → re-derived**.
- `panic-exempt` — obs:512 edited (list gains exemption 5), obs:1308 **no change** (exemption 4 itself; exemption 5
  appended after it), obs:1755 **no change** (Decisions Log).
- `maxframe-list` — security:241 edited (the extended list; true).
- `strict-stamps` — security:207 (offset 789 window read), :229, :234 edited (the stamps-read gap named in each). security:599
  **no change** (the 2026-09-23 initial Decisions Log entry, history). services/viola-state.md:21 **leaf → re-derived**.
- `fake-stdout` — test-plan:1380 edited (`--report-version` changes only the `--version` answer — still true; print
  mode's `ok` is a different mode).

## Curation homes and judgment bases
- 0 `curation` rows and 0 `base` rows across all 18 patterns: no Session Additions, `USER:session-learnings`,
  `docs/session-learnings.md`, `playbook.md` or `drift-base.md` text restates a retired claim.

## Leaves re-derived (step 3; the table's floor plus provenance)
- arch → CLAUDE.md `GENERATED:setup:*` recomputed against the amended sections: **no change** (overview, modules,
  warnings, pointer-table and architecture blocks state nothing the pass retired; "only `viola verify` writes
  `ledger/stamps.json`" still holds). `docs/conventions.md` (Rust error types), `docs/commands.md` (verify + the stamps
  envelope, Standard Contracts), `docs/stack.md` (error types + raw panic frames' dependencies), `docs/workflow.md`
  (CI/CD), `docs/services/viola.md`, `docs/services/viola-agent-claude.md`, `docs/services/viola-state.md`:
  **re-derived**. `docs/gotchas.md`: **no change** (its verify line — every relied-on behaviour is a ledger row — holds).
- security-plan → `docs/security-summary.md` (strict-modes interim gaps), `rules/security.md` (the second dated
  exception + the capture arm), `rules/api.md` (home-path exception list): **re-derived**.
- test-plan → `docs/tests-summary.md` (print mode, cli_verify, hygiene walk): **re-derived**; `rules/testing.md`,
  `rules/verification-harness.md`: **no change** (`stamped_home` is still the interim no-stamp seam; hand-written stamps
  still banned).
- obs-plan → `rules/observability.md` (cli internal-error line, raw frames, capture exemption): **re-derived**;
  `docs/obs-summary.md`: **no change** (it states neither the backtrace source nor verify's streams).
- design-system / layout-templates → `docs/design-summary.md`: **no change** (names the step counter only);
  `rules/frontend.md`: **no change** (its hint rules hold for verify's refusals).
- Binds: test-plan §3 ↔ obs-plan §3 — the harness is unchanged this chunk; the `boot` line-3 pointer was moved in both
  (test-plan :174/:526, obs-plan :870). a11y ↔ obs schema — untouched (diag schemas unchanged).
