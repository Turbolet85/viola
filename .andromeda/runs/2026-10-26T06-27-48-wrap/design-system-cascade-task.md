# Task — design-system · 2026-10-26-tokio-rustsec-2026-0386-lock-bump

## The session
The session changed one dependency: the workspace's exact tokio pin moved from =1.53.1 to =1.53.3, with default-features = false kept, and the lock's tokio entry and checksum moved with it. No other crate moved.
It retired the open advisory RUSTSEC-2026-0386 against the locked tokio. cargo deny check (all four families) now exits 0 with no ignore widened, and CI's supply-chain job reads green on ci#40278510123.
It measured that tokio is still built only in viola-mcp, in viola-channel's optional tokio feature and in the test-only viola-e2e. tokio 1.53.3 declares the same dependency set as 1.53.1, and viola calls none of the four AsyncFd constructors that 1.53.3 deprecates.
No source file, symbol, IPC method, env var, CLI verb, output line, exit code, crate, schema, config, lint, toolchain or workflow file changed. No new external surface or UI element was added.

## Reached files
- Cargo.toml
- Cargo.lock

## Touches
- Any entity that states tokio's version as current truth: it moves from 1.53.1 to 1.53.3. The report's tokio-ver sweep found no design-system site, so a hit here would be a site the sweep missed.
- Any entity that names the advisory RUSTSEC-2026-0386 as open, or the supply-chain CI job as red: both are closed now. The report's sweep found no design-system site.
- Any entity that names the route entry tokio RUSTSEC-2026-0386 lock bump as the owner of a pending item: that entry is now done, and the item it owns is closed.
- Indirect, through tokio's unchanged dependency set: the cli Toolkit claim that no dependency reads a colour or terminal variable, and that the SGR module adds no dependency. Re-read only: the dependency graph outside tokio's own entry did not move.
- Indirect, through tokio's unchanged containment in viola-mcp: the cli decision-order rule that viola mcp writes only MCP frames on stdout, and the platform note that mcp is a hidden verb with no human design surface. Re-read only: viola-mcp's behaviour is unchanged.
- Indirect, through the unchanged dependency versions next to tokio: the cli Toolkit's clap 4.6.7 and windows-sys 0.61.2 pins. Re-read only: neither moved, and the lock delta is the tokio entry alone.

## New facts
- (none)

## Untouched
- The Color Palette detail (core colors, surface scale, text hierarchy, semantic and domain status colors, border progression): a 0.2.0 frontend design input that a lock bump does not reach.
- Typography, Spacing, Border Radius, Motion, Iconography and the Token Test: frontend tokens and validation procedure, with no dependency version in them.
- Brand Identity and its expression-motion values: no readback, wheel, budget or strip behaviour changed.
- The cli tokens: amber DIALOG, dim stale rows, bold NAME and the colour depth choice. No SGR byte or terminal-variable read changed.
- The cli component patterns: the viola list strip table, the viola send readback mirror and its hints, wait / last output and its escaper, the wheel, link and gate verb lines, the plugin install lines, and the verify step counter. No output line changed.
- The cli exit-code phraseology and the viola --help rows: no exit code, verb or help row changed.
- The cli decision order and the stream rules (stdout and stderr split, --json documents, width and truncation): unchanged, since no source file changed.
