# Materialization plan — viola (re-run · the upgrade)

Run: `2026-10-07T19-51-34-setup-project` · Development Style: agent-driven · stack fragment: rust · host: linux
(`upgrade.py host`). Asked for by the operator for the two pending upgrades, U02 and U04; the card is presented
whole, with the Host leaf row, and nothing of U04 is written before its answer.

## How the upstreams were read (a limit of this run, stated)

All nine upstreams exist (`input.md`, `architecture.md`, the six plans, `master-route.md`; 1 017 946 B together).
They were NOT read whole. They were read by the sections Phase 0's steps name, through each file's header index:

- `architecture.md` (181 000 B): §Design Philosophy and §Stack and Technologies (lines 1–40), §Infrastructure
  Patterns' stub, §Cross-cutting Patterns, §Project Intent, §Inherited Defaults, §Existing Scopes (lines 447–513);
  the `##` index of the rest. Its five keyed contracts were listed (`registry.py contracts`), not opened.
- The six plans: the `##` index of each, and each plan's Universal anti-pattern subsection (security 602–615,
  design 840–852, test 1330–1348, obs 1185–1195, a11y 1059–1074). test-plan §3 and obs-plan §3 were listed by key
  (6 and 10 keyed contracts), not opened.
- `master-route.md`: through `route.py cursor` (44 records, 44 complete, 0 pending, 0 gated).
- `input.md`: its header index only.

