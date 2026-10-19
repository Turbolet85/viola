# Codebase Research — 2026-10-19-dialog-hold-for-the-driver

## Scope
- **Depth:** deep · **Reads:** 22 · **Globs/Greps:** 14
- **Harness rules consulted:** `.claude/rules/verification-harness.md` — read in full at P4 once `inputs#I2` Q3 chose
  the live round (72 lines, 17 Session Additions); applied: 2026-10-10 (the stamped version first on `PATH`, read by
  path), 2026-10-11 (`schema-check` reads kept homes only), 2026-09-25 (never pipe `boot`), 2026-09-29 Ext. 2026-10-07
  (a stalled start on the tmpfs backing is reported, never re-run)
- **Platform issues consulted:** none — no runner-only bullet (CI `daf81f4a` green, HEAD `55699c4f` in progress)
- **External inputs:** `inputs#I1` — the operator's take-up direction: the founder-owned P4 forks go to him as one
  dialog

## Files inspected
- `crates/viola-core/src/lib.rs` (1-120, 430-475, 545-560) — `DIALOG_DEADLINE` = 60 s at :19 (doc :16-18 "PROVISIONAL,
  unmeasured"), pinned by `dialog_deadline_is_sixty_seconds` (:440); `EventKind` closed set of 15 (`ALL: [Self; 15]`
  :69, pinned by `kind_table_all_is_the_fifteen_kinds_in_declaration_order` :553); `WAIT_WAKE: [Self; 5]` :89 (pinned
  by `event_kind_wait_wake_is_the_five_driver_relevant_kinds` :469); `data_keys` per kind (:103-125).
- `src/run/dialog.rs` (1-135, 180-392, 430-520, 640-680) — the hold: `hook_dialog` holds only when `cli_verified &&
  pending.is_none() && !unanswerable && wheel == Driver` (:199-202); `await_answer` (:273-306) computes `deadline =
  clock.now() + DIALOG_DEADLINE` (:281) on the injected `Clock`, loops on `STEP` 20 ms, ends on answer / hand-back /
  deadline, then clears `pending`, calls `feed.dialog_settled()`, clears `pending_dialog` and replies
  `reply(dialog_id, None)` at expiry; the span records `deadline_hit` (:298); NO event line and NO obs line is written
  at expiry (`ObsEvent::DialogAnswered` is emitted only in `answer`, :372-381, with `deadline_hit = false`); a late
  `answer` finds no pending and is refused `not-delivered` / `unknown-dialog`, pinned by
  `hook_dialog_expires_at_the_deadline_on_the_injected_clock` (:646-658, `JumpClock` :432).
