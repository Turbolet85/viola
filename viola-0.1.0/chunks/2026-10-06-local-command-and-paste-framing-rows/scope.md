# Scope — 2026-10-06-local-command-and-paste-framing-rows

**Working entry** (`working-route.md:90`, Epoch 3 — Windows slice II: driving verbs and live proof):
Local-command and paste-framing rows — local-command post-conditions and unconfirmable, long-paste wrapper,
tag-escaping, harness-prefix and R8 identity-floor ledger rows, each measured by typed probe.

## Intent
The capability ledger gains the rows the typed-input probe was built to measure and that still stand unmeasured:
what each known local command does when typed (its delivery post-condition, or "none"), how the CLI frames a long
paste, how it escapes tag-like text, which prompt prefixes mark a harness-injected prompt, and that the R8 identity
floor holds. Each row gets its `viola verify` probe and a post-condition. `send` gains its one outcome for a local
command that cannot be confirmed, `unconfirmable`, and `/clear` is confirmed by its new-session post-condition. viola
stays mechanism: the probe measures and stamps, it decides nothing for a driver.

## Authority
- The founder's three-way split ruling (live, 2026-10-05 05:58Z, relayed by the overseer) minted this entry: W2 + W4
  of chunk `2026-10-05-real-cli-verify-probes`, its scope CARRYs 2, 3 and 5, placed after "Dialog rows and re-probe"
  and ahead of "First live test and self-drive" (the entry's first CARRY; verified against that chunk's `scope.md`
  lines 27-43 and 140-193).
- The founder's ruling **R-S2** (live, 2026-10-05) ratified the typed-probe widening these rows ride: real-`claude`
  probes run locally on the Linux dev host only, with no Claude credential on a CI runner, and the probe stamps the
  version it runs.
- The operator's directive at this take-up (inputs#I1):
  - the installed `claude` is now 2.1.289, with 2.1.288 and 2.1.287 still installed: say which version the typed
    probes hit and what that does to the stamped rows;
  - live `claude`: 3 of the founder's cap of 16 are left. Plan inside those 3 if the rows allow; a larger need goes to
    P4 as a founder card with the count priced per row;
  - size the chunk against one builder window;
  - Epoch 3 stays unsplit.
