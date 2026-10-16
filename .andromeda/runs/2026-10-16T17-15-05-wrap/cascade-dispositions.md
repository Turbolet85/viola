# Cascade dispositions — 2026-10-16-remove-the-browser-leg

## Sweep (cascade step 2)

The search: `cascade-patterns.toml` in this run dir, 18 patterns derived from every amendment of this pass before the
first grep — the leg's tokens (`node-pin`, `npm`, `playwright`, `e2e-web`, `ts-plane`, `axum`, `viola-ui`,
`fixed-port`), the narrowed sets and their mechanisms (`gui-keys`, `scan-gui`, `obs-web`, `ui-process`, `a11y-leg`),
the design text (`dejavu`), the deferral wording itself (`stand-until`: «Remove the browser leg», «browser test leg»,
«browser leg»), the moved counts (`probes-13`, `counts`: «nine closed», «the ten values», «both JUnit files», «three
ubuntu jobs» …) and one word pattern (`words`: `browser`, `GUI`, `Node`, `npm`) for a restatement under none of the
named tokens. Baseline `b6bbbd68` (the pre-CI parent). Every control fired.

The first full run (before the last three fixes) listed `total (17 patterns) · 162 rows over 24 files`, `per class ·
new 4/4 · standing 13/4 · leaf 145/18 · curation 0/0 · base 0/0`. It caught three standing sites the apply had left:
test-plan §7 Test data lifecycle «`session.json`, the cookie file and the supervisor» (L972), obs-plan §1 Telemetry
surfaces «`process` (`run|hook|mcp|ui`)» (L138) and §1 Service identity's same list (L209) — all three amended in this
pass. The listing of record is the re-run after them: `total (17 patterns) · 159 rows over 24 files`, `per class · new
4/4 · standing 10/4 · leaf 145/18 · curation 0/0 · base 0/0`; and `words` alone: `total (1 pattern) · 76 rows over 32
files`, `per class · new 2/2 · standing 21/8 · leaf 52/21 · curation 1/1 · base 0/0`.

**Master rows (new + standing), each read:**
- `new` ×4 (`stand-until`; `words` ×2 of the same sites) — test-plan.md:150, 5-command-implementation.md:31,
  build-system.md:15, a11y bootstrap-phases.md:7: this pass's own text, «left with the browser leg» — amended (current
  truth: the leg is gone; the `run` step numbers are kept because the code cites `run` step 4).
- test-plan.md:1203 (`playwright`) — the recorder ban «NEVER use Selenium IDE / Playwright codegen-recorder as test
  author»: a true ban on a recorder, no runner claim — no change.
- build-system.md:15 (`e2e-web`, `ts-plane`, edited) — this pass's text naming the input that left — amended.
- design-system.md:247 (`fixed-port`) — `127.0.0.1:47319` as ATIS anatomy Data, a 0.2.0 design input the restructure
  wrap kept — no change.
- test-plan.md:446 (`gui-keys`, edited) — this pass's note that an older record with `ui` / `cookie_file` still parses —
  amended.
- test-plan.md:818 and obs-plan.md:647 (`obs-web`) — the E4 and GUI-view scenario headings over the 0.2.0 deferral
  stubs — no change.
- design-system.md:243 and :269 (`dejavu`, edited) — «the Linux fallback» in this pass's text — amended.
- `words` standing ×21 over 8 files — every one read: the product's npm-shim → `claude.exe` resolution (architecture.md:42,
  :44 @c2727 @c3995, :75, :99, :280; security-plan.md:110, :212 @c910; test-plan.md:54, :301, :498, :553; obs-plan.md:76),
  the real `claude`'s Node/libuv console read (test-plan.md:555 @c1500), the GUI deferral stubs (architecture.md:91,
  :172; obs-plan.md:647), design-system.md:212 («The browser shows the Problem JSON», a 0.2.0 design input), and this
  pass's text (test-plan.md:150; a11y-plan.md:101; a11y ci-integration.md:8; a11y bootstrap-phases.md:7) — true claims
  sharing a token, no change.

