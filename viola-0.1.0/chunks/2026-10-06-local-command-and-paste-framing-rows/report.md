# Report — 2026-10-06-local-command-and-paste-framing-rows

**Chunk:** Local-command and paste-framing rows — as landed: three ledger rows by typed probe (`long-paste-wrapper`,
`tag-escaping`, `local-command-clear`), the compiled local-command list, the unwrap of the CLI's paste frame, Run B's
wait for the input box. `send`'s `unconfirmable`, the harness-prefix row and the identity-floor row did not land
here (the P4 split, inputs#I2).
**Date:** 2026-10-06
**Commits:** `05b5f2b chore(2026-10-06-local-command-and-paste-framing-rows): operator pre-CI commit, for the run
this chunk's verdict reads` (basis: `git log --format='%h %s' 2fbc954..HEAD`, one commit; no fix commit)

## Changes (structured — detectors read this)
- **Files:** (basis: `git diff --name-status 2fbc954`, run dirs left out)
  - product: `crates/viola-agent-claude/src/hook.rs`, `crates/viola-agent-claude/src/ledger.rs`,
    `src/cmd/verify.rs`, `src/cmd/verify/typed.rs`, `src/run/version_gate.rs` (one unit test's pinned row set);
  - test-only binary and harness: `src/bin/viola-fake-agent.rs`, `crates/viola-e2e/src/harness/boot.rs`,
    `crates/viola-e2e/src/harness/run.rs`;
  - tests: `tests/cli_verify.rs`, `tests/cli_send.rs`, `tests/cli_fake_agent.rs`, `tests/hook_events.rs`,
    `tests/contract_ledger_probes.rs`, `tests/contract_fake_agent_drift.rs`, `tests/contract_fixture_hygiene.rs`,
    `tests/support/verify.rs`, `tests/support/home.rs`,
    `crates/viola-agent-claude/proptest-regressions/hook.txt` (one recorded seed);
  - new fixtures, recorded live from the 2.1.287 binary: `fixtures/claude/2.1.287/UserPromptSubmit.paste-1.json`,
    `UserPromptSubmit.paste-2.json`, `SessionEnd.clear-1.json`, `SessionStart.clear-1.json`;
  - the chunk folder (plan, research, scope, scope-record, 7 inputs, 15 evidence files). Untouched:
    `schemas/claude-fixture.v1.json`, `.config/nextest.toml`, `crates/viola-agent-claude/src/screen.rs`,
    `src/run/send.rs`, `src/cmd/send.rs`, `src/human.rs`.
- **Symbols / APIs:**
  - `viola_agent_claude::ledger`: `LedgerRow` gains `LongPasteWrapper`, `TagEscaping`, `LocalCommandClear`
    (ids `long-paste-wrapper`, `tag-escaping`, `local-command-clear`), `LedgerRow::ALL` is `[Self; 17]`; new
    `PROBE_LONG_PASTE` (1 500 bytes, one line), `PROBE_TAG_PASTE` (200 bytes), `PostCondition::NewSession`,
    `LOCAL_COMMANDS` (`/clear` → `NewSession`, `/remote-control` → none), `PROBE_LOCAL_COMMAND` (the list's entry
    with a post-condition), `FRAMING_TURNS`, `framing_stem`, `framing_variants`; `Probes` gains `trusted` (Run B's
    captures in claim order); `check` gains three arms over them.
  - `viola_agent_claude::hook`: `unwrap_pastes` removes, with a matched same-id pair, the two newlines directly
    before its open tag and the one newline directly after its close tag (a third before, a second after and a
    lone one before stay; an unmatched open tag removes nothing); `prompt_text` is `pub(crate)` (the ledger's two
    paste arms call it). `prompt-submitted`'s `text` for a wrapped paste is now the pasted text; its one reader
    outside the crate is still `SendSlot::claim` (sole caller of the result, research graph query, 6 rows).
  - `viola verify` Run B (`src/cmd/verify/typed.rs`): after its first turn it pastes three more compiled texts
    (the long text, the tag-like text, `PROBE_LOCAL_COMMAND`), each only into rows that hold the input-box literal
    and no modal literal. After the long turn and after the tag-like turn the rows come from a new wait
    (`box_wait`, `Run::box_wait`): it ends on a quiet screen (`QUIET_PERIOD`) holding a compiled literal, on a
    poisoned screen, or at `PROBE_DEADLINE` (120 s) from that turn's Stop; a quiet screen with no literal keeps
    waiting past `GATE_MAX_WAIT`. The settle after the local command and every other `Run::settle` call site are
    unchanged.
  - `--record` (`src/cmd/verify.rs`) also writes `UserPromptSubmit.paste-1.json`, `UserPromptSubmit.paste-2.json`,
    `SessionEnd.clear-1.json`, `SessionStart.clear-1.json` through the same scrub and whole-recording refusal.
  - fake agent (test-only binary, feature `fake-agent`): two new argv options, `--framing` (a compiled paste
    text fires its recorded `paste-1` / `paste-2` UserPromptSubmit variant with its bytes unchanged; the probed
    local command fires the recorded `clear-1` SessionEnd then SessionStart and no UserPromptSubmit) and
    `--paste-hint-ms <ms>` (capped at 8 000 ms by a constant; with `--framing`, after the long text's replay and
    its Stop the screen is cleared and `Screen.turn` is drawn only after the hold). No environment variable, no
    `config.json` key, no channel method, no event kind, no socket, no port.
- **Crates / modules:** none added or removed; changed: `viola-agent-claude` (`hook`, `ledger`), the root bin
  (`cmd::verify`, `cmd::verify::typed`, the fake agent bin), `viola-e2e` (`harness::boot`, `harness::run`).
- **Dependencies:** none added, none bumped (`Cargo.toml` and `Cargo.lock` not in the diff).
- **Schema / config:** no schema file changed. The stamp envelope is unchanged in shape; a version entry now
  holds seventeen row ids, and a stamp holding exactly the fourteen older ids, all `pass`, reads unverified
  (`ledger::verified`; unit case in `ledger.rs`, and `stamps_verdict_needs_the_dialog_rows` in
  `src/run/version_gate.rs` moved to seventeen). New fixture variant stems under `fixtures/claude/<version>/`:
  `paste-<n>` (UserPromptSubmit) and `clear-<n>` (SessionEnd, SessionStart); the recorded payloads passed the scrub
  (home → `~`, user name → `<user>`) with no refusal.
- **Spec-master edits:** none.
- **Counts / qualifiers moved:**
  - ledger rows, the `[NN/MM]` counter and `stamped … N pass`: 14 → 17 (stated in architecture
    [CLI Version Compatibility] `:91`, `:412`, `:459`; security-plan `:608`; test-plan `:653`, `:663`;
    design-system `:801`; layout-templates `:471-487`; key files `architecture/crate-dependency-direction.md:6`,
    `architecture/ci-cd-approach.md:20`, `test-plan/5-command-implementation.md:8`, `:53` — basis: the sweep
    `fourteen|/14|14 pass` over the seven masters and `.andromeda/registries/`, 22 hits, listed under Expected
    amendments);
  - Run B's pastes: 1 → 4 (the probe prompt, then three compiled pastes); a verify run is still five `claude`
    starts and still writes six spawn pairs;
  - the fake agent's argv options that serve verify's interactive runs: five → seven (`--framing`,
    `--paste-hint-ms` beside `--trusted-root`, `--screens`, `--turn-stop`, `--dialogs`, `--stop-receipt-hold-ms`
    as the masters count them — architecture `:355`, test-plan `:1076` say "five argv options");
  - recorded sets: stamped `{2.1.287, 2.1.288}` at fourteen rows → stamped `{2.1.287}` at seventeen; drift-only
    `{2.1.283}` → `{2.1.283, 2.1.288}` (`tests/contract_ledger_probes.rs:36`, `:39`); the 2.1.287 set is the spine,
    3 screens, 12 dialog variants, 4 relayed dialog fixtures and now 4 framing variants; 2.1.288 holds no framing
    variant;
  - `LEDGER_ROWS` in `crates/viola-e2e/src/harness/run.rs`: `[&str; 14]` → `[&str; 17]`; boot step 4 passes
    `--framing` beside `--dialogs`;
  - the `verify_window_` test class: one case → two (`verify_window_paste_hint_past_the_gate_maximum_still_stamps`);
  - verify's 300 ms quiet waits across its four interactive runs: seven → ten (Run B gains the two waits after
    its added paste turns and the settle after the local command; basis: the `settle` / `box_wait` call sites of
    `src/cmd/verify/typed.rs` — Run A 1, Run B 5, Run C 2, Run D 2). Stated as "seven 300 ms settles" in the key
    file `test-plan/bootstrap-phases-derive-for-route-setup-project.md:10` and in two source comments this chunk
    did not move (`.config/nextest.toml:19`, `tests/support/verify.rs:390`; the wrap touches no source);
  - Run B's residual under `~/.claude/projects/`: one transcript → two per real-CLI verify (the session and the
    one `/clear` opens).
