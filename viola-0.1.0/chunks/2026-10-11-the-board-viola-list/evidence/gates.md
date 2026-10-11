# Gates — 2026-10-11-the-board-viola-list (implement)

The plan's `[[gate]]` block through `gate.py` (v1.14), run dir `.andromeda/runs/2026-10-11T07-39-58-implement/`
(trail `gate-2026-10-11-the-board-viola-list.json`, one line per firing in `journal.md`). Linux dev host.
Entries 14 to 16 are `leg = 'operator'`: none was fired here.

## Firing 3 — the whole block on the final source (green 13 · red 0 · not-run 3)

```
  1 lint        green · exit 0 · 0.81s · cargo fmt --all --check
  2 lint        green · exit 0 · 1.71s · cargo clippy --workspace --all-targets --features fake-agent -- -D warnings
  3 unit        green · exit 0 · 11.6s · bash scripts/agent-run.sh run --unit
  4 unit        green · exit 0 · 0.72s · run --unit --filter 'test(/board_scan_|board_read_|board_row_|bay_line_|escape_row_|list_document_|budget_stamped_/)'
  5 integration green · exit 0 · 4.93s · run --integration --filter 'binary(cli_list) | binary(chaos_forward_compat) | binary(chaos_torn_append) | binary(cli_budget) | binary(cli_revive)'
  6 probe       green · exit 0 · 0.01s · the preservation guard, git diff --quiet 995dc55a3d6b -- <44 paths>
  7 smoke       green · exit 0 · cleanup --session p-board-smoke
  8 smoke       green · exit 0 · 4.37s · boot --session p-board-smoke --instance builder
  9 smoke       green · exit 0 · status --session p-board-smoke
 10 probe       green · exit 0 · bash scripts/g2-zero-panics.sh
 11 probe       green · exit 0 · bash scripts/agent-run.sh schema-check
 12 smoke       green · exit 0 · cleanup --session p-board-smoke
 13 probe       green · exit 0 · 65.07s · bash scripts/agent-run.sh pre-push
entries 16 · green 13 · red 0 · recorded 0 · timeout 0 · not-run 3
```

- Entry 4: the seven prefixes select the new inline cases: `70 tests run: 70 passed` across 9 binaries.
- Entry 5: `35 tests run: 35 passed`, every new root case among them: the ten `cli_list` cases (two
  `cfg(unix)` refusals, two `e5_` rstest cases), `chaos_lines_of_a_newer_build_are_counted_and_the_board_still_reads`,
  the torn-append case with its new `list --json` reading, and `path6_list_shows_the_reading_and_the_pause`.
- Entry 13's document: `"ok":true,"stage":"linux-tests"`; coverage suite 2406 passed / 0 failed, doctest 0,
  playwright 1 passed, gate breaches `[]`. `target/agent-run/artifacts/llvm-cov-summary.json` `data[0].totals`:
  lines 98.13 %, functions 98 %, regions 97.92 %.

## Firing 4 — `--only 1,2,5` after the last source edit (green 3)

`tests/cli_list.rs`'s `UNREACHABLE` constant took `#[cfg(unix)]` after firing 3. The Windows target's
clippy (`CARGO_TARGET_DIR=target/wincheck cargo clippy --workspace --all-targets --features fake-agent
--target x86_64-pc-windows-msvc -- -D warnings`, testing.md 2026-10-10) read it as dead code, exit 101. After the
edit it read exit 0, and so did the host clippy and `cargo fmt --all --check`. Entries 1, 2 and 5 were then
re-fired: all green, entry 5 again `35 passed`.

## The one red: firing 1, entry 3 — not this chunk's

Firing 1 ran the whole block: green 12, red 1 (entry 3, `run --unit`). One case failed:
`viola-e2e harness::cleanup::tests::remove_session_home_removes_a_boot_created_parent`, which panicked at
`crates/viola-e2e/src/harness/cleanup.rs:366:43` with `mkdir: … AlreadyExists`. That line is
`fs::create_dir_all(ws.e2e_home())` over the link `target/e2e-home`. Entry 3 re-fired alone (firing 2) was
green. The cause is measured, so the red is closed on its cause, not on the green re-run:

- The host booted at 2026-10-11 07:18:11Z (`uptime -s`), after the last wrap. The link's target is on tmpfs
  (`/tmp/viola-e2e-home-<uid>`), and a reboot empties it, so the link dangled when the block started.
- The target's birth time is 2026-10-11 07:50:10Z (`stat %w`), inside firing 1's entry 3. A parallel harness
  test re-made it through `Workspace::ensure_e2e_home` ("made again 0700 when it is gone").
- std's `create_dir_all` over a dangling link returns `AlreadyExists`, and over the same link with its target
  present returns `Ok`. A scratch probe measured both sides (`rustc`, session scratchpad). The case calls
  `create_dir_all` on the link itself, not `ensure_e2e_home`, so it fails whenever it runs before the re-make.
- This chunk edits no `viola-e2e` file: entry 6, the preservation guard over `crates/viola-e2e`, is green.

The defect is the harness case's own base preparation on the first run after a reboot. Its owner is the
wrap's to pin.

## Writes outside the block

The six remove-the-guard runs are in `evidence/red-green.md`. The chunk claims no capability
(`matrix.py show --chunk`: claimed 0), so no `ref` was written.
