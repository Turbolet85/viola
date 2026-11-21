# Founder forks — «Harness-prefix and identity-floor ledger rows» (2026-11-20-harness-prefix-and-identity-floor-ledger-rows), end of P3

Written 2026-11-20 at P3's end, before P4, under the overseer's directive (`inputs#I1` (3)). The chunk is promoted
(pending, tx 3536); scope rev 2 and research rev 2 stand in the store (`doc:2026-11-20-harness-prefix-and-identity-floor-ledger-rows/scope`
· `/research`). No live start was made, nothing under `~/.claude*` was read, no plan is written. Every fact below
cites research (`research#…`) or the code at `b3eb184c`; «unmeasured» marks what only a live start can tell.

## 0. The window

This window ran Setup, P1, P2 (156 pickers, 7 task writers, 7 history agents) and P3; its fill is not measurable from
inside it. It holds P4 and P5 on one answer; `/andromeda-implement` runs in a fresh window. The size question is
item 7 below.

## 1. What each row relies on today without a row, and what breaks on a new CLI

- **Harness prefixes.** `prompt_origin` (`crates/viola-agent-claude/src/hook.rs:266`, its one product caller
  `data_of` at hook.rs:203) files a UserPromptSubmit prompt `harness` when its raw start is one of four compiled
  literals (hook.rs:135-140); the wheel moves only on a `human` prompt (`src/run/send/slot.rs:219`,
  `decision/harness-prompts-not-human`). Measured once, on 2.1.287 (the live-shape cases at hook.rs:1123-1136).
  **When the assumption fails** (a CLI that renames, escapes or re-wraps the cross-session envelope): a message
  another session sends into a wrapped session is filed `human`, so the wheel moves to the human. The driver's next
  send is then refused as if a person had typed, until a `release`. The person is never blocked, and no prompt is
  lost. The reverse failure (a typed prompt filed `harness`) is the accepted side effect already ratified.
  **Measured now, statically, no start** (research, host reads): both installed bundles carry the CLI's own table of
  16 harness tag names. viola's four literals cover three of them (`task-notification`, `agent-message`,
  `cross-session-message`); `teammate-message`, `tick`, `remote-review`, `coordinator-relay`, two slack tags and
  five artifact tags among others are not covered. Whether any of those arrives as a UserPromptSubmit prompt is
  unmeasured.
- **Identity floor.** `plan_strip` (`crates/viola-agent-claude/src/lib.rs:61-80`) removes every inherited `CLAUDE*`
  name from the child unless `claude_env_keep` keeps it. It always removes the eleven floor names
  (`IDENTITY_FLOOR`, lib.rs:17-29), which were measured once on a wrapped host in 2026-09.
  - **What the product actually relies on:** every name by which a CLI hands its own session identity to a child
    process begins with `CLAUDE`, so the prefix rule catches it. **When that fails:** an identity name without the
    prefix passes into a wrapped child. A `viola run` started from inside a Claude session would then hand that
    child the parent's identity, for example its messaging socket.
  - **The floor is a narrower reliance:** a new identity name with the prefix, kept by a user's `claude_env_keep`,
    reaches the child because it is not on the floor.
  - **Measured now, no start:** this session's own Bash child (claude 2.1.328, `inputs#I1` Facts) carries eleven
    `CLAUDE*` names (names read, never values; research). One of them, **`CLAUDE_CODE_CHILD_NOTES_DIR`, is on no
    list**, and the floor's `CLAUDE_CODE_BRIDGE_SESSION_ID` is absent. What a hook child of a verify child carries
    is unmeasured.

## 2. «A second session sending into a verify child» — the shape

**What the bundles show** (2.1.328, static, research):
- A session listens on its own local socket, `$XDG_RUNTIME_DIR/cc-socks/<pid>.sock`, or on a path given by
  `--messaging-socket-path`.
- It publishes an auth key into «the session registry directory» of the Claude config it reads.
- A peer injects a message with one auth line holding the token, then one `{"type":"user",…}` line. An
  unauthenticated line closes the connection.
- The message's `from` is «sender-asserted, a label, not an identity».
- The token (`CLAUDE_CODE_MESSAGING_TOKEN`) is on the CLI's own secret list.

No viola endpoint, channel, paste or hook reaches the child on this path. Run B is unwrapped: verify holds no wheel,
so no wheel moves in any path below.

**Paths, the recommended first:**

