# Cascade dispositions — 2026-10-26-server-verification-before-any-frame

## The search
One `cascade.py sweep` call (trail `cascade-2026-10-26-server-verification-before-any-frame.json`), baseline `6bdfdda8`
(the parent of the oldest pre-CI commit `4aa9def9`), over the apply writers' `## Patterns`: `apply-a11y-plan.md`,
`apply-architecture.p1.md`, `apply-architecture.p2.md`, `apply-design-system.md`, `apply-layout-templates.md`,
`apply-obs-plan.p1.md` · `.p2` · `.p3`, `apply-security-plan.p1.md`, `.raw-apply-security-plan.p2-2.md` (the payload
whose blocks landed; the ruled re-spawn's `apply-security-plan.p2.md` carries `(none)`: its one block re-states a
landed line), `apply-security-plan.p3.md`, `apply-test-plan.p1.md` · `.p2`. 46 patterns, each over the seven masters'
bodies and sets, every registry file, the curation homes, the judgment bases and the leaves; every zero-row pattern's
control fired. Main's own texts this pass: the hand removal `security-plan-detail/input-validation.md:88–95` (F354 (a),
a removal: it states nothing) and the two a11y-plan body lines below (amended from this listing's rows).

Listing: `total (46 patterns) · 20 rows over 12 files` · `per class · new 0/0 · standing 15/7 · leaf 5/5 · curation 0/0 · base 0/0`.

## Rows
- `a11y-01` (on all three OS legs), 11 standing:
  - `.andromeda/a11y-plan.md:261` — amended (main, this pass): the §8 Error recovery Verification broad line now names
    the frame verbs' server-verify refusal on windows-2025 and ubuntu-latest only, as its detail
    (`cognitive-accessibility.md:26`) does.
  - `.andromeda/a11y-plan-detail/cognitive-accessibility.md:26` (edited) — amended this pass; re-read, the exception
    stands once, no duplicate.
  - `.andromeda/a11y-plan.md:74`, `:196`, `:198`, `:312`; `a11y-plan-detail/aria-patterns-roles.md:19`, `:26`, `:32`;
    `registries/contracts/a11y-plan/a11y-testing-tool-pick.md:4`; `registries/contracts/a11y-plan/keyboard-test-harness.md:3`
    (@c7582) — no change: each names another suite (the coverage suite, the outer-PTY and TTY-only cases, the keyboard
    harness), which does run on all three legs; the server-verify negatives are none of them.
- `a11y-02` (trycmd pins every fixed CLI string), 2 standing:
  - `.andromeda/a11y-plan.md:273` — amended (main, this pass): the server-verify hint joins the literal-pinned list.
  - `.andromeda/a11y-plan-detail/cognitive-accessibility.md:36` (edited) — amended this pass; no duplicate.
- `lt-01`, 1 standing: `.andromeda/layout-templates.md:329` (edited) — amended this pass; re-read, the two exit-21
  causes stated once.
- `sec3-03` (the liveness-only pre-check wording), 1 standing:
  `.andromeda/architecture-detail/established-decisions.md:150` — no change: the held `link` / `unlink` verbs'
  unbuilt crossing (`fanout-results.md`, architecture cascade 9 rejected).
- Leaf rows, 5 — each added to step 3's set:
  `.claude/docs/services/viola-channel.md:25` (`arch2-01`), `.claude/docs/services/viola-state.md:6` (`arch2-01`),
  `.claude/docs/services/viola.md:45` (`arch2-01`), `.claude/rules/security.md:12` (`sec2-06`),
  `.claude/rules/testing.md:18` (`tp1-08`).

After this listing, the classify read-back read four `provenance` rows, each a who-ruled phrase on the fork-1 text the
orchestrator's own `· as:` direction had asked for (`established-decisions.md:79`, `input-validation.md:52`,
`test-plan.md:590`, `e2e-test-strategy.md:235`); the ruled re-spawn `apply-r3` removed the phrase at each and kept the
decision, a narrowing of wording that adds no claim (`classify.py check` PASS on each section after it; `detail.py check`
0 defects). The ruling itself is the sidecar entries' (writing rule 7).

