# P5 controls — the preservation guard, plan entry 15

Fired as the entry itself, `gate.py run --chunk 2026-11-01-unix-endpoint-and-home-hardening --docs-root . --entry 15 --project-root
$TMPDIR/claude-1000/-home-turbolet-dev-projects-viola/cf080ff9-c28a-4e1a-a043-f8927043f502/scratchpad/guard-clone`, over a scratch clone (`git clone
--shared` of the project at HEAD 6ba49f53086a658d2f73b0990b298b9107633d86), 2026-11-01T16:32Z, no `--run-dir`.

| input in the clone | entry line | the guard's last lines |
|---|---|---|
| unmodified | green · exit 0 | `guard: allowed 0 · companions excluded 0 · changed 0 · excluded 0` / `guard: clean` |
| six listed files edited, the new test file minted | green · exit 0 | `guard: allowed 7 — .config/nextest.toml, crates/viola-channel/src/server.rs, crates/viola-core/src/lib.rs, crates/viola-state/src/strict.rs, schemas/diag-line.v1.json, src/cmd/hook.rs, tests/security_negatives_socket_dir.rs · companions excluded 0 · changed 7 · excluded 0` / `guard: clean` |
| the same, plus `src/cmd/list.rs`, `deny.toml` and an unlisted `tests/not_in_the_plan.rs` | red · `last line 'guard: clean' ✗ (last: 'guard: outside 3 files')` | `guard: allowed 7 — … · changed 10 · excluded 0` / `guard: outside 3 files` |

The real tree's reading, entry 15 at P5: green, `guard: allowed 0 · companions excluded 0 · changed 0 · excluded 0`, `guard: clean`.