- **Dev-tool versions:** the `claude` CLI on the dev host's PATH (the CLI the typed probes default to; mise
  `latest`) read CHANGED: 2.1.288 → 2.1.289, installed 2026-10-06 (research M1, read ~19:20Z); 2.1.287 and 2.1.288
  stay installed. Every live session of this chunk named the 2.1.287 binary by path; nothing ran on 2.1.289,
  which has no recorded set and no stamp. Not this line's subject: any crate.
- **Harness / gate surface:** harness `boot` step 4 stamps seventeen rows through the fake agent with
  `--framing`; `run --local-live`'s row literals are seventeen; no new harness command, no CI step, no nextest
  override.
- **Cross-project / external claims:**
  - CI: run ci#37534758441, measured sha `05b5f2b5371b`, conclusion success, 15/15 checks, wall 481 s
    (`evidence/operator-pass.md`). This wrap's commit adds to that tree: report, spec bodies, route, state.
  - The 2.1.287 CLI's behaviour, measured live four times (step 0, first round, rehearsal, second round;
    `evidence/step0-shapes.md`, `record-round-red.md`, `rehearsal-shapes.md`, `record-round-green.md`): a
    1 500-byte one-line paste arrives as two newlines, `<pasted_content id="X">` + newline, the text, newline +
    `</pasted_content id="X">`, one newline (ids `7ccf`, `31a3`, `dead`, `7602`); a 200-byte paste is not wrapped;
    typed `pasted_content` tags arrive with `<\`, a typed `<task-notification>` mid-text arrives as typed; a
    pasted `/clear` fires SessionEnd (`reason` `clear`) then SessionStart (`source` `clear`, a new `session_id`,
    no `model` key) and no UserPromptSubmit; after a long paste the footer reads `paste again to expand` and holds
    no `for agents` literal for 8.000 s from the paste (the rehearsal's raw bytes), which was 6.5 s after that
    turn's Stop. Read statically from the binary's script, not measured: the wrap threshold (over 800 characters
    or more than 2 lines), the feature flag behind it, the no-added-newline case for a text ending in a newline.
  - The dev host is shared: another project's cargo build wrote 10.6 GB and 5.2 GB to the same volume inside the
    two red windows of the first full gate block (`evidence/block-reds-host-contention.md`). The operator's
    disposition, relayed by the overseer at this wrap: environment, relayed to overseer1 (its F167); no bound or
    test moves.
  - inputs (from `inputs.py verify`): I1 · message · n/a; I2 · message · n/a; I3 · message · n/a; I4 · message ·
    n/a; I5 · message · n/a; I6 · message · n/a; I7 · message · n/a — `7 entries · drifted 0 · vanished 0 ·
    broken 0 · uncited 0 · unparsed 0`.
- **Reverted / negative API facts:** none shipped-then-removed. Step 6's control put `unwrap_pastes` back to
  its HEAD body in the working tree for one run and restored it; nothing of it is in the diff.
- **Insufficient fixes (written, kept, not the remedy):** none. (The wait is the remedy for verify's Run B. It
  is not written for, and does not change, the product readiness gate — see the first disproved-claims entry's
  neighbour below.)
- **Spec claims disproved by measurement:**
  1. architecture [CLI Version Compatibility], the long-paste wrapper bullet (`architecture.md:81`): "unwraps only
     that exact same-id pair and keeps the ends byte for byte". Measured: the CLI frames its pair with two
     newlines before and one after, so the kept ends made the normalised text differ from the sent text and a
     wrapped `send` could not be claimed (`evidence/send-wrapped-control.md`: exit 13 with the old body). The
     unwrap now removes that frame and nothing wider.
  2. architecture, the Tag escaping bullet (`architecture.md:89`): "the CLI inserts a backslash after `<` in
     tag-like text the user typed". Measured on 2.1.287, mid-text: only the typed `pasted_content` tags were
     escaped; a typed `<task-notification>` arrived as typed. The start-of-prompt position, the one
     `prompt_origin` reads, is unmeasured (operator disposition 4: it rides the harness-prefix row to "First live
     test and self-drive").
  3. test-plan §6 Scenario Path 2, step 3 (`test-plan.md:738`): "Send a text longer than the fixture's paste-wrap
     threshold, so the fake agent emits the `<pasted_content id=…>` form". The fake agent has no threshold and
     emits no wrapper of its own: with `--framing` it replays the recorded `paste-1` prompt for the one compiled
     long text. The wrapped half of that step landed as `send_long_text_wrapped_by_the_cli_is_confirmed`; the
     literal-tag half is still unbuilt.
  4. architecture [Delivery Confirmation] (`architecture.md:49`): "No local-command row is compiled yet". The
     list and the `/clear` row are compiled; `send` does not consume them yet (it still ends a local command
     `not-delivered` / `no-prompt-submitted`; `send_window_local_command_is_not_presumed_delivered` is untouched).
  - Not a disproved claim, a reading with no end-to-end measurement (operator disposition 1: a CARRY on the
    minted "Local-command send outcomes" entry): the product's readiness gate reads the same `for agents` literal
    with a 5 s maximum (architecture [Screen Model]), and for 8.0 s after a long paste this CLI's footer does not
    hold it. No `send` was run in that window.
- **Expected amendments (from plan):** (site searches: the sweeps named per line, over the seven masters and
  every file under `.andromeda/registries/`)
  1. architecture [CLI Version Compatibility], verify paragraph — Run B waits for the input-box literal after each
     added turn, up to the probe deadline; the measured footer hint. **carried**: Symbols / APIs (Run B) and
     Cross-project (the 8.0 s hint). Sites: `Run B` 5 hits in architecture (`:91`), `paste hint` 0 hits anywhere.
  2. security-plan §Threat Model Summary, Child process spawning — Run B's added pastes go only into a settled
     input box, waited for up to the probe deadline. **carried**: Symbols / APIs. Sites: `Run B` 3 hits in
     security-plan (`:126`, `:275`, `:284`), `four interactive` `:126`, `:244`.
  3. test-plan §7 Fake agent — `--paste-hint-ms`, capped; §5 Cross-module patterns → CLI — the `verify_window_`
     class holds a second case. **carried**: Symbols / APIs, Counts. Sites: `argv option` test-plan `:1076`;
     `verify_window_` 0 hits in test-plan's body, 3 in its key file
     `test-plan/bootstrap-phases-derive-for-route-setup-project.md:10-11`.
  4. architecture §Occupied Resources → Repository — the fake agent's argv options count `--paste-hint-ms`
     beside `--framing`. **carried**: Counts (five → seven). Sites: `argv option` architecture `:355`.
  5. architecture [CLI Version Compatibility] — seventeen rows and their list; the three rows as landed and
     measured on 2.1.287; Run B at four compiled pastes. **carried**: Symbols / APIs, Counts. Sites: `fourteen`
     architecture `:91` ×3, `:412`, `:459`; key files `architecture/crate-dependency-direction.md:6`,
     `architecture/ci-cd-approach.md:20`.
  6. architecture, the long-paste wrapper bullet and the Tag escaping bullet as measured. **carried**: Spec claims
     disproved 1 and 2. Sites: `long-paste` 5 hits (`:81`, `:89`, `:91` ×2, `:292`), `Tag escap` 4 hits (`:70`,
     `:89`, `:91`, `:292`), `byte for byte` architecture `:81`. The wrap judges it on the reading that it is not a
     boundary widening (inputs#I5).
  7. architecture [Delivery Confirmation] — the list and the `/clear` row are compiled, `send` consumes them at
     "Local-command send outcomes"; the matching sentence gains the frame's removal. **carried**: Spec claims
     disproved 4 and 1. Sites: `local-command` architecture `:49` ×2, `:292` (the `prompt-submitted` contract).
  8. architecture §Occupied Resources → Repository — `paste-<n>` / `clear-<n>` variants, `--framing`, 2.1.287
     stamped with 2.1.283 and 2.1.288 drift-only. **carried**: Schema / config, Counts. Sites: `2.1.288`
     architecture `:412` ×2, `drift-only` `:412`.
  9. security-plan §Threat Model Summary and §Data Protection — Run B's four pastes, the transcript residual one
     higher per verify; §Security Anti-Patterns → Universal — `/14` → `/17`. **carried**: Counts. Sites: `/14` and
     `fourteen` security-plan `:608`; `Run B` `:126`, `:275`.
  10. test-plan §3 `boot` step 4 and `run --local-live`, §5 CLI, §7 Fake agent, §6 Path 2 step 3, §4 unit —
      seventeen rows, the two lists, `--framing`, the paste-wrap line as built, the framing cases. **carried**:
      Counts, Harness, Spec claims disproved 3. Sites: `fourteen|/14|14 pass` test-plan `:653` ×3, `:663` ×2;
      `--dialogs` `:438`, `:653`, `:663`, `:1076`; `2.1.288` `:438`, `:663`, `:1084` ×3; `paste-wrap` `:738`;
      key file `test-plan/5-command-implementation.md:8` ×3, `:53` ×3.
  11. obs-plan §4 Edge flows → `verify` — `MM` at 17, six spawn pairs unchanged. **carried**: Counts. Sites:
      `MM` obs-plan `:735`; `--dialogs` `:737`; `six spawn` `:866`.
  12. design-system §Surface: cli → Component Patterns → 5 — `[NN/17]`, the three rows named with their words.
      **carried**: Counts, Symbols (the words: `a long paste unwraps to the text as pasted` · `tag-like text
      un-escapes to the text as pasted` · `/clear starts a new session and submits no prompt`). Sites:
      design-system `:801` ×3.
  13. layout-templates §Output structure — `viola verify` — the wireframe counters, the ledger-order list and
      the examples at 17. **carried**: Counts. Sites: `/14]` layout-templates `:471`, `:473`, `:474`, `:476`, `:477`,
      `:479` (six counters), the two summary lines `:480` (`14 pass`) and `:484` (`12 pass  2 fail`, the same count
      without its token), the ledger-order list and `MM is 14` at `:487` — 9 sites on 9 lines. The rows print in
      the order `[15/17] long-paste-wrapper`, `[16/17] tag-escaping`, `[17/17] local-command-clear`
      (`evidence/record-round-green.md`).
  14. Route changes (not a spec master; P5's): mint "Local-command send outcomes" after this entry; re-home the
      harness-prefix and identity-floor rows to "First live test and self-drive" with research M6 and M7; move
      that entry's `--local-live` count from fourteen to seventeen; the start-of-prompt `<task-notification>`
      measurement travels with the harness-prefix row; the readiness-gate reading; the local-command paste
      guard's missing control; the three unmeasured paste shapes of research M10. **carried to P5** with the
      operator's four dispositions (Decisions below).
- **Coverage of new surfaces**
  - `unwrap_pastes` frame removal (normalisation of the hook's `prompt`) → validation n/a (no new input class or
    crossing; the hook reader's `take(MAX_FRAME)` is unchanged) · instrumentation n/a · PII n/a (the text reaches
    `events.ndjson` as before, no log line) · tests unit (8 labelled cases, the reworked property) + integ
    (`hook_events`, `cli_send`) · a11y n/a · tokens n/a
  - Run B's three added pastes and its wait → validation n/a (four compiled literals, pasted only into a settled
    input box with no modal; no key into a dialog) · instrumentation log✓ (six spawn pairs, unchanged; G4 and the
    secret scan green in CI) · PII n/a (no row text, prompt or path in any line) · tests unit (`box_wait_` ×3,
    the three arm cases) + integ (`cli_verify`) · a11y n/a · tokens n/a
  - three new `viola verify` step lines on stdout → validation n/a · instrumentation n/a · PII n/a · tests integ
    (`STEPS_PASS` literals) · a11y ✓ (static ASCII, one line per row, no SGR, no redraw: P6) · tokens n/a
  - four recorded fixture variants → validation ✓ (the scrub and the `unclean` refusal; the hygiene walk) ·
    instrumentation n/a · PII redacted✓ · tests integ (drift contract byte for byte, hygiene) · a11y n/a ·
    tokens n/a
  - fake agent `--framing` / `--paste-hint-ms` (test-only) → validation n/a · instrumentation n/a · PII n/a ·
    tests unit (`Opts::parse`, cap) + integ (`cli_fake_agent`, `cli_verify`) · a11y n/a · tokens n/a

## Deviations from intent
- **The scope shrank at P4** (inputs#I2, the founder live and the overseer founder-delegated): `send`'s
  `unconfirmable` and `/clear`'s confirmation go to a new entry, "Local-command send outcomes"; the harness-prefix
  row and the identity-floor row go to "First live test and self-drive". The master record's description still
  names all of them; the flip rewrites it.
- **Two STOPs and two plan revisions inside the chunk.** Step 0's STOP 3 (the frame's newlines) folded the unwrap
  fix here (inputs#I4). The first record round read `15 pass  2 fail` and was not retried; the founder's answer
  (inputs#I6) added the wait, one rehearsal and a second round, and raised the live cap from 8 to 12. All 12 were
  spent, each a ledger row written before its session (`evidence/live-sessions.md`).
- **As built, beyond the plan's letter:** one test beyond the plan in the first run
  (`verify_pastes_nothing_more_once_the_turn_screen_shows_a_modal`, with its remove-the-guard control); the
  dialog-replay contract runs a set that holds no framing variant without `--framing` and expects exit 1 for it;
  the seventeenth step line in `src/cmd/verify.rs`'s unit test is asserted by counter, id and verdict, not its
  words (a gate entry greps that file for a local-command literal).
- **This run:** step 13 names "three named unit entries" and "four guard probes" where the block holds four and
  five — all thirteen fixture-free entries were run before the round; the red of step 12 was run a second time
  with the failing home kept, to read what the trusted run typed; before the round the three arms and the record
  scrub were run over the rehearsal's own captures by a scratch helper (no session); the fake agent's edits went
  in as one scripted write with unique-anchor asserts; the hygiene entry was previewed read-only before the
  operator pass fired it.
- **Scope record** (`gate.py scope`: `clean — changed 22 · listed 21 · recorded 1`):
  - companion: `src/run/version_gate.rs` · serves `crates/viola-agent-claude/src/ledger.rs` · self.

## Decisions & corrections
- The operator's dispositions at this wrap (relayed by the overseer, verbatim in the wrap arguments): (1) the
  readiness-gate reading is a CARRY on the minted "Local-command send outcomes" entry, its first consumer, marked
  unmeasured end to end; (2) the missing control for the guard before the local-command paste is a route-resolve
  card, recommended-first; (3) the host-contention reds are environment, relayed to overseer1 (its F167): no bound
  or test moves; (4) the start-of-prompt task-notification reading rides the harness-prefix row to `:92`.
- The operator's word at the operator pass: a timing red on
  `fake_agent_dialog_replay_matches_every_recorded_dialog_set` is read under the designed-floor rule, never by
  raising a bound alone. No such red occurred (9.98 s at most, on the ubuntu leg).
- A live CLI's settled screen after a long paste is not the input box for 8 s: a probe that pastes several texts
  waits on the literal, not on a timer.
- A step-0 record that says "settled" must carry how long each settle took: step 0's own log held the 5.8 s
  reading that explained the first red round, and the record had left it out.
- A dry run of the row arms and the record scrub over a rehearsal's real captures costs no session and was done
  before the one non-retriable round.
- A nextest kill of a verify-driven test leaves verify's probe dirs at the repository root (21 found and
  removed): a runner kill is not one of verify's exit paths.
- A local red where every process-spawning test stalls by one common amount, while tests that spawn nothing pass
  in milliseconds, is a host stall: read other sessions' build dirs by modification time per window before
  touching the tree; and a targeted re-run overwrites the red entries' logs, so read them first.
- Sweep hazards: on this host `grep -E` is ugrep and refuses a bounded-repeat context window
  (`.{0,70}(…).{0,90}` — "exceeds complexity limits"); a site finder went through a short python script. `pgrep -x`
  matches nothing for a name over 15 characters (`viola-fake-agent`): the census read `/proc/<pid>/exe` instead.
  The pattern `/14` also hits `exit 13 (or 10/11/12/14)` in layout-templates `:351`, which is an exit-code list.

## Outcome
- **Acceptance criteria**, each against the diff:
  - a long paste as the CLI sends it normalises to the pasted text — met (8 cases green; red first on the
    untouched unwrap, `evidence/unwrap-red-green.md`; live in three sessions);
  - the unwrap removes only the measured framing; no trim — met (the three "stays" cases, the unmatched-open case,
    `typed_pair`, the property);
  - `send_long_text_wrapped_by_the_cli_is_confirmed` passes, red with the HEAD body — met
    (`evidence/send-wrapped-control.md`: exit 13, then green);
  - `LedgerRow::ALL` holds 17 rows, each new row with an arm and a Run B probe — met;
  - each new arm reads its capture (three named unit cases) — met (`"passed":3`);
  - the local-command list and both probe texts are compiled literals in `viola-agent-claude` — met (the grep
    entry reads exit 1, no output);
  - the live record entry prints `stamped 2.1.287  17 pass  0 fail`, the variants pass the scrub — met
    (`evidence/round-211429Z.txt`, `record-round-green.md`);
  - a fourteen-row stamp reads unverified; verify stays the only writer of `ledger/stamps.json` — met;
  - Run B pastes only its four compiled texts into a settled input box; exactly 12 live sessions, each a ledger
    row written first — met (the ledger entry reads 12 rows);
  - after each added turn the next paste waits for the literal, at most the probe deadline — met (the new case
    red `15 pass  2 fail` before the change and green after, `evidence/run-b-wait-red-green.md`; `box_wait_`
    `"passed":3`);
  - the product's readiness gate is unchanged — met (`screen.rs` not in the diff; `settled_` `"passed":3`);
  - the rehearsal's record holds the timings and the five STOP conditions — met (`evidence/rehearsal-shapes.md`);
  - no environment read, `config.json` key or test seam is added — met (the guard entry reads no output; the two
    new options are argv of the test binary);
  - `contract_ledger_probes` with `STAMPED = ["2.1.287"]` — met; the two new named cases — met (`"passed":2`);
    the drift contract and hygiene over the four variants, no committed fixture modified — met;
  - `agent-run.sh run` and `pre-push` green — met (second full block; pre-push again before the commit);
  - the `[NN/17]` lines with the three rows' words, `stamped` last — met; six spawn pairs, G4 and the secret scan
    — met (CI);
  - the send strand otherwise unchanged — met (the guard entry green);
  - CI green on the pushed HEAD on all three OSes — met: ci#37534758441 at `05b5f2b`.
- **Gates** (the block's 28 entries by `run`; verdicts from the second full run of the block unless said):
  - `cargo fmt --all --check` — green · `cargo clippy --workspace --all-targets --features fake-agent -- -D
    warnings` — green · `bash scripts/agent-run.sh run --unit` — green;
  - the four named unit entries (`check_framing_rows…|check_paste_rows…|check_local_command_clear…`,
    `prompt_text_drops_the_cli_framing_around_a_pair`, `box_wait_`, `settled_`) — green (3 · 8 · 3 · 3);
  - `… run --integration --filter 'test(/verify_window_paste_hint_past_the_gate_maximum_still_stamps/)'` — green;
  - `"$HOME/.local/share/mise/installs/claude/2.1.287/claude" --version` (`leg = 'round'`) and the record entry
    (`leg = 'live'`) — fired once as the second round: `round: COMPLETE · legs fired 1/1`, both lines `green`
    (`evidence/round-211429Z.txt`). Their first firing, step 10, read `round: STOPPED at 7 (red …)`
    (`evidence/round-203444Z.txt`), answered by inputs#I6. Never re-fired here;
  - the preservation guard, the send-strand guard, the `screen.rs` guard, the local-command grep (exit 1, no
    output), the no-ignore / no-sleep / no-env guard — green;
  - `… --filter 'binary(cli_verify) | binary(contract_ledger_probes) | binary(contract_fake_agent_drift) |
    binary(contract_fixture_hygiene) | binary(cli_fake_agent)'` — green (117 passed). In the first full run it
    read red (22 timeouts, 2 boot failures), and so did `… --filter
    'test(/send_long_text_wrapped_by_the_cli_is_confirmed/)'` (wrapper not ready): **not this tree** — every
    process-spawning test stalled about 18 s while another project's build wrote 10.6 GB and 5.2 GB to the same
    volume in exactly those windows; both read green three times afterwards with no such write
    (`evidence/block-reds-host-contention.md`). Final verdict green; the operator dispositioned the cause as
    environment (disposition 3);
  - the two named integration cases — green (2); `bash scripts/agent-run.sh run` — green (1312 + 326 passed);
  - smoke: `cleanup`, `boot --session p-rows-smoke --instance builder`, `status`, `cleanup` — green (boot stamps
    17 rows; the last cleanup `processes_gone` and `endpoint_gone` true);
  - the session-ledger count entry (`test "$n" -eq 12`) — green; `bash scripts/agent-run.sh pre-push` — green
    (coverage 1638/1638, playwright 1/1, no breaches);
  - `leg = 'operator'`: the hygiene read — `hygiene: clean`, exit 0; the push — exit 0,
    `2fbc954..05b5f2b  HEAD -> build/viola-0.1.0`; `ci.py conclusion --sha HEAD --wait 1800` — exit 0,
    `05b5f2b5371b verdict: green · checks 15/15 · wall 481 s · runs ci#37534758441 completed/success`
    (`evidence/operator-pass.md`).
