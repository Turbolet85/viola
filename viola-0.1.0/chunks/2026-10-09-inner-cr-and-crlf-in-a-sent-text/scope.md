# Scope — 2026-10-09-inner-cr-and-crlf-in-a-sent-text · Inner CR and CRLF in a sent text

**Working entry** (`viola-0.1.0/working-route.md:107`, the head of the markerless tail, in
`### Epoch 4 — Session state & governance`): Inner CR and CRLF in a sent text — typed as the LF the CLI submits, so
the send is confirmed and the driver keeps the wheel — plus one CARRY block (folded below; completeness check
`route.py pins` → the run dir's trail: one `:107` row, 1901 chars, no abstention).

**The operator's direction at take-up** (`inputs#I1`, the invocation, verbatim in the snapshot):
- the founder's cap for this chunk is **2 live `claude` starts, 2.1.287 by path**: one planned for the after-change
  confirmation and one spare. The before-change readings are starts 3 and 4 of chunk 2026-10-09-epoch-3-cleanup and
  are not re-taken;
- the rig is that chunk's headless pty host (`evidence/live-pty.py`): no compositor, no window, no key;
- a gate entry that only reads a record takes no `artifact` key;
- the chunk is sized against one builder window;
- anything that widens a boundary or adds a refusal detail comes to P4 as a card with every option priced.

## What this chunk builds

### 1. The typed text holds no CR
- Source: the entry's title and its CARRY. **The founder's ruling**, 2026-10-09T16:51Z, live in the overseer's
  dialog with both inner readings shown (the other options shown: refuse before typing; leave as measured), relayed
  by the operator at the 2026-10-09-epoch-3-cleanup wrap (that chunk's `inputs/I9-relay-1.md.txt`, re-read at P1)
  and placed as an entry of its own at the head on the operator's answer (that chunk's `inputs/I10-relay-2.md.txt`,
  re-read at P1): `send` types a
  CR or a CRLF inside a text as the LF the CLI submits, so such a send is confirmed.
- Today, re-read at P1 at HEAD `3c5e012` (the file is unchanged since `211da16`, the sha the entry names):
  `viola_agent_claude::hook::typed_text` is `text.trim_end_matches(['\r', '\n'])`
  (`crates/viola-agent-claude/src/hook.rs:207-209`). It returns a slice of the received text, so it can remove an
  ending and nothing else.
- The rule as this chunk reads the ruling: inside the typed text every CR LF pair is one LF and every other CR is
  one LF, and the trailing CR and LF characters still go, every one of them. The typed text then holds no CR
  anywhere. The order of the two steps does not change the result. Closed at P3 (research.md §Scope premise
  closure): the match is an equality on the in-flight typed text (`src/run/send.rs:130`) and the paste is one write
  of that same text (`crates/viola-pty/src/pump.rs:77-88`), so a typed text with LF where the received text held a
  CR is what both read. The exact wording of the rule is shown on the P5 review card.
- `Measured at chunk 2026-10-09-epoch-3-cleanup on live claude 2.1.287 on the Linux dev host
  (evidence/live-run.md, evidence/live-readings.ndjson, after-inner-crlf and after-inner-cr): the CLI submitted a
  pasted text's inner CR LF as one LF, and its lone inner CR as one LF.` Spot-checked at P3: the record file holds
  the two `reading` lines and the two `equality` lines (lines 7 to 10), and `hook.rs` is unchanged since the build
  those readings ran on.
- `hypothesis: every inner CR and every inner CR LF is submitted as one LF wherever it stands (two single
  readings, one text each)`. `Not measured: several inner CRs, an inner LF CR, any CLI version but 2.1.287.` Closed at
  P3 as not leaned on: with the rule no `send` types a CR, so no step rests on what the CLI does with one. It stays
  the entry's hypothesis, unverified here.
- [premise-corrected: research.md §Scope premise closure — no recorded measurement of an LF typed inside a paste
  exists] With the rule `send` never types a CR. What is leaned on instead is that the CLI submits a pasted text's
  inner LF unchanged. That is already leaned on today by every multi-line `send`, and it is **not measured**: the
  two compiled paste literals of `viola verify` hold no newline, the five LFs of the recorded `paste-1` prompt are
  the CLI's own frame, and no reading in the two live record files sent a text with an LF inside it. The two inner
  readings show the CLI turning a typed CR into an LF, never what it does with a typed LF. So no ledger row and no
  recorded reading covers the shape. By the scope's own boundary this goes to the operator as a P4 card, with the
  options priced: a live reading in this chunk's planned start, or an eighteenth ledger row with a fifth Run B
  paste.
- `Today the typed text still holds the CR, so the exact match fails, the prompt is filed human, the wheel goes to
  the human and the send ends not-delivered / no-prompt-submitted, exit 13, though the text was delivered and
  answered.` Verified at P3: the in-flight text is the typed text (`src/run/send.rs:341-345`), `SendSlot::claim`
  compares it for equality (`:130`), and a prompt the hook filed `human` that is not claimed moves the wheel
  (`:242-244`).
- **Answered at P4 (`inputs#I2`; the overseer, 2026-10-09, in the P4 dialog, three options shown with their
  prices).** The live reading, and the row stays owed. The planned start sends a text with one LF inside and no CR
  first, as a control with a prediction, then the CR shapes; the readings are recorded as measured on 2.1.287. No
  ledger row, no fifth Run B paste, no re-stamp: the count stays seventeen. Nothing is widened and nothing is ruled
  about the reach of `v1-34`: the row for this shape stays owed to it, beside its two owed rows. A control that is
  not confirmed ends the live work and is reported.

### 2. What reads the typed text
- Source: the CARRY: "With the ruling the text `send` types is no longer the received text less an ending, so what
  `text_bytes`, the local-command classification and the in-flight match read is settled at take-up."
- Today, re-read at P1 in `src/run/send.rs:320-366`: `validate_paste_text` runs on the text as received, then the
  typed text is taken once, and the empty-text check, `classify`, the in-flight text, `ctx.issue(text.len())`
  (`text_bytes`) and the paste all read that one value. The client (`src/cmd/send.rs:190-197`) reads it only for
  the empty-text check.
- Settled here as the rule the masters already state, kept: the typed text is taken once and is the only text
  `send` uses. So the in-flight match, the paste, `text_bytes`, the local-command classification and both
  empty-text checks read the typed text with its inner CRs already turned to LF. `text_bytes` for a text with CR LF
  pairs is then smaller than the sent byte count by one per pair. Verified at P3: one value feeds all five reads in
  the wrapper (`src/run/send.rs:323`, `:324`, `:334`, `:342`, `:365`, `:366`), and the paste span's own
  `text_bytes` is the length of the text it is handed (`crates/viola-pty/src/pump.rs:75`).
- The local-command classification gains no new member from this: a listed command holds no newline, a text with
  an inner newline is never exactly a listed command, and the trailing case (`/clear` and a CRLF is `/clear`)
  already stands. Verified at P3: the compiled list is two entries, `/clear` and `/remote-control`
  (`crates/viola-agent-claude/src/ledger.rs:151-154`), neither holding a CR or an LF.
- A text of only CR and LF characters stays `empty-text` by the same check; its hint line does not change.
  Verified at P3: every CR becomes an LF and every trailing LF goes, so nothing is left; the hint is the fixed
  string at `src/human.rs:216`, which no step touches.
- Validation is unchanged: `validate_paste_text` runs first, on the text as received, and CR stays an allowed
  character. No refusal detail is added and none changes its rung. If P3 or P4 finds one is needed, it is a P4 card
  (`inputs#I1`).

### 3. Tests at the tier that owns each outcome
- Source: the CARRY: "No test at any tier pins the inner outcome."
- Done = the inner outcome is pinned at each tier test-plan §4 gives it: the rule itself (the `typed_text` table and
  its property), the wrapper's send (a text with an inner CR or CRLF is pasted with LF, its prompt claimed, the
  send confirmed, the wheel left with the driver), and the driver-visible outcome where a tier above the unit
  tier owns it.
