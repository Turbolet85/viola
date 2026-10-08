# Scope — 2026-10-08-first-live-test-and-self-drive

**Working entry** (`working-route.md:102`, Epoch 3 — Windows slice II: driving verbs and live proof; its last entry):
First live test and self-drive — overseer drives a builder through viola, founder takes the wheel, overseer drops
the prototype.

## Intent
On the Linux dev host the product binary carries one real driving session: an overseer session sends a pipeline
skill to a builder session wrapped by `viola run`, waits for the turn, reads the final text, answers a review,
clears between skills, and a key typed in the session's terminal takes the wheel. The founder is not at the keyboard
for this chunk: that key is a real one, typed into the live session's terminal window through a compositor by an
agent (inputs#I3). `[premise-corrected: evidence/key-probe.md — the desktop session is locked (hyprctl locked true,
the shell's lock held since 2026-10-07T19:39:15Z), so no window of the desktop compositor takes keyboard focus; the
founder ruled 2026-10-08T06:43Z that nobody unlocks it (inputs#I12)]` The compositor is one this chunk starts
itself: a Hyprland nested as one client window of the desktop compositor, with a minimal config and sockets of its
own, foot and wtype inside it (inputs#I12, inputs#I13). The builder's CLI is `claude` 2.1.287 named by path, in a
viola home this chunk stamps itself.
`[premise-corrected: evidence/terminal-replies.md — a live CLI in a real terminal does not keep the wheel with the
driver: start 6 logged wheel {human, human-input} 237 ms after its child started, no key typed, and the first send
was refused exit 10. The terminal (foot 1.28.0) answers the CLI's queries on viola's stdin, and seven reply shapes
are outside the wheel's closed list (src/run/wheel.rs:496-503)]` So this chunk also changes the product: the seven
measured reply shapes join the closed list, each by its exact grammar (inputs#I16), and the live run is retried in
a fresh session on the standing compositor.
The overseer's own switch from the prototype to viola is not made in this chunk: the founder ruled at P4 that the
live test is proved now and the switch waits for the entries that close its gaps (inputs#I7).

The same live session reads what earlier chunks proved under the fake agent only and left to this entry: a decided
dialog never renders, a `send` under the real paste hint, a `send` ending in newlines, and the fake agent's Unix
fidelity. `run --local-live` is fired on the dev host for the first time.

Every live `claude` start is counted against one cap of 8 for the whole chunk.

## Authority
- Founder rulings, live, 2026-10-07T05:43Z, relayed by the overseer (inputs#I2 §1; the entry's first CARRY):
  - **R-L1**: the first live test runs on the Linux dev host, and `verification-matrix.json#v1-33`'s acceptance is
    re-worded to that host. Windows live behaviour stays proven on the CI runner under the fake agent until an
    interactive Windows host exists.
  - **R-L3**: `claude` 2.1.287, named by path; the live-session cap for this entry is 8; PATH `claude` stays
    unstamped.
  - **R-L2** (for the record): Epoch 3 stays one epoch; this entry is its last.
- A founder ruling given live at 2026-10-08T05:02Z through the overseer's dialog, relayed in an operator note during
  this phase's P1, after the promotion (inputs#I3, a verbatim copy). Recorded as his:
  - the founder is NOT at the keyboard for this chunk. This replaces "founder-attended" in the entry's first CARRY
    and the first line of the take-up directive below; the rest of that directive stands;
  - the wheel takeover is proved by a real key typed into the live session's terminal window through the
    compositor by an agent, not by his hands. `wtype` and `hyprctl` are installed on this host; "whether such a key
    moves the wheel is unmeasured and is this phase research to settle";
  - at claim, `v1-33`'s acceptance is re-worded from the founder's typing to a key typed in the session terminal.
- The P4 answers (inputs#I7, a verbatim copy):
  - **the switch** (the founder, live in the overseer's dialog, 2026-10-08T06:07Z, the three options and their
    prices shown to him): live test now, switch later. This chunk proves the first live test and records the gap
    list; `v1-33` is not claimed here; the wrap pins the switch on the route;
  - **the hint line** (the founder, the same minute, three options shown): it advises sending again and says that a
    refusal that repeats needs a person; the literal wording is brought on the P5 card;
  - **`run --local-live`** (the operator; it stays inside the founder's cap of 8): one firing with the 2.1.287
    install dir first on `PATH`, and the home it stamps is this chunk's home. A red verify round stops the chunk and
    returns to the operator;
  - **the driver** (the operator): the implementing session drives. The key probe is step 0 of implement, before
    any live start; a failed or impossible probe stops the chunk and returns the takeover to the founder.
- The operator's review at P5 (inputs#I8, a verbatim copy; no approval word came with it): the live session also
  raises a permission dialog and a plan dialog, each answered by its id, and the run reader requires all three
  kinds; a kind the live CLI does not raise is recorded and `v1-31` is left pooled with a dated note; DPMS and the
  guard are re-read right before the key of the takeover. The approval followed (inputs#I9, a verbatim copy), and
  with it the reading of a kind that is not raised: the run reader reads red by its letter, nothing is pushed, and
  `v1-31` is released at a plan revision.
- The operator's two notes on the key probe during this phase (inputs#I4 and inputs#I6, verbatim copies): the
  guard above; and, after the probe was blocked, the test with DPMS on, with this word for its failure: "the
  fake-agent key probe becomes step 0 of implement, with the DPMS-on precondition written into the plan for the
  live run too". The test failed to move focus (research M1), so that word stands.
- The revision of 2026-10-08 (implement stopped on its plan at step 0; `evidence/key-probe.md`):
  - the overseer's focus measurements of 06:26Z to 06:28Z (inputs#I10, a copy): the focus dispatcher moves the
    workspace and the cursor and never the focused window. Its line "No locker process, `LockedHint=no`" is
    corrected by the operator (inputs#I11): those two probes cannot see this lock;
  - the operator's answer to implement's question on step 0 (inputs#I11, a verbatim copy): steps 1 to 3 now, stop
    before step 4; no lever past the lock, no key, no live start, no compositor of the chunk's own until the
    founder's word;
  - the founder's ruling, live, 2026-10-08T06:43Z, relayed by the overseer in the word that opened the revision
    (inputs#I12, a verbatim copy): the takeover key is typed in a compositor of the chunk's own, "a nested or
    headless Hyprland this chunk starts with a minimal config and socket of its own, foot and wtype inside it",
    "and the desktop lock stays untouched; nobody unlocks the session". Steps 1 to 3 stand as done;
  - the founder's answer to the revision's question, himself, live in the overseer dialog, relayed verbatim
    (inputs#I13): the nested form, its price accepted, with ONE compositor start for all live work. He was told
    that the start wakes the screens of the locked desktop and may end and relaunch the shell that holds the lock.
    Added by him: the desktop lock is read (the shell's answer and `hyprctl locked` on the desktop instance, a read
    only) right after the nested start and at the end of the live work, both recorded; "if either reads unlocked,
    stop at once and tell me".
- The second revision of 2026-10-08 (implement stopped on its plan at the first `send` of start 6;
  `evidence/terminal-replies.md`):
  - the operator's answer on the stop (inputs#I15, a verbatim copy): close the live session, measure which
    terminal reply the classifier takes for typing with no live start and no key, read the desktop lock and record
    it, and leave the own compositor up with no window: "the founder ruled ONE compositor start for all live work
    … so the one start is kept and stretched instead". The fix is "a plan revision through the phase door, in this
    chunk, with its own red-green case", and the live run is retried on the same compositor with the 2 starts
    left. The bound: "if no live start is made within 3 hours of this answer, end the compositor, take the 9a
    reading and report" (the answer arrived before 07:32:09Z). An unlocked reading still stops everything (S6);
  - the operator's word that opened this revision (inputs#I16, a verbatim copy): "the seven reply shapes measured
    in evidence/terminal-replies.md join the closed list in src/run/wheel.rs, each by its exact grammar, each with
    a red-green case and a negative control that the nearest human key still moves the wheel (F-W2 holds: a reply
    the terminal writes by itself is not a human key, and no human key may be read as a reply)". Research first
    reads which queries the CLI sends at its start, "from the recorded fixtures under fixtures/claude if they hold
    them (hypothesis), so start 7 is not spent on a missed shape".
- The operator's directive at this take-up (inputs#I1, a verbatim copy):
  - "the founder is at the keyboard for this chunk" — superseded by inputs#I3 above;
  - the dated note on `v1-33` overrules its requirement line and title, so the claim re-words the acceptance to the
    Linux dev host;
  - 2.1.287 by path, a live-session cap of 8, and the chunk stamps its own viola home;
  - the wording of the `input-not-ready` hint line goes to P4 as a question for the founder: it still advises
    `viola wait`, which woke on nothing in the live run;
  - the two CARRYs written for the Epoch 3 boundary audit are the audit's, not this chunk's scope.
- Earlier answers this entry inherits (in this repository):
  - the dated note on `v1-33` (0-pending wrap `2026-10-07T05-47-07-wrap`): the acceptance text was left as it was,
    and "its claiming chunk (\"First live test and self-drive\") writes the re-worded text at claim. The title and
    `requirements.md:48` still say \"on Windows\"";
  - the founder's hint-card answer of 2026-10-07T10:29Z (chunk 2026-10-07-live-rows-and-paste-shapes-on-the-dev-host,
    `evidence/hint-card-answer.md`): the gate waits for the input box and the bound is 8.5 s. The hint line's wording
    was not part of that answer and is still open;
  - the founder's ruling of 2026-10-07T15:21Z: `send` types a text without its trailing LF characters.

## What it builds (the entry and its eight freight blocks folded)

### W1 — the first live test (the entry's own line; CARRY 1; `v1-33`)
- One live run on the Linux dev host with the product binary, the founder not at the keyboard (inputs#I3): the
  overseer sends a pipeline skill to the builder, waits for the turn, reads the final text, answers a review, clears
  between skills, and a key typed in the session's terminal takes the wheel.
- The key is a real one: an agent types it into the live session's terminal window through a compositor
  (`wtype`, with `hyprctl` to find the window), so it reaches `viola run` as terminal input, the way a
  hand-typed key does. `[premise-corrected: evidence/comp-probe.md — the compositor is the chunk's own nested
  Hyprland, not the desktop's. Measured 2026-10-08T06:51Z over the fake agent: the probe window took focus by
  itself on the own instance, the focus wrote no wheel record, wtype x wrote wheel {holder: human, cause:
  human-input}, the receipt holds key 78, a driver send exited 10]` Every `hyprctl` call of the chunk names the own
  instance (a private runtime dir and its signature); the desktop instance is asked one thing only, `locked`, a
  read (inputs#I13).
- The own compositor is started once for all live work (inputs#I13). Verified (evidence/comp-probe.md): its window
  on the locked desktop counts as activity for the desktop's idle monitor, the shell runs its wake, and in the one
  nested run made the shell that holds the lock lost its Wayland connection 4 s later, exited 255 and was
  relaunched; it logged `lock-stranded: recovering` and `secure=true` 2.2 s after. The founder accepted this price
  (inputs#I13). The headless form, which opens no window on the desktop, aborts at start on this build
  (`CBackend::create() failed!`, twice).
- The desktop lock is read right after the nested start and at the end of the live work: the shell's answer and
  `hyprctl locked` on the desktop instance, both recorded; an unlocked reading stops the chunk at once and goes to
  the operator (inputs#I13).
- Measured in the same run: the child of the nested probe window saw a terminal of 26 columns by 14 rows (the fake
  agent's `size` receipt), on an output of 621x688 at the compositor's automatic scale. A live `claude` session needs
  a usable terminal; the plan sets the output's scale, the gaps and the font, and reads the size under the fake
  agent before any live start.
- "whether such a key moves the wheel is unmeasured" (inputs#I3). `[premise-corrected: research M1 — P3 could not
  take the measurement. In four guarded runs over the fake agent, keyboard focus never left the overseer's window,
  with DPMS on and the probe window mapped in the last one; no key was typed in any]` From the code a key that
  reaches the terminal is an editing byte and moves the wheel (`src/run/wheel.rs:306-313`, `:360-369`); that a
  compositor key reaches the window is the unmeasured half. On the operator's word (inputs#I6) the fake-agent key
  probe is implement's step 0, before any live start, with DPMS on as a precondition of the probe and of the live
  run. `[premise-corrected: evidence/key-probe.md, inputs#I12 — the DPMS precondition belonged to the desktop
  compositor, which this chunk no longer dispatches to; the own compositor's output has no power state to set, and
  the guard reads the own instance]` If the probe cannot be made to pass, the chunk stops there and the takeover
  returns to the founder; it is never replaced by a programmatic stand-in for the keystroke.
- A compositor key lands in whichever window holds the focus, on a desktop other sessions share. Verified (research
  M1): the guard, the active window's address read immediately before `wtype`, stopped every run that would have
  typed elsewhere. The guard is the operator's (inputs#I4): type only into the window this chunk opened, and never
  focus or type into the `viola.viola-builder` or `overseer.viola-overseer` windows. A key into any other window is
  a defect of the run, not a retry.
- The builder is a `viola run` child on `claude` 2.1.287 named by path, in a home this chunk stamps (W4).
- The driving goes through the CLI verbs the slice has landed; no MCP server ships (`plugin/.mcp.json` is empty and
  there is no `mcp` verb, research M5). Step by step: `send` with the text on stdin, `wait --after` the send's
  cursor, `last` for the final text, a second `send` for the review answer, `send` of `/clear` between skills.
- "Answers a review" is a typed reply, a `send`. `[premise-corrected: research M4 — the brief's pass record reads
  "the answer to *Ready to continue?* arrived" (refs/viola-brief.md:290-295), and its R7 puts review answers on the
  keystroke side and dialogs on the hook side (:109-110)]` The dialog a driver answers by `dialog_id` is W3's
  reading, in the same session.
- Who plays the overseer in the run is not stated by the entry, the intent or the brief (research M4 read both). A
  product verb run from the implementing session reports `from: viola-builder`, because the prototype's wrapper sets
  `VIOLA_NAME` in its environment (research M2). It is P4's to settle.
- The takeover is read on the product's own records: a `wheel` record with holder `human` and cause `human-input`,
  then a driver `send` refused `human-typing`, exit 10. No driver-facing line suggests `release`.
- The wheel's return is not part of `v1-33`'s acceptance. Verified (research M3): from any session carrying
  `VIOLA_NAME` a `viola release` is refused `release-from-driver`, exit 20; a `release` with `VIOLA_NAME` dropped
  moves the wheel, and run by an agent that is the step-around the security extract names. This chunk does neither
  unless the founder rules it: the takeover is the run's last step and the wheel is left with the human.
- "The product binary" is the build `scripts/release-check.sh` makes and judges: `cargo build --release --locked
  --bin viola`, last line `release-check: viola only` (research M8). A build carrying `fake-agent` or `test-support`
  is not the product.

### W2 — self-drive: the overseer drops the prototype (the entry's own line; `v1-33`'s last clause)
- After the live test the overseer drives its builder through viola instead of the prototype.
- Measured at this take-up (2026-10-08, `ps -eo pid,comm,args`): every builder on this host, this
  session's own (`viola-builder`) included, runs under the prototype binary,
  `../additional/viola-lab/prototype/target/debug/viola run <name> --dialog-timeout 3500 -- claude --plugin-dir
  ../additional/viola-lab/prototype/plugin …`. The overseer's own procedure for driving is
  `../additional/viola-overseer/driving-guide.md`. Both are outside this repository.
- The switch itself is made outside this repository, by the overseer and the founder: the start line and three
  scripts that name the prototype (`vstart.sh`, `waitm.sh`, `clear-worker.sh`) and the guide (inputs#I5).
- Verified (research M9, 25 items read): five things the overseer's procedure relies on have no counterpart in the
  product at HEAD, so the switch cannot be made as the guide stands: `run --dialog-timeout` with a dialog held until
  the overseer answers (the product hands a dialog to the human after a compiled 60 s); `list` before every send
  (route entry `working-route.md:115`); the raw screen read for CLI modals and the context reading; `allow <n>`; and
  the scripts above. What "the overseer switches" means for this chunk is a founder question at P4, and it decides
  whether `v1-33` is fully provable here. No gap is closed by a guess.
- The switch replaces the wrapper under a running builder session, and the session that implements this chunk is
  itself a prototype-wrapped builder (research M2). When a switch happens relative to this chunk's implement and
  wrap rides the same founder question.

### W3 — a decided dialog never renders (CARRY 3; the clause `v1-31` still owes)
- "the decision effect's second half — a dialog a driver decided never renders on the real screen — is measured
  live here". The decision taking effect was the re-probe of chunk 2026-10-05-dialog-rows-and-re-probe; this is the
  screen half.
- It needs a verified home: `cli_verified` is decided once at `run`'s start from the strict stamps read, and no
  decision flows without it (research M8). It is read in the same stamped 2.1.287 session as W1.
- `[premise-corrected: research M11 — `run` never reads screen content and `Screen::rows()` is `verify`'s alone; the
  tests extract bars a verdict from parsed screen content]` "Never renders" is read on the records: `dialog-raised`,
  `dialog-answered` and `hook-decision` with `decision_emitted:true` and `deadline_hit:false` on one `dialog_id`,
  then the turn's `turn-ended` with no `wheel` record and no key typed. With nobody at the desk a rendered dialog
  would hold the turn. This is a reading of the effect, not of the pixels.
- The reading is taken for each of the three kinds `v1-31`'s acceptance names (the operator's review at P5,
  inputs#I8): a question, a permission answered `deny` for a command that changes nothing, and a plan, each raised
  on the driver's request in the live session, before the takeover, and answered by its id. If the live CLI does
  not raise one of them, that is recorded and `v1-31` is left pooled with a dated note.

### W4 — `run --local-live`, fired on the dev host (CARRY 4)
- "fire `agent-run.sh run --local-live` on the dev host at seventeen rows … it passes when verify exits 0, each of
  the seventeen ids names one `  pass` step line, and the last line reads `stamped <version>  <n> pass  0 fail`".
  The row count stands at seventeen (`LEDGER_ROWS: [&str; 17]`, `crates/viola-e2e/src/harness/run.rs:333`); chunk
  2026-10-07-live-rows-and-paste-shapes-on-the-dev-host landed no row.
- Verified at HEAD (research M6): `local_live` runs `viola --home <home> verify` with no program and the harness
  has no pass-through, so a bare `run --local-live` reaches PATH `claude`, 2.1.289, which stays unstamped by R-L3.
  Fired as written it would stamp the wrong version: the entry's words and the ruling pull apart, a founder question
  for P4, never a lean. Passed alone it runs no other suite, builds the harness `viola` (with `fake-agent`) and
  stamps `target/e2e-home/viola-live-<pid>/home`, which nothing removes.
- Verified (research M7): one `viola verify` is a `--version` read and five `claude` sessions (the print probe and
  four PTY runs). The home this chunk stamps for W1 and a separate `--local-live` firing are two verifies, ten
  starts, and pass the cap with the builder's one. See Sizing.

### W5 — live readings earlier chunks left here (CARRY 6, CARRY 7)
- A `send` issued under the real CLI's paste hint: "Delivery after the gate's wait is measured under the fake
  agent's hold only (that chunk's `evidence/hint-red-green.md`), so the 8.5 s bound against the real 8.0 s hint is
  this entry's to read on a live session."
- "Unmeasured on a live CLI and this entry's to read: a `send` whose text ends in newlines is confirmed and the
  driver keeps the wheel; a text of only newlines, typed as an empty text; a trailing CR; and `/clear` followed by a
  newline, which is now classified as `/clear` (unit-tier only; the founder was shown this consequence)."
- These are turns of the W1 builder session and cost no further start (each is a `send` into the running session;
  `/clear` followed by a newline starts a new CLI session inside the same process). A reading that falsifies a
  compiled behaviour is a product defect; whether its fix lands here or is carried is P4's, and the reading is
  recorded either way.
- This entry stamps its own home and leans on the earlier by-path home
  (`target/e2e-home/viola-reverify-20261007T124408Z/`) for nothing.

### W6 — the Linux live confirmation (CARRY 1, its last clause)
- "this run also stands as the Linux live confirmation, one live viola run that checks the fake agent's Unix
  fidelity" (the Epoch 7 entry of that name was retired at the 2026-10-07 0-pending wrap as a consequence of R-L1).
- `[premise-corrected: research M12 — the retired entry is not in route-archive.md; it is kept verbatim in
  .andromeda/runs/2026-10-07T05-47-07-wrap/adaptation-record.md:53-56 and names no comparison]` The comparisons are
  the plan's to name. Research lists five the live run can take from its own records at no further start: the first
  three event lines, the `claude-child` start line (`pty_backend`, the stripped and kept names), the wrapper's
  `endpoint_kind`, the live child's open fd numbers, and whether `session-end` lands before the child is gone.
- The live run on the Linux host comes before "Unix endpoint and home hardening" (Epoch 7) lands. That is the
  route's order already; nothing to do here.

### W7 — the `input-not-ready` hint line (inputs#I1; the handoff's open founder item)
- The line reads `{name} was not ready for input; viola wait {name}, then send again` (`src/human.rs:215-216`).
- Measured on live 2.1.287 (chunk 2026-10-07-live-rows-and-paste-shapes-on-the-dev-host,
  `evidence/hint-window.md:12`, `:83`, `:98`): a `viola wait` with no cursor, issued in the hint window, was woken
  by no record and returned `timed_out` at its own deadline.
- `[premise-corrected: research M10 — the wait running out is one of five causes]` The refusal arises when no child
  is attached yet, when the screen model is poisoned, when the screen was never quiet within 8.5 s, and on a
  verified CLI when a modal stands or when the screen is quiet with no input box at 8.5 s. In none of them is a turn
  running, so a cursor-less `viola wait` has nothing to wake on; a later `send` is the only thing a driver can do,
  and it fails the same way while a modal or a poisoned model stands.
- The wording is the founder's: P4 brings it as one question, each candidate line shown as it would print, with what
  each one tells a driver to do. No wording is chosen before his answer.

### W8 — the wheel's closed list on a real terminal (the second revision; inputs#I15, inputs#I16)
- The closed non-editing list of `src/run/wheel.rs` (F-W2, the founder's ruling of 2026-10-04) gains the seven
  reply shapes measured on foot 1.28.0 (`evidence/reply-probe.ndjson`), each by its exact grammar and no wider:
  `CSI 0 n`; `CSI ? 997;1 n` and `CSI ? 997;2 n`; `CSI 4;h;w t`; `CSI 6;h;w t`; `CSI 8;rows;cols t`;
  `CSI 48;rows;cols;h;w t`; `CSI > 4;n m`. Verified at HEAD: none of them is on the list
  (`is_reply`, `src/run/wheel.rs:496-503`; the probe read each as typing).
- Each shape has a red-green case (red on the tree before the list changes, green after) and negative controls:
  the nearest human keys still move the wheel, and a sequence one field or one prefix away from the shape is still
  typing. F-W2 holds as the operator restated it: a reply the terminal writes by itself is not a human key, and
  no human key may be read as a reply.
- "Research first reads which queries claude 2.1.287 sends at its start, from the recorded fixtures … (hypothesis)"
  (inputs#I16). `[premise-corrected: .andromeda/runs/2026-10-08T07-43-15-phase/cli-reply-parser.md — the fixtures
  hold no query bytes (0 files under fixtures/ carry an ESC; the screen files are signature rows). The CLI's own
  reply parser, read from its binary, names ten response types; on foot two of them are read as typing today,
  CSI ? 997;1 n and CSI 6;h;w t, both among the seven. The one type the first probe had not sent, the cursor
  position report CSI ? r;c R, draws no answer from foot (measured 07:47Z)]` Which queries the CLI sends in its
  first 237 ms, as bytes, stays unmeasured: that needs a live start.
- The fix is proved three ways before start 7 is spent: the unit cases; the reply probe re-run on the fixed product
  build, on the standing compositor, with no live start and no key (the seven read as not typing, the controls
  unchanged); and a key probe on the fixed build (a typed key still takes the wheel).
- The live run is then retried in a fresh session, start 7. If start 7 also loses the wheel before any key, it is
  closed at once and the chunk returns to the operator; start 8 is never spent on a repeat.
- The own compositor stands from the one start of 07:26:12Z. The bound of inputs#I15 holds: no live start by
  10:32Z ends it.

## Not this chunk's (inputs#I1)
Two CARRY blocks on the entry are written for the Epoch 3 boundary audit that follows it. On the operator's word
they are the audit's:
- CARRY 2 (chunk 2026-10-04-windows-boundary-mutation-workflow): the 25 Windows-dispatch survivors and the
  `#[cfg(unix)]` twins `windows-mutants.yml` grades missed (coordinates in that chunk's
  `evidence/windows-dispatch.md`).
- CARRY 5 (chunk 2026-10-07-test-homes-off-the-contended-volume): whether a copied tree carries the
  `target/e2e-home` link, and the `backing/` directory a keeper mutant leaves (that chunk's
  `evidence/keeper-control.md`).
Nothing in this chunk classifies a survivor, runs a mutation pass or answers the copied-tree question. Both blocks
must reach the audit whole: the plan names them for the wrap, so the flip does not archive them as spent.

## Observations
- watch: a raw coverage profile refused at the ubuntu merge, `no profile can be merged`; one mechanism was measured
  and removed, and the writer on ci#37627485806 attempt 1 is not provable from the run (0/3; since
  2026-10-07-a-send-ending-in-a-newline-is-confirmed). The count is the wrap's to move.

## Premises carried by the freight (closed at P3, `research.md`)
- 2.1.287 is installed and answers by path. Verified at this take-up:
  `~/.local/share/mise/installs/claude/2.1.287/claude --version` prints `2.1.287 (Claude Code)`.
- PATH `claude` is 2.1.289. Verified at this take-up: `claude --version` prints `2.1.289 (Claude Code)`, PATH
  resolves to `~/.local/share/mise/installs/claude/latest/claude`.
- The dated note on `v1-33` says what the directive says it says. Verified: `matrix.py show --id v1-33`, the note of
  2026-10-07 quoted under Authority.
- `v1-31`'s notes leave "the dialog never renders" to this entry. Verified: its three notes of 2026-10-04 name
  `:90`'s live test, the line this entry held then.
- `LEDGER_ROWS` holds seventeen ids. Verified: `crates/viola-e2e/src/harness/run.rs:333`; the suite is `local_live`
  at `:357`.
- The gate's bound is 8.5 s. Verified: `GATE_MAX_WAIT` is 8500 ms at `crates/viola-agent-claude/src/screen.rs:16`,
  `QUIET_PERIOD` 300 ms at `:10`.
- `send` types the text without its trailing LF characters. Verified: `typed_text` at
  `crates/viola-agent-claude/src/hook.rs:202`.
- The earlier by-path home is on a tmpfs. Verified at this take-up: `target/e2e-home` is a link, its backing reads
  `tmpfs`, and `viola-reverify-20261007T124408Z` is its only entry.
- The evidence files the freight cites exist: `windows-dispatch.md`, `keeper-control.md`, `hint-red-green.md`,
  `ci-attempt-1.md`, `profraw-red-green.md`, each in its chunk's `evidence/`.
- "`wtype` and `hyprctl` are installed on this host" (inputs#I3). Verified at this take-up: `command -v` resolves
  both under `/usr/bin`.
- "the real 8.0 s hint". Spot-checked at P3: chunk 2026-10-07-live-rows-and-paste-shapes-on-the-dev-host's
  `evidence/hint-card-answer.md` records nine timings of 8.000 to 8.023 s, and its `evidence/hint-window.md:83` the
  cursor-less `wait` that returned `timed_out` at 20.017 s.

## Boundaries
- No live `claude` start beyond 8 in the whole chunk; a round that would pass the cap stops and returns to the
  founder.
- Only `claude` 2.1.287 by path is driven and stamped; PATH `claude` gets no stamp and no fixture set.
- The `v1-33` claim re-words the acceptance only, in the two places the founder ruled: the host (R-L1) and the
  typing (inputs#I3). Its title and `requirements.md:48` are not edited.
- A compositor key is sent only into this chunk's own live session window, after its focus is read back. No key is
  sent into another session's window, and no stand-in (a write to the PTY, a channel frame, a test seam) is called
  the typed key.
- The desktop lock is not touched: nobody unlocks the session, no key goes to the desktop compositor, and no
  dispatch is sent to the desktop instance (inputs#I12). The only call that names the desktop instance is the
  `locked` read (inputs#I13). The own compositor is started once; a second start needs the operator's word.
- No byte is typed by viola into any CLI-native dialog; a start that shows a modal is killed with no key (the
  standing `verify` rulings). A dialog is answered only through the product's own `answer` path.
- `viola release` is the founder's verb. Nothing this chunk writes for a driver suggests it.
- No token, launch URL, cookie or `CLAUDE*` value reaches a log, an evidence file or a fixture; names only.
- viola writes nothing under `~/.claude`; the CLI's own transcripts of this chunk's live sessions are the accepted
  residual class.
- Nothing under `../additional/` is edited by this chunk: the prototype tree, the overseer's guide and its start
  scripts are the overseer's and the founder's (research M9 names the files a switch would change).
- The three Windows-only live items stay with "Windows-only live measurements" (`working-route.md:144`).
- The operator desk's items (the handoff's list) are not removed.

## Sizing (P3 measures, P4 decides)
- The cap is 8 live starts. Verified (research M7): a `viola verify` is 5, the builder session is 1. A separate
  `run --local-live` firing is 5 more, 11 in all, which passes the cap; how it is fired is the founder's at P4.
- A red verify round costs another 5 on any path, so no path leaves room for one. The plan states what each step
  spends, and a red round stops the chunk and returns to the founder.
- The key probe over the fake agent (step 0) spends no live start. It runs inside the one start of the own
  compositor, which also carries the live run and the readings session (inputs#I13).
- Done by the stopped implement run and standing (inputs#I12): the hint line (W7), the standing gates and the
  product build (`evidence/hint-pins-first.md`; the gate trail of `.andromeda/runs/2026-10-08T06-28-55-implement/`).
- Spent by the second implement run (`evidence/live-sessions.ndjson`): 6 of 8 live starts (the round's five, and
  start 6, which lost the wheel at its start). Left: start 7 for the live run's retry and start 8 for the readings
  session. No spare: a start 7 that fails the same way is not repeated (W8).
- Done by the second implement run and standing: step 0 (the own compositor's one start and the key probe), the
  round (`2.1.287`, 17 pass, 0 fail; its home is `H`), the fidelity readings of start 6.
- The work is one live run and its evidence, plus whatever product change the hint wording (W7) or a falsified
  reading (W5) brings. The five gaps that stop the switch (W2) are other route entries' work or the overseer's own;
  none is sized into this chunk unless the founder says so.

## CI read at Setup (what shipped since the last flip)
- `0dafa09a2a6f` — green · checks 15/15 · wall 434 s · ci#37727524960.
- `2861e19517bd` — green · checks 15/15 · wall 539 s · ci#37676147892.
No red, nothing to disposition.

## Capabilities
The entry names two.
- `v1-33` (CARRY 1) is NOT claimed by this chunk: its last clause, the overseer driving through viola instead of
  the prototype, is not made here (inputs#I7). The chunk measures the rest of its acceptance. The re-wording the
  founder ruled (the Linux dev host, R-L1; a key typed in the session terminal, inputs#I3) is written by the chunk
  that claims it; a dated note records both rulings and this one.
- `v1-31` (CARRY 3) is claimed: its last clause is measured here, and the clauses landed earlier stay proven by
  their tests.
