# Report — 2026-10-06-local-command-send-outcomes

**Chunk:** Local-command send outcomes — send's unconfirmable outcome with its mirror line, /clear confirmed by its new-session post-condition
**Date:** 2026-10-06
**Commits:** (base `11c77f7`, the parent of the oldest pre-CI commit; `git log --format='%h %s' 11c77f7..HEAD`)
- `8e66926` chore(2026-10-06-local-command-send-outcomes): operator pre-CI commit, for the run this chunk's verdict reads
- `690aefa` test(2026-10-06-local-command-send-outcomes): operator fix on the operator's word, /remote-control read under --json end to end

## Changes (structured — detectors read this)
- **Files** (`git diff --stat 11c77f7 -- src tests crates .config`: 10 files, 736 insertions, 90 deletions):
  - product: `src/run/send.rs`, `src/cmd/run.rs`, `src/cmd/send.rs`, `src/human.rs`;
  - test-only binary: `src/bin/viola-fake-agent.rs`;
  - tests: `tests/cli_send.rs`, `tests/cli_fake_agent.rs`, `tests/cli_verify.rs`, `tests/support/verify.rs` (a comment);
  - config: `.config/nextest.toml` (a comment; no bound, filter or count moved);
  - new: `viola-0.1.0/chunks/2026-10-06-local-command-send-outcomes/evidence/` (four records).
  - `src/cmd/verify/typed.rs` was neutralised for a control and restored: no net change
    (`git diff --quiet 11c77f7 -- src/cmd/verify/typed.rs` exits 0).
