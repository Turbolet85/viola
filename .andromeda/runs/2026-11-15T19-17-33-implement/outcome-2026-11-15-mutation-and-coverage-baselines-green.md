# Outcome — 2026-11-15-mutation-and-coverage-baselines-green (implement run 2026-11-15T19-17-33-implement)

**Outcome: green.** Every non-leg gate is green except entry 15 (`{ git diff --name-only 73b06303cdff -- src 'crates/*/src/*'; … } | grep -v -x 'crates/viola-e2e/src/bin/viola-hypr-tap.rs' | wc -l`), which reads `red · last line '0' ✗ (last: '6')`. That red is the plan's wording against the operator-ordered fold (inputs#I4). The six files are test-module-only edits, shown hunk by hunk inside `#[cfg(test)]` with a both-ways control in `evidence/fold-test-modules.md`. The entry's re-wording is the plan revision door's (`/andromeda-phase`), owed before the wrap's light gate. All 13 controls hold; the planted-hang pair is recorded; the timings are recorded.

Next → the OPERATOR PASS, run in this window per the invocation: entry 18 (`gate.py hygiene`), the pre-CI commit, entry 19 (the push), entry 20 (`ci.py conclusion --sha HEAD --wait 1200`). Then the wrap. No deploy is started.

## Files changed
- The plan's lists: `tests/hook_fail_open.rs`, `tests/support/home.rs`, `tests/support/watch.rs`, `.config/nextest.toml`, `tests/cli_verify.rs`, `tests/cli_verify/runs.rs`, `tests/cli_verify/stamps.rs`, `crates/viola-e2e/src/bin/viola-hypr-tap.rs` (inline test only), new `crates/viola-e2e/tests/hypr_tap.rs`.
- The fold (8 widening lines, inputs#I4): `crates/viola-channel/src/{lib,client,server,tokio_client}.rs`, `crates/viola-e2e/src/harness/cleanup.rs` and `src/cmd/client.rs` (each `#[cfg(test)]` lines only), `crates/viola-channel/tests/channel_frames.rs` and `tests/security_negatives_server_identity.rs`.

## Deviations, with justifications
1. **The fold** — the cold baseline (operator entry 13, first firing) read `viola-channel tokio_client::tests::tokio_client_server_euid_is_this_user` red on `sun_path` under the scratch `TMPDIR`. It is the same class as item 1 and predates the chunk. The operator chose «Fold it in now (widening)» with the overseer's four bounds (relay `.andromeda/runs/2026-11-15T19-17-33-implement/relay-1.md`, inputs#I4). Every `viola-test-chan-` socket that was joined under `tempfile::tempdir()` moved to a short `/tmp` root (`vs-`), in one pass. The re-fired cold baseline reads 3286/3286 passed.
2. **Entry 15 red on its wording** — the plan's probe counts files, so test-module edits read as product changes. /implement may not edit the plan, so the re-wording is owed to `/andromeda-phase`'s revision door (bound 2 of inputs#I4). The replacement probe (`test_hunks.py`) and its both-ways control are recorded in `evidence/fold-test-modules.md`.
3. **Step 5's sweep, one hit left** — `crates/viola-pty/src/lib.rs:614`'s test-module comment «Below the nextest `mutants` profile's 10 s kill» (`CHILD_WITHIN` 7 s) is stale wording. It is left no-change: it sits outside every list and outside the fold's word, which covers socket sites, and its 7 s bound still sits below 15 s. It is carried to the wrap as a wording drift.
4. **Control i1's edit site** — the plan names «the socket directory reverted to `tempfile::tempdir()`». The control edits `SocketRoot::new`'s `.tempdir_in("/tmp")` → `.tempdir()`, because `serve` and the bound test both take `SocketRoot::new()` (two identical `old` sites in `hook_fail_open.rs`). Both named cases read `failed`.

Scope record (`gate.py scope`: `scope: clean — changed 17 · listed 9 · recorded 8 (companion 0 · mechanical 0 · in-intent 0 · widening 8) · absorbed 0 · excluded 222`): 8 × widening, each with word: "Fold it in now (widening) …" — the operator, 2026-11-15 (its notes the overseer's answer, inputs#I4).

## Guard entries
None (the block holds no `gate.py guard` entry).

## Process hygiene
| process | started by | final state |
|---|---|---|
| cargo / nextest / test binaries of every gate entry (block runs 1–2) | this run (through gate.py) | terminated (census: none of this repository's binaries live) |
| the 13 controls' firings in `~/.cache/andromeda-control/e53b09b0` | this run (gate.py control) | terminated; clone removed |
| the planted-hang pair in `~/dev/projects/viola-mutants-scratch/hang-mhCgsY` | this run | terminated |
| operator entries 13 (×2) and 14 | this run (gate.py --operator) | terminated |
| `viola` ×7 (the bridge `~/.local/viola-product/bin/viola`, viola-lab prototype `run`/`wait` sessions, `~/.viola/bin/0.1.0-…/viola mcp`) | NOT DERIVED — other trees and sessions | left running: not this run's — their owners |

Census: `gate.py census … --args projects/viola/target …` → `census: 7 running`, none from this repository's `target/`.

Left on disk for the operator to remove: `~/dev/projects/viola-mutants-scratch/cold-*` (two copies with their targets) and `~/dev/projects/viola-mutants-scratch/hang-mhCgsY`.

## Decisions & corrections
- The operator's word (inputs#I4, 2026-11-15): «Fold it in now (widening)». The overseer's bounds: cfg(test) lines only; entry 15 re-worded through the plan's revision door; a workspace sweep of `viola-test-chan-` sockets under `tempfile::tempdir()`; the cold baseline re-fired.
- Sweep hazard: a sun_path sweep keyed on the audit's one casualty (`tests/hook_fail_open.rs`) missed five sibling sites. The workspace grep `git grep -n 'viola-test-chan-' -- '*.rs'` found 8 sites, one of them red cold.
- Guard refusals: the bash guard refused `cd` into the skill's references dir (re-issued with an absolute path) and a `cat` heredoc to a file (re-issued as a Write-tool script).
- Host load was 18–164 through every timing reading (another project's `cargo test --workspace`). Each reading carries its load.
