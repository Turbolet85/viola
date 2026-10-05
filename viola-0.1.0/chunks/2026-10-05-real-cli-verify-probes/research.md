# Codebase Research — 2026-10-05-real-cli-verify-probes

_Revised 2026-10-05 after implement's step-0 STOP (`evidence/screen-probe-2.1.288.md`) and the founder's two live
rulings (phase run dir `.andromeda/runs/2026-10-05T07-05-09-phase/relay-3.md`, `relay-4.md`). M1-M11 are the take-up
research, kept. M12-M18 are the revision's. The file lists below are rewritten to the files this chunk writes after
the split and the revision._

_Revision 3, 2026-10-05: M19-M21 added after implement's step-0 STOP 3 (`evidence/screen-probe-2.1.288.md` §"Step 0 of
plan revision 2") and the founder's live rulings of ~08:50Z (`.andromeda/runs/2026-10-05T09-09-05-phase/relay-6.md`).
M13's "starts with no dialog" holds for the trust dialog only (M19). The file lists are unchanged._

## Scope
- **Depth:** deep · **Reads:** 14 at take-up, plus 9 at the revision · **Globs/Greps:** 12, plus 8 · **Graph
  queries:** 1 (`tree-query-2026-10-05-real-cli-verify-probes.json`, rust plane, in the take-up run dir
  `.andromeda/runs/2026-10-05T00-16-17-phase/`)
- **Harness rules consulted:** `.claude/rules/verification-harness.md`, read in full (16 330 B, 9 Session Additions)
  - its §Fake agent modes line;
  - the `run --local-live` contract;
  - 2026-09-25 "never pipe `agent-run.sh boot`";
  - 2026-10-04 "keep a mutation run's `TMPDIR` short".
- **Platform issues consulted:** none. No runner-only bullet: CI on both shas read green (re-read at the revision's
  Setup 5a: `caae9ec` green 15/15 wall 250 s; `ed2edde` green 15/15 wall 339 s).
- **External inputs:**
  - `inputs#I1`: the overseer's relay `split84-route-adaptation.md` (R-S1 / R-S2 / R-S3 and the partition);
  - `inputs#I2`: the operator's take-up directive (one builder window, a split card at P4, count the live runs).
  - The installed `claude` binaries were read for strings only (no API use). What they showed is recorded in
    `evidence/screen-probe-2.1.288.md` and M13-M14 below.

## Measured facts
- **M1 — the installed CLI** (`claude --version`, no API use): `2.1.288 (Claude Code)`, the mise install on PATH
  (`~/.local/share/mise/installs/claude/latest/claude`). This shell carries 10 inherited `CLAUDE*` names (`env | cut
  -d= -f1 | grep -c '^CLAUDE'`), names only.
- **M2 — verify already strips R8 for its children.**
  - `src/cmd/verify.rs:137` builds `plan_strip(...)` from the inherited names and the persistent set.
  - The resulting `StripPlan` goes to both `run_logged` spawns (`:141` version probe, `:163` print probe) through
    `run_bounded`.
  - A PTY child built from the same `StripPlan` (as `viola_pty::SpawnSpec.env_remove`,
    `crates/viola-pty/src/lib.rs:45-52`) keeps that. Nothing new is crossed.
- **M3 — `run`'s readiness gate passes no signatures.**
  - `src/run/gate.rs:165` calls `model.screen.verdict(None, waiting_since, clock.now())`, so the gate is the partial
    gate (quiet + not poisoned) on every CLI build.
  - `send`'s confirmation window is the compiled `CONFIRM_WINDOW_FALLBACK` (`src/run/send.rs:109`, `:182`).
  - `cli_verified` is computed by the version gate (`src/cmd/run.rs:220`, `:417`). It does not reach `pump_child`
    (`:475`), which calls `gate::start(SystemClock, size)` at `:487`. `Launched` is the struct at `:156`.
- **M4 — the fake agent renders no screen.**
  - Its stdout carries only the `--version` answer and print mode's one reply line
    (`src/bin/viola-fake-agent.rs:3`).
  - Interactive mode (`main`, `:575-612`) enters raw mode, writes the `start` / `cwd` / `env` receipts
    (`start_receipts`, `:366`), fires SessionStart at launch (`:603`), runs any script and reads keys
    (`read_stdin`, `:552`).
  - `verified` needs every `LedgerRow::ALL` row `pass` (`crates/viola-agent-claude/src/ledger.rs:283-293`).
- **M5 — `unconfirmable` does not exist in code** (moved with W2; not built here).
- **M6 — the six-row literal sites** (`grep -c '/06\]'`):
  - `tests/cli_verify.rs`: 8 hits;
  - `crates/viola-e2e/src/harness/run.rs`: 7 hits, plus `const LEDGER_ROWS: [&str; 6]` at `:333-340` and the test
    literal `LIVE_PASS` at `:898`;
  - `tests/contract_ledger_probes.rs:16-21`: the six ids, and `:70` `"stamped {version}  6 pass  0 fail"`.
