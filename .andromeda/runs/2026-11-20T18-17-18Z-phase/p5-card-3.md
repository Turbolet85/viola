# P5 card 3 — «Harness-prefix and identity-floor ledger rows» (2026-11-20-harness-prefix-and-identity-floor-ledger-rows)

**Which card this is.** The relay calls it «card 2». On disk it is `p5-card-3.md`, because `p5-card-2.md` was the
re-prompt after the overseer's P5 question. This card replaces both.

**The plan:** `doc:2026-11-20-harness-prefix-and-identity-floor-ledger-rows/plan`, **rev 5**. Its render is at
`.andromeda/cache/docs/2026-11-20-harness-prefix-and-identity-floor-ledger-rows/plan.md`; open it in an editor.
- No plan stands in the chunk's folder.
- Writing in that file changes no record until a `put`.
- `python -X utf8 ~/.claude/skills/andromeda-tools/scripts/rec.py diff --root . 2026-11-20-harness-prefix-and-identity-floor-ledger-rows`
  is a diff of the chunk's records.

**Inputs:**
- `inputs#I1`: the directive;
- `inputs#I2`: the P3 relay;
- `inputs#I4`: the P5 question;
- `inputs#I3`: the forks' answers and the review, read at 2026-11-21T06:13:58Z;
- the block, `.andromeda/runs/2026-11-20T18-17-18Z-phase/founder-forks.md`, with the P5 answer and the forks'
  answers appended.

## 0. The forks are answered — the founder's own words (`inputs#I3`)

- **F-P4-2 — (c):** Run B carries the compiled `--settings` `{"crossSessionInbound":"accept"}` for that one verify
  child, with (a)'s residual kept for what it cannot override.
  - It is ratified over the rows the P5 answer's point 4 names, and nothing wider.
  - (a) alone and (b) are declined.
- **F-P4-1 — (a):** `(security-plan · decision/live-rig-kept-stamp-owner-only)` names `HF` and `HG` beside CS, WS and
  PQ, its rule unchanged.
- **The hold is lifted.**
- **T1–T5 stand as leaned** (`inputs#I3` item 2).

## 1. What rev 5 changed (the review's items 1–3), step by step

- **Step 9:** `CROSS_SESSION_SETTINGS` = `{"crossSessionInbound":"accept"}` gets its home in `ledger.rs`, beside
  `DIALOG_SETTINGS`. One line is added (T3): the address stays the session id, and a lookup miss reads `no-prompt` —
  a finding, with no new mechanism.
- **Step 12:** Run B's `trusted_args` carry `SETTINGS_FLAG` + `CROSS_SESSION_SETTINGS` before the plugin dir, the
  form `dialog_run` uses. The sender carries none. What still wins over the literal, each failing the row:
  - managed policy;
  - a repo or local setting that tightens;
  - an invalid value;
  - the CLI's `receive` switch.
- **Step 14:**
  - The fake receiver reads its inbox only when its own argv carries `--settings` equal to the literal.
  - New fake case: `receiver_reads_no_inbox_without_the_accept_settings`.
