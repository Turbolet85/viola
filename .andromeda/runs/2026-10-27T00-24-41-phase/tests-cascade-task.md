# Task — tests · 2026-10-27-pinned-copy-race-and-reproducible-builds

## The chunk
Closes the pinned-copy race: two first `viola run`s of one version on a fresh home must not both replace `bin/<key>/viola(.exe)`, and no agent's hook spawn of the pinned path may fail because another start's write of it is in flight.
The fix lives at the absent-copy arm of `viola_state::pin::pin_bytes` / `pin_exe`, reached from `run`'s start and `viola verify`, and rests on a mechanism P3 or a two-sided reproduction establishes (the `ETXTBSY` reading is only a hypothesis).
A two-sided witness: the race reproduced red before the fix and read clean after it on the same rig, beside the single-instance control, and unable to pass vacuously.
Measures, and changes nothing on its basis: whether two clean release builds (each from an empty target dir) of one committed tree give one file, and where they do not, which ELF sections and notes differ. Any build or product change it suggests is the founder's, raised as a P4 fork.
Re-asserts that the hash-before-reuse rule, the per-start plugin rewrite and the home-tree strict-modes check stand unchanged.

## Reached files
- crates/viola-state/src/pin.rs · edits pin_bytes, pin_exe (the absent-copy arm's replace_private_shared call)
- crates/viola-state/src/fs.rs
- src/cmd/run.rs · edits the start's pin_bytes call
- src/cmd/verify.rs · edits the pin_exe call
- scripts/release-check.sh · re-reads its release build of viola (the profile and CARGO_TARGET_DIR the measurement uses)

## Needs
- The pinned copy `bin/<version>-<hash>/viola(.exe)` and its absent-copy write on a fresh home under two concurrent first starts of one version
- `replace_private_shared` / `replace_private` (temp file written, `sync_all`, `persist`): what it holds open across the rename, and what a second start's replace does to a path the first start's agent may already be spawning
- The fake agent's hook spawn of the absolute pinned `command` from the plugin files (direct spawn, no shell), its `hook` receipt (`ran:false` on the failed spawn) and the missing `session-start` record: the race's observable signal
- The two-sided witness: an unstamped two-instance harness boot repeated (the measured 6 of 24 red), or a narrower concurrent-pin control, red before the fix and clean after on the same rig, beside the 0-of-24 single-instance control; the race window must be shown reachable by the control
- The harness `boot` of an unstamped session with two instances and its session record: the rig the CARRY's measurement used
- Path 7's test-side workaround (one instance booted; a stamped home pinned by `viola verify` first) stands; whether Path 7 returns to two instances is P4's
- Path 1 start sequence: the pinned copy and the plugin folder precede the child spawn; a tampered pinned exe still exits 1 and is never overwritten
- Hash-before-reuse: a present copy re-hashed (truncated SHA-256) and refused on any difference; the fix may not turn a mismatch into an overwrite
- The filesystem negatives over the pinned copy stay green: a widened pinned copy or `bin/` makes `run` exit 1 before any log, and a pinned-exe hash mismatch exits 1
- The private-write rule the fix must keep through `fs.rs`: the temp file's mode set before any byte is written, the pinned exe 0700, dirs 0700, every replaced file through the one helper
- `plugin/<key>/` rewritten whole at every start through the same `replace_private_shared`: whether a concurrent start's rewrite can fail a reader of `hooks.json` / `.mcp.json` (P3's to read)
- Windows x64 `pin_companions` on `bin/<key>/conpty/` already waits out `ERROR_SHARING_VIOLATION` (measured 5–11 ms); whether the exe's arm on Windows needs the same, read on the CI runner only, with the `seed_conpty` carve-out for Windows x64 test starts
- A race witness under the zero-flakiness rule: it is deterministic or it is red. No retry, no `#[ignore]` parking and no sleep-based synchronisation; its verdict comes from logged events, receipts and exit codes
- Measurement B: two clean `--release` builds of `viola` from empty target dirs of one committed tree, compared by file sha256 and by section (`.text`, `.rodata`, `.data`, `.data.rel.ro`, `.eh_frame`, `.gcc_except_table`, the build-id note, local symbols). It runs on the Linux dev host at least; whether CI's three OSes are read too is P4's
- Measurement B is a local measurement kept in the chunk's `evidence/`, not a test layer and never a CI step. It records no cause it has not measured
- Which readers key on the pinned hash (the `bin/<key>/` and `plugin/<key>/` dirs, stamps or snapshots keyed by it): P3's read, the product reason a rebuild that reads as another binary matters
- The contract suite's literal vectors that pin the pinned-exe content hash to its library stay unchanged

