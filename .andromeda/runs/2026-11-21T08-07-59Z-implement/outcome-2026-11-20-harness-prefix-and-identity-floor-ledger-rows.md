# Outcome — 2026-11-20-harness-prefix-and-identity-floor-ledger-rows

**Run:** 2026-11-21T08-07-59Z-implement · re-entry at R1 after the plan's revision 1 (plan rev 10), under
`inputs#I13` (the implement directive, snapped this run).

## Outcome word: green (+ smoke)

- Phase B: entries 1-12 green (`gate-…json` of this run; entry 10 re-fired after the oracle mend, entry 12 re-fired
  after it too: both green). Cut guard `cut: whole`; name census exit 1, no output.
- Controls: `kept-names` holds, `floor-unknown` holds (control-1); `floor-grows` DID NOT HOLD at its first firing,
  held after the oracle mend (control-2). Records: `evidence/control-2.txt`, `evidence/control-3.txt`.
- Host before HG (2026-11-21T08:22:21Z): QUIET (5 s stalled, this repository's own gates).
- Round (2026-11-21T08:22:28Z, whole form): `round: COMPLETE · legs fired 1/1 · … green 8 · red 0`. `HG` green on every
  atom: `verify-exit: 0`, `identity-floor: pass`, `env-unknown: 0`, `pass-count: 28/28`, `failing: none`,
  `screen-b`..`f` box, `record: 31`, `kept-stamp: written`, `keys: none`, `census: 0`, `credentials: removed`,
  `scratch: absent`, `user-config: unchanged`. Ledger: HG base line 2, HF-2 unspent.
- After-reads: census `census: 0` (history unmoved) · kept stamp `28 ok` (`browser-tools-prompt: absent`) · env names
  `added` 13 names, `unknown: none`, `other_count: 14`, `unshaped: 0` · `live-stamp: unmoved` · name diff 2 lines.
- Landing: `evidence/landing.md` (identity read clean, 32 files). Entry 25 (`diff -r`) green.
- After-landing layers: first firing red (unit 1 fail, integration 5 fail, e2e 3 fail) — every case on a literal
  pinning the old set's recorded session id; four pins moved; re-fired green: unit, integration (594/594), e2e
  (334/334). `contract_ledger_probes_pass_over_every_stamped_set`, `env_fixtures_pass_hygiene` and every
  `contract_fake_agent_drift` replay PASS.
- Smoke (operator-keyed entries 29-31, fired by `--operator` after the landing): boot ok, status `ready` with both
  wrappers `cli_version 2.1.328`, `cli_verified true`; cleanup `processes_gone`, `endpoint_gone`.
- WATCH `cli_wheel path5_a_turn_left_running_is_cleared_by_pause_then_release`: PASS in every local integration run
  (12.log, 12.2.log, 27.2.log); not exercised in CI yet (the operator pass).
- Matrix: `v2-26` planned → implemented (ref: live HG, the canary test, `contract_ledger_probes`, the identity table).

Next → the OPERATOR PASS in this window: entries 32 (pre-push), 33 (hygiene, fired as written), the land and the
pre-CI commit, 34 (push), 35 (`ci.py conclusion`, fired as written); `evidence/operator-pass.md` lands.

## Files changed
59 files (`cites.py added`, base `b3eb184c`): research's lists (46 listed, 28 fixture files absorbed by the derived
item) and the scope record's 13 recorded lines; `gate.py scope`: clean.

## Deviations (each with its justification)
1. R1's seven files, `test_cases_framing.rs` and `contract_fake_agent_drift.rs` were brought back to their
   `b3eb184c` text by reverse-applying each file's own diff (`git apply -R`), then the 28-row literals set by hand in
   the last; the other cut files by Edit and short scripts. The plan rejects `git checkout --`; a reverse patch is an
   edit of the working file, its result read by the cut guard (entry 3: `cut: whole`).
2. `crates/viola-e2e/tests/rig_rehearsal.rs`: four `pass-count` atoms read `27/27` — stale already in the stopped
   run's tree, and outside C1's sweep pattern (which keys `29/29`). Moved to `28/28` (a listed file).