- **Symbols / APIs:**
  - **`send` consumes the compiled local-command list.** In `src/run/send.rs` the text of a send is classified by
    exact equality with an entry of `viola_agent_claude::ledger::LOCAL_COMMANDS` (the text as sent: no trim, no case
    folding, never a leading-slash test). Three classes:
    - not listed: an ordinary send, as before;
    - listed and its post-condition is none, or the CLI version is not verified (`/remote-control` on every
      version, `/clear` on a version with no passing stamp): `send-issued`, the paste, then at once, with no
      confirmation window, the outcome `unconfirmable`;
    - listed with `PostCondition::NewSession` on a verified version (`/clear`): `send-issued`, the paste, then the
      10 s window. The send waits for its post-condition only; a `prompt-submitted` never claims it.
  - **The `unconfirmable` outcome.** The channel reply is `{"ok":{"confirmed":false,"detail":"unconfirmable","cursor":<cursor>}}`,
    an `ok`, never a refusal. `viola send --json` prints `{"v":1,"ok":{"confirmed":false,"detail":"unconfirmable","cursor":<n>}}`,
    exit 0, nothing on stderr. In human mode it prints one line on stdout, exit 0, nothing on stderr:
    `[  ] unconfirmable  builder  local command, no measured post-condition` (the open box, the word, the name in
    the mirror's shared column, one fixed note for both causes; no time, no cursor, no hint).
  - **The post-condition claim.** The hook-event tap (`append_hook_event`) confirms the in-flight send when that
    send waits for a new session and the appended `session-start` line's `cause` is `clear` and its
    `agent_session_id` is a string that differs from the id the slot remembers. Order as for a prompt: claim under
    the lock, append the line unchanged, settle with that line's `ts`. The send then ends like any confirmed send:
    record `send-confirmed {cursor}`, reply `{"ok":{"submitted_at":<that ts>,"cursor":<cursor>}}`, human line
    `[RB] read back`. If the window closes first the send is `not-delivered` / `no-prompt-submitted` with its
    cursor (no new detail).
  - **The remembered session id.** The send slot now keeps the `agent_session_id` of the last `session-start` the
    tap took, whatever its cause, under the same lock as the in-flight send. A line with no string id clears it.
    With nothing remembered, any string id counts as new. The id is read from the event's `data` as a field that
    may be absent or `null`. Nothing persists it and no snapshot carries it.
  - **`SendSlot::new(clock, cli_verified, wheel)`** — one new parameter, the version gate's `cli_verified`. Its two
    callers outside `src/run/send.rs` are both in `src/cmd/run.rs`: `start` passes the value it already read from
    the version gate, the test helper `methods` passes `false`. No stamps read was added: the send path reads no
    `ledger/stamps.json` (gate `grep -rnE 'read_stamps|stamps\.json' src/run/send.rs src/cmd/send.rs`: no output).
  - **`human::write_send_unconfirmable(out, name)`** — the fourth mirror writer, beside `write_send_open`,
    `write_read_back`, `write_send_unable`; one `write_all`, through `mirror_line`. Sole caller: the `send` client's
    new `Out::unconfirmable` arm (`src/cmd/send.rs`), which takes every `ok` whose `confirmed` is `false`.
  - **Refusal order unchanged.** Every refusal rung (params, `validate_paste_text`, the wheel, the one in flight or
    a running turn, the readiness gate) applies to a listed command exactly as to any text; a failed paste is
    `input-not-ready` with the cursor.
  - **Fake agent: `--tag-turn-screen <phase>`** — a new argv option of the test-only `viola-fake-agent`. With
    `--framing` and `--turn-stop`, after the turn of the compiled tag-like paste and after no other turn, the agent
    draws `Screen.<phase>.json` of its set in place of `Screen.turn`. A missing file draws nothing. No env seam.
  - No new channel method, event kind, `ObsEvent`, `NotDelivered` detail, env var, `config.json` key, CLI flag of
    `viola`, port, socket or file.
- **Crates / modules:** none added or removed. Changed: the root bin (`src/run/send.rs`, `src/cmd/send.rs`,
  `src/cmd/run.rs`, `src/human.rs`) and its test-only `[[bin]]`. `crates/viola-agent-claude/src/ledger.rs`,
  `crates/viola-agent-claude/src/screen.rs` and `src/run/gate.rs` are untouched (gate
  `git diff --quiet 11c77f7c9349 -- crates/viola-agent-claude/src/screen.rs src/run/gate.rs crates/viola-agent-claude/src/ledger.rs src/cmd/verify/typed.rs`, green).
- **Dependencies:** none. No manifest or lockfile changed.
- **Schema / config:**
  - `events.ndjson`: the `send-confirmed` record's `data` gains an optional field. A confirmed send still writes
    `{cursor}`; an unconfirmable send writes `{cursor, confirmed:false}`. No reader of that record exists in
    product code.
  - process log (`run-<name>.ndjson`): an unconfirmable send writes one `send-issued` and one `send-confirmed` with
    `confirmed:false`, both with `corr` equal to the cursor and the same `rpc_id`, `conn`/`srv_conn`, `from`,
    `from_trust`, `duration_ms` fields the confirmed line carries; no `send-refused`. `schemas/diag-line.v1.json`
    already admitted a boolean `confirmed` on the send lines: no schema change.
  - no `config.json` key, no ledger row, no stamp shape, no fixture recorded.
- **Spec-master edits:** none.
- **Counts / qualifiers moved:**
  - the fake agent's argv options: seven → eight (`--tag-turn-screen`). Stated at `architecture.md:355`
    ("seven argv options") and at `test-plan.md:1077` ("seven argv options (no env) for verify's four interactive
    runs", then the list) (`grep -n 'seven argv\|argv option' .andromeda/*.md`: architecture 1 line, test-plan
    1 line). [corrected at P2: this bullet first said the test-plan line carried no count; the test-plan detector
    read the line and it does]
  - the readback mirror's lines: three → four. Stated in the keyed contract
    `.andromeda/registries/contracts/architecture/project-directory-structure.md:21` (the `human.rs` line lists
    `[  ] open` / `[RB] read back` / `[/ ] unable`).
  - `send_refusal_order`: 12 → 15 cases. No master states the count (`grep send_refusal_order .andromeda/*.md`,
    sidecars excluded: 0 hits).
  - the verify settle count in two comments: "seven" → ten. No master moved: the key file
    `bootstrap-phases-derive-for-route-setup-project.md` already said ten.
- **Dev-tool versions:** none — no host tool installed, upgraded or re-read. PATH `claude` was not run.
- **Harness / gate surface:** none. No `scripts/agent-run.*` verb, harness subcommand, CI step, status shape or
  nextest bound changed. `.config/nextest.toml` changed in one comment only.
- **Cross-project / external claims:**
  - CI, the push remote `origin` (Turbolet85/viola): ci#37546848541 measured `8e66926`, green 15/15, wall 456 s;
    ci#37547948274 measured `690aefa` (the final HEAD of the operator pass), green 15/15, wall 387 s. Both read by
    `ci.py conclusion --sha HEAD --wait 1800` (`evidence/operator-pass.md`). The three `test` legs of the final run:
    ubuntu-latest 1662 passed, macos-latest 1658 passed, windows-2025 1697 passed.
  - inputs (`inputs.py verify`): `I1 · message: the operator, in the /andromeda-phase invocation, 2026-10-06T22:33Z · copy message · n/a — a message has no live source`;
    `I2 · message: the overseer, founder-delegated, unattended, answering the P4 readiness-gate card, 2026-10-06T22:55Z · copy message · n/a`.
    Summary: 2 entries, drifted 0, vanished 0, broken 0, uncited 0, unparsed 0.
  - no live `claude` session: zero (inputs#I1).
- **Reverted / negative API facts:**
  - the `hint` case's hold was written at the plan's 6 000 ms and shortened to 3 000 ms before the first commit
    (Deviations 1). Nothing else was written and removed.
  - two one-shot controls neutralised a product line each and restored it: the third check of `framing_turns`
    (`src/cmd/verify/typed.rs`) and the two `warn_if_rewritten` calls (`src/cmd/send.rs`). Neither file carries a
    trace of it.
- **Insufficient fixes (written, kept, not the remedy):** none.
- **Spec claims disproved by measurement:** none. One reading is now measured where it was derived: on a verified
  CLI a `send` issued while the fake agent holds the paste hint (a quiet cleared screen with no compiled literal)
  ends `not-delivered` / `input-not-ready` with nothing typed, and the same sequence without the hold is
  delivered (`evidence/paste-hint-send.md`). This agrees with what architecture [Screen Model] states for a quiet
  screen that holds neither literal. Measured under the fake agent only; no `send` was run in the real CLI's hint
  window.
- **Expected amendments (from plan)** (nine entries; each search ran over the seven masters and every file under
  `.andromeda/registries/`, sidecars excluded):
  1. architecture §Established Decisions [Delivery Confirmation] — `send` consumes the list; the exact-text match,
     the remembered session id, the `submitted_at` source, the missed post-condition's detail; the "owed to this
     route entry" sentence retired. **carried** — Symbols / APIs bullets 1 to 4. Sites: `grep -n 'does not consume'`:
     `architecture.md:49` (1), `test-plan.md:233` and `:750` (2), registries 0. The retired sentence at
     `architecture.md:49` reads "`send` does not consume them yet (owed to the "Local-command send outcomes" route
     entry), so today every local command, `/clear` included, still ends `not-delivered` / `no-prompt-submitted`,
     never `ok`".
  2. architecture §Standard Contracts (Event `data` per kind) — `send-confirmed` gains the optional
     `confirmed:false`. **carried** — Schema / config bullet 1. Sites: `grep -n 'send-confirmed'`: architecture
     4 lines (`:49`, `:163` the kind list, `:303` "`send-confirmed`: `{cursor}`.", `:306`), obs-plan 9, a11y-plan 2,
     registries 3 files (`obs-plan/snapshot-paste-to-ai-integration.md`, `obs-plan/log-format-json-schema.md`,
     `test-plan/log-format.md`). The data shape itself is stated at `architecture.md:303` alone.
  3. architecture §Established Decisions [CLI Version Compatibility] — the post-long-paste reading is measured end
     to end under the fake agent; the gate unchanged. **carried** — Spec claims disproved bullet (the measured
     reading) and Crates / modules (the gate's files untouched). Sites: `grep -n 'long paste'`: architecture 2
     lines (`:49`, `:91`); `:91` holds the measured hint window ("holds no input-box literal for 8.0 s from the
     paste, which was 6.5 s after that turn's Stop") and "The readiness gate of `viola run` ([Screen Model]) is
     unchanged by this".
  4. architecture §Occupied Resources (Binary, subcommands and exit codes) — the fake agent's eighth argv option.
     **carried** — Symbols / APIs, the fake-agent bullet, and Counts bullet 1. Site: `architecture.md:355`
     ("seven argv options", 1 hit).
  5. architecture §Infrastructure Patterns → Project directory structure — the `human.rs` line's fourth mirror
     line. **carried** — Symbols / APIs (`write_send_unconfirmable`) and Counts bullet 2. Site: the key file
     `.andromeda/registries/contracts/architecture/project-directory-structure.md:21` (`grep -rn 'human\.rs'`:
     architecture body 0, that key file 1, `architecture/build-system.md` 1 naming the file without the list).
  6. test-plan §6 Path 2 and §1 Critical paths — the `local` verdict as landed, the confirmed `/clear`, the off-list
     slash text, the forced-window pair; §7 Fake agent — `--tag-turn-screen`. **carried** — Symbols / APIs and
     Coverage of new surfaces. Sites: `test-plan.md:233` (the Path's statement, "`send` does not consume them yet
     … until then it is `not-delivered`"), `:750` (the `local` verdict: "exit 13 … while `send` does not consume the
     compiled local-command list … exit 0 with `{confirmed:false, …}` is owed to "Local-command send outcomes""),
     `:751` (Playwright: "once "Local-command send outcomes" makes `send` return it"), `:1077` (the fake-agent
     option list).
  7. obs-plan §4 Scenario "Confirmed `send` (CL-1)" — `run.confirm_window` also closes on a post-condition, and an
     unconfirmable send opens none. **carried** — Symbols / APIs bullets 1 and 3. Sites: `grep -n 'confirm_window'`:
     obs-plan 4 lines (`:313`, `:632`, `:637`, `:648`), registries 1 file
     (`obs-plan/snapshot-paste-to-ai-integration.md`). `:648` reads "`run.confirm_window` closes on the matching
     `prompt-submitted{origin:"driver"}` or window expiry".
  8. layout-templates §Surface: cli (Signature placement) — the filled outcome's second trigger, the
     post-condition. **carried** — Symbols / APIs bullet 3. Site: `layout-templates.md:351` ("`[RB] read back …` on
     stdout with exit 0 when the matching `prompt-submitted` confirms the send"; `grep -n 'Signature placement'`:
     2 lines, `:44` and `:349`).
  9. design-system §Surface: cli → Component Patterns 2 — the same trigger; no new word. **carried** — Symbols /
     APIs bullet 3 (the confirmed `/clear` prints `[RB] read back`; the unconfirmable line uses the existing word).
     Sites: `grep -n 'Component Patterns'`: design-system 2 lines (`:530`, `:731`).
- **Coverage of new surfaces:**
  - `send`'s local-command decision and the `unconfirmable` reply (wrapper, `src/run/send.rs`) → validation
    mechanism✓ (the text passes `validate_paste_text` and every earlier rung first; the decision is a closed
    compiled list read by exact equality) · instrumentation log✓ (`send-issued` + `send-confirmed{confirmed:false}`,
    `corr` = cursor, obs-plan §6 fields only) · PII redacted✓ (the command's text is in no role log line; asserted
    by `send_window_local_command_is_not_presumed_delivered`) · tests unit + integ (five `send_local_command_`
    functions, 13 cases; three `send_refusal_order` cases; `send_window_local_command_is_not_presumed_delivered`,
    `send_window_slash_text_off_the_list_is_not_delivered`) · a11y n/a · tokens n/a
  - the post-condition claim on `session-start` (wrapper tap) → validation mechanism✓ (both fields read as data
    that may be absent or `null`; neither then confirms) · instrumentation log✓ (the ordinary `send-confirmed`
    line; `run.confirm_window` spans the wait) · PII n/a (a session id is appended as the hook filed it, as
    before) · tests unit + integ (`send_local_command_clear_is_confirmed_by_a_new_session`,
    `…_is_not_confirmed_by_another_session_start` ×3, `…_window_expiry_…`;
    `send_clear_on_a_verified_cli_is_confirmed_by_its_new_session`) · a11y n/a · tokens n/a
  - the CLI's unconfirmable output (`viola send`, human and `--json`) → validation n/a (output) · instrumentation
    n/a (the client's existing `process-exit` line, exit 0) · PII n/a (fixed strings and the instance name) · tests
    unit + integ (`write_send_unconfirmable_is_the_open_box_and_the_fixed_note_in_one_write`,
    `write_send_mirror_words_line_up`, `send_writers_return_the_writer_error`; the integration case reads both
    forms) · a11y ASCII, no ESC byte, one line, asserted✓ · tokens the existing word `unconfirmable` and the open
    box✓
  - `viola-fake-agent --tag-turn-screen` (test-only) → validation n/a · instrumentation n/a · PII n/a · tests
    unit + integ (`opts_parse_…` ×2, `fake_agent_tag_turn_screen_draws_the_named_screen_after_the_tag_turn_alone`,
    and its consumer `verify_pastes_no_local_command_once_the_tag_turn_screen_shows_a_modal`) · a11y n/a · tokens n/a
  - no web surface, no UI element, no new region.

## Deviations from intent
1. **The `hint` case's hold is 3 000 ms, not the plan's 6 000 ms.** At 6 000 ms the case read 10.549 s on the dev
   host. It carries no `send_window_` or `verify_window_` prefix, so under the nextest `mutants` profile its kill
   is 10 s (5 s × 2): a mutation run's unmutated baseline would have lost it. The plan's own note directs
   "shorten the hold; never move the bound". At 3 000 ms it reads 7.437 s here and 7.693 s to 7.793 s on the three
   CI legs. No bound moved.
2. **One more wait in that test.** After the `turn-ended` record it also waits for the Stop hook's line in the
   fake agent's receipt, because the agent draws its next screen right after that line: without it the second send
   could reach the gate before the cleared screen was drawn.
3. **The text is classified when the slot is reserved, not after the gate.** The plan says "after the gate". The
   classification is a pure read of the compiled list and changes no refusal rung; the in-flight entry needs it
   from the start so that a `prompt-submitted` can never claim a listed command.
4. **The remembered id where the plan was silent.** With nothing remembered, any string id is new (the plan's own
   unit case, whose record kinds are `send-issued`, `session-start`, `send-confirmed`, requires it); a
   `session-start` with no string id clears the remembered one.
5. **The rewritten-path control's red reading stopped at the name position**, the loop's first pass. Both calls
   were neutralised in that run and the green reading covers both positions
   (`evidence/rewritten-path-warning-control.md`).
6. **`evidence/rewritten-path-warning-control.md` does not spell the test's literal.** The hygiene read takes a
   drive path in an evidence file for a host path; the record describes the literal instead.
7. **After the implement report, on the operator's word:** `send_window_local_command_is_not_presumed_delivered`
   now sends `/remote-control` under `--json` too and asserts its exit 0 and its one document, so the
   `v1-29` acceptance sentence ("`/clear` on an unverified CLI and `/remote-control` each exit 0 with
   `{confirmed:false, detail:"unconfirmable", cursor}`") holds end to end as written. The human-mode assertions
   stay. One fix commit, `690aefa`.

scope record: none — `gate.py scope` clean, 0 recorded (`changed 10 · listed 10 · recorded 0 · excluded 56`, base
`11c77f7`).

## Decisions & corrections
- The operator, with the implement invocation: no live session; run the operator pass with the CI read as usual;
  a stalled-start red is re-read once no other build runs, and with no quiet window, stop and name the minutes.
- The operator, after the implement report: close the named gap "by the test and not by the wording" (Deviations 7).
- The overseer's dispositions, relayed by the operator with the wrap invocation (2026-10-06):
  1. the `input-not-ready` hint finding and the measured paste-hint reading go to "First live test and
     self-drive" as planned; what `send` does while the hint stands stays the founder's decision, recorded as
     open, nothing decided here;
  2. `v1-29` keeps its stated limit in the matrix text;
  3. a stalled-start red at the light gate with another build running: no loop, stop at once and name the minutes.
- Decided in the chunk: one lock for the in-flight send and the remembered session id; one fixed note for both
  unconfirmable causes; `no-prompt-submitted` as the detail of a missed post-condition (the plan's leans).
- Process facts worth keeping:
  - the red-first reading was taken with the decision left unwired at two call sites: the four cases that need it
    hang on a fixed clock until nextest's `ci` kill (120 s), which is the red, not a stall;
  - a test with no window prefix runs under a 10 s kill in the nextest `mutants` profile, tighter than its 20 s
    CI kill: a new stamped-home case is sized against the smaller one;
  - the hygiene read fires on a quoted Windows drive path in evidence, whoever it belongs to;
  - a `cat` heredoc redirected to a file is refused by the Bash guard on the Linux host too;
  - a backticked word inside a double-quoted grep pattern is command-substituted by bash (it ran the installed
    `viola`; nothing was written).

## Outcome
**Acceptance criteria**, each re-asserted against the diff:
- (matrix) `v1-29`: MET — every clause has a named, passing witness, the `/remote-control` `--json` document
  included since `690aefa`; its stated limit (the `/clear` confirmation is proven on the recorded 2.1.287
  `clear-1` variants replayed by the fake agent; the live proof is owed to "First live test and self-drive")
  stands, by the overseer's disposition.
- (arch) the `unconfirmable` channel payload, its `--json` mirror, exit 0, never a refusal: MET
  (`send_window_local_command_is_not_presumed_delivered`).
- (arch) `/clear` confirmed only by a `session-start` with cause `clear` and a new id on a verified version;
  `unconfirmable` on an unverified one: MET.
- (arch) an off-list slash text still ends `not-delivered`, exit 13; the earlier rungs apply to a listed command:
  MET.
- (arch) the post-condition is read through normalised events and the compiled list; std-thread; no env var,
  `config.json` key, ledger row or dependency: MET (the diff holds no manifest, no `ledger.rs` change, no
  `env::var`).
- (security) a refused control character is refused first: MET (unchanged rung; the existing rows green).
- (security) the decision names only `LOCAL_COMMANDS` and `cli_verified`; no second stamps read: MET.
- (security) closed codes and fixed strings only; the command's text in no role log line: MET.
- (security) the `framing_turns` guard restored; no new env var or flag in `viola`; the new option in the
  test-only binary only: MET.
- (obs) one `send-issued` and one `send-confirmed{confirmed:false}` per unconfirmable send, `corr` = cursor, no
  `send-refused`: MET; G4 and G2 passed in CI (the `test` jobs green).
- (layouts) the unconfirmable line on stdout, exit 0, no `unable` or `hint:` on stderr, one document under
  `--json`: MET.
- (design) a post-condition-confirmed `/clear` prints `[RB] read back`; no new word; no SGR byte: MET.
- (a11y) no SGR, cursor-movement or redraw sequence; the tui boundary cases green: MET.
- (tests) the readiness-gate reading measured: MET (`send_under_the_paste_hint_on_a_verified_cli`, two cases).
- (tests) the guard's control, red then green: MET (`evidence/local-command-guard-control.md`).
- (tests) both comments read ten; no nextest bound moved: MET.
- (tests) every entry green or recorded as the block says; coverage holds inside `pre-push`: MET.
- (tests) CI green on the three OSes for the final HEAD: MET — ci#37547948274 on `690aefa`.

**Gates** (the block's 24 entries; /implement's one full run on the final source tree, then the operator pass):
- `cargo fmt --all --check` — green · exit 0
- `cargo clippy --workspace --all-targets --features fake-agent -- -D warnings` — green · exit 0
- `bash scripts/agent-run.sh run --unit` — green · exit 0, `"ok":true` (1329 passed)
- `bash scripts/agent-run.sh run --unit --filter 'test(/send_local_command_/)'` — green · `"passed":13,"failed":0`
- `bash scripts/agent-run.sh run --unit --filter 'test(/send_refusal_order/)'` — green · `"passed":15,"failed":0`
- `bash scripts/agent-run.sh run --unit --filter 'test(/write_send_/)'` — green · `"passed":4,"failed":0`
- `bash scripts/agent-run.sh run --integration --filter 'test(/send_window_local_command_is_not_presumed_delivered|…/)'` — green · `"passed":4,"failed":0` (re-run green after the fix, 13.18 s)
- `bash scripts/agent-run.sh run --integration --filter 'test(/send_under_the_paste_hint_on_a_verified_cli/)'` — green · `"passed":2,"failed":0`
- `bash scripts/agent-run.sh run --integration --filter 'test(/verify_pastes_no_local_command_…|fake_agent_tag_turn_screen_…/)'` — green · `"passed":2,"failed":0`
- `bash scripts/agent-run.sh run --integration --filter 'binary(cli_send) | binary(cli_verify) | …'` — green · exit 0, `"ok":true`
- `git diff --quiet 11c77f7c9349 -- crates/viola-agent-claude/src/screen.rs src/run/gate.rs crates/viola-agent-claude/src/ledger.rs src/cmd/verify/typed.rs` — green · exit 0
- `grep -c 'LOCAL_COMMANDS' src/run/send.rs` — green · exit 0
- `grep -rnE 'read_stamps|stamps\.json' src/run/send.rs src/cmd/send.rs` — green · exit 1, no output
- `! (git diff 11c77f7c9349 -- tests crates/viola-e2e | grep -E …) && ! (… env::var …)` — green · exit 0, no output
- `grep -rn 'seven 300 ms' .config/nextest.toml tests/support/verify.rs` — green · exit 1, no output
- `bash scripts/agent-run.sh run` — green · exit 0, `"ok":true` (1329 unit, 333 integration)
- `bash scripts/agent-run.sh cleanup --session p-send-smoke` — green
- `bash scripts/agent-run.sh boot --session p-send-smoke --instance builder` — green · `"ok":true`
- `bash scripts/agent-run.sh status --session p-send-smoke` — green · `"state":"ready"`
- `bash scripts/agent-run.sh cleanup --session p-send-smoke` — green · `"processes_gone":true`, `"endpoint_gone":true`
- `bash scripts/agent-run.sh pre-push` — green · `"ok":true`, `"stage":"linux-tests"` (three readings: 42.5 s in the block, 43 s and 79.55 s in the operator pass; coverage 1662/1662, playwright 1/1, no breaches)
- `python -X utf8 ~/.claude/skills/andromeda-phase/../andromeda-tools/scripts/gate.py hygiene` — `leg = 'operator'`, driven by hand: first read `refused 1 files` (Deviations 6), then `hygiene: clean` before each commit
- `git diff --quiet && git diff --cached --quiet && git push origin HEAD` — `leg = 'operator'`: exit 0 twice (`11c77f7..8e66926`, `8e66926..690aefa`)
- `python -X utf8 ~/.claude/skills/andromeda-phase/../andromeda-tools/scripts/ci.py conclusion --sha HEAD --wait 1800` — `leg = 'operator'`: exit 0, `verdict: green` twice; the final HEAD's run is ci#37547948274
- Smoke: boot, status and cleanup ran as the four smoke entries above; P3 did not re-drive them.

**Watches:** `pre-push`'s coverage merge failing on truncated profiles while another build writes to the volume ·
3 green run(s) [the block's entry at 2026-10-06T23:25Z, the operator pass at 23:28Z, the operator pass at 23:39Z
with another build running, load average about 100] · not recurred.

**Outcome basis:** the operator pass ran, so the verdicts rest on its final state: the two commits above and
ci#37547948274 on `690aefa`, recorded in `evidence/operator-pass.md`. The implement conversation is present in
this window (implement's P4 report as given, then the operator's directive that produced `690aefa`). The source
tree the block ran on differs from the final HEAD by `tests/cli_send.rs` alone, whose entry and `pre-push` were
re-run green before the fix commit.

**Process hygiene** (implement's census, re-measured at 2026-10-06T23:48Z from the host's process list):

| process | started by | final state |
|---|---|---|
| harness session `p-send-smoke` (supervisor, wrapper, fake agent) | this chunk's gate run | terminated (`processes_gone:true`; none in the list) |
| test wrappers and fake agents of the gate runs | this chunk's gate runs | terminated (none in the list) |
| the `viola-lab` prototype wrappers and their `viola wait` drivers | the operator, before this chunk | left running — not this chunk's |
