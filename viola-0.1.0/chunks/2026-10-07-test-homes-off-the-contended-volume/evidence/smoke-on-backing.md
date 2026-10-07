# The boot smoke on the backing (step 7)

/implement P3's own boot on the Linux dev host, 2026-10-07T07:50Z, with `target/e2e-home` a link to the owner-only
tmpfs directory (`evidence/backing.md`). Each harness step's output went to a scratch file; `boot` was not piped.

## The session
- `bash scripts/agent-run.sh boot --session p-homes-smoke --instance builder`: exit 0, `"ok":true`.
- `bash scripts/agent-run.sh status --session p-homes-smoke`: exit 0, `"ok":true`, `"state":"ready"`.

The three readings below were taken with that session booted and before its `cleanup`.

## 1. The G2 start point, old against new (the one-off control of step 4)
| start point | files printed |
|---|---|
| `find target/e2e-home -path '*/diagnostics/*.ndjson' -print` (the old one) | 0 |
| `find target/e2e-home/ -path '*/diagnostics/*.ndjson' -print` (the new one) | 3 |

The three: `hook-builder.ndjson`, `run-builder.ndjson` and `cli-viola-builder.ndjson`, under the one
`viola-session-*/home/diagnostics/`. Over the link the old start point finds nothing, so G2 would read its
fail-closed `empty scope`; with the trailing slash the walk descends the link.

## 2. The modes (`stat -c '%a'`)
| path under the session's home | mode |
|---|---|
| `home` | 700 |
| `diagnostics/cli-viola-builder.ndjson` | 600 |
| `diagnostics/hook-builder.ndjson` | 600 |
| `diagnostics/run-builder.ndjson` | 600 |
| `bin/*/viola` (the pinned copy, one) | 700 |

## 3. `grep -rc 'strict-modes-failed\|symlink-ignored'` over the home's `diagnostics/`
0 in each of the three files.

## After
- `bash scripts/agent-run.sh cleanup --session p-homes-smoke`: exit 0, `"ok":true`, `"cleaned":["p-homes-smoke"]`,
  `"processes_gone":true`, `"endpoint_gone":true`, `"home_removed":true`, `"killed":[]`.
- `ls -A target/e2e-home/`: 0 entries, no `viola-session-*` left.

## The same smoke as gate entries
The gate block's six smoke entries ran before this boot, in the run of 07:48Z to 07:50Z: `cleanup`, `boot`,
`status`, `bash scripts/g2-zero-panics.sh` (`g2: clean`), `bash scripts/agent-run.sh schema-check` (`"ok":true`,
3 files, 22 lines, 0 torn, no failure) and `cleanup`, all green.
