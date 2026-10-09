# Codebase Research — 2026-10-09-inner-cr-and-crlf-in-a-sent-text

_Rewritten at the plan's revision (2026-10-09T19:20Z, `inputs#I5`). The take-up's findings stand below as read at
the base commit `3c5e012b9a43`; what the stopped implement run found is stated beside them, and the two lists hold
the revised write set._

## Scope
- **Depth:** moderate · **Reads:** 22 at the take-up, 3 at the revision · **Globs/Greps:** 15 at the take-up, 4 and
  one sweep at the revision
- **Harness rules consulted:** `.claude/rules/verification-harness.md` — read in full with its 10 Session Additions;
  applied: 2026-09-25 (never pipe `boot`; each harness step's output goes to a file), 2026-09-27 (a `test(/…/)`
  filter is a regex over test names; a bare name selects nothing and reads `nextest-exit-4`), 2026-10-07 (the fake
  agent receipts the submitting Enter as a `key` line of its own ahead of the prompt's line), 2026-09-29 and its
  2026-10-07 extension (a stalled-start red on the dev host is a finding about the tmpfs backing: read it, stop,
  report, never re-run for green). `.claude/rules/testing.md` — its 35 Session Additions read; applied: 2026-09-24
  (wait on the exact line a test asserts), 2026-09-25 (every new guard test carries its remove-the-guard run),
  2026-10-04 (no mutation entry in a chunk's gate block), 2026-10-06 (a root test with no `send_window_` or
  `verify_window_` name runs under the `mutants` profile's 10 s kill), 2026-10-07 (a product command started from
  a bridge-wrapped session runs with `VIOLA_NAME`, `VIOLA_DIR` and `VIOLA_BIN` removed and `--home` given).
  `.claude/rules/ci.md` — read at the revision: The operator pass, its six steps.
- **Platform issues consulted:** none — no runner-only bullet and no CI-reading entry outside the operator leg
- **External inputs:** `inputs#I1` — the operator's direction at the take-up: the founder's cap of 2 live `claude`
  starts on 2.1.287 by path (one planned after the change, one spare; the before-change readings not re-taken), the
  rig named, the gate-entry rule, the one-window size and the P4 card rule. `inputs#I2` — the P4 card on the
  unmeasured inner LF and its answer (the overseer, 2026-10-09): a live control reading in the planned start, no
  ledger row, the row stays owed to `v1-34`. `inputs#I3` — the implement invocation: the live work's conduct and
  the word for the implementer to run the operator pass with the attempt number read. `inputs#I4` — the answer
  given in the implementing session's dialog on the block's contradiction: the companion edit stays, the live work
  runs, no pre-CI commit and no push while an entry is red. `inputs#I5` — the operator's word at the revision: the
  two defects and nothing else; the live readings and the rehearsal stand, no start is made, the re-entry fires
  the whole block and runs the operator pass.

## Files inspected
At the take-up, coordinates at the base commit `3c5e012b9a43`:
- `crates/viola-agent-claude/src/hook.rs` (150-260, 640-843) — the rule `typed_text` at 207-209 was
  `text.trim_end_matches(['\r', '\n'])`, returning a slice of the received text; its doc comment 194-206 stated the
  rule and its measured basis. The case table `typed_text_drops_every_trailing_newline_and_nothing_else` at 663-686
  held seventeen cases; three of them pinned an inner CR as kept (`an_inner_cr` 674, `an_inner_crlf` 675,
  `a_tab_after_a_crlf` 678). No property covered `typed_text`: the test-module function also named `typed_text`
  at 732 is a proptest strategy for the prompt normaliser and holds no CR piece.
- `src/run/send.rs` (1-1314 read; 1315-1833 swept by function name and by every line holding a CR escape: its six
  CR-bearing cases are trailing tails of the local-command tests, none pins an inner CR) — the wrapper's `send` at
  305-376: `validate_paste_text` on the text as received (320), the typed text taken once (323), then the empty
  check (324), `classify` (334), the in-flight text (342), `ctx.issue(text.len())` (365) and the paste (366) all
  read that one value. `SendSlot::claim` at 125-137 is the equality. The unit helpers
  `confirmed_as(sent, typed, origin)` (909-948) and the trailing-newline cases (962-972, 1064-1106) are the forms
  the new cases took. The module doc comment 1-11 stated the rule.
- `src/cmd/send.rs` (1-335) — the client reads the typed text only in `refusal_of` (190-197), for emptiness; its
  cases at 300-327.
- `crates/viola-pty/src/pump.rs` (40-95) — `PasteHandle::paste(&self, text: &str)` writes the bracketed text and the
  submitting CR in one `write_all`; its span's `text_bytes` is `text.len()` (75). It takes a `&str`, so an owned
  typed text needs no change here.
- `src/bin/viola-fake-agent.rs` (440-494, 564-636) — inside a paste every byte is prompt content; the prompt is
  receipted with `text` and `hex`.
- `tests/cli_send.rs` (225-400, outline) — `send_text_ending_in_newlines_is_typed_without_them_and_confirmed` (289)
  reads the exit, the `--json` document, the receipt's `hex` and `text`, the three records, `text_bytes` on the role
  line, the canary's absence from the role files, and a following send. It boots over
  `fixtures/fake-scripts/path3.json` and ends the turn with `end_turn` before its second send.
- `crates/viola-agent-claude/src/ledger.rs` (111-163) — `LOCAL_COMMANDS` is two entries; the probe paste literals.
- `src/human.rs` (grep) — the `empty-text` hint is the fixed string at 216, pinned at 387-388 and 422-423.
- `.config/nextest.toml` (grep) — `binary(cli_send)` is in the `ci` profile's 10 s × 2 class; under the `mutants`
  profile a case with neither window name is killed at 10 s.
- `viola-0.1.0/chunks/2026-10-09-epoch-3-cleanup/evidence/` — `live-run.md` (whole), `live-preconditions.md`
  (whole), `rehearsal.md` (whole), `live-pty.py` (whole), `live-start.sh` (whole), `live-read.py` (1-237),
  `live-drive.py` (1-60), `live-ledger.py` (1-30), `live-readings.ndjson` (the newline fields of its ten lines).
- `viola-0.1.0/chunks/2026-10-09-epoch-3-cleanup/inputs/` — `I9-relay-1.md.txt` and `I10-relay-2.md.txt` (whole):
  the ruling as relayed and its placement.
- `.andromeda/playbook.md` (whole) — two `verdict: escalate` patterns, both "Boundary widening".

At the revision, the tree as the stopped run left it (HEAD still `3c5e012b9a43`, nothing committed):
- `tests/channel_paste_validation.rs` (whole, 169 lines) — three tests over a raw channel client:
  `channel_paste_refuses_each_control_class` (50), `send_empty_text_straight_to_the_wrapper_is_refused` (98) and
  `channel_paste_accepts_lf_cr_tab_and_multibyte` (153). The third sends
  `{CANARY}`, LF, `line`, CR, `carriage`, TAB, `tab é 中 🙂` (156) and reads the fake agent's one receipted prompt.
  At the base commit it asserted that prompt equal to the text as sent (164 there). The stopped run's edit: the
  expected prompt is a second literal, the same text with the CR as one LF (157), asserted at 166; the module
  comment's sentence says an inner CR is typed as one LF (4-5). `git diff --stat 3c5e012b9a43` reads 8 lines, 5
  added and 3 removed. The first two tests are untouched.
- `viola-0.1.0/chunks/2026-10-09-inner-cr-and-crlf-in-a-sent-text/evidence/` — `live-preconditions.md`,
  `live-run.md`, `rehearsal.md`, `red-green.md` (whole), the three record files (their row and line counts and the
  graded fields), and `scope-record.md` beside them (one line, `companion`).
- `crates/viola-agent-claude/src/hook.rs` (194-228, the tables' and the property's lines by grep) — the rule as
  landed, `typed_text` at 220-227, returning `Cow<'_, str>`; its doc comment 196-219 names the six confirmed
  reading ids.

## Graph impact (from the code-graph query; trace `tree-query-2026-10-09-inner-cr-and-crlf-in-a-sent-text.json` in the take-up's run dir `.andromeda/runs/2026-10-09T18-22-18-phase/`)
- **typed_text** (`hook/typed_text().`, the product function) — 6 rows on `calls` and the same 6 on `refs`
  (rust plane): two product call sites, `refusal_of` @ `src/cmd/send.rs:194` and `send` @ `src/run/send.rs:323`;
  their two `use` lines, `src/cmd/send.rs:17` and `src/run/send.rs:20`; one test call @
  `crates/viola-agent-claude/src/hook.rs:685`. The sixth row is another symbol of the same name,
  `hook/tests/typed_text().`, the proptest strategy, called @ `hook.rs:829`. A changed return type reaches exactly
  the two product callers. Borne out by the stopped run: the changed signature compiled with those two callers
  and no third (`cargo check --workspace --all-targets --features fake-agent`, exit 0).
- **crate edges** — 2 rows: `viola` → `viola-agent-claude`, `viola-agent-claude` → `viola-core`. The rule's crate
  has one consumer, the root bin, and no manifest moves.
- **Name sweep** (`typed_text|text_bytes` over `*.rs` under `src`, `crates/*/src`, `crates/*/tests` and `tests`):
  5 files · 4 changed (`hook.rs`, `src/run/send.rs`, `src/cmd/send.rs`, `tests/cli_send.rs`) · 1 no-change
  (`crates/viola-pty/src/pump.rs`: it takes a `&str` and reads its length). `fuzz/fuzz_targets/paste_text.rs`
  names `validate_paste_text` only, which this chunk does not change. **This sweep was too narrow for the
  companion question**: a test that pins the rule's DATA through the wrapper, without naming `typed_text` or
  `text_bytes`, is not a hit of it. The sweep that answers the question is under Scope premise closure.
- No new graph query was made at the revision: no symbol moves, and the companion is a data pin, which a call
  query does not find.

## Patterns detected
- **One typed text, taken once** (`src/run/send.rs:320-366` at the base commit): validation on the received text,
  then one value read by every later step. The rule changes inside the taking and nowhere else.
- **Measured shapes as labelled cases** (`hook.rs:664-680` at the base commit): one rstest table, each case a
  readable label, the live-measured ones naming the chunk that measured them.
- **A send of `sent` that types `typed`** (`src/run/send.rs:909-948` at the base commit): `confirmed_as` drives a
  send on a clock that stands still until the hook's prompt is on disk, then asserts the paste, the four records,
  the reply and the wheel. The negative beside it (`:1064-1106`) shows a prompt that is not the typed text
  claiming nothing.
- **The cross-process floor and its proof** (`tests/cli_send.rs:279-366` at the base commit): under the fake agent
  the exit and the records are a floor; the receipt's `hex` and the prompt's `text` tell the typed text from the
  sent one.
- **A raw channel client reads the same receipt** (`tests/channel_paste_validation.rs:34-43`, `:160-166`): the
  test connects to the snapshot's endpoint, requests `send` itself and reads the fake agent's receipted prompt.
  It skips the client's checks, not the wrapper's `send`, so it reads the typed text like every other sender.
- **A hand-driven live rig with a ledger** (the last chunk's `evidence/`): each start ledgered before it is made by
  `live-ledger.py`, readings appended by `live-read.py` as codes, counts and equalities only, the sent texts kept in
  a private directory outside the tree.

## Conventions to follow
- **Test names** `<subject>_<condition>_<expected>`, rstest `#[case::<label>]`, inline `#[cfg(test)] mod tests`
  (`.claude/rules/testing.md` §Naming).
- **Oracles are literals**: the expected typed text and `text_bytes` are written out in the test, never computed by
  calling `typed_text` (`.claude/rules/testing.md` §What to assert).
- **Synthetic texts carrying the tests' canary** (`tests/cli_send.rs:23`, `src/run/send.rs:518`,
  `tests/channel_paste_validation.rs:18`).
- **A new test's name decides its kill line**: no new name carries `send_window_` or `verify_window_`
  (`.config/nextest.toml:47-60`).
- **Live records hold no text**: codes, counts, times and equalities only (`live-read.py:3-5`).

## Scope premise closure
Each `[inferred]` bullet of `scope.md`, closed against the reads above. Scope was amended in the same pass, at the
take-up and again at the revision.

| scope bullet | closure |
|---|---|
| §1 the rule (CR LF → LF, other CR → LF, trailing CR and LF gone) | verified: the match is an equality on the in-flight typed text (`src/run/send.rs:130`) and the paste writes that same text (`pump.rs:77-88`) |
| §1 `Measured … the CLI submitted … inner CR LF as one LF, and its lone inner CR as one LF` | spot-checked: the last chunk's `live-readings.ndjson` lines 7 to 10 hold both readings and both equalities |
| §1 `hypothesis: every inner CR and every inner CR LF is submitted as one LF wherever it stands` | not verifiable without the CLI; closed as not leaned on, since no `send` types a CR after the change |
| §1 the CLI submits a pasted inner LF unchanged, "already measured" | **falsified** at the take-up, see below; measured since by the stopped run's control reading `rule-inner-lf` |
| §1 `Today the typed text still holds the CR …` | verified at the base commit, `src/run/send.rs:341-345`, `:130`, `:242-244` |
| §2 one typed text for the match, the paste, `text_bytes`, the classification and both empty checks | verified at `src/run/send.rs:323-366` and `pump.rs:75` |
| §2 the classification gains no member | verified: `ledger.rs:151-154`, two entries, no CR or LF in either |
| §2 a text of only CR and LF stays `empty-text`, hint unchanged | verified: nothing is left after the rule; `src/human.rs:216` |
| §3 a fake-agent test proves the outcome only by reading the typed or submitted text | verified at `viola-fake-agent.rs:590-597`, `:466`, `:484-493` |
| §3 the existing cases that pin an inner CR as kept stand in the `typed_text` table | **falsified** at the revision, see below: a root integration test pinned it too |
| §5 the reading order | corrected at the take-up: a control first, short texts |
| §5 the live work stands, no start is made (`inputs#I5`) | verified: `live-sessions.ndjson` holds 4 rows, one of kind `start`, its `cap` 2 (`jq -c '{kind, n, of}'`); `live-readings.ndjson` and `rehearsal-readings.ndjson` hold 6 lines each (`wc -l`), every one `confirmed` true |
| §5 the loaded sections equal, the file hashes not (`inputs#I5`) | verified at 2026-10-09T19:19:17Z, see below |

**The falsified premise of the take-up.** Scope §1 said P3 would find the inner-LF shape already measured by the
paste-framing row and the multi-line fixtures. It was not:
- the two compiled paste literals hold no newline: `sed -n '111,150p' crates/viola-agent-claude/src/ledger.rs | grep -c -F '\n'` prints 0;
- the recorded prompts under `fixtures/claude/2.1.287/`: `UserPromptSubmit.default.json` 0 LF,
  `UserPromptSubmit.paste-2.json` 0 LF, `UserPromptSubmit.paste-1.json` 5 LF, and those five are the CLI's own
  frame around the pair;
- no reading in the two earlier live record files sent a text with an LF inside it.

So what the CLI does with an LF typed inside a paste had no ledger row and no recorded reading. It has a recorded
reading now (this chunk's `evidence/live-readings.ndjson`, `rule-inner-lf`: confirmed, filed `driver`, the prompt
equal to the sent text) and still no ledger row; the row stays owed to `v1-34` (`inputs#I2`).

**The falsified premise of the revision.** Scope §3 said the cases pinning an inner CR as kept were the
`typed_text` table's. The stopped run's first block run found one more: `channel_paste_accepts_lf_cr_tab_and_multibyte`
read red in the default selection (1 failed of 353) and in `pre-push` (1 failed of 1824), an assertion on a value
at `tests/channel_paste_validation.rs:164`, left the text with LF and right the text with CR. The take-up's name
sweep could not find it, and the plan's preservation guard listed its file as unchanged.

The companion sweep, re-run at the revision over the language's whole source tree: every `*.rs` line under `src`,
`crates`, `tests` and `fuzz` holding a CR escape that does not end its string literal (a script in the session
scratchpad, a walk that follows each CR escape through the CR and LF escapes after it and counts the line when a
content character follows): **34 lines in 11 files · 25 lines in 5 files changed · 9 lines in 6 files no-change**.
- changed, this chunk's own cases or the companion: `crates/viola-agent-claude/src/hook.rs` 10,
  `src/run/send.rs` 8, `src/cmd/send.rs` 2, `tests/cli_send.rs` 4, `tests/channel_paste_validation.rs` 1;
- no-change, none sends a text through `send`: `crates/viola-agent-claude/src/screen.rs` 3 (bytes fed to the
  screen model), `src/bin/viola-fake-agent.rs` 2 (a screen and the fake agent's own paste parser, fed directly),
  `src/human.rs` 1 (the escaping of a shown text), `src/run/wheel.rs` 1 (a terminal reply),
  `tests/cli_controls_not_disableable.rs` 1 (a text with an ESC, refused `control-character`, nothing typed),
  `tests/cli_wait_last.rs` 1 (an assistant message).

The measured backstop beside the sweep: with the companion edit in the tree the block's default selection read
`"ok":true`, unit 1471 and integration 353, and `pre-push` read 1824 of 1824 (the stopped run's whole-block
firing, 19:07Z to 19:09Z). No other case in the suite fails on the rule.

**The build file and what it loads** (re-read at the revision, 2026-10-09T19:19:17Z, HEAD `3c5e012b9a43` with the
stopped run's edits): `sha256sum` of the live home's pinned copy
`target/e2e-home/viola-live-4043089/home/bin/0.1.0-b4659b98029d0f94/viola` reads prefix `b4659b98029d0f94`,
4 000 408 B; of `target/release-check/release/viola`, `4057b91d84010eca`, 4 000 416 B. Each of `.text`, `.rodata`,
`.data`, `.data.rel.ro`, `.eh_frame` and `.gcc_except_table`, read with `objcopy -O binary --only-section=<name>`
and hashed, is equal in the two files. The stopped run's record (`evidence/live-preconditions.md`, its last
section) holds the rest: six builds, three hashes over two comment texts, the build repeatable for a given text,
the stripped files differing in 20 bytes of the build-id note, the unstripped ones in three local symbol names.
Why the first build's file differs from a rebuild of the same comment text is **not known**, and nothing here
closes it.

**The reading order.** A long paste starts the CLI's paste hint, a timer of 8.0 s (`scratch-session.md`,
`hint-window.md` of chunk 2026-10-07-live-rows-and-paste-shapes-on-the-dev-host); the wrap threshold is read
statically, not measured. The live texts therefore stayed short, the control went first, and each shape of unknown
outcome came after every shape with a measured basis.

## Mechanisms the plan will originate
- **The rule decides the match.** For the received text `a` CR LF `b`, the typed text is `a` LF `b`; the in-flight
  text is that value (`src/run/send.rs:343-347` now, the frame that produces it); a hook prompt whose normalised
  text is `a` LF `b` equals it at `:131`, is relabelled `driver` and settles the send. Under the fake agent the
  prompt is the typed bytes (`viola-fake-agent.rs:590-597`, `:466`), so the equality holds there by construction.
- **Red before green at the unit tier and across processes.** Measured by the stopped run
  (`evidence/red-green.md`): 17 of 18 and 3 of 3 red on the present rule, 18 and 3 green on the new one.
- **The companion's equality.** The raw channel test's text is `{CANARY}` LF `line` CR `carriage` TAB …; the
  wrapper's `send` validates it (LF, CR and TAB are allowed), takes the typed text at `src/run/send.rs:324`, and
  pastes that value; the fake agent receipts the pasted bytes as the prompt's `text`. So the receipted prompt is
  the sent text with its one CR as one LF, the literal at `tests/channel_paste_validation.rs:157`. Its control is
  the stopped run's own reading: the old expectation red on the new rule (left LF, right CR), the new one green.
- **The reader's new equality.** `live-read.py` computes "the sent text with every CR LF and every other CR as one
  LF, less its newline ending" itself, without the product; its `selftest` reads 16 literal rows, 0 mismatches.
- **The loaded sections tie the live readings to the final tree.** A process runs what the file's loaded sections
  hold. Six sections hashing equal between the live-start build and the final build is the equality step 12
  needs; a file hash is a stronger equality the build does not give, because it moves with comment text.

## Live work: what stands now
Read at the revision, 2026-10-09T19:19Z:
- one live start was made of the cap of two, ledgered at 19:04:12Z, 9 s before it; its six sends were confirmed,
  the control first; the session was stopped through the rig and left no process (`evidence/live-run.md`);
- the rehearsal under the fake agent held on the same build (`evidence/rehearsal.md`);
- the stamped home `target/e2e-home/viola-live-4043089/home` stands on the tmpfs, with the pinned copy of the
  live-start build under its `bin/`. It is gone at a reboot; the section reading above was taken while it stood;
- `"$HOME/.local/share/mise/installs/claude/2.1.287/claude" --version` is not read again: no start is made.

## New files to create
- none — the chunk's evidence files and its copy of the rig live under the version workspace, which the scope read excludes

## Files to modify
- `crates/viola-agent-claude/src/hook.rs` — the rule, its doc comment, its case tables and a property
- `src/run/send.rs` — the wrapper takes the typed text as a value it may own; the module comment; unit cases
- `src/cmd/send.rs` — the client's cases; its read of the typed text is unchanged in its text
- `tests/cli_send.rs` — one cross-process case
- `tests/channel_paste_validation.rs` — the companion: the expected prompt of the raw-channel case is the typed text

## Open questions
- An LF typed inside a paste is leaned on and not measured: a live reading in this chunk's planned start, or an
  eighteenth ledger row with a fifth Run B paste? → blocks: plan-decision (the P4 card; the ledger rule's limits
  are the founder's). Answered at P4 (`inputs#I2`): the live reading; no row; the row stays owed to `v1-34`.
  Read since: confirmed.