3. `tests/tui_env_strip.rs`: the `floor-grows` control did not hold — `plan_strip` strips every unkept `CLAUDE*` name,
   so an oracle keeping none cannot see the floor. The case now lists its thirteen floor names in `claude_env_keep`
   (the unknown name nowhere) and asserts none is kept; the control then held (a listed file).
4. After the landing, four test sources pinned the old set's recorded session ids; moved to the re-recorded ids
   (scope record, companion, serving `fixtures/claude/2.1.328/`): `tests/support/start.rs`, `tests/chaos_revive.rs`,
   `tests/cli_send.rs`, `src/bin/viola-fake-agent/test_cases_opts.rs`.
5. `src/cmd/verify.rs` scrub unit case: its non-env file name moved from the cut stem `cross-session-1` to `paste-2`.
6. `crates/viola-e2e/src/rig/mod.rs`: `code_or_unread` removed with the cut lines (its only reader).
7. F2 says «each of the fifteen names (thirteen floor, the unknown, none kept)»: `STRIPPED` holds fourteen entries
   (thirteen floor names and the one unknown); the count fifteen matches no list of the plan; implemented as fourteen.
8. Control evidence files are `control-2.txt` / `control-3.txt`: the stopped run's `control-1.txt` stands.
9. The operator-keyed smoke entries 29-31 were fired by this run after the landing (plan E1 lists them for no pass).

Scope record (rev 3; `gate.py scope`: clean — changed 59 · listed 46 · recorded 13): companion 10 — `Cargo.toml`,
`.config/nextest.toml`, `crates/viola-e2e/src/harness/secret_scan.rs`, `src/run/version_gate.rs`,
`tests/cli_verify/stamps.rs`, `tests/contract_fixture_hygiene.rs` (the stopped run's, self) and
`tests/support/start.rs`, `tests/chaos_revive.rs`, `tests/cli_send.rs`, `src/bin/viola-fake-agent/test_cases_opts.rs`
(this run's, self); in-intent 3 — `tests/security_negatives_env_capture.rs`,
`src/bin/viola-fake-agent/test_cases_modes.rs`, `tests/support/verify.rs` (self); widening 0.

## Surfaced / stuck
none.

## Process hygiene
| process | started by | final state |
|---|---|---|
| cargo / nextest / harness runs of entries 1-12, 25-28 | this run | terminated (each entry exited) |
| the controls' clones and their firings | this run (`gate.py control`) | terminated; clones removed (both summaries) |
| `viola-live-rig` HG and its children (verify, claude 2.1.328 runs A-F) | a leg | terminated — `census: 0` in the rig summary and entry 21 |
| harness session `floor-smoke` (supervisor, two wrappers, fake agents) | this run (entry 29) | terminated — cleanup `processes_gone: true` |

`gate.py census` (comm `viola-live-rig`, `viola-fake-agent`, `viola-harness`, `cargo-nextest`; args
`projects/viola/target/debug/viola`, `target/harness/debug/viola`, `installs/claude/2.1.328`): `census: none running`.

## Decisions & corrections
- Sweep hazard: a count literal the stopped run never moved (`pass-count` `27/27`, while `rows` went 27 → 29) escapes
  a retired-shape pattern keyed on the stopped tree's figure; read every count atom in the files the set feeds.
- Sweep hazard: a whole re-record of a fixture set moves its recorded session ids, which four test sources pin as
  literals (`FIXTURE_SESSION` in `tests/support/start.rs` and `tests/chaos_revive.rs`, the clear id in
  `tests/cli_send.rs`, the fake agent's resume case) — red only after the landing.
- A remove-the-guard control over a floor guard needs its oracle to keep the floor names: the strip removes an unkept
  name anyway (control-2 DOES NOT HOLD → held after the mend).
- Guard refusal: the Bash guard refused a `rec.py query` piped into `sed` (one re-issue, the rev read from the earlier
  `docs` view).
- The directive (`inputs#I13`): HG is the one base start; no start followed it.
