# Founder forks — 2026-11-19-context-as-a-local-command (phase run 2026-11-19T19-12-52Z-phase, P3 end)

The phase stopped at the end of P3, before P4, as the directive (`inputs#I1` §3) asks. Nothing below is decided; every
recommendation is marked. Each fact cites the code at `afa8699c` or the chunk's research (`doc:2026-11-19-context-as-a-local-command/research`).

## The one block

### 1. What the new row is
- **Name:** `local-command-context` (`LedgerRow::LocalCommandContext`), its words «/context writes its stdout tag to
  the transcript and submits no prompt»; appended last, after `browser-tools-prompt` (the 25th row).
- **Probe:** Run B's eighth compiled paste, the literal `/context`, typed only into the settled input box right after
  the `/mcp disable viola-probe-absent` echo paste and before `/compact`, waited up to the probe deadline.
- **Post-condition read:** the count of `<local-command-stdout>` tags in Run B's own session transcript past its end
  before the paste (`echo::tags_since`: no symlink, owner's regular file, `MAX_FRAME`, the count alone), with no new
  UserPromptSubmit capture. `pass` = at least one tag, no prompt, inside the confirm window.
- **What it means for a version:** a probe that sees no tag stamps `fail` (only the browser-tools row may read
  `absent`). The row is **RELIED**, because `relied()` is every row (`ledger.rs:149-151`). So:
  - every binary built after it reads every stamped version (2.1.328 included) as **unverified** until that version
    is re-stamped with 25 rows;
  - a version whose `/context` writes no tag reads unverified **whole**, and its sends go transport-only.

### 2. The send reading
- **Today:** `/context` is unlisted, so it is sent as a prompt and waits for a `prompt-submitted` that never comes.
  That is `L2`'s `not-delivered:no-prompt-submitted`, and the listing is what mends it.
- **After the change, on a verified CLI with a readable transcript:**
  - one new tag → confirmed;
  - no tag by the window's end → still `not-delivered/no-prompt-submitted`.
- **After the change, read `ok` `confirmed:false` `unconfirmable` at once:**
  - on an unverified CLI, which includes every 24-row stamp;
  - with no transcript remembered;
  - with a transcript declared but not yet on disk. This is `L2`'s case: `context-transcript: absent` as the
    session's first act.
- So `L2`'s exact start would read `unconfirmable` after the change, never confirmed (`src/run/send.rs:188-199`).
  Fork F2 asks about that case.

### 3. «Stamped live»: what it needs and what stays true
- **A `viola verify` against the real claude 2.1.328.** Read from code and from `L1`'s report, it runs 9 `claude`
  processes:
  - one `--version`;
  - one print-mode `-p` probe on haiku;
  - Runs A–F, six interactive PTY children;
  - Run B's one `claude agents --json` read.
- **Every byte typed is a compiled literal of Run B (and Runs C/D/E/F as today):**
  - the probe prompt;
  - the long, tag-like and newline pastes;
  - `/mcp disable viola-probe-absent`;
  - **`/context` (new)**;
  - `/compact`;
  - `/clear`.

  Verify types no key into any dialog.
- **Files written**, under the home the start runs in:
  - its `ledger/stamps.json`, where the 2.1.328 entry is **replaced whole**;
  - its verify trust record, written and removed;
  - the CLI's own Run B/C/D transcripts and the print-mode project dir, which are the accepted residuals;
  - one more `/context` line in Run B's transcript, plus whatever the CLI writes for `/context` itself (unmeasured).
- **What stays true of the running pinned product and this builder:**
  - The pinned binary (`a59da9da`'s release) is not rebuilt, replaced or re-pinned by this chunk.
  - It keeps reading 24 rows. A 25-row stamp of 2.1.328 still reads verified to it **only if all 24 of its rows
    pass again in that one run**.
  - So a stamp written into the operator's real `~/.viola` puts the pinned product's next start at risk: one row
    failing turns it transport-only (`merge_stamp` replaces the entry). A stamp written into a scratch home touches
    nothing the pinned product reads.

### 4. Window and split
- **Recommended: no entry split, one implement window.** That window does:
  - the product change (a list entry, a variant, a probe constant, one Run B paste, the fake's framing echo, and the
    `24` → `25` pins in 11 files);
  - its fake-agent rehearsals in CI;
  - the rig's `CX` change.
- The live starts are the operator leg of the same chunk. This matches the `/compact` and `/mcp` precedents, each one
  chunk.
- The **real-home re-stamp stays with the re-pin**, its own operator step (F1). That step is the moment the running
  product moves to a 25-row binary, and a 25-row stamp of the live home is only needed then.
- The alternative split is (a) the product change and rehearsals, then (b) a later entry for the live stamp. It
  costs a second phase and buys nothing a leg doesn't give, unless F1 picks the real home. Then (b) should ride the
  re-pin step.

### 5. Every live start's shape (before the count)
- **S1 — the stamp** (rig arm `VR`: verify of the rig's own scratch viola home, its record kept; recommended under F1-A).
  - **Process:** the rig, the debug `viola` of this chunk's tree, and 9 `claude` (§3).
  - **Typed:** only verify's compiled literals (§3). The rig types no key, and no wheel exists (verify drives its
    own children).
  - **Stop:** verify's own exit, or the rig's no-key group kill.
  - **Writes:**
    - in the rig's own scratch dir, the rig's private Claude config:
      - the credentials copy, 0600, removed on exit;
      - the seeded `.claude.json`;
      - Run B/C/D transcripts;
    - the scratch viola home's `ledger/stamps.json`;
    - the kept record and frames under `target/live-rig/S1/record-2.1.328/` (gitignored).
  - **Under the real home**, by the rig's standing rows: one read of the credentials file (token window plus copy)
    and the `~/.claude.json` stamps before and after. Nothing is written there.
  - **Predicate that can fail:** `local-command-context: pass` and `failing: none` at 25/25. The red reading is
    `local-command-context: fail`, or any other row `fail`.
  - **Reads:**
    - row words;
    - hook payload key names and counts, as verify does today;
    - the transcript's tag count;
    - first-screen closed words.

    No text. **No new kind of reading.** The new *paste* is the widening (F3).
- **S2 — the `/context` send and the hint cells** (rig arm `CX`, changed; recommended under F4-A).
  - **Process:** the rig, the debug `viola run` (wrapped), and 1 `claude`.
  - **Before the start:** S1's kept 25-row stamp is copied into CX's scratch home, so the CLI reads verified (F4
    names this).
  - **Sends and keys:**
    - **send 1**, a compiled prompt (`reply with the single word ok`), so the transcript exists;
    - **send 2**, `/context`;
    - only after send 2's reading, the hint keys (F5): `x`, then DEL.
  - **Wheel:** the sends are driver sends, and no key is typed before them, so **no key moves the wheel before a
    send**. The keys after them move the wheel to the human, and no send follows.
  - **Stop:** the rig's Ctrl-C with the box read.
  - **Writes:**
    - the scratch viola home's instance dir (events, snapshot, diagnostics);
    - the private config's transcript;
    - frames `send-1`, `send-2`, `after-x` and `after-del` under `target/live-rig/S2/` (gitignored).
  - **Under the real home:** the same standing credentials read and stamps as S1.
  - **Predicates that can fail:**
    - `send-2-reply: ok-confirmed`; red is `unconfirmable` or `not-delivered:*`;
    - `context-submitted: no`; red is `yes`;
    - `context-tags` ≥ 1; red is `0`.
  - **Reads:**
    - event kinds;
    - `prompt-submitted`'s text compared to the compiled literal (as `CX` does today);
    - the reply's closed words;
    - the tag count;
    - **cell attribute words of every drawn hint cell past the cursor's cell (cells 2..N), then the typed cell's
      word.** Never a text.

    The hint cells past the first are **a reading no row names yet**. The standing rig row says «the box row's cell
    attributes», and `SF` read one cell. **This is a widening, said here.**
- **Neither start reads or writes anything under the operator's real `~/.viola`** under the recommendations.

### 6. Cap and spare
- **Recommended cap: 2 base starts (S1, S2), plus 1 spare.** The spare re-fires a start that the host, the token
  window or the rig's own refusal stopped before its reading. It is never a re-try of a red reading.

### 7. Hint-reading keys (C3)
- **Recommended: 2 keys, `x` then DEL, typed only after S2's send-2 reading,** with the box settled:
  - the hint cells 2..N are read before any key;
  - `x` gives the typed cell (cell 1, the cursor then past it);
  - DEL restores the box.
- **The alternative is 4 keys, `x`, `x`, DEL, DEL,** to read two typed cells against hint cells 2 and 3 position by
  position.

### 8. No-start gate entries and after-reads (per capped start)
- **S1:**
  - before, no-start reads:
    - the rig scratch for its label is absent;
    - `target/live-rig/S1/record-2.1.328/` is absent;
    - `~/.viola/ledger/stamps.json`'s sha is recorded (to show it unmoved after);
    - the debug `viola` hash, read right before the start;
    - `claude --version` by path reads 2.1.328;
    - `df` headroom;
    - `hostwatch.py read --last 15` is QUIET;
  - after:
    - the kept record holds a 25-row 2.1.328 entry;
    - `~/.viola/ledger/stamps.json`'s sha is unmoved;
    - the census reads 0.
- **S2:**
  - before, no-start reads:
    - S1's kept stamp is present;
    - CX's scratch is absent;
    - `target/live-rig/S2/` is absent;
    - the same hash, version, `df` and host reads as S1;
  - after:
    - the kept frames are present;
    - `~/.viola/ledger/stamps.json`'s sha is unmoved;
    - the census reads 0.
- Each entry sits ahead of its own start, so a round resumed at that start still fires it.

## The forks (his; the recommendation marked)

- **F1 — which home S1 stamps.**
  - **A (Recommended):** the rig's scratch viola home (`VR`). Cost: the live home stays at 24 rows until the re-pin
    step re-stamps it. That is harmless to the pinned product, which reads 24.
  - **B:** the real `~/.viola` through `VH`. Cost: a read of the real config's key names and a write of the live
    stamps. If any of the 24 rows fails in that one run, the pinned product goes transport-only on its next start.
- **F2 — a `/context` sent before the session's transcript exists** (`L2`'s case).
  - **A (Recommended):** keep today's rule. It reads `ok` `confirmed:false` `unconfirmable`, nothing presumed. Cost:
    a first-act `/context` never reads confirmed.
  - **B:** count from offset 0 of a declared transcript not yet on disk. Cost: it rewords
    `decision/transcript-read-bounded` («from the recorded pre-send offset»), which is a security-plan widening. A
    tag the human's own concurrent `/context` writes would then confirm the driver's send.
- **F3 — Run B's eighth paste, `/context`.** Approving it widens the ratified verify child set (security history:
  each added Run B paste was his own word, the `/compact` and `/mcp` ones the precedents) and adds its residual: one
  transcript line, plus whatever `/context` writes, unmeasured.
  - **A (Recommended):** approve it, with the residual measured at S1 and named.
  - **B:** no probe. Cost: no ledger row, so the entry cannot be built as written.
