# Founder forks — phase of «Key after a resize» (2026-11-21-key-after-a-resize), the P3 block

**This block holds forks of yours: K1 (whether to measure, and how), K2 (the start cap), K3 (the keys and the
widenings), K4 (what «verified» rests on, and what a LOST reading means for v2-27).** The phase holds here, before P4.
Nothing is planned past this block until your word; no live start was made, nothing under your real `~/.claude*` was
read, and the CLI bundles were read as strings only (inputs#I1, the phase directive).

Written 2026-11-22T00:05:19Z. Run `.andromeda/runs/2026-11-21T23-32-36Z-phase/`; scope `doc:2026-11-21-key-after-a-resize/scope`
rev 3; research `doc:2026-11-21-key-after-a-resize/research` rev 1.

## 1. The window

One take-up window: P1 (promotion), P2 (7 task writers · 156 picker draws in 3 parts · 7 history agents, every
check passed first time, 957 of 4788 rows picked, no master at `picked 0`), P3 (this research). The plan (P4), its
review (P5) and the implement run each fit one window after your word; the live round (at most 3 starts) fits inside
implement's window.

## 2. What can be told with NO start

- **Where the claim comes from — a Windows hypothesis, never a Linux fact.** Chunk `2026-09-29-h2-conpty-resize-probe`
  measured 13 of 200 keys lost on the windows-2025 runner's inbox ConPTY, with a Rust test child reading by
  `ReadConsoleW` (ci#36527891850); a sideloaded ConPTY read 0 of 200 beside the inbox one's 14 of 200 in one run
  (chunk `2026-09-29-sideloaded-conpty`). The real `claude` was never read on any OS. The question was carried as an
  open measurement through «First live test and self-drive» → «Windows-only live measurements» → the `another-os`
  residuals → this entry (census C-288 / C-729, 2026-11-08); capability `v2-27`, its absorbed residual
  `res-wqnre2ex`.
- **On Linux, the kernel side is already held by a test.** `spawn_delivers_a_key_written_right_after_a_resize_to_a_child_not_yet_reading`
  (`crates/viola-pty/src/lib.rs:886`) writes a key right after `resize` returned, on openpty, and the Rust child reads
  it with the new size — green on CI's `ubuntu-latest` leg (ci#44960846431). A resize (TIOCSWINSZ) does not flush the
  tty's input. What stays open is the CLI's own handling above the kernel.
- **What viola does around a resize** (research «Files inspected»):
  - The pump POLLS the host size every `RESIZE_EVERY` (250 ms) and calls `pty.resize` on a change; there is no SIGWINCH
    handler. The same poll offers the size to the send gate first, which rebuilds its screen blank.
  - A human key goes straight to the child, never held (two architecture rows forbid it). A key typed into
    `viola run` right after a host resize therefore reaches the child BEFORE the child's own resize, by up to one poll.
  - A driver `send` whose gate wait spans the resize waits `QUIET_PERIOD` and, on a verified CLI, the redrawn input box
    (up to `GATE_MAX_WAIT`). One window stays open: a resize polled after the gate read `Ready` and before the paste's
    `write_all` (`src/run/send.rs:136` → `:166`).
  - A lost or cut paste is never presumed delivered: the confirmation needs the exact typed text (trailing whitespace
    tolerated) and reads `not-delivered` / `no-prompt-submitted` after `CONFIRM_WINDOW_FALLBACK`. But the paste's
    Enter is its last byte, so a paste cut at its head could still be submitted and run as a turn before viola reports
    it undelivered.
  - Tests holding it: the three `viola-pty` real-PTY resize tests, `tests/tui_pty_seam.rs` (2), `tests/tui_passthrough.rs`
    `resize_reaches_the_child`, `tests/tui_wheel.rs` `tui_focus_mouse_and_resize_never_take_the_wheel`, the gate's
    `overflow_poisons_…_until_a_resize`, the screen's resize cases. The fake agent keeps every byte through a resize and
    receipts the size by a watcher.
- **What the CLI does on a resize — a READING of the 2.1.328 and 2.1.330 strings, never a measurement**
  (`obs-plan · decision/claude-internals-not-a-contract`):
  - The same on both versions. A stdout `resize` handler re-reads the size, invalidates the inline renderer when the
    width changes or the height shrinks, and re-renders. A SIGWINCH handler re-probes cell pixels, and only when the
    terminal answered that probe at start (default off; nothing answers it in the rig's pty).
  - No stdin pause, flush or raw-mode toggle stands on either path (`stdin.pause()` once, on the bridge's SIGHUP shutdown;
    `tcflush` · `discardInput` · `flushInput` 0 times), and no debounce was found.
  - The strings show a redraw, no input-mode reset and no input flush. Whether the input box's state survives the
    re-render cannot be read from the minified strings.
- **Only a start can tell** whether the CLI's input box keeps a byte typed into it 0, 20 or 100 ms after a resize on
  Linux, on the versions you name.

## 3. The answer paths

**K1 — whether to measure, and how** (the recommendation first):

- **A (recommended) — a new unwrapped rig arm `KR` measuring the CLI itself.** The rig resizes its own pty and types
  `x` at set delays after it, reading kept / lost as closed words. It measures exactly the open part, the CLI's
  handling of a byte after a resize. viola's own forwarding is already held by tests, and no product file changes.
  *How it fails:* it does not measure viola's poll lag or the driver's paste. A reading of KEPT says «no loss in 12
  rounds at these delays on this version», never «never loses». The CLI may draw a kept key late (handled by a re-read
  before `lost`, below).
- **B — no start: close on the reading.** This records §2 as the answer, keeps `v2-27` unclaimed with a dated note,
  and re-words the gotchas / services OPEN items as «not measured; no mechanism read». *How it fails:* the entry and
  `v2-27` say «measured». That word is not met, so `v2-27` stays pooled or is deferred on your word. Zero starts.
- **C — a wrapped arm through `viola run`.** The rig resizes viola's outer pty, and viola forwards the resize within its
  poll. *How it fails:* the key reaches the child BEFORE viola forwards the resize in most rounds, so it mostly
  measures nothing «after» a resize. A typed `x` moves the wheel to `human` (`wheel{cause:human-input}`) in every round.
  It needs a viola home and stamp per version (the kept `HF` / `HG` / `XS` products, or a fresh verify per start: more
  starts).
- **Set aside — a 30th relied ledger row with a `viola verify` probe.** No send relies on the shape
  (`architecture · decision/no-probe-for-unrelied-shapes`). A probe would make verify type keys after a resize into
  every stamped CLI: a widening of verify's own key set, which today types pastes only.
- **Set aside — paste rounds (a bracketed paste after the resize).** This is a new key class for the rig's grant, and
  the driver's path already waits on the redraw in the gate (§2). The entry and `v2-27` name a key.

**«Fixed if it does» — priced now, chosen only AFTER the reading:**

- **F0 — nothing but the record** (reading KEPT on both versions): `v2-27` closes on the readings. The wrap re-words
  `gotchas.md` H2 «product window» and `services/viola-pty.md` OPEN. No product change.
- **F1 — the driver's paste re-gated across a resize** (reading LOST; recommended among the fixes). A size change
  polled between the gate's `Ready` and the paste sends the send back into the gate, the existing quiet period and box
  redraw, under the existing `GATE_MAX_WAIT`. It is decided under the flight lock, which a human key never takes.
  - Reaches `src/run/send.rs` and `src/run/gate.rs`, never `viola-pty`'s write; no new constant.
  - Rows moved: `architecture · decision/vt100-readiness-gate`, `decision/gate-timings-compiled` (unchanged values,
    re-read), `decision/send-flight-lock`; `a11y-plan · decision/time-bounds-fall-on-the-driver` /
    `decision/time-bounds-never-on-human` (no new bound, named as unchanged); `test-plan`'s readiness-gate rows (a new
    case).
  - Human keys stay unheld.
- **F2 — read-back and one re-type** — against `architecture · decision/unconfirmed-delivery-not-delivered` («never
  retried blindly»). A re-type after a paste that did arrive submits it twice. It is also a typed character class the
  received text did not hold (security-history, `2026-10-09-inner-cr-and-crlf-in-a-sent-text`). Not recommended.
- **F3 — a refusal**: a send arriving within a set time after a resize is refused `input-not-ready` with a new detail
  word. It widens the closed detail / exit-cause set (security-history, `2026-10-31-exit-cause-code-catalogue`: each new
  word your own) and adds a constant the driver must wait out. Weaker than F1.
- **A human key lost inside the CLI cannot be fixed by viola** without holding keys behind a resize, which two
  architecture rows and `a11y-plan · decision/human-keystrokes-never-blocked` forbid: see K4.

**The paste carry `cry-7mgt7ybs`** is not taken: no path above opens the paste's `write_all` (A, B and C change no
product file; F1 / F3 wait or refuse before the paste). It re-pins whole at the wrap. `cry-m267ghbx` re-pins whole (no
`viola-pty` edit). `cry-lqe3bfwx` reads met in `1b22388f` (chunk 2026-11-17-standing-live-rig): the wrap retires it.
`cry-teayoana` is the boundary's. `cry-234cbfoa` is acted on only at its next red.

## 4. Every reached master row (searched in all seven rows files and their histories)

**Reached on the recommended path A** (each a widening said here, whole):

- `security-plan · decision/live-round-rig-keys-own-pty` — **WIDENED**. The grant «one x and one DEL in a ready box»
  grows to a resize of the rig's own pty plus up to 13 `x` and 13 DEL per start, each `x` into a box read empty,
  each DEL right after its `x`'s reading. Still only into its own pty, never a dialog, the stop needing no word.
- `security-plan · decision/live-rig-hb-arm-keeps-attributes-only` — **a sibling row added** for `KR` («types `x` /
  DEL after its own resize into its own unwrapped pty before any send, keeps only cell attributes, counts and
  delays»). The HB row's name is kept; the new reading is added beside it, never folded into a wider class.
- `a11y-plan · decision/no-screen-content-parsing` — **its except gains `KR` by name**: box-row cell attributes after a
  resize, closed words, never text.
- `test-plan · decision/no-child-screen-assertions` — the same except, `KR` named.
- `test-plan · decision/key-after-resize-waits-for-size` — **its except gains the `KR` rehearsal** beside the standing
  `hold` sibling: the rehearsal over the fake agent types right after its resize, on purpose.
- `test-plan · decision/standing-live-rig` — holds (a new arm of the one rig, its rehearsal in CI, its live starts
  ledgered, refused under CI); its arm list grows by `KR`.
- `test-plan · decision/live-failing-reading-not-verification`, `decision/rig-no-key-read-from-keys-word`;
  `security-plan · decision/live-session-screens-kept-local`, `decision/live-rig-isolated-child`,
  `decision/live-rig-credentials-copy`, `decision/live-rig-seed-from-bundle-logic` — hold unchanged; the arm runs
  inside them.
- `obs-plan · decision/claude-internals-not-a-contract` — holds: §2's CLI facts are readings.

**Reached only on the other paths:**

- C: `a11y-plan · decision/wheel-ignores-terminal-events`, `test-plan · decision/focus-mouse-resize-keep-wheel`,
  `decision/human-takes-wheel-on-edit` — a typed `x` moves the wheel.
- F1: the rows under F1 above.
- F2: `architecture · decision/unconfirmed-delivery-not-delivered`, `security-plan · decision/typed-text-line-ending-normalisation`.
- F3: `layout-templates · contract/viola-send-output/unable`, `contract/cli-exit-codes/*`, the closed detail set.
- Ledger row: `architecture · decision/capability-ledger-gate`, `layout-templates · contract/viola-verify-output/step-line`.

**Read as NOT reached by A:**

- `architecture · decision/human-key-never-held-behind-resize`, `decision/no-key-held-behind-resize`,
  `decision/human-always-wins-wheel`.
- `test-plan · decision/pty-tested-on-real-openpty`, `decision/resize-baseline-is-spawn-size`.
- `obs-plan · decision/vt100-feed-panic-contained`, `decision/claude-boundary-only-instrumentation`.
- Every `design-system` and `layout-templates` row: no output, word or exit changes.

**No cell is past its bound by the additions above.**

## 5. Every live start's SHAPE (path A)

**The labels and the cap.** `KR` (claude 2.1.330), `KR8` (claude 2.1.328), spare `KR-2` (2.1.330 again, or the version
whose start read `confounded`). One fresh ledger file, `{work_dir}/evidence/kr-ledger.ndjson` (gitignored), `--cap 2
--spare 1`, `--cap` the running total of base starts in that file.

**Before each start, from no start** (gate entries placed immediately before each start's entry, so a round resumed at
any start re-fires them):

1. `python3 ~/dev/projects/additional/andromedaV3/andromeda-overseer/docs/tools/hostwatch.py read --last 15 --for viola`.
   It must read QUIET; a CONTENDED reading naming a neighbour stops the round and is re-read, never retried into.
2. `mise ls claude`, and `<install dir>/<version>/claude --version` by path, read equal to the start's version.
3. `viola-live-rig census` reads no process left by an earlier rig.
4. The sha256 of each kept product `HF`, `HG`, `XS`, `XS-2` (outside the tree, under the gitignored `target/live-rig/`)
   recorded. The same read after the round must be equal: KR reads, writes and removes none of them.
5. The build's hash of `viola-live-rig` read right before the start.

**PRESENT before a start:**
- the version's `claude` under `~/.local/share/mise/installs/claude/<version>/`;
- the four kept products;
- the credentials file the rig copies (its own token-window check refuses under 20 minutes left: uncounted, stop and
  say so).

**ABSENT before a start:**
- a rig process (census);
- this label's out dir;
- an earlier start's scratch.

**The processes.** `viola-live-rig` on the host, and one `claude <version>` as session leader of the rig's own pty,
which starts at 210×45 (the rig's size). Its cwd is a fresh scratch folder seeded trusted in a private
`CLAUDE_CONFIG_DIR` (HB's seed, from bundle logic; no status-line key). No viola process runs and no wheel exists,
since the start is unwrapped. The CLI makes its own start-time network calls as every earlier HB start did; no prompt
is sent, so no model turn runs.

**Every byte typed, and into what:**

1. **The first frame is read.** If the trust dialog, any modal, or no input box stands, no key is typed, the start
   reads `refused shape` and stops. It still counts.
2. **Control round (no resize).** Settle until the box reads up, then read the hint cells. Type one `x` (0x78) into the
   box read empty; settle; keep the frame; read the box row's first drawn cell's attribute (the *typed word*). Type one
   DEL (0x7f); settle; keep the frame; read the box empty with its hint word (the *hint word*). A control in which the
   typed and hint words are equal reads `confounded`: no round runs, and the start stops.
3. **12 rounds, each run in this order:**
   1. Check that the box reads empty with the hint word. If it does not, the round reads `dirty-box` and the start stops.
   2. Resize the rig's own pty, alternating 210×45 → 180×40 → 210×45 …, so the width changes every round; the rig's own
      screen model is rebuilt at the new size.
   3. Wait the round's delay: 0 ms (written as soon as `resize` returned), 20 ms or 100 ms, four rounds each,
      interleaved 0 · 20 · 100 · 0 · …
   4. Type one `x`, settle (the rig's `QUIET` 400 ms, bounded by `plan.settle`), keep the frame, and read the attribute:
      - the typed word → `kept`;
      - the hint word → re-read once after a further settle → still the hint word → `lost`;
      - anything else → `unread (<reason>)`.
   5. Type one DEL, settle, keep the frame.

   Per start: at most 13 `x` and 13 DEL, 12 resizes, no other byte.
4. **No dialog is raised, and none is answered.** A modal appearing mid-round makes the next key refuse
   (`key-not-ready`); the start reads `unread` and stops.
5. **The stop.** Ctrl-C into its own pty every `STOP_PRESS` until the child exits, its group SIGKILLed at
   `STOP_KILL_AFTER`. Ctrl-C needs no word, being the stop's own.

**Files written, and where:**
- under `target/live-rig/<run>/` (gitignored, local): each kept frame's rows in a 0600 file (at most 27 per start),
  `frames.ndjson` (one closed-word line per frame), the summary;
- the ledger: one line with the clock before the start, one closed-word summary after;
- the scratch config and the transient credentials copy, removed at teardown.

Nothing lands in the tree but the closed-word summary the plan's evidence quotes.

**Under which home.** Your real HOME is the process's home (as every HB start), with the private `CLAUDE_CONFIG_DIR`.

**What each start READS, and from where:**
- its own pty's output only, as closed words and cell attributes;
- the credentials file once, for its expiry (copied 0600, transient, unrecorded);
- your `~/.claude.json`, after a moved stamp only, searched for the rig's own needles, the bytes dropped and one closed
  word kept (`decision/live-rig-isolated-child`).

Those last two are the only reads under your real home, and both are the rig's standing behaviour.

**What the CLI itself puts on the screen.** That screen is kept only in the local 0600 frames, which never land. Under
the private config it shows:
- its own banner: version, model word and possibly a plan or account word;
- the scratch cwd's path, under your home directory;
- its footer hints.

It shows no status line (the seed sets none). The summary holds none of it: attribute words, counts and delays only.

**Whether a typed key moves the wheel.** No: there is no wheel in an unwrapped start. (On C it would, in every round.)

**The predicate each start can FAIL.** The start's summary line `kr-kept 12/12` with no `unread` round:
- **red: any `lost`.** It is recorded as `lost k/12` with the delays it fell on, and goes to the fix fork;
- `confounded` or more than 3 `unread` rounds: the start reads unverified on that version and the spare may be spent
  (`decision/live-failing-reading-not-verification`).

The rehearsal over the fake agent (in CI) must read `kept 12/12`: the fake agent keeps every byte, so a red there is
the rig's own fault.

**Versions measured.** 2.1.330 (the host's `latest`; `KR`) and 2.1.328 (the stamped fixture set's; `KR8`).

## 6. The capability, and what «verified» would rest on

`v2-27` today: «Whether a key typed right after a terminal resize reaches real claude on Linux is measured and
recorded; if it is lost, a fix makes it arrive and a test holds it.»

**Proposed concrete form (path A):** «On Linux, real `claude` 2.1.330 and 2.1.328 each run unwrapped in the live rig's
own pty under its scratch home. After each of 12 resizes alternating 210×45 ↔ 180×40, one `x` typed 0, 20 or 100 ms
after the resize returned reads `kept` (the box row's attribute word equal to the no-resize control's typed word) in
12 of 12 rounds per version; the readings are recorded as closed words in the chunk's evidence, and the arm's
rehearsal over the fake agent holds the mechanics in CI. Any `lost` reading goes to the fix fork.»

«Verified» rests on the two live summaries (KR, KR8), read as KEPT, plus the rehearsal's CI PASS line.

## 7. The forks (your word)

- **K1 — the path.** A (recommended) · B · C (§3).
- **K2 — the start cap.** Recommended: **2 base starts + 1 spare** (KR on 2.1.330, KR8 on 2.1.328; the spare for a start
  read `refused shape`, `confounded`, or cut by the token window or a host stall). A start refused before its ledger
  line is uncounted.
- **K3 — the keys and the widenings.** Up to 13 `x` and 13 DEL per start, plus 12 resizes of the rig's own pty, into a
  box read empty, never into a dialog. The widenings named in §4, all of them: the rig grant row widened, a sibling
  row beside the HB row, `KR` named in two screen-reading excepts and in the key-after-resize test except.
- **K4 — «verified», and a LOST reading.**
  - (a) Accept §6's concrete form as what «verified» rests on.
  - (b) If any round reads `lost`, `v2-27`'s «a fix makes it arrive» cannot hold for a HUMAN key without holding keys
    behind a resize, which the architecture forbids. **Recommended:** concretize the fix half to the driver's paste
    (F1), with the human-key loss recorded as the CLI's, carried as a finding. The alternative is to defer `v2-27`'s fix
    half on your word.
  - (c) If both read KEPT: F0, `v2-27` closes on the readings.

**Not yours, decided here as technical:** the delays (0 / 20 / 100 ms), the sizes (210×45 ↔ 180×40), the round count
(12) and the re-read-before-`lost` control. The overseer may change any of them by relay.

## 8. One implement window?

Yes. The arm is one function in `crates/viola-e2e/src/rig/mod.rs` beside `arm_hb`, plus:
- its grant in `keys.rs` and the model's resize in `screen.rs`;
- its argv word in `bin/viola-live-rig.rs`;
- its rehearsal in `crates/viola-e2e/tests/rig_rehearsal.rs`.

The live round is at most 3 starts of about 2 minutes each. F1, if the reading calls for it, is a second, small change
the plan would add only after the reading, on your word.

## 9. The answers (appended 2026-11-22T08:07:39Z, verbatim from the overseer's relay `../additional/viola-overseer/relay-resize-p3.md`, snapped at P4)

### The operator's own words — given in one dialog and read at 2026-11-22T08:07:34Z

1. **K1, the path: «Рука рига KR, без обёртки (Recommended)».** Your (A): a new unwrapped rig arm `KR` — a real
   `claude` in the rig's own pty under a scratch folder and a private config, 12 rounds of a resize of its own
   pty (210×45 ↔ 180×40) then one `x` 0, 20 or 100 ms after it, kept / lost read from cell attributes as closed
   words, frames kept locally 0600 and never landed, no prompt sent, no product file changed. He was shown that
   it does not measure viola's own poll lag or the driver's paste, and that KEPT means «no loss in 12 rounds at
   these delays», never «never». (B) and (C) are declined.
2. **K2, the cap: «2 базовых + 1 запасной (Recommended)».** `KR` on 2.1.330 and `KR8` on 2.1.328, one spare, in
   the shape shown to him: the process's home is his real HOME (as every earlier unwrapped rig start) with the
   private `CLAUDE_CONFIG_DIR`; his credentials file read once per start by the rig (a 0600 copy, removed); his
   `~/.claude.json` searched for the rig's own needles only after a moved stamp; nothing written under his real
   `~/.claude*` or `~/.viola`; the stop is the rig's Ctrl-C into its own pty. The spare: a start that read a
   wrong screen (`refused shape`), `confounded`, or was cut by the token window or a host stall — never a red
   reading (a `lost`). A third base start is his.
3. **K3, the keys and the widenings: «Да, как описано (Recommended)».** Up to 13 `x` and 13 DEL per start plus 12
   resizes of the rig's own pty, each `x` only into a box read empty, never into a dialog, a modal stops the
   start. The five rows of your §4 widen as named, each by NAME, none folded into a class. This is his
   ratification of those widenings and nothing wider.
4. **K4, «verified» and a LOST reading: «Починка только вставки драйвера (Recommended)».** §6's concrete form is
   what «verified» rests on. Both starts KEPT → F0: v2-27 closes on the readings, no product change. Any `lost`
   → the fix half is concretized to the DRIVER'S PASTE (F1: a resize polled between the gate's ready and the
   paste sends the send back into the gate; no new constant; human keys never held), the human-key loss recorded
   as the CLI's and carried as a finding. **F1 is planned only AFTER the reading and comes back to him:** on a
   `lost`, implement stops with the summary's closed words, and the fix is a plan revision.

### Technical answers (the overseer's)

5. The delays, sizes, round count and the re-read before `lost` stand as you chose them. The starts are fired by
   YOU in implement's round, `leg = 'live'` entries, `--cap` the running total (1, 2), `--spare 1`.
6. **The frames stay local and the summary holds no screen text:** the evidence line is written from typed
   fields (closed words, counts, delays); say on the card what a kept frame can hold of his (the banner's account
   or plan word, the scratch cwd's path under his home) and that no frame is read into a message, a log or the
   tree. Hygiene's read of the evidence dir is an entry.
7. **A `lost` is a red that is NOT retried and NOT a spare's:** the round stops after the start that read it,
   the other version's start still runs only if the plan orders the two starts independent — say which, and why
   (my lean: both run, since each version's reading is its own; the stop comes after the round).
8. The carries as your block disposes them: the paste carry and `cry-m267ghbx` re-pin whole at the wrap,
   `cry-lqe3bfwx` retires as met in `1b22388f` (name the test or code line that meets it), `cry-teayoana` is
   the boundary's, the rehearsal red only at its next red.
9. **The card, as before:** control ids 1–16 characters, each control on an entry filtered to its guard's own
   cases; every after-read of state OTHER actors move keyed to the round leg; the round's span holds round and
   live entries only and the whole-round dry-run shown; the kept products' sha (derive the set from
   `ls target/live-rig/`) and the live home's stamp sha taken right before the round and read after. A row NOT
   in your §4 that P4 finds reached is a fork in `founder-forks.md`.
10. **The seam after P5.** When the plan is approved the phase ENDS and nothing else starts. On an account-limit
    refusal, stop where you are.

**Authority:** items 1–4 the operator's own answers to K1–K4 (his words, relayed verbatim by the overseer); items
5–10 the overseer's technical answers.