Not looked for: the claim in wordings no writer's pattern carries; the leaves' own staleness beyond these rows is step 3's
comparers', not this listing's.

## Leaves
Set: `cascade.py leaves --add` the step-2 leaf rows (`total · pass · 14 group(s) · 27 leaves · 355622 B · unmapped 2`), pass
mode (this chunk does not close Epoch 6: five markerless entries stand under its header). Unmapped, dispositioned:
`.claude/rules/host-linux.md` (host) — the host's own leaf, no comparer's; `.claude/rules/testing-src.md` — pointers to
`testing.md` sections alone, it states no master claim. Comparers: one batch `leaf`, 14 of 14 `check` pass.
Apply (`leaves --apply … --only`): `total · blocks 35 · applied 29 · anchor absent 5 · held 1`.
- a11y-plan, architecture+design-system+layout-templates, design-system+layout-templates, obs-plan,
  security-plan+obs-plan — pass · 0 findings.
- architecture-1 — pass · 3 findings · 3 applied (viola-channel.md:25, :6; conventions.md:33).
- architecture-2 — pass · 7 findings · 2 applied (viola-mcp.md:22, viola-state.md:6) · 5 by hand: the comparer cited
  `.claude/docs/services/viola.md` :81, :91–94 (`anchor moved` at the dry run; the file holds 58 lines), the texts stand
  at :32, :42, :43, :44, :45 — each written with the Edit tool on its quoted leaf-text, replace-with as given.
- architecture-3 — pass · 3 findings · 2 applied (events.md:30, :31) · 1 declined: `.claude/docs/stack.md:34` (omission)
  — the peer-pid read and the start-time note are detail of `stack-and-technologies.md`; a reader of the stack leaf acts on
  no pin or version it lacks; the detail stays in its master.
- architecture+security-plan — pass · 3 findings · 3 applied (api.md:23, :29, :30).
- architecture+test-plan+obs-plan — pass · 1 finding · 1 applied (commands.md:49).
- masters — pass · 2 findings · 2 applied (CLAUDE.md:22, :23, the `GENERATED:setup:modules` block).
- security-plan — pass · 11 findings · 11 applied (rules/security.md:12 ×8, security-summary.md:15, :17 ×2).
- test-plan — pass · 4 findings · 4 applied (rules/testing.md:18, :34; tests-summary.md:33; verification-harness.md:41).
- test-plan+obs-plan — pass · 1 finding · 1 applied (rules/observability.md:40).

## The resume pass (inputs#I17, 2026-10-26)
At the 2a resume, one amendment main raised and wrote itself (amendment-flow §Apply: «a text no writer wrote — an item
main raised after the batch — is main's own»), on the overseer's founder-delegated answer `inputs#I17`: test-plan.md:670
(§7, the fake agent's exit) gains the exit's SIGHUP disposition (the folded fix `7948850a`, the report's addendum,
`inputs#I14`). An in-place extension of one line (609 chars before, under 2 000: the Edit tool); `classify.py check
--section "## 7. Test Data & Fixtures"` → `VERDICT: PASS`.

The search: one `cascade.py sweep` call over main's own `cascade-patterns.toml` (trail
`cascade-2026-10-26-server-verification-before-any-frame.json`), baseline `6bdfdda8`, pattern `fakeexit` — the claim's
own words (`session leader|foreground group|process_group\(0\)|exit-time coverage write|SIGHUP|ignore_hangup`, case
folded) over the seven masters' bodies and sets, every registry file, the curation homes, the judgment bases and the
leaves; its control fired (`test-plan.md:670 (prose)`). Nothing is retired: the line's earlier clause (the hooks' own
process groups) stays true and stands.