- **(A) Recommended — a second CLI session in print mode, a model turn, the CLI's own send.**
  - **Process:** verify spawns one more direct child under the R8 strip, once Run B's box has settled after its
    framing pastes. The child is `claude -p <compiled prompt> --model haiku --no-session-persistence`, its one
    allowed tool the CLI's cross-session send tool (a compiled flag literal), its cwd verify's existing print-probe
    dir.
  - **What it sends:** the compiled prompt names Run B's session id, read from Run B's first SessionStart capture as
    `resume_target` already does, as the only target, and one compiled synthetic text.
  - **What the child shows:** Run B receives it as a harness turn. Its UserPromptSubmit is captured by the existing
    capture arm.
  - **What is read:** only the raw start of that captured prompt, against the four compiled literals. One closed
    word is kept: the literal it matched, or `none`. No prompt text and no `from` value is kept.
  - **Model turns:** two — the sender's, and Run B's reply.
  - **Time added:** two short turns, roughly 10-30 s, unmeasured.
  - **Config residual:** no new transcript (`--no-session-persistence`), no new project dir (it reuses the
    print-probe's cwd). Whether the sender session publishes a registry entry is unmeasured.
  - **How it fails:** the model can refuse, or send to the wrong peer. In a user's real config the registry lists
    that user's other live sessions, so a misaddressed send would put the probe text into one of the user's own
    sessions. The compiled prompt naming one session id lowers this risk; it does not close it. The send tool's
    name and its addressing in print mode are unmeasured.
- **(B) Set aside — verify writes the frame itself on the socket lane.**
  - **Process:** verify starts Run B with `--messaging-socket-path` inside its own 0700 probe dir. It reads the auth
    key the child published into the Claude config's session registry, then writes one compiled message itself.
  - **Gains:** deterministic, no model turn on the sender, no misaddressing.
  - **How it fails:** verify reads a credential of the CLI's that viola does not own, a crossing of the NEVER-log
    floor's boundary. The key's file and form are unmeasured. The sender is viola, not «a second session», which
    departs from the entry's own words.
- **(C) Set aside — verify hands the child a token of its own.**
  - **Process:** verify sets a fresh random `CLAUDE_CODE_MESSAGING_TOKEN` in Run B's environment, then writes the
    frame with it.
  - **How it fails:** it puts a floor name into a verify child against R8's «always stripped» (a widening), and
    whether the CLI adopts a given token is unmeasured. The sender is again viola, not a session.

**Recommendation:** (A). It is the entry's own wording and touches no secret. It needs the founder's word on its
misaddressing risk in a user's real config: a widening of verify's child set, and a residual to accept or refuse.

## 3. The env capture — the private-outward edge

**Recommended path (E1) — the capture arm records names.**
- **Who captures:** the existing hidden `hook --capture` arm, fired by Run B's SessionStart. It writes one more
  0600 file, `Env.<k>.json`, into the same 0700 probe dir.
- **What the file holds:** the hook process's environment NAMES only. The names are read from `std::env::vars_os()`
  keys, sorted, the values dropped before any write. Only names matching `^CLAUDE[A-Z0-9_]*$` are kept, and every
  other name is counted only.
- **How verify reads it:** in memory, removing its own kept names (`claude_env_keep`) first.
- **Where it lands:**
  - the stamp's `measured` block for that version, as a sorted list of names;
  - under `--record`, one `Env.recorded-1.json` through the shared scrub. A name holding the user word, a home path
    or an email shape refuses the whole recording, naming the file and its closed code, never the content;
  - no log line, no event, no screen read.
- **No value can be captured, proven by:** a test modelled on `tests/tui_env_strip.rs` that sets a canary VALUE in
  every floor name and one unknown name, runs the capture arm, and walks the captures dir, stamps and recording for
  the canary bytes. It fails if one is found. A second case keeps a user-named name through `claude_env_keep` and
  shows it is removed before anything is recorded.
- **Predicate, recommended (P1):** the row passes when the capture was read, and every `CLAUDE*` name the CLI added
  is either on the floor, or on a compiled list of non-identity hook names (`CLAUDE_PLUGIN_ROOT`,
  `CLAUDE_PROJECT_DIR`, `CLAUDE_ENV_FILE`, as `decision/identity-floor-eleven-names`' why names them). An unknown
  added name fails the row.
  - **Consequence:** if a hook child carries `CLAUDE_CODE_CHILD_NOTES_DIR` (unmeasured), 2.1.328 stamps the row
    `fail` and reads unverified, which means transport only, until it is classified. Classifying it as identity
    makes the floor twelve names, against «the floor stays eleven names». That is the founder's ruling, at the first
    live reading.
  - **(P2) set aside:** pass whenever the capture was read, and record the names for review. It never fails on a new
    name, so it measures nothing it could refuse — a ledger row whose post-condition cannot fail.
  - **(P3) set aside:** all names the CLI added, `CLAUDE*` or not, which tests the prefix reliance itself. It needs
    the arm to read every name, user-named ones included, into the probe dir. Non-`CLAUDE` names the CLI adds are
    unmeasured, so it may fail spuriously.

**Set aside (E2) — a new hidden arm or a separate probe child:** it widens the child set again for the same
reading.

## 4. Rows, versions, stamps, the fixture set

- **Count:** 2 relied rows, 27 → 29. Proposed ids `cross-session-prefix` and `identity-floor`, the words compiled.
- **Effect on stamps:** every stamped entry becomes short of two ids and reads unverified until re-verified
  (`verified` reads every relied id, `ledger.rs:170-173`, `test_cases_stamps.rs:978`). That covers the rig's kept
  27-row 2.1.328 stamp, the live home's 24-row stamp (already short), and any user's stamp. With `verify_new_cli`
  on, a start meeting a short entry runs the automatic verify once (`src/run/auto_verify.rs:385`), now longer by
  path (A)'s two turns. That falls in `decision/human-keystrokes-never-blocked`' pre-child window: a key waits in
  the terminal's buffer, never on viola.
- **The fixture set:** the one stamped set, `fixtures/claude/2.1.328/`, needs the two new payload classes for the
  fake agent's CI replay (`decision/fake-agent-matches-verify-fixtures`). That means one live
  `viola verify --record` of 2.1.328, re-recorded whole as the last chunk was, or synthetic payloads beside the
  recorded set, which the founder refused for the 27th row.
- **«Stamped live» means, recommended:** 2.1.328 re-recorded whole by one live `viola verify --record` in the rig's
  scratch home. 2.1.329, read verified 27/27 once in a scratch home now gone (`inputs#I1`), is offered as one
  optional stamp-only start, no record. The default version constants stay 2.1.328. The live home's re-stamp stays
  the re-pin step's.

## 5. Every master row the change reaches (all seven searched; the picked rows read for behaviour)

**Reached — a widening, said here whole:**
- **The verify child set.**
  - Adds one print-mode sender child and one more Run B turn, path (A).
  - Rows: `architecture · decision/verify-child-set` · `security-plan · decision/verify-children-direct-spawn`.
- **The capture arm.**
  - Now reads its process's env names and writes a second file.
  - Rows: `security-plan · decision/hook-capture-arm` · `decision/capture-arms-isolated` ·
    `decision/probe-capture-arms-inert` · `architecture · contract/hook-capture-arm/payload-file` · `test-plan ·
    decision/new-env-read-needs-log-entry` (a Decisions Log entry owed).
- **Residuals.**
  - Run B's extra turn and the sender session, plus any registry entry it publishes.
  - Rows: `architecture · decision/verify-transcripts-accepted-residual` · `security-plan ·
    decision/verify-real-cli-transcripts-accepted` · `decision/verify-trusted-runs-risk`.
  - The misaddressing risk of path (A) would be a new accepted residual, beside
    `decision/verify-echo-probe-residuals` and `decision/verify-context-probe-residual`.

**Reached — wording or shape follows:**
- **Stamps:** `architecture · contract/ledger-stamps/content` · `contract/ledger-stamps/rows` (new measured
  names); `decision/capability-ledger` (holds).
- **Fixtures and recording:**
  - `test-plan · decision/hook-payloads-recorded-by-verify` · `decision/fake-agent-matches-verify-fixtures` ·
    `decision/fake-agent-replay-for-stamps` · `decision/one-verify-stamp-per-build`;
  - `security-plan · decision/fixture-scrub-and-refusal` · `decision/recorder-scrub` ·
    `decision/verify-probe-captures-transient` (an env-names fixture through the scrub).
- **Env names:** `test-plan · decision/env-names-only`; `security-plan · decision/r8-claude-env-strip` (names
  only, now also read by the probe).
- **The obs plan:**
  - `obs-plan · contract/verify-spawn-subject/*` · `contract/diag-subject/*` (a new subject for the sender child) ·
    `contract/process-log/child-and-probe-lifecycle`;
  - `obs-plan · decision/identity-floor-always-stripped`: «beyond the floor there is no compile-time list of known
    names». Predicate (P1)'s compiled non-identity list contradicts it — a wording change the founder rules with
    (P1).
- **The live rig:** `test-plan · decision/standing-live-rig` (new live labels under the cap).
- **Time bounds:** `a11y-plan · decision/time-bounds-never-on-human` · `decision/time-bounds-fall-on-the-driver`
  (the sender's deadline, a new bound on viola's own child) · `decision/human-keystrokes-never-blocked`' except
  (the automatic-verify window lengthens).

**Reached only on a branch not recommended:**
- `architecture · decision/harness-prompt-prefixes` · `security-plan · decision/harness-prefix-literals` ·
  `test-plan · contract/prompt-submitted/origin-harness` — only if the literals widen to more of the CLI's 16
  tags (no part of this entry).
- `architecture / security-plan · decision/identity-floor-eleven-names` — only on a twelfth name.

**Read as not reached:**
- `decision/harness-prompts-not-human` · `decision/harness-turn-keeps-the-wheel` ·
  `decision/injected-turn-keeps-wheel` (no wheel in verify);
- `decision/r8-identity-floor` · `decision/child-env-claude-strip` · `decision/claude-env-stripped-from-child` ·
  `contract/config-json/claude-env-keep` (the strip is unchanged);
- `decision/compiled-classification-sources` (the probe reads compiled literals only);
- `decision/no-screen-content-parsing` and its four widenings (the probe reads a hook payload, never the screen);
- `decision/stamps-written-only-by-verify` · `decision/verify-record-only-clean` ·
  `decision/live-verify-never-in-ci` · `decision/real-claude-local-only`;
- `decision/env-vars-not-configuration` (names configure nothing);
- design-system and layout-templates' verify output rows (`contract/viola-verify-output/*`, the relied count `MM`
  generic: counts move, no text);
- `design-system · contract/cli-verification/*`;
- `decision/live-rig-prompt-capture-closed` (the rig reads nothing new).

## 6. The live starts — every shape before the count

There is one base start and two options. All run in the rig's scratch home: its private viola home and its private
Claude config, seeded from the ratified credentials copy (`decision/live-rig-credentials-copy`, its one read under
the real home). Nothing else is read or written under the founder's real home.

**Start 1 (base) — 2.1.328 `viola verify --record` in the scratch home.**
- **Label:** the rig's `S0R` shape, or a new label for this chunk.
- **Processes:** the rig; `viola verify`; its version probe; its print-mode probe; Runs A-F (six PTY children at
  80×24); Run B's one `agents --json` read; and path (A)'s one sender child. All are direct spawns under the R8
  strip.
- **Bytes typed:** no key into any dialog. Only Run B's compiled pastes go into a settled box, as today.
- **Dialogs:**
  - Runs C and D are answered only by the capture arm's closed probe ids, as today.
  - Run A's trust modal is killed with no key.
  - A browser-tools prompt is left standing.
  - The sender child raises none: print mode, one allowed tool.
- **The stop:** verify exits on its own. The rig's TERM is the bound.
- **Files written:**
  - in the scratch viola home: `ledger/stamps.json`, `ledger/probes/<pid>/` removed at the end;
  - in the scratch Claude config: Run B/C/D transcripts, the CLI's registry entries, the trust record;
  - the rig's kept record dir under `target/live-rig/`: the recorded set, through the scrub;
  - the rig's ledger file: one line, clock time first.
- **Reads:** step lines and the summary (closed words); the stamp's rows and the measured names (names, counts);
  the cross-session closed word (which literal matched, from a hook payload's raw start); the env names (names,
  from the capture file). No text and no value.
- **Wheel:** nothing moves it; there is no wheel in verify.
- **Predicate that can fail:** `[28/29] cross-session-prefix … pass`, `[29/29] identity-floor … pass` and
  `stamped 2.1.328  29 pass  0 fail`, with the recording written (exit 0).
- **Red readings:** `fail` on either new row (the closed word `none`, or an unknown name); an unclean-recording
  refusal with its closed code; a stop.

**Start 2 (option) — 2.1.329 `viola verify`, no record, scratch home.** The same shape, `--record` absent, no
recording written. It reads the same closed words. Predicate: `stamped 2.1.329  29 pass  0 fail`; red: `fail` on a
row.

**Spare (one):** for a start the rig refuses before its ledger line (the token window under 20 minutes, a CONTENDED
host read with a neighbour named), or one stopped by host cause. It is never spent on a red reading, which stays a
finding.

**Recommended cap:** 1 base + 1 spare, and +1 only if the founder takes Start 2.

**Absent or present on disk before each start** (each with a no-start gate entry right before it, placed so a
resumed round still fires it):
- present: the 2.1.328 install dir first on the rig's `PATH`, `claude --version` read by path (for Start 2, the
  2.1.329 dir);
