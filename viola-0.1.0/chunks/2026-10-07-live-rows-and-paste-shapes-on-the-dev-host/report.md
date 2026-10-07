# Report — 2026-10-07-live-rows-and-paste-shapes-on-the-dev-host

**Chunk:** Live rows and paste shapes on the dev host — the harness-prefix and identity-floor rows, three paste shapes and the paste-hint window, measured on claude 2.1.287 by path
**Date:** 2026-10-07
**Commits:** `33d2084` chore(2026-10-07-live-rows-and-paste-shapes-on-the-dev-host): operator pre-CI commit, for the run this chunk's verdict reads (the one commit since the last wrap; basis `git log --format='%h %s' e7e5bf7..HEAD`)

A measuring chunk. Three live `claude` 2.1.287 sessions were run by path under one cap of 10 (3 used), each ledgered
before it was made. One product file changed. No ledger row landed, no row count moved, no fixture, no fake-agent
mode, and the readiness gate is where the base commit left it.

## Changes (structured — detectors read this)
- **Files:** `crates/viola-agent-claude/src/hook.rs` (99 insertions, 13 deletions; basis `git diff --stat e7e5bf7 --
  crates`). Beside it only the chunk folder (`evidence/` 12 files, `inputs/` I5 and I6), the run dirs and
  bookkeeping. `crates/viola-agent-claude/proptest-regressions/hook.txt` is unchanged (no failing seed).