- **F4 — the `CX` start (S2).**
  - **A (Recommended):** S2 as shaped above (a prompt turn first, the stamp copied in, then `/context`).
  - **B:** no S2. Cost: the product send is proven only over the fake agent, and C3 rides no start and stays open.
- **F5 — the hint-reading keys and the cell widening (C3).**
  - **A (Recommended):** 2 keys, `x` then DEL after send 2, with the hint cells 2..N read as attribute words.
  - **B:** 4 keys, two typed cells.
  - **C:** no keys. C3 stays open, re-pinned.
- **F6 — the cap.**
  - **A (Recommended):** 2 + 1 spare.
  - **B:** 1 (S1 alone) + 1 spare, which pairs with F4-B.

The phase stops here, before P4. P4 plans on his answers. C4 (the intermittent rehearsal red) needs no word: it is
acted on only at its next red, its printed summary and `after-del` frame read first.

## The answers (`inputs#I2`, the relay read and snapped at P4)

**The founder's own words**, given in a dialog and read at 2026-11-19T19:50:29Z, relayed verbatim by the overseer:
- **F3:** «Разрешить пробу (Recommended)».
  - Run B gains its eighth compiled paste, `/context`.
  - `local-command-context` is the 25th relied row.
  - He was told the consequence: every later binary reads every stamped version unverified until it is re-stamped
    with 25 rows, and the pinned product is untouched until its next re-pin.
  - The residual is measured at S1 and named in the report.
