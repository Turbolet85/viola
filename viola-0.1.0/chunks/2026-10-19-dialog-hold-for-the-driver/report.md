# Report — 2026-10-19-dialog-hold-for-the-driver

**Chunk:** a longer or configurable dialog hold and a record of an expired hold (gap-list group 2)
**Date:** 2026-10-19
**Commits:** `3def5b5b chore(2026-10-19-dialog-hold-for-the-driver): operator pre-CI commit, for the run this chunk's
verdict reads` · `d146c50c fix(2026-10-19-dialog-hold-for-the-driver): a held dialog's pending_dialog is on the snapshot
before its line is logged, and cleared when the append fails` (since `last_wrap` 2026-10-19T07:33:54Z; the 0-pending
wrap's `55699c4f` before them)

## Changes (structured — detectors read this)
- **Files:** 14 sources, all on research's Files to modify (`gate.py scope`: `clean — changed 14 · listed 14 ·
  recorded 0`, base `55699c4f`, the parent of pre-CI commit `3def5b5b`):
  - `crates/viola-core/src/lib.rs`
  - `src/run/dialog.rs`
  - `src/cmd/run.rs`
  - `src/cmd/hook.rs`
  - `plugin/hooks/hooks.json`
  - `crates/viola-agent-claude/src/lib.rs`
  - `src/obs.rs`
  - `schemas/config.v1.json`
  - `src/cmd/wait.rs`
  - `src/human/wait.rs`
  - `src/run/wait.rs`
  - `crates/viola-state/tests/state_events.rs`
  - `crates/viola-state/src/events.rs`
  - `tests/cli_controls_not_disableable.rs`

  New, in the chunk folder: `evidence/live-pty.py` and `evidence/live-ledger.py` (byte copies of
  `chunks/2026-10-18-plugin-precedence-live-readings/evidence/`, `cmp` 0 each), `evidence/live-sessions.ndjson`,
  `evidence/live-round.md` and `evidence/operator-pass.md`.
- **Symbols / APIs** (coordinates copied from `## New text, by line`):
  - `viola_core::DIALOG_HOLD_DEFAULT` (60 s), `DIALOG_HOLD_MIN` (60 s) and `DIALOG_HOLD_MAX` (3600 s) are the hold's
    homes, each doc-commented PROVISIONAL (`crates/viola-core/src/lib.rs` 16-27). They replace
    `viola_core::DIALOG_DEADLINE` (60 s), which is **removed**: its readers were `src/cmd/hook.rs`,
    `src/run/dialog.rs` and `crates/viola-agent-claude/src/lib.rs`. The count entry reads `0` over `src`, `crates`
    and `tests`.
  - `viola_core::DialogHold`: a nutype over `u16`, `greater_or_equal = 60, less_or_equal = 3600`, with `duration()`
    and `Default` = `DIALOG_HOLD_DEFAULT` (`lib.rs` 248-271; `impl DialogHold` 257-261, `impl Default` 263-270). It
    is the only way to build a hold, so no value makes it zero or unbounded.
  - `viola_core::EventKind::DialogExpired`, kebab `dialog-expired`, `data_keys` `["dialog_id", "dialog_kind"]`.
    `EventKind::ALL: [Self; 16]` (was 15, the new kind last). `EventKind::WAIT_WAKE: [Self; 6]` (was 5): `turn-ended`
    · `question` · `permission` · `plan` · `session-end` · `dialog-expired`. `wait_outcome`
    (`crates/viola-channel/src/server.rs`) and `src/run/wait.rs`'s scan derive from `WAIT_WAKE` unchanged, so the
    channel `outcome` and both `wait`s (CLI and the `mcp` tool) take the kind with no edit.
  - `run::dialog::read_hold(home) -> (DialogHold, Vec<ConfigRejection>)` (`src/run/dialog.rs` 155-168, with
    `parse_hold` 170-182 and `whole_hold` 184-187): the twin of `run::budget::read_thresholds`.
  - `DialogSlot::hold_for(self, hold: Duration) -> Self` (`dialog.rs` 219-223). `DialogSlot::new` keeps its seven
    arguments and starts at `DIALOG_HOLD_DEFAULT`; its callers are `src/cmd/run.rs` (the start, which chains
    `.hold_for(hold.duration())`, `run.rs` 288-299), `run.rs`'s in-file `methods` test helper and
    `src/run/wheel.rs`'s test helper (both unchanged, at the default) and `dialog.rs`'s own test helpers.
  - `DialogSlot::await_answer`: its deadline is `clock.now() + self.hold`, and the span `run.dialog_await`'s
    `deadline_ms` reports `self.hold`. At expiry (`deadline_hit`) it calls the new `record_expiry` (`dialog.rs`
    372-394): one `dialog-expired` event line (`source: wrapper`, `{dialog_id, dialog_kind}`, no question, plan or
    tool text) appended through `WaitFeed::appending`, so a parked `wait` wakes, and one `obs_event!(INFO,
    DialogAnswered, corr = dialog_id, dialog_kind, deadline_hit = true, duration_ms)` with no `from`. A failed append
    changes nothing else, and the reply stays `null`. A hand-back or an answer writes neither.
  - `DialogSlot::register` (the operator-pass fix, `d146c50c`): when the dialog is held, `pending_dialog` is written to
    `snapshot.json` **before** the dialog's event line is appended, and written back to none when that append fails.
    It was written after the append, beside the `dialog-raised` obs line, since chunk
    2026-10-04-dialog-answers-by-dialog-id (`dialog.rs` 296-310).
  - `src/cmd/hook.rs`: `dialog_deadline(started)` = `started + DIALOG_HOLD_MAX + REPLY_GRACE` = 3605 s (was
    `DIALOG_DEADLINE + REPLY_GRACE` = 65 s). The hook's bound now follows the ceiling, not each start's hold: the
    wrapper replies at its own hold's end.
  - `src/cmd/wait.rs`: `DIALOG_KINDS: [&str; 4]` adds `dialog-expired`, and `fn dialog_form(kind)` (39-41) decides the
    human line form. `viola wait` woken by `dialog-expired` prints `dialog-expired  <name>  dialog <id>  cursor <n>`
    (the planning form; the founder's wording owed, `inputs#I3` row 3).
  - No IPC method, endpoint, port, socket or env var was added or changed.
- **Crates / modules:** none added or removed. Changed: `viola-core`, `viola-agent-claude` (tests only), `viola-state`
  (tests only), and the root bin's `run::dialog`, `cmd::run`, `cmd::hook`, `cmd::wait`, `human::wait`, `obs` and
  `run::wait` (tests only).
- **Dependencies:** none (no manifest or lockfile change; the preservation guard is green).
- **Schema / config:**
  - `config.json` gains the key `dialog_hold_s`: a whole number of seconds, 60–3600, default 60. It is read once at the
    wrapper's start through `take(MAX_FRAME)` and `v` = 1, right after `read_thresholds`. Absent, unreadable or
    malformed, or a `v` other than 1, gives the default with no rejection (`read_diagnostics_level` reports the
    document). Any other value (0, 59, 3601, -1, 600.5, `"600"`, `null`, `true`) gives 60 plus exactly one
    `parse-rejected{parser:"config-json", detail:"malformed"}`. Ratified as a boundary widening by the founder's own
    answer (`inputs#I2` Q1 «Ключ, по умолчанию 60»).
  - `src/obs.rs` `CONFIG_KEYS: [&str; 6]` (was 5) adds `dialog_hold_s`, so a document holding it reads no
    `unknown-keys`.
  - `schemas/config.v1.json` gains `"dialog_hold_s": { "type": "integer", "minimum": 60, "maximum": 3600 }`.
  - The embedded `plugin/hooks/hooks.json`: the two dialog hooks' `"timeout"` (PreToolUse `AskUserQuestion|ExitPlanMode`
    and PermissionRequest) go from **75 to 3615**. The spine `timeout` 5 entries are unchanged, and the file is still
    rendered by `@@VIOLA_BIN@@` substitution alone, so `plugin/<key>/` stays byte-identical across starts.
  - The event-line contract gains `dialog-expired` `{dialog_id, dialog_kind}`, `source: wrapper`. It is the first
    wrapper-appended kind that wakes `wait`.
  - No redaction or scrub shape changed.
- **Spec-master edits:** none (the masters are amended at this wrap's P2).
- **Retired:** `viola_core::DIALOG_DEADLINE` (the constant; its meaning moves to `DIALOG_HOLD_DEFAULT` /
  `DIALOG_HOLD_MAX`). No census row.
- **Counts / qualifiers moved** (each with its pattern id; basis `cascade.py sweep --pre-pass`, trail
  `.andromeda/runs/2026-10-19T09-28-14-wrap/cascade-2026-10-19-dialog-hold-for-the-driver-pre-pass.json`, baseline
  HEAD `d146c50c`):
  - The dialog hold: compiled 60 s (`DIALOG_DEADLINE`) → `config.json` `dialog_hold_s`, 60–3600, default 60
    (`DIALOG_HOLD_DEFAULT` / `_MIN` / `_MAX`). Pattern `deadline`: 5 standing rows.
    - `architecture.md:66` ×3: stale.
    - `architecture.md:67` @c9228: no-change (the hold's end, a true claim sharing the token, reworded only if
      the detector judges it).
    - `security-plan.md:202` @c1199: stale.
    - `obs-plan.md:758`: no-change (the statusline bound, "not a dialog deadline").
    - `a11y-plan.md:285` ×2 @c605/@c623: stale.
  - The hooks.json dialog `timeout` 75 → 3615. Pattern `timeout75`: 4 standing rows.
    - `architecture.md:66` @c2568: stale.
    - `architecture.md:95` @c2583: stale.
    - `architecture.md:309`: stale (the "`timeout` 75" note).
    - `a11y-plan.md:285` @c683: stale.
  - The hook's reply bound 65 s (`DIALOG_DEADLINE + 5 s`) → 3605 s (`DIALOG_HOLD_MAX + 5 s`). Pattern `hookbound`: 2
    standing rows.
    - `architecture.md:66` @c2461: stale.
    - `security-plan.md:202` @c1199: stale.
  - Event kinds 15 → 16 (rule: the length of `EventKind::ALL`). Pattern `kinds15`: 1 standing row.
    - `test-plan.md:574` ×2: stale.
  - The wake set 5 → 6 kinds (rule: `EventKind::WAIT_WAKE`). Pattern `wake5`: 4 standing rows.
    - `architecture.md:223`: stale (the five listed).
    - `test-plan.md:206`: stale.
    - `test-plan.md:517`: stale.
    - `obs-plan.md:596`: stale (the `outcome` list of five).
  - "Wrapper-appended records are log-only" gains one exception. Pattern `logonly`: 4 standing rows.
    - `architecture.md:254` ×2: stale by omission (the wrapper-appended paragraph; `dialog-expired` is
      wrapper-appended and wakes `wait`).
    - `architecture.md:67` @c8729: no-change (the `wheel` event alone).
    - `architecture.md:154`: no-change (`activity`).
    - `design-system.md:182`: no-change (`activity`).
  - `CONFIG_KEYS` 5 → 6 names, and `config.json` gains a key. Pattern `cfgkeys`: 6 standing rows.
    - `architecture.md:339`: stale by omission (the key list).
    - `architecture.md:366` ×2: stale ("the five `CONFIG_KEYS`"; the schema's properties).
    - `test-plan.md:594`: stale by omission (the controls table's history).
    - `test-plan.md:596` ×2: no-change (the set-equality rule, no count).
    - `a11y-plan.md:285` @c2746: no-change (the budget keys).
    - `registries/contracts/obs-plan/otel-sdk-init.md:17` ×2: stale ("`CONFIG_KEYS` holds five names"; the start's
      reads after `ProcessStart`).
  - The controls table `SETTINGS` 9 → 10 rows (rule: `SETTINGS` in `tests/cli_controls_not_disableable.rs`). Pattern
    `ctrltable`: 11 standing rows.
    - `test-plan.md:594`: stale by omission (gains the hold row and its booted case).
    - `test-plan.md:571`: no-change (the budget case).
    - `test-plan.md:841`: no-change (the seam row).
    - Label (other `SETTINGS` identifiers, `DIALOG_SETTINGS` / `SETTINGS_FLAG` / `USER_SETTINGS`), 8 rows:
      `architecture.md:87`, `:310`, `:311`, `:348`; `security-plan.md:115`, `:216`, `:247`; and
      `registries/contracts/architecture/crate-dependency-direction.md:6` ×2.
  - `dialog-answered` at expiry with `deadline_hit` true; the hook's `deadline_hit:true` becomes its backstop only.
    Pattern `deadlinehit`: 9 standing rows.
    - `obs-plan.md:615`: stale (the hook's `hook-decision{deadline_hit:true}` "on deadline"; at the wrapper's hold end
      the hook is handed `null` and logs `decision_emitted:false, deadline_hit:false`).
    - `obs-plan.md:610`: stale by omission (`from` / `from_trust` absent at expiry).
    - `obs-plan.md:291`, `:607`, `:611`, `:747`, `:748`: no-change (field lists).
    - `obs-plan.md:642`, `:645`: no-change (statusline).
  - `pending_dialog` written before the dialog's line (the `d146c50c` fix). Pattern `pendingsnap`: 4 standing rows.
    - `architecture.md:206`: stale by omission (when the field is written).
    - `architecture.md:49` @c701: no-change (the wait feed).
    - `architecture.md:192`: no-change (the replay's absent fields).
    - `architecture.md:200`: no-change (the snapshot shape).
  - `dialog-expired` and the `wait` line form stand in no master. Patterns `expired` and `waitline`: 0, the refusal
    `control: expired, waitline, owner-title, owner-generic never fired over the seven masters' pre-pass text
    (d146c50c)` its basis.
  - Owner pattern `owner-title` ("Dialog hold for the driver"): control unfired, the same refusal line its basis.
  - Leaf rows (19 over 10 files) are the cascade's: `CLAUDE.md:20`, `.claude/rules/events.md:17`,
    `.claude/rules/security.md:28`, `.claude/docs/services/viola-core.md:6`, `:20`, `.claude/docs/services/viola.md:24`,
    `:41`, `:43`, `.claude/docs/services/viola-state.md:45`, `.claude/docs/services/viola-channel.md:23`,
    `.claude/docs/services/viola-agent-claude.md:22` (label), `.claude/docs/tests-summary.md:27`,
    `.claude/docs/obs-summary.md:69`, `:72`.
  - Sweep reach: 10 patterns · 13 moved facts, 13 with a pattern (the report's count: the ten fired, `expired` and
    `waitline` with their refusal, and the matrix note below with `no master site`) · owner pattern control unfired ·
    by line 3 lines · 4 offsets.
- **Dev-tool versions:** none. claude 2.1.287 was re-read by path (`2.1.287 (Claude Code)`) on the Linux dev host
  before the live round, unchanged.
- **Harness / gate surface:** none (no `scripts/agent-run.*`, harness verb, CI step or status shape changed). The plan's
  23 `[[gate]]` entries are unchanged.
- **Cross-project / external claims:**
  - CI on `3def5b5bafc6` (the pre-CI commit): **red**, ci#39226239371, attempt 1, `failed 1: test (macos-latest)`
    (`viola::cross_list_rows cross_list_rows_style_each_span_beside_its_word_on_a_terminal`, the board read
    `DIALOG none` on a held dialog). Cause, read from the code, not reproduced on a macOS host: the line-before-snapshot
    order in `register`, older than this chunk. Fixed by `d146c50c` (Deviation 4).
  - CI on `d146c50c6fe2` (the final HEAD): **green**, ci#39228538322, attempt 1, checks 15/15, wall 652 s.
  - Live CLI: claude 2.1.287 on the Linux dev host (`evidence/live-round.md`).
  - Inputs (`inputs.py verify`): `inputs: 4 entries — unchanged 0 · drifted 0 · vanished 0 · broken 0 · altered 0 ·
    unreachable 0 · n/a 4 · uncited 2 · unparsed 0`, before this report existed.
    - `I1 · message: the operator, the /andromeda-phase invocation, 2026-10-19T07:38Z · copy message · n/a` (cited by
      scope.md, research.md and plan.md).
    - `I2 · message: the founder, live in the overseer dialog 2026-10-19T08:04Z, relayed verbatim by the operator ·
      copy message · n/a` (cited by research.md and plan.md).
    - `I3 · message: the operator, in this session, at the P5 review prompt, 2026-10-19 · copy message · n/a`, cited
      here as `inputs#I3`. It names the card `.andromeda/runs/2026-10-19T07-38-25-phase/p5-card-1.md`, whose
      `## Beyond the plan` rows are, verbatim, under the operator's word «yes» on the card as shown (`relay-3.md` of
      that phase run; the operator's word, not the founder's, PROVISIONAL on any fork the founder owns):
      1. **Owner of the hold's Windows leg:** "Windows-only live measurements" (an existing entry): a CARRY naming the
         live reading of a dialog hook held past 75 s on claude 2.1.287 (measured on the Linux dev host at this chunk).
      2. **Owner of the 3600 s ceiling beyond the measured ~660 s:** "Overseer switch to viola" (an existing entry): a
         CARRY saying that its live switch, with the overseer's home at `dialog_hold_s` 3600, is where a hold near the
         ceiling is first read live. The ceiling stays PROVISIONAL until then.
      3. **The founder's wording owed** for the `dialog-expired` kind word and its `wait` line (design-system /
         layout-templates planning forms): carried on the handoff for his Epoch 5 boundary batch.
    - `I4 · message: the operator, the /andromeda-wrap-session invocation, 2026-10-19T09:28Z · copy message · n/a`,
      cited here as `inputs#I4`: the macOS red at `3def5b5b` had a cause older than this chunk, and its fix `d146c50c`
      is this chunk's, recorded as a deviation; the clippy PREREQ closes; the founder's wording for `dialog-expired` and
      its `wait` line is owed to his Epoch 5 batch; write the card's three Beyond-the-plan rows.
- **Reverted / negative API facts:**
  - `DialogSlot::new` taking the hold as an eighth argument was written, then reverted for `hold_for`. It tripped
    `clippy::too_many_arguments` (an `#[expect]` was added), and it forced an edit of `src/run/wheel.rs`, which the
    preservation guard read red.
  - A `wait`-based form of the order case (a parked `wait` woken by the dialog line, then a snapshot read) was written
    and replaced: it stayed green 12 of 12 runs on the old order and cannot separate the two orders on this host.
- **Insufficient fixes (written, kept, not the remedy):** none.
- **Spec claims disproved by measurement:**
  - "On deadline the hook emits `hook-decision{decision_emitted:false, deadline_hit:true}`" (`obs-plan.md:615`;
    pattern `deadlinehit`). At HEAD before this chunk the wrapper already replied `null` at its own deadline, 5 s before
    the hook's bound, so the hook logged `deadline_hit:false` (research.md, scope.md's premise-corrected bullet; read
    at P3, not measured live). The hook's `deadline_hit:true` is written only when its own read times out.
  - The live round measured the answer path, not an expiry (`deadline_hit` false at 671 s on both sides).
- **Expected amendments (from plan):**
  - architecture [Hook Contract] (the hold, the hook's bound, the timeout, `null` at expiry, one `dialog-expired`):
    carried by Symbols / APIs and Schema / config. Patterns `deadline` (architecture 2: :66, :67), `timeout75`
    (architecture 3), `hookbound` (architecture 1). Pattern "Boundary widening the founder answered himself"
    (`inputs#I2` Q1).
  - architecture §Occupied Resources (`config.json`, `hooks.json` `timeout` 75 → 3615): carried by Schema / config.
    Patterns `cfgkeys` (architecture 3: :339, :366 ×2), `timeout75` (architecture :95, :309).
  - architecture §Conventions (normalised kinds) and §Standard Contracts (Event `data`, `wait`): carried by Symbols /
    APIs. Patterns `wake5` (architecture :223), `logonly` (architecture :254), `kinds15` (architecture 0; test-plan 1).
    The architecture kind list carries no count, so the new kind itself has no standing site (`expired`: control
    unfired).
  - architecture §Cross-cutting Patterns, Config management: carried by Schema / config. Pattern `cfgkeys`
    (architecture :339).
  - security-plan §Input Validation (Configuration values row; Hook stdin row, the bound 3605 s): carried by Schema /
    config and Symbols / APIs. Patterns `hookbound` / `deadline` (security-plan 1: :202). The Configuration values row
    names no key by these patterns (0 security-plan hits on `cfgkeys`); its sweep is the detector's.
  - obs-plan §4 Scenario: Dialog → `answer` (expiry `dialog-answered{deadline_hit:true}` with no `from`; the hook's
    `decision_emitted:false, deadline_hit:false` on a `null`; `deadline_ms` the configured hold), plus the `wait`
    scenario's `woken_kind` / `outcome` and §6: carried by Symbols / APIs and Spec claims disproved. Patterns
    `deadlinehit` (obs-plan 9: :615, :610 stale) and `wake5` (obs-plan :596).
  - test-plan §5 Cross-module patterns (On-disk: sixteen kinds; CLI: the controls table at ten rows) and §4 (the wake
    set at six): carried by Counts / qualifiers moved. Patterns `kinds15` (test-plan :574), `ctrltable` (test-plan
    :594), `wake5` (test-plan :206, :517).
  - design-system §Surface: cli §Component Patterns 3 and layout-templates §Output structure — `viola wait` / `viola
    last` (`dialog-expired` takes the dialog form, the founder's wording owed): carried by Symbols / APIs. Pattern
    `waitline`: 0 (refusal line), so the sites are the detectors' to locate.
  - a11y-plan §8 Cognitive Accessibility → Timeout extensions: carried by Schema / config. Patterns `deadline` /
    `timeout75` (a11y-plan 1: :285 ×2).
  - architecture [Hook Contract] (the live reading: a dialog hook held about 660 s returned the driver's decision on
    2.1.287, Linux; the Windows leg owed): carried by Outcome's live criterion and `evidence/live-round.md`. Pattern
    `deadline` (architecture :66).
  - The plan's metadata note on `v1-21` (no claim; it stays pooled): `no master site — a matrix note written at P5
    (phase run `note-v1-21.md`)`.
- **Coverage of new surfaces:**
  - `config.json` `dialog_hold_s` → validation `DialogHold` nutype + `take(MAX_FRAME)` + `v` = 1 ✓ ·
    instrumentation `parse-rejected{config-json, malformed}` ✓ · PII n/a · tests unit (`read_hold_*`) + integration
    (`setting_does_not_unbound_the_dialog_hold`, booted) · a11y n/a · tokens n/a.
  - `dialog-expired` event line → validation closed `EventKind` + `data_keys` ✓ · instrumentation
    `dialog-answered{deadline_hit:true}` ✓ · PII no upstream text ✓ · tests unit
    (`hook_dialog_expiry_writes_one_dialog_expired_line_and_one_answered_line`, `wait_each_wake_kind_wakes` row) +
    integration (`state_events.rs`) · a11y n/a · tokens n/a.
  - `viola wait` line for `dialog-expired` → validation n/a · instrumentation n/a · PII escaped writer ✓ · tests unit
    (`wait_line_form_dialog_expired_takes_the_dialog_form`, the `human::wait` pin) · a11y ASCII, no SGR ✓ · tokens n/a
    (plain).
  - The hook's dialog bound and the `hooks.json` timeouts → validation n/a · instrumentation `hook-decision` unchanged
    ✓ · PII n/a · tests unit (`dialog_deadline_is_the_longest_hold_plus_the_reply_grace`,
    `hooks_json_dialog_timeouts_outlast_the_longest_hold`) + live (`evidence/live-round.md`) · a11y n/a · tokens n/a.

## Deviations from intent
1. **The hold reaches the slot through `DialogSlot::hold_for`**, not as an argument of `DialogSlot::new` (plan step
   4). As first written, the eighth argument needed a `clippy::too_many_arguments` exemption and an edit of the
   out-of-list `src/run/wheel.rs` (a test helper calls `new`; research's Graph impact missed it). The preservation
   guard (gate entry 11) read red on that edit in the first block firing. The cause was removed: `new` starts at the
   default, the start chains `.hold_for(hold.duration())`, `wheel.rs` was restored to `55699c4f`, the lint exemption
   is gone, and the scope record written for that edit was deleted. The in-file caller in `src/cmd/run.rs` and the
   `slot` helper take the default unchanged.
2. **`dialog_hold_takes_whole_seconds_from_sixty_to_thirty_six_hundred` is a literal-table `#[test]`**, not an rstest
   (step 1). `viola-core` has no rstest dev-dependency, and the guard forbids a manifest edit. It holds the plan's 8
   values.
3. **The live round's first census**, keyed on the bare text `target/debug/viola`, listed the overseer pair's prototype
   bridge (another tree's build of the same name under `~/dev/projects/additional/viola-lab/`). It was re-keyed on this
   repository's path, which read none running before and after. Nothing was done to those processes.
4. **`register` writes `pending_dialog` before the dialog's line, and clears it on a failed append**: the fix commit
   `d146c50c`, outside the plan's steps and inside a listed file. It answers the CI red at `3def5b5b` (the macOS
   `cross_list_rows` case). That red's cause, the line-before-snapshot order, predates this chunk (written at
   2026-10-04-dialog-answers-by-dialog-id). It is folded in as a red met during the chunk, on the operator's word
   (`inputs#I4`). Two unit cases were added: `hook_dialog_held_is_on_the_snapshot_when_its_line_is_logged` and
   `hook_dialog_append_that_failed_clears_the_pending_dialog`. Remove-the-guard readings: the old order red 3 of 3;
   the clear removed, red; both restored (`cmp` 0). The live round ran on the build before this fix (sha256 prefix
   `5152f377a903d8ed`), and the fix touches nothing the round read.

Scope record: none — `gate.py scope` clean, 0 recorded (the one line written at implement, for `src/run/wheel.rs`, was
removed with the edit it recorded, Deviation 1).

## Decisions & corrections
- The founder's cap for this chunk, restated by the operator at the `/implement` invocation (2026-10-19): «ONE viola
  verify plus ONE wrapped claude 2.1.287 start (his dialog 2026-10-19T08:04Z); one dialog answered by viola answer at
  >= 660 s; no key typed into any dialog; ledger each start with date -u before it; a red never re-fired … No mutation
  run. Then the operator pass with the ci.py conclusion read (leg=operator), and stop ready for the wrap.» Kept: 1 + 1,
  the answer at 671.3 s, 0 keys, a `date -u` before each ledger row, 0 reds, no mutation run (`evidence/live-round.md`).
- The operator's wrap directive (`inputs#I4`): the macOS red's cause predates the chunk, its fix is this chunk's and
  recorded as a deviation; the clippy PREREQ closes; the founder's wording for `dialog-expired` is owed to his Epoch 5
  batch; the card's three Beyond-the-plan rows are written.
- Sweep hazard: `gate.py census --args target/debug/viola` matches the overseer pair's prototype bridge
  (`~/dev/projects/additional/viola-lab/prototype/target/debug/viola`), 11 processes read before the round. On this
  host a census of this repository's processes keys on `projects/viola/target/debug/viola`, never the bare relative
  path (a recurrence of host-linux.md 2026-10-07, which names `pgrep -x viola`, in the census tool's `--args` form).
- A test that must prove a WRITE ORDER (state on disk before a line is appended) is not proven by a reader that wakes
  on the line: on the Linux dev host that form stayed green 12 of 12 runs on the reversed order. A capture layer that
  reads the disk at the obs line emitted right after the append, on the same thread, is deterministic: red 3 of 3 on
  the reversed order (`src/run/dialog.rs` `hook_dialog_held_is_on_the_snapshot_when_its_line_is_logged`).
- A standing rule recurred as a runner-only red: events.md 2026-10-16 (a snapshot field a reader expects once a line is
  visible is written before the line). `register` had broken it since 2026-10-04 for `pending_dialog`, met only on
  `macos-latest` (ci#39226239371).
- The plan template's preservation guard reads red on a recorded companion edit (a carried handoff item for the
  operator's word). It recurred at gate entry 11, and was resolved in the design (`hold_for`) rather than argued.
- A plan step asked for an rstest in `crates/viola-core`, which has no rstest dev-dependency while the same plan's
  guard forbids a manifest edit: a planning hazard (that crate's tests are literal tables).
- Guard refusal: the Bash guard refused one `cd` into a skill's references dir at implement Setup. It was re-issued
  once with an absolute path.
- The live round's answer body was built in memory from the saved `wait` document and piped to `viola answer`'s stdin,
  so no file holds the question text (security-plan §Secret Management, obs-plan §8).
- The `ci.md` order held through one red. The record, the fix commit (never an amend) and the pass repeated from step
  1 are in `evidence/operator-pass.md` §5–§6.

## Outcome
Acceptance criteria, against the diff:
- (arch) The hold's homes are `DIALOG_HOLD_DEFAULT` / `_MIN` / `_MAX`, `DIALOG_DEADLINE` reads 0 in every Rust
  source, the rendered `timeout` is 3615 (margin 15 s over `DIALOG_HOLD_MAX`), and the hook's bound is
  `DIALOG_HOLD_MAX + 5 s`: **met** (gate entries 6 and 10; `hooks_json_dialog_timeouts_outlast_the_longest_hold`,
  `dialog_deadline_is_the_longest_hold_plus_the_reply_grace`).
- (arch, security) `dialog_hold_s` is read once at start, every out-of-range or non-integer value gives 60 and exactly
  one `parse-rejected`, no env or flag sets it, and it is a `SETTINGS` row: **met** (`read_hold_*` rstests;
  `setting_does_not_unbound_the_dialog_hold`, 5 cases; `every_flag_variable_and_config_key_is_a_row_of_the_table`).
- (tests) The 120 s hold is pending one `STEP` before and `null` at 120 s; a hand-back or an answer writes no
  `dialog-expired`: **met** (`hook_dialog_holds_until_the_configured_hold_on_the_injected_clock`,
  `hook_dialog_hand_back_writes_no_dialog_expired_line`).
- (arch, obs) An expiry appends exactly one `dialog-expired` (`source` wrapper, keys exactly `dialog_id` /
  `dialog_kind`), wakes a parked `wait`, and logs one `dialog-answered{deadline_hit:true}` under `corr`: **met**
  (`hook_dialog_expiry_writes_one_…`, `wait_each_wake_kind_wakes::case_6_dialog_expired`). The `mcp` `wait` passes
  the same document (no edit).
- (arch, security) At expiry the reply stays `null`, and a late `answer` is `not-delivered` / `unknown-dialog`: **met**
  (the expiry case's late answer; Path 4 in the E2E layer).
- (tests) `ALL` holds 16 and `WAIT_WAKE` 6; `state_events.rs` round-trips 16 with zero counts: **met**.
- (design, layouts, a11y) The `dialog-expired` line is ASCII, without SGR, written once: **met** (the `human::wait`
  pin, `dialog_form`).
- (a11y) Any human key still hands the held dialog back at once: **met**, unchanged (`hand_back`;
  `hook_dialog_held_is_handed_back_null_at_once_on_a_wheel_move`).
- (tests) The gate block is green, and CI is green on the final HEAD: **met** (ci#39228538322 on `d146c50c`).
- (manual, live) **met**: `evidence/live-round.md`, 1 verify (`stamped 2.1.287  24 pass  0 fail`) and 1 start, each
  ledgered first, 0 keys, 0 reds.
  - The answer at 671.3 s.
  - The wrapper's `dialog-answered` `deadline_hit` false, `duration_ms` 671 301.
  - The hook's `hook-decision` `decision_emitted` true, `duration_ms` 671 302.
  - `last` equals `beta`; `dialog-expired` 0.
  - `dialog_pending` true at 300 s.
  - Linux only; the Windows leg is owed (`inputs#I3` row 1).
- (security, obs) No `evidence/` file holds a `CLAUDE*` value, an absolute path, prompt, question or assistant text, a
  session id or a `detail-*` line: **met** (`gate.py hygiene` clean before each commit; the files reviewed at write).
- No matrix capability is claimed: **met** (`matrix.py show --chunk`: 0).

Gates (implement's third firing, on the fix's uncommitted tree; trail
`.andromeda/runs/2026-10-19T08-22-37-implement/gate-2026-10-19-dialog-hold-for-the-driver.json`), each green:
- `cargo fmt --all --check`
- `cargo clippy --workspace --all-targets --features fake-agent -- -D warnings`: closes the PREREQ
- the G1 `rg … instrument` probe (exit 1, no output)
- `bash scripts/lint-probes.sh`
- `agent-run.sh run --unit`
- the named-unit filter
- `run --integration`
- the two-binary integration filter
- `run --e2e`
- the `DIALOG_DEADLINE` count (last line 0)
- the preservation guard
- `cargo deny check`
- `cargo build --bin viola`
- the smoke `cleanup` / `boot` / `status` / `cleanup` (`processes_gone` and `endpoint_gone` true)
- `g2-zero-panics.sh`
- `schema-check`
- `pre-push`

The first firing read the guard red on `src/run/wheel.rs` (Deviation 1). The operator legs, recorded in
`evidence/operator-pass.md`:
- hygiene `clean` before each commit;
- the push entry green twice (`55699c4f→3def5b5b`, `3def5b5b→d146c50c`);
- the `ci.py conclusion` read red on `3def5b5b` (ci#39226239371, `test (macos-latest)`, fixed by `d146c50c`) and green
  on `d146c50c` (ci#39228538322, 15/15).

The Windows-target lint read exit 0 before each push. Smoke: the booted smoke ran as gate entries. The live round is
the boot-path proof.

Watches: `run::send::budget::tests::send_budget_paused_is_refused_with_the_window_word_and_no_cursor` case_4 is not
exercised. Only nextest ran, locally and in CI; no plain `cargo test`. It stands 0/3.

Outcome basis: the operator pass ran. The verdicts above rest on its final state: the commits `3def5b5b`, `d146c50c`
and CI on `d146c50c`, recorded in `evidence/operator-pass.md`. Implement's report is
`.andromeda/runs/2026-10-19T08-22-37-implement/outcome-2026-10-19-dialog-hold-for-the-driver.md`, rewritten after the
red. This window holds the whole run.

Process hygiene:
- The booted smoke supervisor, wrapper and fake agent were started by a leg and terminated by entry 19's `cleanup`.
- `viola verify` and its children were started by this run and exited 0.
- `live-pty.py` → `viola run builder` → claude 2.1.287 were started by this run and terminated by the stop file (3
  presses, exit 0).
- The background `list` / `answer` waiters were started by this run and exited 0.
- Re-measured here: `gate.py census --args projects/viola/target/debug/viola --args viola-live-hold --args hold-live-
  --args p-dhd-smoke --args projects/viola/target/harness` reads `census: none running` (trail
  `.andromeda/runs/2026-10-19T09-28-14-wrap/census-2026-10-19-dialog-hold-for-the-driver.json`).
## New text, by line
Generated by `cites.py added` (cites v1.4); pasted by `splice.py`. No line of this section is typed or edited.
The diff: 55699c4f (the parent of the oldest pre-CI commit 3def5b5b) → the work tree.
A row is a block this chunk added: `{first}-{last}`, `@{head}` its head line where not the first, «the head line».

### crates/viola-agent-claude/src/lib.rs — added 6 line(s) in 5 range(s)
added: 564 · 567 · 618 · 620 · 628-629
### crates/viola-core/src/lib.rs — added 70 line(s) in 16 range(s)
added: 16-27 · 73 · 78 · 94 · 99 · 105 · 131 · 152 · 248-271 · 477-492 · 520 · 529-530 · 536 · 580-584 · 610 · 630
- 257-261 «impl DialogHold {»
  - 258-260 «pub fn duration(self) -> Duration {»
- 263-270 «impl Default for DialogHold {»
  - 264-269 «fn default() -> Self {»
### crates/viola-state/src/events.rs — added 5 line(s) in 2 range(s)
added: 908-911 · 936
### crates/viola-state/tests/state_events.rs — added 7 line(s) in 3 range(s)
added: 56 · 125-129 · 141
### plugin/hooks/hooks.json — added 2 line(s) in 2 range(s)
added: 10 · 13
### schemas/config.v1.json — added 2 line(s) in 1 range(s)
added: 15-16
### src/cmd/hook.rs — added 9 line(s) in 8 range(s)
added: 26 · 64 · 67-68 · 70 · 697 · 701 · 705 · 708
### src/cmd/run.rs — added 16 line(s) in 2 range(s)
added: 263-266 · 288-299
  - 264-266 «for rejection in rejections {»
  - 288-299 «let dialogs = Arc::new(»
### src/cmd/wait.rs — added 22 line(s) in 3 range(s)
added: 36-41 · 101 · 111-125
- 39-41 «fn dialog_form(kind: &str) -> bool {»
- 112-125 @113 «mod tests {»
  - 116-124 @117 «fn wait_line_form_dialog_expired_takes_the_dialog_form() {»
### src/human/wait.rs — added 4 line(s) in 1 range(s)
added: 83-86
### src/obs.rs — added 2 line(s) in 2 range(s)
added: 32 · 38
### src/run/dialog.rs — added 386 line(s) in 22 range(s)
added: 4-9 · 11-13 · 25-26 · 28 · 31 · 75 · 153-188 · 190-191 · 203 · 219-224 · 278-279 · 296-310 · 331-333 · 337 · 340
       358 · 361-363 · 372-395 · 572-589 · 601 · 765-963 · 1221-1279
- 155-168 @158 «pub(crate) fn read_hold(home: &Path) -> (DialogHold, Vec<ConfigRejection>) {»
  - 160-167 @161 «match File::open(home.join("config.json")) {»
- 170-182 «fn parse_hold(bytes: &[u8]) -> (DialogHold, Vec<ConfigRejection>) {»
  - 171-173 «let Ok(Value::Object(map)) = serde_json::from_slice::<Value>(bytes) else {»
  - 174-176 «if map.get("v").and_then(Value::as_u64) != Some(1) {»
  - 177-181 «match map.get(HOLD_KEY).map(whole_hold) {»
- 184-187 «fn whole_hold(value: &Value) -> Option<DialogHold> {»
  - 219-223 @220 «pub(crate) fn hold_for(mut self, hold: Duration) -> Self {»
  - 372-394 @375 «fn record_expiry(&self, pending: &Pending) {»
  - 765-772 «fn hold_of(config: Option<&str>) -> (u64, Vec<ConfigRejection>) {»
  - 774-786 @781 «fn read_hold_takes_a_whole_number_of_seconds_in_range(»
  - 788-804 @797 «fn read_hold_falls_back_to_sixty_with_one_rejection(#[case] bad: &str) {»
  - 806-811 @808 «struct Stepped {»
  - 816-828 «impl StepClock {»
  - 830-838 «impl Clock for StepClock {»
  - 840-847 @842 «fn read_twice_more(clock: &StepClock) {»
  - 849-879 @850 «fn hook_dialog_holds_until_the_configured_hold_on_the_injected_clock() {»
  - 885-895 «impl<S: tracing::Subscriber> tracing_subscriber::Layer<S> for Lines {»
  - 897-904 «fn capture_lines<R>(f: impl FnOnce() -> R) -> (R, Vec<Value>) {»
  - 906-938 @907 «fn hook_dialog_expiry_writes_one_dialog_expired_line_and_one_answered_line() {»
  - 940-962 @941 «fn hook_dialog_hand_back_writes_no_dialog_expired_line() {»
  - 1221-1239 @1224 «fn hook_dialog_append_that_failed_clears_the_pending_dialog() {»
  - 1245-1258 «impl<S: tracing::Subscriber> tracing_subscriber::Layer<S> for RaisedReads {»
  - 1260-1278 @1263 «fn hook_dialog_held_is_on_the_snapshot_when_its_line_is_logged() {»
### src/run/wait.rs — added 1 line(s) in 1 range(s)
added: 364
### tests/cli_controls_not_disableable.rs — added 30 line(s) in 4 range(s)
added: 7-8 · 394-419 · 422 · 447
- 394-418 @405 «fn setting_does_not_unbound_the_dialog_hold(stamped_home: StampedHome, #[case] config: &str) {»
  - 409-413 «let rejected: Vec<Value> =»