- present: the credentials copy the rig seeds;
- absent: a scratch home left from an earlier rig;
- absent: any `target/live-rig/` record dir for this label;
- present: the hard link's build hash read right before the start;
- read: `hostwatch.py read --last 15` QUIET.
- After Start 1, an entry reads the kept recording and stamp (the round leg).

## 7. One window, or a split

**Recommended: split.**
- **This chunk keeps the identity-floor row (v2-26).** One probe through an arm that exists, one recorded fixture
  class, deterministic, no new child. Its one open premise, a hook child's names, is read by Start 1 itself.
- **The harness-prefix row (v2-25) becomes its own entry next.**
  - It carries path (A)'s three unmeasured premises: the CLI's send tool and its addressing in print mode, a
    cross-session prompt arriving as a UserPromptSubmit on 2.1.328/329, and registry visibility in a private
    config.
  - It adds a new child and a new residual.
  - Each of the last two chunks cost a second founder round on its live leg; two mechanisms in one window double
    that exposure.
- **Set aside — both in one window:** it holds only if the founder answers items 2-6 whole now. A red on the sender
  child would then hold the identity-floor row's stamp hostage, since one verify stamps both.

## 8. The capability claims and what «verified» rests on

- **v2-26 (this chunk, on the split):** verify's `identity-floor` row reads `pass` on claude 2.1.328 in the rig's
  scratch home.
  - **What it means:** every `CLAUDE*` name the CLI adds to its hook child is on the floor or the compiled
    non-identity list; the stamp lists those names; no value reaches any capture, log, stamp or fixture.
  - **«Verified» rests on:** Start 1's step line and the stamp it keeps; the canary-value test green in CI; the
    fake agent replaying the recorded `Env` fixture in CI.
