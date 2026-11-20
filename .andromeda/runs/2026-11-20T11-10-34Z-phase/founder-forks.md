# Founder forks — 2026-11-20-permissionrequest-question-answered (phase run 2026-11-20T11-10-34Z, end of P3)

The phase stops here, before P4, under the overseer's directive (`inputs#I1` item 3). Nothing is planned, capped or
started until the founder's word on this block. No live start was made and nothing under the real `~/.claude*` was read
in this phase. The installed 2.1.328 and 2.1.329 bundles were read as strings only, with no start.

## The block

**Window.** One implement window holds the product change, the probe, the fake-agent tests and the master rows. The
live re-record is one operator leg inside it. Recommendation: 2.1.329 is NOT in this window (Q3), so the entry does
not split. If the founder wants 2.1.329 stamped, it leaves as its own route entry placed before «Version done-check»,
or folds into the product re-pin step. The size risk is the stamped-set cascade (Q3), not the code.

**How a question is answered today, and what changes** (code at `1e9f1ea3`):
- Today: a question's PreToolUse is held, and the driver's `answer` maps to PreToolUse `allow` + `updatedInput`.
  A PermissionRequest that repeats an armed question (same tool, equal input) is answered from that arm, null unless
  it is a plan revise, and raises nothing (run/dialog.rs:243-247, 477-493).
- A question is «first raised by PermissionRequest» only when NO arm matches. That happens when its PreToolUse never
  reached the wrapper, or when a later PreToolUse of the same tool dropped its arm, as two parallel questions can
  (run/dialog.rs:259). Such a question is logged but never held: `unanswerable` (run/dialog.rs:250). Its reply is null
  at once, the human answers, and a later `answer` is refused `unknown-dialog`.
- For the driver: such a question is held like any dialog, and `answer` with `answers`/`annotations` (the same shape
  as today, CLI and MCP unchanged) makes the hook print
  `{"hookSpecificOutput":{"hookEventName":"PermissionRequest","decision":{"behavior":"allow","updatedInput":<the input + answers + annotations>}}}`.
- For the human at the keyboard: unchanged. A key takes the wheel and hands the held dialog back null at once, the CLI
  draws its own dialog, and a human wheel or an unverified CLI holds nothing.
- For a dialog that expires: unchanged. At the hold's end the reply is null, with one `dialog-expired` and
  `dialog-answered{deadline_hit:true}`. A later answer is `unknown-dialog`.

**The `updatedInput` shape the CLI accepts, and where that reading comes from.** It comes from the installed bundles,
read as strings: 2.1.328 (sha256 `a982d38030b97e75…`) and 2.1.329 (`30e2724e2aecb1bf…`). The PermissionRequest hook's
`allow` with `updatedInput` re-runs the tool's own permission check on the changed input and returns
`allow`/`updatedInput`/`decisionReason{hook PermissionRequest}`. The two versions carry the same branch; only the
minified names differ. The shape assumed is the PreToolUse one: the tool input plus `answers` (question text →
label) and any `annotations`. It has never been run on a PermissionRequest. **One unmeasured branch:** a session of
kind `strict` refuses any changed input («…runs a call only as it was asked. Nothing ran.»). The default is
`launcher`. Which kind a viola-wrapped interactive session runs is unread.

**When the CLI ignores or rejects the answer.**
- An older CLI, or one whose stamp lacks the new row or reads it `fail`, is unverified: the question is null at once,
  `answer` is refused `unverified-cli` (exit 12), and the human answers in the CLI as today. That is the conservative
  side, and it is the new row's job.
- A malformed shape never reaches the CLI: `Response::parse` and `validate_paste_text` refuse first (exit 13 or
  invalid-params). viola does not check answer keys against the question texts, as on the PreToolUse path today.
- On a verified version, a `strict`-kind session would refuse with the CLI's fixed message. The human then sees a
  denied tool call and no dialog. No predicate below covers that kind; it stays a named, unmeasured residual.