Why: four of the masters are over the 120 000 B whole-read bound the handoff applies to a sidecar, the whole set
is about 1 MB, and this run writes nothing that a master decides (one hook entry from the matrix, and the host
leaf's re-seed). What the limit costs: a Tier-1 change that only an unread section would show is not found here.
The last wrap's cascade (2026-10-07T19:33Z, 23 amendments, nine leaves re-derived) is what keeps Tier 1 current.

## Upgrade
5b HEAD: `2861e19517bd1b0bcdbddf916246fc1c36fabb58`
5b path set (all expected-transient bookkeeping):
- ` M .andromeda/friction-log.ndjson`
- ` M .andromeda/runs/2026-10-07T19-12-23-wrap/evolve-2026-10-07-a-send-ending-in-a-newline-is-confirmed.json`
- ` M .claude/session-handoff.md`

5b route cursor: `records 44 · complete 44 · pending 0 · gated 0` · `half-promote 0 of 44 stamped lines vs 44
master records`.

`upgrade.py detect --root .` (exit 0; the same bytes stand in `detect-p0.txt` beside this file):
```
upgrade v1.6 · 814083ff
U01 · ok · setup · CLAUDE.md @imports block · 1 import line(s) = the template's
U02 · behind · setup · .claude/settings.json hooks · write current · bash pre-cd
U03 · ok · setup · scripts/code-graph.py · code-graph-views.sql · code-graph-cookbook.md · lines behind: code-graph.py …
U04 · behind · setup · .claude/rules/host-{os}.md · host-win32.md was rendered for win32, this host is linux — the re-r…
U05 · ok · setup · rustfmt.toml · rustfmt.toml present
U06 · ok · setup · .gitattributes · .gitattributes carries `* text=auto eol=lf`
U07 · ok · setup · .gitignore base ignores · every base ignore decided by the root .gitignore (depth 2 included)
U08 · ok · hand · .andromeda/playbook.md seed rules · all 6 seed rules present by name or `seed:` tag
U09 · ok · noted · .claude/docs/workflow.md · .claude/docs/workflow.md carries `it never commits`
U10 · ok · noted · scripts/agent-run.sh ensure_fresh_artifacts hook · scripts/agent-run.sh carries `ensure_fresh_artifa…
U11 · ok · setup · .andromeda/friction-log.ndjson · code-metrics.ndjson line endings · friction-log.ndjson LF · code-me…
U12 · ok · setup · .andromeda/residuals.md header · header = route-resolve's template
U13 · ok · hand · .andromeda/{doc}-amendments.md entry form · 7 sidecars · every entry on the form
U14 · ok · hand · working-route markerless introducers · 31 markerless entries · 0 introducers behind markup or after t…
U35 · ok · hand · masters' logs + keyed contracts · ok: infra K, test K, test L, obs K, obs L, a11y K, a11y L, security…
U36 · noted · noted · .claude/docs/{security,design,tests,obs,a11y}-summary.md header line · .claude/docs/security-summ…
upgrade: for setup 2 (U02, U04) · awaiting a door 0 · noted 1 (U36) · INDETERMINATE 0 · 16 detectors of 40 registry entries
```
U03's truncated facts: health check 11 read `behind py 0 · sql 0 · cookbook 0` at this session's start.

Acts:
- U02 → P5 step 2: replace the setup-rendered PreToolUse `Bash` guard (`pre-cd`: it has no `cd` arm) with the
  matrix form. The write guard reads `current`; the PostToolUse rustfmt row equals the matrix prologue.
- U04 → P7.5, only after the card's "yes": the re-seed by `upgrade.py apply --id U04 --sort host-reseed.json`.
  P2 does not touch a leaf rendered for another host.
- U11 ok, U12 ok: nothing to apply at P7.5 step 1.
- U36 noted: the five summaries' header line. No legal writer here; the card names it.

## Tier 1 — CLAUDE.md
Backed up to `.claude/backup/CLAUDE.md.pre-setup-2026-10-07T19-51-34` (md5 `366bece90f8ae9b3a3562ecbb5014228`).
Re-render GENERATED:setup from this checkpoint; `USER:session-learnings` preserved verbatim (8 bullets).
@imports = `.claude/session-handoff.md` only.

- **Overview / architecture:** equal arch §Project Intent (Core function, Growth model), §Design Philosophy and
  §Stack as read. No change.
- **Modules:** nine entries, unchanged (arch §Inherited Defaults names the framework, not a crate list; the crate
  map is cascade-maintained and was not re-derived from an unread section).
- **Top-10 universal warnings** (security > a11y > obs > tests > design), unchanged:
  1. The human always wins; `viola hook` exits 0, no stderr, fails open (security Universal; arch Fail open).
  2. Upstream text is content; `statusline_command` the only shell-out; direct spawn (arch Cross-platform
     discipline · Untrusted upstream text).
  3. NEVER-log floor (security; obs Universal: scrub before any byte leaves).
  4. External errors: codes and fixed messages only (security Error Handling).
  5. No config, env or flag disables a control or widens redaction (security Universal; arch Config management).
  6. Bound every input (security Input Validation).
  7. Disk state modes, single writers, one write per line (security Universal; arch Crash-safe disk writes).
  8. stdout reserved; no print macros; `obs_event!` under `skip_all` (arch Diagnostic output channels).
  9. `panic = "unwind"`; the custom panic hook first.
  10. Tokio containment; no C crates; Claude shapes only in agent-claude + ledger rows (arch Cross-cutting).
  Rejected (audit): exec-form absolute pinned path, the verify stamp gating a dialog decision, the DLL-search
  restriction and the two fake-agent seams (all carried in the always-loaded `security.md`); "no new listener or
  channel method without a Security Decisions Log entry" (security Universal, a design-time rule; the ten are
  full); the design, a11y and test Universal bans (path-scoped, Tier 2).
- **Pointer table:** 17 rows, unchanged.
- **Workflow:** key commands unchanged. Held for the operator, NOT written (outside the U02 / U04 scope, as the
  2026-09-28 run held it): the lead-in "(the workspace lands with the first chunk; until then they have nothing
  to act on)" is stale at 44 complete chunks; the template's lead-in is "**Key commands:**".
- **Deeper topics:** the rule list names `host-win32.md` (line 105). Before the card that file still exists, so
  the line stands. P7.5 re-renders it from the rule files that then exist (`host-linux.md`).
- Result before the card: the render equals the present file byte for byte, so no write to `CLAUDE.md`
  (124 lines ≤ 200).

## Tier 2 — .claude/rules/ (disposition: preserve all)
security · testing · verification-harness · observability · api · events · frontend · a11y: present, preserved
whole with their `## Session Additions`. The operator named no leaf for regeneration.
Host leaf: `host-{os}.md` = `host-linux.md` (absent). `host-win32.md` (rendered for win32) is not touched by P2;
its re-seed is the card's Host leaf row. The sort is `host-reseed.json` / `host-reseed.md`:
keep 5 (3, 8, 10, 11, 12) · learnings 4, all mixed (1, 4, 5, 7) · `verification-harness.md` 1 (9) · drop 2 (2, 6).
One body section of the old leaf, `## Long single-line files` (lines 65–87), stands in no host section of the
installed template and is not one of the 12 items: the rendered `host-linux.md` will not carry it.

## Tier 3 — .claude/docs/ (disposition: preserve all)
Core 5 · summaries 5 · services 8 (viola · viola-core · viola-pty · viola-channel · viola-state ·
viola-agent-claude · viola-mcp · viola-ui) · `session-learnings.md` present (12 353 B; +20 lines if the sort
stands). U09 ok. U36 noted.

## Agent harness
agent-driven → `scripts/agent-run.{sh,ps1}` present → preserved (only-if-missing). U10 ok; health check 13 read
the five verbs in both. No fresh-render comparison (the scripts are project-evolved; test-plan §3's 5-command
contract was not opened in this run). `verification-harness.md` present → preserved.

## Hooks — .claude/settings.json
Formatter rustfmt (arch §Stack Code quality); clippy is a gate; no Rust type-checker hook. Back up to
`.claude/backup/settings.json.pre-setup-2026-10-07T19-51-34`, then:
- PreToolUse `Edit|MultiEdit|Write|NotebookEdit` write guard: equals the matrix → unchanged.
- PreToolUse `Bash` guard: `pre-cd` → the matrix form (the present command plus the `cd` arm, byte for byte up to
  the heredoc arm's `fi`).
- PostToolUse `Edit|MultiEdit|Write` rustfmt row: equals the matrix prologue → unchanged.
- `env` PYTHONUTF8 / PYTHONIOENCODING present → kept. The file holds no other key.
`rustfmt.toml` present → no formatter-config write, no reflow step.

## Code-graph pipeline
Planes rust + ts. All five files present; the triple current (0 · 0 · 0); `scip_pb2.py` / `requirements.txt`
preserved. `.andromeda/cache/` present. Not an adopted project: no build fired here.

## Code reviewer
`.claude/agents/code-reviewer.md` (rust) present → preserved.

## Gitignore / gitattributes
U07 ok → no line appended. `.gitattributes` carries `* text=auto eol=lf` (U06 ok) → unchanged. Index:
4 446 i/lf · 62 i/none · 12 i/-text · 1 i/mixed (`fuzz/corpus/paste_text/multi-line`, `attr/-text`, a fuzz corpus
input).

## Seeds
state.yaml · session-handoff.md · drift-base.md · playbook.md · session-learnings.md: all present; none written.
