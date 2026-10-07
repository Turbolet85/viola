# Scope — 2026-10-07-a-send-ending-in-a-newline-is-confirmed

**Working entry** (`working-route.md:100`, Epoch 3 — Windows slice II: driving verbs and live proof):
A send ending in a newline is confirmed — a driver's text whose last byte is a newline is reported delivered when it
is, and the driver keeps the wheel.

## Intent
Today a `send` whose text ends in a newline is typed, delivered and runs a turn, and is still reported
`not-delivered` / `no-prompt-submitted` (exit 13) when the 10 s window closes; its own prompt is filed `human` and
the wheel moves to the human (`cause` `human-input`). This chunk makes that send end as any delivered send does:
its prompt is the driver's, the send is confirmed, and the wheel stays with the driver. The remedy is not chosen
on the route: it is this chunk's own P4 card.

The chunk also carries one item that is not on the entry: the open CI red on the last wrap's commit (§The folded
red), folded on the operator's word with its own acceptance and its own witness.

## Authority
- **Placement** (the entry's first CARRY, the overseer's disposition at the
  2026-10-07-live-rows-and-paste-shapes-on-the-dev-host wrap): a corrective entry at the head, right after "Send
  waits out the paste hint" and ahead of "First live test and self-drive".
- **The remedy and any live start are this entry's own phase cards** (the same CARRY).
- **The operator's directive at this take-up** (inputs#I1, a verbatim copy):
  - no live `claude` session and no capability: the plan holds zero live starts, and the live proof rides "First
    live test and self-drive" (`working-route.md:102`);
  - the CI red on `9f2bebe` is folded as one `[inferred]` item with its own acceptance and witness: the dead
    process is named by the session-learnings method, and the red is closed by cause, never by a re-run or a
    looser merge;
  - if the two do not fit one builder window, P4 says so and the operator mints the red its own entry at the head;
  - the founder is away: a remedy card that is his is answered provisionally by the operator and listed for him;
  - a boundary widening is not answered, it is held.

## What the chunk builds
1. **The claim.** The in-flight send's own prompt is recognised when the sent text's last byte is a newline. Site
   re-verified at the take-up: `SendSlot::claim`, `src/run/send.rs:120-132`, compares `f.text == text` exactly
   (`:125`); `append_hook_event` (`:210-245`) relabels the line `origin:"driver"` only on a claim (`:216-222`) and
   moves the wheel for a prompt still filed `human` (`:237-239`).
2. **The outcome the entry names.** For a sent text whose last byte is a newline, on the path the remedy covers:
   `send-issued`, the `prompt-submitted` line with `origin:"driver"`, `send-confirmed`, the `ok` result; no
   `wheel` line, the wheel still `driver`, and a following `send` not refused `human-typing`.
3. **The remedy is a P4 card.** Three shapes, none chosen on the route: the wrapper accepts the prompt that is the
   sent text less its last newline; `send` removes the last newline before the paste; `send` refuses a text
   ending in a newline with nothing typed. Closed at P3 against the extracts: the second meets security-plan's
   ban on stripping a character from the sent text and changes what `prompt-submitted.text` and `text_bytes`
   mean; the third refuses an allowed character, needs a refusal detail and a hint no plan holds, and does not
   meet the entry's own sentence ("is reported delivered when it is"); the first changes the exact-match sentence
   of [Delivery Confirmation] and meets the ledger rule (item 4).
   - **3a. Decided at P4** (inputs#I2; the overseer's provisional answer for the founder, who is away; his to
     confirm or overturn, and listed for him with every option): `send` removes every trailing LF before the
     paste. The text is validated as received; classification, the paste and the exact claim all read the one
     typed text. Basis as answered: nothing new crosses, the claim stays an exact match on one text, and
     security-plan's reason for reject-not-strip (the exact match) is kept. Price as answered: more of the
     driver's bytes are removed than the one newline the measurement shows. Every master line the wrap touches
     for it says provisional; if the wrap's judge escalates the ban's amendment, it is held for the founder.
   - **3c. A consequence, read at P5.** A listed local command followed by newlines becomes that command
     (`/clear` and a newline clears the session). One existing unit case pins the opposite today
     (`src/run/send.rs:1403`) and turns with the build. It is on the founder's list with the remedy.
   - **3b.** With 3a no `send` relies on the drop, so item 4's question does not arise for this build: no ledger
     row, no probe, no stamp.
