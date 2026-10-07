# Scope — 2026-10-07-live-rows-and-paste-shapes-on-the-dev-host

**Working entry** (`working-route.md:96`, Epoch 3 — Windows slice II: driving verbs and live proof):
Live rows and paste shapes on the dev host — the harness-prefix and identity-floor rows, three paste shapes and the
paste-hint window, measured on the real CLI.

## Intent
On the Linux dev host, against `claude` 2.1.287 named by path, the behaviours this entry names are measured on the
real CLI and recorded: what a harness-injected prompt reads at UserPromptSubmit (the cross-session form and the
task-notification form), which `CLAUDE*` identity names that CLI hands a child, three paste shapes no version has
measured, and the paste-hint window, timed a second time and with a `send` run inside it. The chunk ends with the
founder holding a card that carries the hint numbers. Nothing about the readiness gate changes before that card.

It runs unattended, and every live `claude` start is counted against one cap of 10 for the whole chunk.

## Authority
- Founder rulings, live, 2026-10-07T05:43Z, relayed by the overseer (inputs#I2 §1; the entry's first CARRY):
  - **R-L2**: this entry was split from "First live test and self-drive"; Epoch 3 stays one epoch.
  - **R-L3**: the rows and shapes are measured and recorded on `claude` 2.1.287, named by path; the live-session cap
    is 10; PATH `claude` stays unstamped.
  - **R-L4**: what `send` does while the paste hint stands is not decided. This entry times the hint a second time
    on the live CLI and brings the founder a card with the numbers; the choice is made on that card; nothing about
    the gate changes before it.
- The operator's directive at this take-up (inputs#I1, a verbatim copy):
  - 2.1.287 by path, a live cap of 10 for the whole chunk, R-L4 as above;
  - a probe path that lets a second session send into a `verify` child, or that records environment names, is a
    boundary widening: it goes to P4 as a founder card, every option priced, with what each row loses if it is
    refused;
  - the chunk is sized against one builder window; if the six CARRYs do not fit, a split card is brought, priced:
    a split is the founder's word.
- The P4 answers (inputs#I3, a verbatim copy):
  - **the cross-session prompt** (a boundary widening, the founder's own live answer of 2026-10-07T09:43Z, given
    after the widening was shown to him, relayed by the overseer): one fixed synthetic message from the builder
    session into the scratch 2.1.287 session, through the CLI's own peer messaging; no product code, `viola verify`
    untouched. The row gets a first measurement and no per-version stamp, so `v1-34` is not claimable from it;
  - **the identity-floor names** (a boundary widening, the founder's own live answer of the same minute, relayed
    by the overseer): the scratch probe's own hook writes the sorted `CLAUDE*` names once, names only and never a
    value; the product's capture arm is untouched;
  - **the chunk's shape** (the founder's own live answer of the same minute): measure here, rows next. Neither
    card asked for a probed ledger row, so no entry is minted now, and `v1-34`'s two rows stay owed, named in the
    handoff;
  - **the hint run's verified home** (the overseer, a technical fork): a byte copy of the verify-written home of
    the 2026-10-06 record round, the original untouched; the evidence says in words that it is a copy and never a
    hand-written stamp. The founder knows not to delete `target/e2e-home.disk` before this chunk wraps.
- The P5 review (inputs#I4, a verbatim copy; the overseer's, with the approval): the plan's three leans and four
  additions stand; the unstamped-home hint run measures what the product already does on an unverified CLI, a
  reading and no new widening, and the evidence says so; the hint card reaches the founder through the overseer,
  who returns his words.
- Earlier answers this entry inherits (in this repository):
  - the two rows were re-homed out of chunk 2026-10-06-local-command-and-paste-framing-rows by its P4 answer 3,
    the overseer founder-delegated (`chunks/2026-10-06-local-command-and-paste-framing-rows/inputs/I2-relay-2.md.txt`);
  - the readiness gate was left as it is at chunk 2026-10-06-local-command-send-outcomes (its P4 card's option B),
    and this entry is where the remedy is chosen (that chunk's `evidence/paste-hint-send.md`).

## What it builds (the entry's six CARRY blocks folded)

### W1 — the harness-prefix row, with its cross-session measurement (CARRY 2)
- A real cross-session UserPromptSubmit `prompt` is measured on 2.1.287: what its raw start reads, so the two
  compiled cross-session prefixes (`<\cross-session-message`, `<cross-session-message`) rest on a measurement made
  in this repository and not on the relayed one (architecture [CLI Version Compatibility], Harness prompt prefixes).
  It needs a second session sending to the probed one.
- The `<task-notification>` form at the hook layer. `[premise-corrected: research M2, the driver's own log read by
  counts: 28 of 136 UserPromptSubmit prompts start with the tag, unescaped, with no preface]` It is measured for
  the CLI this builder session runs (PATH `claude`), not for 2.1.287. Still unmeasured on any version: whether a
  typed `<task-notification>` at the very start of a prompt arrives escaped (mid-text it arrived as typed on
  2.1.287). That typed reading is this chunk's; it needs one paste.
- The `<agent-message from=` form. `[premise-corrected: a hand-back reaches a session only when that session itself
  runs a subagent, which no planned live session does]` It stays measured at the hook layer on PATH `claude` only
  (75 of 136 prompts in the same log, research M2); no 2.1.287 reading of it is planned.
- "The harness-prefix ledger row" means a row of `viola_agent_claude::ledger` with a `viola verify` probe and a
  post-condition, stamped per CLI version (verified: `crates/viola-agent-claude/src/ledger.rs:20-110`, `:740-770`;
  the arch extract). Whether it lands as such a row depends on its probe path, which no standing ruling gives it:
  P4's founder card (below, "The probe-path card"). The measurement itself needs no product change (research M4).

### W2 — the R8 identity-floor row (CARRY 2)
- The `CLAUDE*` names a 2.1.287 session exports to a child are measured by name on this host and read against the
  11-name `IDENTITY_FLOOR`. Measured so far on 2.1.289 only (10 names, all on the floor).
- A ledger row for it needs a probe that can see a child's environment names. The capture arm reads no environment
  and writes only the hook's stdin (verified: `src/cmd/hook.rs:94-95`, `:359-372`), so the row needs that arm, or
  another probe, to record names: the second widening of the probe-path card.
- One names-only path is already ruled (research M5): `viola run`'s `process-start{subject:"claude-child"}` line
  lists the `CLAUDE*` names it removed from its own inherited environment (`src/run/mod.rs:39-60`). A `viola run`
  this builder starts records the names of the session that started it (PATH `claude`), at no widening; it is not
  a 2.1.287 reading.

### W3 — three paste shapes (CARRY 3)
- Measured on 2.1.287, each as the raw UserPromptSubmit `prompt` it produces:
  1. a wrapped paste beside typed text;
  2. two wrapped pastes in one prompt;
  3. a pasted text ending in a newline.
- Each reading is put against the compiled `hook::unwrap_pastes`: does the normalised prompt equal what was
  pasted and typed.
- Added at P5's validation-1 (the plan surfaced it): shape 1 is read in both orders, typed then paste and paste
  then typed, and shape 3 is read for a short, unwrapped text as well as a long one, since the claim compares both
  the same way. They are turns of the same session and cost no start.
- "hypothesis, read statically from the 2.1.287 binary: the CLI adds no newline before the close tag for it, so the
  unwrap would return that text without its last newline and a wrapped `send` ending in a newline would go
  unclaimed". Research M6 re-derived the code half at HEAD (`paste_pair` returns the text without that newline, and
  the claim compares exactly, so the send would end `no-prompt-submitted`); the CLI half stays the hypothesis. Its
  test is shape 3's paste.
- Shapes 1 and 2 already stand as unit cases written from the one-paste measurement (`hook.rs:556-563`): a guess
  the live readings confirm or correct (research M6).
- A shape whose reading falsifies the compiled unwrap is a product defect in delivery matching. Whether its fix
  lands in this chunk or is carried is a sizing question for P4; the measurement and its record land here either
  way.
- "recorded" (R-L3). `[premise-corrected: a fixture under fixtures/claude/ comes only from the recorder, test-plan
  §7 per the tests extract]` A scratch probe's reading is recorded in the chunk's `evidence/` and as a unit case
  with a literal oracle; it becomes a fixture variant only if the shape becomes a Run B paste, which is a further
  compiled literal needing the founder's showing.

### W4 — the paste-hint window, and the founder's card (CARRY 4; R-L4)
- The hint is timed a second time on live 2.1.287 (first reading: 8.0 s from a long paste, 6.5 s of them after the
  turn's Stop).
- A `send` is run inside the real CLI's hint window, which no chunk has done: the fake-agent reading (a `send`
  there ends `not-delivered` / `input-not-ready` with nothing typed) is confirmed or corrected on the real CLI.
- "hypothesis: a `viola wait` issued then has no later event to wake on (read from the wake set, not run)".
  Research M7 re-established it by reading (`src/run/wait.rs:135-170`: with no `--after` the scan starts at the
  log's end at the call, and the turn's `turn-ended` is already logged); it is still not run. The hint a driver gets
  says `viola wait <name>, then send again` (`src/human.rs:215-217`). Its test is a `viola wait` run in the same
  live window, bounded by `--timeout-ms`.
- The same sequence has a second reading (research M7): on an unstamped home the gate is partial, a quiet screen is
  `Ready`, and the second text is typed while the hint stands. What the real CLI does with a paste under
  `paste again to expand` is the number the card's "type under the hint" option needs. It costs one more start.
- A verified home for the first reading needs a seventeen-row 2.1.287 stamp. One stands on the operator's desk
  (research M7); without it the reading costs a full by-path verify, five starts.
- The founder's card: the numbers above, and the four options the CARRY names for what `send` does while the hint
  stands (wait on a quiet literal-less screen, a longer bound, type under the hint, stay as it is), each priced.
- The gate, `GATE_MAX_WAIT` and `QUIET_PERIOD` are unchanged until the founder answers that card.
- The numbers exist only after the live run, so the card is brought during `/andromeda-implement`, not at P4.
  Whether the chosen remedy is then built inside this chunk or becomes its own route entry is part of P4's sizing.

### W5 — probe dirs a kill leaves behind (CARRY 5)
- "a kill is not one of verify's exit paths, so a killed live verify leaves its probe dirs too" (the measured half:
  a runner kill of a verify-driven test left 21 `.viola-verify-<pid>*` dirs at the workspace root). Verified at
  HEAD (research M8): the dirs are held by a drop guard and verify handles no signal
  (`src/cmd/verify/typed.rs:334-351`).
- Every live round of this chunk starts a real CLI in a probe dir, so it meets that exposure: a census of the
  root's `.viola-verify-*` dirs before and after every live round, recorded.
- Whether `viola verify` gains a way to remove what a killed run left (a product change inside a ruled boundary:
  "removed on every exit path") or the leftovers stay a recorded residual with a removal step is P4's. A leftover's
  name carries a pid and no start time, so the repository's owner-record sweep does not fit it as it is
  (research M8).
- Measured at this take-up (2026-10-07T08:52:43Z, `ls -ld .viola-verify-*`): ten such dirs stand at the repository
  root now, dated 2026-10-06T22:12:45Z to 22:12:50Z: four `-dialogs` (empty) and six `-plan` (one entry each),
  beside the operator desk's `.viola-verify-2095228/`. They are gitignored (`.gitignore:18`). They are not this
  chunk's making; their disposition is the operator's.

### W6 — the `--local-live` row list (CARRY 6)
- test-plan §3 5-command implementation's `--local-live` bullet names fourteen of the seventeen row ids; it is
  brought current with the rows this chunk lands. The spec text itself is the wrap's to amend: the plan names it
  under its expected amendments.
- If a row lands, `LEDGER_ROWS` and every count literal follow it, every stamped version needs a full re-verify,
  and `:98`'s `run --local-live` count follows. `[premise-corrected: research M9 re-derived the sites at seventeen
  rows: 75 lines in six files, and the step line's denominator is derived from the row list, not a literal]`

## The probe-path card (P4, the founder's; inputs#I1)
Two widenings, neither inside a standing ruling:
1. a second session sends into a probed child (W1's cross-session half). `[premise-corrected: research M4 — the
   strip removes the parent session's socket and token from the child's environment, which is verified, but that
   does not establish that the child cannot be addressed: wrapped builder sessions are listed as peers on this
   host]` Whether a 2.1.287 child started under viola's strip is listed is unmeasured; the first scratch session
   reads it. No change to the strip or to `viola verify` is needed for the measurement: what crosses is one
   message from a second Claude session into a probed session;
2. a probe records environment names (W2): the capture arm's ruled shape reads no environment.
Every option is priced in live sessions and in boundary moved, and each carries what the row loses if it is
refused. Nothing is built on either path before the founder's answer.

Decided at P4 (inputs#I3): both measurements ride the one scratch session, by one synthetic peer message and by
the scratch hook's names-only list. No ledger row lands in this chunk, no row count moves, and no product code is
written on either path. The two rows stay owed.

## Premises carried by the freight (closed at P3, `research.md`)
- "PATH `claude` on the dev host is 2.1.289". Verified at this take-up: `claude --version` prints
  `2.1.289 (Claude Code)`, PATH resolves to `~/.local/share/mise/installs/claude/latest/claude`.
- 2.1.287 is installed and answers by path. Verified: `~/.local/share/mise/installs/claude/2.1.287/claude --version`
  prints `2.1.287 (Claude Code)`.
- "2.1.287 is the one stamped set and 2.1.288 is drift-only". Verified: `tests/contract_ledger_probes.rs:36`
  (`STAMPED: ["2.1.287"]`), `:39` (`DRIFT_ONLY: ["2.1.283", "2.1.288"]`); `fixtures/claude/` holds those three.
- "a bare `viola verify` or `run --local-live` hits [PATH `claude`]". Verified for the harness: `local_live` passes
  no program (`crates/viola-e2e/src/harness/run.rs:379-385`). So this chunk's live rounds name the binary after
  `--`; firing `run --local-live` itself stays `:98`'s.
- "a `verify` child is R8-stripped of the messaging socket and token, and the capture arm records no
  environment". Verified: the floor holds both messaging names (`crates/viola-agent-claude/src/lib.rs:14-26`), the
  spawn removes the planned names (`src/cmd/verify/typed.rs:544-551`), and the arm's inputs are the event, the dir
  and stdin (`src/cmd/hook.rs:94-95`, `:359-372`).
- "Measured for them on 2.1.289: the 10 exported `CLAUDE*` names are all on the 11-name floor on Linux". Verified
  against that chunk's research M7 (`research.md:158-167`); re-read here by count only: this session's tool
  environment exports 10 `CLAUDE*` names.
- "14 subagent hand-backs reached UserPromptSubmit starting at `<agent-message from=` with no preface (the driver's
  own log)". Verified against that chunk's research M6 (`research.md:149-156`) and its inputs#I2 answer 3. The log
  is outside this repository and is not re-read.
- "mid-text it arrived as typed on 2.1.287". Verified: that chunk's `evidence/step0-shapes.md:69`, `:130`.
- "8.0 s from a long paste on 2.1.287, 6.5 s of them after the turn's Stop". Verified: that chunk's
  `evidence/rehearsal-shapes.md:5-6`, `:102`, `:112` (a timer from the paste: 8.000 s between two footer draws).
- "on a verified CLI a `send` issued while the fake agent holds the paste hint ... ends `not-delivered` /
  `input-not-ready` with nothing typed". Verified: chunk 2026-10-06-local-command-send-outcomes
  `evidence/paste-hint-send.md:19-29`; the hint string at `src/human.rs:215-217`.
- `GATE_MAX_WAIT` is 5 s and `QUIET_PERIOD` 300 ms. Verified: `crates/viola-agent-claude/src/screen.rs:10`, `:14`.
- "a runner kill of a verify-driven test left 21 `.viola-verify-<pid>*` dirs". Verified: that chunk's
  `evidence/block-reds-host-contention.md:65-71`.
- "`--local-live` bullet names fourteen of `LEDGER_ROWS`' seventeen ids". Verified:
  `.andromeda/registries/contracts/test-plan/5-command-implementation.md:53` names the six spine rows and eight
  more; `LEDGER_ROWS: [&str; 17]` at `crates/viola-e2e/src/harness/run.rs:333-351` adds `long-paste-wrapper`,
  `tag-escaping`, `local-command-clear`.
- The ledger holds seventeen rows today, neither of this entry's two among them. Verified:
  `crates/viola-agent-claude/src/ledger.rs:20-59`; architecture names both as owed to this entry.
- One `viola verify` is five `claude` starts, and a scratch shape probe is one. Verified: the print probe and four
  runs (`src/cmd/verify/typed.rs:63-84`). Priced per piece in research M10: the measurements alone take 3 starts;
  rows landing in the same chunk bring it to 8, and one red record round then passes the cap.

## Boundaries
- No change to the readiness gate, `GATE_MAX_WAIT` or `QUIET_PERIOD` before the founder's answer to the hint card.
- No probe path across either widening before the founder's answer to the probe-path card.
- No live start beyond 10 in the whole chunk; a round that would pass the cap stops and returns to the founder.
- Only `claude` 2.1.287 by path is measured and recorded; PATH `claude` (2.1.289) gets no stamp and no fixture set.
- No byte typed into any CLI dialog; a start that shows a modal is killed with no key (the standing `verify`
  rulings). `/remote-control` is never typed.
- No environment value is recorded anywhere, names only; no `CLAUDE*` value reaches a log, fixture or evidence file.
- Nothing is written under `~/.claude` by viola; the CLI's own transcripts of this chunk's live sessions are the
  accepted residual class.
- `run --local-live`'s live firing, the "dialog never renders" clause of `v1-31`, `v1-33` and the Epoch 3 boundary
  audit stay with "First live test and self-drive" (`working-route.md:98`).
- The three Windows-only live items stay with "Windows-only live measurements" (`working-route.md:140`).
- The ten probe dirs standing at the root, and the operator desk's items, are not removed without the operator's
  word.

## Sizing (inputs#I1; P3 measures, P4 decides)
- Five strands with little shared code: the two rows and their probe paths (W1, W2), the paste shapes (W3), the
  hint window and its card (W4), the kill leftovers (W5), the row list (W6). Research M11: the measurements, their
  evidence and the unit cases are one kind of work; a landed row adds a probe, a recorded variant, a fake-agent
  replay, 75 literal lines and a record round, and the cross-session row adds a second session to a probe's shape.
- The measured anchors: the last two chunks each took a whole phase, implement and wrap cycle, with implement
  ending near half the builder window (51.7 % and 48.9 %, inputs#I2 R-L2).
- If the folded freight does not fit one builder window, P4 brings a split card, priced; the split is the
  founder's word.

## CI read at Setup (what shipped since the last flip)
- `e7e5bf75db8a` — green · checks 15/15 · wall 407 s · ci#37595307628.
No red, nothing to disposition.

## Capabilities
None named by the entry. The claimable pool is read at P4.