- **Symbols / APIs:**
  - `unwrap_pastes` (private, `hook.rs`; sole caller `prompt_text`, whose callers are `data_of` and
    `ledger::pasted`, per research's graph) changed behaviour: after a pair's close tag and its one newline, a
    **second newline is removed too when text follows it that is not the next pair's own two-newline frame**. So
    `prompt-submitted.text` for a paste followed by typed text no longer keeps one newline between them.
  - New private const `NEXT_FRAME` (`"\n\n<pasted_content id=\""`) in `hook.rs`. No public symbol, IPC method,
    endpoint, event kind, socket, port, env var or CLI flag was added or changed.
  - `HARNESS_PREFIXES` and `IDENTITY_FLOOR` are unchanged. `paste_pair` is unchanged.
  - Doc comments brought current: `prompt_origin` (the typed `<task-notification>` at a prompt's start; the
    cross-session form as measured) and `unwrap_pastes` (the frame).
  - Tests (inline, `mod tests`): new `prompt_text_live_shape_normalises_as_measured` (5 cases) and
    `prompt_origin_live_shape_files_as_harness` (2 cases); `two_framed_pairs` now carries one id on both pairs;
    `prompt_text_prop_round_trips_a_wrapped_paste` gained a typed tail after the pair. `hook.rs` holds 51 labelled
    cases (44 at the base; basis `/usr/bin/grep -c 'case::'`).
- **Crates / modules:** none added or removed; `viola-agent-claude` changed (`hook.rs` only).
- **Dependencies:** none.
- **Schema / config:** none. No fixture under `fixtures/claude/` changed (the preservation guard reads so).
- **Spec-master edits:** none (this wrap's P2 applies them).
- **Counts / qualifiers moved:** the unit-case count moved by seven: the Linux coverage suite reads 1682 (1675 at
  the base), CI legs 1682 / 1678 / 1706. No master states those totals (`1675|1682`: 0 hits in architecture,
  test-plan, obs-plan, security-plan). The ledger row count stays **seventeen**; `LEDGER_ROWS` is unchanged.
  test-plan's "the eight labelled cases of `prompt_text_drops_the_cli_framing_around_a_pair`" still holds: that
  table has eight cases, one re-identified.
- **Dev-tool versions:** none — `claude` (the CLI under test) re-read at `2.1.287 (Claude Code)` by path on the dev
  host, three times before the three starts; PATH `claude` (the CLI this builder runs) stays 2.1.289 and unstamped.
- **Harness / gate surface:** none. `scripts/`, `crates/viola-e2e`, `.github`, `.config` are unchanged.
- **Cross-project / external claims:**
  - CI: ci#37609247992 on `33d20843c8a6`, `verdict: green · checks 15/15 · wall 449 s` (read by `ci.py conclusion`,
    recorded in `evidence/operator-pass.md`). The sha is the record: this wrap's commit adds to that tree.
  - The CLI's behaviour (the subject of every measurement below) lives outside this repository: the 2.1.287
    binary under the user's mise install dir, by path.
  - This session's peer listing (the CLI's own peer messaging): 9 peers before start 1, 10 at the hold.
  - The driver's own log of this builder session (another tree's bridge, outside this repository), read by counts
    only (`evidence/harness-prompts-driver-log.md`).
  - Residual outside the repository: the CLI's own transcripts of the three live sessions under the user's Claude
    projects dir (the accepted class).
  - Inputs (`inputs.py verify`, this wrap's P1: `6 entries — unchanged 1 · drifted 0 · vanished 0 · broken 0 ·
    altered 0 · unreachable 0 · n/a 5 · uncited 2 · unparsed 0`):
    - `I1 · message: the operator, in the /andromeda-phase invocation · copy message · n/a`
    - `I2 · ../additional/viola-overseer/d94-route-adaptation.md · copy no-repo · unchanged`
    - `I3 · message: the answers to the P4 founder card · copy message · n/a`
    - `I4 · message: the overseer, with the approval word at the P5 review · copy message · n/a`
    - `I5 · message: the overseer, answering this run's question on the three scratch homes at implement · copy
      message · n/a` — uncited by scope, research and plan (it arrived at implement); cited here (inputs#I5) and in
      `evidence/hint-window.md`.
    - `I6 · message: the founder, live, 2026-10-07T10:29Z, relayed by the overseer · copy message · n/a` — uncited
      by scope, research and plan for the same reason; cited here (inputs#I6) and in `evidence/hint-card-answer.md`.
- **Reverted / negative API facts:** none shipped. Two things were deliberately not built: no change to the claim's
  exact match or the send side for a text ending in a newline (STOP 5), and nothing from the founder's hint answer.
- **Insufficient fixes (written, kept, not the remedy):** the `unwrap_pastes` fix resolves the paste-then-typed
  shape only. It does not resolve the loss of a pasted text's last newline, which no change inside `hook.rs` can:
  the CLI drops it before the hook is called. The remainder is owned by the route (the STOP 5 card of this wrap).
- **Spec claims disproved by measurement** (all on `claude` 2.1.287, Linux dev host; evidence files under
  `evidence/`):
  1. **The pair's frame is "two newlines before the open tag and one after the close".** Stated in architecture
     [Delivery Confirmation] (`architecture.md:49`), [CLI Version Compatibility] long-paste wrapper (`:81`),
     §Standard Contracts `prompt-submitted` (`:292`), and test-plan §4 (`test-plan.md:564`, "a second after … stay").
     Measured: when typed text follows the pair the CLI writes **two** newlines after the close tag; at the prompt's
     end one; between two adjacent pairs three in all (`scratch-session.md`, paste-then-typed and two-pastes). The
     code now follows the measurement.
  2. **"the id is 4 hex characters and differs per paste"** (`architecture.md:81`). Measured: every pair of one
     session carried one id (`eec9`: four prompts, five pairs, the two pairs of one prompt included). Across sessions
     the ids differ (`7ccf`, `31a3`, `dead` at the prior chunk).
  3. **"the cross-session message arrives escaped, `<\cross-session-message from="…" from-name="…">` … (relayed, not
     yet measured here)"** (`architecture.md:89`; the relayed basis at `:80`). Measured here: it arrived **unescaped**,
     `<cross-session-message from="<value>" from-name="<value>" from-mode="<value>">`, a newline, the text, a newline
     and `</cross-session-message>`; three attributes, `from-mode` unknown to the static read. Filed `harness` by the
     plain compiled prefix. The escaped form was not seen; it stays a compiled prefix (the founder's 2026-09-29
     ratification is untouched).
  4. **"whether a typed `<task-notification>` at the very start of a prompt arrives escaped (unmeasured …)"**
     (`architecture.md:80`, `:89`). Measured: it arrives as typed, so a human who types it first is filed `harness`
     and does not flip the wheel.
  5. **"Unmeasured: a wrapped paste beside typed text, two wrapped pastes in one prompt, a pasted text ending in a
     newline"** and the static reading "a pasted text already ending in a newline gets none added before the close
     tag" (`architecture.md:81`). All three are measured; the static reading holds live. An unwrapped (short) text
     loses its last newline too.
  6. **Every delivered send is confirmed** (architecture [Delivery Confirmation], `architecture.md:49`; "for a
     wrapped paste `text` is the pasted text, and delivery matching compares it with the sent text directly",
     `:292`). Measured false for a text whose last byte is a newline, on both hint homes: the text is delivered and
     runs a turn, `prompt-submitted.text` is the text less its last newline and is filed `human`, the wheel moves to
     the human (`cause` `human-input`), and the send ends exit 13 `not-delivered` / `no-prompt-submitted` after the
     10 s window (`hint-window.md` step 7; `live-shape-red-green.md`). **STOP 5: not fixed, nothing built.**
  7. **"No `send` was run in the real CLI's hint window … What `send` should do while the hint stands is the
     founder's decision and is open"** (`architecture.md:91`). Now run and decided (below).
  8. **The `input-not-ready` hint's advice, `viola wait <name>, then send again`** (design-system §Surface: cli,
     `design-system.md:764`; `src/human.rs`). Measured: a no-cursor `viola wait` issued in the window woke on nothing
     and ran to its 20 s deadline; `wait --after` the earlier cursor returns at once inside the window. The advice
     does not lead out of the window. The line is unchanged in this chunk.
  9. **"the 11 names measured on the Windows host"** as the identity floor's whole origin (`architecture.md:380`,
     `security-plan.md:452`, `test-plan.md:86`, `:559`, `obs-plan.md:725`). Not disproved; a Linux reading now
     stands beside it (below), and it differs: **STOP 6, a report.**
- **The measurements** (facts the amendments carry; none changed code beyond `hook.rs`):
  - **Paste shapes** (one scratch session, start 1): typed-then-paste normalises to the bytes sent (the typed
    bytes, two newlines, the pair, one newline); paste-then-typed did not on the base commit (fixed); two-pastes
    does (two pairs, three newlines between, one id); long-ending-newline and short-ending-newline do not and
    cannot (item 6).
  - **The peer message** (the founder's first answer, inputs#I3): exactly one new idle peer; one fixed synthetic
    message sent once; the scratch session's UserPromptSubmit fired 6.7 s into the hold; 0 messages came back.
  - **The names** (the founder's second answer, inputs#I3): the scratch hook's first invocation (SessionStart), in a
    child started with every inherited `CLAUDE*` name removed, held **12** names: `CLAUDECODE`,
    `CLAUDE_CODE_CHILD_SESSION`, `CLAUDE_CODE_ENTRYPOINT`, `CLAUDE_CODE_MESSAGING_SOCKET`,
    `CLAUDE_CODE_MESSAGING_TOKEN`, `CLAUDE_CODE_SESSION_ATTENDED`, `CLAUDE_CODE_SESSION_ID`, `CLAUDE_ENV_FILE`,
    `CLAUDE_PID`, `CLAUDE_PLUGIN_DATA`, `CLAUDE_PLUGIN_ROOT`, `CLAUDE_PROJECT_DIR`. **Outside the eleven: 4**
    (`CLAUDE_ENV_FILE`, `CLAUDE_PLUGIN_DATA`, `CLAUDE_PLUGIN_ROOT`, `CLAUDE_PROJECT_DIR`). Of the eleven, absent
    there: `CLAUDE_CODE_BRIDGE_SESSION_ID`, `CLAUDE_CODE_EXECPATH`, `CLAUDE_EFFORT`. architecture §Occupied Resources
    already registers `CLAUDE_PLUGIN_ROOT` and `CLAUDE_PLUGIN_DATA` as provided to plugin processes
    (`architecture.md:381`); `CLAUDE_ENV_FILE` and `CLAUDE_PROJECT_DIR` are in no master. The R8 prefix rule removes
    every inherited `CLAUDE*` name outside the persistent set, so the four are stripped today whenever they are
    inherited; the floor only decides what a persistent-set entry cannot keep.
  - Beside it, the one path already ruled: both hint runs' `process-start{subject:"claude-child"}` line lists the
    names PATH `claude` (2.1.289) hands this builder session's tool environment: 10 names, all on the eleven
    (`CLAUDE_CODE_BRIDGE_SESSION_ID` absent).
  - **The hint:** a timer of 8.0 s from the last long paste (eight new timings, 8.005 s to 8.023 s; the first
    chunk's 8.000 s); 3.764 s to 6.968 s of it after the turn's Stop; a second long paste restarts it, a short one
    does not. Typed bytes alone also take the input-box literal off the footer.
  - **Verified home** (a byte copy of the verify-written 2.1.287 stamp home; `cli_verified` `true`): a `send` 1 ms
    after the Stop → exit 13 `not-delivered` / `input-not-ready` after 0.626 s, nothing typed, no `parse-rejected`
    line before it; its human lines recorded as printed; a no-cursor `wait --timeout-ms 20000` → `timed_out` after
    20.017 s; `wait --after` the first cursor → the logged `turn-ended` in 4 ms; a `send` 20.653 s after the Stop →
    confirmed.
  - **Unstamped home** (`cli_verified` `false`; a reading of what the product already does, no new widening,
    inputs#I4): the same `send` was typed under the hint and the CLI submitted it as its own prompt, confirmed in
    0.642 s; again 2.159 s after the Stop, 0.649 s. Short texts only; a long or repeated text under the hint is
    unmeasured.
  - **The founder's answer to the hint card** (his own, live, 2026-10-07T10:29Z, relayed by the overseer;
    inputs#I6; `evidence/hint-card-answer.md`): options 1 and 2 together — on a verified CLI the gate waits for the
    input box on a quiet literal-less screen, and the bound rises to 8.5 s. He was told the price: one by-path
    re-verify (5 live starts), `send`'s longest block 18.5 s, the fake agent's 8 000 ms hold cap to raise, no human
    keystroke delayed. Nothing is built from it here.
  - **The driver's own log** (PATH `claude`, counts only): 140 UserPromptSubmit prompts, 75 start
    `<agent-message from=`, 29 `<task-notification>`, 0 either cross-session form.
  - **Probe dirs:** each of the three rounds made one and left none; the eleven dirs standing before are the same
    eleven. The kill-leftover finding (research M8) stands unbuilt: a leftover carries a pid and no start time, so a
    remover needs an owner record first.
- **Expected amendments (from plan):**
  - architecture [CLI Version Compatibility], harness prefixes and start-of-prompt — **carried**: disproved items 3
    and 4. Sites: `not yet measured` 7 hits in architecture.md (2 on this subject, `:80`, `:89`; 5 on local-command
    post-conditions and a Windows mouse report, no change), 0 in the other six masters and in
    `.andromeda/registries/`; `nmeasured` 4 hits in architecture.md (`:80`, `:81`, `:89` this subject; `:47` a
    Windows resize, no change), 1 in layout-templates.md (`:351`, an unmeasured local command, no change), 1 in
    test-plan.md (`:622`, the Windows resize, no change).
  - architecture [CLI Version Compatibility], the long-paste wrapper's three "Unmeasured" shapes — **carried**:
    disproved items 1, 2 and 5. Sites: `architecture.md:81`; the frame's wording also at `:49`, `:292` and
    `test-plan.md:564` (`after the close`: 3 hits architecture, 1 test-plan, 0 elsewhere).
  - architecture [CLI Version Compatibility], the hint paragraph and "owed to Live rows and paste shapes on the dev
    host" — **carried**: disproved items 7 and 8, the hint and home measurements, the founder's answer; the two rows
    stay owed with their readings and no entry minted (inputs#I3). Sites: `Live rows and paste shapes` 3 hits in
    architecture.md (`:80`, `:91` twice), 0 elsewhere; `paste again to expand` 1 hit (`:91`).
  - architecture §Occupied Resources → Environment variables, the floor's origin — **carried**: the names
    measurement. Sites: `architecture.md:380`, `:381`; the same origin sentence at `security-plan.md:452`,
    `test-plan.md:86`, `:559`, `obs-plan.md:725` (`measured on the Windows host|measured on a wrapped host`: 2, 1,
    2, 1 hits; one architecture hit, `:47`, is the PTY wrapper's, no change).
  - security-plan §Threat Model Summary, the two scratch-probe measurements as the founder's live answers of
    2026-10-07T09:43Z relayed by the overseer — **carried**: the peer-message and names measurements. No sentence
    for them exists yet (`scratch`: 6 hits in security-plan.md, all the mutation scratch, no change).
  - test-plan §3 → 5-command implementation, the `--local-live` bullet's row list — **carried**: this chunk changed
    neither value; the fact is that `LEDGER_ROWS` holds seventeen ids and the bullet names fourteen
    (`long-paste-wrapper`, `tag-escaping`, `local-command-clear` missing). Site: the key file
    `.andromeda/registries/contracts/test-plan/5-command-implementation.md:53` (`local-live`: 5 hits there, 4 in
    test-plan.md, 3 in architecture.md, 1 in obs-plan.md; only `:53` lists row ids).
  - test-plan §4 Unit Test Strategy, the unwrap's case list — **carried**: the Symbols bullet's tests and disproved
    item 1. Site: `test-plan.md:564` (`two_framed_pairs|text_then_framed_pair`: 0 hits in any master).
- **Coverage of new surfaces:**
  - `prompt-submitted.text` after a paste followed by typed text (the changed normalisation, no new surface) →
    validation n/a · instrumentation n/a · PII n/a (the text already travels as event content, never to a log) ·
    tests unit (the `live_shape` cases, the property, two remove-the-condition controls) · a11y n/a · tokens n/a
  - No new external-input surface, hot-path operation, log line or UI element.

## Deviations from intent
- **The two live homes were moved, not removed** (plan step 5). A plain `rm -r` of an own-made scratch home was
  denied by the permission layer; on the overseer's direction (inputs#I5) the two live homes and the fake-agent
  rehearsal home were moved by full name, before any gate, into `target/e2e-home.disk/` (inside the workspace,
  git-ignored, on the founder's desk list). Their deletion is the founder's at the desk.
- **One verb added to the hint sequence** (plan step 4): a human-mode `send` right after the refused one on the
  verified home, because the layouts acceptance asks for the live refusal's lines as printed while the step runs
  every verb with `--json`.
- **STOP 5 and the live rounds.** The STOP list's header ends the live work at a STOP; STOP 5 is evaluated at step 7,
  after every live round, and the falsifying shapes were known after start 1. The rounds went on in plan order: they
  do not depend on the unwrap, and step 4.7 is the hypothesis's own end-to-end test.
- **The long bodies of the `live_shape` cases** are rebuilt from a test-local generator (head, filler, tail) rather
  than five 1 500-byte literals; each case asserts the measured prompt's length, and a reading script checked each
  rebuilt string equal to its capture.
- **Unmeasured, named:** a long or a repeated text pasted under the hint.
- scope record: none — `gate.py scope` clean, 0 recorded (`changed 1 · listed 1`, base `e7e5bf75`).

## Decisions & corrections
- **The founder (live, 10:29Z, relayed):** the hint remedy is options 1 and 2 together, the bound 8.5 s (inputs#I6).
- **The overseer (at implement):** a removal the permission layer denied is not done through another tool; move the
  dirs into `target/e2e-home.disk/` and record where they stand (inputs#I5).
- **The overseer (with this wrap's invocation), four dispositions:** (1) mint the entry for the founder's hint
  answer right after this one and ahead of "First live test and self-drive", its live cap the one by-path re-verify
  of 5 starts he was told, more returns to him; (2) STOP 5, STOP 6 and the kill-leftover finding are route-resolve
  cards, recommended-first; `IDENTITY_FLOOR` does not move without the founder; (3) the two scratch measurements are
  ratified by his live answers of 09:43Z; (4) the two `v1-34` rows stay owed, named in the handoff.
- **Mechanics learned this session:**
  - an answer that arrives in the session is snapshotted with `inputs.py snap --message-file … --origin …`;
    `--source` refuses a file inside the repository and one under the temp dir;
  - a backgrounded gate block's output file ends with the harness's own `[exited with code N]` line, so a wait that
    reads the summary as the last line never ends early;
  - every product command a live round starts from this bridge-wrapped builder runs with `VIOLA_NAME`,
    `VIOLA_DIR`, `VIOLA_BIN` removed and `--home` given, and a `viola run` meant to record the session's own
    `CLAUDE*` names must keep them (research M1, M5);
  - the Bash guard refused a `cat` heredoc with a file target again; an apostrophe inside a single-quoted JSON
    argument ended the quote.
- **Sweep hazards:** `nmeasured` also reads "an unmeasured local command" (layout-templates, architecture) and the
  Windows resize sentences, none this subject; `not yet measured` is five times the local-command post-condition
  phrase; `newline` reads the ndjson reader and the fake agent's fixture newline (7 hits in test-plan, 2 this
  subject); `scratch` in security-plan is the mutation scratch only.

## Outcome
Acceptance criteria, each re-asserted against the diff and the records:
- (tests) every live start ledgered before it was made, 2.1.287 by path, three in all, the three sessions present —
  **met**: `jq … live-sessions.ndjson` green; the three `start` lines read 10:05:27Z, 10:13:58Z, 10:15:08Z against
  starts at 10:05:36Z, 10:14:03Z, 10:15:13Z.
- (security) no probe dir of this chunk stands; the others untouched — **met**: three `census` `after` lines,
  `own_left` 0; `probe-dir-census.md`.
- (arch) the six shape readings recorded with raw prompt and normalisation — **met**: the shape reader green.
- (tests) each measured shape is a `live_shape` case; a falsified unwrap fixed inside `hook.rs` with red and green in
  `evidence/`, or reported under STOP 5 — **met** by both arms: paste-then-typed fixed (red 10:18:17Z, green
  10:18:34Z), the two newline shapes reported under STOP 5 with their failing cases recorded
  (`live-shape-red-green.md`).
- (security) the peer message at most once, fixed text, the one new idle peer; names bare against a literal copy —
  **met**: the reader green.
- (obs) no `CLAUDE*` value, no assistant text, no host path in any committed file — **met**: hygiene `clean` (read
  92); the evidence holds names, codes, counts, times and synthetic prompts only; the live homes are git-ignored.
- (arch) the hint timed at least twice more; `send` and a no-cursor `wait` in the window on a verified home, a `send`
  on an unstamped one; the founder's answer recorded — **met**: eight timings; the hint reader green.
- (layouts) the card states per option whether the human lines and the hint stay; the live refusal's lines recorded
  as printed — **met**: `hint-card.md`, `hint-window.md` step 3b.
- (a11y) the card states per option whether a human keystroke is delayed or refused — **met**: `hint-card.md`.
- (arch) the gate, `GATE_MAX_WAIT`, `QUIET_PERIOD`, `send_hint`, the floor, the ledger rows, verify, the capture arm,
  fixtures and the fake agent unchanged — **met**: the preservation guard exits 0; the diff holds `hook.rs` only.
- (tests) `run` and `pre-push` exit 0 with `"ok":true` — **met**.
- (obs) G2 and G4 clean over the smoke home — **met**.
- (tests) CI green on the pre-CI commit, the run id named — **met**: ci#37609247992.

No capability is claimed (`matrix.py show`: claimed 0); `v1-34` stays unclaimed, its two rows owed.

Gates, by `run`, from implement's one full run (17 green, 0 red) and the operator pass:
- `cargo fmt --all --check` — green · exit 0
- `cargo clippy --workspace --all-targets --features fake-agent -- -D warnings` — green · exit 0
- `bash scripts/agent-run.sh run --unit` — green · exit 0, `"ok":true`
- `bash scripts/agent-run.sh run --unit --filter 'test(/live_shape/)'` — green · exit 0, `"ok":true` (7 cases)
- `bash scripts/agent-run.sh run` — green · exit 0, `"ok":true`
- `git diff --quiet e7e5bf75db8a -- src …` (the preservation guard) — green · exit 0
- the four `jq -e -s` readers (session ledger, six shapes, the two ratified measurements, the hint record) — green
- `bash scripts/agent-run.sh cleanup --session p-live-smoke` — green
- `bash scripts/agent-run.sh boot --session p-live-smoke --instance builder` — green · `"ok":true`
- `bash scripts/agent-run.sh status --session p-live-smoke` — green · `"state":"ready"`
- `bash scripts/g2-zero-panics.sh` — green
- `bash scripts/agent-run.sh schema-check` — green · `"ok":true`
- `bash scripts/agent-run.sh cleanup --session p-live-smoke` — green · `processes_gone` and `endpoint_gone` true
- `bash scripts/agent-run.sh pre-push` — green · `"ok":true`, `"stage":"linux-tests"`, coverage 1682/1682; re-run by
  hand before the push, green, the same counts
- `gate.py hygiene` (`leg = 'operator'`) — by hand: exit 0, `hygiene: clean`
- `git diff --quiet && git diff --cached --quiet && git push origin HEAD` (`leg = 'operator'`) — the operator pass:
  exit 0, `e7e5bf7..33d2084`
- `ci.py conclusion --sha HEAD --wait 1800` (`leg = 'operator'`) — by hand: exit 0, `verdict: green`, 15/15

Smoke: the plan's own boot → status → G2 → G4 → cleanup entries, green on the final tree; not re-driven.

Watches: none folded.

Outcome basis: the operator pass ran. The verdicts rest on its final state: the one commit `33d2084` and its CI run
ci#37609247992, recorded in `evidence/operator-pass.md` (the record P7 re-verifies); no fix commit was needed. The
implement conversation is present in this window and is the basis for the deviations and decisions above.

Process hygiene (implement's census at 10:52:12Z; re-measured at this wrap, 10:57:01Z: no process of this chunk
alive — no process whose executable is under this repository's `target/` or is the 2.1.287 binary, 11 probe dirs
at the root, the same eleven; the four `ci.py conclusion` processes the name pattern matched belong to another
project's sessions, by their cwd and their start after this chunk's wait had ended):

| process | started by | final state |
|---|---|---|
| scratch drivers, stand-in, helper, hook scripts | this run | terminated |
| three `claude` 2.1.287 sessions | this run's drivers | terminated (Ctrl-C twice, exit 0 each; none killed) |
| `viola run` ×3 and the fake-agent rehearsal | this run's drivers | terminated |
| harness smoke session | the gate block | terminated (`processes_gone` true) |
| `ci.py conclusion` wait | this run | terminated |
