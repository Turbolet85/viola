# Scope — 2026-10-06-local-command-send-outcomes

**Working entry** (`working-route.md:92`, Epoch 3 — Windows slice II: driving verbs and live proof):
Local-command send outcomes — send's unconfirmable outcome with its mirror line, and /clear confirmed by its
new-session post-condition.

## Intent
`send` consumes the compiled local-command list. A driver that sends a ledger-listed local command no longer waits
out the confirmation window to a `not-delivered` / `no-prompt-submitted`: `/clear` is confirmed by its measured
post-condition (a SessionStart with source `clear` and a new `session_id`), and a local command with no measured
post-condition returns `ok {confirmed:false, detail:"unconfirmable", cursor}`, drawn on the readback mirror as
`[  ] unconfirmable`. Nothing is presumed delivered. viola stays mechanism: it reports which of the three happened
and decides nothing for the driver.

## Authority
- The founder's two-chunk ruling (live, 2026-10-06T19:38Z, relayed by the overseer) minted this entry at the
  2026-10-06-local-command-and-paste-framing-rows wrap, after that chunk and ahead of "First live test and
  self-drive" (the entry's first CARRY; verified against that chunk's `scope.md` lines 30-40 and 129-133, where its
  inputs#I2 is cited).
- The operator's directive at this take-up (inputs#I1; closed at P3 against the snapshot, a verbatim copy):
  - no live `claude` session: plan zero, there is no cap;
  - the readiness-gate CARRY goes to P4 as a card with every option priced, the options to include measuring it
    under the fake agent with the paste-hint hold, and leaving the gate as it is. A change to the 5 s maximum, or to
    what `send` does while the hint stands, is not decided before that card;
  - five builders share this host tonight: a baseline red of the stalled-start shape is re-read once no other build
    runs, before it is treated as real;
  - size the chunk against one builder window;
  - Epoch 3 stays one epoch.

## What it builds (the entry's freight folded, in route order)

### W1 — `send`'s local-command outcomes (CARRY 1)
- `send` gains its `ok {confirmed:false, detail:"unconfirmable", cursor}` outcome, with the `[  ] unconfirmable`
  mirror writer (the wire shape verified at `architecture.md:266` and `:275`; the mirror line at
  `layout-templates.md:351` and `:419`: stdout, exit 0, never filled).
- `/clear` is confirmed by a SessionStart `clear` with a new `session_id`.
- `send_window_local_command_is_not_presumed_delivered` changes (verified: `tests/cli_send.rs:373`; at HEAD it pins
  exit 13 `no-prompt-submitted` for `/clear` under `--local-command-mode`).
