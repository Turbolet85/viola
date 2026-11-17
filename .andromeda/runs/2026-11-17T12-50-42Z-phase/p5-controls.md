# P5 controls — 2026-11-17-supply-chain-green-and-booted-tests-end

Clone: `git clone --shared` of the project at 4ef8f4b90d46e4be5de6afb9826a385dbda51b3e, at
`$TMPDIR/claude-1000/-home-turbolet-dev-projects-viola/964d0c0c-1213-4ae9-9a58-7dbaec43245b/scratchpad/clone-p5`.
Each entry fired as itself: `gate.py run --chunk 2026-11-17-supply-chain-green-and-booted-tests-end --docs-root . --entry {n} --project-root {clone}`.

## Unmodified clone (must read as the tree read)
- entry 3 `cargo deny check` — red · exit 0 ✗ (exit 1) · the tree read red the same (RUSTSEC-2026-0543)
- entry 5 the `tomllib` ignore-id read — red · last line `RUSTSEC-2026-0457 RUSTSEC-2026-0461 RUSTSEC-2026-0462` · the tree read the same

## Must-pass input: the clone's `deny.toml` with `{ id = "RUSTSEC-2026-0543", reason = "probe" }` added to `ignore`
- entry 3 — green · exit 0 · its atom `advisories ok, bans ok, licenses ok, sources ok` holds
- entry 4 `cargo deny --manifest-path fuzz/Cargo.toml check advisories sources` — green · exit 0
- entry 5 — green · last line `RUSTSEC-2026-0457 RUSTSEC-2026-0461 RUSTSEC-2026-0462 RUSTSEC-2026-0543`

The fail side of entries 3 and 5 is the unmodified clone above (the id absent).
