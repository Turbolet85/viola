# Classified — tests · test-strategy.b1

## Answers
test-strategy#1.1 · yes · the refusal-order matrices and the Unix and Windows strict-modes unit cases in `viola-state` run on all three OSes
test-strategy#1.2 · yes · the new refusals on CLI, hook, MCP and the on-disk contracts need crate-level suites over a real endpoint and a real tempfile home
test-strategy#1.3 · yes · E2E Paths 1, 6 and 7 carry the tampered pinned exe (exit 1), the statusline no-run and the one-instance boot
test-strategy#1.4 · no · the chunk changes no parser surface (paste, ndjson framing, hook stdin, vt100 feed, agents JSON, `viola_name`), and the property and fuzz suites are Not needed
test-strategy#1.5 · yes · the hook's added check must keep the spine hooks under `SPINE_DEADLINE` on hyperfine max and `CONNECT_DEADLINE` below it
test-strategy#1.6 · yes · the filesystem fault family and the E5 snapshot-corruption leg (reached only on the P4 fork) are chaos-tier work
test-strategy#1.7 · no · the mutation gate is judged only at the epoch boundary, never per chunk
test-strategy#1.8 · no · the chunk changes no fake-agent replay and no recorded dialog, framing or hook fixture set
test-strategy#2.1 · no · standing S2 · integration binaries named `<surface>_<topic>.rs`, helpers shared from `tests/support` and never copied
test-strategy#3.1 · yes · the server-identity and hook impostor cases must stand on a home that passes the new check ahead of the server check
test-strategy#3.2 · no · the chunk changes no paste validation, and Paths 2–5 and the paste properties are Not needed
test-strategy#3.3 · yes · the hook's nine events fail open (exit 0, no body, empty stderr) on a refused home, and the statusline command must not run
test-strategy#3.4 · yes · `viola mcp`'s frame tools refuse through `live_endpoint` with `instance-unreachable` and `detail: null`, and `TestWrapper` must stand on a passing home
test-strategy#3.5 · yes · `--home` and `VIOLA_DIR` failing strict-modes give exit 1, 21 or hook 0, and the completeness case covers every `VIOLA_*` var, flag and `config.json` key
test-strategy#3.6 · yes · group- or world-writable home and instance dirs are refused, the Windows owner and DACL cases, and the single-writer guards for `snapshot.json` and `ledger/stamps.json`
test-strategy#3.7 · yes · the statusline shell-out must not run on a refused home, and the pinned exe is re-hashed before reuse
test-strategy#3.8 · yes · the build-reproducibility measurement compares the §9 release-build route, which `release-check` holds to no test-only binary or feature
test-strategy#3.9 · no · the chunk changes no parser surface, and the property suite and cargo-fuzz targets are Not needed
test-strategy#3.10 · yes · the chaos suite's file-fault leg and the E5 snapshot-corruption leg (only on the P4 fork)
test-strategy#3.11 · yes · the hook's added check must keep the spine hooks under `SPINE_DEADLINE` on hyperfine max
test-strategy#3.12 · yes · the per-surface refusal (CLI exit 21 and MCP `instance-unreachable` with `detail: null`) is the cross-surface parity to assert
test-strategy#3.13 · yes · the Windows strict-modes and DACL cases are proved on `windows-2025` and the cfg legs run on their own runner, the scope's own multi-OS obligation (S9 exception)
test-strategy#3.14 · no · the chunk adds no per-CLI-version shape or fixture, and the stamps read is a strict-modes file rule
test-strategy#3.15 · yes · the pinned-exe hash literal vectors the chunk adds are §6 Contract-suite cases
test-strategy#3.16 · yes · the scope names the error sanitization suite: refusal lines and diagnostics files (0600, no pid, path or endpoint)
