# Report — 2026-10-09-epoch-3-cleanup

**Chunk:** Epoch 3 cleanup — the part that stayed after the split: the `viola-e2e` kill, the trailing CR and CRLF strip with the empty-text refusal, three rule homes and one gotchas line
**Date:** 2026-10-09T17:13Z
**Commits:** `211da16` chore(2026-10-09-epoch-3-cleanup): operator pre-CI commit, for the run this chunk's verdict reads (the only commit since the last wrap, `59e791e`; basis `git log --format='%h %s' 59e791e..HEAD`)

## Changes (structured — detectors read this)
- **Files:** changed from the base `59e791e` (basis `git diff --name-only 59e791e`, 94 paths, 18 outside the run dirs and the chunk folder):
  - product: `crates/viola-core/src/lib.rs`, `crates/viola-agent-claude/src/hook.rs`, `src/run/send.rs`, `src/cmd/send.rs`, `src/human.rs`;
  - tests and test configuration: `tests/cli_send.rs`, `tests/channel_paste_validation.rs`, `tests/contract_lints.rs`, `crates/viola-e2e/tests/harness_lifecycle.rs`, `.config/nextest.toml`;
  - leaves written by the chunk as its own stated work (plan step 14): new `.claude/rules/ci.md` and `.claude/rules/testing-src.md`; edited `.claude/rules/verification-harness.md` (a `## Host pressure` section, one entry) and `.claude/docs/gotchas.md` (one entry);
  - the chunk folder: `scope.md`, `research.md`, `plan.md`, `inputs/` (nine entries), `evidence/` (fifteen files).