- **F1:** «Scratch-home стенда (Recommended)».
  - S1 stamps the rig's scratch viola home under the rig's private config.
  - Under the real home, only the standing reads: `~/.claude/.credentials.json` once (a 0600 copy, removed) and
    `~/.claude.json` stamped before and after.
  - `~/.viola/ledger/stamps.json` is neither read for content nor written; its sha before and after is the witness.
  - The live-home re-stamp stays with the re-pin step.
- **F6 with F4:** «2 + 1 запасной (Recommended)». Two base starts plus one spare:
  - S1: one verify, 9 `claude` one at a time.
  - S2: one wrapped `claude`, a compiled prompt send first, then `/context`, with S1's kept stamp copied in first.
  - The spare only for a start stopped before its reading by the host, the token window or the rig's own refusal,
    never to repeat a red.
  - claude 2.1.328 by path. A third base start is his.
- **F5:** «2 клавиши: x, DEL (Recommended)».
  - On S2, after send 2's reading, the hint's drawn cells past the cursor's cell are read as attribute words before
    any key; then `x`, the typed cell's word, then DEL.
  - That reading is the widening he ratified, in exactly that width.

**The overseer's technical answers:**
- **F2: A.** Today's rule stays: a first-act `/context` reads `ok` `confirmed:false` `unconfirmable`. It is
  documented in the plan, with a fake-agent rehearsal reading exactly those words.
- **Split:** no entry split; one implement window, with the live starts in the operator leg.
- **S2's hint words are readings, never a red.** `unread` with its reason is recorded. S2's stopping reds are:
  - `send-2-reply` not confirmed;
  - `context-submitted: yes`;
  - a tag count of 0;
  - the standing teardown atoms.
- **A version whose `/context` writes no tag** stamps `fail` and reads unverified whole. Say it in the plan and in
  the row's words; no `absent` word.
- **Fixtures and the `STAMPED` literal:** say whether they move, and keep them green without a recording start, or
  write a fork.
  - **Answered at P4 from code:** the `STAMPED` list (`tests/contract_ledger_probes.rs:44`) and the recorded sets do
    not move.
  - The suite runs `viola verify` against the fake with `--framing --transcript`.
  - The fake's framing echo writes the tag into its own transcript and reads no fixture
    (`src/bin/viola-fake-agent.rs:784-786`).
  - So a framing echo for the `/context` probe stamps every set 25/25 with no new recording.
  - `ROW_IDS` and the `24 pass` literal move to 25. No fork.
- **Figures:** no leaf gains a count; the pins move in code and tests only.
- **After P5:** the approved plan ends the phase. No timer tool. On an account-limit refusal, stop where you are.
