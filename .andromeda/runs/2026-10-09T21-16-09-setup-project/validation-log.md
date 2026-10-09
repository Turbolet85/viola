# Validation log — viola · re-run, upgrade form · 2026-10-09

## Upgrade re-detect (Phase 7.5 step 2)

Phase 7.5 step 1 applied nothing: U11 and U12 read `ok`, and no host re-seed was proposed (U04 `ok`).

`upgrade.py detect --root .` (`upgrade v1.8 · 30b07c54`), the rows this run wrote:

```
U02 · ok-uncommitted · setup · .claude/settings.json hooks · write current · bash current · PostToolUse on the stdin pr…
U04 · ok · setup · .claude/rules/host-{os}.md · every template line present above `## Session Additions`
```

✓ — U02 written, not yet committed (Phase 9 commits it). U04 is read because the operator's `regenerate
host-linux.md` at the card rewrote the leaf's body; it read `ok` before the write and reads `ok` after it. No
setup-class row reads `INDETERMINATE`. U36 stays `noted` (the card's, never this check's).

## Pre-flight and checks 1–14

`health.py check --root . --stack rust --style agent-driven --run-dir …` (`health v1.0 · 1358832b`; trail
`health-no-marker.json`):

| check | status | facts |
|---|---|---|
| pre-flight | ✓ | CLAUDE.md at `./CLAUDE.md` |
| 1 size | ✓ | 125/200 lines · T1 2.0 KB · 0 of 9 bullets over 600 B |
| 2 markers | ✓ | 20 markers · 10 starts · 0 orphan / mismatch / unclosed |
| 3 `@`-imports (hand) | ✓ | one line, `@.claude/session-handoff.md` (`CLAUDE.md:91`), the file present and non-empty |
| 4 rule files | ✓ | 11 files · frontmatter 9/9 parsed · always-loaded 2 (16.7 KB: `security.md` 12.2, `host-linux.md` 4.5 — 4.8 before the regenerate) |
| 5 core docs | ✓ | 5/5 |
| 6 architecture staleness (hand) | ✓ | `architecture.md` mtime 2026-10-09 23:00:54 +0200 is older than `CLAUDE.md` (2026-10-09 23:16:52 +0200, written at this run) |
| 7 handoff (hand) | ✓ | `.claude/session-handoff.md` present, non-empty |
| 8 .gitignore | ✓ | 6/6 entries satisfied by the root `.gitignore` (fragment: rust) |
| 9 plans + summaries | ✓ | plans 6/6 by name · summaries 5/5 by name |
| 10 master route | ✓ | present |
| 11 operational artifacts | ✓ | seeded 2/2 · planes rust, ts · pipeline 4/4 · behind py 0 · sql 0 · cookbook 0 |
| 12 state.yaml (hand) | ✓ | `schema_version: 3`, the three lean fields only |
| 13 agent harness | ✓ | agent-driven · harness 2/2 · `test -x` ok · verbs sh 5/5 · ps1 5/5 |
| 14 pointer table | ✓ | 17 entries (threshold 5) |

## Hook smoke (each hook fed its input on stdin, the command as stored in `.claude/settings.json`)

Run from the Bash tool as one script file by path, from the project root, the six Bash-guard inputs as JSON files
(a command that quotes the banned heredoc form is itself refused by the guard). The two PreToolUse commands as
stored: `bash ~/.claude/skills/andromeda-tools/hooks/write-guard.sh` · `bash ~/.claude/skills/andromeda-tools/hooks/bash-guard.sh`.

| arm | expected | got |
|---|---|---|
| `jq` | present | present (`/usr/bin/jq`) |
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

No arm exited 126 or 127: the install's two guard scripts are reachable from this host.

## Summary

14 ✓ / 0 ⚠ / 0 – / 0 ✗ · hook smoke ✓ · upgrade re-detect ✓. Decision: commit.
