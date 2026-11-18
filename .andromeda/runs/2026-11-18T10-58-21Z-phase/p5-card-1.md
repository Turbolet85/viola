# P5 review — 2026-11-18-session-folder-trusted-by-viola «Session folder trusted by viola» (viola-0.2.0)

The plan to open in an editor: `.andromeda/cache/docs/2026-11-18-session-folder-trusted-by-viola/plan.md` (rev 2,
rendered from the store). No plan stands in a chunk's folder. Writing in that file changes no record until a `put`.
`python -X utf8 ~/.claude/skills/andromeda-tools/scripts/rec.py diff --root . 2026-11-18-session-folder-trusted-by-viola`
is a diff of the chunk's records.

## Goal
A session viola starts in a folder the CLI never trusted opens at its input box with no trust dialog and no key typed
by viola. The write point is `viola run` and a passed `viola revive`, which share one `start`.
- **When it writes:** directly after the version gate and before the bind, on a verified CLI only.
- **How it decides:** viola reads the child's Claude config the CLI's way (the exact key, then the parent walk).
- **What it writes:** when nothing trusts the folder, ONLY `projects[<the CLI's key>].hasTrustDialogAccepted = true`:
  - under the CLI's lock when free;
  - by atomic replace that keeps the file's mode;
  - read back exact, at most 3 attempts.
- **F-T1:** an entry that stands without a true flag gets the flag alone; every other key of it is kept.
- **Log:** one closed `trust-record` word per start in the run log, nothing of the file.
- **Stops:** a refused file form, a lost record, an unverified CLI, a trusting parent and the home folder each write
  nothing and leave the dialog to the human.
- **Live proof:** `S1`, with its control `S2`, on 2.1.325 under the founder's cap.

## Top acceptance criteria (8 in the plan)
- **v2-33:**
  - on the fake agent: `run_in_an_untrusted_folder_writes_the_record_and_types_no_key` and
    `revive_writes_the_record_for_its_recorded_cwd`;
  - live: `S1` (`first-screen: box` · `run-trust: written` · `entry-flag: true` · `sentinel: equal`) against `S2`
    (`first-screen: trust`).
- **Each refusal writes nothing:** absent, link, not regular, malformed, oversize, lock held, unreadable, foreign, lost.
  The bytes, the lock and the siblings stay as found.
- **No record in these cases:** an unverified CLI, the flag already true, a trusting parent, the home folder.
- **F-T1:** `run_with_the_entry_flag_false_sets_only_the_flag`.
- **The kept-dir file:** `run_with_the_config_dir_kept_writes_that_file`.
- **Log and terminal:** one schema-clean `trust-record` line, and no viola byte on the terminal.
- **Controls:** six, each «holds»: `trust-nocall` · `trust-gate` · `trust-walk` · `trust-flagonly` · `trust-exact` ·
  `trust-home`, each `--timeout 900`.

## Touchpoints (16 files to modify, no new file)
- **Product:** `crates/viola-agent-claude/src/trust.rs` · `src/trust_record.rs` · `src/cmd/run.rs` ·
  `src/cmd/verify.rs` · `crates/viola-core/src/obs.rs` · `schemas/diag-line.v1.json`
- **Fake agent:** `src/bin/viola-fake-agent.rs` · `src/bin/viola-fake-agent/test_cases_opts.rs`
- **Tests:** `tests/support/home.rs` · `tests/run_cli.rs` · `tests/cli_revive.rs` · `tests/tui_passthrough.rs`
- **Rig:** `crates/viola-e2e/src/rig/mod.rs` · `crates/viola-e2e/src/rig/isolate.rs` ·
  `crates/viola-e2e/src/bin/viola-live-rig.rs` · `crates/viola-e2e/tests/rig_rehearsal.rs`

## Test commands (22 entries)
- **Lint:** fmt · clippy (both seen green at the last wrap).
- **Unit:** `cargo nextest run -p viola-agent-claude` (baseline green 624) · `run --unit` (green 2727).
- **Integration:** the named binaries `run_cli` · `cli_revive` · `tui_passthrough` · `cli_verify` ·
  `contract_bounded_readers` · `contract_error_bodies` · `contract_diag_schema` (green 103, 53.1 s); then
  `run --integration` (green 531).