- **Symbols / APIs:**
  - `viola_core::NotDelivered` gains the variant `EmptyText`, wire value `empty-text` (`crates/viola-core/src/lib.rs:128`, `:139`; its literal pinned in the unit table at `:404`). It is a sixth value of an existing closed enum. It rides the existing refusal `not-delivered` and the existing exit 13.
  - `viola_agent_claude::hook::typed_text` (`crates/viola-agent-claude/src/hook.rs:194-208`): the typed text is the text without its trailing CR and LF characters, every one of them in any order (`text.trim_end_matches(['\r', '\n'])`), and nothing else: not a TAB, a space, or a CR or LF that is not at the very end. Before this chunk it removed trailing LF only.
  - `typed_text` now has two product callers, both in the root bin: the wrapper's `send` (`src/run/send.rs`, as before) and the client's new private `refusal_of` (`src/cmd/send.rs:194-196`). It is no longer a sole-caller function.
  - The wrapper's `send` (`src/run/send.rs:324-326`): directly after the typed text is taken and before the first wheel read, an empty typed text returns `not-delivered` / `empty-text` through the path `control-character` takes (`ctx.not_delivered(None, …)`): nothing reserved, issued or typed, no cursor, the wheel and the running-turn state not read.
  - The client (`src/cmd/send.rs:188-200`): a new private `refusal_of(text) -> Option<NotDelivered>` reads a refused character first, then an empty typed text; `deliver` calls it before `live_endpoint`, so the refusal is made before any frame, with the four acts the control-character refusal takes there (the client's `send-refused` line, `out.unable`, `log_self_exit(13)`, exit 13).
  - `src/human.rs`, `send_hint` (`:215-218`): the cause `empty-text` gives exactly `the text is empty once its trailing newlines are removed; send a text with content`.
  - The refusal order of `send` is now: `control-character`, then `empty-text`, then the two wheel reads and the rest as before. A text that holds a refused character and ends in CR, LF or CRLF is still `control-character` on both sides.
  - Consequence of the wider strip, pinned by a unit case: a listed local command followed by CR or CRLF classifies as that command (`/clear` and a CRLF is `/clear`), as one followed by LF already did.
  - No channel method, event kind, exit code, env var, config key, capability-ledger row, `v` bump, endpoint, socket or port was added or changed. `validate_paste_text` is unchanged; `SendSlot::claim` still compares exactly.
- **Crates / modules:** none added or removed. Changed: `viola-core`, `viola-agent-claude`, the root bin `viola`; in `viola-e2e` one test's name only.
- **Dependencies:** none added, none bumped (`Cargo.toml` and `Cargo.lock` unchanged: the preservation guard `git diff --quiet 59e791e9d321 -- … Cargo.toml Cargo.lock …` reads green).
- **Schema / config:**
  - `.config/nextest.toml`, `[profile.mutants]` (`:43-50`): the `test(/verify_window_/)` override (15 s × 3, a 45 s kill) now stands first, ahead of `package(viola-e2e)` (15 s × 2) and `test(/send_window_/)`. Its period and count are unchanged. `[profile.ci]` is not edited.
  - `schemas/diag-line.v1.json` is unchanged: `send-refused.detail` is an open string there. The new detail value appears on `send-refused` event records and diag lines with no new field.
- **Spec-master edits:** none.
- **Counts / qualifiers moved:**
  - `NotDelivered` variants 5 → 6 (basis: the enum at `crates/viola-core/src/lib.rs`, the base read in research.md).
  - `send_hint` causes: five and `human-typing` → six and `human-typing` (basis: the diff of `src/human.rs`, one arm and one entry in each of its three unit lists).
  - `typed_text`'s case table: nine labelled cases → seventeen (basis: the diff of `hook.rs`: one case replaced, eight added).
  - `typed_text` product callers: 1 → 2 (above).
  - Tests whose names carry `trailing_newline`: unchanged in number; six of them in `src/run/send.rs` each now take LF, CR and CRLF tails as rstest cases.
  - The `mutants` profile's override order (above). The renamed harness test now takes the `verify_window_` class's kill in both profiles: 45 s under `mutants` (was the package's 30 s), 60 s under `ci` (was the profile's 120 s).
  - Suite sizes read at this chunk on the dev host: unit 1450, integration 350, `pre-push` coverage 1800/1800, playwright 1/1, the `viola-e2e` baseline under the `mutants` profile 258 of 258 (basis: implement's fix-loop record and the gate logs of this run).
- **Dev-tool versions:** none — `claude` (the CLI under test) re-read at 2.1.287 by path on the dev host; `cargo-nextest` 0.9.146, `cargo-mutants` 27.1.0, `cargo-llvm-cov` 0.9.1, `rustc` 1.98.1 and node v24.21.0 re-read unchanged by `pre-push`'s `tools` stage.
- **Harness / gate surface:**
  - `crates/viola-e2e/tests/harness_lifecycle.rs:265`: the test is now `verify_window_boot_with_an_unknown_cli_version_is_verify_failed`; its body is unchanged.
  - `tests/contract_lints.rs`: `test_deadlines_sit_below_the_nextest_kill_line` finds the `package(viola-e2e)` override among all the `mutants` overrides wherever it stands; a new case, `mutants_verify_window_override_stands_before_the_harness_package_override` (`:281-302`), pins the order.
  - No change to `scripts/agent-run.*`, `viola-harness`, the fake agent, the status shape, the log format or any workflow under `.github/`.
  - The floor the kill was moved for, measured (`evidence/e2e-floor.md`): verify's four interactive runs against a CLI version with no fixture set waited 8577, 8561, 8563 and 8562 ms; the test passed in 34.482 s under profile `ci`. The planted-hang control (`evidence/planted-hang.md`): TIMEOUT at 45.005 s under `mutants`, PASS at 84.513 s under the default profile.
- **Cross-project / external claims:**
  - CI: run `ci#37963309241` on `211da169f2db`, event `push`, conclusion `success`, 15 of 15 checks, wall 416 s, `run_attempt` 1 (`evidence/operator-pass.md`). The sha is the record: this wrap's commit adds to that tree.
  - The behaviour of `claude` 2.1.287 with a pasted text's CR and LF characters, measured live on the Linux dev host in four headless starts of a founder's cap of five (`evidence/live-run.md`, `live-readings.ndjson`, `live-sessions.ndjson`):
    - on the build before the change, a text ending in CR LF and one ending in CR CR were each submitted by the CLI without the whole ending, filed `human`, not confirmed, exit 13, the wheel to the human (`trailing-crlf`, `trailing-cr-cr`);
    - on the build after the change, texts ending in CR, CR LF and CR CR were each typed without the ending, filed `driver` and confirmed with no `wheel` record (`after-cr`, `after-crlf`, `after-cr-cr`); a text of one CR LF was refused by the client in 1 ms, exit 13, `not-delivered` / `empty-text`, with no record in the wrapper (`after-empty-text`);
    - a text with one CR LF inside it and no newline at its end: the CLI submitted the CR LF as one LF (`after-inner-crlf`);
    - a text with one lone CR inside it and no newline at its end: the CLI submitted the CR as one LF (`after-inner-cr`);
    - in both inner cases the typed text still held the CR, the exact match failed, the prompt was filed `human`, the wheel went to the human, the turn ended, and the send ended exit 13 `not-delivered` / `no-prompt-submitted`.
    - Not measured: several inner CRs, an inner LF CR, any other CLI version.
  - The inputs (`inputs.py verify`, 9 entries: unchanged 1 · drifted 0 · vanished 0 · broken 0 · n/a 8):
    - `I1` · message: the operator, in the /andromeda-phase invocation · copy · n/a (a message has no live source) · cited
    - `I2` · `../additional/andromedaV3:andromeda-overseer/docs/tools/hostwatch.py` · pointer committed@7d315d4e · unchanged · cited
    - `I3` · message: the operator, in the P4 fork dialog · copy · n/a · cited
    - `I4` · message: the operator, at the P5 review · copy · n/a · cited
    - `I5` · message: the operator, in the /andromeda-implement invocation · copy · n/a · cited
    - `I6` · message: the operator, in the implement P2 dialog · copy · n/a · cited
    - `I7` · message: the operator, in the /andromeda-phase invocation of the revision · copy · n/a · cited
    - `I8` · message: the operator, in the /andromeda-implement invocation of the re-entry · copy · n/a · cited by this bullet and by `evidence/operator-pass.md` (`verify` printed it UNCITED: the chunk's three planning files were written before it)
    - `I9` · message: the operator, in the /andromeda-wrap-session invocation; item three carries the founder's ruling of 2026-10-09T16:51Z from the overseer's dialog, relayed · copy · n/a · snapped at this wrap, cited by this bullet (`inputs#I9`)
  - A founder's ruling reached this wrap by relay (`inputs#I9`), given after both inner readings were shown to him: `send` types a CR or a CR LF inside a text as the LF the CLI submits, so such a send is confirmed. It is NOT landed by this chunk: no product file implements it. Its placement is this wrap's route-resolve item.
- **Reverted / negative API facts:** none shipped and reverted. Two scratch edits were made and removed as controls, both recorded: the planted 50 s wait in the renamed test (its marker counts 0 in the file), and three guard removals restored (`evidence/guard-red-green.md`: 12, 6 and 31 red of 70 cases).
- **Insufficient fixes (written, kept, not the remedy):** the strip is correct and shipped for an ENDING of CR and LF characters, and it does not make a text with a CR or a CR LF INSIDE it confirmable. Such a send is still delivered, answered, reported `not-delivered` / `no-prompt-submitted`, and takes the wheel (the two inner readings above). That is by the plan (the ruling of 2026-10-09 morning covers the ending; stripping an inner newline and a tolerant match were rejected approaches). The remainder is owned by the founder's later ruling (`inputs#I9`) and the route entry this wrap pins it on.
- **Spec claims disproved by measurement:**
  - architecture §Established Decisions → [Delivery Confirmation] (`.andromeda/architecture.md:49`) and test-plan §6 (`.andromeda/test-plan.md:750`) state, as measured on 2026-10-08, that "a text of only newlines is issued with `text_bytes` 0, draws no `prompt-submitted`, and ends `not-delivered` / `no-prompt-submitted` (exit 13) when the window closes". After this chunk that is no longer what the product does: such a text is refused at once as `empty-text` and is never issued (`after-empty-text`; the unit and cross-process cases). The 2026-10-08 measurement stands as a record of the earlier build.
  - The same two passages state that "a text ending in one CR is typed with the CR … the exact match fails, the prompt is filed `human`". After this chunk a text ending in CR is typed without it and confirmed (`after-cr`).
  - No statement about an inner CR or CR LF was found in the masters (pattern `inner` beside `CR`, read in the passages the `trailing LF` sweep returned); the two inner readings are new facts, not disproofs.
- **Expected amendments (from plan):** the searches ran over the seven masters and every file under `.andromeda/registries/` (`grep -c -E`, lines counted).
  - architecture §Established Decisions → [Delivery Confirmation] (the typed text without trailing CR and LF; an empty typed text refused `empty-text`; the trailing-CR and newline-only sentences replaced by this chunk's readings; `/clear` with CR or CRLF) — carried: Symbols / APIs and the two disproved claims. Sites: `trailing LF` architecture 4 lines (`:49` twice in one line, `:70`, `:286`, `:292`), `only newlines` architecture 1 line (`:49`, twice in it).
  - architecture [Human Takeover / Wheel], §Standard Contracts → Channel methods and Event `data` per kind (the same typed-text sentence) — carried: Symbols / APIs. Sites: `:70`, `:286`, `:292` of the same sweep.
  - architecture §Conventions → Error handling schema (`empty-text` in the detail list and in the order, after `control-character`) — carried: Symbols / APIs. Sites: `control-character` architecture 2 lines; `empty-text` 0 hits in every master and key file.
  - architecture §Infrastructure Patterns → Project directory structure, key file (the self-healing entry cited by title where it holds `:93`) — carried: it is the wrap's citation item, not a code fact. Site: `.andromeda/registries/contracts/architecture/project-directory-structure.md:49` ("healing owed to route :93"), 1 hit.
  - security-plan §Input Validation, §Threat Model Summary, §Security Anti-Patterns → Input (the typed-text sentence and the empty-text refusal, worded so that "never refuse LF, CR or TAB" stands) — carried: Symbols / APIs. Sites: `trailing LF` security-plan 4 lines (`:73`, `:74`, `:226`, `:543`); `control-character` 7 lines.
  - security-plan §Authentication & Authorization, IPC client-side server verification row (the two Epoch 6 entries cited by title, four times) — carried as the citation item. Site: `.andromeda/security-plan.md:209`, the pair `:125` / `:127` four times in one line.
  - test-plan §4 (the `typed_text` cases and the matcher cases), §6 Path 2 (the two measured bullets), §6 Security control negatives (the entry cited by title, twice), §3 Bootstrap phases `test-runner-install` (the `mutants` override order) — carried: Counts / qualifiers moved, Schema / config, Harness / gate surface, the disproved claims. Sites: `trailing LF` test-plan 4 lines (`:233`, `:238`, `:565`, `:582`), `only newlines` `:750`, `:127` twice in `:992`; `verify_window_` test-plan 1 line and the key file `registries/contracts/test-plan/bootstrap-phases-derive-for-route-setup-project.md` 2 lines, `package(viola-e2e)` that key file 1 line.
  - obs-plan §1, §4 Scenario: Confirmed `send`, §6 Additive field catalog and `detail` code catalog (`text_bytes` wording and `empty-text`); obs-plan §4 Scenario: `wait` / `last` (the two entries cited by title) — carried: Symbols / APIs, Schema / config. Sites: `trailing LF` obs-plan 2 lines (`:636`, `:819`), `text_bytes` 5 lines, `control-character` 5 lines, the stale pair at `:660`.
  - design-system §Color Palette → Domain status colors, §Surface: cli → Component Patterns 2 and Exit-code phraseology (the `empty-text` row, hint and word) — carried: Symbols / APIs (the hint's exact wording). Sites: `control-character` design-system 3 lines, `not-delivered` 17 lines.
  - layout-templates §Surface: cli — Component — Primary content block 2 (the new refusal pair, decided before any request) — carried: Symbols / APIs. Sites: `not-delivered` layout-templates 5 lines (`:92`, `:222`, `:414`, `:463`, `:558`); `control-character` 0 lines there.
  - The owed route write (the split-off entry at the head, ahead of "Windows mutation grade") — route-resolve's, P5 of this wrap; not an amendment.
  - No ledger-note entry is listed. a11y-plan has no entry: its `not-delivered` rows (4 lines) carry no detail list (read at the sweep), and §8 Error recovery already covers a refusal with its hint as the last stderr line.
- **Coverage of new surfaces:**
  - the `empty-text` refusal on `send` (CLI stdin / `--file`, and the channel `send` frame) → validation mechanism✓ (the client before any frame and the wrapper first, after `validate_paste_text`; closed enum) · instrumentation log✓ (one `send-refused` line per deciding side with the closed `refusal` and `detail`, its `side`, and no text; the wrapper's carries `corr`, `conn` and `rpc_id`) · PII redacted✓ (no text in any line; `text_bytes` is a length) · tests unit/integ✓ (`deliver_empty_text_is_refused_before_any_frame`, `refusal_of_reads_a_refused_character_before_an_empty_text`, `send_empty_text_is_refused_first_after_control_character`, `send_empty_text_under_a_human_wheel_is_still_empty_text`, `send_empty_text_while_a_turn_runs_is_still_empty_text`; cross-process `send_empty_text_is_refused_at_once`, `send_empty_text_straight_to_the_wrapper_is_refused`) · a11y ✓ (the struck mirror line, the hint as the last stderr line, no SGR byte; one document under `--json`) · tokens n/a (a CLI line, no colour of its own)
  - the wider `typed_text` (an existing operation, changed) → validation mechanism✓ (it runs after `validate_paste_text` on the text as received) · instrumentation log✓ (`text_bytes` is the typed text's length on `send-issued` and `pty.paste_write`) · PII n/a · tests unit/integ✓ (the seventeen-case table; the CR, CRLF and two-CR tails of `send_text_ending_in_newlines_is_typed_without_them_and_confirmed`) · a11y n/a · tokens n/a
  - No UI element, no web surface.

## Deviations from intent
- **The split (the founder's word, `inputs#I3`).** The working-route entry names six things; this chunk built the part that stayed. The audit's M1, M2 and M3, the eighteen Linux survivors and the `viola-e2e` whole-unit score left with the split and are owed to the entry this wrap mints at the head of the route. The title's "viola-e2e scored" is met here only as far as the unit's unmutated baseline passing under the `mutants` profile.
- **Step order (implement, first run).** Plan step 14 (the rule homes and the gotchas line) was written before steps 12 and 13, because gate entries 11 to 13 read those files and stand before the release-check entry step 12 fires the block up to.
- **Two gate entries could not read green as first written.** The two `jq` readers of the live records carried an `artifact` key naming files they only read, so each read `artifact STALE` with exit 0 and `true`. Implement stopped before the operator pass and the operator had the plan revised (`inputs#I6`, `inputs#I7`): the key left both entries, and one reading was added as start 4 of the cap of five.
- **One unit case beyond the plan's count** in a listed file: `refusal_of_reads_a_refused_character_before_an_empty_text` in `src/cmd/send.rs` (`:315-327`), beside the planned `deliver_empty_text_is_refused_before_any_frame`; and the helper `refusal_of`, which the plan describes as acts inside `deliver`. Both sit inside the plan's step 9 and its file.
- **The one-shot readings** (the floor test under profile `ci`, the planted hang, the guard removals) were run through `cargo nextest` in the harness's own argument form, not through `agent-run.sh run`, because the harness document holds no status line.
- **The rig's private directory** is named by the plan and recorded in no chunk artifact. The re-entering session found the first session's by a file search and reused it, so the journal and the row files stay one sequence.
- **The equality line of `after-inner-cr`** carries three fields beyond the plan's list (`prompt_submitted_records`, `prompt_origin` and the byte counts), as the inner-CRLF line did.
- **A restart ended the implementing session inside the operator pass**, before any commit; the pass was finished on the resumed tree after the hygiene and scope readings were taken again (`evidence/operator-pass.md`).
- scope record: none — `gate.py scope` clean, 0 recorded (changed 10 · listed 10; base `59e791e9`, the parent of the pre-CI commit).

## Decisions & corrections
- The founder (relayed, `inputs#I3`): the split in two; the detail's name `empty-text`, its condition (the typed text is empty, the empty text included) and its hint's wording. The overseer: its rung, beside `control-character`, ahead of both wheel reads. The operator: the kill takes the `verify_window_` name; every stale citation site found is in the citation item.
- The operator at the review (`inputs#I4`): the live sessions run headless on a plain pty the rig opens itself, rehearsed under the fake agent first; no compositor, no window, no key.
- The operator at the stop (`inputs#I6`, `inputs#I7`): revise the plan first; the `artifact` key off the two readers; one lone-inner-CR reading as start 4 of 5, no behaviour change.
- The founder at 2026-10-09T16:51Z (relayed, `inputs#I9`), both inner readings shown, the other options shown being "refuse before typing" and "leave as measured": `send` types a CR or a CR LF inside a text as the LF the CLI submits, so such a send is confirmed.
- A gate entry that only READS a file a hand-driven step wrote takes no `artifact` key: the key's freshness bound is the entry's own start, so a reader reads `STALE` at every firing.
- A plan whose gate block lists a probe of a file ahead of the entry a step fires the block "up to" fixes the order of the steps that write that file, whatever the step numbers say.
- `py_compile` on a script kept in `evidence/` leaves a bytecode cache there; scripts in the chunk folder are run with `python3 -B`.
- The private directory of a hand-driven rig is recorded nowhere in the tree by design (no host path in evidence), so a re-entry after a context reset has to find it; it stood under the earlier session's scratch directory.
- Sweep hazard: a bare `:93` or `:125` also matches clock times and byte counts in the masters; the stale route numbers are found with their backticks or with the word `route` beside them.

## Outcome
- Acceptance criteria, each against the diff:
  - A `send` whose text ends in LF, CR or CRLF, or several of them, is typed without them and confirmed — met: `typed_text` and its seventeen cases; the CR, CRLF and two-CR tails of the cross-process case; the six `trailing_newline` unit tests with their three tails; live `after-cr`, `after-crlf`, `after-cr-cr`.
  - A text whose typed text is empty is refused `not-delivered` / `empty-text`, exit 13, by the client before any frame and by the wrapper — met: the five unit and two cross-process cases; live `after-empty-text`. The rung stands after `control-character` and ahead of both wheel reads.
  - A text ending in CR, LF or CRLF that also holds a refused character is still `control-character` — met: `send_a_refused_character_before_a_trailing_newline_is_control_character` (three tails) and the client's `a_refused_character_then_a_crlf` case.
  - Human mode prints the struck mirror line and exactly the founder's hint last; `--json` is one document — met: `send_empty_text_is_refused_at_once` and the unable-line unit case.
  - Each deciding side leaves one `send-refused` line with the closed pair, its `side`, no text — met: the same two cross-process cases; `g2-zero-panics.sh` and `schema-check` green.
  - The chunk adds no exit code, channel method, env var, config key, ledger row, `v` bump or manifest edge — met against the diff: the modify-set holds none; the preservation guard is green.
  - The renamed test passes under profile `ci` and the unit's unmutated tests pass under the `mutants` profile with no TIMEOUT — met: 258 tests run, 258 passed; both evidence files hold their readings.
  - The override order is pinned — met: the new `contract_lints` case, green in the default selection.
  - The live work stays inside the cap and headless — met: four `start` rows, cap 5, each on 2.1.287 and on a plain pty, the fake-agent rehearsal held, no compositor row; eight named readings.
  - The rule homes — met: the three read entries green.
  - `pre-push` reads `ok` true at `linux-tests`, and `ci.py conclusion` reads `verdict: green` — met: `ci#37963309241`.
  - (wrap) Every stale bare route number cited by title, and (wrap) the split-off entry minted — this wrap's P2 and P5.
- Gates (the whole block, one firing of this wrap's implement run, 16:48:34Z to 16:52:11Z: 27 entries, 24 green, 0 red, 3 not run):
  - `cargo fmt --all --check` — green · exit 0
  - `cargo clippy --workspace --all-targets --features fake-agent -- -D warnings` — green · exit 0
  - `bash scripts/agent-run.sh run --unit` — green · exit 0, `"ok":true`
  - `bash scripts/agent-run.sh run --unit --filter 'test(/empty_text/)'` — green
  - `bash scripts/agent-run.sh run` — green · 51.54 s
  - `bash scripts/agent-run.sh run --integration --filter 'test(/send_empty_text/)'` — green
  - `bash scripts/agent-run.sh run --integration --filter 'test(/verify_window_boot_with_an_unknown_cli_version/)'` — green · 36.21 s
  - `CARGO_TARGET_DIR=target/mutants cargo build --package viola --features fake-agent` — green
  - `NEXTEST_PROFILE=mutants … cargo nextest run --package viola-e2e --features fake-agent` — green · `258 tests run: 258 passed`, no TIMEOUT
  - `git diff --quiet 59e791e9d321 -- …` (the preservation guard) — green
  - `grep -c 'verify_window_' .claude/docs/gotchas.md` — green
  - `head -q -n 1 .claude/rules/ci.md .claude/rules/testing-src.md | grep -c -x -e '---'` — green · last line 2
  - `grep -c 'hostwatch.py' .claude/rules/verification-harness.md` — green
  - `CARGO_TARGET_DIR=target/release-check cargo clean --release -p viola && … bash scripts/release-check.sh` — green · last line `release-check: viola only`, the artifact fresh
  - `bash scripts/agent-run.sh cleanup --session p-e3-smoke` — green
  - `bash scripts/agent-run.sh boot --session p-e3-smoke --instance builder` — green
  - `bash scripts/agent-run.sh status --session p-e3-smoke` — green · `state` `ready`
  - `bash scripts/g2-zero-panics.sh` — green · `g2: clean`
  - `bash scripts/agent-run.sh schema-check` — green · 112 files, 1456 lines, 0 failures
  - `bash scripts/agent-run.sh cleanup --session p-e3-smoke` — green · `processes_gone` and `endpoint_gone` true
  - `"$HOME/.local/share/mise/installs/claude/2.1.287/claude" --version` — green · `2.1.287 (Claude Code)`
  - `jq -e -s '([.[] | select(.kind == "start")]) as $s | …' …/evidence/live-sessions.ndjson` — green · `true`
  - `jq -e -s '([.[] | select(.kind == "reading") | .id]) as $r | …' …/evidence/live-readings.ndjson` — green · `true`
  - `bash scripts/agent-run.sh pre-push` — green · 69.09 s, and again before the pass, 70.89 s: coverage 1800/1800, playwright 1/1, no breach
  - `python -X utf8 ~/…/gate.py hygiene` (`leg = 'operator'`) — driven by hand: exit 0, `hygiene: clean`, read three times, the last at 17:01:15Z right before the commit
  - `git diff --quiet && git diff --cached --quiet && git push origin HEAD` (`leg = 'operator'`) — driven by the implementer on the operator's word: exit 0, `59e791e..211da16`
  - `python -X utf8 ~/…/ci.py conclusion --sha HEAD --wait 1800` (`leg = 'operator'`) — exit 0, `verdict: green · checks 15/15`, `ci#37963309241`, attempt 1
  - No `defer` entry. Smoke: the plan lists the boot, status and cleanup sequence as block entries; it ran green there and was not driven a second time.
- Watches: none folded (the coverage-profile watch was retired at the last wrap).
- Outcome basis: the operator pass ran, so the verdicts rest on its final state: the one pass commit `211da16` and its CI run `ci#37963309241`, recorded in `evidence/operator-pass.md`. The re-entry's implement report (steps 16 to 18) is in this conversation. Steps 1 to 15 were another session's: their basis is the chunk's own evidence files, the plan's state paragraph at the revision, and that run's evolve records (the friction log filtered to this chunk and both spellings of the implement skill: three step records and three friction records before the re-entry). That run's process census is not in those records: unmeasured for steps 1 to 15 beyond what `live-sessions.ndjson` holds (every census row reads 0 processes left).
- Process hygiene (the re-entry and the pass, measured against the host's process list at 17:08:57Z): the `live-pty.py` host, `viola run` and `claude` 2.1.287 of start 4 — started by this run — terminated (Ctrl-C through the pty, exit 0, not killed); the harness smoke session — started by the block's boot entry — terminated (`processes_gone` true); the gate block, `pre-push` and `ci.py` — terminated. One compositor process stands on the host, the desktop's own since 2026-10-04; this chunk started none.
## New text, by line
Generated by `cites.py added` (cites v1.3); pasted by `splice.py`. No line of this section is typed or edited.
The diff: 59e791e9 (the parent of the oldest pre-CI commit 211da169) → the work tree.
A row is a block this chunk added: `{first}-{last}`, `@{head}` its head line where not the first, «the head line».

### .config/nextest.toml — added 8 line(s) in 1 range(s)
added: 43-50
### crates/viola-agent-claude/src/hook.rs — added 26 line(s) in 7 range(s)
added: 194-206 · 208 · 660-662 · 666-670 · 672 · 674-675 · 678
### crates/viola-core/src/lib.rs — added 3 line(s) in 3 range(s)
added: 128 · 139 · 404
### crates/viola-e2e/tests/harness_lifecycle.rs — added 1 line(s) in 1 range(s)
added: 265
### src/cmd/send.rs — added 47 line(s) in 5 range(s)
added: 3-5 · 17 · 188-190 · 192-200 · 298-328
  - 194-196 «typed_text(text)»
  - 298-313 @304 «fn deliver_empty_text_is_refused_before_any_frame(#[case] text: &str) {»
  - 315-327 @322 «fn refusal_of_reads_a_refused_character_before_an_empty_text(»
### src/human.rs — added 20 line(s) in 4 range(s)
added: 215-218 · 379-389 · 421-424 · 556
### src/run/send.rs — added 168 line(s) in 17 range(s)
added: 5-7 · 300-304 · 324-326 · 723-724 · 726-761 · 960-961 · 965-969 · 974-983 · 986-1050 · 1064-1068 · 1075
       1642-1649 · 1655 · 1669-1677 · 1683 · 1693-1703 · 1710
  - 324-326 «if text.is_empty() {»
  - 1004-1021 @1007 «fn send_empty_text_under_a_human_wheel_is_still_empty_text() {»
  - 1023-1040 @1025 «fn send_empty_text_while_a_turn_runs_is_still_empty_text() {»
### tests/channel_paste_validation.rs — added 60 line(s) in 2 range(s)
added: 4-5 · 93-150
- 93-149 @97 «fn send_empty_text_straight_to_the_wrapper_is_refused() {»
  - 101-108 «for text in texts {»
  - 111-114 «let refused: Vec<&Value> = events»
  - 116-122 «for record in refused {»
  - 125-130 «let role = support::ndjson::read_lines(»
  - 131-134 «let lines: Vec<&Value> = role»
  - 135-139 «assert_eq!(»
  - 140-147 «for line in lines {»
### tests/cli_send.rs — added 84 line(s) in 4 range(s)
added: 279-282 · 286-288 · 309-312 · 368-440
  - 309-312 «assert!(»
- 368-439 @377 «fn send_empty_text_is_refused_at_once(#[case] text: &str) {»
  - 386-390 «assert_eq!(»
  - 399-402 «assert_eq!(»
  - 404-408 «assert_eq!(»
  - 412-415 «let refused: Vec<&Value> = client»
  - 416-420 «assert_eq!(»
  - 421-425 «for line in refused {»
  - 426-429 «let exits: Vec<&Value> = client»
  - 433-437 «assert!(»
### tests/contract_lints.rs — added 65 line(s) in 6 range(s)
added: 14-18 · 20-33 · 112-117 · 129-132 · 248-253 · 274-303
  - 21-33 «lines»
  - 113-116 «assert_eq!(»
  - 249-252 «let e2e_body = overrides»
- 274-279 @275 «fn override_at(overrides: &[Vec<String>], filter: &str) -> Option<usize> {»
  - 276-278 «overrides»
- 281-302 @285 «fn mutants_verify_window_override_stands_before_the_harness_package_override() {»
  - 291-294 «assert!(»
  - 296-297 «let swapped =»
