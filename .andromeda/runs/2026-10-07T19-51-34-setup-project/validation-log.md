# Validation log — setup-project re-run (the upgrade: U02, U04)

Read at 2026-10-08T04:25Z, after Phase 7.5. The card was answered "yes" by the founder (relayed by the operator):
the sort of the 12 items as listed, items 2 and 6 dropped into the run dir.

## Phase 7.5

- U11 ok, U12 ok at P0: no apply.
- The host leaf's re-seed: `upgrade.py apply --root . --id U04 --sort host-reseed.json --run-dir {run_dir}`, exit 0.
  `.claude/rules/host-win32.md` (rendered for win32) → `.claude/rules/host-linux.md`, 10 467 B → 4 506 B every turn;
  12 items, 3 988 B found on disk where the sort says: keep 5 · learnings 4 · `verification-harness.md` 1 · drop 2.
  The old leaf is backed up (`.claude/backup/host-win32.md.pre-setup-2026-10-07T19-51-34-setup-project`, gitignored)
  and removed (`removed.md`).
- `CLAUDE.md:105` re-rendered: the `GENERATED:setup:deeper-topics` rule list names `host-linux.md`
  (1 insertion, 1 deletion; 124 lines). No other line of `CLAUDE.md` names the old leaf.
- Read back: `host-linux.md` holds the five kept items below `## Session Additions`; `verification-harness.md` +1
  line (item 9); `session-learnings.md` +20 lines (items 1, 4, 5, 7 under their titles); `host-reseed-dropped.md`
  holds items 2 and 6 verbatim.
- Not carried, as the card said: the old leaf's `## Long single-line files` section. It stands in the backup and in
  git history only.

## Upgrade re-detect (`detect-p75.txt`, exit 0)

| Row | State | Result |
|---|---|---|
| U02 `.claude/settings.json` hooks | `ok-uncommitted` · write current · bash current | ✓ |
| U04 `.claude/rules/host-{os}.md` | `ok-uncommitted` · every template line present above `## Session Additions` | ✓ |

No setup-class row reads `INDETERMINATE`. U36 stays `noted` (the card's, not this check's).

## Health (`health.py check --root . --stack rust --style agent-driven`, exit 0; trail `health-no-marker.json`)

| Check | Status | Diagnostic |
|---|---|---|
| Pre-flight | ✓ | CLAUDE.md at ./CLAUDE.md |
| 1 CLAUDE.md size | ✓ | 124/200 lines · T1 1.8 KB · 0 of 8 bullets over 600 B |
| 2 section markers | ✓ | 20 markers · 10 starts · 0 orphan / mismatch / unclosed |
| 3 `@`-imports valid | ✓ | by hand: one import, `@.claude/session-handoff.md` (line 91), the file exists |
| 4 rule frontmatter | ✓ | 9 rule files · 7/7 parsed · always-loaded 2 (16.2 KB): `security.md` 11.8 KB, `host-linux.md` 4.4 KB |
| 5 core docs | ✓ | 5/5 |
| 6 architecture staleness | ✓ | by hand: `architecture.md` 2026-10-07T19:24Z, `CLAUDE.md` 2026-10-08T04:24Z (arch is the older) |
| 7 session-handoff.md | ✓ | by hand: present, non-empty |
| 8 .gitignore | ✓ | 6/6 entries (fragment: rust) |
| 9 plans + summaries | ✓ | 6/6 · 5/5 |
| 10 master-route | ✓ | present |
| 11 code-graph + seeds | ✓ | seeded 2/2 · planes rust, ts · pipeline 4/4 · behind 0 · 0 · 0 |
| 12 state.yaml | ✓ | by hand: `schema_version: 3` and the three lean fields, four flat scalar lines. Read by eye, not through a YAML library (PyYAML is not installed on this host) |
| 13 agent harness | ✓ | harness 2/2 · `test -x` ok · verbs sh 5/5 · ps1 5/5 |
| 14 pointer table | ✓ | 17 entries |

## Hook smoke (each command read from `.claude/settings.json` with jq and fed its input on stdin, from the Bash tool)

`jq` present (`/usr/bin/jq`).

| Arm | Want | Got |
|---|---|---|
| formatter: PostToolUse on a mis-formatted `.setup-validation-test.rs` | exit 0, file changed | exit 0, changed (rustfmt reflowed it); the temp file deleted, read back absent |
| Bash guard: `cat > x.md <<'EOF'…` | 2 | 2 |
| Bash guard: `python - <<'PY'…` | 0 | 0 |
| Bash guard: `cd .andromeda && ls` | 2 | 2 |
| Bash guard: `cd . && ls` | 0 | 0 |
| Bash guard: `ls && cd .andromeda` | 2 | 2 |
| Bash guard: `ls; ( cd .andromeda && ls )` | 0 | 0 |
| write guard: `src/x.rs` | 0 | 0 |
| write guard: `target` + a backslash path | 2 | 2 |
| write guard: `C:` + backslashes + `target` | 2 | 2 |

Hook smoke: ✓ (formatter · Bash guard · write guard · jq).

## Limits of this run (stated on the card)

- The upstreams were read by section, not whole (`materialization-plan.md`, its first section).
- `upgrade.py detect` was also run once after the P5 write, as a read-back; the letter has Phases 1–6 never
  re-run it. Read-only.

## Summary

14 ✓ / 0 ⚠ / 0 – / 0 ✗ · hook smoke ✓ · upgrade re-detect ✓.
Decision: commit (pre-flight pass, zero ✗).
