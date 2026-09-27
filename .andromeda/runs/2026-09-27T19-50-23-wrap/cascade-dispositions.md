# Cascade dispositions — wrap 2026-09-27T19-50-23 · chunk 2026-09-27-browser-verdict-reachability

## Step 1 — bodies amended
architecture, security-plan, test-plan, a11y-plan, design-system, obs-plan (layout-templates: none). The applied set is
`fanout-results.md` (A1–A17, S1–S6, S9–S11 as ratified, T1–T16, T18–T31, O1–O3) plus the step-2 folds below.

## Step 2 — the sweep
Search: `cascade-patterns.toml` (14 patterns), run as `cascade.py sweep` against the baseline `49f64447` (the pre-CI parent).
Every pattern's known-positive control fired on the pre-pass masters. The patterns and the phrasings they stand for:
- `linux-only-arm`: `browser-linux-only`.
- `ubuntu-e2e-job`: "ubuntu E2E/browser job".
- `browser-ubuntu`: browser/Playwright … ubuntu only / on ubuntu / ubuntu leg … only.
- `runner-node`: runner-image Node.
- `npx-invoke`: `npx --prefix e2e-web playwright test`, the retired invocation.
- `axe-declared`: "already declared", the axe pin.
- `ready-10s`: "bounded at 10 s".
- `linux-run-gate`: `linux{run,gate}`.
- `all-browser`: `--all` … browser/playwright.
- `two-nightly`: "two ubuntu jobs".
- `never-sudo`: "The script never runs sudo".
- `other-launcher`: "One other launcher exists".
- `gui-on-ubuntu`: "headless GUI checks (in CI) run on ubuntu".
- `gate-cov-doc`: `gate --require coverage,doctest` + a closing backtick (the pre-playwright gate).

Zero-row patterns (their controls fired): `ready-10s`, `linux-run-gate`, `all-browser`, `two-nightly`, `never-sudo`. Each
states only that its phrasing no longer stands anywhere swept.

Rows and dispositions:
- `test-plan.md:1929` linux-only-arm new: this pass's Decisions Log entry naming the retirement → no change.
- `test-plan.md:1753` ubuntu-e2e-job standing: a 2026-09-2x Decisions Log entry (Z6), history → no change.
- `architecture.md:552`, `test-plan.md:796`, `test-plan.md:1444` browser-ubuntu new: "`--with-deps` on ubuntu", true new
  text → no change.
- `test-plan.md:1661` browser-ubuntu standing: the §12 initial entry, history (T17 rejected) → no change.
- `a11y-plan.md:132` browser-ubuntu standing: a11y §1, the verbatim a11y-scope copy (playbook "Verbatim upstream copy") → no
  change.
- `.claude/rules/testing.md:20` browser-ubuntu leaf → **re-derived** (step 3).
- `architecture.md:37`, `security-plan.md:710`, `test-plan.md:1670` (edited) runner-node: this pass's text stating the
  retirement → no change.
- `a11y-plan.md:1122` npx-invoke standing: a11y §9 Pipeline integration restated the retired invocation → **amended in this
  pass** (the locked Playwright CLI), sidecar below.
- `.claude/rules/testing.md:47`, `.claude/docs/commands.md:42` npx-invoke leaf → **re-derived** (step 3).
- `a11y-plan.md:132`, `a11y-plan.md:196` axe-declared standing: a11y §1, the verbatim copy → no change (the axe deferral lands
  in §3 a11y-tooling-install).
- `a11y-plan.md:1306` axe-declared standing: §12 Decisions Log key decision, history → no change.
- `security-plan.md:429` other-launcher standing edited (window @c604): "One other launcher exists, `scripts/wsl-exec.sh`" now
  sits beside the new root launch → **amended in this pass** to "One other user-level launcher exists".
- `design-system.md:932` gui-on-ubuntu standing: a 2026-09-24 Decisions Log line, history → no change. (The three body
  restatements — design-system:227, :253, :670 — were amended at O2.)
- `architecture.md:569` (@c986) and `test-plan.md:662` gate-cov-doc standing edited: the host `windows-tests` stage, which
  stays `coverage,doctest` (fork 2) → no change, true.
- `a11y-plan.md:1115` gate-cov-doc standing: a11y §9's Unit/integration row named the test job's gate as `coverage,doctest`
  → **amended in this pass** (`coverage,doctest,playwright`; the E2E row gains the three-OS pipe and its `gate`).
- `.claude/docs/commands.md:33` gate-cov-doc leaf ×2: the pre-push description → **re-derived** (step 3; the Linux half gains
  `run --browser` + the playwright gate, the host half stays true).

Curation homes and judgment bases: 0 rows on every pattern.

## Step 3 — leaves re-derived
Changed sources: architecture, security-plan, test-plan, a11y-plan, design-system, obs-plan. Leaves were enumerated by the
table plus provenance (`grep "Extracted from"` over `.claude/docs` and `.claude/rules`). Then every leaf was grepped for the
touched tokens (`ubuntu|--browser|Playwright|npx|wsl-provision|runner-image|Node|axe|coverage,doctest|nightly|root test|
WITHIN|e2e-web|10 s|junit`):
- CLAUDE.md `GENERATED:setup:overview`: the `e2e-web/` key-directory line was recomputed from arch's tree. The modules,
  warnings, pointer-table and architecture blocks were re-read against arch and are unchanged (no amended fact reaches them).
- `.claude/docs/stack.md` (arch §Stack, mirrored verbatim): the CI/CD row, the new Browser e2e row, the `jq` list (twice),
  the Audit line and the Node line.
- `.claude/docs/commands.md` (arch §Standard Contracts / CI):
  - the `wsl-provision.sh` line;
  - the new `--install-deps` (operator-only) and `install-node.sh` lines;
  - the `jq` line and the browser dependency line;
  - the pre-push description, the built `run` grammar and the browser suite command;
  - the new `npm-audit.sh` line.
- `.claude/docs/workflow.md`: the nightly and test-job sentence, and the pre-push parenthetical.
- `.claude/docs/tests-summary.md`: the test-runner line.
- `.claude/docs/security-summary.md`: the pre-push/WSL bullet (Node, the PATH component, the operator-only root launch) and a
  new supply-chain bullet.
- `.claude/rules/security.md`: the audit tool, a new Node line and the WSL2 pins line with the operator-only root install.
- `.claude/rules/testing.md`: the E2E line (:20) and the running-tests line (:47).
- `.claude/rules/verification-harness.md`: the pre-push line (:24) and the runner-seam line (:45).
- Read and left unchanged (no hit, or a hit still true): `gotchas.md` (:103, :109, :110), `obs-summary.md` (:51),
  `a11y-summary.md`, `design-summary.md`, `conventions.md`, `.claude/rules/{a11y,observability,frontend,events,api}.md`.
- Preserve-verbatim homes: untouched. The two `## Session Additions` / `USER:*` regions were never edited.