- A test under the fake agent that only asserts "confirmed" passes before the change too, because the fake agent
  submits what was typed. A test at that tier pins the outcome only if it also reads the typed or submitted text
  for the absence of a CR. The fake agent does not learn the CLI's CR reading. Verified at P3: inside a bracketed
  paste the fake agent keeps every byte as prompt content (`src/bin/viola-fake-agent.rs:590-597`), fires
  UserPromptSubmit with that text (`:466`) and receipts its `text` and `hex` (`:484-493`); the existing case
  `send_text_ending_in_newlines_is_typed_without_them_and_confirmed` (`tests/cli_send.rs:289`) already reads the
  receipt's `hex` and is the form the new case takes.
- [premise-corrected: the stopped implement run, 2026-10-09T18:59Z — a second place pinned an inner CR as typed,
  `tests/channel_paste_validation.rs:164` at the base commit, an assertion on a value in the block's default
  selection and in `pre-push`] The existing cases that say an inner CR is left in place are re-read and changed
  with the rule, not deleted. They stand in two places, not one: the `typed_text` table's inner cases
  (`hook.rs:674`, `:675`, `:678` at the base commit), and the root integration test
  `channel_paste_accepts_lf_cr_tab_and_multibyte` in `tests/channel_paste_validation.rs`, which sends a text with a
  lone inner CR over the raw channel and asserted the fake agent's prompt equal to the text as sent. With the rule
  that prompt is the typed text, the CR as one LF. The test keeps its name and its subject (the wrapper accepts LF,
  CR, TAB and multibyte text, refuses none of them and types one prompt); only its expected prompt moves. By the
  operator's word (`inputs#I5`) the file is one of the chunk's files to modify and its edit is a step of the plan.
  Closed at the revision's P3: a sweep of every Rust source line holding a CR that does not end its string
  literal, over `src`, `crates`, `tests` and `fuzz`, reads 34 lines in 11 files; 25 lines in 5 files are this
  chunk's own cases or this test, and the 9 lines in 6 other files send no text through `send` (research.md
  §Scope premise closure).