Listing: `total (1 pattern) · 4 rows over 4 files` · `per class · new 0/0 · standing 2/2 · leaf 2/2 · curation 0/0 · base 0/0`.
- `.andromeda/test-plan.md:670` · standing edited ×8 — amended (this pass's line; re-read: no intra-line duplicate, the
  hooks' clause and the new exit clause each stated once).
- `.andromeda/test-plan-detail/e2e-test-strategy.md:324` · standing ×3 — no change: the `--exit-no-eof` holder's output
  outliving the session leader per OS, as measured in CI; the holder still runs in its own group (it now also inherits
  SIGHUP ignored, which changes nothing that line states).
- `.claude/rules/verification-harness.md:41` · leaf ×4 — to step 3's set (the comparer recomputes it).
- `.claude/docs/services/viola-pty.md:44` · leaf — to step 3's set (the holder's output per OS; the comparer recomputes it).

### Leaves (the resume pass)
Set: `cascade.py leaves --add .claude/rules/verification-harness.md .claude/docs/services/viola-pty.md` (`total · pass ·
14 group(s) · 27 leaves · 365262 B · unmapped 2`), pass mode against baseline `6bdfdda8` — every master still differs from
it, so every group re-ran against its sources as they now stand (the first pass's leaf files and deltas moved aside to
`aside-leaf-p2/`). Unmapped, dispositioned as the first pass: `.claude/rules/host-linux.md` (host),
`.claude/rules/testing-src.md` (pointers only). Comparers: one batch `leaf` (rendered `--fresh`), `fanout.py check`
`VERDICT: 14 of 14 pass`, `[PATH_CHECK] PASS`. Apply (`leaves --apply … --only`): `total · blocks 11 · applied 3 ·
reshaped 1 · held 7`.
- a11y-plan, architecture+design-system+layout-templates, architecture+security-plan, architecture+test-plan+obs-plan,
  design-system+layout-templates, masters, security-plan+obs-plan, test-plan+obs-plan — pass · 0 findings.
- architecture-1 — pass · 1 finding · 1 applied (viola-channel.md:25, stale: «every frame-writing client takes the
  liveness-only `live_endpoint` read» — `viola hook`'s paths take one shared `read_snapshot` before `connect_by`,
  established-decisions.md:110).
- architecture-2 — pass · 2 findings · 2 declined: viola.md:45 (the exit-21 output forms) and viola-state.md:28 (every
  other reader reading an unreadable snapshot as absent) — detail of conventions.md / established-decisions.md; the leaf's
  statements stand true and a reader acts on nothing against them.
- architecture-3 — pass · 3 findings · 1 applied (rules/events.md:31, stale, the same hook-path read as
  architecture-1's) · 2 declined: stack.md:14 and :25 (the peer-pid read and the start time as the server check's
  observed side) — detail of stack-and-technologies.md, as the first pass declined stack.md:34.
- obs-plan — pass · 1 finding · 1 declined: obs-summary.md:67 (the `server-verify-failed` line shapes) — the summary
  names the code; the shapes stay in log-coverage.md.
- security-plan — pass · 1 finding · 1 declined: rules/security.md:12 (the CLI's `process-exit` line) — the always-loaded
  rule names the refusal and its exit; the line's shape is the observability rule's and authentication-authorization.md's.
- test-plan — pass · 3 findings · 1 applied (rules/testing.md:18, the ETXTBSY planting rule of
  test-data-fixtures.md:33: a test author following the leaf would otherwise plant copies from threads) · 1 by hand
  (rules/verification-harness.md:41 #1, `reshaped` by the tool for the `\x03` backslash: written with the Edit tool on its
  leaf-text, replace-with as given — this pass's own amendment) · 1 declined: verification-harness.md:41 #2 («reaped» →
  «killed and reaped» for the held process) — the leaf's statement stands true.