- **v2-25 (its own entry on the split):** verify's `cross-session-prefix` row reads `pass` on 2.1.328: a second
  session's one send into Run B arrives as a UserPromptSubmit whose raw start matches a compiled harness literal.
  «Verified» rests on that start's step line and stamp, and on the recorded payload replayed in CI.
- A stand-in, a stub or a synthetic payload verifies neither.

## The forks — the founder's word

1. **The split** — (a) **split: this chunk the identity-floor row, the harness-prefix row its own next entry
   (Recommended)** — cost: one more phase and implement cycle later. (b) Both here — cost: items 2-6 all answered
   now, and one red holds both stamps.
2. **The second session's path** (now on (b), else at the next entry) — (a) **path (A), a print-mode CLI sender
   with a model turn, its misaddressing risk in a real config accepted as a residual (Recommended)** — cost: the
   new residual and two turns per verify. (b) Path (B), verify reads the CLI's published auth key — cost: a
   credential crossing, and the entry's words change. (c) Path (C), a token of verify's own in Run B's env — cost:
   an R8 floor widening, its adoption unmeasured.
3. **The env capture** — (a) **E1, through the capture arm, `CLAUDE*` names only, others counted, kept names removed
   in memory (Recommended)** — cost: a widening of the ratified capture arm and a Decisions Log entry. (b) E2, a
   new hidden arm or probe child — cost: a child-set widening.
