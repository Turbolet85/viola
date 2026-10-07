# Report — 2026-10-07-a-send-ending-in-a-newline-is-confirmed

**Chunk:** A send ending in a newline is confirmed — a driver's text whose last byte is a newline is reported delivered when it is, and the driver keeps the wheel; the CI red on 9f2bebe closed by cause
**Date:** 2026-10-07
**Commits:** `0093ffe` chore(2026-10-07-a-send-ending-in-a-newline-is-confirmed): operator pre-CI commit, for the run this chunk's verdict reads · `d047fec` docs(2026-10-07-a-send-ending-in-a-newline-is-confirmed): operator fix on the operator's word, the remedy worded as the founder's ruling of 2026-10-07T15:21Z (basis `git log --format='%h %s' 9f2bebe5..HEAD`; `9f2bebe5` is the parent of the oldest pre-CI commit and this chunk's base)

Authority, as the operator directed at this wrap (the run dir's `directive.md`): the remedy is the founder's
live ruling of 2026-10-07T15:21Z, relayed by the overseer (inputs#I10), every option and the `/clear`-plus-newline
consequence shown to him: strip every trailing LF. It was first the overseer's delegate answer of 2026-10-07 on
the P4 card (inputs#I2). `plan.md`'s Goal, step 1 and "Listed for the founder", `research.md` and `scope.md`
still say provisional: they keep the wording of their time and are not edited.

## Changes (structured — detectors read this)
- **Files:** five source and test files, exactly research's five (`gate.py scope` at this wrap: `changed 5 ·
  listed 5 · recorded 0`, base `9f2bebe5`): `crates/viola-agent-claude/src/hook.rs`, `src/run/send.rs`,
  `tests/cli_send.rs`, `src/cmd/run.rs` (one test only), and the new `scripts/profraw-census.sh`. New: the chunk's
  `evidence/` (four records and `guards/read-attempt1.py`) and `inputs/` (ten entries). Size: 395 insertions,
  13 deletions over the five (`git diff --stat 9f2bebe -- crates src tests scripts`).
- **Symbols / APIs:**
  - New public function `viola_agent_claude::hook::typed_text(&str) -> &str`: the sent text without its trailing
    LF characters (`text.trim_end_matches('\n')`). It removes nothing else: not a CR, a TAB or a space, and no
    newline that is not at the very end. Sole product caller: `send` in `src/run/send.rs`.
  - The wrapper's `send` (`src/run/send.rs`): `validate_paste_text` still runs first, on the text as received.
    Directly after it the typed text is taken once, and from there on it is the only text `send` uses: the
    local-command classification (`classify`), the in-flight text the claim compares, the `text_bytes` on
    `send-issued` and on the `pty.paste_write` span, and the paste itself. `SendSlot::claim` is not changed: the
    match is still exact, now against the typed text. Its sole caller is still `append_hook_event`.
  - What a driver sees. A `send` whose text ends in one or more LF characters is typed without them, its
    `prompt-submitted` is relabelled `driver`, the send ends `ok {submitted_at, cursor}`, no `wheel` record is
    appended and the driver keeps the wheel. Before this chunk that send was delivered, reported `not-delivered`
    / `no-prompt-submitted` after the 10 s window, its prompt filed `human` and the wheel moved to the human.
  - `prompt-submitted.text` for such a send is the text as typed (without the trailing LF), not as sent.
  - A consequence of the one typed text: a listed local command followed by newlines is now that command.
    `/clear` and a newline is classified as `/clear` (confirmed by its new session on a verified CLI,
    `unconfirmable` at once elsewhere). Before, it was typed as an ordinary text, and one unit case pinned that;
    that case turned. The founder was shown this consequence with the options (inputs#I10). Proved at the unit
    layer only.
  - A text that ends in a newline and holds a refused control character is still refused `not-delivered` /
    `control-character` with nothing typed, first in the wrapper's order.
  - A text of only newlines is typed as an empty text. What an empty send does is unchanged and not measured
    here.
  - The CLI's own behaviour is unchanged and its measurement stands: `claude` 2.1.287 drops a pasted text's last
    newline before UserPromptSubmit. After this chunk no `send` relies on it, because `send` never types a text
    that ends in a newline.
  - The test `cmd::run::tests::start_opens_the_scenario_one_spans_under_run_start` (`src/cmd/run.rs`) starts its
    wrapper with the host program `whoami` as the child, not the test binary with `--list`. Every span assertion
    is unchanged. No product code in `src/cmd/run.rs` changed.
  - No new IPC method, endpoint, event kind, channel method, socket, port, env var, CLI flag, config key, crate,
    dependency edge, ledger row, stamp, compiled literal, fixture, fake-agent option or turn script. No refusal
    word, detail, hint or column was added. `src/human.rs`, `src/cmd/send.rs`, `src/run/wheel.rs`,
    `src/run/gate.rs` and `crates/viola-core` are untouched (the preservation guard entry, green).
- **Crates / modules:** none added or removed. `viola-agent-claude` gains one public function; the root bin reads
  it through the dependency edge it already has.
- **Dependencies:** none added, none bumped. `Cargo.toml` and `Cargo.lock` are untouched (the guard entry).
- **Schema / config:** none. The diagnostics schemas, `.config/nextest.toml` and every fixture are untouched (the
  guard entry). `text_bytes` keeps its field and type; its value for a send ending in newlines is now the typed
  text's length.
- **Spec-master edits:** none before this wrap.
- **Counts / qualifiers moved:**
  - test-plan §4 What unit tests cover → root bin states "`send_local_command_`, five functions, 13 cases"
    (`test-plan.md:581`). Measured now: eight functions, 15 cases. Three functions were added with one case each
    and `send_local_command_decision_is_exact_text_never_a_leading_slash` went from five cases to four (basis: the
    scratch counter over `fn` names and `#[case]` lines of `src/run/send.rs` at `9f2bebe` and in the work tree:
    5 · 13, then 8 · 15).
  - The per-leg test totals moved by seventeen (ubuntu 1695 → 1712, macos 1691 → 1708, windows 1719 → 1736). No
    master, registry file, rule or docs leaf states any of the six numbers (pattern
    `\b(1695|1691|1719|1712|1708|1736)\b` over the seven masters, `.andromeda/registries/**`, `.claude/docs`,
    `.claude/rules` and `CLAUDE.md`: 0 hits).
  - The ledger stays the closed seventeen-row set (the guard entry covers `ledger.rs`).
- **Dev-tool versions:** none.
- **Harness / gate surface:**
  - New `scripts/profraw-census.sh <runs> <workers> [<name>]`, a dev-host witness. It finds the newest
    instrumented root-bin test binary under `target/llvm-cov-target/debug/deps/` that lists the start test, runs
    that one test `<runs>` times over `<workers>` loops with a per-run `LLVM_PROFILE_FILE` prefix set inside the
    script, counts and sizes each run's profiles and removes them. Its last line is `profraw-census: runs N ·
    passed N · profiles P · third K · short S`. Exit 0 only when every run passed, P equals N and K and S are 0;
    exit 1 on any other count; exit 2 on a usage error or when no instrumented binary is found. It writes under
    `target/profraw-census/<UTC stamp>-<pid>/`, or `target/profraw-census/<name>/` with the third word; both are
    gitignored by the `target/` line. It is not wired into CI, `pre-push` or the harness's five commands: its
    only caller is this chunk's plan gate entry.
  - No workflow file, harness command, status or verdict shape, nextest `retries`, coverage ignore regex or
    gate-required suite changed (the guard entry covers `.github`, `.config` and `crates/viola-e2e`).
- **Cross-project / external claims:**
  - ci#37643226001 measured `0093ffe` (the pre-CI commit): green 15/15, `run_attempt` 1.
  - ci#37644414657 measured `d047fec` (the final sha): green 15/15, wall 507 s, `run_attempt` 1. This is the run
    the chunk's verdict reads. The wrap's own commit adds to that tree.
  - ci#37627485806 attempt 1 measured `9f2bebe` (the previous wrap's commit): failure, 14 of 15 jobs green,
    `test (ubuntu-latest)` red at the coverage merge on a profile `llvm-profdata` refused. Read from the run
    while its artifacts stand (they expire 2026-10-14); the record is `evidence/ci-attempt-1.md`. Attempt 2 of
    the same run is green and closes nothing.
  - `claude` 2.1.287 drops a pasted text's last newline: measured at chunk
    2026-10-07-live-rows-and-paste-shapes-on-the-dev-host, not re-measured here. No live `claude` session was
    started by this chunk.
  - Inputs, from `inputs.py verify` at this wrap (`inputs: 10 entries — unchanged 0 · drifted 0 · vanished 0 ·
    broken 0 · altered 0 · unreachable 0 · n/a 10 · uncited 1 · unparsed 0`). Every entry is a message copy, so
    none has a live source to drift from:
    - I1 · message: the operator, the `/andromeda-phase` invocation, 13:36Z · copy · n/a
    - I2 · message: the overseer, the P4 card answers, 13:58Z (provisional for the founder then) · copy · n/a
    - I3 · message: the overseer, the P5 review · copy · n/a
    - I4 · message: the operator, with the `/andromeda-implement` invocation, 14:15Z · copy · n/a
    - I5 · message: the overseer, the step 7 card, 14:30Z · copy · n/a
    - I6 · message: the overseer, the census card, 14:37Z · copy · n/a
    - I7 · message: the overseer, the revision invocation, 14:40Z · copy · n/a
    - I8 · message: the overseer, the word at the revision's review, 14:50Z · copy · n/a
    - I9 · message: the operator, with the implement re-entry invocation, 15:11Z · copy · n/a · UNCITED by the
      four chunk documents the tool reads (the evidence files cite it; `plan.md` could not, implement does not
      write it)
    - I10 · message: the overseer, the note during the operator pass, received 15:22Z after `0093ffe` was pushed
      · copy · n/a
- **Reverted / negative API facts:**
  - Step 7 as first planned was a child-entry test in the test binary. It was not built and nothing of it
    shipped: the wrapper puts `--plugin-dir <dir>` first in a child's arguments and libtest exits 101 on it
    before any test body runs.
  - The census script first held a two-whole-profiles-a-run rule. It was revised to one a run before any graded
    use, through the plan's revision (inputs#I7, inputs#I8).
- **Insufficient fixes (written, kept, not the remedy):** none.
- **Spec claims disproved by measurement:** none in a spec master. Two plan-level claims were measured false at
  implement and are already corrected in the revised plan, so nothing is left to dispose here: that a child's
  arguments are the test's to give (the wrapper prepends its plugin flag), and that a clean census run leaves two
  profiles (with a host child the version probe's child is not instrumented either, so it leaves one).
- **Expected amendments (from plan):** the sites were located with three patterns over the seven masters and
  every file under `.andromeda/registries/` (a scratch reader that prints each hit with its line, char offset
  and a bounded window). A: `trailing (newline|LF)|ends? in a newline|ending in a newline|last newline|final
  newline|one byte short` — architecture 8, test-plan 2, all others and the registries 0. B: `the sent text|text
  as sent|as sent\b|exact text|exact match|exactly equals` and three more phrasings — architecture 8,
  security-plan 3, design-system 1, layout-templates 1, test-plan 3, registries 0. C: `never strip|not
  strip|never stripped|reject, never` — architecture 2, security-plan 1, test-plan 1, registries 0.
  - architecture §Established Decisions → [Delivery Confirmation] — carried (Symbols: the typed text, the exact
    match). Sites: `architecture.md:49` at c892 (the match compares "the sent text"), c1911 to c2330 (the
    exception sentence, "No remedy is built"), c3980 ("the text as sent is classified").
  - architecture §Established Decisions → [Human Takeover / Wheel] — carried (Symbols: the driver keeps the
    wheel). Site: `architecture.md:70` at c2250 (the exception sentence).
  - architecture §Standard Contracts → Event `data` per kind, `prompt-submitted` — carried (Symbols:
    `prompt-submitted.text` is the text as typed). Sites: `architecture.md:292` at c78, c498 and c678 to c812;
    and the `hook.event` row at `architecture.md:286` c547 ("exactly equals the in-flight `send`'s text"), which
    the plan's list does not name and the same fact reaches.
  - architecture §Established Decisions → [CLI Version Compatibility], the long-paste wrapper row — carried
    (Symbols: the CLI measurement stands, no `send` relies on it). Site: `architecture.md:81` at c1632 to c1953.
  - architecture §Occupied Resources → Repository — carried (Harness / gate surface: the script and
    `target/profraw-census/`). Site: the `target/` rows from `architecture.md:422` on (pattern
    `target/deny-probes`: `:428`).
  - security-plan §Input Validation (Paste text row) and §Security Anti-Patterns → Input — carried (Symbols:
    validation first on the text as received, then the trailing LF removed). Sites: `security-plan.md:223` at
    c780 ("Reject, don't strip: stripping would break the delivery-confirmation exact match") and
    `security-plan.md:540` ("NEVER strip them silently"); and `security-plan.md:74` at c359 (delivery confirmation
    "compares `prompt-submitted` text with the sent text"), which the list does not name. The ruling is the
    founder's (inputs#I10); the operator's word at this wrap is that this amendment is not held for him.
  - test-plan §4 What unit tests cover → root bin, §6 Scenario: Path 2 and §10 Coverage thresholds → Stack
    adjustments — carried (Symbols, Counts, Cross-project claims). Sites: `test-plan.md:581` (the matcher bullet,
    "equals the sent text", and the five-functions count), `test-plan.md:733` to `:754` (§6 Scenario: Path 2;
    it names no case for a text ending in a newline, and the new cross-process case
    `send_text_ending_in_newlines_is_typed_without_them_and_confirmed` belongs to its verification signal),
    `test-plan.md:233` (§1 Test Scope Summary, the same path: "by exact text equality" of the local-command
    list, which the list does not name and the same fact reaches), `test-plan.md:1207` at c604 to c737 (the
    corrupt-profile sentence, pattern
    `profraw|corrupt|no profile can be merged|invalid instrumentation|llvm-profdata`: test-plan 8 hits, of which
    `:1207` ×2 and `:1078` speak of a coverage profile).
  - obs-plan §4 Scenario: Confirmed `send` (CL-1) from driver to readback — carried (Schema / config:
    `text_bytes` counts the typed text). Sites: `obs-plan.md:636` and `:641` (pattern `text_bytes`: obs-plan 5,
    all others 0).
- **Coverage of new surfaces:**
  - `typed_text` on `send`'s text path (no new external surface: the same `send.text` input) → validation
    `validate_paste_text` on the text as received, before the rule✓ · instrumentation the existing
    `send-issued{text_bytes}` line and `pty.paste_write` span, now the typed text's length✓ · PII the text
    reaches no log, `text_bytes` only✓ · tests unit 16 (nine in `hook.rs`, seven in `send.rs`) and integration 2
    (`tests/cli_send.rs`), each read red before the rule was wired, on all three CI OSes · a11y n/a (no UI; the
    `[RB] read back` line is unchanged) · tokens n/a
  - `scripts/profraw-census.sh` (a dev-host script; its three words are its only input) → validation two
    positive-integer checks and a one-path-segment name class, `.` and `..` refused, exit 2✓ · instrumentation
    n/a · PII n/a (it reads profile sizes only) · tests its own known-verdict controls, one-shot and recorded
    (must-fail on the untouched test, must-pass, two planted readings), no unit test · a11y n/a · tokens n/a

## Deviations from intent
- **Step 7 was built as a host child, not as planned.** The take-up's step made the wrapper's child a
  child-entry test of the test binary. That cannot be built (above). On the step 7 card the overseer chose an
  uninstrumented host child, `whoami` (inputs#I5). The plan's revision then rewrote step 7 to the step as built.
- **The census counts one whole profile a run, not two.** With a host child neither child of the wrapper is
  instrumented. Implement stopped before the census entry with nothing pushed, on the overseer's word
  (inputs#I6); the plan was revised through `/andromeda-phase` (inputs#I7) and a planted control was added at its
  review (inputs#I8). The red side of the census stands as read on the untouched test under the two-a-run rule;
  the untouched test was not rebuilt for a second red reading.
- **The script refuses the names `.` and `..`.** Step 6's character class admits them; they would aim the census
  directory at `target/profraw-census/` itself or at `target/`. Not in the step's letter; recorded in
  `evidence/profraw-red-green.md`.
- **The rewording landed as a fix commit, not before the pre-CI commit.** The operator's note asked for it
  before the pre-CI commit and reached the session at 15:22Z, after `0093ffe` was pushed at 15:21:23Z. The
  rewording is `d047fec`, the final sha. Its push was held until the first run had ended.
- **Implement edited `plan.md`.** Four "Expected amendments" lines were reworded from provisional to the
  founder's ruling, on the operator's word in that note (inputs#I10). Implement's letter says it never writes
  the plan. Nothing else in the plan moved.
- **The planted control was not read again after the rewording.** The script did not change between its two
  readings and the rewording, and the control reads the script's rule on planted files, not the binary. The
  must-pass control and the census entry were read again.
- **A limit, stated as a limit:** the closure of the red is by mechanism. The mechanism is measured at HEAD, and
  pid 10799's identity is not provable from the run. Two earlier runs with the same message (ci#36529038462,
  ci#36481260151) are not claimed closed: whether they had this cause is not measurable now.
- **Scope record** (`gate.py scope` at this wrap: `scope: clean — changed 5 · listed 5 · recorded 0 (companion 0
  · mechanical 0 · in-intent 0 · widening 0) · absorbed 0 · excluded 111`). One line, on a file research lists
  (the tool prints `record: listed — src/cmd/run.rs`):
  - in-intent · `src/cmd/run.rs` · serves step 7 · word: "overseer (a technical fork): the uninstrumented host
    child. It removes the mechanism by construction: a child that writes no profile cannot leave a corrupt one,
    whenever the kill lands. Record the deviation from step 7's wording in the scope record with this word, …" —
    the overseer, 2026-10-07, on the step 7 card asked at implement P1 (inputs#I5). The whole word is in
    `scope-record.md`.

## Decisions & corrections
- The remedy: strip every trailing LF after validation. The overseer's delegate answer on the P4 card
  (inputs#I2), then the founder's live ruling at 2026-10-07T15:21Z (inputs#I10). The options shown were a
  tolerant claim in the wrapper (a boundary widening, not chosen), refusing the text, removing exactly one
  newline, and holding the send half.
- The folded red: the overseer's fork on step 7 was the uninstrumented host child (inputs#I5).
- No push goes out while a gate entry of the block is red by its own letter, even a plan defect (inputs#I6).
  Implement stopped for the plan's revision under it.
- A correction of the implementer's own claim. On the step 7 card it told the operator the census gate would
  read as planned with a host child. It reasoned about the wrapper's child alone; the version probe runs the same
  program, so its profile goes too. The must-pass control read one profile a run and the claim was corrected on a
  second card. The operator's first answer had rested on it.
- A correction of implement's P4 report: it said nine census directories stand under `target/profraw-census/`.
  Read at this wrap: eleven, nine stamped and the two planted ones. All are gitignored tallies.
- The census script first read the binary's test list through a pipe to a reader that stops at the first match,
  which broke the lister's pipe and read `no instrumented binary`. The listing is now read whole from a file.
- A name collision the plan did not see: `hook.rs`'s test module already held a proptest strategy named
  `typed_text`, which shadows the new function there. The table calls `super::typed_text`.
- A time typed into an inputs origin label read ten minutes ahead of `date -u`. `inputs.py` has no amend verb;
  the label was corrected by an anchored edit of the manifest and `verify` reads clean.
- `gh api` on a job's logs, redirected to a file, wrote 0 bytes and exited 1 until `--allow-escape-sequences`
  was passed.
- An append to an evidence file written as a `cat` heredoc with a file target was refused by the Bash guard, as
  the host rule file says; the text went in through the Edit tool.
- Sweep hazards: `never stripped` at `architecture.md:136` and `test-plan.md:314` speaks of refused control
  characters and stays true; `stripping` in `security-plan.md:223` is the claim this chunk gives an exception to.
  `trailing newline` at `test-plan.md:1061` is a fixture file's own last byte, not a sent text's. `last newline`
  at `test-plan.md:564` names the two `hook.rs` pins that stay.

## Outcome
Acceptance criteria, each re-asserted against the diff:
- (arch) the three records in order and `ok {submitted_at, cursor}` for a text ending in a newline — MET: the two
  confirmed unit cases and the cross-process case (one LF, two LF), green on the three CI OSes.
- (arch, a11y) no `wheel` record, the holder stays `driver`, a following `send` is not refused `human-typing`; a
  prompt that is not the typed text is appended as the hook filed it — MET: the cross-process case's second send
  and the negative control `send_a_prompt_that_keeps_the_trailing_newline_claims_nothing`;
  `send_an_unsent_human_prompt_takes_the_wheel_before_its_line_and_a_harness_one_does_not` is unchanged and
  passes in the unit entry.
- (arch) the rule is defined in `viola-agent-claude` and only read by `src/run/send.rs`; no new dependency edge;
  the ledger unchanged — MET: the diff, and the guard entry green.
- (security) a text ending in a newline that holds a refused control character is refused `control-character`
  with nothing typed — MET: `send_a_refused_character_before_a_trailing_newline_is_control_character`. It pins the
  outcome, not the order: LF is allowed, so the refusal is the same whichever comes first.
- (security) a `prompt-submitted` text that differs from the typed text is filed by its hook `origin` and the
  send is not confirmed — MET: the negative control.
- (security, arch) no environment read in a product crate, CLI flag, config key, event kind, channel method or
  stamp write — MET: the diff adds none; the guard entry and CI green. The census script sets
  `LLVM_PROFILE_FILE` inside itself only, and is not a product crate.
- (obs) one `send-issued` and one `send-confirmed` on one cursor, no `send-refused`, `text_bytes` the typed
  text's length; schemas unchanged; G2 and G4 pass; the canary in no home-level diagnostics file — MET: the
  cross-process case, `bash scripts/agent-run.sh schema-check` and `bash scripts/g2-zero-panics.sh` green.
- (design, layouts, a11y) the `[RB] read back` line on stdout with exit 0, one `ok` document under `--json`, no
  new refusal word, detail, hint or column — MET: the cross-process case; `src/human.rs` untouched.
- (tests) the unit filter green with sixteen passed, and the same cases red before step 2 — MET (the entry, and
  `evidence/newline-red-green.md`: six cases red on the pasted value; the control-character case cannot be read
  red that way and is recorded so; the rule's own table red with the rule neutralised).
- (tests) the integration filter green with two passed, red before step 2 at the receipt's bytes — MET.
- (tests) `pre-push` green on the uncommitted tree, no breach, coverage floors held — MET (coverage 1712/1712,
  playwright 1/1).
- (tests) the folded red closed by its named cause — MET with the stated limit: `grep -c '"--list"'
  src/cmd/run.rs` reads 0; the census ends `runs 4800 · passed 4800 · profiles 4800 · third 0 · short 0`; the
  evidence holds the census on the untouched test (3 of 4 800 runs with a third profile, all 3 short) and the two
  planted readings, each exit 1, with no planted file left; `retries`, the ignore regex and the required suites
  are unchanged.
- (security, obs) no workflow file changes and no artifact gains a member — MET: `.github` in the guard entry.
- (arch) the CI run reads `verdict: green` and `run_attempt` reads 1 — MET: ci#37644414657 on `d047fec`, and
  ci#37643226001 on `0093ffe` before it.
- The report states the red's limit — MET: under Deviations, in the plan's terms.

Gates, by `run`, in block order. The basis is the operator pass's final state (below); this wrap's light gate
re-runs the block and its result is in the commit and the console report.
- `cargo fmt --all --check` — green (exit 0)
- `cargo clippy --workspace --all-targets --features fake-agent -- -D warnings` — green (exit 0)
- `bash scripts/agent-run.sh run --unit` — green (exit 0, `"ok":true`)
- `bash scripts/agent-run.sh run --unit --filter 'test(/trailing_newline/)'` — green (16 passed, 0 failed)
- `bash scripts/agent-run.sh run` — green (exit 0, `"ok":true`)
- `bash scripts/agent-run.sh run --integration --filter 'test(/ending_in_newlines/)'` — green (2 passed, 0
  failed)
- `grep -c '"--list"' src/cmd/run.rs` — green (exit 1, last line `0`)
- `git diff --quiet 9f2bebe5102b -- src/bin src/human.rs …` (the preservation guard) — green (exit 0)
- `bash scripts/agent-run.sh cleanup --session p-newline-smoke` (pre-clean) — green
- `bash scripts/agent-run.sh boot --session p-newline-smoke --instance builder` — green
- `bash scripts/agent-run.sh status --session p-newline-smoke` — green
- `bash scripts/g2-zero-panics.sh` — green
- `bash scripts/agent-run.sh schema-check` — green
- `bash scripts/agent-run.sh cleanup --session p-newline-smoke` — green (`processes_gone:true`,
  `endpoint_gone:true`)
- `bash scripts/agent-run.sh pre-push` — green (exit 0, `"stage":"linux-tests"`; 56.61 s at the last read)
- `bash scripts/profraw-census.sh 4800 48` — green (exit 0, the last line as the entry asks; 138.47 s at the last
  read). Before its graded use the script's controls read as required: must-fail on the untouched test (exit 1,
  `profiles 9603 · third 3 · short 3`, under the two-a-run rule it then held), must-pass `48 1` (exit 0), and the
  two planted readings (exit 1 with `third 1 · short 0`, exit 1 with `third 1 · short 1`).
- `python -X utf8 ~/.claude/skills/andromeda-phase/../andromeda-tools/scripts/gate.py hygiene` — `leg =
  'operator'`, driven by hand in the pass: exit 0, `hygiene: clean`, at each commit.
- `git diff --quiet && git diff --cached --quiet && git push origin HEAD` — `leg = 'operator'`: exit 0 twice,
  `9f2bebe..0093ffe` and `0093ffe..d047fec`.
- `python -X utf8 ~/.claude/skills/andromeda-phase/../andromeda-tools/scripts/ci.py conclusion --sha HEAD --wait
  1800` — `leg = 'operator'`: exit 0, `verdict: green · checks 15/15`, on `0093ffe` (ci#37643226001) and on the
  final sha `d047fec` (ci#37644414657).
- `gh api "repos/{owner}/{repo}/actions/runs?head_sha=$(git rev-parse HEAD)" --jq '… | max'` — `leg =
  'operator'`: exit 0, last line `1` on `d047fec`, and `1` on `0093ffe`.
- Smoke: the boot path did not change. Boot, status and cleanup ran as the three smoke entries above; not driven
  a second time.
- No entry was deferred, skipped or voided. No red arose in either read of the block, so no red was re-run.

Watches: none folded.

Outcome basis: the operator pass ran, so the verdicts above rest on its final state: the commits `0093ffe` and
`d047fec` and the final HEAD's run ci#37644414657, recorded in `evidence/operator-pass.md`, with
`evidence/profraw-red-green.md` and `evidence/newline-red-green.md` for the local readings. The block was read
green twice on the dev host, before and after the rewording (entries 1 to 16 each time). The session's
conversation was compacted before this wrap, so the basis was read from disk as the operator directed: the
implement run dir `.andromeda/runs/2026-10-07T15-11-35-implement/`, the chunk's `evidence/`, and this chunk's
fifteen implement records in `.andromeda/friction-log.ndjson` (filtered to the marker and both skill spellings).
Implement's P4 report was given in this session after the compaction; it is used for the process census below and
corrected above where the disk reads otherwise. One observation from the pass, cause not measured: the final
run's Windows leg was slower than the first run's (the start test 6.334 s against 0.591 s; two paste-hint cases
of the previous chunk 10.27 s against 8.05 s, crossing nextest's 10 s slow line; all passed). The only source
difference between the two shas is two doc comments.

Unmeasured, carried to "First live test and self-drive": a send of a text ending in newlines on a live CLI after
this build; a text of only newlines; a trailing CR; `/clear` and a newline on a live CLI. On Windows and macOS
`whoami` as the wrapper's child is now measured in CI (both legs green, the start test passed on each).

Process hygiene, from implement's P4 census and re-measured at this wrap:
- the smoke session `p-newline-smoke` (wrapper and fake agent), booted twice by the block's smoke entries —
  terminated; each cleanup reported `processes_gone:true` and `endpoint_gone:true`;
- the census workers and their test children — terminated;
- two `ci.py` waits — exited by themselves;
- no live `claude` session was started;
- re-measured 2026-10-07T19:15Z: 0 processes whose `/proc/<pid>/exe` lies under the repository root. The `viola`
  and `claude` processes on the host belong to other trees and sessions.