- **The answers to the P4 founder card** (2026-10-06 ~19:38Z, relayed by the operator; verbatim in inputs#I2):
  - **two chunks** (the founder, live, relayed by the overseer): Epoch 3 stays one epoch at 15 entries;
  - **Run B takes three more compiled pastes**, a long synthetic text, a tag-like synthetic text and `/clear` (the
    founder, live, after the widening was shown to him, relayed by the overseer). This is the playbook's "Boundary
    widening" pattern, ratified by his live answer;
  - **2.1.287 alone is recorded and stamped**, and the live cap for this chunk is **8 sessions**: step 0 = 1, one
    2.1.287 record round = 5, 2 spare for step-0 re-runs. The 3 left of the old cap are inside the 8. A red record
    round returns to the founder (the overseer, founder-delegated: the founder's live word of ~19:18Z lets the
    overseer set this chunk's cap alone);
  - **the cross-session half of the harness-prefix row and the identity-floor row re-home** to "First live test and
    self-drive" (the overseer, founder-delegated).

- **The direction on step 0's STOP 3** (2026-10-06 ~20:05Z, the overseer, founder-delegated; verbatim in inputs#I4),
  given after `/andromeda-implement` stopped before any code (`evidence/step0-shapes.md`):
  - fold the fix into this chunk: the unwrap consumes the CLI's framing newlines around a pair, bounded to what
    step 0 measured;
  - a fake-agent case replays the recorded wrapped shape through `send`, red at HEAD and green after;
  - the send-strand guard is narrowed only as far as that case needs;
  - the `hook.rs:171` comment is corrected to the measured mid-text reading;
  - step 0 is not re-run: its captures are the evidence. The cap stays 8, with 1 used.

- **The founder's answer on the red record round** (live, 2026-10-06T20:49Z, the implement report's card shown to
  him, relayed by the overseer; verbatim in inputs#I6):
  - option 1 of the card: Run B waits for the input-box literal to return after each added turn, bounded by the
    probe deadline, and the guard is kept;
  - the live cap for this chunk rises from 8 to **12**;
  - first one spare session, a step-0-shaped rehearsal of Run B's exact sequence that measures how long the paste
    hint stands; then one second record round of 5;
  - a second red round returns to the founder;
  - the product readiness gate and its 5 s maximum stay untouched here; that reading is the wrap's to route.

## Revision after the red record round (P4 re-synthesis, 2026-10-06, inputs#I6)
- **Measured at the first record round** (2.1.287, rows 2-6 of `evidence/live-sessions.md`;
  `evidence/record-round-red.md`, `evidence/round-203444Z.txt`): `stamped 2.1.287  15 pass  2 fail`.
  `long-paste-wrapper` passed on a prompt framed as step 0 measured, with another id. `tag-escaping` and
  `local-command-clear` failed because neither text was pasted: Run B stopped after the long-paste turn.
- **Why, as far as it is measured:** after a long paste the CLI's footer reads `paste again to expand` and no row
  holds the compiled input-box literal `for agents`. At step 0 the literal came back 5.8 s after that turn's Stop
  (judged with 1 s of quiet). Run B's settle gives up 5 s after the Stop when no compiled literal is on a quiet
  screen, and Run B pastes only into rows that hold the literal. The hint's length is inferred from one run, not
  measured as a timer; the round's own screen bytes are gone with its probe dir.
- **Added to "Built here":**
  - after each added turn (the long text, the tag-like text) Run B waits until the input-box literal is back on a
    quiet screen with no modal, for at most the probe deadline counted from that turn's Stop, and only then pastes
    the next text. A modal, or the deadline, ends the added pastes with no key. The guard stays;
  - the fake agent can hold a literal-less screen after the long paste's replay, for a test that forces the window
    open past the gate's 5 s maximum: red before the change, green after;
  - one rehearsal session before any change to the wait: a scratch probe outside the repository that runs Run B's
    exact sequence and records, per added turn, how long the literal stays away;
  - one second record round on 2.1.287 (5 sessions). The cap is 12: 6 used, 1 rehearsal, 5 for the round, none
    spare.
- **What the first implement run left in the tree** (uncommitted; not redone): the unwrap change and its cases, the
  three rows and their arms, Run B's pastes with the guard, verify's recording, the fake agent's `--framing`, the
  seventeen-row literals, the contract and hygiene cases, and one test beyond the plan
  (`verify_pastes_nothing_more_once_the_turn_screen_shows_a_modal`). One file outside research's lists was edited
  and recorded: `src/run/version_gate.rs`, a unit test that pinned a fourteen-row stamp as verified
  (`scope-record.md`).
- **Not touched here** (inputs#I6): `GATE_MAX_WAIT`, the readiness verdict in
  `crates/viola-agent-claude/src/screen.rs`, and `send`'s gate. Whether a `send` issued within seconds of a long
  paste meets the gate's maximum wait on a verified CLI is a reading for the wrap to route, not built or measured
  here.
- **Why it belongs here** (justified divergence, so the intent was incomplete): the rows cannot be stamped unless
  their probes are pasted, and the probes are pasted only into a settled input box. The wait is part of the probe.

## Revision after step 0 (P4 re-synthesis, 2026-10-06, inputs#I4)
- **Measured at step 0** (one live session on 2.1.287, `evidence/step0-shapes.md`, `evidence/step0-prompts.json`):
  - a 1 500-byte paste arrives as two newlines, the `<pasted_content id="X">` pair around the text, one newline.
    HEAD's `prompt_text` keeps the three newlines, so the normalised text is not the pasted text;
  - a 200-byte paste holding typed `pasted_content` tags and a typed `<task-notification>` mid-text arrives
    unwrapped, the two `pasted_content` tags escaped with `<\` and the `<task-notification>` as typed. It normalises
    to the pasted text;
  - a pasted `/clear` fires SessionEnd (`reason` `clear`) and then SessionStart (`source` `clear`, a new
    `session_id`), and no UserPromptSubmit. The input box settles again.
- **Added to "Built here":**
  - `unwrap_pastes` also removes the framing step 0 measured around a pair: the two newlines directly before the
    open tag and the one newline directly after the close tag. Nothing wider: a third newline before, or a second
    after, stays;
  - the tests that pin the old shape move with it (`hook.rs` `cli_pair` and the wrapped-paste property,
    `tests/hook_events.rs` `paste`);
  - one new case in `tests/cli_send.rs`: the fake agent, with `--framing`, answers a sent long text with the
    recorded wrapped prompt, and `send` confirms it. This is test-plan §6 Path 2 step 3's first half;
  - the doc comment on `prompt_origin` says what was measured.
- **Why it belongs here** (justified divergence, so the intent was incomplete): architecture [Delivery
  Confirmation] says matching compares the sent text with `prompt-submitted`'s `text` exactly after the wrapper is
  removed (`architecture.md:49`, `:292`), and `SendSlot::claim` does compare exactly (`src/run/send.rs:84`). With
  the measured framing the long-paste row cannot pass and a wrapped `send` cannot be claimed. The row and the fix
  are one subject.
- **Still not built here:** `send`'s `unconfirmable` outcome, `/clear`'s confirmation, the mirror writer and the
  change to `send_window_local_command_is_not_presumed_delivered`. `src/run/send.rs`, `src/cmd/send.rs` and
  `src/human.rs` stay untouched; `tests/cli_send.rs` only gains lines.
- **Not measured, so not decided here:** whether a typed `<task-notification>` at the very start of a prompt
  arrives escaped (the position `prompt_origin` reads). It travels with the harness-prefix row to "First live test
  and self-drive".

## Scope after the split (P4, 2026-10-06)
- **Built here:** the three rows a typed probe can measure — the long-paste wrapper, tag escaping, and the local
  command `/clear` — with the compiled local-command list, Run B's three added pastes, their recordings on 2.1.287,
  the fake agent's replay, and the row count moved from 14 to 17 at every site. 2.1.288 moves to the drift-only list.
  2.1.289 stays unstamped.
- **Moved to a new entry, "Local-command send outcomes", right after this one and ahead of "First live test and
  self-drive":** `send`'s `ok {confirmed:false, detail:"unconfirmable", cursor}` outcome, its `[  ] unconfirmable`
  mirror writer, `/clear` confirmed by its new-session post-condition, the change to
  `send_window_local_command_is_not_presumed_delivered`, and the claim on `verification-matrix.json#v1-29`. It needs
  no live session. Until it lands, `send` still ends a local command `not-delivered` / `no-prompt-submitted`.
- **Moved to "First live test and self-drive":** the harness-prefix row with its cross-session measurement, and the
  R8 identity-floor row. Two measurements travel with them: the floor's names on Linux at 2.1.289 (research M7), and
  the hook-layer reading that 14 hand-backs on 2.1.289 started at the `<agent-message from=` tag with no preface
  (research M6, inputs#I2). The `<task-notification>` form is unmeasured at the hook layer.
- This chunk's wrap mints the new entry and re-homes the freight through route-resolve. W2 and W4 below stay as the
  folded record of what the entry carried.

## What it builds (the four CARRYs folded, in route order)

### CARRY 1 — the entry's own record
- Minted at the 2026-10-05-real-cli-verify-probes wrap on the three-way split ruling. No work of its own; it is the
  provenance of W2 and W4 below.

### W2 — local-command rows and `unconfirmable` (CARRY 2, from 2026-10-04-confirmed-send-with-cl-1-records, fork F2)
- The local-command ledger rows: each known local command maps to its measured post-condition, or to "none". `/clear`
  maps to a SessionStart with source `clear` plus a new `session_id`. Each row has a typed `viola verify` probe.
- `send` gains its `ok {confirmed:false, detail:"unconfirmable", cursor}` outcome, with its `[  ] unconfirmable`
  mirror writer.
- At HEAD a local command ends `not-delivered` / `no-prompt-submitted`. The test
  `send_window_local_command_is_not_presumed_delivered` pins that (verified: `tests/cli_send.rs:373`), and it changes
  here.
- `verification-matrix.json#v1-29` is claimable here per its notes (verified against the cap, read through
  `matrix.py show`): of its three owed clauses, "only when the input box is ready" landed at
  2026-10-05-real-cli-verify-probes, and the two left are owed to this entry — `unconfirmable` only for a ledger-listed
  local command without a measured post-condition, and `/clear` confirmed by its new-session post-condition.
- The ledger lists two local commands, as architecture [Delivery Confirmation] fixes them: `/clear`, whose
  post-condition the typed probe measures, and `/remote-control`, which has none and so ends `unconfirmable`. Only
  `/clear` is typed by a probe; `/remote-control` never is (it would open a remote-control link). Closed at P3
  (research M5); the row costs no session of its own (research M3, M4).

### W4 — paste-framing and identity rows (CARRYs 3 and 4)
- The long-paste wrapper, tag-escaping and harness-prefix ledger rows, with live typed probes, and the R8
  identity-floor row (CARRY 3, from 2026-09-28-capability-ledger-and-viola-verify).
- The harness prefixes gained `<\cross-session-message` and `<cross-session-message` on a RELAYED measurement
  (CARRY 4, from 2026-09-29-fake-agent-drift-contract; founder-ratified live 2026-09-29 15:21:44). That measurement is
  overseer1's F115 on andromeda-worker plus the viola-lab prototype's `harness_injected`; there is no fixture or
  ledger row here. Verified at HEAD: the four prefixes are compiled at `crates/viola-agent-claude/src/hook.rs:106-109`.
  The entry asks to measure a real cross-session UserPromptSubmit `prompt`, escaped or plain, and record the
  harness-prefix ledger row.
- "as measured at chunk 2026-10-05-real-cli-verify-probes research M10, producing one needs a second session sending
  to the probed one — nothing in this repository causes one, and the messaging socket and token are R8-stripped from
  the child" (CARRY 4, verbatim). Verified at HEAD from the code, not from the cited coordinate (that `research.md`
  names M10 only as moved with W4, lines 58 and 218): both messaging names are on `IDENTITY_FLOOR`
  (`crates/viola-agent-claude/src/lib.rs:19-20`), a 2.1.289 session exports both, and the tag's literal lives in
  `hook.rs` alone (research M6). So a typed probe cannot produce a cross-session prompt, and the cross-session half
  of the harness-prefix row has no probe inside the standing rulings. It goes to P4 as a founder fork.
- HYPOTHESIS (CARRY 4, kept verbatim): "observed in this orchestrating Claude
  Code 2.1.283 session and never measured at the hook layer: a subagent's hand-back reaches the model framed `Another
  Claude session sent a message:` ahead of `<agent-message from="…">` (every doc-agent return of that chunk's wrap
  arrived so, and one had its quoted tags neutralised `<` → `<\`); if the UserPromptSubmit `prompt` carries that
  preface, `starts_with("<agent-message from=")` misses it (observed again at the model layer on 2.1.288, research
  M10, and in the 2026-10-05-real-cli-verify-probes wrap's seven doc-agent hand-backs)". [premise-corrected: on
  2.1.289 the 14 hand-backs of this phase reached UserPromptSubmit starting at `<agent-message from=`, with no
  preface — the driver's own log, relayed in inputs#I2 and re-counted at P4 (research M6)] The preface is a
  model-layer frame; the compiled prefix matches the hook's prompt. The `<task-notification>` form is still
  unmeasured at the hook layer. The plan leans on nothing from it; it moves with the harness-prefix row.

### The version the probes hit, and the stamped rows (the operator's directive, inputs#I1)
- Measured at this take-up (2026-10-06 ~19:20Z): `claude` on PATH is the mise `latest` install, and it answers
  `2.1.289 (Claude Code)`. mise lists 2.1.287, 2.1.288 and 2.1.289. The directive's fact holds.
- Measured: the committed recorded sets are `fixtures/claude/2.1.283/`, `2.1.287/` and `2.1.288/`. There is none for
  2.1.289, and the dev host has no `~/.viola/ledger/` (the live stamps of the two earlier chunks were written into
  per-run record homes).
- A `viola verify` given no binary runs PATH `claude` (`src/cmd/verify.rs:103-106`), so a typed probe fired today
  hits **2.1.289**, and the stamp names the version the probed binary answers. The earlier chunks reached 2.1.288 and
  2.1.287 only by naming each binary after `--`. Verified at P3 (research M1).
- Adding a row grows `LedgerRow::ALL` past 14, and `ledger::verified` needs every row `pass`
  (`crates/viola-agent-claude/src/ledger.rs:799`). So the 14-row stamps and recorded sets of 2.1.288 and 2.1.287 read
  **unverified** from the first build with a 15th row, until each is recorded again with the new shapes or moved to
  the drift-only list; 2.1.289 has no stamp and no set, and reads unverified until its first full verify. The test
  fleet's default version is 2.1.287 at four sites, so the default set must replay every row. Verified at P3
  (research M2).

### The live-session price (the operator's directive, inputs#I1)
- Measured from the dialog chunk's ledger (`2026-10-05-dialog-rows-and-re-probe/evidence/live-sessions.md`, rows
  4-13): one full `viola verify` on one version is **5 live sessions** — the print-mode probe, then Runs A, B, C and D.
  Two versions cost 10. A scratch shape probe ahead of any code (the "step 0" of both earlier chunks) costs 1 each.
- The cap has 3 left. So one full stamp of one version does not fit in the 3 as verify runs today.
- `verify` has no way to run a subset of its runs (`VerifyArgs` holds `record` and `program` only), and a subset that
  stamped would reopen the retired R2 gap. Verified at P3 (research M4). So the 3 buy a step-0 shape probe and two
  spares, and any stamp is a founder card.
- The price per row (research M4): the local-command, long-paste and tag-escaping rows each cost 0 sessions of their
  own, because they ride Run B as added pastes. The shared cost is 1 for step 0 on 2.1.289 and 5 per stamped version:
  6 for 2.1.289 alone, 11 with 2.1.288, 16 for all three. The cross-session half of the harness-prefix row and the
  identity-floor row have no probe path inside the standing rulings (research M6, M7).
- The identity floor was measured by name on this Linux host at 2.1.289: 10 exported `CLAUDE*` names, all on the
  11-name floor, none outside it (research M7).

### Sizing (the operator's directive, inputs#I1)
- One builder window. The measured anchor is the split record of 2026-10-05-real-cli-verify-probes: a lighter entry
  ran its builder to 86.7 % of the window. This entry carries two code strands (W2: a new `send` outcome, its mirror
  writer and the local-command rows; W4: four rows and their probes), plus the fake-agent replay and recordings for
  every new row. If research shows it does not fit, P4 brings a split card; the new entry stays inside Epoch 3.

## Boundaries (not this chunk)
- Owned by `:92` First live test and self-drive:
  - firing `agent-run.sh run --local-live` on the dev host. Its CARRY names fourteen rows; this chunk moves that
    count, and the wrap's route-resolve re-points the number;
  - "the dialog never renders" (`verification-matrix.json#v1-31`);
  - the Windows-only live items (H2's key loss after a resize, the DA1 no-terminal stall, F-W3's real-terminal mouse
    report) and where that entry's live proof runs;
  - the Epoch 3 boundary audit's Windows-dispatch survivors.
- No Claude credential on any CI runner. The live probe is local to the Linux dev host (R-S2).
- No byte typed into any dialog by a `verify` child, and nothing written under `~/.claude` by viola (security rules,
  the founder's ruling of 2026-10-05). A local command that opens a CLI-native dialog or a picker is inside that rule.
- The founder's seven dated security exceptions stay as they are; this chunk widens none. Their owner is Epoch 6.
- R-S3: the U02 setup upgrade and the `host-win32.md` regenerate run at the Epoch 3 boundary, not here.

## Surfaces and contracts it touches
- `src/cmd/verify.rs` and its typed runs: the new rows' probes.
- `crates/viola-agent-claude/src/ledger.rs`: `LedgerRow`, `ALL`, the row checks, the stamp merge.
- `crates/viola-agent-claude/src/hook.rs`: the harness prefixes, the long-paste unwrap, the tag un-escape.
- `send`'s outcome set (`unconfirmable`), its confirmation window, and the readback mirror writer.
- The R8 strip plan (the identity floor) as the row's subject.
- The fake agent, `fixtures/claude/<version>/` and `fixtures/fake-scripts/`: replay of the new rows.
- `crates/viola-e2e/src/harness/run.rs`: `LEDGER_ROWS`.
- The stamp file `ledger/stamps.json`: verify stays its only writer.
- Specs: architecture [CLI Version Compatibility] (the local-command, long-paste, tag-escaping, harness-prefix and
  identity-floor rows), [Delivery Confirmation], the `send` wire contract; test-plan §6 / §10; obs-plan §6;
  security-plan (the subprocess boundary, the `verify` children rule).

## CI read at take-up (Setup 5a: every commit from the last wrap's flip `2fbc954` through HEAD)
- `2fbc954` (the permission-end-to-end wrap commit, HEAD): green, ci#37338433069, 15/15 checks, wall 368 s.
- Nothing red to fold.