4. **The capability-ledger question.** The newline drop is an undocumented CLI behaviour. Closed at P3:
   architecture's ledger rule holds one ruled limit ("a shape no `send` relies on needs no probe") and no
   "tolerates, does not rely on" class; a new row is a probe, a fifth compiled paste and a re-stamp of every
   stamped version (five `claude` starts a verify), which this chunk may not start (inputs#I1); the one earlier
   reliance on an unprobed shape was named a boundary widening and ratified by the founder. So whether a remedy
   needs a row, and whether building it without one is a widening, is a ruling asked on the P4 card, not a
   reading.
5. **The tests.** A case that sends a text ending in a newline and reads the confirmed outcome of item 2, with a
   control that fails when the remedy is removed.
   `[premise-corrected: the fake agent does not reproduce the drop — submit fires UserPromptSubmit with the typed
   text, last newline included (src/bin/viola-fake-agent.rs:466), so under it a text ending in a newline is
   confirmed today and a plain end-to-end case would pass with no remedy]`
   With the remedy the card chose (item 3a) no drop needs replaying: the typed text never ends in a newline, so
   the discriminating reading is what the fake agent received, read from its receipt. The exit code and the
   records are a floor under the fake agent, because it echoes. (For the tolerant-claim shape, not chosen, the
   drop could have been composed from `--suppress-prompt-submit` and a gated script step firing the recorded
   `UserPromptSubmit.default`; nothing of that is built.)
6. **The pins that turn.** The two hook cases that pin today's normalised shape for a text ending in a newline
   (`crates/viola-agent-claude/src/hook.rs:622`, `:627`, `long_ending_newline` and `short_ending_newline`) stay
   as they are if the remedy sits in the wrapper, and are re-read if it does not.
7. **No live proof here.** The chunk starts no live session and stamps nothing. What stays unmeasured on a live
   CLI after this build is named in the plan and carried to "First live test and self-drive".

## Folded freight (the entry's second CARRY)
Each claim keeps its marker. P3 closed each against the artifact named: the evidence pointers still hold at HEAD,
and the code path of item 10 was re-read.
8. "measured at that chunk on `claude` 2.1.287, on a verified and an unverified home
   (`evidence/hint-window.md` step 7, `scratch-session.md`, `live-shape-red-green.md`): the CLI drops a pasted
   text's last newline before UserPromptSubmit. For a wrapped text it adds no newline before the close tag, so
   the prompt is byte for byte that of the same text without it; an unwrapped text loses it too." Coordinates
   re-verified at the take-up: the three files exist under
   `viola-0.1.0/chunks/2026-10-07-live-rows-and-paste-shapes-on-the-dev-host/evidence/`; `hint-window.md:86` and
   `:125` are step 7 on the two homes (exit 13, `not-delivered` / `no-prompt-submitted`, 10.014 s and 10.020 s);
   `scratch-session.md:61-62` are the two newline shapes (1 557 chars for the wrapped one, 95 chars against 96
   bytes for the short one).
9. "A fake-agent replay of the drop needs a recorded fixture first (test-plan §7)." Read at P3
   (`test-plan.md:1063`, `:1077`): a recorded payload comes only from `viola verify --record`, `--framing`
   replays a recorded variant with its bytes unchanged, and no shape is invented before a recording. No committed
   variant records a text ending in a newline, and none can be recorded with zero live starts. Item 5's
   composition invents no shape: it fires a recorded variant unchanged and suppresses the echo with an existing
   mode.
10. "`prompt-submitted.text` is then one byte short, the exact match fails, the prompt is filed `human`, the wheel
    moves (`cause` `human-input`), the turn runs, and the send ends exit 13 `not-delivered` /
    `no-prompt-submitted` after the 10 s window." Verified at HEAD: the claim at `send.rs:125`, the wheel move at
    `:237-239`, the window's expiry at `:362-365`; `claim` has one caller (`append_hook_event`, `:219`).
11. "No fix inside `hook.rs` exists: the two prompts are the same bytes there." Verified: the hook receives the
    prompt only, never the sent text (`hook.rs:599-600`).
12. "architecture [Delivery Confirmation], [Human Takeover / Wheel] and the `prompt-submitted` contract state it
    as measured and unfixed."
    `[premise-corrected: four homes, not three — architecture.md:49, :70 and :292, and the long-paste wrapper row
    of [CLI Version Compatibility] at :81]`
    They are the wrap's to amend once the build lands; phase edits none.
13. "Unmeasured: more than one trailing newline; a text that is only newlines." Both stay unmeasured in this
    chunk. The plan says what the build does for each and claims no measurement.

## The folded red
14. **ci#37627485806 attempt 1 · sha `9f2bebe5102b8df1053b4b5c26225d1f6352f0e4` · job
    `test (ubuntu-latest)` · failed steps `Coverage and doctest (sh shim)` and `Gate verdict`.** Read from that
    attempt's failed log at the take-up:
    - `Summary [ 144.959s] 1695 tests run: 1695 passed (5 slow), 0 skipped`
    - `warning: /home/runner/work/viola/viola/target/llvm-cov-target/viola-10799-3177174534174374433_3.profraw:
      invalid instrumentation profile data (file header is corrupt)`
    - `error: no profile can be merged`
    - the harness's own verdict: suite `coverage`, `passed 1695`, `failed 2`, failures `llvm-cov-exit-1` and
      `llvm-cov-summary-missing`, archived `target/run-archive/30`; the gate's breaches `suite-failed` (coverage,
      `failed 2`) and `artifact-missing` (`llvm-cov-summary.json`), then `suite-missing` and `artifact-missing`
      for `playwright` (read at P3: the browser step was skipped after the coverage step failed).
    Attempt 2 of the same run, on the same sha, is green 15/15. Closed at P3 against that run, never against
    HEAD: the attempt stands as recorded.
    - **Its subject does not intersect what this chunk builds.** It is folded on the operator's word
      (inputs#I1), as promotion's third arm allows, with its own acceptance criterion and a witness that cannot
      pass vacuously.
    - **Closed by cause.** A green re-run, a merge that skips or tolerates a corrupt profile, and a retry of the
      coverage step are not closures.
      `[premise-corrected: the method in .claude/docs/session-learnings.md:12 reads the refused profile's own
      counters, and the profile is not on the run (below), so the dead process cannot be named by it on this
      attempt]`
    - **The evidence.** Three artifacts of attempt 1 stand on the run until 2026-10-14: `harness-ubuntu-latest`
      (31 348 310 B), `junit-ubuntu-latest` and `diag-ubuntu-latest`.
      `[premise-corrected: the harness artifact holds 10 members and no .profraw — the upload takes
      target/agent-run/ and the profile sat in target/llvm-cov-target/]`
    - **What names the cause instead** (research.md, "The folded red"):
      - the profile's name carries pid 10799; the diagnostics artifact brackets that pid between 13:21:39.511Z
        and 13:21:41.038Z, a span in which no test home wrote a line; the JUnit artifact puts
        `cmd::run::tests::start_opens_the_scenario_one_spans_under_run_start` inside it, a test whose home lies
        outside the uploaded tree;
      - that test started the instrumented test binary as a PTY child and killed it at once; a child that exits
        by itself writes its profile as it exits;
        `[premise-corrected: the child never listed anything — the wrapper puts its plugin flag first in a child's
        arguments (src/run/mod.rs:125-128), so it ran as "<test binary> --plugin-dir <dir> --list", and libtest
        refuses that flag: "error: Unrecognized option: 'plugin-dir'", exit 101, 0 bytes on stdout (measured at
        implement, evidence/profraw-red-green.md). It exited by itself at once, which is the mechanism as stated]`
      - measured at HEAD on the last pre-push's instrumented binary: of 1 200 loaded runs of that one test, one
        left a profile of 69 632 B against 128 264 B, which `llvm-profdata` refuses with attempt 1's two lines.
    - **What stays unproven:** that pid 10799 was that child. The file is gone and the runner's binary signature
      names nothing on this host. One `tui_wheel` case was also alive in the span. The plan states the closure
      with this limit.
    - **The earlier instances.** The same message was traced at chunk 2026-10-04-running-turn-refusal
      (`evidence/watch-profraw.md`) to a `viola hook` cut off at its exit, on a local run where the file stood;
      that fix (`test-plan.md:1078`) does not reach this path. Two earlier CI runs on the same leg,
      ci#36529038462 and ci#36481260151, stand in `test-plan.md:1207` as "recorded, not established"; the test
      was added before both, and whether they had this cause is not measurable now.
    - **The cause is in this repository**: a test of the root bin.
    - **Decided at P4** (inputs#I2, the overseer, technical): closed in this chunk, limit stated. A counted
      witness on the instrumented binary is added, baselined red. The report states that the mechanism is
      measured and pid 10799's identity is not provable from the run. No new member in a CI upload.
      `[premise-corrected: the P4 answer's fix, a child that cannot exit by itself, cannot be built — a
      child-entry test of the test binary is never entered through start, because libtest exits 101 on the
      wrapper's plugin flag before any test body runs (evidence/profraw-red-green.md)]`
    - **Decided at implement, on the step 7 card** (inputs#I5, the overseer, a technical fork): the test's child
      is an uninstrumented host program, `whoami`. A child that writes no profile cannot leave a corrupt one,
      whenever the kill lands. It may still exit by itself. The deviation from the planned step is in
      `scope-record.md` with the overseer's word. The Windows and macOS halves are unmeasured before CI: a
      Windows red is read and fixed by cause in its own commit, and the green-on-first-attempt rule binds the
      final sha.
    - **The census counts one whole profile a run** (inputs#I7; closed at this revision against
      `evidence/profraw-red-green.md`). The version probe runs the same program as the wrapper's child
      (`src/run/version_gate.rs:106-117`), so with a host program neither child is instrumented and only the
      test process writes a profile: 48 runs one at a time on the fixed test each left one profile of
      129 512 B. The take-up plan's two-a-run count described the test binary as the program.
    - **The red side of the census stands as recorded** (inputs#I7): 4 800 loaded runs on the untouched test
      under the two-a-run rule, 3 runs with a third profile, all 3 short (0 B, 73 728 B and 124 288 B against
      128 264 B). The untouched test is not rebuilt for a second red reading.
    - **The revised rule gets its own failing side as a planted control** (inputs#I8, the operator's word at the
      revision's review; closed against `scripts/profraw-census.sh:21`, `:51` and `:53`): one census run given one
      extra profile file and one given a short file each read exit 1; the plants are removed and the removal is
      checked; both readings go to `evidence/`. No rebuild. The script counts a run's files in a directory only
      it can name today, so the control needs the script to take the census directory's name as an optional
      word.
    - **No push goes out with a gate entry red by its own letter** (inputs#I6, the overseer): implement stopped
      before the 4 800-run census and the operator pass, and finishes them against the revised entry.
    - **Size.** If the send remedy and the red do not fit one builder window, P4 says so (inputs#I1).

## CI verdicts read at Setup
- `9f2bebe5102b verdict: green · checks 15/15 · wall 806 s · runs ci#37627485806 completed/success` (`ci.py`,
  the one sha from the last flip through HEAD). The tool reads the run's latest attempt; attempt 1's red is item
  14, read with `gh run view 37627485806 --attempt 1`.

## Boundaries
- Zero live starts; no `viola verify` round; no stamp written; no new fixture recorded.
- No capability is claimed (inputs#I1).
- No boundary widening: no new `VIOLA_*` variable, flag, config key or exemption. One the plan would need is
  held, not answered.
- The `input-not-ready` hint line, `v1-34`'s two owed rows and the Windows-only live items are not this chunk's.
- The spec masters are read-only here; the exception sentences of item 12 move at the wrap.