- `src/cmd/hook.rs` (55-75, 180-340, 690-712) — the hook's own bound `dialog_deadline(started) = started +
  DIALOG_DEADLINE + REPLY_GRACE` (5 s, :64-70); `handle_dialog` writes `hook-decision` via `dialog_decided`: a wrapper
  reply of `null` is `Asked::Replied{response: None}` → `hook-decision` at INFO with `corr`, `decision_emitted:false`,
  `deadline_hit:false` (:305-336); `deadline_hit:true` / `detail:"deadline"` (WARN) only when the hook's OWN read
  times out (`Asked::Deadline`, :260, :316). Tests pin the grace (:696-702) and `DIALOG_DEADLINE + REPLY_GRACE < 75 s`
  (:706-708).
- `plugin/hooks/hooks.json` (full) — PreToolUse `AskUserQuestion|ExitPlanMode` and PermissionRequest `timeout` 75
  (:10, :13); spine hooks 5; a compiled template rendered by `@@VIOLA_BIN@@` substitution only.
- `crates/viola-agent-claude/src/lib.rs` (560-645) — the rendered `hooks.json` literal pinned at :564/:567 (timeout
  75) and `hooks_json_dialog_timeouts_outlast_the_dialog_deadline` (:617-641, margin ≥ 10 s over `DIALOG_DEADLINE`).
- `src/cmd/run.rs` (250-300, 390-410) — start order: `read_thresholds(&home)` (:258) then `DialogSlot::new(SystemClock,
  gate.cli_verified, …)` (:284); `pin_and_plugin` writes `plugin/<pinned.key>/` through `replace_private_shared`
  (:403-409) — the key is the pinned exe's version-hash, shared by every start of that binary in a home.
- `src/run/budget.rs` (20-100) — the bounded-key template: `read_thresholds` (`take(MAX_FRAME)`, `v` == 1, per-key
  `whole_percent`, a bad value → default + `ConfigRejection::Malformed`).
- `src/obs.rs` (28-40) — `CONFIG_KEYS: [&str; 5]` (`v`, `diagnostics_level`, `claude_env_keep`, `five_hour`,
  `seven_day`); `schemas/config.v1.json` (full) — the same five properties.
- `src/run/wait.rs` (56-215, 657-700) — `appending` (any line, under the feed lock, notifies), `appending_dialog`
  (keeps a held dialog's start so `wait` without `after` returns it), `dialog_settled`; `scan` wakes only on
  `EventKind::WAIT_WAKE` (:200-206).
- `src/run/send/slot.rs` (215-232) — a non-dialog line appended through `feed.appending` (the precedent for a
  wrapper-side line reaching `wait`).
- `crates/viola-channel/src/server.rs` (405-420) — `wait_outcome` derives the `channel-response` `outcome` from
  `EventKind::WAIT_WAKE`, so a kind added there flows into `outcome` with no edit; `schemas/diag-line.v1.json:43`
  types `outcome` as an open string.
- `src/cmd/wait.rs` (37, 85-106) — `DIALOG_KINDS: [&str; 3]` decides the human line form: a listed kind prints
  `<kind>  <name>  dialog <id>  cursor <n>`, any other the time form.
- `src/human/wait.rs` (1-80) — `write_woken` / `write_woken_dialog`, generic over the kind word, escaped, one write.
- `src/run/wheel.rs` (280-298) — `record` calls `dialogs.hand_back()` on every move to `human`, independent of the
  hold's length.
- `crates/viola-state/tests/state_events.rs` (125-150) — the round-trip over `every_kind()`, `lines.len() == 15`.
- `crates/viola-agent-claude/src/ledger.rs` (49, 142-160) — `LedgerRow::ALL: [Self; 24]`; no row names a hook
  timeout (`/usr/bin/grep -rn -i "hook.timeout\|hook-timeout\|dialog-hold" crates/viola-agent-claude/src`: 0 hits).
- `viola-0.1.0/chunks/2026-10-08-first-live-test-and-self-drive/evidence/gap-list.md` (full) — group 2 = items 27, 28.
- `viola-0.1.0/chunks/2026-10-18-plugin-precedence-live-readings/evidence/live-round.md` (index) — the last live
  round's shape (3 starts of a cap of 3, `hyprctl locked` read first, rig outside the work tree).
- `target/e2e-home/*/` ledger reads — the only standing `ledger/stamps.json` is
  `viola-record-20261013T232122Z/vhome/` (2026-10-13), which predates the 24-row relied set, so no standing home
  holds a stamp `run` reads `cli_verified:true` from (`test -f …/ledger/stamps.json` over every `target/e2e-home/*/`
  and its `whome/` / `ihome/`).

## Graph impact (from the code-graph query; trace `.andromeda/runs/2026-10-19T07-38-25-phase/tree-query-2026-10-19-dialog-hold-for-the-driver.json`)
- **DIALOG_DEADLINE** — 9 refs, 4 production files: `crates/viola-core/src/lib.rs:441` (pin),
  `crates/viola-agent-claude/src/lib.rs:628` (timeout margin test), `src/cmd/hook.rs:26,69,700,707`,
  `src/run/dialog.rs:22,278,281` — every reader of the hold is one of these four files.
- **WAIT_WAKE** — `crates/viola-channel/src/server.rs:417` (outcome), `crates/viola-core/src/lib.rs:470` (pin),
  `src/run/wait.rs:201` (scan) — a kind added to the set reaches `wait` and the logged `outcome` with no other edit.
- **dialog_deadline / REPLY_GRACE** — `src/cmd/hook.rs:112,699,889,908` / `:69,707` — hook-local.
- **plugin_files** — `src/cmd/run.rs:405`, `crates/viola-agent-claude/src/lib.rs:590,621`.
- **read_thresholds** — `src/cmd/run.rs:259`, `src/cmd/list.rs:127`, `src/run/budget.rs:452-461` (tests).
- **hand_back** — one caller, `src/run/wheel.rs:295`; **await_answer** — one, `src/run/dialog.rs:223`;
  **dialog_settled** — `src/run/dialog.rs:300`, `src/run/wait.rs:676`.

## Patterns detected
- **The hold runs on the injected clock** (`src/run/dialog.rs:281`): expiry is already unit-driven (`JumpClock`), so a
  longer or configured hold needs no real-time wait in any test.
- **The wrapper ends the hold, the hook's bound is a backstop** (`src/cmd/hook.rs:64-70`, `src/run/dialog.rs:281`): at
  expiry the wrapper replies `null` 5 s before the hook would give up. The hook's bound and the `hooks.json` timeout
  only need to outlast the LONGEST hold the wrapper can hold — so they can stay compiled at a ceiling while the hold
  itself is read at start.
- **A bounded config key** (`src/run/budget.rs:56-95`): read once at start, per-key default on a bad value with one
  rejection, no value switches the control off.
- **A wrapper line that wakes `wait`** goes through `WaitFeed::appending` (`src/run/send/slot.rs:228`) and is woken
  only if its kind is in `WAIT_WAKE` (`src/run/wait.rs:200-206`).

## Conventions to follow
- **One named home per time bound, PROVISIONAL until measured** (`crates/viola-core/src/lib.rs:16-19`;
  `COMPACT_CONFIRM_WINDOW` precedent).
- **Kinds only in `viola-core`, kebab-case, `data_keys` closed** (`crates/viola-core/src/lib.rs:46-125`).
- **Literal expectations in tests** (`state_events.rs:136` `15`, `lib.rs:553-570` kind list): every pin moves as a
  literal.

## New files to create
- `viola-0.1.0/chunks/2026-10-19-dialog-hold-for-the-driver/evidence/` — the live round's two copied rig scripts, its start ledger and `live-round.md`

## Files to modify
- `crates/viola-core/src/lib.rs` — the hold's named home(s) and its bounded type; the expired-hold kind in `EventKind`, `ALL`, `WAIT_WAKE`, `data_keys`, `as_str`, and their pins
- `src/run/dialog.rs` — the hold read from `config.json` and taken at start; at expiry the expired-hold line through the wait feed and a `dialog-answered` line with `deadline_hit` true; unit cases on the injected clock
- `crates/viola-state/src/events.rs` — the in-file kind-list read-back case gains the new kind
- `src/cmd/hook.rs` — the hook's reply bound against the hold's ceiling; its two pins
- `plugin/hooks/hooks.json` — the two dialog `timeout` values
- `crates/viola-agent-claude/src/lib.rs` — the rendered `hooks.json` literal pins and the margin test
- `src/cmd/run.rs` — the hold read at start and passed to `DialogSlot::new`; the in-file `DialogSlot::new` test caller
- `src/cmd/wait.rs` — `DIALOG_KINDS` takes the expired-hold kind (dialog form)
- `src/human/wait.rs` — the expired-hold line pinned beside the design lines
- `src/run/wait.rs` — the wake-kind rstest rows
- `crates/viola-state/tests/state_events.rs` — the kind round-trip count and its new row
- `src/obs.rs` — `CONFIG_KEYS` takes the hold key
- `schemas/config.v1.json` — the hold key's property
- `tests/cli_controls_not_disableable.rs` — the hold key's `SETTINGS` row and its booted case

## Sweeps
- `DIALOG_DEADLINE` over the work tree (`/usr/bin/grep -rln`, `target/`, `.git/`, `runs/` excluded): 4 code files (all
  on the list) · the masters and `.claude/docs/services/viola-core.md`, `viola.md` (no change here: wrap amendments and
  the doc agent) · chunk records, `CLAUDE.md` backups, the matrix and the route (no change: records).
- `"timeout": 75` / `from_secs(75)` over `*.rs` + `*.json`: 5 hits · 5 on the list (`plugin/hooks/hooks.json:10,13`,
  `crates/viola-agent-claude/src/lib.rs:564,567`, `src/cmd/hook.rs:707`).
- `WAIT_WAKE` over `*.rs`: 4 hits · `crates/viola-core/src/lib.rs:89,470` changed · `src/run/wait.rs:201`,
  `crates/viola-channel/src/server.rs:417` no-change (derived from the set).
- `[Self; 15]` / `EventKind::ALL` over `*.rs`: `crates/viola-core/src/lib.rs:69,554` changed ·
  `crates/viola-core/src/obs.rs:31` no-change (the `ObsEvent` set, 15 by coincidence; no new obs `event` is planned).
- `DIALOG_KINDS` over `*.rs`: 1 file (`src/cmd/wait.rs`) changed.

## Open questions
- none — the three P4 forks this list carried (the hold's length and setter, the reply at expiry, the live start cap)
  are answered by the founder's own word, `inputs#I2` (Q1 «Ключ, по умолчанию 60», Q2 «Отдать человеку», Q3 «Verify +
  1 старт, 11 мин»).