4. **The predicate** — (a) **P1, an unknown added name fails the row, with a compiled non-identity list and the
   obs row reworded (Recommended)** — cost: 2.1.328 may read unverified until a floor ruling on a name such as
   `CLAUDE_CODE_CHILD_NOTES_DIR`. (b) P2, names recorded, never failing — cost: a row that cannot fail. (c) P3,
   every added name, any prefix — cost: user-named names read into the probe dir, and a spurious fail possible.
5. **Versions and recording** — (a) **2.1.328 re-recorded whole by one live `viola verify --record` (Recommended)**
   — cost: one start. (b) That, plus a 2.1.329 stamp-only start — cost: one more start. (c) Synthetic payloads
   beside the recorded set — refused for the 27th row; asked again.
6. **The cap** — (a) **1 base + 1 spare (Recommended)** — cost: per item 6. (b) 2 base + 1 spare with 5 (b).
7. **What «verified» rests on** — (a) **as item 8 (Recommended)**.

The block waits for the founder: the cap, the bytes into any CLI dialog (none proposed), every widening above, what
«verified» rests on, and the one read under the real home that the rig's credentials copy makes. The phase stops
here, at P3's end.

## The answers (appended 2026-11-21T05:41:49Z at P4's resume; snapped `inputs#I2`)

The founder's own words, given in two overseer dialogs and read at 2026-11-21T05:41:11Z, relayed verbatim in
`inputs#I2` (authority: the founder's own word, relayed by the overseer as delegate of the transport only):

1. **Fork 1, the split — (b) «Обе строки в этом чанке».** NOT split: this chunk takes both rows, `identity-floor`
   (v2-26) and `cross-session-prefix` (v2-25), 27 → 29. Shown its cost: the second session's path decided now, and a
   red on the sender holding the identity row's stamp too.
2. **Fork 2 — (a) path (A), «claude-отправитель в print-режиме (Recommended)».** One more direct child of verify,
   `claude -p` on Haiku with no session persistence, its one allowed tool the CLI's own cross-session send, the
   compiled prompt naming Run B's session id as the only target; only the raw start of the captured prompt read
   against the four compiled literals, one closed word kept. Ratified: that widening of verify's child set and the
   misaddressing residual (in a real config, under the automatic verify too, no human), over the rows §5 names for
   it, and nothing wider. (B) and (C) declined: verify reads no CLI credential and sets no floor name.
3. **Fork 3 — (a) E1, «Да, через рукав захвата (Recommended)».** The capture arm reads its process's env NAMES, keeps
   `CLAUDE*` names only, counts the rest, drops every value before any write; the user's kept names removed in
   memory; one more 0600 file in the same probe dir; names land in the stamp's measured block and, under `--record`,
   in one fixture through the scrub; no log line, no screen read. Ratified over the rows §5 names, the canary-value
   test its proof.
4. **Fork 4 — (a) P1, «Неизвестное имя валит строку (Recommended)».** An unknown added `CLAUDE*` name fails the row;
   the compiled non-identity list (`CLAUDE_PLUGIN_ROOT`, `CLAUDE_PROJECT_DIR`, `CLAUDE_ENV_FILE`) stands; the obs row
   saying no such list exists is reworded. Told: a new name in a new CLI reads that version unverified until he
   classifies it and a build ships; `CLAUDE_CODE_CHILD_NOTES_DIR` may read red at the first live reading.
5. **Forks 5–7 — «2 базовых (второй условно) + 1 запасной (Recommended)».** 2.1.328 ALONE, re-recorded whole by a
   live `viola verify --record` in the rig's scratch home; no 2.1.329 start; no synthetic payload in the recorded
   set. Two base starts and one spare on one ledger: the first (`--cap 1`) records and first measures a hook child's
   names; the second (`--cap 2`) opens ONLY IF the first read red on the identity row for an unknown name ALONE,
   every other row green (the cross-session row among them) — the names then go to him by the overseer's dialog, he
   classifies each, the code is mended through the revision door, and the second start re-records. A red of any other
   cause (the cross-session row, another row, a scrub refusal) is a finding: implement stops, no start follows. The
   spare only for a start the rig refuses before its ledger line or a host CONTENDED by a neighbour. From his real
   home `~/.claude/.credentials.json` is READ once per start by the rig; nothing written under his real `~/.claude*`
   or `~/.viola`; no key and no byte into any CLI dialog. «Verified»: the live start's step line and stamp, the
   canary-value test green in CI, the fake agent replaying the recorded fixture in CI — v2-26 on the identity row and
   the `Env` fixture, v2-25 on the cross-session row and its recorded payload. A third base start is his.

The overseer's technical answers 6–11 (`inputs#I2`): the sender bounded and inert (own deadline, the send tool only,
no persistence, the print-probe's cwd; a refusal, a missing tool or an unknown target reads `fail` with a closed
word, no retry); the misaddressing narrowed in code (one id; the exact-id form if the tool takes one; a compiled
synthetic text that says it is a viola verify probe; no read of other sessions or the registry); the stamp's and
fixture's names are the CLI's own after the kept names are removed, the canary test also covering a user-word name
(the scrub refuses, file and closed code) and the non-`CLAUDE` count a number only; the order (fake-agent cases, the
canary test and every control green before the round; no-start entries right before each start); a row reached
outside §5 is a fork written here, not a fold; the phase ENDS at the approval.