- **M7 / M8 / M10** — dialog and cross-session facts, moved with W3 / W4 (not built here).
- **M9 — the fixture schema and walk are JSON-hook-only.**
  - `schemas/claude-fixture.v1.json` requires `hook_event_name`.
  - `tests/contract_fixture_hygiene.rs:41` walks every `*.json` file under each version dir.
- **M11 — CI since the last wrap**: green on `caae9ec` and `ed2edde` (Setup 5a, read twice).
- **M12 — step 0, live, 2.1.288, session 1 of the cap** (`evidence/screen-probe-2.1.288.md`):
  - In a fresh untrusted dir, the first quiet screen is the workspace trust dialog. It settled at 300 ms of quiet
    847 ms after spawn, and the rows were unchanged at 2 000 ms of quiet.
  - The dialog's focus starts on "❯ No, exit". The one key (Enter) chose it: exit 1, no trust written.
  - No hook fired while the dialog was up, SessionStart included.
  - The dialog's rows carry the cwd, absolute and wrapped over two rows, with 1-column padding.
- **M13 — the trust check, read from the 2.1.288 binary.**
  - Trust is inherited: the check walks up from the cwd to the first ancestor whose
    `projects[<dir>].hasTrustDialogAccepted` is true (`zE` via `k0` / `jE`).
  - `/home/turbolet/dev/projects/viola` is a trusted project (`~/.claude.json`), so every dir under the repo,
    `target/e2e-home/` included, starts with no dialog.
  - The dialog is a confirm component with `cancelFirst:!0, focus:"cancel", hideIndexes:!0` and an input-refusal
    window (`refuseInput` / `openedAt`).
  - The CLI's own text says home-directory trust is session-only. So the default `~/.viola` is never under a
    persisted trusted parent.
- **M14 — 2.1.287 shows the same dialog** (strings in the 2.1.287 binary, no session): `Yes, I trust this folder`,
  `Quick safety check: Is this a project you created`, `Accessing workspace:`, and the exact
  `cancelFirst:!0,focus:"cancel",confirmLabel:"Yes, I trust this folder"` render. The modal literals hold for both
  recorded versions.
- **M15 — every verify caller already runs from the repo root.**
  - The harness `boot` step 4 (`crates/viola-e2e/src/harness/boot.rs:187-200`) passes no `current_dir` to the
    `verify` spawn, so it inherits the harness's cwd, the repo root (the shim runs `cargo run` there).
  - `local_live` (`crates/viola-e2e/src/harness/run.rs:363-368`) spawns verify with `.current_dir(&ws.root)`.
  - Test support (`tests/support/verify.rs:88-118`) spawns `viola` with the test process's cwd, the package root =
    the repo root.
  - The plan's record gates run from the repo root.

  So Run B's `<cwd>/.viola-verify-<pid>/` lands in the repo root for every caller. The repo root is trusted on the
  dev host, and for the fake agent in CI it is whatever the fake's trust rule says (M17).
- **M16 — `.gitignore` has no entry for a root-level probe dir.** It ignores `target/` and gate outputs. A
  `.viola-verify-<pid>/` left by a killed verify would show in `git status`.
- **M17 — the fake agent needs a trust rule, not a per-run flag.** Verify passes the same `-- program args` to both
  interactive runs. So the fake can tell Run A from Run B only by its cwd, as the real CLI does. Mirroring M13's
  ancestor walk with a `--trusted-root <DIR>` argument keeps that single source and needs no env var.
- **M18 — the drift and ledger contracts walk version dirs** (`tests/contract_fake_agent_drift.rs:22-32`,
  `tests/contract_ledger_probes.rs:25-35`).
  - Drift compares only `<Event>.default.json`, so `Screen.*.json` files beside them are ignored.
  - The ledger contract stamps every dir found, 2.1.283 included. It needs the literal stamp and drift-only lists.
  - `tempfile = "=3.27.0"` is a workspace dependency (`Cargo.toml:207`, used at `:143`), which gives Run A's 0700
    temp dir without a new crate.
