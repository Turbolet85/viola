# Cascade dispositions — wrap of 2026-10-10-viola-revive

The search: `cascade.py sweep` over `cascade-patterns.toml`, 32 patterns, each derived from an amendment of this
pass after the last body edit of the fan-out's list, each with its control fired on the pre-pass masters (baseline
`00c73fda`). The listing's own last two lines:

    total (32 patterns) · 47 rows over 17 files
    per class · new 0/0 · standing 34/8 · leaf 13/9 · curation 0/0 · base 0/0

What the patterns look for: the retired "no reader / no caller / library code" claim of the replay and its
owed-reader sentence (`no-reader`, `no-caller`, `no-process-calls`, `library-code`, `owed-to-revive`, `first-reader`,
`four-callers`); the two moved counts (`wait-23`, `eight-options`); the `refuse` writer's one caller
(`refuse-only-run`); the child's cwd having one source (`cwd-source`); the one relied-on shape without a row
(`one-shape`); the no-daemon sentence and the role sentence (`every-other-verb`, `other-verb-cli`); the one flag
passed to the child (`child-flag`); the closed exit-1 detail set, by token and by its prose (`exit1-set`,
`exit1-causes`); the verb lists in both spellings and the help group (`verbs-dot`, `verbs-comma`, `help-setup`); the
fixed-message form's one user (`fixed-form-only`); `viola run` as the one passthrough, by three phrasings (`sgr-run`,
`run-passthrough`, `run-tui`); the snapshot's key list in three phrasings (`snapshots-hold`, `snapshot-keys`,
`no-event-fields`); the panic exemptions (`exemptions`); the endpoint wait (`endpoint-wait`); strict-modes as exit 1
for three roles only (`strict-exit21`); `--json` of every data verb (`json-every-verb`); the fake agent's one set
field (`prompt-only`).

What was not looked for: a restatement of an amended claim that carries none of these phrasings; any text outside
the seven masters, the registry files, the three curation homes, the two judgment bases and the leaf bodies.

Twelve patterns printed no row with their control fired (`no-process-calls`, `owed-to-revive`, `first-reader`,
`wait-23`, `eight-options`, `child-flag`, `verbs-comma`, `help-setup`, `fixed-form-only`, `snapshot-keys`,
`no-event-fields`, and `no-reader` / `no-caller` in the masters): the retired wording stands in no master or key
file. That is a statement about those patterns.

## Master and key-file rows (standing 34 rows, 8 files)

Rows marked `edited` are lines this pass amended, where the token stands inside the new text by design. Each was
re-read for an intra-line duplicate of the retired claim.

- `architecture.md:247` `library-code`, `four-callers` (edited) → amended: the fallback is library code with one
  product reader; the four callers of `read_snapshot` are named. No duplicate on the line.
- `architecture.md:47` `cwd-source` ×2 (edited) → amended: the first hit is `run`'s source, the second the new
  sentence's "its own current directory". Both stand as written.
- `security-plan.md:244` `cwd-source` (edited) → amended.
- `architecture.md:403`, `security-plan.md:230` `refuse-only-run` → no change: "called only by `viola verify`" is
  said of `update_stamps` and of the hidden `hook --capture` arm, another subject sharing the phrase.
- `obs-plan.md:901` `other-verb-cli` (edited) → amended: the sentence now names `revive` beside `run`.
- `obs-plan.md:621` `exit1-set` (edited) → amended: `run`'s set stands and revive's three details follow it.
- `obs-plan.md:308`, `:465` `exit1-causes` (edited) → amended.
- `design-system.md:43` `sgr-run`, `run-passthrough` → amended after the listing: the 0.0 row's sentence names a
  passed `viola revive` beside `viola run`.
- `design-system.md:887` `sgr-run` → amended after the listing: the ban's reason names a passed `viola revive`.
- `a11y-plan.md:94`, `:124`, `:184`, `:727`, `:962`, `:1001` `sgr-run`, `run-passthrough`, `run-tui` (edited) →
  amended; the surface keeps its name, the `viola run` TUI, and each line names a passed revive.