### 4. The masters and their leaves
- Source: the CARRY: "architecture [Delivery Confirmation], security-plan §Input Validation and test-plan §4 say an
  inner CR or LF is typed as received and the match is exact; the chunk that lands the ruling amends them."
- Re-read at P1: the sentences stand at `architecture.md:49` (twice, the second naming the inner readings as
  carried), `:136`, `:286`, `:292`; `security-plan.md:73`, `:74`, `:226`, `:543`; `test-plan.md:233`, `:565`, `:582`,
  `:750`; `obs-plan.md:636`, `:819` (`text_bytes`); `layout-templates.md:557`. Leaves that repeat them: `CLAUDE.md`
  (Critical Warnings), `.claude/rules/security.md`, `.claude/rules/events.md`, `.claude/docs/gotchas.md`,
  `.claude/docs/security-summary.md`, `.claude/docs/services/viola.md`, `.claude/docs/services/viola-agent-claude.md`.
- Phase amends no master. The plan lists them as the wrap's expected amendments; the code's own doc comments are
  the chunk's.

### 5. The live confirmation
- Source: the CARRY ("A live confirmation needs a founder's cap of starts of its own: none is given for this entry")
  and the operator's direction, which gives it (`inputs#I1`): 2 starts on 2.1.287 by path, one planned after the
  change and one spare.
- [premise-corrected: research.md §Scope premise closure — the order needs a control first, and the texts must stay
  short] One session on the changed build reads short texts, each under 100 bytes and of at most three lines, so
  that none starts the CLI's long-paste hint (a timer of 8.0 s from a long paste; its threshold is read statically,
  not measured). The order: first a text with one LF inside and no CR, the control for the shape every later reading
  rests on; then one CR LF inside; one lone CR inside; then the shapes not measured before (two CRs in a row inside,
  an LF CR inside, a three-line text with CR LF at the end of its first two lines). Each waits for the turn before
  to end. Each is predicted confirmed, filed `driver`, with no `wheel` record. A send that is not confirmed hands
  the wheel to the human and ends that session's readings; no `release` and no `pause` is run by the builder. The
  spare start is used only for readings the first could not take, and only when the first ended for a reason that
  is not the rule's.