- **E2E:** `run --e2e` (green 320).
- **Probe:** `release-check` (green).
- **Live round:**
  - the install by path (`2.1.325 (Claude Code)` read at P3);
  - census → build hash → **S1** (`--cap 1 --spare 1`) → census → **S2** (`--cap 1 --spare 0`) → census.
- **Operator:** `trust-smoke` boot, status, cleanup · pre-push · hygiene · push · `ci.py --wait 1800`.
- No mutation run.

## The live starts (inputs#I2 items 1–2, 6–7)
- **S1, one rig start, 8 `claude` processes.**
  - Step 1: a live `viola verify` in a trusted git folder (1 print probe, 6 interactive, 1 agents read). Step 2
    starts only on `step1-verified: yes`: every relied row reads `pass`, which contains the 21-row floor and
    `trust-record: pass`.
  - Step 2: `viola run` in a fresh untrusted folder, not a repository, with the sentinel kept in memory.
  - **Keys:** none.
  - **Stop:** Ctrl-C into its own pty only with the box read, the no-key group kill with a dialog. `keys:` and `stop:`
    are printed side by side.
  - **Spare:** `S1-2`, only on a cause S1 itself showed, after a mend through a plan revision.
- **S2, the control:** plain `claude`, same folder shape. **Keys:** none. **Stop:** always the no-key group kill.
  **Spare:** none.
- **Both arms:** seed `claudeInChromeDefaultEnabled: false` and the sentinel. viola writes only the rig's private
  config. The founder's `~/.claude.json` is read only by the mark witness.
- **«Exactly the one key» (inputs#I2 item 6):** read at the writer's read-back (`run-trust: written`, the exact
  predicate). At the first screen only the flag still standing is read (`entry-flag: true`).
- **Ordering:** each start is ledgered with `date -u` first. A token-window refusal is uncounted, with no timer. A
  further start is the founder's.

## Concretization preview — matrix#v2-33 (method e2e)
- **Current acceptance:** «A session viola starts in a folder the CLI has never trusted opens at its input box with no
  trust dialog shown and no key typed by viola, and the user's own trust of other folders is unchanged.»
- **Proposed concrete form:** «A session viola starts — `viola run`, or a `viola revive` whose preflight passed — in a
  folder the CLI has never trusted, on a CLI version whose stamp holds every relied ledger row `pass`, opens at its
  input box with no trust dialog shown and no key typed by viola, viola having written exactly
  `projects[<the CLI's key>].hasTrustDialogAccepted = true` into the config file the child reads, and every other
  folder's entry unchanged byte for byte. Proven:
  - on the fake agent by `run_in_an_untrusted_folder_writes_the_record_and_types_no_key` and
    `revive_writes_the_record_for_its_recorded_cwd`;
  - live on claude 2.1.325 by `S1` (`first-screen: box`, `run-trust: written`, `entry-flag: true`,
    `sentinel: equal`) against the control `S2` (`first-screen: trust`).»
- **What the concrete form adds:** it names the verified-CLI precondition, the `revive` arm, the exact record and the
  two witnesses. It weakens no outcome: «input box · no dialog · no key · other folders unchanged» all stand.
- **The verified-CLI precondition** is the product's own gate (a home stamped before the 23rd row writes nothing),
  carried as directive item 4 and the scope.

## Expected amendments (wrap)
- **Boundary widening, the founder's own answers:** R2, R3, F-T1 and the S1 shape with its sentinel (inputs#I2).
  Class `rul-r74hshif` / `rul-bznresq5`: the wrap halts once, as the answer card.
- **The rows to amend:**
  - security-plan: three no-write rows, `trust-record-writer-guarded`, `trust-record-closed-words`, two revive-cwd
    rows, four `CLAUDE_CONFIG_DIR`-value rows, `live-rig-seed-from-bundle-logic`, plus a Decisions Log entry;
  - architecture: `verify-own-trust-record`, `run-start-order`, five env rows, `trust-writer-words/key-present`,
    `instance-snapshot/cwd`, plus new rows for the start's words;
  - obs-plan: five rows plus the `trust-record` event's catalog row and its Decisions Log entry (an overseer answer,
    inputs#I2 item 5);
  - test-plan: `fake-agent-cli/trust-config`;
  - a11y-plan: the two time-bound rows.
