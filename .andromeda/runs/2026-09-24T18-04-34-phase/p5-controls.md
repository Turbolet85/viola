# P5 known-positive controls — 2026-09-24-epoch-1-cleanup

Each control copies a file into the run dir with a planted fault, runs the gate's own grep against the copy, then deletes the copy. The
`.rs` copies were removed so they cannot enter the chunk diff as untracked Rust sources. The readings were taken 2026-09-24 at HEAD 9df9e45.

| gate | planted input | reading | verdict |
|---|---|---|---|
| `grep -c 'take(MAX_FRAME)' src/obs.rs` | `src/obs.rs` with `file.take(MAX_FRAME).read_to_end` → `file.read_to_end` (sed) | `0`, exit 1 | fails as required (`expect` = exit 0, last line 1) |
| `grep -c '#!\[allow(clippy::print_stdout, clippy::print_stderr)\]' src/bin/viola-fake-agent.rs` | fake agent with the crate-level allow line deleted | `0`, exit 1 | fails as required |
| same | fake agent with the allow turned into a non-crate `#[allow(…)]` | `0`, exit 1 | fails as required (a function-level move is caught) |