## Standing conventions
- S1 · Test functions named snake_case `<subject>_<condition>_<expected>`, critical-path scenarios `path<N>_<slug>` · .andromeda/test-plan-detail/test-strategy.md
- S2 · Integration binaries named with the `<surface>_<topic>.rs` layer prefix (`cli_`, `chaos_`, … in root; `path_`, `cross_` in `viola-e2e`) so nextest filtersets select them · .andromeda/test-plan-detail/test-strategy.md
- S3 · Every test gets a fresh home of its own at a not-yet-existing path under `target/e2e-home`, made through `create_private_dir` when the test makes it; started wrappers sit in a `Drop` guard that kills and waits; nothing is shared · .andromeda/test-plan-detail/integration-test-strategy.md
- S4 · No sleep for synchronisation, waits keyed on `events.ndjson` byte offsets or logged events, nextest `retries = 0`, no `#[ignore]` parking · §2 Test Strategy and §10 Quality Gates of the body
- S5 · Table-driven cases use labelled rstest `#[case]`s; `viola-e2e` uses a labelled case table, `viola-core` literal tables · .andromeda/test-plan-detail/unit-test-strategy.md
- S6 · Env set per child with `Command::env`, never `set_var`; an `.env_clear()` re-adds `LLVM_PROFILE_FILE`; a test-started `viola` drops `VIOLA_NAME` / `VIOLA_DIR` / `VIOLA_BIN` · §11 Test Anti-Patterns and .andromeda/test-plan-detail/integration-test-strategy.md
- S7 · Per-OS coverage thresholds hold, with the ignore regex unwidened · .andromeda/test-plan-detail/quality-gates-coverage-targets.md
- S8 · OS-branch code is verified only on its own CI runner, and a Linux-only local run is no proof for Windows or macOS · .andromeda/test-plan-detail/ci-integration.md
- S9 · No hand-written `stamps.json`, `snapshot.json` or `budget.json`: stamps come from `viola verify` against the fake agent · .andromeda/test-plan-detail/test-data-fixtures.md
- S10 · Live-appended files are read through the complete-lines reader `tests/support/ndjson.rs`, and helpers are shared in `tests/support/`, never copied · .andromeda/test-plan-detail/test-strategy.md

## Not needed
- Paths 2–6 (confirmed `send`, `wait` / `last`, dialog `answer`, the wheel and `release`, the budget governor): the chunk changes no send, readback, dialog, wheel or budget behaviour
- MCP stdio surface, its handler tests and the recorded `Mcp.` exchanges: no MCP tool or frame changes
- E2 R8 strip, E3 `link` / `unlink`, E5 snapshot replay recovery: untouched scenarios
- `viola list` board, unwrapped rows, `list` row integrity and the `claude agents --json` join: no board change
- Property and fuzz parser targets: the chunk adds or changes no parser
- Mutation gate: judged at the epoch boundary through the code audit, not per chunk
- Unix socket and socket-directory hardening, the client euid check and macOS server verification: «Unix endpoint and home hardening»'s
- Bounded-reader cases (`take(MAX_FRAME)` on external readers): «Bounded inputs at every boundary»'s
- Release packaging and the version's gates on three OSes: «Version done-check»'s
- The exit-cause matrix (`cross_exit_causes.rs`, not built): owed to «Exit-cause code catalogue»
- Hook fail-open matrix, statusline pass-through and `budget.json` writes: no hook decision or statusline path changes
- `viola verify`'s live probe runs, the ledger rows and the real-CLI fixture refresh: only verify's `pin_exe` call is reached, not its probes
- ConPTY vendor verification (`conpty-vendor.sh`) and the vendored binaries' pins: the companions' bytes and pins do not change
- Cross-surface CLI/MCP parity and human-mode SGR layouts: no output surface changes