- `verification-matrix.json#v1-29`: its two owed clauses are owed here (verified through `matrix.py show --id
  v1-29`; the cap is `planned`, unclaimed, its four notes each name this entry or its predecessors): `unconfirmable`
  only for a ledger-listed local command without a measured post-condition, and `/clear` confirmed by its
  new-session post-condition. With them the whole acceptance is reachable, so the cap is claimable here.
  [premise-corrected: one more clause had no witness. "a rewritten path draws a warning" is tested as a
  predicate only, and no case reads the line (research M12, found at P5's premise check). This chunk adds that
  case, so the claim rests on a witness for every clause]
- No live session (the entry; inputs#I1).
- "the list and the row are compiled (`ledger::LOCAL_COMMANDS`, row `local-command-clear`, stamped on 2.1.287) and
  `send` does not consume them" (CARRY 1, verbatim). Verified at P3: `LOCAL_COMMANDS` is at
  `crates/viola-agent-claude/src/ledger.rs:151`, a two-entry list of command and optional post-condition, and
  nothing under `src/run/` or `src/cmd/send.rs` names it (research M1).
- "`SendSlot` holds no `cli_verified` (that chunk's research M5)" (CARRY 1, verbatim). Verified at P3: the struct at
  `src/run/send.rs:49-55` has `clock`, `wheel`, `io`, `in_flight`, `settled`. `run` knows the value at
  `src/cmd/run.rs:209` and builds the slot at `:216`, after the version gate, so it is threaded through
  `SendSlot::new` and no second stamps read is added (research M2).
- Which case a command falls in follows architecture [Delivery Confirmation] (`architecture.md:49`, read whole at
  P3; `:4`, `:457`): a ledger-listed local command whose post-condition is "none" or not yet measured on this CLI
  version returns `unconfirmable`; one whose measured post-condition is met returns `ok`, confirmed. So
  `/remote-control` is `unconfirmable` on every version, `/clear` is confirmed on a verified version, and `/clear`
  on an unverified version is `unconfirmable`. Closed at P3 (research M3):
  - a confirmed `/clear` returns the one confirmed payload, `{submitted_at, cursor}` (`architecture.md:275`), and
    prints the `[RB] read back` line;
  - a listed command whose measured post-condition does not arrive in the window is `not-delivered`
    (`architecture.md:49`: "Any other unconfirmed send is `not-delivered`"). No spec names its detail; the plan
    leans on the existing `no-prompt-submitted` and says so;
  - the text is matched exactly against the compiled list, with no trim, the unmodified text that is typed;
  - "a new `session_id`" has nothing to compare with at HEAD: the wrapper keeps no session id (research M4), so the
    send slot has to remember the id of the last `session-start` it appended.
- The process-log side: `send-confirmed{confirmed:false}` is the `unconfirmable` counterpart (`obs-plan.md:591`,
  `:643`). Verified at P3: `schemas/diag-line.v1.json:59-65` admits a boolean `confirmed` on the three send lines,
  so no schema change is needed (research M5).
- No spec names the `events.ndjson` record of an `unconfirmable` send (the arch extract's open point). Closed at P3
  as a lean for P4: `send-confirmed` with an added `confirmed:false`, the form obs-plan gives the process log and
  design-system asks of the CL-1 records; no reader of that record exists at HEAD (research M5).
- The surfaces, verified at P3: the wrapper's send slot and its hook-event tap, the `send` client's rendering in the
  human and the `--json` view, and the human mirror writers. The fake agent's `--local-command-mode` and
  `--framing` are used as they are: the first skips every slash prompt, the second replays `/clear`'s recorded
  SessionEnd and SessionStart (research M6).

### W2 — the readiness-gate reading (CARRY 2): a P4 card, nothing decided before it
- The operator's disposition at the prior wrap, relayed by the overseer: this entry is the reading's first consumer.
- The operator's directive (inputs#I1): it comes to P4 as a card with every option priced. Two options are named
  and must be on it: measure it under the fake agent with the paste-hint hold; leave the gate as it is. A change to
  the 5 s maximum, or to what `send` does while the hint stands, is not decided before the card.
- "the gate reads the same `for agents` literal with a 5 s maximum" (CARRY 2, verbatim).
  [premise-corrected: the gate reads that literal (`crates/viola-agent-claude/src/screen.rs:31`, `:145`), but the
  5 s maximum bounds only a screen that never goes quiet (`screen.rs:131-135`); a quiet verified screen with no
  literal is refused at once, at the first quiet instant (`screen.rs:147-151`, pinned by `src/run/gate.rs:562-567`)]
  So the 5 s number does not decide a `send` issued while the hint stands (research M7).
- "on 2.1.287 the footer holds no such literal for 8.0 s after a long paste, 6.5 s of them after the turn's Stop
  (measured at that chunk's `evidence/rehearsal-shapes.md`)" (CARRY 2, verbatim). Spot-checked at P3: the file states
  both numbers (its lines 5-6, 102 and 112: a timer from the paste, one run, one CLI version).
- "hypothesis: a `send` issued in that window on a verified CLI ends `input-not-ready`; no `send` was run in it"
  (CARRY 2, verbatim). Re-derived at P3 by reading, not by a run: the verdict is `input-not-ready` about 300 ms
  after the screen goes quiet, with nothing typed (research M7). End to end through `send` it is still unmeasured;
  that measurement is an option on the card.
- The fake agent already has the hold: `--paste-hint-ms`, capped at 8 000 ms
  (`src/bin/viola-fake-agent.rs:32-33`). Verified at P3: it acts in the agent's own turn handling for the compiled
  long text under `--framing` and `--turn-stop`, whoever pasted it, so a stamped wrapper reaches it from a `send`
  test with no new option (research M8).

- **The card's answer** (the overseer, founder-delegated, unattended, 2026-10-06; verbatim in inputs#I2): option B.
  The gate stays as it is, and the reading is measured under the fake agent with the paste-hint hold: one case in
  `tests/cli_send.rs`, no product line. Waiting on a literal-less screen, a longer bound and typing under the hint
  each change what `send` does; they are the founder's and are not decided here. The side finding, that the
  `input-not-ready` hint tells a driver to `viola wait` when the turn has already ended, is carried to "First live
  test and self-drive" with the measured reading beside it, as the place where the remedy is chosen.

### W3 — the missing remove-the-guard control (CARRY 3)
- The operator's answer at the prior wrap's route card (the overseer founder-delegated, 2026-10-06): the owner is
  this entry.
- `framing_turns` checks the rows before it pastes `PROBE_LOCAL_COMMAND` (verified: `src/cmd/verify/typed.rs:257`,
  the paste at `:283`), and that check has no control of its own (verified: the prior chunk's
  `evidence/paste-guard-control.md`, whose control neutralised the guard before the long and the tag-like pastes).
- The control needs a fake-agent screen variant drawn after the tag-like turn alone (a modal literal beside the
  input box), then the guard neutralised red and restored green, recorded as `evidence/paste-guard-control.md` did
  for the first guard.
- "no test shows a modal after the tag-like turn only, because the fake agent draws the same `turn` screen after
  every turn" (CARRY 3, verbatim). Verified at P3: with `--turn-stop` the agent draws the set's `turn` screen after
  every turn (`src/bin/viola-fake-agent.rs:474`), and the one modal test writes its modal into that `turn` screen
  (`tests/cli_verify.rs:966-969`), so the modal shows from the first turn on (research M9).

### W4 — two stale comments (PREREQ)
- `.config/nextest.toml:19` and `tests/support/verify.rs:390` still say verify makes "seven 300 ms settles"
  (verified: both lines read so). It is ten since the prior chunk, as the key file
  `.andromeda/registries/contracts/test-plan/bootstrap-phases-derive-for-route-setup-project.md` says. Both comments
  are brought to it.
- "it is ten". Verified at P3 two ways: the key file reads "ten 300 ms waits", and verify's quiet waits count to ten
  at their call sites in `src/cmd/verify/typed.rs` (research M10).

### Observation
- watch: `pre-push`'s coverage merge failing on truncated profiles, `llvm-cov-exit-1` with three raw profiles of
  4 709 read "file header is corrupt" and every test passed, while another session's build wrote 10.8 GB to the
  same volume (0/3; since 2026-10-06-local-command-and-paste-framing-rows).

## Baseline reds on a shared host (the operator's directive, inputs#I1)
- Five builders share this host tonight. A baseline red of the stalled-start shape is not treated as real until it
  has been re-read with no other build running.
- The shape, as the prior chunk measured it (`evidence/block-reds-host-contention.md`, read at P3): every test that
  spawns a process stalls at its start by one common amount, tests that spawn nothing pass in milliseconds, and no
  assertion fails on a value. `.claude/rules/verification-harness.md` (2026-09-29, extended 2026-10-06) carries the
  rule.

## Sizing (the operator's directive, inputs#I1)
- One builder window. Four work items: one code strand in the wrapper and the client (W1), one card whose built
  half is unknown until P4 (W2), one fake-agent variant with a red/green control (W3), two comments (W4). No live
  session, no recording, no row count moved.
- The measured anchor: the prior chunk held three rows, their probes, a live record round and a mid-run revision in
  one window with three implement runs. This one is lighter on every axis but W2. If research or the card shows it
  does not fit, P4 brings a split card; a new entry stays inside Epoch 3.

## Boundaries (not this chunk)
- No live `claude` session, no `viola verify --record`, no stamp, no fixture recorded (inputs#I1).
- `GATE_MAX_WAIT`, the readiness verdict in `crates/viola-agent-claude/src/screen.rs` and what `send` does while
  the paste hint stands: untouched unless the P4 card's answer says otherwise.
- The ledger: no row added, `LedgerRow::ALL` stays seventeen, `LOCAL_COMMANDS` stays the two commands.
- `/remote-control` is never typed by a probe.
- The web readback's `data-rb="unconfirmable"` state and its a11y verdicts are owed to the Epoch 8 entries.
- Owned by "First live test and self-drive" (`:94`): every live proof, the harness-prefix and identity-floor rows,
  PATH `claude` 2.1.289.
- The founder's seven dated security exceptions stay as they are. Their owner is Epoch 6.
- R-S3: the U02 setup upgrade and the `host-win32.md` regenerate run at the Epoch 3 boundary, not here.

## Surfaces and contracts it touches
- `src/run/send.rs`: the send slot, its confirmation window, the hook-event tap.
- `src/cmd/send.rs` and `src/human.rs`: the client's outcome rendering and the readback mirror writers.
- `src/cmd/run.rs`: where `cli_verified` is known.
- `crates/viola-agent-claude/src/ledger.rs`: `LOCAL_COMMANDS` and `PostCondition`, read, not changed.
- `src/bin/viola-fake-agent.rs`: `--local-command-mode`, the screen variant for W3, the paste-hint hold for W2.
- `src/cmd/verify/typed.rs`: `framing_turns`, touched only by W3's control (neutralised, then restored).
- `tests/cli_send.rs`, `tests/cli_verify.rs`, `tests/support/verify.rs`, `.config/nextest.toml`.
- Specs: architecture [Delivery Confirmation], the `send` wire contract and the refusal order; layout-templates'
  readback mirror; test-plan §6 Path 2 (`local`) and §10; obs-plan §6 (`send-confirmed{confirmed:false}`);
  design-system's readback vocabulary as the mirror's source.

## CI read at take-up (Setup 5a: every commit from the last wrap's flip `11c77f7` through HEAD)
- `11c77f7` (the local-command-and-paste-framing-rows wrap commit, HEAD): green, ci#37540328144, 15/15 checks,
  wall 382 s.
- Nothing red to fold.