- The rig is the last chunk's (`evidence/live-pty.py`, read at P1; `live-start.sh`, `live-drive.py`, `live-read.py`,
  `live-ledger.py` beside it): a plain pty, no compositor, no window, no key, each start ledgered before it is
  made. The stamped home `target/e2e-home/viola-live-4043089/home` was read at P1 and again at P3 as present on the
  tmpfs, stamped for 2.1.287 with 17 rows `pass`. It is gone at a reboot. A missing one costs a real-CLI
  `viola verify`, which is five `claude` starts (`architecture.md:91`) and so more than this chunk's cap: the live
  step then stops and reports, and no re-verify is made under this cap.
- The reader `live-read.py` gains one equality for the new shapes, so it is an instrument this chunk changes. Its
  known-verdict controls run under the fake agent, with no live start, before the live start: on the build before
  the change the equality must read false, on the build after it true.
- The before-change readings are the last chunk's `after-inner-crlf` (start 3) and `after-inner-cr` (start 4), cited
  where they stand and not re-taken.
- (`inputs#I5`, the operator's word at the revision.) The live work is done and stands: the rehearsal held and
  start 1 of the cap of two took all six readings, each confirmed (this chunk's `evidence/live-run.md`,
  `evidence/rehearsal.md`, `evidence/live-sessions.ndjson`). No start is made by the re-entry, and the spare start
  stays unmade. Closed at the revision's P3 against those files: four ledger rows, one of kind `start` under a cap
  of 2; six `reading` lines in each of the two record files.
- (`inputs#I5`.) What ties the live readings to the final tree is the code and data the build loads, not the build
  file's hash. The live start ran on the build of sha256 prefix `b4659b98029d0f94`; the final tree builds
  `4057b91d84010eca`. The two files differ in the linker's build-id note and in three compiler-named local symbols;
  their loaded sections (`.text`, `.rodata`, `.data`, `.data.rel.ro`, `.eh_frame`, `.gcc_except_table`) each hash
  the same. The file hash moves with comment text alone, and one further difference is **not explained**: putting
  the comment back to its earlier text gave a third hash, not the first build's. It stays recorded as not
  explained (`evidence/live-preconditions.md`, its last section); no step and no record explains it away. Closed
  at the revision's P3, re-read at 2026-10-09T19:19:17Z: the live home's pinned copy of the first build against
  `target/release-check/release/viola`, the six sections equal, the two file hashes as written here.

## Excluded — owed to other route entries or to the founder
- **A second measurement of the before-change behaviour**, and any reading on a CLI version other than 2.1.287.
- **The wheel's return by a human `release` on a live session**: still the founder's, not measured here.
- **The consequence of the trailing strip not yet shown to the founder** (a listed local command followed by CR or
  CRLF classifies as that command): carried in the handoff; this chunk changes nothing about it.
- **The MCP `send` tool**: the route entry "MCP server for drivers". It reaches the same wrapper `send`.
- **The audit's M1 to M3, the Linux survivors and the `viola-e2e` score**: the route entry "Epoch 3 cleanup II".

## Boundaries
- No control retired or relaxed, and no boundary widened. `validate_paste_text` still runs first on the text as
  received, still refuses every other C0, DEL and C1, and never strips a refused character. CR is an allowed
  character; the typed text carries an LF where the received text carried a CR or a CR LF, and nothing else differs.
- No refusal detail is added. After the change no `send` relies on the CLI's reading of a typed CR. P3 found one
  shape a `send` does rely on that is not measured, an LF typed inside a paste (§1). Answered at P4 (`inputs#I2`):
  this chunk measures it live and lands no ledger row and no `viola verify` probe; the row stays owed to `v1-34`.
- Live `claude` starts: at most 2, 2.1.287 by path, each ledgered before it is made (`inputs#I1`). viola writes
  nothing under the user's `.claude` directory. One was made; after the revision none is made (`inputs#I5`).
- Evidence lives in `chunks/2026-10-09-inner-cr-and-crlf-in-a-sent-text/evidence/`.

## CI read at Setup (the last wrap's flip `3c5e012` → HEAD, the same commit)
| sha | verdict | wall |
|---|---|---|
| `3c5e012b9a43` | green · 15/15 · ci#37968128647 | 439 s |

No red and no `not green` to disposition.
