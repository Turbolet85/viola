# Validation log — viola · re-run, upgrade form · 2026-10-09

## Upgrade re-detect (Phase 7.5 step 2)

Phase 7.5 step 1 applied nothing: U11 and U12 read `ok`, and no host re-seed was proposed (U04 `ok`).

`upgrade.py detect --root .` (`upgrade v1.7 · 970f0065`), the one row this run wrote:

```
U48 · ok-uncommitted · setup · CLAUDE.md pointer rows · 4 of 4 indexes named
```

✓ — written, not yet committed (Phase 9 commits it). No setup-class row reads `INDETERMINATE`. U36 stays `noted`
(the card's, never this check's).

## Pre-flight and checks 1–14

`health.py check --root . --stack rust --style agent-driven --run-dir …` (`health v1.0 · 1358832b`; trail
`health-no-marker.json`):

| check | status | facts |
|---|---|---|
| pre-flight | ✓ | CLAUDE.md at `./CLAUDE.md` |
| 1 size | ✓ | 124/200 lines · T1 1.8 KB · 0 of 8 bullets over 600 B |
| 2 markers | ✓ | 20 markers · 10 starts · 0 orphan / mismatch / unclosed |
| 3 `@`-imports (hand) | ✓ | one line, `@.claude/session-handoff.md` (`CLAUDE.md:91`), the file present and non-empty |
| 4 rule files | ✓ | 9 files · frontmatter 7/7 parsed · always-loaded 2 (16.5 KB: `security.md`, `host-linux.md`) |
| 5 core docs | ✓ | 5/5 |
| 6 architecture staleness (hand) | ✓ | `architecture.md` mtime 2026-10-08 11:26:09 +0200 is older than `CLAUDE.md` (written at this run) |
| 7 handoff (hand) | ✓ | `.claude/session-handoff.md` present, non-empty |
| 8 .gitignore | ✓ | 6/6 entries satisfied by the root `.gitignore` (fragment: rust) |
| 9 plans + summaries | ✓ | plans 6/6 by name · summaries 5/5 by name |
| 10 master route | ✓ | present |
| 11 operational artifacts | ✓ | seeded 2/2 · planes rust, ts · pipeline 4/4 · behind py 0 · sql 0 · cookbook 0 |
| 12 state.yaml (hand) | ✓ | `schema_version: 3`, the three lean fields only |
| 13 agent harness | ✓ | agent-driven · harness 2/2 · `test -x` ok · verbs sh 5/5 · ps1 5/5 |
| 14 pointer table | ✓ | 17 entries (threshold 5) |

## Hook smoke (each hook fed its input on stdin, the command as stored in `.claude/settings.json`)

| arm | expected | got |
|---|---|---|
| `jq` | present | present |
| formatter (PostToolUse) on a mis-formatted `.setup-validation-test.rs` | exit 0, file changed | exit 0, changed; temp file removed |
| Bash guard · `cat` heredoc with a file target | 2 | 2 |
| Bash guard · python heredoc, no file target | 0 | 0 |
| Bash guard · `cd .andromeda && ls` | 2 | 2 |
| Bash guard · `cd . && ls` | 0 | 0 |
| Bash guard · `ls && cd .andromeda` | 2 | 2 |
| Bash guard · `ls; ( cd .andromeda && ls )` | 0 | 0 |
| write guard · `src/x.rs` | 0 | 0 |
| write guard · `target`, a backslash, `x.rs` (printf octal form) | 2 | 2 |
| write guard · a drive path through `target` with backslashes (printf octal form) | 2 | 2 |

## Summary

14 ✓ / 0 ⚠ / 0 – / 0 ✗ · hook smoke ✓ · upgrade re-detect ✓. Decision: commit.
