# Scope — 2026-10-19-dialog-hold-for-the-driver

**Working entry (title, verbatim):** Dialog hold for the driver — a longer or configurable dialog hold and a record of
an expired hold (gap-list group 2)

**Epoch:** Epoch 5 — Driver surface · **version:** viola-0.1.0

## What it builds

- A dialog the wrapper holds for a driver's answer can be held long enough for a driver that relays the dialog to a
  person and back: the hold is longer than today's compiled `DIALOG_DEADLINE` (60 s, "PROVISIONAL, unmeasured",
  `crates/viola-core/src/lib.rs:16-19`), or configurable, or both (the entry: "a longer or configurable dialog hold").
- An expired hold leaves a record: today the hold's end at the deadline writes no event line, `wait` reports
  nothing, and a late `answer` is refused `unknown-dialog` (gap-list item 28,
  `viola-0.1.0/chunks/2026-10-08-first-live-test-and-self-drive/evidence/gap-list.md`). The entry: "a record of an
  expired hold".
- The record is a normalised event line in `events.ndjson` (a new kind beside `question` / `permission` / `plan`,
  carrying `dialog_id` and the dialog kind, no upstream text), so `wait` (CLI and the `mcp` `wait` tool) wakes on it
  and a driver learns the dialog is gone; the event catalog (obs-plan), the closed kind set (`viola-core`), the
  event-line contract (architecture §Standard Contracts) and the human `wait` line (`src/human/wait.rs`,
  `src/cmd/wait.rs`) each take it. (Closed at P3: a wrapper line reaches `wait` through `WaitFeed::appending`,
  `src/run/send/slot.rs:228`, and wakes it only when its kind is in `EventKind::WAIT_WAKE`, `src/run/wait.rs:200-206`;
  the logged `outcome` derives from the same set, `crates/viola-channel/src/server.rs:417`.)
- [premise-corrected: the wrapper replies `null` at its own deadline, 5 s before the hook's bound
  (`src/run/dialog.rs:281`, `src/cmd/hook.rs:64-70`), so the hook bound and `hooks.json` need only outlast the LONGEST
  hold] The hook side follows the hold's ceiling, not each start's hold: each dialog hook's `hooks.json` `timeout`
  (75 s today, `plugin/hooks/hooks.json:10,13`) must outlast the longest hold by at least 10 s (architecture [Hook
  Contract]; `hooks_json_dialog_timeouts_outlast_the_dialog_deadline`, `crates/viola-agent-claude/src/lib.rs:617-641`),
  and the hook's own bound (`DIALOG_DEADLINE + REPLY_GRACE`, `src/cmd/hook.rs:64-70`) moves with it. A configurable
  hold therefore leaves the rendered `timeout` a compiled literal and `plugin/<key>/` byte-identical across starts
  (`replace_private_shared`, `src/cmd/run.rs:403-409`).
- That the CLI honours a dialog hook `timeout` far past 75 s (the prototype held 3500 s, gap-list item 27, guide
  L55-56; a prototype hold of 58 min is recorded in
  `viola-0.1.0/chunks/2026-10-17-mcp-server-for-drivers/inputs/I2-relay-2.md.txt`) is a CLI behaviour viola has not
  measured on 2.1.287 — under the project's rule a capability-ledger row with a `viola verify` probe, or a measured
  live reading, or a ruled limit. (Closed at P3: no ledger row names it, `LedgerRow::ALL` is 24,
  `crates/viola-agent-claude/src/ledger.rs:49`; the hold engages only on a stamped CLI with the wheel `driver`,
  `src/run/dialog.rs:199-202`, and no standing home holds a stamp of the 24-row set, so a live reading needs its own
  `viola verify` first.)
- The reply at expiry is a product question: today `null` (the CLI's own menu, left to the human,
  `src/run/dialog.rs:273-306`); the prototype denied with a fixed message and the driver answered later as a prompt
  (gap-list item 28). A viola-built deny at expiry would be viola deciding a dialog no one answered. (Closed at P3.)
- At HEAD an expired hold leaves no line saying so: the wrapper writes no `dialog-answered` at expiry
  (`ObsEvent::DialogAnswered` only in `answer`, `src/run/dialog.rs:372-381`), and the hook, handed a `null` reply,
  writes `hook-decision` with `deadline_hit:false` (`src/cmd/hook.rs:305-336`) — obs-plan §4's Dialog → `answer`
  scenario names `dialog-answered{…deadline_hit…}` and a `hook-decision{deadline_hit:true}` at the deadline. Closed at
  P3 (read at HEAD): the chunk writes `dialog-answered` with `deadline_hit:true` at expiry; the hook's
  `deadline_hit:true` stays its own backstop's (the wrapper unreachable), and obs-plan's sentence is amended to say so.

## Boundaries

- The human always wins: a human key during a held dialog still hands it back at once (`hand_back`,
  `src/run/dialog.rs`), however long the hold.
- No config key, env var or flag may disable a control; a hold setting is bounded (a closed range), never "forever".
- Not in this chunk: gap-list groups 3 (screen read), 4 (`allow <n>`) and 5 (the start line and scripts, outside this
  repository); the switch itself and `v1-33` (owned by "Overseer switch to viola").

## Folded freight

- CARRY (minted at the 2026-10-17-mcp-server-for-drivers wrap from its P5 review card's row 1, approved by the
  operator's word, `viola-0.1.0/chunks/2026-10-17-mcp-server-for-drivers/inputs/I3-relay-3.md.txt`): carrying out the founder's own answer to P4 fork Q3 «Гр. 2: удержание
  диалога» (`viola-0.1.0/chunks/2026-10-17-mcp-server-for-drivers/inputs/I2-relay-2.md.txt`, relayed by the
  overseer): gap-list group 2 alone must close before the overseer switch; `DIALOG_DEADLINE` is a compiled 60 s,
  PROVISIONAL, and no route entry named the group before this one.
- PREREQ: close rust gate deferral (deferred since 2026-10-18-plugin-precedence-live-readings: clippy (entry 5) —
  record: that chunk's `report.md:193`, `cargo clippy --workspace --all-targets --features fake-agent -- -D warnings`
  "not run — defer (zero Rust delta)"). This chunk's plan runs clippy; it closes with this chunk's Rust delta.
- watch: `run::send::budget::tests::send_budget_paused_is_refused_with_the_window_word_and_no_cursor` case_4 red once
  under a plain `cargo test` (libtest threads; its log capture found no `send-refused` line), green on three re-runs
  and in every nextest gate (0/3; since 2026-10-18-compact-post-condition-ledger-row; retires: a recurrence, or 3
  green runs).

## CI read at take-up (Setup 5a, `ci-…` trail in the phase run dir)

- `daf81f4ab91f` (the last flip, 2026-10-18-plugin-precedence-live-readings) — green · checks 15/15 · wall 672 s ·
  ci#39215959439.
- `55699c4f2d26` (the 0-pending wrap's route adaptation, HEAD) — in progress · ci#39216864300: verdict not yet
  available, read neither green nor red.

## Inputs

- `inputs#I1` — the operator's take-up direction: the founder-owned P4 forks (a boundary widening, a live `claude`
  start cap, the hold length if it is a product decision) go to him as one dialog.
