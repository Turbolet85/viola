# P5 review — revision 1 — 2026-11-18-session-folder-trusted-by-viola «Session folder trusted by viola» (viola-0.2.0)

Open the plan in an editor: `.andromeda/cache/docs/2026-11-18-session-folder-trusted-by-viola/plan.md` (rev 3,
rendered from the store). No plan is kept in the chunk's folder, and editing that file changes no record until a
`put`. For a diff of the chunk's records, run
`python -X utf8 ~/.claude/skills/andromeda-tools/scripts/rec.py diff --root . 2026-11-18-session-folder-trusted-by-viola`.
This card shows only what changed since the approved plan (rev 2, card
`.andromeda/runs/2026-11-18T10-58-21Z-phase/p5-card-1.md`).

## The stop and the word
- **Stop:** `S2` (entry 14) was refused with `refused: cap` before its ledger line: no `claude` ran, and the start is
  uncounted and unspent.
  - **Cause:** the rig's `admit` counts every base start in the ledger FILE against `--cap`
    (`crates/viola-e2e/src/rig/ledger.rs:82-98`).
  - `S1` and `S2` share `evidence/live-starts.ndjson`, so `S1` had already spent the `--cap 1` on entry 14.
- **Word:** inputs#I4 (the invocation) and inputs#I5 (the overseer's revision relay).

## Changed lines
- **Entry 14:** `--cap 1 --spare 0` → `--cap 2 --spare 0`, on the same ledger. Its `note` now says that `--cap` counts
  the base starts in the shared ledger file.
- **Unchanged:** entry 12 (`S1`, `--cap 1 --spare 1`), the rig's `admit` and the founder's cap (S1 one plus one spare,
  S2 one, no spare).
- **Step 14 and the Implementation notes' cap line:** both now say «`--cap` is the count of base starts in the ledger
  FILE, so the second arm's number is the total», which is the same cap, not a wider one.
- **Step 9:** one paragraph added. Every other test-side `run` / `revive` gets a test-owned user home:
  - `support::home::unmade_user_home` in `tests/support/cli.rs` and the seven binaries;
  - the session's own `user/` for the harness's supervised wrappers.

  This work was built by the stopped run (its deviation 1).
- **`Files to modify`: 16 → 26.** The ten added files:
  - `tests/support/cli.rs`
  - `tests/chaos_snapshot_replay.rs` · `tests/cli_controls_not_disableable.rs` · `tests/cli_fake_agent.rs` ·
    `tests/cli_version_gate.rs` · `tests/cross_exit_causes.rs` · `tests/cross_killed_test.rs` ·
    `tests/channel_error_bodies.rs`
  - `crates/viola-e2e/src/harness/boot.rs` · `crates/viola-e2e/src/harness/supervise.rs`

  Why they are needed: a start under the real `HOME` would write a trust record into the operator's real config.
- **A new `## Revision 1` section in the plan** names:
  - what is done, with its evidence: entries 1–8 green, six controls holding, entries 9–13 green, and `S1` GREEN
    (ledgered 13:13:26Z, build `f058ab7040082253`);
  - the re-entry: `/andromeda-implement --live-legs --entry 14`, then entry 15, then `S2` read against `S1`. On green,
    the ref and the operator pass follow. A RED `S2` stops implement before the operator pass.
- **Scope rev 4:** two bullets were added and verified in the same pass (the `--cap` semantics, and the test-owned
  user home). No bullet was falsified.
- **Research:** the lists were synced to the revision's delta (+10 −0), rev 3. They were not re-derived since the
  take-up's rev 2. The only reads were the ten diffs, `admit`, and the `trust_config_file` sweep re-run (it hits no
  added file).

## Checks (P5 step 1)
- **Gates:** the fence parses, 22 entries. Baselines: 2 seen green, 6 `new` carried with their baselines unchanged,
  14 legs exempt.
- **Lists and sync:** `scope lists: parsed 26 items` · `scope sync: PASS · delta +10 −0`.
- **planlint:** 0 hits. Its 12 (d) listing is 5 claims, all carried from rev 2 and read as still holding.
- **Inputs:** 5 entries, 0 drifted, 0 unparsed. I3 (the implement directive) is uncited by the plan, which is
  information only.
- **Validation-1:** aligned with scope rev 4.

## matrix#v2-33
The claim stands unchanged: the same acceptance and the same chunk, with no re-concretization. Its live witness is
still `S1` (`first-screen: box`, `run-trust: written`, `entry-flag: true`, `sentinel: equal`) against `S2`
(`first-screen: trust`).

## After the word
The phase ENDS (inputs#I5 item 5). Implement resumes at entry 14 alone.

## Beyond the plan
none (standing — `.andromeda/runs/2026-11-18T10-58-21Z-phase/p5-card-1.md` held none; this revision adds none)

Apply? (yes / review / cancel)