- **M19 — step 0 of revision 2, live, sessions 2 and 3 of the cap** (`evidence/screen-probe-2.1.288.md` §"Step 0 of
  plan revision 2", `evidence/live-sessions.md` rows 2-3).
  - **Run A held:**
    - in a fresh 0700 `/tmp` dir with no plugin and no input byte, the first settled screen is the trust dialog, at
      847 ms (300 ms quiet), with `Yes, I trust this folder` on a row;
    - no other modal appeared;
    - the kill ended it (exit 1);
    - STOP 2 read clean: no `projects` key and no projects dir for the dir.
  - **Run B stopped (STOP 3).** In `<repo root>/.viola-verify-<pid>/` the first settled screen (563 ms) is a second
    CLI-native dialog, "Allow external CLAUDE.md file imports?":
    - focus is on "No, disable external imports", with "Yes, allow external imports" below;
    - one row holds the import's absolute home path;
    - no hook fired before it, SessionStart included.
  - **Cause, read from the 2.1.288 binary:**
    - The repo CLAUDE.md's `@.claude/session-handoff.md` (`CLAUDE.md:91`) lies outside a subdir cwd, which makes it an
      external include. Repo-root sessions are unaffected.
    - The project config is keyed by the cwd's git root (`IOe`), so every subdir of the repo reads the root's
      `projects` entry.
    - The dialog shows only while that entry has neither `hasClaudeMdExternalIncludesApproved` nor
      `hasClaudeMdExternalIncludesWarningShown` (`pPo`).
    - Either answer writes `Approved: <choice>, WarningShown: true` (`P9t`).
    - It carries the same input-refusal window as the trust dialog.
  - **Both recorded versions carry it:** 2.1.287 and 2.1.288 each hold `Allow external CLAUDE.md file imports?`,
    `Yes, allow external imports` and `Yes, I trust this folder` (strings read, no session).
- **M20 — the stored flags before the founder's hand answer** (read 2026-10-05T09:09:22Z, booleans only): the repo
  root's `projects` entry reads `hasTrustDialogAccepted: true`, `hasClaudeMdExternalIncludesApproved: false`,
  `hasClaudeMdExternalIncludesWarningShown: false`. `~/.claude.json` holds 8 project keys, none `viola-verify`, and
  `~/.claude/projects/` holds 13 dirs.
  - After the founder's ruled "No, disable external imports" the entry reads `Approved: false, WarningShown: true`
    (`P9t`).
  - It was set by the overseer on the founder's amended live ruling (~09:05Z), an atomic edit with a backup kept, and
    the overseer verified it at 09:10Z and 09:11:52Z (`relay-7.md`). P5 re-read it at 2026-10-05T09:15:20Z:
    `hasTrustDialogAccepted: true`, `Approved: false`, `WarningShown: true`, 8 project keys (none `viola-verify`),
    13 projects dirs.
  - Under `pPo` that state shows no dialog to any subdir of the repo, and the import stays unloaded there.
- **M21 — the two code sites the rulings touch.**
  - `Signatures.modals` is `&'static [&'static str]` (`crates/viola-agent-claude/src/screen.rs:21-24`), so a second
    modal literal is one more slice element. `verdict` reads it unchanged (`:96`).
  - The `verify` subcommand's help is a one-line doc comment (`src/cmd/mod.rs:52-53`) that clap renders as `about`. A
    following doc paragraph renders only in `--help`, not `-h`.

## Files inspected
- Revision 3: `crates/viola-agent-claude/src/screen.rs:21-24,96` (`Signatures`, `verdict`), `src/cmd/mod.rs:52-53`
  (the `verify` help), `CLAUDE.md:91` (the `@` import), and `~/.claude.json` (booleans and key counts only).
- `src/cmd/verify.rs` (full): the print-mode probe, the `ProbeDir` drop guard, `check_rows`, `record`, `step_line`.
- `crates/viola-agent-claude/src/ledger.rs` (full): `LedgerRow` (6), `check`, `merge_stamp`, `verified`,
  `capture_plugin_files`, `scrub`, `is_clean`.
- `crates/viola-agent-claude/src/screen.rs` (full): the three PROVISIONAL constants, `Signatures`, `verdict`.
- `crates/viola-pty/src/lib.rs:34-260`, `crates/viola-pty/src/pump.rs:1-160`: `Size::DEFAULT`, `SpawnSpec`, `spawn`,
  `Pty::kill`, `pump_with_paste`, `PasteHandle::paste` (one bracketed paste + `\r`).
- `src/run/gate.rs:165`, `src/cmd/run.rs:149-487`: the `None` signatures, `Launched`, `cli_verified`, `pump_child`.
- `src/bin/viola-fake-agent.rs:62-80,360-400,520-612`: options, receipts, print turn, interactive main.
- `tests/support/verify.rs` (full), `tests/support/home.rs:242-285`: the verify helper and `stamped_home`.
- `crates/viola-e2e/src/harness/boot.rs:185-215`, `run.rs:333-395`, `supervise.rs:30-90`: boot step 4,
  `local_live`, the wrapper spawn's fake args.
- `tests/contract_ledger_probes.rs` (full), `tests/contract_fake_agent_drift.rs:1-60`,
  `tests/contract_fixture_hygiene.rs:25-45`, `.gitignore`.

## Graph impact
- **`ledger::verified`** has 2 production callers: the `use` at `src/run/version_gate.rs:16`, and `stamps_verdict`
  at `src/run/version_gate.rs:170`. Growing `LedgerRow::ALL` flips every stamp lacking the new rows to unverified
  through this one path.
- **`ledger::check`**: 1 production caller, `check_step` at `src/cmd/verify.rs:275`; 16 unit sites in `ledger.rs`.
- **`ledger::merge_stamp`**: `measure` at `src/cmd/verify.rs:182`; test stamps at `src/run/version_gate.rs:180,184`.
- **`ledger::capture_plugin_files`**: `ProbeDir::create` at `src/cmd/verify.rs:213`; 2 unit tests.
- **`gate::start`**: 1 production caller, `pump_child` at `src/cmd/run.rs:487`.
- Crate edges are unchanged: the PTY drive sits in the root bin, which already depends on `viola-pty` and
  `viola-agent-claude`.

## Patterns detected
- **Probe dir with drop guard** (`src/cmd/verify.rs:200-234`): every probe artifact lives under
  `ledger/probes/<pid>/` and is removed on drop. The two new child cwds take the same guard.
- **Spawn pair at the call site** (`src/cmd/verify.rs:68-92`): `run_logged` wraps a spawn in
  `process-start` / `process-exit{subject}`.
- **Pure screen model, bin-side feed** (`src/run/gate.rs`, `screen.rs`): the feed runs under `catch_unwind`, and the
  clock is injected.

## Conventions to follow
- **Literal oracles**: the row ids and counts are test literals, never `LedgerRow::ALL`
  (`tests/contract_ledger_probes.rs:16`).
- **Consumer-first fake-agent modes**: a mode lands only after the recorded fixture it replays (test-plan history
  `2026-09-24-fake-agent-and-test-data-fixtures`).
- **Fake-agent options are argv, never env** (`src/bin/viola-fake-agent.rs:62-80`; security-plan test seams).

## New files to create
- `src/cmd/verify/typed.rs` — the two interactive PTY runs (Run A untrusted, Run B trusted), the feed, the quiet waits, the paste, the end.
- `schemas/claude-screen.v1.json` — the screen-fixture class schema.
- derived `fixtures/claude/2.1.288/*` by `viola verify --record` — the recorded 2.1.288 hook payloads and screen fixtures.
- derived `fixtures/claude/2.1.287/Screen.*.json` by `viola verify --record` — the 2.1.287 screen fixtures.

## Files to modify
- `src/cmd/verify.rs` — the two interactive runs wired into `measure`, `Probes`, and `record`'s screen fixtures.
- `src/cmd/mod.rs` — the `verify` subcommand's help names the trusted-folder requirement.
- `src/cmd/run.rs` — `Launched` carries `cli_verified` to `pump_child`.
- `src/run/gate.rs` — `gate::start` takes the signatures.
- `crates/viola-agent-claude/src/ledger.rs` — four rows, `TypedRun` / `Probes`, `measured.typed_probe`, the screen keep-and-check helpers.
- `crates/viola-agent-claude/src/screen.rs` — `SIGNATURES`, the constants' docs.
- `crates/viola-agent-claude/src/lib.rs` — re-exports.
- `src/bin/viola-fake-agent.rs` — `--screens`, `--trusted-root`, `--turn-stop`.
- `schemas/diag-line.v1.json` — the `verify-pty-probe` subject.
- `.gitignore` — the root-level `.viola-verify-*/` probe dir.
- `tests/support/verify.rs` — the fake-agent flags for every verify.
- `tests/support/home.rs` — `stamped_home` and the wrapper boot's fake-agent flags.
- `crates/viola-e2e/src/harness/boot.rs` — boot step 4's fake-agent flags.
- `crates/viola-e2e/src/harness/supervise.rs` — the wrapper's fake-agent flags.
- `crates/viola-e2e/src/harness/run.rs` — `LEDGER_ROWS` and `LIVE_PASS` at ten.
- `tests/cli_verify.rs` — the step counter, the row literals, the new arms.
- `tests/contract_ledger_probes.rs` — ten ids, the literal stamp and drift-only lists.
- `tests/contract_fixture_hygiene.rs` — the screen class walk and planted reds.
- `tests/contract_diag_schema.rs` — the new subject value.
- `tests/cli_send.rs` — the full-gate witness.

## Open questions
- none. The take-up's three plan-decision questions closed:
  - the split, by the founder's split ruling (W1 + W5 here);
  - M7 and M10, by moving W3 and W4 to their own entries;
  - the trust handling and the two probe dirs, by the founder's rulings of ~07:00Z and 08:25Z (`relay-3.md`,
    `relay-4.md`).