- The full list with handles is in the plan's Implementation notes.

## The P5 block (directive item 12)
- **The union, verbatim:**
  ```
  architecture: rows 966 · packs 10 · picked 142 (89 by every draw) · standing 30 · 44882 B
  security-plan: rows 612 · packs 7 · picked 112 (74 by every draw) · standing 13 · 34231 B
  design-system: rows 316 · packs 4 · picked 34 (13 by every draw) · standing 15 · 10549 B
  layout-templates: rows 222 · packs 3 · picked 10 (6 by every draw) · standing 22 · 2686 B
  test-plan: rows 1343 · packs 14 · picked 229 (91 by every draw) · standing 56 · 58872 B
  obs-plan: rows 1025 · packs 11 · picked 177 (82 by every draw) · standing 50 · 42854 B
  a11y-plan: rows 235 · packs 3 · picked 53 (30 by every draw) · standing 18 · 15455 B
  rows-union: 7 master(s) · picked 757 of 4719 · rule any
  ```
- **Rows read whole:** 209 529 B, the sum of the union's `B`, under 250 000. Security and architecture were read at P3;
  test, obs, design, layouts and a11y at P4, each once. All 7 standing lists and all 7 history files were read whole.
- **Every cited row is in the union:** yes. Each of the plan's handles is a line of its master's rows file; none was
  looked up outside it. Four handles were first written with a wrong sheet (`span-trace-coverage` for
  `4-span-trace-coverage`, `pii-scrubbing-compliance` for `8-pii-scrubbing-compliance`, `test-data-fixtures` for
  `7-test-data-fixtures`, `e2e-test-strategy` for `6-e2e-test-strategy`). They were corrected before the first `put`.
- **`[RETYPED]` lines, verbatim:**
  ```
  arch-pick-p4 · [RETYPED] line 16: decision/fake-agent-test-seam-variables is no row of the pack — read as decision/fake-agent-test-seams, the pack's row 16
  arch-pick-p4.d3 · [RETYPED] line 44: decision/anyhow-only-in-root-bin-crate is no row of the pack — read as decision/anyhow-only-in-root-bin, the pack's row 44
  tests-pick-p6 · [RETYPED] line 49: decision/hook-fails-open-silently is no row of the pack — read as decision/hook-fails-open, the pack's row 49
  ```
- **Re-spawns:** none. Each of the three batches passed on the first try (7 tasks, 156 picker draws in 3 parts,
  7 history agents); there were 0 dead draws.
- **Non-zero exits and refusals:** no tool exited non-zero. Three hook refusals, each re-issued once:
  - the Bash guard's `cd` arm refused a `cd` into a skill's references directory (new-session) and a `cd` into this
    run dir (P4);
  - the `stamp-ahead` hook refused the plan's first `Generated:` stamp, an estimate. It was re-read from `date -u`.
- **`UNPARSED` / `INDETERMINATE` / `NOT DERIVED`:** none printed.
- **Where a letter was wrong or silent:**
  1. **Directive item 3** names «the two security rows» that say viola writes nothing into `~/.claude.json`. The rows
     read FOUR such rows: arch `verify-own-trust-record`, and security `no-writes-under-claude-home`,
     `viola-writes-nothing-in-claude-dir` and `nothing-written-under-claude-home`. All four are Expected amendments.
  2. **The obs history file's item 4** names the writer at `crates/viola-agent-claude/src/trust_record.rs`. It stands at
     `src/trust_record.rs`; the pure half is `crates/viola-agent-claude/src/trust.rs`.
  3. **The distill checkpoint's criteria 4–5** ask for a reading of the history and rows files before the letter's P4
     reads them. They were left to the plan step's record.
  4. **The relay's word list (inputs#I2 item 5)** names no word for the home-folder skip. The plan adds `home-folder`,
     said so in step 3.
  5. **The baseline walls** were short (`run --unit` 7.8 s, `run --e2e` 9.7 s, on cached builds). The pass counts were
     read from each entry's log before the baselines were written.

## Beyond the plan
none

Apply? (yes / review / cancel)
