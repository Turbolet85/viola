# Validation Log — setup-project 2026-09-24T07-33-41

**Pre-flight:** ✓ `CLAUDE.md` present.

| # | Check | Status | Diagnostic |
|---|---|---|---|
| 1 | CLAUDE.md size | ✓ | 120/200 lines · 11.7 KB · Tier-1 region 0.9 KB, 0 bullets over 600 B |
| 2 | Section markers | ✓ | parser clean (9 GENERATED, 1 USER) |
| 3 | `@`-imports | ✓ | `@.claude/session-handoff.md` resolves |
| 4 | Rule files | ✓ | 9 files, frontmatter parses; always-loaded 10.8 KB (security.md 4.5 KB, host-win32.md 6.3 KB); others 3.7–4.6 KB, 0 Session Additions |
| 5 | Docs 5 core | ✓ | stack · conventions · commands · gotchas · workflow |
| 6 | Architecture staleness | ✓ | architecture.md older than CLAUDE.md by 9.8 h |
| 7 | session-handoff.md | ✓ | present, non-empty (fresh seed) |
| 8 | .gitignore Claude + stack | ✓ | `.claude/backup/`, `.claude/settings.local.json`, Rust fragment |
| 9 | 6 plans + 5 summaries | ✓ | all present by name |
| 10 | master-route present | ✓ | empty at greenfield (no chunk promoted) |
| 11 | Operational artifacts | ✓ | drift-base + playbook + 5 code-graph files; rust-analyzer ✓, scip-typescript ✓, duckdb + protobuf ✓; code-graph.py / views.sql / cookbook current with templates |
| 12 | state.yaml lean | ✓ | schema_version 3, lean fields only |
| 13 | Agent harness | ✓ | 5 commands in both shims; `test -x scripts/agent-run.sh` on-host ✓ (a first probe via a python-spawned `bash` resolved WSL's bash — measured the probe, not the file; re-measured from the Bash tool) |
| 14 | Pointer table | ✓ | 17 entries |
| — | Hook smoke | ✓ | formatter: PostToolUse command fed stdin JSON reflowed a mis-formatted `.rs` (exit 0, file changed; temp removed); Bash guard deny arm exit 2, allow arm exit 0; jq present; generated-dir guard blocks `target/` under both `/` and `\` paths, passes src/docs paths |

**Summary:** 14 ✓ / 0 ⚠ / 0 – / 0 ✗ · hook smoke ✓
**Decision:** commit (pre-flight passed, zero ✗).

**Deviation recorded:** the generated-dir guard adds `path=${path//\\//}` before its match — the matrix form exited 0 on every Windows backslash path (measured), i.e. it never fired for the Write/Edit tools on this host. Candidate fix for `references/hooks-matrix.md`.