**Curation row:** `.claude/docs/session-learnings.md:85` («a browser step», the `gh auth` device flow) — unrelated to
this leg; no P3 extension.

**Base rows:** none.

**Leaf rows:** 145 (`words` adds 52) over the leaves below — each leaf joins step 3's set; none stands in for the
recompute.

## Leaves (cascade step 3)

Mode: pass (the chunk does not close Epoch 5: `route.py epoch` reads it markerless 7 · pending 1). `cascade.py leaves`
set 14 groups, 28 leaves; one comparer each, one batch inside one snapshot pair (before `tree 56bf4990…`, after `tree
68bb7555…`, index `a310f23a…` and head `fac2d9de…` unchanged; `git diff-tree` printed exactly the 14 `A` lines of
`leaf-{group}.md`). The apply's line: `total · blocks 84 · applied 61 · preserved 7 · reshaped 15 · held 1`.

Per group (findings · applied by the tool · by hand · declined):
- a11y-plan — 14 · 9 · 0 · 5: `.claude/rules/a11y.md:3`–`:7` (`preserved`, the `paths:` frontmatter, setup's layout;
  the web-only rule file is named for the founder, not deleted).
- architecture-1 — 0. architecture-3 — 0. security-plan+obs-plan — 0.
- architecture-2 — 9 · 7 · 2 · 0: `stack.md:31` (the Browser e2e row) deleted by hand (`reshaped`, a table row);
  `services/viola-ui.md:9` held and worded by hand («None: … have left the tree.»: this chunk removed them).
- architecture+design-system — 2 · 2 · 0 · 0.
- architecture+security-plan — 1 · 0 · 0 · 1: `.claude/rules/api.md:4` (`preserved`, frontmatter).
- the CLAUDE.md group — 1 · 0 · 1 · 0: the `e2e-web/` key-directory line deleted by hand (`reshaped`, an emptied line).
- architecture+test-plan+obs-plan (`commands.md`) — 21 · 12 · 9 · 0: lines 12, 13, 15, 43, 63, 68 deleted by hand; the
  table cells of 31, 34 and 36#1 rewritten by hand (`--browser`, `ui` out).
- design-system — 1 · 1 · 0 · 0. obs-plan — 7 · 5 · 2 · 0: `obs-summary.md:18` (a table cell) and `:48` (deleted) by hand.
- security-plan — 7 · 6 · 1 · 0: `rules/security.md:35` (the Node line) deleted by hand.
- test-plan — 17 · 16 · 0 · 1: `.claude/rules/testing.md:6` (`preserved`, frontmatter).
- test-plan+obs-plan — 4 · 3 · 1 · 0: `rules/observability.md:33` deleted by hand.

Beyond the comparers: the re-sweep after the apply (`total (17 patterns) · 54 rows over 18 files`, `per class · new 4/4 ·
standing 10/4 · leaf 40/12`) found `stack.md` lines no master states any more — getrandom / constant_time_eq (the GUI
token), `npm-audit.sh` (Audit, `jq`), tower-http in viola-ui, axum-test / tower / reqwest / eventsource-client, the Node
line, the browser a11y libraries and «DejaVu as the Linux CI render» (each checked absent from every master and key file
by `git grep -c`) — rewritten by hand. The rest of its leaf rows are true or the leaf layout's own: the frontmatter
`paths:` above; `commands.md:5`, `:16`, `a11y-summary.md:33`, `viola-ui.md:9` (this pass's text); CLAUDE.md:26 / :100
(the deferral and the `viola-ui.md` stub); `tests-summary.md:32` (E4 deferred); `viola-core.md:24` («`a11y-violation` is
not a variant», true).

Unmapped: `.claude/rules/host-linux.md` (the host leaf, no master's) and `.claude/rules/testing-src.md` (no statement
of the leg: `grep -i browser|playwright|e2e-web|node|npm|fixed-port|ui-restart|gui` 0 hits) — no comparer, no edit.
