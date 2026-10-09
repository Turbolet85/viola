# Report — 2026-10-09-inner-cr-and-crlf-in-a-sent-text

**Chunk:** Inner CR and CRLF in a sent text — a CR or a CRLF inside a driver's text is typed as the LF the CLI submits, so the send is confirmed and the driver keeps the wheel
**Date:** 2026-10-09T20:51Z
**Commits:** `308099b chore(2026-10-09-inner-cr-and-crlf-in-a-sent-text): operator pre-CI commit, for the run this chunk's verdict reads` (the one commit above the base `3c5e012b9a43`; `git log --format='%h %s' 3c5e012b9a43..HEAD`, 1 line)

## Changes (structured — detectors read this)
- **Files:** five, all modified, none new (`git diff --stat 3c5e012b9a43 -- src crates tests`: 5 files, 330 insertions, 26 deletions):
  - `crates/viola-agent-claude/src/hook.rs` — the rule `typed_text`, its doc comment, two test tables, one test, one property;
  - `src/run/send.rs` — the wrapper's `send` (its one taking of the typed text), the module comment, unit cases;
  - `src/cmd/send.rs` — three unit cases; no product line changed;
  - `tests/cli_send.rs` — one cross-process rstest, three cases;
  - `tests/channel_paste_validation.rs` — the raw-channel accepted case expects the typed text (the companion, see Deviations).
  Beside them, under the chunk folder only: `evidence/` (thirteen files: the rig's five scripts, three ndjson record files, five records) and `inputs/` (seven entries).
- **Symbols / APIs:**
  - `viola_agent_claude::hook::typed_text` (`hook.rs:220-227`; the body's new lines `222-226`, from the listing below). **The rule changed:** the typed text is the received text with every CR LF pair as one LF, every other CR as one LF, and without its trailing CR and LF characters. It holds no CR. Before this chunk it removed the ending and nothing else, so an inner CR was typed.
  - **Its return type changed:** `&str` → `Cow<'_, str>`. It is borrowed when no CR is left inside the text once the ending is gone (a text with no CR at all, and a text whose only CRs are its ending), owned when an inner CR is replaced. The value is the same either way.
  - Its product callers are unchanged in number: the wrapper's `send` (`src/run/send.rs:324-325`, takes it once directly after `validate_paste_text`, and reads that one value for the empty check, the local-command classification, the in-flight match, `text_bytes` and the paste) and the client's `refusal_of` (`src/cmd/send.rs:194`, no text changed). No other site in the root bin rewrites a CR. `validate_paste_text` is unchanged and still runs first, on the text as received. `answer` does not call `typed_text`: answer free text is typed as validated.
  - No IPC method, channel frame, event kind, logged field, endpoint, socket, port, env var, exit code, refusal detail or CLI flag is added, removed or renamed. The match in the wrapper stays an exact equality with no tolerance on the submitted side.
- **Crates / modules:** changed `viola-agent-claude` (`hook.rs` only) and the root bin (`src/run/send.rs`, `src/cmd/send.rs` tests). None added, none removed. No dependency edge changed.
- **Dependencies:** none added, none bumped (`Cargo.toml` and `Cargo.lock` equal the base commit: the preservation guard entry, green). `std::borrow::Cow` is the standard library's.
- **Schema / config:** none. No schema file, config key, ledger row, fixture, fake-agent option or `v` bump (`fixtures`, `schemas`, `plugin`, `.config`, `crates/viola-agent-claude/src/ledger.rs` equal the base commit: the same guard).
- **Spec-master edits:** none by the chunk. The masters that state the old rule are amended at this wrap (Expected amendments below).
- **Counts / qualifiers moved:**
  - The unit table of the ending, in `hook.rs`: renamed `typed_text_drops_every_trailing_newline_and_nothing_else` → `typed_text_drops_every_trailing_newline`, and **17 → 14 cases** (three cases that pinned a CR as kept were removed: `an_inner_cr`, `an_inner_crlf`, `a_tab_after_a_crlf`). Basis: plan.md step 3's Done line, an `awk` count of `#[case::` over the table. test-plan §4 states "seventeen labelled cases of one table" (`test-plan.md:565`).
  - New beside it: the table `typed_text_every_inner_cr_is_one_lf`, **10 cases** (the same basis); the test `typed_text_copies_only_a_text_with_an_inner_cr`; the property `typed_text_prop_holds_no_cr_and_every_inner_cr_is_one_lf`, 512 cases.
  - The suite as run at the re-entry (the gate block of 19:29:37Z to 19:31:48Z, its documents): unit 1471 of 1471; integration 353 of 353; `pre-push` coverage 1824 of 1824.
  - The `cr_is_one_lf` filter selects 18 unit cases and 3 integration cases (`evidence/red-green.md`, "Green").
  - Capability-ledger rows gating `cli_verified`: seventeen, unchanged. Capabilities claimed by this chunk: 0.
- **Dev-tool versions:** none — `claude` re-read at 2.1.287 by path (`"$HOME/.local/share/mise/installs/claude/2.1.287/claude" --version` → `2.1.287 (Claude Code)`, dev host, 2026-10-09T19:29Z, the block's by-path probe).
- **Harness / gate surface:** none. No `agent-run` verb, CI step, status shape or nextest profile changed (`.github`, `.config` equal the base commit). The rig in `evidence/` is this chunk's local measurement tool, four scripts byte for byte the last chunk's and `live-read.py` extended by three `reading` fields and a `selftest` verb; it is not a harness command.
- **Cross-project / external claims:**
  - **CI:** run `ci#37981185305` on sha `308099b92f96` (the pre-CI commit), event `push`: `verdict: green · checks 15/15 · wall 717 s`, `run_attempt` 1, the only run on the sha, all fifteen jobs `success` at attempt 1 (`evidence/operator-pass.md`, "Entry 23"). The verdict was taken on that tree; this wrap's commit adds no source file to it.
  - **The `claude` CLI, 2.1.287, measured live on the Linux dev host in one headless session of six sends** (`evidence/live-run.md`, `evidence/live-readings.ndjson`): with the rule, a text holding one inner CR LF, one lone inner CR, two CRs in a row, an LF then a CR, and CR LF on two of three lines were each confirmed, filed `driver`, with no `wheel` record and no CR in the submitted prompt. First, as the control: **an LF typed inside a bracketed paste is submitted unchanged** (`rule-inner-lf`), a shape every multi-line `send` leans on and that no ledger row or `viola verify` probe covers; its row stays owed to `v1-34` (`inputs#I2`). Not measured: any CLI version but 2.1.287; a long or wrapped multi-line text (every text was under 100 bytes, at most three lines); an LF typed inside a paste on Windows; what the CLI does with a typed CR CR or LF CR (with the rule no `send` types a CR).
  - **The ruling the chunk lands** is the founder's, 2026-10-09T16:51Z, live in the overseer's dialog, the options shown, relayed by the operator; on disk as relayed in `viola-0.1.0/chunks/2026-10-09-epoch-3-cleanup/inputs/I9-relay-1.md.txt`.
  - **Inputs** (`inputs.py verify`, 7 entries, each `n/a — a message has no live source`, none drifted, vanished or broken, 0 unparsed):
    - `I1` · message: the operator, the arguments of the phase invocation · copy · n/a (cited by scope, research and plan);
    - `I2` · message: the overseer, the P4 dialog · copy · n/a (cited);
    - `I3` · message: the operator, the implement invocation · copy · n/a (cited);
    - `I4` · message: the answer in the implementing session's question dialog · copy · n/a (cited);
    - `I5` · message: the operator, the phase invocation of the revision · copy · n/a (cited);
    - `I6` · message: the operator, the implement invocation of the re-entry · copy · n/a. `verify` printed it `UNCITED`: its citations stand in `evidence/operator-pass.md` and `evidence/live-preconditions.md`, which `verify` does not read. Cited here: `inputs#I6` is the word for the operator pass at the re-entry and for "no live start is made";
    - `I7` · message: the operator, the wrap invocation, 2026-10-09T20:50Z · copy · n/a. Snapped at this wrap and cited here: `inputs#I7` carries the CLAUDE.md Key-commands line, the founder's word that no epoch boundary is minted inside Epoch 4, and the two route-resolve questions;
    - `I8` · message: the answers in this wrap's question dialog, 2026-10-09T20:58Z · copy · n/a. Snapped at this wrap, after the fan-out was sent, and cited here: `inputs#I8` places the owed `v1-34` row as a new entry at the head of Epoch 5 (the founder's word, live in the overseer's dialog, relayed) and names "Home and code-bearing file integrity" as the owner of the unexplained file-hash difference.
- **Reverted / negative API facts:** none in the shipped tree. One measurement-only edit was made and undone inside the stopped run: `typed_text`'s doc comment was put back to its earlier text for two builds (builds 4 and 5 of `evidence/live-preconditions.md`) and then restored; the final `hook.rs` is byte-identical to the build read three times at prefix `4057b91d84010eca`.
- **Insufficient fixes (written, kept, not the remedy):** none.
- **Spec claims disproved by measurement:**
  - architecture §Established Decisions [Delivery Confirmation] (`architecture.md:49`), test-plan §6 Path 2 (`test-plan.md:750`): "A CR or a CR LF inside a text that has no newline at its end is not covered by the strip … the send ends `not-delivered` / `no-prompt-submitted`", "no test at any tier pins the inner outcome, and no fix has landed (carried on the working route)". **False on this tree:** the rule lands here, the outcome is pinned at the unit tier and across processes, and six live readings confirm it on 2.1.287 (`evidence/live-run.md`). The earlier readings `after-inner-crlf` and `after-inner-cr` stand as records of the build before the rule.
  - The same sentence's not-measured list ("several inner CRs, an inner LF CR") is superseded for the build with the rule: `rule-inner-cr-cr` and `rule-inner-lf-cr` read confirmed. What the CLI does with a typed CR CR or LF CR stays not measured, and nothing leans on it.
  - Every master sentence that defines the typed text as "the sent text without its trailing CR and LF characters" alone, or `typed_text` as "every trailing CR and LF … and nothing else … not … a CR or LF that is not at the very end" (sites under Expected amendments) is now too narrow: an inner CR is replaced.
  - plan.md step 12 as first written ("the build's hash must equal step 8's") was measured false for the file and true for what the file loads; the plan's revision (`inputs#I5`) already replaced it. No master states it.
- **Expected amendments (from plan):** sites located by `/usr/bin/grep -n -o -i -E '(trailing CR|typed text|inner CR|typed_text)'` over the seven masters: architecture 8 lines, security-plan 4, test-plan 8, obs-plan 4, layout-templates 1, design-system 0, a11y-plan 0; every file under `.andromeda/registries/`: 0 hits.
  - architecture §Established Decisions [Delivery Confirmation] — **carried** (Symbols / APIs, External claims, Spec claims disproved). Sites: `architecture.md:49` (the matching sentence; the `typed_text` definition "and nothing else"; "that ruling covers the ending only"; the inner-case paragraph "is not covered by the strip … carried on the working route"; the local-command sentence stays true).
  - architecture §Conventions → Error handling schema and §Standard Contracts (`hook.event`; `prompt-submitted`) — **carried** (Symbols / APIs). Sites: `architecture.md:136` ("the typed text, the text without its trailing CR and LF characters, is empty"), `:286`, `:292` ("the sent text without its trailing CR and LF characters"; "`text` is the text as typed, not as sent"). `:70` ([Human Takeover / Wheel]: "confirmed for LF … and … for CR and CRLF") names the ending's chunks and is read for the inner case too.
  - security-plan §Input Validation (Paste text row), §Threat Model Summary (Attack surface: CLI input), §Security Anti-Patterns § Input — **carried** (Symbols / APIs: validation first and unchanged, no new input class, CR and LF both allowed, no refused character typed or removed). Sites: `security-plan.md:226` ("Trailing CR and LF, the one removal, of allowed characters … and nothing else"), `:73`, `:74`, `:543` ("dropping the trailing CR and LF characters … is not this stripping").
  - test-plan §1 Path 2, §4 (the `typed_text` bullet and its counts; the matcher bullet), §6 Path 2 — **carried** (Counts / qualifiers moved, Spec claims disproved, External claims). Sites: `test-plan.md:233`, `:565` ("seventeen labelled cases of one table whose name carries …"), `:582`, `:750`.
  - test-plan §5 Integration Test Strategy, the wrapper-side paste validation bullet — **carried** (Files; Symbols: a raw channel client gets the rule from the wrapper; `channel_paste_accepts_lf_cr_tab_and_multibyte` expects the typed text, its one inner CR as one LF). Site: `test-plan.md:638`.
  - obs-plan §4 Scenario: Confirmed `send` and §6 Additive field catalog — **carried** (the `text_bytes` gloss follows the typed text; live: sent 70 bytes with one CR LF logged `text_bytes` 69, sent 82 with two logged 80). Sites: `obs-plan.md:636`, `:819`.
  - layout-templates §Surface: cli, "Refusals decided before any request" — **carried as a read**: `layout-templates.md:557` ("one whose typed text is empty once its trailing CR and LF characters are removed"). An inner CR is content and never makes a text empty (CR LF CR is `empty-text` because all of it is ending). The line and the hint do not change; whether the wording is too narrow is the detector's read.
  - Owed, for route-resolve (no master): the ledger row for an LF typed inside a paste, owed to `v1-34`; the unexplained file-hash difference. Both are the operator's word at this wrap (`inputs#I7`).
- **Coverage of new surfaces:** no new external surface, hot-path operation or UI element. The changed one:
  - `typed_text` (the text `send` types) → validation `validate_paste_text` first, unchanged✓ (the property: a typed text of a valid text is valid) · instrumentation existing `send-issued` / `send-confirmed` lines, `text_bytes` the typed length, no field added✓ · PII n/a (no text logged; the canary in no home-level file, read by the cross-process case)✓ · tests unit (2 tables, 1 test, 1 property, 9 send cases) / integration (cross-process rstest ×3, the raw-channel case) / live (6 readings, a local measurement, not a tier) · a11y n/a (the one `[RB] read back` line unchanged) · tokens n/a.

## Deviations from intent
- **The companion edit.** `tests/channel_paste_validation.rs` was outside research's first list. The stopped run changed it when the default selection and `pre-push` read red on its one accepted-text case (an assertion on a value: the prompt came back with LF, the test expected CR). The plan's revision then listed the file (`inputs#I5`). The edit is 8 lines against the base: a second literal for the typed text, asserted in place of the sent one.
- **`typed_text` borrows more often than planned.** The plan said "borrowed when the received text holds no CR". As written it removes the ending first and copies only a text with an inner CR, so a text whose only CRs are its ending is borrowed too. The value is identical; one test outside step 3's list pins both sides (`typed_text_copies_only_a_text_with_an_inner_cr`).
- **The no-tolerance test has two cases** (a CR LF inside, a lone CR inside), not one.
- **The rehearsal's false side on the old build was not taken as a session.** The reader's `selftest` (16 cases, 8 it must read false) and the red side of `red-green.md` read it.
- **The file's hash.** The final build's file (`4057b91d84010eca`, 4 000 416 B) is not the live start's (`b4659b98029d0f94`, 4 000 408 B). The six loaded sections hash equal in both, read three times (19:14:28Z, 19:19:17Z, 19:32:06Z). **The difference is recorded as not explained**, and nothing here explains it.
- **The implementer drove the operator pass** (the pre-CI commit, the push, the CI read) on the operator's word for this chunk (`inputs#I3`, `inputs#I5`, `inputs#I6`; `.claude/rules/ci.md`).
- **Smoke was not driven again by hand at the re-entry:** the boot, `status` `ready`, G2, G4 and the cleanup ran as gate entries of the whole block.
- **Scope record** (`gate.py scope`: `scope: clean — changed 5 · listed 5 · recorded 0`, base `3c5e012b`, `record: listed — tests/channel_paste_validation.rs`):
  - companion: `tests/channel_paste_validation.rs` · serves `crates/viola-agent-claude/src/hook.rs` · authority self. The file is now in research's list, so the tool prints the line as `listed`; it stays as the record of how the edit was made.

## Decisions & corrections
- The founder's ruling (2026-10-09T16:51Z, relayed): `send` types a CR or a CR LF inside a text as the LF the CLI submits. Landed here.
- The overseer's P4 answer (`inputs#I2`): the LF-inside-a-paste shape is measured in the one live start as a control, read first, and lands no ledger row; the row stays owed to `v1-34`.
- The revision (`inputs#I5`): a loaded-section reading replaces file-hash equality as the proof that the live readings are the final tree's; the file-hash difference stays recorded as not explained and no step looks for a cause.
- The operator at the re-entry (`inputs#I6`): no live start is made; a red where no assertion failed on a value is read against the backing and `hostwatch.py` first, never re-run for green; the final sha needs green on its first attempt.
- The operator at this wrap (`inputs#I7`): CLAUDE.md's first Workflow line becomes `**Key commands:**`; no epoch boundary is minted inside Epoch 4 (the founder's word, 2026-10-09).
- Sweep hazards met: `gh run list --commit` given a 12-character sha printed an empty list with exit 0 while the run existed; the full sha listed it. A release-check entry that finishes in about 2 s still rebuilt the product crate (`cargo clean --release -p viola` clears only that crate), so a short wall time is not a stale artifact: read the artifact's mtime.
- A build's file hash moved with comment text alone (builds 2 against 4), so a file hash is not a proof that two builds load the same code; the loaded sections are.

## Outcome
**Acceptance criteria, each re-read against the diff:**
1. The rule, and where it lives — **met**: `typed_text` (`hook.rs:220-227`); callers `send` and `refusal_of` only; unit filter 18 of 18, unit 1471 of 1471.
2. A send with an inner CR or CR LF is one paste of the typed text, confirmed, the wheel kept — **met**: the two `cr_is_one_lf` filter entries green, the default selection green.
3. The raw channel gets the same rule — **met**: `channel_paste_accepts_lf_cr_tab_and_multibyte` with its literal; the file's two refusal tests unchanged; default selection green.
4. Refusal order and detail list unchanged; no tolerance in the match — **met**: the added cases in `send_refusal_order`, `send_empty_text_is_refused_first_after_control_character`, `refusal_of_reads_a_refused_character_before_an_empty_text`, and the two-case no-tolerance test.
5. The property (valid stays valid, no CR, no foreign character) — **met**: 512 cases, in the unit filter entry.
6. The human line and `--json` document unchanged; no line, word, column or hint added — **met**: the cross-process case; `src/human.rs` at the base commit (the guard).
7. One `send-issued` with the typed length, one `send-confirmed` on the same `corr`; canary in no home-level file; G2 and G4 green — **met**.
8. Nothing added to any catalog, ledger, fixture or manifest — **met**: the preservation guard green.
9. `pre-push` `ok` at `linux-tests` and CI `verdict: green` on three OSes — **met**: `pre-push` green twice, coverage 1824/1824; `ci#37981185305`, 15/15, first attempt.
10. The reader's controls before its graded use — **met**: `selftest: 16 cases, 0 mismatches`; the rehearsal six of six; `red-green.md` both sides.
11. The live work inside the cap and headless — **met**: one start of two, 2.1.287, plain pty, the rehearsal held first, no compositor row; the three graded ids confirmed; the three further ids recorded as read; no prediction contradicted.
12. The live readings are the final tree's by what the build loads — **met**: six of six sections equal at 19:32:06Z; the two file hashes recorded as different and not explained.
13. The masters that state the old rule are amended by the wrap — **owed to this wrap's Phase 2**.

No capability is claimed; `v1-10` and `v1-34` stand as they were, `v1-34` with the take-up's dated note.

**Gates** (the re-entry's whole firing, 19:29:37Z to 19:31:48Z: 23 entries, 20 green, 0 red, 3 not run as the operator's leg):
- `cargo fmt --all --check` — green, exit 0.
- `cargo clippy --workspace --all-targets --features fake-agent -- -D warnings` — green, exit 0.
- `bash scripts/agent-run.sh run --unit` — green, exit 0, `"ok":true`; 1471 of 1471.
- `bash scripts/agent-run.sh run --unit --filter 'test(/cr_is_one_lf/)'` — green; 18 of 18.
- `bash scripts/agent-run.sh run` — green; unit 1471, integration 353 of 353.
- `bash scripts/agent-run.sh run --integration --filter 'test(/cr_is_one_lf/)'` — green; 3 of 3.
- `git diff --quiet 3c5e012b9a43 -- …` (the preservation guard) — green, exit 0. It read red at the stopped run while it still named `tests/channel_paste_validation.rs`; the revision took that file out of its read.
- `python3 …/evidence/live-read.py selftest` — green; `selftest: 16 cases, 0 mismatches`.
- `CARGO_TARGET_DIR=target/release-check cargo clean --release -p viola && … bash scripts/release-check.sh` — green; last line `release-check: viola only`; artifact fresh.
- `bash scripts/agent-run.sh cleanup --session p-icr-smoke` (pre-clean) — green.
- `bash scripts/agent-run.sh boot --session p-icr-smoke --instance builder` — green.
- `bash scripts/agent-run.sh status --session p-icr-smoke` — green; `ready`.
- `bash scripts/g2-zero-panics.sh` — green; `g2: clean`.
- `bash scripts/agent-run.sh schema-check` — green; 118 files, 1672 lines.
- `bash scripts/agent-run.sh cleanup --session p-icr-smoke` — green; `processes_gone` and `endpoint_gone` true.
- `"$HOME/.local/share/mise/installs/claude/2.1.287/claude" --version` — green; `2.1.287 (Claude Code)`.
- `jq -e -s … evidence/rehearsal-readings.ndjson` — green; `true`.
- `jq -e -s … evidence/live-sessions.ndjson` — green; `true`.
- `jq -e -s … evidence/live-readings.ndjson` — green; `true`.
- `bash scripts/agent-run.sh pre-push` — green, twice (58.02 s in the block; 58.9 s alone before the push); coverage 1824/1824, doctest 0/0, playwright 1/1, no breach.
- `python -X utf8 …/gate.py hygiene` (`leg = 'operator'`) — driven by hand: `hygiene: clean`, exit 0, read three times before the commit.
- `git diff --quiet && git diff --cached --quiet && git push origin HEAD` (`leg = 'operator'`) — driven by hand: exit 0, `3c5e012..308099b  HEAD -> build/viola-0.1.0`, no force.
- `python -X utf8 …/ci.py conclusion --sha HEAD --wait 1800` (`leg = 'operator'`) — driven by hand: exit 0, `308099b92f96 verdict: green · checks 15/15 · wall 717 s · runs ci#37981185305 completed/success`; `run_attempt` 1.
- No entry carries `defer`. No red at the re-entry; no fix commit.
- Smoke: the boot path changed (the product binary), and the smoke entries above ran green in the block.

**Watches:** none folded.

**Outcome basis:** the operator pass ran, so the verdicts rest on its final state: the pre-CI commit `308099b` and its CI run `ci#37981185305`, recorded in `evidence/operator-pass.md`. Above that: the re-entry's implement conversation and its P4 report (held by this wrap's session through a context summary; the gate listings re-read from the run's own output), `evidence/live-preconditions.md` (its last section), and for steps 1 to 11 the plan's `Done` lines and its "For the report, carried from the stopped run" note, which the revision wrote from the stopped run's report. The CI wall read 717 s against the last chunk's 416 s; which job took longer was not read.

**Process hygiene** (implement's census of 19:47:45Z, re-measured here at 2026-10-09T20:51:30Z over the host's process list, each `viola` process decided by its executable path):
- the gate block and its smoke session `p-icr-smoke` (wrapper, fake agent) — started by the re-entry, ended by the block's own cleanup entry; 0 left.
- the `pre-push` re-run and the `ci.py` wait — exited.
- the stopped run's rehearsal and its one live session — stopped through the rig at 19:03:36Z and 19:05:22Z; 0 processes left (`evidence/live-run.md`, "The census").
- re-measured: 0 processes of this repository's builds; no `cargo`, `rustc` or nextest process. Eight `viola` processes stand on the host, each the other tree's bridge build by its executable path; none is this chunk's.
- left on the tmpfs behind `target/e2e-home` (gone at a reboot): the live home's two new instance directories `icrrehearse` and `icrlive`, and the pinned copy of the live start's build, which still stood at this read.
## New text, by line
Generated by `cites.py added` (cites v1.3); pasted by `splice.py`. No line of this section is typed or edited.
The diff: 3c5e012b (the parent of the oldest pre-CI commit 308099b9) → the work tree.
A row is a block this chunk added: `{first}-{last}`, `@{head}` its head line where not the first, «the head line».

### crates/viola-agent-claude/src/hook.rs — added 108 line(s) in 8 range(s)
added: 5-6 · 196-200 · 208-226 · 678-680 · 696 · 700-728 · 786-817 · 897-913
  - 222-226 «if text.contains('\r') {»
  - 700-716 @714 «fn typed_text_every_inner_cr_is_one_lf(#[case] sent: &str, #[case] typed: &str) {»
  - 718-727 @721 «fn typed_text_copies_only_a_text_with_an_inner_cr() {»
  - 786-797 @788 «fn received_text() -> impl Strategy<Value = String> {»
  - 799-816 @801 «fn every_cr_as_one_lf(received: &str) -> String {»
### src/cmd/send.rs — added 6 line(s) in 1 range(s)
added: 322-327
### src/run/send.rs — added 93 line(s) in 5 range(s)
added: 4-8 · 324-325 · 764-769 · 982-1060 · 1070
  - 1006-1011 «fn send_text_whose_inner_cr_is_one_lf_is_typed_and_confirmed(»
  - 1013-1059 @1020 «fn send_a_prompt_that_keeps_the_cr_while_its_inner_cr_is_one_lf_claims_nothing(»
### tests/channel_paste_validation.rs — added 5 line(s) in 3 range(s)
added: 4-6 · 157 · 166
### tests/cli_send.rs — added 118 line(s) in 1 range(s)
added: 368-485
- 389-484 «fn send_text_whose_inner_cr_is_one_lf_is_typed_with_lf_and_confirmed(»
  - 401-404 «assert!(»
  - 423-426 «assert_eq!(»
  - 430-434 «let line = |event: &str| -> Value {»
  - 440-446 «for file in ["run-builder.ndjson", "cli-builder.ndjson"] {»
  - 451-456 «assert!(»
  - 457-461 «let human = send(»
  - 468-472 «assert!(»
  - 475-478 «assert_eq!(»
  - 479-482 «assert!(»
