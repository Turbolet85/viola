# Founder forks — «A fired scheduled wake keeps the wheel» (chunk 2026-11-19-a-fired-scheduled-wake-keeps-the-wheel)

The phase stopped here, at the end of P3 and before P4, on the directive's item 3 (inputs#I1). Nothing below is planned
around: every fork waits for the founder's word. Written 2026-11-19 (UTC), taken up at `33321186`. The scope is
`doc:2026-11-19-a-fired-scheduled-wake-keeps-the-wheel/scope` rev 3, and the research is `…/research` rev 1.

## The window
- One builder window ran Setup, P1 (promotion tx 3230), P2 and P3. The P2 row read: 7 task writers, 156 picker draws
  in 3 parts with one `EVERY_ROW` re-spawn that passed, the union picking 1362 of 4745 rows, and 7 history agents.
  P3 made 14 reads, 16 greps and 2 graph queries, plus bounded greps of the installed 2.1.328 bundle.
- No live start, no read of the real `~/.claude*`, and no wake or timer tool.
- P4 (the plan) and P5 (the review card) remain. They resume in this run dir on the founder's word.

## What the code says (research, read at HEAD)
- The wheel moves to the human from exactly two product places (code graph, `human_input`'s callers):
  - the stdin observer `src/run/wheel.rs:308`, which moves it at the human's key, in the read that carries the key,
    before the byte reaches the CLI;
  - the hook's human-prompt path `src/run/send/slot.rs:251`, which moves it when a `prompt-submitted` the hook filed
    `human` is not the in-flight send's.
- So any prompt whose bytes passed viola's stdin has already moved the wheel before its hook fires. A fired wake types
  no byte. Its only path to the wheel is the hook's `human` filing.
- No field marks a wake on 2.1.328. The UserPromptSubmit hook input spreads `...!1` where `source` stood: compiled
  out, in both builders. The wake's source reaches only in-process callback hooks. No hook event fires for a
  scheduler fire (33 hook schemas read).
- The turn end already counts pending wakes: `scheduled_wakes` is the Stop's `session_crons` length. It read 1 before
  the fire in both forms on 2.1.325 (R3B, R3D). No relied ledger row stands behind it; it is data only.

## F1 — The means (recommended first)

**(A) Recommended — the byte source, gated on the wake count.** A `human` prompt keeps the wheel where it is only when
both of these hold. In every other case it takes the wheel, as today.
- No human editing byte crossed viola's stdin since the previous `prompt-submitted` (or the start).
- The last turn end counted at least one scheduled wake.

The details:
- The turn still starts. The driver's next send reads `turn-running` until that turn ends, then it is confirmed. A
  question dialog raised in that turn is held for the driver, because the wheel stays `driver`.
- What it rests on:
  - the stdin observer (viola's own code);
  - `session_crons` on the Stop (a CLI shape, F3);
  - the CLI submitting a wake with no byte on the pty (measured: R3, R3D, R3-326 typed no key and fired).
- How it fails: a count that reads 0 or null while a wake is pending. The wake then takes the wheel exactly as today:
  the conservative side.
- Which human acts it can and cannot tell apart:
  - **A prompt typed or pasted while a wake is pending:** told apart, filed human. Its bytes cross stdin and moved
    the wheel at the key.
  - **A key typed during the wake's fire:** told human. That is a false negative, on the conservative side.
  - **A second wrapper, or a resumed session in another terminal:** told apart. Its prompts never reach this
    wrapper's hook.
  - **A message the human typed while a turn ran (queued), then a `viola release` before the CLI submitted it, with
    a wake counted:** CANNOT be told apart. It is read as a wake, and the wheel stays with the driver the human just
    released it to. This is the false positive that matters most.
  - **A prompt submitted by the mouse alone:** CANNOT be told apart, and whether 2.1.328 can do this is unmeasured.
    Mouse reports are on the non-editing list, so such a prompt reads as keyless.
  - **A cloud scheduled-task fire that `session_crons` does not count:** taken as human, as today. Conservative.
- An older CLI, or one whose Stop lacks the key (`null`): human, as today. An unverified CLI: the rule is off and
  every prompt is human, as today, because it rests on a CLI shape (F3).

**Set aside, each with its false positive:**
- **(B) A transcript read of the CLI's in-memory `wakeupSource` / `textNotTyped`.**
  - Why set aside: they are written only behind a feature value that defaults false (`pCt()`). That they reach the
    transcript at all, and before the hook, is unmeasured.
  - How it fails: an absent field reads as human.
  - False positive: none, since only a fire writes it. It would cost a live reading and a new crossing of transcript
    content.
- **(C) A text match of the fired prompt against the scheduling call's `prompt` (the CronCreate / ScheduleWakeup
  tool input).**
  - Why set aside: it reads tool-input text (upstream content) into memory.
  - It also leans on two unmeasured shapes: how `/loop 1m` arms its cron, and the tool input's field.
  - False positive: a human typing the same text while a wake is pending, which (A) already tells apart by the key.
- **(D) A hook for the fire:** none exists on 2.1.328.
- **(E) The count dropping after the fire:** read only at the fired turn's end, too late for the wheel.
- **(F) The `source` key:** compiled out. The founder retired that clause on 2026-11-17.

**The widening (A) makes, said here:** a `human`-filed prompt no longer always takes the wheel (`decision/human-prompt-confirms-wheel-take`, the wheel's origin rules). The precedents are the harness-prefix widening and its accepted false positive; the survey-key ruling («Выкинуть фичу») says a CLI-driven input keeping the wheel is the founder's own word.

Options: **(A) (Recommended)** · (A) without the count gate (no human byte alone: cheaper, no F3, but every keyless prompt keeps the wheel — any future CLI-injected prompt included) · (B) · (C) · none (the entry stands unbuilt).

## F2 — What the fired prompt's line says
- **`harness` by a wrapper relabel (Recommended).** No new word. A `harness` prompt already never moves the wheel
  and never wakes `wait`. It is a second relabel path beside `driver`, and the event contract «the hook files only
  human|harness» gains «the wrapper relabels a told wake `harness`».
- A new origin word `wake`. This widens the closed `driver|human|harness` set read by `events.ndjson`, `wait --json`
  and MCP `wait`.
- The line keeps `human`, with no wheel move. No contract word changes, but the log contradicts the wheel.

## F3 — A ledger row behind `session_crons`
- **A shape row (Recommended), `stop-session-crons`:** its probe reads the Stop captures Run B already takes. The key
  must be present and an array. It adds no run, start, paste or wait to verify.
  - It proves the shape, not that a pending wake counts. That rests on R3B/R3D on 2.1.325 and on the live wake
    reading (S3) on 2.1.328.
  - Cost: every stamped version reads unverified until re-stamped with 26 rows. The live home (24 rows) is already
    owed a re-stamp at the product re-pin, so this cost folds into that step.
- A semantic row: verify schedules and fires a wake inside Run B (`/loop 1m …`). This adds at least 60 s plus a
  model turn per verify, writes a cron into the probe session, leans on a model tool call
  (CronCreate / ScheduleWakeup), and costs the same re-stamp. If chosen, the entry splits: this row becomes its own
  chunk after this one.
- No row, on the StopFailure precedent: a named exception on the founder's own word, with no re-stamp. A count that
  reads null degrades to today's behaviour.

## F4 — The split
- **No split (Recommended).** One implement window holds:
  - the stdin mark;
  - the slot rule and the relabel;
  - the event-contract change;
  - the F3 shape row (if chosen);
  - a fake-agent turn ending with a non-empty `session_crons` followed by a keyless plain prompt (a synthetic Stop
    beside the recorded sets: no recorded set holds a non-empty one);
  - the root wheel test (`tests/cli_wheel.rs`);
  - the rig's hint arm and its wake-wheel reading.
- The entry splits only under F3's semantic row (the verify probe becomes its own chunk) or if the founder wants the
  live wake reading in a later chunk.

## F5 — The live starts: shape before count (none made in this phase)
None of these starts types a key that moves a wheel before a send. S2 types keys into an unwrapped claude, so there is
no wheel. S1 and S3 type no key.

**S1 — `CS-26`, the rig's verify into its scratch home.** Only if F3 adds a row: it writes a 26-row kept stamp for
2.1.328.
- **Processes:** the rig, then `viola verify --home <the rig's scratch viola home>`, then verify's own children (the
  print probe and the interactive runs, as `CS` today). claude 2.1.328 is taken by its path.
- **Bytes typed:** none by the rig. Verify's compiled Run B pastes are as stamped today (eight); F3's shape row adds
  none.
- **Wake:** none scheduled.
- **Stop:** verify's own end.
- **Files:** the scratch viola home's `ledger/stamps.json` and its kept copy beside `target/live-rig/context-stamp.json`
  (a new kept file for 26 rows). The rig's private Claude config and Run B / C / D transcripts sit under the rig's
  scratch user HOME, never the real one.
- **Fail predicate:** the stamp reads 26 of 26 relied rows pass, the new row among them. Red: any row `fail` or
  `absent`.
- **Reads:** row names and pass/absent/fail words; counts.
- **Widening:** none beyond F3's row.

**S2 — `HB`, the third hint reading (carry cry-iuatjqkx).**
- **Processes:** the rig, then an unwrapped `claude` 2.1.328 in the rig's scratch cwd, under the rig's private,
  seeded config. No viola, no send, no turn.
- **Keys:** the founder's (below). Typed into the fresh box before any send; no send follows.
- **Wake:** none.
- **Stop:** the rig's own (Ctrl-C into the child, then the kill line).
- **Files:** kept frames `first`, `after-x` and `after-del`, and the summary, under the rig's kept dir for the arm;
  the scratch user HOME's config.
- **Fail predicate:** `box-up: yes` and at least one drawn hint cell past the cursor's. Red: `no-cell` on a fresh
  box, which would falsify the carry's premise that a fresh box draws the hint.
- **Recorded readings, not atoms:** `hint-told-apart` yes/no and the attribute words.
- **Reads:** cell attributes only (the existing `hint_cells` / `box_attr` words), never text.
- **Widening:** a new arm word (`HB`) of an existing reading class. No new reading kind.

**S3 — `R3W`, the fired wake under the new product** (only if the founder wants a live proof).
- **Processes:**
  - the rig;
  - `viola run` built from this tree, under the rig's scratch viola home (seeded from the kept stamp: S1's 26-row
    stamp under F3's row, else the 25-row `target/live-rig/context-stamp.json`);
  - claude 2.1.328;
  - `viola send` processes, two of them.
- **Bytes typed by the wrapper's paste, not keys:** `/loop 1m Reply with the single word ok.`, then after the fired
  turn ends, `Reply with the single word ok.`. Both are compiled literals of `crates/viola-e2e/src/rig/wake.rs`.
- **Wake:** scheduled by the CLI's own `/loop 1m`. It fires about 60 s after send 1's turn, and the rig stands up to
  its stand bound.
- **Stop:** the rig's Ctrl-C into the wrapper. The cron dies with the session.
- **Files:**
  - the scratch viola home's instance dir (`events.ndjson`, snapshot, diagnostics);
  - the rig's frames and summary, and its `capture.ndjson` (key names and the source word only, as ratified);
  - the CLI's transcripts under the scratch user HOME.
- **Fail predicate:** `send-1-wakes ≥ 1`, `wake: fired`, the fired line's origin the F2 word, no `wheel` line
  after the fire, and `send-2-reply: ok`. Red: `wake-origin: human`, a `wheel` line to `human`, or `send-2-reply`
  `human-typing`.
- **Reads:** closed words and counts. The rig reads viola's own `wheel` lines and the F2 origin word from the scratch
  events log, which is NEW: a reading of viola's own events, no CLI text, said here as a widening.
- **Turns:** three model turns.
- **Token window:** the rig's own check. A start refused for it is uncounted.

**What each start needs on disk outside the tree, each with a no-start gate entry right before it, so a resumed round
still fires the gate:**
- **S1:**
  - PRESENT: the claude 2.1.328 install path and a runtime dir outside the temp root.
  - ABSENT: the arm's kept dir.
  - After: an entry reads the kept 26-row stamp.
- **S2:**
  - PRESENT: the 2.1.328 path.
  - ABSENT: the arm's kept dir.
  - After: an entry reads its three frames.
- **S3:**
  - PRESENT: the kept stamp it seeds from, the 2.1.328 path, and the runtime dir.
  - ABSENT: a live `rig-r3` endpoint in the scratch home, and the arm's kept dir.
  - After: an entry reads the instance's events and the frames.
- A host read (`hostwatch.py read --last 15`) comes before each round. A CONTENDED reading naming a neighbour stops
  the round, and it is re-read.
- The census entry after the last live leg is keyed to the operator leg.
- One ledger, one count: `--cap` is the running total.

## F6 — What «verified» rests on, and the cap

**The concretized claim (v2-36):**
- **Holds:** under `viola run`, on a CLI whose stamp counts every relied row, a prompt submitted with no human editing
  byte on viola's stdin since the previous prompt, while the last turn end counted at least one scheduled wake:
  - leaves the wheel with its holder;
  - is logged with F2's origin;
  - and after its turn ends, the driver's next send is confirmed and the next question dialog is held.
- **Still takes the wheel:** a prompt with a human byte, or with no wake counted.

**«Verified» rests on:**
- **(Recommended)** the root rehearsal under the fake agent AND S3 on 2.1.328;
- or the rehearsal alone, with S3 left to the product re-pin round.

**Recommended cap:**
- Under F3's shape row: 3 base starts (S1, S2, S3) plus 1 spare.
- Under no row: 2 base starts (S2, S3) plus 1 spare.
- The spare is for one re-fire of S3 if the wake does not fire inside the stand bound, or of S2 if the box is not up
  at the first frame. It is never a retry of a red.

**The keys the hint reading needs:** `x` then DEL (the founder's earlier grant «2 клавиши: x, DEL»), typed into
S2's fresh box. Recommended as before, or another pair of his choosing.

**What waits for the founder:** the cap; the keys; the S3 widening (the rig reading viola's own `wheel` lines);
F1's widening of the wheel rule; F2's contract word; F3; and what «verified» rests on. No start reads or writes under
his real home: every start runs under the rig's scratch user HOME and scratch viola home.

## The carries, disposed by name (into P4 on the word)
- **cry-umbkmr7p:** the basis of F1, verified at HEAD (no source reaches a command hook on 2.1.328).
- **cry-4tpqeixz:** the measured part is spot-checked. Its hypothesis is verified as the equality above and is F1's
  (A).
- **cry-iuatjqkx (C3):** S2, under F5 and F6.
- **cry-yx2svady (the rehearsal red):** acted on only at its next red. The plan carries its procedure (read the
  printed rig summary and the kept `after-del` frame, name the cause, then mend the wait), with no retry rule and no
  loosened entry.
- **cry-b3gskjj3 (the boot-timeout WATCH, 0/3):** read at each gate run, with no bound moved. No criterion, task or
  entry.

## The founder's answer
Arrived as the overseer's relay `relay-wake-p3.md` (snapped `inputs#I3`, step phase:P4), its founder's part given
in a dialog and read at 2026-11-20T06:06:04Z. Appended verbatim below; P4 resumed on it in this run dir.

> 1. **F1: «Клавиши + счётчик (Recommended)».** Means (A): a prompt the hook files `human` leaves the wheel where it
>    is only when BOTH hold — no human editing byte crossed viola's stdin since the previous prompt, AND the last
>    turn end counted at least one scheduled wake; in every other case it takes the wheel as today. He was shown,
>    in these words, what it cannot tell apart: a message he typed while a turn ran (queued by the CLI), then his
>    `viola release` before the CLI submitted it, with a wake counted — read as a wake, the wheel staying with the
>    driver; and a prompt submitted by the mouse alone (unmeasured on 2.1.328). This is his ratification of that
>    one widening of the wheel rule (`decision/human-prompt-confirms-wheel-take`) with that named false positive,
>    and nothing wider: on an unverified or older CLI, or a count that reads 0 or null, every prompt is human.
> 2. **F3: «Строка формы (Recommended)».** A shape row, the 26th relied row, behind the turn end's wake count: its
>    probe reads the Stop captures verify already takes — the key present and an array; no run, start, paste or
>    wait is added to verify. He was told: every later build reads every stamped version unverified until
>    re-stamped, and the live home's re-stamp (now 26 rows) folds into the re-pin step, outside this chunk.
> 3. **F5/F6: «Все старты + 1 запасной (Recommended)».** THREE base starts and ONE spare, claude 2.1.328 by path,
>    under the rig's private config and scratch homes — `S1` (the rig's verify into its scratch home, the 26-row
>    kept stamp), `S2` (the third hint reading: an UNWRAPPED claude, one `x` and one DEL into the fresh input box
>    of the rig's own pty before any send, no send at all, cell attributes only, never a text), `S3` (the fired
>    wake under the new product: the two compiled sends as your block words them, the CLI's own `/loop 1m`, three
>    model turns). The rig reading viola's OWN `wheel` lines and the origin word from its scratch events log is
>    the reading he ratified with `S3`, in that width. «Verified» for the capability rests on the fake-agent
>    rehearsal AND `S3`. The spare is spent only on `S3` when the wake does not fire inside the stand bound, or on
>    `S2` when the box is not up at the first frame — never on a red reading. From his real home
>    `~/.claude/.credentials.json` is READ once by the rig per start (a 0600 copy removed at teardown) and
>    `~/.claude.json` stamped read-only; nothing is written under his real `~/.claude*` or `~/.viola`; no byte into
>    any CLI dialog. A fourth base start is his.
> 4. **F2: «Перемаркировать в harness (Recommended)».** The wrapper relabels a told wake's prompt `harness`; no new
>    origin word enters the closed set; the event contract gains that one clause.
>
> Technical answers (the overseer's): 5. F4 no split, one implement window; a window that does not hold it is said on
> the card with its cut. 6. The false positive is TESTED by a fake-agent rehearsal of the queued-message case; the
> plan says whether the stdin mark clears at a turn end or only at a prompt; a mark surviving to the NEXT prompt is
> the conservative default, its cost said; a `viola release` clearing the mark is then the only remaining door,
> named. 7. S3's fail words are stopping reds; its wait bounded and named beside the expected ~60 s; S2's `no-cell`
> is a red of that start's premise and stops nothing else — the round goes on to S3. 8. S1 first; S3 seeds from
> S1's kept 26-row stamp; a no-start entry reads that stamp PRESENT before S3 and each arm's kept dir ABSENT before
> its start; `--cap` running (S1 1, S2 2, S3 3); the host read before EACH start; the token window the rig's own.
> 9. The recorded fixture sets and `STAMPED` stay green without a recording start (the synthetic Stop named as
> synthetic); else a fork here, not a fourth start. 10. Leaves hold no figure the code states. 11. When the plan is
> approved the phase ENDS; no wake or timer tool; on an account-limit refusal, stop.