- **Watches:** none folded on this entry. Opened at this wrap's light gate (written here after the fan-out, P7.1):
  `pre-push`'s coverage merge failed once on three truncated raw profiles of 4 709, every one of the 1 638 tests
  passing, while another session's build wrote 10.8 GB to the same volume
  (`.andromeda/runs/2026-10-06T21-43-53-wrap/light-gate-red.md`); pinned as a `WATCH:` on "Local-command send
  outcomes", origin this chunk, 0 green runs so far.
- **Outcome basis:** the operator pass ran (`05b5f2b`, one commit, no fix commit); the verdicts rest on its final
  state and on ci#37534758441. This session held the third implement run's conversation (steps 11-15) and the
  operator pass. The first two implement runs (step 0's STOP; steps 1-8 and the red round) were other sessions:
  what they did is taken from the plan's two revisions and "As built" notes, the evidence files, and this chunk's
  own implement records of those two runs in the friction log (11 records, read filtered to the chunk and the
  skill).
- **Process hygiene** (implement P4's census at 2026-10-06T21:30Z, re-measured there against the process list):
  the rehearsal's `claude` — terminated (Ctrl-C twice, exit 0); the record round's five `claude` starts —
  terminated (no survivor reported; no `claude` with a probe-dir cwd); the smoke session — terminated; the
  `viola verify` children of the killed tests — terminated, their 21 empty probe dirs removed; a scratch host
  sampler — ended on its own timer. No process of this repository's binaries was running. Left on disk:
  `.viola-verify-2095228/` (the operator desk, untouched) and `target/e2e-home/viola-test-WYNVH7` (a test home kept
  for the red reading; its removal was refused by the permission layer; git-ignored, the operator's to delete).
