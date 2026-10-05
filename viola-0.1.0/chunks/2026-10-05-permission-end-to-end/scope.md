# Scope — 2026-10-05-permission-end-to-end

**Working entry** (`working-route.md:88`, Epoch 3 — Windows slice II: driving verbs and live proof):
Permission end to end — the permission kind's end-to-end case and wake witness, and the PermissionRequest body for a
question first raised there.

## Intent
The `permission` dialog kind gets the end-to-end case the `question` and `plan` kinds already have (test-plan §6 Path
4): an ordinary-tool PermissionRequest, replayed from the recorded fixtures, is logged once with a wrapper-assigned
`dialog_id`, wakes a parked `viola wait`, is answered by id with `viola answer`, and the hook prints the decision body
the answer maps to. That case is also `verification-matrix.json#v1-30`'s owed end-to-end wake witness for the dialog
kinds. The PermissionRequest body for a `question` first raised by PermissionRequest is measured; a body is built from
the measurement, or the `null` to the human stays with a recorded reason. A stale owner name in `tests/cli_answer.rs`
is reworded. One testing rule gains its designed-floor exception (the operator's fold). viola stays mechanism: it
carries the driver's answer and decides nothing.

## Authority
- The founder's split ruling, live at the 2026-10-05-dialog-rows-and-re-probe P4 (via the overseer's AskUserQuestion,
  relayed by the operator; that chunk's `inputs/I3-relay-2.md.txt`): W3d and W6 of that chunk's scope leave it for this
  entry, right after it and ahead of "Local-command and paste-framing rows". Epoch 3 stays one epoch (14 entries).
- Chunk 2026-10-04-dialog-answers-by-dialog-id's route pins (P5-approved), re-pointed here: the `permission` kind's
  Path 4 case and its `v1-30` wake witness (unit / insta only at HEAD).
- The 2026-10-05 0-pending wrap's re-point: `tests/cli_answer.rs:9` names this case's owner by a stale name.
- The operator's directive at this take-up (inputs#I1):
  - fold one item, overseer-decided: the `.claude/rules/testing.md` 2026-09-28 timing-red entry gains the designed-floor
    exception the `verify_window_` class used at :86;
  - plan zero live `claude` sessions if possible and name any needed. The founder's cap of 16 has 3 left;
  - Epoch 3 stays unsplit.

## What it builds (the three CARRYs and the fold, in route order)

### CARRY 1 — the split's own record
- This entry is W3d + W6 of `2026-10-05-dialog-rows-and-re-probe`'s scope (its `scope.md` §W3d, §W6), minted at that
  chunk's wrap. "Local-command and paste-framing rows" (`:90`) and "First live test and self-drive" (`:92`) follow it.
- Sizing anchor: the previous chunk (W3a–W3c) needed four CI rounds and 16 live sessions (its
  `evidence/live-sessions.md`, `evidence/ci-rounds.md`). This entry is much lighter.

### W3d-a — the `permission` end-to-end case and the `v1-30` wake witness (CARRY 2)
- The case runs over the ordinary-tool fixtures chunk 2026-10-05-dialog-rows-and-re-probe recorded:
  `PermissionRequest.permission-1.json` and `PostToolUse.permission-1.json` under `fixtures/claude/2.1.288/` and
  `fixtures/claude/2.1.287/` (both present at HEAD, verified at take-up by listing).
- What it asserts, in the shape `tests/cli_answer.rs` already uses for `question` and `plan` (its header, `:1-8`):
  - the PermissionRequest is logged once, with a `dialog_id`;
  - it wakes a `viola wait` that is parked;
  - `viola answer` answers it by id;
  - the hook prints the matching PermissionRequest body (`allow`, and `deny` with a `message`).
  It runs on all three CI OSes through the fake agent. No live session is needed.
- `question` and `plan` already have an end-to-end wake witness at HEAD in `tests/cli_answer.rs`
  (`path4_dialogs_are_logged_once_woken_and_answered_by_id`, docstring `:246-248`, asserts `:256-267` and `:316-324`;
  research), so only `permission` is missing.
- `v1-30` is `verified`, claimed by `2026-10-04-wait-and-last`. Its acceptance says the dialog kinds' end-to-end
  wait witness is owed to "working-route.md:78" (research M5). That claim was re-pointed twice and now lands here.
  `matrix.py claim` refuses a verified cap, so how this witness is recorded is a P4/P5 question (a `notes` line, or
  the wrap's reconcile of the stale clause). The cap's acceptance must not be weakened.

- Added at P5 validation-1 (intent-incomplete, from the extracts and the matrix pool):
  - the `permission` kind's unstamped negative. The existing unstamped case (`tests/cli_answer.rs:581-624`) raises
    only a question; the security extract asks that a `permission` `allow` be refused `unverified-cli` too;
  - `verification-matrix.json#v1-16` ("Permission suggestions have a place in the answer"). Its "if not" branch is
    already recorded at HEAD: `.andromeda/architecture.md:280` names the omission as a deliberate v1 limit (the
    founder's live ruling, 2026-10-04). Its pool note waited only on that record. The permission case adds the end-to-end
    half: an `allow` answer carrying suggestion fields emits no suggestion (`Response::parse` skips unknown fields,
    `dialog.rs:199-202`).

### W3d-b — the PermissionRequest body for a `question` first raised by PermissionRequest (CARRY 2)
- Unmeasured at HEAD; viola leaves it `null` to the human. "Measure it" is this entry's work. Whether a body is then
  built, or the `null` stays with a recorded reason, follows the measurement.
- Known before this chunk: the prototype's 3 AskUserQuestion PermissionRequests each came after an unanswered PreToolUse,
  so none was first raised by PermissionRequest (the previous chunk's `inputs#I2`). That chunk's plan also asserted that
  a hook-answered AskUserQuestion raises no PermissionRequest. Neither shows when a question *first* reaches
  PermissionRequest.
- Measured with zero live sessions: a static read of both installed CLI bundles (research M1). On 2.1.288 and
  2.1.287 a PermissionRequest `allow` satisfies AskUserQuestion's user interaction only when it carries
  `updatedInput` (both tools in the predicate's set: `ExitPlanMode`, `AskUserQuestion`); a bare `allow` yields no
  decision and the dialog renders. With AskUserQuestion's permission result mapping `updatedInput.answers` /
  `annotations` (the previous chunk's M6), the candidate body is PermissionRequest `allow` + `updatedInput` = the input
  plus `answers` (and any `annotations`). A static read is not a ledger post-condition: nothing was run.
- [premise-corrected: research M2 — arms are one per tool (`src/run/dialog.rs:199`), so the case is reachable] A
  question first raised by PermissionRequest is reachable while viola is installed, rarely: two parallel
  AskUserQuestion calls (the second PreToolUse drops the first arm), a wrapper restart between the two hooks, or a
  PreToolUse hook that failed open before its `hook.dialog` frame. It is not "unreachable".
- Building the body has a measured cost (research M3): a new undocumented CLI dependence needs its own ledger row and
  `viola verify` probe, then a re-stamp of both installed versions — at least 8 live sessions against the 3 left, and
  a 15th row this scope rules out. Whether the `null` stays (with M1–M3 as its recorded reason) or a body is built is
  P4's, with that cost stated.

### W6 — reword a stale owner name (CARRY 3)
- `tests/cli_answer.rs:9` reads "The `permission` kind's end-to-end case is owed to the live test (working-route
  `:84`)". W3d-a lands the case in this file, so the line is rewritten to describe the case it now holds, with this
  entry's name and coordinate (`:88`) as its origin.
- [premise-corrected: research — `relayed()` reads `fixtures/claude/2.1.287/` and `path4.json` replays the relayed
  `ask-user-question` / `exit-plan-mode` variants, still committed (superseded but kept)] The header's `:7-8` is
  accurate for the existing case at HEAD. It turns partly wrong once the permission case lands, because that case
  replays the recorded `permission-1` files, so W6 rewrites those lines with `:9`.

### F1 — the designed-floor exception to the timing-red rule (the operator's fold, overseer-decided; inputs#I1)
- `.claude/rules/testing.md:70` (2026-09-28): "A timing red is never fixed by raising a timeout or a test bound…". The
  amendment adds the exception chunk :86 used for its `verify_window_` class (that chunk's
  `evidence/verify-window-class.md`). A bound may move only when both hold:
  - the slow part is a floor that measurement across runs shows is by design (not a regression, not contention);
  - a planted-hang control proves that a hang is still killed under the moved bound.
- Who writes it: the file's `## Session Additions` is owned by `/wrap-session` (its own line, verified), so the lean
  is that this chunk decides the amended text and the wrap's curation applies it (an Expected amendment).
- [premise-corrected: research M4 — :86 recorded one control, for the 20 s override only] The class already has a
  one-off planted-hang control for its 20 s binary override: a planted `sleep(40 s)` after `verify` under `--profile
  ci` read `TIMEOUT [20.003s]`, and `PASS [43.346s]` under the default profile. The 45 s `test(/verify_window_/)`
  override has none. This chunk runs that one control the same way, one-off, with both readings recorded in
  `evidence/` (no committed test, no new seam, no `.config/nextest.toml` change), so the class meets the exception's
  own condition in full.

## Live sessions (operator directive)
- Target: zero. W3d-a, W6 and F1 need none. W3d-b needs none if the static read settles it. Any live session the plan
  needs is named and counted in it; the founder's cap has 3 of 16 left. Counting convention (the previous chunk's
  research M13): a full `viola verify` run spends three sessions or more, and is not planned here.

## Sizing
- One builder window. Epoch 3 stays one epoch: if the work does not fit, P4 brings a split card for entries inside
  Epoch 3, never a new epoch boundary.

## Boundaries (not this chunk)
- "First live test and self-drive" (`:92`) owns: "the dialog never renders" (`verification-matrix.json#v1-31`); where
  its live proof runs; the Windows-only live items (H2, the DA1 stall, F-W3's real-terminal mouse report); the Epoch 3
  boundary audit's Windows-dispatch survivors; `local-live`'s live firing.
- "Local-command and paste-framing rows" (`:90`) owns: the local-command rows and `unconfirmable`, the long-paste
  wrapper, tag escaping, the harness prefixes, the R8 identity floor (`v1-29`).
- No ledger row is added and no re-stamp is planned: the dialog tier (14 rows) landed at the previous chunk.
- No Claude credential on any CI runner.
- R-S3: the U02 setup upgrade and the `host-win32.md` regenerate run at the Epoch 3 boundary ritual, not here.
- The founder's dated security exceptions stay as they are, including the sixth (`viola answer` and `viola hook
  pre-tool-use|permission-request`'s `hook.dialog` frame). This chunk widens none of them (owner: Epoch 6).

## Surfaces and contracts it touches
- `tests/cli_answer.rs` — the `permission` case (W3d-a), the header reword (W6).
- `tests/support/` — fake-agent and home helpers the case needs.
- The fake agent and `fixtures/claude/{2.1.288,2.1.287}/` — `PermissionRequest.permission-1.json` and
  `PostToolUse.permission-1.json` replay.
- `crates/viola-agent-claude/src/dialog.rs` — the PermissionRequest mapping. A question's body only if W3d-b's
  measurement calls for one.
- `src/cmd/hook/` — the `permission-request` arm. `src/run/wait.rs` — the wake set (unit-proven at HEAD).
- `.config/nextest.toml` — read, not changed: F1's one-off planted-hang control runs under its `ci` profile and is
  reverted (research M4).
- `.claude/rules/testing.md:70` — F1's amendment (wrap curation, per the lean above).
- `ledger/stamps.json` — not touched (verify stays its only writer; no re-stamp here).
- Specs: test-plan §6 Path 4 and §10; architecture §Standard Contracts (`hook.dialog`, `answer`) and [Hook Contract];
  security-plan (the dialog-decision rule; the sixth dated exception); obs-plan §6 (dialog events).

## CI read at take-up (Setup 5a: every commit from the last wrap's flip `2bd08e9` through HEAD)
- `2bd08e9` (the 2026-10-05-dialog-rows-and-re-probe wrap): green, ci#37328148791, 15/15 checks, wall 415 s.
- Nothing red to fold.