**Q1 — the answer path** (recommended first):
1. **(Recommended) Hold the arm-less question.** Remove `unanswerable` for it and add the
   `(PermissionRequest, Question)` arm to `decision_body`. Arms keep today's `continued`. Cost: 2 product files, 4
   unit cases rewritten, Path 4's permission case rewritten. It fails only where the CLI ignores the body (the strict
   residual above).
2. Also re-raise the continuation of an UNANSWERED arm. Set aside: an expired hold or a human hand-back would come back
   to the driver after the human or the expiry already had it. That breaks the human-always-wins boundary and the
   expiry fall-back, and gives one call two dialog ids.
3. Keep both parallel arms instead (no `retain` drop) and never answer on PermissionRequest. Set aside: a PreToolUse
   that never reached the wrapper still leaves the question to the human, so v2-20 is not met.
4. `deny` + a message carrying the answers. Set aside: the model reads a denial, not a tool result, so the cap fails.

**Q2 — where the probe lives** (recommended first):
1. **(Recommended) A fourth compiled prompt, last in Run C.** It asks one question. Its PreToolUse (ordinal 4) gets
   no answer file, as Run D's first does. The CLI then raises the question's PermissionRequest (ordinal 2), and a sixth
   closed `ProbeAnswer` id answers it through `probe_body` → `decision_body` (every question's first option). Pass
   needs that turn to hold a question PreToolUse with no decision, then a PermissionRequest with an equal
   `tool_input`, then a PostToolUse of the same call whose `tool_response.answers` equals the first labels.
   `question-answer`'s «no question PermissionRequest anywhere in the run» clause is scoped to its own turn; its intent
   (the PreToolUse answer satisfied the dialog) holds.
   - Cost: one model turn more in Run C (its time is unmeasured), no new spawn pair, run or first-screen key. One more
     transcript turn in Run C's accepted residual, nothing outside the probe dir. Placed last, so a hang costs only
     the new row.
2. A new Run G. Set aside: a seventh PTY run, a new spawn pair and first-screen keys, a wider verify child set and a
   longer verify for nothing the fourth prompt does not read.

**What a 27th relied row does.** Every existing stamp reads unverified until re-stamped:
- the live home's 24-row stamp (already short);
- the rig's kept 26-row 2.1.328 stamp;
- every committed stamped set under the fake agent: `STAMPED` = 2.1.287 · 2.1.302 · 2.1.325 · 2.1.328
  (tests/contract_ledger_probes.rs:46), none of which holds the new question variants;
- every stamped test home, since `DEFAULT_CLI_VERSION` = 2.1.287 (crates/viola-e2e/src/harness/boot.rs:24).

**Q3 — which versions are re-stamped, and in which home** (recommended first):
1. **(Recommended) 2.1.328 only, re-recorded whole by one live `viola verify --record` in the rig's scratch home.**
   2.1.287, 2.1.302 and 2.1.325 move to `DRIFT_ONLY` (their dialog variants still replay byte for byte), and
   `DEFAULT_CLI_VERSION` moves to 2.1.328. Cost: 1 start, three versions lose stamped coverage, and the default-version
   move re-runs every stamped test home (a sweep).
2. 2.1.328 and 2.1.329 (a first whole set of 2.1.329). Cost: 2 starts. 2.1.329's first screens and every row are
   unmeasured, so a first-screen drift spends its start. It needs the window split above.
3. Re-record every stamped version (2.1.287, 2.1.302, 2.1.325, 2.1.328). Cost: 4 starts for versions no session runs.
4. (Added after the overseer's P3 question 1; see the answers below.) **Synthetic question variants beside the
   recorded 2.1.287 · 2.1.302 · 2.1.325 sets**, so they stay in `STAMPED` and `DEFAULT_CLI_VERSION` does not move,
   with 2.1.328 still recorded live by S1. Cost:
   - 0 starts for the three old sets and 1 for 2.1.328;
   - 9 variant files (an unanswered question PreToolUse, its PermissionRequest with an equal `tool_input`, the
     PostToolUse with the first labels, × 3 sets), plus a provenance note per set in the form of `RELAYED.md`;
   - no list or default move;
   - a founder ruling widening test-plan `decision/hook-payloads-recorded-by-verify`, whose only exception is
     StopFailure.
   It cannot prove that those three versions raise the PermissionRequest or act on `updatedInput`: the fake agent
   replays the authored PostToolUse, so their row passes by construction.
The live home's re-stamp stays the product re-pin step's in every option.

**Every reached master row** (all seven searched, the record in research's `## Reached master rows`):
- **Reached:**
  - architecture `decision/permission-request-question-stays-null` — retired or replaced: this chunk ends its rule;
  - architecture `contract/answer-refusals/unknown-dialog` — `unknown-dialog` narrows to a late answer or an expired
    hold;
  - architecture `contract/hook-contract/permission-request` — gains the question's `allow` + `updatedInput` form;
  - test-plan `decision/dialog-answer-mapping`, `contract/hook-decision-body/question` and
    `contract/hook-decision/question` — each gains the PermissionRequest question form;
  - security-plan `decision/capture-answers-closed-probe-ids` — its rule is unchanged, and its closed set grows by
    one id;
  - test-plan `decision/ledger-probe-contract-suite` — its words hold and its literal lists move.
- **Read as not reached:**
  - architecture: the hook-to-kind map and continuation contracts; `hook-tiers`; the pre-tool-use contract; the
    exit-plan-mode decision; the capability-ledger and verified-version rows (by rule, no count);
    `verify-child-set` and first-screen (reached only on Q2's option 2); the step and summary lines;
  - security-plan: driver-permission-policy, mcp/channel `answer`, dialog-answers-need-verify-stamp,
    stamps-written-only-by-verify, unverified-cli-gate, exit 12, verify-writes-no-dialog-byte, and the two probe
    residual rows;
  - test-plan: the permission and plan body rows, hook stdout, fake-agent script, version-gate, verify summary,
    answer-unverified;
  - obs-plan: every ledger, verify and expiry row (no new line, field, subject or code);
  - design-system: both cli-verification rows;
  - layout-templates: verify exit-0;
  - a11y-plan: no hit.

**Widenings, said whole here:**
- (W1) viola answers a question first raised through PermissionRequest. This retires
  `decision/permission-request-question-stays-null`; the driver's answers ride an `allow` the CLI acts on.
- (W2) Run C gains a fourth compiled prompt and a second answered `allow`. The capture arm's closed `ProbeAnswer` set
  gains a sixth id. Each verify paste so far was a ratified widening.
- (W3) On Q3's option 1, three committed versions leave the stamped list and the default test version moves.
- The answers reaching the CLI through PermissionRequest are the same driver-supplied content as on PreToolUse, so
  `decision/driver-permission-policy-outside-viola` is read as covering them. That reading is the founder's to confirm.

**Every live start's shape, before the count:**
- **S1: claude 2.1.328, `viola verify --record` through `viola-live-rig`'s record arm, in the rig's scratch home.**
  - **Home:** the rig's own private Claude config, never the live `~/.viola` and never `~/.claude`.
  - **Processes:**
    - the rig;
    - `viola verify --record`, with the 2.1.328 install dir first on its `PATH` and `claude --version` read by path
      before the start;
    - `claude --version`;
    - the print probe;
    - Runs A–F as PTY children at 80×24 under the R8 strip;
    - the one `claude agents --json` read in Run B.
  - **Every byte typed:** only verify's compiled pastes.
    - Run B: the probe prompt, the long, tag-like and newline texts, `/mcp disable viola-probe-absent`, `/context`,
      `/compact`, `/clear`.
    - Run C: its three prompts plus the new fourth.
    - Run D: the plan prompt.
    - Runs A, E and F: none.
    - No key into any dialog.
  - **Every dialog, and how it is answered:** only by the capture hook's closed ids.
    - Run C: PreToolUse 1 `questions-first-and-free-text`; 2 and 3 `question-first-option`; PermissionRequest 1
      `permit-allow` (its `touch` in its own dir); PreToolUse 4 unanswered; PermissionRequest 2 the new id.
    - Run D: as today.
    - Trust: Run F writes and removes its own trust record; a modal start is killed with no key; a browser-tools
      prompt is left standing.
  - **The stop:** each run ends at its turn end or its probe deadline. Verify prints its summary. The rig kills its
    process group at its bound. A stopped verify cleans up as `interrupted`.
  - **Files written:**
    - the scratch home's `ledger/stamps.json` (27 rows);
    - the kept stamp under `target/live-rig/` (owner-only, no-clobber);
    - `fixtures/claude/2.1.328/` through the `--record` scrub, refused whole on an unclean file;
    - 0700 probe dirs, removed at the end;
    - the CLI's own Run B, C and D transcripts under the private config's `projects/` (the accepted residual).
  - **A predicate the reading can FAIL:** the new row reads `pass`.
    - Red: the new turn holds no question PermissionRequest capture (the CLI did not raise one).
    - Red: a PermissionRequest but no PostToolUse whose answers equal the first labels (body ignored or refused).
    - Red: the turn never settles before the probe deadline.
    - Also red: any of the 26 other rows `fail` (the scoped `question-answer` among them), or a `--record` refusal.
  - **What it READS:**
    - row words (`pass`/`absent`/`fail`) and first-screen closed words with whole-number settles, from verify's stdout
      and the stamp;
    - hook payloads, from the capture arm, kept only as scrubbed recorded fixtures;
    - the screen, only by compiled literals.
    - No text is read from a screen. The token window is the rig's own check.
  - **Wheel:** no wrapper and no wheel; no typed key or answered dialog moves anything before a send.
- **S2: claude 2.1.329, the same shape** (only on Q3's option 2). Its first full recording also reads `fail` on a
  first-screen literal the compiled signatures do not hold.

**What each start needs ABSENT or PRESENT outside the tree.** Each item is a no-start gate entry right before its
start, so a resumed round still fires it:
- PRESENT: `~/.local/share/mise/installs/claude/2.1.328/claude` with sha256 prefix `a982d38030b97e75`.
- ABSENT: the rig's scratch home path (fresh).
- PRESENT: a verified owner-only `XDG_RUNTIME_DIR`.
- ABSENT: any `.viola-verify-*` dir in the probe cwd.
- PRESENT: `target/debug/viola`, its hash read right before the start.
- QUIET: `hostwatch.py read --last 15`.
- An entry AFTER the start reads the kept stamp as 27 rows, each `pass` (`absent` on browser-tools-prompt alone), and
  the fixture dir's diff as the expected variants only.

**The capability's concretized claim (v2-20).** On a verified CLI with the driver at the wheel, a question raised by
PermissionRequest with no matching armed dialog is held. `viola answer` (CLI and MCP) carrying answers makes the hook
print the `allow` + `updatedInput` body above. A human key, an expired hold and an unverified CLI leave it to the
human, as today. **«Verified» rests on:**
- Path 4 over the fake agent: `path4-permission.json`, the hook receipt holding the body, and MCP/CLI parity;
- the 27th row `pass` in `contract_ledger_probes` over every set left stamped;
- the S1 live reading `pass` on 2.1.328 in the rig's scratch home.
A failing or confounded live reading is recorded as read, never as verification.

**Recommended cap: 1 start (S1)** on Q3's option 1, **plus 1 spare.** The spare is re-fired only after S1's red cause
is read from its own record and mended in code; never a retry, and never for a green. Option 2 makes it 2 + 1.

**Carry.** `cry-3qv2r3iy` (the intermittent `rig_rehearsal_reads_the_box_empty_after_x_and_del` red) is acted on only
at its next red: read the rig summary and the `after-del` frame, name the cause, then mend. No retry, no loosened
entry.

**The founder's, waiting on his word:**
- Q1, Q2, Q3;
- the cap;
- that no key or byte goes into any CLI dialog (the probe answers by hook only);
- W1–W3, and the driver-policy reading;
- what «verified» rests on;
- any read or write under his real home. The rig's live arm checks its token window from the real credentials file:
  that read is the rig's standing grant, re-confirmed per chunk under `decision/live-round-rig-keys-own-pty`.

## Answers to the overseer's P3 questions

Answered from the tree and the store, with no start. The relay was read whole: `relay-permreq-p3-ask.md`.

**1. Why Q3 had no start-free option for the old sets, and whether the synthetic form holds.**
- **The precedent's premise, measured:** the 26th row (`stop-session-crons`) kept the four sets stamped with no
  recording start because every set's RECORDED `Stop.default.json` already carried `session_crons`. `grep -c
  session_crons fixtures/claude/<v>/Stop.default.json` reads 1 for 2.1.287, 2.1.302, 2.1.325 and 2.1.328. In 2.1.287
  it has been there since commit `7f1364f0` (2026-10-04, `git log -S session_crons`). The chunk report reads
  «`STAMPED` unmoved, no recording start». Its synthetic pieces were elsewhere:
  - a synthetic Stop added to the spine test set in `tests/support/verify.rs`;
  - the fake agent's argv-only `--stop-crons` for the wake tests.
  No synthetic payload stood inside a stamped set. The new row is different: no committed set records a question
  PermissionRequest that follows an unanswered PreToolUse inside verify's Run C, so the payload the row reads does not
  exist in any of them.
- **Whether the form holds:** no test refuses it.
  - `contract_fixture_hygiene::claude_fixtures_pass_hygiene` checks schema, absolute paths, username and email only.
  - `contract_fake_agent_drift` compares the fake's replay with the same file, so it passes.
  - The `--record` scrub never sees a hand-written file.
  - The refusing reader is the rule row test-plan `decision/hook-payloads-recorded-by-verify`: «Hook payload fixtures
    are recorded locally by live viola verify…», with StopFailure its only exception. The one earlier non-recorded
    payload in a set (2.1.287's four `RELAYED.md` files) stood on the founder's own live ruling, and
    `decision/relayed-fixtures-kept-superseded` keeps them only as superseded.
  - So the form holds only with a new exception to that row, which is the founder's. It is added above as Q3's
    option 4.
- **Its price:** 9 files, 3 provenance notes, one rule-row widening. No change to the `STAMPED` or `DRIFT_ONLY`
  lists, `ROW_IDS` still grows to 27, and no `DEFAULT_CLI_VERSION` move.
- **What it can NOT prove that a recording does:**
  - that 2.1.287, 2.1.302 and 2.1.325 raise a question PermissionRequest after an unanswered PreToolUse
    (2.1.287 has partial evidence: its relayed `PermissionRequest.ask-user-question.json` is a live interactive
    capture of one, taken outside verify);
  - that they act on `updatedInput`: the PostToolUse answers would be authored, and the fake replays them, so the row
    reads `pass` by construction;
  - each version's own payload keys.
  A stamp of those versions would then claim a behaviour its version was never measured to have.
- **2.1.328 under option 4:** recorded live by S1, recommended, because v2-20's «verified» needs one measured
  reading. Making it synthetic too (0 starts) leaves the body read from strings only, never run.

**2. Which session kind S1 measures.**
- **Read from the bundle strings, no start:** neither bundle builds a literal `kind:"strict"` object
  (`grep -c 'kind:"strict"'` reads 0 in 2.1.328 and in 2.1.329). The comparison `kind==="strict"` stands 20 times in
  2.1.328 and 23 in 2.1.329. In 2.1.328 the frozen `kind:"launcher"` (`Tbe`) is what the interactive local permission
  path passes: `…forRemoteExecution===!0||D.personOnly===!0?new Promise(()=>{}):Tun(Tbe,…` and `Wct(…,Tbe,…)`. Where a
  strict kind is built, and which flag or environment value selects it, is unread: the other `"strict"` strings found
  belong to the sandbox and policy settings, not to this kind.
- **viola's spawn code:**
  - `viola run`'s child gets `--plugin-dir <plugin>` plus the user's own arguments (src/run/mod.rs:215).
  - verify's Run C gets `--plugin-dir` plus `--settings` with the session `ask` rule (crates/viola-agent-claude/src/
    ledger.rs `DIALOG_SETTINGS`). Run D adds `--permission-mode plan` (src/cmd/verify/typed/runs.rs:165).
  - Both are interactive PTY starts under the R8 strip. Neither is print mode, and neither passes a
    remote-execution flag.
- **Reading:** on the strings, both run the same interactive path, so the same `launcher` kind. S1's `pass` on Run C
  is therefore read as covering a wrapped session on that version, unless the user's own arguments to `viola run`
  select another path.
- **Unmeasured, and what would read it:** a user argument that could select the strict kind is unmeasured. No
  start-free reading beyond these strings exists. A live reading is S1's own PermissionRequest payload, kept in its
  recorded fixture: its `permission_mode` key and the PostToolUse that follows. That proves the launcher path for
  Run C's flags only, not for every argument a user may pass.

**3. What moves under Q3's option 1.**
- **Stops running against 2.1.287, 2.1.302 and 2.1.325 as stamped:**
  - `tests/contract_ledger_probes.rs::contract_ledger_probes_pass_over_every_stamped_set` (the three leave `STAMPED`
    for `DRIFT_ONLY`; `contract_fake_agent_drift` keeps replaying them byte for byte, unchanged).
  - Every case on the stamped home at `RECORDED_CLI_VERSION = "2.1.287"` (tests/support/fake.rs:16) moves to 2.1.328
    and keeps running there. That is 13 root test files that name the constant: `channel_error_bodies`, `cli_answer`,
    `cli_fake_agent`, `cli_instance_state`, `cli_list`, `cli_send`, `cli_verify/runs`, `cli_version_gate`,
    `cli_wait_last`, `cli_wheel`, `hook_events`, `tui_passthrough` and `support/home`. In all, 39 root and viola-e2e
    test files take `stamped_home`/`StampedHome` (`grep -rln`).
  - The harness boot's `DEFAULT_CLI_VERSION` (crates/viola-e2e/src/harness/boot.rs:24), so every `harness_session`
    (4 viola-e2e test files), `agent-run.sh boot` and `run --perf`.
  - The fake agent's own `DEFAULT_CLI_VERSION` (src/bin/viola-fake-agent.rs:41).
  - Files naming 2.1.302 or 2.1.325 by literal (grep, e.g. `tests/cli_verify/runs.rs`, `crates/viola-e2e/tests/
    rig_rehearsal.rs`, `crates/viola-e2e/src/rig/`) are read case by case at P4. Any of them that asserts a verified or
    `0 fail` stamp on those sets moves to the drift-only reading or to 2.1.328.
- **Verified capabilities resting on them:** none rests on those three sets being STAMPED. Of the 18 `verified` caps
  of viola-0.2.0, the records that cite them (a store query over the capability records) are:
  - v2-04 and v2-05: their refs name `tests/contract_ledger_probes.rs`, which keeps running green over the stamped
    set left (2.1.328, 27 rows).
  - v2-02, v2-03, v2-16, v2-19, v2-23, v2-32, v2-33, v2-35 and v2-36: they cite LIVE readings or kept live stamps on
    2.1.325 or 2.1.326, never the committed fake-agent set, and those readings stand as taken.
  - v2-35's record also names the 2.1.287 · 2.1.302 · 2.1.325 bundles as read-from-source, which this does not touch.

The phase stops here again: no start, no P4.

## The founder's answers (relayed by the overseer; his words read at 2026-11-20T12:01:52Z)

Relayed in `~/dev/projects/additional/viola-overseer/relay-permreq-p3.md`, copied here verbatim in substance, with no
option's meaning changed. Authority: the founder's own answers, given in two of the overseer's dialogs (items 1–6); the
overseer's technical answers (items 7–12).

1. **Q1: «Удерживать такой вопрос (Recommended)».** Option 1: the arm-less question raised by PermissionRequest is
   held and answered `allow` + `updatedInput`; a human key, an expired hold and an unverified CLI leave it to the human
   as today. His ratification of W1 — the rule `decision/permission-request-question-stays-null` ends — over the nine
   reached rows this block names and nothing wider. He was shown the nine rows by master, the strict-kind residual as
   unmeasured, and the reading that `decision/driver-permission-policy-outside-viola` covers the answers; he chose
   the path with that reading stated. Options 2–4 declined.
2. **Q2: «4-й промпт в конце Run C (Recommended)».** Option 1 and his ratification of W2: a fourth compiled prompt last
   in Run C, a second answered `allow`, the sixth closed probe id, and `question-answer`'s clause scoped to its own
   turn. Run G declined.
3. **Q3: «Только 2.1.328 (Recommended)»** (his second dialog, after he first wrote that stamping should follow the
   host's CLI version automatically — see 6). Option 1 and his ratification of W3: 2.1.328 alone is re-recorded whole,
   live, in the rig's scratch home; 2.1.287, 2.1.302 and 2.1.325 move to `DRIFT_ONLY`; `DEFAULT_CLI_VERSION` and
   `RECORDED_CLI_VERSION` move to 2.1.328. Option 4 declined: no exception to
   `decision/hook-payloads-recorded-by-verify`. 2.1.329 is NOT in this chunk.
4. **The cap: «1 старт + 1 запасной (Recommended)».** ONE base start `S1` (`--cap 1`) and ONE spare, in the block's
   shape: claude 2.1.328 by path, `viola verify --record` through the rig, the rig's private config and scratch home,
   only verify's compiled pastes typed, NO key and NO byte into any CLI dialog (the capture hook's closed ids answer),
   the kept stamp under `target/live-rig/`, `fixtures/claude/2.1.328/` through the scrub. The spare is spent only
   after S1's red cause is read from its own record and mended in code — never a retry, never for a green. A start the
   rig refuses for the token window is uncounted. From his real home `~/.claude/.credentials.json` is READ once per
   start by the rig (a 0600 copy removed on every exit path); nothing is written under his real `~/.claude*` or
   `~/.viola`. A second base start is his.
5. **What «verified» rests on:** Path 4 over the fake agent, the 27th row `pass` in `contract_ledger_probes` over every
   set left stamped, AND the live reading `S1` `pass`. A failing or confounded live reading is recorded as read, never
   as verification.
6. **A route direction, not this plan:** a new CLI version on the host stamped automatically, in the product, as a new
   route entry right after this chunk; this chunk's wrap takes it at route-resolve. Not planned here.

**The overseer's technical answers:**

7. The strict-kind residual is written in the plan as unmeasured (which user argument could select it), with where it
   stands after the wrap proposed (a CARRY with a real first consumer, or a row's except); S1's recorded
   PermissionRequest payload is read for its `permission_mode` key as a READING, not an expect atom.
8. The stamped-to-drift-only move is planned as a list: every file naming 2.1.287, 2.1.302 or 2.1.325 by literal or
   through the two default-version constants, with what it becomes (runs on 2.1.328 · drift-only reading ·
   unchanged); the P5 card gives the count of each; a test that would silently stop asserting anything is named.
9. Every no-start entry stands right before `S1` so a resumed round fires it; the entry AFTER reads the kept stamp as
   27 rows and the fixture dir's diff as the expected variants only. The live home's `~/.viola/ledger/stamps.json` is
   read for its sha alone, before and after, and must read unmoved. The host is read before the start; the token
   window is the rig's own check.
10. A red `S1` stops implement BEFORE the operator pass, with the start's own words; the fake-agent tests, the
    controls and the recorded sets' suites are part of green before the start is made.
11. Leaves hold no figure the code states; the row count moves in code and tests only.
12. When the plan is approved the phase ENDS and nothing else starts. On an account-limit refusal, stop where you are.
