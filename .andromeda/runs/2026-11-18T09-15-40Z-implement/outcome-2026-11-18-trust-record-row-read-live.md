# Outcome — 2026-11-18-trust-record-row-read-live (implement run 2026-11-18T09-15-40Z, plan rev 10, Revision 4)

**Outcome: green.** Re-entry at step 27 (inputs#I19, the operator's directive). Steps 27–29 are built and green. The
block read `entries 19 · green 7 · red 0` on its first firing. **S0B**, the base start of the founder's new cap, was
ledgered at 2026-11-18T09:21:13Z on the fresh ledger `evidence/browser-starts.ndjson` (`--cap 1 --spare 1`). It read
**`verify-exit 0 · pass-count 23/23 · failing: none · trust-record: pass`**, so the floor HELD and the `trust-record`
row is VERIFIED on 2.1.325 in the rig's private home. It ran with `keys: none` and `stop: none` (verify exited by
itself), and `kept: none`. It was not refused for the token window. The spare `S0B-2` is not spent. Smoke (operator
entries 13–15) is green.

Next → the OPERATOR PASS, which this window runs on the directive: entries 16 (`pre-push`), 17 (`gate.py hygiene`, as
written), the land, the pre-CI commit, 18 (the push) and 19 (`ci.py conclusion`, as written); its record goes in
`evidence/operator-pass.md`. Then `/andromeda-wrap-session`.

## Files changed this run (3, all in research's lists; scope: clean — changed 19 · listed 19 · recorded 0)
- `crates/viola-e2e/src/rig/isolate.rs` — `Marker::Off` (`Value::Bool(false)`). `USED_MARKERS` now holds five keys:
  `hasSeenAutoDefaultNotice`, `lastReleaseNotesSeen`, `lastClawdEntranceVersion`, `migrationVersion` and
  `claudeInChromeDefaultEnabled`. `hasSeenAutoModeEntryWarning` and `lastOnboardingVersion` are gone. The doc comment
  gives each key's 2.1.325 bundle reason. The doc comments of `seed_used_markers` and `seed_markers_word` no longer
  state a count. The case is renamed `seed_used_markers_writes_the_bundle_markers_and_keeps_the_seed` and pins the
  exact sorted key set. It also checks `claudeInChromeDefaultEnabled` reads `false` and the two dropped keys are absent.
- `crates/viola-e2e/src/rig/mod.rs` — `Arm::of_label` takes `S0B` and `S0B-2` as `V`, and its doc comment names them.
  The comment beside the marker call in `start` now names the browser-tools key. `the_arm_words_and_labels_parse`
  covers `S0B` and `S0B-2` (→ `V`) and `S0B-3` (→ none).
- `crates/viola-e2e/src/bin/viola-live-rig.rs` — the module comment lists `S0B` among the `live` labels that run
  `viola verify`.
(Steps 1–26's files from runs 01-02-09Z, 01-39-23Z, 06-24-01Z and 08-10-59Z stand uncommitted beside these.)

## Gates (trail `.andromeda/runs/2026-11-18T09-15-40Z-implement/gate-2026-11-18-trust-record-row-read-live.json`)
- Block, first firing: `entries 19 · green 7 · red 0 · recorded 0 · timeout 0 · not-run 12`, one iteration and no
  red. Entry 4 read `PASS … rig::isolate::tests::seed_used_markers_writes_the_bundle_markers_and_keeps_the_seed` and
  `PASS … rig::tests::the_arm_words_and_labels_parse`.
- No control was fired. Revision 4 names none (inputs#I17 item 7); the renamed unit case is the pin.
- Hygiene came back `clean`, then the land (`unchanged · 32 file(s)`), both before the start as step 29 asks.
- Live round (`evidence/round-092108Z.txt`): `round: COMPLETE · legs fired 1/1 · entries 19 · green 5 · red 0`.
  - The install was read by path (entry 8).
  - The census read 0 before (entry 9) and after (entry 12).
  - The build was `f654a8b51b07f334`, the same as the summary's `viola-build`.
  - All 56 of entry 11's atoms held, and the artifact `browser-starts.ndjson` was fresh.
  - The summary is recorded whole in `evidence/live-starts.md` §S0B.
- Smoke (P3), operator entries 13–15 (`evidence/smoke-operator.txt`): boot green, status `ready` (two instances
  `live`, `cli_verified: true`), cleanup green (`processes_gone`, `endpoint_gone`).
- No `gate.py guard` entry. No deferral. No matrix ref: the chunk owns no coverage row (0 rows), and no capability is
  claimed.

## The reading (`evidence/live-starts.md` §S0B)
- A: `trust` at 741 ms (by design; killed with no key).
- B–F: `box` at 1 191–1 427 ms, each `lit=footer-agents@…,footer-cycle@…,rule@…,prompt@… adj=yes first=banner col=80`.
  Run E started (a resume target existed) and read `box`.
- Set beside S0R-3, the rig differs only by steps 27–28. B–F's first screen moved from `none` at 8 500 ms (the
  browser-tools prompt) to `box`.
- The host also carries claude 2.1.327 as mise's `latest`. The start read 2.1.325 by path, as the directive names.

## A fix after the report (the operator pass, `evidence/operator-pass.md`)
- **The red:** the pass's first `pre-push` read red on
  `viola-e2e::rig_rehearsal rig_keeps_no_credential_or_host_path_in_any_record` («the tree's status moved»). The
  extra status rows were three `crates/viola-e2e/default_*.profraw` files.
- **The cause:** the seed's `cli_version` spawns the instrumented fake agent under `env_clear()` in the test's cwd.
  This is this chunk's own (Revision 1).
- **The fix:** `crates/viola-e2e/src/rig/mod.rs` `cli_version(program, cwd: Option<&Path>)`. The seed passes the
  scratch root, and the live preflight passes `None`.
- **Clean-up:** the three stray profile files were removed.
- **Re-fired:** the block read `entries 19 · green 7 · red 0`, and `pre-push` read green. The report was rewritten
  (rev 2), with the new-text listing `new-text-2.md`.
- **S0B stands:** S0B was read before this fix, and the fix changes only the probe's cwd.

## Deviations from the plan, with justifications
- None beyond the fix above. Scope record: none (`gate.py scope` `clean`, 0 recorded).

## Process hygiene
| process | started by | final state |
|---|---|---|
| `viola-live-rig live S0B`, its `viola verify` and its `claude` 2.1.325 children | a leg (entry 11) | terminated: the summary read `census: 0`; entry 12 read `census: 0`; the round reported no survivors |
| gate entries 1–10 and 12 (cargo, nextest, harness, rig census) | this run | terminated (each entry's tree censused by the tool) |
| harness session `screen-smoke` (supervisor, two wrappers, two fake agents) | this run (operator entry 13) | terminated by entry 15 (`processes_gone: true`) |
| `gate.py census --comm viola-live-rig --comm viola-fake-agent --comm viola-harness --args projects/viola/target/` (09:24Z) | — | `census: none running` |

## Decisions & corrections
- The directive was followed:
  - ONE live start, S0B, on a fresh ledger, ledgered with the clock's time first (09:21:13Z);
  - `keys: none`, with `stop:` printed beside it;
  - no spare spent;
  - no scheduled-wake or timer tool called.
- The newest install moved to 2.1.327 (`latest`). The start stayed on 2.1.325 by path, on the operator's word
  (verification-harness.md 2026-10-10, extended 2026-11-13).
- Bash guard refusals, each re-issued once in the form the guard passes:
  - a `cd` into a references dir;
  - a `cd` into a crate dir;
  - a `rec.py render` piped into `grep`.
