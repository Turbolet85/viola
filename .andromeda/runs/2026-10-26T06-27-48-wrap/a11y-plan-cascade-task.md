# Task — a11y-plan · 2026-10-26-tokio-rustsec-2026-0386-lock-bump

## The session
The workspace's exact tokio pin moved from =1.53.1 to =1.53.3 (Cargo.toml line 242), and the lock's one tokio entry was rewritten to version 1.53.3 with a new checksum (Cargo.lock lines 1633 and 1635). The lock delta is 4 lines, the tokio entry only.
Retired: the open advisory RUSTSEC-2026-0386 against the locked tokio. cargo deny check (all four families, cargo-deny 0.20.2) exits 0, and CI job supply-chain went from red to green on ci#40278510123 (15/15 checks).
No source, crate manifest, symbol, IPC method, CLI verb, output string, exit code, env var, test, suite, gate, lint, toolchain or workflow file changed. tokio-macros, tokio-stream, tokio-util and every other crate stayed where they were.
Measured unchanged: tokio is still built only in viola-mcp, in viola-channel's optional tokio feature and in the test-only viola-e2e. The sole-root bans, channel-tokio-free and deny-probes (13/13) are green. fmt, clippy (host and x86_64-pc-windows-msvc) and the MSRV 1.96 check are green. The harness run --unit and pre-push are green (coverage suite 3006 passed, 0 failed). Smoke boot, g2-zero-panics and schema-check are green.

## Reached files
- Cargo.toml
- Cargo.lock

## Touches
- The test-tool versions in the cli / tui tool-reach and CI stage tables (assert_cmd 2.2.2, predicates 3.1.4, trycmd 1.2.1, portable-pty =0.8.1): re-read against a lock where only tokio moved, all unchanged.
- The portable-pty =0.8.1 pin behind the tui outer-PTY boundary driver: unchanged. The pinned set changed only for tokio.
- The not-assertable entity "Claude Code plugin: hooks and the stdio MCP server; wrapper IPC channel": its Tokio runtime (viola-mcp, viola-channel's tokio feature) now resolves tokio 1.53.3. Its surface and its no-UI reason are unchanged.
- The not-assertable entity "Internal crates (modular monolith)": no crate was added, removed or changed. Only the resolved tokio version under the existing crates moved.
- The Cognitive timeout list (the MCP wait cap, the claude agents --json read and the rest): no time bound, deadline or cap value changed. They are implemented over the same code, now on tokio 1.53.3.
- The CI gate `gate --require coverage,doctest` on the per-OS test job: unchanged. The coverage suite was green both locally (pre-push) and on CI ci#40278510123.
- The CI Integration platform and matrix (ci.yml; windows-2025, macos-latest, ubuntu-latest): unchanged. The job that went from red to green is supply-chain, which is not an a11y stage.
- The suite enum (nextest-integration, nextest-e2e, coverage): no suite was added. The no-new-suite rule held.
- The P4 wheel / keystroke boundary and the P6 board output-discipline cases: no test or code changed, and the coverage suite that holds them passed (3006 passed, 0 failed).
- The SLO invariant naming the epoch-boundary mutation audit (run --mutants): not run and not changed by this chunk. No crate test moved.

## New facts
- (none)

## Untouched
- The A11y Scope Summary entities and the cli / tui surfaces (CLI verbs, viola run passthrough, the viola revive --json owed item): no verb, mode or output changed.
- The cli landmark equivalents (BAY context line, caption line, the -- UNWRAPPED - READ-ONLY -- separator, the --help group labels): no output string changed.
- Visual Design Verification (amber DIALOG, dim stale rows, bold NAME, the NO_COLOR / TERM=dumb / --json SGR rules, NAME truncation): no style code or colour-fact read changed.
- Error recovery (the unable / hint: layout, exit codes, the hint-free unknown exit 14, the --json document): no refusal, code or hint changed.
- Plain language and Consistency (the fixed copy set, kebab-case codes beside unable, the CAPTION order NAME · LIVE · STATUS · WHEEL · DIALOG · CLI): no pinned string or literal moved.
- Screen Reader Support (the manual NVDA / VoiceOver / Orca passes on P4 and P6): founder-owned, not run and not changed.
- The owed items "Exit-cause code catalogue", "Linux and macOS parity" and "Explicit session links": none of them is closed or touched by this chunk.
- The A11y Decisions Log and the a11y-plan-contracts registry keys: no a11y decision and no harness contract changed.