- **Step 18:**
  - `cross_session_prefix_reads_pass_over_the_fake_sender` also asserts that Run B's argv carries the literal and the
    sender's carries no `--settings`.
  - The keep-the-red case (review item 1) is ONE rstest, `cross_session_prefix_keeps_its_red_under_the_accept_settings`:
    - `case_1_no_prompt_arrives` → `sender-failed`;
    - `case_2_arrives_unprefixed` → `unprefixed`;
    - each reads `fail`, exit 1, with the literal on Run B's argv.
  - It replaces `cross_session_prefix_reads_fail_when_no_message_arrives`.
  - The sender-exits-0 `no-prompt` path stays at unit level (step 16's `case_3_no_prompt`). Over the fake it would
    wait out the 30 s window, past the 20 s kill.
- **Expected amendments:**
  - W4 (the accept literal) joins W1–W3 under `rul-bznresq5`, with F-P4-1.
  - `verify-children-direct-spawn` gains Run B's argv.
  - `verify-trusted-runs-risk` names Run B's inbound window: a stray message reads a red, never a false pass.
  - The new residual row's proposed words carry three things:
    - W3, the misaddressing;
    - W4's leftovers: managed, repo, invalid value, the switch;
    - T5's three leavings, named UNMEASURED: a registry entry, a history line, the `cc-socks` socket.
- **Also folded:** Metadata (inputs, authority, the revision line); touchpoints (`ledger.rs`, `runs.rs`, the fake
  cases, `cli_verify.rs`); one new acceptance criterion (accept); two Constraints lines ((a) alone and (b) declined;
  no settings on the sender, nothing wider than the one literal); and entry 13's `note`.
- **The master-row slip, review item 3:** `env-names-only` is obs-plan's row. It was fixed in the plan at rev 4 and
  stands so at rev 5. The block's §5 line is left as written.
- **Nothing else moved** (review item 4):
  - the cap — TWO base starts, `HG` only after an unknown-name red on the identity row alone and his
    classification; ONE spare, `HF-2`;
  - both rows' predicates;
  - 2.1.328 alone;
  - what «verified» rests on;
  - all 38 gate entries' `run` texts. Every baseline and every clone control stands as recorded.

## 2. The plan at a glance (unchanged but for §1)

- **Two relied rows, 27 → 29:**
  - `cross-session-prefix` (v2-25): a `claude -p` sender on Haiku whose one allowed tool is `SendMessage`. It sends
    one message to Run B's session id, between `/compact` and `/clear`. The arrival's raw start is read against the
    four literals.
  - `identity-floor` (v2-26): the capture arm writes `CLAUDE*` names, never values, on SessionStart. Kept names are
    removed in memory. An unknown or unshaped name fails the row.
- **Order:**
  - A: the capture arm and the canary test, 27 rows, whole workspace;
  - B: rows, sender, fake replay, rig labels, three controls;
  - C: `S1` = `HF`, `--cap 1 --spare 1`;
  - D: the landing and every layer;
  - E: the operator pass.
- **The live start:** 2.1.328 by path, the rig's scratch home and private config. No key and no byte into any dialog.
  The credentials file is read once.

## 3. Concretization previews (claims written only after the word)

- **v2-25:** «verify's relied row `cross-session-prefix` reads `pass` on claude 2.1.328 live in the rig's scratch
  home.»
  - `S1` `HF` reads `cross-session-prefix: pass`, `cross-session: harness`, `pass-count: 29/29`, and the kept stamp
    `29 ok`.
  - Run B starts with the compiled `crossSessionInbound` accept literal.
  - A print-mode sender's one `SendMessage` to Run B's session id arrives as a UserPromptSubmit whose raw start is a
    compiled harness literal.
  - The recorded `UserPromptSubmit.cross-session-1.json` is replayed by the fake agent in CI: `contract_ledger_probes`
    reads 2.1.328 at `29 pass 0 fail`.
- **v2-26:** unchanged from card 1.
  - `S1` reads `identity-floor: pass` and `env-unknown: 0`.
  - Every `CLAUDE*` name the CLI adds to a hook child is on the floor or the three-name list.
  - `security_negatives_env_capture` is green in CI.
  - The recorded `Env.recorded-1.json` is replayed in CI.
- Neither is gated. A red `S1` verifies neither.

## 4. Checks, all re-run on rev 5 (every check after an edit, as the letter requires)

- `gate.py run --dry-run`: exit 0, 38 entries.
- `planlint`: `hits: 0` · 12 (c) `0 of 21` · 12 (d) 3 claims listed, each read and holding · 4 (17) `0 of 23`.
- `gate.py baseline`: `seen green 11 · not seen 0 · new (key) 12 · leg 15`. Every new non-leg entry carries its
  `baseline`, unchanged since rev 2.
- `gate.py scope --lists-only`: `scope lists: parsed 23 items (new 0 · modify 22 · derived 1)`.
- `inputs.py verify`: `4 entries — unchanged 4 · … uncited 0 · unparsed 0`.
- Cited rows: `cited 56 · missing 0` against the union and the standing lists.
- **Check 6, size, WARN:** about 780 lines of prose outside the fence (1070 rendered, 291 fence).
- **Validation-1:** aligned. The settings literal is the founder's answer inside the entry's probe; nothing wider.

## 5. The run, verbatim where a tool printed it (unchanged since card 1)

The union lines, the `rows-union` line, the three `[RETYPED]` lines, the one re-spawn, the non-zero exits and the
letter notes stand as card 1 §4 printed them: `.andromeda/runs/2026-11-20T18-17-18Z-phase/p5-card-1.md`.

Added since card 1:
- `inputs.py snap` → I3 and I4, both exit 0.
- No new refusal, `UNPARSED`, `INDETERMINATE` or `NOT DERIVED` line.

## Beyond the plan

none

---
Apply? (yes / review / cancel)
