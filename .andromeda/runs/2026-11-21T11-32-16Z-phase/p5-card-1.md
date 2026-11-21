# P5 review card 1 — «Cross-session prefix ledger row» (`2026-11-21-cross-session-prefix-ledger-row`)

**The plan to open:** `.andromeda/cache/docs/2026-11-21-cross-session-prefix-ledger-row/plan.md` (rev 3). No plan stands
in the chunk's folder; writing in that file changes no record until a `put`. `python -X utf8
~/.claude/skills/andromeda-tools/scripts/rec.py diff --root . 2026-11-21-cross-session-prefix-ledger-row` is a diff of the
chunk's records. The answers it rests on: `founder-forks.md` of `.andromeda/runs/2026-11-21T11-32-16Z-phase/` (§0–§9 and
the answers appended at P4), `inputs#I2`.

## Goal
The 29th relied ledger row `cross-session-prefix` (v2-25): a print-mode `claude -p` sender (Haiku, its one allowed tool
`SendMessage`) sends one compiled message to Run B's session id, Run B started with the compiled
`{"crossSessionInbound":"accept"}` literal; the row passes only when the message arrives as a UserPromptSubmit whose raw
start is a compiled harness literal. The first start `XS` also diagnoses, by the operator's F1 answer: the sender's
capture plugin runs a NEW names-only mode of the capture arm — only `Tool.<k>.json` files of closed words, never a
payload — and the stamp gains `sender_tools` (`send` · `search` · `list` · `other`) and `send_result` (`delivered` ·
`refused` · `none` · `unread`).

## Shape
- **Phase A** (28 rows): the names-only mode, its classifiers, the canary and no-payload cases; whole workspace green.
- **Phase B**: the row, `harness_prefix`, the sender texts, the accept literal, Run B's cross-session turn between
  `/compact` and `/clear`, the fake sender/receiver, the rig's `XS`/`XS-2` labels and four summary lines, every relied
  count to 29; three remove-the-guard controls (`prefix-match`, `names-only-no-payload`, `unknown-shape-unread`).
- **Phase C**: the round — `XS` on 2.1.328 by path, `--cap 1 --spare 1`; `XS-2` only after a red that NAMES a cause and
  its mend through the revision door. The cause table (readings → C1…C5, the ambiguous triples said as ambiguous) is in
  `## Implementation notes`.
- **Phase D**: on 29/29 only, the record copied over `fixtures/claude/2.1.328/` (32 → 33, predicted).
- **Phase E**: the operator pass.

## Top acceptance criteria
- `matrix#v2-25` — `XS`: `cross-session-prefix: pass`, `cross-session: harness`, `pass-count: 29/29`, kept stamp
  `29 ok`; the fake-agent pass case; after the landing `contract_ledger_probes` at `29 pass 0 fail`; the row keeps its
  red under the accept literal (`cross_session_prefix_keeps_its_red_under_the_accept_settings`).
- Names-only: only closed-word `Tool.<k>.json` files, no payload file, no canary byte; the `names-only-no-payload`
  control holds; what a SIGKILL leaves is closed words only.
- `send_result` reads only the boolean in the tool's stated shapes; any other shape `unread`, never `refused`.
- After the round: `kept-floor: unchanged` (`HF`/`HG`), `live-stamp: unmoved`, `stale-sockets: none added`.

## Touchpoints
31 research list items (30 modify, 1 derived: the re-recorded 2.1.328 set): the agent crate (`ledger.rs`, `hook.rs`,
`lib.rs`, four test files), `src/cmd/hook.rs`, `src/cmd/verify.rs` and `typed/{.,runs,turns}.rs`, the fake agent,
`tests/{cli_verify*,contract_*,security_negatives_env_capture,support/verify}`, the rig, `schemas/diag-line.v1.json`.

## Test commands
44 `[[gate]]` entries: 16 seen green on the previous chunk; 8 `new` baseline-run here (12 green; 14 red, vacuous until
its cases exist; 27–31 and 34 red on absent inputs, each with a must-pass and a must-fail control fired as the entry in
a scratch clone, `p5-controls.md`); 20 `leg` entries (round, live, operator).

## The concretization preview (v2-25)
- **Now:** «verify's relied ledger row `cross-session-prefix` reads `pass` on claude 2.1.328 live in the rig's scratch
  home: `S1` (rig label `HF`) prints `cross-session-prefix: pass`, `cross-session: harness` and `pass-count: 29/29`,
  and its kept stamp reads `29 ok`. Its probe: Run B starts with the compiled `crossSessionInbound` accept literal; a
  print-mode `claude -p` sender's one `SendMessage` to Run B's session id arrives as a UserPromptSubmit whose raw start
  is one of the four compiled harness literals (outcome `harness`). The recorded `UserPromptSubmit.cross-session-1.json`
  in `fixtures/claude/2.1.328/` is replayed by the fake agent in CI, `contract_ledger_probes` reading 2.1.328 at
  `29 pass 0 fail`; under the accept literal the row still reads `fail` when no prompt arrives or it arrives unprefixed
  (`cross_session_prefix_keeps_its_red_under_the_accept_settings`). A red `S1` verifies nothing.»
- **Proposed:** the same words with the start renamed — «`XS` (rig label `XS`, kept stamp `cross-stamp.json`)» in place
  of «`S1` (rig label `HF`)», and «A red `XS` verifies nothing.» Nothing else moves; the diagnostic words
  `sender_tools` / `send_result` are readings and enter no acceptance.

## Checks
- `gate.py run --dry-run`: 44 entries parse, exit 0. `planlint.py check`: `hits: 0`. `inputs.py verify`: 2 entries,
  unchanged 2, uncited 0. `gate.py scope --lists-only`: `scope lists: parsed 31 items`. Modify paths: all exist.
- **Found and fixed at P5:** entries 29 and 30 (the `HF`/`HG` kept-stamp and live-stamp after-reads) read GREEN with
  every input absent (empty equals empty); re-spelt with presence tests, re-baselined red, controls green and red as
  they must (`p5-controls.md`, run 2).
- **Warn:** plan prose 439 lines, about 10 % past the ~400 soft bound.
- **Read by search:** the rows files (sum 311 586 B, past 250 000 B) and the seven history files (by the chunk's
  terms); every cited row present. Check 11 (b): the reused rig's last report records no harm to the host.
- **Item 10:** `contract_ledger_probes` stamps `2.1.328` alone (`DRIFT_ONLY` the five older sets): a 29th id reads none
  of them red; no fork. No row outside the block's §4 found reached (item 11).
- Validation-1: aligned with `doc:2026-11-21-cross-session-prefix-ledger-row/scope` (rev 3) and the working entry.

## Beyond the plan
none

Apply? (yes / review / cancel)