- `test-plan.md:691` `run-passthrough` → no change: the E2E driver table names the tui surface by its name; the
  driver (the outer PTY) is the same for a revived start, and `tests/chaos_revive.rs` uses it.
- `security-plan.md:47` `snapshots-hold` (edited) → amended.
- `architecture.md:91`, `:377`, `:404`, `security-plan.md:279`, `obs-plan.md:1067` `exemptions` → no change: each
  names the hidden `hook --capture` arm for what it is, not the exemption list. `obs-plan.md:512` (edited) →
  amended: the list's inline copy names `viola revive --list`.
- `architecture.md:408` `endpoint-wait` (edited) → amended.
- `registries/contracts/test-plan/5-command-implementation.md:33` `endpoint-wait` ×2 (edited) → amended: the stop
  path's wait and, in the new sentence, the wait the kill path replaces.
- `registries/contracts/test-plan/5-command-implementation.md:32` `library-code` → no change: "the same library
  code as the `boot` / `cleanup` commands", the harness fixture, another subject (read by offset, c124 of 923).
- `test-plan.md:647` `strict-exit21`, `test-plan.md:1071` `prompt-only`, `layout-templates.md:556`
  `json-every-verb` (edited) → amended.

## Leaf rows (13 rows, 9 files) and the re-derived set

- `.claude/rules/events.md:30` `no-reader`, `library-code` → re-derived (the replay's one product reader, no `cwd`
  from a replay, `session_chain`).
- `CLAUDE.md:24` `no-caller` → re-derived (the modules line of `viola-state`); `CLAUDE.md:43` `one-shape` →
  re-derived (two relied-on shapes, both owed on "Paste newline ledger row"); `CLAUDE.md:85` `every-other-verb` →
  re-derived (each wrapper owns one endpoint).
- `.claude/docs/services/viola-state.md:6` `no-caller`, `library-code` → re-derived.
- `.claude/docs/obs-summary.md:68` `exit1-set` → re-derived (D-20's codes).
- `.claude/docs/stack.md:12` `verbs-dot` → re-derived.
- `.claude/docs/a11y-summary.md:46` `run-tui` → no change: the ban's sentence holds as the leaf words it.
- `.claude/docs/services/viola.md:37` `snapshots-hold` → no change: the `Snapshots` holder type, another subject.
- `.claude/rules/observability.md:39` `exemptions` → re-derived: a line for revive's two arms follows it.
- `.claude/docs/gotchas.md:123` `endpoint-wait` → re-derived (the rule after a kill).

Leaves re-derived with no row, by recomputing from the amended sources: `CLAUDE.md` modules lines of
`viola-agent-claude` and the root bin; `.claude/rules/security.md` (one line for revive, above `## Session
Additions`); `.claude/docs/security-summary.md`, `tests-summary.md`, `conventions.md`, `commands.md`,
`services/viola.md`, `services/viola-agent-claude.md`. `design-summary.md` holds no verb list or help group and
`a11y-summary.md` no entity list (read by `grep` for `revive`, `plugin install`, `setup`): not changed.

## Curation homes and judgment bases

0 rows in each. Nothing routed to P3 or to the propose-and-approve channel from this sweep.

## Binds

`test-plan §3 ↔ obs-plan §3`: no harness command, status shape or log format changed; the two new
`process-exit` details are enumerated in neither §3. `a11y-plan schema ↔ obs-plan schema`: the a11y violation
schema is untouched. Both pairs agree as before.

## The registry

`registry.py check` printed `0 defect(s) — clean` for architecture, test-plan, obs-plan and a11y-plan after the
five key-file edits (`project-directory-structure.md`, `5-command-implementation.md`, `otel-sdk-init.md`,
`logging-stack.md`, `keyboard-test-harness.md`).
