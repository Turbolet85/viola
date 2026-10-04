# Cascade dispositions — 2026-10-04-the-wheel

**Search.** `cascade.py sweep` (v1.1) over `cascade-patterns.toml`, 20 patterns, written after the last body amendment
of this pass (architecture A1–A10 · security S1–S8 · test T1 + T2′ · obs B1–B9 · a11y Y1–Y5 · layout L1); baseline
`eb53a582` (the oldest pre-CI commit's parent); every control fired on the pre-pass masters. Full listing:
`sweep-out.txt`. Phrasings swept, not only names: the retired non-editing claim (`focus ?[,/] ?mouse`,
`mouse,? and resize`), the span value (`human-key`), the retired `release` shapes (`{budget?}`, `` `pause` `{}` ``,
`carr(y|ies) `from``, `from a driver rather than`), the dated-gap enumerations (`sixth`, `liveness-only`), the verb
enumerations (`` `answer`,? (and )?`verify` ``, `` `last` and `answer` have producers ``, `(`send`, `wait`, `last`,
`answer`)`, `` `answer.client` ``, `` `answer`, `cli` ``), the running-turn claim (`clears? the running-turn`), the
open question (`left to the security specialist`), the schema gap (`pending an enum extension`), the console read
(`\^Z|0x1A|ReadConsole|Ctrl-Z`), the controls table (`four verb negatives`), Bootstrap amendment 4.
Sections read beside the rows: architecture [Human Takeover / Wheel] · [PTY] · [MCP] · §Standard Contracts Channel
methods · §Conventions exit 1 · §Occupied Resources diagnostics; security-plan :205–:235, :397, :524; obs-plan §1 Path 5,
§2 naming, §4 span kinds / surface table / wheel scenario, §6 catalog; a11y-plan §1 :90–:100, :321, §4 :573, §11 :990.

## Master / registry rows
- `a11y-plan.md:100`, `:321`, `:990` fmr-list — **amended** this pass (Y2 · Y3 · Y5): the edited lines keep the
  focus/mouse/resize words and now add terminal replies + the F-W3 Windows clause; no retired claim stands.
- `a11y-plan.md:100`, `:573` mouse-resize — **amended** (Y2 · Y4): `:573`'s "(3) … mouse and resize sequences do not
  move the wheel" stays as the case's name, followed by the step-wise form and the Unix / windows-2025 split.
- `a11y-plan.md:90` verb-list — **no change**: a full verb list that already holds `pause`, `release` (true).
- `security-plan.md:205`, `:207`, `:230`, `:524` sixth-gap / liveness — **amended** (S4 · S5 · S6 · S8): each
  enumeration now carries the seventh gap; `:205 @c4831` is this pass's new text (window read).
- `architecture.md:70` turn-clear — **new** (A8): "is to clear … as built, none exists beyond the in-flight `send`
  slot … `release` returns the wheel alone" — the design clause kept as intent, the as-built status beside it.
- `architecture.md:47` ctrl-z ×6 + `:17`, key file `project-directory-structure.md:43` — **new** (A9 · A10 · A6).
  `architecture.md:47 @c3337` and `test-plan.md:621` — **no change**: `claude`'s own `ReadConsoleInputW` (the H2
  resize-loss question), a different subject sharing the token (window read).
- `registries/contracts/test-plan/log-format.md:14` carry-from — **amended** this pass (the tests↔obs §3 bind:
  test-plan's Log format mirrors obs-plan's `release-from-driver` corr rule, B8) — the sweep's one master-text
  find outside the fan-out: `release` "that carries `from`" → "a string `from`; another type is a plain `-32602`".
- `obs-plan.md:325`, `:667` client-spans — **no change**: the `answer` dialog scenario's driver-side spans (true).
- `obs-plan.md:573`, `:581` client-spans — **new** (B5 · B4).
- `obs-plan.md:660` liveness — **no change**: scoped to `wait` / `last`'s own scenario (true).
- 0-row patterns (control fired): `human-key`, `{budget?}`, `` `pause` `{}` ``, `left to the security specialist`,
  `from a driver rather than`, `pending an enum extension`, `` `answer`, `cli` ``, `four verb negatives`, amendment 4,
  the producers list, the `from` readers list — each retired claim is gone from every master and registry file.

## Base rows
- `playbook.md:57` sixth-gap — **no change**: "the sixth is the never-routine class" (the playbook's sixth rule).

## Curation-home rows
- none. (Read beside the sweep: CLAUDE.md `USER:session-learnings` "a `release` carrying `from` is refused `-32602`"
  — still true at its grain: a string `from` is refused; left to P3, not routed.)

## Leaf rows → step 3 (re-derived)
- `.claude/rules/security.md:14` — the seventh gap appended (F-W1).
- `.claude/docs/security-summary.md:18` — the seventh gap in the interim list.
- `.claude/docs/services/viola.md:22` (the non-editing list + F-W3), `:30` (the cli verb list), `:37` (the wrapper
  modules `wheel.rs` / `snapshot.rs`), a new **Wheel verbs** bullet; `:38` / `:40` — **no change** (answer / wait-last
  scoped, true).
- `.claude/docs/services/viola-state.md:22` — `pause` / `release` join the liveness-only snapshot readers.
- `.claude/docs/gotchas.md:108–109`, `.claude/docs/services/viola-pty.md:30` ctrl-z — **no change** (the H2 resize
  loss; a different read).
- Beyond the rows, recomputed from the amended sections: `services/viola-pty.md` (`host_stdin()`),
  `services/viola-channel.md` (`ProtocolError::ReleaseFromDriver`), `services/viola-core.md` (`HumanTyping`,
  `WheelCause`), `docs/tests-summary.md` Path 5 (as landed, owed `:102` / `:139`), CLAUDE.md `GENERATED:setup:modules`
  viola-pty (`host_stdin`). Checked and unchanged: CLAUDE.md overview / warnings / architecture blocks,
  `docs/conventions.md` (`viola pause|release <target> [--budget]` — the design surface; `--budget` is `:91`'s),
  `docs/commands.md`, `docs/stack.md`, `docs/obs-summary.md` and `docs/a11y-summary.md` (no retired claim),
  `rules/observability.md`, `rules/a11y.md`, `rules/testing.md`.
- Late amendment (raised at P3 from the report's curation correction, a master claim corrected at its source):
  test-plan §3 → 5-command implementation, Test selection — the one-Rust-test example `run --e2e --filter
  'test(/path2_send_confirms/)'` → `run --integration --filter …` (`--e2e` a usage error, exit 2, until the first E2E
  binary). The sweep was RE-RUN after it with pattern `e2e-one` (`run --e2e --filter`): 0 rows, its control fired on
  the key file's pre-pass text — `.claude/rules/testing.md:46` (the leaf) re-derived before the re-run. Every other
  pattern's rows are identical to the first run (`sweep-out.txt` holds the re-run). `test-plan.md:1144` (`agent-run
  run --e2e` as the E2E tier's CI command) — **no change**: the planned tier, not a one-test example.
- Lateral binds: test-plan §3 ↔ obs-plan §3 — the Log format `release-from-driver` corr rule now reads the same in
  both key files; a11y schema ↔ obs schema — untouched.