## Live-round facts (P4, read for the live leg)
- The one trusted `proj/` outside the work tree on disk: `target/e2e-home/viola-live-plugin-20261019T061245Z/proj/`
  (rig B of chunk 2026-10-18-plugin-precedence-live-readings, on the tmpfs behind the `target/e2e-home` link). Read at
  P4: `git -C … rev-parse --show-toplevel` names `proj/` itself (its own `git init`); `~/.claude.json`'s verdict for its
  resolved path reads `trusted` (a scratch reader keeping the verdict word alone); its `.claude/settings.local.json`
  holds `enabledPlugins` `{}` and `extraKnownMarketplaces` `{}` (that round's removal left no plugin). A fresh `proj/`
  would show the folder-trust dialog, and `inputs#I2` Q3 allows no key into any dialog.
- `viola verify` against the real CLI runs from the repository root (a trusted parent of its probe dirs), its home a
  `vhome` under `target/e2e-home/`, `VIOLA_*` removed (`chunks/2026-10-18-compact-post-condition-ledger-row/plan.md`
  record entry; its round: print probe + Runs A–E, `stamped 2.1.287  24 pass  0 fail`, `src/cmd/verify.rs:277`).
- `live-pty.py` types nothing into its pty, keeps no screen, stops by Ctrl-C every 500 ms
  (`chunks/2026-10-18-plugin-precedence-live-readings/evidence/live-pty.py:1-18`); `live-ledger.py` writes one
  validated row per call (`…/evidence/live-ledger.py:1-12`).
- The PATH rule for the stamped version (verification-harness.md 2026-10-10) and `hyprctl locked` read first
  (host-linux.md 2026-10-08).
