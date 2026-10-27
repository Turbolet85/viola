# P5 review card 1 — 2026-10-27-pinned-copy-race-and-reproducible-builds

Plan: `viola-0.1.0/chunks/2026-10-27-pinned-copy-race-and-reproducible-builds/plan.md` (453 lines, about 175 of them the gate fence; 12 steps; 11 acceptance
criteria; 25 gate entries). Research: `…/research.md`. Scope (the val-1 anchor, closed at P3): `…/scope.md`.

## Goal
- **The race closed.** The absent copy of `bin/<key>/viola(.exe)` is placed once, already closed, and never replaced:
  `fs::place_private_new` (temp written at its mode, synced, the handle closed, then `persist_noclobber`), used by
  `pin_bytes`'s absent-copy arm; a lost placement re-hashes the copy that stands (equal → reused, different →
  `pinned-hash-mismatch`, left as found). Mechanism re-derived at P3: viola renames while holding the write handle
  (`fs.rs:288–303`, `tempfile` `persist`), and an exec racing that window fails `ETXTBSY` (scratch control 4059 / 16283
  open-handle vs 0 / 13066 closed). Still unmeasured, and the plan's step 3: the errno of the CARRY's own failed boots.
- **Measurement B.** Two clean release builds of one tree through `scripts/release-check.sh`'s build, compared by file
  and by every section, two controls; acting on a difference is the founder's.
- **Both reds of `ci#40425866096` owned.** Red 1 (Path 1's 20 s timeout, ubuntu): owned here; it is NOT the race (one
  stamped home, stalled in step 5; 13.468 s green vs 20.089 s red under a runner-wide slow tail); phases measured, slow
  work removed, kill line unchanged. Red 2 (the Windows plain-home oracle, `strict.rs:778`): folded here on the
  operator's word (`inputs#I4`, delegate overseer1, PROVISIONAL), read by the grant's right, with a control.

## Top acceptance criteria
- The forced-window exec case green on Linux/macOS CI and the dev host, its remove-the-guard run red with `ETXTBSY`.
- A present name never replaced (`Present`, byte-identical, no temp left); the pinned copy 0700, mode before any byte.
- Hash-before-reuse under a lost placement; Path 1 step 5 still exits 1 `pinned-hash-mismatch`.
- The product race two-sided on the CARRY's rig (before with errno → 0 of 24 after; single-instance control both sides).
- Path 7 on the default two-instance pair boots ready on all three CI OSes.
- Measurement B recorded by file and section; Red 1 measured and fixed (or stop rule S2); Red 2 green on `windows-2025`.

## Touchpoints
`crates/viola-state/src/{fs,pin,strict}.rs` · `src/bin/viola-fake-agent.rs` (`spawn_os_error` on a failed hook spawn) ·
`crates/viola-e2e/tests/mcp_tools.rs` · `tests/path1_run_start.rs` · the chunk's `evidence/`. `src/cmd/run.rs`,
`src/cmd/verify.rs`, `.config/nextest.toml` and the schema stay unchanged (the guard entry holds it).

## Test commands
fmt · clippy (host and Windows target) · G1 · lint-probes · `run --unit` · the viola-state pin/placement filter · the
root integration set beside the pin · `run --e2e` (Path 1, Path 7) · `cargo deny` (+ viola-state's `deny-sync.toml` bans)
· release-check probe · the preservation guard (base `99977116`, controls in `p5-controls.md`) · the harness smoke ·
G2 · G4 · `run --perf` · `pre-push` · then the operator pass's hygiene, push and `ci.py` read. All 16 new entries
baselined green on the untouched tree (P5, HEAD `99977116182b`).

## Capabilities
None claimed: the 21 unclaimed caps (`matrix.py show --unclaimed`) hold none this chunk's acceptance proves, and the
working-route entry names none. No concretization preview.

## Sampler (this run drew `sample: yes`)
- architecture — `sampler: E carried 54 · the cascade lost 4 (conventions#4.1, occupied-resources#1.1, stack-and-technologies#1.24, standard-contracts#11.1) · listed 4 (established-decisions#25.1, occupied-resources#31.1, occupied-resources#34.1, stack-and-technologies#1.15)`
- security-plan — `sampler: E carried 48 · the cascade lost 2 (dependency-security#4.1, threat-model-summary#9.1) · listed 4 (dependency-security#2.1, dependency-security#7.1, secret-management#1.1, secret-management#2.1)`
- design-system — `sampler: E carried 8 · the cascade lost 0`
- test-plan — `sampler: E carried 69 · the cascade lost 5 (ci-integration#1.9, quality-gates-coverage-targets#3.1, test-strategy#1.2, test-strategy#3.8, unit-test-strategy#2.1) · listed 7 (ci-integration#1.12, quality-gates-coverage-targets#1.1, quality-gates-coverage-targets#2.0, test-data-fixtures#2.2, test-scope-summary#7.1, test-strategy#1.1, test-strategy#2.1)`
- obs-plan — `sampler: E carried 65 · the cascade lost 7 (metric-coverage#1.1, metric-coverage#1.2, slo-invariants-telemetry-budgets#2.2, span-trace-coverage#4.4, telemetry-strategy#1.1, telemetry-strategy#1.4, telemetry-strategy#3.12) · listed 6 (ci-integration#2.3, ci-integration#4.1, metric-coverage#2.11, pii-scrubbing-compliance#1.10, pii-scrubbing-compliance#1.13, telemetry-strategy#2.1)`
- a11y-plan — `sampler: E carried 16 · the cascade lost 0`
- layout-templates — read whole, no cascade.
Every lost item stands in the plan's Implementation notes, marked `(sampler)`.

## Decisive leans (each on the artifact named in the plan's Provenance)
- Path 7 back to the default `overseer` + `builder` pair (its comment names the race as the workaround's only reason).
- Measurement B on the Linux dev host only (the entry asks two builds of one tree; a CI comparison job is not asked).
- Red 1 owned here (it intersects: Path 1 steps 5–6 drive `pin_bytes`).
- No test seam: the forced window is a private function parameter, so no env read or seam widens the boundary.
- `cargo clean` of `target/release-check` replaced by a target dir the script makes and removes itself (P5 check 11).

## Stop rules
- S1: the base reads 0 of 24 failed two-instance boots → "not reproduced on this rig", no product red claimed.
- S2: Path 1's phases name no removable work without changing its contract → stop and report, nothing cut, no bound moved.

## Records for the operator
- **F363** (`inputs#I2`): the strong reader `tests-cascade-g3` failed `QUOTES_VERBATIM` twice; its second re-spawn, past
  the contract's one-retry limit, ran as a one-off on overseer1's word under the founder's night word, and passed
  (friction-log `2026-10-27T01:28:02Z-b`).
- PROVISIONAL, owed the founder's own word at the epoch boundary: red 2's fold (`inputs#I4`, delegate overseer1).
- test-plan's no-stub rule cited at `.andromeda/test-plan.md:717` (:718 blank, `inputs#I4`).

## Beyond the plan
- none
