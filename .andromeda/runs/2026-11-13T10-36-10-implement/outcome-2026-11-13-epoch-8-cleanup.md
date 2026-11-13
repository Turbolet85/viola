# Implement outcome — 2026-11-13-epoch-8-cleanup

**Outcome: green + smoke.** Gate block green on the final tree (firing 2: `entries 33 · green 29 · red 0 · recorded 1 ·
timeout 0 · not-run 3`; after the last test-support edit, `--only 2,3,7,12,14,15,30`: `green 7 · red 0`). The 25
survivor controls hold. Smoke ✓. Matrix: `v2-01` and `v2-06` → `implemented`. Next → the OPERATOR PASS (entries 31
hygiene · 32 push · 33 `ci.py`), then the wrap. The project holds at the seam before the wrap (the operator's
directive for this run).

## Files changed (47: 34 modified · 13 new, every one in research's lists — `gate.py scope`: `clean — changed 47 · listed 47 · recorded 0`)
- Shared scanner: `tests/support/source_scan.rs` (new); `tests/contract_error_bodies.rs`, `tests/contract_bounded_readers.rs`,
  `tests/contract_os_gated_tests.rs` (hand scanners gone; the fake agent's module dir left out of both `product_files`).
- Merges: `src/cmd/hook.rs` (`both_hook_paths_refuse_inside_hook_handle`), `src/run/mod.rs` (`#[cfg(test)] first()`),
  `src/run/links.rs`, `src/run/snapshot.rs`.
- Splits: `crates/viola-agent-claude/src/ledger/test_cases.rs` → + `test_cases_{stamps,checks,runs,framing}.rs`;
  `src/bin/viola-fake-agent.rs` → + `src/bin/viola-fake-agent/{agents,input,statusline,test_cases,test_cases_opts,test_cases_modes}.rs`;
  `tests/cli_verify.rs` → + `tests/cli_verify/{stamps,runs}.rs`. Test names and `#[case]` rows equal to HEAD's
  (111 tests / 140 rows · 37 / 0 · 33 / 15); tokei entry `0`.
- Dead code: `crates/viola-state/src/strict.rs` (`check_present`), `.claude/docs/services/viola-state.md` (through
  `rec.py harness land`, rev 6, tx 932).
- Survivor tests: `crates/viola-state/src/{fs,pin}.rs`, `crates/viola-mcp/src/{lib,stdin}.rs`,
  `crates/viola-e2e/src/harness/mod.rs` (`writable_bytes`), `crates/viola-e2e/src/bin/viola-mcp-tap.rs`,
  `crates/viola-e2e/tests/mcp_tap.rs` (new).
- Homes: `tests/support/home.rs` (`private_dir`, and `SocketRoot::made` through it) + the 14 files (30 calls).
- Cookie: `crates/viola-e2e/src/harness/secret_scan.rs`. CARRY: `crates/viola-e2e/src/harness/run.rs`.

## Gate results (trail `gate-2026-11-13-epoch-8-cleanup.json`)
- Firing 1: red at 15 (`run --e2e`: `source_scan … case_12_escaped_char_literal`, `left: "c =    ';"`) and 30
  (`pre-push`: `case_02_nested_block_comment`, `left: "a                   e"` vs my literal). Firing 2 green.
- Counts: `viola-agent-claude` 608 passed · `viola-state` 285 (283 + 2) · `viola-mcp` 87 (85 + 2) · `viola-e2e`
  302 (291 + 11); census binaries 84/84; release-check `release-check: viola only`; probes 16–21 green, 19 every
  file `:0`; entry 1 recorded `tmpfs 32G 9.6G 22G 31% /tmp`.
- No deferral: every entry ran. No `gate.py guard` entry in the block.

## Survivor controls (`evidence/survivors.ndjson`, `evidence/control-1.txt`, `evidence/control-2.txt`)
- Firing 1 (`evidence/controls.toml`, 25 controls, entries 9 · 10 · 11, `--timeout 900`):
  `control: 4 holds · 0 timeout · 0 does not hold · 21 unreproduced · clone removed`.
- Firing 2 (`evidence/controls-e2e.toml`, the 21 e2e controls on entries 12 · 15, edits unchanged):
  `control: 21 holds · 0 timeout · 0 does not hold · 0 unreproduced · clone removed`.
- Each removed firing read red on this chunk's own witness (named per site in the ledger). Clones under
  `~/.cache/andromeda-control/…` (home disk, 328 G free), removed — items 7 and 8.
- `stdin.rs:15` `CHUNK` recorded equivalent with its reason; `strict.rs:192` left with the deleted function.

## Deviations from plan (with justification)
1. **Step 7, `pin.rs:91`** — the plan's case (a directory at the name → `Err`) does not separate the `NotFound → true`
   mutant: tempfile 3.27 persists by `renameat2(RENAME_NOREPLACE)`, so the directory reads `EEXIST` → `Present` →
   the same `EISDIR` from `file_key`. The case also pins `bin/<key>/`'s mtime to a fixed instant and asserts it
   stands (the mutant makes and removes a temp file there). Control `state-pin-91` holds.
2. **Step 7, `lib.rs`** — viola-mcp has no "scoped capture" and no `tracing-subscriber` dev-dependency: the case
   carries a minimal scoped `tracing::Subscriber` in its own test module (no global install, no manifest change).
3. **Step 5(c)** — the two `cfg(unix)` cases of `tests/cli_verify.rs` (and their `cfg(unix)` helpers) stay in the root
   file: `contract_os_gated_tests` walks only files directly under `tests/`, and their `GATES` rows would read
   `no longer stands`.
4. **Step 5(b)** — the fake agent's moved items take `pub(super)` on their signatures (bodies unchanged); its tests
   module (853 code lines) split again into `test_cases_{opts,modes}.rs`; rustfmt re-indented the moved test
   bodies by one level (no string literal changed).
5. **Step 8** — entry 11 cannot reproduce in `gate.py control`'s clone (it builds no root-package bin;
   `harness::supervise`'s cases spawn `target/debug/viola-fake-agent`: `doesn't exist on the filesystem`). The 21
   e2e controls were fired again on 12 (`run --unit`) and 15 (`run --e2e`), the layers carrying each witness.
6. **Step 9** — the call `tests/cli_verify.rs` held moved into `tests/cli_verify/stamps.rs` with the split; it was
   re-homed there. The plan's entry 19 greps `tests/cli_verify.rs` alone and reads `:0` either way.
   `SocketRoot::made` (`tests/support/home.rs`) takes the helper too, so the support file holds the one call.
7. **Fold (`inputs#I1`)** — the shared scanner's escaped-quote case found a defect the HEAD census scanners carried:
   `char_literal_len` measured `'\''` as closing at the escaped quote. It now searches past the escape; census
   verdicts unchanged (84/84; the one product `'\''`, `src/human/verbs.rs:117`, masked the same before).
8. **`v2-06`** — its acceptance names `unreadable` among the classes hitting a planted case; none was planted.
   `secret_scan_flags_a_file_it_cannot_read` added (the sentence now true as written).

## Where the plan, a letter or a row got it wrong
- Plan step 7's `pin.rs` mechanism basis (deviation 1) and its "crate's scoped capture" (deviation 2).
- Plan step 8's control entry for viola-e2e (deviation 5).
- Plan entry 19's file list after step 5(c) (deviation 6): a probe blind to the moved file.
- Plan / `v2-06`: `rec.py find --root . --doc test-plan --text cookie` is case-sensitive and finds **1** row
  (`contract/secret-scan-class/cookie`); the four rows the plan counts need `--regex "(?i)cookie"`:
  `contract/secret-scan-class/cookie`, `decision/logs-carry-no-secrets`, `decision/no-secrets-in-logs`,
  `decision/secrets-never-in-logs` — the Expected amendments the wrap lands.
- My own: three miscounted scanner case literals (firing 1's pre-push red), fixed to the scanner's output.

## Surfaced / stuck
None stuck. For the wrap: the Expected amendments the plan lists (test-plan · security-plan · obs-plan cookie
rows; test-plan §2 shared helpers; architecture's directory tree, which also gains the fake agent's `agents`,
`input`, `statusline` modules).

## Process hygiene
| process | started by | final state |
|---|---|---|
| gate block, controls (2 clones), targeted re-run | this run | terminated — the tool's own tree termination; both clones `removed` |
| smoke session `p3-smoke` (supervisor, wrapper, fake agent) | this run | terminated — `cleanup ok:true, processes_gone true, endpoint_gone true, killed []` |
| `viola` ×7 in the census (bridge `~/.viola/bin/…/viola mcp`; viola-lab prototype `viola run` ×4 and `wait` ×1; `~/.local/viola-product/bin/viola wait`) | NOT DERIVED — none under this repository's `target/`, all older than this run | left running: not this run's — the operator's |

## Hygiene and land
- `gate.py hygiene` first read `refused 1 files — P1 1` (`.andromeda/runs/2026-11-13T07-19-39-phase/p5-card-1.md:143`,
  `tmp`: the phase card's prose spelled the gate log dir by its host temp path); rewritten to the contract's printed form
  `$TMPDIR/andromeda-gate/…`, nothing else changed; re-read `hygiene: clean`. `rec.py land`: `5 new · 0 changed ·
  0 gone · 5 kept · tx 938`.

## Decisions & corrections
- The operator's directive for this run: no live `claude` start and no cargo-mutants run; every `gate.py` control
  under its 900 s kill deadline; each long run journaled in the run dir before waiting
  (`journal.md`); the operator pass with the `ci.py` read; STOP before the wrap.
- Guard refusals, one re-issue each: bash-guard refused a `cd` into a crate dir and a `rec.py` read piped into
  `head`.
- The stamp-ahead hook refused an estimated time in `journal.md` (`2026-11-13T10:56Z`, clock 10:51:58Z),
  corrected from `date -u` — a recurrence of the standing learning that a timestamp is read, never estimated.
- Delegate word carried from the phase (`relay-2.md`): `fn digit_runs` is not folded here; it goes as a CARRY onto
  «Rows-proposal rule work» at this chunk's wrap.
