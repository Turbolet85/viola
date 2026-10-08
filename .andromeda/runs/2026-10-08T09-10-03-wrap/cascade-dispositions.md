# Cascade dispositions — 2026-10-08-first-live-test-and-self-drive

The search: `cascade-patterns.toml` of this run dir, 17 patterns, run once by `cascade.py sweep` after the last body amendment of the pass (baseline `0dafa09a`). Every pattern's control fired on the pre-pass masters. Two patterns returned 0 rows (`hint-old-b`, `hint-open`): a statement about those patterns, their retired wordings stand nowhere. Not looked for: a restatement of a retired claim that uses none of these wordings; the rows below are what was seen.

## Masters and key files

| row | disposition |
|---|---|
| architecture.md:91 `hint-old`, `hint-advice` (new) | this pass's own text: the old advice quoted as history |
| architecture.md:70 `list-closed`, `list-replies`, `f-w2` (standing, edited) | amended: the list as extended; the original enumeration stands inside it |
| registries/contracts/a11y-plan/keyboard-test-harness.md:8 (three patterns, standing, edited) | amended: the same |
| security-plan.md:77 `list-replies`, `f-w2` (new) | this pass's own text (the new vector) |
| test-plan.md:811 `list-replies` ×2 (new) | this pass's own text (Path 5, as landed) |
| a11y-plan.md:100, :321, :990 `list-replies`; :100 `f-w2` (standing) | no change: each names the class by reference and stays true with the list extended (read; the a11y detector read the same) |
| architecture.md:49 `trailing` ×2 (standing, edited) | amended; "typed as an empty text" stands and is true (`text_bytes` 0) |
| test-plan.md:750 `trailing` ×2 (standing, edited) | amended; the test names stand |
| architecture.md:7, :491 `win-live` (new) | this pass's own text |
| a11y-plan.md:137 `win-live` (standing) | amended in this pass: it cited architecture as calling Windows "the live-supported target"; now "the first target" (a cross-master citation of amended wording) |
| architecture.md:377, :454 `viola-dir-step` (standing, edited) | amended: the designed order kept, its as-landed state and owner added |
| obs-plan.md:235, registries/contracts/obs-plan/log-file-location.md:4, otel-sdk-init.md:11 `viola-dir-step` (standing) | no change: the designed resolution; its build is owned by the route entry "CLI machine contract" (the three obs proposals were rejected as a sequencing deferral, `fanout-results.md`) |
| architecture.md:47, :386, :424 ×3, test-plan.md:623, :668, obs-plan.md:737, 5-command-implementation.md:55, :138, :159 ×3 `local-live` (standing) | no change: read; each states where `--local-live` runs, what it refuses or which suite value it is, none states what a lone selector runs. architecture.md:424 read by offset window (`cascade.py window --line 424 --at 424`): "the one real-`claude` `viola verify` of the local-only `run --local-live`" names the home that run makes, true |
| test-plan.md:698, :1261, 5-command-implementation.md:24 ×3, :53 ×4 `local-live` (standing, edited) | amended |
| 5-command-implementation.md:24 `default-all` (standing, edited) | amended: the default scoped |
| test-plan.md:102, :698 `real-cli-local` (new) | this pass's own text |
| test-plan.md:184 `real-cli-local` (standing) | no change: "Local-only mode adds `viola verify` against the real CLI" asserts no order and no "only" |
| architecture.md:424 `real-cli-local` (standing) | no change (the row above) |
| obs-plan.md:1023 `probe-literal` (new) | this pass's own text (the reword made before the first citation sweep's write) |
| architecture.md:50, security-plan.md:239 `route-101` (new) | the two citations re-pointed by hand at the first sweep, `working-route.md:105` |

## Leaves (each joins the re-derive set)

| row | disposition |
|---|---|
| .claude/docs/services/viola.md:22 `list-closed`, `list-replies`, `f-w2` | re-derived: the list as extended |
| .claude/docs/services/viola.md:37 `f-w2` | no change: names the classifier by its ruling |
| .claude/docs/gotchas.md:84 `owed-live`, `unmeasured-live` | re-derived: the `/clear` live proof as measured |
| .claude/docs/tests-summary.md:26 `trailing` | re-derived: the live readings and the trailing-CR outcome; :29 (Path 5) re-derived too, by the table, with no row |
| CLAUDE.md:85 `win-live` | re-derived (`GENERATED:setup:architecture`): Windows the first target, first live on the Linux dev host |
| .claude/docs/commands.md:22 `win-live` | re-derived |
| .claude/docs/conventions.md:44, .claude/docs/services/viola-state.md:6 `viola-dir-step` | re-derived: the as-landed state carried with the order |
| .claude/docs/commands.md:30, gotchas.md:109, services/viola-pty.md:31, tests-summary.md:21, workflow.md:49 `local-live` | no change: read; none states what a lone `--local-live` runs or the default |
| CLAUDE.md:24, .claude/rules/events.md:34, .claude/docs/services/viola-state.md:6 `route-101` | re-derived: route `:105` (each a generated or body line, none in a curation home) |

Leaves read by the table and left: `design-summary.md` and `security-summary.md` (neither quotes the hint, the list or the vector), `a11y-summary.md` (its Path 4 line names the boundary, no list), `obs-summary.md` (the §9 reword is form only), `.claude/rules/*` other than `events.md` (0 rows).

## Curation homes and judgment bases

0 rows in `USER:session-learnings`, any `## Session Additions`, `docs/session-learnings.md`, `playbook.md` and `drift-base.md`.