## P4 — two rows reached outside §5, and what the bundle shows (written 2026-11-21T05:50:28Z)

P4 re-read the picked rows of all seven masters for what the sender PRINTS, WAITS on and WRITES and what the env
capture READS (`inputs#I2` item 10). Two consequences reach rows §5 does not name. Each is a fork for the founder.
The plan carries the recommended option, held behind that fork: no implement starts before his word.

**F-P4-1. The kept stamp's label list.** `(security-plan · decision/live-rig-kept-stamp-owner-only)` reads «The live
rig keeps a stamp from its CS, WS or PQ scratch home owner-only and no-clobber…». The start this chunk makes keeps
its stamp (what «verified» rests on, his item 5) under two new labels: `HF` (the base start) and `HG` (the
conditional second start). A spare is `HF-2`.
- (a) **The row names `HF` and `HG` beside CS, WS and PQ, its rule unchanged (Recommended).** Cost: one row
  amended at the wrap. The named exception grows by adding names beside the named ones (CLAUDE.md, Session
  Learnings: «A master row's named exception grows by adding the new reading beside the named ones»).
- (b) The start reuses the label `PQ` and its `question-stamp.json`. Cost: no row amended. The 27-row stamp
  standing there must be moved aside first, and the two chunks' kept stamps share one name.

**F-P4-2. A user setting that turns cross-session messages away reads every CLI version unverified.** The 2.1.328
bundle (static read, no start) holds a per-session setting, `crossSessionInbound`. Its values can turn inbound
cross-session messages away, or hold each one for the user's approval. Unset, a message is delivered only when the
sender's permission-mode class matches the receiver's; a mismatched sender's message is held for approval. Verify's
private-config start (the rig's) has it unset, and both sessions read the same settings, so the classes match. But a
user whose Claude settings refuse or hold cross-session messages gets `cross-session-prefix` `fail` on every
version. Under `(architecture · decision/capability-ledger)` and `(architecture ·
decision/unverified-cli-transport-only)`, every one of that user's versions then reads unverified: transport only,
no dialog answers, `unverified-cli` to drivers. Verify reads no setting and no sender text, so it cannot tell this
cause from a CLI that changed. If the message is held, a dialog may stand in Run B: verify types no key into it, and
Run B ends by a kill (plan, Run B's end).
- (c) **added at P5, see «Answer to the overseer's P5 question» point 4: Run B carries a compiled `--settings`
  `{"crossSessionInbound":"accept"}` for that one verify child, with (a)'s residual kept for what it cannot override
  (Recommended).** Cost:
  - one more argv widening of Run B;
  - an open inbound window for Run B's life;
  - managed, repo and invalid-value settings and the `receive` kill switch still fail the row.
  The row can still fail on a CLI that changed delivery.
- (a) **Accept it as a named residual beside the misaddressing residual: the row fails and the version reads
  unverified (recommended before the P5 answer; now the residual under (c)).** Cost: such a user can never stamp a version. The remedy is theirs: change the setting
  or keep the version unverified. P1's strictness holds: the row can fail on a CLI that changed delivery.
- (b) The row reads `absent` when the sender exits 0 and no message arrives, and `absent` counts, as it does on
  browser-tools. Cost: a CLI that drops or re-routes cross-session messages also reads `absent`, so the row cannot
  fail on the very change it exists to catch.

**Technical leans, the overseer's to strike** (`inputs#I2` items 6–7; no founder ruling moves):
- **T1, where the send goes in Run B.** Between the `/compact` paste and the `/clear` paste, not after `/clear`.
  `/compact` keeps the session id; `/clear` makes a new session. So at that point the id the compiled prompt names
  is still Run B's first SessionStart id, read as `resume_target` reads it (§2). Run B's agents read has just listed
  that id live. Every paste before it has gone in.
- **T2, Run B's end.** Run B ends by Ctrl-C only if the box guard reads the box up after the cross-session turn.
  Otherwise Run B ends by a kill with no key, and `/clear` is not pasted. This keeps «no byte into any dialog» if a
  held message raises one. Every other end of Run B is unchanged.
- **T3, the address.** The bundle's send tool is `SendMessage`, input `to` and `message`. Its exact address form is
  a `uds:` socket path, or a bare `/…/*.sock` path. A bare session id falls to the tool's `other` scheme, which
  resolves it by a lookup that is unmeasured. Kept: the founder's words, the session id. The socket-path form would
  have verify compute the CLI's socket path, an unmeasured assumption written into the prompt.
- **T4, the deferred tool.** The bundle marks `SendMessage` `shouldDefer`, so in print mode the model may first
  need the CLI's tool search to load it. Kept: one allowed tool, `SendMessage` (his words). A sender that cannot
  load the tool reads `no-prompt`, a red of the cross-session row: a finding, and implement stops.
- **T5, what the sender leaves.** Run with `--no-session-persistence`, it writes no transcript and, in the
  print-probe's cwd, no new project dir. Unmeasured: a session-registry entry, an input-history line, and a socket
  under `$XDG_RUNTIME_DIR/cc-socks/` while it runs. The residual row that carries them is
  `(security-plan · decision/verify-real-cli-transcripts-accepted)` and its architecture twin, plus the new
  misaddressing residual row (Expected amendments).

## Answer to the overseer's P5 question (written 2026-11-21T06:10:27Z)

The question came in `relay-harness-p5-ask.md`. It is answered from the tree and from the 2.1.328 and 2.1.329
bundles read as strings, with no start. Both bundles hold the same logic for each point below. No plan line was
changed.

**1. The values.** The closed set is `accept` · `hold` · `refuse` (the bundle's own list).
- `accept` delivers at once, with no hold and no mode-class check, from any local sender.
- `hold` parks each message for the user's approval. A held message expires, by default after 5 minutes.
- `refuse` turns the message away (`opt-out`).
- An unrecognised value in a settings file forces `hold` while it stands. In managed settings it is read as
  `refuse`.

So yes: **`accept` accepts an inbound message from any local sender without holding it.**
- The setting is read only after one CLI-internal gate: when the CLI's `receive` gate is off, every message is
  refused (`kill-switch`).
- That gate's source is not readable from strings. It is unmeasured.

**2. Where it is read, and what wins.** The resolver reads two groups of settings sources in order.
- **First defined wins:** `policySettings` (managed), then `flagSettings` (a `--settings` argument), then
  `userSettings`.
- **Then the repo may only tighten:** `localSettings` and `projectSettings` can replace the value only with a
  stricter one (`hold` > `accept`, `refuse` > `hold`). The bundle's messages say «a repo may only tighten, so your
  own "accept" cannot override it». The same messages say managed policy cannot be overridden.
- **Last:** an invalid-value warning in any file forces `hold`.

So a compiled `--settings` on Run B's own command line sets it for that one session. It is the same mechanism
`DIALOG_SETTINGS` uses for Run C's rule (`SETTINGS_FLAG`, `src/cmd/verify/typed/runs.rs`). It overrides the user's
own `userSettings`. It does not override:
- managed policy;
- a repo, project or local settings file that tightens to `hold` or `refuse`;
- an invalid value in any file;
- the `receive` kill switch.

**3. The permission-mode class (the point that matters most).**
- **The class is binary.** A session is `bypass` only when its permission mode is `bypassPermissions` (or the CLI's
  bypass-available form). Every other mode is `prompting`.
- **What the receiver decides, with no `crossSessionInbound` value set:**
  - **The sender's class is read and compared:** the message is accepted iff the two classes are equal.
  - **The sender's class is not read:** a `prompting` receiver accepts.
  - **A `bypass` receiver** holds the message (`no-mode-asserted` or `bypass-default`).
  - Whether a socket-lane sender's class is read at all sits behind a further internal predicate. It is unmeasured.
- **Under the plan as written, both of verify's children run `prompting`:**
  - neither verify nor the rig passes a permission mode to Run B or to the sender. Run D alone carries
    `--permission-mode plan`, at `src/cmd/verify/typed/runs.rs:168`;
  - a tree search of `crates/viola-e2e/src/rig/`, `src/cmd/verify*` and `ledger.rs` for `defaultMode`,
    `bypassPermissions`, `dangerously` and `crossSessionInbound` finds nothing, so the rig's private config seeds no
    mode;
  - `-p` print mode does not change the class: the class is read from the mode, and the sender's mode is the
    default.

**So the message is NOT held by construction on this host under the rig's private config.** Both sessions are
`prompting`, and a `prompting` receiver accepts whether or not the sender's class is read. `/etc/claude-code` does
not exist on this host. Whether some managed-settings path holds a value is not readable from strings, and is
unmeasured.

It IS held wherever any settings source gives Run B a bypass default mode, or gives `crossSessionInbound` the value
`hold`. That is a user's real config under the automatic verify, not the rig's start. It is refused wherever the
value is `refuse` or the kill switch is off.

**4. The third option of F-P4-2.** The compiled setting exists, so F-P4-2 gains:
- **(c) Run B carries a compiled `--settings` literal, `{"crossSessionInbound":"accept"}`, for that one verify child
  (Recommended, with (a)'s residual kept for what (c) cannot override).**
  - **What it widens:**
    - Run B's argv gains `--settings`, as Runs C and D already carry it.
    - For Run B's whole life, a few minutes rather than seconds, any local session of the same user that can reach
      its socket and authenticate is delivered at once. There is no approval hold and no mode check. Reaching the
      socket means the user's own runtime dir, and the auth key published in the session registry.
    - This is still inside the 0700 probe dir and under the R8 strip.
    - A stray message from another live session of the user's in that window becomes a Run B turn. It shifts
      verify's counts and can redden other rows (a red, never a false pass), and its text is run by Run B's model in
      the probe dir under `prompting`.
  - **Rows it reaches:**
    - `(architecture · decision/verify-child-set)` (already W1);
    - `(security-plan · decision/verify-children-direct-spawn)` (Run B's argv);
    - `(security-plan · decision/verify-trusted-runs-risk)` (an inbound window on Run B);
    - the new residual row of W3, now naming what (c) leaves.

    `(architecture · decision/claude-flag-literals-in-agent-crate)` holds: the `--settings` literal already lives
    in the agent crate. `(architecture · absence/status-line-interception)` («no --settings at start») concerns
    `viola run`'s start, not verify. Not reached.
  - **What stays unmeasured, and fails the row in a real config:**
    - a managed or repo `hold` / `refuse`;
    - an invalid value in any file;
    - the `receive` kill switch;
    - whether a future CLI that drops the key rejects the `--settings` document (a Run B start failure would redden
      many rows, not only this one).
  - **Can the row still FAIL on a CLI that changed delivery? Yes.** `accept` only removes the hold. The row still
    passes only on an arrived UserPromptSubmit whose raw start is a compiled harness literal. A CLI that drops,
    re-routes, renames or re-wraps the message still reads `no-prompt` or `unprefixed`.
  - **Cost against (a):** one more argv widening and an open inbound window on Run B, in exchange for
    users whose own `userSettings` hold or refuse, or whose user-level default mode is bypass, no longer reading
    every version unverified.
  - **On his word for (c):** the plan returns to P4 (step 12's Run B args, step 9's literal, step 18's fake case, the
    residual row's words).
- (a) keeps its text. (b) keeps its cost.

**What the plan as written implies for `S1`:** no hold by construction under the rig's private config, as point 3
says. The three premises (the tool's addressing, the deferred tool, the kill switch) stay what only `S1` measures.

## The P4 forks' answers (appended 2026-11-21T06:14:19Z; snapped `inputs#I3`)

These are the founder's own words, given in an overseer dialog, read at 2026-11-21T06:13:58Z and relayed verbatim
in `inputs#I3`.

- **F-P4-2 — (c), «Run B несёт настройку accept (Recommended)».**
  - What is ratified: Run B carries the compiled `--settings` literal `{"crossSessionInbound":"accept"}` for that one
    verify child, and (a)'s residual stands for what it cannot override. This ratifies that widening over the rows
    point 4 above names, and nothing wider.
  - He was shown:
    - that it overrides the user's own settings and default mode;
    - that for Run B's life any other local session of the user's can deliver it a message with no approval, run by
      Run B's model in the private probe dir under a prompting mode. A stray message reads a red, never a false
      pass;
    - that managed policy, a repo setting, an invalid value or the CLI's internal switch still fail the row;
    - that the row still fails on a CLI that changed delivery.
  - Declined: (a) alone, and (b).
- **F-P4-1 — (a), «Добавить HF и HG в строку (Recommended)».** `(security-plan ·
  decision/live-rig-kept-stamp-owner-only)` names `HF` and `HG` beside CS, WS and PQ, its rule unchanged.

**The overseer's review** (`inputs#I3`, items 1–4). The plan returns to P4 for these items only:
- (c) is folded: Run B's args, the literal's home, the fake-agent case that reads the flag, the residual row's
  words, and one keep-the-red case.
- T1–T5 stand. Two notes become plan text: T3's lookup miss reads `no-prompt`, a finding; T5's three leavings are
  named unmeasured in the residual row.
- The `env-names-only` slip is said fixed.
- Nothing else moves.

## Revision 1 — a row outside §5 the narrowed change reaches (written 2026-11-21T07:49:35Z, revision run `2026-11-21T07-47-46Z-phase`)

The revision follows the founder's answers after the red `S1` (`inputs#I10`): the `identity-floor` row alone lands,
and the floor grows to thirteen names (`CLAUDE_CODE_CHILD_NOTES_DIR` and `CLAUDE_CODE_LAUNCHED_BY_SESSION_IDS`), the
non-identity list to four (`CLAUDE_PLUGIN_DATA`). His answer 2 says the rows stating «eleven names» are reworded.

Searched (`rec.py find --regex`, `eleven` · `11 … identity/names/floor` · `identity floor` · `IDENTITY_FLOOR`, over
architecture, security-plan, test-plan and obs-plan), the floor's count stands in these rows:
- in §5: `architecture · decision/identity-floor-eleven-names` · `security-plan · decision/identity-floor-eleven-names`
  (both «on a twelfth name», now taken) · `security-plan · decision/r8-claude-env-strip` · `obs-plan ·
  decision/identity-floor-always-stripped` («The 11 names»);
- **NOT in §5** — three test-plan rows:
  - `test-plan · decision/r8-claude-env-strip` («always including the 11-name identity floor»);
  - `test-plan · decision/claude-env-strip` («the eleven-name identity floor … the floor stays eleven names»);
  - `test-plan · decision/literal-test-oracles` («the 11 identity-floor names … written as literals»).

Read as holding, no count stated: `architecture · decision/r8-identity-floor` · `decision/child-env-claude-strip` ·
`security-plan · decision/claude-credentials-stripped-from-child` · `test-plan ·
decision/identity-floor-tested-against-literal-copy` · `contract/config-json/claude-env-keep` (§5 read them as not
reached; with the floor's growth two names stop being keepable, and their words still say so generically).

**Fork F-R1 — the three test-plan rows.**
- (a) **Reworded with the floor rows (lean)**: each names the floor by its thirteen names, or «the identity floor»
  with no count, as the other rows are reworded under answer 2. The lean is not decisive: his answer named the rows
  stating «eleven names», and these state the count in other words («11-name», «11 identity-floor names»).
- (b) Left as they stand: three test-plan rows then state a floor of eleven beside a thirteen-name `IDENTITY_FLOOR`.

**Answered (written 2026-11-21T08:04:48Z, `inputs#I11` item 2):** (a), inside the founder's ratification of the
thirteen-name floor — he was told the rows stating the old count are reworded; the three rows state the same floor in
other words, so they are reworded with the other floor rows, by NAME, no count where a row can avoid one. Authority:
`delegate overseer` — a delegate's confirm, PROVISIONAL for the epoch batch; not a fork that holds implement.
